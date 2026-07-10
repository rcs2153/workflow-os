#![allow(clippy::expect_used, clippy::panic)]
//! Behavior tests for v0 project/spec YAML parsing.

use workflow_core::{
    canonical_yaml_content_hash, parse_policy_spec_yaml, parse_project_manifest_yaml,
    parse_skill_spec_yaml, parse_test_spec_yaml, parse_workflow_spec_yaml,
    WorkReportArtifactApprovalProofMarkerGatePolicy,
    WorkReportArtifactApprovalProofMarkerRequirement,
    WorkReportArtifactHighAssuranceDisclosurePolicy, WorkReportArtifactHighAssuranceRequirement,
    WorkflowOsErrorKind, SUPPORTED_SCHEMA_VERSION,
};

#[test]
fn parses_valid_project_manifest() {
    let manifest = parse_project_manifest_yaml(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
project:
  id: acme/approval
  name: Acme Approval
  description: Local approval workflow examples.
layout:
  workflows: workflows
  skills: skills
  policies: policies
  tests: tests
config:
  - environment: dev
    vars:
      - name: approval_timeout
        value: 1h
"
    ))
    .expect("manifest parses");

    assert_eq!(manifest.project.id.as_str(), "acme/approval");
    assert_eq!(manifest.layout.workflows, "workflows");
    assert_eq!(manifest.config[0].vars[0].name, "approval_timeout");
}

#[test]
fn rejects_missing_schema_version() {
    let error = parse_project_manifest_yaml(
        r"
project:
  id: acme/approval
  name: Acme Approval
",
    )
    .expect_err("missing schema_version is rejected");

    assert_eq!(error.kind(), WorkflowOsErrorKind::Validation);
    assert_eq!(error.code(), "schema_version.missing");
}

#[test]
fn rejects_unsupported_schema_version() {
    let error = parse_project_manifest_yaml(
        r"
schema_version: workflowos.dev/v99
project:
  id: acme/approval
  name: Acme Approval
",
    )
    .expect_err("unsupported schema_version is rejected");

    assert_eq!(error.kind(), WorkflowOsErrorKind::Validation);
    assert_eq!(error.code(), "schema_version.unsupported");
}

#[test]
fn parses_workflow_file_shell() {
    let workflow = parse_workflow_spec_yaml(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/request-review
version: v0
name: Request Review
description: Human approval shell.
steps:
  - id: draft
    skill_ref:
      id: local/draft-summary
      version: v0
    input_mapping:
      - from:
          type: literal
          value: access-review
        to: request_kind
"
    ))
    .expect("workflow shell parses");

    assert_eq!(workflow.id.as_str(), "approval/request-review");
    assert_eq!(
        workflow.steps[0].skill_ref.id.as_str(),
        "local/draft-summary"
    );
    assert!(workflow.spec_content_hash.is_some());
}

#[test]
fn parses_workflow_report_artifact_requirement_shell() {
    let workflow = parse_workflow_spec_yaml(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/request-review
version: v0
name: Request Review
description: Human approval shell.
report_artifact_requirements:
  high_assurance_approval: validated_fail_closed_disclosure_required
  approval_proof_markers: marker_required
steps:
  - id: draft
    skill_ref:
      id: local/draft-summary
      version: v0
    input_mapping:
      - from:
          type: literal
          value: access-review
        to: request_kind
"
    ))
    .expect("workflow shell parses");

    assert_eq!(
        workflow
            .report_artifact_requirements
            .high_assurance_approval,
        WorkReportArtifactHighAssuranceRequirement::ValidatedFailClosedDisclosureRequired
    );
    assert_eq!(
        workflow
            .report_artifact_requirements
            .high_assurance_approval
            .to_high_assurance_disclosure_policy(),
        WorkReportArtifactHighAssuranceDisclosurePolicy::require_validated_fail_closed()
    );
    assert_eq!(
        workflow.report_artifact_requirements.approval_proof_markers,
        WorkReportArtifactApprovalProofMarkerRequirement::MarkerRequired
    );
    assert_eq!(
        workflow
            .report_artifact_requirements
            .approval_proof_markers
            .to_approval_proof_marker_gate_policy(),
        Some(WorkReportArtifactApprovalProofMarkerGatePolicy::require_present_markers())
    );
}

#[test]
fn rejects_unknown_workflow_report_artifact_requirement_field() {
    let error = parse_workflow_spec_yaml(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/request-review
version: v0
name: Request Review
report_artifact_requirements:
  high_assurance_approval: not_required
  future_gate: required
steps:
  - id: draft
    skill_ref:
      id: local/draft-summary
      version: v0
"
    ))
    .expect_err("unknown report artifact requirement field is rejected");

    assert_eq!(error.kind(), WorkflowOsErrorKind::Parse);
    assert_eq!(error.code(), "spec.parse");
}

#[test]
fn rejects_unknown_workflow_report_artifact_requirement_value() {
    let error = parse_workflow_spec_yaml(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/request-review
version: v0
name: Request Review
report_artifact_requirements:
  high_assurance_approval: future_required
steps:
  - id: draft
    skill_ref:
      id: local/draft-summary
      version: v0
"
    ))
    .expect_err("unknown report artifact requirement value is rejected");

    assert_eq!(error.kind(), WorkflowOsErrorKind::Parse);
    assert_eq!(error.code(), "spec.parse");
}

#[test]
fn rejects_unknown_workflow_report_artifact_proof_marker_requirement_value() {
    let error = parse_workflow_spec_yaml(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/request-review
version: v0
name: Request Review
report_artifact_requirements:
  approval_proof_markers: public_card_required
steps:
  - id: draft
    skill_ref:
      id: local/draft-summary
      version: v0
"
    ))
    .expect_err("unknown proof-marker report artifact requirement value is rejected");

    assert_eq!(error.kind(), WorkflowOsErrorKind::Parse);
    assert_eq!(error.code(), "spec.parse");
}

#[test]
fn parses_skill_file_shell() {
    let skill = parse_skill_spec_yaml(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/draft-summary
version: v0
name: Draft Summary
description: Local-only drafting skill shell.
input_contract:
  fields:
    - name: request_kind
      field_type: string
  required:
    - request_kind
"
    ))
    .expect("skill shell parses");

    assert_eq!(skill.id.as_str(), "local/draft-summary");
    assert_eq!(skill.version.as_str(), "v0");
}

#[test]
fn parses_policy_file_shell() {
    let policy = parse_policy_spec_yaml(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/default
name: Default Approval Policy
description: Requires human approval for ambiguous actions.
rules:
  - id: require-human
    effect: require_approval
    actor: system
"
    ))
    .expect("policy shell parses");

    assert_eq!(policy.id.as_str(), "approval/default");
    assert_eq!(
        policy.rules[0].actor.as_ref().expect("actor").as_str(),
        "system"
    );
}

#[test]
fn parses_test_file_shell() {
    let test = parse_test_spec_yaml(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/request-review-basic
name: Request review shell parses
target:
  id: approval/request-review
  version: v0
assertions:
  - id: requires-approval
    description: Workflow shell declares an approval-gated step.
"
    ))
    .expect("test shell parses");

    assert_eq!(test.id, "approval/request-review-basic");
    assert_eq!(test.target.id, "approval/request-review");
}

#[test]
fn rejects_secret_keys_in_specs() {
    let error = parse_workflow_spec_yaml(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: approval/request-review
version: v0
name: Request Review
steps:
  - id: draft
    skill_ref:
      id: local/draft-summary
    input_mapping:
      - from:
          type: literal
          value: not-allowed
        to: api_key
"
    ))
    .expect_err("secret-looking key is rejected");

    assert_eq!(error.kind(), WorkflowOsErrorKind::Security);
    assert_eq!(error.code(), "spec.secret_disallowed");
}

#[test]
fn rejects_secret_values_in_specs() {
    let error = parse_project_manifest_yaml(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
project:
  id: acme/approval
  name: Acme Approval
config:
  - environment: dev
    vars:
      - name: endpoint
        value: secret:abc123
"
    ))
    .expect_err("secret-looking value is rejected");

    assert_eq!(error.kind(), WorkflowOsErrorKind::Security);
    assert_eq!(error.code(), "spec.secret_disallowed");
}

#[test]
fn content_hash_is_stable_across_equivalent_mapping_order() {
    let first = canonical_yaml_content_hash(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
project:
  id: acme/approval
  name: Acme Approval
"
    ))
    .expect("first hash computes");

    let second = canonical_yaml_content_hash(&format!(
        r"
project:
  name: Acme Approval
  id: acme/approval
schema_version: {SUPPORTED_SCHEMA_VERSION}
"
    ))
    .expect("second hash computes");

    assert_eq!(first, second);
}
