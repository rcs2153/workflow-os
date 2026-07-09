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
const GITHUB_AUTH_SECRET_MAX_BYTES: usize = 8 * 1024;
const GITHUB_PROVIDER_HTTP_BASE_URL_MAX_BYTES: usize = 256;
const GITHUB_PROVIDER_HTTP_COMMENT_ID_MAX_BYTES: usize = 128;
const GITHUB_PROVIDER_LOOKUP_MARKER_MAX_BYTES: usize = 128;
const GITHUB_PROVIDER_LOOKUP_OBSERVATION_MAX_COUNT: usize = 16;
const GITHUB_PROVIDER_LOOKUP_HTTP_PAGE_SIZE: u16 = 100;

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

/// Caller-supplied auth material for a future GitHub PR comment provider call.
///
/// This model is intentionally not serializable. It must not be stored,
/// logged, copied into reports, or loaded from hidden global state.
#[derive(Clone, Eq, PartialEq)]
pub struct GitHubPullRequestCommentProviderAuth {
    secret: String,
    scope_summary: Option<String>,
}

impl GitHubPullRequestCommentProviderAuth {
    /// Creates bounded caller-supplied auth material.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when auth material is missing or too large.
    pub fn new(
        secret: impl Into<String>,
        scope_summary: Option<String>,
    ) -> Result<Self, WorkflowOsError> {
        let auth = Self {
            secret: secret.into(),
            scope_summary,
        };
        auth.validate()?;
        Ok(auth)
    }

    /// Validates this auth boundary.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when auth material is unsafe.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.secret.is_empty() {
            return Err(github_write_error(
                "github_pr_comment_provider.auth.missing",
                "GitHub PR comment provider auth is required",
            ));
        }
        if self.secret.len() > GITHUB_AUTH_SECRET_MAX_BYTES {
            return Err(github_write_error(
                "github_pr_comment_provider.auth.too_long",
                "GitHub PR comment provider auth exceeds the allowed size",
            ));
        }
        if let Some(scope_summary) = &self.scope_summary {
            validate_summary("provider auth scope summary", scope_summary)?;
        }
        Ok(())
    }

    /// Returns the secret only to an explicitly injected provider client.
    #[must_use]
    pub fn secret_for_provider(&self) -> &str {
        &self.secret
    }

    /// Returns the optional bounded auth scope summary.
    #[must_use]
    pub fn scope_summary(&self) -> Option<&str> {
        self.scope_summary.as_deref()
    }
}

impl fmt::Debug for GitHubPullRequestCommentProviderAuth {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderAuth")
            .field("secret", &"[REDACTED]")
            .field(
                "scope_summary",
                &self.scope_summary.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
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

/// Explicit input for constructing a GitHub PR comment provider-call request.
///
/// This input is the model boundary only. It does not call GitHub, load auth,
/// transition side-effect state, append events, emit audit records, write report
/// artifacts, mutate workflow runs, write files, or expose CLI output.
#[derive(Clone)]
pub struct GitHubPullRequestCommentProviderCallInput<'a> {
    /// Already-attempted GitHub PR comment side-effect record.
    pub attempted_record: &'a SideEffectRecord,
    /// Target pull request.
    pub target: GitHubPullRequestCommentTarget,
    /// Bounded comment body to submit to the injected provider.
    pub comment_body: String,
    /// Idempotency key expected to match the attempted side-effect record.
    pub idempotency_key: IdempotencyKey,
    /// Explicit write mode. Must be `LiveSandbox` for this boundary.
    pub mode: GitHubPullRequestCommentWriteMode,
    /// Caller-supplied auth material for an injected provider.
    pub auth: GitHubPullRequestCommentProviderAuth,
    /// Explicit live-call opt-in.
    pub live_call_enabled: bool,
    /// Explicit provider-call opt-in.
    pub provider_call_enabled: bool,
    /// Bounded call summary.
    pub summary: String,
    /// Sensitivity assigned to the call request.
    pub sensitivity: SideEffectSensitivity,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
}

impl fmt::Debug for GitHubPullRequestCommentProviderCallInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderCallInput")
            .field("attempted_record", &"[REDACTED]")
            .field("target", &self.target)
            .field("comment_body", &"[REDACTED]")
            .field("idempotency_key", &"[REDACTED]")
            .field("mode", &self.mode)
            .field("auth", &self.auth)
            .field("live_call_enabled", &self.live_call_enabled)
            .field("provider_call_enabled", &self.provider_call_enabled)
            .field("summary", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .finish()
    }
}

/// Validated request passed to an injected GitHub PR comment provider client.
///
/// The request is intentionally not serializable because it carries auth
/// material. It is also intentionally not executable on its own: callers must
/// pass it to an injected `GitHubPullRequestCommentProvider`.
pub struct GitHubPullRequestCommentProviderCallRequest {
    side_effect_id: SideEffectId,
    correlation_id: Option<CorrelationId>,
    target: GitHubPullRequestCommentTarget,
    comment_body: String,
    idempotency_key: IdempotencyKey,
    mode: GitHubPullRequestCommentWriteMode,
    auth: GitHubPullRequestCommentProviderAuth,
    summary: String,
    sensitivity: SideEffectSensitivity,
    redaction: RedactionMetadata,
}

impl GitHubPullRequestCommentProviderCallRequest {
    /// Creates a validated provider-call request model.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when pre-call gates fail.
    pub fn new(
        input: GitHubPullRequestCommentProviderCallInput<'_>,
    ) -> Result<Self, WorkflowOsError> {
        validate_provider_call_input(&input)?;
        Ok(Self {
            side_effect_id: input.attempted_record.side_effect_id().clone(),
            correlation_id: input.attempted_record.correlation_id().cloned(),
            target: input.target,
            comment_body: input.comment_body,
            idempotency_key: input.idempotency_key,
            mode: input.mode,
            auth: input.auth,
            summary: input.summary,
            sensitivity: input.sensitivity,
            redaction: input.redaction,
        })
    }

    /// Returns the side-effect ID bound to this provider-call request.
    #[must_use]
    pub const fn side_effect_id(&self) -> &SideEffectId {
        &self.side_effect_id
    }

    /// Returns the optional correlation ID bound to this provider-call request.
    #[must_use]
    pub const fn correlation_id(&self) -> Option<&CorrelationId> {
        self.correlation_id.as_ref()
    }

    /// Returns the target pull request.
    #[must_use]
    pub const fn target(&self) -> &GitHubPullRequestCommentTarget {
        &self.target
    }

    /// Returns the bounded comment body for the injected provider.
    #[must_use]
    pub fn comment_body(&self) -> &str {
        &self.comment_body
    }

    /// Returns the idempotency key.
    #[must_use]
    pub const fn idempotency_key(&self) -> &IdempotencyKey {
        &self.idempotency_key
    }

    /// Returns the provider-call mode.
    #[must_use]
    pub const fn mode(&self) -> GitHubPullRequestCommentWriteMode {
        self.mode
    }

    /// Returns caller-supplied auth for the injected provider.
    #[must_use]
    pub const fn auth(&self) -> &GitHubPullRequestCommentProviderAuth {
        &self.auth
    }

    /// Returns the bounded summary.
    #[must_use]
    pub fn summary(&self) -> &str {
        &self.summary
    }

    /// Returns the sensitivity assigned to this provider-call request.
    #[must_use]
    pub const fn sensitivity(&self) -> SideEffectSensitivity {
        self.sensitivity
    }

    /// Returns redaction metadata associated with this provider-call request.
    #[must_use]
    pub const fn redaction(&self) -> &RedactionMetadata {
        &self.redaction
    }

    /// Returns whether this request authorizes a provider call through an injected client.
    #[must_use]
    pub const fn provider_call_allowed(&self) -> bool {
        true
    }

    /// Returns whether this request appends workflow events.
    #[must_use]
    pub const fn workflow_event_append_allowed(&self) -> bool {
        false
    }

    /// Returns whether this request writes report artifacts.
    #[must_use]
    pub const fn report_artifact_write_allowed(&self) -> bool {
        false
    }
}

impl fmt::Debug for GitHubPullRequestCommentProviderCallRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderCallRequest")
            .field("side_effect_id", &"[REDACTED]")
            .field(
                "correlation_id",
                &self.correlation_id.as_ref().map(|_| "[REDACTED]"),
            )
            .field("target", &self.target)
            .field("comment_body", &"[REDACTED]")
            .field("idempotency_key", &"[REDACTED]")
            .field("mode", &self.mode)
            .field("auth", &self.auth)
            .field("summary", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .field("provider_call_allowed", &true)
            .field("workflow_event_append_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .finish()
    }
}

/// Injected provider boundary for future GitHub PR comment writes.
///
/// Implementations may call GitHub, but this trait definition does not provide
/// a network client, auth loading, retries, lifecycle transitions, event appends,
/// report artifact writes, CLI behavior, schemas, examples, or hosted behavior.
pub trait GitHubPullRequestCommentProvider {
    /// Creates a pull request comment through an injected provider client.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the provider call fails before a
    /// classified provider response can be returned.
    fn create_pull_request_comment(
        &self,
        request: &GitHubPullRequestCommentProviderCallRequest,
    ) -> Result<GitHubPullRequestCommentWriteResponse, WorkflowOsError>;
}

/// Injected transport request for a concrete GitHub PR comment provider client.
///
/// This request is intentionally not serializable. It contains auth and comment
/// text for the injected transport only, and its `Debug` implementation redacts
/// all caller-supplied payload values.
pub struct GitHubPullRequestCommentHttpRequest {
    method: &'static str,
    url: String,
    authorization_header: String,
    body: String,
    idempotency_key: IdempotencyKey,
}

impl GitHubPullRequestCommentHttpRequest {
    /// Returns the HTTP method.
    #[must_use]
    pub const fn method(&self) -> &'static str {
        self.method
    }

    /// Returns the fully constructed request URL for the injected transport.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the authorization header for the injected transport.
    ///
    /// Callers must not log, serialize, or store this value.
    #[must_use]
    pub fn authorization_header_for_transport(&self) -> &str {
        &self.authorization_header
    }

    /// Returns the JSON request body for the injected transport.
    ///
    /// Callers must not log, serialize, or store this value.
    #[must_use]
    pub fn body_for_transport(&self) -> &str {
        &self.body
    }

    /// Returns the idempotency key associated with this provider request.
    #[must_use]
    pub const fn idempotency_key(&self) -> &IdempotencyKey {
        &self.idempotency_key
    }
}

impl fmt::Debug for GitHubPullRequestCommentHttpRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentHttpRequest")
            .field("method", &self.method)
            .field("url", &"[REDACTED]")
            .field("authorization_header", &"[REDACTED]")
            .field("body", &"[REDACTED]")
            .field("idempotency_key", &"[REDACTED]")
            .finish()
    }
}

/// Parsed, bounded HTTP response supplied by an injected GitHub PR comment
/// transport.
///
/// The transport boundary deliberately exposes only status and a parsed comment
/// ID. Raw provider response bodies are not part of the model.
#[derive(Clone, Eq, PartialEq)]
pub struct GitHubPullRequestCommentHttpResponse {
    status: u16,
    provider_comment_id: Option<String>,
}

impl GitHubPullRequestCommentHttpResponse {
    /// Creates a bounded HTTP response for the concrete provider client.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the status or parsed comment ID
    /// is invalid.
    pub fn new(status: u16, provider_comment_id: Option<String>) -> Result<Self, WorkflowOsError> {
        let response = Self {
            status,
            provider_comment_id,
        };
        response.validate()?;
        Ok(response)
    }

    /// Validates the bounded HTTP response shape.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if !(100..=599).contains(&self.status) {
            return Err(github_write_error(
                "github_pr_comment_provider_http.status.invalid",
                "GitHub PR comment provider HTTP status is invalid",
            ));
        }
        if let Some(provider_comment_id) = &self.provider_comment_id {
            validate_provider_comment_id(provider_comment_id)?;
        }
        Ok(())
    }

    /// Returns the HTTP status code.
    #[must_use]
    pub const fn status(&self) -> u16 {
        self.status
    }

    /// Returns the parsed provider comment ID when available.
    #[must_use]
    pub fn provider_comment_id(&self) -> Option<&str> {
        self.provider_comment_id.as_deref()
    }
}

impl fmt::Debug for GitHubPullRequestCommentHttpResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentHttpResponse")
            .field("status", &self.status)
            .field(
                "provider_comment_id",
                &self.provider_comment_id.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

/// Injected HTTP transport for the concrete GitHub PR comment provider client.
///
/// Implementations may perform network I/O, but the trait does not load auth,
/// append events, mutate side-effect state, write artifacts, or emit CLI output.
pub trait GitHubPullRequestCommentHttpTransport {
    /// Sends one already-constructed GitHub PR comment request.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error for local/transport failures before a
    /// bounded provider response is available.
    fn send(
        &self,
        request: &GitHubPullRequestCommentHttpRequest,
    ) -> Result<GitHubPullRequestCommentHttpResponse, WorkflowOsError>;
}

/// Concrete GitHub PR comment provider client with injected transport only.
///
/// This client implements request construction, explicit auth use, and bounded
/// provider response classification. It does not discover credentials, create
/// side-effect records, transition stores, append workflow events, write report
/// artifacts, print CLI output, add schemas/examples, or integrate with an
/// executor.
pub struct GitHubPullRequestCommentHttpProvider<T> {
    transport: T,
    api_base_url: String,
    auth: GitHubPullRequestCommentProviderAuth,
    sensitivity: SideEffectSensitivity,
    redaction: RedactionMetadata,
}

impl<T> GitHubPullRequestCommentHttpProvider<T> {
    /// Creates a concrete provider client backed by an injected transport.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the base URL, auth, or redaction
    /// metadata are invalid.
    pub fn new(
        transport: T,
        api_base_url: impl Into<String>,
        auth: GitHubPullRequestCommentProviderAuth,
        sensitivity: SideEffectSensitivity,
        redaction: RedactionMetadata,
    ) -> Result<Self, WorkflowOsError> {
        let provider = Self {
            transport,
            api_base_url: api_base_url.into(),
            auth,
            sensitivity,
            redaction,
        };
        provider.validate()?;
        Ok(provider)
    }

    /// Validates this explicit concrete provider boundary.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_provider_http_base_url(&self.api_base_url)?;
        self.auth.validate()?;
        validate_redaction_metadata(&self.redaction)?;
        Ok(())
    }

    /// Returns the configured API base URL.
    #[must_use]
    pub fn api_base_url(&self) -> &str {
        &self.api_base_url
    }

    /// Returns the configured sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> SideEffectSensitivity {
        self.sensitivity
    }

    /// Returns whether this provider client appends workflow events.
    #[must_use]
    pub const fn workflow_event_append_allowed(&self) -> bool {
        false
    }

    /// Returns whether this provider client writes report artifacts.
    #[must_use]
    pub const fn report_artifact_write_allowed(&self) -> bool {
        false
    }

    /// Returns whether this provider client writes side-effect records.
    #[must_use]
    pub const fn side_effect_record_write_allowed(&self) -> bool {
        false
    }
}

impl<T: GitHubPullRequestCommentHttpTransport> GitHubPullRequestCommentProvider
    for GitHubPullRequestCommentHttpProvider<T>
{
    fn create_pull_request_comment(
        &self,
        request: &GitHubPullRequestCommentProviderCallRequest,
    ) -> Result<GitHubPullRequestCommentWriteResponse, WorkflowOsError> {
        self.validate()?;
        if request.mode() != GitHubPullRequestCommentWriteMode::LiveSandbox {
            return Err(github_write_error(
                "github_pr_comment_provider_http.mode.unsupported",
                "GitHub PR comment HTTP provider requires live sandbox mode",
            ));
        }
        if request.auth().secret_for_provider() != self.auth.secret_for_provider() {
            return Err(github_write_error(
                "github_pr_comment_provider_http.auth.mismatch",
                "GitHub PR comment HTTP provider auth must match the validated provider-call request",
            ));
        }

        let http_request = self.http_request(request)?;
        let http_response = self.transport.send(&http_request).map_err(|_| {
            github_write_error(
                "github_pr_comment_provider_http.transport_unclassified",
                "GitHub PR comment HTTP transport failed before a classified provider response",
            )
        })?;

        self.classify_response(request, &http_response)
    }
}

impl<T> GitHubPullRequestCommentHttpProvider<T> {
    fn http_request(
        &self,
        request: &GitHubPullRequestCommentProviderCallRequest,
    ) -> Result<GitHubPullRequestCommentHttpRequest, WorkflowOsError> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}/comments",
            self.api_base_url.trim_end_matches('/'),
            request.target().owner(),
            request.target().repository(),
            request.target().pull_request_number()
        );
        validate_provider_http_url(&url)?;
        let body = serde_json::json!({ "body": request.comment_body() }).to_string();
        validate_comment_body(request.comment_body())?;

        Ok(GitHubPullRequestCommentHttpRequest {
            method: "POST",
            url,
            authorization_header: format!("Bearer {}", self.auth.secret_for_provider()),
            body,
            idempotency_key: request.idempotency_key().clone(),
        })
    }

    fn classify_response(
        &self,
        request: &GitHubPullRequestCommentProviderCallRequest,
        response: &GitHubPullRequestCommentHttpResponse,
    ) -> Result<GitHubPullRequestCommentWriteResponse, WorkflowOsError> {
        response.validate()?;
        let correlation_id = request.correlation_id().cloned().ok_or_else(|| {
            github_write_error(
                "github_pr_comment_provider_http.correlation_id.missing",
                "GitHub PR comment HTTP provider requires a correlation ID",
            )
        })?;
        if (200..=299).contains(&response.status()) {
            let provider_comment_id = response.provider_comment_id().ok_or_else(|| {
                github_write_error(
                    "github_pr_comment_provider_http.comment_id.missing",
                    "GitHub PR comment provider success requires a parsed comment ID",
                )
            })?;
            let provider_comment_reference =
                provider_comment_reference(request.target(), provider_comment_id)?;
            return GitHubPullRequestCommentWriteResponse::new(
                GitHubPullRequestCommentWriteResponseDefinition {
                    correlation_id,
                    mode: request.mode(),
                    outcome: GitHubPullRequestCommentWriteOutcome::ProviderSucceeded,
                    provider_comment_reference: Some(provider_comment_reference),
                    provider_error_code: None,
                    summary: "GitHub PR comment provider call succeeded".to_owned(),
                    sensitivity: conservative_max_sensitivity(
                        request.sensitivity(),
                        Some(self.sensitivity),
                    ),
                    redaction: self.redaction.clone(),
                },
            )
            .map_err(|_| {
                github_write_error(
                    "github_pr_comment_provider_http.response.invalid",
                    "GitHub PR comment provider success response is invalid",
                )
            });
        }

        let provider_error_code = classify_provider_http_status(response.status());
        GitHubPullRequestCommentWriteResponse::new(
            GitHubPullRequestCommentWriteResponseDefinition {
                correlation_id,
                mode: request.mode(),
                outcome: GitHubPullRequestCommentWriteOutcome::ProviderFailed,
                provider_comment_reference: None,
                provider_error_code: Some(provider_error_code.to_owned()),
                summary: provider_failure_summary(provider_error_code).to_owned(),
                sensitivity: conservative_max_sensitivity(
                    request.sensitivity(),
                    Some(self.sensitivity),
                ),
                redaction: self.redaction.clone(),
            },
        )
        .map_err(|_| {
            github_write_error(
                "github_pr_comment_provider_http.response.invalid",
                "GitHub PR comment provider failure response is invalid",
            )
        })
    }
}

impl<T> fmt::Debug for GitHubPullRequestCommentHttpProvider<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentHttpProvider")
            .field("transport", &"[REDACTED]")
            .field("api_base_url", &"[REDACTED]")
            .field("auth", &self.auth)
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .field("workflow_event_append_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .field("side_effect_record_write_allowed", &false)
            .finish()
    }
}

/// Injected transport request for the concrete GitHub PR comment lookup client.
///
/// This request is intentionally not serializable. It contains auth and target
/// details for the injected transport only, and its `Debug` implementation
/// redacts all caller-supplied payload values.
pub struct GitHubPullRequestCommentLookupHttpRequest {
    method: &'static str,
    url: String,
    authorization_header: String,
    idempotency_key: IdempotencyKey,
    expected_provider_reference: Option<String>,
    expected_managed_marker: Option<String>,
}

impl GitHubPullRequestCommentLookupHttpRequest {
    /// Returns the HTTP method.
    #[must_use]
    pub const fn method(&self) -> &'static str {
        self.method
    }

    /// Returns the fully constructed request URL for the injected transport.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the authorization header for the injected transport.
    ///
    /// Callers must not log, serialize, or store this value.
    #[must_use]
    pub fn authorization_header_for_transport(&self) -> &str {
        &self.authorization_header
    }

    /// Returns the idempotency key associated with this lookup request.
    #[must_use]
    pub const fn idempotency_key(&self) -> &IdempotencyKey {
        &self.idempotency_key
    }

    /// Returns the expected stable provider reference, when supplied.
    #[must_use]
    pub fn expected_provider_reference(&self) -> Option<&str> {
        self.expected_provider_reference.as_deref()
    }

    /// Returns the expected managed marker, when supplied.
    #[must_use]
    pub fn expected_managed_marker(&self) -> Option<&str> {
        self.expected_managed_marker.as_deref()
    }
}

impl fmt::Debug for GitHubPullRequestCommentLookupHttpRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentLookupHttpRequest")
            .field("method", &self.method)
            .field("url", &"[REDACTED]")
            .field("authorization_header", &"[REDACTED]")
            .field("idempotency_key", &"[REDACTED]")
            .field(
                "has_expected_provider_reference",
                &self.expected_provider_reference.is_some(),
            )
            .field(
                "has_expected_managed_marker",
                &self.expected_managed_marker.is_some(),
            )
            .finish()
    }
}

/// Parsed, bounded HTTP response supplied by an injected GitHub PR comment
/// lookup transport.
///
/// The transport boundary deliberately exposes only status and bounded lookup
/// observations. Raw provider response bodies are not part of the model.
#[derive(Clone, Eq, PartialEq)]
pub struct GitHubPullRequestCommentLookupHttpResponse {
    status: u16,
    observations: Vec<GitHubPullRequestCommentProviderLookupObservation>,
}

impl GitHubPullRequestCommentLookupHttpResponse {
    /// Creates a bounded HTTP lookup response for the concrete lookup client.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the status or parsed
    /// observations are invalid.
    pub fn new(
        status: u16,
        observations: Vec<GitHubPullRequestCommentProviderLookupObservation>,
    ) -> Result<Self, WorkflowOsError> {
        let response = Self {
            status,
            observations,
        };
        response.validate()?;
        Ok(response)
    }

    /// Validates the bounded HTTP lookup response shape.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if !(100..=599).contains(&self.status) {
            return Err(github_write_error(
                "github_pr_comment_provider_lookup_http.status.invalid",
                "GitHub PR comment lookup HTTP status is invalid",
            ));
        }
        if self.observations.len() > GITHUB_PROVIDER_LOOKUP_OBSERVATION_MAX_COUNT {
            return Err(github_write_error(
                "github_pr_comment_provider_lookup_http.response_too_many_observations",
                "GitHub PR comment lookup HTTP response contains too many observations",
            ));
        }
        for observation in &self.observations {
            observation.validate().map_err(|_| {
                github_write_error(
                    "github_pr_comment_provider_lookup_http.observation.invalid",
                    "GitHub PR comment lookup HTTP observation is invalid",
                )
            })?;
        }
        Ok(())
    }

    /// Returns the HTTP status code.
    #[must_use]
    pub const fn status(&self) -> u16 {
        self.status
    }

    /// Returns bounded provider-side lookup observations.
    #[must_use]
    pub fn observations(&self) -> &[GitHubPullRequestCommentProviderLookupObservation] {
        &self.observations
    }
}

impl fmt::Debug for GitHubPullRequestCommentLookupHttpResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentLookupHttpResponse")
            .field("status", &self.status)
            .field("observation_count", &self.observations.len())
            .finish()
    }
}

/// Injected HTTP transport for the concrete GitHub PR comment lookup client.
///
/// Implementations may perform network I/O, but the trait does not load auth,
/// append events, mutate side-effect state, write artifacts, or emit CLI output.
pub trait GitHubPullRequestCommentLookupHttpTransport {
    /// Sends one already-constructed GitHub PR comment lookup request.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error for local/transport failures before a
    /// bounded lookup response is available.
    fn send_lookup(
        &self,
        request: &GitHubPullRequestCommentLookupHttpRequest,
    ) -> Result<GitHubPullRequestCommentLookupHttpResponse, WorkflowOsError>;
}

/// Concrete GitHub PR comment lookup client with injected transport only.
///
/// This client implements request construction, explicit auth use, and bounded
/// provider lookup classification. It does not discover credentials, create
/// side-effect records, transition stores, append workflow events, write report
/// artifacts, print CLI output, add schemas/examples, or integrate with an
/// executor.
pub struct GitHubPullRequestCommentLookupHttpClient<T> {
    transport: T,
    api_base_url: String,
    auth: GitHubPullRequestCommentProviderAuth,
    sensitivity: SideEffectSensitivity,
    redaction: RedactionMetadata,
}

impl<T> GitHubPullRequestCommentLookupHttpClient<T> {
    /// Creates a concrete lookup client backed by an injected transport.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the base URL, auth, or redaction
    /// metadata are invalid.
    pub fn new(
        transport: T,
        api_base_url: impl Into<String>,
        auth: GitHubPullRequestCommentProviderAuth,
        sensitivity: SideEffectSensitivity,
        redaction: RedactionMetadata,
    ) -> Result<Self, WorkflowOsError> {
        let client = Self {
            transport,
            api_base_url: api_base_url.into(),
            auth,
            sensitivity,
            redaction,
        };
        client.validate()?;
        Ok(client)
    }

    /// Validates this explicit concrete lookup boundary.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_provider_http_base_url(&self.api_base_url)?;
        self.auth.validate()?;
        validate_redaction_metadata(&self.redaction)?;
        Ok(())
    }

    /// Returns the configured API base URL.
    #[must_use]
    pub fn api_base_url(&self) -> &str {
        &self.api_base_url
    }

    /// Returns the configured sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> SideEffectSensitivity {
        self.sensitivity
    }

    /// Returns whether this lookup client appends workflow events.
    #[must_use]
    pub const fn workflow_event_append_allowed(&self) -> bool {
        false
    }

    /// Returns whether this lookup client writes report artifacts.
    #[must_use]
    pub const fn report_artifact_write_allowed(&self) -> bool {
        false
    }

    /// Returns whether this lookup client writes side-effect records.
    #[must_use]
    pub const fn side_effect_record_write_allowed(&self) -> bool {
        false
    }
}

impl<T: GitHubPullRequestCommentLookupHttpTransport> GitHubPullRequestCommentProviderLookupClient
    for GitHubPullRequestCommentLookupHttpClient<T>
{
    fn lookup_pull_request_comment(
        &self,
        request: &GitHubPullRequestCommentProviderLookupRequest,
    ) -> Result<GitHubPullRequestCommentProviderLookupResponse, WorkflowOsError> {
        self.validate()?;
        if request.auth().secret_for_provider() != self.auth.secret_for_provider() {
            return Err(github_write_error(
                "github_pr_comment_provider_lookup_http.auth.mismatch",
                "GitHub PR comment lookup HTTP client auth must match the validated lookup request",
            ));
        }

        let http_request = self.http_request(request)?;
        let http_response = self.transport.send_lookup(&http_request).map_err(|_| {
            github_write_error(
                "github_pr_comment_provider_lookup_http.transport_unclassified",
                "GitHub PR comment lookup HTTP transport failed before a classified provider response",
            )
        })?;

        self.classify_lookup_response(request, &http_response)
    }
}

impl<T> GitHubPullRequestCommentLookupHttpClient<T> {
    fn http_request(
        &self,
        request: &GitHubPullRequestCommentProviderLookupRequest,
    ) -> Result<GitHubPullRequestCommentLookupHttpRequest, WorkflowOsError> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}/comments?per_page={}",
            self.api_base_url.trim_end_matches('/'),
            request.target().owner(),
            request.target().repository(),
            request.target().pull_request_number(),
            GITHUB_PROVIDER_LOOKUP_HTTP_PAGE_SIZE
        );
        validate_provider_http_url(&url)?;

        Ok(GitHubPullRequestCommentLookupHttpRequest {
            method: "GET",
            url,
            authorization_header: format!("Bearer {}", self.auth.secret_for_provider()),
            idempotency_key: request.idempotency_key().clone(),
            expected_provider_reference: request.expected_provider_reference().map(str::to_owned),
            expected_managed_marker: request.expected_managed_marker().map(str::to_owned),
        })
    }

    fn classify_lookup_response(
        &self,
        request: &GitHubPullRequestCommentProviderLookupRequest,
        response: &GitHubPullRequestCommentLookupHttpResponse,
    ) -> Result<GitHubPullRequestCommentProviderLookupResponse, WorkflowOsError> {
        response.validate()?;
        let (outcome, provider_error_code) = classify_lookup_http_status(response.status());
        let observations = if outcome == GitHubPullRequestCommentProviderLookupOutcome::Completed {
            response.observations().to_vec()
        } else {
            Vec::new()
        };

        GitHubPullRequestCommentProviderLookupResponse::new(
            GitHubPullRequestCommentProviderLookupResponseDefinition {
                outcome,
                observations,
                provider_error_code: provider_error_code.map(str::to_owned),
                sensitivity: conservative_max_sensitivity(
                    request.sensitivity(),
                    Some(self.sensitivity),
                ),
                redaction: self.redaction.clone(),
            },
        )
        .map_err(|_| {
            github_write_error(
                "github_pr_comment_provider_lookup_http.response.invalid",
                "GitHub PR comment lookup HTTP response is invalid",
            )
        })
    }
}

impl<T> fmt::Debug for GitHubPullRequestCommentLookupHttpClient<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentLookupHttpClient")
            .field("transport", &"[REDACTED]")
            .field("api_base_url", &"[REDACTED]")
            .field("auth", &self.auth)
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .field("workflow_event_append_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .field("side_effect_record_write_allowed", &false)
            .finish()
    }
}

/// Explicit input for orchestrating one injected GitHub PR comment provider call.
///
/// This helper input permits calling only the supplied provider trait. It does
/// not authorize concrete network clients, auth loading, workflow event appends,
/// audit emission, report artifact writes, executor integration, CLI behavior,
/// schemas, examples, hosted behavior, or release posture changes.
#[derive(Clone)]
pub struct GitHubPullRequestCommentProviderCallOrchestrationInput<'a> {
    /// Provider-call request input.
    pub provider_call: GitHubPullRequestCommentProviderCallInput<'a>,
    /// Timestamp for the completed/failed lifecycle transition.
    pub transitioned_at: Timestamp,
    /// Stable non-secret references to add during the lifecycle transition.
    pub transition_references: Vec<SideEffectReference>,
    /// Count of associated evidence references.
    pub evidence_reference_count: u32,
}

impl fmt::Debug for GitHubPullRequestCommentProviderCallOrchestrationInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderCallOrchestrationInput")
            .field("provider_call", &self.provider_call)
            .field("transitioned_at", &self.transitioned_at)
            .field(
                "transition_reference_count",
                &self.transition_references.len(),
            )
            .field("evidence_reference_count", &self.evidence_reference_count)
            .field("workflow_event_append_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .finish()
    }
}

/// Bounded result for injected GitHub PR comment provider-call orchestration.
pub struct GitHubPullRequestCommentProviderCallOrchestrationResult {
    provider_response: GitHubPullRequestCommentWriteResponse,
    outcome_transition: SideEffectLifecycleTransitionResult,
}

impl GitHubPullRequestCommentProviderCallOrchestrationResult {
    /// Returns the validated provider response.
    #[must_use]
    pub const fn provider_response(&self) -> &GitHubPullRequestCommentWriteResponse {
        &self.provider_response
    }

    /// Returns the store-backed outcome transition result.
    #[must_use]
    pub const fn outcome_transition(&self) -> &SideEffectLifecycleTransitionResult {
        &self.outcome_transition
    }

    /// Returns whether this helper appended workflow events.
    #[must_use]
    pub const fn workflow_event_appended(&self) -> bool {
        false
    }

    /// Returns whether this helper wrote report artifacts.
    #[must_use]
    pub const fn report_artifact_written(&self) -> bool {
        false
    }

    /// Consumes the result into its owned parts.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        GitHubPullRequestCommentWriteResponse,
        SideEffectLifecycleTransitionResult,
    ) {
        (self.provider_response, self.outcome_transition)
    }
}

impl fmt::Debug for GitHubPullRequestCommentProviderCallOrchestrationResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderCallOrchestrationResult")
            .field("provider_response", &self.provider_response)
            .field(
                "outcome_lifecycle_state",
                &self.outcome_transition.record().lifecycle_state(),
            )
            .field("workflow_event_appended", &false)
            .field("report_artifact_written", &false)
            .finish()
    }
}

/// Bounded error for injected GitHub PR comment provider-call orchestration.
///
/// The error preserves whether a classified provider response existed before a
/// local lifecycle transition failed. This is required so executor-integrated
/// write paths do not confuse "provider was not called" with "provider returned
/// a response but local reconciliation failed."
#[derive(Clone, Eq, PartialEq)]
pub struct GitHubPullRequestCommentProviderCallOrchestrationError {
    error: Box<WorkflowOsError>,
    provider_response: Option<Box<GitHubPullRequestCommentWriteResponse>>,
}

impl GitHubPullRequestCommentProviderCallOrchestrationError {
    /// Creates an orchestration error before a classified provider response is
    /// available.
    #[must_use]
    pub fn without_provider_response(error: WorkflowOsError) -> Self {
        Self {
            error: Box::new(error),
            provider_response: None,
        }
    }

    /// Creates an orchestration error after a classified provider response was
    /// returned.
    #[must_use]
    pub fn with_provider_response(
        error: WorkflowOsError,
        provider_response: GitHubPullRequestCommentWriteResponse,
    ) -> Self {
        Self {
            error: Box::new(error),
            provider_response: Some(Box::new(provider_response)),
        }
    }

    /// Returns the stable error code.
    #[must_use]
    pub fn code(&self) -> &str {
        self.error.code()
    }

    /// Returns the underlying structured error.
    #[must_use]
    pub fn error(&self) -> &WorkflowOsError {
        &self.error
    }

    /// Returns the classified provider response, when one existed before the
    /// local error.
    #[must_use]
    pub fn provider_response(&self) -> Option<&GitHubPullRequestCommentWriteResponse> {
        self.provider_response.as_deref()
    }

    /// Returns whether the provider was called or may have been called.
    #[must_use]
    pub fn provider_call_attempted(&self) -> bool {
        self.provider_response.is_some()
            || self.error.code() == "github_pr_comment_provider.call_unclassified"
    }

    /// Consumes the error into owned parts.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        WorkflowOsError,
        Option<GitHubPullRequestCommentWriteResponse>,
    ) {
        (
            *self.error,
            self.provider_response.map(|response| *response),
        )
    }
}

impl fmt::Debug for GitHubPullRequestCommentProviderCallOrchestrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderCallOrchestrationError")
            .field("error_code", &self.error.code())
            .field("has_provider_response", &self.provider_response.is_some())
            .field("provider_call_attempted", &self.provider_call_attempted())
            .field(
                "provider_response_outcome",
                &self
                    .provider_response
                    .as_ref()
                    .map(|response| response.outcome()),
            )
            .finish()
    }
}

/// Reconciliation status for a GitHub PR comment provider write.
///
/// This vocabulary captures whether provider and local lifecycle state agree,
/// or whether an operator/future reconciliation helper must resolve ambiguity
/// before retrying or continuing a live write path.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHubPullRequestCommentProviderWriteReconciliationStatus {
    /// No provider call was attempted.
    ProviderNotCalled,
    /// Provider succeeded and local completed transition succeeded.
    ProviderSucceededLocalCompleted,
    /// Provider returned classified failure and local failed transition succeeded.
    ProviderFailedLocalFailed,
    /// Provider succeeded but local completed transition did not complete.
    ProviderSucceededLocalTransitionFailed,
    /// Provider returned classified failure but local failed transition did not complete.
    ProviderFailedLocalTransitionFailed,
    /// Provider call was attempted but no reliable provider response is available.
    ProviderResponseAmbiguous,
    /// Local lifecycle state cannot be reconciled with the provider outcome.
    LocalStateAmbiguous,
    /// A later explicit reconciliation step is required before retry or closure.
    ReconciliationRequired,
}

/// Explicit input for classifying GitHub PR comment provider write reconciliation.
///
/// This input is pure model/helper context. It does not authorize provider
/// calls, store writes, workflow event appends, audit emission, report artifact
/// writes, executor integration, CLI behavior, schemas, examples, hosted
/// behavior, or release posture changes.
pub struct GitHubPullRequestCommentProviderWriteReconciliationInput<'a> {
    /// The attempted side-effect record associated with the provider call.
    pub attempted_record: &'a SideEffectRecord,
    /// Classified provider response, when available.
    pub provider_response: Option<&'a GitHubPullRequestCommentWriteResponse>,
    /// Local lifecycle transition result, when it completed.
    pub local_transition: Option<&'a SideEffectLifecycleTransitionResult>,
    /// Whether a provider call was attempted.
    pub provider_call_attempted: bool,
    /// Stable local transition error code when a local transition failed.
    pub local_transition_error_code: Option<String>,
    /// Stable ambiguity error code when provider response is unavailable.
    pub ambiguity_error_code: Option<String>,
    /// Sensitivity assigned to the reconciliation candidate.
    pub sensitivity: SideEffectSensitivity,
    /// Redaction metadata for the reconciliation candidate.
    pub redaction: RedactionMetadata,
}

impl fmt::Debug for GitHubPullRequestCommentProviderWriteReconciliationInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderWriteReconciliationInput")
            .field(
                "attempted_lifecycle_state",
                &self.attempted_record.lifecycle_state(),
            )
            .field("has_provider_response", &self.provider_response.is_some())
            .field("has_local_transition", &self.local_transition.is_some())
            .field("provider_call_attempted", &self.provider_call_attempted)
            .field(
                "has_local_transition_error_code",
                &self.local_transition_error_code.is_some(),
            )
            .field(
                "has_ambiguity_error_code",
                &self.ambiguity_error_code.is_some(),
            )
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .field("provider_call_allowed", &false)
            .field("workflow_event_append_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .finish()
    }
}

/// Bounded reconciliation candidate for one GitHub PR comment provider write.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GitHubPullRequestCommentProviderWriteReconciliationCandidate {
    side_effect_id: SideEffectId,
    idempotency_key: IdempotencyKey,
    target_kind: SideEffectTargetKind,
    provider_kind: String,
    local_lifecycle_state: SideEffectLifecycleState,
    status: GitHubPullRequestCommentProviderWriteReconciliationStatus,
    provider_reference: Option<String>,
    provider_error_code: Option<String>,
    retry_blocked: bool,
    operator_action_required: bool,
    sensitivity: SideEffectSensitivity,
    redaction: RedactionMetadata,
}

impl GitHubPullRequestCommentProviderWriteReconciliationCandidate {
    /// Creates and validates a bounded reconciliation candidate.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the candidate is invalid.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        side_effect_id: SideEffectId,
        idempotency_key: IdempotencyKey,
        target_kind: SideEffectTargetKind,
        provider_kind: impl Into<String>,
        local_lifecycle_state: SideEffectLifecycleState,
        status: GitHubPullRequestCommentProviderWriteReconciliationStatus,
        provider_reference: Option<String>,
        provider_error_code: Option<String>,
        retry_blocked: bool,
        operator_action_required: bool,
        sensitivity: SideEffectSensitivity,
        redaction: RedactionMetadata,
    ) -> Result<Self, WorkflowOsError> {
        let candidate = Self {
            side_effect_id,
            idempotency_key,
            target_kind,
            provider_kind: provider_kind.into(),
            local_lifecycle_state,
            status,
            provider_reference,
            provider_error_code,
            retry_blocked,
            operator_action_required,
            sensitivity,
            redaction,
        };
        candidate.validate()?;
        Ok(candidate)
    }

    /// Validates this reconciliation candidate.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the candidate violates privacy
    /// or reconciliation invariants.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        SideEffectId::new(self.side_effect_id.as_str()).map_err(|_| {
            github_write_error(
                "github_pr_comment_reconciliation.side_effect_id.invalid",
                "GitHub PR comment reconciliation side-effect ID is invalid",
            )
        })?;
        IdempotencyKey::new(self.idempotency_key.as_str()).map_err(|_| {
            github_write_error(
                "github_pr_comment_reconciliation.idempotency.invalid",
                "GitHub PR comment reconciliation idempotency key is invalid",
            )
        })?;
        if self.target_kind == SideEffectTargetKind::Unknown {
            return Err(github_write_error(
                "github_pr_comment_reconciliation.target_kind.unknown",
                "GitHub PR comment reconciliation target kind must be known",
            ));
        }
        validate_provider_kind(&self.provider_kind)?;
        if let Some(provider_reference) = &self.provider_reference {
            validate_provider_reference(
                Some(provider_reference),
                "github_pr_comment_reconciliation.provider_reference.missing",
            )
            .map_err(|_| {
                github_write_error(
                    "github_pr_comment_reconciliation.provider_reference.invalid",
                    "GitHub PR comment reconciliation provider reference is invalid",
                )
            })?;
        }
        if let Some(provider_error_code) = &self.provider_error_code {
            validate_error_code(Some(provider_error_code)).map_err(|_| {
                github_write_error(
                    "github_pr_comment_reconciliation.provider_error.invalid",
                    "GitHub PR comment reconciliation provider error code is invalid",
                )
            })?;
        }
        validate_redaction_metadata(&self.redaction)?;
        Ok(())
    }

    /// Returns the side-effect ID.
    #[must_use]
    pub const fn side_effect_id(&self) -> &SideEffectId {
        &self.side_effect_id
    }

    /// Returns the idempotency key.
    #[must_use]
    pub const fn idempotency_key(&self) -> &IdempotencyKey {
        &self.idempotency_key
    }

    /// Returns target kind.
    #[must_use]
    pub const fn target_kind(&self) -> SideEffectTargetKind {
        self.target_kind
    }

    /// Returns provider kind.
    #[must_use]
    pub fn provider_kind(&self) -> &str {
        &self.provider_kind
    }

    /// Returns observed local lifecycle state.
    #[must_use]
    pub const fn local_lifecycle_state(&self) -> SideEffectLifecycleState {
        self.local_lifecycle_state
    }

    /// Returns reconciliation status.
    #[must_use]
    pub const fn status(&self) -> GitHubPullRequestCommentProviderWriteReconciliationStatus {
        self.status
    }

    /// Returns bounded provider reference when known.
    #[must_use]
    pub fn provider_reference(&self) -> Option<&str> {
        self.provider_reference.as_deref()
    }

    /// Returns bounded provider error code when known.
    #[must_use]
    pub fn provider_error_code(&self) -> Option<&str> {
        self.provider_error_code.as_deref()
    }

    /// Returns whether retry must be blocked until reconciliation is resolved.
    #[must_use]
    pub const fn retry_blocked(&self) -> bool {
        self.retry_blocked
    }

    /// Returns whether operator/future reconciliation action is required.
    #[must_use]
    pub const fn operator_action_required(&self) -> bool {
        self.operator_action_required
    }

    /// Returns whether this candidate performed provider calls.
    #[must_use]
    pub const fn provider_call_performed(&self) -> bool {
        false
    }

    /// Returns whether this candidate appended workflow events.
    #[must_use]
    pub const fn workflow_event_appended(&self) -> bool {
        false
    }

    /// Returns whether this candidate wrote report artifacts.
    #[must_use]
    pub const fn report_artifact_written(&self) -> bool {
        false
    }
}

impl fmt::Debug for GitHubPullRequestCommentProviderWriteReconciliationCandidate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderWriteReconciliationCandidate")
            .field("side_effect_id", &"[REDACTED]")
            .field("idempotency_key", &"[REDACTED]")
            .field("target_kind", &self.target_kind)
            .field("provider_kind", &self.provider_kind)
            .field("local_lifecycle_state", &self.local_lifecycle_state)
            .field("status", &self.status)
            .field("has_provider_reference", &self.provider_reference.is_some())
            .field(
                "has_provider_error_code",
                &self.provider_error_code.is_some(),
            )
            .field("retry_blocked", &self.retry_blocked)
            .field("operator_action_required", &self.operator_action_required)
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .field("provider_call_performed", &false)
            .field("workflow_event_appended", &false)
            .field("report_artifact_written", &false)
            .finish()
    }
}

impl<'de> Deserialize<'de> for GitHubPullRequestCommentProviderWriteReconciliationCandidate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            side_effect_id: SideEffectId,
            idempotency_key: IdempotencyKey,
            target_kind: SideEffectTargetKind,
            provider_kind: String,
            local_lifecycle_state: SideEffectLifecycleState,
            status: GitHubPullRequestCommentProviderWriteReconciliationStatus,
            provider_reference: Option<String>,
            provider_error_code: Option<String>,
            retry_blocked: bool,
            operator_action_required: bool,
            sensitivity: SideEffectSensitivity,
            redaction: RedactionMetadata,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.side_effect_id,
            wire.idempotency_key,
            wire.target_kind,
            wire.provider_kind,
            wire.local_lifecycle_state,
            wire.status,
            wire.provider_reference,
            wire.provider_error_code,
            wire.retry_blocked,
            wire.operator_action_required,
            wire.sensitivity,
            wire.redaction,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// Explicit provider lookup outcome supplied by an injected lookup client.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHubPullRequestCommentProviderLookupOutcome {
    /// Lookup completed with a bounded response.
    Completed,
    /// Lookup was not authorized by supplied auth or provider policy.
    NotAuthorized,
    /// Lookup could not reach or classify the provider.
    Unavailable,
    /// Lookup was rate-limited.
    RateLimited,
    /// Lookup response shape was not trusted.
    ResponseUntrusted,
}

/// Reconciliation posture for bounded GitHub PR comment provider lookup.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHubPullRequestCommentProviderLookupReconciliationPosture {
    /// Matching provider-side comment was observed.
    RemoteCommentObserved,
    /// Lookup completed and no matching comment was observed.
    RemoteCommentAbsent,
    /// Lookup results were conflicting or insufficiently deterministic.
    RemoteCommentAmbiguous,
    /// Lookup was not authorized.
    LookupNotAuthorized,
    /// Lookup was unavailable.
    LookupUnavailable,
    /// Lookup was rate-limited.
    LookupRateLimited,
    /// Target identity failed validation before lookup.
    LookupTargetInvalid,
    /// Lookup response failed trust/shape validation.
    LookupResponseUntrusted,
}

/// Bounded next-action vocabulary for GitHub PR comment provider lookup reconciliation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHubPullRequestCommentProviderLookupReconciliationNextAction {
    /// Plan an explicit manual repair phase before artifact writes.
    PlanManualStateRepair,
    /// Reevaluate retry eligibility outside this helper.
    ReevaluateRetryEligibility,
    /// Resolve ambiguous provider observations manually.
    ResolveRemoteAmbiguity,
    /// Supply authorized lookup context.
    ProvideAuthorizedLookup,
    /// Retry lookup later outside this helper.
    RetryLookupLater,
    /// Fix lookup target or response construction.
    FixLookupInput,
}

/// Public definition for one bounded provider-side lookup observation.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GitHubPullRequestCommentProviderLookupObservationDefinition {
    /// Observed pull request target.
    pub target: GitHubPullRequestCommentTarget,
    /// Optional stable provider comment reference.
    pub provider_comment_reference: Option<String>,
    /// Optional bounded Workflow OS managed marker.
    pub managed_marker: Option<String>,
}

impl fmt::Debug for GitHubPullRequestCommentProviderLookupObservationDefinition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderLookupObservationDefinition")
            .field("target", &self.target)
            .field(
                "provider_comment_reference",
                &self
                    .provider_comment_reference
                    .as_ref()
                    .map(|_| "[REDACTED]"),
            )
            .field(
                "managed_marker",
                &self.managed_marker.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

/// Bounded provider-side lookup observation.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GitHubPullRequestCommentProviderLookupObservation {
    target: GitHubPullRequestCommentTarget,
    provider_comment_reference: Option<String>,
    managed_marker: Option<String>,
}

impl GitHubPullRequestCommentProviderLookupObservation {
    /// Creates a bounded provider-side lookup observation.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the observation is unsafe.
    pub fn new(
        definition: GitHubPullRequestCommentProviderLookupObservationDefinition,
    ) -> Result<Self, WorkflowOsError> {
        let observation = Self {
            target: definition.target,
            provider_comment_reference: definition.provider_comment_reference,
            managed_marker: definition.managed_marker,
        };
        observation.validate()?;
        Ok(observation)
    }

    /// Validates this provider-side lookup observation.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the observation is unsafe.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.target.validate().map_err(|_| {
            github_write_error(
                "github_pr_comment_provider_lookup_reconciliation.target_invalid",
                "GitHub PR comment lookup observation target is invalid",
            )
        })?;
        if self.provider_comment_reference.is_none() && self.managed_marker.is_none() {
            return Err(github_write_error(
                "github_pr_comment_provider_lookup_reconciliation.observation_signal_missing",
                "GitHub PR comment lookup observation requires a deterministic match signal",
            ));
        }
        if let Some(provider_reference) = &self.provider_comment_reference {
            validate_provider_reference(
                Some(provider_reference),
                "github_pr_comment_provider_lookup_reconciliation.provider_reference_missing",
            )
            .map_err(|_| {
                github_write_error(
                    "github_pr_comment_provider_lookup_reconciliation.provider_reference_invalid",
                    "GitHub PR comment lookup provider reference is invalid",
                )
            })?;
        }
        if let Some(managed_marker) = &self.managed_marker {
            validate_lookup_managed_marker(managed_marker)?;
        }
        Ok(())
    }

    /// Returns the observed target.
    #[must_use]
    pub const fn target(&self) -> &GitHubPullRequestCommentTarget {
        &self.target
    }

    /// Returns the observed stable provider comment reference, when available.
    #[must_use]
    pub fn provider_comment_reference(&self) -> Option<&str> {
        self.provider_comment_reference.as_deref()
    }

    /// Returns the observed bounded managed marker, when available.
    #[must_use]
    pub fn managed_marker(&self) -> Option<&str> {
        self.managed_marker.as_deref()
    }
}

impl fmt::Debug for GitHubPullRequestCommentProviderLookupObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderLookupObservation")
            .field("target", &self.target)
            .field(
                "has_provider_comment_reference",
                &self.provider_comment_reference.is_some(),
            )
            .field("has_managed_marker", &self.managed_marker.is_some())
            .finish()
    }
}

impl<'de> Deserialize<'de> for GitHubPullRequestCommentProviderLookupObservation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definition =
            GitHubPullRequestCommentProviderLookupObservationDefinition::deserialize(deserializer)?;
        Self::new(definition).map_err(serde::de::Error::custom)
    }
}

/// Public definition for a bounded provider lookup response.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GitHubPullRequestCommentProviderLookupResponseDefinition {
    /// Classified lookup outcome.
    pub outcome: GitHubPullRequestCommentProviderLookupOutcome,
    /// Bounded provider-side observations.
    pub observations: Vec<GitHubPullRequestCommentProviderLookupObservation>,
    /// Optional stable provider lookup error code.
    pub provider_error_code: Option<String>,
    /// Sensitivity assigned to the response.
    pub sensitivity: SideEffectSensitivity,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
}

impl fmt::Debug for GitHubPullRequestCommentProviderLookupResponseDefinition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderLookupResponseDefinition")
            .field("outcome", &self.outcome)
            .field("observation_count", &self.observations.len())
            .field("provider_error_code", &self.provider_error_code)
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .finish()
    }
}

/// Bounded provider lookup response supplied by an injected client.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GitHubPullRequestCommentProviderLookupResponse {
    outcome: GitHubPullRequestCommentProviderLookupOutcome,
    observations: Vec<GitHubPullRequestCommentProviderLookupObservation>,
    provider_error_code: Option<String>,
    sensitivity: SideEffectSensitivity,
    redaction: RedactionMetadata,
}

impl GitHubPullRequestCommentProviderLookupResponse {
    /// Creates a bounded provider lookup response.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the response shape is unsafe.
    pub fn new(
        definition: GitHubPullRequestCommentProviderLookupResponseDefinition,
    ) -> Result<Self, WorkflowOsError> {
        let response = Self {
            outcome: definition.outcome,
            observations: definition.observations,
            provider_error_code: definition.provider_error_code,
            sensitivity: definition.sensitivity,
            redaction: definition.redaction,
        };
        response.validate()?;
        Ok(response)
    }

    /// Validates this provider lookup response.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the response shape is unsafe.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.observations.len() > GITHUB_PROVIDER_LOOKUP_OBSERVATION_MAX_COUNT {
            return Err(github_write_error(
                "github_pr_comment_provider_lookup_reconciliation.response_too_many_observations",
                "GitHub PR comment lookup response contains too many observations",
            ));
        }
        for observation in &self.observations {
            observation.validate()?;
        }
        match self.outcome {
            GitHubPullRequestCommentProviderLookupOutcome::Completed => {
                if let Some(provider_error_code) = &self.provider_error_code {
                    validate_error_code(Some(provider_error_code)).map_err(|_| {
                        github_write_error(
                            "github_pr_comment_provider_lookup_reconciliation.provider_error_invalid",
                            "GitHub PR comment lookup provider error code is invalid",
                        )
                    })?;
                }
            }
            GitHubPullRequestCommentProviderLookupOutcome::NotAuthorized
            | GitHubPullRequestCommentProviderLookupOutcome::Unavailable
            | GitHubPullRequestCommentProviderLookupOutcome::RateLimited
            | GitHubPullRequestCommentProviderLookupOutcome::ResponseUntrusted => {
                let provider_error_code = self.provider_error_code.as_deref().ok_or_else(|| {
                    github_write_error(
                        "github_pr_comment_provider_lookup_reconciliation.provider_error_missing",
                        "GitHub PR comment lookup response requires a provider error code",
                    )
                })?;
                validate_error_code(Some(provider_error_code)).map_err(|_| {
                    github_write_error(
                        "github_pr_comment_provider_lookup_reconciliation.provider_error_invalid",
                        "GitHub PR comment lookup provider error code is invalid",
                    )
                })?;
            }
        }
        validate_redaction_metadata(&self.redaction)?;
        Ok(())
    }

    /// Returns the lookup outcome.
    #[must_use]
    pub const fn outcome(&self) -> GitHubPullRequestCommentProviderLookupOutcome {
        self.outcome
    }

    /// Returns bounded provider-side observations.
    #[must_use]
    pub fn observations(&self) -> &[GitHubPullRequestCommentProviderLookupObservation] {
        &self.observations
    }

    /// Returns optional stable provider lookup error code.
    #[must_use]
    pub fn provider_error_code(&self) -> Option<&str> {
        self.provider_error_code.as_deref()
    }

    /// Returns response sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> SideEffectSensitivity {
        self.sensitivity
    }
}

impl fmt::Debug for GitHubPullRequestCommentProviderLookupResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderLookupResponse")
            .field("outcome", &self.outcome)
            .field("observation_count", &self.observations.len())
            .field("provider_error_code", &self.provider_error_code)
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for GitHubPullRequestCommentProviderLookupResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definition =
            GitHubPullRequestCommentProviderLookupResponseDefinition::deserialize(deserializer)?;
        Self::new(definition).map_err(serde::de::Error::custom)
    }
}

/// Explicit input for provider lookup/query reconciliation.
pub struct GitHubPullRequestCommentProviderLookupReconciliationInput<'a> {
    /// Attempted side-effect record associated with the provider write.
    pub attempted_record: &'a SideEffectRecord,
    /// Expected pull request target.
    pub target: GitHubPullRequestCommentTarget,
    /// Optional expected provider comment reference.
    pub expected_provider_reference: Option<String>,
    /// Optional expected bounded Workflow OS managed marker.
    pub expected_managed_marker: Option<String>,
    /// Caller-supplied auth material for the injected lookup client.
    pub auth: GitHubPullRequestCommentProviderAuth,
    /// Sensitivity assigned to the lookup reconciliation.
    pub sensitivity: SideEffectSensitivity,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
}

impl fmt::Debug for GitHubPullRequestCommentProviderLookupReconciliationInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderLookupReconciliationInput")
            .field(
                "attempted_lifecycle_state",
                &self.attempted_record.lifecycle_state(),
            )
            .field("target", &self.target)
            .field(
                "has_expected_provider_reference",
                &self.expected_provider_reference.is_some(),
            )
            .field(
                "has_expected_managed_marker",
                &self.expected_managed_marker.is_some(),
            )
            .field("auth", &self.auth)
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .field("workflow_event_append_allowed", &false)
            .field("side_effect_record_write_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .finish()
    }
}

/// Validated request passed to an injected provider lookup client.
pub struct GitHubPullRequestCommentProviderLookupRequest {
    side_effect_id: SideEffectId,
    idempotency_key: IdempotencyKey,
    target: GitHubPullRequestCommentTarget,
    expected_provider_reference: Option<String>,
    expected_managed_marker: Option<String>,
    auth: GitHubPullRequestCommentProviderAuth,
    sensitivity: SideEffectSensitivity,
    redaction: RedactionMetadata,
}

impl GitHubPullRequestCommentProviderLookupRequest {
    /// Creates a validated provider lookup request.
    ///
    /// # Errors
    ///
    /// Returns stable, non-leaking errors when pre-lookup gates fail.
    pub fn new(
        input: GitHubPullRequestCommentProviderLookupReconciliationInput<'_>,
    ) -> Result<Self, WorkflowOsError> {
        validate_github_pr_comment_attempted_outcome_record(input.attempted_record).map_err(
            |_| {
                github_write_error(
                    "github_pr_comment_provider_lookup_reconciliation.attempted_record_invalid",
                    "GitHub PR comment lookup requires an attempted record",
                )
            },
        )?;
        input.target.validate().map_err(|_| {
            github_write_error(
                "github_pr_comment_provider_lookup_reconciliation.target_invalid",
                "GitHub PR comment lookup target is invalid",
            )
        })?;
        if input.attempted_record.target().reference() != input.target.reference() {
            return Err(github_write_error(
                "github_pr_comment_provider_lookup_reconciliation.target_mismatch",
                "GitHub PR comment lookup target must match the attempted record",
            ));
        }
        input.auth.validate().map_err(|_| {
            github_write_error(
                "github_pr_comment_provider_lookup_reconciliation.auth_denied",
                "GitHub PR comment lookup auth is invalid",
            )
        })?;
        if let Some(provider_reference) = &input.expected_provider_reference {
            validate_provider_reference(
                Some(provider_reference),
                "github_pr_comment_provider_lookup_reconciliation.provider_reference_missing",
            )
            .map_err(|_| {
                github_write_error(
                    "github_pr_comment_provider_lookup_reconciliation.provider_reference_invalid",
                    "GitHub PR comment lookup expected provider reference is invalid",
                )
            })?;
        }
        if let Some(managed_marker) = &input.expected_managed_marker {
            validate_lookup_managed_marker(managed_marker)?;
        }
        validate_redaction_metadata(&input.redaction)?;

        Ok(Self {
            side_effect_id: input.attempted_record.side_effect_id().clone(),
            idempotency_key: input.attempted_record.idempotency().key().clone(),
            target: input.target,
            expected_provider_reference: input.expected_provider_reference,
            expected_managed_marker: input.expected_managed_marker,
            auth: input.auth,
            sensitivity: input.sensitivity,
            redaction: input.redaction,
        })
    }

    /// Returns the side-effect ID.
    #[must_use]
    pub const fn side_effect_id(&self) -> &SideEffectId {
        &self.side_effect_id
    }

    /// Returns the idempotency key.
    #[must_use]
    pub const fn idempotency_key(&self) -> &IdempotencyKey {
        &self.idempotency_key
    }

    /// Returns the expected target.
    #[must_use]
    pub const fn target(&self) -> &GitHubPullRequestCommentTarget {
        &self.target
    }

    /// Returns the expected provider reference.
    #[must_use]
    pub fn expected_provider_reference(&self) -> Option<&str> {
        self.expected_provider_reference.as_deref()
    }

    /// Returns the expected managed marker.
    #[must_use]
    pub fn expected_managed_marker(&self) -> Option<&str> {
        self.expected_managed_marker.as_deref()
    }

    /// Returns caller-supplied auth for the injected lookup client.
    #[must_use]
    pub const fn auth(&self) -> &GitHubPullRequestCommentProviderAuth {
        &self.auth
    }

    /// Returns sensitivity assigned to this request.
    #[must_use]
    pub const fn sensitivity(&self) -> SideEffectSensitivity {
        self.sensitivity
    }

    /// Returns whether this request appends workflow events.
    #[must_use]
    pub const fn workflow_event_append_allowed(&self) -> bool {
        false
    }

    /// Returns whether this request writes side-effect records.
    #[must_use]
    pub const fn side_effect_record_write_allowed(&self) -> bool {
        false
    }

    /// Returns whether this request writes report artifacts.
    #[must_use]
    pub const fn report_artifact_write_allowed(&self) -> bool {
        false
    }
}

impl fmt::Debug for GitHubPullRequestCommentProviderLookupRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderLookupRequest")
            .field("side_effect_id", &"[REDACTED]")
            .field("idempotency_key", &"[REDACTED]")
            .field("target", &self.target)
            .field(
                "has_expected_provider_reference",
                &self.expected_provider_reference.is_some(),
            )
            .field(
                "has_expected_managed_marker",
                &self.expected_managed_marker.is_some(),
            )
            .field("auth", &self.auth)
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .field("workflow_event_append_allowed", &false)
            .field("side_effect_record_write_allowed", &false)
            .field("report_artifact_write_allowed", &false)
            .finish()
    }
}

/// Injected provider lookup/query boundary for GitHub PR comments.
pub trait GitHubPullRequestCommentProviderLookupClient {
    /// Looks up bounded provider-side observations.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error for local client failures before a
    /// bounded lookup response can be returned.
    fn lookup_pull_request_comment(
        &self,
        request: &GitHubPullRequestCommentProviderLookupRequest,
    ) -> Result<GitHubPullRequestCommentProviderLookupResponse, WorkflowOsError>;
}

/// Bounded result for provider lookup/query reconciliation.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GitHubPullRequestCommentProviderLookupReconciliationResult {
    side_effect_id: SideEffectId,
    idempotency_key: IdempotencyKey,
    target_kind: SideEffectTargetKind,
    provider_kind: String,
    local_lifecycle_state: SideEffectLifecycleState,
    posture: GitHubPullRequestCommentProviderLookupReconciliationPosture,
    observed_provider_reference: Option<String>,
    observed_match_count: u32,
    provider_error_code: Option<String>,
    next_action: GitHubPullRequestCommentProviderLookupReconciliationNextAction,
    sensitivity: SideEffectSensitivity,
    redaction: RedactionMetadata,
}

impl GitHubPullRequestCommentProviderLookupReconciliationResult {
    /// Creates and validates a bounded lookup reconciliation result.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the result shape is unsafe.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        side_effect_id: SideEffectId,
        idempotency_key: IdempotencyKey,
        target_kind: SideEffectTargetKind,
        provider_kind: impl Into<String>,
        local_lifecycle_state: SideEffectLifecycleState,
        posture: GitHubPullRequestCommentProviderLookupReconciliationPosture,
        observed_provider_reference: Option<String>,
        observed_match_count: u32,
        provider_error_code: Option<String>,
        next_action: GitHubPullRequestCommentProviderLookupReconciliationNextAction,
        sensitivity: SideEffectSensitivity,
        redaction: RedactionMetadata,
    ) -> Result<Self, WorkflowOsError> {
        let result = Self {
            side_effect_id,
            idempotency_key,
            target_kind,
            provider_kind: provider_kind.into(),
            local_lifecycle_state,
            posture,
            observed_provider_reference,
            observed_match_count,
            provider_error_code,
            next_action,
            sensitivity,
            redaction,
        };
        result.validate()?;
        Ok(result)
    }

    /// Validates this lookup reconciliation result.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the result shape is unsafe.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        SideEffectId::new(self.side_effect_id.as_str()).map_err(|_| {
            github_write_error(
                "github_pr_comment_provider_lookup_reconciliation.side_effect_id_invalid",
                "GitHub PR comment lookup reconciliation side-effect ID is invalid",
            )
        })?;
        IdempotencyKey::new(self.idempotency_key.as_str()).map_err(|_| {
            github_write_error(
                "github_pr_comment_provider_lookup_reconciliation.idempotency_invalid",
                "GitHub PR comment lookup reconciliation idempotency key is invalid",
            )
        })?;
        if self.target_kind == SideEffectTargetKind::Unknown {
            return Err(github_write_error(
                "github_pr_comment_provider_lookup_reconciliation.target_kind_unknown",
                "GitHub PR comment lookup reconciliation target kind must be known",
            ));
        }
        validate_provider_kind(&self.provider_kind)?;
        if self.observed_match_count as usize > GITHUB_PROVIDER_LOOKUP_OBSERVATION_MAX_COUNT {
            return Err(github_write_error(
                "github_pr_comment_provider_lookup_reconciliation.match_count_too_large",
                "GitHub PR comment lookup reconciliation match count is too large",
            ));
        }
        if let Some(provider_reference) = &self.observed_provider_reference {
            validate_provider_reference(
                Some(provider_reference),
                "github_pr_comment_provider_lookup_reconciliation.provider_reference_missing",
            )
            .map_err(|_| {
                github_write_error(
                    "github_pr_comment_provider_lookup_reconciliation.provider_reference_invalid",
                    "GitHub PR comment lookup reconciliation observed provider reference is invalid",
                )
            })?;
        }
        if let Some(provider_error_code) = &self.provider_error_code {
            validate_error_code(Some(provider_error_code)).map_err(|_| {
                github_write_error(
                    "github_pr_comment_provider_lookup_reconciliation.provider_error_invalid",
                    "GitHub PR comment lookup reconciliation provider error code is invalid",
                )
            })?;
        }
        if self.posture
            == GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentObserved
            && self.observed_provider_reference.is_none()
        {
            return Err(github_write_error(
                "github_pr_comment_provider_lookup_reconciliation.provider_reference_missing",
                "GitHub PR comment lookup observation requires a provider reference",
            ));
        }
        validate_redaction_metadata(&self.redaction)?;
        Ok(())
    }

    /// Returns lookup reconciliation posture.
    #[must_use]
    pub const fn posture(&self) -> GitHubPullRequestCommentProviderLookupReconciliationPosture {
        self.posture
    }

    /// Returns observed provider reference, if any.
    #[must_use]
    pub fn observed_provider_reference(&self) -> Option<&str> {
        self.observed_provider_reference.as_deref()
    }

    /// Returns observed deterministic match count.
    #[must_use]
    pub const fn observed_match_count(&self) -> u32 {
        self.observed_match_count
    }

    /// Returns stable provider error code, if any.
    #[must_use]
    pub fn provider_error_code(&self) -> Option<&str> {
        self.provider_error_code.as_deref()
    }

    /// Returns whether retry remains blocked.
    #[must_use]
    pub fn retry_blocked(&self) -> bool {
        lookup_posture_blocks_retry(self.posture)
    }

    /// Returns whether manual state repair may be planned later.
    #[must_use]
    pub fn manual_state_repair_may_be_planned(&self) -> bool {
        self.posture
            == GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentObserved
    }

    /// Returns whether report artifact writes remain blocked.
    #[must_use]
    pub const fn artifact_write_blocked(&self) -> bool {
        true
    }

    /// Returns whether artifact write may proceed.
    #[must_use]
    pub const fn artifact_write_may_proceed(&self) -> bool {
        false
    }

    /// Returns whether operator action is required.
    #[must_use]
    pub fn operator_action_required(&self) -> bool {
        lookup_posture_requires_operator(self.posture)
    }

    /// Returns bounded next action.
    #[must_use]
    pub const fn next_action(
        &self,
    ) -> GitHubPullRequestCommentProviderLookupReconciliationNextAction {
        self.next_action
    }

    /// Returns whether this helper appended workflow events.
    #[must_use]
    pub const fn workflow_event_appended(&self) -> bool {
        false
    }

    /// Returns whether this helper mutated side-effect records.
    #[must_use]
    pub const fn side_effect_record_mutated(&self) -> bool {
        false
    }

    /// Returns whether this helper wrote report artifacts.
    #[must_use]
    pub const fn report_artifact_written(&self) -> bool {
        false
    }

    /// Returns whether this helper emitted CLI output.
    #[must_use]
    pub const fn cli_output_emitted(&self) -> bool {
        false
    }
}

impl fmt::Debug for GitHubPullRequestCommentProviderLookupReconciliationResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentProviderLookupReconciliationResult")
            .field("side_effect_id", &"[REDACTED]")
            .field("idempotency_key", &"[REDACTED]")
            .field("target_kind", &self.target_kind)
            .field("provider_kind", &self.provider_kind)
            .field("local_lifecycle_state", &self.local_lifecycle_state)
            .field("posture", &self.posture)
            .field(
                "has_observed_provider_reference",
                &self.observed_provider_reference.is_some(),
            )
            .field("observed_match_count", &self.observed_match_count)
            .field("provider_error_code", &self.provider_error_code)
            .field("retry_blocked", &self.retry_blocked())
            .field(
                "manual_state_repair_may_be_planned",
                &self.manual_state_repair_may_be_planned(),
            )
            .field("artifact_write_blocked", &true)
            .field("operator_action_required", &self.operator_action_required())
            .field("next_action", &self.next_action)
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .field("workflow_event_appended", &false)
            .field("side_effect_record_mutated", &false)
            .field("report_artifact_written", &false)
            .field("cli_output_emitted", &false)
            .finish()
    }
}

impl<'de> Deserialize<'de> for GitHubPullRequestCommentProviderLookupReconciliationResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            side_effect_id: SideEffectId,
            idempotency_key: IdempotencyKey,
            target_kind: SideEffectTargetKind,
            provider_kind: String,
            local_lifecycle_state: SideEffectLifecycleState,
            posture: GitHubPullRequestCommentProviderLookupReconciliationPosture,
            observed_provider_reference: Option<String>,
            observed_match_count: u32,
            provider_error_code: Option<String>,
            next_action: GitHubPullRequestCommentProviderLookupReconciliationNextAction,
            sensitivity: SideEffectSensitivity,
            redaction: RedactionMetadata,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.side_effect_id,
            wire.idempotency_key,
            wire.target_kind,
            wire.provider_kind,
            wire.local_lifecycle_state,
            wire.posture,
            wire.observed_provider_reference,
            wire.observed_match_count,
            wire.provider_error_code,
            wire.next_action,
            wire.sensitivity,
            wire.redaction,
        )
        .map_err(serde::de::Error::custom)
    }
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

/// Orchestrates one injected GitHub PR comment provider call into a lifecycle outcome.
///
/// This helper validates all pre-call gates, invokes only the caller-supplied
/// provider trait, and applies a completed or failed lifecycle transition from
/// the validated provider response. It does not provide a concrete network
/// client, load auth, append workflow events, emit audit records, write report
/// artifacts, mutate workflow runs, expose CLI output, add schemas/examples, or
/// change release posture.
///
/// # Errors
///
/// Returns stable, non-leaking errors when pre-call gates fail, the provider
/// returns an unclassified error, the provider response is not a provider
/// success/failure outcome, or the store-backed transition fails.
pub fn orchestrate_github_pr_comment_provider_call(
    store: &impl SideEffectRecordStore,
    provider: &impl GitHubPullRequestCommentProvider,
    input: GitHubPullRequestCommentProviderCallOrchestrationInput<'_>,
) -> Result<
    GitHubPullRequestCommentProviderCallOrchestrationResult,
    GitHubPullRequestCommentProviderCallOrchestrationError,
> {
    validate_references_for_side_effect_record(&input.transition_references).map_err(
        GitHubPullRequestCommentProviderCallOrchestrationError::without_provider_response,
    )?;
    let transitioned_at = input.transitioned_at;
    let transition_references = input.transition_references.clone();
    let evidence_reference_count = input.evidence_reference_count;

    let request = GitHubPullRequestCommentProviderCallRequest::new(input.provider_call).map_err(
        GitHubPullRequestCommentProviderCallOrchestrationError::without_provider_response,
    )?;
    let stored_record = store
        .read_side_effect_record(request.side_effect_id())
        .map_err(|_| {
            GitHubPullRequestCommentProviderCallOrchestrationError::without_provider_response(
                github_write_error(
                    "github_pr_comment_provider.store_read_failed",
                    "GitHub PR comment provider call could not read the SideEffect record",
                ),
            )
        })?
        .ok_or_else(|| {
            GitHubPullRequestCommentProviderCallOrchestrationError::without_provider_response(
                github_write_error(
                    "github_pr_comment_provider.record_missing",
                    "GitHub PR comment provider call requires an existing SideEffect record",
                ),
            )
        })?;
    validate_provider_call_store_record(&stored_record, &request).map_err(
        GitHubPullRequestCommentProviderCallOrchestrationError::without_provider_response,
    )?;

    let provider_response = provider
        .create_pull_request_comment(&request)
        .map_err(|_| {
            GitHubPullRequestCommentProviderCallOrchestrationError::without_provider_response(
                github_write_error(
                "github_pr_comment_provider.call_unclassified",
                "GitHub PR comment provider call failed before a classified response was returned",
                ),
            )
        })?;

    let outcome_transition = transition_github_pr_comment_provider_response(
        store,
        &request,
        transitioned_at,
        &transition_references,
        evidence_reference_count,
        &provider_response,
    )?;

    Ok(GitHubPullRequestCommentProviderCallOrchestrationResult {
        provider_response,
        outcome_transition,
    })
}

fn transition_github_pr_comment_provider_response(
    store: &impl SideEffectRecordStore,
    request: &GitHubPullRequestCommentProviderCallRequest,
    transitioned_at: Timestamp,
    transition_references: &[SideEffectReference],
    evidence_reference_count: u32,
    provider_response: &GitHubPullRequestCommentWriteResponse,
) -> Result<
    SideEffectLifecycleTransitionResult,
    GitHubPullRequestCommentProviderCallOrchestrationError,
> {
    match provider_response.outcome() {
        GitHubPullRequestCommentWriteOutcome::ProviderSucceeded => {
            transition_github_pr_comment_provider_success(
                store,
                request,
                transitioned_at,
                transition_references,
                evidence_reference_count,
                provider_response,
            )
        }
        GitHubPullRequestCommentWriteOutcome::ProviderFailed => {
            transition_github_pr_comment_provider_failure(
                store,
                request,
                transitioned_at,
                transition_references,
                evidence_reference_count,
                provider_response,
            )
        }
        GitHubPullRequestCommentWriteOutcome::FixtureValidated
        | GitHubPullRequestCommentWriteOutcome::DryRunValidated => Err(
            GitHubPullRequestCommentProviderCallOrchestrationError::with_provider_response(
                github_write_error(
                    "github_pr_comment_provider.response.outcome_unsupported",
                    "GitHub PR comment provider call requires provider success or provider failure response",
                ),
                provider_response.clone(),
            ),
        ),
    }
}

fn transition_github_pr_comment_provider_success(
    store: &impl SideEffectRecordStore,
    request: &GitHubPullRequestCommentProviderCallRequest,
    transitioned_at: Timestamp,
    transition_references: &[SideEffectReference],
    evidence_reference_count: u32,
    provider_response: &GitHubPullRequestCommentWriteResponse,
) -> Result<
    SideEffectLifecycleTransitionResult,
    GitHubPullRequestCommentProviderCallOrchestrationError,
> {
    let provider_reference = provider_response
        .provider_comment_reference()
        .ok_or_else(|| {
            GitHubPullRequestCommentProviderCallOrchestrationError::with_provider_response(
                github_write_error(
                    "github_pr_comment_provider.response.provider_reference_missing",
                    "GitHub PR comment provider success requires a provider comment reference",
                ),
                provider_response.clone(),
            )
        })?;
    let outcome_reference = SideEffectOutcomeReference::new(
        SideEffectOutcomeReferenceKind::Outcome,
        provider_reference.to_owned(),
    )
    .map_err(|_| {
        GitHubPullRequestCommentProviderCallOrchestrationError::with_provider_response(
            github_write_error(
                "github_pr_comment_provider.response.provider_reference_invalid",
                "GitHub PR comment provider reference is invalid",
            ),
            provider_response.clone(),
        )
    })?;
    crate::transition_side_effect_to_completed_in_store(
        store,
        crate::SideEffectCompleteTransitionStoreInput {
            side_effect_id: request.side_effect_id(),
            transitioned_at,
            outcome_reference,
            summary: Some(provider_response.summary().to_owned()),
            additional_references: transition_references.to_vec(),
            evidence_reference_count,
        },
    )
    .map_err(|error| {
        GitHubPullRequestCommentProviderCallOrchestrationError::with_provider_response(
            map_github_pr_comment_outcome_transition_error(&error),
            provider_response.clone(),
        )
    })
}

fn transition_github_pr_comment_provider_failure(
    store: &impl SideEffectRecordStore,
    request: &GitHubPullRequestCommentProviderCallRequest,
    transitioned_at: Timestamp,
    transition_references: &[SideEffectReference],
    evidence_reference_count: u32,
    provider_response: &GitHubPullRequestCommentWriteResponse,
) -> Result<
    SideEffectLifecycleTransitionResult,
    GitHubPullRequestCommentProviderCallOrchestrationError,
> {
    let provider_error_code = provider_response.provider_error_code().ok_or_else(|| {
        GitHubPullRequestCommentProviderCallOrchestrationError::with_provider_response(
            github_write_error(
                "github_pr_comment_provider.response.provider_error_missing",
                "GitHub PR comment provider failure requires a provider error code",
            ),
            provider_response.clone(),
        )
    })?;
    crate::transition_side_effect_to_failed_in_store(
        store,
        crate::SideEffectFailTransitionStoreInput {
            side_effect_id: request.side_effect_id(),
            transitioned_at,
            outcome_reference: None,
            reason_codes: vec![provider_error_code.to_owned()],
            summary: Some(provider_response.summary().to_owned()),
            additional_references: transition_references.to_vec(),
            evidence_reference_count,
        },
    )
    .map_err(|error| {
        GitHubPullRequestCommentProviderCallOrchestrationError::with_provider_response(
            map_github_pr_comment_outcome_transition_error(&error),
            provider_response.clone(),
        )
    })
}

/// Classifies GitHub PR comment provider write reconciliation state.
///
/// This helper is pure and reference-first. It does not call providers, read or
/// write stores, append workflow events, emit audit records, write report
/// artifacts, mutate workflow runs, expose CLI output, add schemas/examples, or
/// change release posture.
///
/// # Errors
///
/// Returns stable, non-leaking errors when inputs are unsafe or cannot be
/// represented as a bounded reconciliation candidate.
pub fn reconcile_github_pr_comment_provider_write(
    input: GitHubPullRequestCommentProviderWriteReconciliationInput<'_>,
) -> Result<GitHubPullRequestCommentProviderWriteReconciliationCandidate, WorkflowOsError> {
    validate_github_pr_comment_attempted_outcome_record(input.attempted_record).map_err(|_| {
        github_write_error(
            "github_pr_comment_reconciliation.attempted_record.invalid",
            "GitHub PR comment reconciliation requires an attempted record",
        )
    })?;
    validate_redaction_metadata(&input.redaction)?;
    if let Some(code) = &input.local_transition_error_code {
        validate_error_code(Some(code)).map_err(|_| {
            github_write_error(
                "github_pr_comment_reconciliation.local_transition_error.invalid",
                "GitHub PR comment reconciliation local transition error code is invalid",
            )
        })?;
    }
    if let Some(code) = &input.ambiguity_error_code {
        validate_error_code(Some(code)).map_err(|_| {
            github_write_error(
                "github_pr_comment_reconciliation.ambiguity_error.invalid",
                "GitHub PR comment reconciliation ambiguity error code is invalid",
            )
        })?;
    }

    let local_transition_state = input
        .local_transition
        .map(|transition| transition.record().lifecycle_state());
    let local_transition_matches_record = input.local_transition.map_or(true, |transition| {
        transition.record().side_effect_id() == input.attempted_record.side_effect_id()
    });

    let (status, provider_reference, provider_error_code) = match input.provider_response {
        Some(response) => reconciliation_from_provider_response(
            response,
            local_transition_state,
            local_transition_matches_record,
            input.local_transition_error_code.as_deref(),
        )?,
        None if !input.provider_call_attempted && input.local_transition.is_none() => (
            GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderNotCalled,
            None,
            None,
        ),
        None if input.local_transition.is_some() => (
            GitHubPullRequestCommentProviderWriteReconciliationStatus::LocalStateAmbiguous,
            None,
            input.local_transition_error_code.clone(),
        ),
        None => (
            GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderResponseAmbiguous,
            None,
            Some(
                input
                    .ambiguity_error_code
                    .unwrap_or_else(|| "github.transport_unclassified".to_owned()),
            ),
        ),
    };

    let retry_blocked = reconciliation_status_blocks_retry(status);
    let operator_action_required = reconciliation_status_requires_operator(status);
    GitHubPullRequestCommentProviderWriteReconciliationCandidate::new(
        input.attempted_record.side_effect_id().clone(),
        input.attempted_record.idempotency().key().clone(),
        input.attempted_record.target().kind(),
        "github_pr_comment",
        local_transition_state.unwrap_or_else(|| input.attempted_record.lifecycle_state()),
        status,
        provider_reference,
        provider_error_code,
        retry_blocked,
        operator_action_required,
        input.sensitivity,
        input.redaction,
    )
}

/// Performs bounded GitHub PR comment provider lookup reconciliation.
///
/// This helper validates explicit lookup inputs, calls only the caller-supplied
/// lookup client, and classifies the bounded provider-side observation. Provider
/// lookup remains observation only: this helper does not append workflow events,
/// repair state, mutate side-effect records, write report artifacts, retry
/// provider writes, expose CLI output, add schemas/examples, or change workflow
/// semantics.
///
/// # Errors
///
/// Returns stable, non-leaking errors when pre-lookup input validation fails,
/// the injected client fails before returning a bounded response, or the
/// reconciliation result cannot be represented safely.
pub fn reconcile_github_pr_comment_provider_lookup(
    client: &impl GitHubPullRequestCommentProviderLookupClient,
    input: GitHubPullRequestCommentProviderLookupReconciliationInput<'_>,
) -> Result<GitHubPullRequestCommentProviderLookupReconciliationResult, WorkflowOsError> {
    let request = GitHubPullRequestCommentProviderLookupRequest::new(input)?;
    let response = client.lookup_pull_request_comment(&request).map_err(|_| {
        github_write_error(
            "github_pr_comment_provider_lookup_reconciliation.lookup_unavailable",
            "GitHub PR comment provider lookup failed before a bounded response was returned",
        )
    })?;
    response.validate()?;

    let (posture, observed_provider_reference, observed_match_count, provider_error_code) =
        classify_lookup_response(&request, &response)?;
    let next_action = lookup_posture_next_action(posture);

    GitHubPullRequestCommentProviderLookupReconciliationResult::new(
        request.side_effect_id().clone(),
        request.idempotency_key().clone(),
        SideEffectTargetKind::AdapterResource,
        "github_pr_comment",
        SideEffectLifecycleState::Attempted,
        posture,
        observed_provider_reference,
        observed_match_count,
        provider_error_code,
        next_action,
        conservative_max_sensitivity(request.sensitivity(), Some(response.sensitivity())),
        request.redaction.clone(),
    )
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

fn reconciliation_from_provider_response(
    response: &GitHubPullRequestCommentWriteResponse,
    local_transition_state: Option<SideEffectLifecycleState>,
    local_transition_matches_record: bool,
    local_transition_error_code: Option<&str>,
) -> Result<
    (
        GitHubPullRequestCommentProviderWriteReconciliationStatus,
        Option<String>,
        Option<String>,
    ),
    WorkflowOsError,
> {
    match response.outcome() {
        GitHubPullRequestCommentWriteOutcome::ProviderSucceeded => {
            let provider_reference = response
                .provider_comment_reference()
                .ok_or_else(|| {
                    github_write_error(
                        "github_pr_comment_reconciliation.provider_reference.missing",
                        "GitHub PR comment reconciliation requires provider reference for success",
                    )
                })?
                .to_owned();
            validate_provider_reference(
                Some(&provider_reference),
                "github_pr_comment_reconciliation.provider_reference.missing",
            )
            .map_err(|_| {
                github_write_error(
                    "github_pr_comment_reconciliation.provider_reference.invalid",
                    "GitHub PR comment reconciliation provider reference is invalid",
                )
            })?;

            let status = if local_transition_state == Some(SideEffectLifecycleState::Completed)
                && local_transition_matches_record
            {
                GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderSucceededLocalCompleted
            } else if local_transition_state.is_some() {
                GitHubPullRequestCommentProviderWriteReconciliationStatus::LocalStateAmbiguous
            } else {
                require_local_transition_error_code(
                    local_transition_error_code,
                    "github_pr_comment_reconciliation.remote_success_local_transition_failed",
                )?;
                GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderSucceededLocalTransitionFailed
            };
            Ok((status, Some(provider_reference), None))
        }
        GitHubPullRequestCommentWriteOutcome::ProviderFailed => {
            let provider_error_code = response
                .provider_error_code()
                .ok_or_else(|| {
                    github_write_error(
                        "github_pr_comment_reconciliation.provider_error.missing",
                        "GitHub PR comment reconciliation requires provider error code for failure",
                    )
                })?
                .to_owned();
            validate_error_code(Some(&provider_error_code)).map_err(|_| {
                github_write_error(
                    "github_pr_comment_reconciliation.provider_error.invalid",
                    "GitHub PR comment reconciliation provider error code is invalid",
                )
            })?;

            let status = if local_transition_state == Some(SideEffectLifecycleState::Failed)
                && local_transition_matches_record
            {
                GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderFailedLocalFailed
            } else if local_transition_state.is_some() {
                GitHubPullRequestCommentProviderWriteReconciliationStatus::LocalStateAmbiguous
            } else {
                require_local_transition_error_code(
                    local_transition_error_code,
                    "github_pr_comment_reconciliation.remote_failure_local_transition_failed",
                )?;
                GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderFailedLocalTransitionFailed
            };
            Ok((status, None, Some(provider_error_code)))
        }
        GitHubPullRequestCommentWriteOutcome::FixtureValidated
        | GitHubPullRequestCommentWriteOutcome::DryRunValidated => Err(github_write_error(
            "github_pr_comment_reconciliation.provider_response.unsupported",
            "GitHub PR comment reconciliation requires provider success, provider failure, or ambiguity",
        )),
    }
}

fn require_local_transition_error_code(
    value: Option<&str>,
    code: &'static str,
) -> Result<(), WorkflowOsError> {
    let Some(value) = value else {
        return Err(github_write_error(
            code,
            "GitHub PR comment reconciliation requires a bounded local transition error code",
        ));
    };
    validate_error_code(Some(value)).map_err(|_| {
        github_write_error(
            "github_pr_comment_reconciliation.local_transition_error.invalid",
            "GitHub PR comment reconciliation local transition error code is invalid",
        )
    })
}

fn reconciliation_status_blocks_retry(
    status: GitHubPullRequestCommentProviderWriteReconciliationStatus,
) -> bool {
    matches!(
        status,
        GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderSucceededLocalTransitionFailed
            | GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderFailedLocalTransitionFailed
            | GitHubPullRequestCommentProviderWriteReconciliationStatus::ProviderResponseAmbiguous
            | GitHubPullRequestCommentProviderWriteReconciliationStatus::LocalStateAmbiguous
            | GitHubPullRequestCommentProviderWriteReconciliationStatus::ReconciliationRequired
    )
}

fn reconciliation_status_requires_operator(
    status: GitHubPullRequestCommentProviderWriteReconciliationStatus,
) -> bool {
    reconciliation_status_blocks_retry(status)
}

fn classify_lookup_response(
    request: &GitHubPullRequestCommentProviderLookupRequest,
    response: &GitHubPullRequestCommentProviderLookupResponse,
) -> Result<
    (
        GitHubPullRequestCommentProviderLookupReconciliationPosture,
        Option<String>,
        u32,
        Option<String>,
    ),
    WorkflowOsError,
> {
    match response.outcome() {
        GitHubPullRequestCommentProviderLookupOutcome::Completed => {
            let matches = matching_lookup_observations(request, response);
            if matches.is_empty() && response.observations().is_empty() {
                return Ok((
                    GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentAbsent,
                    None,
                    0,
                    response.provider_error_code().map(ToOwned::to_owned),
                ));
            }
            if matches.len() == 1 {
                let provider_reference = matches[0]
                    .provider_comment_reference()
                    .ok_or_else(|| {
                        github_write_error(
                            "github_pr_comment_provider_lookup_reconciliation.provider_reference_missing",
                            "GitHub PR comment lookup match requires a provider reference",
                        )
                    })?
                    .to_owned();
                return Ok((
                    GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentObserved,
                    Some(provider_reference),
                    1,
                    response.provider_error_code().map(ToOwned::to_owned),
                ));
            }
            Ok((
                GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentAmbiguous,
                None,
                matches.len().try_into().map_err(|_| {
                    github_write_error(
                        "github_pr_comment_provider_lookup_reconciliation.match_count_too_large",
                        "GitHub PR comment lookup match count is too large",
                    )
                })?,
                response.provider_error_code().map(ToOwned::to_owned),
            ))
        }
        GitHubPullRequestCommentProviderLookupOutcome::NotAuthorized => Ok((
            GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupNotAuthorized,
            None,
            0,
            response.provider_error_code().map(ToOwned::to_owned),
        )),
        GitHubPullRequestCommentProviderLookupOutcome::Unavailable => Ok((
            GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupUnavailable,
            None,
            0,
            response.provider_error_code().map(ToOwned::to_owned),
        )),
        GitHubPullRequestCommentProviderLookupOutcome::RateLimited => Ok((
            GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupRateLimited,
            None,
            0,
            response.provider_error_code().map(ToOwned::to_owned),
        )),
        GitHubPullRequestCommentProviderLookupOutcome::ResponseUntrusted => Ok((
            GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupResponseUntrusted,
            None,
            0,
            response.provider_error_code().map(ToOwned::to_owned),
        )),
    }
}

fn matching_lookup_observations<'a>(
    request: &GitHubPullRequestCommentProviderLookupRequest,
    response: &'a GitHubPullRequestCommentProviderLookupResponse,
) -> Vec<&'a GitHubPullRequestCommentProviderLookupObservation> {
    response
        .observations()
        .iter()
        .filter(|observation| {
            observation.target().reference() == request.target().reference()
                && lookup_observation_matches_expected_signal(request, observation)
        })
        .collect()
}

fn lookup_observation_matches_expected_signal(
    request: &GitHubPullRequestCommentProviderLookupRequest,
    observation: &GitHubPullRequestCommentProviderLookupObservation,
) -> bool {
    let provider_reference_matches = request
        .expected_provider_reference()
        .is_some_and(|expected| observation.provider_comment_reference() == Some(expected));
    let marker_matches = request
        .expected_managed_marker()
        .is_some_and(|expected| observation.managed_marker() == Some(expected));
    provider_reference_matches || marker_matches
}

fn lookup_posture_blocks_retry(
    posture: GitHubPullRequestCommentProviderLookupReconciliationPosture,
) -> bool {
    matches!(
        posture,
        GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentObserved
            | GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentAmbiguous
            | GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupNotAuthorized
            | GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupUnavailable
            | GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupRateLimited
            | GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupTargetInvalid
            | GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupResponseUntrusted
    )
}

fn lookup_posture_requires_operator(
    posture: GitHubPullRequestCommentProviderLookupReconciliationPosture,
) -> bool {
    posture != GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentAbsent
}

fn lookup_posture_next_action(
    posture: GitHubPullRequestCommentProviderLookupReconciliationPosture,
) -> GitHubPullRequestCommentProviderLookupReconciliationNextAction {
    match posture {
        GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentObserved => {
            GitHubPullRequestCommentProviderLookupReconciliationNextAction::PlanManualStateRepair
        }
        GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentAbsent => {
            GitHubPullRequestCommentProviderLookupReconciliationNextAction::ReevaluateRetryEligibility
        }
        GitHubPullRequestCommentProviderLookupReconciliationPosture::RemoteCommentAmbiguous => {
            GitHubPullRequestCommentProviderLookupReconciliationNextAction::ResolveRemoteAmbiguity
        }
        GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupNotAuthorized => {
            GitHubPullRequestCommentProviderLookupReconciliationNextAction::ProvideAuthorizedLookup
        }
        GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupUnavailable
        | GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupRateLimited => {
            GitHubPullRequestCommentProviderLookupReconciliationNextAction::RetryLookupLater
        }
        GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupTargetInvalid
        | GitHubPullRequestCommentProviderLookupReconciliationPosture::LookupResponseUntrusted => {
            GitHubPullRequestCommentProviderLookupReconciliationNextAction::FixLookupInput
        }
    }
}

fn validate_provider_kind(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(github_write_error(
            "github_pr_comment_reconciliation.provider_kind.empty",
            "GitHub PR comment reconciliation provider kind cannot be empty",
        ));
    }
    if value.len() > GITHUB_PROVIDER_REFERENCE_MAX_BYTES {
        return Err(github_write_error(
            "github_pr_comment_reconciliation.provider_kind.too_long",
            "GitHub PR comment reconciliation provider kind is too long",
        ));
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(github_write_error(
            "github_pr_comment_reconciliation.provider_kind.invalid",
            "GitHub PR comment reconciliation provider kind contains an invalid character",
        ));
    }
    validate_not_secret_like("GitHub PR comment reconciliation provider kind", value)
}

fn validate_lookup_managed_marker(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(github_write_error(
            "github_pr_comment_provider_lookup_reconciliation.managed_marker_empty",
            "GitHub PR comment lookup managed marker cannot be empty",
        ));
    }
    if value.len() > GITHUB_PROVIDER_LOOKUP_MARKER_MAX_BYTES {
        return Err(github_write_error(
            "github_pr_comment_provider_lookup_reconciliation.managed_marker_too_long",
            "GitHub PR comment lookup managed marker is too long",
        ));
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
    {
        return Err(github_write_error(
            "github_pr_comment_provider_lookup_reconciliation.managed_marker_invalid",
            "GitHub PR comment lookup managed marker contains an invalid character",
        ));
    }
    validate_not_secret_like("GitHub PR comment lookup managed marker", value)
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

fn validate_provider_call_input(
    input: &GitHubPullRequestCommentProviderCallInput<'_>,
) -> Result<(), WorkflowOsError> {
    input.attempted_record.validate().map_err(|_| {
        github_write_error(
            "github_pr_comment_provider.record.invalid",
            "GitHub PR comment provider call requires a valid attempted SideEffect record",
        )
    })?;
    validate_github_pr_comment_attempted_outcome_record(input.attempted_record)?;

    let expected_target = input.target.reference();
    if input.attempted_record.target().kind() != SideEffectTargetKind::AdapterResource
        || input.attempted_record.target().reference() != expected_target
    {
        return Err(github_write_error(
            "github_pr_comment_provider.target.mismatch",
            "GitHub PR comment provider call target must match the attempted SideEffect record",
        ));
    }
    if input.attempted_record.idempotency().key() != &input.idempotency_key {
        return Err(github_write_error(
            "github_pr_comment_provider.idempotency.mismatch",
            "GitHub PR comment provider call idempotency must match the attempted SideEffect record",
        ));
    }
    if input
        .attempted_record
        .authority()
        .policy_references
        .is_empty()
    {
        return Err(github_write_error(
            "github_pr_comment_provider.policy_reference.missing",
            "GitHub PR comment provider call requires policy references",
        ));
    }
    if input.attempted_record.authority().decision == SideEffectAuthorityDecision::ApprovedByHuman
        && input
            .attempted_record
            .authority()
            .approval_references
            .is_empty()
    {
        return Err(github_write_error(
            "github_pr_comment_provider.approval_reference.missing",
            "GitHub PR comment provider call requires approval references for human-approved authority",
        ));
    }
    if input.mode != GitHubPullRequestCommentWriteMode::LiveSandbox {
        return Err(github_write_error(
            "github_pr_comment_provider.mode.unsupported",
            "GitHub PR comment provider call requires explicit live sandbox mode",
        ));
    }
    if !input.live_call_enabled {
        return Err(github_write_error(
            "github_pr_comment_provider.live_call_disabled",
            "GitHub PR comment provider call requires explicit live-call enablement",
        ));
    }
    if !input.provider_call_enabled {
        return Err(github_write_error(
            "github_pr_comment_provider.provider_call_disabled",
            "GitHub PR comment provider call requires explicit provider-call enablement",
        ));
    }

    input.target.validate()?;
    validate_comment_body(&input.comment_body)?;
    input.auth.validate()?;
    validate_summary("provider call summary", &input.summary)?;
    validate_redaction_metadata(&input.redaction)?;
    Ok(())
}

fn validate_provider_http_base_url(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(github_write_error(
            "github_pr_comment_provider_http.base_url.empty",
            "GitHub PR comment provider HTTP base URL cannot be empty",
        ));
    }
    if value.len() > GITHUB_PROVIDER_HTTP_BASE_URL_MAX_BYTES {
        return Err(github_write_error(
            "github_pr_comment_provider_http.base_url.too_long",
            format!(
                "GitHub PR comment provider HTTP base URL cannot exceed {GITHUB_PROVIDER_HTTP_BASE_URL_MAX_BYTES} bytes"
            ),
        ));
    }
    if !value.starts_with("https://") && !value.starts_with("http://localhost") {
        return Err(github_write_error(
            "github_pr_comment_provider_http.base_url.unsupported",
            "GitHub PR comment provider HTTP base URL must be HTTPS or localhost",
        ));
    }
    if value.contains('?') || value.contains('#') || value.contains(' ') {
        return Err(github_write_error(
            "github_pr_comment_provider_http.base_url.invalid",
            "GitHub PR comment provider HTTP base URL contains an invalid character",
        ));
    }
    validate_not_secret_like("GitHub PR comment provider HTTP base URL", value)
}

fn validate_provider_http_url(value: &str) -> Result<(), WorkflowOsError> {
    if value.len() > GITHUB_PROVIDER_HTTP_BASE_URL_MAX_BYTES + (GITHUB_NAME_MAX_BYTES * 2) + 64 {
        return Err(github_write_error(
            "github_pr_comment_provider_http.url.too_long",
            "GitHub PR comment provider HTTP URL exceeds the allowed size",
        ));
    }
    validate_not_secret_like("GitHub PR comment provider HTTP URL", value)
}

fn validate_provider_comment_id(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(github_write_error(
            "github_pr_comment_provider_http.comment_id.empty",
            "GitHub PR comment provider comment ID cannot be empty",
        ));
    }
    if value.len() > GITHUB_PROVIDER_HTTP_COMMENT_ID_MAX_BYTES {
        return Err(github_write_error(
            "github_pr_comment_provider_http.comment_id.too_long",
            format!(
                "GitHub PR comment provider comment ID cannot exceed {GITHUB_PROVIDER_HTTP_COMMENT_ID_MAX_BYTES} bytes"
            ),
        ));
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(github_write_error(
            "github_pr_comment_provider_http.comment_id.invalid",
            "GitHub PR comment provider comment ID contains an invalid character",
        ));
    }
    validate_not_secret_like("GitHub PR comment provider comment ID", value)
}

fn provider_comment_reference(
    target: &GitHubPullRequestCommentTarget,
    provider_comment_id: &str,
) -> Result<String, WorkflowOsError> {
    validate_provider_comment_id(provider_comment_id)?;
    let reference = format!(
        "github/pr-comment/{}/{}/{}/{}",
        target.owner(),
        target.repository(),
        target.pull_request_number(),
        provider_comment_id
    );
    validate_provider_reference(
        Some(&reference),
        "github_pr_comment_provider_http.provider_reference.missing",
    )?;
    Ok(reference)
}

fn classify_provider_http_status(status: u16) -> &'static str {
    match status {
        401 => "github.auth_failed",
        403 => "github.forbidden",
        404 => "github.not_found",
        408 => "github.timeout",
        409 => "github.conflict",
        422 => "github.validation_failed",
        429 => "github.rate_limited",
        500..=599 => "github.server_error",
        _ => "github.transport_unclassified",
    }
}

fn classify_lookup_http_status(
    status: u16,
) -> (
    GitHubPullRequestCommentProviderLookupOutcome,
    Option<&'static str>,
) {
    match status {
        200..=299 => (
            GitHubPullRequestCommentProviderLookupOutcome::Completed,
            None,
        ),
        401 => (
            GitHubPullRequestCommentProviderLookupOutcome::NotAuthorized,
            Some("github.auth_failed"),
        ),
        403 => (
            GitHubPullRequestCommentProviderLookupOutcome::NotAuthorized,
            Some("github.forbidden"),
        ),
        404 => (
            GitHubPullRequestCommentProviderLookupOutcome::Unavailable,
            Some("github.not_found"),
        ),
        408 => (
            GitHubPullRequestCommentProviderLookupOutcome::Unavailable,
            Some("github.timeout"),
        ),
        429 => (
            GitHubPullRequestCommentProviderLookupOutcome::RateLimited,
            Some("github.rate_limited"),
        ),
        500..=599 => (
            GitHubPullRequestCommentProviderLookupOutcome::Unavailable,
            Some("github.server_error"),
        ),
        _ => (
            GitHubPullRequestCommentProviderLookupOutcome::ResponseUntrusted,
            Some("github.response_untrusted"),
        ),
    }
}

fn provider_failure_summary(provider_error_code: &str) -> &'static str {
    match provider_error_code {
        "github.auth_failed" => "GitHub PR comment provider authentication failed",
        "github.forbidden" => "GitHub PR comment provider permission failed",
        "github.not_found" => "GitHub PR comment provider target was not found",
        "github.timeout" => "GitHub PR comment provider request timed out",
        "github.conflict" => "GitHub PR comment provider request conflicted",
        "github.validation_failed" => "GitHub PR comment provider validation failed",
        "github.rate_limited" => "GitHub PR comment provider was rate limited",
        "github.server_error" => "GitHub PR comment provider returned a server error",
        _ => "GitHub PR comment provider returned an unclassified failure",
    }
}

fn validate_provider_call_store_record(
    record: &SideEffectRecord,
    request: &GitHubPullRequestCommentProviderCallRequest,
) -> Result<(), WorkflowOsError> {
    validate_github_pr_comment_attempted_outcome_record(record)?;
    if record.target().reference() != request.target().reference()
        || record.idempotency().key() != request.idempotency_key()
    {
        return Err(github_write_error(
            "github_pr_comment_provider.store_record.mismatch",
            "GitHub PR comment provider call store record must match the validated request",
        ));
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
