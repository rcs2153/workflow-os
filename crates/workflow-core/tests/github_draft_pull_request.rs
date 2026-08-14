#![allow(clippy::expect_used)]

//! Focused tests for the explicit draft GitHub pull request provider mutation slice.

use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::json;
use sha2::{Digest, Sha256};
use workflow_core::{
    compute_approval_presentation_content_hash, execute_github_draft_pull_request_mutation,
    generate_terminal_local_work_report, github_com_draft_pull_request_http_provider,
    resolve_capability_authority, Action, ActorId, AdapterId, AdapterWriteCapability,
    AdapterWritePolicyDecision, AdapterWritePreflightRequest,
    AdapterWritePreflightRequestDefinition, AdapterWriteReadinessPolicy, AdapterWriteTarget,
    AdapterWriteTargetKind, ApprovalDecision, ApprovalDecisionKind,
    ApprovalDecisionProofEnforcementMode, ApprovalDecisionProofMarker,
    ApprovalDecisionProofMarkerDefinition, ApprovalDecisionProofValidationPolicy,
    ApprovalPresentationChannel, ApprovalPresentationId, ApprovalPresentationRecord,
    ApprovalPresentationRecordDefinition, ApprovalPresentationSensitivity, ApprovalRequest,
    Capability, CapabilityAvailability, CapabilityAvailabilityRecord, CapabilityDelegationPosture,
    CapabilityGrant, CapabilityGrantDefinition, CapabilityGrantId, CapabilityGrantLifecycle,
    CapabilityGrantRequirements, CapabilityGrantScope, CapabilityReference, CapabilityResolution,
    CapabilityResolutionInput, CapabilityResourceKind, CapabilityResourceScope, CorrelationId,
    EventId, EventSequenceNumber, GitHubDraftPullRequestContent,
    GitHubDraftPullRequestCreateOutcome, GitHubDraftPullRequestLookupResult,
    GitHubDraftPullRequestMutationInput, GitHubDraftPullRequestMutationStatus,
    GitHubDraftPullRequestObservation, GitHubDraftPullRequestProvider,
    GitHubDraftPullRequestProviderRequest, GitHubDraftPullRequestRefObservation,
    GitHubDraftPullRequestTarget, GitHubPullRequestCommentProviderAuth,
    GovernanceAssessmentBinding, GovernanceRuntimeFactSnapshotBindingVersion, IdempotencyKey,
    ImmutableRunBundleBinding, IntegrationId, LocalStateBackend, PolicyDecision, RedactionMetadata,
    SchemaVersion, SideEffectId, SideEffectLifecycleState, SideEffectReference,
    SideEffectReferenceKind, SideEffectSensitivity, SpecContentHash, StepId,
    TerminalLocalWorkReportInput, Timestamp, WorkReportArtifactRecord, WorkReportContractId,
    WorkReportContractVersion, WorkReportId, WorkReportSensitivity, WorkflowId, WorkflowOsError,
    WorkflowRun, WorkflowRunEvent, WorkflowRunEventKind, WorkflowRunId, WorkflowVersion,
    GITHUB_DRAFT_PULL_REQUEST_CREATE_CAPABILITY,
};

const HEAD_SHA: &str = "1111111111111111111111111111111111111111";
const BASE_SHA: &str = "2222222222222222222222222222222222222222";
const MOVED_SHA: &str = "3333333333333333333333333333333333333333";
const APPROVAL_ID: &str = "approval/run-draft-pr/create";
const POLICY_EVENT_ID: &str = "event/draft-pr-policy";

static STATE_COUNTER: AtomicU64 = AtomicU64::new(0);

struct TestState {
    backend: LocalStateBackend,
    root: PathBuf,
}

impl Drop for TestState {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn state() -> TestState {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let sequence = STATE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "workflow-os-draft-pr-{}-{nonce}-{sequence}",
        std::process::id()
    ));
    TestState {
        backend: LocalStateBackend::new(&root).expect("state backend"),
        root,
    }
}

fn time(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).expect("timestamp")
}

fn bundle() -> ImmutableRunBundleBinding {
    serde_json::from_value(json!({
        "bundle_id": "bundle/draft-pr",
        "bundle_version": "v1",
        "root_hash": SpecContentHash::from_text("draft-pr-bundle").as_str(),
    }))
    .expect("bundle binding")
}

fn governance_assessment() -> GovernanceAssessmentBinding {
    governance_assessment_at(time("2026-08-14T09:59:30Z"), time("2026-08-14T10:00:00Z"))
}

fn current_governance_assessment() -> GovernanceAssessmentBinding {
    governance_assessment_at(time("2026-08-14T10:01:30Z"), time("2026-08-14T10:02:00Z"))
}

fn governance_assessment_at(
    observed_at: Timestamp,
    evaluated_at: Timestamp,
) -> GovernanceAssessmentBinding {
    let bundle_binding = bundle();
    let registration_commitment = SpecContentHash::from_text("draft-pr-runtime-fact-source");
    let initial_snapshot_commitment =
        SpecContentHash::from_text(&format!("draft-pr-runtime-fact-snapshot-{evaluated_at}"));
    let runtime_fact_commitment = SpecContentHash::from_text("draft-pr-current-runtime-facts");
    let aggregate_fingerprint = SpecContentHash::from_text("draft-pr-assessment");
    let binding_commitment = hash_serializable(
        "workflow-os/governance-runtime-fact-snapshot-binding/v1",
        &(
            GovernanceRuntimeFactSnapshotBindingVersion::V1,
            &registration_commitment,
            &bundle_binding,
            &initial_snapshot_commitment,
            &runtime_fact_commitment,
            1_u32,
            observed_at,
            evaluated_at,
            300_u32,
            &aggregate_fingerprint,
        ),
    );
    serde_json::from_value(json!({
        "binding_version": "v3",
        "assessment_set_algorithm": "v1",
        "workflow_id": workflow_id().as_str(),
        "run_id": run_id().as_str(),
        "immutable_run_bundle": bundle_binding,
        "aggregate_fingerprint": aggregate_fingerprint,
        "step_count": 1,
        "execution": "require_approval",
        "disclosure": "visible",
        "completeness": "complete",
        "runtime_fact_snapshot_binding": {
            "binding_version": "v1",
            "source_registration_commitment": registration_commitment,
            "immutable_run_bundle": bundle(),
            "initial_snapshot_commitment": initial_snapshot_commitment,
            "runtime_fact_commitment": runtime_fact_commitment,
            "runtime_fact_count": 1,
            "observed_at": observed_at,
            "evaluated_at": evaluated_at,
            "effective_maximum_observation_age_seconds": 300,
            "assessment_aggregate_fingerprint": aggregate_fingerprint,
            "binding_commitment": binding_commitment,
        },
    }))
    .expect("governance assessment")
}

fn hash_serializable(domain: &str, value: &impl serde::Serialize) -> SpecContentHash {
    let bytes = serde_json::to_vec(value).expect("serialize commitment material");
    let mut hasher = Sha256::new();
    for (label, value) in [("domain", domain.as_bytes()), ("value", bytes.as_slice())] {
        for part in [label.as_bytes(), value] {
            hasher.update((part.len() as u64).to_be_bytes());
            hasher.update(part);
        }
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn actor() -> ActorId {
    ActorId::new("user/maintainer").expect("actor")
}

fn run_id() -> WorkflowRunId {
    WorkflowRunId::new("run-draft-pr").expect("run id")
}

fn workflow_id() -> WorkflowId {
    WorkflowId::new("workflow/draft-pr").expect("workflow id")
}

fn step_id() -> StepId {
    StepId::new("create-draft-pr").expect("step id")
}

fn target() -> GitHubDraftPullRequestTarget {
    GitHubDraftPullRequestTarget::new(
        "rcs2153",
        "workflow-os",
        "rcs2153",
        "codex/draft-pr",
        HEAD_SHA,
        "main",
        BASE_SHA,
    )
    .expect("target")
}

fn content() -> GitHubDraftPullRequestContent {
    GitHubDraftPullRequestContent::new(
        "v1",
        "Add bounded provider mutation",
        "Implements the approved draft-only provider slice.",
        "workflow-os-managed-draft-pr",
    )
    .expect("content")
}

fn approval_request() -> ApprovalRequest {
    ApprovalRequest {
        approval_id: APPROVAL_ID.to_owned(),
        run_id: run_id(),
        workflow_id: workflow_id(),
        schema_version: SchemaVersion::new("workflowos.dev/v0").expect("schema"),
        workflow_version: WorkflowVersion::new("v1").expect("version"),
        spec_content_hash: SpecContentHash::from_text("draft-pr-workflow"),
        resolved_execution_context_hash: None,
        step_id: Some(step_id()),
        skill_id: None,
        skill_version: None,
        governance_approval_binding: None,
        requested_by: ActorId::new("system/workflow-os").expect("requester"),
        correlation_id: CorrelationId::new("correlation/draft-pr").expect("correlation"),
        idempotency_key: Some(IdempotencyKey::new("approval-draft-pr").expect("key")),
        reason: "approve one bounded draft pull request create attempt".to_owned(),
        requested_at: time("2026-08-14T10:00:00Z"),
        expires_after: Some("30m".to_owned()),
        expires_at: None,
        decision: None,
    }
}

fn approval_decision() -> ApprovalDecision {
    let presentation = presentation(&content());
    ApprovalDecision {
        approval_id: APPROVAL_ID.to_owned(),
        actor: actor(),
        decided_at: time("2026-08-14T10:01:00Z"),
        decision: ApprovalDecisionKind::Granted,
        reason: "bounded draft pull request creation approved".to_owned(),
        correlation_id: CorrelationId::new("correlation/draft-pr").expect("correlation"),
        proof_marker: Some(
            ApprovalDecisionProofMarker::new(ApprovalDecisionProofMarkerDefinition {
                enforcement_mode:
                    ApprovalDecisionProofEnforcementMode::ApprovalPresentationRequired,
                presentation_id: presentation.presentation_id().clone(),
                presentation_content_hash: presentation.content_hash().clone(),
                proof_validated_at: time("2026-08-14T10:01:00Z"),
                proof_validation_policy:
                    ApprovalDecisionProofValidationPolicy::ApprovalPresentationRequestMatch,
                proof_age_ms: Some(30_000),
                proof_freshness_limit_ms: Some(1_800_000),
                proof_record_sensitivity: ApprovalPresentationSensitivity::Internal,
                redaction: RedactionMetadata::empty(),
            })
            .expect("proof marker"),
        ),
    }
}

fn policy_decision() -> PolicyDecision {
    PolicyDecision {
        allowed: true,
        requires_approval: true,
        reason_codes: vec!["policy.provider_write.approved".to_owned()],
        violations: Vec::new(),
        action: Action::InvokeAdapter,
        capabilities: vec![Capability::ExternalWrite, Capability::AdapterInvoke],
        actor: Some(actor()),
        workflow_id: Some(workflow_id()),
        run_id: Some(run_id()),
        correlation_id: Some(CorrelationId::new("correlation/draft-pr").expect("correlation")),
    }
}

fn event(sequence: u64, event_id: &str, kind: WorkflowRunEventKind) -> WorkflowRunEvent {
    let idempotency_key = matches!(&kind, WorkflowRunEventKind::GovernanceAssessmentBound(_))
        .then(|| IdempotencyKey::new("governance-assessment-draft-pr").expect("key"));
    WorkflowRunEvent {
        sequence_number: EventSequenceNumber::new(sequence).expect("sequence"),
        event_id: EventId::new(event_id).expect("event id"),
        timestamp: time("2026-08-14T10:00:00Z"),
        run_id: run_id(),
        workflow_id: workflow_id(),
        schema_version: SchemaVersion::new("workflowos.dev/v0").expect("schema"),
        workflow_version: WorkflowVersion::new("v1").expect("version"),
        spec_content_hash: SpecContentHash::from_text("draft-pr-workflow"),
        correlation_id: Some(CorrelationId::new("correlation/draft-pr").expect("correlation")),
        actor: Some(actor()),
        idempotency_key,
        kind,
    }
}

fn terminal_run_with(
    policy_decision: PolicyDecision,
    approval_decision: ApprovalDecision,
) -> WorkflowRun {
    WorkflowRun::rehydrate(&[
        event(
            1,
            "event/draft-pr-created",
            WorkflowRunEventKind::RunCreated {
                summary: None,
                immutable_run_bundle: Some(bundle()),
            },
        ),
        event(
            2,
            "event/draft-pr-governance",
            WorkflowRunEventKind::GovernanceAssessmentBound(Box::new(governance_assessment())),
        ),
        event(
            3,
            "event/draft-pr-validated",
            WorkflowRunEventKind::RunValidated,
        ),
        event(
            4,
            "event/draft-pr-started",
            WorkflowRunEventKind::RunStarted,
        ),
        event(
            5,
            POLICY_EVENT_ID,
            WorkflowRunEventKind::PolicyDecisionRecorded(Box::new(policy_decision)),
        ),
        event(
            6,
            "event/draft-pr-approval-requested",
            WorkflowRunEventKind::ApprovalRequested(Box::new(approval_request())),
        ),
        event(
            7,
            "event/draft-pr-approval-granted",
            WorkflowRunEventKind::ApprovalGranted(approval_decision),
        ),
        event(
            8,
            "event/draft-pr-resumed",
            WorkflowRunEventKind::RunResumed,
        ),
        event(
            9,
            "event/draft-pr-completed",
            WorkflowRunEventKind::RunCompleted,
        ),
    ])
    .expect("terminal run")
}

fn terminal_run() -> WorkflowRun {
    terminal_run_with(policy_decision(), approval_decision())
}

fn presentation(content: &GitHubDraftPullRequestContent) -> ApprovalPresentationRecord {
    let request = approval_request();
    let requested_action = format!(
        "create draft github pull request content {}",
        content.commitment()
    );
    let strict_non_goals = vec!["no merge or non-draft pull request".to_owned()];
    let touched = vec!["one GitHub repository pull request collection".to_owned()];
    let validation = vec!["observe exact refs before and after create".to_owned()];
    let channel = ApprovalPresentationChannel::Terminal;
    let sensitivity = ApprovalPresentationSensitivity::Internal;
    let hash = compute_approval_presentation_content_hash(
        &request.run_id,
        &request.approval_id,
        &request.workflow_id,
        Some(&request.workflow_version),
        Some(&request.schema_version),
        request.step_id.as_ref(),
        &requested_action,
        "create one bounded managed draft pull request",
        "exact repository, branches, content commitment, and single create attempt",
        &strict_non_goals,
        &touched,
        &validation,
        "the immutable governed run is complete",
        "perform lookup-before-create and reconcile the outcome",
        &channel,
        sensitivity,
    )
    .expect("presentation hash");
    ApprovalPresentationRecord::new(ApprovalPresentationRecordDefinition {
        presentation_id: ApprovalPresentationId::new("presentation/draft-pr").expect("id"),
        run_id: request.run_id,
        approval_id: request.approval_id,
        workflow_id: request.workflow_id,
        workflow_version: Some(request.workflow_version),
        schema_version: Some(request.schema_version),
        step_id: request.step_id,
        requested_action,
        work_summary: "create one bounded managed draft pull request".to_owned(),
        approved_scope: "exact repository, branches, content commitment, and single create attempt"
            .to_owned(),
        strict_non_goals,
        expected_touched_surfaces: touched,
        validation_expectations: validation,
        why_now: "the immutable governed run is complete".to_owned(),
        next_action: "perform lookup-before-create and reconcile the outcome".to_owned(),
        presented_at: time("2026-08-14T10:00:30Z"),
        presented_by: ActorId::new("system/workflow-os").expect("presenter"),
        channel,
        content_hash: hash,
        redaction: RedactionMetadata::empty(),
        sensitivity,
    })
    .expect("presentation")
}

fn capability_resolution() -> CapabilityResolution {
    capability_resolution_for(&target())
}

fn capability_resolution_for(target: &GitHubDraftPullRequestTarget) -> CapabilityResolution {
    let capability =
        CapabilityReference::new(GITHUB_DRAFT_PULL_REQUEST_CREATE_CAPABILITY).expect("capability");
    let resource = CapabilityResourceScope::new(
        CapabilityResourceKind::Repository,
        target.repository_reference(),
    )
    .expect("resource");
    let availability = CapabilityAvailabilityRecord::new(
        capability.clone(),
        resource.clone(),
        CapabilityAvailability::Available,
        time("2026-08-14T10:01:30Z"),
        RedactionMetadata::empty(),
    )
    .expect("availability");
    let grant = CapabilityGrant::new(CapabilityGrantDefinition {
        grant_id: CapabilityGrantId::new("grant/draft-pr").expect("grant id"),
        subject: actor(),
        capability: capability.clone(),
        resource: resource.clone(),
        scope: CapabilityGrantScope::new(workflow_id(), Some(run_id()), Some(step_id()), None)
            .expect("scope"),
        issuer: ActorId::new("user/authority-issuer").expect("issuer"),
        issued_at: time("2026-08-14T09:00:00Z"),
        expires_at: Some(time("2026-08-14T11:00:00Z")),
        lifecycle: CapabilityGrantLifecycle::Active,
        revocation_reference: None,
        delegation: CapabilityDelegationPosture::Disabled,
        requirements: CapabilityGrantRequirements::default(),
        sensitivity_ceiling: WorkReportSensitivity::Confidential,
        redaction: RedactionMetadata::empty(),
    })
    .expect("grant");
    resolve_capability_authority(&CapabilityResolutionInput {
        capability: &capability,
        resource: &resource,
        actor: &actor(),
        workflow_id: &workflow_id(),
        run_id: &run_id(),
        step_id: &step_id(),
        harness_contract_id: None,
        requested_sensitivity: WorkReportSensitivity::Internal,
        evaluated_at: time("2026-08-14T10:02:00Z"),
        availability_records: &[availability],
        grants: &[grant],
    })
    .expect("resolution")
}

fn preflight_for(target: &GitHubDraftPullRequestTarget) -> AdapterWritePreflightRequest {
    AdapterWritePreflightRequest::new(AdapterWritePreflightRequestDefinition {
        capability: AdapterWriteCapability::GitHubPullRequestCreate,
        target: AdapterWriteTarget::new(
            AdapterWriteTargetKind::GitHubRepository,
            target.operation_reference(),
        )
        .expect("write target"),
        side_effect_id: Some(SideEffectId::new("side-effect/draft-pr").expect("side effect")),
        idempotency_key: Some(IdempotencyKey::new("draft-pr-create-v1").expect("key")),
        policy_decision: AdapterWritePolicyDecision::Allowed,
        policy_references: vec![SideEffectReference::new(
            SideEffectReferenceKind::PolicyDecision,
            POLICY_EVENT_ID,
        )
        .expect("policy ref")],
        requires_approval: true,
        approval_references: vec![SideEffectReference::new(
            SideEffectReferenceKind::ApprovalDecision,
            APPROVAL_ID,
        )
        .expect("approval ref")],
        high_assurance_required: false,
        high_assurance_references: Vec::new(),
        summary: "create one bounded managed draft pull request".to_owned(),
        sensitivity: SideEffectSensitivity::Internal,
        redaction: RedactionMetadata::empty(),
        readiness_policy: AdapterWriteReadinessPolicy::local_sandbox_draft_pull_request_only(),
    })
    .expect("preflight")
}

fn report_artifact(run: &WorkflowRun) -> WorkReportArtifactRecord {
    let report = generate_terminal_local_work_report(TerminalLocalWorkReportInput {
        report_id: WorkReportId::new("work-report/draft-pr").expect("report id"),
        report_contract_id: WorkReportContractId::new("contract/draft-pr").expect("contract id"),
        report_contract_version: WorkReportContractVersion::new("v1").expect("contract version"),
        run,
        generated_at: time("2026-08-14T10:01:15Z"),
        generated_by: actor(),
        correlation_id: Some(CorrelationId::new("correlation/draft-pr").expect("correlation")),
        sensitivity: WorkReportSensitivity::Internal,
        redaction: RedactionMetadata::empty(),
        evidence_reference_ids: Vec::new(),
        validation_reference_ids: Vec::new(),
        local_check_result_references: Vec::new(),
        workflow_event_ids: Vec::new(),
        audit_event_ids: Vec::new(),
        adapter_telemetry_references: Vec::new(),
        policy_event_ids: Vec::new(),
        approval_reference_ids: Vec::new(),
        approval_proof_marker_citation_policy: None,
        high_assurance_approval: None,
        typed_handoff_ids: Vec::new(),
        agent_harness_hook_invocation_ids: Vec::new(),
        agent_harness_hook_disclosure_ids: Vec::new(),
        side_effect_ids: Vec::new(),
        github_pr_comment_provider_disclosures: Vec::new(),
        incomplete_work: vec!["Provider mutation closure remains explicit.".to_owned()],
        known_limitations: vec!["GitHub transport is injected.".to_owned()],
        risks: vec!["Branch refs remain mutable at the provider.".to_owned()],
        handoff_notes: vec!["Review the governed draft before readiness.".to_owned()],
    })
    .expect("terminal report");
    WorkReportArtifactRecord::new(report).expect("report artifact")
}

struct Provider {
    observations: RefCell<VecDeque<GitHubDraftPullRequestRefObservation>>,
    lookup: GitHubDraftPullRequestLookupResult,
    create: GitHubDraftPullRequestCreateOutcome,
    transport_calls: Cell<usize>,
    create_calls: Cell<usize>,
}

impl Provider {
    fn created() -> Self {
        let refs = refs(HEAD_SHA, BASE_SHA);
        Self {
            observations: RefCell::new(VecDeque::from([refs.clone(), refs.clone()])),
            lookup: GitHubDraftPullRequestLookupResult::NotFound,
            create: GitHubDraftPullRequestCreateOutcome::Created(observation(refs)),
            transport_calls: Cell::new(0),
            create_calls: Cell::new(0),
        }
    }
}

impl GitHubDraftPullRequestProvider for Provider {
    fn observe_refs(
        &self,
        _request: &GitHubDraftPullRequestProviderRequest,
    ) -> Result<GitHubDraftPullRequestRefObservation, WorkflowOsError> {
        self.transport_calls.set(self.transport_calls.get() + 1);
        self.observations
            .borrow_mut()
            .pop_front()
            .ok_or_else(|| WorkflowOsError::validation("test.observation.missing", "missing"))
    }

    fn lookup(
        &self,
        _request: &GitHubDraftPullRequestProviderRequest,
    ) -> Result<GitHubDraftPullRequestLookupResult, WorkflowOsError> {
        self.transport_calls.set(self.transport_calls.get() + 1);
        Ok(self.lookup.clone())
    }

    fn create(
        &self,
        _request: &GitHubDraftPullRequestProviderRequest,
    ) -> Result<GitHubDraftPullRequestCreateOutcome, WorkflowOsError> {
        self.transport_calls.set(self.transport_calls.get() + 1);
        self.create_calls.set(self.create_calls.get() + 1);
        Ok(self.create.clone())
    }
}

fn refs(head: &str, base: &str) -> GitHubDraftPullRequestRefObservation {
    GitHubDraftPullRequestRefObservation::new(head, base).expect("refs")
}

fn observation(refs: GitHubDraftPullRequestRefObservation) -> GitHubDraftPullRequestObservation {
    GitHubDraftPullRequestObservation::new("github/pull/42", true, refs, true).expect("observation")
}

fn execute(
    state: &LocalStateBackend,
    provider: &Provider,
    run: &WorkflowRun,
    resolution: &CapabilityResolution,
    content: GitHubDraftPullRequestContent,
    presentation: &ApprovalPresentationRecord,
) -> Result<workflow_core::GitHubDraftPullRequestMutationResult, WorkflowOsError> {
    let artifact = report_artifact(run);
    let current_assessment = current_governance_assessment();
    execute_with_artifact(
        state,
        provider,
        run,
        resolution,
        content,
        presentation,
        &artifact,
        &current_assessment,
    )
}

// Keeping each governed input explicit makes the security-focused tests readable.
#[allow(clippy::too_many_arguments)]
fn execute_with_artifact(
    state: &LocalStateBackend,
    provider: &Provider,
    run: &WorkflowRun,
    resolution: &CapabilityResolution,
    content: GitHubDraftPullRequestContent,
    presentation: &ApprovalPresentationRecord,
    artifact: &WorkReportArtifactRecord,
    current_assessment: &GovernanceAssessmentBinding,
) -> Result<workflow_core::GitHubDraftPullRequestMutationResult, WorkflowOsError> {
    execute_with_artifact_for_target(
        state,
        provider,
        run,
        resolution,
        content,
        presentation,
        artifact,
        current_assessment,
        target(),
        GitHubPullRequestCommentProviderAuth::new(
            "github-test-auth-secret",
            Some("repo-scoped draft pull request creation".to_owned()),
        )
        .expect("auth"),
    )
}

// The live smoke uses the same full governed fixture with a private concrete provider.
#[allow(clippy::too_many_arguments)]
fn execute_with_artifact_for_target<P: GitHubDraftPullRequestProvider>(
    state: &LocalStateBackend,
    provider: &P,
    run: &WorkflowRun,
    resolution: &CapabilityResolution,
    content: GitHubDraftPullRequestContent,
    presentation: &ApprovalPresentationRecord,
    artifact: &WorkReportArtifactRecord,
    current_assessment: &GovernanceAssessmentBinding,
    target: GitHubDraftPullRequestTarget,
    auth: GitHubPullRequestCommentProviderAuth,
) -> Result<workflow_core::GitHubDraftPullRequestMutationResult, WorkflowOsError> {
    let request = approval_request();
    execute_github_draft_pull_request_mutation(
        state,
        provider,
        &GitHubDraftPullRequestMutationInput {
            run,
            preflight: preflight_for(&target),
            capability_resolution: resolution,
            governance_assessment: run
                .snapshot
                .governance_assessment_binding
                .as_ref()
                .expect("governance assessment"),
            current_governance_assessment: current_assessment,
            approval_request: &request,
            approval_presentation: presentation,
            actor: actor(),
            step_id: step_id(),
            adapter_id: AdapterId::new("github").expect("adapter"),
            integration_id: IntegrationId::new("github/sandbox").expect("integration"),
            work_report_artifact: artifact,
            target,
            content,
            auth,
            proposed_at: time("2026-08-14T10:01:30Z"),
            attempted_at: time("2026-08-14T10:02:00Z"),
            outcome_at: time("2026-08-14T10:02:30Z"),
            correlation_id: Some(CorrelationId::new("correlation/draft-pr").expect("correlation")),
            sensitivity: SideEffectSensitivity::Internal,
            redaction: RedactionMetadata::empty(),
        },
    )
}

fn assert_live_completed_result(
    result: &workflow_core::GitHubDraftPullRequestMutationResult,
    created_is_allowed: bool,
) {
    if created_is_allowed {
        assert!(matches!(
            result.disclosure().status(),
            GitHubDraftPullRequestMutationStatus::Created
                | GitHubDraftPullRequestMutationStatus::ExistingManaged
        ));
    } else {
        assert_eq!(
            result.disclosure().status(),
            GitHubDraftPullRequestMutationStatus::ExistingManaged
        );
    }
    assert!(result.disclosure().lookup_performed());
    assert!(!result.disclosure().retry_blocked());
    assert_eq!(
        result
            .outcome_transition()
            .expect("completed outcome")
            .record()
            .lifecycle_state(),
        SideEffectLifecycleState::Completed
    );
    assert!(result.evidence().is_some());
    assert_eq!(result.report_citations().len(), 2);
}

#[test]
#[ignore = "requires explicit opt-in, a dedicated GitHub sandbox repository, and caller-supplied access"]
fn live_github_com_transport_creates_or_reconciles_one_exact_managed_draft() {
    const ENABLE: &str = "WORKFLOW_OS_GITHUB_DRAFT_PR_SANDBOX_SMOKE";
    const TOKEN: &str = "WORKFLOW_OS_GITHUB_DRAFT_PR_SANDBOX_TOKEN";
    const HEAD_BRANCH: &str = "WORKFLOW_OS_GITHUB_DRAFT_PR_SANDBOX_HEAD_BRANCH";
    const HEAD_SHA_ENV: &str = "WORKFLOW_OS_GITHUB_DRAFT_PR_SANDBOX_HEAD_SHA";
    const BASE_BRANCH: &str = "WORKFLOW_OS_GITHUB_DRAFT_PR_SANDBOX_BASE_BRANCH";
    const BASE_SHA_ENV: &str = "WORKFLOW_OS_GITHUB_DRAFT_PR_SANDBOX_BASE_SHA";
    const SANDBOX_OWNER: &str = "rcs2153";
    const SANDBOX_REPOSITORY: &str = "workflow-os-sandbox";

    assert_eq!(
        std::env::var(ENABLE).as_deref(),
        Ok("1"),
        "live sandbox smoke requires exact opt-in"
    );
    let required =
        |name: &str| std::env::var(name).expect("required live sandbox input is unavailable");
    let token = required(TOKEN);
    let head_branch = required(HEAD_BRANCH);
    let expected_head_sha = required(HEAD_SHA_ENV);
    let base_branch = required(BASE_BRANCH);
    let observed_base_sha = required(BASE_SHA_ENV);
    let live_target = GitHubDraftPullRequestTarget::new(
        SANDBOX_OWNER,
        SANDBOX_REPOSITORY,
        SANDBOX_OWNER,
        head_branch,
        expected_head_sha,
        base_branch,
        observed_base_sha,
    )
    .expect("validated allowlisted sandbox target");
    let live_content = GitHubDraftPullRequestContent::new(
        "sandbox-v1",
        "Workflow OS governed draft pull request sandbox proof",
        "This draft is persistent provider state created by an ignored Workflow OS sandbox smoke.",
        "workflow-os-github-draft-pr-sandbox-v1",
    )
    .expect("bounded smoke content");
    let run = terminal_run();
    let presentation = presentation(&live_content);
    let artifact = report_artifact(&run);
    let current_assessment = current_governance_assessment();
    let resolution = capability_resolution_for(&live_target);

    let execute_once = |state: &LocalStateBackend| {
        let provider = github_com_draft_pull_request_http_provider();
        execute_with_artifact_for_target(
            state,
            &provider,
            &run,
            &resolution,
            live_content.clone(),
            &presentation,
            &artifact,
            &current_assessment,
            live_target.clone(),
            GitHubPullRequestCommentProviderAuth::new(
                token.clone(),
                Some("allowlisted sandbox contents read and pull requests write".to_owned()),
            )
            .expect("explicit sandbox auth"),
        )
    };

    let first_state = state();
    let first = execute_once(&first_state.backend).expect("first create-or-reconcile result");
    assert_live_completed_result(&first, true);
    if first.disclosure().status() == GitHubDraftPullRequestMutationStatus::Created {
        assert!(first.disclosure().create_attempted());
        assert!(first.disclosure().post_create_observation_performed());
    } else {
        assert!(!first.disclosure().create_attempted());
    }

    let reconciliation_state = state();
    let reconciliation = execute_once(&reconciliation_state.backend)
        .expect("second independent lookup-before-create reconciliation");
    assert_live_completed_result(&reconciliation, false);
    assert!(!reconciliation.disclosure().create_attempted());
    let debug = format!("{first:?}{reconciliation:?}");
    assert!(!debug.contains(&token));
    assert!(!debug.contains(SANDBOX_REPOSITORY));
}

#[test]
fn created_draft_completes_once_with_evidence_and_report_citations() {
    let state = state();
    let provider = Provider::created();
    let run = terminal_run();
    let resolution = capability_resolution();
    let content = content();
    let presentation = presentation(&content);

    let result = execute(
        &state.backend,
        &provider,
        &run,
        &resolution,
        content,
        &presentation,
    )
    .expect("mutation");

    assert_eq!(
        result.disclosure().status(),
        GitHubDraftPullRequestMutationStatus::Created
    );
    assert_eq!(provider.create_calls.get(), 1);
    assert!(result.disclosure().post_create_observation_performed());
    assert!(!result.disclosure().retry_blocked());
    assert!(!result.disclosure().operator_action_required());
    assert_eq!(
        result
            .outcome_transition()
            .expect("outcome")
            .record()
            .lifecycle_state(),
        SideEffectLifecycleState::Completed
    );
    assert!(result.evidence().is_some());
    assert_eq!(result.report_citations().len(), 2);
    let debug = format!("{result:?}");
    assert!(!debug.contains("github-test-auth-secret"));
    assert!(!debug.contains("rcs2153"));
}

#[test]
fn existing_managed_draft_is_reused_without_create() {
    let state = state();
    let refs = refs(HEAD_SHA, BASE_SHA);
    let provider = Provider {
        observations: RefCell::new(VecDeque::from([refs.clone()])),
        lookup: GitHubDraftPullRequestLookupResult::ExactManaged(observation(refs)),
        create: GitHubDraftPullRequestCreateOutcome::Ambiguous {
            code: "must-not-run".to_owned(),
        },
        transport_calls: Cell::new(0),
        create_calls: Cell::new(0),
    };
    let run = terminal_run();
    let resolution = capability_resolution();
    let content = content();
    let presentation = presentation(&content);

    let result = execute(
        &state.backend,
        &provider,
        &run,
        &resolution,
        content,
        &presentation,
    )
    .expect("reconciled existing draft");

    assert_eq!(
        result.disclosure().status(),
        GitHubDraftPullRequestMutationStatus::ExistingManaged
    );
    assert_eq!(provider.create_calls.get(), 0);
    assert!(!result.disclosure().create_attempted());
}

#[test]
fn ambiguous_create_is_not_retried_and_does_not_claim_post_observation() {
    let state = state();
    let provider = Provider {
        observations: RefCell::new(VecDeque::from([refs(HEAD_SHA, BASE_SHA)])),
        lookup: GitHubDraftPullRequestLookupResult::NotFound,
        create: GitHubDraftPullRequestCreateOutcome::Ambiguous {
            code: "transport-outcome-unknown".to_owned(),
        },
        transport_calls: Cell::new(0),
        create_calls: Cell::new(0),
    };
    let run = terminal_run();
    let resolution = capability_resolution();
    let content = content();
    let presentation = presentation(&content);

    let result = execute(
        &state.backend,
        &provider,
        &run,
        &resolution,
        content,
        &presentation,
    )
    .expect("bounded ambiguous result");

    assert_eq!(
        result.disclosure().status(),
        GitHubDraftPullRequestMutationStatus::Ambiguous
    );
    assert_eq!(provider.create_calls.get(), 1);
    assert!(result.disclosure().retry_blocked());
    assert!(result.disclosure().operator_action_required());
    assert!(!result.disclosure().post_create_observation_performed());
    assert!(result.outcome_transition().is_none());
    assert_eq!(
        result
            .attempted_transition()
            .expect("attempted")
            .record()
            .lifecycle_state(),
        SideEffectLifecycleState::Attempted
    );
}

#[test]
fn known_rejection_requires_a_fresh_governed_attempt() {
    let state = state();
    let provider = Provider {
        observations: RefCell::new(VecDeque::from([refs(HEAD_SHA, BASE_SHA)])),
        lookup: GitHubDraftPullRequestLookupResult::NotFound,
        create: GitHubDraftPullRequestCreateOutcome::KnownRejected {
            code: "http-403".to_owned(),
        },
        transport_calls: Cell::new(0),
        create_calls: Cell::new(0),
    };
    let run = terminal_run();
    let resolution = capability_resolution();
    let content = content();
    let presentation = presentation(&content);

    let result = execute(
        &state.backend,
        &provider,
        &run,
        &resolution,
        content,
        &presentation,
    )
    .expect("bounded known rejection");

    assert_eq!(
        result.disclosure().status(),
        GitHubDraftPullRequestMutationStatus::KnownRejected
    );
    assert_eq!(provider.create_calls.get(), 1);
    assert!(result.disclosure().retry_blocked());
    assert!(result.disclosure().operator_action_required());
    assert_eq!(
        result
            .outcome_transition()
            .expect("failed outcome")
            .record()
            .lifecycle_state(),
        SideEffectLifecycleState::Failed
    );
}

#[test]
fn pre_create_ref_drift_fails_closed_before_create() {
    let state = state();
    let provider = Provider {
        observations: RefCell::new(VecDeque::from([refs(MOVED_SHA, BASE_SHA)])),
        lookup: GitHubDraftPullRequestLookupResult::NotFound,
        create: GitHubDraftPullRequestCreateOutcome::Ambiguous {
            code: "must-not-run".to_owned(),
        },
        transport_calls: Cell::new(0),
        create_calls: Cell::new(0),
    };
    let run = terminal_run();
    let resolution = capability_resolution();
    let content = content();
    let presentation = presentation(&content);

    let error = execute(
        &state.backend,
        &provider,
        &run,
        &resolution,
        content,
        &presentation,
    )
    .expect_err("drift must fail");

    assert_eq!(
        error.code(),
        "github_draft_pull_request.provider.pre_observation_drift"
    );
    assert_eq!(provider.create_calls.get(), 0);
}

#[test]
fn approval_must_bind_the_exact_content_commitment() {
    let state = state();
    let provider = Provider::created();
    let run = terminal_run();
    let resolution = capability_resolution();
    let approved_content = content();
    let presentation = presentation(&approved_content);
    let changed_content = GitHubDraftPullRequestContent::new(
        "v1",
        "Changed title after approval",
        "Implements the approved draft-only provider slice.",
        "workflow-os-managed-draft-pr",
    )
    .expect("changed content");

    let error = execute(
        &state.backend,
        &provider,
        &run,
        &resolution,
        changed_content,
        &presentation,
    )
    .expect_err("commitment drift must fail");

    assert_eq!(
        error.code(),
        "github_draft_pull_request.approval.content_commitment_mismatch"
    );
    assert_eq!(provider.transport_calls.get(), 0);
}

#[test]
fn secret_like_pull_request_content_is_rejected_without_leakage() {
    let marker = "ghp_1234567890abcdefghijklmnopqrstuvwxyz";
    let error = GitHubDraftPullRequestContent::new(
        "v1",
        "Safe title",
        marker,
        "workflow-os-managed-draft-pr",
    )
    .expect_err("secret-like body must fail");

    assert!(!error.to_string().contains(marker));
    assert!(!format!("{error:?}").contains(marker));
}

#[test]
fn policy_must_authorize_adapter_invocation_with_external_write_capabilities() {
    let state = state();
    let provider = Provider::created();
    let mut policy = policy_decision();
    policy.action = Action::StartWorkflow;
    policy.capabilities = vec![Capability::AdapterInvoke];
    let run = terminal_run_with(policy, approval_decision());
    let resolution = capability_resolution();
    let content = content();
    let presentation = presentation(&content);

    let error = execute(
        &state.backend,
        &provider,
        &run,
        &resolution,
        content,
        &presentation,
    )
    .expect_err("unrelated policy decision must not authorize provider mutation");

    assert_eq!(
        error.code(),
        "github_draft_pull_request.policy.decision_mismatch"
    );
    assert_eq!(provider.transport_calls.get(), 0);
}

#[test]
fn granted_approval_requires_the_exact_presentation_proof_marker() {
    let state = state();
    let provider = Provider::created();
    let mut decision = approval_decision();
    decision.proof_marker = None;
    let run = terminal_run_with(policy_decision(), decision);
    let resolution = capability_resolution();
    let content = content();
    let presentation = presentation(&content);

    let error = execute(
        &state.backend,
        &provider,
        &run,
        &resolution,
        content,
        &presentation,
    )
    .expect_err("marker-free approval must not authorize provider mutation");

    assert_eq!(
        error.code(),
        "github_draft_pull_request.approval.proof_marker_missing"
    );
    assert_eq!(provider.transport_calls.get(), 0);
}

#[test]
fn stale_runtime_fact_assessment_is_rejected_before_provider_use() {
    let state = state();
    let provider = Provider::created();
    let run = terminal_run();
    let resolution = capability_resolution();
    let content = content();
    let presentation = presentation(&content);
    let artifact = report_artifact(&run);
    let stale_assessment = governance_assessment();

    let error = execute_with_artifact(
        &state.backend,
        &provider,
        &run,
        &resolution,
        content,
        &presentation,
        &artifact,
        &stale_assessment,
    )
    .expect_err("stale reassessment must not authorize provider mutation");

    assert_eq!(
        error.code(),
        "github_draft_pull_request.governance.reassessment_stale"
    );
    assert_eq!(provider.transport_calls.get(), 0);
}
