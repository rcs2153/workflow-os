#![allow(clippy::expect_used, clippy::panic)]
//! Behavior tests for v0 workflow and skill definition models.

use workflow_core::{
    canonical_yaml_content_hash, parse_skill_spec_yaml, parse_workflow_spec_yaml, AutonomyLevel,
    ContractExampleValue, LifecycleStatus, MappingExpression, TriggerKind,
    SUPPORTED_SCHEMA_VERSION,
};

fn representative_workflow_yaml() -> String {
    format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: enterprise/review-request
version: v0
display_name: Review Request
description: Generic approval workflow model.
owner:
  owning_team: platform-operations
  maintainer: platform-owner
  escalation_contact: platform-escalation
  lifecycle_status: experimental
autonomy_level: level_2
triggers:
  - id: manual-start
    kind: manual
    description: Local manual trigger.
    deduplication_key: request.id
state_model:
  type: inline
  states:
    - received
    - reviewed
steps:
  - id: draft-summary
    skill_ref:
      id: local/draft-summary
      version: v0
    input_mapping:
      - from:
          type: field
          path: request.description
        to: request_description
    output_mapping:
      - from:
          type: field
          path: summary
        to: review.summary
    policy_requirements:
      - id: approval/default
    idempotency_key_strategy:
      type: derived
    timeout:
      duration: 10m
    retry_policy:
      policy:
        id: retry/default
    escalation_policy:
      policy:
        id: escalation/default
    approval_policy:
      policy:
        id: approval/default
    terminal_behavior: escalate
branches:
  - id: needs-approval
    condition: review.risk == 'high'
    target_step: draft-summary
approval_requirements:
  - id: human-review
    reason: Sensitive or ambiguous enterprise work requires review.
    approver: platform-approver
    expires_after:
      duration: 1h
retry_policy_refs:
  - id: retry/default
escalation_policy_refs:
  - id: escalation/default
timeout_policy:
  max_duration:
    duration: 4h
  on_timeout: escalate
cancellation_behavior: stop
audit_requirements:
  required: true
  events:
    - RunCreated
    - ApprovalRequired
  store_references_only: true
observability_requirements:
  metrics:
    - workflow_latency
    - approval_wait_time
  tracing: true
  latency_tracking: true
tags:
  - approval
  - generic
"
    )
}

fn representative_skill_yaml() -> String {
    format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/draft-summary
version: v0
display_name: Draft Summary
description: Drafts a non-binding summary for human review.
owner:
  owning_team: platform-operations
  maintainer: platform-owner
  escalation_contact: platform-escalation
  lifecycle_status: stable
input_contract:
  fields:
    - name: request_description
      field_type: string
      description: Request details to summarize.
      sensitive: true
      redaction: summary_only
  required:
    - request_description
  examples:
    - name: redacted-input
      values:
        - kind: sensitive
          field: request_description
          value: internal request details
output_contract:
  fields:
    - name: summary
      field_type: string
      description: Draft summary.
  required:
    - summary
allowed_capabilities:
  - name: local_compute
    reason: Drafting uses local model or deterministic local implementation.
adapter_requirements:
  - adapter_id: local/model
    integration_id: local/default
    capabilities:
      - draft_text
failure_modes:
  - code: input_too_large
    description: Input exceeds configured limits.
    retryable: false
evaluation_criteria:
  - name: faithful_summary
    description: Summary must not invent facts.
retry_compatibility: requires_policy
approval_sensitivity: medium
audit_requirements:
  required: true
  events:
    - SkillInvocationPlanned
  store_references_only: true
observability_requirements:
  metrics:
    - skill_latency
  tracing: true
  latency_tracking: true
tags:
  - drafting
  - local
"
    )
}

#[test]
fn parses_representative_workflow_definition() {
    let workflow =
        parse_workflow_spec_yaml(&representative_workflow_yaml()).expect("workflow parses");

    assert_eq!(workflow.id.as_str(), "enterprise/review-request");
    assert_eq!(workflow.display_name, "Review Request");
    assert_eq!(
        workflow.autonomy_level,
        AutonomyLevel::Level2GuidedWithApproval
    );
    assert_eq!(
        workflow.owner.lifecycle_status,
        LifecycleStatus::Experimental
    );
    assert_eq!(workflow.triggers[0].kind, TriggerKind::Manual);
    assert_eq!(workflow.steps[0].id.as_str(), "draft-summary");
    assert_eq!(
        workflow.steps[0].skill_ref.id.as_str(),
        "local/draft-summary"
    );
    assert!(workflow.audit_requirements.required);
    assert!(workflow.spec_content_hash.is_some());
}

#[test]
fn parses_representative_skill_definition() {
    let skill = parse_skill_spec_yaml(&representative_skill_yaml()).expect("skill parses");

    assert_eq!(skill.id.as_str(), "local/draft-summary");
    assert_eq!(skill.display_name, "Draft Summary");
    assert_eq!(skill.owner.lifecycle_status, LifecycleStatus::Stable);
    assert!(skill.input_contract.fields[0].sensitive);
    assert_eq!(skill.allowed_capabilities[0].name, "local_compute");
    assert_eq!(
        skill.adapter_requirements[0].adapter_id.as_str(),
        "local/model"
    );
}

#[test]
fn rejects_unsupported_lifecycle_status() {
    let error = parse_skill_spec_yaml(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/draft-summary
version: v0
display_name: Draft Summary
owner:
  lifecycle_status: preview
"
    ))
    .expect_err("unsupported lifecycle status is rejected");

    assert_eq!(error.code(), "spec.parse");
}

#[test]
fn workflow_serializes_and_deserializes_round_trip() {
    let workflow =
        parse_workflow_spec_yaml(&representative_workflow_yaml()).expect("workflow parses");
    let serialized = serde_yaml::to_string(&workflow).expect("workflow serializes");
    let reparsed = serde_yaml::from_str(&serialized).expect("workflow deserializes");

    assert_eq!(workflow, reparsed);
}

#[test]
fn sensitive_example_values_are_redacted() {
    let skill = parse_skill_spec_yaml(&representative_skill_yaml()).expect("skill parses");
    let example_value = &skill.input_contract.examples[0].values[0];

    match example_value {
        ContractExampleValue::Sensitive { value, .. } => {
            assert_eq!(value.to_string(), "[REDACTED]");
            assert!(format!("{value:?}").contains("[REDACTED]"));
            assert!(!format!("{value:?}").contains("internal request details"));
            assert_eq!(
                serde_json::to_string(value).expect("redacted value serializes"),
                "\"[REDACTED]\""
            );
        }
        ContractExampleValue::Plain { .. } => panic!("expected sensitive example value"),
    }
}

#[test]
fn workflow_content_hash_is_stable_for_equivalent_content() {
    let first = canonical_yaml_content_hash(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: enterprise/review-request
version: v0
display_name: Review Request
"
    ))
    .expect("first hash computes");

    let second = canonical_yaml_content_hash(&format!(
        r"
display_name: Review Request
version: v0
id: enterprise/review-request
schema_version: {SUPPORTED_SCHEMA_VERSION}
"
    ))
    .expect("second hash computes");

    assert_eq!(first, second);
}

#[test]
fn autonomy_level_defaults_and_explicit_values_parse() {
    let defaulted = parse_workflow_spec_yaml(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: enterprise/default-autonomy
version: v0
display_name: Default Autonomy
"
    ))
    .expect("workflow parses");

    let explicit = parse_workflow_spec_yaml(&format!(
        r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: enterprise/explicit-autonomy
version: v0
display_name: Explicit Autonomy
autonomy_level: level_2
"
    ))
    .expect("workflow parses");

    assert_eq!(defaulted.autonomy_level, AutonomyLevel::Level1Assistive);
    assert_eq!(
        explicit.autonomy_level,
        AutonomyLevel::Level2GuidedWithApproval
    );
}

#[test]
fn step_reference_shape_is_strongly_typed() {
    let workflow =
        parse_workflow_spec_yaml(&representative_workflow_yaml()).expect("workflow parses");
    let step = &workflow.steps[0];

    assert_eq!(step.id.as_str(), "draft-summary");
    assert_eq!(step.skill_ref.id.as_str(), "local/draft-summary");
    assert_eq!(
        step.skill_ref
            .version
            .as_ref()
            .expect("skill version")
            .as_str(),
        "v0"
    );
    assert!(matches!(
        step.input_mapping[0].from,
        MappingExpression::Field { .. }
    ));
}
