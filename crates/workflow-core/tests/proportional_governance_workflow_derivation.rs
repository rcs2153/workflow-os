#![allow(clippy::expect_used)]

//! Deterministic workflow-declaration governance derivation tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    derive_workflow_step_governance_assessment_input, load_project, validate_project_bundle,
    GovernanceDisclosureObligation, GovernanceExecutionRequirement, GovernanceStrictnessProfile,
    GovernanceWorkloadActionClass, GovernanceWorkloadAuthorityPosture,
    GovernanceWorkloadEvidenceCheckPosture, GovernanceWorkloadSensitivity,
    GovernanceWorkloadSideEffectPosture, SpecContentHash, StepId, WorkflowId, WorkflowOsErrorKind,
    WorkflowStepGovernanceDerivationRequest, SUPPORTED_SCHEMA_VERSION,
};

static NEXT_PROJECT: AtomicU64 = AtomicU64::new(1);

struct TestProject {
    root: PathBuf,
}

impl TestProject {
    fn new(name: &str) -> Self {
        let id = NEXT_PROJECT.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "workflow-os-governance-derivation-{name}-{}-{id}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("stale fixture is removed");
        }
        fs::create_dir_all(&root).expect("fixture root is created");
        Self { root }
    }

    fn path(&self) -> &Path {
        &self.root
    }

    fn write(&self, relative: &str, contents: &str) {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("fixture parent is created");
        }
        fs::write(path, contents).expect("fixture file is written");
    }

    fn write_project(&self, capability: &str, sensitivity: &str, approval: bool) {
        self.write_manifest();
        self.write_policies();
        self.write_skill(capability, sensitivity);
        self.write_workflow(approval);
    }

    fn write_manifest(&self) {
        self.write(
            "workflow-os.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
project:
  id: test/governance-derivation
  name: Governance Derivation
layout:
  workflows: workflows
  skills: skills
  policies: policies
  tests: tests
"
            ),
        );
    }

    fn write_policies(&self) {
        self.write(
            "policies/local.policy.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/allow
name: Local Allow
rules:
  - id: allow
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
name: Approval
rules:
  - id: require
    effect: require_approval
"
            ),
        );
        self.write(
            "policies/unrelated.policy.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: unrelated/policy
name: Unrelated
rules:
  - id: allow
    effect: allow_local
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
name: Escalation
rules:
  - id: escalate
    effect: escalate
"
            ),
        );
    }

    fn write_skill(&self, capability: &str, sensitivity: &str) {
        self.write(
            "skills/action.skill.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/action
version: v0
display_name: Local Action
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
    - name: result
      field_type: string
  required:
    - result
allowed_capabilities:
  - name: {capability}
approval_sensitivity: {sensitivity}
failure_modes:
  - code: failed
    description: Action failed.
evaluation_criteria:
  - name: bounded
    description: Action remains bounded.
"
            ),
        );
    }

    fn write_workflow(&self, approval: bool) {
        let approval_policy = if approval {
            "\n    approval_policy:\n      policy:\n        id: approval/default"
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
display_name: Main
owner:
  lifecycle_status: stable
autonomy_level: level_1
triggers:
  - id: manual
    kind: manual
steps:
  - id: action
    skill_ref:
      id: local/action
      version: v0
    policy_requirements:
      - id: local/allow{approval_policy}
    terminal_behavior: fail_workflow
audit_requirements:
  required: true
  events:
    - RunCreated
cancellation_behavior: stop
observability_requirements:
  metrics:
    - workflow_latency
retry_policy_refs:
  - id: retry/bounded
escalation_policy_refs:
  - id: escalation/default
"
            ),
        );
    }

    fn bundle(&self) -> workflow_core::ProjectBundle {
        let loaded = load_project(self.path());
        assert!(
            !loaded.has_errors(),
            "fixture loader diagnostics: {:?}",
            loaded.diagnostics
        );
        let bundle = loaded.bundle.expect("fixture bundle is available");
        let validation = validate_project_bundle(&bundle);
        assert!(
            !validation.has_errors(),
            "fixture validation diagnostics: {:?}",
            validation.diagnostics
        );
        bundle
    }
}

impl Drop for TestProject {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn request(bundle: &workflow_core::ProjectBundle) -> WorkflowStepGovernanceDerivationRequest<'_> {
    WorkflowStepGovernanceDerivationRequest {
        project: bundle,
        workflow_id: &bundle.workflows[0].definition.id,
        step_id: &bundle.workflows[0].definition.steps[0].id,
        profile: GovernanceStrictnessProfile::ObserveAndReport,
        authority: None,
        evidence_and_checks: None,
        side_effect: None,
        prior_execution: None,
        prior_disclosure: None,
        steward_minimum: None,
    }
}

#[test]
fn read_only_declarations_derive_bounded_facts_and_keep_runtime_facts_unknown() {
    let fixture = TestProject::new("read-only");
    fixture.write_project("local.read", "low", false);
    let bundle = fixture.bundle();

    let input = derive_workflow_step_governance_assessment_input(&request(&bundle))
        .expect("valid declarations derive an input");

    assert_eq!(input.action_class, GovernanceWorkloadActionClass::ReadOnly);
    assert_eq!(input.sensitivity, GovernanceWorkloadSensitivity::Routine);
    assert_eq!(input.side_effect, GovernanceWorkloadSideEffectPosture::None);
    assert_eq!(input.authority, GovernanceWorkloadAuthorityPosture::Unknown);
    assert_eq!(
        input.evidence_and_checks,
        GovernanceWorkloadEvidenceCheckPosture::Unknown
    );
}

#[test]
fn local_write_and_sensitive_approval_declarations_raise_posture_without_inventing_reversibility() {
    let fixture = TestProject::new("local-write");
    fixture.write_project("local.write", "high", true);
    let bundle = fixture.bundle();

    let input = derive_workflow_step_governance_assessment_input(&request(&bundle))
        .expect("valid declarations derive an input");

    assert_eq!(
        input.action_class,
        GovernanceWorkloadActionClass::LocalMutation
    );
    assert_eq!(input.sensitivity, GovernanceWorkloadSensitivity::Restricted);
    assert_eq!(
        input.side_effect,
        GovernanceWorkloadSideEffectPosture::Unknown
    );
    assert_eq!(
        input.policy_minimum.execution(),
        GovernanceExecutionRequirement::RequireApproval
    );
    assert_eq!(
        input.policy_minimum.disclosure(),
        GovernanceDisclosureObligation::VisibleRequired
    );
}

#[test]
fn caller_can_supply_bounded_runtime_facts_without_weakening_declarations() {
    let fixture = TestProject::new("runtime-facts");
    fixture.write_project("local.write", "low", false);
    let bundle = fixture.bundle();
    let mut derivation = request(&bundle);
    derivation.authority = Some(GovernanceWorkloadAuthorityPosture::Sufficient);
    derivation.evidence_and_checks = Some(GovernanceWorkloadEvidenceCheckPosture::Satisfied);
    derivation.side_effect = Some(GovernanceWorkloadSideEffectPosture::LocalReversible);

    let input = derive_workflow_step_governance_assessment_input(&derivation)
        .expect("compatible runtime facts are accepted");

    assert_eq!(
        input.authority,
        GovernanceWorkloadAuthorityPosture::Sufficient
    );
    assert_eq!(
        input.evidence_and_checks,
        GovernanceWorkloadEvidenceCheckPosture::Satisfied
    );
    assert_eq!(
        input.side_effect,
        GovernanceWorkloadSideEffectPosture::LocalReversible
    );
}

#[test]
fn contradictory_side_effect_fact_fails_with_stable_non_leaking_error() {
    let fixture = TestProject::new("side-effect-mismatch");
    fixture.write_project("local.read", "low", false);
    let bundle = fixture.bundle();
    let mut derivation = request(&bundle);
    derivation.side_effect = Some(GovernanceWorkloadSideEffectPosture::ExternalIrreversible);

    let error = derive_workflow_step_governance_assessment_input(&derivation)
        .expect_err("contradictory posture must fail");

    assert_eq!(error.kind(), WorkflowOsErrorKind::Validation);
    assert_eq!(
        error.code(),
        "governance.proportional.derivation.side_effect_mismatch"
    );
    assert!(!error.to_string().contains("local/main"));
    assert!(!error.to_string().contains("action"));
}

#[test]
fn relevant_definition_hashes_invalidate_but_unrelated_policy_hashes_do_not() {
    let fixture = TestProject::new("definition-root");
    fixture.write_project("local.read", "low", false);
    let bundle = fixture.bundle();
    let baseline = derive_workflow_step_governance_assessment_input(&request(&bundle))
        .expect("baseline derives")
        .definition_root;

    let mut relevant = bundle.clone();
    relevant.skills[0].content_hash = SpecContentHash::from_text("changed relevant skill");
    let changed = derive_workflow_step_governance_assessment_input(&request(&relevant))
        .expect("changed relevant definition derives")
        .definition_root;
    assert_ne!(baseline, changed);

    for policy_id in ["retry/bounded", "escalation/default"] {
        let mut workflow_policy = bundle.clone();
        let policy = workflow_policy
            .policies
            .iter_mut()
            .find(|policy| policy.definition.id.as_str() == policy_id)
            .expect("workflow-level policy exists");
        policy.content_hash = SpecContentHash::from_text("changed workflow-level policy");
        let changed = derive_workflow_step_governance_assessment_input(&request(&workflow_policy))
            .expect("changed workflow-level policy derives")
            .definition_root;
        assert_ne!(baseline, changed);
    }

    let mut unrelated = bundle.clone();
    let unrelated_policy = unrelated
        .policies
        .iter_mut()
        .find(|policy| policy.definition.id.as_str() == "unrelated/policy")
        .expect("unrelated policy exists");
    unrelated_policy.content_hash = SpecContentHash::from_text("changed unrelated policy");
    let unchanged = derive_workflow_step_governance_assessment_input(&request(&unrelated))
        .expect("unrelated definition change derives")
        .definition_root;
    assert_eq!(baseline, unchanged);
}

#[test]
fn unresolved_identity_and_debug_output_are_bounded() {
    let fixture = TestProject::new("bounded-errors");
    fixture.write_project("local.read", "low", false);
    let bundle = fixture.bundle();
    let workflow_id = WorkflowId::new("secret/workflow-name").expect("test id is valid");
    let step_id = StepId::new("secret-step-name").expect("test id is valid");
    let derivation = WorkflowStepGovernanceDerivationRequest {
        project: &bundle,
        workflow_id: &workflow_id,
        step_id: &step_id,
        profile: GovernanceStrictnessProfile::ObserveAndReport,
        authority: None,
        evidence_and_checks: None,
        side_effect: None,
        prior_execution: None,
        prior_disclosure: None,
        steward_minimum: None,
    };
    let debug = format!("{derivation:?}");
    assert!(!debug.contains("secret/workflow-name"));
    assert!(!debug.contains("secret-step-name"));

    let error = derive_workflow_step_governance_assessment_input(&derivation)
        .expect_err("unresolved workflow must fail");
    assert_eq!(
        error.code(),
        "governance.proportional.derivation.workflow_unresolved"
    );
    assert!(!error.to_string().contains("secret/workflow-name"));
}
