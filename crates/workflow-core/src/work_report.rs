use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{SchemaVersion, WorkflowOsError};

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
