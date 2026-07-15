#![allow(clippy::expect_used, clippy::panic, clippy::too_many_lines)]
//! Behavior tests for create-only local immutable run-bundle storage.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    build_immutable_run_bundle, load_project, ActorId, ImmutableRunBundleBuildRequest,
    ImmutableRunBundleBuildResult, ImmutableRunBundleExecutionPosture,
    ImmutableRunBundleHandlerPosture, ImmutableRunBundleHandlerReference, ImmutableRunBundleId,
    ImmutableRunBundleReferencePosture, ImmutableRunBundleSensitivity, ImmutableRunBundleVersion,
    LocalImmutableRunBundleStore, SkillId, SkillVersion, SpecContentHash, Timestamp, WorkflowId,
    WorkflowOsErrorKind, WorkflowRunId, SUPPORTED_SCHEMA_VERSION,
};

static NEXT_TEST_ROOT: AtomicU64 = AtomicU64::new(1);

struct TestRoot {
    path: PathBuf,
}

impl TestRoot {
    fn new(name: &str) -> Self {
        let id = NEXT_TEST_ROOT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "workflow-os-immutable-bundle-store-{name}-{}-{id}",
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

fn write_project(root: &TestRoot, policy_effect: &str) {
    root.write(
        "workflow-os.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nproject:\n  id: bundle/project\n  name: Bundle Project\n"
        ),
    );
    root.write(
        "workflows/build.workflow.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: bundle/build\nversion: v1\ndisplay_name: Build Bundle\ntriggers:\n  - id: manual-start\n    kind: manual\nsteps:\n  - id: inspect\n    skill_ref:\n      id: local/check\n      version: v1\n    policy_requirements:\n      - id: local/read-only\n    terminal_behavior: fail_workflow\ncancellation_behavior: stop\naudit_requirements:\n  required: true\n  events: [RunCreated, RunCompleted]\n  store_references_only: true\nobservability_requirements:\n  metrics: [workflow_latency]\n  tracing: true\n  latency_tracking: true\n"
        ),
    );
    root.write(
        "skills/check.skill.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/check\nversion: v1\ndisplay_name: Local Check\ninput_contract:\n  fields:\n    - name: request\n      field_type: string\noutput_contract:\n  fields:\n    - name: summary\n      field_type: string\nfailure_modes:\n  - code: check_failed\n    description: Local check failed.\n    retryable: false\naudit_requirements:\n  required: true\n  events: [SkillInvocationRequested]\n  store_references_only: true\nobservability_requirements:\n  metrics: [skill_latency]\n  tracing: true\n  latency_tracking: true\n"
        ),
    );
    root.write(
        "policies/read-only.policy.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/read-only\nname: Read Only\nrules:\n  - id: allow-local\n    effect: {policy_effect}\n"
        ),
    );
}

fn build_bundle(root: &TestRoot, bundle_id: &str, run_id: &str) -> ImmutableRunBundleBuildResult {
    build_bundle_with_sensitivity(
        root,
        bundle_id,
        run_id,
        ImmutableRunBundleSensitivity::Internal,
    )
}

fn build_bundle_with_sensitivity(
    root: &TestRoot,
    bundle_id: &str,
    run_id: &str,
    sensitivity: ImmutableRunBundleSensitivity,
) -> ImmutableRunBundleBuildResult {
    let loaded = load_project(root.path());
    assert!(!loaded.has_errors(), "{:?}", loaded.diagnostics);
    let project = loaded.bundle.expect("loaded project");
    let workflow_id = WorkflowId::new("bundle/build").expect("workflow id");
    build_immutable_run_bundle(ImmutableRunBundleBuildRequest {
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
            ImmutableRunBundleReferencePosture::CommittedReference,
        )
        .expect("execution posture"),
        handlers: vec![ImmutableRunBundleHandlerReference {
            skill_id: SkillId::new("local/check").expect("skill id"),
            skill_version: SkillVersion::new("v1").expect("skill version"),
            posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
        }],
        created_at: Timestamp::parse_rfc3339("2026-07-13T12:00:00Z").expect("timestamp"),
        created_by: ActorId::new("system/kernel").expect("actor"),
        sensitivity,
        redaction_required: true,
    })
    .expect("bundle built")
}

fn encoded_id_file_name(value: &str) -> String {
    use std::fmt::Write as _;

    let encoded = value.as_bytes().iter().fold(
        String::with_capacity(value.len() * 2),
        |mut encoded, byte| {
            write!(&mut encoded, "{byte:02x}").expect("writing to String succeeds");
            encoded
        },
    );
    format!("{encoded}.json")
}

fn record_path(root: &Path, hash: &SpecContentHash) -> PathBuf {
    root.join("definition-records")
        .join(format!("{}.json", hash.as_str()))
}

fn manifest_path(root: &Path, run_id: &str) -> PathBuf {
    root.join("manifests").join(encoded_id_file_name(run_id))
}

#[test]
fn writes_and_reads_complete_bundle_across_store_restart() {
    let project = TestRoot::new("round-trip-project");
    let storage = TestRoot::new("round-trip-storage");
    write_project(&project, "allow_local");
    let bundle = build_bundle(&project, "bundle/run-1", "run-1");

    LocalImmutableRunBundleStore::new(storage.path())
        .write_bundle(&bundle)
        .expect("bundle written");
    let reopened = LocalImmutableRunBundleStore::new(storage.path());
    let stored = reopened
        .read_bundle(bundle.manifest().run_id(), bundle.manifest().bundle_id())
        .expect("bundle read after restart");

    assert_eq!(stored.manifest(), bundle.manifest());
    assert_eq!(stored.definition_records().len(), 3);
    for expected in bundle.definition_records() {
        assert!(stored.definition_records().contains(expected));
    }
}

#[test]
fn later_canonical_variant_does_not_make_existing_bundle_ambiguous() {
    let project = TestRoot::new("variant-project");
    let storage = TestRoot::new("variant-storage");
    write_project(&project, "allow_local");
    let internal = build_bundle(&project, "bundle/internal", "run-internal");
    let restricted = build_bundle_with_sensitivity(
        &project,
        "bundle/restricted",
        "run-restricted",
        ImmutableRunBundleSensitivity::Restricted,
    );
    let store = LocalImmutableRunBundleStore::new(storage.path());

    store
        .write_bundle(&internal)
        .expect("internal bundle written");
    store
        .write_bundle(&restricted)
        .expect("restricted bundle written");

    assert_eq!(
        store
            .read_bundle(
                internal.manifest().run_id(),
                internal.manifest().bundle_id()
            )
            .expect("internal bundle remains readable")
            .manifest(),
        internal.manifest()
    );
    assert_eq!(
        store
            .read_bundle(
                restricted.manifest().run_id(),
                restricted.manifest().bundle_id()
            )
            .expect("restricted bundle readable")
            .manifest(),
        restricted.manifest()
    );
}

#[test]
fn identical_content_addressed_record_write_is_idempotent() {
    let project = TestRoot::new("idempotent-project");
    let storage = TestRoot::new("idempotent-storage");
    write_project(&project, "allow_local");
    let bundle = build_bundle(&project, "bundle/run-1", "run-1");
    let store = LocalImmutableRunBundleStore::new(storage.path());
    let record = &bundle.definition_records()[0];

    store
        .write_definition_record_if_absent(record)
        .expect("first write");
    store
        .write_definition_record_if_absent(record)
        .expect("identical second write");

    assert_eq!(
        store
            .read_definition_record(record.canonical_record_hash())
            .expect("record read"),
        *record
    );
}

#[test]
fn conflicting_content_at_existing_address_fails_without_overwrite() {
    let first_project = TestRoot::new("conflict-first-project");
    let storage = TestRoot::new("conflict-storage");
    write_project(&first_project, "allow_local");
    let first = build_bundle(&first_project, "bundle/run-1", "run-1");
    let changed = build_bundle_with_sensitivity(
        &first_project,
        "bundle/run-2",
        "run-2",
        ImmutableRunBundleSensitivity::Restricted,
    );
    let expected = &first.definition_records()[0];
    let different = changed
        .definition_records()
        .iter()
        .find(|record| record.canonical_record_hash() != expected.canonical_record_hash())
        .expect("different record");
    let path = record_path(storage.path(), expected.canonical_record_hash());
    fs::create_dir_all(path.parent().expect("parent")).expect("directory created");
    fs::write(
        &path,
        serde_json::to_vec_pretty(different).expect("serialize"),
    )
    .expect("different record injected");
    let store = LocalImmutableRunBundleStore::new(storage.path());

    let error = store
        .write_definition_record_if_absent(expected)
        .expect_err("conflict rejected");

    assert_eq!(error.code(), "immutable_run_bundle_store.record_conflict");
    assert_eq!(
        serde_json::from_slice::<workflow_core::ImmutableRunBundleDefinitionRecord>(
            &fs::read(path).expect("record remains")
        )
        .expect("stored record valid"),
        *different
    );
}

#[test]
fn duplicate_manifest_write_is_rejected_without_rebinding_run() {
    let project = TestRoot::new("duplicate-project");
    let storage = TestRoot::new("duplicate-storage");
    write_project(&project, "allow_local");
    let bundle = build_bundle(&project, "bundle/run-1", "run-private-1");
    let store = LocalImmutableRunBundleStore::new(storage.path());
    store.write_bundle(&bundle).expect("first write");

    let error = store.write_bundle(&bundle).expect_err("duplicate rejected");

    assert_eq!(error.kind(), WorkflowOsErrorKind::InvalidState);
    assert_eq!(error.code(), "immutable_run_bundle_store.manifest_exists");
    assert!(!error.to_string().contains("run-private-1"));
}

#[test]
fn run_ids_are_encoded_and_record_hashes_are_safe_file_names() {
    let project = TestRoot::new("safe-key-project");
    let storage = TestRoot::new("safe-key-storage");
    write_project(&project, "allow_local");
    let bundle = build_bundle(&project, "bundle/safe-key", "run/safe-key");
    let store = LocalImmutableRunBundleStore::new(storage.path());
    store.write_bundle(&bundle).expect("bundle written");

    assert!(manifest_path(storage.path(), "run/safe-key").exists());
    assert!(!storage.path().join("manifests/run").exists());
    for record in bundle.definition_records() {
        assert!(record_path(storage.path(), record.canonical_record_hash()).exists());
    }
}

#[test]
fn manifest_publication_fails_when_a_definition_is_missing() {
    let project = TestRoot::new("missing-project");
    let storage = TestRoot::new("missing-storage");
    write_project(&project, "allow_local");
    let bundle = build_bundle(&project, "bundle/run-1", "run-1");
    let store = LocalImmutableRunBundleStore::new(storage.path());

    let error = store
        .write_manifest_create_only(bundle.manifest())
        .expect_err("missing definitions reject manifest");

    assert_eq!(
        error.code(),
        "immutable_run_bundle_store.definition_missing"
    );
    assert!(!manifest_path(storage.path(), "run-1").exists());
    assert!(!storage.path().join("manifests").exists());
}

#[test]
fn corrupt_record_fails_closed_without_leaking_payload_or_publishing_manifest() {
    let project = TestRoot::new("corrupt-project");
    let storage = TestRoot::new("corrupt-storage");
    write_project(&project, "allow_local");
    let bundle = build_bundle(&project, "bundle/run-1", "run-1");
    let store = LocalImmutableRunBundleStore::new(storage.path());
    for record in bundle.definition_records() {
        store
            .write_definition_record_if_absent(record)
            .expect("record written");
    }
    let corrupted = &bundle.definition_records()[0];
    fs::write(
        record_path(storage.path(), corrupted.canonical_record_hash()),
        b"{\"authorization\":\"bearer private token\"",
    )
    .expect("corruption injected");

    let error = store
        .write_manifest_create_only(bundle.manifest())
        .expect_err("corruption rejected");

    assert_eq!(error.code(), "immutable_run_bundle_store.invalid_record");
    assert!(!error.to_string().contains("bearer"));
    assert!(!error.to_string().contains("token"));
    assert!(!manifest_path(storage.path(), "run-1").exists());
}

#[test]
fn missing_record_after_restart_makes_complete_bundle_unavailable() {
    let project = TestRoot::new("restart-missing-project");
    let storage = TestRoot::new("restart-missing-storage");
    write_project(&project, "allow_local");
    let bundle = build_bundle(&project, "bundle/run-1", "run-1");
    LocalImmutableRunBundleStore::new(storage.path())
        .write_bundle(&bundle)
        .expect("bundle written");
    let removed = &bundle.definition_records()[0];
    fs::remove_file(record_path(storage.path(), removed.canonical_record_hash()))
        .expect("record removed");
    let reopened = LocalImmutableRunBundleStore::new(storage.path());

    let error = reopened
        .read_bundle(bundle.manifest().run_id(), bundle.manifest().bundle_id())
        .expect_err("missing stored record rejected");

    assert_eq!(error.code(), "immutable_run_bundle_store.not_found");
    assert!(!error.to_string().contains("bundle/run-1"));
}

#[test]
fn manifest_storage_identity_mismatch_fails_closed() {
    let project = TestRoot::new("identity-project");
    let storage = TestRoot::new("identity-storage");
    write_project(&project, "allow_local");
    let bundle = build_bundle(&project, "bundle/run-1", "run-1");
    let store = LocalImmutableRunBundleStore::new(storage.path());
    store.write_bundle(&bundle).expect("bundle written");
    let wrong_run = WorkflowRunId::new("run-private-2").expect("run id");
    let wrong_path = manifest_path(storage.path(), wrong_run.as_str());
    fs::copy(manifest_path(storage.path(), "run-1"), &wrong_path)
        .expect("manifest copied to wrong address");

    let error = store
        .read_manifest(&wrong_run, bundle.manifest().bundle_id())
        .expect_err("identity mismatch rejected");

    assert_eq!(error.code(), "immutable_run_bundle_store.identity_mismatch");
    assert!(!error.to_string().contains("run-private-2"));
}

#[test]
fn failed_manifest_publication_leaves_no_temporary_files() {
    let project = TestRoot::new("temp-cleanup-project");
    let storage = TestRoot::new("temp-cleanup-storage");
    write_project(&project, "allow_local");
    let bundle = build_bundle(&project, "bundle/run-1", "run-1");
    let store = LocalImmutableRunBundleStore::new(storage.path());
    store.write_bundle(&bundle).expect("bundle written");
    let _error = store.write_bundle(&bundle).expect_err("duplicate rejected");

    let temp_files = fs::read_dir(storage.path().join("manifests"))
        .expect("manifest directory")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|value| value.to_str()) != Some("json"))
        .count();
    assert_eq!(temp_files, 0);
}

#[test]
fn store_debug_is_redacted_and_writes_no_runtime_state() {
    let project = TestRoot::new("debug-project");
    let storage = TestRoot::new("private-token-storage");
    write_project(&project, "allow_local");
    let bundle = build_bundle(&project, "bundle/run-1", "run-1");
    let store = LocalImmutableRunBundleStore::new(storage.path());

    store.write_bundle(&bundle).expect("bundle written");
    let debug = format!("{store:?}");

    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains(storage.path().to_string_lossy().as_ref()));
    assert!(!project.path().join(".workflow-os/state").exists());
    assert!(!storage.path().join("state").exists());
    assert!(!storage.path().join("events").exists());
}
