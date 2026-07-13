#![allow(clippy::expect_used, clippy::panic, clippy::too_many_lines)]
//! Behavior tests for the pure in-memory immutable run-bundle builder.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    build_immutable_run_bundle, load_project, validate_project_bundle, ActorId,
    ImmutableRunBundleBuildRequest, ImmutableRunBundleDefinitionKind,
    ImmutableRunBundleExecutionPosture, ImmutableRunBundleHandlerPosture,
    ImmutableRunBundleHandlerReference, ImmutableRunBundleId, ImmutableRunBundleReferencePosture,
    ImmutableRunBundleSensitivity, ImmutableRunBundleVersion, SkillId, SkillVersion,
    SpecContentHash, StepId, Timestamp, WorkflowId, WorkflowRunId, SUPPORTED_SCHEMA_VERSION,
};

static NEXT_TEST_PROJECT: AtomicU64 = AtomicU64::new(1);

struct TestProject {
    root: PathBuf,
}

impl TestProject {
    fn new() -> Self {
        let id = NEXT_TEST_PROJECT.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "workflow-os-immutable-bundle-builder-{}-{id}",
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

    fn write(&self, relative_path: &str, content: &str) {
        let path = self.root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent directory is created");
        }
        fs::write(path, content).expect("test file is written");
    }

    fn write_valid_project(&self) {
        self.write(
            "workflow-os.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
project:
  id: bundle/project
  name: Bundle Project
"
            ),
        );
        self.write(
            "workflows/build.workflow.yml",
            &format!(
                r"
# source comment must not enter canonical records
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: bundle/build
version: v1
display_name: Build Bundle
triggers:
  - id: manual-start
    kind: manual
steps:
  - id: inspect
    skill_ref:
      id: local/check
      version: v1
    policy_requirements:
      - id: local/read-only
    terminal_behavior: fail_workflow
  - id: verify
    skill_ref:
      id: local/check
      version: v1
    policy_requirements:
      - id: local/read-only
    terminal_behavior: fail_workflow
cancellation_behavior: stop
audit_requirements:
  required: true
  events:
    - RunCreated
    - RunCompleted
  store_references_only: true
observability_requirements:
  metrics:
    - workflow_latency
  tracing: true
  latency_tracking: true
"
            ),
        );
        self.write(
            "skills/check.skill.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/check
version: v1
display_name: Local Check
input_contract:
  fields:
    - name: request
      field_type: string
output_contract:
  fields:
    - name: summary
      field_type: string
failure_modes:
  - code: check_failed
    description: The local check failed.
    retryable: false
audit_requirements:
  required: true
  events:
    - SkillInvocationRequested
  store_references_only: true
observability_requirements:
  metrics:
    - skill_latency
  tracing: true
  latency_tracking: true
"
            ),
        );
        self.write(
            "skills/unused.skill.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/unused
version: v1
display_name: Unused Skill
input_contract:
  fields:
    - name: request
      field_type: string
output_contract:
  fields:
    - name: summary
      field_type: string
failure_modes:
  - code: unused_failed
    description: The unused skill failed.
    retryable: false
audit_requirements:
  required: true
  events:
    - SkillInvocationRequested
  store_references_only: true
observability_requirements:
  metrics:
    - skill_latency
  tracing: true
  latency_tracking: true
"
            ),
        );
        self.write(
            "policies/read-only.policy.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/read-only
name: Read Only
rules:
  - id: allow-local
    effect: allow_local
"
            ),
        );
        self.write(
            "policies/unused.policy.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/unused
name: Unused Policy
rules:
  - id: allow-local
    effect: allow_local
"
            ),
        );
    }
}

impl Drop for TestProject {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn handler() -> ImmutableRunBundleHandlerReference {
    ImmutableRunBundleHandlerReference {
        skill_id: SkillId::new("local/check").expect("skill id"),
        skill_version: SkillVersion::new("v1").expect("skill version"),
        posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
    }
}

fn build<'a>(
    project: &'a workflow_core::ProjectBundle,
    workflow_id: &'a WorkflowId,
    handlers: Vec<ImmutableRunBundleHandlerReference>,
) -> Result<workflow_core::ImmutableRunBundleBuildResult, workflow_core::WorkflowOsError> {
    build_immutable_run_bundle(ImmutableRunBundleBuildRequest {
        project,
        workflow_id,
        bundle_id: ImmutableRunBundleId::new("bundle/run-1").expect("bundle id"),
        bundle_version: ImmutableRunBundleVersion::new("v1").expect("bundle version"),
        run_id: WorkflowRunId::new("run-1").expect("run id"),
        resolved_execution_context_hash: SpecContentHash::from_text("resolved context"),
        execution_posture: ImmutableRunBundleExecutionPosture::new(
            vec![StepId::new("inspect").expect("step id")],
            ImmutableRunBundleReferencePosture::NotSupplied,
            ImmutableRunBundleReferencePosture::NotSupplied,
            ImmutableRunBundleReferencePosture::CommittedReference,
        )
        .expect("execution posture"),
        handlers,
        created_at: Timestamp::parse_rfc3339("2026-07-13T12:00:00Z").expect("timestamp"),
        created_by: ActorId::new("system/kernel").expect("actor"),
        sensitivity: ImmutableRunBundleSensitivity::Internal,
        redaction_required: true,
    })
}

fn loaded_project() -> (TestProject, workflow_core::ProjectBundle) {
    let project = TestProject::new();
    project.write_valid_project();
    let loaded = load_project(project.path());
    assert!(!loaded.has_errors());
    let bundle = loaded.bundle.expect("project bundle");
    let validation = validate_project_bundle(&bundle);
    assert!(!validation.has_errors(), "{:?}", validation.diagnostics);
    (project, bundle)
}

#[test]
fn builds_matching_manifest_and_deduplicated_canonical_records() {
    let (_project, bundle) = loaded_project();
    let workflow_id = WorkflowId::new("bundle/build").expect("workflow id");

    let result = build(&bundle, &workflow_id, vec![handler()]).expect("bundle builds");

    assert_eq!(result.definition_records().len(), 3);
    assert_eq!(result.manifest().definitions().len(), 4);
    assert_eq!(
        result
            .manifest()
            .definitions()
            .iter()
            .filter(|reference| reference.kind() == ImmutableRunBundleDefinitionKind::Skill)
            .count(),
        2
    );
    assert_eq!(result.manifest().handlers().len(), 1);
    assert_eq!(result.manifest().workflow_id(), &workflow_id);
}

#[test]
fn sources_definition_hashes_from_loaded_specs_and_excludes_unreferenced_definitions() {
    let (_project, bundle) = loaded_project();
    let workflow_id = WorkflowId::new("bundle/build").expect("workflow id");

    let result = build(&bundle, &workflow_id, vec![handler()]).expect("bundle builds");

    let source_hashes = result
        .definition_records()
        .iter()
        .map(workflow_core::ImmutableRunBundleDefinitionRecord::source_content_hash)
        .collect::<Vec<_>>();
    assert!(source_hashes.contains(&&bundle.workflows[0].content_hash));
    assert!(source_hashes.contains(&&bundle.skills[0].content_hash));
    assert!(source_hashes.contains(&&bundle.policies[0].content_hash));
    assert!(!result
        .definition_records()
        .iter()
        .any(|record| record.definition_id().contains("unused")));
}

#[test]
fn identical_explicit_inputs_produce_identical_root_hashes() {
    let (_project, bundle) = loaded_project();
    let workflow_id = WorkflowId::new("bundle/build").expect("workflow id");

    let first = build(&bundle, &workflow_id, vec![handler()]).expect("first bundle");
    let second = build(&bundle, &workflow_id, vec![handler()]).expect("second bundle");

    assert_eq!(first.manifest().root_hash(), second.manifest().root_hash());
}

#[test]
fn mismatched_handler_posture_fails_closed() {
    let (_project, bundle) = loaded_project();
    let workflow_id = WorkflowId::new("bundle/build").expect("workflow id");

    let error = build(&bundle, &workflow_id, Vec::new()).expect_err("handler mismatch");

    assert_eq!(error.code(), "immutable_run_bundle.handlers.skill_mismatch");
}

#[test]
fn missing_workflow_fails_without_leaking_the_requested_identity() {
    let (_project, bundle) = loaded_project();
    let missing = WorkflowId::new("bundle/private-workflow").expect("workflow id");

    let error = build(&bundle, &missing, vec![handler()]).expect_err("missing workflow");

    assert_eq!(
        error.code(),
        "immutable_run_bundle.builder.workflow_not_found"
    );
    assert!(!error.to_string().contains("private-workflow"));
}

#[test]
fn invalid_project_fails_before_record_construction() {
    let (_project, mut bundle) = loaded_project();
    bundle.skills.clear();
    let workflow_id = WorkflowId::new("bundle/build").expect("workflow id");

    let error = build(&bundle, &workflow_id, vec![handler()]).expect_err("invalid project");

    assert_eq!(error.code(), "immutable_run_bundle.builder.project_invalid");
}

#[test]
fn builder_debug_and_storage_shapes_exclude_paths_comments_and_raw_yaml() {
    let (project, bundle) = loaded_project();
    let workflow_id = WorkflowId::new("bundle/build").expect("workflow id");
    let request = ImmutableRunBundleBuildRequest {
        project: &bundle,
        workflow_id: &workflow_id,
        bundle_id: ImmutableRunBundleId::new("bundle/run-1").expect("bundle id"),
        bundle_version: ImmutableRunBundleVersion::new("v1").expect("bundle version"),
        run_id: WorkflowRunId::new("run-1").expect("run id"),
        resolved_execution_context_hash: SpecContentHash::from_text("resolved context"),
        execution_posture: ImmutableRunBundleExecutionPosture::new(
            Vec::new(),
            ImmutableRunBundleReferencePosture::NotSupplied,
            ImmutableRunBundleReferencePosture::NotSupplied,
            ImmutableRunBundleReferencePosture::NotSupplied,
        )
        .expect("execution posture"),
        handlers: vec![handler()],
        created_at: Timestamp::parse_rfc3339("2026-07-13T12:00:00Z").expect("timestamp"),
        created_by: ActorId::new("system/kernel").expect("actor"),
        sensitivity: ImmutableRunBundleSensitivity::Internal,
        redaction_required: true,
    };
    let request_debug = format!("{request:?}");

    assert!(!request_debug.contains(project.path().to_string_lossy().as_ref()));
    assert!(!request_debug.contains("bundle/build"));

    let result = build_immutable_run_bundle(request).expect("bundle builds");
    let result_debug = format!("{result:?}");
    let records_json = serde_json::to_string(result.definition_records()).expect("records json");
    assert!(!result_debug.contains("bundle/build"));
    assert!(!result_debug.contains(project.path().to_string_lossy().as_ref()));
    assert!(!records_json.contains(project.path().to_string_lossy().as_ref()));
    assert!(!records_json.contains("source comment must not enter canonical records"));
    assert!(!records_json.contains("raw_yaml"));
}

#[test]
fn builder_creates_no_runtime_state_or_filesystem_artifacts() {
    let (project, bundle) = loaded_project();
    let workflow_id = WorkflowId::new("bundle/build").expect("workflow id");

    let _result = build(&bundle, &workflow_id, vec![handler()]).expect("bundle builds");

    assert!(!project.path().join(".workflow-os/state").exists());
    assert!(!project.path().join("artifacts").exists());
}
