#![deny(unsafe_code)]
#![doc = "Canonical Rust core crate for Workflow OS."]
#![doc = ""]
#![doc = "This crate contains the canonical local-first kernel foundation described"]
#![doc = "in the project charter."]

mod adapters;
mod audit;
mod ci;
mod definitions;
mod diagnostic;
mod error;
mod evidence;
mod executor;
mod github;
mod harness;
mod identifiers;
mod jira;
mod loader;
mod local_check;
mod observability;
mod policy;
mod project;
mod redaction;
mod runtime;
mod state;
mod timestamp;
mod typed_handoff;
mod validation;
mod work_report;

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
pub(crate) use diagnostic::with_spec_file_evidence_from_source_location;
pub use diagnostic::{Diagnostic, DiagnosticSeverity, SourceLocation};
pub use error::{WorkflowOsError, WorkflowOsErrorKind};
pub use evidence::{
    ApprovalReferenceId, EvidenceKind, EvidenceMetadata, EvidenceRedactionMetadata,
    EvidenceReference, EvidenceReferenceId, EvidenceReferenceRequiredFields,
    EvidenceReferenceTarget, EvidenceRetentionHint, EvidenceScope, EvidenceSensitivity,
    EvidenceSourceComponent, ValidationReferenceId,
};
pub use executor::{
    LocalApprovalDecisionRequest, LocalCancellationRequest, LocalExecutionBeforeReportHookInput,
    LocalExecutionBeforeSkillInvocationHookInput, LocalExecutionReportInputs,
    LocalExecutionRequest, LocalExecutionWithReportRequest, LocalExecutionWithReportResult,
    LocalExecutor, LocalSkillRegistry, LocalTimeoutPolicy, SkillHandler, SkillInput, SkillOutput,
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
    AgentHarnessHookWorkflowEvent, AgentHarnessHookWorkflowEventDefinition, ApprovalDecision,
    ApprovalDecisionKind, ApprovalRequest, CancellationRecord, EscalationRecord,
    EventSequenceNumber, FailureClass, FailureRecord, RetryRecord, RunRehydration, SkillInvocation,
    SkillInvocationAttempt, StateTransition, WorkflowRun, WorkflowRunEvent, WorkflowRunEventKind,
    WorkflowRunEventKindName, WorkflowRunIdentity, WorkflowRunSnapshot, WorkflowRunStatus,
};
pub use state::{
    AdapterTelemetryStore, ApprovalStore, BackendHealthCheck, EventLogStore, IdempotencyResult,
    IdempotencyStore, IdempotencyWrite, LocalStateBackend, LocalStateInspection, LocalStateIssue,
    LocalStateIssueSeverity, LockLease, LockStore, PolicyAuditStore, ProjectStateRecord,
    ProjectStateStore, RunSnapshotStore, StateBackend, WorkReportArtifactStore,
};
pub use timestamp::Timestamp;
pub use typed_handoff::{
    TypedHandoff, TypedHandoffContract, TypedHandoffContractDefinition, TypedHandoffContractId,
    TypedHandoffContractVersion, TypedHandoffDefinition, TypedHandoffEndpoint,
    TypedHandoffEndpointKind, TypedHandoffFailureSemantics, TypedHandoffId, TypedHandoffReference,
    TypedHandoffReferenceTarget, TypedHandoffStatus, TypedHandoffTextItem,
};
pub use validation::{validate_loaded_project, validate_project_bundle, ValidationResult};
pub use work_report::{
    expose_terminal_local_work_report_result, generate_terminal_local_work_report,
    TerminalLocalWorkReportInput, TerminalLocalWorkReportResult, WorkReport,
    WorkReportArtifactMetadata, WorkReportArtifactRecord, WorkReportCitation,
    WorkReportCitationDefinition, WorkReportCitationKind, WorkReportCitationRequirement,
    WorkReportCitationTarget, WorkReportContract, WorkReportContractDefinition,
    WorkReportContractId, WorkReportContractVersion, WorkReportDefinition,
    WorkReportDisclosureKind, WorkReportDisclosureRequirements, WorkReportGenerationContext,
    WorkReportHandoffNote, WorkReportId, WorkReportIncompleteWorkDisclosure,
    WorkReportKnownLimitation, WorkReportRedactionPolicy, WorkReportRisk, WorkReportSection,
    WorkReportSectionKind, WorkReportSectionRequirement, WorkReportSensitivity,
    WorkReportStableReference, WorkReportStatus,
};

/// Human-readable name for the canonical Rust core crate.
pub const CRATE_NAME: &str = "workflow-core";

/// Current pre-release maturity marker for the core crate.
pub const MATURITY: &str = "foundation";
pub use adapters::{
    AdapterAction, AdapterCapability, AdapterCapabilityDiscovery, AdapterDryRun, AdapterError,
    AdapterErrorKind, AdapterEvent, AdapterEventSource, AdapterHealth, AdapterHealthCheck,
    AdapterIdempotencyStrategy, AdapterInvocationRecord, AdapterKind, AdapterObservabilityRecord,
    AdapterOperationMode, AdapterPolicyPrecheck, AdapterPolicyPrecheckProvenance,
    AdapterReadOperation, AdapterRedactionPolicy, AdapterRedactionStrategy, AdapterRequest,
    AdapterResponse, AdapterResponseSize, AdapterResponseStatus, AdapterRunScope,
    AdapterRuntimeAuditRecord, AdapterRuntimeObservabilityRecord, AdapterTelemetryRecord,
    AdapterTimeoutPolicy, AdapterWriteOperation,
};
pub use audit::{
    AuditEvent, AuditSink, FailingAuditSink, LocalAuditSink, LocalStructuredLogger,
    PolicyAuditRecord, PolicyAuditScope, RedactionDisposition, RedactionFieldState,
    RedactionMetadata, StructuredLogRecord, StructuredLogger,
};
pub use ci::{
    ci_actions, github_actions_read_request, GitHubActionsFixtureClient, GitHubActionsHttpResponse,
    GitHubActionsLiveReadOnlyClient, GitHubActionsReadOnlyAdapter, GitHubActionsReadOnlyClient,
    GitHubActionsReadOnlyConfig, GitHubActionsReadOutcome,
};
pub use github::{
    github_actions, github_read_request, GitHubFixtureClient, GitHubHttpResponse,
    GitHubLiveReadOnlyClient, GitHubReadOnlyAdapter, GitHubReadOnlyClient, GitHubReadOnlyConfig,
    GitHubReadOutcome,
};
pub use harness::{
    execute_runtime_agent_harness_hook, execute_runtime_agent_harness_hook_failed_closed,
    invoke_agent_harness_hook, invoke_agent_harness_hook_failed_closed,
    AgentHarnessHookAuditRecord, AgentHarnessHookAuditRecordDefinition, AgentHarnessHookContract,
    AgentHarnessHookContractDefinition, AgentHarnessHookContractId,
    AgentHarnessHookContractVersion, AgentHarnessHookDisclosure, AgentHarnessHookDisclosureKind,
    AgentHarnessHookFailureSemantics, AgentHarnessHookInputRequirement,
    AgentHarnessHookInvocationId, AgentHarnessHookInvocationInput,
    AgentHarnessHookInvocationResult, AgentHarnessHookInvocationResultDefinition,
    AgentHarnessHookInvocationStatus, AgentHarnessHookKind, AgentHarnessHookNamedReference,
    AgentHarnessHookOutputRequirement, AgentHarnessHookReference,
    AgentHarnessHookSideEffectAllowance, HarnessApprovalRequirement, HarnessAuthorityScope,
    HarnessContextRequirement, HarnessContract, HarnessContractDefinition, HarnessContractId,
    HarnessContractVersion, HarnessEvidenceRequirement, HarnessExecutionPolicy,
    HarnessFailureSemantics, HarnessHandoffRequirement, HarnessInputRequirement,
    HarnessOutputRequirement, HarnessSideEffectAllowance, HarnessToolAllowance, HarnessToolKind,
    RuntimeAgentHarnessHookInput, RuntimeAgentHarnessHookResult,
};
pub use jira::{
    jira_actions, jira_read_request, JiraFixtureClient, JiraHttpResponse, JiraLiveReadOnlyClient,
    JiraReadOnlyAdapter, JiraReadOnlyClient, JiraReadOnlyConfig, JiraReadOutcome,
};
pub use local_check::{
    DocsCheckLocalHandler, LocalCheckCommandContract, LocalCheckCommandContractDefinition,
    LocalCheckCommandId, LocalCheckCommandKind, LocalCheckEnvironmentPolicy,
    LocalCheckExecutionPosture, LocalCheckNetworkPolicy, LocalCheckOutputCapturePolicy,
    LocalCheckProcessOutput, LocalCheckProcessRequest, LocalCheckProcessRunner,
    LocalCheckRedactionPolicy, LocalCheckRegisteredHandler, LocalCheckRegistrationMode,
    LocalCheckRegistrationProfile, LocalCheckResult, LocalCheckResultDefinition,
    LocalCheckResultId, LocalCheckResultReference, LocalCheckResultReferenceDefinition,
    LocalCheckResultStatus, LocalCheckSideEffectBoundary, LocalCheckSideEffectBoundaryDefinition,
    LocalCheckSideEffectClass, LocalCheckSideEffectKind, LocalCheckWorkingDirectoryPolicy,
    TestOnlyDocsCheckHandler, TestOnlyWorkflowOsValidateDogfoodHandler,
};
pub use observability::{
    LocalObservabilitySink, ObservabilityEvent, ObservabilityEventKind, ObservabilitySink,
};
