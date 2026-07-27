#![allow(clippy::expect_used)]
//! Current authority fact-set model tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    build_immutable_run_bundle, load_project, ActorId, AuthorityFactCompletenessPosture,
    AuthorityFactSourceKind, CapabilityAvailability, CapabilityAvailabilityRecord,
    CurrentAuthorityFactSet, CurrentAuthorityFactSetInput, GovernedContextAccessLevel,
    GovernedContextReferenceTarget, HarnessContractId, HarnessContractVersion,
    ImmutableRunBundleBuildRequest, ImmutableRunBundleExecutionPosture,
    ImmutableRunBundleHandlerPosture, ImmutableRunBundleHandlerReference, ImmutableRunBundleId,
    ImmutableRunBundleReferencePosture, ImmutableRunBundleSensitivity, ImmutableRunBundleVersion,
    LocalImmutableRunBundleStore, RedactionMetadata, RequiredContextContractBinding,
    RequiredContextExecutionBinding, RequiredContextExecutionBindingInput,
    RequiredContextObligation, RequiredContextRequirement, RequiredContextRequirementId, SkillId,
    SkillVersion, SpecContentHash, StepId, Timestamp, WorkReportId, WorkReportSensitivity,
    WorkflowId, WorkflowRunId, SUPPORTED_SCHEMA_VERSION,
};

static NEXT_ROOT: AtomicU64 = AtomicU64::new(1);

struct TestRoot(PathBuf);

impl TestRoot {
    fn new(name: &str) -> Self {
        let id = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "workflow-os-current-authority-{name}-{}-{id}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("test root");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn write(&self, relative: &str, content: &str) {
        let path = self.0.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent");
        }
        fs::write(path, content).expect("fixture");
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

fn fixture() -> (
    workflow_core::StoredImmutableRunBundle,
    RequiredContextContractBinding,
    RequiredContextExecutionBinding,
) {
    let project_root = TestRoot::new("project");
    let store_root = TestRoot::new("store");
    project_root.write(
        "workflow-os.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nproject:\n  id: authority/project\n  name: Authority Project\n"
        ),
    );
    project_root.write(
        "workflows/build.workflow.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: authority/build\nversion: v1\ndisplay_name: Authority Build\ntriggers:\n  - id: manual\n    kind: manual\nsteps:\n  - id: consume\n    skill_ref:\n      id: local/check\n      version: v1\n    policy_requirements:\n      - id: local/read-only\n    terminal_behavior: fail_workflow\ncancellation_behavior: stop\naudit_requirements:\n  required: true\n  events: [RunCreated]\n  store_references_only: true\nobservability_requirements:\n  metrics: [workflow_latency]\n  tracing: true\n  latency_tracking: true\n"
        ),
    );
    project_root.write(
        "skills/check.skill.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/check\nversion: v1\ndisplay_name: Check\nallowed_capabilities:\n  - name: local.read\ninput_contract:\n  fields:\n    - name: request\n      field_type: string\noutput_contract:\n  fields:\n    - name: summary\n      field_type: string\nfailure_modes:\n  - code: failed\n    description: Failed.\n    retryable: false\naudit_requirements:\n  required: true\n  events: [SkillInvocationRequested]\n  store_references_only: true\nobservability_requirements:\n  metrics: [skill_latency]\n  tracing: true\n  latency_tracking: true\n"
        ),
    );
    project_root.write(
        "policies/read-only.policy.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/read-only\nname: Read only\nrules:\n  - id: allow\n    effect: allow_local\n"
        ),
    );
    let loaded = load_project(project_root.path());
    assert!(!loaded.has_errors(), "{:?}", loaded.diagnostics);
    let project = loaded.bundle.expect("project");
    let built = build_immutable_run_bundle(ImmutableRunBundleBuildRequest {
        project: &project,
        workflow_id: &WorkflowId::new("authority/build").expect("workflow"),
        bundle_id: ImmutableRunBundleId::new("bundle/authority").expect("bundle"),
        bundle_version: ImmutableRunBundleVersion::new("v1").expect("version"),
        run_id: WorkflowRunId::new("run-authority").expect("run"),
        resolved_execution_context_hash: SpecContentHash::from_text("context"),
        execution_posture: ImmutableRunBundleExecutionPosture::new(
            Vec::new(),
            ImmutableRunBundleReferencePosture::NotSupplied,
            ImmutableRunBundleReferencePosture::NotSupplied,
            ImmutableRunBundleReferencePosture::NotSupplied,
        )
        .expect("posture"),
        handlers: vec![ImmutableRunBundleHandlerReference {
            skill_id: SkillId::new("local/check").expect("skill"),
            skill_version: SkillVersion::new("v1").expect("skill version"),
            posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
        }],
        created_at: timestamp("2026-07-26T10:00:00Z"),
        created_by: ActorId::new("system/kernel").expect("actor"),
        sensitivity: ImmutableRunBundleSensitivity::Internal,
        redaction_required: true,
    })
    .expect("built");
    let store = LocalImmutableRunBundleStore::new(store_root.path());
    store.write_bundle(&built).expect("write");
    let stored = store
        .read_bundle(built.manifest().run_id(), built.manifest().bundle_id())
        .expect("read");
    let contract = RequiredContextContractBinding::new(
        HarnessContractId::new("harness/context").expect("contract"),
        HarnessContractVersion::new("v1").expect("contract version"),
        vec![
            RequiredContextRequirement::new(
                RequiredContextRequirementId::new("required/report-reference")
                    .expect("requirement"),
                GovernedContextReferenceTarget::WorkReport(
                    WorkReportId::new("report/current").expect("report"),
                ),
                GovernedContextAccessLevel::ReferenceOnly,
                RequiredContextObligation::Required,
                WorkReportSensitivity::Internal,
            )
            .expect("requirement"),
            RequiredContextRequirement::new(
                RequiredContextRequirementId::new("required/report-metadata").expect("requirement"),
                GovernedContextReferenceTarget::WorkReport(
                    WorkReportId::new("report/metadata").expect("report"),
                ),
                GovernedContextAccessLevel::BoundedMetadata,
                RequiredContextObligation::Required,
                WorkReportSensitivity::Internal,
            )
            .expect("requirement"),
        ],
    )
    .expect("contract");
    let binding = RequiredContextExecutionBinding::new(RequiredContextExecutionBindingInput {
        bundle: &stored,
        contract: &contract,
        actor: ActorId::new("agent/consumer").expect("actor"),
        step_id: StepId::new("consume").expect("step"),
        maximum_sensitivity: WorkReportSensitivity::Internal,
        bound_at: timestamp("2026-07-26T10:10:00Z"),
    })
    .expect("binding");
    (stored, contract, binding)
}

fn availability(contract: &RequiredContextContractBinding) -> Vec<CapabilityAvailabilityRecord> {
    contract
        .requirements()
        .iter()
        .map(|requirement| {
            CapabilityAvailabilityRecord::new(
                requirement
                    .access_level()
                    .required_capability()
                    .expect("capability"),
                requirement
                    .target()
                    .capability_resource()
                    .expect("resource"),
                CapabilityAvailability::Available,
                timestamp("2026-07-26T10:20:00Z"),
                RedactionMetadata::empty(),
            )
            .expect("availability")
        })
        .collect()
}

fn fact_set(
    binding: &RequiredContextExecutionBinding,
    contract: &RequiredContextContractBinding,
    availability_records: Vec<CapabilityAvailabilityRecord>,
) -> Result<CurrentAuthorityFactSet, workflow_core::WorkflowOsError> {
    CurrentAuthorityFactSet::new(CurrentAuthorityFactSetInput {
        execution_binding: binding,
        contract,
        source_kind: AuthorityFactSourceKind::InMemoryInventorySnapshot,
        source_snapshot_hash: SpecContentHash::from_text("snapshot"),
        source_observed_at: timestamp("2026-07-26T10:20:00Z"),
        completeness: AuthorityFactCompletenessPosture::CompleteForExactQuery,
        evaluated_at: timestamp("2026-07-26T10:30:00Z"),
        grants: Vec::new(),
        availability_records,
    })
}

#[test]
fn complete_fact_set_derives_queries_and_round_trips() {
    let (_bundle, contract, binding) = fixture();
    let value = fact_set(&binding, &contract, availability(&contract)).expect("fact set");

    assert_eq!(value.query_set().queries().len(), 2);
    assert_eq!(value.availability_records().len(), 2);
    assert!(value.grants().is_empty());
    assert_eq!(
        value.fact_set_hash().as_str(),
        "ca724e50983d9fbccc1ded97e958466fe0b86e303bd57d7c68608dc5e0f16af3"
    );
    value.validate().expect("valid");

    let serialized = serde_json::to_string(&value).expect("serialize");
    let decoded: CurrentAuthorityFactSet = serde_json::from_str(&serialized).expect("deserialize");
    assert_eq!(decoded, value);
}

#[test]
fn complete_posture_rejects_missing_and_duplicate_availability() {
    let (_bundle, contract, binding) = fixture();
    let missing = fact_set(&binding, &contract, Vec::new()).expect_err("missing");
    assert_eq!(
        missing.code(),
        "current_authority.fact_set.availability.incomplete"
    );

    let mut records = availability(&contract);
    records.push(records[0].clone());
    let duplicate = fact_set(&binding, &contract, records).expect_err("duplicate");
    assert_eq!(
        duplicate.code(),
        "current_authority.fact_set.availability.duplicate"
    );
}

#[test]
fn wire_tampering_fails_closed_without_leaking_values() {
    let (_bundle, contract, binding) = fixture();
    let value = fact_set(&binding, &contract, availability(&contract)).expect("fact set");
    let mut wire = serde_json::to_value(&value).expect("json");
    let secret = "token-super-secret-value";
    wire["execution_binding_hash"] = serde_json::Value::String(secret.to_owned());
    let error = serde_json::from_value::<CurrentAuthorityFactSet>(wire).expect_err("tamper");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn noncanonical_wire_record_order_fails_closed() {
    let (_bundle, contract, binding) = fixture();
    let value = fact_set(&binding, &contract, availability(&contract)).expect("fact set");
    let mut wire = serde_json::to_value(&value).expect("json");
    wire["availability_records"]
        .as_array_mut()
        .expect("availability array")
        .reverse();

    let error = serde_json::from_value::<CurrentAuthorityFactSet>(wire).expect_err("order");
    assert_eq!(error.to_string(), "invalid current authority fact set");
}

#[test]
fn debug_and_serialized_shape_are_payload_free() {
    let (_bundle, contract, binding) = fixture();
    let value = fact_set(&binding, &contract, availability(&contract)).expect("fact set");
    let debug = format!("{value:?}");
    for forbidden in [
        "report/current",
        "report/metadata",
        "agent/consumer",
        "harness/context",
        "run-authority",
        value.fact_set_hash().as_str(),
    ] {
        assert!(!debug.contains(forbidden));
    }

    let wire = serde_json::to_value(&value).expect("json");
    let object = wire.as_object().expect("object");
    for forbidden in [
        "payload",
        "source_content",
        "command_output",
        "provider_payload",
        "credential",
        "token",
    ] {
        assert!(!object.contains_key(forbidden));
    }
}
