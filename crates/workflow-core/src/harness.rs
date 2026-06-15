use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    RedactionMetadata, SchemaVersion, WorkReportRedactionPolicy, WorkReportSensitivity,
    WorkflowOsError,
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

fn validation_error(code: &'static str, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}
