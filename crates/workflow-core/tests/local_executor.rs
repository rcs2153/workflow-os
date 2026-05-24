#![allow(clippy::expect_used)]
//! Behavior tests for the first minimal local executor.

use std::cell::Cell;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    ActorId, ApprovalDecisionKind, ConservativePolicyEngine, CorrelationId, EventId, EventLogStore,
    EventSequenceNumber, FailingAuditSink, LocalApprovalDecisionRequest, LocalAuditSink,
    LocalCancellationRequest, LocalExecutionRequest, LocalExecutor, LocalObservabilitySink,
    LocalSkillRegistry, LocalStateBackend, LocalStructuredLogger, ObservabilityEventKind,
    SkillHandler, SkillId, SkillInput, SkillOutput, SkillVersion, SpecContentHash, StateBackend,
    TimeoutBehavior, Timestamp, WorkflowId, WorkflowOsError, WorkflowOsErrorKind, WorkflowRunEvent,
    WorkflowRunEventKind, WorkflowRunEventKindName, WorkflowRunId, WorkflowRunStatus,
    WorkflowVersion, SUPPORTED_SCHEMA_VERSION,
};

static NEXT_TEST_PROJECT: AtomicU64 = AtomicU64::new(1);

struct TestProject {
    root: PathBuf,
}

impl TestProject {
    fn new(name: &str) -> Self {
        let id = NEXT_TEST_PROJECT.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "workflow-os-executor-{name}-{}-{id}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("stale test project cleanup succeeds");
        }
        fs::create_dir_all(&root).expect("test project root is created");
        Self { root }
    }

    fn path(&self) -> &Path {
        &self.root
    }

    fn state_root(&self) -> PathBuf {
        self.root.join(".workflow-os-state")
    }

    fn write(&self, relative_path: &str, content: &str) {
        let path = self.root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent directory is created");
        }
        fs::write(path, content).expect("test file is written");
    }

    fn write_manifest(&self) {
        self.write(
            "workflow-os.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
project:
  id: acme/executor
  name: Acme Executor
layout:
  workflows: workflows
  skills: skills
  policies: policies
  tests: tests
"
            ),
        );
    }

    fn write_local_skill(&self) {
        self.write(
            "skills/echo.skill.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/echo
version: v0
display_name: Echo
owner:
  lifecycle_status: stable
input_contract:
  fields:
    - name: request
      field_type: string
  required:
    - request
output_contract:
  fields:
    - name: summary
      field_type: string
  required:
    - summary
failure_modes:
  - code: failed
    description: Handler failed.
evaluation_criteria:
  - name: deterministic
    description: Output is deterministic.
audit_requirements:
  required: true
  events:
    - SkillInvocationRequested
observability_requirements:
  metrics:
    - skill_latency
"
            ),
        );
    }

    fn write_external_skill(&self) {
        self.write(
            "skills/external.skill.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: symbolic/external-action
version: v0
display_name: External Action
owner:
  lifecycle_status: stable
input_contract:
  fields:
    - name: request
      field_type: string
  required:
    - request
output_contract:
  fields:
    - name: summary
      field_type: string
  required:
    - summary
allowed_capabilities:
  - name: external.write
adapter_requirements:
  - adapter_id: symbolic/external
    capabilities:
      - external.write
failure_modes:
  - code: failed
    description: External action failed.
evaluation_criteria:
  - name: reviewed
    description: External action is reviewed.
audit_requirements:
  required: true
  events:
    - SkillInvocationRequested
observability_requirements:
  metrics:
    - skill_latency
"
            ),
        );
    }

    fn write_policy(&self) {
        self.write(
            "policies/local.policy.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/allow
name: Local Allow
rules:
  - id: local-only
    effect: allow_local
  - id: approve
    effect: require_approval
  - id: retry
    effect: retry
  - id: bounded
    effect: bounded_retry
  - id: attempts
    effect: max_attempts=3
  - id: escalate
    effect: escalate
"
            ),
        );
    }

    fn write_workflow(&self, skill_id: &str) {
        self.write_workflow_with_autonomy(skill_id, "level_1");
    }

    fn write_workflow_with_autonomy(&self, skill_id: &str, autonomy_level: &str) {
        self.write_workflow_with_runtime_options(skill_id, autonomy_level, false, false, false);
    }

    fn write_approval_workflow(&self) {
        self.write_workflow_with_runtime_options("local/echo", "level_2", true, false, false);
    }

    fn write_retry_workflow(&self, escalates: bool) {
        self.write_workflow_with_runtime_options("local/echo", "level_1", false, true, escalates);
    }

    fn write_workflow_with_runtime_options(
        &self,
        skill_id: &str,
        autonomy_level: &str,
        approval_gated: bool,
        retry_enabled: bool,
        escalation_enabled: bool,
    ) {
        let approval_policy = if approval_gated {
            r"
    approval_policy:
      policy:
        id: local/allow"
        } else {
            ""
        };
        let approval_requirements = if approval_gated {
            r"
approval_requirements:
  - id: local-human-approval
    reason: Human approval required before local execution.
    expires_after:
      duration: 30m"
        } else {
            ""
        };
        let retry_policy = if retry_enabled {
            r"
    retry_policy:
      policy:
        id: local/allow"
        } else {
            ""
        };
        let escalation_policy = if escalation_enabled {
            r"
    escalation_policy:
      policy:
        id: local/allow"
        } else {
            ""
        };
        self.write(
            "workflows/main.workflow.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/main
version: v0
display_name: Local Main
owner:
  lifecycle_status: stable
autonomy_level: {autonomy_level}
triggers:
  - id: manual
    kind: manual
steps:
  - id: echo
    skill_ref:
      id: {skill_id}
      version: v0
    input_mapping:
      - from:
          type: literal
          value: hello
        to: request
    policy_requirements:
      - id: local/allow
{approval_policy}
{retry_policy}
{escalation_policy}
    timeout:
      duration: 1m
    terminal_behavior: fail_workflow
{approval_requirements}
timeout_policy:
  max_duration:
    duration: 1h
  on_timeout: escalate
cancellation_behavior: stop
audit_requirements:
  required: true
  events:
    - RunCreated
observability_requirements:
  metrics:
    - workflow_latency
"
            ),
        );
    }

    fn write_valid_project(&self) {
        self.write_manifest();
        self.write_policy();
        self.write_local_skill();
        self.write_workflow("local/echo");
    }

    fn write_approval_project(&self) {
        self.write_manifest();
        self.write_policy();
        self.write_local_skill();
        self.write_approval_workflow();
    }

    fn write_retry_project(&self, escalates: bool) {
        self.write_manifest();
        self.write_policy();
        self.write_local_skill();
        self.write_retry_workflow(escalates);
    }

    fn write_external_project(&self) {
        self.write_manifest();
        self.write_policy();
        self.write_external_skill();
        self.write_workflow_with_autonomy("symbolic/external-action", "level_2");
    }

    fn request(&self, run_id: Option<WorkflowRunId>) -> LocalExecutionRequest {
        LocalExecutionRequest {
            project_root: self.path().to_path_buf(),
            workflow_id: WorkflowId::new("local/main").expect("workflow id"),
            run_id,
            correlation_id: CorrelationId::new("correlation/local-executor").expect("correlation"),
            actor: ActorId::new("system/local-executor").expect("actor"),
        }
    }

    fn approval_request(
        &self,
        run_id: WorkflowRunId,
        approval_id: String,
        decision: ApprovalDecisionKind,
    ) -> LocalApprovalDecisionRequest {
        LocalApprovalDecisionRequest {
            project_root: self.path().to_path_buf(),
            run_id,
            approval_id,
            decision,
            actor: ActorId::new("user/approver").expect("actor"),
            reason: "manual local approval decision".to_owned(),
            correlation_id: CorrelationId::new("correlation/local-approval").expect("correlation"),
        }
    }

    fn cancellation_request(run_id: WorkflowRunId) -> LocalCancellationRequest {
        LocalCancellationRequest {
            run_id,
            actor: ActorId::new("user/canceler").expect("actor"),
            reason: "manual local cancellation".to_owned(),
            correlation_id: CorrelationId::new("correlation/local-cancel").expect("correlation"),
        }
    }
}

impl Drop for TestProject {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

struct EchoHandler {
    calls: Rc<Cell<u32>>,
}

impl SkillHandler for EchoHandler {
    fn invoke(&self, input: SkillInput) -> Result<SkillOutput, WorkflowOsError> {
        self.calls.set(self.calls.get() + 1);
        let mut values = BTreeMap::new();
        values.insert(
            "summary".to_owned(),
            input
                .values
                .get("request")
                .cloned()
                .unwrap_or_else(|| "empty".to_owned()),
        );
        Ok(SkillOutput::new(
            values,
            Some("local-handler-output/summary".to_owned()),
        ))
    }
}

struct FailingHandler;

impl SkillHandler for FailingHandler {
    fn invoke(&self, _input: SkillInput) -> Result<SkillOutput, WorkflowOsError> {
        Err(WorkflowOsError::new(
            WorkflowOsErrorKind::InvalidState,
            "test.skill.failed",
            "handler failed deterministically",
        ))
    }
}

struct SecretOutputHandler;

impl SkillHandler for SecretOutputHandler {
    fn invoke(&self, _input: SkillInput) -> Result<SkillOutput, WorkflowOsError> {
        let mut values = BTreeMap::new();
        values.insert("summary".to_owned(), "safe summary".to_owned());
        Ok(SkillOutput::new(
            values,
            Some("secret-token-should-not-log".to_owned()),
        ))
    }
}

struct TransientThenSuccessHandler {
    calls: Rc<Cell<u32>>,
    failures_before_success: u32,
}

impl SkillHandler for TransientThenSuccessHandler {
    fn invoke(&self, input: SkillInput) -> Result<SkillOutput, WorkflowOsError> {
        let next = self.calls.get() + 1;
        self.calls.set(next);
        if next <= self.failures_before_success {
            return Err(WorkflowOsError::new(
                WorkflowOsErrorKind::InvalidState,
                "test.skill.transient",
                "transient local skill failure",
            ));
        }
        let mut values = BTreeMap::new();
        values.insert(
            "summary".to_owned(),
            input
                .values
                .get("request")
                .cloned()
                .unwrap_or_else(|| "empty".to_owned()),
        );
        Ok(SkillOutput::new(
            values,
            Some("local-handler-output/retry-summary".to_owned()),
        ))
    }
}

fn registry(handler: Box<dyn SkillHandler>) -> LocalSkillRegistry {
    let mut registry = LocalSkillRegistry::new();
    registry.register(
        SkillId::new("local/echo").expect("skill id"),
        SkillVersion::new("v0").expect("skill version"),
        handler,
    );
    registry
}

#[test]
fn execute_valid_single_step_workflow() {
    let project = TestProject::new("valid");
    project.write_valid_project();
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::clone(&calls),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("run executes");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(run.events.len(), 9);
    assert_eq!(calls.get(), 1);
    assert!(run
        .events
        .iter()
        .all(|event| event.correlation_id.is_some()));
}

#[test]
fn skill_failure_produces_failed_run() {
    let project = TestProject::new("skill-failure");
    project.write_valid_project();
    let registry = registry(Box::new(FailingHandler));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("run records failure");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Failed);
    assert!(run.events.iter().any(|event| matches!(
        event.kind,
        WorkflowRunEventKind::SkillInvocationFailed { .. }
    )));
    assert_eq!(
        run.snapshot.failure.expect("failure").code,
        "test.skill.failed"
    );
}

#[test]
fn events_are_persisted_in_order() {
    let project = TestProject::new("event-order");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("run executes");
    let events = backend
        .read_events(&run.snapshot.identity.run_id)
        .expect("events are read");

    assert_eq!(events.len(), 9);
    for (index, event) in events.iter().enumerate() {
        assert_eq!(
            event.sequence_number.get(),
            u64::try_from(index + 1).expect("index fits")
        );
    }
}

#[test]
fn run_can_be_rehydrated_after_execution() {
    let project = TestProject::new("rehydrate");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("run executes");
    let rehydrated = backend
        .rehydrate_run(&run.snapshot.identity.run_id)
        .expect("run rehydrates");

    assert_eq!(rehydrated.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(rehydrated.snapshot.identity, run.snapshot.identity);
}

#[test]
fn duplicate_invocation_idempotency_returns_existing_run() {
    let project = TestProject::new("duplicate-idempotency");
    project.write_valid_project();
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::clone(&calls),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let run_id = WorkflowRunId::new("run/duplicate-local-executor").expect("run id");

    let first = executor
        .execute(&project.request(Some(run_id.clone())))
        .expect("first run executes");
    let second = executor
        .execute(&project.request(Some(run_id)))
        .expect("second run rehydrates");

    assert_eq!(first.snapshot.identity, second.snapshot.identity);
    assert_eq!(calls.get(), 1);
    assert_eq!(second.events.len(), 9);
}

#[test]
fn missing_skill_handler_fails_safely() {
    let project = TestProject::new("missing-handler");
    project.write_valid_project();
    let registry = LocalSkillRegistry::new();
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("missing handler is recorded as failed run");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Failed);
    assert_eq!(
        run.snapshot.failure.expect("failure").code,
        "executor.skill_handler.missing"
    );
}

#[test]
fn invalid_project_does_not_execute() {
    let project = TestProject::new("invalid");
    project.write_manifest();
    project.write_policy();
    project.write_local_skill();
    project.write(
        "workflows/main.workflow.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/main
version: v0
display_name: Invalid Main
owner:
  lifecycle_status: stable
cancellation_behavior: stop
audit_requirements:
  required: true
  events:
    - RunCreated
observability_requirements:
  metrics:
    - workflow_latency
"
        ),
    );
    let run_id = WorkflowRunId::new("run/invalid-project").expect("run id");
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let error = executor
        .execute(&project.request(Some(run_id.clone())))
        .expect_err("invalid project is rejected");
    let events = backend.read_events(&run_id).expect("events are read");

    assert_eq!(error.code(), "executor.project.invalid");
    assert!(events.is_empty());
}

#[test]
fn external_adapter_skill_is_rejected_without_side_effects() {
    let project = TestProject::new("external-rejected");
    project.write_external_project();
    let calls = Rc::new(Cell::new(0));
    let mut registry = LocalSkillRegistry::new();
    registry.register(
        SkillId::new("symbolic/external-action").expect("skill id"),
        SkillVersion::new("v0").expect("skill version"),
        Box::new(EchoHandler {
            calls: Rc::clone(&calls),
        }),
    );
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let run_id = WorkflowRunId::new("run/external-rejected").expect("run id");

    let run = executor
        .execute(&project.request(Some(run_id.clone())))
        .expect("external adapter denial is recorded");
    let events = backend.read_events(&run_id).expect("events are read");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Failed);
    assert_eq!(
        run.snapshot.failure.expect("failure").code,
        "policy.deny.adapter_invoke_v0"
    );
    assert!(events
        .iter()
        .any(|event| matches!(event.kind, WorkflowRunEventKind::PolicyDecisionRecorded(_))));
    assert_eq!(calls.get(), 0);
}

#[test]
fn kill_switch_prevents_new_execution() {
    let project = TestProject::new("kill-switch");
    project.write_valid_project();
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::clone(&calls),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new_with_policy(
        &backend,
        &registry,
        ConservativePolicyEngine::kill_switch(),
    );
    let run_id = WorkflowRunId::new("run/kill-switch").expect("run id");

    let error = executor
        .execute(&project.request(Some(run_id.clone())))
        .expect_err("kill switch denies execution");
    let events = backend.read_events(&run_id).expect("events are read");

    assert_eq!(error.code(), "policy.deny.kill_switch");
    assert!(events.is_empty());
    assert_eq!(calls.get(), 0);
}

#[test]
fn policy_decision_is_emitted_before_skill_action() {
    let project = TestProject::new("policy-before-action");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("run executes");
    let policy_position = run
        .events
        .iter()
        .position(|event| matches!(event.kind, WorkflowRunEventKind::PolicyDecisionRecorded(_)))
        .expect("policy decision event");
    let skill_started_position = run
        .events
        .iter()
        .position(|event| matches!(event.kind, WorkflowRunEventKind::SkillInvocationStarted(_)))
        .expect("skill started event");

    assert!(policy_position < skill_started_position);
}

#[test]
fn policy_decision_audit_event_is_recorded() {
    let project = TestProject::new("policy-audit");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("run executes");

    assert_eq!(run.snapshot.policy_decisions.len(), 1);
    assert!(run.snapshot.policy_decisions[0].allowed);
    assert!(run.snapshot.policy_decisions[0]
        .reason_codes
        .contains(&"policy.allow.default_conservative".to_owned()));
}

#[test]
fn approval_gated_workflow_pauses_before_skill_execution() {
    let project = TestProject::new("approval-pauses");
    project.write_approval_project();
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::clone(&calls),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("run pauses for approval");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::WaitingForApproval);
    assert_eq!(calls.get(), 0);
    assert_eq!(run.snapshot.approval_requests.len(), 1);
    assert!(run
        .events
        .iter()
        .any(|event| matches!(event.kind, WorkflowRunEventKind::ApprovalRequested(_))));
}

#[test]
fn approval_grant_resumes_and_executes_skill() {
    let project = TestProject::new("approval-grant");
    project.write_approval_project();
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::clone(&calls),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let paused = executor
        .execute(&project.request(None))
        .expect("run pauses for approval");
    let approval = paused.snapshot.approval_requests[0].clone();

    let completed = executor
        .decide_approval(project.approval_request(
            paused.snapshot.identity.run_id,
            approval.approval_id,
            ApprovalDecisionKind::Granted,
        ))
        .expect("approval resumes run");

    assert_eq!(completed.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(calls.get(), 1);
    assert!(completed
        .events
        .iter()
        .any(|event| matches!(event.kind, WorkflowRunEventKind::ApprovalGranted(_))));
    assert!(completed
        .events
        .iter()
        .any(|event| matches!(event.kind, WorkflowRunEventKind::RunResumed)));
    let decision = completed.snapshot.approval_requests[0]
        .decision
        .as_ref()
        .expect("decision");
    assert_eq!(decision.decision, ApprovalDecisionKind::Granted);
    assert_eq!(decision.reason, "manual local approval decision");
}

#[test]
fn approval_denial_fails_closed() {
    let project = TestProject::new("approval-deny");
    project.write_approval_project();
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::clone(&calls),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let paused = executor
        .execute(&project.request(None))
        .expect("run pauses for approval");
    let approval = paused.snapshot.approval_requests[0].clone();

    let failed = executor
        .decide_approval(project.approval_request(
            paused.snapshot.identity.run_id,
            approval.approval_id,
            ApprovalDecisionKind::Denied,
        ))
        .expect("denial fails run");

    assert_eq!(failed.snapshot.status, WorkflowRunStatus::Failed);
    assert_eq!(calls.get(), 0);
    assert!(failed
        .events
        .iter()
        .any(|event| matches!(event.kind, WorkflowRunEventKind::ApprovalDenied(_))));
    assert_eq!(
        failed.snapshot.failure.expect("failure").code,
        "executor.approval.denied"
    );
}

#[test]
fn duplicate_approval_grant_is_rejected_after_completion() {
    let project = TestProject::new("approval-duplicate");
    project.write_approval_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let paused = executor
        .execute(&project.request(None))
        .expect("run pauses for approval");
    let run_id = paused.snapshot.identity.run_id.clone();
    let approval_id = paused.snapshot.approval_requests[0].approval_id.clone();
    executor
        .decide_approval(project.approval_request(
            run_id.clone(),
            approval_id.clone(),
            ApprovalDecisionKind::Granted,
        ))
        .expect("first grant succeeds");

    let error = executor
        .decide_approval(project.approval_request(
            run_id,
            approval_id,
            ApprovalDecisionKind::Granted,
        ))
        .expect_err("duplicate grant is rejected");

    assert_eq!(error.code(), "executor.approval.terminal");
}

#[test]
fn approval_after_terminal_state_is_rejected() {
    let project = TestProject::new("approval-terminal");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let completed = executor
        .execute(&project.request(None))
        .expect("run completes");

    let error = executor
        .decide_approval(project.approval_request(
            completed.snapshot.identity.run_id,
            "approval/nonexistent".to_owned(),
            ApprovalDecisionKind::Granted,
        ))
        .expect_err("approval after terminal is rejected");

    assert_eq!(error.code(), "executor.approval.terminal");
}

#[test]
fn approval_state_survives_restart_and_rehydration() {
    let project = TestProject::new("approval-restart");
    project.write_approval_project();
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::clone(&calls),
    }));
    let state_root = project.state_root();
    let backend = LocalStateBackend::new(&state_root).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let paused = executor
        .execute(&project.request(None))
        .expect("run pauses for approval");

    let restarted_backend = LocalStateBackend::new(state_root).expect("state backend restarts");
    let rehydrated = restarted_backend
        .rehydrate_run(&paused.snapshot.identity.run_id)
        .expect("waiting run rehydrates");

    assert_eq!(
        rehydrated.snapshot.status,
        WorkflowRunStatus::WaitingForApproval
    );
    assert_eq!(rehydrated.snapshot.approval_requests.len(), 1);
    assert_eq!(calls.get(), 0);
}

#[test]
fn approval_audit_events_include_actor_timestamp_reason_and_correlation() {
    let project = TestProject::new("approval-audit");
    project.write_approval_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let paused = executor
        .execute(&project.request(None))
        .expect("run pauses for approval");
    let approval = paused.snapshot.approval_requests[0].clone();

    let completed = executor
        .decide_approval(project.approval_request(
            paused.snapshot.identity.run_id,
            approval.approval_id,
            ApprovalDecisionKind::Granted,
        ))
        .expect("approval succeeds");

    let granted = completed
        .events
        .iter()
        .find_map(|event| match &event.kind {
            WorkflowRunEventKind::ApprovalGranted(decision) => Some((event, decision)),
            _ => None,
        })
        .expect("approval granted event exists");
    assert!(granted.0.actor.is_some());
    assert!(granted.0.correlation_id.is_some());
    assert_eq!(granted.1.actor.as_str(), "user/approver");
    assert_eq!(granted.1.reason, "manual local approval decision");
    assert_eq!(
        granted.1.correlation_id.as_str(),
        "correlation/local-approval"
    );
}

#[test]
fn approval_event_sequence_rehydrates_through_state_model() {
    let project = TestProject::new("approval-sequence");
    project.write_approval_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let paused = executor
        .execute(&project.request(None))
        .expect("run pauses for approval");
    let approval = paused.snapshot.approval_requests[0].clone();
    let completed = executor
        .decide_approval(project.approval_request(
            paused.snapshot.identity.run_id,
            approval.approval_id,
            ApprovalDecisionKind::Granted,
        ))
        .expect("approval succeeds");

    let replayed = workflow_core::WorkflowRun::rehydrate(&completed.events).expect("replays");

    assert_eq!(replayed.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(
        replayed.snapshot.approval_requests[0]
            .decision
            .as_ref()
            .expect("decision")
            .decision,
        ApprovalDecisionKind::Granted
    );
}

#[test]
fn retry_succeeds_after_transient_failure() {
    let project = TestProject::new("retry-success");
    project.write_retry_project(false);
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(TransientThenSuccessHandler {
        calls: Rc::clone(&calls),
        failures_before_success: 1,
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("retry succeeds");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(calls.get(), 2);
    assert_eq!(run.snapshot.skill_invocations[0].attempts.len(), 2);
    assert!(run
        .events
        .iter()
        .any(|event| matches!(event.kind, WorkflowRunEventKind::RetryScheduled(_))));
    assert!(run
        .events
        .iter()
        .any(|event| matches!(event.kind, WorkflowRunEventKind::RetryStarted(_))));
}

#[test]
fn retry_exhaustion_escalates() {
    let project = TestProject::new("retry-escalates");
    project.write_retry_project(true);
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(TransientThenSuccessHandler {
        calls: Rc::clone(&calls),
        failures_before_success: 99,
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("retry exhaustion escalates");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Escalated);
    assert_eq!(calls.get(), 3);
    assert!(run
        .events
        .iter()
        .any(|event| matches!(event.kind, WorkflowRunEventKind::RetryExhausted(_))));
    assert_eq!(run.snapshot.escalations.len(), 1);
    assert_eq!(run.snapshot.escalations[0].attempts, 3);
}

#[test]
fn retry_exhaustion_fails_without_escalation_policy() {
    let project = TestProject::new("retry-fails");
    project.write_retry_project(false);
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(TransientThenSuccessHandler {
        calls: Rc::clone(&calls),
        failures_before_success: 99,
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("retry exhaustion fails");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Failed);
    assert_eq!(calls.get(), 3);
    assert!(run
        .events
        .iter()
        .any(|event| matches!(event.kind, WorkflowRunEventKind::RetryExhausted(_))));
    assert!(run.snapshot.escalations.is_empty());
}

#[test]
fn retry_attempt_events_are_ordered() {
    let project = TestProject::new("retry-order");
    project.write_retry_project(false);
    let registry = registry(Box::new(TransientThenSuccessHandler {
        calls: Rc::new(Cell::new(0)),
        failures_before_success: 1,
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("retry succeeds");
    let names = run
        .events
        .iter()
        .map(workflow_core::WorkflowRunEvent::kind)
        .collect::<Vec<_>>();
    let failed = names
        .iter()
        .position(|kind| *kind == workflow_core::WorkflowRunEventKindName::SkillInvocationFailed)
        .expect("failed event");
    let scheduled = names
        .iter()
        .position(|kind| *kind == workflow_core::WorkflowRunEventKindName::RetryScheduled)
        .expect("retry scheduled");
    let started = names
        .iter()
        .position(|kind| *kind == workflow_core::WorkflowRunEventKindName::RetryStarted)
        .expect("retry started");
    assert!(failed < scheduled);
    assert!(scheduled < started);
}

#[test]
fn duplicate_retry_run_does_not_repeat_side_effects() {
    let project = TestProject::new("retry-idempotent");
    project.write_retry_project(false);
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(TransientThenSuccessHandler {
        calls: Rc::clone(&calls),
        failures_before_success: 1,
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let run_id = WorkflowRunId::new("run/retry-idempotent").expect("run id");

    executor
        .execute(&project.request(Some(run_id.clone())))
        .expect("first execution succeeds");
    executor
        .execute(&project.request(Some(run_id)))
        .expect("second execution rehydrates");

    assert_eq!(calls.get(), 2);
}

#[test]
fn cancellation_from_running_state() {
    let project = TestProject::new("cancel-running");
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let registry = LocalSkillRegistry::new();
    let executor = LocalExecutor::new(&backend, &registry);
    let run_id = append_running_run(&backend);

    let run = executor
        .cancel_run(TestProject::cancellation_request(run_id))
        .expect("running run cancels");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Canceled);
    assert_eq!(
        run.snapshot.cancellation.expect("cancellation").reason,
        "manual local cancellation"
    );
}

#[test]
fn cancellation_from_waiting_approval_state() {
    let project = TestProject::new("cancel-approval");
    project.write_approval_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let paused = executor
        .execute(&project.request(None))
        .expect("run waits for approval");

    let canceled = executor
        .cancel_run(TestProject::cancellation_request(
            paused.snapshot.identity.run_id,
        ))
        .expect("waiting run cancels");

    assert_eq!(canceled.snapshot.status, WorkflowRunStatus::Canceled);
}

#[test]
fn cancellation_after_terminal_state_is_rejected() {
    let project = TestProject::new("cancel-terminal");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let completed = executor
        .execute(&project.request(None))
        .expect("run completes");

    let error = executor
        .cancel_run(TestProject::cancellation_request(
            completed.snapshot.identity.run_id,
        ))
        .expect_err("terminal cancellation rejected");

    assert_eq!(error.code(), "executor.cancellation.terminal");
}

#[test]
fn escalation_context_includes_required_fields() {
    let project = TestProject::new("escalation-context");
    project.write_retry_project(true);
    let registry = registry(Box::new(TransientThenSuccessHandler {
        calls: Rc::new(Cell::new(0)),
        failures_before_success: 99,
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("run escalates");
    let escalation = &run.snapshot.escalations[0];

    assert_eq!(escalation.run_id, run.snapshot.identity.run_id);
    assert_eq!(escalation.step_id.as_ref().expect("step").as_str(), "echo");
    assert_eq!(
        escalation.skill_id.as_ref().expect("skill").as_str(),
        "local/echo"
    );
    assert_eq!(escalation.last_error, "test.skill.transient");
    assert!(!escalation.suggested_next_action.is_empty());
}

#[test]
fn timeout_policy_is_parsed_and_represented() {
    let project = TestProject::new("timeout-policy");
    project.write_valid_project();

    let timeout = LocalExecutor::<LocalStateBackend>::timeout_policy_for_project(
        project.path().to_path_buf(),
        WorkflowId::new("local/main").expect("workflow id"),
    )
    .expect("timeout policy loads")
    .expect("timeout policy exists");

    assert_eq!(timeout.max_duration, "1h");
    assert_eq!(timeout.on_timeout, TimeoutBehavior::Escalate);
}

#[test]
fn audit_event_emitted_for_run_creation() {
    let project = TestProject::new("audit-run-created");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let audit = LocalAuditSink::new();
    let executor = LocalExecutor::new_with_sinks(
        &backend,
        &registry,
        ConservativePolicyEngine::new(),
        audit.clone(),
        LocalObservabilitySink::new(),
        LocalStructuredLogger::new(),
    );

    let run = executor
        .execute(&project.request(None))
        .expect("run executes");

    let events = audit.events();
    let created = events
        .iter()
        .find(|event| event.event_type == WorkflowRunEventKindName::RunCreated)
        .expect("run creation audit event");
    assert_eq!(created.workflow_run_id, run.snapshot.identity.run_id);
    assert_eq!(
        created
            .correlation_id
            .as_ref()
            .expect("correlation")
            .as_str(),
        "correlation/local-executor"
    );
}

#[test]
fn audit_event_emitted_for_policy_decision() {
    let project = TestProject::new("audit-policy");
    project.write_valid_project();
    let success_registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let audit = LocalAuditSink::new();
    let executor = LocalExecutor::new_with_sinks(
        &backend,
        &success_registry,
        ConservativePolicyEngine::new(),
        audit.clone(),
        LocalObservabilitySink::new(),
        LocalStructuredLogger::new(),
    );

    executor
        .execute(&project.request(None))
        .expect("run executes");

    let events = audit.events();
    let policy = events
        .iter()
        .find(|event| event.event_type == WorkflowRunEventKindName::PolicyDecisionRecorded)
        .expect("policy decision audit event");
    assert!(policy.policy_decision_reference.is_some());
    assert!(policy
        .decision_context
        .as_ref()
        .expect("decision context")
        .contains("allow"));
}

#[test]
fn audit_events_emitted_for_approval_request_and_decision() {
    let project = TestProject::new("audit-approval");
    project.write_approval_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let audit = LocalAuditSink::new();
    let executor = LocalExecutor::new_with_sinks(
        &backend,
        &registry,
        ConservativePolicyEngine::new(),
        audit.clone(),
        LocalObservabilitySink::new(),
        LocalStructuredLogger::new(),
    );
    let waiting = executor
        .execute(&project.request(None))
        .expect("run waits for approval");
    let approval_id = waiting.snapshot.approval_requests[0].approval_id.clone();

    executor
        .decide_approval(project.approval_request(
            waiting.snapshot.identity.run_id,
            approval_id,
            ApprovalDecisionKind::Granted,
        ))
        .expect("approval resumes");

    let event_types = audit
        .events()
        .into_iter()
        .map(|event| event.event_type)
        .collect::<Vec<_>>();
    assert!(event_types.contains(&WorkflowRunEventKindName::ApprovalRequested));
    assert!(event_types.contains(&WorkflowRunEventKindName::ApprovalGranted));
}

#[test]
fn audit_events_emitted_for_retry_and_escalation() {
    let project = TestProject::new("audit-retry-escalation");
    project.write_retry_project(true);
    let registry = registry(Box::new(TransientThenSuccessHandler {
        calls: Rc::new(Cell::new(0)),
        failures_before_success: 99,
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let audit = LocalAuditSink::new();
    let executor = LocalExecutor::new_with_sinks(
        &backend,
        &registry,
        ConservativePolicyEngine::new(),
        audit.clone(),
        LocalObservabilitySink::new(),
        LocalStructuredLogger::new(),
    );

    executor
        .execute(&project.request(None))
        .expect("run escalates");

    let event_types = audit
        .events()
        .into_iter()
        .map(|event| event.event_type)
        .collect::<Vec<_>>();
    assert!(event_types.contains(&WorkflowRunEventKindName::RetryStarted));
    assert!(event_types.contains(&WorkflowRunEventKindName::RetryExhausted));
    assert!(event_types.contains(&WorkflowRunEventKindName::EscalationTriggered));
}

#[test]
fn sensitive_fields_are_redacted_from_audit_and_logs() {
    let project = TestProject::new("audit-redaction");
    project.write_valid_project();
    let registry = registry(Box::new(SecretOutputHandler));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let audit = LocalAuditSink::new();
    let logger = LocalStructuredLogger::new();
    let executor = LocalExecutor::new_with_sinks(
        &backend,
        &registry,
        ConservativePolicyEngine::new(),
        audit.clone(),
        LocalObservabilitySink::new(),
        logger.clone(),
    );

    executor
        .execute(&project.request(None))
        .expect("run executes");

    let audit_text = format!("{:?}", audit.events());
    let log_text = format!("{:?}", logger.records());
    assert!(!audit_text.contains("secret-token-should-not-log"));
    assert!(!log_text.contains("secret-token-should-not-log"));
    assert!(audit_text.contains("[REDACTED]"));
}

#[test]
fn observability_event_emitted_for_skill_success_and_failure() {
    let project = TestProject::new("observability-skill");
    project.write_valid_project();
    let success_registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let observability = LocalObservabilitySink::new();
    let executor = LocalExecutor::new_with_sinks(
        &backend,
        &success_registry,
        ConservativePolicyEngine::new(),
        LocalAuditSink::new(),
        observability.clone(),
        LocalStructuredLogger::new(),
    );
    executor
        .execute(&project.request(None))
        .expect("run executes");
    assert!(observability
        .events()
        .iter()
        .any(|event| event.kind == ObservabilityEventKind::SkillInvocationSucceeded));

    let failure_project = TestProject::new("observability-skill-failure");
    failure_project.write_valid_project();
    let failing_registry = registry(Box::new(FailingHandler));
    let failure_backend =
        LocalStateBackend::new(failure_project.state_root()).expect("state backend");
    let failure_observability = LocalObservabilitySink::new();
    let failure_executor = LocalExecutor::new_with_sinks(
        &failure_backend,
        &failing_registry,
        ConservativePolicyEngine::new(),
        LocalAuditSink::new(),
        failure_observability.clone(),
        LocalStructuredLogger::new(),
    );
    failure_executor
        .execute(&failure_project.request(None))
        .expect("run records failure");
    assert!(failure_observability
        .events()
        .iter()
        .any(|event| event.kind == ObservabilityEventKind::SkillInvocationFailed));
}

#[test]
fn audit_sink_failure_is_returned() {
    let project = TestProject::new("audit-sink-failure");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new_with_sinks(
        &backend,
        &registry,
        ConservativePolicyEngine::new(),
        FailingAuditSink,
        LocalObservabilitySink::new(),
        LocalStructuredLogger::new(),
    );

    let error = executor
        .execute(&project.request(None))
        .expect_err("audit sink failure is surfaced");

    assert_eq!(error.code(), "audit.sink.failed");
}

fn append_running_run(backend: &impl StateBackend) -> WorkflowRunId {
    let run_id = WorkflowRunId::new("run/cancel-running").expect("run id");
    let workflow_id = WorkflowId::new("local/main").expect("workflow id");
    let workflow_version = WorkflowVersion::new("v0").expect("workflow version");
    let spec_hash = SpecContentHash::from_text("cancel running");
    for event in [
        running_event(
            1,
            &run_id,
            &workflow_id,
            &workflow_version,
            &spec_hash,
            WorkflowRunEventKind::RunCreated { summary: None },
        ),
        running_event(
            2,
            &run_id,
            &workflow_id,
            &workflow_version,
            &spec_hash,
            WorkflowRunEventKind::RunValidated,
        ),
        running_event(
            3,
            &run_id,
            &workflow_id,
            &workflow_version,
            &spec_hash,
            WorkflowRunEventKind::RunStarted,
        ),
    ] {
        backend.append_event(&event).expect("event appended");
    }
    run_id
}

fn running_event(
    sequence: u64,
    run_id: &WorkflowRunId,
    workflow_id: &WorkflowId,
    workflow_version: &WorkflowVersion,
    spec_hash: &SpecContentHash,
    kind: WorkflowRunEventKind,
) -> WorkflowRunEvent {
    WorkflowRunEvent {
        sequence_number: EventSequenceNumber::new(sequence).expect("sequence"),
        event_id: EventId::new(format!("event/cancel-running/{sequence}")).expect("event id"),
        timestamp: Timestamp::parse_rfc3339("2026-01-01T00:00:00Z").expect("timestamp"),
        run_id: run_id.clone(),
        workflow_id: workflow_id.clone(),
        workflow_version: workflow_version.clone(),
        spec_content_hash: spec_hash.clone(),
        correlation_id: Some(CorrelationId::new("correlation/cancel").expect("correlation")),
        actor: Some(ActorId::new("system/cancel-test").expect("actor")),
        idempotency_key: None,
        kind,
    }
}
