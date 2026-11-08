use rusqlite::{params, OptionalExtension, TransactionBehavior};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[cfg(test)]
use std::cell::Cell;

use crate::authorized_execution_continuity_state::internal::{
    trusted_time_commitment, window_binding_commitment, AuthoritativeAttemptState,
    AuthoritativeWindowState, ContinuityInstanceEligibility, ContinuityRevision,
    ReferenceContinuityState, TrustedTimeObservation, TrustedTimePosture,
};
use crate::dispatch_reservation::{
    admitted_capability, dispatch_admission_event_id, request_commitment, reservation_commitment,
    reservation_error, DispatchAdmissionProjectionCursor, DispatchAdmissionProjectionEvent,
    DispatchAdmissionProjectionEventDefinition, DispatchReservationBinding,
    DispatchReservationOperationId, DispatchReservationOutcome, DispatchReservationRecord,
    DispatchReservationReplayReceipt, DispatchReservationRequest, DispatchReservationStore,
    LiveSandboxReservationWitness, ReservedAttemptDispatchCapability,
};
use crate::live_sandbox_continuation_authority::LiveSandboxProviderBinding;
use crate::provider_comment_observation::{
    can_encode_marked_comment, encode_marked_comment, verify_created_comment_response,
    verify_current_comment, CommentEnumeration, CreatedCommentResponse, CurrentCommentPosture,
    ExpectedComment, MARKER_NAMESPACE,
};
use crate::provider_comment_transport::InstallationObserverProfile;

mod observation_journal;

const fn installation_preparation_schema(version: u32) -> bool {
    matches!(
        version,
        super::INSTALLATION_SEND_PREPARATION_SCHEMA_VERSION
            | super::INSTALLATION_OBSERVATION_JOURNAL_SCHEMA_VERSION
    )
}
use crate::provider_comment_transport::installation_exchange::{
    installation_comment_adapter_commitment, installation_target_revision,
    InstallationCommentTransport, InstallationCreationContext, ReviewedInstallation,
    VerifiedInstallationCreation, VerifiedInstallationScope,
};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use crate::provider_comment_transport::installation_signer_channel::{
    InstallationAppAssertion, InstallationSignerChannel, InstallationSignerRequest,
    ReceivedInstallationSignerRequest,
};
#[cfg(test)]
use crate::provider_comment_transport::PreparedGitHubCommentCreation;
use crate::provider_comment_transport::{
    fixed_comment_exchange_commitment, fixed_comment_observer_commitment, CommentSendWindow,
    StoredGitHubCommentObservation, VerifiedGitHubCommentObservation,
};
use crate::provider_write::GitHubCommentContinuationAction;
use crate::{
    transition_side_effect_to_completed, AuthorizedExecutionAttemptId, AuthorizedExecutionWindowId,
    EventId, EventSequenceNumber, GitHubPullRequestCommentProviderLookupObservation,
    GitHubPullRequestCommentTarget, IdempotencyKey, SideEffectCompleteTransitionInput,
    SideEffectLifecycleState, SideEffectOutcomeReference, SideEffectOutcomeReferenceKind,
    SideEffectRecord, SideEffectReference, SideEffectReferenceKind, SpecContentHash, Timestamp,
    WorkflowOsError, WorkflowOsErrorKind, WorkflowRun, WorkflowRunEvent, WorkflowRunEventKind,
    WorkflowRunSnapshot,
};

use super::continuity_codec::timestamp_parts;
use super::live_sandbox_grant_store::{
    check_dispatch_lane, read_active_live_target, read_live_effect_record, LiveGrantAdmission,
};
use super::{
    append_event_and_project_snapshot, encode_json, snapshot_commitment, SqliteStateBackend,
};
use super::{continuity_codec, continuity_store};

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
struct ProjectionBinding {
    event: DispatchAdmissionProjectionEvent,
    snapshot_commitment: crate::SpecContentHash,
}

struct LiveRecoveryAdmission {
    operation: DispatchReservationOperationId,
    event: DispatchAdmissionProjectionEvent,
    witness: Option<LiveSandboxReservationWitness>,
    committed_at: Timestamp,
}

struct CheckedCommentSend {
    durable: LiveRecoveryAdmission,
    window: CommentSendWindow,
}

#[derive(Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct InstallationSendPreparationRecord {
    operation: String,
    reservation: SpecContentHash,
    invocation: SpecContentHash,
    scope: SpecContentHash,
    marker_commitment: SpecContentHash,
    body_commitment: SpecContentHash,
    clock: TrustedTimeObservation,
    deadline: Timestamp,
}

#[derive(Eq, PartialEq)]
struct InstallationInventoryContext {
    record: InstallationSendPreparationRecord,
    provider: LiveSandboxProviderBinding,
    observer_profile: Option<InstallationObserverProfile>,
}

type CollectedInstallationInventory = (
    InstallationInventoryContext,
    crate::provider_comment_transport::VerifiedInventoryTraversal,
    std::num::NonZeroU64,
);

fn installation_inventory_context(
    tx: &rusqlite::Transaction<'_>,
    operation: &DispatchReservationOperationId,
    action: GitHubCommentContinuationAction<'_>,
    historical: bool,
) -> Result<Option<InstallationInventoryContext>, WorkflowOsError> {
    if !installation_preparation_schema(super::validate_operational_schema(tx)?) {
        return Ok(None);
    }
    let admission = read_live_recovery_admission(tx, operation, action.run)?;
    match verify_comment_disposition(tx, &admission, action.run, action.side_effect)? {
        None => {}
        Some(
            LiveCommentProofKind::InstallationCreation
            | LiveCommentProofKind::InstallationObservation,
        ) if historical => {}
        Some(_) => return Err(not_dispatchable()),
    }
    let witness = admission.witness.as_ref().ok_or_else(corrupt)?;
    let record = read_installation_send_preparation_record(tx, operation, action, &admission)?;
    Ok(record.map(|record| InstallationInventoryContext {
        record,
        provider: witness.provider.clone(),
        observer_profile: witness.installation_observer_profile.clone(),
    }))
}

fn installation_send_preparation_payload(
    record: &InstallationSendPreparationRecord,
) -> Result<(String, SpecContentHash), WorkflowOsError> {
    let payload = encode_json(record, "installation send preparation")?;
    let hash = SpecContentHash::from_text(&format!(
        "workflow-os/installation-send-preparation/v1\0{payload}"
    ));
    Ok((payload, hash))
}

fn read_installation_send_preparation_record(
    tx: &rusqlite::Transaction<'_>,
    operation: &DispatchReservationOperationId,
    action: GitHubCommentContinuationAction<'_>,
    durable: &LiveRecoveryAdmission,
) -> Result<Option<InstallationSendPreparationRecord>, WorkflowOsError> {
    if durable.event.invocation_commitment() != &action.commitment()? {
        return Err(not_dispatchable());
    }
    read_bound_installation_preparation(tx, operation, durable)
}

fn read_bound_installation_preparation(
    tx: &rusqlite::Transaction<'_>,
    operation: &DispatchReservationOperationId,
    durable: &LiveRecoveryAdmission,
) -> Result<Option<InstallationSendPreparationRecord>, WorkflowOsError> {
    let row: Option<(String, String)> = tx.query_row(
        "SELECT payload, payload_hash FROM installation_send_preparations WHERE operation_id=?1",
        [operation.as_str()], |row| Ok((row.get(0)?, row.get(1)?)),
    ).optional().map_err(|_| corrupt())?;
    row.map(|(payload, hash)| {
        let record: InstallationSendPreparationRecord =
            super::decode_json(&payload, "installation send preparation")?;
        let (canonical, commitment) = installation_send_preparation_payload(&record)?;
        let witness = durable.witness.as_ref().ok_or_else(corrupt)?;
        if payload != canonical
            || hash != commitment.as_str()
            || record.operation != operation.as_str()
            || &record.reservation != durable.event.reservation_commitment()
            || &record.invocation != durable.event.invocation_commitment()
            || witness.installation_scope_commitment.as_ref() != Some(&record.scope)
            || witness.installation_issuance_claim_commitment.is_none()
            || witness.installation_send_preparation_version != Some(1)
            || witness.creation_deadline != Some(record.deadline)
            || record.clock.observed_at() < durable.committed_at
            || record.clock.observed_at() >= record.deadline
            || !is_installation_comment_adapter(&witness.provider.adapter_commitment, false)
        {
            return Err(corrupt());
        }
        Ok(record)
    })
    .transpose()
}

pub(crate) struct InstallationCommentSendPermit<'a> {
    context: InstallationCreationContext<'a>,
    window: CommentSendWindow,
}

impl<'a> InstallationCommentSendPermit<'a> {
    pub(crate) fn into_parts(self) -> (InstallationCreationContext<'a>, CommentSendWindow) {
        (self.context, self.window)
    }
}

struct LiveInstallationScopeAdmission<'a> {
    reviewed: ReviewedInstallation<'a>,
    verified: &'a VerifiedInstallationScope,
    fixture: bool,
    claim: Option<&'a InstallationIssuanceClaim>,
}

impl LiveInstallationScopeAdmission<'_> {
    fn validate(
        &self,
        tx: &rusqlite::Transaction<'_>,
        request: &DispatchReservationRequest,
        effect: &crate::SideEffectRecord,
        provider: &LiveSandboxProviderBinding,
        now: Timestamp,
        deadline: Timestamp,
    ) -> Result<(SpecContentHash, Timestamp), WorkflowOsError> {
        verify_reviewed_installation(
            tx,
            request,
            effect,
            provider,
            deadline,
            self.reviewed,
            self.fixture,
        )?;
        if !self.fixture {
            let claim = self.claim.ok_or_else(not_dispatchable)?;
            if *claim.record != read_installation_claim(tx, request)?.ok_or_else(not_dispatchable)?
                || !claim.record.matches(request, effect.side_effect_id(), self.reviewed)
                || now < claim.record.claimed_at || now >= claim.record.deadline
                || self.verified.verified_at() < claim.record.claimed_at
                || !tx.query_row(
                    "SELECT EXISTS(SELECT 1 FROM live_sandbox_grant_uses WHERE attempt_id=?1 AND grant_id=?2)",
                    params![claim.record.attempt, claim.record.grant], |row| row.get::<_, bool>(0),
                ).map_err(|_| corrupt())?
            {
                return Err(not_dispatchable());
            }
        }
        self.verified
            .admission_binding(self.reviewed, now, self.fixture)
            .filter(|(_, expires)| now < *expires && *expires <= deadline)
            .ok_or_else(not_dispatchable)
    }
}

fn verify_reviewed_installation(
    tx: &rusqlite::Transaction<'_>,
    request: &DispatchReservationRequest,
    effect: &SideEffectRecord,
    provider: &LiveSandboxProviderBinding,
    deadline: Timestamp,
    scope: ReviewedInstallation<'_>,
    fixture: bool,
) -> Result<(), WorkflowOsError> {
    let registered = read_active_live_target(tx, effect.target().reference())?;
    let binding = &request.expected_window_binding;
    if !is_installation_comment_adapter(&provider.adapter_commitment, fixture)
        || scope.run_id != &binding.run_id
        || scope.immutable_run != binding.immutable_run_bundle.root_hash()
        || scope.target != registered.target()
        || scope.repository_id != provider.repository_id
        || scope.bot_id != provider.author_id
        || scope.comment_adapter != &provider.adapter_commitment
        || scope.deadline != deadline
        || scope.observer_profile.is_some_and(|profile| {
            fixture
                || profile.reader == scope.bot_id
                || profile.receiver_installation == scope.installation_id
        })
        || (!fixture
            && installation_target_revision(
                scope.target,
                scope.app_id,
                scope.app_slug,
                scope.installation_id,
                provider,
                scope.observer_profile,
            )
            .as_ref()
                != Some(registered.revision()))
    {
        return Err(not_dispatchable());
    }
    Ok(())
}

#[derive(Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct InstallationClaimRecord {
    attempt: String,
    window: String,
    operation: String,
    run: crate::WorkflowRunId,
    effect: crate::SideEffectId,
    grant: String,
    request: SpecContentHash,
    scope: SpecContentHash,
    claimed_at: Timestamp,
    deadline: Timestamp,
}

impl InstallationClaimRecord {
    fn commitment(&self) -> Result<SpecContentHash, WorkflowOsError> {
        Ok(SpecContentHash::from_text(&encode_json(
            self,
            "installation issuance claim",
        )?))
    }

    fn matches(
        &self,
        request: &DispatchReservationRequest,
        effect: &crate::SideEffectId,
        reviewed: ReviewedInstallation<'_>,
    ) -> bool {
        self.attempt == request.capability.attempt_id.as_str()
            && self.window == request.capability.window_id.as_str()
            && self.operation == request.operation_id.as_str()
            && self.run == request.expected_window_binding.run_id
            && &self.effect == effect
            && self.request == request.request_commitment
            && reviewed.issuance_commitment().as_ref() == Some(&self.scope)
            && self.deadline == reviewed.deadline
            && self.claimed_at < self.deadline
    }
}

// The right is process-local and deliberately cannot be cloned or reconstructed.
pub(super) struct InstallationIssuanceClaim {
    record: Box<InstallationClaimRecord>,
}

pub(super) struct InstallationExchangedScope {
    claim: InstallationIssuanceClaim,
    verified: VerifiedInstallationScope,
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub(crate) struct InstallationSignerAdmission(SignerAdmissionRecord);

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SignerAdmissionRecord {
    claim: SpecContentHash,
    operation: String,
    scope: SpecContentHash,
    deadline: Timestamp,
    admitted_at: Timestamp,
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
impl InstallationSignerAdmission {
    pub(crate) fn admitted_at(
        &self,
        request: &ReceivedInstallationSignerRequest,
    ) -> Option<Timestamp> {
        (self.0.claim == request.claim
            && self.0.operation == request.operation
            && self.0.scope == request.scope
            && self.0.deadline == request.deadline
            && self.0.admitted_at < request.deadline)
            .then_some(self.0.admitted_at)
    }
}

#[allow(dead_code)] // No signer or operator caller is admitted yet.
pub(super) enum InstallationIssuanceDisposition {
    Fresh(InstallationIssuanceClaim),
    IssuanceUncertain,
    Reserved(InstallationCommentRecoveryPosture),
}

fn is_installation_comment_adapter(adapter: &SpecContentHash, fixture: bool) -> bool {
    if fixture {
        #[cfg(test)]
        return adapter
            == &crate::provider_comment_transport::installation_exchange::fixture_installation_comment_adapter_commitment();
        #[cfg(not(test))]
        return false;
    }
    adapter == &installation_comment_adapter_commitment()
}

struct ReservedCommentContext {
    reservation: SpecContentHash,
    provider: LiveSandboxProviderBinding,
    marker: String,
    committed_at: Timestamp,
    deadline: Timestamp,
}

impl ReservedCommentContext {
    fn expected<'a>(&'a self, action: GitHubCommentContinuationAction<'a>) -> ExpectedComment<'a> {
        ExpectedComment {
            target: action.target,
            repository_id: self.provider.repository_id,
            author_id: self.provider.author_id,
            known_comment_id: None,
            body: action.body,
            reservation_marker: &self.marker,
            not_before: self.committed_at,
            not_after: self.deadline,
        }
    }
}

fn live_reservation_commitment(
    witness: &LiveSandboxReservationWitness,
) -> Result<SpecContentHash, WorkflowOsError> {
    let mut hash = Sha256::new();
    hash.update(b"workflow-os/live-comment-reservation/v2\0");
    hash.update(serde_json::to_vec(witness).map_err(|_| corrupt())?);
    SpecContentHash::new(format!("{:x}", hash.finalize()))
}

/// Verified, reference-free execution posture of a synthetic sandbox reservation.
/// Neither variant grants permission to dispatch or retry.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SyntheticSandboxRecoveryPosture {
    /// A matching attempted record exists; no completion has been established.
    ReconciliationRequired,
    /// The completed record and its proof event match the exact admission.
    CompletionVerified,
}

/// The verified provenance of a synthetic completion, not live provider evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SyntheticSandboxCompletionProofSource {
    /// A synthetic lookup reconciled the attempted effect.
    Lookup,
    /// The injected creation response was verified and persisted.
    CreationResponse,
}

/// Redacted inspection of one durable synthetic reservation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SyntheticSandboxRecoveryInspection {
    source: Option<SyntheticSandboxCompletionProofSource>,
}

impl SyntheticSandboxRecoveryInspection {
    /// Returns the existing synthetic execution posture.
    #[must_use]
    pub const fn posture(self) -> SyntheticSandboxRecoveryPosture {
        if self.source.is_some() {
            SyntheticSandboxRecoveryPosture::CompletionVerified
        } else {
            SyntheticSandboxRecoveryPosture::ReconciliationRequired
        }
    }

    /// Returns the verified proof source, or none when completion is unresolved.
    #[must_use]
    pub const fn completion_proof_source(self) -> Option<SyntheticSandboxCompletionProofSource> {
        self.source
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)] // No operator caller is admitted yet.
pub(super) enum InstallationCommentRecoveryPosture {
    IssuanceUncertain,
    ReconciliationRequired,
    CreationVerified,
    ObservationReconciled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum LiveCommentDispatchPosture {
    PersistedSyntheticResponse { comment_id: std::num::NonZeroU64 },
    RecoveryOnly,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum LiveCommentProofKind {
    Lookup,
    CreationResponse,
    InstallationCreation,
    InstallationObservation,
    InstallationFixtureCreation,
}

impl LiveCommentProofKind {
    const fn synthetic(self) -> Option<SyntheticSandboxCompletionProofSource> {
        match self {
            Self::Lookup => Some(SyntheticSandboxCompletionProofSource::Lookup),
            Self::CreationResponse => Some(SyntheticSandboxCompletionProofSource::CreationResponse),
            Self::InstallationCreation
            | Self::InstallationObservation
            | Self::InstallationFixtureCreation => None,
        }
    }

    const fn fixture(self) -> bool {
        match self {
            Self::Lookup
            | Self::CreationResponse
            | Self::InstallationCreation
            | Self::InstallationObservation => false,
            Self::InstallationFixtureCreation => true,
        }
    }
}

// Only a trusted synthetic lookup may supply this result until a provider
// adapter can independently verify author and exact accepted payload.
#[allow(dead_code)] // No production transport is connected.
pub(super) enum LiveCommentLookup {
    Absent,
    Ambiguous,
    Unauthorized,
    Unavailable,
    Observed {
        comment: GitHubPullRequestCommentProviderLookupObservation,
        verified_action: SpecContentHash,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum LiveCommentRecovery {
    Absent,
    Ambiguous,
    Unauthorized,
    Unavailable,
    Mismatch,
    Observed(GitHubPullRequestCommentProviderLookupObservation),
}

pub(super) fn live_comment_marker(
    action: &SpecContentHash,
    reservation: &SpecContentHash,
) -> String {
    let mut hash = Sha256::new();
    hash.update(b"workflow-os/live-comment-recovery/v1\0");
    hash.update(action.as_str().as_bytes());
    hash.update(b"\0");
    hash.update(reservation.as_str().as_bytes());
    format!("{MARKER_NAMESPACE}{:x}", hash.finalize())
}

fn live_effect_commitment(
    domain: &[u8],
    first: &SpecContentHash,
    second: &SpecContentHash,
    effect: &SideEffectRecord,
) -> Result<SpecContentHash, WorkflowOsError> {
    let mut hash = Sha256::new();
    hash.update(domain);
    hash.update(serde_json::to_vec(&(first, second, effect)).map_err(|_| corrupt())?);
    SpecContentHash::new(format!("{:x}", hash.finalize()))
}

#[cfg(test)]
thread_local! {
    static INJECTED_COMMIT_FAULT: Cell<Option<InjectedDispatchCommitFault>> = const { Cell::new(None) };
}

#[cfg(test)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum InjectedDispatchCommitFault {
    Before,
    After,
}

#[cfg(test)]
pub(super) fn inject_dispatch_commit_fault(fault: InjectedDispatchCommitFault) {
    INJECTED_COMMIT_FAULT.with(|slot| slot.set(Some(fault)));
}

impl DispatchReservationStore for SqliteStateBackend {
    fn reserve_attempt_dispatch(
        &self,
        request: DispatchReservationRequest,
    ) -> Result<DispatchReservationOutcome, WorkflowOsError> {
        self.reserve_attempt_dispatch_checked(request, None, |_, _, _| Ok(()))
    }
}

impl SqliteStateBackend {
    #[allow(dead_code)] // Private until a reviewed signer caller exists.
    pub(super) fn claim_installation_exchange(
        &self,
        request: &DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        reviewed: ReviewedInstallation<'_>,
    ) -> Result<InstallationIssuanceDisposition, WorkflowOsError> {
        self.claim_installation_exchange_with_fault(request, admission, reviewed, |_| Ok(()))
    }

    pub(super) fn claim_installation_exchange_with_fault(
        &self,
        request: &DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        reviewed: ReviewedInstallation<'_>,
        commit_boundary: impl FnMut(bool) -> Result<(), WorkflowOsError>,
    ) -> Result<InstallationIssuanceDisposition, WorkflowOsError> {
        self.claim_installation_exchange_internal(
            request,
            admission,
            reviewed,
            false,
            commit_boundary,
        )
    }

    fn claim_installation_exchange_internal(
        &self,
        request: &DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        reviewed: ReviewedInstallation<'_>,
        require_signer_schema: bool,
        mut commit_boundary: impl FnMut(bool) -> Result<(), WorkflowOsError>,
    ) -> Result<InstallationIssuanceDisposition, WorkflowOsError> {
        if request.request_commitment != request_commitment(request)
            || request.operation_id != installation_dispatch_operation(request)?
            || request.workflow_lifecycle_commitment.is_some()
            || reviewed.run_id != &request.expected_window_binding.run_id
            || reviewed.immutable_run
                != request
                    .expected_window_binding
                    .immutable_run_bundle
                    .root_hash()
            || !can_encode_marked_comment(admission.body)
        {
            return Err(not_dispatchable());
        }
        let run = reviewed.run_id;
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| storage_error("begin_failed"))?;
        let schema = super::validate_operational_schema(&tx)?;
        let claim = read_installation_claim(&tx, request)?;
        if claim.as_ref().is_some_and(|prior| {
            prior.grant != admission.grant_id.as_str()
                || !prior.matches(request, admission.side_effect, reviewed)
        }) {
            return Err(not_dispatchable());
        }
        if let Some(posture) =
            existing_installation_disposition(&tx, request, admission.side_effect)?
        {
            return Ok(InstallationIssuanceDisposition::Reserved(posture));
        }
        if claim.is_some() {
            return Ok(InstallationIssuanceDisposition::IssuanceUncertain);
        }
        if require_signer_schema && !installation_signer_schema(schema) {
            return Err(not_dispatchable());
        }
        let now = verify_current_installation_exchange(&tx, request, admission, reviewed)?;
        let record = InstallationClaimRecord {
            attempt: request.capability.attempt_id.as_str().to_owned(),
            window: request.capability.window_id.as_str().to_owned(),
            operation: request.operation_id.as_str().to_owned(),
            run: run.clone(),
            effect: admission.side_effect.clone(),
            grant: admission.grant_id.as_str().to_owned(),
            request: request.request_commitment.clone(),
            scope: reviewed
                .issuance_commitment()
                .ok_or_else(not_dispatchable)?,
            claimed_at: now,
            deadline: reviewed.deadline,
        };
        let payload = encode_json(&record, "installation issuance claim")?;
        let hash = SpecContentHash::from_text(&payload);
        tx.execute(
            "INSERT INTO installation_issuance_claims (attempt_id, window_id, operation_id, payload, payload_hash) VALUES (?1,?2,?3,?4,?5)",
            params![record.attempt, record.window, record.operation, payload, hash.as_str()],
        ).map_err(|_| storage_error("write_failed"))?;
        let bound = tx.execute(
            "UPDATE live_sandbox_grants SET issuance_claim_hash=?1 WHERE grant_id=?2 AND attempt_id=?3 AND issuance_claim_hash IS NULL",
            params![hash.as_str(), record.grant, record.attempt],
        ).map_err(|_| storage_error("write_failed"))?;
        if bound != 1 {
            return Err(corrupt());
        }
        commit_boundary(false)?;
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        commit_boundary(true)?;
        Ok(InstallationIssuanceDisposition::Fresh(
            InstallationIssuanceClaim {
                record: Box::new(record),
            },
        ))
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[allow(dead_code)] // No operator route is admitted yet.
    pub(super) fn coordinate_scoped_installation_comment(
        &self,
        request: DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        action: GitHubCommentContinuationAction<'_>,
        reviewed: ReviewedInstallation<'_>,
        open_signer: impl FnOnce() -> Option<InstallationSignerChannel>,
    ) -> Result<InstallationCommentRecoveryPosture, WorkflowOsError> {
        let exchange = |request: &DispatchReservationRequest, claim| {
            self.recheck_installation_signer_exchange(request, admission, reviewed, &claim, None)?;
            let Some(channel) = open_signer() else {
                return Ok(None);
            };
            self.exchange_with_installation_signer(request, admission, reviewed, claim, channel)
        };
        self.coordinate_installation_comment_using(
            request,
            admission,
            action,
            reviewed,
            exchange,
            |request, exchanged| {
                self.dispatch_scoped_installation_comment(
                    request, admission, action, reviewed, exchanged,
                )
            },
        )
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(super) fn coordinate_installation_comment_using(
        &self,
        request: DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        action: GitHubCommentContinuationAction<'_>,
        reviewed: ReviewedInstallation<'_>,
        exchange: impl FnOnce(
            &DispatchReservationRequest,
            InstallationIssuanceClaim,
        ) -> Result<Option<InstallationExchangedScope>, WorkflowOsError>,
        dispatch: impl FnOnce(
            DispatchReservationRequest,
            InstallationExchangedScope,
        ) -> Result<Option<EventId>, WorkflowOsError>,
    ) -> Result<InstallationCommentRecoveryPosture, WorkflowOsError> {
        if action.commitment()? != request.invocation_commitment
            || admission.side_effect != action.side_effect
            || admission.body != action.body
            || reviewed.target != action.target
            || reviewed.run_id != action.run
            || reviewed.immutable_run != action.bundle_root
        {
            return Err(not_dispatchable());
        }
        let operation = request.operation_id.clone();
        match self.claim_installation_exchange_internal(
            &request,
            admission,
            reviewed,
            true,
            |_| Ok(()),
        )? {
            InstallationIssuanceDisposition::Reserved(posture) => return Ok(posture),
            InstallationIssuanceDisposition::IssuanceUncertain => {}
            InstallationIssuanceDisposition::Fresh(claim) => {
                if let Some(exchanged) = exchange(&request, claim)? {
                    dispatch(request, exchanged)?;
                }
            }
        }
        self.inspect_installation_comment_recovery(action.run, &operation, action.side_effect)
    }

    #[cfg(test)]
    pub(super) fn reserve_installation_exchange_for_test(
        &self,
        request: DispatchReservationRequest,
        grant: LiveGrantAdmission<'_>,
        reviewed: ReviewedInstallation<'_>,
        exchanged: &InstallationExchangedScope,
    ) -> Result<Option<Box<ReservedAttemptDispatchCapability>>, WorkflowOsError> {
        self.reserve_live_sandbox_dispatch_with_binding(
            request,
            grant,
            None,
            Some(LiveInstallationScopeAdmission {
                reviewed,
                verified: &exchanged.verified,
                fixture: false,
                claim: Some(&exchanged.claim),
            }),
        )
    }

    #[cfg(all(test, any(target_os = "linux", target_os = "macos")))]
    pub(super) fn dispatch_lost_installation_ack_for_test(
        &self,
        request: DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        action: GitHubCommentContinuationAction<'_>,
        reviewed: ReviewedInstallation<'_>,
        exchanged: &InstallationExchangedScope,
        outcome: (
            Option<InjectedDispatchCommitFault>,
            impl FnOnce(&str, Timestamp),
        ),
    ) -> Result<Option<EventId>, WorkflowOsError> {
        let operation = request.operation_id.clone();
        let capability = self
            .reserve_installation_exchange_for_test(request, admission, reviewed, exchanged)?
            .ok_or_else(not_dispatchable)?;
        let (fault, accept_without_response) = outcome;
        if let Some(fault) = fault {
            inject_dispatch_commit_fault(fault);
        }
        let (checked, raw_body) = self
            .prepare_installation_comment_send(
                &operation,
                action,
                admission,
                *capability,
                false,
                continuity_store::observe_continuity_trusted_time,
            )?
            .ok_or_else(not_dispatchable)?;
        accept_without_response(&raw_body, checked.window.checked_at);
        Ok(None)
    }

    #[cfg(all(test, any(target_os = "linux", target_os = "macos")))]
    pub(super) fn inspect_prepared_installation_send_for_test(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
    ) -> Result<Option<(SpecContentHash, Timestamp)>, WorkflowOsError> {
        let mut connection = self.existing_read_only_connection()?;
        let tx = connection
            .transaction()
            .map_err(|_| storage_error("begin_failed"))?;
        let admission = read_live_recovery_admission(&tx, operation, action.run)?;
        verify_live_recovery_state(&tx, &admission, action.run, action.side_effect)?;
        let record = read_installation_send_preparation_record(&tx, operation, action, &admission)?;
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(record.map(|record| (record.body_commitment, record.clock.observed_at())))
    }

    #[allow(dead_code)] // No receiver is provisioned for this private verifier.
    pub(super) fn verify_prepared_installation_notification(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        receiver: crate::provider_comment_transport::installation_notification::ReviewedCreationReceiver<'_>,
        notification: crate::provider_comment_transport::installation_notification::CreationNotification<'_>,
    ) -> Result<
        Option<crate::provider_comment_transport::installation_notification::CreationCandidate>,
        WorkflowOsError,
    > {
        use crate::provider_comment_transport::installation_notification::{
            verify_creation_notification, PreparedCreation,
        };
        let mut connection = self.existing_read_only_connection()?;
        let tx = connection
            .transaction()
            .map_err(|_| storage_error("begin_failed"))?;
        if !installation_preparation_schema(super::validate_operational_schema(&tx)?) {
            return Ok(None);
        }
        let admission = read_live_recovery_admission(&tx, operation, action.run)?;
        verify_live_recovery_state(&tx, &admission, action.run, action.side_effect)?;
        let witness = admission.witness.as_ref().ok_or_else(corrupt)?;
        let Some(record) =
            read_installation_send_preparation_record(&tx, operation, action, &admission)?
        else {
            return Ok(None);
        };
        let checked_at = continuity_store::observe_continuity_trusted_time()?;
        let candidate = verify_creation_notification(
            PreparedCreation {
                reservation: &record.reservation,
                writer_scope: &record.scope,
                target: action.target,
                repository: witness.provider.repository_id,
                writer: witness.provider.author_id,
                approved_body: action.body,
                marker_commitment: &record.marker_commitment,
                body_commitment: &record.body_commitment,
                preparation_time: &record.clock,
                checked_at: &checked_at,
                deadline: record.deadline,
                observer_profile: witness.installation_observer_profile.as_ref(),
            },
            receiver,
            notification,
        );
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(candidate)
    }

    #[allow(dead_code)] // Private until an independent read identity is provisioned.
    pub(super) fn observe_prepared_installation_inventory(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        candidate: &crate::provider_comment_transport::installation_notification::CreationCandidate,
        reader: &crate::provider_comment_transport::IndependentCommentInventory<'_>,
    ) -> Result<CurrentCommentPosture, WorkflowOsError> {
        let Some((before, traversal, _)) =
            self.collect_prepared_installation_inventory(operation, action, candidate, reader)?
        else {
            return Ok(CurrentCommentPosture::Unresolved);
        };
        if self
            .read_installation_inventory_context(operation, action)?
            .as_ref()
            == Some(&before)
        {
            Ok(traversal.posture)
        } else {
            Ok(CurrentCommentPosture::Unresolved)
        }
    }

    fn read_installation_inventory_context(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
    ) -> Result<Option<InstallationInventoryContext>, WorkflowOsError> {
        let mut connection = self.existing_read_only_connection()?;
        let tx = connection
            .transaction()
            .map_err(|_| storage_error("begin_failed"))?;
        let context = installation_inventory_context(&tx, operation, action, false)?;
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(context)
    }

    fn collect_prepared_installation_inventory(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        candidate: &crate::provider_comment_transport::installation_notification::CreationCandidate,
        reader: &crate::provider_comment_transport::IndependentCommentInventory<'_>,
    ) -> Result<Option<CollectedInstallationInventory>, WorkflowOsError> {
        use crate::provider_comment_transport::PreparedCommentInventory;
        let Some(context) = self.read_installation_inventory_context(operation, action)? else {
            return Ok(None);
        };
        let InstallationInventoryContext {
            record,
            provider,
            observer_profile,
        } = &context;
        let Some(comment_id) = candidate.prepared_comment_id(
            &record.reservation,
            &record.body_commitment,
            &record.marker_commitment,
            observer_profile.as_ref(),
        ) else {
            return Ok(None);
        };
        // Do not hold the SQLite transaction open across provider I/O.
        let traversal = reader.observe(&PreparedCommentInventory {
            target: action.target,
            repository: provider.repository_id,
            writer: provider.author_id,
            writer_scope: &record.scope,
            comment_id,
            body: &record.body_commitment,
            marker: &record.marker_commitment,
            observer_profile: observer_profile.as_ref(),
        });
        Ok(traversal.map(|value| (context, value, comment_id)))
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[allow(dead_code)] // No operator or harness entry is admitted.
    pub(super) fn exchange_with_installation_signer(
        &self,
        request: &DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        reviewed: ReviewedInstallation<'_>,
        claim: InstallationIssuanceClaim,
        channel: InstallationSignerChannel,
    ) -> Result<Option<InstallationExchangedScope>, WorkflowOsError> {
        self.exchange_with_installation_signer_using(
            request,
            admission,
            reviewed,
            claim,
            channel,
            crate::provider_comment_transport::installation_exchange::exchange_installation_scope,
        )
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(super) fn exchange_with_installation_signer_using(
        &self,
        request: &DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        reviewed: ReviewedInstallation<'_>,
        claim: InstallationIssuanceClaim,
        channel: InstallationSignerChannel,
        exchange: impl FnOnce(
            ReviewedInstallation<'_>,
            InstallationAppAssertion,
        ) -> Option<VerifiedInstallationScope>,
    ) -> Result<Option<InstallationExchangedScope>, WorkflowOsError> {
        self.recheck_installation_signer_exchange(request, admission, reviewed, &claim, None)?;
        let commitment = claim.record.commitment()?;
        let signer_request = InstallationSignerRequest::new(
            &claim.record.operation,
            &commitment,
            &claim.record.scope,
            claim.record.deadline,
        );
        let assertion = channel.request(&signer_request)?;
        let now = self.recheck_installation_signer_exchange(
            request,
            admission,
            reviewed,
            &claim,
            Some(assertion.issued_at()),
        )?;
        let verified = exchange(reviewed, assertion);
        finish_installation_exchange(claim, verified, now)
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[allow(dead_code)] // No signer service is deployed.
    pub(super) fn admit_installation_signer_request(
        &self,
        request: &DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        reviewed: ReviewedInstallation<'_>,
        received: &ReceivedInstallationSignerRequest,
    ) -> Result<InstallationSignerAdmission, WorkflowOsError> {
        self.admit_installation_signer_request_with_fault(
            request,
            admission,
            reviewed,
            received,
            |_| Ok(()),
        )
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(super) fn admit_installation_signer_request_with_fault(
        &self,
        request: &DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        reviewed: ReviewedInstallation<'_>,
        received: &ReceivedInstallationSignerRequest,
        mut commit_boundary: impl FnMut(bool) -> Result<(), WorkflowOsError>,
    ) -> Result<InstallationSignerAdmission, WorkflowOsError> {
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| storage_error("begin_failed"))?;
        require_installation_signer_schema(&tx)?;
        let (record, now) = revalidate_installation_claim(&tx, request, admission, reviewed)?;
        let claim = record.commitment()?;
        if !received.has_current_version()
            || received.operation != record.operation
            || received.claim != claim
            || received.scope != record.scope
            || received.deadline != record.deadline
            || read_installation_signer_admission(&tx, &record)?.is_some()
        {
            return Err(not_dispatchable());
        }
        let admitted = SignerAdmissionRecord {
            claim,
            operation: record.operation.clone(),
            scope: record.scope.clone(),
            deadline: record.deadline,
            admitted_at: now,
        };
        let payload = encode_json(&admitted, "installation signer admission")?;
        let changed = tx.execute(
            "UPDATE installation_issuance_claims SET signer_admission=?1, signer_admission_hash=?2
             WHERE operation_id=?3 AND payload_hash=?4 AND signer_admission IS NULL AND signer_admission_hash IS NULL",
            params![payload, SpecContentHash::from_text(&payload).as_str(), record.operation, admitted.claim.as_str()],
        ).map_err(|_| storage_error("write_failed"))?;
        if changed != 1 {
            return Err(not_dispatchable());
        }
        commit_boundary(false)?;
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        commit_boundary(true)?;
        Ok(InstallationSignerAdmission(admitted))
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn recheck_installation_signer_exchange(
        &self,
        request: &DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        reviewed: ReviewedInstallation<'_>,
        claim: &InstallationIssuanceClaim,
        assertion_issued_at: Option<Timestamp>,
    ) -> Result<Timestamp, WorkflowOsError> {
        let mut connection = self.existing_read_only_connection()?;
        let tx = connection
            .transaction()
            .map_err(|_| storage_error("begin_failed"))?;
        require_installation_signer_schema(&tx)?;
        let (record, now) = revalidate_installation_claim(&tx, request, admission, reviewed)?;
        if record != *claim.record {
            return Err(not_dispatchable());
        }
        match (
            assertion_issued_at,
            read_installation_signer_admission(&tx, &record)?,
        ) {
            (None, None) => {}
            (Some(issued), Some(admitted)) if admitted.admitted_at <= issued && issued <= now => {}
            _ => return Err(not_dispatchable()),
        }
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(now)
    }

    #[cfg(test)]
    pub(super) fn use_installation_exchange_claim(
        &self,
        request: &DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        reviewed: ReviewedInstallation<'_>,
        claim: InstallationIssuanceClaim,
        exchange: impl FnOnce() -> Option<VerifiedInstallationScope>,
    ) -> Result<Option<InstallationExchangedScope>, WorkflowOsError> {
        let now = self.recheck_installation_exchange_claim(request, admission, reviewed, &claim)?;
        finish_installation_exchange(claim, exchange(), now)
    }

    #[allow(dead_code)] // Other targets expose no signer channel.
    fn recheck_installation_exchange_claim(
        &self,
        request: &DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        reviewed: ReviewedInstallation<'_>,
        claim: &InstallationIssuanceClaim,
    ) -> Result<Timestamp, WorkflowOsError> {
        let mut connection = self.existing_read_only_connection()?;
        let tx = connection
            .transaction()
            .map_err(|_| storage_error("begin_failed"))?;
        super::validate_operational_schema(&tx)?;
        let (record, now) = revalidate_installation_claim(&tx, request, admission, reviewed)?;
        if record != *claim.record {
            return Err(not_dispatchable());
        }
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(now)
    }

    /// Inspects an existing synthetic reservation without granting authority,
    /// making provider calls, or mutating the database.
    ///
    /// # Errors
    ///
    /// Returns an error for missing, unsupported, or mismatched durable proof.
    pub fn inspect_synthetic_sandbox_recovery(
        &self,
        run_id: &crate::WorkflowRunId,
        operation_id: &str,
        side_effect_id: &crate::SideEffectId,
    ) -> Result<SyntheticSandboxRecoveryPosture, WorkflowOsError> {
        self.inspect_synthetic_sandbox_recovery_proof(run_id, operation_id, side_effect_id)
            .map(SyntheticSandboxRecoveryInspection::posture)
    }

    /// Inspects synthetic posture and verified provenance in the same read transaction.
    ///
    /// # Errors
    ///
    /// Returns an error for missing, unsupported, or mismatched durable proof.
    pub fn inspect_synthetic_sandbox_recovery_proof(
        &self,
        run_id: &crate::WorkflowRunId,
        operation_id: &str,
        side_effect_id: &crate::SideEffectId,
    ) -> Result<SyntheticSandboxRecoveryInspection, WorkflowOsError> {
        let operation = DispatchReservationOperationId::new(operation_id)?;
        let mut connection = self.existing_read_only_connection()?;
        let tx = connection
            .transaction()
            .map_err(|_| storage_error("begin_failed"))?;
        let admission = read_live_recovery_admission(&tx, &operation, run_id)?;
        verify_live_recovery_state(&tx, &admission, run_id, side_effect_id)
    }

    // Historical disposition is not current write authority. No current grant,
    // clock, provider transport, or send permit participates in this read.
    #[allow(dead_code)] // No operator caller is admitted yet.
    pub(super) fn inspect_installation_comment_recovery(
        &self,
        run_id: &crate::WorkflowRunId,
        operation: &DispatchReservationOperationId,
        side_effect_id: &crate::SideEffectId,
    ) -> Result<InstallationCommentRecoveryPosture, WorkflowOsError> {
        let mut connection = self.existing_read_only_connection()?;
        let tx = connection
            .transaction()
            .map_err(|_| storage_error("begin_failed"))?;
        if read_existing(&tx, operation.as_str())?.is_none() {
            return inspect_unreserved_installation_claim(&tx, run_id, operation, side_effect_id);
        }
        let admission = read_live_recovery_admission(&tx, operation, run_id)?;
        installation_disposition(&tx, &admission, run_id, side_effect_id)
    }

    #[cfg(test)]
    pub(super) fn seed_installation_recovery_for_test(
        &self,
        request: DispatchReservationRequest,
        grant: LiveGrantAdmission<'_>,
        action: GitHubCommentContinuationAction<'_>,
        reviewed: ReviewedInstallation<'_>,
        completed: bool,
    ) -> Result<(), WorkflowOsError> {
        let operation = request.operation_id.clone();
        let InstallationIssuanceDisposition::Fresh(claim) =
            self.claim_installation_exchange(&request, grant, reviewed)?
        else {
            return Err(not_dispatchable());
        };
        let exchanged = self
            .use_installation_exchange_claim(&request, grant, reviewed, claim, || {
                Some(VerifiedInstallationScope::test_receipt_with_live_shape(
                    reviewed,
                    Timestamp::now_utc(),
                    reviewed.deadline,
                ))
            })?
            .ok_or_else(not_dispatchable)?;
        self.reserve_live_sandbox_dispatch_with_binding(
            request,
            grant,
            None,
            Some(LiveInstallationScopeAdmission {
                reviewed,
                verified: &exchanged.verified,
                fixture: false,
                claim: Some(&exchanged.claim),
            }),
        )?
        .ok_or_else(not_dispatchable)?;
        if completed {
            // Seed only the durable contract. This is not a provider receipt.
            let mut connection = self.existing_connection()?;
            let tx = connection
                .transaction_with_behavior(TransactionBehavior::Immediate)
                .map_err(|_| storage_error("begin_failed"))?;
            let admission = read_live_recovery_admission(&tx, &operation, action.run)?;
            persist_live_comment_outcome(
                &tx,
                action,
                &admission,
                SideEffectOutcomeReference::new(
                    SideEffectOutcomeReferenceKind::Outcome,
                    "github/comment/22",
                )?,
                LiveCommentProofKind::InstallationCreation,
            )?;
            tx.commit().map_err(|_| storage_error("commit_failed"))?;
        }
        Ok(())
    }

    /// Uses a historical reservation to check a current observation. This
    /// result cannot enter the synthetic completion writer.
    #[allow(dead_code)] // No production observer is connected.
    pub(super) fn observe_reserved_sandbox_comment<'a>(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        adapter_commitment: &SpecContentHash,
        lookup: impl FnOnce(&GitHubPullRequestCommentTarget, &str) -> CommentEnumeration<'a>,
    ) -> Result<CurrentCommentPosture, WorkflowOsError> {
        self.with_reserved_comment_context(operation, action, adapter_commitment, |_, expected| {
            verify_current_comment(
                expected,
                lookup(expected.target, expected.reservation_marker),
            )
        })
    }

    #[allow(dead_code)] // Not exposed to a live caller.
    pub(super) fn observe_reserved_live_comment(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
    ) -> Result<CurrentCommentPosture, WorkflowOsError> {
        // A verified reservation does not establish the provider's creation clock.
        // Without such a source there is no reason to contact the provider.
        self.with_reserved_comment_context_for_adapters(
            operation,
            action,
            &[
                fixed_comment_observer_commitment(),
                fixed_comment_exchange_commitment(),
            ],
            |_, _| CurrentCommentPosture::Unresolved,
        )
    }

    pub(super) fn with_reserved_comment_context<T>(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        adapter_commitment: &SpecContentHash,
        observe: impl FnOnce(&SpecContentHash, &ExpectedComment<'_>) -> T,
    ) -> Result<T, WorkflowOsError> {
        self.with_reserved_comment_context_for_adapters(
            operation,
            action,
            std::slice::from_ref(adapter_commitment),
            observe,
        )
    }

    fn with_reserved_comment_context_for_adapters<T>(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        allowed_adapters: &[SpecContentHash],
        observe: impl FnOnce(&SpecContentHash, &ExpectedComment<'_>) -> T,
    ) -> Result<T, WorkflowOsError> {
        let mut connection = self.existing_read_only_connection()?;
        let tx = connection
            .transaction()
            .map_err(|_| storage_error("begin_failed"))?;
        let context = read_reserved_comment_context(&tx, operation, action, allowed_adapters)?;
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(observe(&context.reservation, &context.expected(action)))
    }

    #[allow(dead_code)] // No live caller is enabled.
    pub(super) fn persist_reserved_comment_observation(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        witness: VerifiedGitHubCommentObservation,
    ) -> Result<StoredGitHubCommentObservation, WorkflowOsError> {
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| storage_error("begin_failed"))?;
        let context = durable_comment_context(&tx, operation, action)?;
        let record = witness.into_stored();
        if !record.matches(&context.reservation, &context.expected(action)) {
            return Err(not_dispatchable());
        }
        if let Some(existing) = read_comment_observation(&tx, operation, action, &context)? {
            if existing != record {
                return Err(not_dispatchable());
            }
        } else {
            let (payload, hash) = comment_observation_payload(operation, &record)?;
            tx.execute(
                "INSERT INTO live_comment_observations (operation_id, payload, payload_hash) VALUES (?1,?2,?3)",
                params![operation.as_str(), payload, hash.as_str()],
            ).map_err(|_| storage_error("write_failed"))?;
        }
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(record)
    }

    #[allow(dead_code)] // No live caller is enabled.
    pub(super) fn read_reserved_comment_observation(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
    ) -> Result<Option<StoredGitHubCommentObservation>, WorkflowOsError> {
        let mut connection = self.existing_read_only_connection()?;
        let tx = connection
            .transaction()
            .map_err(|_| storage_error("begin_failed"))?;
        let context = durable_comment_context(&tx, operation, action)?;
        let result = read_comment_observation(&tx, operation, action, &context)?;
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(result)
    }

    pub(crate) fn reserve_attempt_dispatch_with_lifecycle(
        &self,
        request: DispatchReservationRequest,
        lifecycle: &crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext,
    ) -> Result<DispatchReservationOutcome, WorkflowOsError> {
        self.reserve_attempt_dispatch_checked(request, Some(lifecycle), |_, _, _| Ok(()))
    }

    /// Runs a kernel-owned admission check on the locked, current state before
    /// writing a new reservation. Exact replay never calls the check or issues
    /// a capability; its receipt is for trusted callers, not caller authentication.
    /// The check may persist a grant-use binding but must not end the transaction
    /// or perform any external effect.
    pub(crate) fn reserve_attempt_dispatch_checked(
        &self,
        request: DispatchReservationRequest,
        lifecycle: Option<
            &crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext,
        >,
        check_admission: impl FnOnce(
            &rusqlite::Transaction<'_>,
            Timestamp,
            Timestamp,
        ) -> Result<(), WorkflowOsError>,
    ) -> Result<DispatchReservationOutcome, WorkflowOsError> {
        self.reserve_dispatch(request, lifecycle, None, None, check_admission)
    }

    /// Live-only admission. Exact replays and ambiguous commits yield no
    /// dispatch right or receipt through this entry point.
    #[allow(dead_code)] // The private live opening path is not connected yet.
    pub(super) fn reserve_live_sandbox_dispatch(
        &self,
        request: DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
    ) -> Result<Option<Box<ReservedAttemptDispatchCapability>>, WorkflowOsError> {
        self.reserve_live_sandbox_dispatch_with_binding(request, admission, None, None)
    }

    #[cfg(test)]
    pub(super) fn reserve_fixture_installation_scope(
        &self,
        request: DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        reviewed: ReviewedInstallation<'_>,
        verified: &VerifiedInstallationScope,
    ) -> Result<Option<Box<ReservedAttemptDispatchCapability>>, WorkflowOsError> {
        self.reserve_live_sandbox_dispatch_with_binding(
            request,
            admission,
            None,
            Some(LiveInstallationScopeAdmission {
                reviewed,
                verified,
                fixture: true,
                claim: None,
            }),
        )
    }

    fn reserve_live_sandbox_dispatch_with_binding(
        &self,
        request: DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        prepared: Option<(&GitHubPullRequestCommentTarget, &LiveSandboxProviderBinding)>,
        scope: Option<LiveInstallationScopeAdmission<'_>>,
    ) -> Result<Option<Box<ReservedAttemptDispatchCapability>>, WorkflowOsError> {
        if !can_encode_marked_comment(admission.body) {
            return Err(not_dispatchable());
        }
        let outcome =
            self.reserve_dispatch(request, None, Some(admission), scope, |tx, _, _| {
                if let Some((target, binding)) = prepared {
                    if read_active_live_target(tx, &target.reference())?.provider_binding()
                        != binding
                    {
                        return Err(not_dispatchable());
                    }
                }
                Ok(())
            })?;
        Ok(match outcome {
            DispatchReservationOutcome::Admitted { capability } => Some(capability),
            DispatchReservationOutcome::AlreadyAdmitted { .. }
            | DispatchReservationOutcome::CommittedButCapabilityUnavailable { .. } => None,
        })
    }

    /// A fresh, committed reservation is the only path to the injected send.
    /// Only an exact synthetic creation response may persist response proof.
    #[allow(dead_code)] // Private until synthetic dispatch is exposed.
    pub(super) fn dispatch_live_sandbox_comment(
        &self,
        request: DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        action: GitHubCommentContinuationAction<'_>,
        send: impl FnOnce(&GitHubPullRequestCommentTarget, &str) -> Option<CreatedCommentResponse>,
    ) -> Result<LiveCommentDispatchPosture, WorkflowOsError> {
        self.dispatch_live_sandbox_comment_with_clock(
            request,
            admission,
            action,
            continuity_store::observe_continuity_trusted_time,
            send,
        )
    }

    pub(super) fn dispatch_live_sandbox_comment_with_clock(
        &self,
        request: DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        action: GitHubCommentContinuationAction<'_>,
        observe: impl FnOnce() -> Result<TrustedTimeObservation, WorkflowOsError>,
        send: impl FnOnce(&GitHubPullRequestCommentTarget, &str) -> Option<CreatedCommentResponse>,
    ) -> Result<LiveCommentDispatchPosture, WorkflowOsError> {
        self.dispatch_live_sandbox_comment_with_binding(
            request,
            admission,
            action,
            None,
            observe,
            |target, body, _| send(target, body),
        )
    }

    #[cfg(test)]
    pub(super) fn dispatch_prepared_loopback_comment(
        &self,
        request: DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        action: GitHubCommentContinuationAction<'_>,
        prepared: PreparedGitHubCommentCreation<'_>,
        handoff_clock: Option<fn(CommentSendWindow) -> Timestamp>,
    ) -> Result<LiveCommentDispatchPosture, WorkflowOsError> {
        let binding = prepared
            .binding_for(action.target)
            .ok_or_else(not_dispatchable)?
            .clone();
        self.dispatch_live_sandbox_comment_with_binding(
            request,
            admission,
            action,
            Some(&binding),
            continuity_store::observe_continuity_trusted_time,
            |_, body, window| match handoff_clock {
                Some(observe) => prepared.send_with_test_clock(body, window, || observe(window)),
                None => prepared.send(body, window),
            },
        )
    }

    fn dispatch_live_sandbox_comment_with_binding(
        &self,
        request: DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        action: GitHubCommentContinuationAction<'_>,
        prepared: Option<&LiveSandboxProviderBinding>,
        observe: impl FnOnce() -> Result<TrustedTimeObservation, WorkflowOsError>,
        send: impl FnOnce(
            &GitHubPullRequestCommentTarget,
            &str,
            CommentSendWindow,
        ) -> Option<CreatedCommentResponse>,
    ) -> Result<LiveCommentDispatchPosture, WorkflowOsError> {
        let invocation = action.commitment()?;
        if invocation != request.invocation_commitment
            || admission.side_effect != action.side_effect
            || admission.body != action.body
        {
            return Err(not_dispatchable());
        }
        let operation = request.operation_id.clone();
        let Some(capability) = self.reserve_live_sandbox_dispatch_with_binding(
            request,
            admission,
            prepared.map(|binding| (action.target, binding)),
            None,
        )?
        else {
            return Ok(LiveCommentDispatchPosture::RecoveryOnly);
        };
        let binding = (*capability)
            .into_parts()
            .attempt_capability
            .dispatch_reservation
            .ok_or_else(corrupt)?;
        let marker = live_comment_marker(&invocation, &binding.reservation_commitment);
        let raw_body = encode_marked_comment(action.body, &marker).ok_or_else(corrupt)?;
        let Some(CheckedCommentSend { durable, window }) = self.checked_comment_send(
            &operation,
            action,
            admission,
            &binding.reservation_commitment,
            observe,
        )?
        else {
            return Ok(LiveCommentDispatchPosture::RecoveryOnly);
        };
        let witness = durable.witness.as_ref().ok_or_else(corrupt)?;
        if prepared.is_some_and(|expected| expected != &witness.provider) {
            return Err(corrupt());
        }
        let response = send(action.target, &raw_body, window);
        let Some(comment_id) = verify_created_comment_response(
            response.as_ref(),
            action.target,
            &witness.provider,
            &raw_body,
        ) else {
            return Ok(LiveCommentDispatchPosture::RecoveryOnly);
        };
        self.persist_synthetic_creation_response(action, &operation, &durable.event, comment_id)
    }

    fn checked_comment_send(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        admission: LiveGrantAdmission<'_>,
        reservation: &SpecContentHash,
        observe: impl FnOnce() -> Result<TrustedTimeObservation, WorkflowOsError>,
    ) -> Result<Option<CheckedCommentSend>, WorkflowOsError> {
        let mut connection = self.existing_read_only_connection()?;
        let Ok(time) = observe() else {
            return Ok(None);
        };
        let tx = connection
            .transaction()
            .map_err(|_| storage_error("begin_failed"))?;
        let result =
            Self::check_comment_send_in(&tx, operation, action, admission, reservation, &time)?;
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(result)
    }

    fn check_comment_send_in(
        tx: &rusqlite::Transaction<'_>,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        admission: LiveGrantAdmission<'_>,
        reservation: &SpecContentHash,
        time: &TrustedTimeObservation,
    ) -> Result<Option<CheckedCommentSend>, WorkflowOsError> {
        let durable = read_live_recovery_admission(tx, operation, action.run)?;
        let witness = durable.witness.as_ref().ok_or_else(corrupt)?;
        if durable.event.reservation_commitment() != reservation
            || durable.event.invocation_commitment() != &action.commitment()?
            || &witness.provider.adapter_commitment != admission.adapter_commitment
        {
            return Err(corrupt());
        }
        let state = continuity_codec::load_snapshot(tx)?;
        let trusted_time = &state.trusted_time;
        let Some(deadline) = witness.creation_deadline else {
            return Ok(None);
        };
        // Admission is irreversible. A paused or time-uncertain caller may
        // reconcile, but must not hand an expired send right to the adapter.
        let current_time = trusted_time.eligibility
            == ContinuityInstanceEligibility::LiveStateEligible
            && trusted_time.posture != TrustedTimePosture::Quarantined
            && time.source() == trusted_time.source
            && time.provenance_commitment() == &trusted_time.provenance_commitment
            && time.epoch_id() == &trusted_time.epoch_id
            && time.observed_at() >= durable.committed_at
            && !trusted_time
                .last_observed_at
                .is_some_and(|watermark| time.observed_at() < watermark)
            && time.observed_at() < deadline;
        if !current_time
            || !current_comment_execution(tx, &state, &durable, action, time, deadline)?
            || admission
                .verify_current_send(
                    tx,
                    &durable.event,
                    action.target,
                    &witness.provider,
                    time.observed_at(),
                )
                .is_err()
        {
            return Ok(None);
        }
        Ok(Some(CheckedCommentSend {
            durable,
            window: CommentSendWindow {
                checked_at: time.observed_at(),
                deadline,
            },
        }))
    }

    fn prepare_installation_comment_send(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        admission: LiveGrantAdmission<'_>,
        capability: ReservedAttemptDispatchCapability,
        fixture: bool,
        observe: impl FnOnce() -> Result<TrustedTimeObservation, WorkflowOsError>,
    ) -> Result<Option<(CheckedCommentSend, String)>, WorkflowOsError> {
        let binding = capability
            .into_parts()
            .attempt_capability
            .dispatch_reservation
            .ok_or_else(corrupt)?;
        let reservation = &binding.reservation_commitment;
        if fixture
            || !installation_preparation_schema(super::validate_operational_schema(
                &self.existing_read_only_connection()?,
            )?)
        {
            return self
                .checked_comment_send(operation, action, admission, reservation, observe)?
                .map(|checked| {
                    if checked.durable.witness.as_ref().is_some_and(|witness| {
                        witness.installation_send_preparation_version.is_some()
                    }) {
                        return Err(not_dispatchable());
                    }
                    let marker = live_comment_marker(&action.commitment()?, reservation);
                    let body = encode_marked_comment(action.body, &marker).ok_or_else(corrupt)?;
                    Ok((checked, body))
                })
                .transpose();
        }
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| storage_error("begin_failed"))?;
        if !installation_preparation_schema(super::validate_operational_schema(&tx)?) {
            return Err(not_dispatchable());
        }
        let time = observe()?;
        let Some(checked) =
            Self::check_comment_send_in(&tx, operation, action, admission, reservation, &time)?
        else {
            return Ok(None);
        };
        let witness = checked.durable.witness.as_ref().ok_or_else(corrupt)?;
        if checked.durable.event.receipt_id() != &binding.receipt_id
            || !is_installation_comment_adapter(&witness.provider.adapter_commitment, false)
        {
            return Err(not_dispatchable());
        }
        let scope = witness
            .installation_scope_commitment
            .clone()
            .ok_or_else(corrupt)?;
        if witness.installation_issuance_claim_commitment.is_none()
            || witness.installation_send_preparation_version != Some(1)
        {
            return Err(not_dispatchable());
        }
        if read_installation_send_preparation_record(&tx, operation, action, &checked.durable)?
            .is_some()
        {
            return Ok(None);
        }
        let mut entropy = [0_u8; 32];
        getrandom::getrandom(&mut entropy).map_err(|_| not_dispatchable())?;
        let marker = format!("{MARKER_NAMESPACE}{}", SpecContentHash::from_bytes(entropy));
        let body = encode_marked_comment(action.body, &marker).ok_or_else(corrupt)?;
        let record = InstallationSendPreparationRecord {
            operation: operation.as_str().to_owned(),
            reservation: reservation.clone(),
            invocation: action.commitment()?,
            scope,
            marker_commitment: SpecContentHash::from_text(&marker),
            body_commitment: SpecContentHash::from_text(&body),
            clock: time,
            deadline: checked.window.deadline,
        };
        let (payload, hash) = installation_send_preparation_payload(&record)?;
        tx.execute(
            "INSERT INTO installation_send_preparations (operation_id, payload, payload_hash) VALUES (?1,?2,?3)",
            params![operation.as_str(), payload, hash.as_str()],
        ).map_err(|_| storage_error("write_failed"))?;
        #[cfg(test)]
        let fault = INJECTED_COMMIT_FAULT.with(Cell::take);
        #[cfg(test)]
        if fault == Some(InjectedDispatchCommitFault::Before) {
            return Err(storage_error("commit_failed"));
        }
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        #[cfg(test)]
        if fault == Some(InjectedDispatchCommitFault::After) {
            return Err(storage_error("commit_failed"));
        }
        Ok(Some((checked, body)))
    }

    #[allow(dead_code)] // No public or operator route is connected.
    pub(super) fn dispatch_scoped_installation_comment(
        &self,
        request: DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        action: GitHubCommentContinuationAction<'_>,
        reviewed: ReviewedInstallation<'_>,
        exchanged: InstallationExchangedScope,
    ) -> Result<Option<EventId>, WorkflowOsError> {
        let InstallationExchangedScope { claim, verified } = exchanged;
        let transport = InstallationCommentTransport::new(verified, reviewed);
        self.dispatch_installation_comment_with_clock(
            request,
            admission,
            action,
            transport,
            Some(&claim),
            continuity_store::observe_continuity_trusted_time,
        )
    }

    #[cfg(test)]
    pub(super) fn dispatch_fixture_installation_comment_with_clock(
        &self,
        request: DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        action: GitHubCommentContinuationAction<'_>,
        transport: InstallationCommentTransport<'_>,
        observe: impl FnOnce() -> Result<TrustedTimeObservation, WorkflowOsError>,
    ) -> Result<Option<EventId>, WorkflowOsError> {
        if !transport.is_fixture() {
            return Err(not_dispatchable());
        }
        self.dispatch_installation_comment_with_clock(
            request, admission, action, transport, None, observe,
        )
    }

    fn dispatch_installation_comment_with_clock(
        &self,
        request: DispatchReservationRequest,
        admission: LiveGrantAdmission<'_>,
        action: GitHubCommentContinuationAction<'_>,
        transport: InstallationCommentTransport<'_>,
        claim: Option<&InstallationIssuanceClaim>,
        observe: impl FnOnce() -> Result<TrustedTimeObservation, WorkflowOsError>,
    ) -> Result<Option<EventId>, WorkflowOsError> {
        let invocation = action.commitment()?;
        let reviewed = transport.reviewed();
        if invocation != request.invocation_commitment
            || admission.side_effect != action.side_effect
            || admission.body != action.body
            || reviewed.target != action.target
        {
            return Err(not_dispatchable());
        }
        let fixture = transport.is_fixture();
        let operation = request.operation_id.clone();
        let Some(capability) = self.reserve_live_sandbox_dispatch_with_binding(
            request,
            admission,
            None,
            Some(LiveInstallationScopeAdmission {
                reviewed,
                verified: transport.verified(),
                fixture,
                claim,
            }),
        )?
        else {
            return Ok(None);
        };
        if transport.verify_pull_request().is_none() {
            return Ok(None);
        }
        let send = self.prepare_installation_comment_send(
            &operation,
            action,
            admission,
            *capability,
            fixture,
            observe,
        )?;
        let Some((CheckedCommentSend { durable, window }, body)) = send else {
            return Ok(None);
        };
        let witness = durable.witness.as_ref().ok_or_else(corrupt)?;
        let scope = witness
            .installation_scope_commitment
            .as_ref()
            .ok_or_else(corrupt)?;
        let permit = InstallationCommentSendPermit {
            context: InstallationCreationContext {
                reservation: durable.event.reservation_commitment(),
                scope,
                target: action.target,
                provider: &witness.provider,
                marked_body: &body,
            },
            window,
        };
        let Some(proof) = transport.send(permit) else {
            return Ok(None);
        };
        #[cfg(test)]
        if fixture {
            return self
                .persist_fixture_installation_creation(&operation, action, &proof)
                .map(Some);
        }
        self.persist_installation_creation(&operation, action, &proof)
            .map(Some)
    }

    fn persist_synthetic_creation_response(
        &self,
        action: GitHubCommentContinuationAction<'_>,
        operation: &DispatchReservationOperationId,
        expected_admission: &DispatchAdmissionProjectionEvent,
        comment_id: std::num::NonZeroU64,
    ) -> Result<LiveCommentDispatchPosture, WorkflowOsError> {
        let outcome = SideEffectOutcomeReference::new(
            SideEffectOutcomeReferenceKind::Outcome,
            format!("github/comment/{comment_id}"),
        )?;
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| storage_error("begin_failed"))?;
        let current = read_live_recovery_admission(&tx, operation, action.run)?;
        if &current.event != expected_admission {
            return Err(corrupt());
        }
        persist_live_comment_outcome(
            &tx,
            action,
            &current,
            outcome,
            LiveCommentProofKind::CreationResponse,
        )?;
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(LiveCommentDispatchPosture::PersistedSyntheticResponse { comment_id })
    }

    #[allow(dead_code)] // No live operator or sender is connected.
    pub(super) fn persist_installation_creation(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        proof: &VerifiedInstallationCreation,
    ) -> Result<EventId, WorkflowOsError> {
        self.persist_installation_creation_with_kind(
            operation,
            action,
            proof,
            LiveCommentProofKind::InstallationCreation,
        )
    }

    #[cfg(test)]
    pub(super) fn persist_fixture_installation_creation(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        proof: &VerifiedInstallationCreation,
    ) -> Result<EventId, WorkflowOsError> {
        self.persist_installation_creation_with_kind(
            operation,
            action,
            proof,
            LiveCommentProofKind::InstallationFixtureCreation,
        )
    }

    fn persist_installation_creation_with_kind(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        proof: &VerifiedInstallationCreation,
        kind: LiveCommentProofKind,
    ) -> Result<EventId, WorkflowOsError> {
        let fixture = kind.fixture();
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| storage_error("begin_failed"))?;
        let durable = read_live_recovery_admission(&tx, operation, action.run)?;
        let witness = durable.witness.as_ref().ok_or_else(not_dispatchable)?;
        let scope = witness
            .installation_scope_commitment
            .as_ref()
            .ok_or_else(not_dispatchable)?;
        let invocation = action.commitment()?;
        if durable.event.invocation_commitment() != &invocation
            || !is_installation_comment_adapter(&witness.provider.adapter_commitment, fixture)
        {
            return Err(not_dispatchable());
        }
        let reservation = durable.event.reservation_commitment();
        let body = match witness.installation_send_preparation_version {
            Some(1) if !fixture => {
                read_installation_send_preparation_record(&tx, operation, action, &durable)?
                    .ok_or_else(not_dispatchable)?
                    .body_commitment
            }
            None => {
                let marker = live_comment_marker(&invocation, reservation);
                let raw =
                    encode_marked_comment(action.body, &marker).ok_or_else(not_dispatchable)?;
                SpecContentHash::from_text(&raw)
            }
            Some(_) => return Err(not_dispatchable()),
        };
        let comment = proof
            .matches_committed_body(
                reservation,
                scope,
                action.target,
                &witness.provider,
                &body,
                fixture,
            )
            .ok_or_else(not_dispatchable)?;
        let outcome = SideEffectOutcomeReference::new(
            SideEffectOutcomeReferenceKind::Outcome,
            format!("github/comment/{comment}"),
        )?;
        let event = persist_live_comment_outcome(&tx, action, &durable, outcome, kind)?;
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(event)
    }

    /// Operator-only lookup. This validates historical admission, not current
    /// permission to write. It never consumes a grant or persists an outcome.
    #[allow(dead_code)] // Private until a reviewed recovery caller exists.
    pub(super) fn recover_live_sandbox_comment(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        lookup: impl FnOnce(&GitHubPullRequestCommentTarget, &str) -> LiveCommentLookup,
    ) -> Result<LiveCommentRecovery, WorkflowOsError> {
        let invocation = action.commitment()?;
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction()
            .map_err(|_| storage_error("begin_failed"))?;
        let admission = read_live_recovery_admission(&tx, operation, action.run)?;
        if admission.event.invocation_commitment() != &invocation {
            return Err(not_dispatchable());
        }
        verify_live_recovery_state(&tx, &admission, action.run, action.side_effect)?;
        let marker = live_comment_marker(&invocation, admission.event.reservation_commitment());
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(match lookup(action.target, &marker) {
            LiveCommentLookup::Absent => LiveCommentRecovery::Absent,
            LiveCommentLookup::Ambiguous => LiveCommentRecovery::Ambiguous,
            LiveCommentLookup::Unauthorized => LiveCommentRecovery::Unauthorized,
            LiveCommentLookup::Unavailable => LiveCommentRecovery::Unavailable,
            LiveCommentLookup::Observed {
                comment,
                verified_action,
            } if verified_action == invocation
                && comment.target() == action.target
                && comment.managed_marker() == Some(marker.as_str())
                && comment.provider_comment_reference().is_some()
                && comment.validate().is_ok() =>
            {
                LiveCommentRecovery::Observed(comment)
            }
            LiveCommentLookup::Observed { .. } => LiveCommentRecovery::Mismatch,
        })
    }

    /// Reconciles synthetic observations without replacing an existing proof.
    #[allow(dead_code)] // Private until synthetic proof is exposed.
    pub(super) fn reconcile_live_sandbox_comment(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        lookup: impl FnOnce(&GitHubPullRequestCommentTarget, &str) -> LiveCommentLookup,
    ) -> Result<Option<EventId>, WorkflowOsError> {
        let LiveCommentRecovery::Observed(comment) =
            self.recover_live_sandbox_comment(operation, action, lookup)?
        else {
            return Ok(None);
        };
        let invocation = action.commitment()?;
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| storage_error("begin_failed"))?;
        let admission = read_live_recovery_admission(&tx, operation, action.run)?;
        if admission.event.invocation_commitment() != &invocation
            || comment.managed_marker()
                != Some(
                    live_comment_marker(&invocation, admission.event.reservation_commitment())
                        .as_str(),
                )
        {
            return Err(not_dispatchable());
        }
        let outcome = SideEffectOutcomeReference::new(
            SideEffectOutcomeReferenceKind::Outcome,
            comment
                .provider_comment_reference()
                .ok_or_else(not_dispatchable)?,
        )?;
        let id = persist_live_comment_outcome(
            &tx,
            action,
            &admission,
            outcome,
            LiveCommentProofKind::Lookup,
        )?;
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(Some(id))
    }

    #[allow(clippy::too_many_lines)]
    fn reserve_dispatch(
        &self,
        mut request: DispatchReservationRequest,
        lifecycle: Option<
            &crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext,
        >,
        live: Option<LiveGrantAdmission<'_>>,
        scope: Option<LiveInstallationScopeAdmission<'_>>,
        check_admission: impl FnOnce(
            &rusqlite::Transaction<'_>,
            Timestamp,
            Timestamp,
        ) -> Result<(), WorkflowOsError>,
    ) -> Result<DispatchReservationOutcome, WorkflowOsError> {
        let expected_request = request_commitment(&request);
        if request.request_commitment != expected_request
            || request.workflow_lifecycle_commitment
                != lifecycle.map(crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext::commitment)
        {
            return Err(reservation_error(
                WorkflowOsErrorKind::Validation,
                "request.invalid",
                "dispatch reservation request is invalid",
            ));
        }
        let mut connection = self.connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| storage_error("begin_failed"))?;
        check_dispatch_lane(
            &transaction,
            &request.capability.window_id,
            &request.capability.attempt_id,
            live.is_some(),
        )?;
        let observation = continuity_store::observe_continuity_trusted_time()?;
        let live = live
            .map(|admission| {
                admission.authenticate(&transaction, &request, observation.observed_at())
            })
            .transpose()?;

        if let Some((record, binding)) = read_existing(&transaction, request.operation_id.as_str())?
        {
            if record.request_commitment != request.request_commitment
                || record.receipt_id != request.receipt_id
            {
                return Err(reservation_error(
                    WorkflowOsErrorKind::Security,
                    "idempotency_conflict",
                    "dispatch reservation identity was reused with different content",
                ));
            }
            validate_existing_projection(&transaction, &record, &binding)?;
            transaction
                .commit()
                .map_err(|_| storage_error("commit_failed"))?;
            return Ok(DispatchReservationOutcome::AlreadyAdmitted {
                receipt: replay_receipt(&record, &binding),
            });
        }

        let mut state = continuity_codec::load_snapshot(&transaction)?;
        let last = verify_fresh_dispatch_state(
            &transaction,
            &state,
            &request,
            &observation,
            live.is_some(),
        )?;
        let window = state
            .windows
            .get_mut(&request.capability.window_id)
            .ok_or_else(not_dispatchable)?;
        let attempt = state
            .attempts
            .get_mut(&request.capability.attempt_id)
            .ok_or_else(not_dispatchable)?;
        check_admission(&transaction, observation.observed_at(), window.expires_at)?;
        let attempted_effect = if let Some(admission) = live {
            Some(admission.consume(
                &transaction,
                &request,
                observation.observed_at(),
                window.expires_at,
            )?)
        } else {
            None
        };
        let next_sequence = crate::EventSequenceNumber::new(
            last.sequence_number
                .get()
                .checked_add(1)
                .ok_or_else(corrupt)?,
        )?;
        let event_id = dispatch_admission_event_id(&request.operation_id)?;
        let terminal_sequence = if lifecycle.is_some() {
            crate::EventSequenceNumber::new(
                next_sequence.get().checked_add(1).ok_or_else(corrupt)?,
            )?
        } else {
            next_sequence
        };
        let terminal_event_id = if let Some(lifecycle) = lifecycle {
            lifecycle.event_id("invocation-started", request.operation_id.as_str())?
        } else {
            event_id.clone()
        };
        let admission_cursor =
            DispatchAdmissionProjectionCursor::new(next_sequence, event_id.clone());
        let trusted_commitment = trusted_time_commitment(&observation);
        let next_revision = request.capability.window_revision.checked_next()?;
        let live_witness = if let Some((effect, provider, deadline)) = attempted_effect {
            let observer_profile = scope
                .as_ref()
                .and_then(|scope| scope.reviewed.observer_profile);
            let claim_commitment = scope
                .as_ref()
                .and_then(|scope| scope.claim)
                .map(|claim| claim.record.commitment())
                .transpose()?;
            // The schema was validated on entry, before the deferred grant-use FK.
            let preparation_version = if claim_commitment.is_some() {
                let version: u32 = transaction
                    .pragma_query_value(None, "user_version", |row| row.get(0))
                    .map_err(|_| corrupt())?;
                installation_preparation_schema(version).then_some(1)
            } else {
                None
            };
            if observer_profile.is_some() && preparation_version.is_none() {
                return Err(not_dispatchable());
            }
            let (scope_commitment, deadline) = if let Some(scope) = scope {
                let (commitment, effective_deadline) = scope.validate(
                    &transaction,
                    &request,
                    &effect,
                    &provider,
                    observation.observed_at(),
                    deadline,
                )?;
                (Some(commitment), effective_deadline)
            } else {
                if is_installation_comment_adapter(&provider.adapter_commitment, false)
                    || is_installation_comment_adapter(&provider.adapter_commitment, true)
                {
                    return Err(not_dispatchable());
                }
                (None, deadline)
            };
            Some(LiveSandboxReservationWitness {
                attempted_record_commitment: live_effect_commitment(
                    b"workflow-os/live-comment-attempt/v1\0",
                    &request.request_commitment,
                    &trusted_commitment,
                    &effect,
                )?,
                provider,
                creation_deadline: Some(deadline),
                installation_scope_commitment: scope_commitment,
                installation_issuance_claim_commitment: claim_commitment,
                installation_send_preparation_version: preparation_version,
                installation_observer_profile: observer_profile.cloned(),
            })
        } else {
            None
        };
        let reservation_binding = if let Some(witness) = &live_witness {
            live_reservation_commitment(witness)?
        } else {
            reservation_commitment(&request, attempt.revision, &trusted_commitment)
        };
        let event =
            DispatchAdmissionProjectionEvent::new(DispatchAdmissionProjectionEventDefinition {
                operation_id: request.operation_id.clone(),
                receipt_id: request.receipt_id.clone(),
                window_id: request.capability.window_id.clone(),
                attempt_id: request.capability.attempt_id.clone(),
                window_revision: next_revision.get(),
                attempt_revision: next_revision.get(),
                request_commitment: request.request_commitment.clone(),
                invocation_commitment: request.invocation_commitment.clone(),
                executor_commitment: request.executor_commitment.clone(),
                authority_commitment: window.authority_commitment.clone(),
                governance_commitment: window.governance_commitment.clone(),
                trusted_time_commitment: trusted_commitment,
                reservation_commitment: reservation_binding.clone(),
                expected_input_cursor: DispatchAdmissionProjectionCursor::new(
                    last.sequence_number,
                    last.event_id.clone(),
                ),
                committed_result_cursor: admission_cursor.clone(),
            })?;

        let continuity_cursor =
            crate::authorized_execution_continuity_state::internal::ContinuityCursor {
                sequence_number: terminal_sequence,
                event_id: terminal_event_id.clone(),
            };
        window.cursor = continuity_cursor.clone();
        window.revision = next_revision;
        window.trusted_time_watermark = observation.observed_at();
        attempt.cursor = continuity_cursor.clone();
        attempt.revision = next_revision;
        state.trusted_time.last_observed_at = Some(observation.observed_at());
        state.trusted_time.posture = TrustedTimePosture::Healthy;
        state.trusted_time.revision = state.trusted_time.revision.checked_next()?;
        continuity_store::persist_trusted_time(&transaction, &state.trusted_time)?;
        continuity_store::persist_window(&transaction, window)?;
        continuity_store::persist_attempt(&transaction, attempt)?;

        let workflow_event = WorkflowRunEvent {
            sequence_number: next_sequence,
            event_id,
            timestamp: observation.observed_at(),
            run_id: request.expected_window_binding.run_id.clone(),
            workflow_id: request.expected_window_binding.workflow_id.clone(),
            schema_version: last.schema_version.clone(),
            workflow_version: last.workflow_version.clone(),
            spec_content_hash: last.spec_content_hash.clone(),
            correlation_id: None,
            actor: None,
            idempotency_key: Some(IdempotencyKey::new(format!(
                "dispatch-reservation/{}",
                short_hash(request.operation_id.as_str())
            ))?),
            kind: WorkflowRunEventKind::AuthorizedExecutionAttemptDispatchAdmitted(Box::new(
                event.clone(),
            )),
        };
        let snapshot = append_event_and_project_snapshot(&transaction, &workflow_event)?;
        let snapshot_payload = encode_json(&snapshot, "snapshot")?;
        let projection_snapshot_binding = snapshot_commitment(&snapshot_payload);
        if let Some(lifecycle) = lifecycle {
            append_event_and_project_snapshot(
                &transaction,
                &WorkflowRunEvent {
                    sequence_number: terminal_sequence,
                    event_id: terminal_event_id,
                    timestamp: observation.observed_at(),
                    run_id: request.expected_window_binding.run_id.clone(),
                    workflow_id: request.expected_window_binding.workflow_id.clone(),
                    schema_version: last.schema_version.clone(),
                    workflow_version: last.workflow_version.clone(),
                    spec_content_hash: last.spec_content_hash.clone(),
                    correlation_id: None,
                    actor: None,
                    idempotency_key: Some(lifecycle.idempotency_key("invocation-started")?),
                    kind: lifecycle.started_event(&request.capability.attempt_id)?,
                },
            )?;
        }
        let record = DispatchReservationRecord {
            operation_id: request.operation_id.clone(),
            receipt_id: request.receipt_id.clone(),
            window_id: request.capability.window_id.clone(),
            attempt_id: request.capability.attempt_id.clone(),
            window_revision: next_revision,
            attempt_revision: next_revision,
            request_commitment: request.request_commitment.clone(),
            reservation_commitment: reservation_binding.clone(),
            committed_at: observation.observed_at(),
            live_witness,
        };
        insert_reservation(
            &transaction,
            &request.expected_window_binding.run_id,
            &record,
            &event,
        )?;
        insert_projection(
            &transaction,
            &record,
            &ProjectionBinding {
                event: event.clone(),
                snapshot_commitment: projection_snapshot_binding,
            },
        )?;
        #[cfg(test)]
        let commit_fault = INJECTED_COMMIT_FAULT.with(Cell::take);
        #[cfg(not(test))]
        let commit_fault: Option<()> = None;
        #[cfg(test)]
        if commit_fault == Some(InjectedDispatchCommitFault::Before) {
            return Err(storage_error("write_failed"));
        }
        if transaction.commit().is_err() {
            return reconcile_committed_reservation(
                self,
                request.operation_id.as_str(),
                &request.request_commitment,
                &request.receipt_id,
            );
        }
        #[cfg(test)]
        if commit_fault == Some(InjectedDispatchCommitFault::After) {
            return reconcile_committed_reservation(
                self,
                request.operation_id.as_str(),
                &request.request_commitment,
                &request.receipt_id,
            );
        }
        #[cfg(not(test))]
        let _ = commit_fault;

        request.expected_window_binding.cursor = continuity_cursor.clone();
        request.capability.cursor = continuity_cursor;
        request.capability.window_revision = next_revision;
        request.capability.window_binding_commitment =
            window_binding_commitment(&request.expected_window_binding);
        let capability = admitted_capability(
            request.capability,
            request.expected_window_binding,
            request.operation_binding_commitment,
            request.receipt_id,
            reservation_binding,
            admission_cursor,
        );
        Ok(DispatchReservationOutcome::Admitted {
            capability: Box::new(capability),
        })
    }
}

pub(super) fn installation_dispatch_operation(
    request: &DispatchReservationRequest,
) -> Result<DispatchReservationOperationId, WorkflowOsError> {
    let identity = encode_json(
        &(
            "workflow-os/installation-dispatch-operation/v1",
            &request.capability.attempt_id,
            &request.invocation_commitment,
        ),
        "installation operation",
    )?;
    DispatchReservationOperationId::new(format!(
        "dispatch/installation/{}",
        SpecContentHash::from_text(&identity)
    ))
}

fn read_installation_claim(
    tx: &rusqlite::Transaction<'_>,
    request: &DispatchReservationRequest,
) -> Result<Option<InstallationClaimRecord>, WorkflowOsError> {
    read_installation_claim_by_key(
        tx,
        request.capability.attempt_id.as_str(),
        request.capability.window_id.as_str(),
        request.operation_id.as_str(),
    )
}

fn read_installation_claim_by_key(
    tx: &rusqlite::Transaction<'_>,
    expected_attempt: &str,
    expected_window: &str,
    expected_operation: &str,
) -> Result<Option<InstallationClaimRecord>, WorkflowOsError> {
    // Callers validate the full schema before writes. Reservation consumption
    // temporarily has a deferred grant-use foreign key until the send row is inserted.
    let version: u32 = tx
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .map_err(|_| corrupt())?;
    if !matches!(
        version,
        super::INSTALLATION_ISSUANCE_CLAIM_SCHEMA_VERSION
            | super::INSTALLATION_SIGNER_ADMISSION_SCHEMA_VERSION
            | super::INSTALLATION_SEND_PREPARATION_SCHEMA_VERSION
            | super::INSTALLATION_OBSERVATION_JOURNAL_SCHEMA_VERSION
    ) {
        return Err(not_dispatchable());
    }
    let row = tx.query_row(
        "SELECT attempt_id,window_id,operation_id,payload,payload_hash FROM installation_issuance_claims WHERE attempt_id=?1 OR operation_id=?2",
        params![expected_attempt, expected_operation],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?, row.get::<_, String>(4)?)),
    ).optional().map_err(|_| corrupt())?;
    let mut statement = tx.prepare(
        "SELECT grant_id,issuance_claim_hash FROM live_sandbox_grants WHERE attempt_id=?1 AND issuance_claim_hash IS NOT NULL",
    ).map_err(|_| corrupt())?;
    let mut grants = statement.query([expected_attempt]).map_err(|_| corrupt())?;
    let grant = grants
        .next()
        .map_err(|_| corrupt())?
        .map(|row| Ok::<_, rusqlite::Error>((row.get::<_, String>(0)?, row.get::<_, String>(1)?)))
        .transpose()
        .map_err(|_| corrupt())?;
    if grants.next().map_err(|_| corrupt())?.is_some() {
        return Err(corrupt());
    }
    let Some((attempt, window, operation, payload, hash)) = row else {
        if grant.is_some() {
            return Err(corrupt());
        }
        return Ok(None);
    };
    let record: InstallationClaimRecord =
        super::decode_json(&payload, "installation issuance claim").map_err(|_| corrupt())?;
    if record.attempt != attempt
        || record.window != window
        || record.operation != operation
        || attempt != expected_attempt
        || window != expected_window
        || operation != expected_operation
        || hash != SpecContentHash::from_text(&payload).as_str()
        || grant.as_ref() != Some(&(record.grant.clone(), hash))
    {
        return Err(corrupt());
    }
    Ok(Some(record))
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn require_installation_signer_schema(
    tx: &rusqlite::Transaction<'_>,
) -> Result<(), WorkflowOsError> {
    if !installation_signer_schema(super::validate_operational_schema(tx)?) {
        return Err(not_dispatchable());
    }
    Ok(())
}

const fn installation_signer_schema(version: u32) -> bool {
    matches!(
        version,
        super::INSTALLATION_SIGNER_ADMISSION_SCHEMA_VERSION
            | super::INSTALLATION_SEND_PREPARATION_SCHEMA_VERSION
            | super::INSTALLATION_OBSERVATION_JOURNAL_SCHEMA_VERSION
    )
}

fn read_installation_signer_admission(
    tx: &rusqlite::Transaction<'_>,
    claim: &InstallationClaimRecord,
) -> Result<Option<SignerAdmissionRecord>, WorkflowOsError> {
    let (payload, hash): (Option<String>, Option<String>) = tx.query_row(
        "SELECT signer_admission,signer_admission_hash FROM installation_issuance_claims WHERE operation_id=?1",
        [claim.operation.as_str()], |row| Ok((row.get(0)?, row.get(1)?)),
    ).map_err(|_| corrupt())?;
    match (payload, hash) {
        (None, None) => Ok(None),
        (Some(payload), Some(hash)) if SpecContentHash::from_text(&payload).as_str() == hash => {
            let admission: SignerAdmissionRecord =
                super::decode_json(&payload, "installation signer admission")
                    .map_err(|_| corrupt())?;
            if admission.claim != claim.commitment()?
                || admission.operation != claim.operation
                || admission.scope != claim.scope
                || admission.deadline != claim.deadline
                || admission.admitted_at < claim.claimed_at
                || admission.admitted_at >= claim.deadline
            {
                return Err(corrupt());
            }
            Ok(Some(admission))
        }
        _ => Err(corrupt()),
    }
}

fn revalidate_installation_claim(
    tx: &rusqlite::Transaction<'_>,
    request: &DispatchReservationRequest,
    admission: LiveGrantAdmission<'_>,
    reviewed: ReviewedInstallation<'_>,
) -> Result<(InstallationClaimRecord, Timestamp), WorkflowOsError> {
    let record = read_installation_claim(tx, request)?.ok_or_else(not_dispatchable)?;
    if !record.matches(request, admission.side_effect, reviewed)
        || record.grant != admission.grant_id.as_str()
        || request.request_commitment != request_commitment(request)
        || existing_installation_disposition(tx, request, admission.side_effect)?.is_some()
    {
        return Err(not_dispatchable());
    }
    let now = verify_current_installation_exchange(tx, request, admission, reviewed)?;
    if now < record.claimed_at {
        return Err(not_dispatchable());
    }
    Ok((record, now))
}

fn existing_installation_disposition(
    tx: &rusqlite::Transaction<'_>,
    request: &DispatchReservationRequest,
    effect: &crate::SideEffectId,
) -> Result<Option<InstallationCommentRecoveryPosture>, WorkflowOsError> {
    let run = &request.expected_window_binding.run_id;
    if let Some((record, _)) = read_existing(tx, request.operation_id.as_str())? {
        if record.request_commitment != request.request_commitment
            || record.receipt_id != request.receipt_id
            || record.window_id != request.capability.window_id
            || record.attempt_id != request.capability.attempt_id
        {
            return Err(not_dispatchable());
        }
        let admission = read_live_recovery_admission(tx, &request.operation_id, run)?;
        return installation_disposition(tx, &admission, run, effect).map(Some);
    }
    require_no_installation_dispatch(
        tx,
        run,
        &request.operation_id,
        &request.capability.attempt_id,
    )?;
    Ok(None)
}

fn require_no_installation_dispatch(
    tx: &rusqlite::Transaction<'_>,
    run: &crate::WorkflowRunId,
    operation: &DispatchReservationOperationId,
    attempt: &AuthorizedExecutionAttemptId,
) -> Result<(), WorkflowOsError> {
    let contradictory: bool = tx
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM dispatch_reservations WHERE attempt_id=?1)
             OR EXISTS(SELECT 1 FROM dispatch_reservation_projection_bindings WHERE operation_id=?2)
             OR EXISTS(SELECT 1 FROM live_sandbox_grant_uses WHERE attempt_id=?1)
             OR EXISTS(SELECT 1 FROM events WHERE event_id=?3)",
            params![
                attempt.as_str(),
                operation.as_str(),
                dispatch_admission_event_id(operation)?.as_str()
            ],
            |row| row.get(0),
        )
        .map_err(|_| corrupt())?;
    if contradictory || SqliteStateBackend::read_events_with_connection(tx, run)?.iter().any(|event| {
        matches!(&event.kind, WorkflowRunEventKind::AuthorizedExecutionAttemptDispatchAdmitted(admitted)
            if admitted.attempt_id() == attempt)
    }) {
        return Err(corrupt());
    }
    Ok(())
}

fn inspect_unreserved_installation_claim(
    tx: &rusqlite::Transaction<'_>,
    run: &crate::WorkflowRunId,
    operation: &DispatchReservationOperationId,
    effect_id: &crate::SideEffectId,
) -> Result<InstallationCommentRecoveryPosture, WorkflowOsError> {
    let version = super::validate_operational_schema(tx)?;
    if !matches!(
        version,
        super::INSTALLATION_ISSUANCE_CLAIM_SCHEMA_VERSION
            | super::INSTALLATION_SIGNER_ADMISSION_SCHEMA_VERSION
            | super::INSTALLATION_SEND_PREPARATION_SCHEMA_VERSION
            | super::INSTALLATION_OBSERVATION_JOURNAL_SCHEMA_VERSION
    ) {
        return Err(not_dispatchable());
    }
    let (attempt, window): (String, String) = tx
        .query_row(
            "SELECT attempt_id,window_id FROM installation_issuance_claims WHERE operation_id=?1",
            [operation.as_str()],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(|_| corrupt())?
        .ok_or_else(not_dispatchable)?;
    let attempt = AuthorizedExecutionAttemptId::new(attempt).map_err(|_| corrupt())?;
    let window = AuthorizedExecutionWindowId::new(window).map_err(|_| corrupt())?;
    check_dispatch_lane(tx, &window, &attempt, true)?;
    let claim =
        read_installation_claim_by_key(tx, attempt.as_str(), window.as_str(), operation.as_str())?
            .ok_or_else(corrupt)?;
    if &claim.run != run || &claim.effect != effect_id || claim.claimed_at >= claim.deadline {
        return Err(not_dispatchable());
    }
    if installation_signer_schema(version) {
        read_installation_signer_admission(tx, &claim)?;
    }
    require_no_installation_dispatch(tx, run, operation, &attempt)?;
    let state = continuity_codec::load_snapshot(tx)?;
    let window = state.windows.get(&window).ok_or_else(corrupt)?;
    let attempt = state.attempts.get(&attempt).ok_or_else(corrupt)?;
    let effect = read_live_effect_record(tx, effect_id)?;
    if &window.run_id != run
        || attempt.window_id != window.window_id
        || attempt.subject_actor_id != window.subject_actor_id
        || attempt.authority_commitment != window.authority_commitment
        || claim.deadline > window.expires_at
        || effect.side_effect_id() != effect_id
        || effect.run_id() != run
        || effect.step_id() != Some(&window.step_id)
        || effect.lifecycle_state() != SideEffectLifecycleState::Attempted
    {
        return Err(not_dispatchable());
    }
    Ok(InstallationCommentRecoveryPosture::IssuanceUncertain)
}

fn verify_current_installation_exchange(
    tx: &rusqlite::Transaction<'_>,
    request: &DispatchReservationRequest,
    admission: LiveGrantAdmission<'_>,
    reviewed: ReviewedInstallation<'_>,
) -> Result<Timestamp, WorkflowOsError> {
    check_dispatch_lane(
        tx,
        &request.capability.window_id,
        &request.capability.attempt_id,
        true,
    )?;
    let observation = continuity_store::observe_continuity_trusted_time()?;
    let now = observation.observed_at();
    let state = continuity_codec::load_snapshot(tx)?;
    verify_fresh_dispatch_state(tx, &state, request, &observation, true)?;
    let window = state
        .windows
        .get(&request.capability.window_id)
        .ok_or_else(not_dispatchable)?;
    let (effect, provider, deadline) = admission.authenticate(tx, request, now)?.verify_dispatch(
        tx,
        request,
        now,
        window.expires_at,
    )?;
    verify_reviewed_installation(tx, request, &effect, &provider, deadline, reviewed, false)?;
    if now >= deadline {
        return Err(not_dispatchable());
    }
    Ok(now)
}

fn verify_fresh_dispatch_state(
    tx: &rusqlite::Transaction<'_>,
    state: &ReferenceContinuityState,
    request: &DispatchReservationRequest,
    observation: &TrustedTimeObservation,
    live: bool,
) -> Result<WorkflowRunEvent, WorkflowOsError> {
    let window = state
        .windows
        .get(&request.capability.window_id)
        .ok_or_else(not_dispatchable)?;
    let attempt = state
        .attempts
        .get(&request.capability.attempt_id)
        .ok_or_else(not_dispatchable)?;
    if state.trusted_time.eligibility != ContinuityInstanceEligibility::LiveStateEligible
        || state.trusted_time.posture == TrustedTimePosture::Quarantined
        || observation.source() != state.trusted_time.source
        || observation.provenance_commitment() != &state.trusted_time.provenance_commitment
        || observation.epoch_id() != &state.trusted_time.epoch_id
        || state
            .trusted_time
            .last_observed_at
            .is_some_and(|prior| observation.observed_at() < prior)
        || (live
            && (observation.epoch_id() != &window.trusted_time_epoch_id
                || observation.observed_at() < window.trusted_time_watermark))
        || observation.observed_at() >= window.expires_at
        || window.state != AuthoritativeWindowState::Executing
        || (live && window.active_yield.is_some())
        || attempt.state != AuthoritativeAttemptState::Started
        || window.revision != request.capability.window_revision
        || attempt.revision != ContinuityRevision::new(1)?
        || window.cursor != request.capability.cursor
        || attempt.cursor != request.capability.cursor
        || window.workflow_id != request.expected_window_binding.workflow_id
        || window.run_id != request.expected_window_binding.run_id
        || window.step_id != request.expected_window_binding.step_id
        || window.subject_actor_id != request.expected_window_binding.subject_actor_id
        || window.immutable_run_bundle != request.expected_window_binding.immutable_run_bundle
        || window.governance_commitment != request.expected_window_binding.governance_commitment
        || window.authority_commitment != request.expected_window_binding.authority_commitment
        || request.capability.subject_actor_id != window.subject_actor_id
        || request.capability.authority_commitment != window.authority_commitment
        || request.capability.window_binding_commitment
            != window_binding_commitment(&request.expected_window_binding)
        || attempt.window_id != window.window_id
        || attempt.subject_actor_id != window.subject_actor_id
        || attempt.authority_commitment != window.authority_commitment
        || attempt.consume_operation_id != request.capability.consume_operation_id
    {
        return Err(not_dispatchable());
    }
    let conflicting: i64 = tx
        .query_row(
            "SELECT COUNT(*) FROM dispatch_reservations WHERE attempt_id=?1",
            [request.capability.attempt_id.as_str()],
            |row| row.get(0),
        )
        .map_err(|_| storage_error("read_failed"))?;
    if conflicting != 0 {
        return Err(not_dispatchable());
    }
    let last = SqliteStateBackend::read_events_with_connection(
        tx,
        &request.expected_window_binding.run_id,
    )?
    .pop()
    .ok_or_else(corrupt)?;
    if last.sequence_number != request.capability.cursor.sequence_number
        || last.event_id != request.capability.cursor.event_id
    {
        return Err(not_dispatchable());
    }
    Ok(last)
}

fn current_comment_execution(
    tx: &rusqlite::Transaction<'_>,
    state: &ReferenceContinuityState,
    durable: &LiveRecoveryAdmission,
    action: GitHubCommentContinuationAction<'_>,
    time: &TrustedTimeObservation,
    deadline: Timestamp,
) -> Result<bool, WorkflowOsError> {
    let event = &durable.event;
    let (Some(window), Some(attempt)) = (
        state.windows.get(event.window_id()),
        state.attempts.get(event.attempt_id()),
    ) else {
        return Ok(false);
    };
    let cursor = event.committed_result_cursor();
    if window.state != AuthoritativeWindowState::Executing
        || attempt.state != AuthoritativeAttemptState::Started
        || window.active_yield.is_some()
        || window.revision.get() != event.window_revision()
        || attempt.revision.get() != event.attempt_revision()
        || window.cursor.sequence_number != cursor.sequence_number()
        || &window.cursor.event_id != cursor.event_id()
        || attempt.cursor != window.cursor
        || &window.window_id != event.window_id()
        || &attempt.attempt_id != event.attempt_id()
        || attempt.window_id != window.window_id
        || &window.run_id != action.run
        || &window.step_id != action.step
        || &window.subject_actor_id != action.original_actor
        || attempt.subject_actor_id != window.subject_actor_id
        || window.immutable_run_bundle.root_hash() != action.bundle_root
        || &window.authority_commitment != event.authority_commitment()
        || attempt.authority_commitment != window.authority_commitment
        || &window.governance_commitment != event.governance_commitment()
        || &window.trusted_time_epoch_id != time.epoch_id()
        || time.observed_at() < window.trusted_time_watermark
        || deadline > window.expires_at
    {
        return Ok(false);
    }
    // The historical projection was validated in this same transaction.
    // A newer run event is not permission to use its old send cursor.
    let current: (i64, String) = tx
        .query_row(
            "SELECT last_sequence_number,last_event_id FROM snapshots WHERE run_id=?1",
            [action.run.as_str()],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|_| corrupt())?;
    if u64::try_from(current.0).ok() != Some(cursor.sequence_number().get())
        || current.1 != cursor.event_id().as_str()
    {
        return Ok(false);
    }
    let effect = read_live_effect_record(tx, action.side_effect)?;
    Ok(verify_live_comment_attempt(durable, &effect).is_ok())
}

fn read_reserved_comment_context(
    tx: &rusqlite::Transaction<'_>,
    operation: &DispatchReservationOperationId,
    action: GitHubCommentContinuationAction<'_>,
    allowed_adapters: &[SpecContentHash],
) -> Result<ReservedCommentContext, WorkflowOsError> {
    let invocation = action.commitment()?;
    let admission = read_live_recovery_admission(tx, operation, action.run)?;
    let witness = admission.witness.as_ref().ok_or_else(not_dispatchable)?;
    let deadline = witness.creation_deadline.ok_or_else(not_dispatchable)?;
    if admission.event.invocation_commitment() != &invocation
        || !allowed_adapters.contains(&witness.provider.adapter_commitment)
        || admission.committed_at >= deadline
    {
        return Err(not_dispatchable());
    }
    verify_live_recovery_state(tx, &admission, action.run, action.side_effect)?;
    let reservation = admission.event.reservation_commitment().clone();
    Ok(ReservedCommentContext {
        marker: live_comment_marker(&invocation, &reservation),
        reservation,
        provider: witness.provider.clone(),
        committed_at: admission.committed_at,
        deadline,
    })
}

fn durable_comment_context(
    tx: &rusqlite::Transaction<'_>,
    operation: &DispatchReservationOperationId,
    action: GitHubCommentContinuationAction<'_>,
) -> Result<ReservedCommentContext, WorkflowOsError> {
    if !matches!(
        super::validate_operational_schema(tx)?,
        super::LIVE_COMMENT_OBSERVATION_SCHEMA_VERSION
            | super::INSTALLATION_ISSUANCE_CLAIM_SCHEMA_VERSION
            | super::INSTALLATION_SIGNER_ADMISSION_SCHEMA_VERSION
            | super::INSTALLATION_SEND_PREPARATION_SCHEMA_VERSION
            | super::INSTALLATION_OBSERVATION_JOURNAL_SCHEMA_VERSION
    ) {
        return Err(not_dispatchable());
    }
    read_reserved_comment_context(
        tx,
        operation,
        action,
        &[
            fixed_comment_observer_commitment(),
            fixed_comment_exchange_commitment(),
        ],
    )
}

fn comment_observation_payload(
    operation: &DispatchReservationOperationId,
    record: &StoredGitHubCommentObservation,
) -> Result<(String, SpecContentHash), WorkflowOsError> {
    let payload = encode_json(record, "provider comment observation")?;
    let commitment = serde_json::to_vec(&(
        "workflow-os/sqlite-comment-observation/v1",
        operation.as_str(),
        &payload,
    ))
    .map_err(|_| corrupt())?;
    Ok((payload, SpecContentHash::from_bytes(commitment)))
}

fn read_comment_observation(
    tx: &rusqlite::Transaction<'_>,
    operation: &DispatchReservationOperationId,
    action: GitHubCommentContinuationAction<'_>,
    context: &ReservedCommentContext,
) -> Result<Option<StoredGitHubCommentObservation>, WorkflowOsError> {
    let row: Option<(String, String)> = tx
        .query_row(
            "SELECT payload, payload_hash FROM live_comment_observations WHERE operation_id=?1",
            [operation.as_str()],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(|_| storage_error("read_failed"))?;
    row.map(|(payload, hash)| {
        let record = super::decode_json(&payload, "provider comment observation")?;
        let (canonical, commitment) = comment_observation_payload(operation, &record)?;
        if payload != canonical
            || hash != commitment.as_str()
            || !record.matches(&context.reservation, &context.expected(action))
        {
            return Err(corrupt());
        }
        Ok(record)
    })
    .transpose()
}

fn read_live_recovery_admission(
    tx: &rusqlite::Transaction<'_>,
    operation: &DispatchReservationOperationId,
    run_id: &crate::WorkflowRunId,
) -> Result<LiveRecoveryAdmission, WorkflowOsError> {
    let (record, binding) = read_existing(tx, operation.as_str())?.ok_or_else(not_dispatchable)?;
    check_dispatch_lane(tx, &record.window_id, &record.attempt_id, true)?;
    validate_existing_projection(tx, &record, &binding)?;
    let has_exact_live_use: bool = tx
        .query_row(
            "SELECT EXISTS(
           SELECT 1 FROM events e JOIN live_sandbox_grant_uses u ON u.attempt_id=?3
           JOIN live_sandbox_grants g ON g.grant_id=u.grant_id AND g.attempt_id=u.attempt_id
           WHERE e.event_id=?1 AND e.run_id=?2 AND g.binding=?4
         )",
            params![
                binding.event.committed_result_cursor().event_id().as_str(),
                run_id.as_str(),
                record.attempt_id.as_str(),
                binding.event.executor_commitment().as_str(),
            ],
            |row| row.get(0),
        )
        .map_err(|_| corrupt())?;
    if !has_exact_live_use {
        return Err(not_dispatchable());
    }
    if let Some(commitment) = record
        .live_witness
        .as_ref()
        .and_then(|witness| witness.installation_issuance_claim_commitment.as_ref())
    {
        let claim = read_installation_claim_by_key(
            tx,
            record.attempt_id.as_str(),
            record.window_id.as_str(),
            operation.as_str(),
        )?
        .ok_or_else(corrupt)?;
        if &claim.commitment()? != commitment || &claim.run != run_id
            || claim.request != record.request_commitment || claim.claimed_at > record.committed_at
            || record.committed_at >= claim.deadline
            || !tx.query_row(
                "SELECT EXISTS(SELECT 1 FROM live_sandbox_grant_uses WHERE attempt_id=?1 AND grant_id=?2)",
                params![claim.attempt, claim.grant], |row| row.get::<_, bool>(0),
            ).map_err(|_| corrupt())?
        {
            return Err(corrupt());
        }
    }
    Ok(LiveRecoveryAdmission {
        operation: operation.clone(),
        event: binding.event,
        witness: record.live_witness,
        committed_at: record.committed_at,
    })
}

#[cfg(any(test, target_os = "linux", target_os = "macos"))]
fn finish_installation_exchange(
    claim: InstallationIssuanceClaim,
    verified: Option<VerifiedInstallationScope>,
    checked_at: Timestamp,
) -> Result<Option<InstallationExchangedScope>, WorkflowOsError> {
    let Some(verified) = verified else {
        return Ok(None);
    };
    if verified.verified_at() < checked_at {
        return Err(not_dispatchable());
    }
    Ok(Some(InstallationExchangedScope { claim, verified }))
}

fn installation_disposition(
    tx: &rusqlite::Transaction<'_>,
    admission: &LiveRecoveryAdmission,
    run: &crate::WorkflowRunId,
    effect: &crate::SideEffectId,
) -> Result<InstallationCommentRecoveryPosture, WorkflowOsError> {
    if !comment_proof_source_matches(admission, LiveCommentProofKind::InstallationCreation)
        || !admission
            .witness
            .as_ref()
            .and_then(|witness| witness.creation_deadline)
            .is_some_and(|deadline| admission.committed_at < deadline)
    {
        return Err(not_dispatchable());
    }
    match verify_comment_disposition(tx, admission, run, effect)? {
        Some(LiveCommentProofKind::InstallationCreation) => {
            Ok(InstallationCommentRecoveryPosture::CreationVerified)
        }
        Some(LiveCommentProofKind::InstallationObservation) => {
            Ok(InstallationCommentRecoveryPosture::ObservationReconciled)
        }
        None => Ok(InstallationCommentRecoveryPosture::ReconciliationRequired),
        Some(_) => Err(not_dispatchable()),
    }
}

fn verify_live_recovery_state(
    tx: &rusqlite::Transaction<'_>,
    admission: &LiveRecoveryAdmission,
    run_id: &crate::WorkflowRunId,
    side_effect_id: &crate::SideEffectId,
) -> Result<SyntheticSandboxRecoveryInspection, WorkflowOsError> {
    let source = verify_comment_disposition(tx, admission, run_id, side_effect_id)?
        .map(|kind| kind.synthetic().ok_or_else(not_dispatchable))
        .transpose()?;
    Ok(SyntheticSandboxRecoveryInspection { source })
}

fn verify_comment_disposition(
    tx: &rusqlite::Transaction<'_>,
    admission: &LiveRecoveryAdmission,
    run_id: &crate::WorkflowRunId,
    side_effect_id: &crate::SideEffectId,
) -> Result<Option<LiveCommentProofKind>, WorkflowOsError> {
    let history = SqliteStateBackend::read_events_with_connection(tx, run_id)?;
    let effect = read_live_effect_record(tx, side_effect_id)?;
    if effect.side_effect_id() != side_effect_id || effect.run_id() != run_id {
        return Err(not_dispatchable());
    }
    if let Some((event, kind)) = find_live_comment_proof(&admission.event, &history)? {
        verify_live_comment_proof(tx, admission, &effect, event, kind)?;
        return Ok(Some(kind));
    }
    verify_live_comment_attempt(admission, &effect)?;
    Ok(None)
}

fn live_comment_proof_id(
    admission: &DispatchAdmissionProjectionEvent,
    kind: LiveCommentProofKind,
) -> Result<EventId, WorkflowOsError> {
    let prefix = match kind {
        LiveCommentProofKind::Lookup => "live-comment-proof",
        LiveCommentProofKind::CreationResponse => "live-comment-response-proof",
        LiveCommentProofKind::InstallationCreation => "installation-comment-created",
        LiveCommentProofKind::InstallationObservation => "installation-comment-observed",
        LiveCommentProofKind::InstallationFixtureCreation => "fixture-installation-comment-created",
    };
    EventId::new(format!("{prefix}/{}", admission.reservation_commitment()))
}

fn find_live_comment_proof<'a>(
    admission: &DispatchAdmissionProjectionEvent,
    history: &'a [WorkflowRunEvent],
) -> Result<Option<(&'a WorkflowRunEvent, LiveCommentProofKind)>, WorkflowOsError> {
    let mut proof = None;
    for kind in [
        LiveCommentProofKind::Lookup,
        LiveCommentProofKind::CreationResponse,
        LiveCommentProofKind::InstallationCreation,
        LiveCommentProofKind::InstallationObservation,
        LiveCommentProofKind::InstallationFixtureCreation,
    ] {
        let id = live_comment_proof_id(admission, kind)?;
        if history
            .iter()
            .find(|event| event.event_id == id)
            .is_some_and(|event| proof.replace((event, kind)).is_some())
        {
            return Err(corrupt());
        }
    }
    Ok(proof)
}

fn verify_live_comment_attempt(
    admission: &LiveRecoveryAdmission,
    effect: &SideEffectRecord,
) -> Result<(), WorkflowOsError> {
    let attempted = live_effect_commitment(
        b"workflow-os/live-comment-attempt/v1\0",
        admission.event.request_commitment(),
        admission.event.trusted_time_commitment(),
        effect,
    )?;
    if effect.lifecycle_state() != SideEffectLifecycleState::Attempted
        || &attempted
            != admission.witness.as_ref().map_or_else(
                || admission.event.reservation_commitment(),
                |witness| &witness.attempted_record_commitment,
            )
    {
        return Err(not_dispatchable());
    }
    Ok(())
}

fn verify_live_comment_proof(
    tx: &rusqlite::Transaction<'_>,
    admission: &LiveRecoveryAdmission,
    effect: &SideEffectRecord,
    event: &WorkflowRunEvent,
    kind: LiveCommentProofKind,
) -> Result<(), WorkflowOsError> {
    let expected = crate::side_effect::transition_event(effect, 0)?;
    let admission_event = admission.event.committed_result_cursor();
    let key_matches = if kind == LiveCommentProofKind::InstallationObservation {
        observation_journal::verify_observed_completion(tx, admission, effect, event)?
    } else {
        event.idempotency_key.as_ref()
            == Some(&live_comment_proof_key(admission, effect, kind, None)?)
    };
    if !comment_proof_source_matches(admission, kind)
        || effect.lifecycle_state() != SideEffectLifecycleState::Completed
        || !effect
            .outcome_reference()
            .is_some_and(|outcome| outcome.kind() == SideEffectOutcomeReferenceKind::Outcome)
        || event.sequence_number <= admission_event.sequence_number()
        || Some(event.timestamp) != effect.updated_at()
        || !key_matches
        || !effect.references().iter().any(|reference| {
            reference.kind() == SideEffectReferenceKind::WorkflowEvent
                && reference.reference() == admission_event.event_id().as_str()
        })
        || !matches!(&event.kind, WorkflowRunEventKind::SideEffectCompleted(payload) if payload.as_ref() == &expected)
    {
        return Err(not_dispatchable());
    }
    Ok(())
}

fn live_comment_proof_key(
    admission: &LiveRecoveryAdmission,
    effect: &SideEffectRecord,
    kind: LiveCommentProofKind,
    observation: Option<&SpecContentHash>,
) -> Result<IdempotencyKey, WorkflowOsError> {
    if (kind == LiveCommentProofKind::InstallationObservation) != observation.is_some() {
        return Err(corrupt());
    }
    let domain: &[u8] = match (kind, admission.witness.is_some()) {
        (LiveCommentProofKind::Lookup, false) => b"workflow-os/live-comment-proof/v1\0",
        (LiveCommentProofKind::Lookup, true) => b"workflow-os/live-comment-proof/v2\0",
        (LiveCommentProofKind::CreationResponse, true) => {
            b"workflow-os/live-comment-response-proof/v1\0"
        }
        (LiveCommentProofKind::InstallationCreation, true) => {
            b"workflow-os/github-installation-comment-created/v1\0"
        }
        (LiveCommentProofKind::InstallationObservation, true) => {
            b"workflow-os/github-installation-comment-observed/v1\0"
        }
        (LiveCommentProofKind::InstallationFixtureCreation, true) => {
            b"workflow-os/fixture-installation-comment-created/v1\0"
        }
        (
            LiveCommentProofKind::CreationResponse
            | LiveCommentProofKind::InstallationCreation
            | LiveCommentProofKind::InstallationObservation
            | LiveCommentProofKind::InstallationFixtureCreation,
            false,
        ) => return Err(corrupt()),
    };
    let proof = live_effect_commitment(
        domain,
        admission.event.invocation_commitment(),
        admission.event.reservation_commitment(),
        effect,
    )?;
    let proof = observation.map_or(proof.clone(), |receipt| {
        SpecContentHash::from_text(&format!(
            "workflow-os/installation-observation-outcome/v1\0{proof}\0{receipt}"
        ))
    });
    IdempotencyKey::new(format!("live-comment-proof/{proof}"))
}

fn comment_proof_source_matches(
    admission: &LiveRecoveryAdmission,
    kind: LiveCommentProofKind,
) -> bool {
    let scoped = admission
        .witness
        .as_ref()
        .filter(|witness| witness.installation_scope_commitment.is_some());
    if kind.synthetic().is_some() {
        scoped.is_none()
    } else {
        scoped.is_some_and(|witness| {
            is_installation_comment_adapter(&witness.provider.adapter_commitment, kind.fixture())
        })
    }
}

fn persist_live_comment_outcome(
    tx: &rusqlite::Transaction<'_>,
    action: GitHubCommentContinuationAction<'_>,
    admission: &LiveRecoveryAdmission,
    outcome: SideEffectOutcomeReference,
    kind: LiveCommentProofKind,
) -> Result<EventId, WorkflowOsError> {
    persist_bound_live_comment_outcome(tx, action, admission, outcome, kind, None)
}

fn persist_bound_live_comment_outcome(
    tx: &rusqlite::Transaction<'_>,
    action: GitHubCommentContinuationAction<'_>,
    admission: &LiveRecoveryAdmission,
    outcome: SideEffectOutcomeReference,
    kind: LiveCommentProofKind,
    observation: Option<&SpecContentHash>,
) -> Result<EventId, WorkflowOsError> {
    if !comment_proof_source_matches(admission, kind) {
        return Err(not_dispatchable());
    }
    let history = SqliteStateBackend::read_events_with_connection(tx, action.run)?;
    let run = WorkflowRun::rehydrate(&history)?;
    let identity = &run.snapshot.identity;
    let effect = read_live_effect_record(tx, action.side_effect)?;
    let admission_event = admission.event.committed_result_cursor().event_id();
    if effect.side_effect_id() != action.side_effect
        || effect.run_id() != action.run
        || effect.workflow_id() != &identity.workflow_id
        || effect.workflow_version() != &identity.workflow_version
        || effect.schema_version() != &identity.schema_version
        || effect.spec_hash() != &identity.spec_content_hash
        || effect.step_id() != Some(action.step)
        || effect.actor() != Some(action.original_actor)
        || effect.target().reference() != action.target.reference()
        || effect.idempotency().key() != action.idempotency_key
    {
        return Err(not_dispatchable());
    }
    if let Some((event, existing_kind)) = find_live_comment_proof(&admission.event, &history)? {
        if effect.outcome_reference() != Some(&outcome)
            || (kind != LiveCommentProofKind::Lookup && existing_kind != kind)
        {
            return Err(not_dispatchable());
        }
        verify_live_comment_proof(tx, admission, &effect, event, existing_kind)?;
        return Ok(event.event_id.clone());
    }
    let id = live_comment_proof_id(&admission.event, kind)?;
    verify_live_comment_attempt(admission, &effect)?;
    let last = history.last().ok_or_else(corrupt)?;
    let now = continuity_store::observe_continuity_trusted_time()?.observed_at();
    if now < last.timestamp {
        return Err(not_dispatchable());
    }
    let transition = transition_side_effect_to_completed(SideEffectCompleteTransitionInput {
        prior_record: &effect,
        transitioned_at: now,
        outcome_reference: outcome,
        summary: None,
        additional_references: vec![SideEffectReference::new(
            SideEffectReferenceKind::WorkflowEvent,
            admission_event.as_str(),
        )?],
        evidence_reference_count: 0,
    })?;
    let updated = transition.record();
    let changed = tx.execute(
        "UPDATE side_effect_records SET payload=?1 WHERE side_effect_id=?2 AND run_id=?3 AND workflow_id=?4",
        params![encode_json(updated, "live comment outcome")?, action.side_effect.as_str(),
            action.run.as_str(), identity.workflow_id.as_str()],
    ).map_err(|_| storage_error("write_failed"))?;
    if changed != 1 {
        return Err(corrupt());
    }
    append_event_and_project_snapshot(
        tx,
        &WorkflowRunEvent {
            sequence_number: EventSequenceNumber::new(
                last.sequence_number
                    .get()
                    .checked_add(1)
                    .ok_or_else(corrupt)?,
            )?,
            event_id: id.clone(),
            timestamp: now,
            run_id: identity.run_id.clone(),
            workflow_id: identity.workflow_id.clone(),
            schema_version: identity.schema_version.clone(),
            workflow_version: identity.workflow_version.clone(),
            spec_content_hash: identity.spec_content_hash.clone(),
            correlation_id: updated.correlation_id().cloned(),
            actor: None,
            idempotency_key: Some(live_comment_proof_key(
                admission,
                updated,
                kind,
                observation,
            )?),
            kind: WorkflowRunEventKind::SideEffectCompleted(Box::new(transition.event().clone())),
        },
    )?;
    Ok(id)
}

fn insert_reservation(
    transaction: &rusqlite::Transaction<'_>,
    run_id: &crate::WorkflowRunId,
    record: &DispatchReservationRecord,
    event: &DispatchAdmissionProjectionEvent,
) -> Result<(), WorkflowOsError> {
    let (seconds, nanos) = timestamp_parts(record.committed_at);
    let sequence = i64::try_from(event.committed_result_cursor().sequence_number().get())
        .map_err(|_| corrupt())?;
    transaction
        .execute(
            "INSERT INTO dispatch_reservations
             (attempt_id,window_id,run_id,operation_id,receipt_id,request_commitment,
              reservation_commitment,result_event_id,result_sequence,committed_seconds,
              committed_nanos,record_json)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)",
            params![
                record.attempt_id.as_str(),
                record.window_id.as_str(),
                run_id.as_str(),
                record.operation_id.as_str(),
                record.receipt_id.as_str(),
                record.request_commitment.as_str(),
                record.reservation_commitment.as_str(),
                event.committed_result_cursor().event_id().as_str(),
                sequence,
                seconds,
                nanos,
                encode_json(record, "dispatch reservation")?
            ],
        )
        .map_err(|_| storage_error("write_failed"))?;
    Ok(())
}

fn insert_projection(
    transaction: &rusqlite::Transaction<'_>,
    record: &DispatchReservationRecord,
    binding: &ProjectionBinding,
) -> Result<(), WorkflowOsError> {
    transaction
        .execute(
            "INSERT INTO dispatch_reservation_projection_bindings
             (operation_id,receipt_id,projection_commitment,snapshot_commitment,binding_json)
             VALUES (?1,?2,?3,?4,?5)",
            params![
                record.operation_id.as_str(),
                record.receipt_id.as_str(),
                binding.event.projection_commitment().as_str(),
                binding.snapshot_commitment.as_str(),
                encode_json(binding, "dispatch reservation projection")?
            ],
        )
        .map_err(|_| storage_error("write_failed"))?;
    Ok(())
}

fn read_existing(
    connection: &rusqlite::Connection,
    operation_id: &str,
) -> Result<Option<(DispatchReservationRecord, ProjectionBinding)>, WorkflowOsError> {
    let record_row = connection
        .query_row(
            "SELECT operation_id,receipt_id,attempt_id,window_id,request_commitment,
                    reservation_commitment,result_event_id,result_sequence,record_json
             FROM dispatch_reservations WHERE operation_id=?1",
            params![operation_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, String>(6)?,
                    row.get::<_, i64>(7)?,
                    row.get::<_, String>(8)?,
                ))
            },
        )
        .optional()
        .map_err(|_| storage_error("read_failed"))?;
    let Some(record_row) = record_row else {
        return Ok(None);
    };
    let binding_row = connection
        .query_row(
            "SELECT operation_id,receipt_id,projection_commitment,snapshot_commitment,binding_json
             FROM dispatch_reservation_projection_bindings WHERE operation_id=?1",
            params![operation_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        )
        .optional()
        .map_err(|_| storage_error("read_failed"))?
        .ok_or_else(corrupt)?;
    let record: DispatchReservationRecord =
        serde_json::from_str(&record_row.8).map_err(|_| corrupt())?;
    let binding: ProjectionBinding = serde_json::from_str(&binding_row.4).map_err(|_| corrupt())?;
    let sequence = i64::try_from(
        binding
            .event
            .committed_result_cursor()
            .sequence_number()
            .get(),
    )
    .map_err(|_| corrupt())?;
    if record_row.0 != record.operation_id.as_str()
        || record_row.1 != record.receipt_id.as_str()
        || record_row.2 != record.attempt_id.as_str()
        || record_row.3 != record.window_id.as_str()
        || record_row.4 != record.request_commitment.as_str()
        || record_row.5 != record.reservation_commitment.as_str()
        || record_row.6 != binding.event.committed_result_cursor().event_id().as_str()
        || record_row.7 != sequence
        || binding_row.0 != record.operation_id.as_str()
        || binding_row.1 != record.receipt_id.as_str()
        || binding_row.2 != binding.event.projection_commitment().as_str()
        || binding_row.3 != binding.snapshot_commitment.as_str()
    {
        return Err(corrupt());
    }
    Ok(Some((record, binding)))
}

fn validate_existing_projection(
    connection: &rusqlite::Connection,
    record: &DispatchReservationRecord,
    binding: &ProjectionBinding,
) -> Result<(), WorkflowOsError> {
    if binding.event.receipt_id() != &record.receipt_id
        || binding.event.attempt_id() != &record.attempt_id
        || binding.event.reservation_commitment() != &record.reservation_commitment
    {
        return Err(corrupt());
    }
    if let Some(witness) = &record.live_witness {
        if live_reservation_commitment(witness)? != record.reservation_commitment {
            return Err(corrupt());
        }
    }
    let durable_event_payload = connection
        .query_row(
            "SELECT payload FROM events WHERE event_id=?1",
            params![binding.event.committed_result_cursor().event_id().as_str()],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_| storage_error("read_failed"))?
        .ok_or_else(corrupt)?;
    let durable_event: WorkflowRunEvent =
        serde_json::from_str(&durable_event_payload).map_err(|_| corrupt())?;
    let durable_projection = match &durable_event.kind {
        WorkflowRunEventKind::AuthorizedExecutionAttemptDispatchAdmitted(value) => value.as_ref(),
        _ => return Err(corrupt()),
    };
    if durable_projection != &binding.event
        || durable_event.timestamp != record.committed_at
        || durable_event.sequence_number
            != binding.event.committed_result_cursor().sequence_number()
        || durable_event.event_id != *binding.event.committed_result_cursor().event_id()
    {
        return Err(corrupt());
    }

    let history =
        SqliteStateBackend::read_events_with_connection(connection, &durable_event.run_id)
            .map_err(|_| corrupt())?;
    let result_index = usize::try_from(
        binding
            .event
            .committed_result_cursor()
            .sequence_number()
            .get()
            .checked_sub(1)
            .ok_or_else(corrupt)?,
    )
    .map_err(|_| corrupt())?;
    let expected_index = usize::try_from(
        binding
            .event
            .expected_input_cursor()
            .sequence_number()
            .get()
            .checked_sub(1)
            .ok_or_else(corrupt)?,
    )
    .map_err(|_| corrupt())?;
    let expected_event = history.get(expected_index).ok_or_else(corrupt)?;
    let result_event = history.get(result_index).ok_or_else(corrupt)?;
    if expected_event.event_id != *binding.event.expected_input_cursor().event_id()
        || result_event != &durable_event
    {
        return Err(corrupt());
    }

    let projected_snapshot = WorkflowRun::rehydrate(&history[..=result_index])?.snapshot;
    if projected_snapshot.last_dispatch_admission_projection
        != Some(
            crate::dispatch_reservation::DispatchAdmissionProjectionSnapshot::from_event(
                &binding.event,
            ),
        )
        || snapshot_commitment(&encode_json(&projected_snapshot, "snapshot")?)
            != binding.snapshot_commitment
    {
        return Err(corrupt());
    }

    let current_row: (i64, String, String, String) = connection
        .query_row(
            "SELECT last_sequence_number,last_event_id,snapshot_commitment,payload
             FROM snapshots WHERE run_id=?1",
            params![durable_event.run_id.as_str()],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .map_err(|_| corrupt())?;
    let current_snapshot: WorkflowRunSnapshot =
        super::decode_json(&current_row.3, "snapshot").map_err(|_| corrupt())?;
    let derived_snapshot = WorkflowRun::rehydrate(&history)?.snapshot;
    let current_sequence =
        i64::try_from(current_snapshot.last_sequence_number.get()).map_err(|_| corrupt())?;
    if current_snapshot != derived_snapshot
        || current_row.0 != current_sequence
        || current_row.1 != current_snapshot.last_event_id.as_str()
        || current_row.2
            != snapshot_commitment(&encode_json(&current_snapshot, "snapshot")?).as_str()
    {
        return Err(corrupt());
    }
    Ok(())
}

fn reconcile_committed_reservation(
    backend: &SqliteStateBackend,
    operation_id: &str,
    request_commitment: &crate::SpecContentHash,
    receipt_id: &crate::dispatch_reservation::DispatchReservationReceiptId,
) -> Result<DispatchReservationOutcome, WorkflowOsError> {
    let connection = backend.connection()?;
    match read_existing(&connection, operation_id)? {
        Some((record, binding))
            if record.request_commitment == *request_commitment
                && record.receipt_id == *receipt_id =>
        {
            validate_existing_projection(&connection, &record, &binding)?;
            Ok(
                DispatchReservationOutcome::CommittedButCapabilityUnavailable {
                    receipt: replay_receipt(&record, &binding),
                },
            )
        }
        Some(_) => Err(reservation_error(
            WorkflowOsErrorKind::Security,
            "idempotency_conflict",
            "dispatch reservation identity was reused with different content",
        )),
        None => Err(storage_error("commit_ambiguous")),
    }
}

fn replay_receipt(
    record: &DispatchReservationRecord,
    binding: &ProjectionBinding,
) -> DispatchReservationReplayReceipt {
    DispatchReservationReplayReceipt::new(
        record.receipt_id.clone(),
        record.reservation_commitment.clone(),
        binding.event.committed_result_cursor().clone(),
    )
}

pub(super) fn validate_dispatch_reservation_binding(
    connection: &rusqlite::Connection,
    attempt_id: &crate::AuthorizedExecutionAttemptId,
    supplied: &DispatchReservationBinding,
) -> Result<(), WorkflowOsError> {
    let operation_id = connection
        .query_row(
            "SELECT operation_id FROM dispatch_reservations WHERE attempt_id=?1",
            params![attempt_id.as_str()],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_| storage_error("read_failed"))?
        .ok_or_else(dispatch_binding_missing)?;
    let (record, projection) = read_existing(connection, &operation_id)?.ok_or_else(corrupt)?;
    validate_existing_projection(connection, &record, &projection)?;
    if record.attempt_id != *attempt_id
        || record.receipt_id != supplied.receipt_id
        || record.reservation_commitment != supplied.reservation_commitment
        || projection.event.receipt_id() != &supplied.receipt_id
        || projection.event.attempt_id() != attempt_id
        || projection.event.reservation_commitment() != &supplied.reservation_commitment
        || projection.event.committed_result_cursor() != &supplied.admission_cursor
    {
        return Err(dispatch_binding_invalid());
    }
    Ok(())
}

fn short_hash(value: &str) -> String {
    use sha2::{Digest, Sha256};
    use std::fmt::Write as _;

    let digest = Sha256::digest(value.as_bytes());
    let mut rendered = String::with_capacity(24);
    for byte in &digest[..12] {
        let _ = write!(rendered, "{byte:02x}");
    }
    rendered
}

fn not_dispatchable() -> WorkflowOsError {
    reservation_error(
        WorkflowOsErrorKind::InvalidState,
        "attempt_not_dispatchable",
        "authorized attempt is not dispatchable",
    )
}

fn dispatch_binding_missing() -> WorkflowOsError {
    reservation_error(
        WorkflowOsErrorKind::Security,
        "binding_missing",
        "dispatch reservation binding is required",
    )
}

fn dispatch_binding_invalid() -> WorkflowOsError {
    reservation_error(
        WorkflowOsErrorKind::Security,
        "binding_invalid",
        "dispatch reservation binding is invalid",
    )
}

fn corrupt() -> WorkflowOsError {
    reservation_error(
        WorkflowOsErrorKind::InvalidState,
        "recovery_required",
        "dispatch reservation state requires recovery",
    )
}

fn storage_error(suffix: &'static str) -> WorkflowOsError {
    reservation_error(
        WorkflowOsErrorKind::InvalidState,
        suffix,
        "dispatch reservation storage operation failed",
    )
}
