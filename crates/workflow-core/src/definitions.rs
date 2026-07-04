use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::work_report::WorkReportArtifactHighAssuranceRequirement;
use crate::{
    ActorId, AdapterId, IntegrationId, PolicyId, RedactedValue, SchemaVersion, SkillId,
    SkillVersion, SourceLocation, SpecContentHash, StepId, WorkflowId, WorkflowVersion,
};

/// Lifecycle status for public workflow and skill definitions.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleStatus {
    /// The definition is still changing and must be treated as experimental.
    #[default]
    Experimental,
    /// The definition is stable within its declared version.
    Stable,
    /// The definition is retained for compatibility but should not be used for new work.
    Deprecated,
}

/// Governed autonomy level declared by a workflow definition.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutonomyLevel {
    /// Level 1: assistive behavior with no autonomous external side effects.
    #[serde(alias = "level_1")]
    #[default]
    Level1Assistive,
    /// Level 2: guided execution with human approval before governed action.
    #[serde(alias = "level_2")]
    Level2GuidedWithApproval,
    /// Level 3: conditional autonomy, requiring explicit future policy enablement.
    #[serde(alias = "level_3")]
    Level3ConditionalAutonomy,
    /// Level 4: scaled automation, requiring explicit future policy enablement.
    #[serde(alias = "level_4")]
    Level4ScaledAutomation,
}

/// Ownership and operational contact metadata for a public definition.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct OwnershipMetadata {
    /// Team accountable for the definition.
    pub owning_team: Option<String>,
    /// Primary maintainer or maintainer group.
    pub maintainer: Option<ActorId>,
    /// Escalation contact for production-shaped failures or approvals.
    pub escalation_contact: Option<ActorId>,
    /// Lifecycle status for compatibility and documentation.
    pub lifecycle_status: LifecycleStatus,
}

/// Workflow-authored report artifact requirements.
///
/// This is a schema-facing declaration surface only. It does not generate
/// reports, write artifacts, or derive runtime artifact gate inputs by itself.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ReportArtifactRequirements {
    /// Required high-assurance approval disclosure posture for terminal report artifacts.
    pub high_assurance_approval: WorkReportArtifactHighAssuranceRequirement,
}

/// A declarative v0 workflow definition.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowDefinition {
    /// Schema version for this workflow spec.
    pub schema_version: SchemaVersion,
    /// Stable workflow identifier.
    pub id: WorkflowId,
    /// Version identifier for this workflow definition.
    pub version: WorkflowVersion,
    /// Human-readable workflow display name.
    #[serde(alias = "name")]
    pub display_name: String,
    /// Optional workflow description.
    #[serde(default)]
    pub description: Option<String>,
    /// Ownership and lifecycle metadata.
    #[serde(default)]
    pub owner: OwnershipMetadata,
    /// Governed autonomy level. Missing values default to Level 1.
    #[serde(default)]
    pub autonomy_level: AutonomyLevel,
    /// Whether the workflow is disabled by default. Required for v0 Level 3/4 declarations.
    #[serde(default)]
    pub disabled_by_default: bool,
    /// Trigger declarations. These are model-only in v0 and are not executed yet.
    #[serde(default)]
    pub triggers: Vec<TriggerDefinition>,
    /// Optional state model reference or inline declaration.
    #[serde(default)]
    pub state_model: Option<StateModelDefinition>,
    /// Ordered step declarations.
    #[serde(default)]
    pub steps: Vec<StepDefinition>,
    /// Conditional branch declarations.
    #[serde(default)]
    pub branches: Vec<ConditionalBranchDefinition>,
    /// Workflow-level approval requirements.
    #[serde(default)]
    pub approval_requirements: Vec<ApprovalRequirement>,
    /// Workflow-level retry policy references.
    #[serde(default)]
    pub retry_policy_refs: Vec<PolicyReference>,
    /// Workflow-level escalation policy references.
    #[serde(default)]
    pub escalation_policy_refs: Vec<PolicyReference>,
    /// Workflow-level timeout policy.
    #[serde(default)]
    pub timeout_policy: Option<TimeoutPolicy>,
    /// Cancellation behavior for this workflow.
    #[serde(default)]
    pub cancellation_behavior: Option<CancellationBehavior>,
    /// Audit requirements declared by this workflow.
    #[serde(default)]
    pub audit_requirements: AuditRequirements,
    /// Observability requirements declared by this workflow.
    #[serde(default)]
    pub observability_requirements: ObservabilityRequirements,
    /// Terminal report artifact requirements declared by this workflow.
    #[serde(default)]
    pub report_artifact_requirements: ReportArtifactRequirements,
    /// Free-form non-secret tags for discovery and documentation.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Canonical spec content hash, populated by the parser when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec_content_hash: Option<SpecContentHash>,
    /// Optional source location for diagnostics. Plain string parsing cannot populate spans yet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_location: Option<SourceLocation>,
}

/// A declarative v0 skill definition.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkillDefinition {
    /// Schema version for this skill spec.
    pub schema_version: SchemaVersion,
    /// Stable skill identifier.
    pub id: SkillId,
    /// Version identifier for this skill definition.
    pub version: SkillVersion,
    /// Human-readable skill display name.
    #[serde(alias = "name")]
    pub display_name: String,
    /// Optional skill description.
    #[serde(default)]
    pub description: Option<String>,
    /// Ownership and lifecycle metadata.
    #[serde(default)]
    pub owner: OwnershipMetadata,
    /// Skill input contract.
    #[serde(default)]
    pub input_contract: DataContract,
    /// Skill output contract.
    #[serde(default)]
    pub output_contract: DataContract,
    /// Capability names the skill may request through policy.
    #[serde(default)]
    pub allowed_capabilities: Vec<CapabilityRequirement>,
    /// Adapter requirements. These are declarations only in v0.
    #[serde(default)]
    pub adapter_requirements: Vec<AdapterRequirement>,
    /// Declared failure modes.
    #[serde(default)]
    pub failure_modes: Vec<FailureMode>,
    /// Criteria for future evaluation and tests.
    #[serde(default)]
    pub evaluation_criteria: Vec<EvaluationCriterion>,
    /// Whether this skill is compatible with retry.
    #[serde(default)]
    pub retry_compatibility: RetryCompatibility,
    /// Approval sensitivity for this skill.
    #[serde(default)]
    pub approval_sensitivity: ApprovalSensitivity,
    /// Audit requirements declared by this skill.
    #[serde(default)]
    pub audit_requirements: AuditRequirements,
    /// Observability requirements declared by this skill.
    #[serde(default)]
    pub observability_requirements: ObservabilityRequirements,
    /// Free-form non-secret tags for discovery and documentation.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Optional source location for diagnostics. Plain string parsing cannot populate spans yet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_location: Option<SourceLocation>,
}

/// Compatibility alias for the project-layer workflow spec document.
pub type WorkflowSpecDocument = WorkflowDefinition;

/// Compatibility alias for the project-layer skill spec document.
pub type SkillSpecDocument = SkillDefinition;

/// A workflow step declaration.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StepDefinition {
    /// Step identifier local to the workflow.
    pub id: StepId,
    /// Referenced skill ID and optional version.
    #[serde(alias = "skill")]
    pub skill_ref: SkillReference,
    /// Input mappings from workflow context into the skill contract.
    #[serde(default)]
    pub input_mapping: Vec<ValueMapping>,
    /// Output mappings from skill result into workflow context.
    #[serde(default)]
    pub output_mapping: Vec<ValueMapping>,
    /// Required policy references for the step.
    #[serde(default)]
    pub policy_requirements: Vec<PolicyReference>,
    /// Idempotency key strategy for future invocation.
    #[serde(default)]
    pub idempotency_key_strategy: IdempotencyKeyStrategy,
    /// Step timeout.
    #[serde(default)]
    pub timeout: Option<DurationSpec>,
    /// Step-level retry policy.
    #[serde(default)]
    pub retry_policy: Option<RetryPolicyRef>,
    /// Step-level escalation policy.
    #[serde(default)]
    pub escalation_policy: Option<EscalationPolicyRef>,
    /// Step-level approval policy.
    #[serde(default)]
    pub approval_policy: Option<ApprovalPolicyRef>,
    /// Terminal behavior for terminal step outcomes.
    #[serde(default)]
    pub terminal_behavior: Option<TerminalBehavior>,
    /// Optional source location for diagnostics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_location: Option<SourceLocation>,
}

/// Reference to a skill definition.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkillReference {
    /// Referenced skill ID.
    pub id: SkillId,
    /// Referenced skill version.
    #[serde(default)]
    pub version: Option<SkillVersion>,
}

/// Reference to a policy definition.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyReference {
    /// Referenced policy ID.
    pub id: PolicyId,
    /// Optional policy version string. Policy version modeling is deferred.
    #[serde(default)]
    pub version: Option<String>,
}

/// A typed value mapping between two named locations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValueMapping {
    /// Source expression, field path, or literal reference.
    pub from: MappingExpression,
    /// Destination field path.
    pub to: String,
}

/// Mapping expression used by declarative input and output mappings.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MappingExpression {
    /// Read a field path from workflow or skill context.
    Field {
        /// Field path to read.
        path: String,
    },
    /// Use a non-secret literal value.
    Literal {
        /// Literal string value.
        value: String,
    },
    /// Read a non-secret environment or config reference by name.
    ConfigRef {
        /// Configuration reference name.
        name: String,
    },
}

/// Trigger declaration. Triggers are parsed but not executed in this layer.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TriggerDefinition {
    /// Trigger identifier local to the workflow.
    pub id: String,
    /// Trigger kind.
    pub kind: TriggerKind,
    /// Optional description.
    #[serde(default)]
    pub description: Option<String>,
    /// Optional deduplication key expression for future runtime use.
    #[serde(default)]
    pub deduplication_key: Option<String>,
}

/// Supported v0 trigger declaration kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerKind {
    /// Manual local trigger.
    Manual,
    /// File-based local trigger declaration.
    File,
    /// Scheduled local trigger declaration.
    Schedule,
    /// External event declaration. Real external adapters are deferred.
    ExternalEvent,
}

/// Inline state model declaration or reference.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StateModelDefinition {
    /// Reference to a named state model.
    Reference {
        /// State model reference.
        id: String,
    },
    /// Inline state model declaration.
    Inline {
        /// State names declared for future validation/runtime use.
        states: Vec<String>,
    },
}

/// Conditional branch declaration.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConditionalBranchDefinition {
    /// Branch identifier local to the workflow.
    pub id: String,
    /// Condition expression. The expression language is intentionally not implemented yet.
    pub condition: String,
    /// Step to execute when the condition matches.
    pub target_step: StepId,
}

/// Workflow-level approval requirement.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalRequirement {
    /// Approval requirement identifier.
    pub id: String,
    /// Human-readable reason for the approval requirement.
    pub reason: String,
    /// Required approver actor or group.
    #[serde(default)]
    pub approver: Option<ActorId>,
    /// Expiration behavior for the approval request.
    #[serde(default)]
    pub expires_after: Option<DurationSpec>,
}

/// Duration specification preserved as a human-authored string.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DurationSpec {
    /// Human-readable duration, such as `30s`, `10m`, or `1h`.
    pub duration: String,
}

/// Workflow timeout policy.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimeoutPolicy {
    /// Maximum workflow duration.
    pub max_duration: DurationSpec,
    /// Behavior after timeout.
    #[serde(default)]
    pub on_timeout: TimeoutBehavior,
}

/// Timeout behavior declaration.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeoutBehavior {
    /// Escalate on timeout.
    #[default]
    Escalate,
    /// Fail terminally on timeout.
    Fail,
    /// Cancel the run on timeout.
    Cancel,
}

/// Cancellation behavior declaration.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CancellationBehavior {
    /// Cancellation records the cancellation and stops future work.
    #[default]
    Stop,
    /// Cancellation attempts future compensating behavior. Runtime support is deferred.
    Compensate,
}

/// Retry policy reference for a step.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetryPolicyRef {
    /// Referenced policy ID.
    pub policy: PolicyReference,
}

/// Escalation policy reference for a step.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EscalationPolicyRef {
    /// Referenced policy ID.
    pub policy: PolicyReference,
}

/// Approval policy reference for a step.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalPolicyRef {
    /// Referenced policy ID.
    pub policy: PolicyReference,
}

/// Idempotency key strategy for future skill invocation.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IdempotencyKeyStrategy {
    /// Runtime derives the key from workflow ID, version, run ID, step ID, and attempt.
    #[default]
    Derived,
    /// Use a declared non-secret expression as the key material.
    Expression {
        /// Non-secret expression for deterministic key derivation.
        value: String,
    },
}

/// Terminal behavior for step outcomes.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalBehavior {
    /// Fail the workflow if the step fails terminally.
    #[default]
    FailWorkflow,
    /// Escalate the workflow if the step fails terminally.
    Escalate,
    /// Continue after recording the terminal step outcome.
    Continue,
}

/// Input or output data contract.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DataContract {
    /// Schema-like field definitions.
    pub fields: Vec<ContractField>,
    /// Required field names.
    pub required: Vec<String>,
    /// Non-secret or redacted examples.
    pub examples: Vec<ContractExample>,
}

/// Schema-like field definition for a contract.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractField {
    /// Field name.
    pub name: String,
    /// Field type.
    pub field_type: ContractFieldType,
    /// Optional description.
    #[serde(default)]
    pub description: Option<String>,
    /// Whether this field carries sensitive data.
    #[serde(default)]
    pub sensitive: bool,
    /// Redaction behavior for sensitive data.
    #[serde(default)]
    pub redaction: Option<RedactionBehavior>,
}

/// Contract field types supported by the v0 declaration model.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractFieldType {
    /// UTF-8 string.
    String,
    /// Boolean.
    Boolean,
    /// Number.
    Number,
    /// Object.
    Object,
    /// Array.
    Array,
}

/// Redaction behavior for fields and examples.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionBehavior {
    /// Redact the full value.
    #[default]
    Full,
    /// Store a summary only.
    SummaryOnly,
    /// Store references only.
    ReferenceOnly,
}

/// A contract example.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ContractExample {
    /// Example name.
    pub name: Option<String>,
    /// Example field values.
    pub values: Vec<ContractExampleValue>,
}

/// One example field value with explicit sensitivity handling.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ContractExampleValue {
    /// Non-sensitive example value.
    Plain {
        /// Field name.
        field: String,
        /// Non-sensitive example value.
        value: String,
    },
    /// Sensitive example value. Display, debug, and serialization are redacted.
    Sensitive {
        /// Field name.
        field: String,
        /// Sensitive example value.
        value: RedactedValue<String>,
    },
}

/// Capability required by a skill.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityRequirement {
    /// Capability name.
    pub name: String,
    /// Optional reason the skill needs the capability.
    #[serde(default)]
    pub reason: Option<String>,
}

/// Adapter requirement declared by a skill.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterRequirement {
    /// Required adapter identifier.
    pub adapter_id: AdapterId,
    /// Optional integration identifier.
    #[serde(default)]
    pub integration_id: Option<IntegrationId>,
    /// Required capability names for this adapter boundary.
    #[serde(default)]
    pub capabilities: Vec<String>,
}

/// Failure mode declared by a skill.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FailureMode {
    /// Stable failure mode code.
    pub code: String,
    /// Human-readable failure mode description.
    pub description: String,
    /// Whether this failure mode may be retried.
    #[serde(default)]
    pub retryable: bool,
}

/// Evaluation criterion for future tests and documentation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvaluationCriterion {
    /// Criterion name.
    pub name: String,
    /// Human-readable description.
    pub description: String,
}

/// Retry compatibility declared by a skill.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetryCompatibility {
    /// Skill is retry-compatible when idempotency requirements are satisfied.
    #[default]
    Compatible,
    /// Skill must not be retried automatically.
    NotCompatible,
    /// Skill requires explicit policy before retry.
    RequiresPolicy,
}

/// Approval sensitivity declared by a skill.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalSensitivity {
    /// No special approval sensitivity.
    #[default]
    Low,
    /// Sensitive or ambiguous use should require approval.
    Medium,
    /// Human approval should be required before use.
    High,
}

/// Audit requirements declared by workflows and skills.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AuditRequirements {
    /// Whether audit events are required for meaningful future behavior.
    pub required: bool,
    /// Event names or categories expected by the definition.
    pub events: Vec<String>,
    /// Whether input/output payloads must be stored only as references by default.
    pub store_references_only: bool,
}

/// Observability requirements declared by workflows and skills.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ObservabilityRequirements {
    /// Metric names expected by future runtime behavior.
    pub metrics: Vec<String>,
    /// Whether tracing is expected.
    pub tracing: bool,
    /// Whether latency tracking is expected.
    pub latency_tracking: bool,
}

/// Extra non-core metadata for future documentation generators.
pub type DocumentationMetadata = BTreeMap<String, String>;
