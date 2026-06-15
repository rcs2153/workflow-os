use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ApprovalReferenceId, EventId, EvidenceReferenceId, HarnessContractId, RedactionMetadata,
    SchemaVersion, ValidationReferenceId, WorkReportId, WorkReportRedactionPolicy,
    WorkReportSensitivity, WorkReportStableReference, WorkflowId, WorkflowOsError, WorkflowRunId,
};

const HANDOFF_IDENTIFIER_MAX_BYTES: usize = 128;
const HANDOFF_TEXT_MAX_BYTES: usize = 1_000;
const HANDOFF_REDACTION_FIELD_MAX_BYTES: usize = 128;
const HANDOFF_REDACTION_REASON_MAX_BYTES: usize = 512;
const HANDOFF_REDACTION_MAX_ENTRIES: usize = 64;

/// Identifier for a typed handoff value.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct TypedHandoffId(String);

impl TypedHandoffId {
    /// Creates a validated typed handoff ID.
    ///
    /// # Errors
    ///
    /// Returns an error when the ID is empty, too long, contains unsupported
    /// characters, or looks like a secret.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("TypedHandoffId", &value)?;
        Ok(Self(value))
    }

    /// Returns the ID as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TypedHandoffId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for TypedHandoffId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("TypedHandoffId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<TypedHandoffId> for String {
    fn from(value: TypedHandoffId) -> Self {
        value.0
    }
}

impl TryFrom<String> for TypedHandoffId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for TypedHandoffId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Identifier for a typed handoff contract.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct TypedHandoffContractId(String);

impl TypedHandoffContractId {
    /// Creates a validated typed handoff contract ID.
    ///
    /// # Errors
    ///
    /// Returns an error when the ID is invalid or secret-like.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("TypedHandoffContractId", &value)?;
        Ok(Self(value))
    }

    /// Returns the ID as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TypedHandoffContractId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for TypedHandoffContractId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("TypedHandoffContractId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<TypedHandoffContractId> for String {
    fn from(value: TypedHandoffContractId) -> Self {
        value.0
    }
}

impl TryFrom<String> for TypedHandoffContractId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for TypedHandoffContractId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Version for a typed handoff contract.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct TypedHandoffContractVersion(String);

impl TypedHandoffContractVersion {
    /// Creates a validated typed handoff contract version.
    ///
    /// # Errors
    ///
    /// Returns an error when the version is invalid or secret-like.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("TypedHandoffContractVersion", &value)?;
        Ok(Self(value))
    }

    /// Returns the version as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TypedHandoffContractVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for TypedHandoffContractVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("TypedHandoffContractVersion")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<TypedHandoffContractVersion> for String {
    fn from(value: TypedHandoffContractVersion) -> Self {
        value.0
    }
}

impl TryFrom<String> for TypedHandoffContractVersion {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for TypedHandoffContractVersion {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Endpoint kind for a typed handoff.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypedHandoffEndpointKind {
    /// A future harness boundary.
    Harness,
    /// A workflow step.
    WorkflowStep,
    /// A named workflow phase.
    WorkflowPhase,
    /// A generated work report boundary.
    WorkReport,
    /// An operator or manual review boundary.
    OperatorReview,
}

/// Lifecycle status for a typed handoff value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypedHandoffStatus {
    /// Handoff is proposed but not yet ready.
    Proposed,
    /// Handoff is ready for the target boundary.
    Ready,
    /// Handoff was accepted by the target boundary.
    Accepted,
    /// Handoff was rejected by the target boundary.
    Rejected,
    /// Handoff is blocked and needs external resolution.
    Blocked,
    /// Handoff was superseded by another handoff.
    Superseded,
    /// Handoff is explicitly incomplete.
    Incomplete,
}

/// Failure semantics declared by a typed handoff contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypedHandoffFailureSemantics {
    /// Block the target boundary.
    BlockTarget,
    /// Reject the handoff.
    RejectHandoff,
    /// Require approval before acceptance.
    RequireApproval,
    /// Accept partial output with explicit disclosure.
    AcceptPartialWithDisclosure,
    /// Escalate to an operator.
    Escalate,
}

/// Bounded endpoint reference for typed handoffs.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypedHandoffEndpoint {
    name: String,
    kind: TypedHandoffEndpointKind,
    reference: WorkReportStableReference,
}

impl TypedHandoffEndpoint {
    /// Creates a validated typed handoff endpoint.
    ///
    /// # Errors
    ///
    /// Returns an error when the endpoint name is invalid.
    pub fn new(
        name: impl Into<String>,
        kind: TypedHandoffEndpointKind,
        reference: WorkReportStableReference,
    ) -> Result<Self, WorkflowOsError> {
        let endpoint = Self {
            name: name.into(),
            kind,
            reference,
        };
        endpoint.validate()?;
        Ok(endpoint)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_identifier("typed handoff endpoint name", &self.name)
    }

    /// Returns the endpoint name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the endpoint kind.
    #[must_use]
    pub const fn kind(&self) -> TypedHandoffEndpointKind {
        self.kind
    }

    /// Returns the endpoint stable reference.
    #[must_use]
    pub const fn reference(&self) -> &WorkReportStableReference {
        &self.reference
    }
}

impl fmt::Debug for TypedHandoffEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypedHandoffEndpoint")
            .field("name", &"[REDACTED]")
            .field("kind", &self.kind)
            .field("reference", &self.reference)
            .finish()
    }
}

/// Stable target carried by a typed handoff reference.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TypedHandoffReferenceTarget {
    /// Generic stable input reference.
    Input {
        /// Stable reference.
        reference: WorkReportStableReference,
    },
    /// Generic stable output reference.
    Output {
        /// Stable reference.
        reference: WorkReportStableReference,
    },
    /// Evidence reference ID.
    EvidenceReference {
        /// Evidence reference ID.
        evidence_reference_id: EvidenceReferenceId,
    },
    /// Validation reference ID.
    ValidationReference {
        /// Validation reference ID.
        validation_reference_id: ValidationReferenceId,
    },
    /// Local check result reference.
    LocalCheckResult {
        /// Stable local check result reference.
        reference: WorkReportStableReference,
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
    /// Policy decision event ID.
    PolicyDecision {
        /// Policy decision event ID.
        event_id: EventId,
    },
    /// Approval decision reference ID.
    ApprovalDecision {
        /// Approval reference ID.
        approval_reference_id: ApprovalReferenceId,
    },
    /// Work report ID.
    WorkReport {
        /// Work report ID.
        work_report_id: WorkReportId,
    },
    /// Adapter telemetry stable reference.
    AdapterTelemetry {
        /// Stable adapter telemetry reference.
        reference: WorkReportStableReference,
    },
}

impl TypedHandoffReferenceTarget {
    /// Returns a stable kind label for duplicate checking and debugging.
    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        match self {
            Self::Input { .. } => "input",
            Self::Output { .. } => "output",
            Self::EvidenceReference { .. } => "evidence_reference",
            Self::ValidationReference { .. } => "validation_reference",
            Self::LocalCheckResult { .. } => "local_check_result",
            Self::WorkflowEvent { .. } => "workflow_event",
            Self::AuditEvent { .. } => "audit_event",
            Self::PolicyDecision { .. } => "policy_decision",
            Self::ApprovalDecision { .. } => "approval_decision",
            Self::WorkReport { .. } => "work_report",
            Self::AdapterTelemetry { .. } => "adapter_telemetry",
        }
    }
}

impl fmt::Debug for TypedHandoffReferenceTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypedHandoffReferenceTarget")
            .field("kind", &self.kind_name())
            .field("reference", &"[REDACTED]")
            .finish()
    }
}

/// Reference carried by a typed handoff.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypedHandoffReference {
    name: String,
    target: TypedHandoffReferenceTarget,
    required: bool,
}

impl TypedHandoffReference {
    /// Creates a validated typed handoff reference.
    ///
    /// # Errors
    ///
    /// Returns an error when the reference name is invalid.
    pub fn new(
        name: impl Into<String>,
        target: TypedHandoffReferenceTarget,
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
        validate_identifier("typed handoff reference name", &self.name)
    }

    /// Returns the reference name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the target.
    #[must_use]
    pub const fn target(&self) -> &TypedHandoffReferenceTarget {
        &self.target
    }

    /// Returns whether the reference is required.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}

impl fmt::Debug for TypedHandoffReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypedHandoffReference")
            .field("name", &"[REDACTED]")
            .field("target", &self.target)
            .field("required", &self.required)
            .finish()
    }
}

/// Bounded text item carried by a typed handoff.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypedHandoffTextItem {
    name: String,
    text: String,
}

impl TypedHandoffTextItem {
    /// Creates a validated typed handoff text item.
    ///
    /// # Errors
    ///
    /// Returns an error when the name or text is invalid.
    pub fn new(name: impl Into<String>, text: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let item = Self {
            name: name.into(),
            text: text.into(),
        };
        item.validate()?;
        Ok(item)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_identifier("typed handoff text item name", &self.name)?;
        validate_text("typed handoff text item", &self.text)
    }

    /// Returns the item name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the bounded text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl fmt::Debug for TypedHandoffTextItem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypedHandoffTextItem")
            .field("name", &"[REDACTED]")
            .field("text", &"[REDACTED]")
            .finish()
    }
}

/// Domain-neutral typed handoff contract.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct TypedHandoffContract {
    contract_id: TypedHandoffContractId,
    contract_version: TypedHandoffContractVersion,
    schema_version: SchemaVersion,
    source_harness_contract_id: Option<HarnessContractId>,
    target_harness_contract_id: Option<HarnessContractId>,
    required_input_references: Vec<TypedHandoffReference>,
    required_output_references: Vec<TypedHandoffReference>,
    required_evidence_references: Vec<TypedHandoffReference>,
    required_validation_references: Vec<TypedHandoffReference>,
    required_obligations: Vec<TypedHandoffTextItem>,
    failure_semantics: Vec<TypedHandoffFailureSemantics>,
    sensitivity: WorkReportSensitivity,
    redaction_policy: WorkReportRedactionPolicy,
    redaction: RedactionMetadata,
}

/// Input fields for constructing a validated `TypedHandoffContract`.
pub struct TypedHandoffContractDefinition {
    /// Handoff contract ID.
    pub contract_id: TypedHandoffContractId,
    /// Handoff contract version.
    pub contract_version: TypedHandoffContractVersion,
    /// Schema version associated with the handoff contract model.
    pub schema_version: SchemaVersion,
    /// Optional source harness contract ID.
    pub source_harness_contract_id: Option<HarnessContractId>,
    /// Optional target harness contract ID.
    pub target_harness_contract_id: Option<HarnessContractId>,
    /// Required input references.
    pub required_input_references: Vec<TypedHandoffReference>,
    /// Required output references.
    pub required_output_references: Vec<TypedHandoffReference>,
    /// Required evidence references.
    pub required_evidence_references: Vec<TypedHandoffReference>,
    /// Required validation references.
    pub required_validation_references: Vec<TypedHandoffReference>,
    /// Required obligations.
    pub required_obligations: Vec<TypedHandoffTextItem>,
    /// Failure semantics.
    pub failure_semantics: Vec<TypedHandoffFailureSemantics>,
    /// Sensitivity classification.
    pub sensitivity: WorkReportSensitivity,
    /// Redaction policy.
    pub redaction_policy: WorkReportRedactionPolicy,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
}

impl TypedHandoffContract {
    /// Creates a validated typed handoff contract.
    ///
    /// # Errors
    ///
    /// Returns an error when required fields are missing, duplicated,
    /// unbounded, or secret-like.
    pub fn new(definition: TypedHandoffContractDefinition) -> Result<Self, WorkflowOsError> {
        let contract = Self {
            contract_id: definition.contract_id,
            contract_version: definition.contract_version,
            schema_version: definition.schema_version,
            source_harness_contract_id: definition.source_harness_contract_id,
            target_harness_contract_id: definition.target_harness_contract_id,
            required_input_references: definition.required_input_references,
            required_output_references: definition.required_output_references,
            required_evidence_references: definition.required_evidence_references,
            required_validation_references: definition.required_validation_references,
            required_obligations: definition.required_obligations,
            failure_semantics: definition.failure_semantics,
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
    /// Returns a stable non-leaking error when invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_not_secret_like("schema version", self.schema_version.as_str())?;
        validate_reference_list(
            "typed_handoff.contract.inputs.required",
            "typed handoff contracts require at least one input reference declaration",
            &self.required_input_references,
        )?;
        validate_reference_list(
            "typed_handoff.contract.outputs.required",
            "typed handoff contracts require at least one output reference declaration",
            &self.required_output_references,
        )?;
        validate_reference_list(
            "typed_handoff.contract.evidence.required",
            "typed handoff contracts require at least one evidence reference declaration",
            &self.required_evidence_references,
        )?;
        validate_reference_list(
            "typed_handoff.contract.validation.required",
            "typed handoff contracts require at least one validation reference declaration",
            &self.required_validation_references,
        )?;
        validate_text_item_list(
            "typed_handoff.contract.obligations.required",
            "typed handoff contracts require at least one obligation declaration",
            &self.required_obligations,
        )?;
        validate_enum_list(
            "typed_handoff.contract.failure.required",
            "typed handoff contracts require at least one failure semantic",
            "typed_handoff.contract.failure.duplicate",
            "typed handoff contracts cannot declare duplicate failure semantics",
            &self.failure_semantics,
        )?;
        validate_redaction_metadata(&self.redaction)?;
        Ok(())
    }

    /// Returns the contract ID.
    #[must_use]
    pub const fn contract_id(&self) -> &TypedHandoffContractId {
        &self.contract_id
    }

    /// Returns the contract version.
    #[must_use]
    pub const fn contract_version(&self) -> &TypedHandoffContractVersion {
        &self.contract_version
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> &SchemaVersion {
        &self.schema_version
    }

    /// Returns required input references.
    #[must_use]
    pub fn required_input_references(&self) -> &[TypedHandoffReference] {
        &self.required_input_references
    }

    /// Returns required output references.
    #[must_use]
    pub fn required_output_references(&self) -> &[TypedHandoffReference] {
        &self.required_output_references
    }

    /// Returns required evidence references.
    #[must_use]
    pub fn required_evidence_references(&self) -> &[TypedHandoffReference] {
        &self.required_evidence_references
    }

    /// Returns required validation references.
    #[must_use]
    pub fn required_validation_references(&self) -> &[TypedHandoffReference] {
        &self.required_validation_references
    }

    /// Returns required obligations.
    #[must_use]
    pub fn required_obligations(&self) -> &[TypedHandoffTextItem] {
        &self.required_obligations
    }

    /// Returns failure semantics.
    #[must_use]
    pub fn failure_semantics(&self) -> &[TypedHandoffFailureSemantics] {
        &self.failure_semantics
    }
}

impl fmt::Debug for TypedHandoffContract {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypedHandoffContract")
            .field("contract_id", &self.contract_id)
            .field("contract_version", &self.contract_version)
            .field("schema_version", &self.schema_version)
            .field(
                "source_harness_contract_id",
                &self
                    .source_harness_contract_id
                    .as_ref()
                    .map(|_| "[REDACTED]"),
            )
            .field(
                "target_harness_contract_id",
                &self
                    .target_harness_contract_id
                    .as_ref()
                    .map(|_| "[REDACTED]"),
            )
            .field(
                "input_reference_count",
                &self.required_input_references.len(),
            )
            .field(
                "output_reference_count",
                &self.required_output_references.len(),
            )
            .field(
                "evidence_reference_count",
                &self.required_evidence_references.len(),
            )
            .field(
                "validation_reference_count",
                &self.required_validation_references.len(),
            )
            .field("obligation_count", &self.required_obligations.len())
            .field("failure_semantic_count", &self.failure_semantics.len())
            .field("sensitivity", &self.sensitivity)
            .field("redaction_policy", &self.redaction_policy)
            .field("redaction", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for TypedHandoffContract {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TypedHandoffContractWire {
            contract_id: TypedHandoffContractId,
            contract_version: TypedHandoffContractVersion,
            schema_version: SchemaVersion,
            source_harness_contract_id: Option<HarnessContractId>,
            target_harness_contract_id: Option<HarnessContractId>,
            required_input_references: Vec<TypedHandoffReference>,
            required_output_references: Vec<TypedHandoffReference>,
            required_evidence_references: Vec<TypedHandoffReference>,
            required_validation_references: Vec<TypedHandoffReference>,
            required_obligations: Vec<TypedHandoffTextItem>,
            failure_semantics: Vec<TypedHandoffFailureSemantics>,
            sensitivity: WorkReportSensitivity,
            redaction_policy: WorkReportRedactionPolicy,
            redaction: RedactionMetadata,
        }

        let wire = TypedHandoffContractWire::deserialize(deserializer)?;
        Self::new(TypedHandoffContractDefinition {
            contract_id: wire.contract_id,
            contract_version: wire.contract_version,
            schema_version: wire.schema_version,
            source_harness_contract_id: wire.source_harness_contract_id,
            target_harness_contract_id: wire.target_harness_contract_id,
            required_input_references: wire.required_input_references,
            required_output_references: wire.required_output_references,
            required_evidence_references: wire.required_evidence_references,
            required_validation_references: wire.required_validation_references,
            required_obligations: wire.required_obligations,
            failure_semantics: wire.failure_semantics,
            sensitivity: wire.sensitivity,
            redaction_policy: wire.redaction_policy,
            redaction: wire.redaction,
        })
        .map_err(serde::de::Error::custom)
    }
}

/// Domain-neutral typed handoff value.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct TypedHandoff {
    handoff_id: TypedHandoffId,
    contract_id: TypedHandoffContractId,
    contract_version: TypedHandoffContractVersion,
    schema_version: SchemaVersion,
    workflow_id: Option<WorkflowId>,
    run_id: Option<WorkflowRunId>,
    status: TypedHandoffStatus,
    source: TypedHandoffEndpoint,
    target: TypedHandoffEndpoint,
    input_references: Vec<TypedHandoffReference>,
    output_references: Vec<TypedHandoffReference>,
    evidence_references: Vec<TypedHandoffReference>,
    validation_references: Vec<TypedHandoffReference>,
    audit_references: Vec<TypedHandoffReference>,
    policy_references: Vec<TypedHandoffReference>,
    approval_references: Vec<TypedHandoffReference>,
    work_report_references: Vec<TypedHandoffReference>,
    obligations: Vec<TypedHandoffTextItem>,
    disclosures: Vec<TypedHandoffTextItem>,
    risks: Vec<TypedHandoffTextItem>,
    notes: Vec<TypedHandoffTextItem>,
    sensitivity: WorkReportSensitivity,
    redaction_policy: WorkReportRedactionPolicy,
    redaction: RedactionMetadata,
}

/// Input fields for constructing a validated `TypedHandoff`.
pub struct TypedHandoffDefinition {
    /// Typed handoff ID.
    pub handoff_id: TypedHandoffId,
    /// Contract ID.
    pub contract_id: TypedHandoffContractId,
    /// Contract version.
    pub contract_version: TypedHandoffContractVersion,
    /// Schema version.
    pub schema_version: SchemaVersion,
    /// Optional workflow ID.
    pub workflow_id: Option<WorkflowId>,
    /// Optional run ID.
    pub run_id: Option<WorkflowRunId>,
    /// Handoff status.
    pub status: TypedHandoffStatus,
    /// Source endpoint.
    pub source: TypedHandoffEndpoint,
    /// Target endpoint.
    pub target: TypedHandoffEndpoint,
    /// Input references.
    pub input_references: Vec<TypedHandoffReference>,
    /// Output references.
    pub output_references: Vec<TypedHandoffReference>,
    /// Evidence references.
    pub evidence_references: Vec<TypedHandoffReference>,
    /// Validation references.
    pub validation_references: Vec<TypedHandoffReference>,
    /// Audit references.
    pub audit_references: Vec<TypedHandoffReference>,
    /// Policy references.
    pub policy_references: Vec<TypedHandoffReference>,
    /// Approval references.
    pub approval_references: Vec<TypedHandoffReference>,
    /// Work report references.
    pub work_report_references: Vec<TypedHandoffReference>,
    /// Next obligations.
    pub obligations: Vec<TypedHandoffTextItem>,
    /// Incomplete/deferred work disclosures.
    pub disclosures: Vec<TypedHandoffTextItem>,
    /// Risks.
    pub risks: Vec<TypedHandoffTextItem>,
    /// Bounded notes.
    pub notes: Vec<TypedHandoffTextItem>,
    /// Sensitivity classification.
    pub sensitivity: WorkReportSensitivity,
    /// Redaction policy.
    pub redaction_policy: WorkReportRedactionPolicy,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
}

impl TypedHandoff {
    /// Creates a validated typed handoff.
    ///
    /// # Errors
    ///
    /// Returns an error when required fields are missing, duplicated,
    /// unbounded, or secret-like.
    pub fn new(definition: TypedHandoffDefinition) -> Result<Self, WorkflowOsError> {
        let handoff = Self {
            handoff_id: definition.handoff_id,
            contract_id: definition.contract_id,
            contract_version: definition.contract_version,
            schema_version: definition.schema_version,
            workflow_id: definition.workflow_id,
            run_id: definition.run_id,
            status: definition.status,
            source: definition.source,
            target: definition.target,
            input_references: definition.input_references,
            output_references: definition.output_references,
            evidence_references: definition.evidence_references,
            validation_references: definition.validation_references,
            audit_references: definition.audit_references,
            policy_references: definition.policy_references,
            approval_references: definition.approval_references,
            work_report_references: definition.work_report_references,
            obligations: definition.obligations,
            disclosures: definition.disclosures,
            risks: definition.risks,
            notes: definition.notes,
            sensitivity: definition.sensitivity,
            redaction_policy: definition.redaction_policy,
            redaction: definition.redaction,
        };
        handoff.validate()?;
        Ok(handoff)
    }

    /// Validates the handoff.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_not_secret_like("schema version", self.schema_version.as_str())?;
        self.source.validate()?;
        self.target.validate()?;
        if self.source.reference == self.target.reference {
            return Err(validation_error(
                "typed_handoff.endpoint.same",
                "typed handoff source and target endpoints must be distinct",
            ));
        }
        validate_reference_list(
            "typed_handoff.inputs.required",
            "typed handoffs require at least one input reference",
            &self.input_references,
        )?;
        validate_reference_list(
            "typed_handoff.outputs.required",
            "typed handoffs require at least one output reference",
            &self.output_references,
        )?;
        validate_reference_list(
            "typed_handoff.evidence.required",
            "typed handoffs require at least one evidence reference",
            &self.evidence_references,
        )?;
        validate_reference_list(
            "typed_handoff.validation.required",
            "typed handoffs require at least one validation reference",
            &self.validation_references,
        )?;
        validate_optional_reference_list(&self.audit_references)?;
        validate_optional_reference_list(&self.policy_references)?;
        validate_optional_reference_list(&self.approval_references)?;
        validate_optional_reference_list(&self.work_report_references)?;
        validate_text_item_list(
            "typed_handoff.obligations.required",
            "typed handoffs require at least one next obligation",
            &self.obligations,
        )?;
        validate_optional_text_item_list(&self.disclosures)?;
        validate_optional_text_item_list(&self.risks)?;
        validate_optional_text_item_list(&self.notes)?;
        validate_redaction_metadata(&self.redaction)?;
        Ok(())
    }

    /// Returns the handoff ID.
    #[must_use]
    pub const fn handoff_id(&self) -> &TypedHandoffId {
        &self.handoff_id
    }

    /// Returns the contract ID.
    #[must_use]
    pub const fn contract_id(&self) -> &TypedHandoffContractId {
        &self.contract_id
    }

    /// Returns the contract version.
    #[must_use]
    pub const fn contract_version(&self) -> &TypedHandoffContractVersion {
        &self.contract_version
    }

    /// Returns the status.
    #[must_use]
    pub const fn status(&self) -> TypedHandoffStatus {
        self.status
    }

    /// Returns the source endpoint.
    #[must_use]
    pub const fn source(&self) -> &TypedHandoffEndpoint {
        &self.source
    }

    /// Returns the target endpoint.
    #[must_use]
    pub const fn target(&self) -> &TypedHandoffEndpoint {
        &self.target
    }

    /// Returns input references.
    #[must_use]
    pub fn input_references(&self) -> &[TypedHandoffReference] {
        &self.input_references
    }

    /// Returns output references.
    #[must_use]
    pub fn output_references(&self) -> &[TypedHandoffReference] {
        &self.output_references
    }

    /// Returns evidence references.
    #[must_use]
    pub fn evidence_references(&self) -> &[TypedHandoffReference] {
        &self.evidence_references
    }

    /// Returns validation references.
    #[must_use]
    pub fn validation_references(&self) -> &[TypedHandoffReference] {
        &self.validation_references
    }

    /// Returns obligations.
    #[must_use]
    pub fn obligations(&self) -> &[TypedHandoffTextItem] {
        &self.obligations
    }
}

impl fmt::Debug for TypedHandoff {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypedHandoff")
            .field("handoff_id", &self.handoff_id)
            .field("contract_id", &self.contract_id)
            .field("contract_version", &self.contract_version)
            .field("schema_version", &self.schema_version)
            .field(
                "workflow_id",
                &self.workflow_id.as_ref().map(|_| "[REDACTED]"),
            )
            .field("run_id", &self.run_id.as_ref().map(|_| "[REDACTED]"))
            .field("status", &self.status)
            .field("source", &self.source)
            .field("target", &self.target)
            .field("input_reference_count", &self.input_references.len())
            .field("output_reference_count", &self.output_references.len())
            .field("evidence_reference_count", &self.evidence_references.len())
            .field(
                "validation_reference_count",
                &self.validation_references.len(),
            )
            .field("audit_reference_count", &self.audit_references.len())
            .field("policy_reference_count", &self.policy_references.len())
            .field("approval_reference_count", &self.approval_references.len())
            .field(
                "work_report_reference_count",
                &self.work_report_references.len(),
            )
            .field("obligation_count", &self.obligations.len())
            .field("disclosure_count", &self.disclosures.len())
            .field("risk_count", &self.risks.len())
            .field("note_count", &self.notes.len())
            .field("sensitivity", &self.sensitivity)
            .field("redaction_policy", &self.redaction_policy)
            .field("redaction", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for TypedHandoff {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TypedHandoffWire {
            handoff_id: TypedHandoffId,
            contract_id: TypedHandoffContractId,
            contract_version: TypedHandoffContractVersion,
            schema_version: SchemaVersion,
            workflow_id: Option<WorkflowId>,
            run_id: Option<WorkflowRunId>,
            status: TypedHandoffStatus,
            source: TypedHandoffEndpoint,
            target: TypedHandoffEndpoint,
            input_references: Vec<TypedHandoffReference>,
            output_references: Vec<TypedHandoffReference>,
            evidence_references: Vec<TypedHandoffReference>,
            validation_references: Vec<TypedHandoffReference>,
            audit_references: Vec<TypedHandoffReference>,
            policy_references: Vec<TypedHandoffReference>,
            approval_references: Vec<TypedHandoffReference>,
            work_report_references: Vec<TypedHandoffReference>,
            obligations: Vec<TypedHandoffTextItem>,
            disclosures: Vec<TypedHandoffTextItem>,
            risks: Vec<TypedHandoffTextItem>,
            notes: Vec<TypedHandoffTextItem>,
            sensitivity: WorkReportSensitivity,
            redaction_policy: WorkReportRedactionPolicy,
            redaction: RedactionMetadata,
        }

        let wire = TypedHandoffWire::deserialize(deserializer)?;
        Self::new(TypedHandoffDefinition {
            handoff_id: wire.handoff_id,
            contract_id: wire.contract_id,
            contract_version: wire.contract_version,
            schema_version: wire.schema_version,
            workflow_id: wire.workflow_id,
            run_id: wire.run_id,
            status: wire.status,
            source: wire.source,
            target: wire.target,
            input_references: wire.input_references,
            output_references: wire.output_references,
            evidence_references: wire.evidence_references,
            validation_references: wire.validation_references,
            audit_references: wire.audit_references,
            policy_references: wire.policy_references,
            approval_references: wire.approval_references,
            work_report_references: wire.work_report_references,
            obligations: wire.obligations,
            disclosures: wire.disclosures,
            risks: wire.risks,
            notes: wire.notes,
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
    values: &[TypedHandoffReference],
) -> Result<(), WorkflowOsError> {
    if values.is_empty() {
        return Err(validation_error(required_code, required_message));
    }
    validate_optional_reference_list(values)
}

fn validate_optional_reference_list(
    values: &[TypedHandoffReference],
) -> Result<(), WorkflowOsError> {
    let mut seen = BTreeSet::new();
    for value in values {
        value.validate()?;
        if !seen.insert((value.target.kind_name(), value.name.clone())) {
            return Err(validation_error(
                "typed_handoff.references.duplicate",
                "typed handoffs cannot contain duplicate reference names",
            ));
        }
    }
    Ok(())
}

fn validate_text_item_list(
    required_code: &'static str,
    required_message: &'static str,
    values: &[TypedHandoffTextItem],
) -> Result<(), WorkflowOsError> {
    if values.is_empty() {
        return Err(validation_error(required_code, required_message));
    }
    validate_optional_text_item_list(values)
}

fn validate_optional_text_item_list(
    values: &[TypedHandoffTextItem],
) -> Result<(), WorkflowOsError> {
    let mut seen = BTreeSet::new();
    for value in values {
        value.validate()?;
        if !seen.insert(value.name.clone()) {
            return Err(validation_error(
                "typed_handoff.text_items.duplicate",
                "typed handoffs cannot contain duplicate text item names",
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
            "typed_handoff.identifier.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > HANDOFF_IDENTIFIER_MAX_BYTES {
        return Err(validation_error(
            "typed_handoff.identifier.too_long",
            format!("{type_name} cannot exceed {HANDOFF_IDENTIFIER_MAX_BYTES} bytes"),
        ));
    }

    let is_valid = value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'));

    if !is_valid {
        return Err(validation_error(
            "typed_handoff.identifier.invalid_character",
            format!("{type_name} contains an invalid character"),
        ));
    }

    validate_not_secret_like(type_name, value)
}

fn validate_text(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "typed_handoff.text.empty",
            format!("{type_name} cannot be empty"),
        ));
    }

    if value.len() > HANDOFF_TEXT_MAX_BYTES {
        return Err(validation_error(
            "typed_handoff.text.too_long",
            format!("{type_name} cannot exceed {HANDOFF_TEXT_MAX_BYTES} bytes"),
        ));
    }

    validate_not_secret_like(type_name, value)
}

fn validate_redaction_metadata(redaction: &RedactionMetadata) -> Result<(), WorkflowOsError> {
    if redaction.redacted_fields.len() > HANDOFF_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "typed_handoff.redaction.too_many_fields",
            "typed handoff redaction metadata contains too many fields",
        ));
    }

    if redaction.field_states.len() > HANDOFF_REDACTION_MAX_ENTRIES {
        return Err(validation_error(
            "typed_handoff.redaction.too_many_states",
            "typed handoff redaction metadata contains too many field states",
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
            "typed_handoff.redaction.field.empty",
            "typed handoff redaction field cannot be empty",
        ));
    }

    if value.len() > HANDOFF_REDACTION_FIELD_MAX_BYTES {
        return Err(validation_error(
            "typed_handoff.redaction.field.too_long",
            format!(
                "typed handoff redaction field cannot exceed {HANDOFF_REDACTION_FIELD_MAX_BYTES} bytes"
            ),
        ));
    }

    validate_not_secret_like("typed handoff redaction field", value)
}

fn validate_redaction_reason(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "typed_handoff.redaction.reason.empty",
            "typed handoff redaction reason cannot be empty",
        ));
    }

    if value.len() > HANDOFF_REDACTION_REASON_MAX_BYTES {
        return Err(validation_error(
            "typed_handoff.redaction.reason.too_long",
            format!(
                "typed handoff redaction reason cannot exceed {HANDOFF_REDACTION_REASON_MAX_BYTES} bytes"
            ),
        ));
    }

    validate_not_secret_like("typed handoff redaction reason", value)
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
            "typed_handoff.secret_like_value",
            format!("{type_name} contains sensitive-looking text"),
        ));
    }

    Ok(())
}

fn validation_error(code: &'static str, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}
