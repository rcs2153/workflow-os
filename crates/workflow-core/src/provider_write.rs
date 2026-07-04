use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ActorId, AdapterId, AdapterWriteCapability, AdapterWritePolicyDecision,
    AdapterWritePreflightRequest, AdapterWritePreflightRequestDefinition, AdapterWriteTargetKind,
    CorrelationId, IdempotencyKey, IntegrationId, RedactionMetadata, SchemaVersion, SideEffectId,
    SideEffectReference, SideEffectSensitivity, SpecContentHash, StepId, WorkflowId,
    WorkflowOsError, WorkflowRunId, WorkflowVersion,
};

const GITHUB_NAME_MAX_BYTES: usize = 100;
const GITHUB_COMMENT_BODY_MAX_BYTES: usize = 4 * 1024;
const GITHUB_WRITE_SUMMARY_MAX_BYTES: usize = 512;
const GITHUB_PROVIDER_REFERENCE_MAX_BYTES: usize = 256;
const GITHUB_WRITE_REDACTION_FIELD_MAX_BYTES: usize = 128;
const GITHUB_WRITE_REDACTION_REASON_MAX_BYTES: usize = 512;
const GITHUB_WRITE_REDACTION_MAX_ENTRIES: usize = 64;

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
