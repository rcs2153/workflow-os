use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::{
    decode_json, encode_json, schema_recovery_required, validate_live_authority_schema,
    validate_previous_live_authority_schema, validate_schema_metadata, SqliteStateBackend,
    ADAPTER_SCHEMA_VERSION, LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION,
    LIVE_AUTHORITY_PREVIOUS_SCHEMA_VERSION, LIVE_AUTHORITY_SCHEMA_CHECKSUM,
    LIVE_SANDBOX_AUTHORITY_SCHEMA_V11, LIVE_SANDBOX_CHECK_PROOF_SCHEMA_V12,
};
use crate::authorized_execution_continuity_state::internal::{
    AuthoritativeAttemptState, AuthoritativeWindowState,
};
use crate::dispatch_reservation::DispatchReservationRequest;
use crate::live_sandbox_continuation_authority::{
    LiveSandboxContinuationFacts, LiveSandboxPrincipalVerifier, ResolvedLiveEffect, LIVE_CAPABILITY,
};
use crate::local_check_evidence_assurance::{
    StoredLocalCheckAssurance, StoredLocalCheckAssuranceWrite,
};
use crate::operational_execution_window_opening::{
    operation_binding_commitment, OperationalExecutionWindowOpeningRequest,
};
use crate::{
    Action, ActorId, ApprovalPresentationRecord, AuthorizedExecutionAttemptId,
    AuthorizedExecutionWindowId, Capability, CapabilityGrant, CapabilityGrantId,
    CapabilityGrantLifecycle, LocalCheckResultStatus, SideEffectAuthorityDecision,
    SideEffectCapability, SideEffectLifecycleState, SideEffectRecord, SideEffectReferenceKind,
    SideEffectSensitivity, SideEffectTargetKind, SpecContentHash, Timestamp, WorkReportSensitivity,
    WorkflowOsError, WorkflowRun, WorkflowRunEventKind, WorkflowRunStatus,
};

impl SqliteStateBackend {
    #[cfg(feature = "trusted-host-application-spi")]
    pub(crate) fn live_check_proof_store_enabled(&self) -> Result<bool, WorkflowOsError> {
        Ok(
            super::validate_operational_schema(&self.existing_connection()?)?
                == LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION,
        )
    }

    // Only a Core-owned producer may supply this projection. Do not import
    // caller payloads or local filesystem assurance records into this lane.
    #[cfg(test)]
    pub(crate) fn store_live_check_proof(
        &self,
        projection: &crate::LocalCheckEvidenceAssuranceProjection,
    ) -> Result<(), WorkflowOsError> {
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| required_check_unverified())?;
        validate_live_store(&tx)?;
        let events = Self::read_events_with_connection(&tx, projection.run_id())?;
        let mut matching = events.iter().filter_map(|event| match &event.kind {
            WorkflowRunEventKind::LocalCheckResultRecorded(result)
                if result.result_id() == projection.result_id() =>
            {
                Some((event, result))
            }
            _ => None,
        });
        let (event, result) = matching.next().ok_or_else(required_check_unverified)?;
        if matching.next().is_some() {
            return Err(required_check_unverified());
        }
        persist_live_check_proof(&tx, event, result, projection)?;
        tx.commit().map_err(|_| required_check_unverified())
    }

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
        let upgrading = match version(&tx)? {
            ADAPTER_SCHEMA_VERSION => {
                validate_schema_metadata(&tx)?;
                tx.execute_batch(LIVE_SANDBOX_AUTHORITY_SCHEMA_V11)
                    .map_err(|_| schema_recovery_required())?;
                true
            }
            LIVE_AUTHORITY_PREVIOUS_SCHEMA_VERSION => {
                validate_previous_live_authority_schema(&tx)?;
                true
            }
            LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION => false,
            _ => return Err(schema_recovery_required()),
        };
        if upgrading {
            tx.execute_batch(LIVE_SANDBOX_CHECK_PROOF_SCHEMA_V12)
                .and_then(|()| {
                    tx.execute(
                        "UPDATE schema_metadata SET schema_version=?1, checksum=?2
                         WHERE singleton=1",
                        params![
                            LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION,
                            LIVE_AUTHORITY_SCHEMA_CHECKSUM
                        ],
                    )?;
                    tx.pragma_update(None, "user_version", LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION)
                })
                .map_err(|_| schema_recovery_required())?;
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
        let effect = verify_opening(
            &tx,
            facts,
            &facts.action.commitment()?,
            observation.observed_at(),
        )?;
        let registration = active_principal(&tx, grant)?.commitment()?;
        let binding =
            facts.grant_commitment(grant, observation.observed_at(), &registration, &effect)?;
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

pub(super) fn persist_live_check_proof(
    tx: &Connection,
    event: &crate::WorkflowRunEvent,
    result: &crate::LocalCheckResultRecordedEvent,
    projection: &crate::LocalCheckEvidenceAssuranceProjection,
) -> Result<(), WorkflowOsError> {
    validate_live_store(tx)?;
    exact_check_proof(projection, event, result)?;
    let payload = encode_check_proof(projection)?;
    let event_hash = check_event_commitment(event)?;
    let inserted = tx.execute(
        "INSERT INTO live_sandbox_check_proofs (run_id, result_id, event_id, event_commitment, payload)
         VALUES (?1, ?2, ?3, ?4, ?5) ON CONFLICT(run_id, result_id) DO NOTHING",
        params![event.run_id.as_str(), result.result_id().as_str(), event.event_id.as_str(),
            event_hash.as_str(), payload],
    ).map_err(|_| required_check_unverified())?;
    if inserted == 0 && read_live_check_proof(tx, event, result)? != payload {
        return Err(required_check_unverified());
    }
    Ok(())
}

pub(super) fn verify_live_check_proof_replay(
    tx: &Connection,
    event: &crate::WorkflowRunEvent,
    result: &crate::LocalCheckResultRecordedEvent,
    projection: &crate::LocalCheckEvidenceAssuranceProjection,
) -> Result<(), WorkflowOsError> {
    validate_live_store(tx)?;
    exact_check_proof(projection, event, result)?;
    if read_live_check_proof(tx, event, result)? != encode_check_proof(projection)? {
        return Err(required_check_unverified());
    }
    Ok(())
}

fn encode_check_proof(
    projection: &crate::LocalCheckEvidenceAssuranceProjection,
) -> Result<String, WorkflowOsError> {
    encode_json(
        &StoredLocalCheckAssuranceWrite::new(projection)?,
        "live check proof",
    )
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
        let effect = verify_opening(tx, facts, &action, now)?;
        facts.verify_bound_grant(
            &self.grant,
            now,
            &self.registration,
            &effect,
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
        version @ (LIVE_AUTHORITY_ADAPTER_SCHEMA_VERSION
        | LIVE_AUTHORITY_PREVIOUS_SCHEMA_VERSION) => {
            if version == LIVE_AUTHORITY_PREVIOUS_SCHEMA_VERSION {
                validate_previous_live_authority_schema(tx)?;
                if live {
                    return Err(denied());
                }
            } else {
                validate_live_authority_schema(tx)?;
            }
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
    now: Timestamp,
) -> Result<ResolvedLiveEffect, WorkflowOsError> {
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
    verify_persisted_approval(tx, facts, &run)?;
    resolve_live_effect(tx, facts, &run, &last.event_id, now)
}

fn resolve_live_effect(
    tx: &Connection,
    facts: LiveSandboxContinuationFacts<'_>,
    run: &WorkflowRun,
    opening_event: &crate::EventId,
    now: Timestamp,
) -> Result<ResolvedLiveEffect, WorkflowOsError> {
    let record: SideEffectRecord = super::read_optional_payload(
        tx,
        "side_effect_records",
        "side_effect_id",
        facts.action.side_effect.as_str(),
        "live sandbox attempted effect",
    )?
    .ok_or_else(denied)?;
    let identity = &run.snapshot.identity;
    let bundle = identity.immutable_run_bundle.as_ref().ok_or_else(denied)?;
    let stored = verify_live_bundle(tx, facts, run, bundle)?;
    let [approval] = record.authority().approval_references.as_slice() else {
        return Err(denied());
    };
    if record.side_effect_id() != facts.action.side_effect
        || record.lifecycle_state() != SideEffectLifecycleState::Attempted
        || record.outcome_reference().is_some()
        || record.capability() != SideEffectCapability::GitHubWrite
        || record.authority().decision != SideEffectAuthorityDecision::ApprovedByHuman
        || record.run_id() != &identity.run_id
        || record.workflow_id() != &identity.workflow_id
        || record.workflow_version() != &identity.workflow_version
        || record.schema_version() != &identity.schema_version
        || record.spec_hash() != &identity.spec_content_hash
        || bundle.root_hash() != facts.action.bundle_root
        || record.step_id() != Some(facts.action.step)
        || record.actor() != Some(facts.action.original_actor)
        || record.target().kind() != SideEffectTargetKind::AdapterResource
        || record.target().reference() != facts.action.target.reference()
        || record.idempotency().key() != facts.action.idempotency_key
        || approval.kind() != SideEffectReferenceKind::ApprovalDecision
        || approval.reference() != facts.approval.as_str()
        || facts.requirements.approval_ids() != [facts.approval.clone()]
        || !facts.requirements.policy_ids().is_empty()
        || !facts.requirements.evidence_ids().is_empty()
        || !record.references().iter().any(|reference| {
            reference.kind() == SideEffectReferenceKind::WorkflowEvent
                && reference.reference() == opening_event.as_str()
        })
    {
        return Err(denied());
    }
    let policy = resolve_effect_policy(run, &record)?;
    let checks = resolve_effect_checks(run, &record, facts, bundle)?;
    let proofs = resolve_required_check_proofs(tx, &stored, facts.action.step, &checks, now)?;
    let bundle_sensitivity = match stored.manifest().sensitivity() {
        crate::ImmutableRunBundleSensitivity::Internal => WorkReportSensitivity::Internal,
        crate::ImmutableRunBundleSensitivity::Confidential => WorkReportSensitivity::Confidential,
        crate::ImmutableRunBundleSensitivity::Restricted => WorkReportSensitivity::Secret,
    };
    let sensitivity = match record.sensitivity() {
        SideEffectSensitivity::Public => WorkReportSensitivity::Public,
        SideEffectSensitivity::Internal => WorkReportSensitivity::Internal,
        SideEffectSensitivity::Confidential => WorkReportSensitivity::Confidential,
        SideEffectSensitivity::Regulated => WorkReportSensitivity::Regulated,
        SideEffectSensitivity::Secret => WorkReportSensitivity::Secret,
        SideEffectSensitivity::Unknown => return Err(denied()),
    };
    Ok(ResolvedLiveEffect {
        evidence: if proofs.is_empty() {
            proof_commitment(b"workflow-os/live-effect-evidence/v1\0", &(record, checks))?
        } else {
            proof_commitment(
                b"workflow-os/live-effect-evidence/v2\0",
                &(record, checks, proofs),
            )?
        },
        policy: proof_commitment(b"workflow-os/live-effect-policy/v1\0", &policy)?,
        sensitivity: sensitivity.max(bundle_sensitivity),
    })
}

fn verify_live_bundle(
    tx: &Connection,
    facts: LiveSandboxContinuationFacts<'_>,
    run: &WorkflowRun,
    binding: &crate::ImmutableRunBundleBinding,
) -> Result<crate::StoredImmutableRunBundle, WorkflowOsError> {
    let identity = &run.snapshot.identity;
    let stored = SqliteStateBackend::read_exact_bundle_with_connection(
        tx,
        &identity.run_id,
        binding.bundle_id(),
    )
    .map_err(|_| denied())?;
    let manifest = stored.manifest();
    let approval = run
        .snapshot
        .approval_requests
        .iter()
        .find(|approval| approval.approval_id == facts.approval.as_str())
        .ok_or_else(denied)?;
    if manifest.run_binding() != *binding
        || manifest.workflow_id() != &identity.workflow_id
        || manifest.workflow_version() != &identity.workflow_version
        || manifest.schema_version() != &identity.schema_version
        || manifest.workflow_content_hash() != &identity.spec_content_hash
        || approval.resolved_execution_context_hash.as_ref()
            != Some(manifest.resolved_execution_context_hash())
    {
        return Err(denied());
    }

    Ok(stored)
}

fn resolve_required_check_proofs(
    tx: &Connection,
    stored: &crate::StoredImmutableRunBundle,
    selected_step: &crate::StepId,
    checks: &[(&crate::WorkflowRunEvent, &crate::WorkflowRunEvent)],
    now: Timestamp,
) -> Result<Vec<SpecContentHash>, WorkflowOsError> {
    let workflow =
        crate::executor::immutable_continuation_workflow(stored).map_err(|_| denied())?;
    let selected = workflow
        .steps
        .iter()
        .position(|step| &step.id == selected_step)
        .ok_or_else(denied)?;
    let mut commitments = Vec::new();
    for step in &workflow.steps[..=selected] {
        let record =
            crate::local_check_attestation::authoritative_record_for_step(stored, &step.id)
                .map_err(|_| denied())?;
        if step.local_check_requirements.iter().any(|requirement| {
            requirement.requirement_level() == crate::LocalCheckRequirementLevel::Required
                && !record
                    .declarations()
                    .iter()
                    .any(|declaration| declaration_matches_requirement(declaration, requirement))
        }) {
            return Err(required_check_unverified());
        }
        for declaration in record.declarations().iter().filter(|declaration| {
            declaration.requirement_level() == crate::LocalCheckRequirementLevel::Required
        }) {
            let mut matching = checks.iter().filter_map(|(event, _)| {
                let WorkflowRunEventKind::LocalCheckResultRecorded(result) = &event.kind else {
                    return None;
                };
                (result.step_id() == &step.id && result.command_id() == declaration.command_id())
                    .then_some((*event, result))
            });
            let (event, result) = matching.next().ok_or_else(required_check_unverified)?;
            if matching.next().is_some() {
                return Err(required_check_unverified());
            }
            let payload = read_live_check_proof(tx, event, result)?;
            let StoredLocalCheckAssurance(projection) = decode_json(&payload, "live check proof")
                .map_err(|_| required_check_unverified())?;
            validate_check_proof(&projection, event, result, declaration, now)?;
            commitments.push(proof_commitment(
                b"workflow-os/live-check-proof/v1\0",
                &payload,
            )?);
        }
    }
    Ok(commitments)
}

fn declaration_matches_requirement(
    declaration: &crate::CanonicalLocalCheckDeclaration,
    requirement: &crate::LocalCheckRequirementDeclaration,
) -> bool {
    declaration.requirement_level() == requirement.requirement_level()
        && declaration.requirement_id() == requirement.id()
        && declaration.command_id() == requirement.command_id()
        && declaration.minimum_assurance() == requirement.minimum_assurance()
        && declaration.evidence_assurance_minimum() == requirement.evidence_assurance_minimum()
        && declaration.accepted_statuses() == requirement.accepted_statuses()
        && declaration.freshness() == requirement.freshness()
        && declaration.exact_immutable_run_binding_required()
            == requirement.exact_immutable_run_binding_required()
        && declaration.truncation_allowed() == requirement.truncation_allowed()
        && declaration.network_maximum() == requirement.network_maximum()
        && declaration.side_effect_maximum() == requirement.side_effect_maximum()
}

fn exact_check_proof<'a>(
    projection: &'a crate::LocalCheckEvidenceAssuranceProjection,
    event: &crate::WorkflowRunEvent,
    result: &crate::LocalCheckResultRecordedEvent,
) -> Result<&'a crate::AcceptedLocalCheckAttestation, WorkflowOsError> {
    let proof = projection
        .verifier_proof()
        .ok_or_else(required_check_unverified)?;
    if projection.run_id() != &event.run_id
        || projection.workflow_id() != &event.workflow_id
        || projection.immutable_run_bundle() != result.immutable_run_bundle()
        || projection.step_id() != result.step_id()
        || projection.command_id() != result.command_id()
        || projection.command_kind() != result.command_kind()
        || projection.result_id() != result.result_id()
        || projection.result_status() != result.status()
        || proof.invocation_id() != result.invocation_id()
        || proof.verified_at() > &event.timestamp
    {
        return Err(required_check_unverified());
    }
    Ok(proof)
}

fn validate_check_proof(
    projection: &crate::LocalCheckEvidenceAssuranceProjection,
    event: &crate::WorkflowRunEvent,
    result: &crate::LocalCheckResultRecordedEvent,
    declaration: &crate::CanonicalLocalCheckDeclaration,
    now: Timestamp,
) -> Result<(), WorkflowOsError> {
    let proof = exact_check_proof(projection, event, result)?;
    let requirement = declaration
        .attestation_requirement()
        .map_err(|_| required_check_unverified())?;
    let seconds = requirement
        .freshness()
        .maximum_age_seconds()
        .ok_or_else(required_check_unverified)?;
    if proof.requirement_fingerprint() != requirement.requirement_fingerprint()
        || proof.command_contract_fingerprint() != Some(declaration.command_contract_fingerprint())
        || proof.assurance() != requirement.minimum_assurance()
        || proof.freshness() != requirement.freshness()
        || proof.result_status() != LocalCheckResultStatus::Passed
        || !requirement
            .accepted_statuses()
            .contains(&proof.result_status())
        || (!requirement.truncation_allowed()
            && (proof.stdout_truncated() || proof.stderr_truncated()))
        || declaration
            .evidence_assurance_minimum()
            .is_some_and(|minimum| !projection.satisfies_minimum(minimum))
        || now < event.timestamp
        || now.as_offset_date_time() - proof.observed_completed_at().as_offset_date_time()
            > time::Duration::seconds(i64::from(seconds))
    {
        return Err(required_check_unverified());
    }
    Ok(())
}

fn check_event_commitment(
    event: &crate::WorkflowRunEvent,
) -> Result<SpecContentHash, WorkflowOsError> {
    proof_commitment(b"workflow-os/live-check-result-event/v1\0", event)
}

fn read_live_check_proof(
    tx: &Connection,
    event: &crate::WorkflowRunEvent,
    result: &crate::LocalCheckResultRecordedEvent,
) -> Result<String, WorkflowOsError> {
    let event_hash = check_event_commitment(event)?;
    tx.query_row(
        "SELECT payload FROM live_sandbox_check_proofs
         WHERE run_id=?1 AND result_id=?2 AND event_id=?3 AND event_commitment=?4",
        params![
            event.run_id.as_str(),
            result.result_id().as_str(),
            event.event_id.as_str(),
            event_hash.as_str()
        ],
        |row| row.get(0),
    )
    .optional()
    .map_err(|_| required_check_unverified())?
    .ok_or_else(required_check_unverified)
}

fn required_check_unverified() -> WorkflowOsError {
    WorkflowOsError::security(
        "live_sandbox_continuation.required_check_unverified",
        "required immutable check assurance is unavailable for live admission",
    )
}

fn resolve_effect_policy<'a>(
    run: &'a WorkflowRun,
    record: &SideEffectRecord,
) -> Result<&'a crate::WorkflowRunEvent, WorkflowOsError> {
    let [reference] = record.authority().policy_references.as_slice() else {
        return Err(denied());
    };
    let correlation = record.correlation_id().ok_or_else(denied)?;
    let latest = run
        .events
        .iter()
        .rev()
        .find_map(|event| match &event.kind {
            WorkflowRunEventKind::PolicyDecisionRecorded(decision)
                if decision.action == Action::InvokeAdapter
                    && decision.correlation_id.as_ref() == Some(correlation) =>
            {
                Some((event, decision))
            }
            _ => None,
        })
        .ok_or_else(denied)?;
    if reference.kind() != SideEffectReferenceKind::PolicyDecision
        || reference.reference() != latest.0.event_id.as_str()
        || !latest.1.allowed
        || !latest.1.requires_approval
        || !latest.1.violations.is_empty()
        || latest.1.actor.as_ref() != record.actor()
        || latest.1.workflow_id.as_ref() != Some(record.workflow_id())
        || latest.1.run_id.as_ref() != Some(record.run_id())
        || !latest.1.capabilities.contains(&Capability::ExternalWrite)
        || latest.1.capabilities.iter().any(|capability| {
            !matches!(
                capability,
                Capability::ExternalWrite | Capability::AdapterInvoke
            )
        })
    {
        return Err(denied());
    }
    Ok(latest.0)
}

fn resolve_effect_checks<'a>(
    run: &'a WorkflowRun,
    record: &SideEffectRecord,
    facts: LiveSandboxContinuationFacts<'_>,
    bundle: &crate::ImmutableRunBundleBinding,
) -> Result<Vec<(&'a crate::WorkflowRunEvent, &'a crate::WorkflowRunEvent)>, WorkflowOsError> {
    let mut references = record
        .references()
        .iter()
        .filter(|reference| reference.kind() == SideEffectReferenceKind::LocalCheckResult)
        .map(crate::SideEffectReference::reference)
        .collect::<Vec<_>>();
    let mut required = facts
        .requirements
        .check_ids()
        .iter()
        .map(crate::LocalCheckResultId::as_str)
        .collect::<Vec<_>>();
    references.sort_unstable();
    required.sort_unstable();
    if references.is_empty() || references != required {
        return Err(denied());
    }
    references.into_iter().map(|reference| {
        let mut matching = run.events.iter().filter_map(|event| match &event.kind {
            WorkflowRunEventKind::LocalCheckResultRecorded(result) if result.result_id().as_str() == reference
                => Some((event, result)),
            _ => None,
        });
        let (event, result) = matching.next().ok_or_else(denied)?;
        let terminal = run.events.iter().find(|terminal| {
            terminal.event_id == *result.invocation_terminal_event_id()
                && terminal.sequence_number > event.sequence_number
                && matches!(&terminal.kind, WorkflowRunEventKind::SkillInvocationSucceeded {
                    invocation_id, step_id, ..
                } if invocation_id == result.invocation_id() && step_id == result.step_id())
        }).ok_or_else(denied)?;
        if matching.next().is_some()
            || result.status() != LocalCheckResultStatus::Passed
            || result.immutable_run_bundle() != bundle
            || run.events.iter().any(|later| later.sequence_number > event.sequence_number
                && matches!(&later.kind, WorkflowRunEventKind::LocalCheckResultRecorded(next)
                    if next.step_id() == result.step_id() && next.command_id() == result.command_id()))
        {
            return Err(denied());
        }
        Ok((event, terminal))
    }).collect()
}

fn proof_commitment(
    domain: &[u8],
    value: &impl Serialize,
) -> Result<SpecContentHash, WorkflowOsError> {
    let mut hash = Sha256::new();
    hash.update(domain);
    hash.update(serde_json::to_vec(value).map_err(|_| denied())?);
    SpecContentHash::new(format!("{:x}", hash.finalize()))
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
