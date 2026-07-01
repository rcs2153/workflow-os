use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    EventId, EvidenceReferenceId, LocalCheckResultId, RedactionMetadata, SchemaVersion,
    SideEffectId, ValidationReferenceId, WorkReportId, WorkReportRedactionPolicy,
    WorkReportSensitivity, WorkReportStableReference, WorkflowOsError,
};

const HIGH_ASSURANCE_IDENTIFIER_MAX_BYTES: usize = 128;
const HIGH_ASSURANCE_REDACTION_FIELD_MAX_BYTES: usize = 128;
const HIGH_ASSURANCE_REDACTION_REASON_MAX_BYTES: usize = 512;
const HIGH_ASSURANCE_REDACTION_MAX_ENTRIES: usize = 64;

/// Identifier for a high-assurance approval control.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct HighAssuranceApprovalControlId(String);

impl HighAssuranceApprovalControlId {
    /// Creates a validated high-assurance approval control ID.
    ///
    /// # Errors
    ///
    /// Returns an error when the ID is empty, too long, contains unsupported
    /// characters, or looks like a secret.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("HighAssuranceApprovalControlId", &value)?;
        Ok(Self(value))
    }

    /// Returns the ID as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HighAssuranceApprovalControlId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for HighAssuranceApprovalControlId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("HighAssuranceApprovalControlId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<HighAssuranceApprovalControlId> for String {
    fn from(value: HighAssuranceApprovalControlId) -> Self {
        value.0
    }
}

impl TryFrom<String> for HighAssuranceApprovalControlId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for HighAssuranceApprovalControlId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Version for a high-assurance approval control.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct HighAssuranceApprovalControlVersion(String);

impl HighAssuranceApprovalControlVersion {
    /// Creates a validated high-assurance approval control version.
    ///
    /// # Errors
    ///
    /// Returns an error when the version is empty, too long, contains unsupported
    /// characters, or looks like a secret.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("HighAssuranceApprovalControlVersion", &value)?;
        Ok(Self(value))
    }

    /// Returns the version as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HighAssuranceApprovalControlVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for HighAssuranceApprovalControlVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("HighAssuranceApprovalControlVersion")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<HighAssuranceApprovalControlVersion> for String {
    fn from(value: HighAssuranceApprovalControlVersion) -> Self {
        value.0
    }
}

impl TryFrom<String> for HighAssuranceApprovalControlVersion {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for HighAssuranceApprovalControlVersion {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Domain-neutral protected action classes for high-assurance approval planning.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HighAssuranceProtectedActionKind {
    /// Local skill invocation that is sensitive enough to require stronger approval.
    LocalSkillInvocation,
    /// Read-only adapter access that is sensitive by context.
    AdapterRead,
    /// Future adapter write action. Vocabulary only; writes remain unsupported.
    AdapterWrite,
    /// Future side-effect attempt. Vocabulary only; side-effect execution remains unsupported.
    SideEffectAttempt,
    /// Local report artifact write.
    ReportArtifactWrite,
    /// Workflow transition that requires higher assurance.
    WorkflowTransition,
}

/// Requester/approver separation posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HighAssuranceRequesterApproverRule {
    /// Same actor may approve. Suitable only for lower-risk local posture.
    SameActorAllowed,
    /// Requester and approver must differ.
    MustDiffer,
    /// A human approver must differ from the requester. Identity remains local-only in v0.
    HumanApproverMustDiffer,
}

/// Expiration posture for high-assurance approvals.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HighAssuranceApprovalExpirationPolicy {
    /// No expiration requirement is declared by this control.
    NotRequired,
    /// Approval request must declare expiration metadata.
    RequiredOnRequest,
    /// Approval must be unexpired when the decision is made.
    MustBeUnexpiredAtDecision,
    /// Approval must be unexpired when the protected action is used.
    MustBeUnexpiredAtUse,
}

/// Revocation posture for high-assurance approvals.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HighAssuranceApprovalRevocationPolicy {
    /// Revocation is unsupported by this control.
    Unsupported,
    /// Revocation must be represented by an explicit future event before protected use.
    ExplicitEventBeforeUse,
    /// Revocation after use is report disclosure only.
    ReportOnlyAfterUse,
}

/// Denial behavior for high-assurance approval controls.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HighAssuranceApprovalDenialBehavior {
    /// Fail closed.
    FailClosed,
    /// Block the current step.
    BlockStep,
    /// Cancel the run.
    CancelRun,
    /// Escalate to an operator.
    Escalate,
}

/// Report disclosure requirements for high-assurance approval posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HighAssuranceApprovalReportDisclosure {
    /// Disclose requested approvals.
    Requested,
    /// Disclose granted approvals.
    Granted,
    /// Disclose denied approvals.
    Denied,
    /// Disclose expired approvals.
    Expired,
    /// Disclose revoked approvals.
    Revoked,
    /// Disclose skipped or not-applicable approvals.
    Skipped,
    /// Disclose deferred approval work.
    Deferred,
    /// Disclose evidence considered by reference.
    EvidenceConsidered,
    /// Disclose side effects authorized or denied by approval.
    SideEffectsAuthorized,
}

/// Stable reference target required by a high-assurance approval control.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum HighAssuranceApprovalRequiredReferenceTarget {
    /// Evidence reference ID.
    EvidenceReference {
        /// Evidence reference ID.
        evidence_reference_id: EvidenceReferenceId,
    },
    /// Policy decision event ID.
    PolicyDecision {
        /// Policy decision event ID.
        event_id: EventId,
    },
    /// `SideEffect` record ID.
    SideEffect {
        /// `SideEffect` record ID.
        side_effect_id: SideEffectId,
    },
    /// Validation reference ID.
    ValidationReference {
        /// Validation reference ID.
        validation_reference_id: ValidationReferenceId,
    },
    /// Local check result ID.
    LocalCheckResult {
        /// Local check result ID.
        local_check_result_id: LocalCheckResultId,
    },
    /// Workflow event ID.
    WorkflowEvent {
        /// Workflow event ID.
        event_id: EventId,
    },
    /// Audit event ID.
    AuditEvent {
        /// Audit event ID.
        event_id: EventId,
    },
    /// `WorkReport` ID.
    WorkReport {
        /// `WorkReport` ID.
        work_report_id: WorkReportId,
    },
    /// Adapter telemetry stable reference.
    AdapterTelemetry {
        /// Stable adapter telemetry reference.
        reference: WorkReportStableReference,
    },
}

impl HighAssuranceApprovalRequiredReferenceTarget {
    /// Returns a stable kind label for duplicate checking and debugging.
    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        match self {
            Self::EvidenceReference { .. } => "evidence_reference",
            Self::PolicyDecision { .. } => "policy_decision",
            Self::SideEffect { .. } => "side_effect",
            Self::ValidationReference { .. } => "validation_reference",
            Self::LocalCheckResult { .. } => "local_check_result",
            Self::WorkflowEvent { .. } => "workflow_event",
            Self::AuditEvent { .. } => "audit_event",
            Self::WorkReport { .. } => "work_report",
            Self::AdapterTelemetry { .. } => "adapter_telemetry",
        }
    }
}

impl fmt::Debug for HighAssuranceApprovalRequiredReferenceTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HighAssuranceApprovalRequiredReferenceTarget")
            .field("kind", &self.kind_name())
            .field("reference", &"[REDACTED]")
            .finish()
    }
}

/// Reference requirement carried by a high-assurance approval control.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HighAssuranceApprovalRequiredReference {
    name: String,
    target: HighAssuranceApprovalRequiredReferenceTarget,
    required: bool,
}

impl HighAssuranceApprovalRequiredReference {
    /// Creates a validated high-assurance approval reference requirement.
    ///
    /// # Errors
    ///
    /// Returns an error when the name is invalid or secret-like.
    pub fn new(
        name: impl Into<String>,
        target: HighAssuranceApprovalRequiredReferenceTarget,
        required: bool,
    ) -> Result<Self, WorkflowOsError> {
        let reference = Self {
            name: name.into(),
            target,
            required,
        };
        reference.validate()?;
        Ok(reference)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_identifier(
            "high-assurance approval required reference name",
            &self.name,
        )
    }

    /// Returns the reference name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the required reference target.
    #[must_use]
    pub const fn target(&self) -> &HighAssuranceApprovalRequiredReferenceTarget {
        &self.target
    }

    /// Returns whether the reference is required.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}

impl fmt::Debug for HighAssuranceApprovalRequiredReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HighAssuranceApprovalRequiredReference")
            .field("name", &"[REDACTED]")
            .field("target", &self.target)
            .field("required", &self.required)
            .finish()
    }
}

/// Domain-neutral high-assurance approval control model.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct HighAssuranceApprovalControl {
    control_id: HighAssuranceApprovalControlId,
    control_version: HighAssuranceApprovalControlVersion,
    schema_version: SchemaVersion,
    protected_actions: Vec<HighAssuranceProtectedActionKind>,
    requester_approver_rule: HighAssuranceRequesterApproverRule,
    minimum_approvals: u16,
    required_references: Vec<HighAssuranceApprovalRequiredReference>,
    expiration_policy: HighAssuranceApprovalExpirationPolicy,
    revocation_policy: HighAssuranceApprovalRevocationPolicy,
    denial_behavior: HighAssuranceApprovalDenialBehavior,
    report_disclosures: Vec<HighAssuranceApprovalReportDisclosure>,
    sensitivity: WorkReportSensitivity,
    redaction_policy: WorkReportRedactionPolicy,
    redaction: RedactionMetadata,
}

/// Input fields for constructing a validated `HighAssuranceApprovalControl`.
pub struct HighAssuranceApprovalControlDefinition {
    /// Control ID.
    pub control_id: HighAssuranceApprovalControlId,
    /// Control version.
    pub control_version: HighAssuranceApprovalControlVersion,
    /// Schema version associated with the control model.
    pub schema_version: SchemaVersion,
    /// Protected action classes.
    pub protected_actions: Vec<HighAssuranceProtectedActionKind>,
    /// Requester/approver separation posture.
    pub requester_approver_rule: HighAssuranceRequesterApproverRule,
    /// Minimum number of required approvals.
    pub minimum_approvals: u16,
    /// Required approval packet references.
    pub required_references: Vec<HighAssuranceApprovalRequiredReference>,
    /// Expiration policy.
    pub expiration_policy: HighAssuranceApprovalExpirationPolicy,
    /// Revocation policy.
    pub revocation_policy: HighAssuranceApprovalRevocationPolicy,
    /// Denial behavior.
    pub denial_behavior: HighAssuranceApprovalDenialBehavior,
    /// Report disclosure requirements.
    pub report_disclosures: Vec<HighAssuranceApprovalReportDisclosure>,
    /// Sensitivity classification.
    pub sensitivity: WorkReportSensitivity,
    /// Redaction policy.
    pub redaction_policy: WorkReportRedactionPolicy,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
}

impl HighAssuranceApprovalControl {
    /// Creates a validated high-assurance approval control.
    ///
    /// # Errors
    ///
    /// Returns an error when required fields are missing, duplicated,
    /// unbounded, or secret-like.
    pub fn new(
        definition: HighAssuranceApprovalControlDefinition,
    ) -> Result<Self, WorkflowOsError> {
        let control = Self {
            control_id: definition.control_id,
            control_version: definition.control_version,
            schema_version: definition.schema_version,
            protected_actions: definition.protected_actions,
            requester_approver_rule: definition.requester_approver_rule,
            minimum_approvals: definition.minimum_approvals,
            required_references: definition.required_references,
            expiration_policy: definition.expiration_policy,
            revocation_policy: definition.revocation_policy,
            denial_behavior: definition.denial_behavior,
            report_disclosures: definition.report_disclosures,
            sensitivity: definition.sensitivity,
            redaction_policy: definition.redaction_policy,
            redaction: definition.redaction,
        };
        control.validate()?;
        Ok(control)
    }

    /// Validates the control.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_not_secret_like("schema version", self.schema_version.as_str())?;
        validate_enum_list(
            "high_assurance_approval.protected_actions.required",
            "high-assurance approval controls require at least one protected action",
            "high_assurance_approval.protected_actions.duplicate",
            "high-assurance approval controls cannot declare duplicate protected actions",
            &self.protected_actions,
        )?;

        if self.minimum_approvals == 0 {
            return Err(validation_error(
                "high_assurance_approval.minimum_approvals.invalid",
                "high-assurance approval controls require at least one approval",
            ));
        }

        validate_reference_list(
            "high_assurance_approval.references.required",
            "high-assurance approval controls require at least one reference requirement",
            &self.required_references,
        )?;
        validate_enum_list(
            "high_assurance_approval.report_disclosures.required",
            "high-assurance approval controls require at least one report disclosure",
            "high_assurance_approval.report_disclosures.duplicate",
            "high-assurance approval controls cannot declare duplicate report disclosures",
            &self.report_disclosures,
        )?;
        validate_redaction_metadata(&self.redaction)?;
        Ok(())
    }

    /// Returns the control ID.
    #[must_use]
    pub const fn control_id(&self) -> &HighAssuranceApprovalControlId {
        &self.control_id
    }

    /// Returns the control version.
    #[must_use]
    pub const fn control_version(&self) -> &HighAssuranceApprovalControlVersion {
        &self.control_version
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> &SchemaVersion {
        &self.schema_version
    }

    /// Returns the protected action classes.
    #[must_use]
    pub fn protected_actions(&self) -> &[HighAssuranceProtectedActionKind] {
        &self.protected_actions
    }

    /// Returns the requester/approver separation posture.
    #[must_use]
    pub const fn requester_approver_rule(&self) -> HighAssuranceRequesterApproverRule {
        self.requester_approver_rule
    }

    /// Returns the minimum approval count.
    #[must_use]
    pub const fn minimum_approvals(&self) -> u16 {
        self.minimum_approvals
    }

    /// Returns required reference declarations.
    #[must_use]
    pub fn required_references(&self) -> &[HighAssuranceApprovalRequiredReference] {
        &self.required_references
    }

    /// Returns the expiration policy.
    #[must_use]
    pub const fn expiration_policy(&self) -> HighAssuranceApprovalExpirationPolicy {
        self.expiration_policy
    }

    /// Returns the revocation policy.
    #[must_use]
    pub const fn revocation_policy(&self) -> HighAssuranceApprovalRevocationPolicy {
        self.revocation_policy
    }

    /// Returns the denial behavior.
    #[must_use]
    pub const fn denial_behavior(&self) -> HighAssuranceApprovalDenialBehavior {
        self.denial_behavior
    }

    /// Returns report disclosure requirements.
    #[must_use]
    pub fn report_disclosures(&self) -> &[HighAssuranceApprovalReportDisclosure] {
        &self.report_disclosures
    }

    /// Returns the sensitivity classification.
    #[must_use]
    pub const fn sensitivity(&self) -> WorkReportSensitivity {
        self.sensitivity
    }

    /// Returns the redaction policy.
    #[must_use]
    pub const fn redaction_policy(&self) -> WorkReportRedactionPolicy {
        self.redaction_policy
    }

    /// Returns validated redaction metadata.
    #[must_use]
    pub const fn redaction(&self) -> &RedactionMetadata {
        &self.redaction
    }
}

impl fmt::Debug for HighAssuranceApprovalControl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HighAssuranceApprovalControl")
            .field("control_id", &self.control_id)
            .field("control_version", &self.control_version)
            .field("schema_version", &self.schema_version)
            .field("protected_action_count", &self.protected_actions.len())
            .field("requester_approver_rule", &self.requester_approver_rule)
            .field("minimum_approvals", &self.minimum_approvals)
            .field("required_reference_count", &self.required_references.len())
            .field("expiration_policy", &self.expiration_policy)
            .field("revocation_policy", &self.revocation_policy)
            .field("denial_behavior", &self.denial_behavior)
            .field("report_disclosure_count", &self.report_disclosures.len())
            .field("sensitivity", &self.sensitivity)
            .field("redaction_policy", &self.redaction_policy)
            .field("redaction", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for HighAssuranceApprovalControl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct HighAssuranceApprovalControlWire {
            control_id: HighAssuranceApprovalControlId,
            control_version: HighAssuranceApprovalControlVersion,
            schema_version: SchemaVersion,
            protected_actions: Vec<HighAssuranceProtectedActionKind>,
            requester_approver_rule: HighAssuranceRequesterApproverRule,
            minimum_approvals: u16,
            required_references: Vec<HighAssuranceApprovalRequiredReference>,
            expiration_policy: HighAssuranceApprovalExpirationPolicy,
            revocation_policy: HighAssuranceApprovalRevocationPolicy,
            denial_behavior: HighAssuranceApprovalDenialBehavior,
            report_disclosures: Vec<HighAssuranceApprovalReportDisclosure>,
            sensitivity: WorkReportSensitivity,
            redaction_policy: WorkReportRedactionPolicy,
            redaction: RedactionMetadata,
        }

        let wire = HighAssuranceApprovalControlWire::deserialize(deserializer)?;
        Self::new(HighAssuranceApprovalControlDefinition {
            control_id: wire.control_id,
            control_version: wire.control_version,
            schema_version: wire.schema_version,
            protected_actions: wire.protected_actions,
            requester_approver_rule: wire.requester_approver_rule,
            minimum_approvals: wire.minimum_approvals,
            required_references: wire.required_references,
            expiration_policy: wire.expiration_policy,
            revocation_policy: wire.revocation_policy,
            denial_behavior: wire.denial_behavior,
            report_disclosures: wire.report_disclosures,
            sensitivity: wire.sensitivity,
            redaction_policy: wire.redaction_policy,
            redaction: wire.redaction,
        })
        .map_err(serde::de::Error::custom)
    }
}

fn validate_reference_list(
    required_code: &'static str,
    required_message: &'static str,
    references: &[HighAssuranceApprovalRequiredReference],
) -> Result<(), WorkflowOsError> {
    if references.is_empty() {
        return Err(validation_error(required_code, required_message));
    }

    let mut names = BTreeSet::new();
    for reference in references {
        reference.validate()?;
        if !names.insert(reference.name()) {
            return Err(validation_error(
                "high_assurance_approval.references.duplicate",
                "high-assurance approval controls cannot declare duplicate required reference names",
            ));
        }
    }
    Ok(())
}

fn validate_enum_list<T>(
    required_code: &'static str,
    required_message: &'static str,
    duplicate_code: &'static str,
    duplicate_message: &'static str,
    values: &[T],
) -> Result<(), WorkflowOsError>
where
    T: Ord + Copy,
{
    if values.is_empty() {
        return Err(validation_error(required_code, required_message));
    }

    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(*value) {
            return Err(validation_error(duplicate_code, duplicate_message));
        }
    }
    Ok(())
}

fn validate_identifier(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "high_assurance_approval.identifier.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > HIGH_ASSURANCE_IDENTIFIER_MAX_BYTES {
        return Err(validation_error(
            "high_assurance_approval.identifier.too_long",
            format!("{type_name} cannot exceed {HIGH_ASSURANCE_IDENTIFIER_MAX_BYTES} bytes"),
        ));
    }

    let is_valid = value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'));

    if !is_valid {
        return Err(validation_error(
            "high_assurance_approval.identifier.invalid_character",
            format!("{type_name} contains an invalid character"),
        ));
    }

    validate_not_secret_like(type_name, value)
}

fn validate_redaction_metadata(redaction: &RedactionMetadata) -> Result<(), WorkflowOsError> {
    if redaction.redacted_fields.len() > HIGH_ASSURANCE_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "high_assurance_approval.redaction.too_many_fields",
            "high-assurance approval redaction metadata contains too many fields",
        ));
    }

    if redaction.field_states.len() > HIGH_ASSURANCE_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "high_assurance_approval.redaction.too_many_states",
            "high-assurance approval redaction metadata contains too many field states",
        ));
    }

    for field in &redaction.redacted_fields {
        validate_redaction_field_name(field)?;
    }

    for state in &redaction.field_states {
        validate_redaction_field_name(&state.field)?;
        validate_redaction_reason(&state.reason)?;
    }

    Ok(())
}

fn validate_redaction_field_name(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "high_assurance_approval.redaction.field.empty",
            "high-assurance approval redaction field cannot be empty",
        ));
    }

    if value.len() > HIGH_ASSURANCE_REDACTION_FIELD_MAX_BYTES {
        return Err(validation_error(
            "high_assurance_approval.redaction.field.too_long",
            format!(
                "high-assurance approval redaction field cannot exceed {HIGH_ASSURANCE_REDACTION_FIELD_MAX_BYTES} bytes"
            ),
        ));
    }

    validate_not_secret_like("high-assurance approval redaction field", value)
}

fn validate_redaction_reason(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "high_assurance_approval.redaction.reason.empty",
            "high-assurance approval redaction reason cannot be empty",
        ));
    }

    if value.len() > HIGH_ASSURANCE_REDACTION_REASON_MAX_BYTES {
        return Err(validation_error(
            "high_assurance_approval.redaction.reason.too_long",
            format!(
                "high-assurance approval redaction reason cannot exceed {HIGH_ASSURANCE_REDACTION_REASON_MAX_BYTES} bytes"
            ),
        ));
    }

    validate_not_secret_like("high-assurance approval redaction reason", value)
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
            "high_assurance_approval.secret_like_value",
            format!("{type_name} contains sensitive-looking text"),
        ));
    }

    Ok(())
}

fn validation_error(code: &'static str, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}
