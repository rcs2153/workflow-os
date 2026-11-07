use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};

use super::{
    decode_json, encode_json, schema_recovery_required, validate_live_authority_schema,
    validate_schema_metadata, SqliteStateBackend, ADAPTER_SCHEMA_VERSION,
    LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION, LIVE_AUTHORITY_SCHEMA_CHECKSUM,
    LIVE_SANDBOX_AUTHORITY_SCHEMA_V11,
};
use crate::authorized_execution_continuity_state::internal::{
    AuthoritativeAttemptState, AuthoritativeWindowState,
};
use crate::dispatch_reservation::DispatchReservationRequest;
use crate::live_sandbox_continuation_authority::{
    LiveSandboxContinuationFacts, LiveSandboxPrincipalVerifier, LIVE_CAPABILITY,
};
use crate::operational_execution_window_opening::{
    operation_binding_commitment, OperationalExecutionWindowOpeningRequest,
};
use crate::{
    ActorId, ApprovalPresentationRecord, AuthorizedExecutionAttemptId, AuthorizedExecutionWindowId,
    CapabilityGrant, CapabilityGrantId, CapabilityGrantLifecycle, SpecContentHash, Timestamp,
    WorkflowOsError, WorkflowRun, WorkflowRunEventKind, WorkflowRunStatus,
};

impl SqliteStateBackend {
    pub(crate) fn register_live_sandbox_principal(
        &self,
        verifier: &LiveSandboxPrincipalVerifier,
    ) -> Result<(), WorkflowOsError> {
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| denied())?;
        validate_live_store(&tx)?;
        let changed = tx
            .execute(
                "INSERT INTO live_sandbox_principals (principal_id, verifier_payload)
             VALUES (?1, ?2) ON CONFLICT(principal_id) DO NOTHING",
                params![
                    verifier.principal().as_str(),
                    encode_json(verifier, "live principal verifier")?
                ],
            )
            .map_err(|_| denied())?;
        if changed != 1 {
            return Err(denied());
        }
        tx.commit().map_err(|_| denied())
    }

    pub(crate) fn revoke_live_sandbox_principal(
        &self,
        principal: &ActorId,
    ) -> Result<(), WorkflowOsError> {
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| denied())?;
        validate_live_store(&tx)?;
        let changed = tx
            .execute(
                "UPDATE live_sandbox_principals SET revoked=1 WHERE principal_id=?1 AND revoked=0",
                [principal.as_str()],
            )
            .map_err(|_| denied())?;
        if changed != 1 {
            return Err(denied());
        }
        tx.commit().map_err(|_| denied())
    }

    pub(crate) fn enable_live_sandbox_grant_store(&self) -> Result<(), WorkflowOsError> {
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| schema_recovery_required())?;
        match version(&tx)? {
            ADAPTER_SCHEMA_VERSION => {
                validate_schema_metadata(&tx)?;
                tx.execute_batch(LIVE_SANDBOX_AUTHORITY_SCHEMA_V11)
                    .and_then(|()| {
                        tx.execute(
                            "UPDATE schema_metadata SET schema_version=?1, checksum=?2
                             WHERE singleton=1",
                            params![
                                LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION,
                                LIVE_AUTHORITY_SCHEMA_CHECKSUM
                            ],
                        )?;
                        tx.pragma_update(
                            None,
                            "user_version",
                            LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION,
                        )
                    })
                    .map_err(|_| schema_recovery_required())?;
            }
            LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION => {}
            _ => return Err(schema_recovery_required()),
        }
        validate_live_authority_schema(&tx)?;
        tx.commit().map_err(|_| schema_recovery_required())
    }

    /// Accepts facts from the trusted Core resolver only. The model-facing
    /// boundary must not take these facts or grant payloads from a harness.
    pub(crate) fn issue_live_sandbox_grant(
        &self,
        facts: LiveSandboxContinuationFacts<'_>,
        grant: &CapabilityGrant,
    ) -> Result<SpecContentHash, WorkflowOsError> {
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| denied())?;
        validate_live_store(&tx)?;
        let observation = super::continuity_store::observe_continuity_trusted_time()?;
        let state = super::continuity_codec::load_snapshot(&tx)?;
        let window = state.windows.get(facts.window).ok_or_else(denied)?;
        let attempt = state.attempts.get(facts.attempt).ok_or_else(denied)?;
        if window.state != AuthoritativeWindowState::Executing
            || attempt.state != AuthoritativeAttemptState::Started
            || attempt.window_id != *facts.window
            || attempt.cursor != window.cursor
            || attempt.subject_actor_id != window.subject_actor_id
            || attempt.authority_commitment != window.authority_commitment
            || window.run_id != *facts.action.run
            || window.workflow_id != *facts.scope.workflow_id()
            || window.step_id != *facts.action.step
            || window.subject_actor_id != *facts.action.original_actor
            || window.immutable_run_bundle.root_hash() != facts.action.bundle_root
            || window.expires_at != facts.window_expires_at
            || state.trusted_time.eligibility
                != crate::authorized_execution_continuity_state::internal::ContinuityInstanceEligibility::LiveStateEligible
            || state.trusted_time.posture
                == crate::authorized_execution_continuity_state::internal::TrustedTimePosture::Quarantined
            || observation.source() != state.trusted_time.source
            || observation.provenance_commitment() != &state.trusted_time.provenance_commitment
            || observation.epoch_id() != &state.trusted_time.epoch_id
            || window.trusted_time_epoch_id != *observation.epoch_id()
            || state.trusted_time.last_observed_at.is_some_and(|time| observation.observed_at() < time)
            || observation.observed_at() < window.trusted_time_watermark
        {
            return Err(denied());
        }
        verify_opening(&tx, facts, &facts.action.commitment()?)?;
        let registration = active_principal(&tx, grant)?.commitment()?;
        let binding = facts.grant_commitment(grant, observation.observed_at(), &registration)?;
        let payload = encode_json(grant, "live sandbox grant")?;
        let count = tx
            .execute(
                "INSERT INTO live_sandbox_grants
                 (grant_id, attempt_id, binding, payload, registration_binding)
             SELECT ?1, ?2, ?3, ?4, ?5 WHERE NOT EXISTS
               (SELECT 1 FROM dispatch_reservations WHERE attempt_id=?2)
             ON CONFLICT(grant_id) DO NOTHING",
                params![
                    grant.grant_id().as_str(),
                    facts.attempt.as_str(),
                    binding.as_str(),
                    payload,
                    registration.as_str(),
                ],
            )
            .map_err(|_| denied())?;
        if count != 1 {
            return Err(denied());
        }
        tx.commit().map_err(|_| denied())?;
        Ok(binding)
    }

    pub(crate) fn revoke_live_sandbox_grant(
        &self,
        grant_id: &CapabilityGrantId,
    ) -> Result<(), WorkflowOsError> {
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| denied())?;
        validate_live_store(&tx)?;
        let count = tx
            .execute(
                "UPDATE live_sandbox_grants SET revoked=1 WHERE grant_id=?1 AND revoked=0",
                [grant_id.as_str()],
            )
            .map_err(|_| denied())?;
        if count != 1 {
            return Err(denied());
        }
        tx.commit().map_err(|_| denied())
    }
}

#[derive(Clone, Copy)]
pub(super) struct LiveGrantAdmission<'a> {
    pub(crate) grant_id: &'a CapabilityGrantId,
    pub(crate) facts: LiveSandboxContinuationFacts<'a>,
    pub(crate) credential: &'a [u8; 32],
}

pub(super) struct AuthenticatedLiveGrant<'a> {
    facts: LiveSandboxContinuationFacts<'a>,
    grant: CapabilityGrant,
    registration: SpecContentHash,
}

impl<'a> LiveGrantAdmission<'a> {
    pub(super) fn authenticate(
        self,
        tx: &Transaction<'_>,
        request: &DispatchReservationRequest,
        now: Timestamp,
    ) -> Result<AuthenticatedLiveGrant<'a>, WorkflowOsError> {
        let (binding, payload, registration_binding): (String, String, String) = tx
            .query_row(
                "SELECT binding, payload, registration_binding FROM live_sandbox_grants
                 WHERE grant_id=?1 AND attempt_id=?2 AND revoked=0",
                params![
                    self.grant_id.as_str(),
                    request.capability.attempt_id.as_str()
                ],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .optional()
            .map_err(|_| denied())?
            .ok_or_else(denied)?;
        let grant: CapabilityGrant = decode_json(&payload, "live sandbox grant")?;
        let expires_at = grant.expires_at().ok_or_else(denied)?;
        if grant.grant_id() != self.grant_id
            || binding != request.executor_commitment.as_str()
            || grant.lifecycle() != CapabilityGrantLifecycle::Active
            || grant.capability().as_str() != LIVE_CAPABILITY
            || grant.issued_at() > now
            || now >= expires_at
        {
            return Err(denied());
        }
        let verifier = active_principal(tx, &grant)?;
        verifier
            .authenticate(self.credential)
            .map_err(|_| denied())?;
        let registration = verifier.commitment()?;
        if registration.as_str() != registration_binding {
            return Err(denied());
        }
        Ok(AuthenticatedLiveGrant {
            facts: self.facts,
            grant,
            registration,
        })
    }
}

impl AuthenticatedLiveGrant<'_> {
    pub(super) fn consume(
        self,
        tx: &Transaction<'_>,
        request: &DispatchReservationRequest,
        now: Timestamp,
        window_expiry: Timestamp,
    ) -> Result<(), WorkflowOsError> {
        let facts = self.facts;
        let action = facts.action.commitment()?;
        if facts.window != &request.capability.window_id
            || facts.attempt != &request.capability.attempt_id
            || facts.action.run != &request.expected_window_binding.run_id
            || facts.action.step != &request.expected_window_binding.step_id
            || facts.scope.workflow_id() != &request.expected_window_binding.workflow_id
            || facts.action.original_actor != &request.expected_window_binding.subject_actor_id
            || facts.action.bundle_root
                != request
                    .expected_window_binding
                    .immutable_run_bundle
                    .root_hash()
            || facts.window_expires_at != window_expiry
            || action != request.invocation_commitment
            || request.operation_binding_commitment != opening_binding(facts, &action)
        {
            return Err(denied());
        }
        verify_opening(tx, facts, &action)?;
        facts.verify_bound_grant(
            &self.grant,
            now,
            &self.registration,
            &request.executor_commitment,
        )?;
        tx.execute(
            "INSERT INTO live_sandbox_grant_uses (grant_id, attempt_id) VALUES (?1,?2)",
            params![self.grant.grant_id().as_str(), facts.attempt.as_str()],
        )
        .map_err(|_| denied())?;
        Ok(())
    }
}

fn active_principal(
    tx: &Connection,
    grant: &CapabilityGrant,
) -> Result<LiveSandboxPrincipalVerifier, WorkflowOsError> {
    let payload: String = tx.query_row(
        "SELECT verifier_payload FROM live_sandbox_principals WHERE principal_id=?1 AND revoked=0",
        [grant.subject().as_str()],
        |row| row.get(0),
    ).optional().map_err(|_| denied())?.ok_or_else(denied)?;
    let verifier: LiveSandboxPrincipalVerifier = decode_json(&payload, "live principal verifier")?;
    if verifier.principal() != grant.subject()
        || Some(verifier.harness()) != grant.scope().harness_contract_id()
    {
        return Err(denied());
    }
    Ok(verifier)
}

pub(super) fn check_dispatch_lane(
    tx: &Transaction<'_>,
    window: &AuthorizedExecutionWindowId,
    attempt: &AuthorizedExecutionAttemptId,
    live: bool,
) -> Result<(), WorkflowOsError> {
    let marked = window.as_str().starts_with(LIVE_WINDOW_PREFIX);
    match version(tx)? {
        ADAPTER_SCHEMA_VERSION if !live && !marked => Ok(()),
        LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION => {
            validate_live_authority_schema(tx)?;
            let granted: bool = tx
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM live_sandbox_grants WHERE attempt_id=?1)",
                    [attempt.as_str()],
                    |row| row.get(0),
                )
                .map_err(|_| denied())?;
            if (live && marked && granted) || (!live && !marked && !granted) {
                Ok(())
            } else {
                Err(denied())
            }
        }
        _ => Err(denied()),
    }
}

const LIVE_OPERATION_PREFIX: &str = "opening/live-comment/";
const LIVE_WINDOW_PREFIX: &str = "window/opening/live-comment/";

pub(super) fn live_opening_operation(action: &SpecContentHash) -> String {
    format!("{LIVE_OPERATION_PREFIX}{action}")
}

pub(super) fn check_opening_lane(
    tx: &Transaction<'_>,
    request: &OperationalExecutionWindowOpeningRequest<'_>,
    live: Option<LiveSandboxContinuationFacts<'_>>,
) -> Result<(), WorkflowOsError> {
    let Some(facts) = live else {
        if request
            .operation_id
            .as_str()
            .starts_with(LIVE_OPERATION_PREFIX)
            || request.window_id.as_str().starts_with(LIVE_WINDOW_PREFIX)
        {
            return Err(denied());
        }
        return Ok(());
    };
    validate_live_store(tx)?;
    let action = facts.action.commitment()?;
    let operation = live_opening_operation(&action);
    if request.operation_id.as_str() != operation
        || request.window_id.as_str() != format!("window/{operation}")
        || request.attempt_id.as_str() != format!("attempt/{operation}")
        || request.receipt_id.as_str() != format!("receipt/{operation}")
        || request.maximum_attempts != 1
        || request.workflow_lifecycle_commitment.is_some()
        || request.run_id != *facts.action.run
        || request.step_id != *facts.action.step
        || request.workflow_id != *facts.scope.workflow_id()
        || request.subject_actor_id != *facts.action.original_actor
        || request.immutable_run_bundle.root_hash() != facts.action.bundle_root
        || request.invocation_binding_commitment != action
        || request.window_id != *facts.window
        || request.attempt_id != *facts.attempt
        || request.expires_at != facts.window_expires_at
    {
        return Err(denied());
    }
    Ok(())
}

pub(super) fn verify_live_opening_approval(
    tx: &Transaction<'_>,
    facts: LiveSandboxContinuationFacts<'_>,
    run: &WorkflowRun,
) -> Result<(), WorkflowOsError> {
    let observation = super::continuity_store::observe_continuity_trusted_time()?;
    let now = observation.observed_at();
    facts.verify_exact_approval(now)?;
    if now >= facts.window_expires_at
        || facts.window_expires_at.as_offset_date_time() - now.as_offset_date_time()
            > time::Duration::minutes(5)
        || facts
            .approval_expires_at
            .is_some_and(|expiry| facts.window_expires_at > expiry)
    {
        return Err(denied());
    }
    verify_persisted_approval(tx, facts, run)
}

fn verify_opening(
    tx: &Connection,
    facts: LiveSandboxContinuationFacts<'_>,
    action: &SpecContentHash,
) -> Result<(), WorkflowOsError> {
    let events = SqliteStateBackend::read_events_with_connection(tx, facts.action.run)?;
    let last = events.last().ok_or_else(denied)?;
    let expected = opening_binding(facts, action);
    if !matches!(&last.kind, WorkflowRunEventKind::OperationalExecutionWindowOpened(open)
        if open.attempt_id() == facts.attempt && open.window_id() == facts.window
            && open.operation_id().as_str() == live_opening_operation(action)
            && open.operation_binding_commitment() == &expected)
    {
        return Err(denied());
    }
    let run = WorkflowRun::rehydrate(&events)?;
    verify_persisted_approval(tx, facts, &run)
}

fn verify_persisted_approval(
    tx: &Connection,
    facts: LiveSandboxContinuationFacts<'_>,
    run: &WorkflowRun,
) -> Result<(), WorkflowOsError> {
    let Some(approval) = run
        .snapshot
        .approval_requests
        .iter()
        .find(|request| request.approval_id == facts.approval.as_str())
    else {
        return Err(denied());
    };
    let presentation: Option<ApprovalPresentationRecord> = super::read_optional_payload(
        tx,
        "approval_presentations",
        "presentation_id",
        facts.presentation.presentation_id().as_str(),
        "live sandbox approval presentation",
    )?;
    if run.snapshot.status != WorkflowRunStatus::Running
        || approval.step_id.as_ref() != Some(facts.action.step)
        || approval.expires_at != facts.approval_expires_at
        || approval.decision.as_ref() != Some(facts.decision)
        || presentation.as_ref() != Some(facts.presentation)
        || !run.events.iter().any(|event| {
            &event.event_id == facts.approval_event
                && matches!(&event.kind, WorkflowRunEventKind::ApprovalGranted(decision)
                if decision == facts.decision)
        })
    {
        return Err(denied());
    }
    Ok(())
}

fn opening_binding(
    facts: LiveSandboxContinuationFacts<'_>,
    action: &SpecContentHash,
) -> SpecContentHash {
    operation_binding_commitment(
        facts.scope.workflow_id(),
        facts.action.run,
        facts.action.step,
        action,
    )
}

fn validate_live_store(connection: &Connection) -> Result<(), WorkflowOsError> {
    if version(connection)? != LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION {
        return Err(denied());
    }
    validate_live_authority_schema(connection)
}

fn version(connection: &Connection) -> Result<u32, WorkflowOsError> {
    connection
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .map_err(|_| denied())
}

fn denied() -> WorkflowOsError {
    WorkflowOsError::security(
        "live_sandbox_continuation.storage_denied",
        "live sandbox grant or dispatch admission is unavailable",
    )
}
