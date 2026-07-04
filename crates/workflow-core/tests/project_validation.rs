#![allow(clippy::expect_used)]
//! Deterministic semantic validation tests for loaded Workflow OS projects.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{load_project, validate_loaded_project, SUPPORTED_SCHEMA_VERSION};

static NEXT_TEST_PROJECT: AtomicU64 = AtomicU64::new(1);

struct TestProject {
    root: PathBuf,
}

impl TestProject {
    fn new(name: &str) -> Self {
        let id = NEXT_TEST_PROJECT.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "workflow-os-validation-{name}-{}-{id}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("stale test project cleanup succeeds");
        }
        fs::create_dir_all(&root).expect("project root is created");
        Self { root }
    }

    fn path(&self) -> &Path {
        &self.root
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
  id: acme/validation
  name: Acme Validation
layout:
  workflows: workflows
  skills: skills
  policies: policies
  tests: tests
"
            ),
        );
    }

    fn write_policy_set(&self) {
        self.write(
            "policies/local.policy.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/allow
name: Local Allow
rules:
  - id: local
    effect: allow_local
"
            ),
        );
        self.write(
            "policies/approval.policy.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/default
name: Default Approval
rules:
  - id: require-human
    effect: require_approval
"
            ),
        );
        self.write(
            "policies/retry.policy.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: retry/bounded
name: Bounded Retry
rules:
  - id: retry
    effect: retry
  - id: bounded
    effect: bounded_retry
"
            ),
        );
        self.write(
            "policies/escalation.policy.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: escalation/default
name: Default Escalation
rules:
  - id: escalate
    effect: escalate
"
            ),
        );
    }

    fn write_external_read_policy(&self) {
        self.write(
            "policies/external-read.policy.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: external/read
name: External Read
rules:
  - id: allow-read-only-adapter
    effect: allow_external_read
"
            ),
        );
    }

    fn write_local_skill(&self) {
        self.write(
            "skills/draft.skill.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/draft
version: v0
display_name: Draft
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
    description: Skill failed.
evaluation_criteria:
  - name: grounded
    description: Output is grounded.
audit_requirements:
  required: true
  events:
    - SkillInvocationPlanned
observability_requirements:
  metrics:
    - skill_latency
"
            ),
        );
    }

    fn write_adapter_skill(&self, capability: &str, adapter_capability: &str) {
        self.write(
            "skills/external.skill.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: external/action
version: v0
display_name: External Action
owner:
  lifecycle_status: stable
input_contract:
  fields:
    - name: request
      field_type: string
      sensitive: true
      redaction: reference_only
  required:
    - request
output_contract:
  fields:
    - name: result
      field_type: string
      sensitive: true
      redaction: reference_only
  required:
    - result
allowed_capabilities:
  - name: {capability}
adapter_requirements:
  - adapter_id: symbolic/external
    capabilities:
      - {adapter_capability}
failure_modes:
  - code: failed
    description: External action failed.
evaluation_criteria:
  - name: reviewed
    description: External action is reviewed.
approval_sensitivity: high
audit_requirements:
  required: true
  events:
    - SkillInvocationPlanned
observability_requirements:
  metrics:
    - skill_latency
"
            ),
        );
    }

    fn write_valid_workflow(&self, skill_id: &str, autonomy_level: &str) {
        self.write(
            "workflows/main.workflow.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/main
version: v0
display_name: Main
owner:
  lifecycle_status: stable
autonomy_level: {autonomy_level}
triggers:
  - id: manual
    kind: manual
steps:
  - id: draft
    skill_ref:
      id: {skill_id}
      version: v0
    policy_requirements:
      - id: local/allow
    approval_policy:
      policy:
        id: approval/default
    retry_policy:
      policy:
        id: retry/bounded
    escalation_policy:
      policy:
        id: escalation/default
    timeout:
      duration: 10m
    terminal_behavior: fail_workflow
approval_requirements:
  - id: human-review
    reason: Human review is required.
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

    fn write_valid_external_read_workflow(&self) {
        self.write(
            "workflows/main.workflow.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/main
version: v0
display_name: Main
owner:
  lifecycle_status: stable
autonomy_level: level_2
triggers:
  - id: manual
    kind: manual
steps:
  - id: external
    skill_ref:
      id: external/action
      version: v0
    policy_requirements:
      - id: external/read
    approval_policy:
      policy:
        id: approval/default
    timeout:
      duration: 10m
    terminal_behavior: fail_workflow
approval_requirements:
  - id: human-review
    reason: Human review is required.
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

    fn write_valid_minimal_project(&self) {
        self.write_manifest();
        self.write_policy_set();
        self.write_local_skill();
        self.write_valid_workflow("local/draft", "level_2");
    }
}

impl Drop for TestProject {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn validate(project: &TestProject) -> Vec<String> {
    let loaded = load_project(project.path());
    validate_loaded_project(&loaded)
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code().to_owned())
        .collect()
}

#[test]
fn valid_minimal_project_passes() {
    let project = TestProject::new("valid-minimal");
    project.write_valid_minimal_project();

    let codes = validate(&project);

    assert!(!codes.iter().any(|code| code.starts_with("validation.")));
}

#[test]
fn valid_read_only_adapter_project_passes() {
    let project = TestProject::new("valid-read-only-adapter");
    project.write_manifest();
    project.write_policy_set();
    project.write_external_read_policy();
    project.write_adapter_skill("external.read", "external.read");
    project.write_valid_external_read_workflow();

    let codes = validate(&project);

    assert!(!codes.iter().any(|code| code.starts_with("validation.")));
}

#[test]
fn report_artifact_requirement_not_required_passes() {
    let project = TestProject::new("report-artifact-not-required");
    project.write_valid_minimal_project();
    project.write(
        "workflows/main.workflow.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/main
version: v0
display_name: Main
owner:
  lifecycle_status: stable
autonomy_level: level_2
triggers:
  - id: manual
    kind: manual
steps:
  - id: draft
    skill_ref:
      id: local/draft
      version: v0
    policy_requirements:
      - id: local/allow
    approval_policy:
      policy:
        id: approval/default
    retry_policy:
      policy:
        id: retry/bounded
    escalation_policy:
      policy:
        id: escalation/default
    timeout:
      duration: 10m
    terminal_behavior: fail_workflow
approval_requirements:
  - id: human-review
    reason: Human review is required.
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
report_artifact_requirements:
  high_assurance_approval: not_required
"
        ),
    );

    let codes = validate(&project);

    assert!(!codes.iter().any(|code| code.starts_with("validation.")));
}

#[test]
fn report_artifact_requirement_enforcement_posture_is_rejected_until_runtime_wiring_exists() {
    let project = TestProject::new("report-artifact-runtime-not-enforced");
    project.write_valid_minimal_project();
    project.write(
        "workflows/main.workflow.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/main
version: v0
display_name: Main
owner:
  lifecycle_status: stable
autonomy_level: level_2
triggers:
  - id: manual
    kind: manual
steps:
  - id: draft
    skill_ref:
      id: local/draft
      version: v0
    policy_requirements:
      - id: local/allow
    approval_policy:
      policy:
        id: approval/default
    retry_policy:
      policy:
        id: retry/bounded
    escalation_policy:
      policy:
        id: escalation/default
    timeout:
      duration: 10m
    terminal_behavior: fail_workflow
approval_requirements:
  - id: human-review
    reason: Human review is required.
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
report_artifact_requirements:
  high_assurance_approval: validated_fail_closed_disclosure_required
"
        ),
    );

    let codes = validate(&project);

    assert!(codes.contains(
        &"validation.workflow.report_artifact_requirement.runtime_not_enforced".to_owned()
    ));
}

#[test]
fn missing_skill_reference_is_reported() {
    let project = TestProject::new("missing-skill");
    project.write_valid_minimal_project();
    project.write_valid_workflow("local/missing", "level_2");

    let codes = validate(&project);

    assert!(codes.contains(&"validation.reference.skill_missing".to_owned()));
}

#[test]
fn duplicate_ids_are_reported() {
    let project = TestProject::new("duplicate");
    project.write_valid_minimal_project();
    project.write(
        "workflows/duplicate.workflow.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/main
version: v0
display_name: Duplicate
triggers:
  - id: manual
    kind: manual
steps: []
"
        ),
    );

    let codes = validate(&project);

    assert!(codes.contains(&"loader.duplicate_workflow_id".to_owned()));
    assert!(codes.contains(&"validation.workflow.duplicate_id".to_owned()));
}

#[test]
fn invalid_state_transition_is_reported() {
    let project = TestProject::new("invalid-transition");
    project.write_valid_minimal_project();
    project.write(
        "workflows/main.workflow.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/main
version: v0
display_name: Main
owner:
  lifecycle_status: stable
autonomy_level: level_2
triggers:
  - id: manual
    kind: manual
steps:
  - id: draft
    skill_ref:
      id: local/draft
      version: v0
    approval_policy:
      policy:
        id: approval/default
    terminal_behavior: fail_workflow
branches:
  - id: missing
    condition: always
    target_step: does-not-exist
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

    let codes = validate(&project);

    assert!(codes.contains(&"validation.workflow.invalid_state_transition".to_owned()));
}

#[test]
fn unbounded_retry_is_reported() {
    let project = TestProject::new("unbounded-retry");
    project.write_valid_minimal_project();
    project.write(
        "policies/retry.policy.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: retry/bounded
name: Unbounded Retry
rules:
  - id: retry
    effect: unbounded_retry
"
        ),
    );

    let codes = validate(&project);

    assert!(codes.contains(&"validation.policy.retry_unbounded".to_owned()));
}

#[test]
fn unsupported_policy_effect_is_reported() {
    let project = TestProject::new("unsupported-policy-effect");
    project.write_valid_minimal_project();
    project.write(
        "policies/approval.policy.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/default
name: Invalid Approval
rules:
  - id: unsupported
    effect: allow_github_write
"
        ),
    );

    let codes = validate(&project);

    assert!(codes.contains(&"validation.policy.effect_unsupported".to_owned()));
}

#[test]
fn policy_actor_binding_is_reported_as_unsupported() {
    let project = TestProject::new("policy-actor");
    project.write_valid_minimal_project();
    project.write(
        "policies/approval.policy.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/default
name: Actor Approval
rules:
  - id: approval
    effect: require_approval
    actor: system/approver
"
        ),
    );

    let codes = validate(&project);

    assert!(codes.contains(&"validation.policy.actor_unsupported".to_owned()));
}

#[test]
fn policy_effect_in_wrong_reference_context_is_reported() {
    let project = TestProject::new("policy-context");
    project.write_valid_minimal_project();
    project.write(
        "workflows/main.workflow.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/main
version: v0
display_name: Main
owner:
  lifecycle_status: stable
autonomy_level: level_2
triggers:
  - id: manual
    kind: manual
steps:
  - id: draft
    skill_ref:
      id: local/draft
      version: v0
    policy_requirements:
      - id: approval/default
    timeout:
      duration: 10m
    terminal_behavior: fail_workflow
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

    let codes = validate(&project);

    assert!(codes.contains(&"validation.policy.effect_context_invalid".to_owned()));
}

#[test]
fn external_read_step_requires_allow_external_read_effect() {
    let project = TestProject::new("external-read-missing-policy");
    project.write_manifest();
    project.write_policy_set();
    project.write_adapter_skill("external.read", "external.read");
    project.write_valid_external_read_workflow();
    project.write(
        "workflows/main.workflow.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/main
version: v0
display_name: Main
owner:
  lifecycle_status: stable
autonomy_level: level_2
triggers:
  - id: manual
    kind: manual
steps:
  - id: external
    skill_ref:
      id: external/action
      version: v0
    policy_requirements:
      - id: approval/default
    approval_policy:
      policy:
        id: approval/default
    timeout:
      duration: 10m
    terminal_behavior: fail_workflow
approval_requirements:
  - id: human-review
    reason: Human review is required.
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

    let codes = validate(&project);

    assert!(codes.contains(&"validation.policy.external_read_missing".to_owned()));
}

#[test]
fn external_write_step_is_rejected_before_runtime() {
    let project = TestProject::new("external-write-unsupported");
    project.write_manifest();
    project.write_policy_set();
    project.write_external_read_policy();
    project.write_adapter_skill("external.write", "external.write");
    project.write_valid_external_read_workflow();

    let codes = validate(&project);

    assert!(codes.contains(&"validation.policy.external_write_unsupported".to_owned()));
}

#[test]
fn retry_exhaustion_without_escalation_is_reported() {
    let project = TestProject::new("retry-exhaustion");
    project.write_valid_minimal_project();
    project.write(
        "workflows/main.workflow.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/main
version: v0
display_name: Main
owner:
  lifecycle_status: stable
autonomy_level: level_2
triggers:
  - id: manual
    kind: manual
steps:
  - id: draft
    skill_ref:
      id: local/draft
      version: v0
    approval_policy:
      policy:
        id: approval/default
    retry_policy:
      policy:
        id: retry/bounded
    terminal_behavior: continue
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

    let codes = validate(&project);

    assert!(codes.contains(&"validation.workflow.retry_exhaustion_unsafe".to_owned()));
}

#[test]
fn missing_approval_on_sensitive_action_is_reported() {
    let project = TestProject::new("missing-approval");
    project.write_manifest();
    project.write_policy_set();
    project.write_adapter_skill("external_write", "external_write");
    project.write_valid_workflow("external/action", "level_2");
    project.write(
        "workflows/main.workflow.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/main
version: v0
display_name: Main
owner:
  lifecycle_status: stable
autonomy_level: level_2
triggers:
  - id: manual
    kind: manual
steps:
  - id: external
    skill_ref:
      id: external/action
      version: v0
    policy_requirements:
      - id: approval/default
    timeout:
      duration: 10m
    terminal_behavior: fail_workflow
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

    let codes = validate(&project);

    assert!(codes.contains(&"validation.workflow.approval_policy_missing".to_owned()));
}

#[test]
fn level_3_and_4_are_rejected_by_default() {
    let project = TestProject::new("level-three");
    project.write_valid_minimal_project();
    project.write_valid_workflow("local/draft", "level_3");

    let codes = validate(&project);

    assert!(codes.contains(&"validation.workflow.autonomy_level_unsafe".to_owned()));
}

#[test]
fn undeclared_capability_is_reported() {
    let project = TestProject::new("undeclared-capability");
    project.write_manifest();
    project.write_policy_set();
    project.write_adapter_skill("external_write", "delete_everything");
    project.write_valid_workflow("external/action", "level_2");

    let codes = validate(&project);

    assert!(codes.contains(&"validation.skill.undeclared_capability".to_owned()));
}

#[test]
fn secret_in_spec_is_reported() {
    let project = TestProject::new("secret");
    project.write_valid_minimal_project();
    project.write(
        "workflows/secret.workflow.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/secret
version: v0
display_name: Secret
steps:
  - id: secret
    skill_ref:
      id: local/draft
    input_mapping:
      - from:
          type: literal
          value: secret:abc123
        to: request
"
        ),
    );

    let codes = validate(&project);

    assert!(codes.contains(&"spec.secret_disallowed".to_owned()));
}

#[test]
fn sensitive_field_without_redaction_is_reported() {
    let project = TestProject::new("sensitive-redaction");
    project.write_valid_minimal_project();
    project.write(
        "skills/draft.skill.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/draft
version: v0
display_name: Draft
owner:
  lifecycle_status: stable
input_contract:
  fields:
    - name: request
      field_type: string
      sensitive: true
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
    description: Skill failed.
evaluation_criteria:
  - name: grounded
    description: Output is grounded.
"
        ),
    );

    let codes = validate(&project);

    assert!(codes.contains(&"validation.skill.sensitive_redaction_missing".to_owned()));
}

#[test]
fn multiple_diagnostics_are_accumulated() {
    let project = TestProject::new("multiple");
    project.write_manifest();
    project.write_policy_set();
    project.write_local_skill();
    project.write(
        "workflows/main.workflow.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/main
version: v0
display_name: Main
owner:
  lifecycle_status: stable
autonomy_level: level_3
steps:
  - id: draft
    skill_ref:
      id: local/missing
      version: v0
"
        ),
    );

    let codes = validate(&project);

    assert!(codes.contains(&"validation.workflow.triggers_missing".to_owned()));
    assert!(codes.contains(&"validation.workflow.cancellation_missing".to_owned()));
    assert!(codes.contains(&"validation.workflow.autonomy_level_unsafe".to_owned()));
    assert!(codes.contains(&"validation.reference.skill_missing".to_owned()));
}

#[test]
fn semantic_diagnostics_outside_schema_version_family_keep_order_and_no_evidence() {
    let project = TestProject::new("semantic-no-evidence");
    project.write_manifest();
    project.write_policy_set();
    project.write_local_skill();
    project.write(
        "workflows/main.workflow.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/main
version: v0
display_name: Main
owner:
  lifecycle_status: stable
autonomy_level: level_3
steps:
  - id: draft
    skill_ref:
      id: local/missing
      version: v0
"
        ),
    );

    let loaded = load_project(project.path());
    let result = validate_loaded_project(&loaded);
    let codes: Vec<_> = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code().to_owned())
        .collect();

    assert_eq!(
        codes,
        vec![
            "loader.directory_missing",
            "validation.workflow.triggers_missing",
            "validation.workflow.cancellation_missing",
            "validation.workflow.audit_missing",
            "validation.workflow.observability_missing",
            "validation.workflow.autonomy_level_unsafe",
            "validation.reference.skill_missing",
        ]
    );
    assert!(result
        .diagnostics
        .iter()
        .all(|diagnostic| diagnostic.evidence_references().is_empty()));
}
