use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    discover_side_effect_references, discover_side_effect_references_from_store,
    validate_side_effect_approval_linkage_from_store, ActorId, AgentHarnessHookDisclosureId,
    AgentHarnessHookInvocationId, ApprovalReferenceId, CorrelationId, EventId, EvidenceReferenceId,
    RedactionMetadata, SchemaVersion, SideEffectApprovalLinkageFromStoreInput,
    SideEffectApprovalLinkageFromStoreResult, SideEffectApprovalLinkageStoreLoadMode,
    SideEffectCapability, SideEffectDiscoveryInput, SideEffectId, SideEffectLifecycleState,
    SideEffectMissingRecordPolicy, SideEffectRecord, SideEffectRecordStore,
    SideEffectStoreBackedDiscoveryInput, SideEffectTargetKind, SpecContentHash, Timestamp,
    TypedHandoffId, ValidationReferenceId, WorkReportArtifactStore, WorkflowDefinition, WorkflowId,
    WorkflowOsError, WorkflowRun, WorkflowRunEvent, WorkflowRunEventKind, WorkflowRunId,
    WorkflowRunStatus, WorkflowVersion,
};
use crate::{
    GitHubPullRequestCommentProviderWriteDisclosurePosture,
    GitHubPullRequestCommentProviderWriteReportDisclosure,
};

const REPORT_TEXT_MAX_BYTES: usize = 2_000;
const REPORT_REFERENCE_MAX_BYTES: usize = 256;
const REPORT_REDACTION_FIELD_MAX_BYTES: usize = 128;
const REPORT_REDACTION_REASON_MAX_BYTES: usize = 512;
const REPORT_REDACTION_MAX_ENTRIES: usize = 64;

static NEXT_WORK_REPORT_ID: AtomicU64 = AtomicU64::new(1);

/// Identifier for a generated work report.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct WorkReportId(String);

impl WorkReportId {
    /// Generates a new work report identifier.
    #[must_use]
    pub fn generate() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let counter = NEXT_WORK_REPORT_ID.fetch_add(1, Ordering::Relaxed);
        Self(format!("work-report-{timestamp}-{counter}"))
    }

    /// Creates a validated work report ID.
    ///
    /// # Errors
    ///
    /// Returns an error when the ID is empty, too long, contains invalid
    /// characters, or looks like a secret.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("WorkReportId", &value)?;
        Ok(Self(value))
    }

    /// Returns the ID as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkReportId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for WorkReportId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WorkReportId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<WorkReportId> for String {
    fn from(value: WorkReportId) -> Self {
        value.0
    }
}

impl TryFrom<String> for WorkReportId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for WorkReportId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Identifier for a work report contract.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct WorkReportContractId(String);

impl WorkReportContractId {
    /// Creates a validated work report contract ID.
    ///
    /// # Errors
    ///
    /// Returns an error when the ID is empty, too long, or contains
    /// characters outside the canonical identifier character set.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("WorkReportContractId", &value)?;
        Ok(Self(value))
    }

    /// Returns the ID as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkReportContractId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for WorkReportContractId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WorkReportContractId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<WorkReportContractId> for String {
    fn from(value: WorkReportContractId) -> Self {
        value.0
    }
}

impl TryFrom<String> for WorkReportContractId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for WorkReportContractId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Version for a work report contract.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct WorkReportContractVersion(String);

impl WorkReportContractVersion {
    /// Creates a validated work report contract version.
    ///
    /// # Errors
    ///
    /// Returns an error when the version is empty, too long, or contains
    /// characters outside the canonical identifier character set.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("WorkReportContractVersion", &value)?;
        Ok(Self(value))
    }

    /// Returns the version as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkReportContractVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for WorkReportContractVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WorkReportContractVersion")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<WorkReportContractVersion> for String {
    fn from(value: WorkReportContractVersion) -> Self {
        value.0
    }
}

impl TryFrom<String> for WorkReportContractVersion {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for WorkReportContractVersion {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Domain-neutral section kinds for a future governed work report.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkReportSectionKind {
    /// What the workflow attempted or completed.
    WorkPerformed,
    /// Evidence and citations considered during the work.
    EvidenceConsidered,
    /// Decisions made during the work.
    DecisionsMade,
    /// Policy gates evaluated during the work.
    PolicyGatesEvaluated,
    /// Approvals requested, granted, denied, or missing.
    Approvals,
    /// Validation and quality checks.
    ValidationAndQualityChecks,
    /// Side effects attempted, completed, skipped, denied, failed, or unsupported.
    SideEffects,
    /// Incomplete, deferred, skipped, blocked, or failed work.
    IncompleteOrDeferredWork,
    /// Known limitations affecting report interpretation.
    KnownLimitations,
    /// Residual risks or uncertainty.
    Risks,
    /// Operator or downstream handoff notes.
    OperatorHandoffNotes,
}

impl WorkReportSectionKind {
    /// Returns all v1 section kinds.
    #[must_use]
    pub const fn v1_required_kinds() -> [Self; 11] {
        [
            Self::WorkPerformed,
            Self::EvidenceConsidered,
            Self::DecisionsMade,
            Self::PolicyGatesEvaluated,
            Self::Approvals,
            Self::ValidationAndQualityChecks,
            Self::SideEffects,
            Self::IncompleteOrDeferredWork,
            Self::KnownLimitations,
            Self::Risks,
            Self::OperatorHandoffNotes,
        ]
    }
}

/// Requirement for one report section kind.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WorkReportSectionRequirement {
    /// Required section kind.
    pub kind: WorkReportSectionKind,
}

impl WorkReportSectionRequirement {
    /// Creates a required section.
    #[must_use]
    pub const fn required(kind: WorkReportSectionKind) -> Self {
        Self { kind }
    }
}

/// Citation kinds a future report section may be required to include.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkReportCitationKind {
    /// Citation to an `EvidenceReference`.
    EvidenceReference,
    /// Citation to a workflow event.
    WorkflowEvent,
    /// Citation to an audit event.
    AuditEvent,
    /// Citation to adapter telemetry.
    AdapterTelemetry,
    /// Citation to a validation diagnostic.
    ValidationDiagnostic,
    /// Citation to a local check result reference.
    LocalCheckResult,
    /// Citation to a typed handoff value.
    TypedHandoff,
    /// Citation to an agent harness hook invocation checkpoint.
    AgentHarnessHook,
    /// Citation to a bounded agent harness hook disclosure.
    AgentHarnessHookDisclosure,
    /// Citation to a governed side-effect record.
    SideEffect,
    /// Citation to an approval decision.
    ApprovalDecision,
    /// Citation to a policy decision.
    PolicyDecision,
    /// Future citation to reasoning lineage.
    ReasoningLineageNode,
}

/// Citation requirement for a future report contract.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WorkReportCitationRequirement {
    /// Citation kind.
    pub kind: WorkReportCitationKind,
    /// Whether at least one citation of this kind is required when relevant.
    pub required: bool,
}

/// Disclosure categories a future report contract may require.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkReportDisclosureKind {
    /// Incomplete, deferred, skipped, blocked, or failed work disclosure.
    IncompleteOrDeferredWork,
    /// Known limitations disclosure.
    KnownLimitations,
    /// Risk disclosure.
    Risks,
    /// Side-effect disclosure, including none/skipped/unsupported.
    SideEffects,
}

/// Disclosure requirements for a future report contract.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkReportDisclosureRequirements {
    required: BTreeSet<WorkReportDisclosureKind>,
}

impl WorkReportDisclosureRequirements {
    /// Creates disclosure requirements.
    #[must_use]
    pub fn new(required: impl IntoIterator<Item = WorkReportDisclosureKind>) -> Self {
        Self {
            required: required.into_iter().collect(),
        }
    }

    /// Creates the conservative v1 disclosure requirements.
    #[must_use]
    pub fn v1_required() -> Self {
        Self::new([
            WorkReportDisclosureKind::IncompleteOrDeferredWork,
            WorkReportDisclosureKind::KnownLimitations,
            WorkReportDisclosureKind::Risks,
            WorkReportDisclosureKind::SideEffects,
        ])
    }

    /// Returns whether the disclosure kind is required.
    #[must_use]
    pub fn requires(&self, kind: WorkReportDisclosureKind) -> bool {
        self.required.contains(&kind)
    }

    /// Returns required disclosure kinds.
    #[must_use]
    pub const fn required(&self) -> &BTreeSet<WorkReportDisclosureKind> {
        &self.required
    }
}

impl WorkReportCitationRequirement {
    /// Creates a citation requirement.
    #[must_use]
    pub const fn new(kind: WorkReportCitationKind, required: bool) -> Self {
        Self { kind, required }
    }
}

/// Sensitivity classification for a future work report contract and generated reports.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkReportSensitivity {
    /// Public report contract.
    Public,
    /// Internal report contract.
    Internal,
    /// Confidential report contract.
    Confidential,
    /// Regulated report contract.
    Regulated,
    /// Secret report contract.
    Secret,
    /// Unknown sensitivity, treated conservatively.
    Unknown,
}

impl WorkReportSensitivity {
    /// Conservative default for work report contracts.
    #[must_use]
    pub const fn conservative_default() -> Self {
        Self::Confidential
    }
}

/// Redaction policy for future report generation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkReportRedactionPolicy {
    /// Reports may store references and bounded redacted summaries only.
    ReferenceOnly,
    /// Reports may include bounded summaries after redaction.
    BoundedSummaries,
}

/// Terminal status represented by a generated work report.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkReportStatus {
    /// The workflow run completed successfully.
    Completed,
    /// The workflow run failed terminally.
    Failed,
    /// The workflow run was canceled terminally.
    Canceled,
    /// The workflow ended in an escalation or operator handoff posture.
    Escalated,
    /// The workflow is blocked and requires external resolution.
    Blocked,
}

/// Report-safe high-assurance approval decision posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkReportHighAssuranceApprovalDecision {
    /// A grant was validated before the approval decision event was recorded.
    Granted,
    /// A denial was validated before the approval decision event was recorded.
    Denied,
    /// The decision posture is not supplied to the report generator.
    NotAvailable,
}

/// Report-safe requester/approver separation posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkReportHighAssuranceRequesterApproverPosture {
    /// Requester and approver were validated as different actors.
    MustDifferValidated,
    /// The control allowed the same actor.
    SameActorAllowed,
    /// Human approver separation is deferred or externally governed.
    HumanApproverDeferred,
    /// The posture is not supplied to the report generator.
    NotAvailable,
}

/// Report-safe high-assurance expiration posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkReportHighAssuranceExpirationPosture {
    /// Expiration was not required.
    NotRequired,
    /// Expiration was required on the approval request.
    RequiredOnRequest,
    /// Expiration was validated as unexpired at decision time.
    UnexpiredAtDecision,
    /// Use-time expiration is unsupported by the current local slice.
    UseTimeUnsupported,
    /// The posture is not supplied to the report generator.
    NotAvailable,
}

/// Report-safe high-assurance revocation posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkReportHighAssuranceRevocationPosture {
    /// Revocation enforcement was not required.
    Unsupported,
    /// Explicit revocation-before-use remains deferred by the current local slice.
    ExplicitEventDeferred,
    /// Report-only revocation posture remains deferred by the current local slice.
    ReportOnlyDeferred,
    /// The posture is not supplied to the report generator.
    NotAvailable,
}

/// Report-safe high-assurance approval disclosure.
///
/// This type stores bounded posture and counts only. It intentionally does not
/// store approval payloads, actor IDs, control payloads, provider data, command
/// output, evidence payloads, or free-form approval text.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkReportHighAssuranceApprovalDisclosure {
    validation_used: bool,
    validation_passed: bool,
    decision: WorkReportHighAssuranceApprovalDecision,
    requester_approver_posture: WorkReportHighAssuranceRequesterApproverPosture,
    required_reference_count: usize,
    supplied_reference_count: usize,
    expiration_posture: WorkReportHighAssuranceExpirationPosture,
    revocation_posture: WorkReportHighAssuranceRevocationPosture,
    denial_fail_closed: bool,
}

/// Definition for a report-safe high-assurance approval disclosure.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct WorkReportHighAssuranceApprovalDisclosureDefinition {
    /// Whether high-assurance approval validation was used.
    pub validation_used: bool,
    /// Whether high-assurance approval validation passed.
    pub validation_passed: bool,
    /// Safe decision posture.
    pub decision: WorkReportHighAssuranceApprovalDecision,
    /// Safe requester/approver posture.
    pub requester_approver_posture: WorkReportHighAssuranceRequesterApproverPosture,
    /// Count of required references in the control packet.
    pub required_reference_count: usize,
    /// Count of supplied references in the approval decision packet.
    pub supplied_reference_count: usize,
    /// Safe expiration posture.
    pub expiration_posture: WorkReportHighAssuranceExpirationPosture,
    /// Safe revocation posture.
    pub revocation_posture: WorkReportHighAssuranceRevocationPosture,
    /// Whether denial behavior is fail-closed for this disclosure.
    pub denial_fail_closed: bool,
}

impl WorkReportHighAssuranceApprovalDisclosure {
    /// Creates a validated high-assurance approval disclosure.
    ///
    /// # Errors
    ///
    /// Returns an error when counts are unbounded or the posture is
    /// internally inconsistent.
    pub fn new(
        definition: WorkReportHighAssuranceApprovalDisclosureDefinition,
    ) -> Result<Self, WorkflowOsError> {
        let disclosure = Self {
            validation_used: definition.validation_used,
            validation_passed: definition.validation_passed,
            decision: definition.decision,
            requester_approver_posture: definition.requester_approver_posture,
            required_reference_count: definition.required_reference_count,
            supplied_reference_count: definition.supplied_reference_count,
            expiration_posture: definition.expiration_posture,
            revocation_posture: definition.revocation_posture,
            denial_fail_closed: definition.denial_fail_closed,
        };
        disclosure.validate()?;
        Ok(disclosure)
    }

    /// Validates disclosure consistency.
    ///
    /// # Errors
    ///
    /// Returns an error when counts are unbounded or validation-passed posture
    /// is supplied without validation-used posture.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        const HIGH_ASSURANCE_REFERENCE_COUNT_MAX: usize = 1_024;
        if !self.validation_used && self.validation_passed {
            return Err(validation_error(
                "work_report.high_assurance_approval.validation_inconsistent",
                "high-assurance approval disclosure cannot pass validation that was not used",
            ));
        }
        if self.required_reference_count > HIGH_ASSURANCE_REFERENCE_COUNT_MAX
            || self.supplied_reference_count > HIGH_ASSURANCE_REFERENCE_COUNT_MAX
        {
            return Err(validation_error(
                "work_report.high_assurance_approval.reference_count_unbounded",
                "high-assurance approval disclosure reference counts are unbounded",
            ));
        }
        Ok(())
    }

    /// Returns whether high-assurance validation was used.
    #[must_use]
    pub const fn validation_used(&self) -> bool {
        self.validation_used
    }

    /// Returns whether high-assurance validation passed.
    #[must_use]
    pub const fn validation_passed(&self) -> bool {
        self.validation_passed
    }

    /// Returns the safe decision posture.
    #[must_use]
    pub const fn decision(&self) -> WorkReportHighAssuranceApprovalDecision {
        self.decision
    }

    /// Returns the safe requester/approver posture.
    #[must_use]
    pub const fn requester_approver_posture(
        &self,
    ) -> WorkReportHighAssuranceRequesterApproverPosture {
        self.requester_approver_posture
    }

    /// Returns the required-reference count.
    #[must_use]
    pub const fn required_reference_count(&self) -> usize {
        self.required_reference_count
    }

    /// Returns the supplied-reference count.
    #[must_use]
    pub const fn supplied_reference_count(&self) -> usize {
        self.supplied_reference_count
    }

    /// Returns the safe expiration posture.
    #[must_use]
    pub const fn expiration_posture(&self) -> WorkReportHighAssuranceExpirationPosture {
        self.expiration_posture
    }

    /// Returns the safe revocation posture.
    #[must_use]
    pub const fn revocation_posture(&self) -> WorkReportHighAssuranceRevocationPosture {
        self.revocation_posture
    }

    /// Returns whether denial behavior is fail-closed.
    #[must_use]
    pub const fn denial_fail_closed(&self) -> bool {
        self.denial_fail_closed
    }
}

impl fmt::Debug for WorkReportHighAssuranceApprovalDisclosure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkReportHighAssuranceApprovalDisclosure")
            .field("validation_used", &self.validation_used)
            .field("validation_passed", &self.validation_passed)
            .field("decision", &self.decision)
            .field(
                "requester_approver_posture",
                &self.requester_approver_posture,
            )
            .field("required_reference_count", &self.required_reference_count)
            .field("supplied_reference_count", &self.supplied_reference_count)
            .field("expiration_posture", &self.expiration_posture)
            .field("revocation_posture", &self.revocation_posture)
            .field("denial_fail_closed", &self.denial_fail_closed)
            .finish()
    }
}

impl<'de> Deserialize<'de> for WorkReportHighAssuranceApprovalDisclosure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definition =
            WorkReportHighAssuranceApprovalDisclosureDefinition::deserialize(deserializer)?;
        Self::new(definition).map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for WorkReportHighAssuranceApprovalDisclosureDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            validation_used: bool,
            validation_passed: bool,
            decision: WorkReportHighAssuranceApprovalDecision,
            requester_approver_posture: WorkReportHighAssuranceRequesterApproverPosture,
            required_reference_count: usize,
            supplied_reference_count: usize,
            expiration_posture: WorkReportHighAssuranceExpirationPosture,
            revocation_posture: WorkReportHighAssuranceRevocationPosture,
            denial_fail_closed: bool,
        }

        let wire = Wire::deserialize(deserializer)?;
        let definition = Self {
            validation_used: wire.validation_used,
            validation_passed: wire.validation_passed,
            decision: wire.decision,
            requester_approver_posture: wire.requester_approver_posture,
            required_reference_count: wire.required_reference_count,
            supplied_reference_count: wire.supplied_reference_count,
            expiration_posture: wire.expiration_posture,
            revocation_posture: wire.revocation_posture,
            denial_fail_closed: wire.denial_fail_closed,
        };
        WorkReportHighAssuranceApprovalDisclosure::new(definition)
            .map_err(serde::de::Error::custom)?;
        Ok(definition)
    }
}

/// Stable non-payload reference used for report citation targets that do not
/// yet have a dedicated core identifier.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct WorkReportStableReference(String);

impl WorkReportStableReference {
    /// Creates a validated stable reference.
    ///
    /// # Errors
    ///
    /// Returns an error when the reference is empty, too long, or secret-like.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_reference_text("work report stable reference", &value)?;
        Ok(Self(value))
    }

    /// Returns the reference as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkReportStableReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for WorkReportStableReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WorkReportStableReference")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<WorkReportStableReference> for String {
    fn from(value: WorkReportStableReference) -> Self {
        value.0
    }
}

impl TryFrom<String> for WorkReportStableReference {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Target cited by a work report section.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum WorkReportCitationTarget {
    /// Citation to an `EvidenceReference`.
    EvidenceReference {
        /// Evidence reference ID.
        evidence_reference_id: EvidenceReferenceId,
    },
    /// Citation to a workflow event.
    WorkflowEvent {
        /// Workflow event ID.
        event_id: EventId,
    },
    /// Citation to an audit event.
    AuditEvent {
        /// Audit event ID.
        audit_event_id: EventId,
    },
    /// Citation to an adapter telemetry record.
    AdapterTelemetry {
        /// Stable adapter telemetry reference.
        reference: WorkReportStableReference,
    },
    /// Citation to a validation diagnostic or result.
    ValidationDiagnostic {
        /// Validation reference ID.
        validation_reference_id: ValidationReferenceId,
    },
    /// Citation to a local check result reference.
    LocalCheckResult {
        /// Stable local check result reference.
        reference: WorkReportStableReference,
    },
    /// Citation to a typed handoff value.
    TypedHandoff {
        /// Typed handoff ID.
        typed_handoff_id: TypedHandoffId,
    },
    /// Citation to an agent harness hook invocation checkpoint.
    AgentHarnessHook {
        /// Agent harness hook invocation ID.
        hook_invocation_id: AgentHarnessHookInvocationId,
    },
    /// Citation to a bounded agent harness hook disclosure.
    AgentHarnessHookDisclosure {
        /// Agent harness hook disclosure ID.
        disclosure_id: AgentHarnessHookDisclosureId,
    },
    /// Citation to a governed side-effect record.
    SideEffect {
        /// Side-effect record ID.
        side_effect_id: SideEffectId,
    },
    /// Citation to an approval decision.
    ApprovalDecision {
        /// Approval reference ID.
        approval_reference_id: ApprovalReferenceId,
    },
    /// Citation to a policy decision.
    PolicyDecision {
        /// Policy decision event ID.
        event_id: EventId,
    },
    /// Future citation vocabulary for reasoning lineage.
    ReasoningLineageNode {
        /// Stable future lineage-node reference.
        reference: WorkReportStableReference,
    },
}

impl WorkReportCitationTarget {
    /// Returns the citation kind represented by this target.
    #[must_use]
    pub const fn citation_kind(&self) -> WorkReportCitationKind {
        match self {
            Self::EvidenceReference { .. } => WorkReportCitationKind::EvidenceReference,
            Self::WorkflowEvent { .. } => WorkReportCitationKind::WorkflowEvent,
            Self::AuditEvent { .. } => WorkReportCitationKind::AuditEvent,
            Self::AdapterTelemetry { .. } => WorkReportCitationKind::AdapterTelemetry,
            Self::ValidationDiagnostic { .. } => WorkReportCitationKind::ValidationDiagnostic,
            Self::LocalCheckResult { .. } => WorkReportCitationKind::LocalCheckResult,
            Self::TypedHandoff { .. } => WorkReportCitationKind::TypedHandoff,
            Self::AgentHarnessHook { .. } => WorkReportCitationKind::AgentHarnessHook,
            Self::AgentHarnessHookDisclosure { .. } => {
                WorkReportCitationKind::AgentHarnessHookDisclosure
            }
            Self::SideEffect { .. } => WorkReportCitationKind::SideEffect,
            Self::ApprovalDecision { .. } => WorkReportCitationKind::ApprovalDecision,
            Self::PolicyDecision { .. } => WorkReportCitationKind::PolicyDecision,
            Self::ReasoningLineageNode { .. } => WorkReportCitationKind::ReasoningLineageNode,
        }
    }
}

impl fmt::Debug for WorkReportCitationTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkReportCitationTarget")
            .field("kind", &self.citation_kind())
            .field("reference", &"[REDACTED]")
            .finish()
    }
}

/// Citation from a work report section to a stable referenced artifact.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkReportCitation {
    target: WorkReportCitationTarget,
    summary: Option<String>,
    missing: bool,
    redaction: RedactionMetadata,
    sensitivity: WorkReportSensitivity,
}

/// Input fields for constructing a validated `WorkReportCitation`.
pub struct WorkReportCitationDefinition {
    /// Citation target.
    pub target: WorkReportCitationTarget,
    /// Optional bounded redacted summary.
    pub summary: Option<String>,
    /// Whether the citation is explicitly missing.
    pub missing: bool,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
    /// Sensitivity classification.
    pub sensitivity: WorkReportSensitivity,
}

impl WorkReportCitation {
    /// Creates a validated citation.
    ///
    /// # Errors
    ///
    /// Returns an error when the summary is too large or secret-like.
    pub fn new(definition: WorkReportCitationDefinition) -> Result<Self, WorkflowOsError> {
        let citation = Self {
            target: definition.target,
            summary: definition.summary,
            missing: definition.missing,
            redaction: definition.redaction,
            sensitivity: definition.sensitivity,
        };
        citation.validate()?;
        Ok(citation)
    }

    /// Validates the citation.
    ///
    /// # Errors
    ///
    /// Returns an error when bounded text fields are invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if let Some(summary) = &self.summary {
            validate_report_text("citation summary", summary)?;
        }
        validate_report_redaction_metadata(&self.redaction)?;
        Ok(())
    }

    /// Returns the citation target.
    #[must_use]
    pub const fn target(&self) -> &WorkReportCitationTarget {
        &self.target
    }

    /// Returns the citation kind.
    #[must_use]
    pub const fn citation_kind(&self) -> WorkReportCitationKind {
        self.target.citation_kind()
    }

    /// Returns whether the citation is explicitly missing.
    #[must_use]
    pub const fn missing(&self) -> bool {
        self.missing
    }

    /// Returns the bounded summary.
    #[must_use]
    pub fn summary(&self) -> Option<&str> {
        self.summary.as_deref()
    }
}

impl fmt::Debug for WorkReportCitation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkReportCitation")
            .field("target", &self.target)
            .field("summary", &self.summary.as_ref().map(|_| "[REDACTED]"))
            .field("missing", &self.missing)
            .field(
                "redaction",
                &RedactedRedactionMetadataDebug(&self.redaction),
            )
            .field("sensitivity", &self.sensitivity)
            .finish()
    }
}

impl<'de> Deserialize<'de> for WorkReportCitation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WorkReportCitationWire {
            target: WorkReportCitationTarget,
            summary: Option<String>,
            missing: bool,
            redaction: RedactionMetadata,
            sensitivity: WorkReportSensitivity,
        }

        let wire = WorkReportCitationWire::deserialize(deserializer)?;
        Self::new(WorkReportCitationDefinition {
            target: wire.target,
            summary: wire.summary,
            missing: wire.missing,
            redaction: wire.redaction,
            sensitivity: wire.sensitivity,
        })
        .map_err(serde::de::Error::custom)
    }
}

/// One bounded section of a generated work report.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkReportSection {
    kind: WorkReportSectionKind,
    summary: Option<String>,
    citations: Vec<WorkReportCitation>,
}

impl WorkReportSection {
    /// Creates a validated report section.
    ///
    /// # Errors
    ///
    /// Returns an error when the summary or citations are invalid.
    pub fn new(
        kind: WorkReportSectionKind,
        summary: Option<String>,
        citations: Vec<WorkReportCitation>,
    ) -> Result<Self, WorkflowOsError> {
        let section = Self {
            kind,
            summary,
            citations,
        };
        section.validate()?;
        Ok(section)
    }

    /// Validates the report section.
    ///
    /// # Errors
    ///
    /// Returns an error when bounded text or citation fields are invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if let Some(summary) = &self.summary {
            validate_report_text("section summary", summary)?;
        }
        validate_citations(&self.citations)
    }

    /// Returns the section kind.
    #[must_use]
    pub const fn kind(&self) -> WorkReportSectionKind {
        self.kind
    }

    /// Returns the bounded summary.
    #[must_use]
    pub fn summary(&self) -> Option<&str> {
        self.summary.as_deref()
    }

    /// Returns citations.
    #[must_use]
    pub fn citations(&self) -> &[WorkReportCitation] {
        &self.citations
    }
}

impl fmt::Debug for WorkReportSection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkReportSection")
            .field("kind", &self.kind)
            .field("summary", &self.summary.as_ref().map(|_| "[REDACTED]"))
            .field("citation_count", &self.citations.len())
            .finish()
    }
}

impl<'de> Deserialize<'de> for WorkReportSection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WorkReportSectionWire {
            kind: WorkReportSectionKind,
            summary: Option<String>,
            citations: Vec<WorkReportCitation>,
        }

        let wire = WorkReportSectionWire::deserialize(deserializer)?;
        Self::new(wire.kind, wire.summary, wire.citations).map_err(serde::de::Error::custom)
    }
}

macro_rules! report_note_type {
    ($name:ident, $label:literal) => {
        #[doc = $label]
        #[derive(Clone, Eq, PartialEq, Serialize)]
        pub struct $name {
            summary: String,
            citations: Vec<WorkReportCitation>,
        }

        impl $name {
            /// Creates a validated report note.
            ///
            /// # Errors
            ///
            /// Returns an error when the summary or citations are invalid.
            pub fn new(
                summary: impl Into<String>,
                citations: Vec<WorkReportCitation>,
            ) -> Result<Self, WorkflowOsError> {
                let value = Self {
                    summary: summary.into(),
                    citations,
                };
                value.validate()?;
                Ok(value)
            }

            /// Validates the report note.
            ///
            /// # Errors
            ///
            /// Returns an error when bounded text or citation fields are invalid.
            pub fn validate(&self) -> Result<(), WorkflowOsError> {
                validate_report_text("report disclosure summary", &self.summary)?;
                validate_citations(&self.citations)
            }

            /// Returns the bounded summary.
            #[must_use]
            pub fn summary(&self) -> &str {
                &self.summary
            }

            /// Returns citations.
            #[must_use]
            pub fn citations(&self) -> &[WorkReportCitation] {
                &self.citations
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_struct(stringify!($name))
                    .field("summary", &"[REDACTED]")
                    .field("citation_count", &self.citations.len())
                    .finish()
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                #[derive(Deserialize)]
                struct Wire {
                    summary: String,
                    citations: Vec<WorkReportCitation>,
                }

                let wire = Wire::deserialize(deserializer)?;
                Self::new(wire.summary, wire.citations).map_err(serde::de::Error::custom)
            }
        }
    };
}

report_note_type!(
    WorkReportIncompleteWorkDisclosure,
    "Incomplete, deferred, skipped, blocked, or failed work disclosure."
);
report_note_type!(
    WorkReportKnownLimitation,
    "Known limitation affecting report interpretation."
);
report_note_type!(WorkReportRisk, "Residual risk or uncertainty disclosure.");
report_note_type!(
    WorkReportHandoffNote,
    "Operator or downstream workflow handoff note."
);

/// Identity and generation context for a work report.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkReportGenerationContext {
    /// Workflow ID.
    pub workflow_id: WorkflowId,
    /// Workflow version.
    pub workflow_version: WorkflowVersion,
    /// Workflow spec schema version.
    pub schema_version: SchemaVersion,
    /// Workflow spec content hash.
    pub spec_hash: SpecContentHash,
    /// Workflow run ID.
    pub run_id: WorkflowRunId,
    /// Terminal run status represented by the report.
    pub terminal_run_status: WorkReportStatus,
    /// Report generation timestamp.
    pub generated_at: Timestamp,
    /// Actor or system actor that generated the report.
    pub generated_by: ActorId,
    /// Correlation ID where available.
    pub correlation_id: Option<CorrelationId>,
}

impl fmt::Debug for WorkReportGenerationContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkReportGenerationContext")
            .field("workflow_id", &"[REDACTED]")
            .field("workflow_version", &"[REDACTED]")
            .field("schema_version", &self.schema_version)
            .field("spec_hash", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("terminal_run_status", &self.terminal_run_status)
            .field("generated_at", &self.generated_at)
            .field("generated_by", &"[REDACTED]")
            .field(
                "correlation_id",
                &self.correlation_id.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

/// Domain-neutral generated work report model.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkReport {
    report_id: WorkReportId,
    report_contract_id: WorkReportContractId,
    report_contract_version: WorkReportContractVersion,
    generation_context: WorkReportGenerationContext,
    sections: Vec<WorkReportSection>,
    incomplete_work: Vec<WorkReportIncompleteWorkDisclosure>,
    known_limitations: Vec<WorkReportKnownLimitation>,
    risks: Vec<WorkReportRisk>,
    handoff_notes: Vec<WorkReportHandoffNote>,
    high_assurance_approval: Option<WorkReportHighAssuranceApprovalDisclosure>,
    sensitivity: WorkReportSensitivity,
    redaction: RedactionMetadata,
}

/// Input fields for constructing a validated `WorkReport`.
pub struct WorkReportDefinition {
    /// Report ID.
    pub report_id: WorkReportId,
    /// Report contract ID.
    pub report_contract_id: WorkReportContractId,
    /// Report contract version.
    pub report_contract_version: WorkReportContractVersion,
    /// Generation context.
    pub generation_context: WorkReportGenerationContext,
    /// Report sections.
    pub sections: Vec<WorkReportSection>,
    /// Incomplete or deferred work disclosures.
    pub incomplete_work: Vec<WorkReportIncompleteWorkDisclosure>,
    /// Known limitations.
    pub known_limitations: Vec<WorkReportKnownLimitation>,
    /// Risks.
    pub risks: Vec<WorkReportRisk>,
    /// Operator handoff notes.
    pub handoff_notes: Vec<WorkReportHandoffNote>,
    /// Optional report-safe high-assurance approval disclosure.
    pub high_assurance_approval: Option<WorkReportHighAssuranceApprovalDisclosure>,
    /// Sensitivity.
    pub sensitivity: WorkReportSensitivity,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
}

/// Explicit inputs for in-memory terminal local work report generation.
///
/// The generator borrows an already-terminal run and never mutates runtime
/// state, appends events, writes files, or persists reports.
pub struct TerminalLocalWorkReportInput<'a> {
    /// Report ID.
    pub report_id: WorkReportId,
    /// Report contract ID.
    pub report_contract_id: WorkReportContractId,
    /// Report contract version.
    pub report_contract_version: WorkReportContractVersion,
    /// Already-terminal workflow run.
    pub run: &'a WorkflowRun,
    /// Report generation timestamp.
    pub generated_at: Timestamp,
    /// Actor or system actor generating the in-memory report.
    pub generated_by: ActorId,
    /// Optional correlation ID for report generation.
    pub correlation_id: Option<CorrelationId>,
    /// Report sensitivity.
    pub sensitivity: WorkReportSensitivity,
    /// Report redaction metadata.
    pub redaction: RedactionMetadata,
    /// Existing evidence reference IDs to cite.
    pub evidence_reference_ids: Vec<EvidenceReferenceId>,
    /// Validation diagnostic/result references to cite.
    pub validation_reference_ids: Vec<ValidationReferenceId>,
    /// Stable local check result references to cite.
    pub local_check_result_references: Vec<WorkReportStableReference>,
    /// Workflow event IDs to cite.
    pub workflow_event_ids: Vec<EventId>,
    /// Audit event IDs to cite.
    pub audit_event_ids: Vec<EventId>,
    /// Stable adapter telemetry references to cite.
    pub adapter_telemetry_references: Vec<WorkReportStableReference>,
    /// Policy decision event IDs to cite.
    pub policy_event_ids: Vec<EventId>,
    /// Approval decision references to cite, where stable IDs already exist.
    pub approval_reference_ids: Vec<ApprovalReferenceId>,
    /// Optional report-safe high-assurance approval posture disclosure.
    pub high_assurance_approval: Option<WorkReportHighAssuranceApprovalDisclosure>,
    /// Typed handoff IDs to cite, where stable IDs already exist.
    pub typed_handoff_ids: Vec<TypedHandoffId>,
    /// Agent harness hook invocation IDs to cite, where stable IDs already exist.
    pub agent_harness_hook_invocation_ids: Vec<AgentHarnessHookInvocationId>,
    /// Agent harness hook disclosure IDs to cite, where stable IDs already exist.
    pub agent_harness_hook_disclosure_ids: Vec<AgentHarnessHookDisclosureId>,
    /// `SideEffect` IDs to cite, where stable IDs already exist.
    pub side_effect_ids: Vec<SideEffectId>,
    /// Bounded GitHub PR comment provider reconciliation disclosures.
    pub github_pr_comment_provider_disclosures:
        Vec<GitHubPullRequestCommentProviderWriteReportDisclosure>,
    /// Bounded incomplete/deferred work disclosures.
    pub incomplete_work: Vec<String>,
    /// Bounded known limitations.
    pub known_limitations: Vec<String>,
    /// Bounded risks.
    pub risks: Vec<String>,
    /// Bounded operator handoff notes.
    pub handoff_notes: Vec<String>,
}

/// Explicit `SideEffect` discovery policy for terminal local `WorkReport` generation.
///
/// This policy is opt-in and in-memory only. It does not mutate runtime state,
/// append events, persist reports, write artifacts, execute side effects, or
/// read hidden global state.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TerminalLocalWorkReportSideEffectDiscoveryInput {
    /// Include `SideEffect` workflow events already present on the borrowed run.
    pub include_workflow_events: bool,
    /// Include matching `SideEffectRecord` values from the supplied store.
    pub include_store_records: bool,
    /// Require every discovered `SideEffect` ID to have a matching stored record.
    pub require_records: bool,
}

/// Explicit input for validating `SideEffect` citations in a work report artifact.
///
/// This helper input borrows a validated artifact and does not imply artifact
/// persistence, side-effect discovery, workflow mutation, event emission, or
/// side-effect execution.
#[derive(Clone, Copy)]
pub struct WorkReportArtifactSideEffectIntegrityInput<'a> {
    /// Work report artifact whose cited `SideEffect` IDs should be checked.
    pub artifact: &'a WorkReportArtifactRecord,
    /// Whether every cited `SideEffect` ID must resolve to a stored record.
    pub require_all_side_effect_citations: bool,
}

impl fmt::Debug for WorkReportArtifactSideEffectIntegrityInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkReportArtifactSideEffectIntegrityInput")
            .field("artifact", &"[REDACTED]")
            .field(
                "require_all_side_effect_citations",
                &self.require_all_side_effect_citations,
            )
            .finish()
    }
}

/// Bounded result for work report artifact `SideEffect` citation integrity.
///
/// Counts are reference-only and intentionally do not expose report IDs,
/// run IDs, `SideEffect` IDs, targets, summaries, store paths, or payloads.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct WorkReportArtifactSideEffectIntegrityResult {
    cited: usize,
    resolved: usize,
    missing: usize,
    duplicate_citations: usize,
}

impl WorkReportArtifactSideEffectIntegrityResult {
    /// Returns the unique cited `SideEffect` ID count.
    #[must_use]
    pub const fn cited_side_effect_count(&self) -> usize {
        self.cited
    }

    /// Returns the count of cited IDs that resolved to matching records.
    #[must_use]
    pub const fn resolved_side_effect_count(&self) -> usize {
        self.resolved
    }

    /// Returns the count of cited IDs with no stored record.
    #[must_use]
    pub const fn missing_side_effect_count(&self) -> usize {
        self.missing
    }

    /// Returns the count of duplicate `SideEffect` citations.
    #[must_use]
    pub const fn duplicate_side_effect_citation_count(&self) -> usize {
        self.duplicate_citations
    }
}

impl fmt::Debug for WorkReportArtifactSideEffectIntegrityResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkReportArtifactSideEffectIntegrityResult")
            .field("cited_side_effect_count", &self.cited)
            .field("resolved_side_effect_count", &self.resolved)
            .field("missing_side_effect_count", &self.missing)
            .field(
                "duplicate_side_effect_citation_count",
                &self.duplicate_citations,
            )
            .finish()
    }
}

/// Explicit input for validating GitHub PR comment `SideEffect` citations in a
/// work report artifact.
///
/// This helper input is validation-only. It does not write artifacts, append
/// events, discover side effects, call providers, execute side effects, or
/// mutate workflow state.
#[derive(Clone, Copy)]
pub struct GitHubPullRequestCommentReportArtifactCitationInput<'a> {
    /// Work report artifact that should cite the expected GitHub PR comment
    /// `SideEffect`.
    pub artifact: &'a WorkReportArtifactRecord,
    /// Expected proposed GitHub PR comment `SideEffect` ID.
    pub side_effect_id: &'a SideEffectId,
    /// Optional accepted workflow events supplied by the caller.
    pub workflow_events: Option<&'a [WorkflowRunEvent]>,
    /// Whether the cited record must exist in the supplied store.
    pub require_record: bool,
    /// Whether a matching accepted `SideEffectProposed` workflow event is
    /// required in `workflow_events`.
    pub require_accepted_event: bool,
}

impl fmt::Debug for GitHubPullRequestCommentReportArtifactCitationInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentReportArtifactCitationInput")
            .field("artifact", &"[REDACTED]")
            .field("side_effect_id", &"[REDACTED]")
            .field(
                "workflow_event_count",
                &self.workflow_events.map_or(0, <[WorkflowRunEvent]>::len),
            )
            .field("require_record", &self.require_record)
            .field("require_accepted_event", &self.require_accepted_event)
            .finish()
    }
}

/// Bounded result for GitHub PR comment report artifact citation validation.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct GitHubPullRequestCommentReportArtifactCitationResult {
    side_effect_integrity: WorkReportArtifactSideEffectIntegrityResult,
    record_validated: bool,
    accepted_event_count: usize,
}

impl GitHubPullRequestCommentReportArtifactCitationResult {
    /// Returns the generic `SideEffect` artifact integrity result.
    #[must_use]
    pub const fn side_effect_integrity(&self) -> &WorkReportArtifactSideEffectIntegrityResult {
        &self.side_effect_integrity
    }

    /// Returns whether the expected GitHub PR comment record was loaded and
    /// validated as a proposed GitHub write.
    #[must_use]
    pub const fn record_validated(&self) -> bool {
        self.record_validated
    }

    /// Returns the count of accepted `SideEffectProposed` workflow events that
    /// matched the expected `SideEffect` ID.
    #[must_use]
    pub const fn accepted_event_count(&self) -> usize {
        self.accepted_event_count
    }
}

impl fmt::Debug for GitHubPullRequestCommentReportArtifactCitationResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentReportArtifactCitationResult")
            .field("side_effect_integrity", &self.side_effect_integrity)
            .field("record_validated", &self.record_validated)
            .field("accepted_event_count", &self.accepted_event_count)
            .finish()
    }
}

/// Explicit input for writing a report artifact only after GitHub PR comment
/// `SideEffect` citation validation passes.
///
/// This composition input is local and explicit. It does not generate reports,
/// discover side effects, append events, mutate workflow state, call providers,
/// execute side effects, or expose CLI behavior.
#[derive(Clone, Copy)]
pub struct GitHubPullRequestCommentReportArtifactWriteInput<'a> {
    /// Existing governed artifact write input to run after GitHub citation
    /// validation passes.
    pub governed_write: WorkReportArtifactGovernedWriteInput<'a>,
    /// Expected proposed GitHub PR comment `SideEffect` ID.
    pub side_effect_id: &'a SideEffectId,
    /// Optional accepted workflow events supplied by the caller.
    pub workflow_events: Option<&'a [WorkflowRunEvent]>,
    /// GitHub PR comment citation validation requirements.
    pub citation_policy: GitHubPullRequestCommentReportArtifactCitationPolicy,
}

/// Explicit local integration input for writing a GitHub PR comment report
/// artifact after terminal run/report context is supplied by the caller.
///
/// This integration helper remains local and explicit. It does not run
/// workflows, generate reports, discover side effects, append events, call
/// providers, execute side effects, mutate workflow state, expose CLI behavior,
/// or make artifact writing automatic.
#[derive(Clone, Copy)]
pub struct GitHubPullRequestCommentReportArtifactIntegrationInput<'a> {
    /// Terminal workflow run that produced the report artifact.
    pub run: &'a WorkflowRun,
    /// Validated report artifact to write.
    pub artifact: &'a WorkReportArtifactRecord,
    /// Expected proposed GitHub PR comment `SideEffect` ID.
    pub side_effect_id: &'a SideEffectId,
    /// Optional accepted workflow events supplied by the caller.
    pub workflow_events: Option<&'a [WorkflowRunEvent]>,
    /// Whether every `SideEffect` citation in the artifact must resolve to a
    /// stored `SideEffectRecord`.
    pub require_all_side_effect_citations: bool,
    /// Whether `RequiresApproval` side effects must cite approval references.
    pub require_approval_references_for_requires_approval: bool,
    /// Whether approved or denied side effects must include decision references.
    pub require_decision_for_approved_or_denied: bool,
    /// Optional high-assurance disclosure policy for the artifact.
    pub high_assurance_disclosure_policy: WorkReportArtifactHighAssuranceDisclosurePolicy,
    /// GitHub PR comment citation validation requirements.
    pub citation_policy: GitHubPullRequestCommentReportArtifactCitationPolicy,
}

/// Explicit provider-candidate integration selector for report artifact writes.
///
/// Provider integrations are validation/composition only. They must not call
/// providers, execute side effects, append events, generate reports, or make
/// artifact writes automatic.
#[derive(Clone, Copy, Default)]
pub enum ReportArtifactWriteProviderIntegration<'a> {
    /// No provider-candidate-specific citation gate.
    #[default]
    None,
    /// Validate the artifact as citing an expected proposed GitHub PR comment
    /// `SideEffect` before generic artifact gates run.
    GitHubPullRequestComment {
        /// Expected proposed GitHub PR comment `SideEffect` ID.
        side_effect_id: &'a SideEffectId,
        /// Optional accepted workflow events supplied by the caller.
        workflow_events: Option<&'a [WorkflowRunEvent]>,
        /// GitHub PR comment citation validation requirements.
        citation_policy: GitHubPullRequestCommentReportArtifactCitationPolicy,
    },
}

/// Explicit local integration input for writing a report artifact after
/// composing generic and optional provider-candidate gates.
///
/// This helper input remains local and explicit. It does not run workflows,
/// generate reports, discover side effects, append events, call providers,
/// execute side effects, mutate workflow state, expose CLI behavior, or make
/// artifact writing automatic.
#[derive(Clone, Copy)]
pub struct ReportArtifactWriteIntegrationInput<'a> {
    /// Terminal workflow run that produced the report artifact.
    pub run: &'a WorkflowRun,
    /// Validated report artifact to write.
    pub artifact: &'a WorkReportArtifactRecord,
    /// Whether every `SideEffect` citation in the artifact must resolve to a
    /// stored `SideEffectRecord`.
    pub require_all_side_effect_citations: bool,
    /// Whether `RequiresApproval` side effects must cite approval references.
    pub require_approval_references_for_requires_approval: bool,
    /// Whether approved or denied side effects must include decision references.
    pub require_decision_for_approved_or_denied: bool,
    /// Optional high-assurance disclosure policy for the artifact.
    pub high_assurance_disclosure_policy: WorkReportArtifactHighAssuranceDisclosurePolicy,
    /// Optional provider-candidate-specific integration gate.
    pub provider_integration: ReportArtifactWriteProviderIntegration<'a>,
}

/// Validation policy for GitHub PR comment report artifact citation composition.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GitHubPullRequestCommentReportArtifactCitationPolicy {
    /// Whether the expected GitHub PR comment record must exist.
    pub require_record: bool,
    /// Whether a matching accepted `SideEffectProposed` event is required.
    pub require_accepted_event: bool,
}

impl fmt::Debug for GitHubPullRequestCommentReportArtifactWriteInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentReportArtifactWriteInput")
            .field("governed_write", &self.governed_write)
            .field("side_effect_id", &"[REDACTED]")
            .field(
                "workflow_event_count",
                &self.workflow_events.map_or(0, <[WorkflowRunEvent]>::len),
            )
            .field("citation_policy", &self.citation_policy)
            .finish()
    }
}

impl fmt::Debug for GitHubPullRequestCommentReportArtifactIntegrationInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentReportArtifactIntegrationInput")
            .field("run", &"[REDACTED]")
            .field("artifact", &"[REDACTED]")
            .field("side_effect_id", &"[REDACTED]")
            .field(
                "workflow_event_count",
                &self.workflow_events.map_or(0, <[WorkflowRunEvent]>::len),
            )
            .field(
                "require_all_side_effect_citations",
                &self.require_all_side_effect_citations,
            )
            .field(
                "require_approval_references_for_requires_approval",
                &self.require_approval_references_for_requires_approval,
            )
            .field(
                "require_decision_for_approved_or_denied",
                &self.require_decision_for_approved_or_denied,
            )
            .field(
                "high_assurance_disclosure_policy",
                &self.high_assurance_disclosure_policy,
            )
            .field("citation_policy", &self.citation_policy)
            .finish()
    }
}

impl fmt::Debug for ReportArtifactWriteProviderIntegration<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => formatter.write_str("ReportArtifactWriteProviderIntegration::None"),
            Self::GitHubPullRequestComment {
                workflow_events,
                citation_policy,
                ..
            } => formatter
                .debug_struct("ReportArtifactWriteProviderIntegration::GitHubPullRequestComment")
                .field("side_effect_id", &"[REDACTED]")
                .field(
                    "workflow_event_count",
                    &workflow_events.map_or(0, <[WorkflowRunEvent]>::len),
                )
                .field("citation_policy", citation_policy)
                .finish(),
        }
    }
}

impl fmt::Debug for ReportArtifactWriteIntegrationInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ReportArtifactWriteIntegrationInput")
            .field("run", &"[REDACTED]")
            .field("artifact", &"[REDACTED]")
            .field(
                "require_all_side_effect_citations",
                &self.require_all_side_effect_citations,
            )
            .field(
                "require_approval_references_for_requires_approval",
                &self.require_approval_references_for_requires_approval,
            )
            .field(
                "require_decision_for_approved_or_denied",
                &self.require_decision_for_approved_or_denied,
            )
            .field(
                "high_assurance_disclosure_policy",
                &self.high_assurance_disclosure_policy,
            )
            .field("provider_integration", &self.provider_integration)
            .finish()
    }
}

/// Bounded result from GitHub PR comment report artifact write composition.
#[derive(Clone, Eq, PartialEq)]
pub struct GitHubPullRequestCommentReportArtifactWriteResult {
    github_pr_comment_citation: GitHubPullRequestCommentReportArtifactCitationResult,
    artifact_write: WorkReportArtifactGovernedWriteResult,
}

impl GitHubPullRequestCommentReportArtifactWriteResult {
    /// Returns the GitHub PR comment citation validation result.
    #[must_use]
    pub const fn github_pr_comment_citation(
        &self,
    ) -> &GitHubPullRequestCommentReportArtifactCitationResult {
        &self.github_pr_comment_citation
    }

    /// Returns the governed artifact write result.
    #[must_use]
    pub const fn artifact_write(&self) -> &WorkReportArtifactGovernedWriteResult {
        &self.artifact_write
    }
}

impl fmt::Debug for GitHubPullRequestCommentReportArtifactWriteResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubPullRequestCommentReportArtifactWriteResult")
            .field(
                "github_pr_comment_citation",
                &self.github_pr_comment_citation,
            )
            .field("artifact_write", &self.artifact_write)
            .finish()
    }
}

/// Bounded provider-candidate integration result for report artifact writes.
#[derive(Clone, Eq, PartialEq)]
pub enum ReportArtifactWriteProviderIntegrationResult {
    /// No provider-candidate-specific gate ran.
    None,
    /// GitHub PR comment citation validation ran before artifact write.
    GitHubPullRequestComment {
        /// Bounded GitHub PR comment citation validation result.
        citation: GitHubPullRequestCommentReportArtifactCitationResult,
    },
}

impl ReportArtifactWriteProviderIntegrationResult {
    /// Returns true when no provider-candidate-specific gate ran.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns the GitHub PR comment citation result when that provider gate
    /// ran.
    #[must_use]
    pub const fn github_pr_comment_citation(
        &self,
    ) -> Option<&GitHubPullRequestCommentReportArtifactCitationResult> {
        match self {
            Self::GitHubPullRequestComment { citation } => Some(citation),
            Self::None => None,
        }
    }
}

impl fmt::Debug for ReportArtifactWriteProviderIntegrationResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => formatter.write_str("ReportArtifactWriteProviderIntegrationResult::None"),
            Self::GitHubPullRequestComment { citation } => formatter
                .debug_struct(
                    "ReportArtifactWriteProviderIntegrationResult::GitHubPullRequestComment",
                )
                .field("citation", citation)
                .finish(),
        }
    }
}

/// Bounded result from explicit report artifact write integration.
#[derive(Clone, Eq, PartialEq)]
pub struct ReportArtifactWriteIntegrationResult {
    provider_integration: ReportArtifactWriteProviderIntegrationResult,
    artifact_write: WorkReportArtifactGovernedWriteResult,
}

impl ReportArtifactWriteIntegrationResult {
    /// Returns the provider-candidate-specific integration result.
    #[must_use]
    pub const fn provider_integration(&self) -> &ReportArtifactWriteProviderIntegrationResult {
        &self.provider_integration
    }

    /// Returns the governed artifact write result.
    #[must_use]
    pub const fn artifact_write(&self) -> &WorkReportArtifactGovernedWriteResult {
        &self.artifact_write
    }
}

impl fmt::Debug for ReportArtifactWriteIntegrationResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ReportArtifactWriteIntegrationResult")
            .field("provider_integration", &self.provider_integration)
            .field("artifact_write", &self.artifact_write)
            .finish()
    }
}

/// Internal terminal report artifact requirement model for future workflow-declared
/// report artifact requirements.
///
/// This model is not wired to workflow YAML, schemas, runtime config, CLI, or
/// automatic artifact writing. It is a bounded bridge between future authored
/// requirements and the already-reviewed explicit artifact gate policy.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkReportArtifactRequirement {
    high_assurance_approval: WorkReportArtifactHighAssuranceRequirement,
}

/// Definition used to construct a validated `WorkReportArtifactRequirement`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkReportArtifactRequirementDefinition {
    /// Required high-assurance approval disclosure posture for report artifacts.
    #[serde(default)]
    pub high_assurance_approval: WorkReportArtifactHighAssuranceRequirement,
    /// Explicit future high-assurance requirements that are not supported yet.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unsupported_high_assurance_requirements:
        Vec<WorkReportArtifactUnsupportedHighAssuranceRequirement>,
}

/// Supported high-assurance approval disclosure requirement for a terminal
/// report artifact.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkReportArtifactHighAssuranceRequirement {
    /// Do not require high-assurance approval disclosure before artifact persistence.
    #[default]
    NotRequired,
    /// Require bounded high-assurance approval disclosure to be present.
    DisclosureRequired,
    /// Require disclosure that high-assurance validation was used and passed.
    ValidatedDisclosureRequired,
    /// Require validated disclosure and fail-closed denial behavior posture.
    ValidatedFailClosedDisclosureRequired,
}

/// Unsupported future high-assurance artifact requirement vocabulary.
///
/// These variants let internal callers represent future authored intent while
/// preserving the invariant that unsupported governance is rejected rather than
/// silently accepted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkReportArtifactUnsupportedHighAssuranceRequirement {
    /// Quorum or multi-party approval is not implemented.
    QuorumApproval,
    /// Role-bound or steward-bound authority is not implemented.
    RoleBoundAuthority,
    /// Approval revocation enforcement is not implemented.
    RevocationEnforcement,
    /// External identity provider proof is not implemented.
    ExternalIdentity,
    /// Automatic artifact writing is not implemented.
    AutomaticArtifactWrite,
    /// Side-effect execution or provider mutation is not implemented.
    SideEffectExecution,
}

/// Explicit input for deriving report artifact gate policy from a loaded
/// workflow definition.
#[derive(Clone, Copy)]
pub struct WorkflowReportArtifactGateDerivationInput<'a> {
    /// Loaded workflow definition whose artifact requirement declaration should
    /// be mapped into explicit artifact gate policy.
    pub workflow: &'a WorkflowDefinition,
}

/// Derived report artifact gate policy for a workflow definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowReportArtifactGateDerivation {
    high_assurance_disclosure_policy: WorkReportArtifactHighAssuranceDisclosurePolicy,
}

impl WorkflowReportArtifactGateDerivation {
    /// Returns the derived high-assurance approval disclosure policy.
    #[must_use]
    pub const fn high_assurance_disclosure_policy(
        &self,
    ) -> WorkReportArtifactHighAssuranceDisclosurePolicy {
        self.high_assurance_disclosure_policy
    }
}

/// Derives explicit report artifact gate policy from a workflow declaration.
///
/// This helper is pure and local. It does not validate a project, generate a
/// report, write an artifact, inspect runtime state, append events, or relax
/// semantic validation for enforcement postures.
///
/// # Errors
///
/// Returns a stable error if future unsupported workflow artifact requirement
/// vocabulary reaches derivation. Current supported posture values are
/// exhaustively mapped by the canonical enum.
pub fn derive_workflow_report_artifact_gate_policy(
    input: WorkflowReportArtifactGateDerivationInput<'_>,
) -> Result<WorkflowReportArtifactGateDerivation, WorkflowOsError> {
    Ok(WorkflowReportArtifactGateDerivation {
        high_assurance_disclosure_policy: input
            .workflow
            .report_artifact_requirements
            .high_assurance_approval
            .to_high_assurance_disclosure_policy(),
    })
}

impl WorkReportArtifactRequirement {
    /// Creates a validated internal report artifact requirement.
    ///
    /// # Errors
    ///
    /// Returns a stable validation error when unsupported future requirement
    /// vocabulary is supplied or duplicated.
    pub fn new(
        definition: WorkReportArtifactRequirementDefinition,
    ) -> Result<Self, WorkflowOsError> {
        let WorkReportArtifactRequirementDefinition {
            high_assurance_approval,
            unsupported_high_assurance_requirements,
        } = definition;
        validate_unsupported_high_assurance_requirements(&unsupported_high_assurance_requirements)?;
        Ok(Self {
            high_assurance_approval,
        })
    }

    /// Returns the high-assurance approval disclosure requirement.
    #[must_use]
    pub const fn high_assurance_approval(&self) -> WorkReportArtifactHighAssuranceRequirement {
        self.high_assurance_approval
    }

    /// Maps the internal requirement to the explicit artifact gate policy.
    #[must_use]
    pub const fn high_assurance_disclosure_policy(
        &self,
    ) -> WorkReportArtifactHighAssuranceDisclosurePolicy {
        self.high_assurance_approval
            .to_high_assurance_disclosure_policy()
    }
}

impl WorkReportArtifactHighAssuranceRequirement {
    /// Maps this requirement to the explicit report artifact disclosure gate policy.
    #[must_use]
    pub const fn to_high_assurance_disclosure_policy(
        self,
    ) -> WorkReportArtifactHighAssuranceDisclosurePolicy {
        match self {
            Self::NotRequired => WorkReportArtifactHighAssuranceDisclosurePolicy::disabled(),
            Self::DisclosureRequired => {
                WorkReportArtifactHighAssuranceDisclosurePolicy::require_disclosure()
            }
            Self::ValidatedDisclosureRequired => {
                WorkReportArtifactHighAssuranceDisclosurePolicy::require_validated()
            }
            Self::ValidatedFailClosedDisclosureRequired => {
                WorkReportArtifactHighAssuranceDisclosurePolicy::require_validated_fail_closed()
            }
        }
    }
}

impl Default for WorkReportArtifactRequirementDefinition {
    fn default() -> Self {
        Self {
            high_assurance_approval: WorkReportArtifactHighAssuranceRequirement::NotRequired,
            unsupported_high_assurance_requirements: Vec::new(),
        }
    }
}

impl fmt::Debug for WorkReportArtifactRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkReportArtifactRequirement")
            .field("high_assurance_approval", &self.high_assurance_approval)
            .finish()
    }
}

impl<'de> Deserialize<'de> for WorkReportArtifactRequirement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definition = WorkReportArtifactRequirementDefinition::deserialize(deserializer)?;
        WorkReportArtifactRequirement::new(definition)
            .map_err(|_| serde::de::Error::custom("invalid work report artifact requirement"))
    }
}

/// Explicit policy for validating high-assurance approval disclosure before
/// writing a report artifact.
///
/// This policy is opt-in. A disabled policy preserves existing artifact-write
/// behavior. When enabled, the gate validates only the bounded
/// `WorkReportHighAssuranceApprovalDisclosure` carried by the report artifact;
/// it does not infer posture from workflow events, approval payloads, policy
/// strings, side-effect records, or report text.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorkReportArtifactHighAssuranceDisclosurePolicy {
    mode: WorkReportArtifactHighAssuranceDisclosureGateMode,
}

/// Explicit high-assurance disclosure artifact-gate mode.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WorkReportArtifactHighAssuranceDisclosureGateMode {
    /// Do not enforce high-assurance approval disclosure for artifact writes.
    #[default]
    Disabled,
    /// Require bounded high-assurance approval disclosure with selected posture checks.
    Required {
        /// Require the disclosure to state high-assurance validation was used.
        require_validation_used: bool,
        /// Require the disclosure to state high-assurance validation passed.
        require_validation_passed: bool,
        /// Require the disclosure to state denial behavior is fail-closed.
        require_fail_closed_denial_behavior: bool,
    },
}

impl WorkReportArtifactHighAssuranceDisclosurePolicy {
    /// Returns a disabled policy.
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            mode: WorkReportArtifactHighAssuranceDisclosureGateMode::Disabled,
        }
    }

    /// Returns a policy that requires disclosure presence only.
    #[must_use]
    pub const fn require_disclosure() -> Self {
        Self {
            mode: WorkReportArtifactHighAssuranceDisclosureGateMode::Required {
                require_validation_used: false,
                require_validation_passed: false,
                require_fail_closed_denial_behavior: false,
            },
        }
    }

    /// Returns a policy that requires used and passed validation posture.
    #[must_use]
    pub const fn require_validated() -> Self {
        Self {
            mode: WorkReportArtifactHighAssuranceDisclosureGateMode::Required {
                require_validation_used: true,
                require_validation_passed: true,
                require_fail_closed_denial_behavior: false,
            },
        }
    }

    /// Returns a policy that requires validated posture and fail-closed denial behavior.
    #[must_use]
    pub const fn require_validated_fail_closed() -> Self {
        Self {
            mode: WorkReportArtifactHighAssuranceDisclosureGateMode::Required {
                require_validation_used: true,
                require_validation_passed: true,
                require_fail_closed_denial_behavior: true,
            },
        }
    }

    /// Returns whether this policy enables any high-assurance disclosure gate.
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        matches!(
            self.mode,
            WorkReportArtifactHighAssuranceDisclosureGateMode::Required { .. }
        )
    }

    /// Returns the stricter of two high-assurance disclosure policies.
    #[must_use]
    pub const fn stricter(self, other: Self) -> Self {
        if self.strictness_rank() >= other.strictness_rank() {
            self
        } else {
            other
        }
    }

    const fn strictness_rank(self) -> u8 {
        match self.mode {
            WorkReportArtifactHighAssuranceDisclosureGateMode::Disabled => 0,
            WorkReportArtifactHighAssuranceDisclosureGateMode::Required {
                require_validation_used: false,
                require_validation_passed: false,
                require_fail_closed_denial_behavior: false,
            } => 1,
            WorkReportArtifactHighAssuranceDisclosureGateMode::Required {
                require_fail_closed_denial_behavior: false,
                ..
            } => 2,
            WorkReportArtifactHighAssuranceDisclosureGateMode::Required {
                require_fail_closed_denial_behavior: true,
                ..
            } => 3,
        }
    }

    const fn require_validation_used(self) -> bool {
        matches!(
            self.mode,
            WorkReportArtifactHighAssuranceDisclosureGateMode::Required {
                require_validation_used: true,
                ..
            }
        )
    }

    const fn require_validation_passed(self) -> bool {
        matches!(
            self.mode,
            WorkReportArtifactHighAssuranceDisclosureGateMode::Required {
                require_validation_passed: true,
                ..
            }
        )
    }

    const fn require_fail_closed_denial_behavior(self) -> bool {
        matches!(
            self.mode,
            WorkReportArtifactHighAssuranceDisclosureGateMode::Required {
                require_fail_closed_denial_behavior: true,
                ..
            }
        )
    }
}

/// Whether high-assurance disclosure was present for an artifact gate result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkReportArtifactHighAssuranceDisclosurePresence {
    /// Disclosure was present.
    Present,
}

/// Bounded result from validating high-assurance approval disclosure for a
/// report artifact.
///
/// The result exposes booleans only and intentionally does not expose report
/// IDs, approval IDs, actor IDs, references, paths, control payloads, or report
/// text.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct WorkReportArtifactHighAssuranceDisclosureGateResult {
    disclosure: WorkReportArtifactHighAssuranceDisclosurePresence,
    validation_used: bool,
    validation_passed: bool,
    fail_closed_denial_behavior: bool,
}

impl WorkReportArtifactHighAssuranceDisclosureGateResult {
    /// Returns whether disclosure was present on the report.
    #[must_use]
    pub const fn disclosure_present(&self) -> bool {
        matches!(
            self.disclosure,
            WorkReportArtifactHighAssuranceDisclosurePresence::Present
        )
    }

    /// Returns whether the disclosure states validation was used.
    #[must_use]
    pub const fn validation_used(&self) -> bool {
        self.validation_used
    }

    /// Returns whether the disclosure states validation passed.
    #[must_use]
    pub const fn validation_passed(&self) -> bool {
        self.validation_passed
    }

    /// Returns whether the disclosure states denial behavior is fail-closed.
    #[must_use]
    pub const fn fail_closed_denial_behavior(&self) -> bool {
        self.fail_closed_denial_behavior
    }
}

impl fmt::Debug for WorkReportArtifactHighAssuranceDisclosureGateResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkReportArtifactHighAssuranceDisclosureGateResult")
            .field("disclosure", &self.disclosure)
            .field("validation_used", &self.validation_used)
            .field("validation_passed", &self.validation_passed)
            .field(
                "fail_closed_denial_behavior",
                &self.fail_closed_denial_behavior,
            )
            .finish()
    }
}

/// Explicit governed artifact write input for a validated terminal `WorkReport`.
///
/// This input composes existing report artifact, `SideEffect` referential
/// integrity, and approval-linkage gates. It does not generate reports, discover
/// side effects, append workflow events, mutate workflow state, call providers,
/// execute side effects, or expose CLI behavior.
#[derive(Clone, Copy)]
pub struct WorkReportArtifactGovernedWriteInput<'a> {
    /// Terminal workflow run whose identity is authoritative for the artifact.
    pub run: &'a WorkflowRun,
    /// Validated report artifact to write after gates pass.
    pub artifact: &'a WorkReportArtifactRecord,
    /// Whether every cited `SideEffect` ID must resolve to a stored record.
    pub require_all_side_effect_citations: bool,
    /// Whether `RequiresApproval` side-effect records must cite an approval request.
    pub require_approval_references_for_requires_approval: bool,
    /// Whether approved/denied side-effect records require matching approval decisions.
    pub require_decision_for_approved_or_denied: bool,
    /// Optional high-assurance approval disclosure gate policy.
    pub high_assurance_disclosure_policy: WorkReportArtifactHighAssuranceDisclosurePolicy,
}

impl fmt::Debug for WorkReportArtifactGovernedWriteInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkReportArtifactGovernedWriteInput")
            .field("run", &"[REDACTED]")
            .field("artifact", &"[REDACTED]")
            .field(
                "require_all_side_effect_citations",
                &self.require_all_side_effect_citations,
            )
            .field(
                "require_approval_references_for_requires_approval",
                &self.require_approval_references_for_requires_approval,
            )
            .field(
                "require_decision_for_approved_or_denied",
                &self.require_decision_for_approved_or_denied,
            )
            .field(
                "high_assurance_disclosure_policy",
                &self.high_assurance_disclosure_policy,
            )
            .finish()
    }
}

/// Bounded result from a governed work report artifact write.
///
/// The result intentionally exposes counts and validation summaries only. It
/// does not expose report text, run IDs, side-effect IDs, approval IDs, target
/// references, payloads, paths, or provider data.
#[derive(Clone, Eq, PartialEq)]
pub struct WorkReportArtifactGovernedWriteResult {
    side_effect_integrity: WorkReportArtifactSideEffectIntegrityResult,
    approval_linkage: Option<SideEffectApprovalLinkageFromStoreResult>,
    high_assurance_disclosure: Option<WorkReportArtifactHighAssuranceDisclosureGateResult>,
}

impl WorkReportArtifactGovernedWriteResult {
    /// Returns the side-effect referential integrity result.
    #[must_use]
    pub const fn side_effect_integrity(&self) -> &WorkReportArtifactSideEffectIntegrityResult {
        &self.side_effect_integrity
    }

    /// Returns the approval-linkage result when side-effect citations were present.
    #[must_use]
    pub const fn approval_linkage(&self) -> Option<&SideEffectApprovalLinkageFromStoreResult> {
        self.approval_linkage.as_ref()
    }

    /// Returns high-assurance disclosure gate posture when that gate ran.
    #[must_use]
    pub const fn high_assurance_disclosure(
        &self,
    ) -> Option<&WorkReportArtifactHighAssuranceDisclosureGateResult> {
        self.high_assurance_disclosure.as_ref()
    }
}

impl fmt::Debug for WorkReportArtifactGovernedWriteResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkReportArtifactGovernedWriteResult")
            .field("side_effect_integrity", &self.side_effect_integrity)
            .field("has_approval_linkage", &self.approval_linkage.is_some())
            .field(
                "has_high_assurance_disclosure",
                &self.high_assurance_disclosure.is_some(),
            )
            .finish()
    }
}

/// In-memory runtime result exposure for a terminal local work report.
///
/// This result owns the already-terminal workflow run and the generated report.
/// It is not persisted, serialized as a public schema, written as an artifact,
/// or emitted by CLI.
#[derive(Clone, Eq, PartialEq)]
pub struct TerminalLocalWorkReportResult {
    run: WorkflowRun,
    work_report: WorkReport,
}

impl TerminalLocalWorkReportResult {
    /// Creates an in-memory terminal report result.
    #[must_use]
    pub const fn new(run: WorkflowRun, work_report: WorkReport) -> Self {
        Self { run, work_report }
    }

    /// Returns the terminal workflow run.
    #[must_use]
    pub const fn run(&self) -> &WorkflowRun {
        &self.run
    }

    /// Returns the generated work report.
    #[must_use]
    pub const fn work_report(&self) -> &WorkReport {
        &self.work_report
    }

    /// Consumes the result into owned parts.
    #[must_use]
    pub fn into_parts(self) -> (WorkflowRun, WorkReport) {
        (self.run, self.work_report)
    }
}

impl fmt::Debug for TerminalLocalWorkReportResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TerminalLocalWorkReportResult")
            .field("run_status", &self.run.snapshot.status)
            .field("run_event_count", &self.run.events.len())
            .field("work_report", &"[REDACTED]")
            .finish()
    }
}

/// Generates an in-memory terminal local `WorkReport` from explicit inputs.
///
/// # Errors
///
/// Returns a stable validation error when the run is not a supported terminal
/// status, or when any report, section, citation, disclosure, limitation, risk,
/// handoff-note, redaction, or sensitivity field fails existing `WorkReport`
/// validation.
pub fn generate_terminal_local_work_report(
    input: TerminalLocalWorkReportInput<'_>,
) -> Result<WorkReport, WorkflowOsError> {
    let terminal_status = work_report_status_from_runtime(input.run.snapshot.status)?;
    let identity = &input.run.snapshot.identity;
    let sensitivity = input.sensitivity;
    let redaction = input.redaction.clone();
    let citations = terminal_report_citations(&input, sensitivity, &redaction)?;
    let sections = terminal_report_sections(terminal_status, &citations, &input)?;
    let high_assurance_approval = input.high_assurance_approval.clone();
    let known_limitations = known_limitations_with_high_assurance(
        input.known_limitations,
        high_assurance_approval.as_ref(),
    );

    WorkReport::new(WorkReportDefinition {
        report_id: input.report_id,
        report_contract_id: input.report_contract_id,
        report_contract_version: input.report_contract_version,
        generation_context: WorkReportGenerationContext {
            workflow_id: identity.workflow_id.clone(),
            workflow_version: identity.workflow_version.clone(),
            schema_version: identity.schema_version.clone(),
            spec_hash: identity.spec_content_hash.clone(),
            run_id: identity.run_id.clone(),
            terminal_run_status: terminal_status,
            generated_at: input.generated_at,
            generated_by: input.generated_by,
            correlation_id: input.correlation_id,
        },
        sections,
        incomplete_work: report_notes_or_default(
            input.incomplete_work,
            "No incomplete or deferred work was supplied.",
            WorkReportIncompleteWorkDisclosure::new,
        )?,
        known_limitations: report_notes_or_default(
            known_limitations,
            "No known limitations were supplied.",
            WorkReportKnownLimitation::new,
        )?,
        risks: report_notes_or_default(
            input.risks,
            "No additional risks were supplied.",
            WorkReportRisk::new,
        )?,
        handoff_notes: report_notes_or_default(
            input.handoff_notes,
            "No operator handoff notes were supplied.",
            WorkReportHandoffNote::new,
        )?,
        high_assurance_approval,
        sensitivity,
        redaction,
    })
}

/// Generates a terminal local `WorkReport` after explicit `SideEffect` discovery.
///
/// Discovery is bounded to the already-terminal run supplied in `input`, the
/// explicit `side_effect_ids` already present on that input, and the supplied
/// `SideEffectRecordStore` when store discovery is requested. The helper then
/// delegates to `generate_terminal_local_work_report`, so report construction
/// and redaction validation remain centralized.
///
/// # Errors
///
/// Returns a stable, non-leaking validation error when no discovery source is
/// enabled. Returns the underlying `SideEffect` discovery or `WorkReport`
/// validation error when discovery or report construction fails.
pub fn generate_terminal_local_work_report_with_side_effect_discovery(
    store: &impl SideEffectRecordStore,
    mut input: TerminalLocalWorkReportInput<'_>,
    discovery: TerminalLocalWorkReportSideEffectDiscoveryInput,
) -> Result<WorkReport, WorkflowOsError> {
    if !discovery.include_workflow_events && !discovery.include_store_records {
        return Err(WorkflowOsError::validation(
            "work_report_generation.side_effect_discovery.source_required",
            "side-effect discovery requires an explicit discovery source",
        ));
    }

    let identity = &input.run.snapshot.identity;
    let workflow_events = if discovery.include_workflow_events {
        input.run.events.clone()
    } else {
        Vec::new()
    };

    let discovery_result = if discovery.include_store_records {
        discover_side_effect_references_from_store(
            store,
            &SideEffectStoreBackedDiscoveryInput {
                workflow_id: identity.workflow_id.clone(),
                workflow_version: identity.workflow_version.clone(),
                schema_version: identity.schema_version.clone(),
                spec_hash: identity.spec_content_hash.clone(),
                run_id: identity.run_id.clone(),
                explicit_side_effect_ids: input.side_effect_ids.clone(),
                workflow_events,
                require_records: discovery.require_records,
            },
        )?
    } else {
        discover_side_effect_references(&SideEffectDiscoveryInput {
            workflow_id: identity.workflow_id.clone(),
            workflow_version: identity.workflow_version.clone(),
            schema_version: identity.schema_version.clone(),
            spec_hash: identity.spec_content_hash.clone(),
            run_id: identity.run_id.clone(),
            explicit_side_effect_ids: input.side_effect_ids.clone(),
            workflow_events,
            side_effect_records: Vec::new(),
            require_records: discovery.require_records,
        })?
    };

    input.side_effect_ids = discovery_result
        .references()
        .iter()
        .map(|reference| reference.side_effect_id().clone())
        .collect();

    generate_terminal_local_work_report(input)
}

/// Exposes an in-memory terminal local report alongside its workflow run.
///
/// This additive helper preserves existing executor return types. It clones the
/// already-terminal run supplied by the caller, generates a validated report
/// through `generate_terminal_local_work_report`, and returns both values in
/// memory. It does not mutate runtime state, append events, persist reports,
/// write files, or emit CLI output.
///
/// # Errors
///
/// Returns the same structured, non-leaking errors as terminal local report
/// generation. On error, no result wrapper is returned and the borrowed run is
/// not modified.
pub fn expose_terminal_local_work_report_result(
    input: TerminalLocalWorkReportInput<'_>,
) -> Result<TerminalLocalWorkReportResult, WorkflowOsError> {
    let run = input.run.clone();
    let work_report = generate_terminal_local_work_report(input)?;
    Ok(TerminalLocalWorkReportResult::new(run, work_report))
}

/// Validates `SideEffect` citations in a work report artifact against an explicit store.
///
/// This helper is reference-only and in-memory. It validates already-cited
/// `SideEffect` IDs against the caller-supplied `SideEffectRecordStore` and the
/// report artifact's immutable run identity. It does not write artifacts,
/// discover side effects, create or repair side-effect records, mutate workflow
/// state, append events, call providers, or execute side effects.
///
/// # Errors
///
/// Returns a stable, non-leaking error when the artifact is invalid, a required
/// cited record is missing, a resolved record does not match the report's
/// immutable run identity, or the supplied store cannot read/validate a record.
pub fn validate_work_report_artifact_side_effect_integrity(
    store: &impl SideEffectRecordStore,
    input: WorkReportArtifactSideEffectIntegrityInput<'_>,
) -> Result<WorkReportArtifactSideEffectIntegrityResult, WorkflowOsError> {
    input
        .artifact
        .validate()
        .map_err(|_| side_effect_integrity_error(SIDE_EFFECT_INTEGRITY_INVALID_ARTIFACT))?;

    let (side_effect_ids, duplicate_count) =
        collect_artifact_side_effect_citations(input.artifact.work_report());
    let mut resolved_count = 0usize;
    let mut missing_count = 0usize;
    let context = input.artifact.work_report().generation_context();

    for side_effect_id in &side_effect_ids {
        let record = store
            .read_side_effect_record(side_effect_id)
            .map_err(|error| map_side_effect_integrity_store_error(&error))?;
        let Some(record) = record else {
            missing_count += 1;
            if input.require_all_side_effect_citations {
                return Err(side_effect_integrity_error(
                    SIDE_EFFECT_INTEGRITY_RECORD_MISSING,
                ));
            }
            continue;
        };

        validate_artifact_side_effect_record_identity(context, &record)?;
        resolved_count += 1;
    }

    Ok(WorkReportArtifactSideEffectIntegrityResult {
        cited: side_effect_ids.len(),
        resolved: resolved_count,
        missing: missing_count,
        duplicate_citations: duplicate_count,
    })
}

/// Validates that a work report artifact cites the expected proposed GitHub PR
/// comment `SideEffect` record and, when requested, an accepted
/// `SideEffectProposed` workflow event.
///
/// This helper is validation-only and reference-only. It does not write report
/// artifacts, append events, mutate workflow state, discover records, call
/// providers, or execute side effects.
///
/// # Errors
///
/// Returns stable, non-leaking errors when the expected `SideEffect` citation is
/// missing, the resolved record is not a proposed GitHub PR comment write, the
/// artifact/record identity does not match, or the required accepted workflow
/// event is absent or mismatched.
pub fn validate_github_pr_comment_report_artifact_citations(
    store: &impl SideEffectRecordStore,
    input: GitHubPullRequestCommentReportArtifactCitationInput<'_>,
) -> Result<GitHubPullRequestCommentReportArtifactCitationResult, WorkflowOsError> {
    input
        .artifact
        .validate()
        .map_err(|_| github_pr_comment_report_artifact_citation_error("invalid_artifact"))?;

    let (side_effect_ids, _) = collect_artifact_side_effect_citations(input.artifact.work_report());
    if !side_effect_ids
        .iter()
        .any(|side_effect_id| side_effect_id == input.side_effect_id)
    {
        return Err(github_pr_comment_report_artifact_citation_error(
            "side_effect_missing",
        ));
    }

    let side_effect_integrity = validate_work_report_artifact_side_effect_integrity(
        store,
        WorkReportArtifactSideEffectIntegrityInput {
            artifact: input.artifact,
            require_all_side_effect_citations: input.require_record,
        },
    )
    .map_err(|error| map_github_pr_comment_report_artifact_integrity_error(&error))?;

    let record = store
        .read_side_effect_record(input.side_effect_id)
        .map_err(|error| map_github_pr_comment_report_artifact_store_error(&error))?;
    let record_validated = match record {
        Some(record) => {
            validate_github_pr_comment_report_artifact_record(
                input.artifact.work_report().generation_context(),
                &record,
            )?;
            true
        }
        None if input.require_record => {
            return Err(github_pr_comment_report_artifact_citation_error(
                "record_missing",
            ));
        }
        None => false,
    };

    let accepted_event_count = matching_github_pr_comment_proposed_event_count(input)?;
    if input.require_accepted_event && accepted_event_count == 0 {
        return Err(github_pr_comment_report_artifact_citation_error(
            "event_missing",
        ));
    }

    Ok(GitHubPullRequestCommentReportArtifactCitationResult {
        side_effect_integrity,
        record_validated,
        accepted_event_count,
    })
}

/// Validates and writes a work report artifact after side-effect integrity and
/// approval-linkage gates pass.
///
/// This helper is an explicit local composition boundary. It writes only
/// through the supplied `WorkReportArtifactStore` after validating the artifact,
/// matching it to the supplied terminal run, validating cited `SideEffect`
/// records against the supplied `SideEffectRecordStore`, and validating
/// approval linkage for cited `SideEffect` records. It does not mutate workflow
/// state, append events, emit audit or observability records, call adapters,
/// execute side effects, create side-effect records, or repair citations.
///
/// # Errors
///
/// Returns stable, non-leaking errors when artifact/run identity mismatches,
/// side-effect integrity fails, approval linkage fails, or the artifact store
/// rejects the write.
pub fn write_work_report_artifact_with_side_effect_integrity_and_approval_linkage(
    artifact_store: &impl WorkReportArtifactStore,
    side_effect_store: &impl SideEffectRecordStore,
    input: WorkReportArtifactGovernedWriteInput<'_>,
) -> Result<WorkReportArtifactGovernedWriteResult, WorkflowOsError> {
    input
        .artifact
        .validate()
        .map_err(|_| governed_artifact_write_error("invalid_artifact"))?;
    validate_artifact_matches_run(input.run, input.artifact)?;

    let side_effect_integrity = validate_work_report_artifact_side_effect_integrity(
        side_effect_store,
        WorkReportArtifactSideEffectIntegrityInput {
            artifact: input.artifact,
            require_all_side_effect_citations: input.require_all_side_effect_citations,
        },
    )?;

    let (side_effect_ids, _) = collect_artifact_side_effect_citations(input.artifact.work_report());
    let approval_linkage = if side_effect_ids.is_empty() {
        None
    } else {
        Some(validate_side_effect_approval_linkage_from_store(
            side_effect_store,
            SideEffectApprovalLinkageFromStoreInput {
                run: input.run,
                side_effect_ids: &side_effect_ids,
                load_mode: SideEffectApprovalLinkageStoreLoadMode::ExplicitIds,
                missing_record_policy: if input.require_all_side_effect_citations {
                    SideEffectMissingRecordPolicy::RequireAll
                } else {
                    SideEffectMissingRecordPolicy::CountMissing
                },
                require_approval_references_for_requires_approval: input
                    .require_approval_references_for_requires_approval,
                require_decision_for_approved_or_denied: input
                    .require_decision_for_approved_or_denied,
            },
        )?)
    };

    let high_assurance_disclosure = validate_work_report_artifact_high_assurance_disclosure(
        input.artifact,
        input.high_assurance_disclosure_policy,
    )?;

    artifact_store.write_work_report_artifact(input.artifact)?;

    Ok(WorkReportArtifactGovernedWriteResult {
        side_effect_integrity,
        approval_linkage,
        high_assurance_disclosure,
    })
}

/// Writes a report artifact after validating that it cites the expected
/// proposed GitHub PR comment `SideEffect`.
///
/// This helper is an explicit local composition boundary. It validates the
/// GitHub PR comment citation first, then delegates to the existing governed
/// artifact write helper for generic `SideEffect` integrity, approval linkage,
/// high-assurance disclosure, and store-backed artifact persistence. It does
/// not append events, mutate workflow state, emit audit or observability
/// records, call providers, execute side effects, create side-effect records,
/// or repair citations.
///
/// # Errors
///
/// Returns stable, non-leaking errors when GitHub PR comment citation
/// validation fails, artifact/run identity mismatches, approval linkage fails,
/// or the artifact store rejects the write.
pub fn write_github_pr_comment_report_artifact_with_citations(
    artifact_store: &impl WorkReportArtifactStore,
    side_effect_store: &impl SideEffectRecordStore,
    input: GitHubPullRequestCommentReportArtifactWriteInput<'_>,
) -> Result<GitHubPullRequestCommentReportArtifactWriteResult, WorkflowOsError> {
    let github_pr_comment_citation = validate_github_pr_comment_report_artifact_citations(
        side_effect_store,
        GitHubPullRequestCommentReportArtifactCitationInput {
            artifact: input.governed_write.artifact,
            side_effect_id: input.side_effect_id,
            workflow_events: input.workflow_events,
            require_record: input.citation_policy.require_record,
            require_accepted_event: input.citation_policy.require_accepted_event,
        },
    )
    .map_err(|_| github_pr_comment_report_artifact_write_error("citation_invalid"))?;

    let artifact_write =
        write_work_report_artifact_with_side_effect_integrity_and_approval_linkage(
            artifact_store,
            side_effect_store,
            input.governed_write,
        )
        .map_err(|error| map_github_pr_comment_report_artifact_write_error(&error))?;

    Ok(GitHubPullRequestCommentReportArtifactWriteResult {
        github_pr_comment_citation,
        artifact_write,
    })
}

/// Writes a GitHub PR comment report artifact from explicit local run/report
/// context.
///
/// This helper is executor-adjacent but not executor-integrated. It composes the
/// existing GitHub PR comment citation, generic `SideEffect` referential
/// integrity, approval-linkage, high-assurance disclosure, and artifact-store
/// gates without reading hidden runtime state or performing provider writes.
///
/// # Errors
///
/// Returns stable, non-leaking errors when GitHub PR comment citation
/// validation fails, artifact/run identity mismatches, approval linkage fails,
/// high-assurance disclosure is missing, or the artifact store rejects the
/// write.
pub fn write_github_pr_comment_report_artifact_from_explicit_context(
    artifact_store: &impl WorkReportArtifactStore,
    side_effect_store: &impl SideEffectRecordStore,
    input: GitHubPullRequestCommentReportArtifactIntegrationInput<'_>,
) -> Result<GitHubPullRequestCommentReportArtifactWriteResult, WorkflowOsError> {
    write_github_pr_comment_report_artifact_with_citations(
        artifact_store,
        side_effect_store,
        GitHubPullRequestCommentReportArtifactWriteInput {
            governed_write: WorkReportArtifactGovernedWriteInput {
                run: input.run,
                artifact: input.artifact,
                require_all_side_effect_citations: input.require_all_side_effect_citations,
                require_approval_references_for_requires_approval: input
                    .require_approval_references_for_requires_approval,
                require_decision_for_approved_or_denied: input
                    .require_decision_for_approved_or_denied,
                high_assurance_disclosure_policy: input.high_assurance_disclosure_policy,
            },
            side_effect_id: input.side_effect_id,
            workflow_events: input.workflow_events,
            citation_policy: input.citation_policy,
        },
    )
}

/// Writes a report artifact from explicit local context after composing generic
/// artifact gates and optional provider-candidate integration gates.
///
/// This helper is local and explicit. It does not run workflows, generate
/// reports, discover side effects, append events, call providers, execute side
/// effects, mutate workflow state, expose CLI behavior, or make artifact
/// writing automatic.
///
/// # Errors
///
/// Returns stable, non-leaking errors when provider-candidate citation
/// validation fails, artifact/run identity mismatches, side-effect integrity
/// fails, approval linkage fails, high-assurance disclosure is missing, or the
/// artifact store rejects the write.
pub fn write_report_artifact_with_explicit_integrations(
    artifact_store: &impl WorkReportArtifactStore,
    side_effect_store: &impl SideEffectRecordStore,
    input: ReportArtifactWriteIntegrationInput<'_>,
) -> Result<ReportArtifactWriteIntegrationResult, WorkflowOsError> {
    match input.provider_integration {
        ReportArtifactWriteProviderIntegration::None => {
            let artifact_write =
                write_work_report_artifact_with_side_effect_integrity_and_approval_linkage(
                    artifact_store,
                    side_effect_store,
                    WorkReportArtifactGovernedWriteInput {
                        run: input.run,
                        artifact: input.artifact,
                        require_all_side_effect_citations: input.require_all_side_effect_citations,
                        require_approval_references_for_requires_approval: input
                            .require_approval_references_for_requires_approval,
                        require_decision_for_approved_or_denied: input
                            .require_decision_for_approved_or_denied,
                        high_assurance_disclosure_policy: input.high_assurance_disclosure_policy,
                    },
                )?;

            Ok(ReportArtifactWriteIntegrationResult {
                provider_integration: ReportArtifactWriteProviderIntegrationResult::None,
                artifact_write,
            })
        }
        ReportArtifactWriteProviderIntegration::GitHubPullRequestComment {
            side_effect_id,
            workflow_events,
            citation_policy,
        } => {
            let result = write_github_pr_comment_report_artifact_from_explicit_context(
                artifact_store,
                side_effect_store,
                GitHubPullRequestCommentReportArtifactIntegrationInput {
                    run: input.run,
                    artifact: input.artifact,
                    side_effect_id,
                    workflow_events,
                    require_all_side_effect_citations: input.require_all_side_effect_citations,
                    require_approval_references_for_requires_approval: input
                        .require_approval_references_for_requires_approval,
                    require_decision_for_approved_or_denied: input
                        .require_decision_for_approved_or_denied,
                    high_assurance_disclosure_policy: input.high_assurance_disclosure_policy,
                    citation_policy,
                },
            )?;

            Ok(ReportArtifactWriteIntegrationResult {
                provider_integration:
                    ReportArtifactWriteProviderIntegrationResult::GitHubPullRequestComment {
                        citation: *result.github_pr_comment_citation(),
                    },
                artifact_write: result.artifact_write().clone(),
            })
        }
    }
}

fn validate_work_report_artifact_high_assurance_disclosure(
    artifact: &WorkReportArtifactRecord,
    policy: WorkReportArtifactHighAssuranceDisclosurePolicy,
) -> Result<Option<WorkReportArtifactHighAssuranceDisclosureGateResult>, WorkflowOsError> {
    if !policy.is_enabled() {
        return Ok(None);
    }

    let Some(disclosure) = artifact.work_report().high_assurance_approval() else {
        return Err(high_assurance_disclosure_gate_error("missing"));
    };

    disclosure
        .validate()
        .map_err(|_| high_assurance_disclosure_gate_error("invalid"))?;

    if policy.require_validation_used() && !disclosure.validation_used() {
        return Err(high_assurance_disclosure_gate_error("validation_not_used"));
    }
    if policy.require_validation_passed() && !disclosure.validation_passed() {
        return Err(high_assurance_disclosure_gate_error(
            "validation_not_passed",
        ));
    }
    if policy.require_fail_closed_denial_behavior() && !disclosure.denial_fail_closed() {
        return Err(high_assurance_disclosure_gate_error(
            "denial_not_fail_closed",
        ));
    }

    Ok(Some(WorkReportArtifactHighAssuranceDisclosureGateResult {
        disclosure: WorkReportArtifactHighAssuranceDisclosurePresence::Present,
        validation_used: disclosure.validation_used(),
        validation_passed: disclosure.validation_passed(),
        fail_closed_denial_behavior: disclosure.denial_fail_closed(),
    }))
}

fn validate_unsupported_high_assurance_requirements(
    unsupported: &[WorkReportArtifactUnsupportedHighAssuranceRequirement],
) -> Result<(), WorkflowOsError> {
    let mut seen = BTreeSet::new();
    for requirement in unsupported {
        if !seen.insert(*requirement) {
            return Err(WorkflowOsError::validation(
                "work_report_artifact_requirement.high_assurance.duplicate_unsupported",
                "duplicate unsupported high-assurance artifact requirement",
            ));
        }
    }

    if !unsupported.is_empty() {
        return Err(WorkflowOsError::validation(
            "work_report_artifact_requirement.high_assurance.unsupported",
            "unsupported high-assurance artifact requirement",
        ));
    }

    Ok(())
}

fn validate_artifact_matches_run(
    run: &WorkflowRun,
    artifact: &WorkReportArtifactRecord,
) -> Result<(), WorkflowOsError> {
    let identity = &run.snapshot.identity;
    let context = artifact.work_report().generation_context();
    if context.workflow_id != identity.workflow_id
        || context.workflow_version != identity.workflow_version
        || context.schema_version != identity.schema_version
        || context.spec_hash != identity.spec_content_hash
        || context.run_id != identity.run_id
        || context.terminal_run_status != work_report_status_from_runtime(run.snapshot.status)?
    {
        return Err(governed_artifact_write_error("identity_mismatch"));
    }
    Ok(())
}

const SIDE_EFFECT_INTEGRITY_RECORD_MISSING: &str =
    "work_report_artifact.side_effect_integrity.record_missing";
const SIDE_EFFECT_INTEGRITY_IDENTITY_MISMATCH: &str =
    "work_report_artifact.side_effect_integrity.identity_mismatch";
const SIDE_EFFECT_INTEGRITY_RECORD_CORRUPT: &str =
    "work_report_artifact.side_effect_integrity.record_corrupt";
const SIDE_EFFECT_INTEGRITY_STORE_READ_FAILED: &str =
    "work_report_artifact.side_effect_integrity.store_read_failed";
const SIDE_EFFECT_INTEGRITY_INVALID_ARTIFACT: &str =
    "work_report_artifact.side_effect_integrity.invalid_artifact";

fn collect_artifact_side_effect_citations(report: &WorkReport) -> (Vec<SideEffectId>, usize) {
    let mut ids = BTreeSet::new();
    let mut total_count = 0usize;

    for section in report.sections() {
        collect_side_effect_citations(section.citations(), &mut ids, &mut total_count);
    }
    for disclosure in report.incomplete_work() {
        collect_side_effect_citations(disclosure.citations(), &mut ids, &mut total_count);
    }
    for limitation in report.known_limitations() {
        collect_side_effect_citations(limitation.citations(), &mut ids, &mut total_count);
    }
    for risk in report.risks() {
        collect_side_effect_citations(risk.citations(), &mut ids, &mut total_count);
    }
    for note in report.handoff_notes() {
        collect_side_effect_citations(note.citations(), &mut ids, &mut total_count);
    }

    let unique_count = ids.len();
    (
        ids.into_iter().collect(),
        total_count.saturating_sub(unique_count),
    )
}

fn collect_side_effect_citations(
    citations: &[WorkReportCitation],
    ids: &mut BTreeSet<SideEffectId>,
    total_count: &mut usize,
) {
    for citation in citations {
        if let WorkReportCitationTarget::SideEffect { side_effect_id } = citation.target() {
            *total_count = total_count.saturating_add(1);
            ids.insert(side_effect_id.clone());
        }
    }
}

fn validate_artifact_side_effect_record_identity(
    context: &WorkReportGenerationContext,
    record: &SideEffectRecord,
) -> Result<(), WorkflowOsError> {
    record
        .validate()
        .map_err(|_| side_effect_integrity_error(SIDE_EFFECT_INTEGRITY_RECORD_CORRUPT))?;
    if record.workflow_id() != &context.workflow_id
        || record.workflow_version() != &context.workflow_version
        || record.schema_version() != &context.schema_version
        || record.spec_hash() != &context.spec_hash
        || record.run_id() != &context.run_id
    {
        return Err(side_effect_integrity_error(
            SIDE_EFFECT_INTEGRITY_IDENTITY_MISMATCH,
        ));
    }
    Ok(())
}

fn map_side_effect_integrity_store_error(error: &WorkflowOsError) -> WorkflowOsError {
    match error.code() {
        "side_effect_record.read.corrupt" => {
            side_effect_integrity_error(SIDE_EFFECT_INTEGRITY_RECORD_CORRUPT)
        }
        "side_effect_record.read.identity_mismatch" => {
            side_effect_integrity_error(SIDE_EFFECT_INTEGRITY_IDENTITY_MISMATCH)
        }
        _ => side_effect_integrity_error(SIDE_EFFECT_INTEGRITY_STORE_READ_FAILED),
    }
}

fn side_effect_integrity_error(code: &'static str) -> WorkflowOsError {
    let message = match code {
        SIDE_EFFECT_INTEGRITY_RECORD_MISSING => {
            "required side-effect citation does not resolve to a stored record"
        }
        SIDE_EFFECT_INTEGRITY_IDENTITY_MISMATCH => {
            "side-effect citation does not match artifact immutable run identity"
        }
        SIDE_EFFECT_INTEGRITY_RECORD_CORRUPT => {
            "side-effect citation record could not be read or validated"
        }
        SIDE_EFFECT_INTEGRITY_STORE_READ_FAILED => "side-effect citation store read failed",
        SIDE_EFFECT_INTEGRITY_INVALID_ARTIFACT => {
            "work report artifact could not be validated before side-effect integrity check"
        }
        _ => "work report artifact side-effect integrity check failed",
    };
    WorkflowOsError::invalid_state(code, message)
}

fn validate_github_pr_comment_report_artifact_record(
    context: &WorkReportGenerationContext,
    record: &SideEffectRecord,
) -> Result<(), WorkflowOsError> {
    validate_artifact_side_effect_record_identity(context, record)
        .map_err(|error| map_github_pr_comment_report_artifact_integrity_error(&error))?;
    if record.lifecycle_state() != SideEffectLifecycleState::Proposed {
        return Err(github_pr_comment_report_artifact_citation_error(
            "record_invalid",
        ));
    }
    if record.capability() != SideEffectCapability::GitHubWrite {
        return Err(github_pr_comment_report_artifact_citation_error(
            "record_invalid",
        ));
    }
    if record.target().kind() != SideEffectTargetKind::AdapterResource
        || !record.target().reference().starts_with("github/")
        || !record.target().reference().contains("/pull/")
    {
        return Err(github_pr_comment_report_artifact_citation_error(
            "record_invalid",
        ));
    }
    if record.outcome_reference().is_some() {
        return Err(github_pr_comment_report_artifact_citation_error(
            "record_invalid",
        ));
    }

    Ok(())
}

fn matching_github_pr_comment_proposed_event_count(
    input: GitHubPullRequestCommentReportArtifactCitationInput<'_>,
) -> Result<usize, WorkflowOsError> {
    let Some(events) = input.workflow_events else {
        return Ok(0);
    };
    let context = input.artifact.work_report().generation_context();
    let mut count = 0usize;

    for event in events {
        if event.workflow_id != context.workflow_id
            || event.workflow_version != context.workflow_version
            || event.schema_version != context.schema_version
            || event.spec_content_hash != context.spec_hash
            || event.run_id != context.run_id
        {
            return Err(github_pr_comment_report_artifact_citation_error(
                "event_mismatch",
            ));
        }
        if let WorkflowRunEventKind::SideEffectProposed(payload) = &event.kind {
            if payload.side_effect_id() == input.side_effect_id {
                if payload.lifecycle_state() != SideEffectLifecycleState::Proposed {
                    return Err(github_pr_comment_report_artifact_citation_error(
                        "event_mismatch",
                    ));
                }
                count = count.saturating_add(1);
            }
        }
    }

    Ok(count)
}

fn map_github_pr_comment_report_artifact_integrity_error(
    error: &WorkflowOsError,
) -> WorkflowOsError {
    match error.code() {
        SIDE_EFFECT_INTEGRITY_RECORD_MISSING => {
            github_pr_comment_report_artifact_citation_error("record_missing")
        }
        SIDE_EFFECT_INTEGRITY_IDENTITY_MISMATCH => {
            github_pr_comment_report_artifact_citation_error("identity_mismatch")
        }
        SIDE_EFFECT_INTEGRITY_RECORD_CORRUPT => {
            github_pr_comment_report_artifact_citation_error("record_invalid")
        }
        SIDE_EFFECT_INTEGRITY_STORE_READ_FAILED => {
            github_pr_comment_report_artifact_citation_error("integrity_failed")
        }
        SIDE_EFFECT_INTEGRITY_INVALID_ARTIFACT => {
            github_pr_comment_report_artifact_citation_error("invalid_artifact")
        }
        _ => github_pr_comment_report_artifact_citation_error("integrity_failed"),
    }
}

fn map_github_pr_comment_report_artifact_store_error(error: &WorkflowOsError) -> WorkflowOsError {
    match error.code() {
        "side_effect_record.read.corrupt" => {
            github_pr_comment_report_artifact_citation_error("record_invalid")
        }
        "side_effect_record.read.identity_mismatch" => {
            github_pr_comment_report_artifact_citation_error("identity_mismatch")
        }
        _ => github_pr_comment_report_artifact_citation_error("integrity_failed"),
    }
}

fn github_pr_comment_report_artifact_citation_error(reason: &'static str) -> WorkflowOsError {
    let code = match reason {
        "side_effect_missing" => "github_pr_comment_report_artifact_citation.side_effect_missing",
        "record_missing" => "github_pr_comment_report_artifact_citation.record_missing",
        "record_invalid" => "github_pr_comment_report_artifact_citation.record_invalid",
        "identity_mismatch" => "github_pr_comment_report_artifact_citation.identity_mismatch",
        "event_missing" => "github_pr_comment_report_artifact_citation.event_missing",
        "event_mismatch" => "github_pr_comment_report_artifact_citation.event_mismatch",
        "invalid_artifact" => "github_pr_comment_report_artifact_citation.invalid_artifact",
        _ => "github_pr_comment_report_artifact_citation.integrity_failed",
    };
    let message = match reason {
        "side_effect_missing" => "GitHub PR comment report artifact citation is missing",
        "record_missing" => "GitHub PR comment side-effect record is missing",
        "record_invalid" => "GitHub PR comment side-effect record is invalid",
        "identity_mismatch" => {
            "GitHub PR comment citation does not match artifact immutable run identity"
        }
        "event_missing" => "GitHub PR comment accepted workflow event is missing",
        "event_mismatch" => "GitHub PR comment workflow event does not match the artifact",
        "invalid_artifact" => "work report artifact could not be validated",
        _ => "GitHub PR comment report artifact citation integrity check failed",
    };
    WorkflowOsError::invalid_state(code, message)
}

fn map_github_pr_comment_report_artifact_write_error(error: &WorkflowOsError) -> WorkflowOsError {
    match error.code() {
        "work_report_artifact.governed_write.invalid_artifact"
        | "work_report_artifact.high_assurance_disclosure.missing"
        | "work_report_artifact.high_assurance_disclosure.invalid"
        | "work_report_artifact.high_assurance_disclosure.validation_not_used"
        | "work_report_artifact.high_assurance_disclosure.validation_not_passed"
        | "work_report_artifact.high_assurance_disclosure.denial_not_fail_closed" => {
            github_pr_comment_report_artifact_write_error("invalid_artifact")
        }
        "work_report_artifact.governed_write.identity_mismatch" => {
            github_pr_comment_report_artifact_write_error("identity_mismatch")
        }
        code if code.starts_with("side_effect_approval_linkage.") => {
            github_pr_comment_report_artifact_write_error("approval_linkage_invalid")
        }
        code if code.starts_with("work_report_artifact.side_effect_integrity.") => {
            github_pr_comment_report_artifact_write_error("citation_invalid")
        }
        _ => github_pr_comment_report_artifact_write_error("artifact_write_failed"),
    }
}

fn github_pr_comment_report_artifact_write_error(reason: &'static str) -> WorkflowOsError {
    let code = match reason {
        "invalid_artifact" => "github_pr_comment_report_artifact_write.invalid_artifact",
        "identity_mismatch" => "github_pr_comment_report_artifact_write.identity_mismatch",
        "citation_invalid" => "github_pr_comment_report_artifact_write.citation_invalid",
        "approval_linkage_invalid" => {
            "github_pr_comment_report_artifact_write.approval_linkage_invalid"
        }
        _ => "github_pr_comment_report_artifact_write.artifact_write_failed",
    };
    let message = match reason {
        "invalid_artifact" => "GitHub PR comment report artifact is invalid",
        "identity_mismatch" => {
            "GitHub PR comment report artifact does not match immutable run identity"
        }
        "citation_invalid" => "GitHub PR comment report artifact citation is invalid",
        "approval_linkage_invalid" => {
            "GitHub PR comment report artifact approval linkage is invalid"
        }
        _ => "GitHub PR comment report artifact write failed",
    };
    WorkflowOsError::invalid_state(code, message)
}

fn governed_artifact_write_error(reason: &'static str) -> WorkflowOsError {
    let code = match reason {
        "invalid_artifact" => "work_report_artifact.governed_write.invalid_artifact",
        "identity_mismatch" => "work_report_artifact.governed_write.identity_mismatch",
        _ => "work_report_artifact.governed_write.failed",
    };
    let message = match reason {
        "invalid_artifact" => "work report artifact could not be validated before governed write",
        "identity_mismatch" => {
            "work report artifact does not match the supplied terminal workflow run"
        }
        _ => "work report artifact governed write failed",
    };
    WorkflowOsError::invalid_state(code, message)
}

fn high_assurance_disclosure_gate_error(reason: &'static str) -> WorkflowOsError {
    let code = match reason {
        "missing" => "work_report_artifact.high_assurance_disclosure.missing",
        "invalid" => "work_report_artifact.high_assurance_disclosure.invalid",
        "validation_not_used" => {
            "work_report_artifact.high_assurance_disclosure.validation_not_used"
        }
        "validation_not_passed" => {
            "work_report_artifact.high_assurance_disclosure.validation_not_passed"
        }
        "denial_not_fail_closed" => {
            "work_report_artifact.high_assurance_disclosure.denial_not_fail_closed"
        }
        _ => "work_report_artifact.high_assurance_disclosure.failed",
    };
    let message = match reason {
        "missing" => "required high-assurance approval disclosure is missing",
        "invalid" => "high-assurance approval disclosure is invalid",
        "validation_not_used" => "high-assurance approval validation was not used",
        "validation_not_passed" => "high-assurance approval validation did not pass",
        "denial_not_fail_closed" => {
            "high-assurance approval denial behavior is not disclosed as fail-closed"
        }
        _ => "work report artifact high-assurance disclosure gate failed",
    };
    WorkflowOsError::invalid_state(code, message)
}

struct TerminalReportCitations {
    evidence: Vec<WorkReportCitation>,
    workflow_events: Vec<WorkReportCitation>,
    validation: Vec<WorkReportCitation>,
    local_checks: Vec<WorkReportCitation>,
    agent_harness_hooks: Vec<WorkReportCitation>,
    agent_harness_hook_disclosures: Vec<WorkReportCitation>,
    typed_handoffs: Vec<WorkReportCitation>,
    side_effects: Vec<WorkReportCitation>,
    policy: Vec<WorkReportCitation>,
    approvals: Vec<WorkReportCitation>,
}

fn terminal_report_citations(
    input: &TerminalLocalWorkReportInput<'_>,
    sensitivity: WorkReportSensitivity,
    redaction: &RedactionMetadata,
) -> Result<TerminalReportCitations, WorkflowOsError> {
    Ok(TerminalReportCitations {
        evidence: evidence_citations(
            input.evidence_reference_ids.clone(),
            input.adapter_telemetry_references.clone(),
            input.audit_event_ids.clone(),
            sensitivity,
            redaction,
        )?,
        workflow_events: workflow_event_citations(
            input.workflow_event_ids.clone(),
            sensitivity,
            redaction,
        )?,
        validation: validation_citations(
            input.validation_reference_ids.clone(),
            sensitivity,
            redaction,
        )?,
        local_checks: local_check_citations(
            input.local_check_result_references.clone(),
            sensitivity,
            redaction,
        )?,
        agent_harness_hooks: agent_harness_hook_citations(
            input.agent_harness_hook_invocation_ids.clone(),
            sensitivity,
            redaction,
        )?,
        agent_harness_hook_disclosures: agent_harness_hook_disclosure_citations(
            input.agent_harness_hook_disclosure_ids.clone(),
            sensitivity,
            redaction,
        )?,
        typed_handoffs: typed_handoff_citations(
            input.typed_handoff_ids.clone(),
            sensitivity,
            redaction,
        )?,
        side_effects: side_effect_citations(input.side_effect_ids.clone(), sensitivity, redaction)?,
        policy: policy_citations(input.policy_event_ids.clone(), sensitivity, redaction)?,
        approvals: approval_citations(
            input.approval_reference_ids.clone(),
            sensitivity,
            redaction,
        )?,
    })
}

fn terminal_report_sections(
    terminal_status: WorkReportStatus,
    citations: &TerminalReportCitations,
    input: &TerminalLocalWorkReportInput<'_>,
) -> Result<Vec<WorkReportSection>, WorkflowOsError> {
    Ok(vec![
        report_section(
            WorkReportSectionKind::WorkPerformed,
            work_performed_summary(terminal_status),
            citations.workflow_events.clone(),
        )?,
        report_section(
            WorkReportSectionKind::EvidenceConsidered,
            evidence_summary(citations.evidence.is_empty()),
            citations.evidence.clone(),
        )?,
        report_section(
            WorkReportSectionKind::DecisionsMade,
            decision_summary(citations.policy.is_empty(), citations.approvals.is_empty()),
            combined_citations(citations.policy.clone(), citations.approvals.clone()),
        )?,
        report_section(
            WorkReportSectionKind::PolicyGatesEvaluated,
            policy_summary(citations.policy.is_empty()),
            citations.policy.clone(),
        )?,
        report_section(
            WorkReportSectionKind::Approvals,
            approval_summary(
                citations.approvals.is_empty(),
                input.high_assurance_approval.as_ref(),
            ),
            citations.approvals.clone(),
        )?,
        report_section(
            WorkReportSectionKind::ValidationAndQualityChecks,
            validation_summary(citations),
            combined_citations(
                combined_citations(
                    combined_citations(
                        citations.validation.clone(),
                        citations.local_checks.clone(),
                    ),
                    citations.agent_harness_hooks.clone(),
                ),
                citations.agent_harness_hook_disclosures.clone(),
            ),
        )?,
        report_section(
            WorkReportSectionKind::SideEffects,
            side_effects_summary(citations.side_effects.is_empty(), input),
            citations.side_effects.clone(),
        )?,
        report_section(
            WorkReportSectionKind::IncompleteOrDeferredWork,
            disclosure_section_summary(
                input.incomplete_work.is_empty(),
                "No incomplete or deferred work was supplied.",
                "Incomplete or deferred work disclosures were supplied.",
            ),
            Vec::new(),
        )?,
        report_section(
            WorkReportSectionKind::KnownLimitations,
            disclosure_section_summary(
                input.known_limitations.is_empty(),
                "No known limitations were supplied.",
                "Known limitations were supplied.",
            ),
            Vec::new(),
        )?,
        report_section(
            WorkReportSectionKind::Risks,
            disclosure_section_summary(
                input.risks.is_empty(),
                "No additional risks were supplied.",
                "Risk disclosures were supplied.",
            ),
            Vec::new(),
        )?,
        report_section(
            WorkReportSectionKind::OperatorHandoffNotes,
            disclosure_section_summary(
                input.handoff_notes.is_empty() && citations.typed_handoffs.is_empty(),
                "No operator handoff notes were supplied.",
                "Operator handoff notes were supplied.",
            ),
            citations.typed_handoffs.clone(),
        )?,
    ])
}

fn report_section(
    kind: WorkReportSectionKind,
    summary: &str,
    citations: Vec<WorkReportCitation>,
) -> Result<WorkReportSection, WorkflowOsError> {
    WorkReportSection::new(kind, Some(summary.to_owned()), citations)
}

fn work_report_status_from_runtime(
    status: WorkflowRunStatus,
) -> Result<WorkReportStatus, WorkflowOsError> {
    match status {
        WorkflowRunStatus::Completed => Ok(WorkReportStatus::Completed),
        WorkflowRunStatus::Failed => Ok(WorkReportStatus::Failed),
        WorkflowRunStatus::Canceled => Ok(WorkReportStatus::Canceled),
        WorkflowRunStatus::Created
        | WorkflowRunStatus::Validated
        | WorkflowRunStatus::Running
        | WorkflowRunStatus::WaitingForApproval
        | WorkflowRunStatus::WaitingForExternalEvent
        | WorkflowRunStatus::Retrying
        | WorkflowRunStatus::Escalated => Err(validation_error(
            "work_report_generation.status.not_terminal",
            "terminal local work report generation requires a completed, failed, or canceled run",
        )),
    }
}

fn evidence_citations(
    evidence_reference_ids: Vec<EvidenceReferenceId>,
    adapter_telemetry_references: Vec<WorkReportStableReference>,
    audit_event_ids: Vec<EventId>,
    sensitivity: WorkReportSensitivity,
    redaction: &RedactionMetadata,
) -> Result<Vec<WorkReportCitation>, WorkflowOsError> {
    let mut citations = Vec::new();
    for evidence_reference_id in evidence_reference_ids {
        citations.push(report_citation(
            WorkReportCitationTarget::EvidenceReference {
                evidence_reference_id,
            },
            "Evidence reference considered.",
            sensitivity,
            redaction,
        )?);
    }
    for reference in adapter_telemetry_references {
        citations.push(report_citation(
            WorkReportCitationTarget::AdapterTelemetry { reference },
            "Adapter telemetry reference considered.",
            sensitivity,
            redaction,
        )?);
    }
    for audit_event_id in audit_event_ids {
        citations.push(report_citation(
            WorkReportCitationTarget::AuditEvent { audit_event_id },
            "Audit event reference considered.",
            sensitivity,
            redaction,
        )?);
    }
    Ok(citations)
}

fn workflow_event_citations(
    event_ids: Vec<EventId>,
    sensitivity: WorkReportSensitivity,
    redaction: &RedactionMetadata,
) -> Result<Vec<WorkReportCitation>, WorkflowOsError> {
    event_ids
        .into_iter()
        .map(|event_id| {
            report_citation(
                WorkReportCitationTarget::WorkflowEvent { event_id },
                "Workflow event reference considered.",
                sensitivity,
                redaction,
            )
        })
        .collect()
}

fn validation_citations(
    validation_reference_ids: Vec<ValidationReferenceId>,
    sensitivity: WorkReportSensitivity,
    redaction: &RedactionMetadata,
) -> Result<Vec<WorkReportCitation>, WorkflowOsError> {
    validation_reference_ids
        .into_iter()
        .map(|validation_reference_id| {
            report_citation(
                WorkReportCitationTarget::ValidationDiagnostic {
                    validation_reference_id,
                },
                "Validation diagnostic reference considered.",
                sensitivity,
                redaction,
            )
        })
        .collect()
}

fn local_check_citations(
    references: Vec<WorkReportStableReference>,
    sensitivity: WorkReportSensitivity,
    redaction: &RedactionMetadata,
) -> Result<Vec<WorkReportCitation>, WorkflowOsError> {
    references
        .into_iter()
        .map(|reference| {
            report_citation(
                WorkReportCitationTarget::LocalCheckResult { reference },
                "Local check result reference considered.",
                sensitivity,
                redaction,
            )
        })
        .collect()
}

fn typed_handoff_citations(
    typed_handoff_ids: Vec<TypedHandoffId>,
    sensitivity: WorkReportSensitivity,
    redaction: &RedactionMetadata,
) -> Result<Vec<WorkReportCitation>, WorkflowOsError> {
    typed_handoff_ids
        .into_iter()
        .map(|typed_handoff_id| {
            report_citation(
                WorkReportCitationTarget::TypedHandoff { typed_handoff_id },
                "Typed handoff reference considered.",
                sensitivity,
                redaction,
            )
        })
        .collect()
}

fn side_effect_citations(
    side_effect_ids: Vec<SideEffectId>,
    sensitivity: WorkReportSensitivity,
    redaction: &RedactionMetadata,
) -> Result<Vec<WorkReportCitation>, WorkflowOsError> {
    side_effect_ids
        .into_iter()
        .map(|side_effect_id| {
            report_citation(
                WorkReportCitationTarget::SideEffect { side_effect_id },
                "Side-effect record reference considered.",
                sensitivity,
                redaction,
            )
        })
        .collect()
}

fn agent_harness_hook_citations(
    hook_invocation_ids: Vec<AgentHarnessHookInvocationId>,
    sensitivity: WorkReportSensitivity,
    redaction: &RedactionMetadata,
) -> Result<Vec<WorkReportCitation>, WorkflowOsError> {
    hook_invocation_ids
        .into_iter()
        .map(|hook_invocation_id| {
            report_citation(
                WorkReportCitationTarget::AgentHarnessHook { hook_invocation_id },
                "Agent harness hook checkpoint reference considered.",
                sensitivity,
                redaction,
            )
        })
        .collect()
}

fn agent_harness_hook_disclosure_citations(
    disclosure_ids: Vec<AgentHarnessHookDisclosureId>,
    sensitivity: WorkReportSensitivity,
    redaction: &RedactionMetadata,
) -> Result<Vec<WorkReportCitation>, WorkflowOsError> {
    disclosure_ids
        .into_iter()
        .map(|disclosure_id| {
            report_citation(
                WorkReportCitationTarget::AgentHarnessHookDisclosure { disclosure_id },
                "Agent harness hook disclosure reference considered.",
                sensitivity,
                redaction,
            )
        })
        .collect()
}

fn policy_citations(
    event_ids: Vec<EventId>,
    sensitivity: WorkReportSensitivity,
    redaction: &RedactionMetadata,
) -> Result<Vec<WorkReportCitation>, WorkflowOsError> {
    event_ids
        .into_iter()
        .map(|event_id| {
            report_citation(
                WorkReportCitationTarget::PolicyDecision { event_id },
                "Policy decision reference considered.",
                sensitivity,
                redaction,
            )
        })
        .collect()
}

fn approval_citations(
    approval_reference_ids: Vec<ApprovalReferenceId>,
    sensitivity: WorkReportSensitivity,
    redaction: &RedactionMetadata,
) -> Result<Vec<WorkReportCitation>, WorkflowOsError> {
    approval_reference_ids
        .into_iter()
        .map(|approval_reference_id| {
            report_citation(
                WorkReportCitationTarget::ApprovalDecision {
                    approval_reference_id,
                },
                "Approval decision reference considered.",
                sensitivity,
                redaction,
            )
        })
        .collect()
}

fn report_citation(
    target: WorkReportCitationTarget,
    summary: &str,
    sensitivity: WorkReportSensitivity,
    redaction: &RedactionMetadata,
) -> Result<WorkReportCitation, WorkflowOsError> {
    WorkReportCitation::new(WorkReportCitationDefinition {
        target,
        summary: Some(summary.to_owned()),
        missing: false,
        redaction: redaction.clone(),
        sensitivity,
    })
}

fn combined_citations(
    mut left: Vec<WorkReportCitation>,
    right: Vec<WorkReportCitation>,
) -> Vec<WorkReportCitation> {
    left.extend(right);
    left
}

fn work_performed_summary(status: WorkReportStatus) -> &'static str {
    match status {
        WorkReportStatus::Completed => "Workflow run reached completed terminal status.",
        WorkReportStatus::Failed => "Workflow run reached failed terminal status.",
        WorkReportStatus::Canceled => "Workflow run reached canceled terminal status.",
        WorkReportStatus::Escalated | WorkReportStatus::Blocked => {
            "Workflow run reached terminal handoff status."
        }
    }
}

fn evidence_summary(no_citations: bool) -> &'static str {
    if no_citations {
        "No evidence, audit, or adapter telemetry references were supplied."
    } else {
        "Evidence, audit, or adapter telemetry references were supplied."
    }
}

fn decision_summary(no_policy: bool, no_approvals: bool) -> &'static str {
    if no_policy && no_approvals {
        "No stable policy or approval decision references were supplied."
    } else {
        "Stable policy or approval decision references were supplied."
    }
}

fn policy_summary(no_citations: bool) -> &'static str {
    if no_citations {
        "No stable policy gate references were supplied."
    } else {
        "Stable policy gate references were supplied."
    }
}

fn approval_summary(
    no_citations: bool,
    high_assurance: Option<&WorkReportHighAssuranceApprovalDisclosure>,
) -> &'static str {
    if let Some(disclosure) = high_assurance {
        if disclosure.validation_used() && disclosure.validation_passed() {
            return "High-assurance approval validation was used and passed before approval disclosure; stable approval references are cited when supplied.";
        }
        if disclosure.validation_used() {
            return "High-assurance approval validation was used but did not report a passed posture; stable approval references are cited when supplied.";
        }
        return "High-assurance approval disclosure was supplied without validation-used posture; stable approval references are cited when supplied.";
    }
    if no_citations {
        "No stable approval references were supplied."
    } else {
        "Stable approval references were supplied."
    }
}

fn side_effects_summary(
    no_citations: bool,
    input: &TerminalLocalWorkReportInput<'_>,
) -> &'static str {
    if !input.github_pr_comment_provider_disclosures.is_empty() {
        return provider_disclosure_side_effects_summary(
            input.github_pr_comment_provider_disclosures.as_slice(),
        );
    }
    if no_citations {
        "No write side effects are supported; side effects are none, skipped, or unsupported."
    } else {
        "Side-effect records were supplied as stable references; no side-effect payloads are copied."
    }
}

fn provider_disclosure_side_effects_summary(
    disclosures: &[GitHubPullRequestCommentProviderWriteReportDisclosure],
) -> &'static str {
    let has_missing_event = disclosures.iter().any(|disclosure| {
        matches!(
            disclosure.posture(),
            GitHubPullRequestCommentProviderWriteDisclosurePosture::ProviderSucceededLocalCompletedEventMissing
                | GitHubPullRequestCommentProviderWriteDisclosurePosture::ProviderFailedLocalFailedEventMissing
        )
    });
    if has_missing_event {
        return "GitHub PR comment provider disclosure was supplied; provider/local reconciliation is bounded, and workflow event proof is missing for at least one disclosure.";
    }

    let all_reconciled = disclosures.iter().all(|disclosure| {
        matches!(
            disclosure.posture(),
            GitHubPullRequestCommentProviderWriteDisclosurePosture::ProviderSucceededLocalCompletedEventAppended
                | GitHubPullRequestCommentProviderWriteDisclosurePosture::ProviderFailedLocalFailedEventAppended
        )
    });
    if all_reconciled {
        return "GitHub PR comment provider disclosure was supplied; provider/local reconciliation and workflow event proof are present.";
    }

    "GitHub PR comment provider disclosure was supplied; provider/local reconciliation posture requires bounded operator review."
}

fn validation_summary(citations: &TerminalReportCitations) -> &'static str {
    let no_validation = citations.validation.is_empty();
    let no_local_checks = citations.local_checks.is_empty();
    let no_agent_harness_hooks = citations.agent_harness_hooks.is_empty();
    let no_agent_harness_hook_disclosures = citations.agent_harness_hook_disclosures.is_empty();

    match (
        no_validation,
        no_local_checks,
        no_agent_harness_hooks,
        no_agent_harness_hook_disclosures,
    ) {
        (true, true, true, true) => {
            "No validation diagnostic, local check result, or agent harness hook references were supplied."
        }
        (false, true, true, true) => "Validation diagnostic references were supplied.",
        (true, false, true, true) => "Local check result references were supplied.",
        (true, true, false, true) => "Agent harness hook references were supplied.",
        (true, true, true, false) => {
            "Agent harness hook disclosure references were supplied."
        }
        (false, false, true, true) => {
            "Validation diagnostic and local check result references were supplied."
        }
        (false, true, false, true) => {
            "Validation diagnostic and agent harness hook references were supplied."
        }
        (false, true, true, false) => {
            "Validation diagnostic and agent harness hook disclosure references were supplied."
        }
        (true, false, false, true) => {
            "Local check result and agent harness hook references were supplied."
        }
        (true, false, true, false) => {
            "Local check result and agent harness hook disclosure references were supplied."
        }
        (true, true, false, false) => {
            "Agent harness hook and disclosure references were supplied."
        }
        (false, false, false, true) => {
            "Validation diagnostic, local check result, and agent harness hook references were supplied."
        }
        (false, false, true, false) => {
            "Validation diagnostic, local check result, and agent harness hook disclosure references were supplied."
        }
        (false, true, false, false) => {
            "Validation diagnostic, agent harness hook, and disclosure references were supplied."
        }
        (true, false, false, false) => {
            "Local check result, agent harness hook, and disclosure references were supplied."
        }
        (false, false, false, false) => {
            "Validation diagnostic, local check result, agent harness hook, and disclosure references were supplied."
        }
    }
}

fn disclosure_section_summary(
    is_empty: bool,
    empty_summary: &'static str,
    populated_summary: &'static str,
) -> &'static str {
    if is_empty {
        empty_summary
    } else {
        populated_summary
    }
}

fn report_notes_or_default<T>(
    values: Vec<String>,
    default_summary: &'static str,
    constructor: impl Fn(String, Vec<WorkReportCitation>) -> Result<T, WorkflowOsError>,
) -> Result<Vec<T>, WorkflowOsError> {
    if values.is_empty() {
        return Ok(vec![constructor(default_summary.to_owned(), Vec::new())?]);
    }

    values
        .into_iter()
        .map(|summary| constructor(summary, Vec::new()))
        .collect()
}

fn known_limitations_with_high_assurance(
    mut known_limitations: Vec<String>,
    high_assurance: Option<&WorkReportHighAssuranceApprovalDisclosure>,
) -> Vec<String> {
    if high_assurance.is_some() {
        known_limitations.push(
            "High-assurance approval disclosure is local and explicit; RBAC, IdP, quorum approval, revocation enforcement, workflow-declared controls, and write access are not implemented."
                .to_owned(),
        );
    }
    known_limitations
}

impl WorkReport {
    /// Creates a validated work report model.
    ///
    /// # Errors
    ///
    /// Returns a stable validation error when identity, sections, citations,
    /// disclosure, limitation, risk, or handoff-note fields are invalid.
    pub fn new(definition: WorkReportDefinition) -> Result<Self, WorkflowOsError> {
        let report = Self {
            report_id: definition.report_id,
            report_contract_id: definition.report_contract_id,
            report_contract_version: definition.report_contract_version,
            generation_context: definition.generation_context,
            sections: definition.sections,
            incomplete_work: definition.incomplete_work,
            known_limitations: definition.known_limitations,
            risks: definition.risks,
            handoff_notes: definition.handoff_notes,
            high_assurance_approval: definition.high_assurance_approval,
            sensitivity: definition.sensitivity,
            redaction: definition.redaction,
        };
        report.validate()?;
        Ok(report)
    }

    /// Validates the report model.
    ///
    /// # Errors
    ///
    /// Returns a stable validation error when the report shape is invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_not_secret_like(
            "schema version",
            self.generation_context.schema_version.as_str(),
        )?;
        validate_report_redaction_metadata(&self.redaction)?;
        validate_report_sections(&self.sections)?;

        for section in &self.sections {
            section.validate()?;
        }

        for disclosure in &self.incomplete_work {
            disclosure.validate()?;
        }
        for limitation in &self.known_limitations {
            limitation.validate()?;
        }
        for risk in &self.risks {
            risk.validate()?;
        }
        for note in &self.handoff_notes {
            note.validate()?;
        }
        if let Some(disclosure) = &self.high_assurance_approval {
            disclosure.validate()?;
        }

        Ok(())
    }

    /// Returns the report ID.
    #[must_use]
    pub const fn report_id(&self) -> &WorkReportId {
        &self.report_id
    }

    /// Returns the report contract ID.
    #[must_use]
    pub const fn report_contract_id(&self) -> &WorkReportContractId {
        &self.report_contract_id
    }

    /// Returns the report contract version.
    #[must_use]
    pub const fn report_contract_version(&self) -> &WorkReportContractVersion {
        &self.report_contract_version
    }

    /// Returns generation context.
    #[must_use]
    pub const fn generation_context(&self) -> &WorkReportGenerationContext {
        &self.generation_context
    }

    /// Returns sections.
    #[must_use]
    pub fn sections(&self) -> &[WorkReportSection] {
        &self.sections
    }

    /// Returns incomplete-work disclosures.
    #[must_use]
    pub fn incomplete_work(&self) -> &[WorkReportIncompleteWorkDisclosure] {
        &self.incomplete_work
    }

    /// Returns known limitations.
    #[must_use]
    pub fn known_limitations(&self) -> &[WorkReportKnownLimitation] {
        &self.known_limitations
    }

    /// Returns risks.
    #[must_use]
    pub fn risks(&self) -> &[WorkReportRisk] {
        &self.risks
    }

    /// Returns handoff notes.
    #[must_use]
    pub fn handoff_notes(&self) -> &[WorkReportHandoffNote] {
        &self.handoff_notes
    }

    /// Returns optional high-assurance approval disclosure.
    #[must_use]
    pub const fn high_assurance_approval(
        &self,
    ) -> Option<&WorkReportHighAssuranceApprovalDisclosure> {
        self.high_assurance_approval.as_ref()
    }

    /// Returns sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> WorkReportSensitivity {
        self.sensitivity
    }
}

impl fmt::Debug for WorkReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkReport")
            .field("report_id", &self.report_id)
            .field("report_contract_id", &self.report_contract_id)
            .field("report_contract_version", &self.report_contract_version)
            .field("generation_context", &self.generation_context)
            .field("section_count", &self.sections.len())
            .field("incomplete_work_count", &self.incomplete_work.len())
            .field("known_limitation_count", &self.known_limitations.len())
            .field("risk_count", &self.risks.len())
            .field("handoff_note_count", &self.handoff_notes.len())
            .field(
                "has_high_assurance_approval",
                &self.high_assurance_approval.is_some(),
            )
            .field("sensitivity", &self.sensitivity)
            .field(
                "redaction",
                &RedactedRedactionMetadataDebug(&self.redaction),
            )
            .finish()
    }
}

impl<'de> Deserialize<'de> for WorkReport {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WorkReportWire {
            report_id: WorkReportId,
            report_contract_id: WorkReportContractId,
            report_contract_version: WorkReportContractVersion,
            generation_context: WorkReportGenerationContext,
            sections: Vec<WorkReportSection>,
            incomplete_work: Vec<WorkReportIncompleteWorkDisclosure>,
            known_limitations: Vec<WorkReportKnownLimitation>,
            risks: Vec<WorkReportRisk>,
            handoff_notes: Vec<WorkReportHandoffNote>,
            #[serde(default)]
            high_assurance_approval: Option<WorkReportHighAssuranceApprovalDisclosure>,
            sensitivity: WorkReportSensitivity,
            redaction: RedactionMetadata,
        }

        let wire = WorkReportWire::deserialize(deserializer)?;
        Self::new(WorkReportDefinition {
            report_id: wire.report_id,
            report_contract_id: wire.report_contract_id,
            report_contract_version: wire.report_contract_version,
            generation_context: wire.generation_context,
            sections: wire.sections,
            incomplete_work: wire.incomplete_work,
            known_limitations: wire.known_limitations,
            risks: wire.risks,
            handoff_notes: wire.handoff_notes,
            high_assurance_approval: wire.high_assurance_approval,
            sensitivity: wire.sensitivity,
            redaction: wire.redaction,
        })
        .map_err(serde::de::Error::custom)
    }
}

/// Metadata for a durable local work report artifact.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkReportArtifactMetadata {
    report_id: WorkReportId,
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    schema_version: SchemaVersion,
    spec_hash: SpecContentHash,
    run_id: WorkflowRunId,
    terminal_run_status: WorkReportStatus,
    generated_at: Timestamp,
    sensitivity: WorkReportSensitivity,
    redaction: RedactionMetadata,
}

impl WorkReportArtifactMetadata {
    fn from_report(report: &WorkReport) -> Self {
        let context = report.generation_context();
        Self {
            report_id: report.report_id().clone(),
            workflow_id: context.workflow_id.clone(),
            workflow_version: context.workflow_version.clone(),
            schema_version: context.schema_version.clone(),
            spec_hash: context.spec_hash.clone(),
            run_id: context.run_id.clone(),
            terminal_run_status: context.terminal_run_status,
            generated_at: context.generated_at,
            sensitivity: report.sensitivity(),
            redaction: report.redaction.clone(),
        }
    }

    /// Returns the report ID bound to the artifact.
    #[must_use]
    pub const fn report_id(&self) -> &WorkReportId {
        &self.report_id
    }

    /// Returns the workflow run ID bound to the artifact.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the terminal status represented by the artifact.
    #[must_use]
    pub const fn terminal_run_status(&self) -> WorkReportStatus {
        self.terminal_run_status
    }

    /// Returns the artifact sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> WorkReportSensitivity {
        self.sensitivity
    }

    fn validate_against_report(&self, report: &WorkReport) -> Result<(), WorkflowOsError> {
        validate_report_redaction_metadata(&self.redaction)?;
        let context = report.generation_context();
        if &self.report_id != report.report_id()
            || self.workflow_id != context.workflow_id
            || self.workflow_version != context.workflow_version
            || self.schema_version != context.schema_version
            || self.spec_hash != context.spec_hash
            || self.run_id != context.run_id
            || self.terminal_run_status != context.terminal_run_status
            || self.generated_at != context.generated_at
            || self.sensitivity != report.sensitivity()
        {
            return Err(validation_error(
                "work_report_artifact.identity.mismatch",
                "work report artifact metadata must match the contained report",
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for WorkReportArtifactMetadata {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkReportArtifactMetadata")
            .field("report_id", &"[REDACTED]")
            .field("workflow_id", &"[REDACTED]")
            .field("workflow_version", &"[REDACTED]")
            .field("schema_version", &self.schema_version)
            .field("spec_hash", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("terminal_run_status", &self.terminal_run_status)
            .field("generated_at", &self.generated_at)
            .field("sensitivity", &self.sensitivity)
            .field(
                "redaction",
                &RedactedRedactionMetadataDebug(&self.redaction),
            )
            .finish()
    }
}

impl<'de> Deserialize<'de> for WorkReportArtifactMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WorkReportArtifactMetadataWire {
            report_id: WorkReportId,
            workflow_id: WorkflowId,
            workflow_version: WorkflowVersion,
            schema_version: SchemaVersion,
            spec_hash: SpecContentHash,
            run_id: WorkflowRunId,
            terminal_run_status: WorkReportStatus,
            generated_at: Timestamp,
            sensitivity: WorkReportSensitivity,
            redaction: RedactionMetadata,
        }

        let wire = WorkReportArtifactMetadataWire::deserialize(deserializer)?;
        validate_report_redaction_metadata(&wire.redaction).map_err(serde::de::Error::custom)?;
        Ok(Self {
            report_id: wire.report_id,
            workflow_id: wire.workflow_id,
            workflow_version: wire.workflow_version,
            schema_version: wire.schema_version,
            spec_hash: wire.spec_hash,
            run_id: wire.run_id,
            terminal_run_status: wire.terminal_run_status,
            generated_at: wire.generated_at,
            sensitivity: wire.sensitivity,
            redaction: wire.redaction,
        })
    }
}

/// Durable local artifact record for one validated `WorkReport`.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkReportArtifactRecord {
    metadata: WorkReportArtifactMetadata,
    work_report: WorkReport,
}

impl WorkReportArtifactRecord {
    /// Creates a validated artifact record from an existing report.
    ///
    /// # Errors
    ///
    /// Returns a stable validation error when the report or derived artifact
    /// metadata is invalid.
    pub fn new(work_report: WorkReport) -> Result<Self, WorkflowOsError> {
        work_report.validate()?;
        let metadata = WorkReportArtifactMetadata::from_report(&work_report);
        let artifact = Self {
            metadata,
            work_report,
        };
        artifact.validate()?;
        Ok(artifact)
    }

    /// Validates artifact metadata and contained report.
    ///
    /// # Errors
    ///
    /// Returns a stable validation error when metadata does not match the
    /// contained report, or when either value fails report validation.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.work_report.validate()?;
        self.metadata.validate_against_report(&self.work_report)
    }

    /// Returns artifact metadata.
    #[must_use]
    pub const fn metadata(&self) -> &WorkReportArtifactMetadata {
        &self.metadata
    }

    /// Returns the contained work report.
    #[must_use]
    pub const fn work_report(&self) -> &WorkReport {
        &self.work_report
    }

    /// Returns the artifact report ID.
    #[must_use]
    pub const fn report_id(&self) -> &WorkReportId {
        self.metadata.report_id()
    }

    /// Returns the artifact run ID.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        self.metadata.run_id()
    }
}

impl fmt::Debug for WorkReportArtifactRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkReportArtifactRecord")
            .field("metadata", &self.metadata)
            .field("work_report", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for WorkReportArtifactRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WorkReportArtifactRecordWire {
            metadata: WorkReportArtifactMetadata,
            work_report: WorkReport,
        }

        let wire = WorkReportArtifactRecordWire::deserialize(deserializer)?;
        let artifact = Self {
            metadata: wire.metadata,
            work_report: wire.work_report,
        };
        artifact.validate().map_err(serde::de::Error::custom)?;
        Ok(artifact)
    }
}

/// Domain-neutral contract for future terminal governed work reports.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct WorkReportContract {
    contract_id: WorkReportContractId,
    contract_version: WorkReportContractVersion,
    schema_version: SchemaVersion,
    required_sections: Vec<WorkReportSectionRequirement>,
    citation_requirements: Vec<WorkReportCitationRequirement>,
    redaction_policy: WorkReportRedactionPolicy,
    sensitivity: WorkReportSensitivity,
    disclosure_requirements: WorkReportDisclosureRequirements,
}

/// Input fields for constructing a validated `WorkReportContract`.
pub struct WorkReportContractDefinition {
    /// Report contract ID.
    pub contract_id: WorkReportContractId,
    /// Report contract version.
    pub contract_version: WorkReportContractVersion,
    /// Schema version associated with the contract model.
    pub schema_version: SchemaVersion,
    /// Required report sections.
    pub required_sections: Vec<WorkReportSectionRequirement>,
    /// Citation requirements for future report sections.
    pub citation_requirements: Vec<WorkReportCitationRequirement>,
    /// Redaction policy for future report generation.
    pub redaction_policy: WorkReportRedactionPolicy,
    /// Sensitivity classification.
    pub sensitivity: WorkReportSensitivity,
    /// Required disclosure categories.
    pub disclosure_requirements: WorkReportDisclosureRequirements,
}

impl WorkReportContract {
    /// Creates a validated work report contract.
    ///
    /// # Errors
    ///
    /// Returns an error when required sections or citation requirements are
    /// empty or duplicated, or when disclosure requirements are inconsistent
    /// with the required section list.
    pub fn new(definition: WorkReportContractDefinition) -> Result<Self, WorkflowOsError> {
        let contract = Self {
            contract_id: definition.contract_id,
            contract_version: definition.contract_version,
            schema_version: definition.schema_version,
            required_sections: definition.required_sections,
            citation_requirements: definition.citation_requirements,
            redaction_policy: definition.redaction_policy,
            sensitivity: definition.sensitivity,
            disclosure_requirements: definition.disclosure_requirements,
        };
        contract.validate()?;
        Ok(contract)
    }

    /// Creates a minimal v1 contract requiring all domain-neutral v1 sections.
    ///
    /// # Errors
    ///
    /// Returns an error when supplied identity fields are invalid.
    pub fn v1(
        contract_id: WorkReportContractId,
        contract_version: WorkReportContractVersion,
        schema_version: SchemaVersion,
    ) -> Result<Self, WorkflowOsError> {
        Self::new(WorkReportContractDefinition {
            contract_id,
            contract_version,
            schema_version,
            required_sections: WorkReportSectionKind::v1_required_kinds()
                .into_iter()
                .map(WorkReportSectionRequirement::required)
                .collect(),
            citation_requirements: vec![
                WorkReportCitationRequirement::new(WorkReportCitationKind::EvidenceReference, true),
                WorkReportCitationRequirement::new(WorkReportCitationKind::WorkflowEvent, true),
                WorkReportCitationRequirement::new(WorkReportCitationKind::AuditEvent, true),
                WorkReportCitationRequirement::new(
                    WorkReportCitationKind::ValidationDiagnostic,
                    true,
                ),
                WorkReportCitationRequirement::new(WorkReportCitationKind::ApprovalDecision, false),
                WorkReportCitationRequirement::new(WorkReportCitationKind::AdapterTelemetry, false),
                WorkReportCitationRequirement::new(WorkReportCitationKind::PolicyDecision, true),
            ],
            redaction_policy: WorkReportRedactionPolicy::ReferenceOnly,
            sensitivity: WorkReportSensitivity::conservative_default(),
            disclosure_requirements: WorkReportDisclosureRequirements::v1_required(),
        })
    }

    /// Validates the contract.
    ///
    /// # Errors
    ///
    /// Returns a stable validation error for invalid or inconsistent contract
    /// requirements.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_not_secret_like("schema version", self.schema_version.as_str())?;
        validate_required_sections(&self.required_sections)?;
        validate_citation_requirements(&self.citation_requirements)?;

        if self
            .disclosure_requirements
            .requires(WorkReportDisclosureKind::IncompleteOrDeferredWork)
            && !self
                .required_sections
                .iter()
                .any(|section| section.kind == WorkReportSectionKind::IncompleteOrDeferredWork)
        {
            return Err(validation_error(
                "work_report_contract.incomplete_section_required",
                "incomplete or deferred work disclosure requires the matching section",
            ));
        }

        if self
            .disclosure_requirements
            .requires(WorkReportDisclosureKind::KnownLimitations)
            && !self
                .required_sections
                .iter()
                .any(|section| section.kind == WorkReportSectionKind::KnownLimitations)
        {
            return Err(validation_error(
                "work_report_contract.known_limitations_section_required",
                "known limitations disclosure requires the matching section",
            ));
        }

        if self
            .disclosure_requirements
            .requires(WorkReportDisclosureKind::Risks)
            && !self
                .required_sections
                .iter()
                .any(|section| section.kind == WorkReportSectionKind::Risks)
        {
            return Err(validation_error(
                "work_report_contract.risks_section_required",
                "risk disclosure requires the matching section",
            ));
        }

        if self
            .disclosure_requirements
            .requires(WorkReportDisclosureKind::SideEffects)
            && !self
                .required_sections
                .iter()
                .any(|section| section.kind == WorkReportSectionKind::SideEffects)
        {
            return Err(validation_error(
                "work_report_contract.side_effect_section_required",
                "side-effect disclosure requires the matching section",
            ));
        }

        Ok(())
    }

    /// Returns the contract ID.
    #[must_use]
    pub const fn contract_id(&self) -> &WorkReportContractId {
        &self.contract_id
    }

    /// Returns the contract version.
    #[must_use]
    pub const fn contract_version(&self) -> &WorkReportContractVersion {
        &self.contract_version
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> &SchemaVersion {
        &self.schema_version
    }

    /// Returns required sections.
    #[must_use]
    pub fn required_sections(&self) -> &[WorkReportSectionRequirement] {
        &self.required_sections
    }

    /// Returns citation requirements.
    #[must_use]
    pub fn citation_requirements(&self) -> &[WorkReportCitationRequirement] {
        &self.citation_requirements
    }

    /// Returns the redaction policy.
    #[must_use]
    pub const fn redaction_policy(&self) -> WorkReportRedactionPolicy {
        self.redaction_policy
    }

    /// Returns sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> WorkReportSensitivity {
        self.sensitivity
    }

    /// Returns whether incomplete/deferred work disclosure is required.
    #[must_use]
    pub fn incomplete_work_disclosure_required(&self) -> bool {
        self.disclosure_requirements
            .requires(WorkReportDisclosureKind::IncompleteOrDeferredWork)
    }

    /// Returns whether known limitations disclosure is required.
    #[must_use]
    pub fn known_limitations_disclosure_required(&self) -> bool {
        self.disclosure_requirements
            .requires(WorkReportDisclosureKind::KnownLimitations)
    }

    /// Returns whether risk disclosure is required.
    #[must_use]
    pub fn risks_disclosure_required(&self) -> bool {
        self.disclosure_requirements
            .requires(WorkReportDisclosureKind::Risks)
    }

    /// Returns whether side-effect disclosure is required.
    #[must_use]
    pub fn side_effect_disclosure_required(&self) -> bool {
        self.disclosure_requirements
            .requires(WorkReportDisclosureKind::SideEffects)
    }

    /// Returns disclosure requirements.
    #[must_use]
    pub const fn disclosure_requirements(&self) -> &WorkReportDisclosureRequirements {
        &self.disclosure_requirements
    }
}

impl fmt::Debug for WorkReportContract {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkReportContract")
            .field("contract_id", &self.contract_id)
            .field("contract_version", &self.contract_version)
            .field("schema_version", &self.schema_version)
            .field("required_section_count", &self.required_sections.len())
            .field(
                "citation_requirement_count",
                &self.citation_requirements.len(),
            )
            .field("redaction_policy", &self.redaction_policy)
            .field("sensitivity", &self.sensitivity)
            .field(
                "disclosure_requirement_count",
                &self.disclosure_requirements.required().len(),
            )
            .finish()
    }
}

impl<'de> Deserialize<'de> for WorkReportContract {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WorkReportContractWire {
            contract_id: WorkReportContractId,
            contract_version: WorkReportContractVersion,
            schema_version: SchemaVersion,
            required_sections: Vec<WorkReportSectionRequirement>,
            citation_requirements: Vec<WorkReportCitationRequirement>,
            redaction_policy: WorkReportRedactionPolicy,
            sensitivity: WorkReportSensitivity,
            disclosure_requirements: WorkReportDisclosureRequirements,
        }

        let wire = WorkReportContractWire::deserialize(deserializer)?;
        Self::new(WorkReportContractDefinition {
            contract_id: wire.contract_id,
            contract_version: wire.contract_version,
            schema_version: wire.schema_version,
            required_sections: wire.required_sections,
            citation_requirements: wire.citation_requirements,
            redaction_policy: wire.redaction_policy,
            sensitivity: wire.sensitivity,
            disclosure_requirements: wire.disclosure_requirements,
        })
        .map_err(serde::de::Error::custom)
    }
}

fn validate_required_sections(
    required_sections: &[WorkReportSectionRequirement],
) -> Result<(), WorkflowOsError> {
    if required_sections.is_empty() {
        return Err(validation_error(
            "work_report_contract.sections.required",
            "work report contracts require at least one section",
        ));
    }

    let mut seen = BTreeSet::new();
    for section in required_sections {
        if !seen.insert(section.kind) {
            return Err(validation_error(
                "work_report_contract.sections.duplicate",
                "work report contracts cannot require the same section more than once",
            ));
        }
    }

    Ok(())
}

fn validate_citation_requirements(
    citation_requirements: &[WorkReportCitationRequirement],
) -> Result<(), WorkflowOsError> {
    let mut seen = BTreeSet::new();
    for requirement in citation_requirements {
        if !seen.insert(requirement.kind) {
            return Err(validation_error(
                "work_report_contract.citations.duplicate",
                "work report contracts cannot declare the same citation requirement more than once",
            ));
        }
    }

    Ok(())
}

fn validate_report_sections(sections: &[WorkReportSection]) -> Result<(), WorkflowOsError> {
    if sections.is_empty() {
        return Err(validation_error(
            "work_report.sections.required",
            "work reports require at least one section",
        ));
    }

    let mut seen = BTreeSet::new();
    for section in sections {
        if !seen.insert(section.kind()) {
            return Err(validation_error(
                "work_report.sections.duplicate",
                "work reports cannot include the same core section more than once",
            ));
        }
    }

    for required_kind in WorkReportSectionKind::v1_required_kinds() {
        if !seen.contains(&required_kind) {
            return Err(validation_error(
                "work_report.sections.missing_required",
                "work reports require all v1 core sections",
            ));
        }
    }

    Ok(())
}

fn validate_citations(citations: &[WorkReportCitation]) -> Result<(), WorkflowOsError> {
    for citation in citations {
        citation.validate()?;
    }
    Ok(())
}

fn validate_identifier(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "work_report_contract.identifier.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > 128 {
        return Err(validation_error(
            "work_report_contract.identifier.too_long",
            format!("{type_name} cannot exceed 128 bytes"),
        ));
    }

    let is_valid = value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'));

    if !is_valid {
        return Err(validation_error(
            "work_report_contract.identifier.invalid_character",
            format!("{type_name} contains an invalid character"),
        ));
    }

    validate_not_secret_like(type_name, value)?;

    Ok(())
}

fn validate_reference_text(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "work_report.reference.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > REPORT_REFERENCE_MAX_BYTES {
        return Err(validation_error(
            "work_report.reference.too_long",
            format!("{type_name} cannot exceed {REPORT_REFERENCE_MAX_BYTES} bytes"),
        ));
    }

    validate_not_secret_like(type_name, value)
}

fn validate_report_text(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "work_report.text.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > REPORT_TEXT_MAX_BYTES {
        return Err(validation_error(
            "work_report.text.too_long",
            format!("{type_name} cannot exceed {REPORT_TEXT_MAX_BYTES} bytes"),
        ));
    }

    validate_not_secret_like(type_name, value)
}

fn validate_report_redaction_metadata(
    redaction: &RedactionMetadata,
) -> Result<(), WorkflowOsError> {
    if redaction.redacted_fields.len() > REPORT_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "work_report.redaction.too_many_fields",
            "work report redaction metadata contains too many fields",
        ));
    }

    if redaction.field_states.len() > REPORT_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "work_report.redaction.too_many_states",
            "work report redaction metadata contains too many field states",
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
            "work_report.redaction.field.empty",
            "work report redaction field cannot be empty",
        ));
    }

    if value.len() > REPORT_REDACTION_FIELD_MAX_BYTES {
        return Err(validation_error(
            "work_report.redaction.field.too_long",
            format!(
                "work report redaction field cannot exceed {REPORT_REDACTION_FIELD_MAX_BYTES} bytes"
            ),
        ));
    }

    validate_not_secret_like("work report redaction field", value)
}

fn validate_redaction_reason(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "work_report.redaction.reason.empty",
            "work report redaction reason cannot be empty",
        ));
    }

    if value.len() > REPORT_REDACTION_REASON_MAX_BYTES {
        return Err(validation_error(
            "work_report.redaction.reason.too_long",
            format!(
                "work report redaction reason cannot exceed {REPORT_REDACTION_REASON_MAX_BYTES} bytes"
            ),
        ));
    }

    validate_not_secret_like("work report redaction reason", value)
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
            "work_report_contract.secret_like_identifier",
            format!("{type_name} contains sensitive-looking text"),
        ));
    }

    Ok(())
}

fn validation_error(code: &'static str, message: impl Into<String>) -> WorkflowOsError {
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
