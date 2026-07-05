use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    preflight_adapter_write, ActorId, AdapterId, AdapterKind, AdapterWriteCapability,
    AdapterWritePolicyDecision, AdapterWritePreflightDecision, AdapterWritePreflightRequest,
    AdapterWritePreflightRequestDefinition, AdapterWriteTargetKind, CorrelationId, IdempotencyKey,
    IntegrationId, RedactionMetadata, SchemaVersion, SideEffectAuthority,
    SideEffectAuthorityDecision, SideEffectCapability, SideEffectId, SideEffectIdempotencyBinding,
    SideEffectIdempotencyScope, SideEffectLifecycleState, SideEffectLifecycleTransitionResult,
    SideEffectOutcomeReference, SideEffectOutcomeReferenceKind, SideEffectRecord,
    SideEffectRecordDefinition, SideEffectRecordStore, SideEffectReference, SideEffectSensitivity,
    SideEffectTargetKind, SideEffectTargetReference, SideEffectWorkflowEvent,
    SideEffectWorkflowEventDefinition, SkillId, SkillVersion, SpecContentHash, StepId, Timestamp,
    WorkflowId, WorkflowOsError, WorkflowRun, WorkflowRunId, WorkflowVersion,
};

const GITHUB_NAME_MAX_BYTES: usize = 100;
const GITHUB_COMMENT_BODY_MAX_BYTES: usize = 4 * 1024;
const GITHUB_WRITE_SUMMARY_MAX_BYTES: usize = 512;
const GITHUB_PROVIDER_REFERENCE_MAX_BYTES: usize = 256;
const GITHUB_WRITE_REDACTION_FIELD_MAX_BYTES: usize = 128;
const GITHUB_WRITE_REDACTION_REASON_MAX_BYTES: usize = 512;
const GITHUB_WRITE_REDACTION_MAX_ENTRIES: usize = 64;
const GITHUB_FIXTURE_REFERENCE_MAX_BYTES: usize = 128;

/// Execution mode vocabulary for future GitHub pull request comment writes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHubPullRequestCommentWriteMode {
    /// Fixture-only validation. No provider call is allowed.
    Fixture,
    /// Dry-run construction. No provider call is allowed.
    DryRun,
    /// Future explicitly approved live sandbox mode.
    LiveSandbox,
}

/// Outcome vocabulary for a future GitHub pull request comment write response.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHubPullRequestCommentWriteOutcome {
    /// Request was validated in fixture mode.
    FixtureValidated,
    /// Request was validated in dry-run mode.
    DryRunValidated,
    /// Future provider call succeeded.
    ProviderSucceeded,
    /// Future provider call failed with a classified error.
    ProviderFailed,
}

/// Bounded GitHub pull request target for a future comment write.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GitHubPullRequestCommentTarget {
    owner: String,
    repository: String,
    pull_request_number: u64,
}

impl GitHubPullRequestCommentTarget {
    /// Creates a validated GitHub pull request target.
    ///
    /// # Errors
    ///
    /// Returns an error when owner, repository, or pull request number is unsafe.
    pub fn new(
        owner: impl Into<String>,
        repository: impl Into<String>,
        pull_request_number: u64,
    ) -> Result<Self, WorkflowOsError> {
        let target = Self {
            owner: owner.into(),
            repository: repository.into(),
            pull_request_number,
        };
        target.validate()?;
        Ok(target)
    }

    /// Validates this target.
    ///
    /// # Errors
    ///
    /// Returns an error when this target is invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_github_name("owner", &self.owner)?;
        validate_github_name("repository", &self.repository)?;
        if self.pull_request_number == 0 {
            return Err(github_write_error(
                "github_pr_comment_write.target.pull_request_number",
                "GitHub pull request number must be greater than zero",
            ));
        }
        Ok(())
    }

    /// Returns the GitHub owner.
    #[must_use]
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// Returns the GitHub repository name.
    #[must_use]
    pub fn repository(&self) -> &str {
        &self.repository
    }

    /// Returns the pull request number.
    #[must_use]
    pub const fn pull_request_number(&self) -> u64 {
        self.pull_request_number
    }

    /// Returns the canonical bounded target reference.
    #[must_use]
    pub fn reference(&self) -> String {
        format!(
            "github/{}/{}/pull/{}",
            self.owner, self.repository, self.pull_request_number
        )
    }
}

impl fmt::Debug for GitHubPullRequestCommentTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentTarget")
            .field("owner", &"[REDACTED]")
            .field("repository", &"[REDACTED]")
            .field("pull_request_number", &self.pull_request_number)
            .finish()
    }
}

impl<'de> Deserialize<'de> for GitHubPullRequestCommentTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            owner: String,
            repository: String,
            pull_request_number: u64,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(wire.owner, wire.repository, wire.pull_request_number)
            .map_err(serde::de::Error::custom)
    }
}

/// Public definition used to create a GitHub PR comment write request.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GitHubPullRequestCommentWriteRequestDefinition {
    /// Adapter implementation identity.
    pub adapter_id: AdapterId,
    /// Configured integration identity.
    pub integration_id: IntegrationId,
    /// Correlation ID for the future write boundary.
    pub correlation_id: CorrelationId,
    /// Workflow identity.
    pub workflow_id: WorkflowId,
    /// Workflow version.
    pub workflow_version: WorkflowVersion,
    /// Schema version.
    pub schema_version: SchemaVersion,
    /// Spec hash for the run.
    pub spec_hash: SpecContentHash,
    /// Run ID.
    pub run_id: WorkflowRunId,
    /// Optional step ID.
    pub step_id: Option<StepId>,
    /// Actor or system actor requesting the write.
    pub actor: ActorId,
    /// Target pull request.
    pub target: GitHubPullRequestCommentTarget,
    /// Bounded comment body.
    pub comment_body: String,
    /// Bounded purpose or impact summary.
    pub summary: String,
    /// Proposed `SideEffect` ID.
    pub side_effect_id: SideEffectId,
    /// Idempotency key.
    pub idempotency_key: IdempotencyKey,
    /// Future write mode.
    pub mode: GitHubPullRequestCommentWriteMode,
    /// Preflight request that must classify this write before provider invocation.
    pub preflight: AdapterWritePreflightRequest,
    /// Sensitivity assigned to this request.
    pub sensitivity: SideEffectSensitivity,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
}

impl fmt::Debug for GitHubPullRequestCommentWriteRequestDefinition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentWriteRequestDefinition")
            .field("adapter_id", &self.adapter_id)
            .field("integration_id", &self.integration_id)
            .field("correlation_id", &"[REDACTED]")
            .field("workflow_id", &self.workflow_id)
            .field("workflow_version", &self.workflow_version)
            .field("schema_version", &self.schema_version)
            .field("spec_hash", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("step_id", &self.step_id.as_ref().map(|_| "[REDACTED]"))
            .field("actor", &"[REDACTED]")
            .field("target", &self.target)
            .field("comment_body", &"[REDACTED]")
            .field("summary", &"[REDACTED]")
            .field("side_effect_id", &"[REDACTED]")
            .field("idempotency_key", &"[REDACTED]")
            .field("mode", &self.mode)
            .field("preflight", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .finish()
    }
}

/// Validated, redaction-safe GitHub PR comment write request model.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GitHubPullRequestCommentWriteRequest {
    adapter_id: AdapterId,
    integration_id: IntegrationId,
    correlation_id: CorrelationId,
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    schema_version: SchemaVersion,
    spec_hash: SpecContentHash,
    run_id: WorkflowRunId,
    step_id: Option<StepId>,
    actor: ActorId,
    target: GitHubPullRequestCommentTarget,
    comment_body: String,
    summary: String,
    side_effect_id: SideEffectId,
    idempotency_key: IdempotencyKey,
    mode: GitHubPullRequestCommentWriteMode,
    preflight: AdapterWritePreflightRequest,
    sensitivity: SideEffectSensitivity,
    redaction: RedactionMetadata,
}

impl GitHubPullRequestCommentWriteRequest {
    /// Creates a validated GitHub PR comment write request model.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error when unsafe or incomplete.
    pub fn new(
        definition: GitHubPullRequestCommentWriteRequestDefinition,
    ) -> Result<Self, WorkflowOsError> {
        let request = Self {
            adapter_id: definition.adapter_id,
            integration_id: definition.integration_id,
            correlation_id: definition.correlation_id,
            workflow_id: definition.workflow_id,
            workflow_version: definition.workflow_version,
            schema_version: definition.schema_version,
            spec_hash: definition.spec_hash,
            run_id: definition.run_id,
            step_id: definition.step_id,
            actor: definition.actor,
            target: definition.target,
            comment_body: definition.comment_body,
            summary: definition.summary,
            side_effect_id: definition.side_effect_id,
            idempotency_key: definition.idempotency_key,
            mode: definition.mode,
            preflight: definition.preflight,
            sensitivity: definition.sensitivity,
            redaction: definition.redaction,
        };
        request.validate()?;
        Ok(request)
    }

    /// Validates this request model without calling GitHub.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error when invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.target.validate()?;
        validate_comment_body(&self.comment_body)?;
        validate_summary("summary", &self.summary)?;
        self.preflight.validate()?;
        validate_redaction_metadata(&self.redaction)?;
        validate_preflight_matches_request(self)
    }

    /// Returns the requested mode.
    #[must_use]
    pub const fn mode(&self) -> GitHubPullRequestCommentWriteMode {
        self.mode
    }

    /// Returns the target.
    #[must_use]
    pub const fn target(&self) -> &GitHubPullRequestCommentTarget {
        &self.target
    }

    /// Returns the bounded comment body.
    #[must_use]
    pub fn comment_body(&self) -> &str {
        &self.comment_body
    }

    /// Returns the bounded summary.
    #[must_use]
    pub fn summary(&self) -> &str {
        &self.summary
    }

    /// Returns the proposed `SideEffect` ID.
    #[must_use]
    pub const fn side_effect_id(&self) -> &SideEffectId {
        &self.side_effect_id
    }

    /// Returns the idempotency key.
    #[must_use]
    pub const fn idempotency_key(&self) -> &IdempotencyKey {
        &self.idempotency_key
    }

    /// Returns the preflight request.
    #[must_use]
    pub const fn preflight(&self) -> &AdapterWritePreflightRequest {
        &self.preflight
    }

    /// Returns the adapter ID.
    #[must_use]
    pub const fn adapter_id(&self) -> &AdapterId {
        &self.adapter_id
    }

    /// Returns the integration ID.
    #[must_use]
    pub const fn integration_id(&self) -> &IntegrationId {
        &self.integration_id
    }

    /// Returns the correlation ID.
    #[must_use]
    pub const fn correlation_id(&self) -> &CorrelationId {
        &self.correlation_id
    }

    /// Returns the workflow ID.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the workflow version.
    #[must_use]
    pub const fn workflow_version(&self) -> &WorkflowVersion {
        &self.workflow_version
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> &SchemaVersion {
        &self.schema_version
    }

    /// Returns the spec hash.
    #[must_use]
    pub const fn spec_hash(&self) -> &SpecContentHash {
        &self.spec_hash
    }

    /// Returns the workflow run ID.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the optional step ID.
    #[must_use]
    pub const fn step_id(&self) -> Option<&StepId> {
        self.step_id.as_ref()
    }

    /// Returns the actor requesting the write.
    #[must_use]
    pub const fn actor(&self) -> &ActorId {
        &self.actor
    }

    /// Returns sensitivity assigned to this request.
    #[must_use]
    pub const fn sensitivity(&self) -> SideEffectSensitivity {
        self.sensitivity
    }

    /// Returns redaction metadata.
    #[must_use]
    pub const fn redaction(&self) -> &RedactionMetadata {
        &self.redaction
    }

    /// Returns whether this model authorizes provider calls.
    #[must_use]
    pub const fn provider_call_allowed(&self) -> bool {
        false
    }

    /// Returns whether this model authorizes workflow event appends.
    #[must_use]
    pub const fn workflow_event_append_allowed(&self) -> bool {
        false
    }

    /// Returns whether this model authorizes `SideEffect` lifecycle transitions.
    #[must_use]
    pub const fn side_effect_lifecycle_transition_allowed(&self) -> bool {
        false
    }
}

impl fmt::Debug for GitHubPullRequestCommentWriteRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentWriteRequest")
            .field("adapter_id", &self.adapter_id)
            .field("integration_id", &self.integration_id)
            .field("correlation_id", &"[REDACTED]")
            .field("workflow_id", &self.workflow_id)
            .field("workflow_version", &self.workflow_version)
            .field("schema_version", &self.schema_version)
            .field("spec_hash", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("step_id", &self.step_id.as_ref().map(|_| "[REDACTED]"))
            .field("actor", &"[REDACTED]")
            .field("target", &self.target)
            .field("comment_body", &"[REDACTED]")
            .field("summary", &"[REDACTED]")
            .field("side_effect_id", &"[REDACTED]")
            .field("idempotency_key", &"[REDACTED]")
            .field("mode", &self.mode)
            .field("preflight", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for GitHubPullRequestCommentWriteRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definition = GitHubPullRequestCommentWriteRequestDefinition::deserialize(deserializer)?;
        Self::new(definition).map_err(serde::de::Error::custom)
    }
}

/// Public definition used to create a GitHub PR comment write response model.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GitHubPullRequestCommentWriteResponseDefinition {
    /// Correlation ID for the response boundary.
    pub correlation_id: CorrelationId,
    /// Requested write mode.
    pub mode: GitHubPullRequestCommentWriteMode,
    /// Outcome.
    pub outcome: GitHubPullRequestCommentWriteOutcome,
    /// Optional provider comment reference for future live success.
    pub provider_comment_reference: Option<String>,
    /// Optional classified provider error code for future failure.
    pub provider_error_code: Option<String>,
    /// Bounded redacted summary.
    pub summary: String,
    /// Sensitivity assigned to this response.
    pub sensitivity: SideEffectSensitivity,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
}

impl fmt::Debug for GitHubPullRequestCommentWriteResponseDefinition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentWriteResponseDefinition")
            .field("correlation_id", &"[REDACTED]")
            .field("mode", &self.mode)
            .field("outcome", &self.outcome)
            .field(
                "provider_comment_reference",
                &self
                    .provider_comment_reference
                    .as_ref()
                    .map(|_| "[REDACTED]"),
            )
            .field("provider_error_code", &self.provider_error_code)
            .field("summary", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .finish()
    }
}

/// Validated, redaction-safe GitHub PR comment write response model.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GitHubPullRequestCommentWriteResponse {
    correlation_id: CorrelationId,
    mode: GitHubPullRequestCommentWriteMode,
    outcome: GitHubPullRequestCommentWriteOutcome,
    provider_comment_reference: Option<String>,
    provider_error_code: Option<String>,
    summary: String,
    sensitivity: SideEffectSensitivity,
    redaction: RedactionMetadata,
}

/// Validated GitHub PR comment write request with executed preflight.
#[derive(Clone, Eq, PartialEq)]
pub struct GitHubPullRequestCommentPreflightedWrite {
    request: GitHubPullRequestCommentWriteRequest,
    preflight_decision: AdapterWritePreflightDecision,
}

/// Public definition used to create GitHub PR comment fixture input.
#[derive(Clone, Eq, PartialEq)]
pub struct GitHubPullRequestCommentFixtureDefinition {
    /// Expected target.
    pub target: GitHubPullRequestCommentTarget,
    /// Expected `SideEffect` ID.
    pub side_effect_id: SideEffectId,
    /// Expected idempotency key.
    pub idempotency_key: IdempotencyKey,
    /// Fixture mode.
    pub mode: GitHubPullRequestCommentWriteMode,
    /// Optional fixture-only reference.
    pub fixture_reference: Option<String>,
    /// Bounded fixture summary.
    pub summary: String,
    /// Sensitivity assigned to fixture response.
    pub sensitivity: SideEffectSensitivity,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
}

impl fmt::Debug for GitHubPullRequestCommentFixtureDefinition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentFixtureDefinition")
            .field("target", &self.target)
            .field("side_effect_id", &"[REDACTED]")
            .field("idempotency_key", &"[REDACTED]")
            .field("mode", &self.mode)
            .field(
                "fixture_reference",
                &self.fixture_reference.as_ref().map(|_| "[REDACTED]"),
            )
            .field("summary", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .finish()
    }
}

/// Fixture input for validating a preflighted GitHub PR comment write without provider calls.
#[derive(Clone, Eq, PartialEq)]
pub struct GitHubPullRequestCommentFixture {
    target: GitHubPullRequestCommentTarget,
    side_effect_id: SideEffectId,
    idempotency_key: IdempotencyKey,
    mode: GitHubPullRequestCommentWriteMode,
    fixture_reference: Option<String>,
    summary: String,
    sensitivity: SideEffectSensitivity,
    redaction: RedactionMetadata,
}

/// Explicit input for composing a proposed `SideEffectRecord` from a preflighted GitHub PR comment.
///
/// This input provides only context not safely derivable from the already
/// validated write request. It does not authorize provider calls, persistence,
/// workflow event appends, audit events, report artifacts, or CLI behavior.
#[derive(Clone, Eq, PartialEq)]
pub struct GitHubPullRequestCommentSideEffectRecordInput {
    /// Created timestamp for the proposed record.
    pub created_at: Timestamp,
    /// Optional skill identity associated with the proposal.
    pub skill_id: Option<SkillId>,
    /// Optional skill version associated with the proposal.
    pub skill_version: Option<SkillVersion>,
    /// Optional system actor when the proposal is system-generated.
    pub system_actor: Option<ActorId>,
    /// Additional stable references to include on the proposed record.
    pub additional_references: Vec<SideEffectReference>,
    /// Optional bounded summary override.
    pub summary_override: Option<String>,
    /// Optional sensitivity override. The helper uses the conservative maximum
    /// of this value and the request sensitivity.
    pub sensitivity: Option<SideEffectSensitivity>,
}

/// Explicit input for composing a GitHub PR comment write attempt boundary.
///
/// This input authorizes store-backed local orchestration only. It does not
/// authorize provider calls, workflow event appends, audit emission, report
/// artifact writes, file writes, CLI output, or external side effects.
pub struct GitHubPullRequestCommentWriteAttemptOrchestrationInput<'a> {
    /// Optional fixture/dry-run response from prior validation.
    pub fixture_response: Option<&'a GitHubPullRequestCommentWriteResponse>,
    /// Input for composing and persisting the proposed `SideEffectRecord`.
    pub record_input: GitHubPullRequestCommentSideEffectRecordInput,
    /// Workflow run whose event history proves approval linkage when required.
    pub approval_run: Option<&'a WorkflowRun>,
    /// Whether approval linkage must be validated even when no approval
    /// reference is present on the proposed side effect.
    pub require_approval_linkage: bool,
    /// Timestamp for the attempted lifecycle transition.
    pub transitioned_at: Timestamp,
    /// Optional bounded non-secret attempted transition summary.
    pub transition_summary: Option<String>,
    /// Stable non-secret references to add during the attempted transition.
    pub transition_references: Vec<SideEffectReference>,
    /// Count of associated evidence references.
    pub evidence_reference_count: u32,
}

impl fmt::Debug for GitHubPullRequestCommentWriteAttemptOrchestrationInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentWriteAttemptOrchestrationInput")
            .field("has_fixture_response", &self.fixture_response.is_some())
            .field("record_input", &self.record_input)
            .field("has_approval_run", &self.approval_run.is_some())
            .field("require_approval_linkage", &self.require_approval_linkage)
            .field("transitioned_at", &self.transitioned_at)
            .field(
                "transition_summary",
                &self.transition_summary.as_ref().map(|_| "[REDACTED]"),
            )
            .field(
                "transition_reference_count",
                &self.transition_references.len(),
            )
            .field("evidence_reference_count", &self.evidence_reference_count)
            .field("provider_call_allowed", &false)
            .field("workflow_event_append_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .finish()
    }
}

/// Bounded result for no-provider-call GitHub PR comment write attempt orchestration.
///
/// This result contains the persisted proposed record and the attempted
/// transition output for a caller to cite or append explicitly later. The
/// orchestration helper itself does not append workflow events or call
/// providers.
pub struct GitHubPullRequestCommentWriteAttemptOrchestrationResult {
    proposed_record: SideEffectRecord,
    attempted_transition: SideEffectLifecycleTransitionResult,
    approval_linkage: Option<crate::SideEffectApprovalLinkageFromStoreResult>,
}

impl GitHubPullRequestCommentWriteAttemptOrchestrationResult {
    /// Returns the persisted proposed record.
    #[must_use]
    pub const fn proposed_record(&self) -> &SideEffectRecord {
        &self.proposed_record
    }

    /// Returns the store-backed attempted transition result.
    #[must_use]
    pub const fn attempted_transition(&self) -> &SideEffectLifecycleTransitionResult {
        &self.attempted_transition
    }

    /// Returns approval linkage validation details when linkage was required.
    #[must_use]
    pub const fn approval_linkage(
        &self,
    ) -> Option<&crate::SideEffectApprovalLinkageFromStoreResult> {
        self.approval_linkage.as_ref()
    }

    /// Returns whether a provider call was performed by this helper.
    #[must_use]
    pub const fn provider_call_performed(&self) -> bool {
        false
    }

    /// Returns whether a workflow event was appended by this helper.
    #[must_use]
    pub const fn workflow_event_appended(&self) -> bool {
        false
    }

    /// Returns whether a report artifact was written by this helper.
    #[must_use]
    pub const fn report_artifact_written(&self) -> bool {
        false
    }

    /// Consumes the result into its parts.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        SideEffectRecord,
        SideEffectLifecycleTransitionResult,
        Option<crate::SideEffectApprovalLinkageFromStoreResult>,
    ) {
        (
            self.proposed_record,
            self.attempted_transition,
            self.approval_linkage,
        )
    }
}

impl fmt::Debug for GitHubPullRequestCommentWriteAttemptOrchestrationResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentWriteAttemptOrchestrationResult")
            .field(
                "proposed_lifecycle_state",
                &self.proposed_record.lifecycle_state(),
            )
            .field(
                "attempted_lifecycle_state",
                &self.attempted_transition.record().lifecycle_state(),
            )
            .field("has_approval_linkage", &self.approval_linkage.is_some())
            .field("provider_call_performed", &false)
            .field("workflow_event_appended", &false)
            .field("report_artifact_written", &false)
            .finish()
    }
}

/// Explicit no-provider-call GitHub PR comment write outcome.
///
/// `Completed` is reserved for local fixture/dry-run closure. It does not
/// indicate a live provider write. Live provider outcomes require a separate
/// provider-call phase.
#[derive(Clone, Eq, PartialEq)]
pub enum GitHubPullRequestCommentNoProviderOutcome {
    /// Mark the attempted write as locally completed from fixture/dry-run evidence.
    Completed {
        /// Stable local/fixture/dry-run outcome reference.
        outcome_reference: SideEffectOutcomeReference,
    },
    /// Mark the attempted write as failed with stable non-payload reason codes.
    Failed {
        /// Optional stable local/fixture/dry-run failure reference.
        outcome_reference: Option<SideEffectOutcomeReference>,
        /// Stable non-secret failure reason codes.
        reason_codes: Vec<String>,
    },
}

impl fmt::Debug for GitHubPullRequestCommentNoProviderOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Completed { outcome_reference } => formatter
                .debug_struct("Completed")
                .field("outcome_reference_kind", &outcome_reference.kind())
                .field("outcome_reference", &"[REDACTED]")
                .finish(),
            Self::Failed {
                outcome_reference,
                reason_codes,
            } => formatter
                .debug_struct("Failed")
                .field(
                    "outcome_reference_kind",
                    &outcome_reference
                        .as_ref()
                        .map(SideEffectOutcomeReference::kind),
                )
                .field("outcome_reference", &"[REDACTED]")
                .field("reason_code_count", &reason_codes.len())
                .finish(),
        }
    }
}

/// Explicit input for closing a GitHub PR comment write attempt without provider calls.
///
/// This input authorizes store-backed local lifecycle orchestration only. It
/// does not authorize provider calls, workflow event appends, audit emission,
/// report artifact writes, file writes, CLI output, or external side effects.
#[derive(Clone, Eq, PartialEq)]
pub struct GitHubPullRequestCommentNoProviderOutcomeOrchestrationInput {
    /// Timestamp for the outcome lifecycle transition.
    pub transitioned_at: Timestamp,
    /// Local fixture/dry-run completion or classified failure.
    pub outcome: GitHubPullRequestCommentNoProviderOutcome,
    /// Optional bounded non-secret transition summary.
    pub transition_summary: Option<String>,
    /// Stable non-secret references to add during the transition.
    pub transition_references: Vec<SideEffectReference>,
    /// Count of associated evidence references.
    pub evidence_reference_count: u32,
}

impl fmt::Debug for GitHubPullRequestCommentNoProviderOutcomeOrchestrationInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentNoProviderOutcomeOrchestrationInput")
            .field("transitioned_at", &self.transitioned_at)
            .field("outcome", &self.outcome)
            .field(
                "transition_summary",
                &self.transition_summary.as_ref().map(|_| "[REDACTED]"),
            )
            .field(
                "transition_reference_count",
                &self.transition_references.len(),
            )
            .field("evidence_reference_count", &self.evidence_reference_count)
            .field("provider_call_allowed", &false)
            .field("workflow_event_append_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .finish()
    }
}

/// Bounded result for no-provider-call GitHub PR comment write outcome orchestration.
pub struct GitHubPullRequestCommentNoProviderOutcomeOrchestrationResult {
    outcome_transition: SideEffectLifecycleTransitionResult,
}

impl GitHubPullRequestCommentNoProviderOutcomeOrchestrationResult {
    /// Returns the store-backed outcome transition result.
    #[must_use]
    pub const fn outcome_transition(&self) -> &SideEffectLifecycleTransitionResult {
        &self.outcome_transition
    }

    /// Returns whether a provider call was performed by this helper.
    #[must_use]
    pub const fn provider_call_performed(&self) -> bool {
        false
    }

    /// Returns whether a workflow event was appended by this helper.
    #[must_use]
    pub const fn workflow_event_appended(&self) -> bool {
        false
    }

    /// Returns whether a report artifact was written by this helper.
    #[must_use]
    pub const fn report_artifact_written(&self) -> bool {
        false
    }

    /// Consumes the result into its parts.
    #[must_use]
    pub fn into_parts(self) -> SideEffectLifecycleTransitionResult {
        self.outcome_transition
    }
}

impl fmt::Debug for GitHubPullRequestCommentNoProviderOutcomeOrchestrationResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentNoProviderOutcomeOrchestrationResult")
            .field(
                "outcome_lifecycle_state",
                &self.outcome_transition.record().lifecycle_state(),
            )
            .field("provider_call_performed", &false)
            .field("workflow_event_appended", &false)
            .field("report_artifact_written", &false)
            .finish()
    }
}

/// Expected workflow/run identity for projecting a persisted GitHub PR comment
/// proposed `SideEffectRecord` into a workflow event payload.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GitHubPullRequestCommentSideEffectEventContext {
    /// Expected workflow ID.
    pub workflow_id: WorkflowId,
    /// Expected workflow version.
    pub workflow_version: WorkflowVersion,
    /// Expected schema version.
    pub schema_version: SchemaVersion,
    /// Expected workflow spec hash.
    pub spec_hash: SpecContentHash,
    /// Expected run ID.
    pub run_id: WorkflowRunId,
}

impl fmt::Debug for GitHubPullRequestCommentSideEffectEventContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentSideEffectEventContext")
            .field("workflow_id", &self.workflow_id)
            .field("workflow_version", &self.workflow_version)
            .field("schema_version", &self.schema_version)
            .field("spec_hash", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .finish()
    }
}

impl fmt::Debug for GitHubPullRequestCommentSideEffectRecordInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentSideEffectRecordInput")
            .field("created_at", &self.created_at)
            .field("skill_id", &self.skill_id.as_ref().map(|_| "[REDACTED]"))
            .field(
                "skill_version",
                &self.skill_version.as_ref().map(|_| "[REDACTED]"),
            )
            .field(
                "system_actor",
                &self.system_actor.as_ref().map(|_| "[REDACTED]"),
            )
            .field(
                "additional_reference_count",
                &self.additional_references.len(),
            )
            .field(
                "summary_override",
                &self.summary_override.as_ref().map(|_| "[REDACTED]"),
            )
            .field("sensitivity", &self.sensitivity)
            .field("provider_call_allowed", &false)
            .field("workflow_event_append_allowed", &false)
            .field("side_effect_lifecycle_transition_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .finish()
    }
}

/// Input for building matching GitHub PR comment preflight definitions.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GitHubPullRequestCommentPreflightDefinitionInput {
    /// Target pull request.
    pub target: GitHubPullRequestCommentTarget,
    /// Proposed `SideEffect` ID.
    pub side_effect_id: SideEffectId,
    /// Idempotency key.
    pub idempotency_key: IdempotencyKey,
    /// Policy posture.
    pub policy_decision: AdapterWritePolicyDecision,
    /// Policy references.
    #[serde(default)]
    pub policy_references: Vec<SideEffectReference>,
    /// Approval references.
    #[serde(default)]
    pub approval_references: Vec<SideEffectReference>,
    /// Bounded preflight summary.
    pub summary: String,
    /// Sensitivity.
    pub sensitivity: SideEffectSensitivity,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
}

impl fmt::Debug for GitHubPullRequestCommentPreflightDefinitionInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentPreflightDefinitionInput")
            .field("target", &self.target)
            .field("side_effect_id", &"[REDACTED]")
            .field("idempotency_key", &"[REDACTED]")
            .field("policy_decision", &self.policy_decision)
            .field("policy_reference_count", &self.policy_references.len())
            .field("approval_reference_count", &self.approval_references.len())
            .field("summary", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .finish()
    }
}

impl GitHubPullRequestCommentWriteResponse {
    /// Creates a validated response model.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error when invalid.
    pub fn new(
        definition: GitHubPullRequestCommentWriteResponseDefinition,
    ) -> Result<Self, WorkflowOsError> {
        let response = Self {
            correlation_id: definition.correlation_id,
            mode: definition.mode,
            outcome: definition.outcome,
            provider_comment_reference: definition.provider_comment_reference,
            provider_error_code: definition.provider_error_code,
            summary: definition.summary,
            sensitivity: definition.sensitivity,
            redaction: definition.redaction,
        };
        response.validate()?;
        Ok(response)
    }

    /// Validates this response model.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error when invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_summary("response summary", &self.summary)?;
        validate_redaction_metadata(&self.redaction)?;
        validate_response_shape(self)
    }

    /// Returns the outcome.
    #[must_use]
    pub const fn outcome(&self) -> GitHubPullRequestCommentWriteOutcome {
        self.outcome
    }

    /// Returns the optional provider comment reference.
    #[must_use]
    pub fn provider_comment_reference(&self) -> Option<&str> {
        self.provider_comment_reference.as_deref()
    }

    /// Returns the optional provider error code.
    #[must_use]
    pub fn provider_error_code(&self) -> Option<&str> {
        self.provider_error_code.as_deref()
    }

    /// Returns the response mode.
    #[must_use]
    pub const fn mode(&self) -> GitHubPullRequestCommentWriteMode {
        self.mode
    }

    /// Returns the bounded summary.
    #[must_use]
    pub fn summary(&self) -> &str {
        &self.summary
    }

    /// Returns whether this model authorizes workflow event appends.
    #[must_use]
    pub const fn workflow_event_append_allowed(&self) -> bool {
        false
    }

    /// Returns whether this model authorizes `SideEffect` lifecycle transitions.
    #[must_use]
    pub const fn side_effect_lifecycle_transition_allowed(&self) -> bool {
        false
    }
}

impl GitHubPullRequestCommentFixture {
    /// Creates validated fixture input for a preflighted GitHub PR comment write.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error when fixture input is
    /// unsafe, incomplete, or would imply provider execution.
    pub fn new(
        definition: GitHubPullRequestCommentFixtureDefinition,
    ) -> Result<Self, WorkflowOsError> {
        let fixture = Self {
            target: definition.target,
            side_effect_id: definition.side_effect_id,
            idempotency_key: definition.idempotency_key,
            mode: definition.mode,
            fixture_reference: definition.fixture_reference,
            summary: definition.summary,
            sensitivity: definition.sensitivity,
            redaction: definition.redaction,
        };
        fixture.validate()?;
        Ok(fixture)
    }

    /// Validates this fixture input.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error when invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.target.validate()?;
        match self.mode {
            GitHubPullRequestCommentWriteMode::Fixture
            | GitHubPullRequestCommentWriteMode::DryRun => {}
            GitHubPullRequestCommentWriteMode::LiveSandbox => {
                return Err(github_write_error(
                    "github_pr_comment_fixture.mode.unsupported",
                    "GitHub PR comment fixture validation does not support live sandbox mode",
                ));
            }
        }
        if let Some(reference) = &self.fixture_reference {
            validate_fixture_reference(reference)?;
        }
        validate_summary("fixture summary", &self.summary)?;
        validate_redaction_metadata(&self.redaction)?;
        Ok(())
    }

    /// Returns the fixture target.
    #[must_use]
    pub const fn target(&self) -> &GitHubPullRequestCommentTarget {
        &self.target
    }

    /// Returns the fixture `SideEffect` ID.
    #[must_use]
    pub const fn side_effect_id(&self) -> &SideEffectId {
        &self.side_effect_id
    }

    /// Returns the fixture idempotency key.
    #[must_use]
    pub const fn idempotency_key(&self) -> &IdempotencyKey {
        &self.idempotency_key
    }

    /// Returns the fixture mode.
    #[must_use]
    pub const fn mode(&self) -> GitHubPullRequestCommentWriteMode {
        self.mode
    }

    /// Returns the optional fixture reference.
    #[must_use]
    pub fn fixture_reference(&self) -> Option<&str> {
        self.fixture_reference.as_deref()
    }

    /// Returns the bounded fixture summary.
    #[must_use]
    pub fn summary(&self) -> &str {
        &self.summary
    }

    /// Returns whether this fixture input authorizes provider calls.
    #[must_use]
    pub const fn provider_call_allowed(&self) -> bool {
        false
    }

    /// Returns whether this fixture input authorizes workflow event appends.
    #[must_use]
    pub const fn workflow_event_append_allowed(&self) -> bool {
        false
    }

    /// Returns whether this fixture input authorizes `SideEffect` lifecycle transitions.
    #[must_use]
    pub const fn side_effect_lifecycle_transition_allowed(&self) -> bool {
        false
    }

    /// Returns whether this fixture input authorizes report artifact writes.
    #[must_use]
    pub const fn report_artifact_write_allowed(&self) -> bool {
        false
    }
}

impl GitHubPullRequestCommentPreflightedWrite {
    /// Creates a preflighted GitHub PR comment write model without provider calls.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when preflight fails or does not
    /// match the request.
    pub fn new(request: GitHubPullRequestCommentWriteRequest) -> Result<Self, WorkflowOsError> {
        if request.mode() == GitHubPullRequestCommentWriteMode::LiveSandbox {
            return Err(github_write_error(
                "github_pr_comment_write.preflight.live_sandbox_unsupported",
                "GitHub PR comment live sandbox mode is not supported by preflight composition",
            ));
        }

        let preflight_decision = preflight_adapter_write(request.preflight())?;
        validate_preflight_decision_matches_request(&request, &preflight_decision)?;

        Ok(Self {
            request,
            preflight_decision,
        })
    }

    /// Returns the validated request.
    #[must_use]
    pub const fn request(&self) -> &GitHubPullRequestCommentWriteRequest {
        &self.request
    }

    /// Returns the executed preflight decision.
    #[must_use]
    pub const fn preflight_decision(&self) -> &AdapterWritePreflightDecision {
        &self.preflight_decision
    }

    /// Returns whether this composed model authorizes provider calls.
    #[must_use]
    pub const fn provider_call_allowed(&self) -> bool {
        false
    }

    /// Returns whether this composed model authorizes workflow event appends.
    #[must_use]
    pub const fn workflow_event_append_allowed(&self) -> bool {
        false
    }

    /// Returns whether this composed model authorizes `SideEffect` lifecycle transitions.
    #[must_use]
    pub const fn side_effect_lifecycle_transition_allowed(&self) -> bool {
        false
    }

    /// Returns whether this composed model authorizes report artifact writes.
    #[must_use]
    pub const fn report_artifact_write_allowed(&self) -> bool {
        false
    }
}

impl fmt::Debug for GitHubPullRequestCommentFixture {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentFixture")
            .field("target", &self.target)
            .field("side_effect_id", &"[REDACTED]")
            .field("idempotency_key", &"[REDACTED]")
            .field("mode", &self.mode)
            .field(
                "fixture_reference",
                &self.fixture_reference.as_ref().map(|_| "[REDACTED]"),
            )
            .field("summary", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .field("provider_call_allowed", &false)
            .field("workflow_event_append_allowed", &false)
            .field("side_effect_lifecycle_transition_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .finish()
    }
}

impl fmt::Debug for GitHubPullRequestCommentPreflightedWrite {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentPreflightedWrite")
            .field("request", &"[REDACTED]")
            .field("preflight_decision", &self.preflight_decision)
            .field("provider_call_allowed", &false)
            .field("workflow_event_append_allowed", &false)
            .field("side_effect_lifecycle_transition_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .finish()
    }
}

/// Validates a fixture-only GitHub PR comment write and returns a model response.
///
/// This helper accepts only a preflighted write value. It does not call GitHub,
/// read credentials, append events, transition `SideEffect` lifecycle state, or
/// write report artifacts.
///
/// # Errors
///
/// Returns a stable non-leaking error when fixture inputs do not match the
/// preflighted write or would imply provider execution.
pub fn validate_github_pr_comment_fixture_write(
    preflighted: &GitHubPullRequestCommentPreflightedWrite,
    fixture: &GitHubPullRequestCommentFixture,
) -> Result<GitHubPullRequestCommentWriteResponse, WorkflowOsError> {
    fixture.validate()?;
    validate_fixture_matches_preflighted_write(preflighted, fixture)?;

    GitHubPullRequestCommentWriteResponse::new(GitHubPullRequestCommentWriteResponseDefinition {
        correlation_id: preflighted.request().correlation_id.clone(),
        mode: fixture.mode,
        outcome: match fixture.mode {
            GitHubPullRequestCommentWriteMode::Fixture => {
                GitHubPullRequestCommentWriteOutcome::FixtureValidated
            }
            GitHubPullRequestCommentWriteMode::DryRun => {
                GitHubPullRequestCommentWriteOutcome::DryRunValidated
            }
            GitHubPullRequestCommentWriteMode::LiveSandbox => {
                return Err(github_write_error(
                    "github_pr_comment_fixture.mode.unsupported",
                    "GitHub PR comment fixture validation does not support live sandbox mode",
                ));
            }
        },
        provider_comment_reference: None,
        provider_error_code: None,
        summary: fixture.summary.clone(),
        sensitivity: fixture.sensitivity,
        redaction: fixture.redaction.clone(),
    })
    .map_err(|_| {
        github_write_error(
            "github_pr_comment_fixture.response.invalid",
            "GitHub PR comment fixture response is invalid",
        )
    })
}

/// Composes a validated proposed `SideEffectRecord` for a preflighted GitHub PR comment write.
///
/// This helper is in-memory only. It does not call GitHub, persist records,
/// append workflow events, emit audit events, transition `SideEffect` lifecycle
/// beyond `Proposed`, write report artifacts, write files, or expose CLI output.
///
/// # Errors
///
/// Returns stable, non-leaking errors when the write is not ready for proposed
/// record composition, response posture is unsupported, references are invalid,
/// or the resulting `SideEffectRecord` fails validation.
pub fn compose_github_pr_comment_proposed_side_effect_record(
    preflighted: &GitHubPullRequestCommentPreflightedWrite,
    fixture_response: Option<&GitHubPullRequestCommentWriteResponse>,
    input: GitHubPullRequestCommentSideEffectRecordInput,
) -> Result<SideEffectRecord, WorkflowOsError> {
    validate_side_effect_record_composition_boundary(preflighted, fixture_response)?;
    validate_side_effect_record_input(&input)?;

    let request = preflighted.request();
    let authority = github_pr_comment_side_effect_authority(request)?;
    let target = SideEffectTargetReference::new(
        SideEffectTargetKind::AdapterResource,
        request.target().reference(),
    )
    .map_err(|_| {
        github_write_error(
            "github_pr_comment_side_effect_record.target.invalid",
            "GitHub PR comment SideEffect target is invalid",
        )
    })?;
    let idempotency = SideEffectIdempotencyBinding::new(
        request.idempotency_key().clone(),
        SideEffectIdempotencyScope::Run,
        None,
        None,
    )
    .map_err(|_| {
        github_write_error(
            "github_pr_comment_side_effect_record.idempotency.invalid",
            "GitHub PR comment SideEffect idempotency binding is invalid",
        )
    })?;
    let references = github_pr_comment_side_effect_references(request, &input)?;
    let summary = input
        .summary_override
        .unwrap_or_else(|| request.summary().to_owned());
    validate_summary("side-effect record summary", &summary)?;

    SideEffectRecord::new(SideEffectRecordDefinition {
        side_effect_id: request.side_effect_id().clone(),
        lifecycle_state: SideEffectLifecycleState::Proposed,
        target,
        capability: SideEffectCapability::GitHubWrite,
        authority,
        actor: Some(request.actor().clone()),
        system_actor: input.system_actor,
        workflow_id: request.workflow_id().clone(),
        workflow_version: request.workflow_version().clone(),
        schema_version: request.schema_version().clone(),
        spec_hash: request.spec_hash().clone(),
        run_id: request.run_id().clone(),
        step_id: request.step_id().cloned(),
        skill_id: input.skill_id,
        skill_version: input.skill_version,
        adapter_id: Some(request.adapter_id().clone()),
        adapter_kind: Some(AdapterKind::GitHub),
        integration_id: Some(request.integration_id().clone()),
        idempotency,
        references,
        outcome_reference: None,
        created_at: input.created_at,
        updated_at: None,
        correlation_id: Some(request.correlation_id().clone()),
        summary: Some(summary),
        reason_codes: Vec::new(),
        sensitivity: conservative_max_sensitivity(request.sensitivity(), input.sensitivity),
        redaction: request.redaction().clone(),
    })
    .map_err(|_| {
        github_write_error(
            "github_pr_comment_side_effect_record.record.invalid",
            "GitHub PR comment proposed SideEffect record is invalid",
        )
    })
}

/// Composes and explicitly persists a proposed `SideEffectRecord` for a
/// preflighted GitHub PR comment write.
///
/// This helper is still model/store-only. It does not call GitHub, append
/// workflow events, emit audit events, transition `SideEffect` lifecycle beyond
/// `Proposed`, write report artifacts, write files directly, or expose CLI
/// output. Persistence is limited to the caller-supplied `SideEffectRecordStore`.
///
/// # Errors
///
/// Returns stable, non-leaking errors when proposed record composition fails or
/// when the supplied store rejects the record.
pub fn compose_and_persist_github_pr_comment_proposed_side_effect_record(
    store: &impl SideEffectRecordStore,
    preflighted: &GitHubPullRequestCommentPreflightedWrite,
    fixture_response: Option<&GitHubPullRequestCommentWriteResponse>,
    input: GitHubPullRequestCommentSideEffectRecordInput,
) -> Result<SideEffectRecord, WorkflowOsError> {
    let record = compose_github_pr_comment_proposed_side_effect_record(
        preflighted,
        fixture_response,
        input,
    )?;

    if record.lifecycle_state() != SideEffectLifecycleState::Proposed {
        return Err(WorkflowOsError::invalid_state(
            "github_pr_comment_side_effect_record.persistence.unsupported_lifecycle",
            "GitHub PR comment persistence only supports proposed SideEffect records",
        ));
    }

    store
        .write_side_effect_record(&record)
        .map_err(|error| map_side_effect_record_persistence_error(&error))?;

    Ok(record)
}

/// Composes, persists, approval-checks, and transitions a GitHub PR comment
/// `SideEffectRecord` from `Proposed` to `Attempted` without provider calls.
///
/// This is an explicit local orchestration boundary for write-capable adapter
/// readiness. It composes the already-validated preflighted write into a
/// proposed record, persists it through the caller-supplied store, validates
/// approval linkage when required, and performs the store-backed attempted
/// lifecycle transition. It does not call GitHub, load credentials, append
/// workflow events, emit audit records, write report artifacts, write files, or
/// expose CLI output.
///
/// # Errors
///
/// Returns stable, non-leaking errors when proposed record persistence fails,
/// approval linkage is required but unavailable/invalid, transition inputs are
/// unsafe, or the attempted transition fails.
pub fn orchestrate_github_pr_comment_write_attempt_without_provider_call(
    store: &impl SideEffectRecordStore,
    preflighted: &GitHubPullRequestCommentPreflightedWrite,
    input: GitHubPullRequestCommentWriteAttemptOrchestrationInput<'_>,
) -> Result<GitHubPullRequestCommentWriteAttemptOrchestrationResult, WorkflowOsError> {
    if preflighted.provider_call_allowed()
        || preflighted.workflow_event_append_allowed()
        || preflighted.report_artifact_write_allowed()
    {
        return Err(github_write_error(
            "github_pr_comment_write_attempt.preflight.not_ready",
            "GitHub PR comment preflighted write is not ready for attempted orchestration",
        ));
    }
    if let Some(summary) = &input.transition_summary {
        validate_summary("attempt transition summary", summary)?;
    }
    validate_references_for_side_effect_record(&input.transition_references)?;

    let proposed_record = compose_and_persist_github_pr_comment_proposed_side_effect_record(
        store,
        preflighted,
        input.fixture_response,
        input.record_input,
    )?;

    let approval_linkage = if input.require_approval_linkage
        || !proposed_record.authority().approval_references.is_empty()
    {
        let run = input.approval_run.ok_or_else(|| {
            github_write_error(
                "github_pr_comment_write_attempt.approval_run_missing",
                "GitHub PR comment write attempt requires approval linkage",
            )
        })?;
        let side_effect_ids = [proposed_record.side_effect_id().clone()];
        Some(crate::validate_side_effect_approval_linkage_from_store(
            store,
            crate::SideEffectApprovalLinkageFromStoreInput {
                run,
                side_effect_ids: &side_effect_ids,
                load_mode: crate::SideEffectApprovalLinkageStoreLoadMode::ExplicitIds,
                missing_record_policy: crate::SideEffectMissingRecordPolicy::RequireAll,
                require_approval_references_for_requires_approval: true,
                require_decision_for_approved_or_denied: true,
            },
        )?)
    } else {
        None
    };

    let attempted_transition = crate::transition_side_effect_to_attempted_in_store(
        store,
        crate::SideEffectAttemptTransitionStoreInput {
            side_effect_id: proposed_record.side_effect_id(),
            transitioned_at: input.transitioned_at,
            summary: input.transition_summary,
            additional_references: input.transition_references,
            evidence_reference_count: input.evidence_reference_count,
        },
    )?;

    Ok(GitHubPullRequestCommentWriteAttemptOrchestrationResult {
        proposed_record,
        attempted_transition,
        approval_linkage,
    })
}

/// Closes an attempted GitHub PR comment write side-effect without provider calls.
///
/// This helper is intentionally local and bounded. It loads an existing
/// attempted GitHub PR comment `SideEffectRecord`, validates that the outcome is
/// fixture/dry-run/local rather than provider-backed, applies a completed or
/// failed lifecycle transition in the store, and returns the transitioned record
/// plus reference-only event payload for explicit later use.
///
/// It does not call providers, append workflow events, emit audit records, write
/// report artifacts, mutate workflow runs, write files, expose CLI output, or
/// imply that a live GitHub mutation occurred.
///
/// # Errors
///
/// Returns stable, non-leaking errors when the record is missing, not an
/// attempted GitHub PR comment side-effect, outcome inputs are unsafe, or the
/// store-backed lifecycle transition fails.
pub fn orchestrate_github_pr_comment_no_provider_outcome(
    store: &impl SideEffectRecordStore,
    side_effect_id: &SideEffectId,
    input: GitHubPullRequestCommentNoProviderOutcomeOrchestrationInput,
) -> Result<GitHubPullRequestCommentNoProviderOutcomeOrchestrationResult, WorkflowOsError> {
    if let Some(summary) = &input.transition_summary {
        validate_summary("outcome transition summary", summary)?;
    }
    validate_references_for_side_effect_record(&input.transition_references)?;

    let prior_record = store
        .read_side_effect_record(side_effect_id)
        .map_err(|_| {
            github_write_error(
                "github_pr_comment_write_outcome.store_read_failed",
                "GitHub PR comment write outcome could not read the SideEffect record",
            )
        })?
        .ok_or_else(|| {
            github_write_error(
                "github_pr_comment_write_outcome.record_missing",
                "GitHub PR comment write outcome requires an existing SideEffect record",
            )
        })?;
    validate_github_pr_comment_attempted_outcome_record(&prior_record)?;

    let outcome_transition = match input.outcome {
        GitHubPullRequestCommentNoProviderOutcome::Completed { outcome_reference } => {
            validate_no_provider_completed_outcome_reference(&outcome_reference)?;
            crate::transition_side_effect_to_completed_in_store(
                store,
                crate::SideEffectCompleteTransitionStoreInput {
                    side_effect_id,
                    transitioned_at: input.transitioned_at,
                    outcome_reference,
                    summary: input.transition_summary,
                    additional_references: input.transition_references,
                    evidence_reference_count: input.evidence_reference_count,
                },
            )
            .map_err(|error| map_github_pr_comment_outcome_transition_error(&error))?
        }
        GitHubPullRequestCommentNoProviderOutcome::Failed {
            outcome_reference,
            reason_codes,
        } => {
            if let Some(outcome_reference) = &outcome_reference {
                validate_no_provider_failed_outcome_reference(outcome_reference)?;
            }
            crate::transition_side_effect_to_failed_in_store(
                store,
                crate::SideEffectFailTransitionStoreInput {
                    side_effect_id,
                    transitioned_at: input.transitioned_at,
                    outcome_reference,
                    reason_codes,
                    summary: input.transition_summary,
                    additional_references: input.transition_references,
                    evidence_reference_count: input.evidence_reference_count,
                },
            )
            .map_err(|error| map_github_pr_comment_outcome_transition_error(&error))?
        }
    };

    Ok(GitHubPullRequestCommentNoProviderOutcomeOrchestrationResult { outcome_transition })
}

/// Composes a reference-only `SideEffectProposed` workflow event payload from a
/// persisted GitHub PR comment proposed `SideEffectRecord`.
///
/// This helper does not append the event, emit audit records, mutate providers,
/// transition lifecycle, write report artifacts, or expose CLI output.
///
/// # Errors
///
/// Returns stable, non-leaking errors when the record is not a supported
/// proposed GitHub PR comment record or when expected workflow/run identity does
/// not match.
pub fn compose_github_pr_comment_proposed_side_effect_event(
    record: &SideEffectRecord,
    context: &GitHubPullRequestCommentSideEffectEventContext,
) -> Result<SideEffectWorkflowEvent, WorkflowOsError> {
    validate_github_pr_comment_proposed_event_record(record, context)?;

    let evidence_reference_count = record
        .references()
        .iter()
        .filter(|reference| reference.kind() == crate::SideEffectReferenceKind::EvidenceReference)
        .count()
        .try_into()
        .map_err(|_| {
            github_write_error(
                "github_pr_comment_side_effect_event.reference_count.invalid",
                "GitHub PR comment SideEffect event reference count is invalid",
            )
        })?;

    SideEffectWorkflowEvent::new(SideEffectWorkflowEventDefinition {
        side_effect_id: record.side_effect_id().clone(),
        lifecycle_state: SideEffectLifecycleState::Proposed,
        step_id: record.step_id().cloned(),
        skill_id: record.skill_id().cloned(),
        skill_version: record.skill_version().cloned(),
        correlation_id: record.correlation_id().cloned(),
        references: record.references().to_vec(),
        evidence_reference_count,
        outcome_reference_count: 0,
        redaction: record.redaction().clone(),
        sensitivity: record.sensitivity(),
    })
    .map_err(|_| {
        github_write_error(
            "github_pr_comment_side_effect_event.event.invalid",
            "GitHub PR comment SideEffect proposed event is invalid",
        )
    })
}

/// Loads a persisted GitHub PR comment proposed `SideEffectRecord` and composes
/// a reference-only `SideEffectProposed` workflow event payload.
///
/// This helper reads only through the caller-supplied `SideEffectRecordStore`.
/// It does not append workflow events, emit audit records, write report
/// artifacts, call GitHub, or mutate provider state.
///
/// # Errors
///
/// Returns stable, non-leaking errors when the store read fails, the record is
/// missing, or the loaded record is not eligible for proposed-event projection.
pub fn load_github_pr_comment_proposed_side_effect_event(
    store: &impl SideEffectRecordStore,
    side_effect_id: &SideEffectId,
    context: &GitHubPullRequestCommentSideEffectEventContext,
) -> Result<SideEffectWorkflowEvent, WorkflowOsError> {
    let record = store
        .read_side_effect_record(side_effect_id)
        .map_err(|_| {
            github_write_error(
                "github_pr_comment_side_effect_event.store_read_failed",
                "GitHub PR comment SideEffect record could not be loaded",
            )
        })?
        .ok_or_else(|| {
            github_write_error(
                "github_pr_comment_side_effect_event.record_missing",
                "GitHub PR comment SideEffect record is missing",
            )
        })?;

    compose_github_pr_comment_proposed_side_effect_event(&record, context)
}

impl fmt::Debug for GitHubPullRequestCommentWriteResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentWriteResponse")
            .field("correlation_id", &"[REDACTED]")
            .field("mode", &self.mode)
            .field("outcome", &self.outcome)
            .field(
                "provider_comment_reference",
                &self
                    .provider_comment_reference
                    .as_ref()
                    .map(|_| "[REDACTED]"),
            )
            .field("provider_error_code", &self.provider_error_code)
            .field("summary", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for GitHubPullRequestCommentWriteResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definition =
            GitHubPullRequestCommentWriteResponseDefinition::deserialize(deserializer)?;
        Self::new(definition).map_err(serde::de::Error::custom)
    }
}

fn validate_side_effect_record_composition_boundary(
    preflighted: &GitHubPullRequestCommentPreflightedWrite,
    fixture_response: Option<&GitHubPullRequestCommentWriteResponse>,
) -> Result<(), WorkflowOsError> {
    if preflighted.provider_call_allowed()
        || preflighted.workflow_event_append_allowed()
        || preflighted.side_effect_lifecycle_transition_allowed()
        || preflighted.report_artifact_write_allowed()
    {
        return Err(github_write_error(
            "github_pr_comment_side_effect_record.preflight.not_ready",
            "GitHub PR comment preflighted write is not ready for proposed SideEffect record composition",
        ));
    }

    match preflighted.request().mode() {
        GitHubPullRequestCommentWriteMode::Fixture | GitHubPullRequestCommentWriteMode::DryRun => {}
        GitHubPullRequestCommentWriteMode::LiveSandbox => {
            return Err(github_write_error(
                "github_pr_comment_side_effect_record.mode.unsupported",
                "GitHub PR comment live sandbox mode is not supported for proposed SideEffect record composition",
            ));
        }
    }

    if let Some(response) = fixture_response {
        match response.outcome() {
            GitHubPullRequestCommentWriteOutcome::FixtureValidated
            | GitHubPullRequestCommentWriteOutcome::DryRunValidated => {}
            GitHubPullRequestCommentWriteOutcome::ProviderSucceeded
            | GitHubPullRequestCommentWriteOutcome::ProviderFailed => {
                return Err(github_write_error(
                    "github_pr_comment_side_effect_record.response.unsupported",
                    "GitHub PR comment provider response outcomes are not supported for proposed SideEffect record composition",
                ));
            }
        }
        if response.mode() != preflighted.request().mode() {
            return Err(github_write_error(
                "github_pr_comment_side_effect_record.response.mismatch",
                "GitHub PR comment fixture response mode must match the preflighted write",
            ));
        }
        if response.provider_comment_reference().is_some()
            || response.provider_error_code().is_some()
        {
            return Err(github_write_error(
                "github_pr_comment_side_effect_record.response.unsupported",
                "GitHub PR comment fixture response must not include provider references",
            ));
        }
    }

    Ok(())
}

fn validate_side_effect_record_input(
    input: &GitHubPullRequestCommentSideEffectRecordInput,
) -> Result<(), WorkflowOsError> {
    if input.system_actor.is_some() {
        return Err(github_write_error(
            "github_pr_comment_side_effect_record.authority.unsupported",
            "GitHub PR comment proposed SideEffect records already include a requesting actor",
        ));
    }
    validate_references_for_side_effect_record(&input.additional_references)?;
    if let Some(summary) = &input.summary_override {
        validate_summary("side-effect record summary", summary)?;
    }
    Ok(())
}

fn map_side_effect_record_persistence_error(error: &WorkflowOsError) -> WorkflowOsError {
    match error.code() {
        "side_effect_record.write.duplicate" => WorkflowOsError::invalid_state(
            "github_pr_comment_side_effect_record.persistence.duplicate",
            "GitHub PR comment proposed SideEffect record already exists",
        ),
        "side_effect_record.write.identity_mismatch" => WorkflowOsError::invalid_state(
            "github_pr_comment_side_effect_record.persistence.identity_mismatch",
            "GitHub PR comment proposed SideEffect record conflicts with existing run identity",
        ),
        _ => WorkflowOsError::invalid_state(
            "github_pr_comment_side_effect_record.persistence.store_failed",
            "GitHub PR comment proposed SideEffect record could not be persisted",
        ),
    }
}

fn validate_github_pr_comment_attempted_outcome_record(
    record: &SideEffectRecord,
) -> Result<(), WorkflowOsError> {
    if record.lifecycle_state() != SideEffectLifecycleState::Attempted {
        return Err(github_write_error(
            "github_pr_comment_write_outcome.unsupported_lifecycle",
            "GitHub PR comment write outcome requires an attempted record",
        ));
    }
    if record.capability() != SideEffectCapability::GitHubWrite {
        return Err(github_write_error(
            "github_pr_comment_write_outcome.unsupported_capability",
            "GitHub PR comment write outcome requires GitHub write capability",
        ));
    }
    if record.target().kind() != SideEffectTargetKind::AdapterResource
        || !record.target().reference().starts_with("github/")
        || !record.target().reference().contains("/pull/")
    {
        return Err(github_write_error(
            "github_pr_comment_write_outcome.unsupported_target",
            "GitHub PR comment write outcome requires a GitHub pull request target",
        ));
    }
    if record.outcome_reference().is_some() {
        return Err(github_write_error(
            "github_pr_comment_write_outcome.already_has_outcome",
            "GitHub PR comment write outcome requires a record without an outcome reference",
        ));
    }

    Ok(())
}

fn validate_github_pr_comment_proposed_event_record(
    record: &SideEffectRecord,
    context: &GitHubPullRequestCommentSideEffectEventContext,
) -> Result<(), WorkflowOsError> {
    if record.lifecycle_state() != SideEffectLifecycleState::Proposed {
        return Err(github_write_error(
            "github_pr_comment_side_effect_event.unsupported_lifecycle",
            "GitHub PR comment SideEffect event requires a proposed record",
        ));
    }
    if record.capability() != SideEffectCapability::GitHubWrite {
        return Err(github_write_error(
            "github_pr_comment_side_effect_event.unsupported_capability",
            "GitHub PR comment SideEffect event requires GitHub write capability",
        ));
    }
    if record.target().kind() != SideEffectTargetKind::AdapterResource
        || !record.target().reference().starts_with("github/")
        || !record.target().reference().contains("/pull/")
    {
        return Err(github_write_error(
            "github_pr_comment_side_effect_event.unsupported_target",
            "GitHub PR comment SideEffect event requires a GitHub pull request target",
        ));
    }
    if record.outcome_reference().is_some() {
        return Err(github_write_error(
            "github_pr_comment_side_effect_event.outcome_not_supported",
            "GitHub PR comment proposed SideEffect event cannot include outcome references",
        ));
    }
    if record.workflow_id() != &context.workflow_id
        || record.workflow_version() != &context.workflow_version
        || record.schema_version() != &context.schema_version
        || record.spec_hash() != &context.spec_hash
        || record.run_id() != &context.run_id
    {
        return Err(github_write_error(
            "github_pr_comment_side_effect_event.identity_mismatch",
            "GitHub PR comment SideEffect event context does not match the record identity",
        ));
    }

    Ok(())
}

fn validate_no_provider_completed_outcome_reference(
    reference: &SideEffectOutcomeReference,
) -> Result<(), WorkflowOsError> {
    if reference.kind() != SideEffectOutcomeReferenceKind::Outcome {
        return Err(github_write_error(
            "github_pr_comment_write_outcome.completed_reference_kind",
            "GitHub PR comment local completion requires an outcome reference",
        ));
    }
    validate_no_provider_outcome_reference(reference)
}

fn validate_no_provider_failed_outcome_reference(
    reference: &SideEffectOutcomeReference,
) -> Result<(), WorkflowOsError> {
    if reference.kind() != SideEffectOutcomeReferenceKind::Failure {
        return Err(github_write_error(
            "github_pr_comment_write_outcome.failed_reference_kind",
            "GitHub PR comment local failure requires a failure reference",
        ));
    }
    validate_no_provider_outcome_reference(reference)
}

fn validate_no_provider_outcome_reference(
    reference: &SideEffectOutcomeReference,
) -> Result<(), WorkflowOsError> {
    reference.validate().map_err(|_| {
        github_write_error(
            "github_pr_comment_write_outcome.reference.invalid",
            "GitHub PR comment local outcome reference is invalid",
        )
    })?;
    let value = reference.reference();
    if !(value.starts_with("fixture/")
        || value.starts_with("dry-run/")
        || value.starts_with("local/"))
    {
        return Err(github_write_error(
            "github_pr_comment_write_outcome.provider_reference_not_allowed",
            "GitHub PR comment no-provider outcome requires a local, fixture, or dry-run reference",
        ));
    }
    validate_not_secret_like("GitHub PR comment local outcome reference", value)
}

fn map_github_pr_comment_outcome_transition_error(error: &WorkflowOsError) -> WorkflowOsError {
    match error.code() {
        "side_effect.transition.missing_prior" => github_write_error(
            "github_pr_comment_write_outcome.record_missing",
            "GitHub PR comment write outcome requires an existing SideEffect record",
        ),
        "side_effect.transition.invalid_prior_state" => github_write_error(
            "github_pr_comment_write_outcome.unsupported_lifecycle",
            "GitHub PR comment write outcome requires an attempted record",
        ),
        "side_effect.failure_reference.required" => github_write_error(
            "github_pr_comment_write_outcome.failure_reference_required",
            "GitHub PR comment failed local outcome requires a reason code or failure reference",
        ),
        _ => github_write_error(
            "github_pr_comment_write_outcome.transition_failed",
            "GitHub PR comment write outcome transition failed",
        ),
    }
}

fn github_pr_comment_side_effect_authority(
    request: &GitHubPullRequestCommentWriteRequest,
) -> Result<SideEffectAuthority, WorkflowOsError> {
    let decision = if request.preflight().approval_references().is_empty() {
        SideEffectAuthorityDecision::AllowedByPolicy
    } else {
        SideEffectAuthorityDecision::ApprovedByHuman
    };

    SideEffectAuthority::new(
        decision,
        request.preflight().policy_references().to_vec(),
        request.preflight().approval_references().to_vec(),
    )
    .map_err(|_| {
        github_write_error(
            "github_pr_comment_side_effect_record.authority.unsupported",
            "GitHub PR comment SideEffect authority is invalid",
        )
    })
}

fn github_pr_comment_side_effect_references(
    request: &GitHubPullRequestCommentWriteRequest,
    input: &GitHubPullRequestCommentSideEffectRecordInput,
) -> Result<Vec<SideEffectReference>, WorkflowOsError> {
    let mut references = Vec::new();
    references.extend_from_slice(request.preflight().policy_references());
    references.extend_from_slice(request.preflight().approval_references());
    references.extend_from_slice(&input.additional_references);
    validate_references_for_side_effect_record(&references)?;
    Ok(references)
}

fn validate_references_for_side_effect_record(
    references: &[SideEffectReference],
) -> Result<(), WorkflowOsError> {
    for reference in references {
        reference.validate().map_err(|_| {
            github_write_error(
                "github_pr_comment_side_effect_record.reference.invalid",
                "GitHub PR comment SideEffect reference is invalid",
            )
        })?;
    }
    Ok(())
}

fn conservative_max_sensitivity(
    request: SideEffectSensitivity,
    override_value: Option<SideEffectSensitivity>,
) -> SideEffectSensitivity {
    override_value.map_or(request, |value| request.max(value))
}

fn validate_preflight_matches_request(
    request: &GitHubPullRequestCommentWriteRequest,
) -> Result<(), WorkflowOsError> {
    if request.preflight.capability() != AdapterWriteCapability::GitHubPullRequestComment {
        return Err(github_write_error(
            "github_pr_comment_write.preflight.capability",
            "GitHub PR comment write requires matching preflight capability",
        ));
    }
    if request.preflight.target().kind() != AdapterWriteTargetKind::GitHubPullRequest {
        return Err(github_write_error(
            "github_pr_comment_write.preflight.target_kind",
            "GitHub PR comment write requires GitHub pull request preflight target",
        ));
    }
    if request.preflight.target().reference() != request.target.reference() {
        return Err(github_write_error(
            "github_pr_comment_write.preflight.target_reference",
            "GitHub PR comment write target must match preflight target",
        ));
    }
    if request.preflight.side_effect_id() != Some(&request.side_effect_id) {
        return Err(github_write_error(
            "github_pr_comment_write.preflight.side_effect",
            "GitHub PR comment write side-effect ID must match preflight",
        ));
    }
    if request.preflight.idempotency_key() != Some(&request.idempotency_key) {
        return Err(github_write_error(
            "github_pr_comment_write.preflight.idempotency",
            "GitHub PR comment write idempotency key must match preflight",
        ));
    }
    Ok(())
}

fn validate_preflight_decision_matches_request(
    request: &GitHubPullRequestCommentWriteRequest,
    decision: &AdapterWritePreflightDecision,
) -> Result<(), WorkflowOsError> {
    if decision.capability() != AdapterWriteCapability::GitHubPullRequestComment {
        return Err(github_write_error(
            "github_pr_comment_write.preflight_decision.capability",
            "GitHub PR comment write preflight decision must match capability",
        ));
    }
    if decision.side_effect_id() != request.side_effect_id() {
        return Err(github_write_error(
            "github_pr_comment_write.preflight_decision.side_effect",
            "GitHub PR comment write preflight decision must match side-effect ID",
        ));
    }
    if decision.idempotency_key() != request.idempotency_key() {
        return Err(github_write_error(
            "github_pr_comment_write.preflight_decision.idempotency",
            "GitHub PR comment write preflight decision must match idempotency key",
        ));
    }
    if decision.provider_call_allowed()
        || decision.workflow_event_append_allowed()
        || decision.side_effect_lifecycle_transition_allowed()
        || decision.report_artifact_write_allowed()
    {
        return Err(github_write_error(
            "github_pr_comment_write.preflight_decision.execution_boundary",
            "GitHub PR comment write preflight decision must not authorize execution",
        ));
    }
    Ok(())
}

fn validate_fixture_matches_preflighted_write(
    preflighted: &GitHubPullRequestCommentPreflightedWrite,
    fixture: &GitHubPullRequestCommentFixture,
) -> Result<(), WorkflowOsError> {
    if preflighted.provider_call_allowed()
        || preflighted.workflow_event_append_allowed()
        || preflighted.side_effect_lifecycle_transition_allowed()
        || preflighted.report_artifact_write_allowed()
        || fixture.provider_call_allowed()
        || fixture.workflow_event_append_allowed()
        || fixture.side_effect_lifecycle_transition_allowed()
        || fixture.report_artifact_write_allowed()
    {
        return Err(github_write_error(
            "github_pr_comment_fixture.provider_call_forbidden",
            "GitHub PR comment fixture validation must not authorize execution",
        ));
    }
    if preflighted.request().mode() != fixture.mode() {
        return Err(github_write_error(
            "github_pr_comment_fixture.mode.mismatch",
            "GitHub PR comment fixture mode must match the preflighted request",
        ));
    }
    if preflighted.request().target().reference() != fixture.target().reference() {
        return Err(github_write_error(
            "github_pr_comment_fixture.target.mismatch",
            "GitHub PR comment fixture target must match the preflighted request",
        ));
    }
    if preflighted.request().side_effect_id() != fixture.side_effect_id() {
        return Err(github_write_error(
            "github_pr_comment_fixture.side_effect.mismatch",
            "GitHub PR comment fixture SideEffect ID must match the preflighted request",
        ));
    }
    if preflighted.request().idempotency_key() != fixture.idempotency_key() {
        return Err(github_write_error(
            "github_pr_comment_fixture.idempotency.mismatch",
            "GitHub PR comment fixture idempotency key must match the preflighted request",
        ));
    }
    Ok(())
}

fn validate_response_shape(
    response: &GitHubPullRequestCommentWriteResponse,
) -> Result<(), WorkflowOsError> {
    match response.outcome {
        GitHubPullRequestCommentWriteOutcome::FixtureValidated
        | GitHubPullRequestCommentWriteOutcome::DryRunValidated => {
            if response.provider_comment_reference.is_some() {
                return Err(github_write_error(
                    "github_pr_comment_write.response.provider_reference_unexpected",
                    "fixture and dry-run responses must not include provider comment references",
                ));
            }
            if response.provider_error_code.is_some() {
                return Err(github_write_error(
                    "github_pr_comment_write.response.provider_error_unexpected",
                    "fixture and dry-run responses must not include provider error codes",
                ));
            }
        }
        GitHubPullRequestCommentWriteOutcome::ProviderSucceeded => {
            validate_provider_reference(
                response.provider_comment_reference.as_deref(),
                "github_pr_comment_write.response.provider_reference_missing",
            )?;
            if response.provider_error_code.is_some() {
                return Err(github_write_error(
                    "github_pr_comment_write.response.provider_error_unexpected",
                    "successful responses must not include provider error codes",
                ));
            }
        }
        GitHubPullRequestCommentWriteOutcome::ProviderFailed => {
            if response.provider_comment_reference.is_some() {
                return Err(github_write_error(
                    "github_pr_comment_write.response.provider_reference_unexpected",
                    "failed responses must not include provider comment references",
                ));
            }
            validate_error_code(response.provider_error_code.as_deref())?;
        }
    }
    Ok(())
}

fn validate_fixture_reference(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(github_write_error(
            "github_pr_comment_fixture.reference.empty",
            "GitHub PR comment fixture reference cannot be empty",
        ));
    }
    if value.len() > GITHUB_FIXTURE_REFERENCE_MAX_BYTES {
        return Err(github_write_error(
            "github_pr_comment_fixture.reference.too_long",
            format!(
                "GitHub PR comment fixture reference cannot exceed {GITHUB_FIXTURE_REFERENCE_MAX_BYTES} bytes"
            ),
        ));
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
    {
        return Err(github_write_error(
            "github_pr_comment_fixture.reference.invalid",
            "GitHub PR comment fixture reference contains an invalid character",
        ));
    }
    validate_not_secret_like("GitHub PR comment fixture reference", value)
}

fn validate_github_name(label: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(github_write_error(
            "github_pr_comment_write.target.empty",
            format!("GitHub {label} cannot be empty"),
        ));
    }
    if value.len() > GITHUB_NAME_MAX_BYTES {
        return Err(github_write_error(
            "github_pr_comment_write.target.too_long",
            format!("GitHub {label} cannot exceed {GITHUB_NAME_MAX_BYTES} bytes"),
        ));
    }
    if value.starts_with('.') || value.ends_with('.') {
        return Err(github_write_error(
            "github_pr_comment_write.target.invalid",
            format!("GitHub {label} cannot start or end with a period"),
        ));
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(github_write_error(
            "github_pr_comment_write.target.invalid",
            format!("GitHub {label} contains an invalid character"),
        ));
    }
    validate_not_secret_like(&format!("GitHub {label}"), value)
}

fn validate_comment_body(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(github_write_error(
            "github_pr_comment_write.comment_body.empty",
            "GitHub PR comment body cannot be empty",
        ));
    }
    if value.len() > GITHUB_COMMENT_BODY_MAX_BYTES {
        return Err(github_write_error(
            "github_pr_comment_write.comment_body.too_long",
            format!("GitHub PR comment body cannot exceed {GITHUB_COMMENT_BODY_MAX_BYTES} bytes"),
        ));
    }
    validate_not_secret_like("GitHub PR comment body", value)
}

fn validate_summary(label: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(github_write_error(
            "github_pr_comment_write.summary.empty",
            format!("GitHub PR comment {label} cannot be empty"),
        ));
    }
    if value.len() > GITHUB_WRITE_SUMMARY_MAX_BYTES {
        return Err(github_write_error(
            "github_pr_comment_write.summary.too_long",
            format!(
                "GitHub PR comment {label} cannot exceed {GITHUB_WRITE_SUMMARY_MAX_BYTES} bytes"
            ),
        ));
    }
    validate_not_secret_like("GitHub PR comment summary", value)
}

fn validate_provider_reference(
    value: Option<&str>,
    missing_code: &'static str,
) -> Result<(), WorkflowOsError> {
    let Some(value) = value else {
        return Err(github_write_error(
            missing_code,
            "provider comment reference is required for successful provider responses",
        ));
    };
    if value.is_empty() {
        return Err(github_write_error(
            "github_pr_comment_write.response.provider_reference_empty",
            "provider comment reference cannot be empty",
        ));
    }
    if value.len() > GITHUB_PROVIDER_REFERENCE_MAX_BYTES {
        return Err(github_write_error(
            "github_pr_comment_write.response.provider_reference_too_long",
            format!(
                "provider comment reference cannot exceed {GITHUB_PROVIDER_REFERENCE_MAX_BYTES} bytes"
            ),
        ));
    }
    validate_not_secret_like("provider comment reference", value)
}

fn validate_error_code(value: Option<&str>) -> Result<(), WorkflowOsError> {
    let Some(value) = value else {
        return Err(github_write_error(
            "github_pr_comment_write.response.provider_error_missing",
            "provider error code is required for failed provider responses",
        ));
    };
    if value.is_empty() {
        return Err(github_write_error(
            "github_pr_comment_write.response.provider_error_empty",
            "provider error code cannot be empty",
        ));
    }
    if value.len() > GITHUB_PROVIDER_REFERENCE_MAX_BYTES {
        return Err(github_write_error(
            "github_pr_comment_write.response.provider_error_too_long",
            format!(
                "provider error code cannot exceed {GITHUB_PROVIDER_REFERENCE_MAX_BYTES} bytes"
            ),
        ));
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(github_write_error(
            "github_pr_comment_write.response.provider_error_invalid",
            "provider error code contains an invalid character",
        ));
    }
    validate_not_secret_like("provider error code", value)
}

fn validate_redaction_metadata(redaction: &RedactionMetadata) -> Result<(), WorkflowOsError> {
    if redaction.redacted_fields.len() > GITHUB_WRITE_REDACTION_MAX_ENTRIES {
        return Err(github_write_error(
            "github_pr_comment_write.redaction.too_many_fields",
            "GitHub PR comment write redaction metadata contains too many fields",
        ));
    }
    if redaction.field_states.len() > GITHUB_WRITE_REDACTION_MAX_ENTRIES {
        return Err(github_write_error(
            "github_pr_comment_write.redaction.too_many_states",
            "GitHub PR comment write redaction metadata contains too many field states",
        ));
    }
    for field in &redaction.redacted_fields {
        validate_redaction_field(field)?;
    }
    for state in &redaction.field_states {
        validate_redaction_field(&state.field)?;
        validate_redaction_reason(&state.reason)?;
    }
    Ok(())
}

fn validate_redaction_field(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(github_write_error(
            "github_pr_comment_write.redaction.field.empty",
            "GitHub PR comment write redaction field cannot be empty",
        ));
    }
    if value.len() > GITHUB_WRITE_REDACTION_FIELD_MAX_BYTES {
        return Err(github_write_error(
            "github_pr_comment_write.redaction.field.too_long",
            format!(
                "GitHub PR comment write redaction field cannot exceed {GITHUB_WRITE_REDACTION_FIELD_MAX_BYTES} bytes"
            ),
        ));
    }
    validate_not_secret_like("GitHub PR comment write redaction field", value)
}

fn validate_redaction_reason(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(github_write_error(
            "github_pr_comment_write.redaction.reason.empty",
            "GitHub PR comment write redaction reason cannot be empty",
        ));
    }
    if value.len() > GITHUB_WRITE_REDACTION_REASON_MAX_BYTES {
        return Err(github_write_error(
            "github_pr_comment_write.redaction.reason.too_long",
            format!(
                "GitHub PR comment write redaction reason cannot exceed {GITHUB_WRITE_REDACTION_REASON_MAX_BYTES} bytes"
            ),
        ));
    }
    validate_not_secret_like("GitHub PR comment write redaction reason", value)
}

fn validate_not_secret_like(label: &str, value: &str) -> Result<(), WorkflowOsError> {
    let lowercase = value.to_ascii_lowercase();
    let is_secret_like = lowercase.contains("authorization")
        || lowercase.contains("bearer")
        || lowercase.contains("private_key")
        || lowercase.contains("private-key")
        || lowercase.contains("api_token")
        || lowercase.contains("api-token")
        || lowercase.contains("secret")
        || lowercase.contains("token")
        || lowercase.contains("raw_provider_payload")
        || lowercase.contains("raw_command_output")
        || lowercase.contains("raw_parser_payload")
        || lowercase.contains("raw_spec_contents");

    if is_secret_like {
        return Err(github_write_error(
            "github_pr_comment_write.secret_like_value",
            format!("{label} contains sensitive-looking text"),
        ));
    }

    Ok(())
}

fn github_write_error(code: &'static str, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}

/// Builds a preflight request definition matching a GitHub PR comment target.
///
/// This helper constructs model input only. It does not execute preflight and
/// does not authorize provider calls.
///
/// # Errors
///
/// Returns an error when the derived target reference is invalid.
pub fn github_pr_comment_preflight_definition(
    input: GitHubPullRequestCommentPreflightDefinitionInput,
) -> Result<AdapterWritePreflightRequestDefinition, WorkflowOsError> {
    Ok(AdapterWritePreflightRequestDefinition {
        capability: AdapterWriteCapability::GitHubPullRequestComment,
        target: crate::AdapterWriteTarget::new(
            AdapterWriteTargetKind::GitHubPullRequest,
            input.target.reference(),
        )?,
        side_effect_id: Some(input.side_effect_id),
        idempotency_key: Some(input.idempotency_key),
        policy_decision: input.policy_decision,
        policy_references: input.policy_references,
        requires_approval: false,
        approval_references: input.approval_references,
        high_assurance_required: false,
        high_assurance_references: Vec::new(),
        summary: input.summary,
        sensitivity: input.sensitivity,
        redaction: input.redaction,
        readiness_policy: crate::AdapterWriteReadinessPolicy::local_preview_comments_only(),
    })
}
