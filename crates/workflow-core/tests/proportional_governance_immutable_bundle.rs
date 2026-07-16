#![allow(clippy::expect_used, clippy::panic)]
//! Pure immutable-bundle proportional-governance reassessment tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    assess_immutable_bundle_governance, build_immutable_run_bundle, load_project, ActorId,
    GovernanceExecutionDisposition, GovernancePostureRequirement, GovernanceStrictnessProfile,
    GovernanceWorkloadAuthorityPosture, GovernanceWorkloadEvidenceCheckPosture,
    GovernanceWorkloadSideEffectPosture, ImmutableBundleGovernanceAssessmentRequest,
    ImmutableRunBundleBuildRequest, ImmutableRunBundleExecutionPosture,
    ImmutableRunBundleHandlerPosture, ImmutableRunBundleHandlerReference, ImmutableRunBundleId,
    ImmutableRunBundleReferencePosture, ImmutableRunBundleSensitivity, ImmutableRunBundleVersion,
    LocalImmutableRunBundleStore, SkillId, SkillVersion, SpecContentHash,
    StepGovernanceRuntimeFacts, StepId, Timestamp, WorkflowId, WorkflowRunId,
    SUPPORTED_SCHEMA_VERSION,
};

static NEXT_ROOT: AtomicU64 = AtomicU64::new(1);

struct TestRoot {
    path: PathBuf,
}

impl TestRoot {
    fn new(name: &str) -> Self {
        let id = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "workflow-os-immutable-governance-{name}-{}-{id}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("test root created");
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn write(&self, relative: &str, content: &str) {
        let path = self.path.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent created");
        }
        fs::write(path, content).expect("fixture written");
    }
}

impl Drop for TestRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn write_project(root: &TestRoot, workflow_name: &str) {
    root.write(
        "workflow-os.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nproject:\n  id: governance/project\n  name: Governance Project\n"
        ),
    );
    root.write(
        "workflows/build.workflow.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: governance/build\nversion: v1\ndisplay_name: {workflow_name}\ntriggers:\n  - id: manual-start\n    kind: manual\nsteps:\n  - id: inspect\n    skill_ref:\n      id: local/check\n      version: v1\n    policy_requirements:\n      - id: local/read-only\n    terminal_behavior: fail_workflow\n  - id: verify\n    skill_ref:\n      id: local/check\n      version: v1\n    policy_requirements:\n      - id: local/read-only\n    terminal_behavior: fail_workflow\ncancellation_behavior: stop\naudit_requirements:\n  required: true\n  events: [RunCreated, RunCompleted]\n  store_references_only: true\nobservability_requirements:\n  metrics: [workflow_latency]\n  tracing: true\n  latency_tracking: true\n"
        ),
    );
    root.write(
        "skills/check.skill.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/check\nversion: v1\ndisplay_name: Local Check\nallowed_capabilities:\n  - name: local.read\ninput_contract:\n  fields:\n    - name: request\n      field_type: string\noutput_contract:\n  fields:\n    - name: summary\n      field_type: string\nfailure_modes:\n  - code: check_failed\n    description: Local check failed.\n    retryable: false\naudit_requirements:\n  required: true\n  events: [SkillInvocationRequested]\n  store_references_only: true\nobservability_requirements:\n  metrics: [skill_latency]\n  tracing: true\n  latency_tracking: true\n"
        ),
    );
    root.write(
        "policies/read-only.policy.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/read-only\nname: Read Only\nrules:\n  - id: allow-local\n    effect: allow_local\n"
        ),
    );
}

fn stored_bundle(
    project: &TestRoot,
    storage: &TestRoot,
) -> workflow_core::StoredImmutableRunBundle {
    let loaded = load_project(project.path());
    assert!(!loaded.has_errors(), "{:?}", loaded.diagnostics);
    let project_bundle = loaded.bundle.expect("loaded project");
    let workflow_id = WorkflowId::new("governance/build").expect("workflow id");
    let built = build_immutable_run_bundle(ImmutableRunBundleBuildRequest {
        project: &project_bundle,
        workflow_id: &workflow_id,
        bundle_id: ImmutableRunBundleId::new("bundle/governance-run").expect("bundle id"),
        bundle_version: ImmutableRunBundleVersion::new("v1").expect("bundle version"),
        run_id: WorkflowRunId::new("run-governance").expect("run id"),
        resolved_execution_context_hash: SpecContentHash::from_text("resolved context"),
        execution_posture: ImmutableRunBundleExecutionPosture::new(
            Vec::new(),
            ImmutableRunBundleReferencePosture::NotSupplied,
            ImmutableRunBundleReferencePosture::NotSupplied,
            ImmutableRunBundleReferencePosture::NotSupplied,
        )
        .expect("execution posture"),
        handlers: vec![ImmutableRunBundleHandlerReference {
            skill_id: SkillId::new("local/check").expect("skill id"),
            skill_version: SkillVersion::new("v1").expect("skill version"),
            posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
        }],
        created_at: Timestamp::parse_rfc3339("2026-07-15T12:00:00Z").expect("timestamp"),
        created_by: ActorId::new("system/kernel").expect("actor"),
        sensitivity: ImmutableRunBundleSensitivity::Internal,
        redaction_required: true,
    })
    .expect("bundle built");
    let store = LocalImmutableRunBundleStore::new(storage.path());
    store.write_bundle(&built).expect("bundle written");
    store
        .read_bundle(built.manifest().run_id(), built.manifest().bundle_id())
        .expect("bundle read")
}

fn fact(step: &str) -> StepGovernanceRuntimeFacts {
    StepGovernanceRuntimeFacts::new(
        StepId::new(step).expect("step id"),
        Some(GovernanceWorkloadAuthorityPosture::Sufficient),
        Some(GovernanceWorkloadEvidenceCheckPosture::Satisfied),
        Some(GovernanceWorkloadSideEffectPosture::None),
        None,
        None,
        None,
    )
}

fn assess<'a>(
    bundle: &'a workflow_core::StoredImmutableRunBundle,
    facts: &'a [StepGovernanceRuntimeFacts],
) -> Result<workflow_core::ImmutableBundleGovernanceAssessmentSet, workflow_core::WorkflowOsError> {
    assess_immutable_bundle_governance(&ImmutableBundleGovernanceAssessmentRequest {
        bundle,
        profile: GovernanceStrictnessProfile::ObserveAndReport,
        runtime_facts: facts,
    })
}

#[test]
fn valid_bundle_produces_deterministic_workflow_ordered_assessments() {
    let project = TestRoot::new("valid-project");
    let storage = TestRoot::new("valid-storage");
    write_project(&project, "Governed Build");
    let bundle = stored_bundle(&project, &storage);
    let facts = vec![fact("verify"), fact("inspect")];

    let first = assess(&bundle, &facts).expect("assessment succeeds");
    let second = assess(&bundle, &facts).expect("assessment is deterministic");

    assert_eq!(first, second);
    assert_eq!(first.workflow_id().as_str(), "governance/build");
    assert_eq!(first.run_id().as_str(), "run-governance");
    assert_eq!(first.assessments().len(), 2);
    assert_eq!(first.assessments()[0].step_id().as_str(), "inspect");
    assert_eq!(first.assessments()[1].step_id().as_str(), "verify");
    assert_eq!(
        first.aggregate_fingerprint().as_str(),
        "7fcce41217ae59183776d62edc4f474428f6ce5312252e4adbc1cd1d7729e9e1"
    );
}

#[test]
fn runtime_fact_changes_invalidate_the_aggregate_fingerprint() {
    let project = TestRoot::new("fact-change-project");
    let storage = TestRoot::new("fact-change-storage");
    write_project(&project, "Governed Build");
    let bundle = stored_bundle(&project, &storage);
    let baseline = vec![fact("inspect"), fact("verify")];
    let mut changed = baseline.clone();
    changed[1] = StepGovernanceRuntimeFacts::new(
        StepId::new("verify").expect("step id"),
        Some(GovernanceWorkloadAuthorityPosture::ApprovalRequired),
        Some(GovernanceWorkloadEvidenceCheckPosture::Satisfied),
        Some(GovernanceWorkloadSideEffectPosture::None),
        None,
        None,
        None,
    );

    assert_ne!(
        assess(&bundle, &baseline)
            .expect("baseline")
            .aggregate_fingerprint(),
        assess(&bundle, &changed)
            .expect("changed")
            .aggregate_fingerprint()
    );
}

#[test]
fn runtime_escalation_monotonically_raises_posture_and_invalidates_fingerprint() {
    let project = TestRoot::new("runtime-escalation-project");
    let storage = TestRoot::new("runtime-escalation-storage");
    write_project(&project, "Governed Build");
    let bundle = stored_bundle(&project, &storage);
    let baseline = vec![fact("inspect"), fact("verify")];
    let baseline_result = assess(&bundle, &baseline).expect("baseline");

    for (requirement, expected_execution) in [
        (
            GovernancePostureRequirement::visible(),
            GovernanceExecutionDisposition::Proceed,
        ),
        (
            GovernancePostureRequirement::approval(),
            GovernanceExecutionDisposition::RequireApproval,
        ),
        (
            GovernancePostureRequirement::denied(),
            GovernanceExecutionDisposition::Denied,
        ),
    ] {
        let escalated = vec![
            fact("inspect").with_runtime_escalation(requirement),
            fact("verify"),
        ];
        let result = assess(&bundle, &escalated).expect("escalated assessment");
        assert_eq!(
            result.assessments()[0].assessment().decision().execution(),
            expected_execution
        );
        assert_ne!(
            result.aggregate_fingerprint(),
            baseline_result.aggregate_fingerprint()
        );
    }

    let explicit_quiet = vec![
        fact("inspect").with_runtime_escalation(GovernancePostureRequirement::quiet()),
        fact("verify"),
    ];
    assert_eq!(
        assess(&bundle, &explicit_quiet)
            .expect("quiet escalation")
            .aggregate_fingerprint(),
        baseline_result.aggregate_fingerprint()
    );
}

#[test]
fn relevant_definition_changes_invalidate_but_unreferenced_definitions_do_not() {
    let baseline_project = TestRoot::new("definition-baseline-project");
    let baseline_storage = TestRoot::new("definition-baseline-storage");
    write_project(&baseline_project, "Governed Build");
    let baseline_bundle = stored_bundle(&baseline_project, &baseline_storage);

    let changed_project = TestRoot::new("definition-changed-project");
    let changed_storage = TestRoot::new("definition-changed-storage");
    write_project(&changed_project, "Changed Governed Build");
    let changed_bundle = stored_bundle(&changed_project, &changed_storage);

    let unrelated_project = TestRoot::new("definition-unrelated-project");
    let unrelated_storage = TestRoot::new("definition-unrelated-storage");
    write_project(&unrelated_project, "Governed Build");
    unrelated_project.write(
        "policies/unrelated.policy.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: unrelated/policy\nname: Unrelated\nrules:\n  - id: allow-local\n    effect: allow_local\n"
        ),
    );
    let unrelated_bundle = stored_bundle(&unrelated_project, &unrelated_storage);
    let facts = vec![fact("inspect"), fact("verify")];

    let baseline = assess(&baseline_bundle, &facts).expect("baseline");
    let changed = assess(&changed_bundle, &facts).expect("changed");
    let unrelated = assess(&unrelated_bundle, &facts).expect("unrelated");

    assert_ne!(
        baseline.aggregate_fingerprint(),
        changed.aggregate_fingerprint()
    );
    assert_eq!(
        baseline.aggregate_fingerprint(),
        unrelated.aggregate_fingerprint()
    );
}

#[test]
fn mutable_project_changes_do_not_change_stored_bundle_assessment() {
    let project = TestRoot::new("immutable-source-project");
    let storage = TestRoot::new("immutable-source-storage");
    write_project(&project, "Governed Build");
    let bundle = stored_bundle(&project, &storage);
    let facts = vec![fact("inspect"), fact("verify")];
    let before = assess(&bundle, &facts).expect("before mutation");
    project.write("skills/check.skill.yml", "changed outside immutable bundle");

    let after = assess(&bundle, &facts).expect("after mutation");

    assert_eq!(before, after);
}

#[test]
fn missing_duplicate_and_extra_runtime_facts_fail_closed() {
    let project = TestRoot::new("fact-errors-project");
    let storage = TestRoot::new("fact-errors-storage");
    write_project(&project, "Governed Build");
    let bundle = stored_bundle(&project, &storage);

    let missing = assess(&bundle, &[fact("inspect")]).expect_err("missing rejected");
    assert_eq!(
        missing.code(),
        "governance.proportional.immutable_bundle.runtime_facts_count_mismatch"
    );

    let duplicate =
        assess(&bundle, &[fact("inspect"), fact("inspect")]).expect_err("duplicate rejected");
    assert_eq!(
        duplicate.code(),
        "governance.proportional.immutable_bundle.runtime_facts_duplicate"
    );

    let extra = assess(&bundle, &[fact("inspect"), fact("other")]).expect_err("extra rejected");
    assert_eq!(
        extra.code(),
        "governance.proportional.immutable_bundle.runtime_facts_step_mismatch"
    );
}

#[test]
fn debug_and_errors_do_not_leak_identifiers_or_marker_payloads() {
    let project = TestRoot::new("redaction-project");
    let storage = TestRoot::new("redaction-storage");
    write_project(&project, "Governed Build");
    let bundle = stored_bundle(&project, &storage);
    let facts = vec![fact("inspect"), fact("verify")];
    let result = assess(&bundle, &facts).expect("assessment succeeds");
    let debug = format!("{result:?}");
    assert!(!debug.contains("governance/build"));
    assert!(!debug.contains("run-governance"));
    assert!(!debug.contains(result.aggregate_fingerprint().as_str()));

    let error = assess(&bundle, &[fact("inspect")]).expect_err("missing rejected");
    let rendered = format!("{error:?}");
    assert!(!rendered.contains("inspect"));
    assert!(!rendered.contains("governance/build"));

    let serialized = serde_json::to_string(&result).expect("serialize result");
    for forbidden in [
        "provider-payload-marker",
        "command-output-marker",
        "parser-payload-marker",
        "spec-content-marker",
    ] {
        assert!(!serialized.contains(forbidden));
    }
}
