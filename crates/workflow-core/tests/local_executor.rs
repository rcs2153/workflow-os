#![allow(clippy::expect_used)]
//! Behavior tests for the first minimal local executor.

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use workflow_core::{
    ActorId, ApprovalDecisionKind, ApprovalRequest, ApprovalStore, ConservativePolicyEngine,
    CorrelationId, DocsCheckLocalHandler, EventId, EventLogStore, EventSequenceNumber,
    EvidenceReferenceId, FailingAuditSink, LocalApprovalDecisionRequest, LocalAuditSink,
    LocalCancellationRequest, LocalCheckCommandContract, LocalCheckProcessOutput,
    LocalCheckProcessRequest, LocalCheckProcessRunner, LocalCheckRegistrationProfile,
    LocalExecutionReportInputs, LocalExecutionRequest, LocalExecutionWithReportRequest,
    LocalExecutor, LocalObservabilitySink, LocalSkillRegistry, LocalStateBackend,
    LocalStructuredLogger, ObservabilityEventKind, PolicyAuditScope, PolicyAuditStore,
    RedactedValue, RedactionDisposition, RedactionFieldState, RedactionMetadata, SchemaVersion,
    SkillHandler, SkillId, SkillInput, SkillOutput, SkillVersion, SpecContentHash, StateBackend,
    StepId, TestOnlyWorkflowOsValidateDogfoodHandler, TimeoutBehavior, Timestamp, TypedHandoffId,
    ValidationReferenceId, WorkReportArtifactStore, WorkReportCitationKind,
    WorkReportCitationTarget, WorkReportContractId, WorkReportContractVersion, WorkReportId,
    WorkReportSectionKind, WorkReportSensitivity, WorkReportStableReference, WorkflowId,
    WorkflowOsError, WorkflowOsErrorKind, WorkflowRunEvent, WorkflowRunEventKind,
    WorkflowRunEventKindName, WorkflowRunId, WorkflowRunStatus, WorkflowVersion,
    SUPPORTED_SCHEMA_VERSION,
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

    fn write_local_check_skill(&self) {
        self.write(
            "skills/check-dogfood.skill.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/check-dogfood
version: v0
display_name: Check Dogfood
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
    - name: local_check_status
      field_type: string
  required:
    - summary
    - local_check_status
failure_modes:
  - code: failed
    description: Dogfood validation failed.
evaluation_criteria:
  - name: deterministic
    description: Output is bounded and deterministic.
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

    fn write_docs_check_skill(&self) {
        self.write(
            "skills/check-docs.skill.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/check-docs
version: v0
display_name: Check Docs
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
    - name: local_check_status
      field_type: string
  required:
    - summary
    - local_check_status
failure_modes:
  - code: failed
    description: Docs check failed.
evaluation_criteria:
  - name: deterministic
    description: Output is bounded and deterministic.
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

    fn write_two_step_workflow(&self) {
        self.write_multi_step_workflow(2);
    }

    fn write_three_step_workflow(&self) {
        self.write_multi_step_workflow(3);
    }

    fn write_multi_step_workflow(&self, step_count: u32) {
        let steps = (1..=step_count)
            .map(|step| {
                let terminal_behavior = if step == step_count {
                    "fail_workflow"
                } else {
                    "continue"
                };
                format!(
                    r"
  - id: echo-{step}
    skill_ref:
      id: local/echo
      version: v0
    input_mapping:
      - from:
          type: literal
          value: hello-{step}
        to: request
    policy_requirements:
      - id: local/allow
    timeout:
      duration: 1m
    terminal_behavior: {terminal_behavior}"
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
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
autonomy_level: level_1
triggers:
  - id: manual
    kind: manual
steps:
{steps}
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

    fn write_two_step_project(&self) {
        self.write_manifest();
        self.write_policy();
        self.write_local_skill();
        self.write_two_step_workflow();
    }

    fn write_three_step_project(&self) {
        self.write_manifest();
        self.write_policy();
        self.write_local_skill();
        self.write_three_step_workflow();
    }

    fn write_step_two_approval_project(&self) {
        self.write_manifest();
        self.write_policy();
        self.write_local_skill();
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
autonomy_level: level_2
triggers:
  - id: manual
    kind: manual
steps:
  - id: echo-1
    skill_ref:
      id: local/echo
      version: v0
    input_mapping:
      - from:
          type: literal
          value: hello-1
        to: request
    policy_requirements:
      - id: local/allow
    timeout:
      duration: 1m
    terminal_behavior: continue
  - id: echo-2
    skill_ref:
      id: local/echo
      version: v0
    input_mapping:
      - from:
          type: literal
          value: hello-2
        to: request
    policy_requirements:
      - id: local/allow
    approval_policy:
      policy:
        id: local/allow
    timeout:
      duration: 1m
    terminal_behavior: fail_workflow
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

    fn write_step_two_retry_project(&self, escalates: bool) {
        self.write_manifest();
        self.write_policy();
        self.write_local_skill();
        let escalation_policy = if escalates {
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
  escalation_contact: user/escalation
autonomy_level: level_1
triggers:
  - id: manual
    kind: manual
steps:
  - id: echo-1
    skill_ref:
      id: local/echo
      version: v0
    input_mapping:
      - from:
          type: literal
          value: hello-1
        to: request
    policy_requirements:
      - id: local/allow
    timeout:
      duration: 1m
    terminal_behavior: continue
  - id: echo-2
    skill_ref:
      id: local/echo
      version: v0
    input_mapping:
      - from:
          type: literal
          value: hello-2
        to: request
    policy_requirements:
      - id: local/allow
    retry_policy:
      policy:
        id: local/allow
{escalation_policy}
    timeout:
      duration: 1m
    terminal_behavior: fail_workflow
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

    fn write_step_two_policy_denied_project(&self) {
        self.write_manifest();
        self.write_policy();
        self.write_local_skill();
        self.write_external_skill();
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
autonomy_level: level_2
triggers:
  - id: manual
    kind: manual
steps:
  - id: echo-1
    skill_ref:
      id: local/echo
      version: v0
    input_mapping:
      - from:
          type: literal
          value: hello-1
        to: request
    policy_requirements:
      - id: local/allow
    timeout:
      duration: 1m
    terminal_behavior: continue
  - id: external-2
    skill_ref:
      id: symbolic/external-action
      version: v0
    input_mapping:
      - from:
          type: literal
          value: hello-2
        to: request
    policy_requirements:
      - id: local/allow
    timeout:
      duration: 1m
    terminal_behavior: fail_workflow
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

    fn write_branching_multi_step_project(&self) {
        self.write_two_step_project();
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
autonomy_level: level_1
triggers:
  - id: manual
    kind: manual
steps:
  - id: echo-1
    skill_ref:
      id: local/echo
      version: v0
    input_mapping:
      - from:
          type: literal
          value: hello-1
        to: request
    policy_requirements:
      - id: local/allow
    timeout:
      duration: 1m
    terminal_behavior: continue
  - id: echo-2
    skill_ref:
      id: local/echo
      version: v0
    input_mapping:
      - from:
          type: literal
          value: hello-2
        to: request
    policy_requirements:
      - id: local/allow
    timeout:
      duration: 1m
    terminal_behavior: fail_workflow
branches:
  - id: after-first
    condition: always
    target_step: echo-2
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

    fn write_external_project(&self) {
        self.write_manifest();
        self.write_policy();
        self.write_external_skill();
        self.write_workflow_with_autonomy("symbolic/external-action", "level_2");
    }

    fn write_local_check_project(&self) {
        self.write_manifest();
        self.write_policy();
        self.write_local_check_skill();
        self.write_workflow("local/check-dogfood");
    }

    fn write_docs_check_project(&self) {
        self.write_manifest();
        self.write_policy();
        self.write_docs_check_skill();
        self.write_workflow("local/check-docs");
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

#[derive(Clone)]
struct FakeLocalCheckRunner {
    output: LocalCheckProcessOutput,
    last_request: Arc<Mutex<Option<LocalCheckProcessRequest>>>,
}

impl FakeLocalCheckRunner {
    fn new(output: LocalCheckProcessOutput) -> Self {
        Self {
            output,
            last_request: Arc::new(Mutex::new(None)),
        }
    }
}

impl fmt::Debug for FakeLocalCheckRunner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FakeLocalCheckRunner")
    }
}

impl LocalCheckProcessRunner for FakeLocalCheckRunner {
    fn run(
        &self,
        request: &LocalCheckProcessRequest,
    ) -> Result<LocalCheckProcessOutput, WorkflowOsError> {
        *self.last_request.lock().expect("request lock") = Some(request.clone());
        Ok(self.output.clone())
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

struct StepAwareTransientHandler {
    calls: Rc<Cell<u32>>,
    invoked_steps: Rc<RefCell<Vec<String>>>,
    failing_step: &'static str,
    failures_before_success: usize,
}

impl SkillHandler for StepAwareTransientHandler {
    fn invoke(&self, input: SkillInput) -> Result<SkillOutput, WorkflowOsError> {
        self.calls.set(self.calls.get() + 1);
        self.invoked_steps
            .borrow_mut()
            .push(input.step_id.as_str().to_owned());
        let prior_failures_for_step = self
            .invoked_steps
            .borrow()
            .iter()
            .filter(|step_id| step_id.as_str() == self.failing_step)
            .count()
            .saturating_sub(1);
        if input.step_id.as_str() == self.failing_step
            && prior_failures_for_step < self.failures_before_success
        {
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
            Some(format!("local-handler-output/{}", input.step_id)),
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

fn local_check_registry(handler: TestOnlyWorkflowOsValidateDogfoodHandler) -> LocalSkillRegistry {
    let mut registry = LocalSkillRegistry::new();
    registry.register(
        SkillId::new("local/check-dogfood").expect("skill id"),
        SkillVersion::new("v0").expect("skill version"),
        Box::new(handler),
    );
    registry
}

fn docs_check_registry(handler: DocsCheckLocalHandler) -> LocalSkillRegistry {
    let mut registry = LocalSkillRegistry::new();
    registry
        .register_docs_check_handler(handler)
        .expect("docs check handler registration");
    registry
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn dogfood_project_root() -> PathBuf {
    repository_root().join("dogfood/workflow-os-self-governance")
}

fn dogfood_request(run_id: Option<WorkflowRunId>) -> LocalExecutionRequest {
    LocalExecutionRequest {
        project_root: dogfood_project_root(),
        workflow_id: WorkflowId::new("dg/d").expect("dogfood workflow id"),
        run_id,
        correlation_id: CorrelationId::new("correlation/dogfood-hardening")
            .expect("dogfood correlation"),
        actor: ActorId::new("system/dogfood-hardening-test").expect("dogfood actor"),
    }
}

fn dogfood_approval_request(
    run_id: WorkflowRunId,
    approval_id: String,
    decision: ApprovalDecisionKind,
) -> LocalApprovalDecisionRequest {
    LocalApprovalDecisionRequest {
        project_root: dogfood_project_root(),
        run_id,
        approval_id,
        decision,
        actor: ActorId::new("user/dogfood-reviewer").expect("dogfood reviewer"),
        reason: "bounded dogfood hardening decision".to_owned(),
        correlation_id: CorrelationId::new("correlation/dogfood-approval")
            .expect("dogfood approval correlation"),
    }
}

fn dogfood_execution_with_report_request(run_id: WorkflowRunId) -> LocalExecutionWithReportRequest {
    LocalExecutionWithReportRequest {
        execution: dogfood_request(Some(run_id)),
        report: LocalExecutionReportInputs {
            report_id: WorkReportId::new("report/dogfood-hardening").expect("dogfood report id"),
            report_contract_id: WorkReportContractId::new("governed-handoff/dogfood")
                .expect("dogfood contract id"),
            report_contract_version: WorkReportContractVersion::new("v1")
                .expect("dogfood contract version"),
            generated_at: Timestamp::now_utc(),
            generated_by: ActorId::new("system/dogfood-report-generator")
                .expect("dogfood report actor"),
            sensitivity: WorkReportSensitivity::Confidential,
            redaction: report_redaction(),
            correlation_id: Some(
                CorrelationId::new("correlation/dogfood-report").expect("dogfood report"),
            ),
            evidence_reference_ids: Vec::new(),
            validation_reference_ids: Vec::new(),
            local_check_result_references: Vec::new(),
            workflow_event_ids: Vec::new(),
            audit_event_ids: Vec::new(),
            adapter_telemetry_references: Vec::new(),
            policy_event_ids: Vec::new(),
            approval_reference_ids: Vec::new(),
            typed_handoff_ids: Vec::new(),
            incomplete_work: vec![
                "Real validation and implementation remain outside the kernel.".to_owned(),
            ],
            known_limitations: vec![
                "Dogfood execution uses deterministic placeholder local skill behavior.".to_owned(),
            ],
            risks: vec![
                "Report citations depend on explicitly supplied stable references.".to_owned(),
            ],
            handoff_notes: vec![
                "Review dogfood run history before broadening local check execution.".to_owned(),
            ],
        },
    }
}

fn dogfood_registry(calls: Rc<Cell<u32>>) -> LocalSkillRegistry {
    let mut registry = LocalSkillRegistry::new();
    registry.register(
        SkillId::new("local/d").expect("dogfood skill id"),
        SkillVersion::new("v0").expect("dogfood skill version"),
        Box::new(EchoHandler { calls }),
    );
    registry
}

fn workflow_os_binary() -> PathBuf {
    let binary_name = if cfg!(windows) {
        "workflow-os.exe"
    } else {
        "workflow-os"
    };
    std::env::current_exe()
        .expect("current test binary")
        .parent()
        .and_then(Path::parent)
        .expect("target debug directory")
        .join(binary_name)
}

fn event_position(
    events: &[WorkflowRunEvent],
    matches: impl Fn(&WorkflowRunEventKind) -> bool,
) -> Option<usize> {
    events.iter().position(|event| matches(&event.kind))
}

fn report_redaction() -> RedactionMetadata {
    RedactionMetadata::empty()
}

fn report_redaction_with(field: &str, reason: &str) -> RedactionMetadata {
    RedactionMetadata {
        redacted_fields: vec![field.to_owned()],
        field_states: vec![RedactionFieldState {
            field: field.to_owned(),
            disposition: RedactionDisposition::Redacted,
            reason: reason.to_owned(),
        }],
    }
}

fn report_inputs() -> LocalExecutionReportInputs {
    LocalExecutionReportInputs {
        report_id: WorkReportId::new("report/local-executor").expect("report id"),
        report_contract_id: WorkReportContractId::new("governed-handoff/local")
            .expect("contract id"),
        report_contract_version: WorkReportContractVersion::new("v1").expect("contract version"),
        generated_at: Timestamp::now_utc(),
        generated_by: ActorId::new("system/report-generator").expect("actor"),
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: report_redaction(),
        correlation_id: Some(CorrelationId::new("correlation/report").expect("correlation")),
        evidence_reference_ids: vec![
            EvidenceReferenceId::new("evidence/local-executor").expect("evidence id")
        ],
        validation_reference_ids: vec![
            ValidationReferenceId::new("validation/local-executor").expect("validation id")
        ],
        local_check_result_references: vec![WorkReportStableReference::new(
            "local-check-result/docs/passed",
        )
        .expect("local check ref")],
        workflow_event_ids: Vec::new(),
        audit_event_ids: Vec::new(),
        adapter_telemetry_references: vec![WorkReportStableReference::new(
            "adapter/local-executor",
        )
        .expect("adapter ref")],
        policy_event_ids: Vec::new(),
        approval_reference_ids: Vec::new(),
        typed_handoff_ids: vec![
            TypedHandoffId::new("typed-handoff/local-executor").expect("typed handoff id")
        ],
        incomplete_work: vec!["No deferred work beyond report artifacts.".to_owned()],
        known_limitations: vec!["Executor-integrated result is in memory only.".to_owned()],
        risks: vec!["Report citations depend on supplied stable IDs.".to_owned()],
        handoff_notes: vec![
            "Review generated report citations before artifact planning.".to_owned(),
        ],
    }
}

fn execution_with_report_request(project: &TestProject) -> LocalExecutionWithReportRequest {
    LocalExecutionWithReportRequest {
        execution: project.request(None),
        report: report_inputs(),
    }
}

fn section_summary(report: &workflow_core::WorkReport, kind: WorkReportSectionKind) -> &str {
    report
        .sections()
        .iter()
        .find(|section| section.kind() == kind)
        .and_then(workflow_core::WorkReportSection::summary)
        .expect("section summary")
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
    let step_scheduled = event_position(&run.events, |kind| {
        matches!(kind, WorkflowRunEventKind::StepScheduled { .. })
    })
    .expect("step scheduled event");
    let policy_recorded = event_position(&run.events, |kind| {
        matches!(kind, WorkflowRunEventKind::PolicyDecisionRecorded(_))
    })
    .expect("policy decision event");
    let invocation_requested = event_position(&run.events, |kind| {
        matches!(kind, WorkflowRunEventKind::SkillInvocationRequested(_))
    })
    .expect("skill invocation requested event");
    let invocation_started = event_position(&run.events, |kind| {
        matches!(kind, WorkflowRunEventKind::SkillInvocationStarted(_))
    })
    .expect("skill invocation started event");

    assert!(step_scheduled < policy_recorded);
    assert!(policy_recorded < invocation_requested);
    assert!(invocation_requested < invocation_started);
    assert!(run
        .events
        .iter()
        .all(|event| event.correlation_id.is_some()));
}

#[test]
fn execute_two_step_workflow_runs_steps_in_order() {
    let project = TestProject::new("two-step");
    project.write_two_step_project();
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::clone(&calls),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("two-step run executes");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(calls.get(), 2);
    let scheduled_steps = run
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::StepScheduled { step_id } => Some(step_id.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(scheduled_steps, ["echo-1", "echo-2"]);
    let succeeded_steps = run
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::SkillInvocationSucceeded { step_id, .. } => {
                Some(step_id.as_str())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(succeeded_steps, ["echo-1", "echo-2"]);
    let events = backend
        .read_events(&run.snapshot.identity.run_id)
        .expect("events read");
    assert_eq!(events, run.events);
}

#[test]
fn execute_three_step_workflow_emits_ordered_step_boundaries() {
    let project = TestProject::new("three-step");
    project.write_three_step_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("three-step run executes");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Completed);
    let scheduled_steps = run
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::StepScheduled { step_id } => Some(step_id.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(scheduled_steps, ["echo-1", "echo-2", "echo-3"]);
    for step in ["echo-1", "echo-2", "echo-3"] {
        let scheduled = event_position(&run.events, |kind| {
            matches!(kind, WorkflowRunEventKind::StepScheduled { step_id } if step_id.as_str() == step)
        })
        .expect("step scheduled");
        let requested = run
            .events
            .iter()
            .enumerate()
            .skip(scheduled + 1)
            .find_map(|(index, event)| match &event.kind {
                WorkflowRunEventKind::SkillInvocationRequested(invocation)
                    if invocation.step_id.as_str() == step =>
                {
                    Some(index)
                }
                _ => None,
            })
            .expect("skill requested");
        let policy = run
            .events
            .iter()
            .enumerate()
            .skip(scheduled + 1)
            .take(requested - scheduled - 1)
            .find_map(|(index, event)| {
                matches!(event.kind, WorkflowRunEventKind::PolicyDecisionRecorded(_))
                    .then_some(index)
            })
            .expect("step policy recorded");
        let succeeded = run
            .events
            .iter()
            .enumerate()
            .skip(requested + 1)
            .find_map(|(index, event)| match &event.kind {
                WorkflowRunEventKind::SkillInvocationSucceeded { step_id, .. }
                    if step_id.as_str() == step =>
                {
                    Some(index)
                }
                _ => None,
            })
            .expect("skill succeeded");
        assert!(scheduled < policy);
        assert!(policy < requested);
        assert!(requested < succeeded);
    }
}

#[test]
fn duplicate_multi_step_run_id_does_not_repeat_completed_steps() {
    let project = TestProject::new("two-step-duplicate");
    project.write_two_step_project();
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::clone(&calls),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let run_id = WorkflowRunId::generate();

    let first = executor
        .execute(&project.request(Some(run_id.clone())))
        .expect("first run executes");
    let duplicate = executor
        .execute(&project.request(Some(run_id)))
        .expect("duplicate run rehydrates");

    assert_eq!(first.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(duplicate.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(first.events, duplicate.events);
    assert_eq!(calls.get(), 2);
}

#[test]
fn execute_with_report_returns_completed_multi_step_run_plus_report() {
    let project = TestProject::new("execute-report-multi-step");
    project.write_two_step_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let result = executor
        .execute_with_report(&execution_with_report_request(&project))
        .expect("multi-step run executes with report result");

    assert_eq!(result.run().snapshot.status, WorkflowRunStatus::Completed);
    assert!(result.report_generation_error().is_none());
    let report = result.work_report().expect("report generated");
    assert_eq!(
        report.generation_context().terminal_run_status,
        workflow_core::WorkReportStatus::Completed
    );
    let section_kinds = report
        .sections()
        .iter()
        .map(workflow_core::WorkReportSection::kind)
        .collect::<Vec<_>>();
    assert_eq!(
        section_kinds,
        WorkReportSectionKind::v1_required_kinds().to_vec()
    );
    let scheduled_steps = result
        .run()
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::StepScheduled { step_id } => Some(step_id.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(scheduled_steps, ["echo-1", "echo-2"]);
    let events = backend
        .read_events(&result.run().snapshot.identity.run_id)
        .expect("events read");
    assert_eq!(events, result.run().events);
    assert!(backend
        .list_work_report_artifacts(&result.run().snapshot.identity.run_id)
        .expect("report artifacts listed")
        .is_empty());
}

#[test]
fn branching_multi_step_workflow_fails_closed_without_events() {
    let project = TestProject::new("branching-multi-step");
    project.write_branching_multi_step_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let run_id = WorkflowRunId::new("run/branching-unsupported").expect("run id");

    let error = executor
        .execute(&project.request(Some(run_id.clone())))
        .expect_err("branch execution is unsupported");

    assert_eq!(
        error.code(),
        "executor.workflow.multistep.unsupported_branching"
    );
    assert!(backend
        .read_events(&run_id)
        .expect("events read")
        .is_empty());
}

#[test]
fn later_step_approval_pauses_after_prior_step_completes() {
    let project = TestProject::new("step-two-approval-pause");
    project.write_step_two_approval_project();
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::clone(&calls),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let paused = executor
        .execute(&project.request(None))
        .expect("run pauses on step two");

    assert_eq!(
        paused.snapshot.status,
        WorkflowRunStatus::WaitingForApproval
    );
    assert_eq!(calls.get(), 1);
    assert_eq!(paused.snapshot.approval_requests.len(), 1);
    assert_eq!(
        paused.snapshot.approval_requests[0].step_id.as_str(),
        "echo-2"
    );
    let scheduled_steps = paused
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::StepScheduled { step_id } => Some(step_id.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(scheduled_steps, ["echo-1", "echo-2"]);
    let requested_steps = paused
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::SkillInvocationRequested(invocation) => {
                Some(invocation.step_id.as_str())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(requested_steps, ["echo-1"]);
}

#[test]
fn later_step_approval_grant_resumes_without_rerunning_prior_step() {
    let project = TestProject::new("step-two-approval-grant");
    project.write_step_two_approval_project();
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::clone(&calls),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let paused = executor
        .execute(&project.request(None))
        .expect("run pauses on step two");
    let approval = paused.snapshot.approval_requests[0].clone();

    let completed = executor
        .decide_approval(project.approval_request(
            paused.snapshot.identity.run_id,
            approval.approval_id,
            ApprovalDecisionKind::Granted,
        ))
        .expect("approval resumes step two");

    assert_eq!(completed.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(calls.get(), 2);
    let scheduled_steps = completed
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::StepScheduled { step_id } => Some(step_id.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(scheduled_steps, ["echo-1", "echo-2"]);
    let succeeded_steps = completed
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::SkillInvocationSucceeded { step_id, .. } => {
                Some(step_id.as_str())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(succeeded_steps, ["echo-1", "echo-2"]);
    let resume = event_position(&completed.events, |kind| {
        matches!(kind, WorkflowRunEventKind::RunResumed)
    })
    .expect("run resumed");
    let step_two_request = completed
        .events
        .iter()
        .enumerate()
        .skip(resume + 1)
        .find_map(|(index, event)| match &event.kind {
            WorkflowRunEventKind::SkillInvocationRequested(invocation)
                if invocation.step_id.as_str() == "echo-2" =>
            {
                Some(index)
            }
            _ => None,
        })
        .expect("step two requested after resume");
    assert!(resume < step_two_request);
}

#[test]
fn later_step_approval_denial_fails_without_invoking_denied_step() {
    let project = TestProject::new("step-two-approval-deny");
    project.write_step_two_approval_project();
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::clone(&calls),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let paused = executor
        .execute(&project.request(None))
        .expect("run pauses on step two");
    let approval = paused.snapshot.approval_requests[0].clone();

    let failed = executor
        .decide_approval(project.approval_request(
            paused.snapshot.identity.run_id,
            approval.approval_id,
            ApprovalDecisionKind::Denied,
        ))
        .expect("approval denial fails run");

    assert_eq!(failed.snapshot.status, WorkflowRunStatus::Failed);
    assert_eq!(calls.get(), 1);
    assert_eq!(
        failed.snapshot.failure.expect("failure").code,
        "executor.approval.denied"
    );
    assert!(!failed.events.iter().any(|event| matches!(
        &event.kind,
        WorkflowRunEventKind::SkillInvocationRequested(invocation)
            if invocation.step_id.as_str() == "echo-2"
    )));
}

#[test]
fn cancellation_while_waiting_on_later_step_preserves_prior_step_only() {
    let project = TestProject::new("step-two-cancel");
    project.write_step_two_approval_project();
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::clone(&calls),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let paused = executor
        .execute(&project.request(None))
        .expect("run pauses on step two");

    let canceled = executor
        .cancel_run(TestProject::cancellation_request(
            paused.snapshot.identity.run_id,
        ))
        .expect("waiting multi-step run cancels");

    assert_eq!(canceled.snapshot.status, WorkflowRunStatus::Canceled);
    assert_eq!(calls.get(), 1);
    assert!(!canceled.events.iter().any(|event| matches!(
        &event.kind,
        WorkflowRunEventKind::SkillInvocationRequested(invocation)
            if invocation.step_id.as_str() == "echo-2"
    )));
}

#[test]
fn later_step_retry_success_retries_only_current_step() {
    let project = TestProject::new("step-two-retry-success");
    project.write_step_two_retry_project(false);
    let calls = Rc::new(Cell::new(0));
    let invoked_steps = Rc::new(RefCell::new(Vec::new()));
    let registry = registry(Box::new(StepAwareTransientHandler {
        calls: Rc::clone(&calls),
        invoked_steps: Rc::clone(&invoked_steps),
        failing_step: "echo-2",
        failures_before_success: 1,
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("step two retry succeeds");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(calls.get(), 3);
    assert_eq!(
        invoked_steps.borrow().as_slice(),
        ["echo-1", "echo-2", "echo-2"]
    );
    assert!(run.events.iter().any(|event| matches!(
        &event.kind,
        WorkflowRunEventKind::RetryScheduled(record)
            if record.step_id.as_ref().is_some_and(|step_id| step_id.as_str() == "echo-2")
    )));
}

#[test]
fn later_step_retry_exhaustion_escalates_without_invoking_later_steps() {
    let project = TestProject::new("step-two-retry-escalates");
    project.write_step_two_retry_project(true);
    let calls = Rc::new(Cell::new(0));
    let invoked_steps = Rc::new(RefCell::new(Vec::new()));
    let registry = registry(Box::new(StepAwareTransientHandler {
        calls: Rc::clone(&calls),
        invoked_steps: Rc::clone(&invoked_steps),
        failing_step: "echo-2",
        failures_before_success: 99,
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("step two retry exhaustion escalates");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Escalated);
    assert_eq!(calls.get(), 4);
    assert_eq!(
        invoked_steps.borrow().as_slice(),
        ["echo-1", "echo-2", "echo-2", "echo-2"]
    );
    assert_eq!(run.snapshot.escalations.len(), 1);
    assert_eq!(
        run.snapshot.escalations[0]
            .step_id
            .as_ref()
            .expect("step")
            .as_str(),
        "echo-2"
    );
}

#[test]
fn policy_denial_on_later_step_stops_before_invocation() {
    let project = TestProject::new("step-two-policy-deny");
    project.write_step_two_policy_denied_project();
    let calls = Rc::new(Cell::new(0));
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::clone(&calls),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("step two policy denial records failed run");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Failed);
    assert_eq!(calls.get(), 1);
    assert_eq!(
        run.snapshot.failure.expect("failure").code,
        "policy.deny.adapter_invoke_v0"
    );
    let scheduled_steps = run
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::StepScheduled { step_id } => Some(step_id.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(scheduled_steps, ["echo-1", "external-2"]);
    assert!(!run.events.iter().any(|event| matches!(
        &event.kind,
        WorkflowRunEventKind::SkillInvocationRequested(invocation)
            if invocation.step_id.as_str() == "external-2"
    )));
}

#[test]
fn report_generation_failure_after_multi_step_preserves_completed_run() {
    let project = TestProject::new("multi-step-report-failure");
    project.write_two_step_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let mut request = execution_with_report_request(&project);
    request.report.handoff_notes = vec!["sk-test-secret-like-value".to_owned()];

    let result = executor
        .execute_with_report(&request)
        .expect("execution succeeds even when report generation fails");

    assert_eq!(result.run().snapshot.status, WorkflowRunStatus::Completed);
    assert!(result.work_report().is_none());
    let error = result.report_generation_error().expect("report error");
    assert!(error.code().contains("secret_like"));
    assert!(!format!("{error:?}").contains("sk-test-secret-like-value"));
    let scheduled_steps = result
        .run()
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::StepScheduled { step_id } => Some(step_id.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(scheduled_steps, ["echo-1", "echo-2"]);
    let events = backend
        .read_events(&result.run().snapshot.identity.run_id)
        .expect("events read");
    assert_eq!(events, result.run().events);
}

#[test]
fn dogfood_cancellation_while_waiting_on_planning_approval_stops_downstream_steps() {
    let state = TestProject::new("dogfood-cancel-planning");
    let calls = Rc::new(Cell::new(0));
    let registry = dogfood_registry(Rc::clone(&calls));
    let backend = LocalStateBackend::new(state.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let waiting = executor
        .execute(&dogfood_request(None))
        .expect("dogfood waits for planning approval");

    assert_eq!(
        waiting.snapshot.status,
        WorkflowRunStatus::WaitingForApproval
    );
    assert_eq!(calls.get(), 1);
    let approval_id = waiting.snapshot.approval_requests[0].approval_id.clone();
    assert!(approval_id.contains("/planning-approved"));

    let canceled = executor
        .cancel_run(TestProject::cancellation_request(
            waiting.snapshot.identity.run_id.clone(),
        ))
        .expect("dogfood waiting run cancels");

    assert_eq!(canceled.snapshot.status, WorkflowRunStatus::Canceled);
    assert_eq!(calls.get(), 1);
    let scheduled_steps = canceled
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::StepScheduled { step_id } => Some(step_id.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(scheduled_steps, ["scope-requested", "planning-approved"]);
    let requested_steps = canceled
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::SkillInvocationRequested(invocation) => {
                Some(invocation.step_id.as_str())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(requested_steps, ["scope-requested"]);
    assert!(!canceled.events.iter().any(|event| matches!(
        &event.kind,
        WorkflowRunEventKind::SkillInvocationRequested(invocation)
            if invocation.step_id.as_str() == "implementation-handoff"
                || invocation.step_id.as_str() == "validation-disclosure"
                || invocation.step_id.as_str() == "review-and-report-posture"
    )));
}

#[test]
fn dogfood_duplicate_run_id_rehydrates_completed_run_without_reinvoking_steps() {
    let state = TestProject::new("dogfood-duplicate-run");
    let calls = Rc::new(Cell::new(0));
    let registry = dogfood_registry(Rc::clone(&calls));
    let backend = LocalStateBackend::new(state.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let run_id = WorkflowRunId::generate();

    let waiting = executor
        .execute(&dogfood_request(Some(run_id.clone())))
        .expect("dogfood waits for approval");
    let approval_id = waiting.snapshot.approval_requests[0].approval_id.clone();
    let completed = executor
        .decide_approval(dogfood_approval_request(
            run_id.clone(),
            approval_id,
            ApprovalDecisionKind::Granted,
        ))
        .expect("dogfood approval completes run");
    let calls_after_completion = calls.get();
    let completed_events = completed.events.clone();

    let duplicate = executor
        .execute(&dogfood_request(Some(run_id)))
        .expect("duplicate dogfood run rehydrates");

    assert_eq!(completed.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(duplicate.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(duplicate.events, completed_events);
    assert_eq!(calls.get(), calls_after_completion);
    assert_eq!(calls_after_completion, 5);
    let succeeded_steps = duplicate
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            WorkflowRunEventKind::SkillInvocationSucceeded { step_id, .. } => {
                Some(step_id.as_str())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        succeeded_steps,
        [
            "scope-requested",
            "planning-approved",
            "implementation-handoff",
            "validation-disclosure",
            "review-and-report-posture"
        ]
    );
}

#[test]
fn dogfood_report_bearing_execution_uses_existing_explicit_api_without_artifacts() {
    let state = TestProject::new("dogfood-report-bearing");
    let calls = Rc::new(Cell::new(0));
    let registry = dogfood_registry(Rc::clone(&calls));
    let backend = LocalStateBackend::new(state.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let run_id = WorkflowRunId::generate();

    let waiting = executor
        .execute(&dogfood_request(Some(run_id.clone())))
        .expect("dogfood waits for approval");
    let approval_id = waiting.snapshot.approval_requests[0].approval_id.clone();
    let completed = executor
        .decide_approval(dogfood_approval_request(
            run_id.clone(),
            approval_id,
            ApprovalDecisionKind::Granted,
        ))
        .expect("dogfood approval completes run");

    let result = executor
        .execute_with_report(&dogfood_execution_with_report_request(run_id))
        .expect("completed dogfood run rehydrates with report result");

    assert_eq!(completed.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(result.run().snapshot.status, WorkflowRunStatus::Completed);
    assert!(result.report_generation_error().is_none());
    let report = result.work_report().expect("report generated");
    assert_eq!(
        report
            .sections()
            .iter()
            .map(workflow_core::WorkReportSection::kind)
            .collect::<Vec<_>>(),
        WorkReportSectionKind::v1_required_kinds().to_vec()
    );
    assert_eq!(
        section_summary(report, WorkReportSectionKind::ValidationAndQualityChecks),
        "No validation diagnostic or local check result references were supplied."
    );
    assert!(section_summary(report, WorkReportSectionKind::SideEffects).contains("unsupported"));
    assert!(backend
        .list_work_report_artifacts(&result.run().snapshot.identity.run_id)
        .expect("report artifacts listed")
        .is_empty());
    assert_eq!(calls.get(), 5);
}

#[test]
fn test_only_local_check_handler_executes_dogfood_validate_through_executor() {
    let project = TestProject::new("test-only-local-check");
    project.write_local_check_project();
    let contract =
        LocalCheckCommandContract::dogfood_validate_model_only().expect("valid dogfood contract");
    let handler = TestOnlyWorkflowOsValidateDogfoodHandler::new(
        contract,
        workflow_os_binary(),
        repository_root(),
    )
    .expect("test-only local check handler");
    let registry = local_check_registry(handler);
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("local check workflow executes");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(run.events.len(), 9);
    let success = run
        .events
        .iter()
        .find_map(|event| match &event.kind {
            WorkflowRunEventKind::SkillInvocationSucceeded { output_ref, .. } => {
                output_ref.as_deref()
            }
            _ => None,
        })
        .expect("skill success output ref");
    assert!(success.starts_with("local-check-result/local-check/dogfood-validate/passed"));
    let events = backend
        .read_events(&run.snapshot.identity.run_id)
        .expect("events read");
    assert_eq!(events, run.events);
    assert!(backend
        .list_work_report_artifacts(&run.snapshot.identity.run_id)
        .expect("artifacts list")
        .is_empty());
}

#[test]
fn docs_check_handler_is_not_registered_by_default() {
    let project = TestProject::new("docs-check-not-default");
    project.write_docs_check_project();
    let registry = LocalSkillRegistry::new();
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("missing handler returns failed run");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Failed);
    let failure = run.snapshot.failure.as_ref().expect("failure is recorded");
    assert_eq!(failure.code, "executor.skill_handler.missing");
    assert!(!failure.message.contains("check:docs"));
}

#[test]
fn local_check_registration_none_profile_keeps_registry_default_safe() {
    let project = TestProject::new("docs-check-profile-none");
    project.write_docs_check_project();
    let mut registry = LocalSkillRegistry::new();
    registry
        .register_local_check_profile(LocalCheckRegistrationProfile::none())
        .expect("none profile registers nothing");
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("missing handler returns failed run");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Failed);
    let failure = run.snapshot.failure.as_ref().expect("failure is recorded");
    assert_eq!(failure.code, "executor.skill_handler.missing");
}

#[test]
fn explicit_docs_check_handler_executes_through_executor_without_artifacts() {
    let project = TestProject::new("docs-check-explicit");
    project.write_docs_check_project();
    let runner = Arc::new(FakeLocalCheckRunner::new(
        LocalCheckProcessOutput::completed(Some(0), true, 12, b"docs passed".to_vec(), Vec::new()),
    ));
    let contract = LocalCheckCommandContract::docs_check_model_only().expect("valid docs contract");
    let handler = DocsCheckLocalHandler::new_with_process_runner(
        contract,
        workflow_os_binary(),
        repository_root(),
        Some(project.path().join(".npm-cache")),
        Arc::clone(&runner) as Arc<dyn LocalCheckProcessRunner>,
    )
    .expect("docs check handler");
    let registry = docs_check_registry(handler);
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("docs check workflow executes");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(run.events.len(), 9);
    let success = run
        .events
        .iter()
        .find_map(|event| match &event.kind {
            WorkflowRunEventKind::SkillInvocationSucceeded { output_ref, .. } => {
                output_ref.as_deref()
            }
            _ => None,
        })
        .expect("skill success output ref");
    assert!(success.starts_with("local-check-result/local-check/docs/passed"));
    let request = runner
        .last_request
        .lock()
        .expect("request lock")
        .clone()
        .expect("runner request captured");
    assert_eq!(
        request.arguments(),
        ["run".to_owned(), "check:docs".to_owned()]
    );
    assert!(request.environment().contains_key("PATH"));
    assert!(request.environment().contains_key("NPM_CONFIG_CACHE"));
    let events = backend
        .read_events(&run.snapshot.identity.run_id)
        .expect("events read");
    assert_eq!(events, run.events);
    assert!(backend
        .list_work_report_artifacts(&run.snapshot.identity.run_id)
        .expect("artifacts list")
        .is_empty());
}

#[test]
fn explicit_docs_check_registration_profile_executes_through_executor_without_artifacts() {
    let project = TestProject::new("docs-check-profile-explicit");
    project.write_docs_check_project();
    let runner = Arc::new(FakeLocalCheckRunner::new(
        LocalCheckProcessOutput::completed(Some(0), true, 12, b"docs passed".to_vec(), Vec::new()),
    ));
    let contract = LocalCheckCommandContract::docs_check_model_only().expect("valid docs contract");
    let handler = DocsCheckLocalHandler::new_with_process_runner(
        contract,
        workflow_os_binary(),
        repository_root(),
        Some(project.path().join(".npm-cache")),
        Arc::clone(&runner) as Arc<dyn LocalCheckProcessRunner>,
    )
    .expect("docs check handler");
    let mut registry = LocalSkillRegistry::new();
    registry
        .register_local_check_profile(LocalCheckRegistrationProfile::explicit_docs_check(handler))
        .expect("explicit docs check profile registers handler");
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("docs check workflow executes");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Completed);
    let request = runner
        .last_request
        .lock()
        .expect("request lock")
        .clone()
        .expect("process request captured");
    assert_eq!(request.arguments(), ["run", "check:docs"]);
    assert!(backend
        .list_work_report_artifacts(&run.snapshot.identity.run_id)
        .expect("artifacts list")
        .is_empty());
}

#[test]
fn execute_with_report_returns_completed_run_plus_report() {
    let project = TestProject::new("execute-report-completed");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let result = executor
        .execute_with_report(&execution_with_report_request(&project))
        .expect("run executes with report result");

    assert_eq!(result.run().snapshot.status, WorkflowRunStatus::Completed);
    let report = result.work_report().expect("report generated");
    assert!(result.report_generation_error().is_none());
    assert_eq!(
        report.generation_context().terminal_run_status,
        workflow_core::WorkReportStatus::Completed
    );
    let section_kinds: Vec<_> = report
        .sections()
        .iter()
        .map(workflow_core::WorkReportSection::kind)
        .collect();
    assert_eq!(
        section_kinds,
        WorkReportSectionKind::v1_required_kinds().to_vec()
    );
    let evidence_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::EvidenceConsidered)
        .expect("evidence section");
    assert!(evidence_section.citations().iter().any(|citation| matches!(
        citation.target(),
        WorkReportCitationTarget::EvidenceReference { .. }
    )));
    assert!(evidence_section
        .citations()
        .iter()
        .any(|citation| { citation.citation_kind() == WorkReportCitationKind::AdapterTelemetry }));
    let validation_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::ValidationAndQualityChecks)
        .expect("validation and quality section");
    assert!(validation_section.citations().iter().any(|citation| {
        matches!(
            citation.target(),
            WorkReportCitationTarget::LocalCheckResult { reference }
                if reference.as_str() == "local-check-result/docs/passed"
        ) && citation.citation_kind() == WorkReportCitationKind::LocalCheckResult
    }));
    let handoff_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::OperatorHandoffNotes)
        .expect("operator handoff section");
    assert!(handoff_section.citations().iter().any(|citation| {
        matches!(
            citation.target(),
            WorkReportCitationTarget::TypedHandoff { typed_handoff_id }
                if typed_handoff_id.as_str() == "typed-handoff/local-executor"
        ) && citation.citation_kind() == WorkReportCitationKind::TypedHandoff
    }));

    let events = backend
        .read_events(&result.run().snapshot.identity.run_id)
        .expect("events read");
    assert_eq!(events, result.run().events);
    assert!(backend
        .list_work_report_artifacts(&result.run().snapshot.identity.run_id)
        .expect("report artifacts listed")
        .is_empty());
}

#[test]
fn existing_execute_still_returns_workflow_run_only() {
    let project = TestProject::new("execute-still-run");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run: workflow_core::WorkflowRun = executor
        .execute(&project.request(None))
        .expect("existing execute returns run");

    assert_eq!(run.snapshot.status, WorkflowRunStatus::Completed);
}

#[test]
fn execute_with_report_returns_failed_run_plus_report() {
    let project = TestProject::new("execute-report-failed");
    project.write_valid_project();
    let registry = registry(Box::new(FailingHandler));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let result = executor
        .execute_with_report(&execution_with_report_request(&project))
        .expect("failed run returns report result");

    assert_eq!(result.run().snapshot.status, WorkflowRunStatus::Failed);
    assert!(result.work_report().is_some());
    assert!(result.report_generation_error().is_none());
    assert_eq!(
        result
            .work_report()
            .expect("report")
            .generation_context()
            .terminal_run_status,
        workflow_core::WorkReportStatus::Failed
    );
}

#[test]
fn execute_with_report_non_terminal_run_returns_report_error_without_report() {
    let project = TestProject::new("execute-report-non-terminal");
    project.write_approval_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let result = executor
        .execute_with_report(&execution_with_report_request(&project))
        .expect("waiting run returns result");

    assert_eq!(
        result.run().snapshot.status,
        WorkflowRunStatus::WaitingForApproval
    );
    assert!(result.work_report().is_none());
    let error = result
        .report_generation_error()
        .expect("report generation error");
    assert_eq!(error.code(), "work_report_generation.status.not_terminal");
    assert!(!format!("{error:?}").contains("correlation/report"));
    let events = backend
        .read_events(&result.run().snapshot.identity.run_id)
        .expect("events read");
    assert_eq!(events, result.run().events);
}

#[test]
fn execute_with_report_generation_failure_preserves_run_and_events() {
    let project = TestProject::new("execute-report-secret-input");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let mut request = execution_with_report_request(&project);
    request.report.handoff_notes = vec!["sk-test-secret-like-value".to_owned()];

    let result = executor
        .execute_with_report(&request)
        .expect("execution succeeds even when report generation fails");

    assert_eq!(result.run().snapshot.status, WorkflowRunStatus::Completed);
    assert!(result.work_report().is_none());
    let error = result.report_generation_error().expect("report error");
    assert!(error.code().contains("secret_like"));
    let debug = format!("{result:?}");
    assert!(!debug.contains("sk-test-secret-like-value"));
    assert!(!format!("{error:?}").contains("sk-test-secret-like-value"));
    let events = backend
        .read_events(&result.run().snapshot.identity.run_id)
        .expect("events read");
    assert_eq!(events, result.run().events);
    assert!(!project.path().join("work-report.json").exists());
}

#[test]
fn execute_with_report_absent_references_remain_not_available_text() {
    let project = TestProject::new("execute-report-absent-refs");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let mut request = execution_with_report_request(&project);
    request.report.evidence_reference_ids.clear();
    request.report.validation_reference_ids.clear();
    request.report.local_check_result_references.clear();
    request.report.adapter_telemetry_references.clear();
    request.report.audit_event_ids.clear();
    request.report.typed_handoff_ids.clear();

    let result = executor
        .execute_with_report(&request)
        .expect("run executes with report result");
    let report = result.work_report().expect("report generated");

    assert!(
        section_summary(report, WorkReportSectionKind::EvidenceConsidered)
            .contains("No evidence, audit, or adapter telemetry references were supplied")
    );
    assert!(
        section_summary(report, WorkReportSectionKind::ValidationAndQualityChecks)
            .contains("No validation diagnostic or local check result references were supplied")
    );
    assert!(section_summary(report, WorkReportSectionKind::SideEffects)
        .contains("No write side effects are supported"));
    assert!(report
        .sections()
        .iter()
        .flat_map(workflow_core::WorkReportSection::citations)
        .all(|citation| !citation.missing()));
    let handoff_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::OperatorHandoffNotes)
        .expect("operator handoff section");
    assert!(handoff_section.citations().is_empty());
}

#[test]
fn execute_with_report_cites_supplied_audit_event_ids_without_discovery() {
    let project = TestProject::new("execute-report-audit-citation");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let mut request = execution_with_report_request(&project);
    request.report.audit_event_ids =
        vec![EventId::new("audit-event/supplied-report-citation").expect("valid audit event id")];

    let result = executor
        .execute_with_report(&request)
        .expect("run executes with report result");
    let report = result.work_report().expect("report generated");
    let evidence_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::EvidenceConsidered)
        .expect("evidence section");

    assert!(evidence_section.citations().iter().any(|citation| {
        matches!(
            citation.target(),
            WorkReportCitationTarget::AuditEvent { audit_event_id }
                if audit_event_id.as_str() == "audit-event/supplied-report-citation"
        ) && citation.citation_kind() == WorkReportCitationKind::AuditEvent
            && !citation.missing()
    }));
}

#[test]
fn report_generation_failure_emits_no_report_audit_or_observability_events() {
    let project = TestProject::new("execute-report-no-report-signals");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let audit = LocalAuditSink::new();
    let observability = LocalObservabilitySink::new();
    let executor = LocalExecutor::new_with_sinks(
        &backend,
        &registry,
        ConservativePolicyEngine::new(),
        audit.clone(),
        observability.clone(),
        LocalStructuredLogger::new(),
    );
    let mut request = execution_with_report_request(&project);
    request.report.handoff_notes = vec!["sk-test-secret-like-value".to_owned()];

    let result = executor
        .execute_with_report(&request)
        .expect("execution succeeds even when report generation fails");

    assert_eq!(result.run().snapshot.status, WorkflowRunStatus::Completed);
    assert!(result.work_report().is_none());
    let error = result.report_generation_error().expect("report error");
    assert!(error.code().contains("secret_like"));
    assert_eq!(audit.events().len(), result.run().events.len());
    assert!(audit.policy_records().len() >= 2);
    assert!(audit.adapter_records().is_empty());
    let run_event_ids: Vec<_> = result
        .run()
        .events
        .iter()
        .map(|event| &event.event_id)
        .collect();
    let observability_events = observability.events();
    assert!(!observability_events.is_empty());
    assert!(observability_events.iter().all(|event| event
        .event_id
        .as_ref()
        .is_some_and(|event_id| run_event_ids.contains(&event_id))));
    assert!(observability.adapter_events().is_empty());
}

#[test]
fn execute_with_report_forwards_typed_handoff_ids_without_mutating_run_or_events() {
    let project = TestProject::new("execute-report-typed-handoff");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let request = execution_with_report_request(&project);

    let result = executor
        .execute_with_report(&request)
        .expect("run executes with report result");
    let report = result.work_report().expect("report generated");
    let handoff_section = report
        .sections()
        .iter()
        .find(|section| section.kind() == WorkReportSectionKind::OperatorHandoffNotes)
        .expect("operator handoff section");

    assert!(handoff_section.citations().iter().any(|citation| {
        matches!(
            citation.target(),
            WorkReportCitationTarget::TypedHandoff { typed_handoff_id }
                if typed_handoff_id.as_str() == "typed-handoff/local-executor"
        ) && citation.citation_kind() == WorkReportCitationKind::TypedHandoff
    }));
    let debug = format!("{result:?}");
    let serialized = serde_json::to_string(report).expect("report serializes");
    assert!(!debug.contains("typed-handoff/local-executor"));
    assert!(serialized.contains("typed-handoff/local-executor"));
    assert!(!serialized.contains("handoff obligation"));
    assert!(!serialized.contains("handoff disclosure"));
    assert!(!serialized.contains("raw provider payload"));

    let events = backend
        .read_events(&result.run().snapshot.identity.run_id)
        .expect("events read");
    assert_eq!(events, result.run().events);
    assert!(backend
        .list_work_report_artifacts(&result.run().snapshot.identity.run_id)
        .expect("report artifacts listed")
        .is_empty());
}

#[test]
fn execute_with_report_result_debug_is_redaction_safe() {
    let project = TestProject::new("execute-report-debug");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let mut request = execution_with_report_request(&project);
    request.report.redaction = report_redaction_with("summary", "reference only bounded summary");
    request.report.handoff_notes = vec!["bounded private handoff note".to_owned()];
    let request_debug = format!("{request:?}");
    assert!(!request_debug.contains("bounded private handoff note"));
    assert!(!request_debug.contains("reference only bounded summary"));
    assert!(!request_debug.contains("report/local-executor"));
    assert!(!request_debug.contains("typed-handoff/local-executor"));
    assert!(request_debug.contains("typed_handoff_count"));

    let result = executor
        .execute_with_report(&request)
        .expect("run executes with report result");
    let debug = format!("{result:?}");

    assert!(debug.contains("LocalExecutionWithReportResult"));
    assert!(!debug.contains("report/local-executor"));
    assert!(!debug.contains("reference only bounded summary"));
    assert!(!debug.contains("correlation/report"));
    assert!(!debug.contains("Operator should review cited references"));
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
    let skill_requested_position = run
        .events
        .iter()
        .position(|event| {
            matches!(
                event.kind,
                WorkflowRunEventKind::SkillInvocationRequested(_)
            )
        })
        .expect("skill requested event");
    let skill_started_position = run
        .events
        .iter()
        .position(|event| matches!(event.kind, WorkflowRunEventKind::SkillInvocationStarted(_)))
        .expect("skill started event");

    assert!(policy_position < skill_requested_position);
    assert!(policy_position < skill_started_position);
    assert!(skill_requested_position < skill_started_position);
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
    let step_scheduled = run
        .events
        .iter()
        .position(|event| matches!(event.kind, WorkflowRunEventKind::StepScheduled { .. }))
        .expect("step scheduled before approval");
    let approval_policy = run
        .events
        .iter()
        .position(|event| matches!(event.kind, WorkflowRunEventKind::PolicyDecisionRecorded(_)))
        .expect("approval policy decision");
    let approval_requested = run
        .events
        .iter()
        .position(|event| matches!(event.kind, WorkflowRunEventKind::ApprovalRequested(_)))
        .expect("approval requested");

    assert!(step_scheduled < approval_policy);
    assert!(approval_policy < approval_requested);
    assert!(step_scheduled < approval_requested);
    assert!(!run.events.iter().any(|event| matches!(
        event.kind,
        WorkflowRunEventKind::SkillInvocationRequested(_)
    )));
    assert!(!run
        .events
        .iter()
        .any(|event| matches!(event.kind, WorkflowRunEventKind::SkillInvocationStarted(_))));
}

#[test]
fn approval_request_event_precedes_projection_and_contains_identity_metadata() {
    let project = TestProject::new("approval-event-metadata");
    project.write_approval_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    let run = executor
        .execute(&project.request(None))
        .expect("run pauses for approval");
    let event_approval = run
        .events
        .iter()
        .find_map(|event| match &event.kind {
            WorkflowRunEventKind::ApprovalRequested(approval) => Some(approval.as_ref()),
            _ => None,
        })
        .expect("approval event");
    let projection = backend
        .load_approval_request(&event_approval.approval_id)
        .expect("load projection")
        .expect("projection saved after event");

    assert_eq!(event_approval.run_id, run.snapshot.identity.run_id);
    assert_eq!(
        event_approval.workflow_id,
        run.snapshot.identity.workflow_id
    );
    assert_eq!(
        event_approval.schema_version,
        run.snapshot.identity.schema_version
    );
    assert_eq!(
        event_approval.workflow_version,
        run.snapshot.identity.workflow_version
    );
    assert_eq!(
        event_approval.spec_content_hash,
        run.snapshot.identity.spec_content_hash
    );
    assert_eq!(event_approval.skill_version.as_str(), "v0");
    assert_eq!(
        event_approval.requested_by.as_str(),
        "system/local-executor"
    );
    assert_eq!(
        event_approval.correlation_id.as_str(),
        "correlation/local-executor"
    );
    assert!(event_approval.idempotency_key.is_some());
    assert_eq!(projection, *event_approval);
}

#[test]
fn approval_projection_can_be_rebuilt_from_event_log() {
    let project = TestProject::new("approval-projection-rebuild");
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

    backend
        .delete_approval_request(&approval.approval_id)
        .expect("delete projection");
    let completed = executor
        .decide_approval(project.approval_request(
            paused.snapshot.identity.run_id,
            approval.approval_id.clone(),
            ApprovalDecisionKind::Granted,
        ))
        .expect("approval uses event-backed request and rebuilds projection");
    let rebuilt = backend
        .load_approval_request(&approval.approval_id)
        .expect("load rebuilt projection")
        .expect("projection was rebuilt from event log");

    assert_eq!(completed.snapshot.status, WorkflowRunStatus::Completed);
    assert_eq!(rebuilt.run_id, completed.snapshot.identity.run_id);
}

#[test]
fn approval_projection_without_event_does_not_authorize_decision() {
    let project = TestProject::new("approval-projection-without-event");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);
    let completed = executor
        .execute(&project.request(None))
        .expect("run completes");
    let projection = ApprovalRequest {
        approval_id: "approval/orphan".to_owned(),
        run_id: completed.snapshot.identity.run_id.clone(),
        workflow_id: completed.snapshot.identity.workflow_id.clone(),
        schema_version: completed.snapshot.identity.schema_version.clone(),
        workflow_version: completed.snapshot.identity.workflow_version.clone(),
        spec_content_hash: completed.snapshot.identity.spec_content_hash.clone(),
        step_id: StepId::new("echo").expect("step"),
        skill_id: SkillId::new("local/echo").expect("skill"),
        skill_version: SkillVersion::new("v0").expect("skill version"),
        requested_by: ActorId::new("system/test").expect("actor"),
        correlation_id: CorrelationId::new("correlation/orphan").expect("correlation"),
        idempotency_key: None,
        reason: "orphan projection".to_owned(),
        requested_at: Timestamp::parse_rfc3339("2026-01-01T00:00:00Z").expect("timestamp"),
        expires_after: None,
        expires_at: None,
        decision: None,
    };
    backend
        .save_approval_request(&projection)
        .expect("save orphan projection");

    let error = executor
        .decide_approval(project.approval_request(
            completed.snapshot.identity.run_id,
            projection.approval_id,
            ApprovalDecisionKind::Granted,
        ))
        .expect_err("projection without event cannot bypass event truth");

    assert_eq!(error.code(), "executor.approval.terminal");
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
    let approval_granted = completed
        .events
        .iter()
        .position(|event| matches!(event.kind, WorkflowRunEventKind::ApprovalGranted(_)))
        .expect("approval granted");
    let run_resumed = completed
        .events
        .iter()
        .position(|event| matches!(event.kind, WorkflowRunEventKind::RunResumed))
        .expect("run resumed");
    let invocation_requested = completed
        .events
        .iter()
        .position(|event| {
            matches!(
                event.kind,
                WorkflowRunEventKind::SkillInvocationRequested(_)
            )
        })
        .expect("skill invocation requested");
    let invocation_started = completed
        .events
        .iter()
        .position(|event| matches!(event.kind, WorkflowRunEventKind::SkillInvocationStarted(_)))
        .expect("skill invocation started");
    let invocation_succeeded = completed
        .events
        .iter()
        .position(|event| {
            matches!(
                event.kind,
                WorkflowRunEventKind::SkillInvocationSucceeded { .. }
            )
        })
        .expect("skill invocation succeeded");

    assert!(approval_granted < run_resumed);
    assert!(run_resumed < invocation_requested);
    assert!(invocation_requested < invocation_started);
    assert!(invocation_started < invocation_succeeded);
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
fn approval_denial_after_terminal_state_is_rejected() {
    let project = TestProject::new("approval-denial-terminal");
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
            ApprovalDecisionKind::Denied,
        ))
        .expect_err("denial after terminal is rejected");

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
    assert_eq!(created.workflow_id, run.snapshot.identity.workflow_id);
    assert_eq!(created.schema_version, run.snapshot.identity.schema_version);
    assert_eq!(
        created.workflow_version,
        run.snapshot.identity.workflow_version
    );
    assert_eq!(created.spec_hash, run.snapshot.identity.spec_content_hash);
    assert!(created.actor.is_some());
    assert!(created.action.is_some());
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
    assert_eq!(policy.action, Some(workflow_core::Action::InvokeSkill));
    assert!(policy.correlation_id.is_some());
    assert!(policy.actor.is_some());
    assert!(policy
        .decision_context
        .as_ref()
        .expect("decision context")
        .contains("allow"));
    assert!(policy.redaction.field_states.iter().any(|field| {
        field.field == "decision_context" && field.disposition == RedactionDisposition::Safe
    }));
}

#[test]
fn allowed_start_policy_decision_is_durably_audited() {
    let project = TestProject::new("start-policy-allow-audit");
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
    let records = backend
        .read_policy_audit_records()
        .expect("policy audit records");
    let start = records
        .iter()
        .find(|record| record.scope == PolicyAuditScope::PreRun)
        .expect("pre-run start policy audit");

    assert_eq!(start.action, workflow_core::Action::StartWorkflow);
    assert!(start.allowed);
    assert_eq!(
        start.workflow_run_id.as_ref().expect("pending run id"),
        &run.snapshot.identity.run_id
    );
    assert_eq!(
        start.schema_version.as_ref().expect("schema version"),
        &run.snapshot.identity.schema_version
    );
    assert_eq!(
        start.workflow_version.as_ref().expect("workflow version"),
        &run.snapshot.identity.workflow_version
    );
    assert_eq!(
        start.spec_hash.as_ref().expect("spec hash"),
        &run.snapshot.identity.spec_content_hash
    );
    assert_eq!(
        start.correlation_id.as_ref().expect("correlation").as_str(),
        "correlation/local-executor"
    );
    assert!(audit
        .policy_records()
        .iter()
        .any(|record| record.audit_id == start.audit_id));
}

#[test]
fn denied_start_policy_decision_is_durably_audited_without_creating_run() {
    let project = TestProject::new("start-policy-deny-audit");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new_with_policy(
        &backend,
        &registry,
        ConservativePolicyEngine::kill_switch(),
    );
    let run_id = WorkflowRunId::new("run/denied-start-audit").expect("run id");

    let error = executor
        .execute(&project.request(Some(run_id.clone())))
        .expect_err("kill switch denies start");
    let records = backend
        .read_policy_audit_records()
        .expect("policy audit records");
    let events = backend.read_events(&run_id).expect("events are read");

    assert_eq!(error.code(), "policy.deny.kill_switch");
    assert!(events.is_empty());
    let denied = records
        .iter()
        .find(|record| record.scope == PolicyAuditScope::PreRun)
        .expect("denied pre-run policy audit");
    assert_eq!(denied.action, workflow_core::Action::StartWorkflow);
    assert!(!denied.allowed);
    assert!(denied
        .reason_codes
        .contains(&"policy.deny.kill_switch".to_owned()));
    assert_eq!(
        denied.workflow_run_id.as_ref().expect("pending run id"),
        &run_id
    );
    assert!(denied.workflow_event_id.is_none());
}

#[test]
fn skill_policy_allow_is_durably_audited() {
    let project = TestProject::new("skill-policy-allow-audit");
    project.write_valid_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    executor
        .execute(&project.request(None))
        .expect("run executes");
    let records = backend
        .read_policy_audit_records()
        .expect("policy audit records");

    assert!(records.iter().any(|record| {
        record.scope == PolicyAuditScope::Run
            && record.action == workflow_core::Action::InvokeSkill
            && record.allowed
            && record.workflow_event_id.is_some()
            && record.correlation_id.is_some()
            && record.actor.is_some()
            && record
                .reason_codes
                .contains(&"policy.allow.default_conservative".to_owned())
            && record.redaction.field_states.iter().any(|field| {
                field.field == "policy_context" && field.disposition == RedactionDisposition::Safe
            })
    }));
}

#[test]
fn skill_policy_deny_is_durably_audited() {
    let project = TestProject::new("skill-policy-deny-audit");
    project.write_external_project();
    let mut registry = LocalSkillRegistry::new();
    registry.register(
        SkillId::new("symbolic/external-action").expect("skill id"),
        SkillVersion::new("v0").expect("skill version"),
        Box::new(EchoHandler {
            calls: Rc::new(Cell::new(0)),
        }),
    );
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    executor
        .execute(&project.request(None))
        .expect("denial is recorded as failed run");
    let records = backend
        .read_policy_audit_records()
        .expect("policy audit records");

    assert!(records.iter().any(|record| {
        record.scope == PolicyAuditScope::Run
            && record.action == workflow_core::Action::InvokeAdapter
            && !record.allowed
            && record
                .reason_codes
                .contains(&"policy.deny.adapter_invoke_v0".to_owned())
    }));
}

#[test]
fn approval_required_policy_is_durably_audited() {
    let project = TestProject::new("approval-policy-audit");
    project.write_approval_project();
    let registry = registry(Box::new(EchoHandler {
        calls: Rc::new(Cell::new(0)),
    }));
    let backend = LocalStateBackend::new(project.state_root()).expect("state backend");
    let executor = LocalExecutor::new(&backend, &registry);

    executor
        .execute(&project.request(None))
        .expect("run pauses for approval");
    let records = backend
        .read_policy_audit_records()
        .expect("policy audit records");

    assert!(records.iter().any(|record| {
        record.scope == PolicyAuditScope::Run
            && record.action == workflow_core::Action::RequestApproval
            && record.allowed
            && record.requires_approval
            && record
                .reason_codes
                .contains(&"policy.requires_approval".to_owned())
    }));
}

#[test]
fn skill_invocation_audit_includes_skill_version_and_references() {
    let project = TestProject::new("audit-skill-metadata");
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

    executor
        .execute(&project.request(None))
        .expect("run executes");
    let events = audit.events();
    let requested = events
        .iter()
        .find(|event| event.event_type == WorkflowRunEventKindName::SkillInvocationRequested)
        .expect("skill requested audit event");
    let succeeded = events
        .iter()
        .find(|event| event.event_type == WorkflowRunEventKindName::SkillInvocationSucceeded)
        .expect("skill succeeded audit event");

    for event in [requested, succeeded] {
        assert_eq!(event.step_id.as_ref().expect("step").as_str(), "echo");
        assert_eq!(
            event.skill_id.as_ref().expect("skill").as_str(),
            "local/echo"
        );
        assert_eq!(
            event
                .skill_version
                .as_ref()
                .expect("skill version")
                .as_str(),
            "v0"
        );
        assert!(event.correlation_id.is_some());
        assert!(event.actor.is_some());
        assert!(event.idempotency_key.is_some());
    }
    assert!(requested.input_reference.is_some());
    assert!(requested.redaction.field_states.iter().any(|field| {
        field.field == "input_reference" && field.disposition == RedactionDisposition::ReferenceOnly
    }));
    assert!(succeeded.output_reference.is_some());
    assert!(succeeded.redaction.field_states.iter().any(|field| {
        field.field == "output_reference"
            && field.disposition == RedactionDisposition::ReferenceOnly
    }));
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
    let approval_requested = event_types
        .iter()
        .position(|event_type| *event_type == WorkflowRunEventKindName::ApprovalRequested)
        .expect("approval requested audit event");
    let approval_granted = event_types
        .iter()
        .position(|event_type| *event_type == WorkflowRunEventKindName::ApprovalGranted)
        .expect("approval granted audit event");
    let run_resumed = event_types
        .iter()
        .position(|event_type| *event_type == WorkflowRunEventKindName::RunResumed)
        .expect("run resumed audit event");
    let skill_requested = event_types
        .iter()
        .position(|event_type| *event_type == WorkflowRunEventKindName::SkillInvocationRequested)
        .expect("skill invocation requested audit event");

    assert!(approval_requested < approval_granted);
    assert!(approval_granted < run_resumed);
    assert!(run_resumed < skill_requested);
}

#[test]
fn approval_audit_events_include_actor_reason_and_identity_metadata() {
    let project = TestProject::new("audit-approval-metadata");
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
    let events = audit.events();
    let requested = events
        .iter()
        .find(|event| event.event_type == WorkflowRunEventKindName::ApprovalRequested)
        .expect("approval requested audit event");
    let granted = events
        .iter()
        .find(|event| event.event_type == WorkflowRunEventKindName::ApprovalGranted)
        .expect("approval granted audit event");

    assert_eq!(
        requested.schema_version,
        waiting.snapshot.identity.schema_version
    );
    assert_eq!(
        requested.workflow_version,
        waiting.snapshot.identity.workflow_version
    );
    assert_eq!(
        requested.spec_hash,
        waiting.snapshot.identity.spec_content_hash
    );
    assert_eq!(
        requested
            .skill_version
            .as_ref()
            .expect("skill version")
            .as_str(),
        "v0"
    );
    assert!(requested
        .decision_context
        .as_ref()
        .expect("request reason")
        .contains("approval requested"));
    assert_eq!(
        granted.actor.as_ref().expect("actor").as_str(),
        "user/approver"
    );
    assert!(granted
        .decision_context
        .as_ref()
        .expect("decision reason")
        .contains("manual local approval decision"));
    assert_eq!(
        granted
            .correlation_id
            .as_ref()
            .expect("correlation")
            .as_str(),
        "correlation/local-approval"
    );
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
fn retry_and_escalation_audit_include_required_context_without_raw_payloads() {
    let project = TestProject::new("audit-retry-escalation-metadata");
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
    let events = audit.events();
    let retry = events
        .iter()
        .find(|event| event.event_type == WorkflowRunEventKindName::RetryStarted)
        .expect("retry audit event");
    let escalation = events
        .iter()
        .find(|event| event.event_type == WorkflowRunEventKindName::EscalationTriggered)
        .expect("escalation audit event");

    for event in [retry, escalation] {
        assert_eq!(event.step_id.as_ref().expect("step").as_str(), "echo");
        assert_eq!(
            event.skill_id.as_ref().expect("skill").as_str(),
            "local/echo"
        );
        assert_eq!(
            event
                .skill_version
                .as_ref()
                .expect("skill version")
                .as_str(),
            "v0"
        );
        assert!(event.correlation_id.is_some());
        assert!(event.actor.is_some());
        assert!(!event.decision_context.as_ref().expect("context").is_empty());
    }
    assert!(retry.idempotency_key.is_some());
    let audit_text = format!("{events:?}");
    assert!(!audit_text.contains("request:"));
    assert!(!audit_text.contains("value: hello"));
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
    let succeeded = audit
        .events()
        .into_iter()
        .find(|event| event.event_type == WorkflowRunEventKindName::SkillInvocationSucceeded)
        .expect("skill success audit event");
    assert_eq!(succeeded.output_reference.as_deref(), Some("[REDACTED]"));
    assert!(succeeded.redaction.field_states.iter().any(|field| {
        field.field == "output_reference" && field.disposition == RedactionDisposition::Redacted
    }));
}

#[test]
fn sensitive_input_is_reference_only_in_audit() {
    let project = TestProject::new("audit-input-reference-only");
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

    executor
        .execute(&project.request(None))
        .expect("run executes");
    let requested = audit
        .events()
        .into_iter()
        .find(|event| event.event_type == WorkflowRunEventKindName::SkillInvocationRequested)
        .expect("skill requested audit event");

    assert!(requested.input_reference.is_some());
    assert!(!format!("{requested:?}").contains("value: hello"));
    assert!(requested.redaction.field_states.iter().any(|field| {
        field.field == "input_reference" && field.disposition == RedactionDisposition::ReferenceOnly
    }));
}

#[test]
fn redacted_value_debug_and_display_do_not_leak_inner_value() {
    let value = RedactedValue::new("secret-token-should-not-log");

    assert_eq!(value.to_string(), "[REDACTED]");
    assert_eq!(format!("{value:?}"), "RedactedValue([REDACTED])");
    assert!(!format!("{value:?}").contains("secret-token-should-not-log"));
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
    let records = backend
        .read_policy_audit_records()
        .expect("policy audit records survive sink failure");

    assert_eq!(error.code(), "audit.sink.failed");
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].scope, PolicyAuditScope::PreRun);
    assert!(backend
        .read_events(records[0].workflow_run_id.as_ref().expect("pending run id"))
        .expect("events")
        .is_empty());
}

fn append_running_run(backend: &impl StateBackend) -> WorkflowRunId {
    let run_id = WorkflowRunId::new("run/cancel-running").expect("run id");
    let workflow_id = WorkflowId::new("local/main").expect("workflow id");
    let schema_version = SchemaVersion::new(SUPPORTED_SCHEMA_VERSION).expect("schema version");
    let workflow_version = WorkflowVersion::new("v0").expect("workflow version");
    let spec_hash = SpecContentHash::from_text("cancel running");
    for event in [
        running_event(
            1,
            &run_id,
            &workflow_id,
            &schema_version,
            &workflow_version,
            &spec_hash,
            WorkflowRunEventKind::RunCreated { summary: None },
        ),
        running_event(
            2,
            &run_id,
            &workflow_id,
            &schema_version,
            &workflow_version,
            &spec_hash,
            WorkflowRunEventKind::RunValidated,
        ),
        running_event(
            3,
            &run_id,
            &workflow_id,
            &schema_version,
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
    schema_version: &SchemaVersion,
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
        schema_version: schema_version.clone(),
        workflow_version: workflow_version.clone(),
        spec_content_hash: spec_hash.clone(),
        correlation_id: Some(CorrelationId::new("correlation/cancel").expect("correlation")),
        actor: Some(ActorId::new("system/cancel-test").expect("actor")),
        idempotency_key: None,
        kind,
    }
}
