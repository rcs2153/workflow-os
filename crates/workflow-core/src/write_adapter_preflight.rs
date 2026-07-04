use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    IdempotencyKey, RedactionMetadata, SideEffectId, SideEffectReference, SideEffectReferenceKind,
    SideEffectSensitivity, WorkflowOsError, WorkflowOsErrorKind,
};

const PREFLIGHT_REFERENCE_MAX_BYTES: usize = 256;
const PREFLIGHT_SUMMARY_MAX_BYTES: usize = 512;
const PREFLIGHT_REDACTION_FIELD_MAX_BYTES: usize = 128;
const PREFLIGHT_REDACTION_REASON_MAX_BYTES: usize = 512;
const PREFLIGHT_REDACTION_MAX_ENTRIES: usize = 64;

/// Write capability vocabulary understood by the preflight helper.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterWriteCapability {
    /// Future GitHub pull request comment write.
    GitHubPullRequestComment,
    /// Future Jira issue comment write.
    JiraIssueComment,
    /// Future GitHub pull request creation write. Unsupported by the default preview policy.
    GitHubPullRequestCreate,
    /// Future GitHub merge write. Unsupported by the default preview policy.
    GitHubMerge,
    /// Future Jira issue transition write. Unsupported by the default preview policy.
    JiraIssueTransition,
    /// Future CI rerun or workflow dispatch write. Unsupported by the default preview policy.
    CiRerun,
    /// Generic provider write. Unsupported by the default preview policy.
    GenericProviderWrite,
    /// Unknown write capability. Always rejected.
    Unknown,
}

/// Bounded target kind for future adapter writes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterWriteTargetKind {
    /// GitHub pull request target.
    GitHubPullRequest,
    /// Jira issue target.
    JiraIssue,
    /// CI workflow target.
    CiWorkflow,
    /// Generic external resource target.
    ExternalResource,
    /// Unknown target kind. Always rejected.
    Unknown,
}

/// Policy posture supplied to the preflight helper.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterWritePolicyDecision {
    /// Policy allowed preflight to continue.
    Allowed,
    /// Policy denied the write.
    Denied,
}

/// Bounded, redaction-safe target reference for a future write.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct AdapterWriteTarget {
    kind: AdapterWriteTargetKind,
    reference: String,
}

impl AdapterWriteTarget {
    /// Creates a validated write target.
    ///
    /// # Errors
    ///
    /// Returns an error when the target kind or reference is unsupported,
    /// unbounded, empty, or secret-like.
    pub fn new(
        kind: AdapterWriteTargetKind,
        reference: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let target = Self {
            kind,
            reference: reference.into(),
        };
        target.validate()?;
        Ok(target)
    }

    /// Validates the write target.
    ///
    /// # Errors
    ///
    /// Returns an error when the target is invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.kind == AdapterWriteTargetKind::Unknown {
            return Err(preflight_error(
                "adapter_write_preflight.target.unknown",
                "adapter write target kind must be known",
            ));
        }
        validate_reference("adapter write target reference", &self.reference)
    }

    /// Returns the target kind.
    #[must_use]
    pub const fn kind(&self) -> AdapterWriteTargetKind {
        self.kind
    }

    /// Returns the bounded target reference.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }
}

impl fmt::Debug for AdapterWriteTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AdapterWriteTarget")
            .field("kind", &self.kind)
            .field("reference", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for AdapterWriteTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            kind: AdapterWriteTargetKind,
            reference: String,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(wire.kind, wire.reference).map_err(serde::de::Error::custom)
    }
}

/// Public definition used to create a validated write-readiness policy.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterWriteReadinessPolicyDefinition {
    /// Capabilities supported by this preflight policy.
    pub supported_capabilities: Vec<AdapterWriteCapability>,
    /// Supported capabilities that require approval references.
    #[serde(default)]
    pub sensitive_capabilities: Vec<AdapterWriteCapability>,
}

/// Deterministic write-readiness policy used by the preflight helper.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct AdapterWriteReadinessPolicy {
    supported_capabilities: Vec<AdapterWriteCapability>,
    sensitive_capabilities: Vec<AdapterWriteCapability>,
}

impl AdapterWriteReadinessPolicy {
    /// Creates the default local preview policy.
    ///
    /// The default policy only supports preflight classification for low-risk
    /// comment-shaped writes. It does not authorize provider calls.
    #[must_use]
    pub fn local_preview_comments_only() -> Self {
        Self {
            supported_capabilities: vec![
                AdapterWriteCapability::GitHubPullRequestComment,
                AdapterWriteCapability::JiraIssueComment,
            ],
            sensitive_capabilities: Vec::new(),
        }
    }

    /// Creates a validated readiness policy.
    ///
    /// # Errors
    ///
    /// Returns an error when the policy is empty, duplicated, or references
    /// unknown or unsupported sensitive capabilities.
    pub fn new(definition: AdapterWriteReadinessPolicyDefinition) -> Result<Self, WorkflowOsError> {
        let policy = Self {
            supported_capabilities: definition.supported_capabilities,
            sensitive_capabilities: definition.sensitive_capabilities,
        };
        policy.validate()?;
        Ok(policy)
    }

    /// Validates the readiness policy.
    ///
    /// # Errors
    ///
    /// Returns an error when the policy is invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.supported_capabilities.is_empty() {
            return Err(preflight_error(
                "adapter_write_preflight.policy.supported_capabilities_empty",
                "adapter write readiness policy must support at least one capability",
            ));
        }

        validate_capability_set(
            "adapter_write_preflight.policy.duplicate_supported_capability",
            "adapter write readiness policy contains duplicate supported capabilities",
            &self.supported_capabilities,
        )?;
        validate_capability_set(
            "adapter_write_preflight.policy.duplicate_sensitive_capability",
            "adapter write readiness policy contains duplicate sensitive capabilities",
            &self.sensitive_capabilities,
        )?;

        if self
            .supported_capabilities
            .contains(&AdapterWriteCapability::Unknown)
            || self
                .sensitive_capabilities
                .contains(&AdapterWriteCapability::Unknown)
        {
            return Err(preflight_error(
                "adapter_write_preflight.capability.unknown",
                "adapter write capability must be known",
            ));
        }

        for sensitive in &self.sensitive_capabilities {
            if !self.supported_capabilities.contains(sensitive) {
                return Err(preflight_error(
                    "adapter_write_preflight.policy.sensitive_capability_unsupported",
                    "sensitive adapter write capability must also be supported",
                ));
            }
        }

        Ok(())
    }

    /// Returns supported capabilities.
    #[must_use]
    pub fn supported_capabilities(&self) -> &[AdapterWriteCapability] {
        &self.supported_capabilities
    }

    /// Returns sensitive capabilities.
    #[must_use]
    pub fn sensitive_capabilities(&self) -> &[AdapterWriteCapability] {
        &self.sensitive_capabilities
    }

    fn supports(&self, capability: AdapterWriteCapability) -> bool {
        self.supported_capabilities.contains(&capability)
    }

    fn marks_sensitive(&self, capability: AdapterWriteCapability) -> bool {
        self.sensitive_capabilities.contains(&capability)
    }
}

impl Default for AdapterWriteReadinessPolicy {
    fn default() -> Self {
        Self::local_preview_comments_only()
    }
}

impl fmt::Debug for AdapterWriteReadinessPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AdapterWriteReadinessPolicy")
            .field("supported_capabilities", &self.supported_capabilities)
            .field("sensitive_capabilities", &self.sensitive_capabilities)
            .finish()
    }
}

impl<'de> Deserialize<'de> for AdapterWriteReadinessPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definition = AdapterWriteReadinessPolicyDefinition::deserialize(deserializer)?;
        Self::new(definition).map_err(serde::de::Error::custom)
    }
}

/// Public definition used to create a preflight request.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterWritePreflightRequestDefinition {
    /// Requested future write capability.
    pub capability: AdapterWriteCapability,
    /// Bounded target reference.
    pub target: AdapterWriteTarget,
    /// Proposed `SideEffect` ID for this write.
    pub side_effect_id: Option<SideEffectId>,
    /// Idempotency key that would bind the future provider attempt.
    pub idempotency_key: Option<IdempotencyKey>,
    /// Policy posture supplied by the caller.
    pub policy_decision: AdapterWritePolicyDecision,
    /// Stable policy decision references.
    #[serde(default)]
    pub policy_references: Vec<SideEffectReference>,
    /// Whether approval is explicitly required for this write.
    pub requires_approval: bool,
    /// Stable approval decision references, when required or available.
    #[serde(default)]
    pub approval_references: Vec<SideEffectReference>,
    /// Whether high-assurance approval posture is explicitly required.
    pub high_assurance_required: bool,
    /// Stable references supporting high-assurance posture.
    #[serde(default)]
    pub high_assurance_references: Vec<SideEffectReference>,
    /// Bounded purpose or impact summary.
    pub summary: String,
    /// Sensitivity assigned to this preflight context.
    pub sensitivity: SideEffectSensitivity,
    /// Redaction metadata for this preflight context.
    pub redaction: RedactionMetadata,
    /// Readiness policy used to classify the request.
    pub readiness_policy: AdapterWriteReadinessPolicy,
}

/// Validated, redaction-safe request for write adapter preflight.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct AdapterWritePreflightRequest {
    capability: AdapterWriteCapability,
    target: AdapterWriteTarget,
    side_effect_id: Option<SideEffectId>,
    idempotency_key: Option<IdempotencyKey>,
    policy_decision: AdapterWritePolicyDecision,
    policy_references: Vec<SideEffectReference>,
    requires_approval: bool,
    approval_references: Vec<SideEffectReference>,
    high_assurance_required: bool,
    high_assurance_references: Vec<SideEffectReference>,
    summary: String,
    sensitivity: SideEffectSensitivity,
    redaction: RedactionMetadata,
    readiness_policy: AdapterWriteReadinessPolicy,
}

impl AdapterWritePreflightRequest {
    /// Creates a validated preflight request.
    ///
    /// # Errors
    ///
    /// Returns an error when any readiness input is invalid or unsafe.
    pub fn new(
        definition: AdapterWritePreflightRequestDefinition,
    ) -> Result<Self, WorkflowOsError> {
        let request = Self {
            capability: definition.capability,
            target: definition.target,
            side_effect_id: definition.side_effect_id,
            idempotency_key: definition.idempotency_key,
            policy_decision: definition.policy_decision,
            policy_references: definition.policy_references,
            requires_approval: definition.requires_approval,
            approval_references: definition.approval_references,
            high_assurance_required: definition.high_assurance_required,
            high_assurance_references: definition.high_assurance_references,
            summary: definition.summary,
            sensitivity: definition.sensitivity,
            redaction: definition.redaction,
            readiness_policy: definition.readiness_policy,
        };
        request.validate()?;
        Ok(request)
    }

    /// Validates the preflight request.
    ///
    /// # Errors
    ///
    /// Returns an error when the request is not safe to classify.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.target.validate()?;
        self.readiness_policy.validate()?;
        validate_summary("adapter write preflight summary", &self.summary)?;
        validate_redaction_metadata(&self.redaction)?;

        if self.capability == AdapterWriteCapability::Unknown {
            return Err(preflight_error(
                "adapter_write_preflight.capability.unknown",
                "adapter write capability must be known",
            ));
        }

        Ok(())
    }

    /// Returns the requested capability.
    #[must_use]
    pub const fn capability(&self) -> AdapterWriteCapability {
        self.capability
    }

    /// Returns the bounded target.
    #[must_use]
    pub const fn target(&self) -> &AdapterWriteTarget {
        &self.target
    }

    /// Returns the proposed `SideEffect` ID.
    #[must_use]
    pub const fn side_effect_id(&self) -> Option<&SideEffectId> {
        self.side_effect_id.as_ref()
    }

    /// Returns the idempotency key.
    #[must_use]
    pub const fn idempotency_key(&self) -> Option<&IdempotencyKey> {
        self.idempotency_key.as_ref()
    }

    /// Returns policy references.
    #[must_use]
    pub fn policy_references(&self) -> &[SideEffectReference] {
        &self.policy_references
    }

    /// Returns approval references.
    #[must_use]
    pub fn approval_references(&self) -> &[SideEffectReference] {
        &self.approval_references
    }

    /// Returns high-assurance references.
    #[must_use]
    pub fn high_assurance_references(&self) -> &[SideEffectReference] {
        &self.high_assurance_references
    }

    /// Returns the bounded summary.
    #[must_use]
    pub fn summary(&self) -> &str {
        &self.summary
    }

    /// Returns the sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> SideEffectSensitivity {
        self.sensitivity
    }
}

impl fmt::Debug for AdapterWritePreflightRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AdapterWritePreflightRequest")
            .field("capability", &self.capability)
            .field("target", &self.target)
            .field(
                "side_effect_id",
                &self.side_effect_id.as_ref().map(|_| "[REDACTED]"),
            )
            .field(
                "idempotency_key",
                &self.idempotency_key.as_ref().map(|_| "[REDACTED]"),
            )
            .field("policy_reference_count", &self.policy_references.len())
            .field("policy_decision", &self.policy_decision)
            .field("requires_approval", &self.requires_approval)
            .field("approval_reference_count", &self.approval_references.len())
            .field("high_assurance_required", &self.high_assurance_required)
            .field(
                "high_assurance_reference_count",
                &self.high_assurance_references.len(),
            )
            .field("summary", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .field("readiness_policy", &self.readiness_policy)
            .finish()
    }
}

impl<'de> Deserialize<'de> for AdapterWritePreflightRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definition = AdapterWritePreflightRequestDefinition::deserialize(deserializer)?;
        Self::new(definition).map_err(serde::de::Error::custom)
    }
}

/// Preflight boundary for operations this helper must not perform.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterWritePreflightOperationBoundary {
    /// The operation is outside this helper's authority.
    Prohibited,
}

impl AdapterWritePreflightOperationBoundary {
    const fn allows(self) -> bool {
        match self {
            Self::Prohibited => false,
        }
    }
}

/// Explicit no-execution boundary returned by write preflight.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterWritePreflightExecutionBoundary {
    provider_call: AdapterWritePreflightOperationBoundary,
    side_effect_lifecycle_transition: AdapterWritePreflightOperationBoundary,
    workflow_event_append: AdapterWritePreflightOperationBoundary,
    report_artifact_write: AdapterWritePreflightOperationBoundary,
}

impl AdapterWritePreflightExecutionBoundary {
    const fn prohibited() -> Self {
        Self {
            provider_call: AdapterWritePreflightOperationBoundary::Prohibited,
            side_effect_lifecycle_transition: AdapterWritePreflightOperationBoundary::Prohibited,
            workflow_event_append: AdapterWritePreflightOperationBoundary::Prohibited,
            report_artifact_write: AdapterWritePreflightOperationBoundary::Prohibited,
        }
    }
}

/// Deterministic decision returned by preflight.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterWritePreflightDecision {
    capability: AdapterWriteCapability,
    side_effect_id: SideEffectId,
    idempotency_key: IdempotencyKey,
    policy_reference_count: usize,
    approval_reference_count: usize,
    high_assurance_reference_count: usize,
    reason_codes: Vec<String>,
    execution_boundary: AdapterWritePreflightExecutionBoundary,
}

impl AdapterWritePreflightDecision {
    /// Returns the preflighted capability.
    #[must_use]
    pub const fn capability(&self) -> AdapterWriteCapability {
        self.capability
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

    /// Returns stable reason codes.
    #[must_use]
    pub fn reason_codes(&self) -> &[String] {
        &self.reason_codes
    }

    /// Returns whether this helper allows provider calls.
    #[must_use]
    pub const fn provider_call_allowed(&self) -> bool {
        self.execution_boundary.provider_call.allows()
    }

    /// Returns whether this helper allows side-effect lifecycle transitions.
    #[must_use]
    pub const fn side_effect_lifecycle_transition_allowed(&self) -> bool {
        self.execution_boundary
            .side_effect_lifecycle_transition
            .allows()
    }

    /// Returns whether this helper allows workflow event appends.
    #[must_use]
    pub const fn workflow_event_append_allowed(&self) -> bool {
        self.execution_boundary.workflow_event_append.allows()
    }

    /// Returns whether this helper allows report artifact writes.
    #[must_use]
    pub const fn report_artifact_write_allowed(&self) -> bool {
        self.execution_boundary.report_artifact_write.allows()
    }
}

impl fmt::Debug for AdapterWritePreflightDecision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AdapterWritePreflightDecision")
            .field("capability", &self.capability)
            .field("side_effect_id", &"[REDACTED]")
            .field("idempotency_key", &"[REDACTED]")
            .field("policy_reference_count", &self.policy_reference_count)
            .field("approval_reference_count", &self.approval_reference_count)
            .field(
                "high_assurance_reference_count",
                &self.high_assurance_reference_count,
            )
            .field("reason_codes", &self.reason_codes)
            .field("execution_boundary", &self.execution_boundary)
            .finish()
    }
}

/// Performs deterministic write-adapter preflight without provider calls.
///
/// # Errors
///
/// Returns a stable non-leaking error when required governance posture is
/// missing, unsupported, denied, or unsafe.
pub fn preflight_adapter_write(
    request: &AdapterWritePreflightRequest,
) -> Result<AdapterWritePreflightDecision, WorkflowOsError> {
    request.validate()?;

    if !request.readiness_policy.supports(request.capability) {
        return Err(preflight_error(
            "adapter_write_preflight.capability.unsupported",
            "adapter write capability is not supported by this readiness policy",
        ));
    }

    let side_effect_id = request.side_effect_id.clone().ok_or_else(|| {
        preflight_error(
            "adapter_write_preflight.side_effect.missing",
            "adapter write preflight requires a proposed side-effect id",
        )
    })?;

    let idempotency_key = request.idempotency_key.clone().ok_or_else(|| {
        preflight_error(
            "adapter_write_preflight.idempotency.missing",
            "adapter write preflight requires an idempotency key",
        )
    })?;

    if request.policy_decision == AdapterWritePolicyDecision::Denied {
        return Err(WorkflowOsError::new(
            WorkflowOsErrorKind::PolicyDenied,
            "adapter_write_preflight.policy.denied",
            "adapter write preflight was denied by policy",
        ));
    }

    validate_policy_references(&request.policy_references)?;

    let sensitivity_requires_approval = matches!(
        request.sensitivity,
        SideEffectSensitivity::Confidential
            | SideEffectSensitivity::Regulated
            | SideEffectSensitivity::Secret
            | SideEffectSensitivity::Unknown
    );
    let approval_required = request.requires_approval
        || request.readiness_policy.marks_sensitive(request.capability)
        || sensitivity_requires_approval;

    if approval_required {
        validate_approval_references(&request.approval_references)?;
    } else {
        validate_optional_references(
            &request.approval_references,
            SideEffectReferenceKind::ApprovalDecision,
            "adapter_write_preflight.approval.reference_kind",
            "adapter write approval references must cite approval decisions",
        )?;
    }

    if request.high_assurance_required {
        validate_high_assurance_references(&request.high_assurance_references)?;
    } else {
        validate_optional_high_assurance_references(&request.high_assurance_references)?;
    }

    let mut reason_codes = vec![
        "adapter_write_preflight.ready".to_owned(),
        "adapter_write_preflight.no_provider_call".to_owned(),
        "adapter_write_preflight.no_side_effect_transition".to_owned(),
    ];
    if approval_required {
        reason_codes.push("adapter_write_preflight.approval_verified".to_owned());
    }
    if request.high_assurance_required {
        reason_codes.push("adapter_write_preflight.high_assurance_verified".to_owned());
    }

    Ok(AdapterWritePreflightDecision {
        capability: request.capability,
        side_effect_id,
        idempotency_key,
        policy_reference_count: request.policy_references.len(),
        approval_reference_count: request.approval_references.len(),
        high_assurance_reference_count: request.high_assurance_references.len(),
        reason_codes,
        execution_boundary: AdapterWritePreflightExecutionBoundary::prohibited(),
    })
}

fn validate_capability_set(
    duplicate_code: &'static str,
    duplicate_message: &'static str,
    capabilities: &[AdapterWriteCapability],
) -> Result<(), WorkflowOsError> {
    let mut seen = BTreeSet::new();
    for capability in capabilities {
        if !seen.insert(*capability) {
            return Err(preflight_error(duplicate_code, duplicate_message));
        }
    }
    Ok(())
}

fn validate_policy_references(references: &[SideEffectReference]) -> Result<(), WorkflowOsError> {
    if references.is_empty() {
        return Err(preflight_error(
            "adapter_write_preflight.policy.missing",
            "adapter write preflight requires a policy decision reference",
        ));
    }
    validate_optional_references(
        references,
        SideEffectReferenceKind::PolicyDecision,
        "adapter_write_preflight.policy.reference_kind",
        "adapter write policy references must cite policy decisions",
    )
}

fn validate_approval_references(references: &[SideEffectReference]) -> Result<(), WorkflowOsError> {
    if references.is_empty() {
        return Err(preflight_error(
            "adapter_write_preflight.approval.missing",
            "adapter write preflight requires an approval decision reference",
        ));
    }
    validate_optional_references(
        references,
        SideEffectReferenceKind::ApprovalDecision,
        "adapter_write_preflight.approval.reference_kind",
        "adapter write approval references must cite approval decisions",
    )
}

fn validate_optional_references(
    references: &[SideEffectReference],
    expected_kind: SideEffectReferenceKind,
    code: &'static str,
    message: &'static str,
) -> Result<(), WorkflowOsError> {
    let mut seen = BTreeSet::new();
    for reference in references {
        reference.validate()?;
        if reference.kind() != expected_kind {
            return Err(preflight_error(code, message));
        }
        if !seen.insert(reference) {
            return Err(preflight_error(
                "adapter_write_preflight.reference.duplicate",
                "adapter write preflight contains duplicate references",
            ));
        }
    }
    Ok(())
}

fn validate_high_assurance_references(
    references: &[SideEffectReference],
) -> Result<(), WorkflowOsError> {
    if references.is_empty() {
        return Err(preflight_error(
            "adapter_write_preflight.high_assurance.missing",
            "adapter write preflight requires high-assurance references",
        ));
    }
    validate_optional_high_assurance_references(references)
}

fn validate_optional_high_assurance_references(
    references: &[SideEffectReference],
) -> Result<(), WorkflowOsError> {
    let mut seen = BTreeSet::new();
    for reference in references {
        reference.validate()?;
        if !matches!(
            reference.kind(),
            SideEffectReferenceKind::ApprovalDecision
                | SideEffectReferenceKind::EvidenceReference
                | SideEffectReferenceKind::LocalCheckResult
                | SideEffectReferenceKind::WorkReport
        ) {
            return Err(preflight_error(
                "adapter_write_preflight.high_assurance.reference_kind",
                "adapter write high-assurance references must cite approval, evidence, local check, or work report references",
            ));
        }
        if !seen.insert(reference) {
            return Err(preflight_error(
                "adapter_write_preflight.reference.duplicate",
                "adapter write preflight contains duplicate references",
            ));
        }
    }
    Ok(())
}

fn validate_reference(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(preflight_error(
            "adapter_write_preflight.reference.empty",
            format!("{type_name} cannot be empty"),
        ));
    }
    if value.len() > PREFLIGHT_REFERENCE_MAX_BYTES {
        return Err(preflight_error(
            "adapter_write_preflight.reference.too_long",
            format!("{type_name} cannot exceed {PREFLIGHT_REFERENCE_MAX_BYTES} bytes"),
        ));
    }
    validate_not_secret_like(type_name, value)
}

fn validate_summary(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(preflight_error(
            "adapter_write_preflight.summary.empty",
            format!("{type_name} cannot be empty"),
        ));
    }
    if value.len() > PREFLIGHT_SUMMARY_MAX_BYTES {
        return Err(preflight_error(
            "adapter_write_preflight.summary.too_long",
            format!("{type_name} cannot exceed {PREFLIGHT_SUMMARY_MAX_BYTES} bytes"),
        ));
    }
    validate_not_secret_like(type_name, value)
}

fn validate_redaction_metadata(redaction: &RedactionMetadata) -> Result<(), WorkflowOsError> {
    if redaction.redacted_fields.len() > PREFLIGHT_REDACTION_MAX_ENTRIES {
        return Err(preflight_error(
            "adapter_write_preflight.redaction.too_many_fields",
            "adapter write preflight redaction metadata contains too many fields",
        ));
    }
    if redaction.field_states.len() > PREFLIGHT_REDACTION_MAX_ENTRIES {
        return Err(preflight_error(
            "adapter_write_preflight.redaction.too_many_states",
            "adapter write preflight redaction metadata contains too many field states",
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
        return Err(preflight_error(
            "adapter_write_preflight.redaction.field.empty",
            "adapter write preflight redaction field cannot be empty",
        ));
    }
    if value.len() > PREFLIGHT_REDACTION_FIELD_MAX_BYTES {
        return Err(preflight_error(
            "adapter_write_preflight.redaction.field.too_long",
            format!(
                "adapter write preflight redaction field cannot exceed {PREFLIGHT_REDACTION_FIELD_MAX_BYTES} bytes"
            ),
        ));
    }
    validate_not_secret_like("adapter write preflight redaction field", value)
}

fn validate_redaction_reason(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(preflight_error(
            "adapter_write_preflight.redaction.reason.empty",
            "adapter write preflight redaction reason cannot be empty",
        ));
    }
    if value.len() > PREFLIGHT_REDACTION_REASON_MAX_BYTES {
        return Err(preflight_error(
            "adapter_write_preflight.redaction.reason.too_long",
            format!(
                "adapter write preflight redaction reason cannot exceed {PREFLIGHT_REDACTION_REASON_MAX_BYTES} bytes"
            ),
        ));
    }
    validate_not_secret_like("adapter write preflight redaction reason", value)
}

fn validate_not_secret_like(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    let lowercase = value.to_ascii_lowercase();
    let is_secret_like = lowercase.contains("authorization")
        || lowercase.contains("bearer")
        || lowercase.contains("private_key")
        || lowercase.contains("private-key")
        || lowercase.contains("api_token")
        || lowercase.contains("api-token")
        || lowercase.contains("secret")
        || lowercase.contains("token");

    if is_secret_like {
        return Err(preflight_error(
            "adapter_write_preflight.secret_like_value",
            format!("{type_name} contains sensitive-looking text"),
        ));
    }

    Ok(())
}

fn preflight_error(code: &'static str, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}
