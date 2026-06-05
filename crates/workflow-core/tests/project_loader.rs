#![allow(clippy::expect_used, clippy::panic)]
//! Behavior tests for the v0 project loader.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    load_project, DiagnosticSeverity, EvidenceKind, EvidenceReferenceTarget, EvidenceScope,
    EvidenceSensitivity, SUPPORTED_SCHEMA_VERSION,
};

static NEXT_TEST_PROJECT: AtomicU64 = AtomicU64::new(1);

struct TestProject {
    root: PathBuf,
}

impl TestProject {
    fn new(name: &str) -> Self {
        let id = NEXT_TEST_PROJECT.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "workflow-os-loader-{name}-{}-{id}",
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

    fn write_manifest(&self) {
        self.write(
            "workflow-os.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
project:
  id: acme/approval
  name: Acme Approval
layout:
  workflows: workflows
  skills: skills
  policies: policies
  tests: tests
"
            ),
        );
    }

    fn write_valid_project(&self) {
        self.write_manifest();
        self.write(
            "workflows/request.workflow.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/request
version: v0
display_name: Request Approval
steps:
  - id: draft
    skill_ref:
      id: local/draft-summary
      version: v0
"
            ),
        );
        self.write(
            "skills/draft.skill.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/draft-summary
version: v0
display_name: Draft Summary
"
            ),
        );
        self.write(
            "policies/default.policy.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/default
name: Default Approval
"
            ),
        );
        self.write(
            "tests/request.test.yml",
            &format!(
                r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/request-basic
name: Request basic
target:
  id: approval/request
  version: v0
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

#[test]
fn loads_valid_project_into_bundle() {
    let project = TestProject::new("valid");
    project.write_valid_project();

    let result = load_project(project.path());

    assert!(!result.has_errors());
    let bundle = result.bundle.expect("bundle is loaded");
    assert_eq!(
        bundle.manifest.definition.project.id.as_str(),
        "acme/approval"
    );
    assert_eq!(bundle.workflows.len(), 1);
    assert_eq!(bundle.skills.len(), 1);
    assert_eq!(bundle.policies.len(), 1);
    assert_eq!(bundle.tests.len(), 1);
    assert_eq!(
        bundle.workflows[0]
            .definition
            .source_location
            .as_ref()
            .expect("workflow source")
            .file_path(),
        bundle.workflows[0].path
    );
    assert_eq!(
        bundle.workflows[0]
            .definition
            .spec_content_hash
            .as_ref()
            .expect("workflow hash"),
        &bundle.workflows[0].content_hash
    );
}

#[test]
fn reports_missing_manifest() {
    let project = TestProject::new("missing-manifest");

    let result = load_project(project.path());

    assert!(result.bundle.is_none());
    assert!(result.has_errors());
    assert_eq!(result.diagnostics[0].code(), "loader.manifest_missing");
    assert!(result.diagnostics[0].source_location().is_some());
}

#[test]
fn reports_invalid_yaml_with_line_and_column() {
    let project = TestProject::new("invalid-yaml");
    project.write(
        "workflow-os.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
project:
  id: acme/approval
  name: [unterminated
"
        ),
    );

    let result = load_project(project.path());

    assert!(result.bundle.is_none());
    let diagnostic = result
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code() == "yaml.parse")
        .expect("yaml parse diagnostic");
    let location = diagnostic.source_location().expect("source location");
    assert!(location.line().is_some());
    assert!(location.column().is_some());
}

#[test]
fn reports_unsupported_schema_version() {
    let project = TestProject::new("unsupported-schema");
    project.write(
        "workflow-os.yml",
        r"
schema_version: workflowos.dev/v99
project:
  id: acme/approval
  name: Acme Approval
",
    );

    let result = load_project(project.path());

    assert!(result.bundle.is_none());
    let diagnostic = result
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code() == "schema_version.unsupported")
        .expect("unsupported schema diagnostic");
    assert_eq!(
        diagnostic
            .source_location()
            .expect("source location")
            .document_path(),
        Some("$.schema_version")
    );
}

#[test]
fn schema_version_diagnostic_attaches_spec_file_evidence() {
    let project = TestProject::new("schema-evidence");
    project.write(
        "workflow-os.yml",
        r"
schema_version: workflowos.dev/v99
project:
  id: acme/approval
  name: Acme Approval
",
    );

    let result = load_project(project.path());

    assert_eq!(result.diagnostics.len(), 1);
    let diagnostic = &result.diagnostics[0];
    let source_location = diagnostic
        .source_location()
        .expect("schema diagnostic source location");
    assert_eq!(diagnostic.code(), "schema_version.unsupported");
    assert_eq!(diagnostic.severity(), DiagnosticSeverity::Error);
    assert!(diagnostic.message().contains("unsupported schema_version"));
    assert_eq!(source_location.document_path(), Some("$.schema_version"));
    assert_eq!(diagnostic.evidence_references().len(), 1);

    let evidence = &diagnostic.evidence_references()[0];
    assert_eq!(evidence.kind, EvidenceKind::SpecFile);
    assert_eq!(evidence.scope, EvidenceScope::Project);
    assert_eq!(evidence.sensitivity, EvidenceSensitivity::Confidential);
    assert!(evidence.summary.is_none());
    assert_ne!(evidence.title, diagnostic.message());
    assert_eq!(
        evidence
            .redaction_metadata
            .metadata
            .field_states
            .first()
            .expect("redaction metadata")
            .field,
        "target"
    );
    match &evidence.target {
        EvidenceReferenceTarget::File { path } => {
            assert_eq!(
                path,
                &project.path().join("workflow-os.yml").display().to_string()
            );
            assert!(!path.contains("workflowos.dev/v99"));
        }
        other => panic!("expected file target, got {other:?}"),
    }
    assert!(evidence.metadata.entries().is_empty());
}

#[test]
fn diagnostics_outside_schema_version_family_do_not_get_evidence() {
    let yaml_project = TestProject::new("yaml-no-evidence");
    yaml_project.write(
        "workflow-os.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
project:
  id: acme/approval
  name: [unterminated
"
        ),
    );

    let yaml_result = load_project(yaml_project.path());
    let yaml = yaml_result
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code() == "yaml.parse")
        .expect("yaml parse diagnostic");
    assert!(yaml.evidence_references().is_empty());

    let secret_project = TestProject::new("secret-no-evidence");
    secret_project.write(
        "workflow-os.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
project:
  id: acme/approval
  name: Acme Approval
layout:
  workflows: workflows
config_overlays:
  - id: local
    vars:
      - name: token
        value: secret:abc123
"
        ),
    );

    let secret_result = load_project(secret_project.path());
    let secret = secret_result
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code() == "spec.secret_disallowed")
        .expect("secret diagnostic");
    assert!(secret.evidence_references().is_empty());
}

#[test]
fn reports_duplicate_declared_ids() {
    let project = TestProject::new("duplicate-id");
    project.write_valid_project();
    project.write(
        "workflows/duplicate.workflow.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/request
version: v0
display_name: Duplicate Request Approval
"
        ),
    );

    let result = load_project(project.path());

    assert!(result.has_errors());
    let duplicate = result
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code() == "loader.duplicate_workflow_id")
        .expect("duplicate workflow diagnostic");
    assert_eq!(
        duplicate
            .source_location()
            .expect("source location")
            .document_path(),
        Some("$.id")
    );
}

#[test]
fn missing_required_directories_are_warnings() {
    let project = TestProject::new("missing-dirs");
    project.write_manifest();

    let result = load_project(project.path());

    assert!(result.bundle.is_some());
    assert!(!result.has_errors());
    let warning_count = result
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity() == DiagnosticSeverity::Warning)
        .count();
    assert_eq!(warning_count, 4);
}

#[test]
fn accumulates_multiple_errors_from_discovered_files() {
    let project = TestProject::new("multiple-errors");
    project.write_manifest();
    project.write(
        "workflows/bad.workflow.yml",
        r"
schema_version: workflowos.dev/v99
id: approval/bad
version: v0
display_name: Bad Workflow
",
    );
    project.write(
        "skills/bad.skill.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/bad
version: v0
display_name: Bad Skill
owner:
  lifecycle_status: preview
"
        ),
    );

    let result = load_project(project.path());

    assert!(result.bundle.is_some());
    assert!(result.has_errors());
    assert!(result
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code() == "schema_version.unsupported"));
    assert!(result
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code() == "spec.parse"));
}

#[test]
fn reports_secrets_in_forbidden_fields() {
    let project = TestProject::new("secret");
    project.write_manifest();
    project.write(
        "workflows/secret.workflow.yml",
        &format!(
            r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/secret
version: v0
display_name: Secret Workflow
steps:
  - id: draft
    skill_ref:
      id: local/draft-summary
    input_mapping:
      - from:
          type: literal
          value: secret:abc123
        to: request
"
        ),
    );

    let result = load_project(project.path());

    assert!(result.has_errors());
    let diagnostic = result
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code() == "spec.secret_disallowed")
        .expect("secret diagnostic");
    assert!(diagnostic.source_location().is_some());
}
