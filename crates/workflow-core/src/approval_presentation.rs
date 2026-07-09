use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    ActorId, ApprovalRequest, RedactionMetadata, SchemaVersion, StepId, Timestamp, WorkflowId,
    WorkflowOsError, WorkflowRunId, WorkflowVersion,
};

const PRESENTATION_ID_MAX_BYTES: usize = 128;
const APPROVAL_ID_MAX_BYTES: usize = 128;
const PRESENTATION_TEXT_MAX_BYTES: usize = 512;
const PRESENTATION_COLLECTION_MAX_ITEMS: usize = 32;
const PRESENTATION_CHANNEL_MAX_BYTES: usize = 128;
const PRESENTATION_REDACTION_FIELD_MAX_BYTES: usize = 128;
const PRESENTATION_REDACTION_REASON_MAX_BYTES: usize = 512;
const PRESENTATION_REDACTION_MAX_ENTRIES: usize = 64;
const PROOF_FRESHNESS_MAX_MS: u64 = 86_400_000;

/// Identifier for one approval-presentation proof record.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ApprovalPresentationId(String);

impl ApprovalPresentationId {
    /// Creates a validated approval-presentation ID.
    ///
    /// # Errors
    ///
    /// Returns an error when the ID is empty, too long, contains unsupported
    /// characters, or looks like a secret.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier(
            "approval presentation id",
            &value,
            PRESENTATION_ID_MAX_BYTES,
        )?;
        Ok(Self(value))
    }

    /// Returns the ID as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ApprovalPresentationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for ApprovalPresentationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ApprovalPresentationId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<ApprovalPresentationId> for String {
    fn from(value: ApprovalPresentationId) -> Self {
        value.0
    }
}

impl TryFrom<String> for ApprovalPresentationId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for ApprovalPresentationId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Deterministic SHA-256 content hash for canonical approval-presentation content.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ApprovalPresentationContentHash(String);

impl ApprovalPresentationContentHash {
    /// Creates a content hash from lowercase SHA-256 hex text.
    ///
    /// # Errors
    ///
    /// Returns an error when the value is not a lowercase SHA-256 hex digest.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        let is_valid = value.len() == 64
            && value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));

        if !is_valid {
            return Err(validation_error(
                "approval_presentation.content_hash.invalid",
                "approval presentation content hash must be a lowercase SHA-256 hex digest",
            ));
        }

        Ok(Self(value))
    }

    /// Hashes canonical approval-presentation text using SHA-256.
    #[must_use]
    pub fn from_canonical_text(text: &str) -> Self {
        let digest = Sha256::digest(text.as_bytes());
        Self(hex_lower(&digest))
    }

    /// Returns the lowercase hexadecimal digest.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ApprovalPresentationContentHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for ApprovalPresentationContentHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ApprovalPresentationContentHash")
            .field(&self.0)
            .finish()
    }
}

impl From<ApprovalPresentationContentHash> for String {
    fn from(value: ApprovalPresentationContentHash) -> Self {
        value.0
    }
}

impl TryFrom<String> for ApprovalPresentationContentHash {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for ApprovalPresentationContentHash {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Surface where approval context was presented.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalPresentationChannel {
    /// Terminal output shown to the human or delegated maintainer.
    Terminal,
    /// Chat or conversation surface.
    Chat,
    /// Pull request or review surface.
    PullRequest,
    /// Local report or generated review artifact.
    LocalReport,
    /// Custom bounded surface label.
    Custom(String),
}

impl ApprovalPresentationChannel {
    fn validate(&self) -> Result<(), WorkflowOsError> {
        if let Self::Custom(value) = self {
            validate_bounded_text(
                "approval presentation channel",
                value,
                PRESENTATION_CHANNEL_MAX_BYTES,
                "approval_presentation.channel",
            )?;
        }
        Ok(())
    }
}

impl fmt::Debug for ApprovalPresentationChannel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Terminal => formatter.write_str("Terminal"),
            Self::Chat => formatter.write_str("Chat"),
            Self::PullRequest => formatter.write_str("PullRequest"),
            Self::LocalReport => formatter.write_str("LocalReport"),
            Self::Custom(_) => formatter.write_str("Custom([REDACTED])"),
        }
    }
}

/// Sensitivity classification for approval-presentation proof.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalPresentationSensitivity {
    /// Public metadata.
    Public,
    /// Internal metadata.
    Internal,
    /// Confidential metadata.
    Confidential,
    /// Restricted metadata.
    Restricted,
}

/// How approval-presentation proof was enforced for an approval decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDecisionProofEnforcementMode {
    /// Approval-presentation proof was required before accepting the decision.
    ApprovalPresentationRequired,
}

/// Validation policy used for an approval decision proof marker.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDecisionProofValidationPolicy {
    /// Presentation proof had to match the pending approval request.
    ApprovalPresentationRequestMatch,
}

/// Input fields for constructing an approval decision proof marker.
pub struct ApprovalDecisionProofMarkerDefinition {
    /// Proof enforcement mode.
    pub enforcement_mode: ApprovalDecisionProofEnforcementMode,
    /// Presentation proof ID used for the decision.
    pub presentation_id: ApprovalPresentationId,
    /// Canonical presentation content hash used for the decision.
    pub presentation_content_hash: ApprovalPresentationContentHash,
    /// Timestamp when proof was validated.
    pub proof_validated_at: Timestamp,
    /// Validation policy applied to the proof.
    pub proof_validation_policy: ApprovalDecisionProofValidationPolicy,
    /// Age of the proof in milliseconds when validated, when measured.
    pub proof_age_ms: Option<u64>,
    /// Freshness limit in milliseconds, when enforced.
    pub proof_freshness_limit_ms: Option<u64>,
    /// Sensitivity of the referenced proof record.
    pub proof_record_sensitivity: ApprovalPresentationSensitivity,
    /// Redaction metadata for the marker.
    pub redaction: RedactionMetadata,
}

/// Bounded proof-use marker for future approval decision event payloads.
///
/// This marker is model-only. Constructing it does not change approval
/// behavior, append runtime events, expose approval UI, or create report
/// artifacts.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ApprovalDecisionProofMarker {
    enforcement_mode: ApprovalDecisionProofEnforcementMode,
    presentation_id: ApprovalPresentationId,
    presentation_content_hash: ApprovalPresentationContentHash,
    proof_validated_at: Timestamp,
    proof_validation_policy: ApprovalDecisionProofValidationPolicy,
    proof_age_ms: Option<u64>,
    proof_freshness_limit_ms: Option<u64>,
    proof_record_sensitivity: ApprovalPresentationSensitivity,
    redaction: RedactionMetadata,
}

impl ApprovalDecisionProofMarker {
    /// Creates a validated approval decision proof marker.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when marker metadata is inconsistent,
    /// unbounded, or redaction metadata contains secret-like values.
    pub fn new(definition: ApprovalDecisionProofMarkerDefinition) -> Result<Self, WorkflowOsError> {
        validate_proof_freshness(definition.proof_age_ms, definition.proof_freshness_limit_ms)?;
        validate_marker_redaction_metadata(&definition.redaction)?;

        Ok(Self {
            enforcement_mode: definition.enforcement_mode,
            presentation_id: definition.presentation_id,
            presentation_content_hash: definition.presentation_content_hash,
            proof_validated_at: definition.proof_validated_at,
            proof_validation_policy: definition.proof_validation_policy,
            proof_age_ms: definition.proof_age_ms,
            proof_freshness_limit_ms: definition.proof_freshness_limit_ms,
            proof_record_sensitivity: definition.proof_record_sensitivity,
            redaction: definition.redaction,
        })
    }

    /// Returns the enforcement mode.
    #[must_use]
    pub const fn enforcement_mode(&self) -> ApprovalDecisionProofEnforcementMode {
        self.enforcement_mode
    }

    /// Returns the referenced presentation proof ID.
    #[must_use]
    pub const fn presentation_id(&self) -> &ApprovalPresentationId {
        &self.presentation_id
    }

    /// Returns the referenced presentation content hash.
    #[must_use]
    pub const fn presentation_content_hash(&self) -> &ApprovalPresentationContentHash {
        &self.presentation_content_hash
    }

    /// Returns the proof validation timestamp.
    #[must_use]
    pub const fn proof_validated_at(&self) -> Timestamp {
        self.proof_validated_at
    }

    /// Returns the validation policy.
    #[must_use]
    pub const fn proof_validation_policy(&self) -> ApprovalDecisionProofValidationPolicy {
        self.proof_validation_policy
    }

    /// Returns the proof age in milliseconds, when measured.
    #[must_use]
    pub const fn proof_age_ms(&self) -> Option<u64> {
        self.proof_age_ms
    }

    /// Returns the proof freshness limit in milliseconds, when enforced.
    #[must_use]
    pub const fn proof_freshness_limit_ms(&self) -> Option<u64> {
        self.proof_freshness_limit_ms
    }

    /// Returns the referenced proof record sensitivity.
    #[must_use]
    pub const fn proof_record_sensitivity(&self) -> ApprovalPresentationSensitivity {
        self.proof_record_sensitivity
    }

    /// Returns marker redaction metadata.
    #[must_use]
    pub const fn redaction(&self) -> &RedactionMetadata {
        &self.redaction
    }
}

impl fmt::Debug for ApprovalDecisionProofMarker {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ApprovalDecisionProofMarker")
            .field("enforcement_mode", &self.enforcement_mode)
            .field("presentation_id", &"[REDACTED]")
            .field("presentation_content_hash", &self.presentation_content_hash)
            .field("proof_validated_at", &self.proof_validated_at)
            .field("proof_validation_policy", &self.proof_validation_policy)
            .field("proof_age_ms", &self.proof_age_ms)
            .field("proof_freshness_limit_ms", &self.proof_freshness_limit_ms)
            .field("proof_record_sensitivity", &self.proof_record_sensitivity)
            .field(
                "redaction",
                &RedactedRedactionMetadataDebug(&self.redaction),
            )
            .finish()
    }
}

impl<'de> Deserialize<'de> for ApprovalDecisionProofMarker {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            enforcement_mode: ApprovalDecisionProofEnforcementMode,
            presentation_id: ApprovalPresentationId,
            presentation_content_hash: ApprovalPresentationContentHash,
            proof_validated_at: Timestamp,
            proof_validation_policy: ApprovalDecisionProofValidationPolicy,
            proof_age_ms: Option<u64>,
            proof_freshness_limit_ms: Option<u64>,
            proof_record_sensitivity: ApprovalPresentationSensitivity,
            redaction: RedactionMetadata,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(ApprovalDecisionProofMarkerDefinition {
            enforcement_mode: wire.enforcement_mode,
            presentation_id: wire.presentation_id,
            presentation_content_hash: wire.presentation_content_hash,
            proof_validated_at: wire.proof_validated_at,
            proof_validation_policy: wire.proof_validation_policy,
            proof_age_ms: wire.proof_age_ms,
            proof_freshness_limit_ms: wire.proof_freshness_limit_ms,
            proof_record_sensitivity: wire.proof_record_sensitivity,
            redaction: wire.redaction,
        })
        .map_err(serde::de::Error::custom)
    }
}

/// Input fields for constructing an approval-presentation record.
pub struct ApprovalPresentationRecordDefinition {
    /// Presentation ID.
    pub presentation_id: ApprovalPresentationId,
    /// Workflow run ID.
    pub run_id: WorkflowRunId,
    /// Approval ID from the approval request.
    pub approval_id: String,
    /// Workflow ID.
    pub workflow_id: WorkflowId,
    /// Workflow version, when available.
    pub workflow_version: Option<WorkflowVersion>,
    /// Schema version, when available.
    pub schema_version: Option<SchemaVersion>,
    /// Step ID, when available.
    pub step_id: Option<StepId>,
    /// Requested action shown to the approver.
    pub requested_action: String,
    /// Planned work or work summary shown to the approver.
    pub work_summary: String,
    /// Approved scope shown to the approver.
    pub approved_scope: String,
    /// Strict non-goals shown to the approver.
    pub strict_non_goals: Vec<String>,
    /// Expected touched surfaces shown to the approver.
    pub expected_touched_surfaces: Vec<String>,
    /// Validation/check expectations shown to the approver.
    pub validation_expectations: Vec<String>,
    /// Why this approval is being requested now.
    pub why_now: String,
    /// Next action after approval.
    pub next_action: String,
    /// Presentation timestamp.
    pub presented_at: Timestamp,
    /// Actor or system actor that presented the context.
    pub presented_by: ActorId,
    /// Presentation channel.
    pub channel: ApprovalPresentationChannel,
    /// Stable hash of the canonical presented content.
    pub content_hash: ApprovalPresentationContentHash,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
    /// Sensitivity classification.
    pub sensitivity: ApprovalPresentationSensitivity,
}

/// Local model-only approval-presentation proof record.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ApprovalPresentationRecord {
    presentation_id: ApprovalPresentationId,
    run_id: WorkflowRunId,
    approval_id: String,
    workflow_id: WorkflowId,
    workflow_version: Option<WorkflowVersion>,
    schema_version: Option<SchemaVersion>,
    step_id: Option<StepId>,
    requested_action: String,
    work_summary: String,
    approved_scope: String,
    strict_non_goals: Vec<String>,
    expected_touched_surfaces: Vec<String>,
    validation_expectations: Vec<String>,
    why_now: String,
    next_action: String,
    presented_at: Timestamp,
    presented_by: ActorId,
    channel: ApprovalPresentationChannel,
    content_hash: ApprovalPresentationContentHash,
    redaction: RedactionMetadata,
    sensitivity: ApprovalPresentationSensitivity,
}

impl ApprovalPresentationRecord {
    /// Creates a validated approval-presentation record.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when required fields are missing,
    /// unbounded, secret-like, duplicated, or the content hash does not match
    /// the canonical presentation content.
    pub fn new(definition: ApprovalPresentationRecordDefinition) -> Result<Self, WorkflowOsError> {
        validate_approval_id(&definition.approval_id)?;
        validate_bounded_text(
            "approval presentation requested action",
            &definition.requested_action,
            PRESENTATION_TEXT_MAX_BYTES,
            "approval_presentation.requested_action",
        )?;
        validate_bounded_text(
            "approval presentation work summary",
            &definition.work_summary,
            PRESENTATION_TEXT_MAX_BYTES,
            "approval_presentation.work_summary",
        )?;
        validate_bounded_text(
            "approval presentation approved scope",
            &definition.approved_scope,
            PRESENTATION_TEXT_MAX_BYTES,
            "approval_presentation.approved_scope",
        )?;
        validate_bounded_text_collection(
            "approval presentation strict non-goals",
            &definition.strict_non_goals,
            "approval_presentation.strict_non_goals",
        )?;
        validate_bounded_text_collection(
            "approval presentation touched surfaces",
            &definition.expected_touched_surfaces,
            "approval_presentation.expected_touched_surfaces",
        )?;
        validate_bounded_text_collection(
            "approval presentation validation expectations",
            &definition.validation_expectations,
            "approval_presentation.validation_expectations",
        )?;
        validate_bounded_text(
            "approval presentation why-now context",
            &definition.why_now,
            PRESENTATION_TEXT_MAX_BYTES,
            "approval_presentation.why_now",
        )?;
        validate_bounded_text(
            "approval presentation next action",
            &definition.next_action,
            PRESENTATION_TEXT_MAX_BYTES,
            "approval_presentation.next_action",
        )?;
        definition.channel.validate()?;
        validate_redaction_metadata(&definition.redaction)?;

        let canonical = canonical_approval_presentation_content(
            &definition.run_id,
            &definition.approval_id,
            &definition.workflow_id,
            definition.workflow_version.as_ref(),
            definition.schema_version.as_ref(),
            definition.step_id.as_ref(),
            &definition.requested_action,
            &definition.work_summary,
            &definition.approved_scope,
            &definition.strict_non_goals,
            &definition.expected_touched_surfaces,
            &definition.validation_expectations,
            &definition.why_now,
            &definition.next_action,
            &definition.channel,
            definition.sensitivity,
        )?;
        let expected_hash = ApprovalPresentationContentHash::from_canonical_text(&canonical);
        if expected_hash != definition.content_hash {
            return Err(validation_error(
                "approval_presentation.content_hash.mismatch",
                "approval presentation content hash does not match canonical content",
            ));
        }

        Ok(Self {
            presentation_id: definition.presentation_id,
            run_id: definition.run_id,
            approval_id: definition.approval_id,
            workflow_id: definition.workflow_id,
            workflow_version: definition.workflow_version,
            schema_version: definition.schema_version,
            step_id: definition.step_id,
            requested_action: definition.requested_action,
            work_summary: definition.work_summary,
            approved_scope: definition.approved_scope,
            strict_non_goals: definition.strict_non_goals,
            expected_touched_surfaces: definition.expected_touched_surfaces,
            validation_expectations: definition.validation_expectations,
            why_now: definition.why_now,
            next_action: definition.next_action,
            presented_at: definition.presented_at,
            presented_by: definition.presented_by,
            channel: definition.channel,
            content_hash: definition.content_hash,
            redaction: definition.redaction,
            sensitivity: definition.sensitivity,
        })
    }

    /// Returns the presentation ID.
    #[must_use]
    pub const fn presentation_id(&self) -> &ApprovalPresentationId {
        &self.presentation_id
    }

    /// Returns the workflow run ID.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the approval ID.
    #[must_use]
    pub fn approval_id(&self) -> &str {
        &self.approval_id
    }

    /// Returns the workflow ID.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the workflow version, when available.
    #[must_use]
    pub fn workflow_version(&self) -> Option<&WorkflowVersion> {
        self.workflow_version.as_ref()
    }

    /// Returns the schema version, when available.
    #[must_use]
    pub fn schema_version(&self) -> Option<&SchemaVersion> {
        self.schema_version.as_ref()
    }

    /// Returns the step ID, when available.
    #[must_use]
    pub fn step_id(&self) -> Option<&StepId> {
        self.step_id.as_ref()
    }

    /// Returns the requested action.
    #[must_use]
    pub fn requested_action(&self) -> &str {
        &self.requested_action
    }

    /// Returns the work summary.
    #[must_use]
    pub fn work_summary(&self) -> &str {
        &self.work_summary
    }

    /// Returns the approved scope.
    #[must_use]
    pub fn approved_scope(&self) -> &str {
        &self.approved_scope
    }

    /// Returns strict non-goals.
    #[must_use]
    pub fn strict_non_goals(&self) -> &[String] {
        &self.strict_non_goals
    }

    /// Returns expected touched surfaces.
    #[must_use]
    pub fn expected_touched_surfaces(&self) -> &[String] {
        &self.expected_touched_surfaces
    }

    /// Returns validation expectations.
    #[must_use]
    pub fn validation_expectations(&self) -> &[String] {
        &self.validation_expectations
    }

    /// Returns why-now context.
    #[must_use]
    pub fn why_now(&self) -> &str {
        &self.why_now
    }

    /// Returns next action.
    #[must_use]
    pub fn next_action(&self) -> &str {
        &self.next_action
    }

    /// Returns presentation timestamp.
    #[must_use]
    pub const fn presented_at(&self) -> Timestamp {
        self.presented_at
    }

    /// Returns presenting actor.
    #[must_use]
    pub const fn presented_by(&self) -> &ActorId {
        &self.presented_by
    }

    /// Returns presentation channel.
    #[must_use]
    pub const fn channel(&self) -> &ApprovalPresentationChannel {
        &self.channel
    }

    /// Returns the canonical content hash.
    #[must_use]
    pub const fn content_hash(&self) -> &ApprovalPresentationContentHash {
        &self.content_hash
    }

    /// Returns redaction metadata.
    #[must_use]
    pub const fn redaction(&self) -> &RedactionMetadata {
        &self.redaction
    }

    /// Returns sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> ApprovalPresentationSensitivity {
        self.sensitivity
    }
}

impl fmt::Debug for ApprovalPresentationRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ApprovalPresentationRecord")
            .field("presentation_id", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("approval_id", &"[REDACTED]")
            .field("workflow_id", &"[REDACTED]")
            .field("has_workflow_version", &self.workflow_version.is_some())
            .field("has_schema_version", &self.schema_version.is_some())
            .field("has_step_id", &self.step_id.is_some())
            .field("requested_action", &"[REDACTED]")
            .field("work_summary", &"[REDACTED]")
            .field("approved_scope", &"[REDACTED]")
            .field("strict_non_goal_count", &self.strict_non_goals.len())
            .field(
                "expected_touched_surface_count",
                &self.expected_touched_surfaces.len(),
            )
            .field(
                "validation_expectation_count",
                &self.validation_expectations.len(),
            )
            .field("why_now", &"[REDACTED]")
            .field("next_action", &"[REDACTED]")
            .field("presented_at", &self.presented_at)
            .field("presented_by", &"[REDACTED]")
            .field("channel", &self.channel)
            .field("content_hash", &self.content_hash)
            .field(
                "redaction",
                &RedactedRedactionMetadataDebug(&self.redaction),
            )
            .field("sensitivity", &self.sensitivity)
            .finish()
    }
}

impl<'de> Deserialize<'de> for ApprovalPresentationRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            presentation_id: ApprovalPresentationId,
            run_id: WorkflowRunId,
            approval_id: String,
            workflow_id: WorkflowId,
            workflow_version: Option<WorkflowVersion>,
            schema_version: Option<SchemaVersion>,
            step_id: Option<StepId>,
            requested_action: String,
            work_summary: String,
            approved_scope: String,
            strict_non_goals: Vec<String>,
            expected_touched_surfaces: Vec<String>,
            validation_expectations: Vec<String>,
            why_now: String,
            next_action: String,
            presented_at: Timestamp,
            presented_by: ActorId,
            channel: ApprovalPresentationChannel,
            content_hash: ApprovalPresentationContentHash,
            redaction: RedactionMetadata,
            sensitivity: ApprovalPresentationSensitivity,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(ApprovalPresentationRecordDefinition {
            presentation_id: wire.presentation_id,
            run_id: wire.run_id,
            approval_id: wire.approval_id,
            workflow_id: wire.workflow_id,
            workflow_version: wire.workflow_version,
            schema_version: wire.schema_version,
            step_id: wire.step_id,
            requested_action: wire.requested_action,
            work_summary: wire.work_summary,
            approved_scope: wire.approved_scope,
            strict_non_goals: wire.strict_non_goals,
            expected_touched_surfaces: wire.expected_touched_surfaces,
            validation_expectations: wire.validation_expectations,
            why_now: wire.why_now,
            next_action: wire.next_action,
            presented_at: wire.presented_at,
            presented_by: wire.presented_by,
            channel: wire.channel,
            content_hash: wire.content_hash,
            redaction: wire.redaction,
            sensitivity: wire.sensitivity,
        })
        .map_err(serde::de::Error::custom)
    }
}

/// Explicit inputs for validating presentation proof against an approval request.
#[derive(Clone, Copy)]
pub struct ApprovalPresentationValidationInput<'a> {
    /// Presentation record to validate.
    pub presentation: &'a ApprovalPresentationRecord,
    /// Approval request that the presentation claims to represent.
    pub approval_request: &'a ApprovalRequest,
}

/// Validates that an approval-presentation record matches a supplied approval request.
///
/// # Errors
///
/// Returns a stable non-leaking error when the run, approval, workflow,
/// schema, version, or step identity does not match.
pub fn validate_approval_presentation_for_request(
    input: ApprovalPresentationValidationInput<'_>,
) -> Result<(), WorkflowOsError> {
    if input.presentation.run_id() != &input.approval_request.run_id {
        return Err(validation_error(
            "approval_presentation.request.run_id_mismatch",
            "approval presentation run ID does not match approval request",
        ));
    }
    if input.presentation.approval_id() != input.approval_request.approval_id {
        return Err(validation_error(
            "approval_presentation.request.approval_id_mismatch",
            "approval presentation approval ID does not match approval request",
        ));
    }
    if input.presentation.workflow_id() != &input.approval_request.workflow_id {
        return Err(validation_error(
            "approval_presentation.request.workflow_id_mismatch",
            "approval presentation workflow ID does not match approval request",
        ));
    }
    if input.presentation.workflow_version() != Some(&input.approval_request.workflow_version) {
        return Err(validation_error(
            "approval_presentation.request.workflow_version_mismatch",
            "approval presentation workflow version does not match approval request",
        ));
    }
    if input.presentation.schema_version() != Some(&input.approval_request.schema_version) {
        return Err(validation_error(
            "approval_presentation.request.schema_version_mismatch",
            "approval presentation schema version does not match approval request",
        ));
    }
    if input.presentation.step_id() != Some(&input.approval_request.step_id) {
        return Err(validation_error(
            "approval_presentation.request.step_id_mismatch",
            "approval presentation step ID does not match approval request",
        ));
    }

    Ok(())
}

/// Computes the canonical content hash for approval-presentation fields.
///
/// # Errors
///
/// Returns a stable non-leaking error when fields are missing, unbounded,
/// duplicated, or secret-like.
#[allow(clippy::too_many_arguments)]
pub fn compute_approval_presentation_content_hash(
    run_id: &WorkflowRunId,
    approval_id: &str,
    workflow_id: &WorkflowId,
    workflow_version: Option<&WorkflowVersion>,
    schema_version: Option<&SchemaVersion>,
    step_id: Option<&StepId>,
    requested_action: &str,
    work_summary: &str,
    approved_scope: &str,
    strict_non_goals: &[String],
    expected_touched_surfaces: &[String],
    validation_expectations: &[String],
    why_now: &str,
    next_action: &str,
    channel: &ApprovalPresentationChannel,
    sensitivity: ApprovalPresentationSensitivity,
) -> Result<ApprovalPresentationContentHash, WorkflowOsError> {
    let canonical = canonical_approval_presentation_content(
        run_id,
        approval_id,
        workflow_id,
        workflow_version,
        schema_version,
        step_id,
        requested_action,
        work_summary,
        approved_scope,
        strict_non_goals,
        expected_touched_surfaces,
        validation_expectations,
        why_now,
        next_action,
        channel,
        sensitivity,
    )?;
    Ok(ApprovalPresentationContentHash::from_canonical_text(
        &canonical,
    ))
}

#[allow(clippy::too_many_arguments)]
fn canonical_approval_presentation_content(
    run_id: &WorkflowRunId,
    approval_id: &str,
    workflow_id: &WorkflowId,
    workflow_version: Option<&WorkflowVersion>,
    schema_version: Option<&SchemaVersion>,
    step_id: Option<&StepId>,
    requested_action: &str,
    work_summary: &str,
    approved_scope: &str,
    strict_non_goals: &[String],
    expected_touched_surfaces: &[String],
    validation_expectations: &[String],
    why_now: &str,
    next_action: &str,
    channel: &ApprovalPresentationChannel,
    sensitivity: ApprovalPresentationSensitivity,
) -> Result<String, WorkflowOsError> {
    validate_approval_id(approval_id)?;
    validate_bounded_text(
        "approval presentation requested action",
        requested_action,
        PRESENTATION_TEXT_MAX_BYTES,
        "approval_presentation.requested_action",
    )?;
    validate_bounded_text(
        "approval presentation work summary",
        work_summary,
        PRESENTATION_TEXT_MAX_BYTES,
        "approval_presentation.work_summary",
    )?;
    validate_bounded_text(
        "approval presentation approved scope",
        approved_scope,
        PRESENTATION_TEXT_MAX_BYTES,
        "approval_presentation.approved_scope",
    )?;
    validate_bounded_text_collection(
        "approval presentation strict non-goals",
        strict_non_goals,
        "approval_presentation.strict_non_goals",
    )?;
    validate_bounded_text_collection(
        "approval presentation touched surfaces",
        expected_touched_surfaces,
        "approval_presentation.expected_touched_surfaces",
    )?;
    validate_bounded_text_collection(
        "approval presentation validation expectations",
        validation_expectations,
        "approval_presentation.validation_expectations",
    )?;
    validate_bounded_text(
        "approval presentation why-now context",
        why_now,
        PRESENTATION_TEXT_MAX_BYTES,
        "approval_presentation.why_now",
    )?;
    validate_bounded_text(
        "approval presentation next action",
        next_action,
        PRESENTATION_TEXT_MAX_BYTES,
        "approval_presentation.next_action",
    )?;
    channel.validate()?;

    Ok(format!(
        "approval_presentation.v1\nrun_id={}\napproval_id={}\nworkflow_id={}\nworkflow_version={}\nschema_version={}\nstep_id={}\nrequested_action={}\nwork_summary={}\napproved_scope={}\nstrict_non_goals={}\nexpected_touched_surfaces={}\nvalidation_expectations={}\nwhy_now={}\nnext_action={}\nchannel={}\nsensitivity={:?}\n",
        run_id.as_str(),
        approval_id,
        workflow_id.as_str(),
        workflow_version.map_or("", WorkflowVersion::as_str),
        schema_version.map_or("", SchemaVersion::as_str),
        step_id.map_or("", StepId::as_str),
        requested_action,
        work_summary,
        approved_scope,
        canonical_join(strict_non_goals),
        canonical_join(expected_touched_surfaces),
        canonical_join(validation_expectations),
        why_now,
        next_action,
        canonical_channel(channel),
        sensitivity,
    ))
}

fn validate_approval_id(value: &str) -> Result<(), WorkflowOsError> {
    validate_identifier("approval id", value, APPROVAL_ID_MAX_BYTES)
}

/// Validates an approval ID for approval-presentation lookup boundaries.
///
/// # Errors
///
/// Returns a stable non-leaking error when the approval ID is empty, too long,
/// contains unsupported characters, or looks like a secret.
pub fn validate_approval_presentation_approval_id(value: &str) -> Result<(), WorkflowOsError> {
    validate_approval_id(value)
}

fn validate_identifier(
    type_name: &'static str,
    value: &str,
    max_bytes: usize,
) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "approval_presentation.identifier.empty",
            format!("{type_name} cannot be empty"),
        ));
    }
    if value.len() > max_bytes {
        return Err(validation_error(
            "approval_presentation.identifier.too_long",
            format!("{type_name} cannot exceed {max_bytes} bytes"),
        ));
    }
    let is_valid = value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'));
    if !is_valid {
        return Err(validation_error(
            "approval_presentation.identifier.invalid_character",
            format!("{type_name} contains an invalid character"),
        ));
    }
    validate_not_secret_like(type_name, value)
}

fn validate_bounded_text(
    type_name: &'static str,
    value: &str,
    max_bytes: usize,
    code_prefix: &'static str,
) -> Result<(), WorkflowOsError> {
    if value.trim().is_empty() {
        return Err(validation_error(
            format!("{code_prefix}.empty"),
            format!("{type_name} cannot be empty"),
        ));
    }
    if value.len() > max_bytes {
        return Err(validation_error(
            format!("{code_prefix}.too_long"),
            format!("{type_name} cannot exceed {max_bytes} bytes"),
        ));
    }
    validate_not_secret_like(type_name, value)
}

fn validate_bounded_text_collection(
    type_name: &'static str,
    values: &[String],
    code_prefix: &'static str,
) -> Result<(), WorkflowOsError> {
    if values.is_empty() {
        return Err(validation_error(
            format!("{code_prefix}.empty"),
            format!("{type_name} cannot be empty"),
        ));
    }
    if values.len() > PRESENTATION_COLLECTION_MAX_ITEMS {
        return Err(validation_error(
            format!("{code_prefix}.too_many"),
            format!("{type_name} cannot exceed {PRESENTATION_COLLECTION_MAX_ITEMS} entries"),
        ));
    }

    let mut seen = BTreeSet::new();
    for value in values {
        validate_bounded_text(type_name, value, PRESENTATION_TEXT_MAX_BYTES, code_prefix)?;
        if !seen.insert(value) {
            return Err(validation_error(
                format!("{code_prefix}.duplicate"),
                format!("{type_name} contains duplicate entries"),
            ));
        }
    }

    Ok(())
}

fn validate_redaction_metadata(redaction: &RedactionMetadata) -> Result<(), WorkflowOsError> {
    if redaction.redacted_fields.len() > PRESENTATION_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "approval_presentation.redaction.too_many_fields",
            "approval presentation redaction metadata contains too many fields",
        ));
    }
    if redaction.field_states.len() > PRESENTATION_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "approval_presentation.redaction.too_many_states",
            "approval presentation redaction metadata contains too many field states",
        ));
    }

    for field in &redaction.redacted_fields {
        validate_bounded_text(
            "approval presentation redaction field",
            field,
            PRESENTATION_REDACTION_FIELD_MAX_BYTES,
            "approval_presentation.redaction.field",
        )?;
    }

    for state in &redaction.field_states {
        validate_bounded_text(
            "approval presentation redaction field",
            &state.field,
            PRESENTATION_REDACTION_FIELD_MAX_BYTES,
            "approval_presentation.redaction.field",
        )?;
        validate_bounded_text(
            "approval presentation redaction reason",
            &state.reason,
            PRESENTATION_REDACTION_REASON_MAX_BYTES,
            "approval_presentation.redaction.reason",
        )?;
    }

    Ok(())
}

fn validate_marker_redaction_metadata(
    redaction: &RedactionMetadata,
) -> Result<(), WorkflowOsError> {
    if redaction.redacted_fields.len() > PRESENTATION_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "approval_event_proof_marker.redaction.too_many_fields",
            "approval event proof marker redaction metadata contains too many fields",
        ));
    }
    if redaction.field_states.len() > PRESENTATION_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "approval_event_proof_marker.redaction.too_many_states",
            "approval event proof marker redaction metadata contains too many field states",
        ));
    }

    for field in &redaction.redacted_fields {
        validate_bounded_text(
            "approval event proof marker redaction field",
            field,
            PRESENTATION_REDACTION_FIELD_MAX_BYTES,
            "approval_event_proof_marker.redaction.field",
        )?;
    }

    for state in &redaction.field_states {
        validate_bounded_text(
            "approval event proof marker redaction field",
            &state.field,
            PRESENTATION_REDACTION_FIELD_MAX_BYTES,
            "approval_event_proof_marker.redaction.field",
        )?;
        validate_bounded_text(
            "approval event proof marker redaction reason",
            &state.reason,
            PRESENTATION_REDACTION_REASON_MAX_BYTES,
            "approval_event_proof_marker.redaction.reason",
        )?;
    }

    Ok(())
}

fn validate_proof_freshness(
    proof_age_ms: Option<u64>,
    proof_freshness_limit_ms: Option<u64>,
) -> Result<(), WorkflowOsError> {
    if let Some(value) = proof_age_ms {
        if value > PROOF_FRESHNESS_MAX_MS {
            return Err(validation_error(
                "approval_event_proof_marker.proof_age.too_large",
                "approval event proof marker proof age exceeds the supported bound",
            ));
        }
    }
    if let Some(value) = proof_freshness_limit_ms {
        if value == 0 || value > PROOF_FRESHNESS_MAX_MS {
            return Err(validation_error(
                "approval_event_proof_marker.freshness_limit.invalid",
                "approval event proof marker freshness limit is outside the supported bound",
            ));
        }
    }
    if let (Some(age), Some(limit)) = (proof_age_ms, proof_freshness_limit_ms) {
        if age > limit {
            return Err(validation_error(
                "approval_event_proof_marker.freshness_mismatch",
                "approval event proof marker proof age exceeds the freshness limit",
            ));
        }
    }

    Ok(())
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
        return Err(validation_error(
            "approval_presentation.secret_like_value",
            format!("{type_name} contains sensitive-looking text"),
        ));
    }

    Ok(())
}

fn canonical_join(values: &[String]) -> String {
    values.join("\u{1f}")
}

fn canonical_channel(channel: &ApprovalPresentationChannel) -> String {
    match channel {
        ApprovalPresentationChannel::Terminal => "terminal".to_owned(),
        ApprovalPresentationChannel::Chat => "chat".to_owned(),
        ApprovalPresentationChannel::PullRequest => "pull_request".to_owned(),
        ApprovalPresentationChannel::LocalReport => "local_report".to_owned(),
        ApprovalPresentationChannel::Custom(value) => format!("custom:{value}"),
    }
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn validation_error(code: impl Into<String>, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}

struct RedactedRedactionMetadataDebug<'a>(&'a RedactionMetadata);

impl fmt::Debug for RedactedRedactionMetadataDebug<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RedactionMetadata")
            .field("redacted_field_count", &self.0.redacted_fields.len())
            .field("field_state_count", &self.0.field_states.len())
            .finish()
    }
}
