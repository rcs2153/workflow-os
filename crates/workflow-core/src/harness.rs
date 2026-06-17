use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ActorId, ApprovalReferenceId, CorrelationId, EventId, EvidenceReferenceId, LocalCheckResultId,
    PolicyId, RedactionMetadata, SchemaVersion, SpecContentHash, StepId, Timestamp, TypedHandoffId,
    ValidationReferenceId, WorkReportCitationTarget, WorkReportRedactionPolicy,
    WorkReportSensitivity, WorkflowId, WorkflowOsError, WorkflowRunId, WorkflowVersion,
};

const HARNESS_IDENTIFIER_MAX_BYTES: usize = 128;
const HARNESS_TEXT_MAX_BYTES: usize = 1_000;
const HARNESS_REDACTION_FIELD_MAX_BYTES: usize = 128;
const HARNESS_REDACTION_REASON_MAX_BYTES: usize = 512;
const HARNESS_REDACTION_MAX_ENTRIES: usize = 64;

/// Identifier for a composable harness contract.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct HarnessContractId(String);

impl HarnessContractId {
    /// Creates a validated harness contract ID.
    ///
    /// # Errors
    ///
    /// Returns an error when the ID is empty, too long, contains unsupported
    /// characters, or looks like a secret.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("HarnessContractId", &value)?;
        Ok(Self(value))
    }

    /// Returns the ID as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HarnessContractId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for HarnessContractId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("HarnessContractId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<HarnessContractId> for String {
    fn from(value: HarnessContractId) -> Self {
        value.0
    }
}

impl TryFrom<String> for HarnessContractId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for HarnessContractId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Version for a composable harness contract.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct HarnessContractVersion(String);

impl HarnessContractVersion {
    /// Creates a validated harness contract version.
    ///
    /// # Errors
    ///
    /// Returns an error when the version is empty, too long, contains
    /// unsupported characters, or looks like a secret.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("HarnessContractVersion", &value)?;
        Ok(Self(value))
    }

    /// Returns the version as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HarnessContractVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for HarnessContractVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("HarnessContractVersion")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<HarnessContractVersion> for String {
    fn from(value: HarnessContractVersion) -> Self {
        value.0
    }
}

impl TryFrom<String> for HarnessContractVersion {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for HarnessContractVersion {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Tool or capability family allowed by a harness contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HarnessToolKind {
    /// Workflow OS skill.
    Skill,
    /// Read-only adapter capability.
    AdapterRead,
    /// Local check capability.
    LocalCheck,
    /// Policy evaluation capability.
    PolicyCheck,
    /// Approval request capability.
    ApprovalRequest,
    /// Work report generation capability.
    ReportGeneration,
}

/// Authority delegated to a harness.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HarnessAuthorityScope {
    /// Read bounded context.
    ReadContext,
    /// Run local deterministic checks.
    RunLocalChecks,
    /// Read through adapters.
    ReadAdapters,
    /// Request approval.
    RequestApprovals,
    /// Generate work reports.
    GenerateReports,
    /// Propose side effects without executing them.
    ProposeSideEffects,
}

/// Side-effect allowance declared by a harness contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HarnessSideEffectAllowance {
    /// Side effects are unsupported by this harness.
    Unsupported,
    /// No side effects are allowed.
    None,
    /// Side effects may only be proposed for later policy/approval handling.
    ProposedOnly,
}

/// Failure behavior for a harness.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HarnessFailureSemantics {
    /// Fail the parent workflow.
    FailParentWorkflow,
    /// Produce a blocked handoff.
    ProduceBlockedHandoff,
    /// Require approval before continuing.
    RequireApproval,
    /// Retry according to bounded policy.
    RetryWithPolicy,
    /// Skip and disclose explicitly.
    SkipWithDisclosure,
    /// Produce partial output with known limitations.
    PartialOutputWithLimitations,
    /// Cancel downstream harnesses.
    CancelDownstream,
    /// Escalate to an operator.
    Escalate,
}

/// Required or allowed harness input.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HarnessInputRequirement {
    name: String,
    required: bool,
}

impl HarnessInputRequirement {
    /// Creates a harness input requirement.
    ///
    /// # Errors
    ///
    /// Returns an error when the name is invalid.
    pub fn new(name: impl Into<String>, required: bool) -> Result<Self, WorkflowOsError> {
        let requirement = Self {
            name: name.into(),
            required,
        };
        requirement.validate()?;
        Ok(requirement)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_identifier("harness input name", &self.name)
    }

    /// Returns the input name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the input is required.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}

impl fmt::Debug for HarnessInputRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HarnessInputRequirement")
            .field("name", &"[REDACTED]")
            .field("required", &self.required)
            .finish()
    }
}

/// Context required by a harness.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HarnessContextRequirement {
    name: String,
    required: bool,
}

impl HarnessContextRequirement {
    /// Creates a harness context requirement.
    ///
    /// # Errors
    ///
    /// Returns an error when the name is invalid.
    pub fn new(name: impl Into<String>, required: bool) -> Result<Self, WorkflowOsError> {
        let requirement = Self {
            name: name.into(),
            required,
        };
        requirement.validate()?;
        Ok(requirement)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_identifier("harness context name", &self.name)
    }

    /// Returns the context requirement name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the context is required.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}

impl fmt::Debug for HarnessContextRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HarnessContextRequirement")
            .field("name", &"[REDACTED]")
            .field("required", &self.required)
            .finish()
    }
}

/// Tool allowance declared by a harness contract.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HarnessToolAllowance {
    name: String,
    kind: HarnessToolKind,
}

impl HarnessToolAllowance {
    /// Creates a harness tool allowance.
    ///
    /// # Errors
    ///
    /// Returns an error when the name is invalid.
    pub fn new(name: impl Into<String>, kind: HarnessToolKind) -> Result<Self, WorkflowOsError> {
        let allowance = Self {
            name: name.into(),
            kind,
        };
        allowance.validate()?;
        Ok(allowance)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_identifier("harness tool name", &self.name)
    }

    /// Returns the tool name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the tool kind.
    #[must_use]
    pub const fn kind(&self) -> HarnessToolKind {
        self.kind
    }
}

impl fmt::Debug for HarnessToolAllowance {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HarnessToolAllowance")
            .field("name", &"[REDACTED]")
            .field("kind", &self.kind)
            .finish()
    }
}

/// Output requirement declared by a harness contract.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HarnessOutputRequirement {
    name: String,
    required: bool,
}

impl HarnessOutputRequirement {
    /// Creates a harness output requirement.
    ///
    /// # Errors
    ///
    /// Returns an error when the name is invalid.
    pub fn new(name: impl Into<String>, required: bool) -> Result<Self, WorkflowOsError> {
        let requirement = Self {
            name: name.into(),
            required,
        };
        requirement.validate()?;
        Ok(requirement)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_identifier("harness output name", &self.name)
    }

    /// Returns the output name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the output is required.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}

impl fmt::Debug for HarnessOutputRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HarnessOutputRequirement")
            .field("name", &"[REDACTED]")
            .field("required", &self.required)
            .finish()
    }
}

/// Evidence requirement declared by a harness contract.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HarnessEvidenceRequirement {
    name: String,
    required: bool,
}

impl HarnessEvidenceRequirement {
    /// Creates a harness evidence requirement.
    ///
    /// # Errors
    ///
    /// Returns an error when the name is invalid.
    pub fn new(name: impl Into<String>, required: bool) -> Result<Self, WorkflowOsError> {
        let requirement = Self {
            name: name.into(),
            required,
        };
        requirement.validate()?;
        Ok(requirement)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_identifier("harness evidence name", &self.name)
    }

    /// Returns the evidence requirement name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the evidence is required.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}

impl fmt::Debug for HarnessEvidenceRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HarnessEvidenceRequirement")
            .field("name", &"[REDACTED]")
            .field("required", &self.required)
            .finish()
    }
}

/// Approval requirement declared by a harness contract.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HarnessApprovalRequirement {
    name: String,
    required: bool,
}

impl HarnessApprovalRequirement {
    /// Creates a harness approval requirement.
    ///
    /// # Errors
    ///
    /// Returns an error when the name is invalid.
    pub fn new(name: impl Into<String>, required: bool) -> Result<Self, WorkflowOsError> {
        let requirement = Self {
            name: name.into(),
            required,
        };
        requirement.validate()?;
        Ok(requirement)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_identifier("harness approval name", &self.name)
    }

    /// Returns the approval requirement name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the approval is required.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}

impl fmt::Debug for HarnessApprovalRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HarnessApprovalRequirement")
            .field("name", &"[REDACTED]")
            .field("required", &self.required)
            .finish()
    }
}

/// Handoff requirement declared by a harness contract.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HarnessHandoffRequirement {
    name: String,
    required: bool,
}

impl HarnessHandoffRequirement {
    /// Creates a harness handoff requirement.
    ///
    /// # Errors
    ///
    /// Returns an error when the name is invalid.
    pub fn new(name: impl Into<String>, required: bool) -> Result<Self, WorkflowOsError> {
        let requirement = Self {
            name: name.into(),
            required,
        };
        requirement.validate()?;
        Ok(requirement)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_identifier("harness handoff name", &self.name)
    }

    /// Returns the handoff requirement name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the handoff is required.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}

impl fmt::Debug for HarnessHandoffRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HarnessHandoffRequirement")
            .field("name", &"[REDACTED]")
            .field("required", &self.required)
            .finish()
    }
}

/// Execution policy for a harness contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HarnessExecutionPolicy {
    /// Whether a timeout declaration is required before execution.
    pub timeout_required: bool,
    /// Whether a budget declaration is required before execution.
    pub budget_required: bool,
    /// Whether retries are allowed by the contract.
    pub retry_allowed: bool,
}

impl HarnessExecutionPolicy {
    /// Creates a conservative execution policy.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            timeout_required: true,
            budget_required: true,
            retry_allowed: false,
        }
    }
}

/// Domain-neutral contract for a future bounded harness.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct HarnessContract {
    contract_id: HarnessContractId,
    contract_version: HarnessContractVersion,
    schema_version: SchemaVersion,
    purpose: String,
    input_requirements: Vec<HarnessInputRequirement>,
    context_requirements: Vec<HarnessContextRequirement>,
    tool_allowances: Vec<HarnessToolAllowance>,
    authority_scopes: Vec<HarnessAuthorityScope>,
    side_effect_allowance: HarnessSideEffectAllowance,
    output_requirements: Vec<HarnessOutputRequirement>,
    evidence_requirements: Vec<HarnessEvidenceRequirement>,
    approval_requirements: Vec<HarnessApprovalRequirement>,
    execution_policy: HarnessExecutionPolicy,
    failure_semantics: Vec<HarnessFailureSemantics>,
    handoff_requirements: Vec<HarnessHandoffRequirement>,
    sensitivity: WorkReportSensitivity,
    redaction_policy: WorkReportRedactionPolicy,
    redaction: RedactionMetadata,
}

/// Input fields for constructing a validated `HarnessContract`.
pub struct HarnessContractDefinition {
    /// Harness contract ID.
    pub contract_id: HarnessContractId,
    /// Harness contract version.
    pub contract_version: HarnessContractVersion,
    /// Schema version associated with the contract model.
    pub schema_version: SchemaVersion,
    /// Domain-neutral purpose statement.
    pub purpose: String,
    /// Allowed or required inputs.
    pub input_requirements: Vec<HarnessInputRequirement>,
    /// Required context declarations.
    pub context_requirements: Vec<HarnessContextRequirement>,
    /// Allowed tool families.
    pub tool_allowances: Vec<HarnessToolAllowance>,
    /// Delegated authority scopes.
    pub authority_scopes: Vec<HarnessAuthorityScope>,
    /// Side-effect allowance.
    pub side_effect_allowance: HarnessSideEffectAllowance,
    /// Output requirements.
    pub output_requirements: Vec<HarnessOutputRequirement>,
    /// Evidence requirements.
    pub evidence_requirements: Vec<HarnessEvidenceRequirement>,
    /// Approval requirements.
    pub approval_requirements: Vec<HarnessApprovalRequirement>,
    /// Execution policy.
    pub execution_policy: HarnessExecutionPolicy,
    /// Failure semantics.
    pub failure_semantics: Vec<HarnessFailureSemantics>,
    /// Handoff requirements.
    pub handoff_requirements: Vec<HarnessHandoffRequirement>,
    /// Sensitivity classification.
    pub sensitivity: WorkReportSensitivity,
    /// Redaction policy.
    pub redaction_policy: WorkReportRedactionPolicy,
    /// Redaction metadata for contract fields.
    pub redaction: RedactionMetadata,
}

impl HarnessContract {
    /// Creates a validated harness contract.
    ///
    /// # Errors
    ///
    /// Returns an error when any required contract field is missing,
    /// duplicated, unbounded, or secret-like.
    pub fn new(definition: HarnessContractDefinition) -> Result<Self, WorkflowOsError> {
        let contract = Self {
            contract_id: definition.contract_id,
            contract_version: definition.contract_version,
            schema_version: definition.schema_version,
            purpose: definition.purpose,
            input_requirements: definition.input_requirements,
            context_requirements: definition.context_requirements,
            tool_allowances: definition.tool_allowances,
            authority_scopes: definition.authority_scopes,
            side_effect_allowance: definition.side_effect_allowance,
            output_requirements: definition.output_requirements,
            evidence_requirements: definition.evidence_requirements,
            approval_requirements: definition.approval_requirements,
            execution_policy: definition.execution_policy,
            failure_semantics: definition.failure_semantics,
            handoff_requirements: definition.handoff_requirements,
            sensitivity: definition.sensitivity,
            redaction_policy: definition.redaction_policy,
            redaction: definition.redaction,
        };
        contract.validate()?;
        Ok(contract)
    }

    /// Validates the contract.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error when the contract is
    /// invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_not_secret_like("schema version", self.schema_version.as_str())?;
        validate_text("harness purpose", &self.purpose)?;
        validate_named_requirements(
            "harness_contract.inputs.required",
            "harness contracts require at least one input declaration",
            &self.input_requirements,
            HarnessInputRequirement::validate,
            HarnessInputRequirement::name,
        )?;
        validate_named_requirements(
            "harness_contract.context.required",
            "harness contracts require at least one context declaration",
            &self.context_requirements,
            HarnessContextRequirement::validate,
            HarnessContextRequirement::name,
        )?;
        validate_tool_allowances(&self.tool_allowances)?;
        validate_enum_list(
            "harness_contract.authority.required",
            "harness contracts require at least one authority scope",
            "harness_contract.authority.duplicate",
            "harness contracts cannot declare duplicate authority scopes",
            &self.authority_scopes,
        )?;
        validate_named_requirements(
            "harness_contract.outputs.required",
            "harness contracts require at least one output declaration",
            &self.output_requirements,
            HarnessOutputRequirement::validate,
            HarnessOutputRequirement::name,
        )?;
        validate_named_requirements(
            "harness_contract.evidence.required",
            "harness contracts require at least one evidence declaration",
            &self.evidence_requirements,
            HarnessEvidenceRequirement::validate,
            HarnessEvidenceRequirement::name,
        )?;
        validate_named_requirements(
            "harness_contract.approvals.required",
            "harness contracts require at least one approval declaration",
            &self.approval_requirements,
            HarnessApprovalRequirement::validate,
            HarnessApprovalRequirement::name,
        )?;
        validate_enum_list(
            "harness_contract.failure.required",
            "harness contracts require at least one failure semantic",
            "harness_contract.failure.duplicate",
            "harness contracts cannot declare duplicate failure semantics",
            &self.failure_semantics,
        )?;
        validate_named_requirements(
            "harness_contract.handoffs.required",
            "harness contracts require at least one handoff declaration",
            &self.handoff_requirements,
            HarnessHandoffRequirement::validate,
            HarnessHandoffRequirement::name,
        )?;
        validate_redaction_metadata(&self.redaction)?;
        Ok(())
    }

    /// Returns the contract ID.
    #[must_use]
    pub const fn contract_id(&self) -> &HarnessContractId {
        &self.contract_id
    }

    /// Returns the contract version.
    #[must_use]
    pub const fn contract_version(&self) -> &HarnessContractVersion {
        &self.contract_version
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> &SchemaVersion {
        &self.schema_version
    }

    /// Returns the purpose.
    #[must_use]
    pub fn purpose(&self) -> &str {
        &self.purpose
    }

    /// Returns input requirements.
    #[must_use]
    pub fn input_requirements(&self) -> &[HarnessInputRequirement] {
        &self.input_requirements
    }

    /// Returns context requirements.
    #[must_use]
    pub fn context_requirements(&self) -> &[HarnessContextRequirement] {
        &self.context_requirements
    }

    /// Returns tool allowances.
    #[must_use]
    pub fn tool_allowances(&self) -> &[HarnessToolAllowance] {
        &self.tool_allowances
    }

    /// Returns authority scopes.
    #[must_use]
    pub fn authority_scopes(&self) -> &[HarnessAuthorityScope] {
        &self.authority_scopes
    }

    /// Returns side-effect allowance.
    #[must_use]
    pub const fn side_effect_allowance(&self) -> HarnessSideEffectAllowance {
        self.side_effect_allowance
    }

    /// Returns output requirements.
    #[must_use]
    pub fn output_requirements(&self) -> &[HarnessOutputRequirement] {
        &self.output_requirements
    }

    /// Returns evidence requirements.
    #[must_use]
    pub fn evidence_requirements(&self) -> &[HarnessEvidenceRequirement] {
        &self.evidence_requirements
    }

    /// Returns approval requirements.
    #[must_use]
    pub fn approval_requirements(&self) -> &[HarnessApprovalRequirement] {
        &self.approval_requirements
    }

    /// Returns execution policy.
    #[must_use]
    pub const fn execution_policy(&self) -> HarnessExecutionPolicy {
        self.execution_policy
    }

    /// Returns failure semantics.
    #[must_use]
    pub fn failure_semantics(&self) -> &[HarnessFailureSemantics] {
        &self.failure_semantics
    }

    /// Returns handoff requirements.
    #[must_use]
    pub fn handoff_requirements(&self) -> &[HarnessHandoffRequirement] {
        &self.handoff_requirements
    }

    /// Returns sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> WorkReportSensitivity {
        self.sensitivity
    }

    /// Returns redaction policy.
    #[must_use]
    pub const fn redaction_policy(&self) -> WorkReportRedactionPolicy {
        self.redaction_policy
    }
}

impl fmt::Debug for HarnessContract {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HarnessContract")
            .field("contract_id", &self.contract_id)
            .field("contract_version", &self.contract_version)
            .field("schema_version", &self.schema_version)
            .field("purpose", &"[REDACTED]")
            .field("input_requirement_count", &self.input_requirements.len())
            .field(
                "context_requirement_count",
                &self.context_requirements.len(),
            )
            .field("tool_allowance_count", &self.tool_allowances.len())
            .field("authority_scope_count", &self.authority_scopes.len())
            .field("side_effect_allowance", &self.side_effect_allowance)
            .field("output_requirement_count", &self.output_requirements.len())
            .field(
                "evidence_requirement_count",
                &self.evidence_requirements.len(),
            )
            .field(
                "approval_requirement_count",
                &self.approval_requirements.len(),
            )
            .field("execution_policy", &self.execution_policy)
            .field("failure_semantic_count", &self.failure_semantics.len())
            .field(
                "handoff_requirement_count",
                &self.handoff_requirements.len(),
            )
            .field("sensitivity", &self.sensitivity)
            .field("redaction_policy", &self.redaction_policy)
            .field("redaction", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for HarnessContract {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct HarnessContractWire {
            contract_id: HarnessContractId,
            contract_version: HarnessContractVersion,
            schema_version: SchemaVersion,
            purpose: String,
            input_requirements: Vec<HarnessInputRequirement>,
            context_requirements: Vec<HarnessContextRequirement>,
            tool_allowances: Vec<HarnessToolAllowance>,
            authority_scopes: Vec<HarnessAuthorityScope>,
            side_effect_allowance: HarnessSideEffectAllowance,
            output_requirements: Vec<HarnessOutputRequirement>,
            evidence_requirements: Vec<HarnessEvidenceRequirement>,
            approval_requirements: Vec<HarnessApprovalRequirement>,
            execution_policy: HarnessExecutionPolicy,
            failure_semantics: Vec<HarnessFailureSemantics>,
            handoff_requirements: Vec<HarnessHandoffRequirement>,
            sensitivity: WorkReportSensitivity,
            redaction_policy: WorkReportRedactionPolicy,
            redaction: RedactionMetadata,
        }

        let wire = HarnessContractWire::deserialize(deserializer)?;
        Self::new(HarnessContractDefinition {
            contract_id: wire.contract_id,
            contract_version: wire.contract_version,
            schema_version: wire.schema_version,
            purpose: wire.purpose,
            input_requirements: wire.input_requirements,
            context_requirements: wire.context_requirements,
            tool_allowances: wire.tool_allowances,
            authority_scopes: wire.authority_scopes,
            side_effect_allowance: wire.side_effect_allowance,
            output_requirements: wire.output_requirements,
            evidence_requirements: wire.evidence_requirements,
            approval_requirements: wire.approval_requirements,
            execution_policy: wire.execution_policy,
            failure_semantics: wire.failure_semantics,
            handoff_requirements: wire.handoff_requirements,
            sensitivity: wire.sensitivity,
            redaction_policy: wire.redaction_policy,
            redaction: wire.redaction,
        })
        .map_err(serde::de::Error::custom)
    }
}

/// Identifier for an agent harness hook contract.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct AgentHarnessHookContractId(String);

impl AgentHarnessHookContractId {
    /// Creates a validated agent harness hook contract ID.
    ///
    /// # Errors
    ///
    /// Returns an error when the ID is empty, too long, contains unsupported
    /// characters, or looks like a secret.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_hook_identifier("AgentHarnessHookContractId", &value)?;
        Ok(Self(value))
    }

    /// Returns the ID as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AgentHarnessHookContractId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for AgentHarnessHookContractId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("AgentHarnessHookContractId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<AgentHarnessHookContractId> for String {
    fn from(value: AgentHarnessHookContractId) -> Self {
        value.0
    }
}

impl TryFrom<String> for AgentHarnessHookContractId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for AgentHarnessHookContractId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Version for an agent harness hook contract.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct AgentHarnessHookContractVersion(String);

impl AgentHarnessHookContractVersion {
    /// Creates a validated agent harness hook contract version.
    ///
    /// # Errors
    ///
    /// Returns an error when the version is empty, too long, contains
    /// unsupported characters, or looks like a secret.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_hook_identifier("AgentHarnessHookContractVersion", &value)?;
        Ok(Self(value))
    }

    /// Returns the version as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AgentHarnessHookContractVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for AgentHarnessHookContractVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("AgentHarnessHookContractVersion")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<AgentHarnessHookContractVersion> for String {
    fn from(value: AgentHarnessHookContractVersion) -> Self {
        value.0
    }
}

impl TryFrom<String> for AgentHarnessHookContractVersion {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for AgentHarnessHookContractVersion {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Deterministic checkpoint kind for a future agent harness hook.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentHarnessHookKind {
    /// Before planning begins.
    BeforePlanning,
    /// After planning completes.
    AfterPlanning,
    /// Before implementation begins.
    BeforeImplementation,
    /// After implementation completes.
    AfterImplementation,
    /// Before validation begins.
    BeforeValidation,
    /// Before a local skill invocation begins.
    BeforeSkillInvocation,
    /// After validation completes.
    AfterValidation,
    /// Before review begins.
    BeforeReview,
    /// After review completes.
    AfterReview,
    /// Before terminal report generation.
    BeforeReport,
    /// After terminal report generation.
    AfterReport,
}

/// Failure behavior declared by an agent harness hook contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentHarnessHookFailureSemantics {
    /// Fail closed at the checkpoint.
    FailClosed,
    /// Produce a warning checkpoint without changing runtime semantics.
    WarningOnly,
    /// Require approval before continuation in a future runtime phase.
    RequireApproval,
    /// Escalate to an operator in a future runtime phase.
    Escalate,
    /// Skip with explicit disclosure.
    SkipWithDisclosure,
}

/// Side-effect posture for an agent harness hook contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentHarnessHookSideEffectAllowance {
    /// Hooks do not authorize side effects.
    Unsupported,
    /// Hooks explicitly allow no side effects.
    None,
    /// Side effects would be proposed only; rejected by the model-only hook contract phase.
    ProposedOnly,
}

/// Required or allowed input for an agent harness hook contract.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentHarnessHookInputRequirement {
    name: String,
    required: bool,
}

impl AgentHarnessHookInputRequirement {
    /// Creates an agent harness hook input requirement.
    ///
    /// # Errors
    ///
    /// Returns an error when the name is invalid.
    pub fn new(name: impl Into<String>, required: bool) -> Result<Self, WorkflowOsError> {
        let requirement = Self {
            name: name.into(),
            required,
        };
        requirement.validate()?;
        Ok(requirement)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_hook_identifier("agent harness hook input name", &self.name)
    }

    /// Returns the input name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the input is required.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}

impl fmt::Debug for AgentHarnessHookInputRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentHarnessHookInputRequirement")
            .field("name", &"[REDACTED]")
            .field("required", &self.required)
            .finish()
    }
}

/// Required or allowed output for an agent harness hook contract.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentHarnessHookOutputRequirement {
    name: String,
    required: bool,
}

impl AgentHarnessHookOutputRequirement {
    /// Creates an agent harness hook output requirement.
    ///
    /// # Errors
    ///
    /// Returns an error when the name is invalid.
    pub fn new(name: impl Into<String>, required: bool) -> Result<Self, WorkflowOsError> {
        let requirement = Self {
            name: name.into(),
            required,
        };
        requirement.validate()?;
        Ok(requirement)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_hook_identifier("agent harness hook output name", &self.name)
    }

    /// Returns the output name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the output is required.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}

impl fmt::Debug for AgentHarnessHookOutputRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentHarnessHookOutputRequirement")
            .field("name", &"[REDACTED]")
            .field("required", &self.required)
            .finish()
    }
}

/// Domain-neutral contract for a future deterministic agent harness hook.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct AgentHarnessHookContract {
    contract_id: AgentHarnessHookContractId,
    contract_version: AgentHarnessHookContractVersion,
    schema_version: SchemaVersion,
    hook_kind: AgentHarnessHookKind,
    purpose: String,
    input_requirements: Vec<AgentHarnessHookInputRequirement>,
    output_requirements: Vec<AgentHarnessHookOutputRequirement>,
    failure_semantics: Vec<AgentHarnessHookFailureSemantics>,
    side_effect_allowance: AgentHarnessHookSideEffectAllowance,
    sensitivity: WorkReportSensitivity,
    redaction_policy: WorkReportRedactionPolicy,
    redaction: RedactionMetadata,
}

/// Input fields for constructing a validated `AgentHarnessHookContract`.
pub struct AgentHarnessHookContractDefinition {
    /// Hook contract ID.
    pub contract_id: AgentHarnessHookContractId,
    /// Hook contract version.
    pub contract_version: AgentHarnessHookContractVersion,
    /// Schema version associated with the hook contract model.
    pub schema_version: SchemaVersion,
    /// Deterministic checkpoint kind.
    pub hook_kind: AgentHarnessHookKind,
    /// Domain-neutral purpose statement.
    pub purpose: String,
    /// Allowed or required inputs.
    pub input_requirements: Vec<AgentHarnessHookInputRequirement>,
    /// Required or expected outputs.
    pub output_requirements: Vec<AgentHarnessHookOutputRequirement>,
    /// Failure semantics vocabulary.
    pub failure_semantics: Vec<AgentHarnessHookFailureSemantics>,
    /// Side-effect posture.
    pub side_effect_allowance: AgentHarnessHookSideEffectAllowance,
    /// Sensitivity classification.
    pub sensitivity: WorkReportSensitivity,
    /// Redaction policy.
    pub redaction_policy: WorkReportRedactionPolicy,
    /// Redaction metadata for hook contract fields.
    pub redaction: RedactionMetadata,
}

impl AgentHarnessHookContract {
    /// Creates a validated agent harness hook contract.
    ///
    /// # Errors
    ///
    /// Returns an error when any required field is missing, duplicated,
    /// unbounded, secret-like, or authorizes side effects in this model-only phase.
    pub fn new(definition: AgentHarnessHookContractDefinition) -> Result<Self, WorkflowOsError> {
        let contract = Self {
            contract_id: definition.contract_id,
            contract_version: definition.contract_version,
            schema_version: definition.schema_version,
            hook_kind: definition.hook_kind,
            purpose: definition.purpose,
            input_requirements: definition.input_requirements,
            output_requirements: definition.output_requirements,
            failure_semantics: definition.failure_semantics,
            side_effect_allowance: definition.side_effect_allowance,
            sensitivity: definition.sensitivity,
            redaction_policy: definition.redaction_policy,
            redaction: definition.redaction,
        };
        contract.validate()?;
        Ok(contract)
    }

    /// Validates the hook contract.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error when the hook contract is
    /// invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_hook_not_secret_like("schema version", self.schema_version.as_str())?;
        validate_hook_text("agent harness hook purpose", &self.purpose)?;
        validate_hook_named_requirements(
            "agent_harness_hook.inputs.required",
            "agent harness hook contracts require at least one input declaration",
            "agent_harness_hook.inputs.duplicate",
            "agent harness hook contracts cannot declare duplicate input names",
            &self.input_requirements,
            AgentHarnessHookInputRequirement::validate,
            AgentHarnessHookInputRequirement::name,
        )?;
        validate_hook_named_requirements(
            "agent_harness_hook.outputs.required",
            "agent harness hook contracts require at least one output declaration",
            "agent_harness_hook.outputs.duplicate",
            "agent harness hook contracts cannot declare duplicate output names",
            &self.output_requirements,
            AgentHarnessHookOutputRequirement::validate,
            AgentHarnessHookOutputRequirement::name,
        )?;
        validate_enum_list(
            "agent_harness_hook.failure.required",
            "agent harness hook contracts require at least one failure semantic",
            "agent_harness_hook.failure.duplicate",
            "agent harness hook contracts cannot declare duplicate failure semantics",
            &self.failure_semantics,
        )?;
        if self.side_effect_allowance == AgentHarnessHookSideEffectAllowance::ProposedOnly {
            return Err(validation_error(
                "agent_harness_hook.side_effect.unsupported",
                "agent harness hook contracts cannot authorize side effects in the model-only phase",
            ));
        }
        validate_hook_redaction_metadata(&self.redaction)?;
        Ok(())
    }

    /// Returns the hook contract ID.
    #[must_use]
    pub const fn contract_id(&self) -> &AgentHarnessHookContractId {
        &self.contract_id
    }

    /// Returns the hook contract version.
    #[must_use]
    pub const fn contract_version(&self) -> &AgentHarnessHookContractVersion {
        &self.contract_version
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> &SchemaVersion {
        &self.schema_version
    }

    /// Returns the hook kind.
    #[must_use]
    pub const fn hook_kind(&self) -> AgentHarnessHookKind {
        self.hook_kind
    }

    /// Returns the purpose.
    #[must_use]
    pub fn purpose(&self) -> &str {
        &self.purpose
    }

    /// Returns input requirements.
    #[must_use]
    pub fn input_requirements(&self) -> &[AgentHarnessHookInputRequirement] {
        &self.input_requirements
    }

    /// Returns output requirements.
    #[must_use]
    pub fn output_requirements(&self) -> &[AgentHarnessHookOutputRequirement] {
        &self.output_requirements
    }

    /// Returns failure semantics.
    #[must_use]
    pub fn failure_semantics(&self) -> &[AgentHarnessHookFailureSemantics] {
        &self.failure_semantics
    }

    /// Returns side-effect allowance.
    #[must_use]
    pub const fn side_effect_allowance(&self) -> AgentHarnessHookSideEffectAllowance {
        self.side_effect_allowance
    }

    /// Returns sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> WorkReportSensitivity {
        self.sensitivity
    }

    /// Returns redaction policy.
    #[must_use]
    pub const fn redaction_policy(&self) -> WorkReportRedactionPolicy {
        self.redaction_policy
    }
}

impl fmt::Debug for AgentHarnessHookContract {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentHarnessHookContract")
            .field("contract_id", &self.contract_id)
            .field("contract_version", &self.contract_version)
            .field("schema_version", &self.schema_version)
            .field("hook_kind", &self.hook_kind)
            .field("purpose", &"[REDACTED]")
            .field("input_requirement_count", &self.input_requirements.len())
            .field("output_requirement_count", &self.output_requirements.len())
            .field("failure_semantic_count", &self.failure_semantics.len())
            .field("side_effect_allowance", &self.side_effect_allowance)
            .field("sensitivity", &self.sensitivity)
            .field("redaction_policy", &self.redaction_policy)
            .field("redaction", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for AgentHarnessHookContract {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct AgentHarnessHookContractWire {
            contract_id: AgentHarnessHookContractId,
            contract_version: AgentHarnessHookContractVersion,
            schema_version: SchemaVersion,
            hook_kind: AgentHarnessHookKind,
            purpose: String,
            input_requirements: Vec<AgentHarnessHookInputRequirement>,
            output_requirements: Vec<AgentHarnessHookOutputRequirement>,
            failure_semantics: Vec<AgentHarnessHookFailureSemantics>,
            side_effect_allowance: AgentHarnessHookSideEffectAllowance,
            sensitivity: WorkReportSensitivity,
            redaction_policy: WorkReportRedactionPolicy,
            redaction: RedactionMetadata,
        }

        let wire = AgentHarnessHookContractWire::deserialize(deserializer)?;
        Self::new(AgentHarnessHookContractDefinition {
            contract_id: wire.contract_id,
            contract_version: wire.contract_version,
            schema_version: wire.schema_version,
            hook_kind: wire.hook_kind,
            purpose: wire.purpose,
            input_requirements: wire.input_requirements,
            output_requirements: wire.output_requirements,
            failure_semantics: wire.failure_semantics,
            side_effect_allowance: wire.side_effect_allowance,
            sensitivity: wire.sensitivity,
            redaction_policy: wire.redaction_policy,
            redaction: wire.redaction,
        })
        .map_err(serde::de::Error::custom)
    }
}

/// Stable reference target supplied to a hook invocation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind", content = "id")]
pub enum AgentHarnessHookReference {
    /// `EvidenceReference` ID supplied by the caller.
    EvidenceReference(EvidenceReferenceId),
    /// Local check result ID supplied by the caller.
    LocalCheckResult(LocalCheckResultId),
    /// Typed handoff ID supplied by the caller.
    TypedHandoff(TypedHandoffId),
    /// Validation diagnostic or result reference ID supplied by the caller.
    Validation(ValidationReferenceId),
    /// Workflow event ID supplied by the caller.
    WorkflowEvent(EventId),
    /// Audit event ID supplied by the caller.
    AuditEvent(EventId),
    /// Policy definition ID supplied by the caller.
    Policy(PolicyId),
    /// Policy decision event ID supplied by the caller.
    PolicyDecisionEvent(EventId),
    /// Approval decision reference ID supplied by the caller.
    ApprovalDecision(ApprovalReferenceId),
}

impl AgentHarnessHookReference {
    fn validate(&self) -> Result<(), WorkflowOsError> {
        match self {
            Self::EvidenceReference(value) => {
                validate_invocation_reference("evidence reference", value.as_str())
            }
            Self::LocalCheckResult(value) => {
                validate_invocation_reference("local check result", value.as_str())
            }
            Self::TypedHandoff(value) => {
                validate_invocation_reference("typed handoff", value.as_str())
            }
            Self::Validation(value) => {
                validate_invocation_reference("validation reference", value.as_str())
            }
            Self::WorkflowEvent(value) => {
                validate_invocation_reference("workflow event", value.as_str())
            }
            Self::AuditEvent(value) => validate_invocation_reference("audit event", value.as_str()),
            Self::Policy(value) => validate_invocation_reference("policy", value.as_str()),
            Self::PolicyDecisionEvent(value) => {
                validate_invocation_reference("policy decision event", value.as_str())
            }
            Self::ApprovalDecision(value) => {
                validate_invocation_reference("approval decision", value.as_str())
            }
        }
    }
}

impl fmt::Debug for AgentHarnessHookReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self {
            Self::EvidenceReference(_) => "evidence_reference",
            Self::LocalCheckResult(_) => "local_check_result",
            Self::TypedHandoff(_) => "typed_handoff",
            Self::Validation(_) => "validation",
            Self::WorkflowEvent(_) => "workflow_event",
            Self::AuditEvent(_) => "audit_event",
            Self::Policy(_) => "policy",
            Self::PolicyDecisionEvent(_) => "policy_decision_event",
            Self::ApprovalDecision(_) => "approval_decision",
        };
        formatter
            .debug_struct("AgentHarnessHookReference")
            .field("kind", &kind)
            .field("id", &"[REDACTED]")
            .finish()
    }
}

/// Named stable reference supplied to satisfy a hook input or output.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentHarnessHookNamedReference {
    name: String,
    reference: AgentHarnessHookReference,
}

impl AgentHarnessHookNamedReference {
    /// Creates a validated named hook reference.
    ///
    /// # Errors
    ///
    /// Returns an error when the name or reference is invalid.
    pub fn new(
        name: impl Into<String>,
        reference: AgentHarnessHookReference,
    ) -> Result<Self, WorkflowOsError> {
        let value = Self {
            name: name.into(),
            reference,
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_invocation_identifier("agent harness hook reference name", &self.name)?;
        self.reference.validate()
    }

    /// Returns the reference name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the stable reference target.
    #[must_use]
    pub const fn reference(&self) -> &AgentHarnessHookReference {
        &self.reference
    }
}

impl fmt::Debug for AgentHarnessHookNamedReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentHarnessHookNamedReference")
            .field("name", &"[REDACTED]")
            .field("reference", &self.reference)
            .finish()
    }
}

/// Bounded disclosure kind supplied to a hook invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentHarnessHookDisclosureKind {
    /// Operator or system note.
    Note,
    /// Known limitation.
    Limitation,
    /// Risk disclosure.
    Risk,
    /// Incomplete or deferred work disclosure.
    IncompleteWork,
}

/// Bounded disclosure supplied to a hook invocation.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentHarnessHookDisclosure {
    kind: AgentHarnessHookDisclosureKind,
    text: String,
}

impl AgentHarnessHookDisclosure {
    /// Creates a bounded hook disclosure.
    ///
    /// # Errors
    ///
    /// Returns an error when the disclosure text is empty, unbounded, or
    /// secret-like.
    pub fn new(
        kind: AgentHarnessHookDisclosureKind,
        text: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let value = Self {
            kind,
            text: text.into(),
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_invocation_text("agent harness hook disclosure", &self.text)
    }

    /// Returns the disclosure kind.
    #[must_use]
    pub const fn kind(&self) -> AgentHarnessHookDisclosureKind {
        self.kind
    }

    /// Returns the disclosure text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl fmt::Debug for AgentHarnessHookDisclosure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentHarnessHookDisclosure")
            .field("kind", &self.kind)
            .field("text", &"[REDACTED]")
            .finish()
    }
}

/// Status vocabulary for an in-memory hook invocation result.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentHarnessHookInvocationStatus {
    /// Invocation context validated successfully.
    Passed,
    /// Invocation failed closed.
    FailedClosed,
    /// Invocation produced a warning.
    Warning,
    /// Invocation was skipped with disclosure.
    SkippedWithDisclosure,
    /// Invocation is blocked.
    Blocked,
}

/// Stable identifier for an agent harness hook invocation result or audit
/// record.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct AgentHarnessHookInvocationId(String);

impl AgentHarnessHookInvocationId {
    /// Creates a validated hook invocation ID.
    ///
    /// # Errors
    ///
    /// Returns an error when the ID is empty, too long, contains unsupported
    /// characters, or looks like a secret.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_invocation_identifier("AgentHarnessHookInvocationId", &value)?;
        Ok(Self(value))
    }

    /// Returns the ID as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AgentHarnessHookInvocationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for AgentHarnessHookInvocationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("AgentHarnessHookInvocationId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<AgentHarnessHookInvocationId> for String {
    fn from(value: AgentHarnessHookInvocationId) -> Self {
        value.0
    }
}

impl TryFrom<String> for AgentHarnessHookInvocationId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for AgentHarnessHookInvocationId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Explicit input for validating an in-memory agent harness hook invocation.
#[derive(Clone, Eq, PartialEq)]
pub struct AgentHarnessHookInvocationInput {
    /// Hook contract to validate against.
    pub contract: AgentHarnessHookContract,
    /// Workflow ID.
    pub workflow_id: WorkflowId,
    /// Workflow version.
    pub workflow_version: WorkflowVersion,
    /// Workflow run ID.
    pub run_id: WorkflowRunId,
    /// Schema version.
    pub schema_version: SchemaVersion,
    /// Workflow spec hash.
    pub spec_hash: SpecContentHash,
    /// Hook kind being invoked.
    pub hook_kind: AgentHarnessHookKind,
    /// Actor or system actor invoking the hook.
    pub actor: ActorId,
    /// Invocation timestamp.
    pub invoked_at: Timestamp,
    /// Optional correlation ID.
    pub correlation_id: Option<CorrelationId>,
    /// Optional step ID.
    pub step_id: Option<StepId>,
    /// Optional phase ID.
    pub phase_id: Option<String>,
    /// Supplied input references by stable name.
    pub input_references: Vec<AgentHarnessHookNamedReference>,
    /// Supplied output references by stable name.
    pub output_references: Vec<AgentHarnessHookNamedReference>,
    /// Additional stable references supplied as invocation context.
    pub supplemental_references: Vec<AgentHarnessHookReference>,
    /// Whether required outputs must be present for this invocation point.
    pub require_outputs: bool,
    /// Whether the caller requested or implied a side effect.
    pub side_effect_requested: bool,
    /// Bounded invocation disclosures.
    pub disclosures: Vec<AgentHarnessHookDisclosure>,
    /// Redaction metadata for invocation fields.
    pub redaction: RedactionMetadata,
    /// Sensitivity classification.
    pub sensitivity: WorkReportSensitivity,
}

/// In-memory result of validating an agent harness hook invocation.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct AgentHarnessHookInvocationResult {
    contract_id: AgentHarnessHookContractId,
    contract_version: AgentHarnessHookContractVersion,
    hook_kind: AgentHarnessHookKind,
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    run_id: WorkflowRunId,
    schema_version: SchemaVersion,
    spec_hash: SpecContentHash,
    actor: ActorId,
    invoked_at: Timestamp,
    correlation_id: Option<CorrelationId>,
    step_id: Option<StepId>,
    phase_id: Option<String>,
    status: AgentHarnessHookInvocationStatus,
    input_references: Vec<AgentHarnessHookNamedReference>,
    output_references: Vec<AgentHarnessHookNamedReference>,
    supplemental_references: Vec<AgentHarnessHookReference>,
    disclosures: Vec<AgentHarnessHookDisclosure>,
    redaction: RedactionMetadata,
    sensitivity: WorkReportSensitivity,
}

/// Input fields for constructing a validated `AgentHarnessHookInvocationResult`.
pub struct AgentHarnessHookInvocationResultDefinition {
    /// Hook contract ID.
    pub contract_id: AgentHarnessHookContractId,
    /// Hook contract version.
    pub contract_version: AgentHarnessHookContractVersion,
    /// Hook kind.
    pub hook_kind: AgentHarnessHookKind,
    /// Workflow ID.
    pub workflow_id: WorkflowId,
    /// Workflow version.
    pub workflow_version: WorkflowVersion,
    /// Workflow run ID.
    pub run_id: WorkflowRunId,
    /// Schema version.
    pub schema_version: SchemaVersion,
    /// Workflow spec hash.
    pub spec_hash: SpecContentHash,
    /// Actor or system actor.
    pub actor: ActorId,
    /// Invocation timestamp.
    pub invoked_at: Timestamp,
    /// Optional correlation ID.
    pub correlation_id: Option<CorrelationId>,
    /// Optional step ID.
    pub step_id: Option<StepId>,
    /// Optional phase ID.
    pub phase_id: Option<String>,
    /// Invocation status.
    pub status: AgentHarnessHookInvocationStatus,
    /// Validated input references.
    pub input_references: Vec<AgentHarnessHookNamedReference>,
    /// Validated output references.
    pub output_references: Vec<AgentHarnessHookNamedReference>,
    /// Validated supplemental references.
    pub supplemental_references: Vec<AgentHarnessHookReference>,
    /// Bounded disclosures.
    pub disclosures: Vec<AgentHarnessHookDisclosure>,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
    /// Sensitivity classification.
    pub sensitivity: WorkReportSensitivity,
}

impl AgentHarnessHookInvocationResult {
    /// Creates a validated in-memory hook invocation result.
    ///
    /// # Errors
    ///
    /// Returns an error when the result shape is invalid or contains unsafe
    /// caller-supplied metadata.
    pub fn new(
        definition: AgentHarnessHookInvocationResultDefinition,
    ) -> Result<Self, WorkflowOsError> {
        let result = Self {
            contract_id: definition.contract_id,
            contract_version: definition.contract_version,
            hook_kind: definition.hook_kind,
            workflow_id: definition.workflow_id,
            workflow_version: definition.workflow_version,
            run_id: definition.run_id,
            schema_version: definition.schema_version,
            spec_hash: definition.spec_hash,
            actor: definition.actor,
            invoked_at: definition.invoked_at,
            correlation_id: definition.correlation_id,
            step_id: definition.step_id,
            phase_id: definition.phase_id,
            status: definition.status,
            input_references: definition.input_references,
            output_references: definition.output_references,
            supplemental_references: definition.supplemental_references,
            disclosures: definition.disclosures,
            redaction: definition.redaction,
            sensitivity: definition.sensitivity,
        };
        result.validate()?;
        Ok(result)
    }

    /// Validates the invocation result.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error when the result is invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_invocation_reference("hook contract id", self.contract_id.as_str())?;
        validate_invocation_reference("hook contract version", self.contract_version.as_str())?;
        validate_invocation_reference("workflow id", self.workflow_id.as_str())?;
        validate_invocation_reference("workflow version", self.workflow_version.as_str())?;
        validate_invocation_reference("run id", self.run_id.as_str())?;
        validate_invocation_reference("schema version", self.schema_version.as_str())?;
        validate_invocation_reference("spec hash", self.spec_hash.as_str())?;
        validate_invocation_reference("actor", self.actor.as_str())?;
        if let Some(correlation_id) = &self.correlation_id {
            validate_invocation_reference("correlation id", correlation_id.as_str())?;
        }
        if let Some(step_id) = &self.step_id {
            validate_invocation_reference("step id", step_id.as_str())?;
        }
        if let Some(phase_id) = &self.phase_id {
            validate_invocation_identifier("phase id", phase_id)?;
        }
        validate_named_invocation_references(
            "agent_harness_hook_invocation.inputs.duplicate",
            "agent harness hook invocation cannot contain duplicate input reference names",
            &self.input_references,
        )?;
        validate_named_invocation_references(
            "agent_harness_hook_invocation.outputs.duplicate",
            "agent harness hook invocation cannot contain duplicate output reference names",
            &self.output_references,
        )?;
        for reference in &self.supplemental_references {
            reference.validate()?;
        }
        for disclosure in &self.disclosures {
            disclosure.validate()?;
        }
        validate_invocation_redaction_metadata(&self.redaction)?;
        Ok(())
    }

    /// Returns the hook contract ID.
    #[must_use]
    pub const fn contract_id(&self) -> &AgentHarnessHookContractId {
        &self.contract_id
    }

    /// Returns the hook contract version.
    #[must_use]
    pub const fn contract_version(&self) -> &AgentHarnessHookContractVersion {
        &self.contract_version
    }

    /// Returns the hook kind.
    #[must_use]
    pub const fn hook_kind(&self) -> AgentHarnessHookKind {
        self.hook_kind
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

    /// Returns the run ID.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the status.
    #[must_use]
    pub const fn status(&self) -> AgentHarnessHookInvocationStatus {
        self.status
    }

    /// Returns input references.
    #[must_use]
    pub fn input_references(&self) -> &[AgentHarnessHookNamedReference] {
        &self.input_references
    }

    /// Returns output references.
    #[must_use]
    pub fn output_references(&self) -> &[AgentHarnessHookNamedReference] {
        &self.output_references
    }

    /// Returns supplemental references.
    #[must_use]
    pub fn supplemental_references(&self) -> &[AgentHarnessHookReference] {
        &self.supplemental_references
    }

    /// Returns disclosures.
    #[must_use]
    pub fn disclosures(&self) -> &[AgentHarnessHookDisclosure] {
        &self.disclosures
    }

    /// Returns sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> WorkReportSensitivity {
        self.sensitivity
    }
}

impl fmt::Debug for AgentHarnessHookInvocationResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentHarnessHookInvocationResult")
            .field("contract_id", &self.contract_id)
            .field("contract_version", &self.contract_version)
            .field("hook_kind", &self.hook_kind)
            .field("workflow_id", &"[REDACTED]")
            .field("workflow_version", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("schema_version", &"[REDACTED]")
            .field("spec_hash", &"[REDACTED]")
            .field("actor", &"[REDACTED]")
            .field("invoked_at", &self.invoked_at)
            .field("correlation_id", &"[REDACTED]")
            .field("step_id", &"[REDACTED]")
            .field("phase_id", &"[REDACTED]")
            .field("status", &self.status)
            .field("input_reference_count", &self.input_references.len())
            .field("output_reference_count", &self.output_references.len())
            .field(
                "supplemental_reference_count",
                &self.supplemental_references.len(),
            )
            .field("disclosure_count", &self.disclosures.len())
            .field("redaction", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .finish()
    }
}

impl<'de> Deserialize<'de> for AgentHarnessHookInvocationResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct AgentHarnessHookInvocationResultWire {
            contract_id: AgentHarnessHookContractId,
            contract_version: AgentHarnessHookContractVersion,
            hook_kind: AgentHarnessHookKind,
            workflow_id: WorkflowId,
            workflow_version: WorkflowVersion,
            run_id: WorkflowRunId,
            schema_version: SchemaVersion,
            spec_hash: SpecContentHash,
            actor: ActorId,
            invoked_at: Timestamp,
            correlation_id: Option<CorrelationId>,
            step_id: Option<StepId>,
            phase_id: Option<String>,
            status: AgentHarnessHookInvocationStatus,
            input_references: Vec<AgentHarnessHookNamedReference>,
            output_references: Vec<AgentHarnessHookNamedReference>,
            supplemental_references: Vec<AgentHarnessHookReference>,
            disclosures: Vec<AgentHarnessHookDisclosure>,
            redaction: RedactionMetadata,
            sensitivity: WorkReportSensitivity,
        }

        let wire = AgentHarnessHookInvocationResultWire::deserialize(deserializer)?;
        Self::new(AgentHarnessHookInvocationResultDefinition {
            contract_id: wire.contract_id,
            contract_version: wire.contract_version,
            hook_kind: wire.hook_kind,
            workflow_id: wire.workflow_id,
            workflow_version: wire.workflow_version,
            run_id: wire.run_id,
            schema_version: wire.schema_version,
            spec_hash: wire.spec_hash,
            actor: wire.actor,
            invoked_at: wire.invoked_at,
            correlation_id: wire.correlation_id,
            step_id: wire.step_id,
            phase_id: wire.phase_id,
            status: wire.status,
            input_references: wire.input_references,
            output_references: wire.output_references,
            supplemental_references: wire.supplemental_references,
            disclosures: wire.disclosures,
            redaction: wire.redaction,
            sensitivity: wire.sensitivity,
        })
        .map_err(serde::de::Error::custom)
    }
}

/// Validates an explicit in-memory agent harness hook invocation.
///
/// # Errors
///
/// Returns a stable non-leaking error when invocation context is incomplete,
/// unsafe, or inconsistent with the hook contract.
pub fn invoke_agent_harness_hook(
    input: AgentHarnessHookInvocationInput,
) -> Result<AgentHarnessHookInvocationResult, WorkflowOsError> {
    input.contract.validate()?;

    if input.hook_kind != input.contract.hook_kind() {
        return Err(validation_error(
            "agent_harness_hook_invocation.kind.mismatch",
            "agent harness hook invocation kind must match the hook contract",
        ));
    }

    if input.side_effect_requested {
        return Err(validation_error(
            "agent_harness_hook_invocation.side_effect.unsupported",
            "agent harness hook invocation cannot request side effects",
        ));
    }

    validate_invocation_reference("workflow id", input.workflow_id.as_str())?;
    validate_invocation_reference("workflow version", input.workflow_version.as_str())?;
    validate_invocation_reference("run id", input.run_id.as_str())?;
    validate_invocation_reference("schema version", input.schema_version.as_str())?;
    validate_invocation_reference("spec hash", input.spec_hash.as_str())?;
    validate_invocation_reference("actor", input.actor.as_str())?;
    if let Some(correlation_id) = &input.correlation_id {
        validate_invocation_reference("correlation id", correlation_id.as_str())?;
    }
    if let Some(step_id) = &input.step_id {
        validate_invocation_reference("step id", step_id.as_str())?;
    }
    if let Some(phase_id) = &input.phase_id {
        validate_invocation_identifier("phase id", phase_id)?;
    }

    validate_named_invocation_references(
        "agent_harness_hook_invocation.inputs.duplicate",
        "agent harness hook invocation cannot contain duplicate input reference names",
        &input.input_references,
    )?;
    validate_named_invocation_references(
        "agent_harness_hook_invocation.outputs.duplicate",
        "agent harness hook invocation cannot contain duplicate output reference names",
        &input.output_references,
    )?;
    for reference in &input.supplemental_references {
        reference.validate()?;
    }
    for disclosure in &input.disclosures {
        disclosure.validate()?;
    }
    validate_invocation_redaction_metadata(&input.redaction)?;

    validate_required_hook_references(
        "agent_harness_hook_invocation.inputs.missing_required",
        "agent harness hook invocation is missing a required input reference",
        input.contract.input_requirements(),
        &input.input_references,
    )?;

    if input.require_outputs {
        validate_required_hook_references(
            "agent_harness_hook_invocation.outputs.missing_required",
            "agent harness hook invocation is missing a required output reference",
            input.contract.output_requirements(),
            &input.output_references,
        )?;
    }

    AgentHarnessHookInvocationResult::new(AgentHarnessHookInvocationResultDefinition {
        contract_id: input.contract.contract_id().clone(),
        contract_version: input.contract.contract_version().clone(),
        hook_kind: input.hook_kind,
        workflow_id: input.workflow_id,
        workflow_version: input.workflow_version,
        run_id: input.run_id,
        schema_version: input.schema_version,
        spec_hash: input.spec_hash,
        actor: input.actor,
        invoked_at: input.invoked_at,
        correlation_id: input.correlation_id,
        step_id: input.step_id,
        phase_id: input.phase_id,
        status: AgentHarnessHookInvocationStatus::Passed,
        input_references: input.input_references,
        output_references: input.output_references,
        supplemental_references: input.supplemental_references,
        disclosures: input.disclosures,
        redaction: input.redaction,
        sensitivity: input.sensitivity,
    })
}

/// Model-only audit record for a validated agent harness hook invocation.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct AgentHarnessHookAuditRecord {
    hook_invocation_id: AgentHarnessHookInvocationId,
    contract_id: AgentHarnessHookContractId,
    contract_version: AgentHarnessHookContractVersion,
    hook_kind: AgentHarnessHookKind,
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    run_id: WorkflowRunId,
    schema_version: SchemaVersion,
    spec_hash: SpecContentHash,
    actor: ActorId,
    invoked_at: Timestamp,
    correlation_id: Option<CorrelationId>,
    step_id: Option<StepId>,
    phase_id: Option<String>,
    status: AgentHarnessHookInvocationStatus,
    input_references: Vec<AgentHarnessHookNamedReference>,
    output_references: Vec<AgentHarnessHookNamedReference>,
    supplemental_references: Vec<AgentHarnessHookReference>,
    disclosures: Vec<AgentHarnessHookDisclosure>,
    redaction: RedactionMetadata,
    sensitivity: WorkReportSensitivity,
}

/// Input fields for constructing a validated `AgentHarnessHookAuditRecord`.
pub struct AgentHarnessHookAuditRecordDefinition {
    /// Stable hook invocation ID.
    pub hook_invocation_id: AgentHarnessHookInvocationId,
    /// Hook contract ID.
    pub contract_id: AgentHarnessHookContractId,
    /// Hook contract version.
    pub contract_version: AgentHarnessHookContractVersion,
    /// Hook kind.
    pub hook_kind: AgentHarnessHookKind,
    /// Workflow ID.
    pub workflow_id: WorkflowId,
    /// Workflow version.
    pub workflow_version: WorkflowVersion,
    /// Workflow run ID.
    pub run_id: WorkflowRunId,
    /// Schema version.
    pub schema_version: SchemaVersion,
    /// Workflow spec hash.
    pub spec_hash: SpecContentHash,
    /// Actor or system actor.
    pub actor: ActorId,
    /// Invocation timestamp.
    pub invoked_at: Timestamp,
    /// Optional correlation ID.
    pub correlation_id: Option<CorrelationId>,
    /// Optional step ID.
    pub step_id: Option<StepId>,
    /// Optional phase ID.
    pub phase_id: Option<String>,
    /// Invocation status.
    pub status: AgentHarnessHookInvocationStatus,
    /// Validated input references.
    pub input_references: Vec<AgentHarnessHookNamedReference>,
    /// Validated output references.
    pub output_references: Vec<AgentHarnessHookNamedReference>,
    /// Validated supplemental references.
    pub supplemental_references: Vec<AgentHarnessHookReference>,
    /// Bounded disclosures.
    pub disclosures: Vec<AgentHarnessHookDisclosure>,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
    /// Sensitivity classification.
    pub sensitivity: WorkReportSensitivity,
}

impl AgentHarnessHookAuditRecord {
    /// Creates a validated model-only hook audit record.
    ///
    /// # Errors
    ///
    /// Returns an error when the audit record shape is invalid or contains
    /// unsafe caller-supplied metadata.
    pub fn new(definition: AgentHarnessHookAuditRecordDefinition) -> Result<Self, WorkflowOsError> {
        let record = Self {
            hook_invocation_id: definition.hook_invocation_id,
            contract_id: definition.contract_id,
            contract_version: definition.contract_version,
            hook_kind: definition.hook_kind,
            workflow_id: definition.workflow_id,
            workflow_version: definition.workflow_version,
            run_id: definition.run_id,
            schema_version: definition.schema_version,
            spec_hash: definition.spec_hash,
            actor: definition.actor,
            invoked_at: definition.invoked_at,
            correlation_id: definition.correlation_id,
            step_id: definition.step_id,
            phase_id: definition.phase_id,
            status: definition.status,
            input_references: definition.input_references,
            output_references: definition.output_references,
            supplemental_references: definition.supplemental_references,
            disclosures: definition.disclosures,
            redaction: definition.redaction,
            sensitivity: definition.sensitivity,
        };
        record.validate()?;
        Ok(record)
    }

    /// Builds a model-only audit record from a validated hook invocation
    /// result.
    ///
    /// # Errors
    ///
    /// Returns an error when the supplied ID or resulting audit record is
    /// invalid.
    pub fn from_invocation_result(
        hook_invocation_id: AgentHarnessHookInvocationId,
        result: AgentHarnessHookInvocationResult,
    ) -> Result<Self, WorkflowOsError> {
        Self::new(AgentHarnessHookAuditRecordDefinition {
            hook_invocation_id,
            contract_id: result.contract_id,
            contract_version: result.contract_version,
            hook_kind: result.hook_kind,
            workflow_id: result.workflow_id,
            workflow_version: result.workflow_version,
            run_id: result.run_id,
            schema_version: result.schema_version,
            spec_hash: result.spec_hash,
            actor: result.actor,
            invoked_at: result.invoked_at,
            correlation_id: result.correlation_id,
            step_id: result.step_id,
            phase_id: result.phase_id,
            status: result.status,
            input_references: result.input_references,
            output_references: result.output_references,
            supplemental_references: result.supplemental_references,
            disclosures: result.disclosures,
            redaction: result.redaction,
            sensitivity: result.sensitivity,
        })
    }

    /// Validates the audit record.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error when the record is
    /// invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_invocation_reference("hook invocation id", self.hook_invocation_id.as_str())?;
        validate_invocation_reference("hook contract id", self.contract_id.as_str())?;
        validate_invocation_reference("hook contract version", self.contract_version.as_str())?;
        validate_invocation_reference("workflow id", self.workflow_id.as_str())?;
        validate_invocation_reference("workflow version", self.workflow_version.as_str())?;
        validate_invocation_reference("run id", self.run_id.as_str())?;
        validate_invocation_reference("schema version", self.schema_version.as_str())?;
        validate_invocation_reference("spec hash", self.spec_hash.as_str())?;
        validate_invocation_reference("actor", self.actor.as_str())?;
        if let Some(correlation_id) = &self.correlation_id {
            validate_invocation_reference("correlation id", correlation_id.as_str())?;
        }
        if let Some(step_id) = &self.step_id {
            validate_invocation_reference("step id", step_id.as_str())?;
        }
        if let Some(phase_id) = &self.phase_id {
            validate_invocation_identifier("phase id", phase_id)?;
        }
        validate_named_invocation_references(
            "agent_harness_hook_audit.inputs.duplicate",
            "agent harness hook audit record cannot contain duplicate input reference names",
            &self.input_references,
        )?;
        validate_named_invocation_references(
            "agent_harness_hook_audit.outputs.duplicate",
            "agent harness hook audit record cannot contain duplicate output reference names",
            &self.output_references,
        )?;
        for reference in &self.supplemental_references {
            reference.validate()?;
        }
        for disclosure in &self.disclosures {
            disclosure.validate()?;
        }
        validate_invocation_redaction_metadata(&self.redaction)?;
        Ok(())
    }

    /// Returns the hook invocation ID.
    #[must_use]
    pub const fn hook_invocation_id(&self) -> &AgentHarnessHookInvocationId {
        &self.hook_invocation_id
    }

    /// Returns the hook contract ID.
    #[must_use]
    pub const fn contract_id(&self) -> &AgentHarnessHookContractId {
        &self.contract_id
    }

    /// Returns the hook contract version.
    #[must_use]
    pub const fn contract_version(&self) -> &AgentHarnessHookContractVersion {
        &self.contract_version
    }

    /// Returns the hook kind.
    #[must_use]
    pub const fn hook_kind(&self) -> AgentHarnessHookKind {
        self.hook_kind
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

    /// Returns the run ID.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the invocation status recorded for audit.
    #[must_use]
    pub const fn status(&self) -> AgentHarnessHookInvocationStatus {
        self.status
    }

    /// Returns input references.
    #[must_use]
    pub fn input_references(&self) -> &[AgentHarnessHookNamedReference] {
        &self.input_references
    }

    /// Returns output references.
    #[must_use]
    pub fn output_references(&self) -> &[AgentHarnessHookNamedReference] {
        &self.output_references
    }

    /// Returns supplemental references.
    #[must_use]
    pub fn supplemental_references(&self) -> &[AgentHarnessHookReference] {
        &self.supplemental_references
    }

    /// Returns disclosures.
    #[must_use]
    pub fn disclosures(&self) -> &[AgentHarnessHookDisclosure] {
        &self.disclosures
    }

    /// Returns sensitivity.
    #[must_use]
    pub const fn sensitivity(&self) -> WorkReportSensitivity {
        self.sensitivity
    }
}

impl fmt::Debug for AgentHarnessHookAuditRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentHarnessHookAuditRecord")
            .field("hook_invocation_id", &self.hook_invocation_id)
            .field("contract_id", &self.contract_id)
            .field("contract_version", &self.contract_version)
            .field("hook_kind", &self.hook_kind)
            .field("workflow_id", &"[REDACTED]")
            .field("workflow_version", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("schema_version", &"[REDACTED]")
            .field("spec_hash", &"[REDACTED]")
            .field("actor", &"[REDACTED]")
            .field("invoked_at", &self.invoked_at)
            .field("correlation_id", &"[REDACTED]")
            .field("step_id", &"[REDACTED]")
            .field("phase_id", &"[REDACTED]")
            .field("status", &self.status)
            .field("input_reference_count", &self.input_references.len())
            .field("output_reference_count", &self.output_references.len())
            .field(
                "supplemental_reference_count",
                &self.supplemental_references.len(),
            )
            .field("disclosure_count", &self.disclosures.len())
            .field("redaction", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .finish()
    }
}

impl<'de> Deserialize<'de> for AgentHarnessHookAuditRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct AgentHarnessHookAuditRecordWire {
            hook_invocation_id: AgentHarnessHookInvocationId,
            contract_id: AgentHarnessHookContractId,
            contract_version: AgentHarnessHookContractVersion,
            hook_kind: AgentHarnessHookKind,
            workflow_id: WorkflowId,
            workflow_version: WorkflowVersion,
            run_id: WorkflowRunId,
            schema_version: SchemaVersion,
            spec_hash: SpecContentHash,
            actor: ActorId,
            invoked_at: Timestamp,
            correlation_id: Option<CorrelationId>,
            step_id: Option<StepId>,
            phase_id: Option<String>,
            status: AgentHarnessHookInvocationStatus,
            input_references: Vec<AgentHarnessHookNamedReference>,
            output_references: Vec<AgentHarnessHookNamedReference>,
            supplemental_references: Vec<AgentHarnessHookReference>,
            disclosures: Vec<AgentHarnessHookDisclosure>,
            redaction: RedactionMetadata,
            sensitivity: WorkReportSensitivity,
        }

        let wire = AgentHarnessHookAuditRecordWire::deserialize(deserializer)?;
        Self::new(AgentHarnessHookAuditRecordDefinition {
            hook_invocation_id: wire.hook_invocation_id,
            contract_id: wire.contract_id,
            contract_version: wire.contract_version,
            hook_kind: wire.hook_kind,
            workflow_id: wire.workflow_id,
            workflow_version: wire.workflow_version,
            run_id: wire.run_id,
            schema_version: wire.schema_version,
            spec_hash: wire.spec_hash,
            actor: wire.actor,
            invoked_at: wire.invoked_at,
            correlation_id: wire.correlation_id,
            step_id: wire.step_id,
            phase_id: wire.phase_id,
            status: wire.status,
            input_references: wire.input_references,
            output_references: wire.output_references,
            supplemental_references: wire.supplemental_references,
            disclosures: wire.disclosures,
            redaction: wire.redaction,
            sensitivity: wire.sensitivity,
        })
        .map_err(serde::de::Error::custom)
    }
}

/// Explicit input for in-memory runtime hook execution.
///
/// This input carries an already-validated hook invocation ID and the explicit
/// invocation context. It does not read runtime state, state backends, workflow
/// events, local checks, adapters, or external systems.
pub struct RuntimeAgentHarnessHookInput {
    /// Stable caller-supplied hook invocation ID.
    pub hook_invocation_id: AgentHarnessHookInvocationId,
    /// Explicit invocation context to validate and execute in memory.
    pub invocation: AgentHarnessHookInvocationInput,
}

/// In-memory runtime result for an agent harness hook checkpoint.
///
/// This result is not a workflow event, audit sink emission, persisted record,
/// report artifact, CLI output, command execution result, adapter result, or
/// local check result.
#[derive(Clone, Eq, PartialEq)]
pub struct RuntimeAgentHarnessHookResult {
    hook_invocation_id: AgentHarnessHookInvocationId,
    invocation_result: AgentHarnessHookInvocationResult,
    audit_record: AgentHarnessHookAuditRecord,
}

impl RuntimeAgentHarnessHookResult {
    /// Creates a runtime hook result from validated owned parts.
    ///
    /// # Errors
    ///
    /// Returns an error if the invocation ID does not match the audit record or
    /// the supplied records fail their existing validation boundaries.
    pub fn new(
        hook_invocation_id: AgentHarnessHookInvocationId,
        invocation_result: AgentHarnessHookInvocationResult,
        audit_record: AgentHarnessHookAuditRecord,
    ) -> Result<Self, WorkflowOsError> {
        invocation_result.validate()?;
        audit_record.validate()?;

        if audit_record.hook_invocation_id() != &hook_invocation_id {
            return Err(validation_error(
                "agent_harness_hook_runtime.invocation_id.mismatch",
                "runtime hook result invocation id must match the audit record",
            ));
        }

        if audit_record.contract_id() != invocation_result.contract_id()
            || audit_record.contract_version() != invocation_result.contract_version()
            || audit_record.hook_kind() != invocation_result.hook_kind()
            || audit_record.workflow_id() != invocation_result.workflow_id()
            || audit_record.workflow_version() != invocation_result.workflow_version()
            || audit_record.run_id() != invocation_result.run_id()
            || audit_record.status() != invocation_result.status()
        {
            return Err(validation_error(
                "agent_harness_hook_runtime.records.mismatch",
                "runtime hook result records must describe the same hook invocation",
            ));
        }

        Ok(Self {
            hook_invocation_id,
            invocation_result,
            audit_record,
        })
    }

    /// Returns the stable hook invocation ID.
    #[must_use]
    pub const fn hook_invocation_id(&self) -> &AgentHarnessHookInvocationId {
        &self.hook_invocation_id
    }

    /// Returns the validated in-memory invocation result.
    #[must_use]
    pub const fn invocation_result(&self) -> &AgentHarnessHookInvocationResult {
        &self.invocation_result
    }

    /// Returns the model-only in-memory hook audit record.
    #[must_use]
    pub const fn audit_record(&self) -> &AgentHarnessHookAuditRecord {
        &self.audit_record
    }

    /// Returns a `WorkReport` citation target for this hook invocation ID.
    #[must_use]
    pub fn report_citation_target(&self) -> WorkReportCitationTarget {
        WorkReportCitationTarget::AgentHarnessHook {
            hook_invocation_id: self.hook_invocation_id.clone(),
        }
    }

    /// Consumes the result into owned parts.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        AgentHarnessHookInvocationId,
        AgentHarnessHookInvocationResult,
        AgentHarnessHookAuditRecord,
    ) {
        (
            self.hook_invocation_id,
            self.invocation_result,
            self.audit_record,
        )
    }
}

impl fmt::Debug for RuntimeAgentHarnessHookResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RuntimeAgentHarnessHookResult")
            .field("hook_invocation_id", &self.hook_invocation_id)
            .field("contract_id", &self.invocation_result.contract_id())
            .field(
                "contract_version",
                &self.invocation_result.contract_version(),
            )
            .field("hook_kind", &self.invocation_result.hook_kind())
            .field("workflow_id", &"[REDACTED]")
            .field("workflow_version", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("status", &self.invocation_result.status())
            .field(
                "input_reference_count",
                &self.invocation_result.input_references().len(),
            )
            .field(
                "output_reference_count",
                &self.invocation_result.output_references().len(),
            )
            .field(
                "supplemental_reference_count",
                &self.invocation_result.supplemental_references().len(),
            )
            .field(
                "disclosure_count",
                &self.invocation_result.disclosures().len(),
            )
            .field("audit_record", &"[IN_MEMORY]")
            .finish()
    }
}

/// Executes an explicit agent harness hook checkpoint in memory.
///
/// # Errors
///
/// Returns a stable non-leaking error when the invocation context is invalid,
/// side effects are requested, required references are missing, or the
/// resulting audit record cannot be constructed.
pub fn execute_runtime_agent_harness_hook(
    input: RuntimeAgentHarnessHookInput,
) -> Result<RuntimeAgentHarnessHookResult, WorkflowOsError> {
    let RuntimeAgentHarnessHookInput {
        hook_invocation_id,
        invocation,
    } = input;

    let invocation_result = invoke_agent_harness_hook(invocation)?;
    let audit_record = AgentHarnessHookAuditRecord::from_invocation_result(
        hook_invocation_id.clone(),
        invocation_result.clone(),
    )?;

    RuntimeAgentHarnessHookResult::new(hook_invocation_id, invocation_result, audit_record)
}

fn validate_named_requirements<T>(
    required_code: &'static str,
    required_message: &'static str,
    values: &[T],
    validate: fn(&T) -> Result<(), WorkflowOsError>,
    name: fn(&T) -> &str,
) -> Result<(), WorkflowOsError> {
    if values.is_empty() {
        return Err(validation_error(required_code, required_message));
    }

    let mut seen = BTreeSet::new();
    for value in values {
        validate(value)?;
        if !seen.insert(name(value).to_owned()) {
            return Err(validation_error(
                "harness_contract.requirements.duplicate",
                "harness contracts cannot declare duplicate requirement names",
            ));
        }
    }

    Ok(())
}

fn validate_tool_allowances(values: &[HarnessToolAllowance]) -> Result<(), WorkflowOsError> {
    let mut seen = BTreeSet::new();
    for value in values {
        value.validate()?;
        if !seen.insert((value.kind, value.name.clone())) {
            return Err(validation_error(
                "harness_contract.tools.duplicate",
                "harness contracts cannot declare duplicate tool allowances",
            ));
        }
    }
    Ok(())
}

fn validate_hook_named_requirements<T>(
    required_code: &'static str,
    required_message: &'static str,
    duplicate_code: &'static str,
    duplicate_message: &'static str,
    values: &[T],
    validate: fn(&T) -> Result<(), WorkflowOsError>,
    name: fn(&T) -> &str,
) -> Result<(), WorkflowOsError> {
    if values.is_empty() {
        return Err(validation_error(required_code, required_message));
    }

    let mut seen = BTreeSet::new();
    for value in values {
        validate(value)?;
        if !seen.insert(name(value).to_owned()) {
            return Err(validation_error(duplicate_code, duplicate_message));
        }
    }

    Ok(())
}

trait AgentHarnessHookRequirementView {
    fn name(&self) -> &str;
    fn required(&self) -> bool;
}

impl AgentHarnessHookRequirementView for AgentHarnessHookInputRequirement {
    fn name(&self) -> &str {
        self.name()
    }

    fn required(&self) -> bool {
        self.required()
    }
}

impl AgentHarnessHookRequirementView for AgentHarnessHookOutputRequirement {
    fn name(&self) -> &str {
        self.name()
    }

    fn required(&self) -> bool {
        self.required()
    }
}

fn validate_named_invocation_references(
    duplicate_code: &'static str,
    duplicate_message: &'static str,
    values: &[AgentHarnessHookNamedReference],
) -> Result<(), WorkflowOsError> {
    let mut seen = BTreeSet::new();
    for value in values {
        value.validate()?;
        if !seen.insert(value.name().to_owned()) {
            return Err(validation_error(duplicate_code, duplicate_message));
        }
    }
    Ok(())
}

fn validate_required_hook_references<T>(
    missing_code: &'static str,
    missing_message: &'static str,
    requirements: &[T],
    references: &[AgentHarnessHookNamedReference],
) -> Result<(), WorkflowOsError>
where
    T: AgentHarnessHookRequirementView,
{
    let supplied = references
        .iter()
        .map(AgentHarnessHookNamedReference::name)
        .collect::<BTreeSet<_>>();

    for requirement in requirements {
        if requirement.required() && !supplied.contains(requirement.name()) {
            return Err(validation_error(missing_code, missing_message));
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
    T: Copy + Ord,
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
            "harness_contract.identifier.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > HARNESS_IDENTIFIER_MAX_BYTES {
        return Err(validation_error(
            "harness_contract.identifier.too_long",
            format!("{type_name} cannot exceed {HARNESS_IDENTIFIER_MAX_BYTES} bytes"),
        ));
    }

    let is_valid = value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'));

    if !is_valid {
        return Err(validation_error(
            "harness_contract.identifier.invalid_character",
            format!("{type_name} contains an invalid character"),
        ));
    }

    validate_not_secret_like(type_name, value)
}

fn validate_hook_identifier(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "agent_harness_hook.identifier.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > HARNESS_IDENTIFIER_MAX_BYTES {
        return Err(validation_error(
            "agent_harness_hook.identifier.too_long",
            format!("{type_name} cannot exceed {HARNESS_IDENTIFIER_MAX_BYTES} bytes"),
        ));
    }

    let is_valid = value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'));

    if !is_valid {
        return Err(validation_error(
            "agent_harness_hook.identifier.invalid_character",
            format!("{type_name} contains an invalid character"),
        ));
    }

    validate_hook_not_secret_like(type_name, value)
}

fn validate_invocation_identifier(
    type_name: &'static str,
    value: &str,
) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "agent_harness_hook_invocation.identifier.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > HARNESS_IDENTIFIER_MAX_BYTES {
        return Err(validation_error(
            "agent_harness_hook_invocation.identifier.too_long",
            format!("{type_name} cannot exceed {HARNESS_IDENTIFIER_MAX_BYTES} bytes"),
        ));
    }

    let is_valid = value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'));

    if !is_valid {
        return Err(validation_error(
            "agent_harness_hook_invocation.identifier.invalid_character",
            format!("{type_name} contains an invalid character"),
        ));
    }

    validate_invocation_not_secret_like(type_name, value)
}

fn validate_invocation_reference(
    type_name: &'static str,
    value: &str,
) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "agent_harness_hook_invocation.reference.empty",
            format!("{type_name} reference cannot be empty"),
        ));
    }

    if value.len() > HARNESS_TEXT_MAX_BYTES {
        return Err(validation_error(
            "agent_harness_hook_invocation.reference.too_long",
            format!("{type_name} reference cannot exceed {HARNESS_TEXT_MAX_BYTES} bytes"),
        ));
    }

    validate_invocation_not_secret_like(type_name, value)
}

fn validate_text(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "harness_contract.text.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > HARNESS_TEXT_MAX_BYTES {
        return Err(validation_error(
            "harness_contract.text.too_long",
            format!("{type_name} cannot exceed {HARNESS_TEXT_MAX_BYTES} bytes"),
        ));
    }

    validate_not_secret_like(type_name, value)
}

fn validate_hook_text(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "agent_harness_hook.text.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > HARNESS_TEXT_MAX_BYTES {
        return Err(validation_error(
            "agent_harness_hook.text.too_long",
            format!("{type_name} cannot exceed {HARNESS_TEXT_MAX_BYTES} bytes"),
        ));
    }

    validate_hook_not_secret_like(type_name, value)
}

fn validate_invocation_text(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "agent_harness_hook_invocation.text.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > HARNESS_TEXT_MAX_BYTES {
        return Err(validation_error(
            "agent_harness_hook_invocation.text.too_long",
            format!("{type_name} cannot exceed {HARNESS_TEXT_MAX_BYTES} bytes"),
        ));
    }

    validate_invocation_not_secret_like(type_name, value)
}

fn validate_redaction_metadata(redaction: &RedactionMetadata) -> Result<(), WorkflowOsError> {
    if redaction.redacted_fields.len() > HARNESS_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "harness_contract.redaction.too_many_fields",
            "harness contract redaction metadata contains too many fields",
        ));
    }

    if redaction.field_states.len() > HARNESS_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "harness_contract.redaction.too_many_states",
            "harness contract redaction metadata contains too many field states",
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

fn validate_invocation_redaction_metadata(
    redaction: &RedactionMetadata,
) -> Result<(), WorkflowOsError> {
    if redaction.redacted_fields.len() > HARNESS_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "agent_harness_hook_invocation.redaction.too_many_fields",
            "agent harness hook invocation redaction metadata contains too many fields",
        ));
    }

    if redaction.field_states.len() > HARNESS_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "agent_harness_hook_invocation.redaction.too_many_states",
            "agent harness hook invocation redaction metadata contains too many field states",
        ));
    }

    for field in &redaction.redacted_fields {
        validate_invocation_redaction_field_name(field)?;
    }

    for state in &redaction.field_states {
        validate_invocation_redaction_field_name(&state.field)?;
        validate_invocation_redaction_reason(&state.reason)?;
    }

    Ok(())
}

fn validate_hook_redaction_metadata(redaction: &RedactionMetadata) -> Result<(), WorkflowOsError> {
    if redaction.redacted_fields.len() > HARNESS_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "agent_harness_hook.redaction.too_many_fields",
            "agent harness hook redaction metadata contains too many fields",
        ));
    }

    if redaction.field_states.len() > HARNESS_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "agent_harness_hook.redaction.too_many_states",
            "agent harness hook redaction metadata contains too many field states",
        ));
    }

    for field in &redaction.redacted_fields {
        validate_hook_redaction_field_name(field)?;
    }

    for state in &redaction.field_states {
        validate_hook_redaction_field_name(&state.field)?;
        validate_hook_redaction_reason(&state.reason)?;
    }

    Ok(())
}

fn validate_redaction_field_name(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "harness_contract.redaction.field.empty",
            "harness contract redaction field cannot be empty",
        ));
    }

    if value.len() > HARNESS_REDACTION_FIELD_MAX_BYTES {
        return Err(validation_error(
            "harness_contract.redaction.field.too_long",
            format!(
                "harness contract redaction field cannot exceed {HARNESS_REDACTION_FIELD_MAX_BYTES} bytes"
            ),
        ));
    }

    validate_not_secret_like("harness contract redaction field", value)
}

fn validate_redaction_reason(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "harness_contract.redaction.reason.empty",
            "harness contract redaction reason cannot be empty",
        ));
    }

    if value.len() > HARNESS_REDACTION_REASON_MAX_BYTES {
        return Err(validation_error(
            "harness_contract.redaction.reason.too_long",
            format!(
                "harness contract redaction reason cannot exceed {HARNESS_REDACTION_REASON_MAX_BYTES} bytes"
            ),
        ));
    }

    validate_not_secret_like("harness contract redaction reason", value)
}

fn validate_hook_redaction_field_name(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "agent_harness_hook.redaction.field.empty",
            "agent harness hook redaction field cannot be empty",
        ));
    }

    if value.len() > HARNESS_REDACTION_FIELD_MAX_BYTES {
        return Err(validation_error(
            "agent_harness_hook.redaction.field.too_long",
            format!(
                "agent harness hook redaction field cannot exceed {HARNESS_REDACTION_FIELD_MAX_BYTES} bytes"
            ),
        ));
    }

    validate_hook_not_secret_like("agent harness hook redaction field", value)
}

fn validate_hook_redaction_reason(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "agent_harness_hook.redaction.reason.empty",
            "agent harness hook redaction reason cannot be empty",
        ));
    }

    if value.len() > HARNESS_REDACTION_REASON_MAX_BYTES {
        return Err(validation_error(
            "agent_harness_hook.redaction.reason.too_long",
            format!(
                "agent harness hook redaction reason cannot exceed {HARNESS_REDACTION_REASON_MAX_BYTES} bytes"
            ),
        ));
    }

    validate_hook_not_secret_like("agent harness hook redaction reason", value)
}

fn validate_invocation_redaction_field_name(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "agent_harness_hook_invocation.redaction.field.empty",
            "agent harness hook invocation redaction field cannot be empty",
        ));
    }

    if value.len() > HARNESS_REDACTION_FIELD_MAX_BYTES {
        return Err(validation_error(
            "agent_harness_hook_invocation.redaction.field.too_long",
            format!(
                "agent harness hook invocation redaction field cannot exceed {HARNESS_REDACTION_FIELD_MAX_BYTES} bytes"
            ),
        ));
    }

    validate_invocation_not_secret_like("agent harness hook invocation redaction field", value)
}

fn validate_invocation_redaction_reason(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "agent_harness_hook_invocation.redaction.reason.empty",
            "agent harness hook invocation redaction reason cannot be empty",
        ));
    }

    if value.len() > HARNESS_REDACTION_REASON_MAX_BYTES {
        return Err(validation_error(
            "agent_harness_hook_invocation.redaction.reason.too_long",
            format!(
                "agent harness hook invocation redaction reason cannot exceed {HARNESS_REDACTION_REASON_MAX_BYTES} bytes"
            ),
        ));
    }

    validate_invocation_not_secret_like("agent harness hook invocation redaction reason", value)
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
            "harness_contract.secret_like_identifier",
            format!("{type_name} contains sensitive-looking text"),
        ));
    }

    Ok(())
}

fn validate_invocation_not_secret_like(
    type_name: &'static str,
    value: &str,
) -> Result<(), WorkflowOsError> {
    let lowercase = value.to_ascii_lowercase();
    let is_secret_like = lowercase.contains("authorization")
        || lowercase.contains("bearer")
        || lowercase.contains("private_key")
        || lowercase.contains("private-key")
        || lowercase.contains("api_token")
        || lowercase.contains("api-token")
        || lowercase.contains("raw_prompt")
        || lowercase.contains("raw prompt")
        || lowercase.contains("raw_provider_payload")
        || lowercase.contains("raw provider")
        || lowercase.contains("raw_command_output")
        || lowercase.contains("raw command")
        || lowercase.contains("raw_spec_contents")
        || lowercase.contains("raw spec")
        || lowercase.contains("raw_parser_payload")
        || lowercase.contains("parser payload")
        || lowercase.contains("raw_ci_log")
        || lowercase.contains("raw ci")
        || lowercase.contains("jira body")
        || lowercase.contains("github file")
        || lowercase.contains("environment variable")
        || lowercase.contains("secret")
        || lowercase.contains("token");

    if is_secret_like {
        return Err(validation_error(
            "agent_harness_hook_invocation.secret_like_value",
            format!("{type_name} contains sensitive-looking text"),
        ));
    }

    Ok(())
}

fn validate_hook_not_secret_like(
    type_name: &'static str,
    value: &str,
) -> Result<(), WorkflowOsError> {
    let lowercase = value.to_ascii_lowercase();
    let is_secret_like = lowercase.contains("authorization")
        || lowercase.contains("bearer")
        || lowercase.contains("private_key")
        || lowercase.contains("private-key")
        || lowercase.contains("api_token")
        || lowercase.contains("api-token")
        || lowercase.contains("raw_provider_payload")
        || lowercase.contains("raw provider")
        || lowercase.contains("raw_command_output")
        || lowercase.contains("raw command")
        || lowercase.contains("raw_spec_contents")
        || lowercase.contains("raw spec")
        || lowercase.contains("raw_parser_payload")
        || lowercase.contains("parser payload")
        || lowercase.contains("environment variable")
        || lowercase.contains("secret")
        || lowercase.contains("token");

    if is_secret_like {
        return Err(validation_error(
            "agent_harness_hook.secret_like_value",
            format!("{type_name} contains sensitive-looking text"),
        ));
    }

    Ok(())
}

fn validation_error(code: &'static str, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}
