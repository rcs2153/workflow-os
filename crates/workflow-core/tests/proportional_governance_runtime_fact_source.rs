#![allow(clippy::expect_used, clippy::panic)]
//! Registered current runtime-fact source and freshness tests.

use std::cell::Cell;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    assess_immutable_bundle_governance_from_current_facts, build_immutable_run_bundle,
    load_project, ActorId, GovernanceRuntimeFactAssessmentRequest,
    GovernanceRuntimeFactObservation, GovernanceRuntimeFactObservationDefinition,
    GovernanceRuntimeFactSnapshotId, GovernanceRuntimeFactSource,
    GovernanceRuntimeFactSourceContractVersion, GovernanceRuntimeFactSourceId,
    GovernanceRuntimeFactSourceRegistration, GovernanceRuntimeFactSourceRegistrationDefinition,
    GovernanceRuntimeFactSourceRequest, GovernanceStrictnessProfile,
    GovernanceWorkloadAuthorityPosture, GovernanceWorkloadEvidenceCheckPosture,
    GovernanceWorkloadSideEffectPosture, ImmutableRunBundleBuildRequest,
    ImmutableRunBundleExecutionPosture, ImmutableRunBundleHandlerPosture,
    ImmutableRunBundleHandlerReference, ImmutableRunBundleId, ImmutableRunBundleReferencePosture,
    ImmutableRunBundleSensitivity, ImmutableRunBundleVersion, LocalImmutableRunBundleStore,
    SkillId, SkillVersion, SpecContentHash, StepGovernanceRuntimeFacts, StepId, Timestamp,
    WorkflowId, WorkflowOsError, WorkflowRunId, SUPPORTED_SCHEMA_VERSION,
};

static NEXT_ROOT: AtomicU64 = AtomicU64::new(1);

struct TestRoot {
    path: PathBuf,
}

impl TestRoot {
    fn new(name: &str) -> Self {
        let id = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "workflow-os-runtime-fact-source-{name}-{}-{id}",
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

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).expect("timestamp")
}

fn write_project(root: &TestRoot) {
    root.write(
        "workflow-os.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nproject:\n  id: governance/project\n  name: Governance Project\n"
        ),
    );
    root.write(
        "workflows/build.workflow.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: governance/build\nversion: v1\ndisplay_name: Governed Build\ntriggers:\n  - id: manual-start\n    kind: manual\nsteps:\n  - id: inspect\n    skill_ref:\n      id: local/check\n      version: v1\n    policy_requirements:\n      - id: local/read-only\n    terminal_behavior: fail_workflow\n  - id: verify\n    skill_ref:\n      id: local/check\n      version: v1\n    policy_requirements:\n      - id: local/read-only\n    terminal_behavior: fail_workflow\ncancellation_behavior: stop\naudit_requirements:\n  required: true\n  events: [RunCreated, RunCompleted]\n  store_references_only: true\nobservability_requirements:\n  metrics: [workflow_latency]\n  tracing: true\n  latency_tracking: true\n"
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
    let built = build_immutable_run_bundle(ImmutableRunBundleBuildRequest {
        project: &project_bundle,
        workflow_id: &WorkflowId::new("governance/build").expect("workflow id"),
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
        created_at: timestamp("2026-07-15T12:00:00Z"),
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

fn source_id() -> GovernanceRuntimeFactSourceId {
    GovernanceRuntimeFactSourceId::new("source/local-governance").expect("source id")
}

fn source_version() -> GovernanceRuntimeFactSourceContractVersion {
    GovernanceRuntimeFactSourceContractVersion::new("v1").expect("source version")
}

fn registration(maximum_age: u32) -> GovernanceRuntimeFactSourceRegistration {
    GovernanceRuntimeFactSourceRegistration::new(
        GovernanceRuntimeFactSourceRegistrationDefinition {
            source_id: source_id(),
            contract_version: source_version(),
            configuration_commitment: SpecContentHash::from_text("safe source configuration"),
            core_maximum_observation_age_seconds: maximum_age,
        },
    )
    .expect("registration")
}

struct StaticSource {
    source_id: GovernanceRuntimeFactSourceId,
    contract_version: GovernanceRuntimeFactSourceContractVersion,
    snapshot_id: GovernanceRuntimeFactSnapshotId,
    bundle_binding: workflow_core::ImmutableRunBundleBinding,
    observed_at: Timestamp,
    maximum_age: u32,
    facts: Vec<StepGovernanceRuntimeFacts>,
    calls: Cell<u32>,
    fail_with_secret: bool,
}

impl GovernanceRuntimeFactSource for StaticSource {
    fn observe(
        &self,
        _request: &GovernanceRuntimeFactSourceRequest<'_>,
    ) -> Result<GovernanceRuntimeFactObservation, WorkflowOsError> {
        self.calls.set(self.calls.get() + 1);
        if self.fail_with_secret {
            return Err(WorkflowOsError::validation(
                "source.failure.sk-live-marker",
                "source leaked bearer-secret-marker",
            ));
        }
        GovernanceRuntimeFactObservation::new(GovernanceRuntimeFactObservationDefinition {
            source_id: self.source_id.clone(),
            contract_version: self.contract_version.clone(),
            snapshot_id: self.snapshot_id.clone(),
            bundle_binding: self.bundle_binding.clone(),
            observed_at: self.observed_at,
            source_maximum_observation_age_seconds: self.maximum_age,
            runtime_facts: self.facts.clone(),
        })
    }
}

fn source(
    bundle: &workflow_core::StoredImmutableRunBundle,
    observed_at: &str,
    maximum_age: u32,
) -> StaticSource {
    StaticSource {
        source_id: source_id(),
        contract_version: source_version(),
        snapshot_id: GovernanceRuntimeFactSnapshotId::new("snapshot/current-1")
            .expect("snapshot id"),
        bundle_binding: bundle.manifest().run_binding(),
        observed_at: timestamp(observed_at),
        maximum_age,
        facts: vec![fact("verify"), fact("inspect")],
        calls: Cell::new(0),
        fail_with_secret: false,
    }
}

fn assess<'a>(
    bundle: &'a workflow_core::StoredImmutableRunBundle,
    registration: &'a GovernanceRuntimeFactSourceRegistration,
    source: &'a dyn GovernanceRuntimeFactSource,
    evaluated_at: &str,
) -> Result<workflow_core::GovernanceRuntimeFactAssessment, WorkflowOsError> {
    assess_immutable_bundle_governance_from_current_facts(&GovernanceRuntimeFactAssessmentRequest {
        bundle,
        profile: GovernanceStrictnessProfile::ObserveAndReport,
        registration,
        source,
        evaluated_at: timestamp(evaluated_at),
    })
}

#[test]
fn fresh_registered_source_produces_bound_snapshot_and_assessment() {
    let project = TestRoot::new("fresh-project");
    let storage = TestRoot::new("fresh-storage");
    write_project(&project);
    let bundle = stored_bundle(&project, &storage);
    let registration = registration(120);
    let source = source(&bundle, "2026-07-15T12:00:00Z", 60);

    let result =
        assess(&bundle, &registration, &source, "2026-07-15T12:00:30Z").expect("fresh assessment");

    assert_eq!(source.calls.get(), 1);
    assert_eq!(
        result.snapshot().bundle_binding(),
        &bundle.manifest().run_binding()
    );
    assert_eq!(result.snapshot().runtime_fact_count(), 2);
    assert_eq!(
        result
            .snapshot()
            .effective_maximum_observation_age_seconds(),
        60
    );
    assert_eq!(result.assessment_set().assessments().len(), 2);
    assert_eq!(
        result.assessment_set().assessments()[0].step_id().as_str(),
        "inspect"
    );
    assert_eq!(
        result.assessment_set().assessments()[1].step_id().as_str(),
        "verify"
    );
}

#[test]
fn stricter_core_bound_rejects_stale_observation() {
    let project = TestRoot::new("stale-project");
    let storage = TestRoot::new("stale-storage");
    write_project(&project);
    let bundle = stored_bundle(&project, &storage);
    let registration = registration(30);
    let source = source(&bundle, "2026-07-15T12:00:00Z", 300);

    let error = assess(&bundle, &registration, &source, "2026-07-15T12:00:31Z")
        .expect_err("stale rejected");

    assert_eq!(
        error.code(),
        "governance.proportional.runtime_fact_source.observation.stale"
    );
}

#[test]
fn future_dated_observation_fails_closed() {
    let project = TestRoot::new("future-project");
    let storage = TestRoot::new("future-storage");
    write_project(&project);
    let bundle = stored_bundle(&project, &storage);
    let registration = registration(60);
    let source = source(&bundle, "2026-07-15T12:00:01Z", 60);

    let error = assess(&bundle, &registration, &source, "2026-07-15T12:00:00Z")
        .expect_err("future rejected");

    assert_eq!(
        error.code(),
        "governance.proportional.runtime_fact_source.observation.future_dated"
    );
}

#[test]
fn source_and_bundle_identity_mismatches_fail_closed() {
    let project = TestRoot::new("identity-project");
    let storage = TestRoot::new("identity-storage");
    write_project(&project);
    let bundle = stored_bundle(&project, &storage);
    let registration = registration(60);

    let mut wrong_source = source(&bundle, "2026-07-15T12:00:00Z", 60);
    wrong_source.source_id =
        GovernanceRuntimeFactSourceId::new("source/other").expect("other source id");
    let source_error = assess(
        &bundle,
        &registration,
        &wrong_source,
        "2026-07-15T12:00:10Z",
    )
    .expect_err("source mismatch");
    assert_eq!(
        source_error.code(),
        "governance.proportional.runtime_fact_source.source_identity_mismatch"
    );

    let mut wrong_bundle = source(&bundle, "2026-07-15T12:00:00Z", 60);
    let other_project = TestRoot::new("identity-other-project");
    let other_storage = TestRoot::new("identity-other-storage");
    write_project(&other_project);
    let skill_path = other_project.path().join("skills/check.skill.yml");
    let skill = fs::read_to_string(&skill_path).expect("other skill read");
    fs::write(skill_path, skill.replace("Local Check", "Changed Check"))
        .expect("other skill changed");
    wrong_bundle.bundle_binding = stored_bundle(&other_project, &other_storage)
        .manifest()
        .run_binding();
    let bundle_error = assess(
        &bundle,
        &registration,
        &wrong_bundle,
        "2026-07-15T12:00:10Z",
    )
    .expect_err("bundle mismatch");
    assert_eq!(
        bundle_error.code(),
        "governance.proportional.runtime_fact_source.bundle_binding_mismatch"
    );
}

#[test]
fn exact_coverage_is_required_and_fact_order_is_canonical() {
    let project = TestRoot::new("coverage-project");
    let storage = TestRoot::new("coverage-storage");
    write_project(&project);
    let bundle = stored_bundle(&project, &storage);
    let registration = registration(60);

    let ordered = source(&bundle, "2026-07-15T12:00:00Z", 60);
    let ordered_result =
        assess(&bundle, &registration, &ordered, "2026-07-15T12:00:10Z").expect("ordered source");
    let mut reversed = source(&bundle, "2026-07-15T12:00:00Z", 60);
    reversed.facts.reverse();
    let reversed_result =
        assess(&bundle, &registration, &reversed, "2026-07-15T12:00:10Z").expect("reversed source");
    assert_eq!(
        ordered_result.snapshot().runtime_fact_commitment(),
        reversed_result.snapshot().runtime_fact_commitment()
    );

    let mut missing = source(&bundle, "2026-07-15T12:00:00Z", 60);
    missing.facts.pop();
    let error = assess(&bundle, &registration, &missing, "2026-07-15T12:00:10Z")
        .expect_err("missing fact rejected");
    assert_eq!(
        error.code(),
        "governance.proportional.immutable_bundle.runtime_facts_count_mismatch"
    );
}

#[test]
fn fact_changes_invalidate_snapshot_and_assessment_commitments() {
    let project = TestRoot::new("change-project");
    let storage = TestRoot::new("change-storage");
    write_project(&project);
    let bundle = stored_bundle(&project, &storage);
    let registration = registration(60);
    let baseline = source(&bundle, "2026-07-15T12:00:00Z", 60);
    let baseline_result =
        assess(&bundle, &registration, &baseline, "2026-07-15T12:00:10Z").expect("baseline");
    let mut changed = source(&bundle, "2026-07-15T12:00:00Z", 60);
    changed.facts[0] = StepGovernanceRuntimeFacts::new(
        StepId::new("verify").expect("step id"),
        Some(GovernanceWorkloadAuthorityPosture::ApprovalRequired),
        Some(GovernanceWorkloadEvidenceCheckPosture::Satisfied),
        Some(GovernanceWorkloadSideEffectPosture::None),
        None,
        None,
        None,
    );
    let changed_result =
        assess(&bundle, &registration, &changed, "2026-07-15T12:00:10Z").expect("changed");

    assert_ne!(
        baseline_result.snapshot().runtime_fact_commitment(),
        changed_result.snapshot().runtime_fact_commitment()
    );
    assert_ne!(
        baseline_result.assessment_set().aggregate_fingerprint(),
        changed_result.assessment_set().aggregate_fingerprint()
    );
}

#[test]
fn source_failures_and_debug_output_do_not_leak_secret_like_values() {
    let project = TestRoot::new("privacy-project");
    let storage = TestRoot::new("privacy-storage");
    write_project(&project);
    let bundle = stored_bundle(&project, &storage);
    let registration = registration(60);
    let mut failing = source(&bundle, "2026-07-15T12:00:00Z", 60);
    failing.fail_with_secret = true;

    let error = assess(&bundle, &registration, &failing, "2026-07-15T12:00:10Z")
        .expect_err("failure wrapped");
    let rendered = format!("{error:?}");
    assert_eq!(
        error.code(),
        "governance.proportional.runtime_fact_source.source_failed"
    );
    assert!(!rendered.contains("sk-live-marker"));
    assert!(!rendered.contains("bearer-secret-marker"));

    let source = source(&bundle, "2026-07-15T12:00:00Z", 60);
    let result =
        assess(&bundle, &registration, &source, "2026-07-15T12:00:10Z").expect("assessment");
    let debug = format!("{result:?}");
    assert!(!debug.contains("source/local-governance"));
    assert!(!debug.contains("snapshot/current-1"));
    assert!(!debug.contains(result.snapshot().snapshot_commitment().as_str()));

    let serialized = serde_json::to_string(result.snapshot()).expect("snapshot serialization");
    for forbidden in [
        "provider_payload",
        "command_output",
        "parser_payload",
        "raw_spec_contents",
        "authorization_header",
        "private_key",
    ] {
        assert!(!serialized.contains(forbidden));
    }
}

#[test]
fn secret_like_identifiers_and_invalid_freshness_bounds_are_rejected() {
    let identifier_error = GovernanceRuntimeFactSourceId::new("source/bearer-token")
        .expect_err("secret-like id rejected");
    assert_eq!(
        identifier_error.code(),
        "governance.proportional.runtime_fact_source.identifier.secret_like"
    );

    let freshness_error = GovernanceRuntimeFactSourceRegistration::new(
        GovernanceRuntimeFactSourceRegistrationDefinition {
            source_id: source_id(),
            contract_version: source_version(),
            configuration_commitment: SpecContentHash::from_text("safe config"),
            core_maximum_observation_age_seconds: 0,
        },
    )
    .expect_err("zero freshness rejected");
    assert_eq!(
        freshness_error.code(),
        "governance.proportional.runtime_fact_source.freshness_bound_invalid"
    );
}
