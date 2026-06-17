use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ActorId, AgentHarnessHookDisclosureId, AgentHarnessHookInvocationId, ApprovalReferenceId,
    CorrelationId, EventId, EvidenceReferenceId, RedactionMetadata, SchemaVersion, SideEffectId,
    SpecContentHash, Timestamp, TypedHandoffId, ValidationReferenceId, WorkflowId, WorkflowOsError,
    WorkflowRun, WorkflowRunId, WorkflowRunStatus, WorkflowVersion,
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
    /// Typed handoff IDs to cite, where stable IDs already exist.
    pub typed_handoff_ids: Vec<TypedHandoffId>,
    /// Agent harness hook invocation IDs to cite, where stable IDs already exist.
    pub agent_harness_hook_invocation_ids: Vec<AgentHarnessHookInvocationId>,
    /// Agent harness hook disclosure IDs to cite, where stable IDs already exist.
    pub agent_harness_hook_disclosure_ids: Vec<AgentHarnessHookDisclosureId>,
    /// Bounded incomplete/deferred work disclosures.
    pub incomplete_work: Vec<String>,
    /// Bounded known limitations.
    pub known_limitations: Vec<String>,
    /// Bounded risks.
    pub risks: Vec<String>,
    /// Bounded operator handoff notes.
    pub handoff_notes: Vec<String>,
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
            input.known_limitations,
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
        sensitivity,
        redaction,
    })
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

struct TerminalReportCitations {
    evidence: Vec<WorkReportCitation>,
    workflow_events: Vec<WorkReportCitation>,
    validation: Vec<WorkReportCitation>,
    local_checks: Vec<WorkReportCitation>,
    agent_harness_hooks: Vec<WorkReportCitation>,
    agent_harness_hook_disclosures: Vec<WorkReportCitation>,
    typed_handoffs: Vec<WorkReportCitation>,
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
            approval_summary(citations.approvals.is_empty()),
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
            "No write side effects are supported; side effects are none, skipped, or unsupported.",
            Vec::new(),
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

fn approval_summary(no_citations: bool) -> &'static str {
    if no_citations {
        "No stable approval references were supplied."
    } else {
        "Stable approval references were supplied."
    }
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
