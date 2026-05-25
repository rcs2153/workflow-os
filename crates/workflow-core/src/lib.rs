#![deny(unsafe_code)]
#![doc = "Canonical Rust core crate for Workflow OS."]
#![doc = ""]
#![doc = "This crate contains the canonical local-first kernel foundation described"]
#![doc = "in the project charter."]

mod adapters;
mod audit;
mod definitions;
mod diagnostic;
mod error;
mod executor;
mod identifiers;
mod loader;
mod observability;
mod policy;
mod project;
mod redaction;
mod runtime;
mod state;
mod timestamp;
mod validation;

pub use definitions::{
    AdapterRequirement, ApprovalPolicyRef, ApprovalRequirement, ApprovalSensitivity,
    AuditRequirements, AutonomyLevel, CancellationBehavior, CapabilityRequirement,
    ConditionalBranchDefinition, ContractExample, ContractExampleValue, ContractField,
    ContractFieldType, DataContract, DocumentationMetadata, DurationSpec, EscalationPolicyRef,
    EvaluationCriterion, FailureMode, IdempotencyKeyStrategy, LifecycleStatus, MappingExpression,
    ObservabilityRequirements, OwnershipMetadata, PolicyReference, RedactionBehavior,
    RetryCompatibility, RetryPolicyRef, SkillDefinition, SkillReference, SkillSpecDocument,
    StateModelDefinition, StepDefinition, TerminalBehavior, TimeoutBehavior, TimeoutPolicy,
    TriggerDefinition, TriggerKind, ValueMapping, WorkflowDefinition, WorkflowSpecDocument,
};
pub use diagnostic::{Diagnostic, DiagnosticSeverity, SourceLocation};
pub use error::{WorkflowOsError, WorkflowOsErrorKind};
pub use executor::{
    LocalApprovalDecisionRequest, LocalCancellationRequest, LocalExecutionRequest, LocalExecutor,
    LocalSkillRegistry, LocalTimeoutPolicy, SkillHandler, SkillInput, SkillOutput,
};
pub use identifiers::{
    ActorId, AdapterId, CorrelationId, EventId, IdempotencyKey, IntegrationId, PolicyId, ProjectId,
    SchemaVersion, SkillAttemptId, SkillId, SkillInvocationId, SkillVersion, SpecContentHash,
    StepId, WorkflowId, WorkflowRunId, WorkflowVersion,
};
pub use loader::{load_project, LoadedSpec, ProjectBundle, ProjectLoadResult};
pub use policy::{
    Action, Capability, ConservativePolicyEngine, PolicyDecision, PolicyEvaluationContext,
    PolicyViolation,
};
pub use project::{
    canonical_yaml_content_hash, parse_policy_spec_yaml, parse_project_manifest_yaml,
    parse_skill_spec_yaml, parse_test_spec_yaml, parse_workflow_spec_yaml, ConfigOverlay,
    ConfigVar, EnvironmentRef, PolicyRuleShell, PolicySpecDocument, ProjectLayout, ProjectManifest,
    ProjectMetadata, ReferenceResolutionRules, SpecReference, TestAssertionShell, TestSpecDocument,
    SUPPORTED_SCHEMA_VERSION,
};
pub use redaction::RedactedValue;
pub use runtime::{
    ApprovalDecision, ApprovalDecisionKind, ApprovalRequest, CancellationRecord, EscalationRecord,
    EventSequenceNumber, FailureClass, FailureRecord, RetryRecord, RunRehydration, SkillInvocation,
    SkillInvocationAttempt, StateTransition, WorkflowRun, WorkflowRunEvent, WorkflowRunEventKind,
    WorkflowRunEventKindName, WorkflowRunIdentity, WorkflowRunSnapshot, WorkflowRunStatus,
};
pub use state::{
    ApprovalStore, BackendHealthCheck, EventLogStore, IdempotencyResult, IdempotencyStore,
    IdempotencyWrite, LocalStateBackend, LocalStateInspection, LocalStateIssue,
    LocalStateIssueSeverity, LockLease, LockStore, PolicyAuditStore, ProjectStateRecord,
    ProjectStateStore, RunSnapshotStore, StateBackend,
};
pub use timestamp::Timestamp;
pub use validation::{validate_loaded_project, validate_project_bundle, ValidationResult};

/// Human-readable name for the canonical Rust core crate.
pub const CRATE_NAME: &str = "workflow-core";

/// Current pre-release maturity marker for the core crate.
pub const MATURITY: &str = "foundation";
pub use adapters::{
    AdapterAction, AdapterCapability, AdapterCapabilityDiscovery, AdapterDryRun, AdapterError,
    AdapterErrorKind, AdapterEvent, AdapterEventSource, AdapterHealth, AdapterHealthCheck,
    AdapterIdempotencyStrategy, AdapterInvocationRecord, AdapterKind, AdapterPolicyPrecheck,
    AdapterReadOperation, AdapterRedactionStrategy, AdapterRequest, AdapterResponse,
    AdapterWriteOperation,
};
pub use audit::{
    AuditEvent, AuditSink, FailingAuditSink, LocalAuditSink, LocalStructuredLogger,
    PolicyAuditRecord, PolicyAuditScope, RedactionDisposition, RedactionFieldState,
    RedactionMetadata, StructuredLogRecord, StructuredLogger,
};
pub use observability::{
    LocalObservabilitySink, ObservabilityEvent, ObservabilityEventKind, ObservabilitySink,
};
