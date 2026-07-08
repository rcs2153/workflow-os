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
mod high_assurance_approval;
mod identifiers;
mod jira;
mod loader;
mod local_check;
mod observability;
mod policy;
mod project;
mod provider_write;
mod redaction;
mod runtime;
mod side_effect;
mod side_effect_discovery;
mod state;
mod timestamp;
mod typed_handoff;
mod validation;
mod work_report;
mod workflow_authoring;
mod workflow_catalog;
mod workflow_catalog_store;
mod write_adapter_preflight;

pub use definitions::{
    AdapterRequirement, ApprovalPolicyRef, ApprovalRequirement, ApprovalSensitivity,
    AuditRequirements, AutonomyLevel, CancellationBehavior, CapabilityRequirement,
    ConditionalBranchDefinition, ContractExample, ContractExampleValue, ContractField,
    ContractFieldType, DataContract, DocumentationMetadata, DurationSpec, EscalationPolicyRef,
    EvaluationCriterion, FailureMode, IdempotencyKeyStrategy, LifecycleStatus, MappingExpression,
    ObservabilityRequirements, OwnershipMetadata, PolicyReference, RedactionBehavior,
    ReportArtifactRequirements, RetryCompatibility, RetryPolicyRef, SkillDefinition,
    SkillReference, SkillSpecDocument, StateModelDefinition, StepDefinition, TerminalBehavior,
    TimeoutBehavior, TimeoutPolicy, TriggerDefinition, TriggerKind, ValueMapping,
    WorkflowDefinition, WorkflowSpecDocument,
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
    execute_with_github_pr_comment_provider_write, execute_with_report_and_side_effect_discovery,
    execute_with_report_artifact_and_side_effect_gates,
    load_github_pr_comment_proposed_side_effect_event_input,
    GitHubPullRequestCommentProviderWriteDisclosurePosture,
    GitHubPullRequestCommentProviderWriteReportDisclosure,
    GitHubPullRequestCommentSideEffectAppendInput, LocalApprovalDecisionRequest,
    LocalCancellationRequest, LocalExecutionBeforeReportHookInput,
    LocalExecutionBeforeSkillInvocationCheckpointInputs,
    LocalExecutionBeforeSkillInvocationHookInput, LocalExecutionGitHubPrCommentProviderWriteInputs,
    LocalExecutionHookCheckpointInputs, LocalExecutionReportArtifactInputs,
    LocalExecutionReportArtifactProviderIntegrationInputs, LocalExecutionReportInputs,
    LocalExecutionRequest, LocalExecutionSideEffectDiscoveryInputs,
    LocalExecutionSideEffectEventInput, LocalExecutionSideEffectLifecycleEventInput,
    LocalExecutionWithGitHubPrCommentProviderWriteParts,
    LocalExecutionWithGitHubPrCommentProviderWriteRequest,
    LocalExecutionWithGitHubPrCommentProviderWriteResult,
    LocalExecutionWithReportAndSideEffectDiscoveryRequest, LocalExecutionWithReportArtifactParts,
    LocalExecutionWithReportArtifactRequest, LocalExecutionWithReportArtifactResult,
    LocalExecutionWithReportRequest, LocalExecutionWithReportResult, LocalExecutor,
    LocalHighAssuranceApprovalDecisionRequest,
    LocalHighAssuranceApprovalDecisionWithDisclosureResult, LocalSkillRegistry, LocalTimeoutPolicy,
    SkillHandler, SkillInput, SkillOutput,
};
pub use identifiers::{
    ActorId, AdapterId, CorrelationId, EventId, IdempotencyKey, IntegrationId, PolicyId, ProjectId,
    SchemaVersion, SkillAttemptId, SkillId, SkillInvocationId, SkillVersion, SpecContentHash,
    StepId, WorkflowId, WorkflowRunId, WorkflowVersion,
};
pub use loader::{load_project, LoadedSpec, ProjectBundle, ProjectLoadResult};
pub use policy::{
    Action, Capability, ConservativePolicyEngine, PolicyDecision, PolicyEffect,
    PolicyEffectParseError, PolicyEffectSet, PolicyEvaluationContext, PolicyViolation,
};
pub use project::{
    canonical_yaml_content_hash, parse_policy_spec_yaml, parse_project_manifest_yaml,
    parse_skill_spec_yaml, parse_test_spec_yaml, parse_workflow_spec_yaml, ConfigOverlay,
    ConfigVar, EnvironmentRef, PolicyRuleShell, PolicySpecDocument, ProjectLayout, ProjectManifest,
    ProjectMetadata, ReferenceResolutionRules, SpecReference, TestAssertionShell, TestSpecDocument,
    SUPPORTED_SCHEMA_VERSION,
};
pub use provider_write::{
    compose_and_persist_github_pr_comment_proposed_side_effect_record,
    compose_github_pr_comment_proposed_side_effect_event,
    compose_github_pr_comment_proposed_side_effect_record, github_pr_comment_preflight_definition,
    load_github_pr_comment_proposed_side_effect_event,
    orchestrate_github_pr_comment_no_provider_outcome, orchestrate_github_pr_comment_provider_call,
    orchestrate_github_pr_comment_write_attempt_without_provider_call,
    reconcile_github_pr_comment_provider_write, validate_github_pr_comment_fixture_write,
    GitHubPullRequestCommentFixture, GitHubPullRequestCommentFixtureDefinition,
    GitHubPullRequestCommentHttpProvider, GitHubPullRequestCommentHttpRequest,
    GitHubPullRequestCommentHttpResponse, GitHubPullRequestCommentHttpTransport,
    GitHubPullRequestCommentNoProviderOutcome,
    GitHubPullRequestCommentNoProviderOutcomeOrchestrationInput,
    GitHubPullRequestCommentNoProviderOutcomeOrchestrationResult,
    GitHubPullRequestCommentPreflightDefinitionInput, GitHubPullRequestCommentPreflightedWrite,
    GitHubPullRequestCommentProvider, GitHubPullRequestCommentProviderAuth,
    GitHubPullRequestCommentProviderCallInput,
    GitHubPullRequestCommentProviderCallOrchestrationError,
    GitHubPullRequestCommentProviderCallOrchestrationInput,
    GitHubPullRequestCommentProviderCallOrchestrationResult,
    GitHubPullRequestCommentProviderCallRequest,
    GitHubPullRequestCommentProviderWriteReconciliationCandidate,
    GitHubPullRequestCommentProviderWriteReconciliationInput,
    GitHubPullRequestCommentProviderWriteReconciliationStatus,
    GitHubPullRequestCommentSideEffectEventContext, GitHubPullRequestCommentSideEffectRecordInput,
    GitHubPullRequestCommentTarget, GitHubPullRequestCommentWriteAttemptOrchestrationInput,
    GitHubPullRequestCommentWriteAttemptOrchestrationResult, GitHubPullRequestCommentWriteMode,
    GitHubPullRequestCommentWriteOutcome, GitHubPullRequestCommentWriteRequest,
    GitHubPullRequestCommentWriteRequestDefinition, GitHubPullRequestCommentWriteResponse,
    GitHubPullRequestCommentWriteResponseDefinition,
};
pub use redaction::RedactedValue;
pub use runtime::{
    AgentHarnessHookWorkflowEvent, AgentHarnessHookWorkflowEventDefinition, ApprovalDecision,
    ApprovalDecisionKind, ApprovalRequest, CancellationRecord, EscalationRecord,
    EventSequenceNumber, FailureClass, FailureRecord, RetryRecord, RunRehydration,
    SideEffectWorkflowEvent, SideEffectWorkflowEventDefinition, SkillInvocation,
    SkillInvocationAttempt, StateTransition, WorkflowRun, WorkflowRunEvent, WorkflowRunEventKind,
    WorkflowRunEventKindName, WorkflowRunIdentity, WorkflowRunSnapshot, WorkflowRunStatus,
};
pub use side_effect::{
    transition_side_effect_to_attempted, transition_side_effect_to_attempted_in_store,
    transition_side_effect_to_completed, transition_side_effect_to_completed_in_store,
    transition_side_effect_to_failed, transition_side_effect_to_failed_in_store,
    validate_side_effect_approval_linkage, validate_side_effect_approval_linkage_from_store,
    SideEffectApprovalLinkageFromStoreInput, SideEffectApprovalLinkageFromStoreResult,
    SideEffectApprovalLinkageInput, SideEffectApprovalLinkageResult,
    SideEffectApprovalLinkageStoreLoadMode, SideEffectAttemptTransitionInput,
    SideEffectAttemptTransitionStoreInput, SideEffectAuthority, SideEffectAuthorityDecision,
    SideEffectCapability, SideEffectCompleteTransitionInput,
    SideEffectCompleteTransitionStoreInput, SideEffectFailTransitionInput,
    SideEffectFailTransitionStoreInput, SideEffectId, SideEffectIdempotencyBinding,
    SideEffectIdempotencyScope, SideEffectLifecycleState, SideEffectLifecycleTransitionResult,
    SideEffectMissingRecordPolicy, SideEffectOutcomeReference, SideEffectOutcomeReferenceKind,
    SideEffectRecord, SideEffectRecordDefinition, SideEffectReference, SideEffectReferenceKind,
    SideEffectSensitivity, SideEffectTargetKind, SideEffectTargetReference,
};
pub use side_effect_discovery::{
    discover_side_effect_references, discover_side_effect_references_from_store,
    SideEffectDiscoveryInput, SideEffectDiscoveryReference, SideEffectDiscoveryResult,
    SideEffectDiscoverySource, SideEffectStoreBackedDiscoveryInput,
};
pub use state::{
    AdapterTelemetryStore, ApprovalStore, BackendHealthCheck, EventLogStore, IdempotencyResult,
    IdempotencyStore, IdempotencyWrite, LocalStateBackend, LocalStateInspection, LocalStateIssue,
    LocalStateIssueSeverity, LockLease, LockStore, PolicyAuditStore, ProjectStateRecord,
    ProjectStateStore, RunSnapshotStore, SideEffectRecordStore, StateBackend,
    WorkReportArtifactStore,
};
pub use timestamp::Timestamp;
pub use typed_handoff::{
    TypedHandoff, TypedHandoffContract, TypedHandoffContractDefinition, TypedHandoffContractId,
    TypedHandoffContractVersion, TypedHandoffDefinition, TypedHandoffEndpoint,
    TypedHandoffEndpointKind, TypedHandoffFailureSemantics, TypedHandoffId, TypedHandoffReference,
    TypedHandoffReferenceTarget, TypedHandoffStatus, TypedHandoffTextItem,
};
pub use validation::{
    validate_loaded_project, validate_loaded_project_with_capability, validate_project_bundle,
    validate_project_bundle_with_capability, ProjectValidationCapability, ValidationResult,
};
pub use work_report::{
    derive_workflow_report_artifact_gate_policy, expose_terminal_local_work_report_result,
    generate_terminal_local_work_report,
    generate_terminal_local_work_report_with_side_effect_discovery,
    validate_github_pr_comment_provider_report_artifact_event_proof_gate,
    validate_github_pr_comment_report_artifact_citations,
    validate_work_report_artifact_side_effect_integrity,
    write_github_pr_comment_report_artifact_from_explicit_context,
    write_github_pr_comment_report_artifact_with_citations,
    write_report_artifact_with_explicit_integrations,
    write_work_report_artifact_with_side_effect_integrity_and_approval_linkage,
    GitHubPullRequestCommentProviderReportArtifactEventProofGatePolicy,
    GitHubPullRequestCommentProviderReportArtifactEventProofGateResult,
    GitHubPullRequestCommentReportArtifactCitationInput,
    GitHubPullRequestCommentReportArtifactCitationPolicy,
    GitHubPullRequestCommentReportArtifactCitationResult,
    GitHubPullRequestCommentReportArtifactIntegrationInput,
    GitHubPullRequestCommentReportArtifactWriteInput,
    GitHubPullRequestCommentReportArtifactWriteResult, ReportArtifactWriteIntegrationInput,
    ReportArtifactWriteIntegrationResult, ReportArtifactWriteProviderIntegration,
    ReportArtifactWriteProviderIntegrationResult, TerminalLocalWorkReportInput,
    TerminalLocalWorkReportResult, TerminalLocalWorkReportSideEffectDiscoveryInput, WorkReport,
    WorkReportArtifactGovernedWriteInput, WorkReportArtifactGovernedWriteResult,
    WorkReportArtifactHighAssuranceDisclosureGateResult,
    WorkReportArtifactHighAssuranceDisclosurePolicy, WorkReportArtifactHighAssuranceRequirement,
    WorkReportArtifactMetadata, WorkReportArtifactRecord, WorkReportArtifactRequirement,
    WorkReportArtifactRequirementDefinition, WorkReportArtifactSideEffectIntegrityInput,
    WorkReportArtifactSideEffectIntegrityResult,
    WorkReportArtifactUnsupportedHighAssuranceRequirement, WorkReportCitation,
    WorkReportCitationDefinition, WorkReportCitationKind, WorkReportCitationRequirement,
    WorkReportCitationTarget, WorkReportContract, WorkReportContractDefinition,
    WorkReportContractId, WorkReportContractVersion, WorkReportDefinition,
    WorkReportDisclosureKind, WorkReportDisclosureRequirements, WorkReportGenerationContext,
    WorkReportHandoffNote, WorkReportHighAssuranceApprovalDecision,
    WorkReportHighAssuranceApprovalDisclosure, WorkReportHighAssuranceApprovalDisclosureDefinition,
    WorkReportHighAssuranceExpirationPosture, WorkReportHighAssuranceRequesterApproverPosture,
    WorkReportHighAssuranceRevocationPosture, WorkReportId, WorkReportIncompleteWorkDisclosure,
    WorkReportKnownLimitation, WorkReportRedactionPolicy, WorkReportRisk, WorkReportSection,
    WorkReportSectionKind, WorkReportSectionRequirement, WorkReportSensitivity,
    WorkReportStableReference, WorkReportStatus, WorkflowReportArtifactGateDerivation,
    WorkflowReportArtifactGateDerivationInput,
};
pub use workflow_authoring::{
    review_workflow_draft_for_promotion, WorkflowDraftPromotionPreflightStatus,
    WorkflowDraftStewardReviewAuthorization, WorkflowDraftStewardReviewBoundary,
    WorkflowDraftStewardReviewCard, WorkflowDraftStewardReviewDecision,
    WorkflowDraftStewardReviewInput, WorkflowDraftStewardReviewResult,
};
pub use workflow_catalog::{
    WorkflowArchiveRecord, WorkflowArchiveRecordDefinition, WorkflowArchiveRecordId,
    WorkflowCatalogRecord, WorkflowCatalogRecordDefinition, WorkflowCatalogRecordId,
    WorkflowLifecycleStatus, WorkflowStewardshipDecisionId, WorkflowStewardshipDecisionKind,
    WorkflowStewardshipRecord, WorkflowStewardshipRecordDefinition,
};
pub use workflow_catalog_store::{LocalWorkflowCatalogStore, WorkflowCatalogStoreHealth};
pub use write_adapter_preflight::{
    preflight_adapter_write, AdapterWriteCapability, AdapterWritePolicyDecision,
    AdapterWritePreflightDecision, AdapterWritePreflightRequest,
    AdapterWritePreflightRequestDefinition, AdapterWriteReadinessPolicy,
    AdapterWriteReadinessPolicyDefinition, AdapterWriteTarget, AdapterWriteTargetKind,
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
    AgentHarnessHookContractVersion, AgentHarnessHookDisclosure,
    AgentHarnessHookDisclosureDefinition, AgentHarnessHookDisclosureId,
    AgentHarnessHookDisclosureKind, AgentHarnessHookDisclosureReference,
    AgentHarnessHookDisclosureSeverity, AgentHarnessHookFailureSemantics,
    AgentHarnessHookInputRequirement, AgentHarnessHookInvocationId,
    AgentHarnessHookInvocationInput, AgentHarnessHookInvocationResult,
    AgentHarnessHookInvocationResultDefinition, AgentHarnessHookInvocationStatus,
    AgentHarnessHookKind, AgentHarnessHookNamedReference, AgentHarnessHookOutputRequirement,
    AgentHarnessHookReference, AgentHarnessHookSideEffectAllowance, HarnessApprovalRequirement,
    HarnessAuthorityScope, HarnessContextRequirement, HarnessContract, HarnessContractDefinition,
    HarnessContractId, HarnessContractVersion, HarnessEvidenceRequirement, HarnessExecutionPolicy,
    HarnessFailureSemantics, HarnessHandoffRequirement, HarnessInputRequirement,
    HarnessOutputRequirement, HarnessSideEffectAllowance, HarnessToolAllowance, HarnessToolKind,
    RuntimeAgentHarnessHookInput, RuntimeAgentHarnessHookResult,
};
pub use high_assurance_approval::{
    discover_high_assurance_approval_disclosure, validate_high_assurance_approval_decision,
    HighAssuranceApprovalControl, HighAssuranceApprovalControlDefinition,
    HighAssuranceApprovalControlId, HighAssuranceApprovalControlVersion,
    HighAssuranceApprovalDecisionValidationInput, HighAssuranceApprovalDecisionValidationResult,
    HighAssuranceApprovalDenialBehavior, HighAssuranceApprovalDisclosureDiscoveryInput,
    HighAssuranceApprovalDisclosureDiscoveryResult,
    HighAssuranceApprovalDisclosureNotAvailableReason, HighAssuranceApprovalExpirationPolicy,
    HighAssuranceApprovalReportDisclosure, HighAssuranceApprovalRequiredReference,
    HighAssuranceApprovalRequiredReferenceTarget, HighAssuranceApprovalRevocationPolicy,
    HighAssuranceApprovalSuppliedReference, HighAssuranceProtectedActionKind,
    HighAssuranceRequesterApproverRule,
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
