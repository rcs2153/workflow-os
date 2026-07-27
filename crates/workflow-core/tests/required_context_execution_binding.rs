#![allow(clippy::expect_used, clippy::panic)]
//! Immutable required-context execution-binding tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    build_immutable_run_bundle, load_project, ActorId, EvidenceReferenceId,
    GovernedContextAccessLevel, GovernedContextReferenceTarget, HarnessContractId,
    HarnessContractVersion, ImmutableRunBundleBuildRequest, ImmutableRunBundleExecutionPosture,
    ImmutableRunBundleHandlerPosture, ImmutableRunBundleHandlerReference, ImmutableRunBundleId,
    ImmutableRunBundleReferencePosture, ImmutableRunBundleSensitivity, ImmutableRunBundleVersion,
    LocalImmutableRunBundleStore, RequiredContextContractBinding, RequiredContextExecutionBinding,
    RequiredContextExecutionBindingInput, RequiredContextExecutionBindingVersion,
    RequiredContextObligation, RequiredContextRequirement, RequiredContextRequirementId, SkillId,
    SkillVersion, SpecContentHash, StepId, Timestamp, WorkReportSensitivity, WorkflowId,
    WorkflowOsErrorKind, WorkflowRunId, SUPPORTED_SCHEMA_VERSION,
};

static NEXT_ROOT: AtomicU64 = AtomicU64::new(1);

struct TestRoot(PathBuf);

impl TestRoot {
    fn new(name: &str) -> Self {
        let id = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "workflow-os-required-context-binding-{name}-{}-{id}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("test root created");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn write(&self, relative: &str, content: &str) {
        let path = self.0.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent created");
        }
        fs::write(path, content).expect("fixture written");
    }
}

impl Drop for TestRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).expect("timestamp")
}

fn write_project(root: &TestRoot) {
    root.write(
        "workflow-os.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nproject:\n  id: binding/project\n  name: Binding Project\n"
        ),
    );
    root.write(
        "workflows/build.workflow.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: binding/build\nversion: v1\ndisplay_name: Binding Build\ntriggers:\n  - id: manual-start\n    kind: manual\nsteps:\n  - id: consume\n    skill_ref:\n      id: local/check\n      version: v1\n    policy_requirements:\n      - id: local/read-only\n    terminal_behavior: fail_workflow\ncancellation_behavior: stop\naudit_requirements:\n  required: true\n  events: [RunCreated, RunCompleted]\n  store_references_only: true\nobservability_requirements:\n  metrics: [workflow_latency]\n  tracing: true\n  latency_tracking: true\n"
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
    bundle_id: &str,
    run_id: &str,
) -> workflow_core::StoredImmutableRunBundle {
    let loaded = load_project(project.path());
    assert!(!loaded.has_errors(), "{:?}", loaded.diagnostics);
    let project = loaded.bundle.expect("project");
    let workflow_id = WorkflowId::new("binding/build").expect("workflow");
    let built = build_immutable_run_bundle(ImmutableRunBundleBuildRequest {
        project: &project,
        workflow_id: &workflow_id,
        bundle_id: ImmutableRunBundleId::new(bundle_id).expect("bundle id"),
        bundle_version: ImmutableRunBundleVersion::new("v1").expect("bundle version"),
        run_id: WorkflowRunId::new(run_id).expect("run id"),
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
        created_at: timestamp("2026-07-26T10:00:00Z"),
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

fn contract(id: &str, target: &str) -> RequiredContextContractBinding {
    RequiredContextContractBinding::new(
        HarnessContractId::new(id).expect("contract id"),
        HarnessContractVersion::new("v1").expect("contract version"),
        vec![RequiredContextRequirement::new(
            RequiredContextRequirementId::new("required/evidence").expect("requirement id"),
            GovernedContextReferenceTarget::EvidenceReference(
                EvidenceReferenceId::new(target).expect("evidence id"),
            ),
            GovernedContextAccessLevel::ReferenceOnly,
            RequiredContextObligation::Required,
            WorkReportSensitivity::Confidential,
        )
        .expect("requirement")],
    )
    .expect("contract")
}

fn binding(
    bundle: &workflow_core::StoredImmutableRunBundle,
    contract: &RequiredContextContractBinding,
) -> Result<RequiredContextExecutionBinding, workflow_core::WorkflowOsError> {
    RequiredContextExecutionBinding::new(RequiredContextExecutionBindingInput {
        bundle,
        contract,
        actor: ActorId::new("agent/context-consumer").expect("actor"),
        step_id: StepId::new("consume").expect("step"),
        maximum_sensitivity: WorkReportSensitivity::Confidential,
        bound_at: timestamp("2026-07-26T10:30:00Z"),
    })
}

#[test]
fn valid_binding_derives_immutable_identity_and_is_deterministic() {
    let project = TestRoot::new("valid-project");
    let storage = TestRoot::new("valid-store");
    write_project(&project);
    let bundle = stored_bundle(&project, &storage, "bundle/context", "run-context");
    let contract = contract("harness/context", "evidence/required");

    let first = binding(&bundle, &contract).expect("binding");
    let second = binding(&bundle, &contract).expect("deterministic binding");

    assert_eq!(first, second);
    assert_eq!(
        first.binding_version(),
        RequiredContextExecutionBindingVersion::V1
    );
    assert_eq!(first.workflow_id().as_str(), "binding/build");
    assert_eq!(first.run_id().as_str(), "run-context");
    assert_eq!(first.step_id().as_str(), "consume");
    assert_eq!(first.harness_contract_id(), contract.contract_id());
    assert_eq!(
        first.harness_contract_version(),
        contract.contract_version()
    );
    assert_eq!(first.contract_content_hash(), contract.content_hash());
    assert_eq!(
        first.immutable_run_bundle().root_hash(),
        bundle.manifest().root_hash()
    );
    first.validate().expect("valid commitment");
}

#[test]
fn absent_step_unknown_sensitivity_and_predating_time_fail_closed() {
    let project = TestRoot::new("invalid-project");
    let storage = TestRoot::new("invalid-store");
    write_project(&project);
    let bundle = stored_bundle(&project, &storage, "bundle/context", "run-context");
    let contract = contract("harness/context", "evidence/required");

    let missing = RequiredContextExecutionBinding::new(RequiredContextExecutionBindingInput {
        bundle: &bundle,
        contract: &contract,
        actor: ActorId::new("agent/context-consumer").expect("actor"),
        step_id: StepId::new("missing").expect("step"),
        maximum_sensitivity: WorkReportSensitivity::Confidential,
        bound_at: timestamp("2026-07-26T10:30:00Z"),
    })
    .expect_err("missing step");
    assert_eq!(missing.kind(), WorkflowOsErrorKind::Validation);
    assert_eq!(
        missing.code(),
        "required_context.execution_binding.step.not_found"
    );

    let unknown = RequiredContextExecutionBinding::new(RequiredContextExecutionBindingInput {
        bundle: &bundle,
        contract: &contract,
        actor: ActorId::new("agent/context-consumer").expect("actor"),
        step_id: StepId::new("consume").expect("step"),
        maximum_sensitivity: WorkReportSensitivity::Unknown,
        bound_at: timestamp("2026-07-26T10:30:00Z"),
    })
    .expect_err("unknown sensitivity");
    assert_eq!(
        unknown.code(),
        "required_context.execution_binding.sensitivity.unknown"
    );

    let early = RequiredContextExecutionBinding::new(RequiredContextExecutionBindingInput {
        bundle: &bundle,
        contract: &contract,
        actor: ActorId::new("agent/context-consumer").expect("actor"),
        step_id: StepId::new("consume").expect("step"),
        maximum_sensitivity: WorkReportSensitivity::Confidential,
        bound_at: timestamp("2026-07-26T09:59:59Z"),
    })
    .expect_err("early binding");
    assert_eq!(
        early.code(),
        "required_context.execution_binding.bound_at.before_bundle"
    );
}

#[test]
fn bundle_contract_actor_and_sensitivity_substitution_change_commitment() {
    let project = TestRoot::new("substitution-project");
    let first_store = TestRoot::new("substitution-store-one");
    let second_store = TestRoot::new("substitution-store-two");
    write_project(&project);
    let first_bundle = stored_bundle(
        &project,
        &first_store,
        "bundle/context-one",
        "run-context-one",
    );
    let second_bundle = stored_bundle(
        &project,
        &second_store,
        "bundle/context-two",
        "run-context-two",
    );
    let first_contract = contract("harness/context", "evidence/required");
    let second_contract = contract("harness/context", "evidence/changed");

    let baseline = binding(&first_bundle, &first_contract).expect("baseline");
    let changed_bundle = binding(&second_bundle, &first_contract).expect("changed bundle");
    let changed_contract = binding(&first_bundle, &second_contract).expect("changed contract");
    let changed_actor =
        RequiredContextExecutionBinding::new(RequiredContextExecutionBindingInput {
            bundle: &first_bundle,
            contract: &first_contract,
            actor: ActorId::new("agent/other").expect("actor"),
            step_id: StepId::new("consume").expect("step"),
            maximum_sensitivity: WorkReportSensitivity::Confidential,
            bound_at: timestamp("2026-07-26T10:30:00Z"),
        })
        .expect("changed actor");
    let changed_sensitivity =
        RequiredContextExecutionBinding::new(RequiredContextExecutionBindingInput {
            bundle: &first_bundle,
            contract: &first_contract,
            actor: ActorId::new("agent/context-consumer").expect("actor"),
            step_id: StepId::new("consume").expect("step"),
            maximum_sensitivity: WorkReportSensitivity::Internal,
            bound_at: timestamp("2026-07-26T10:30:00Z"),
        })
        .expect("changed sensitivity");

    for changed in [
        changed_bundle,
        changed_contract,
        changed_actor,
        changed_sensitivity,
    ] {
        assert_ne!(baseline.binding_hash(), changed.binding_hash());
    }
}

#[test]
fn serde_round_trip_and_tamper_detection_fail_closed_without_leakage() {
    let project = TestRoot::new("serde-project");
    let storage = TestRoot::new("serde-store");
    write_project(&project);
    let bundle = stored_bundle(&project, &storage, "bundle/context", "run-context");
    let contract = contract("harness/context", "evidence/required");
    let value = binding(&bundle, &contract).expect("binding");

    let serialized = serde_json::to_string(&value).expect("serialize");
    let decoded: RequiredContextExecutionBinding =
        serde_json::from_str(&serialized).expect("deserialize");
    assert_eq!(decoded, value);

    let mut tampered = serde_json::to_value(&value).expect("json");
    tampered["actor"] = serde_json::Value::String("agent/substituted".to_owned());
    let error = serde_json::from_value::<RequiredContextExecutionBinding>(tampered)
        .expect_err("tamper rejected");
    assert_eq!(
        error.to_string(),
        "invalid required context execution binding"
    );

    let secret = "token-super-secret-value";
    let mut invalid = serde_json::to_value(&value).expect("json");
    invalid["actor"] = serde_json::Value::String(secret.to_owned());
    let error = serde_json::from_value::<RequiredContextExecutionBinding>(invalid)
        .expect_err("secret-like actor rejected");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn debug_and_serialized_shape_are_payload_free() {
    let project = TestRoot::new("privacy-project");
    let storage = TestRoot::new("privacy-store");
    write_project(&project);
    let bundle = stored_bundle(&project, &storage, "bundle/private", "run-private");
    let contract = contract("harness/private", "evidence/private");
    let value = binding(&bundle, &contract).expect("binding");

    let debug = format!("{value:?}");
    for forbidden in [
        "bundle/private",
        "run-private",
        "binding/build",
        "agent/context-consumer",
        "harness/private",
        "evidence/private",
        value.binding_hash().as_str(),
    ] {
        assert!(!debug.contains(forbidden));
    }

    let serialized = serde_json::to_value(&value).expect("serialize");
    let object = serialized.as_object().expect("object");
    assert_eq!(object.len(), 12);
    for forbidden_field in [
        "payload",
        "content",
        "source",
        "command_output",
        "provider_payload",
        "environment",
        "credential",
        "token",
    ] {
        assert!(!object.contains_key(forbidden_field));
    }
}
