#![allow(clippy::expect_used)]

//! High-assurance approval control core model tests.

use serde_json::json;
use workflow_core::{
    discover_high_assurance_approval_disclosure, validate_high_assurance_approval_decision,
    ActorId, ApprovalDecision, ApprovalDecisionKind, ApprovalRequest, CorrelationId, EventId,
    EvidenceReferenceId, HighAssuranceApprovalControl, HighAssuranceApprovalControlDefinition,
    HighAssuranceApprovalControlId, HighAssuranceApprovalControlVersion,
    HighAssuranceApprovalDecisionValidationInput, HighAssuranceApprovalDenialBehavior,
    HighAssuranceApprovalDisclosureDiscoveryInput,
    HighAssuranceApprovalDisclosureNotAvailableReason, HighAssuranceApprovalExpirationPolicy,
    HighAssuranceApprovalReportDisclosure, HighAssuranceApprovalRequiredReference,
    HighAssuranceApprovalRequiredReferenceTarget, HighAssuranceApprovalRevocationPolicy,
    HighAssuranceApprovalSuppliedReference, HighAssuranceProtectedActionKind,
    HighAssuranceRequesterApproverRule, LocalCheckResultId, RedactionDisposition,
    RedactionFieldState, RedactionMetadata, SchemaVersion, SideEffectId, SkillId, SkillVersion,
    SpecContentHash, StepId, Timestamp, ValidationReferenceId,
    WorkReportHighAssuranceApprovalDecision, WorkReportHighAssuranceExpirationPosture,
    WorkReportHighAssuranceRequesterApproverPosture, WorkReportHighAssuranceRevocationPosture,
    WorkReportId, WorkReportRedactionPolicy, WorkReportSensitivity, WorkReportStableReference,
    WorkflowId, WorkflowRunId, WorkflowVersion,
};

fn control_id() -> HighAssuranceApprovalControlId {
    HighAssuranceApprovalControlId::new("approval-control/nuclear-key").expect("valid control id")
}

fn control_version() -> HighAssuranceApprovalControlVersion {
    HighAssuranceApprovalControlVersion::new("v1").expect("valid control version")
}

fn schema_version() -> SchemaVersion {
    SchemaVersion::new("workflowos/v0").expect("valid schema version")
}

fn redaction() -> RedactionMetadata {
    RedactionMetadata {
        redacted_fields: vec!["approval_context".to_owned()],
        field_states: vec![RedactionFieldState {
            field: "approval_context".to_owned(),
            disposition: RedactionDisposition::ReferenceOnly,
            reason: "stores bounded approval references".to_owned(),
        }],
    }
}

fn evidence_reference() -> HighAssuranceApprovalRequiredReference {
    HighAssuranceApprovalRequiredReference::new(
        "evidence_reference",
        HighAssuranceApprovalRequiredReferenceTarget::EvidenceReference {
            evidence_reference_id: EvidenceReferenceId::new("evidence/context")
                .expect("valid evidence reference id"),
        },
        true,
    )
    .expect("valid evidence requirement")
}

fn validation_reference() -> HighAssuranceApprovalRequiredReference {
    HighAssuranceApprovalRequiredReference::new(
        "validation_reference",
        HighAssuranceApprovalRequiredReferenceTarget::ValidationReference {
            validation_reference_id: ValidationReferenceId::new("validation/spec")
                .expect("valid validation reference id"),
        },
        true,
    )
    .expect("valid validation requirement")
}

fn valid_definition() -> HighAssuranceApprovalControlDefinition {
    HighAssuranceApprovalControlDefinition {
        control_id: control_id(),
        control_version: control_version(),
        schema_version: schema_version(),
        protected_actions: vec![
            HighAssuranceProtectedActionKind::AdapterWrite,
            HighAssuranceProtectedActionKind::SideEffectAttempt,
        ],
        requester_approver_rule: HighAssuranceRequesterApproverRule::MustDiffer,
        minimum_approvals: 1,
        required_references: vec![evidence_reference(), validation_reference()],
        expiration_policy: HighAssuranceApprovalExpirationPolicy::MustBeUnexpiredAtUse,
        revocation_policy: HighAssuranceApprovalRevocationPolicy::ExplicitEventBeforeUse,
        denial_behavior: HighAssuranceApprovalDenialBehavior::FailClosed,
        report_disclosures: vec![
            HighAssuranceApprovalReportDisclosure::Requested,
            HighAssuranceApprovalReportDisclosure::Granted,
            HighAssuranceApprovalReportDisclosure::Denied,
            HighAssuranceApprovalReportDisclosure::EvidenceConsidered,
            HighAssuranceApprovalReportDisclosure::SideEffectsAuthorized,
        ],
        sensitivity: WorkReportSensitivity::Confidential,
        redaction_policy: WorkReportRedactionPolicy::ReferenceOnly,
        redaction: redaction(),
    }
}

fn valid_control() -> HighAssuranceApprovalControl {
    HighAssuranceApprovalControl::new(valid_definition()).expect("valid control")
}

fn enforceable_definition() -> HighAssuranceApprovalControlDefinition {
    let mut definition = valid_definition();
    definition.expiration_policy = HighAssuranceApprovalExpirationPolicy::NotRequired;
    definition.revocation_policy = HighAssuranceApprovalRevocationPolicy::Unsupported;
    definition.denial_behavior = HighAssuranceApprovalDenialBehavior::FailClosed;
    definition
}

fn enforceable_control() -> HighAssuranceApprovalControl {
    HighAssuranceApprovalControl::new(enforceable_definition()).expect("valid enforceable control")
}

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).expect("valid timestamp")
}

fn approval_request(requested_by: &str) -> ApprovalRequest {
    ApprovalRequest {
        approval_id: "approval/high-assurance".to_owned(),
        run_id: WorkflowRunId::new("run/high-assurance").expect("valid run id"),
        workflow_id: WorkflowId::new("workflow/high-assurance").expect("valid workflow id"),
        schema_version: schema_version(),
        workflow_version: WorkflowVersion::new("v1").expect("valid workflow version"),
        spec_content_hash: SpecContentHash::from_text("workflow spec"),
        resolved_execution_context_hash: None,
        step_id: StepId::new("step/protected").expect("valid step id"),
        skill_id: SkillId::new("skill/protected").expect("valid skill id"),
        skill_version: SkillVersion::new("v1").expect("valid skill version"),
        requested_by: ActorId::new(requested_by).expect("valid requester"),
        correlation_id: CorrelationId::new("correlation/high-assurance")
            .expect("valid correlation"),
        idempotency_key: None,
        reason: "bounded approval request".to_owned(),
        requested_at: timestamp("2026-06-20T12:00:00Z"),
        expires_after: Some("30m".to_owned()),
        expires_at: Some(timestamp("2026-06-20T12:30:00Z")),
        decision: None,
    }
}

fn approval_decision(actor: &str) -> ApprovalDecision {
    ApprovalDecision {
        approval_id: "approval/high-assurance".to_owned(),
        actor: ActorId::new(actor).expect("valid approver"),
        decided_at: timestamp("2026-06-20T12:05:00Z"),
        decision: ApprovalDecisionKind::Granted,
        reason: "bounded approval decision".to_owned(),
        correlation_id: CorrelationId::new("correlation/high-assurance")
            .expect("valid correlation"),
        proof_marker: None,
    }
}

fn supplied_references() -> Vec<HighAssuranceApprovalSuppliedReference> {
    vec![
        HighAssuranceApprovalSuppliedReference::new(
            "evidence_reference",
            evidence_reference().target().clone(),
        )
        .expect("valid supplied evidence reference"),
        HighAssuranceApprovalSuppliedReference::new(
            "validation_reference",
            validation_reference().target().clone(),
        )
        .expect("valid supplied validation reference"),
    ]
}

fn validate_decision_with(
    request: &ApprovalRequest,
    decision: &ApprovalDecision,
    control: &HighAssuranceApprovalControl,
    references: &[HighAssuranceApprovalSuppliedReference],
) -> Result<
    workflow_core::HighAssuranceApprovalDecisionValidationResult,
    workflow_core::WorkflowOsError,
> {
    validate_high_assurance_approval_decision(&HighAssuranceApprovalDecisionValidationInput {
        approval_request: request,
        approval_decision: decision,
        controls: std::slice::from_ref(control),
        supplied_references: references,
        current_time: timestamp("2026-06-20T12:05:00Z"),
    })
}

fn disclosure_discovery_input() -> HighAssuranceApprovalDisclosureDiscoveryInput {
    HighAssuranceApprovalDisclosureDiscoveryInput {
        validation_used: true,
        validation_passed: true,
        decision: ApprovalDecisionKind::Granted,
        control_count: 1,
        requester_approver_rule: HighAssuranceRequesterApproverRule::MustDiffer,
        required_reference_count: 2,
        supplied_reference_count: 2,
        expiration_policy: HighAssuranceApprovalExpirationPolicy::NotRequired,
        revocation_policy: HighAssuranceApprovalRevocationPolicy::Unsupported,
        denial_behavior: HighAssuranceApprovalDenialBehavior::FailClosed,
    }
}

#[test]
fn valid_minimal_high_assurance_approval_control() {
    let control = valid_control();

    assert_eq!(
        control.control_id().as_str(),
        "approval-control/nuclear-key"
    );
    assert_eq!(control.control_version().as_str(), "v1");
    assert_eq!(control.schema_version().as_str(), "workflowos/v0");
    assert_eq!(control.protected_actions().len(), 2);
    assert_eq!(
        control.requester_approver_rule(),
        HighAssuranceRequesterApproverRule::MustDiffer
    );
    assert_eq!(control.minimum_approvals(), 1);
    assert_eq!(control.required_references().len(), 2);
    assert_eq!(
        control.expiration_policy(),
        HighAssuranceApprovalExpirationPolicy::MustBeUnexpiredAtUse
    );
    assert_eq!(
        control.revocation_policy(),
        HighAssuranceApprovalRevocationPolicy::ExplicitEventBeforeUse
    );
    assert_eq!(
        control.denial_behavior(),
        HighAssuranceApprovalDenialBehavior::FailClosed
    );
    assert_eq!(control.report_disclosures().len(), 5);
    assert_eq!(control.sensitivity(), WorkReportSensitivity::Confidential);
    assert_eq!(
        control.redaction_policy(),
        WorkReportRedactionPolicy::ReferenceOnly
    );
    assert_eq!(control.redaction().redacted_fields.len(), 1);
}

#[test]
fn disclosure_discovery_maps_successful_grant_to_work_report_disclosure() {
    let result = discover_high_assurance_approval_disclosure(disclosure_discovery_input())
        .expect("disclosure discovered");
    let disclosure = result.disclosure().expect("disclosure present");

    assert_eq!(
        disclosure.decision(),
        WorkReportHighAssuranceApprovalDecision::Granted
    );
    assert!(disclosure.validation_used());
    assert!(disclosure.validation_passed());
    assert_eq!(
        disclosure.requester_approver_posture(),
        WorkReportHighAssuranceRequesterApproverPosture::MustDifferValidated
    );
    assert_eq!(
        disclosure.expiration_posture(),
        WorkReportHighAssuranceExpirationPosture::NotRequired
    );
    assert_eq!(
        disclosure.revocation_posture(),
        WorkReportHighAssuranceRevocationPosture::Unsupported
    );
    assert_eq!(disclosure.required_reference_count(), 2);
    assert_eq!(disclosure.supplied_reference_count(), 2);
    assert!(disclosure.denial_fail_closed());
    assert_eq!(result.not_available_reason(), None);
}

#[test]
fn disclosure_discovery_maps_denial_and_deferred_human_posture() {
    let mut input = disclosure_discovery_input();
    input.decision = ApprovalDecisionKind::Denied;
    input.requester_approver_rule = HighAssuranceRequesterApproverRule::HumanApproverMustDiffer;
    input.expiration_policy = HighAssuranceApprovalExpirationPolicy::MustBeUnexpiredAtDecision;

    let result = discover_high_assurance_approval_disclosure(input).expect("disclosure discovered");
    let disclosure = result.disclosure().expect("disclosure present");

    assert_eq!(
        disclosure.decision(),
        WorkReportHighAssuranceApprovalDecision::Denied
    );
    assert_eq!(
        disclosure.requester_approver_posture(),
        WorkReportHighAssuranceRequesterApproverPosture::HumanApproverDeferred
    );
    assert_eq!(
        disclosure.expiration_posture(),
        WorkReportHighAssuranceExpirationPosture::UnexpiredAtDecision
    );
}

#[test]
fn disclosure_discovery_returns_not_available_when_validation_not_used() {
    let mut input = disclosure_discovery_input();
    input.validation_used = false;
    input.validation_passed = false;

    let result =
        discover_high_assurance_approval_disclosure(input).expect("not available is valid");

    assert!(result.disclosure().is_none());
    assert_eq!(
        result.not_available_reason(),
        Some(HighAssuranceApprovalDisclosureNotAvailableReason::ValidationNotUsed)
    );
}

#[test]
fn disclosure_discovery_can_represent_failed_validation_without_decision_claims() {
    let mut input = disclosure_discovery_input();
    input.validation_passed = false;

    let result = discover_high_assurance_approval_disclosure(input)
        .expect("failed validation disclosure discovered");
    let disclosure = result.disclosure().expect("disclosure present");

    assert!(disclosure.validation_used());
    assert!(!disclosure.validation_passed());
    assert_eq!(
        disclosure.decision(),
        WorkReportHighAssuranceApprovalDecision::NotAvailable
    );
    assert_eq!(
        disclosure.requester_approver_posture(),
        WorkReportHighAssuranceRequesterApproverPosture::NotAvailable
    );
    assert_eq!(
        disclosure.expiration_posture(),
        WorkReportHighAssuranceExpirationPosture::NotAvailable
    );
    assert_eq!(
        disclosure.revocation_posture(),
        WorkReportHighAssuranceRevocationPosture::NotAvailable
    );
    assert!(!disclosure.denial_fail_closed());
}

#[test]
fn disclosure_discovery_rejects_inconsistent_validation_without_leaking() {
    let mut input = disclosure_discovery_input();
    input.validation_used = false;
    input.validation_passed = true;

    let error = discover_high_assurance_approval_disclosure(input)
        .expect_err("inconsistent discovery rejected");

    assert_eq!(
        error.code(),
        "high_assurance_approval.disclosure_discovery.validation_inconsistent"
    );
    assert!(!error.to_string().contains("approval/high-assurance"));
}

#[test]
fn disclosure_discovery_rejects_unsupported_passed_posture() {
    let mut input = disclosure_discovery_input();
    input.expiration_policy = HighAssuranceApprovalExpirationPolicy::MustBeUnexpiredAtUse;

    let error = discover_high_assurance_approval_disclosure(input)
        .expect_err("unsupported expiration rejected");

    assert_eq!(
        error.code(),
        "high_assurance_approval.disclosure_discovery.expiration.unsupported"
    );

    let mut input = disclosure_discovery_input();
    input.revocation_policy = HighAssuranceApprovalRevocationPolicy::ExplicitEventBeforeUse;
    let error = discover_high_assurance_approval_disclosure(input)
        .expect_err("unsupported revocation rejected");
    assert_eq!(
        error.code(),
        "high_assurance_approval.disclosure_discovery.revocation.unsupported"
    );

    let mut input = disclosure_discovery_input();
    input.denial_behavior = HighAssuranceApprovalDenialBehavior::Escalate;
    let error = discover_high_assurance_approval_disclosure(input)
        .expect_err("unsupported denial behavior rejected");
    assert_eq!(
        error.code(),
        "high_assurance_approval.disclosure_discovery.denial_behavior.unsupported"
    );
}

#[test]
fn disclosure_discovery_rejects_unbounded_counts() {
    let mut input = disclosure_discovery_input();
    input.required_reference_count = 1_025;

    let error =
        discover_high_assurance_approval_disclosure(input).expect_err("unbounded count rejected");

    assert_eq!(
        error.code(),
        "high_assurance_approval.disclosure_discovery.count_unbounded"
    );
    assert!(!error.to_string().contains("1025"));
}

#[test]
fn disclosure_discovery_debug_output_is_bounded_and_non_leaking() {
    let input = disclosure_discovery_input();
    let input_debug = format!("{input:?}");
    assert!(input_debug.contains("control_count"));
    assert!(!input_debug.contains("approval/high-assurance"));
    assert!(!input_debug.contains("user/requester"));

    let result = discover_high_assurance_approval_disclosure(input).expect("disclosure discovered");
    let result_debug = format!("{result:?}");
    assert!(result_debug.contains("has_disclosure"));
    assert!(!result_debug.contains("approval/high-assurance"));
    assert!(!result_debug.contains("evidence/context"));
}

#[test]
fn decision_validation_accepts_different_requester_and_supplied_references() {
    let request = approval_request("user/requester");
    let decision = approval_decision("user/approver");
    let control = enforceable_control();
    let references = supplied_references();

    let result =
        validate_decision_with(&request, &decision, &control, &references).expect("valid decision");

    assert_eq!(result.control_count(), 1);
    assert_eq!(result.supplied_reference_count(), 2);
}

#[test]
fn decision_validation_allows_same_actor_when_control_allows_it() {
    let request = approval_request("user/operator");
    let decision = approval_decision("user/operator");
    let mut definition = enforceable_definition();
    definition.requester_approver_rule = HighAssuranceRequesterApproverRule::SameActorAllowed;
    let control = HighAssuranceApprovalControl::new(definition).expect("same actor control");
    let references = supplied_references();

    validate_decision_with(&request, &decision, &control, &references).expect("same actor allowed");
}

#[test]
fn decision_validation_rejects_same_actor_without_leaking_actor_id() {
    let request = approval_request("user/requester-secret");
    let decision = approval_decision("user/requester-secret");
    let control = enforceable_control();
    let references = supplied_references();

    let error = validate_decision_with(&request, &decision, &control, &references)
        .expect_err("same actor rejected");

    assert_eq!(
        error.code(),
        "high_assurance_approval.enforcement.requester_approver.same_actor"
    );
    assert!(!error.to_string().contains("requester-secret"));
}

#[test]
fn decision_validation_rejects_missing_required_reference_without_leaking_name() {
    let request = approval_request("user/requester");
    let decision = approval_decision("user/approver");
    let control = enforceable_control();
    let references = vec![HighAssuranceApprovalSuppliedReference::new(
        "evidence_reference",
        evidence_reference().target().clone(),
    )
    .expect("valid supplied reference")];

    let error = validate_decision_with(&request, &decision, &control, &references)
        .expect_err("missing required reference");

    assert_eq!(
        error.code(),
        "high_assurance_approval.enforcement.reference.missing"
    );
    assert!(!error.to_string().contains("validation_reference"));
}

#[test]
fn decision_validation_rejects_reference_target_mismatch() {
    let request = approval_request("user/requester");
    let decision = approval_decision("user/approver");
    let control = enforceable_control();
    let references = vec![
        HighAssuranceApprovalSuppliedReference::new(
            "evidence_reference",
            evidence_reference().target().clone(),
        )
        .expect("valid supplied evidence reference"),
        HighAssuranceApprovalSuppliedReference::new(
            "validation_reference",
            HighAssuranceApprovalRequiredReferenceTarget::ValidationReference {
                validation_reference_id: ValidationReferenceId::new("validation/different")
                    .expect("valid validation reference id"),
            },
        )
        .expect("valid supplied validation reference"),
    ];

    let error = validate_decision_with(&request, &decision, &control, &references)
        .expect_err("target mismatch");

    assert_eq!(
        error.code(),
        "high_assurance_approval.enforcement.reference.target_mismatch"
    );
    assert!(!error.to_string().contains("validation/different"));
}

#[test]
fn decision_validation_rejects_duplicate_supplied_reference_names() {
    let request = approval_request("user/requester");
    let decision = approval_decision("user/approver");
    let control = enforceable_control();
    let references = vec![
        HighAssuranceApprovalSuppliedReference::new(
            "evidence_reference",
            evidence_reference().target().clone(),
        )
        .expect("valid supplied evidence reference"),
        HighAssuranceApprovalSuppliedReference::new(
            "evidence_reference",
            evidence_reference().target().clone(),
        )
        .expect("valid supplied evidence reference"),
    ];

    let error = validate_decision_with(&request, &decision, &control, &references)
        .expect_err("duplicate reference");

    assert_eq!(
        error.code(),
        "high_assurance_approval.enforcement.reference.duplicate"
    );
}

#[test]
fn decision_validation_rejects_unsupported_minimum_approvals() {
    let request = approval_request("user/requester");
    let decision = approval_decision("user/approver");
    let mut definition = enforceable_definition();
    definition.minimum_approvals = 2;
    let control = HighAssuranceApprovalControl::new(definition).expect("valid control vocabulary");
    let references = supplied_references();

    let error = validate_decision_with(&request, &decision, &control, &references)
        .expect_err("multi-approval unsupported");

    assert_eq!(
        error.code(),
        "high_assurance_approval.enforcement.minimum_approvals.unsupported"
    );
}

#[test]
fn decision_validation_rejects_missing_expiration_metadata_when_required() {
    let mut request = approval_request("user/requester");
    request.expires_after = None;
    request.expires_at = None;
    let decision = approval_decision("user/approver");
    let mut definition = enforceable_definition();
    definition.expiration_policy = HighAssuranceApprovalExpirationPolicy::RequiredOnRequest;
    let control = HighAssuranceApprovalControl::new(definition).expect("valid control vocabulary");
    let references = supplied_references();

    let error = validate_decision_with(&request, &decision, &control, &references)
        .expect_err("expiration required");

    assert_eq!(
        error.code(),
        "high_assurance_approval.enforcement.expiration.required"
    );
}

#[test]
fn decision_validation_rejects_expired_decision_time_request() {
    let mut request = approval_request("user/requester");
    request.expires_at = Some(timestamp("2026-06-20T12:01:00Z"));
    let decision = approval_decision("user/approver");
    let mut definition = enforceable_definition();
    definition.expiration_policy = HighAssuranceApprovalExpirationPolicy::MustBeUnexpiredAtDecision;
    let control = HighAssuranceApprovalControl::new(definition).expect("valid control vocabulary");
    let references = supplied_references();

    let error = validate_decision_with(&request, &decision, &control, &references)
        .expect_err("expired request");

    assert_eq!(
        error.code(),
        "high_assurance_approval.enforcement.expiration.expired"
    );
}

#[test]
fn decision_validation_rejects_use_time_expiration_as_unsupported() {
    let request = approval_request("user/requester");
    let decision = approval_decision("user/approver");
    let mut definition = enforceable_definition();
    definition.expiration_policy = HighAssuranceApprovalExpirationPolicy::MustBeUnexpiredAtUse;
    let control = HighAssuranceApprovalControl::new(definition).expect("valid control vocabulary");
    let references = supplied_references();

    let error = validate_decision_with(&request, &decision, &control, &references)
        .expect_err("use-time expiration unsupported");

    assert_eq!(
        error.code(),
        "high_assurance_approval.enforcement.expiration.unsupported"
    );
}

#[test]
fn decision_validation_rejects_revocation_policy_as_unsupported() {
    let request = approval_request("user/requester");
    let decision = approval_decision("user/approver");
    let mut definition = enforceable_definition();
    definition.revocation_policy = HighAssuranceApprovalRevocationPolicy::ExplicitEventBeforeUse;
    let control = HighAssuranceApprovalControl::new(definition).expect("valid control vocabulary");
    let references = supplied_references();

    let error = validate_decision_with(&request, &decision, &control, &references)
        .expect_err("revocation unsupported");

    assert_eq!(
        error.code(),
        "high_assurance_approval.enforcement.revocation.unsupported"
    );
}

#[test]
fn decision_validation_rejects_denial_behavior_as_unsupported() {
    let request = approval_request("user/requester");
    let decision = approval_decision("user/approver");
    let mut definition = enforceable_definition();
    definition.denial_behavior = HighAssuranceApprovalDenialBehavior::Escalate;
    let control = HighAssuranceApprovalControl::new(definition).expect("valid control vocabulary");
    let references = supplied_references();

    let error = validate_decision_with(&request, &decision, &control, &references)
        .expect_err("denial behavior unsupported");

    assert_eq!(
        error.code(),
        "high_assurance_approval.enforcement.denial_behavior.unsupported"
    );
}

#[test]
fn decision_validation_rejects_approval_id_mismatch_without_leaking_ids() {
    let request = approval_request("user/requester");
    let mut decision = approval_decision("user/approver");
    decision.approval_id = "approval/other-secret".to_owned();
    let control = enforceable_control();
    let references = supplied_references();

    let error = validate_decision_with(&request, &decision, &control, &references)
        .expect_err("approval id mismatch");

    assert_eq!(
        error.code(),
        "high_assurance_approval.enforcement.approval_id_mismatch"
    );
    assert!(!error.to_string().contains("other-secret"));
}

#[test]
fn decision_validation_debug_output_does_not_leak_actor_or_reference_values() {
    let request = approval_request("user/requester-secret");
    let decision = approval_decision("user/approver-secret");
    let control = enforceable_control();
    let references = supplied_references();
    let input = HighAssuranceApprovalDecisionValidationInput {
        approval_request: &request,
        approval_decision: &decision,
        controls: std::slice::from_ref(&control),
        supplied_references: &references,
        current_time: timestamp("2026-06-20T12:05:00Z"),
    };

    let debug = format!("{input:?}");

    assert!(!debug.contains("requester-secret"));
    assert!(!debug.contains("approver-secret"));
    assert!(!debug.contains("evidence/context"));
    assert!(!debug.contains("validation/spec"));
}

#[test]
fn invalid_control_id_rejected_without_leaking_value() {
    let error = HighAssuranceApprovalControlId::new("bad id").expect_err("invalid id");

    assert_eq!(
        error.code(),
        "high_assurance_approval.identifier.invalid_character"
    );
    assert!(!error.to_string().contains("bad id"));
}

#[test]
fn invalid_version_rejected_without_leaking_value() {
    let error = HighAssuranceApprovalControlVersion::new("").expect_err("invalid version");

    assert_eq!(error.code(), "high_assurance_approval.identifier.empty");
}

#[test]
fn invalid_schema_version_secret_like_value_is_rejected() {
    let mut definition = valid_definition();
    definition.schema_version =
        SchemaVersion::new("workflowos/token").expect("schema id primitive allows token text");

    let error =
        HighAssuranceApprovalControl::new(definition).expect_err("secret-like schema version");

    assert_eq!(error.code(), "high_assurance_approval.secret_like_value");
    assert!(!error.to_string().contains("workflowos/token"));
}

#[test]
fn empty_protected_actions_rejected() {
    let mut definition = valid_definition();
    definition.protected_actions.clear();

    let error = HighAssuranceApprovalControl::new(definition).expect_err("empty protected actions");

    assert_eq!(
        error.code(),
        "high_assurance_approval.protected_actions.required"
    );
}

#[test]
fn duplicate_protected_actions_rejected() {
    let mut definition = valid_definition();
    definition.protected_actions = vec![
        HighAssuranceProtectedActionKind::AdapterWrite,
        HighAssuranceProtectedActionKind::AdapterWrite,
    ];

    let error =
        HighAssuranceApprovalControl::new(definition).expect_err("duplicate protected actions");

    assert_eq!(
        error.code(),
        "high_assurance_approval.protected_actions.duplicate"
    );
}

#[test]
fn minimum_approval_count_must_be_nonzero() {
    let mut definition = valid_definition();
    definition.minimum_approvals = 0;

    let error = HighAssuranceApprovalControl::new(definition).expect_err("zero approvals");

    assert_eq!(
        error.code(),
        "high_assurance_approval.minimum_approvals.invalid"
    );
}

#[test]
fn empty_required_references_rejected() {
    let mut definition = valid_definition();
    definition.required_references.clear();

    let error =
        HighAssuranceApprovalControl::new(definition).expect_err("empty required references");

    assert_eq!(error.code(), "high_assurance_approval.references.required");
}

#[test]
fn duplicate_required_reference_names_rejected() {
    let mut definition = valid_definition();
    definition.required_references = vec![evidence_reference(), evidence_reference()];

    let error =
        HighAssuranceApprovalControl::new(definition).expect_err("duplicate reference names");

    assert_eq!(error.code(), "high_assurance_approval.references.duplicate");
}

#[test]
fn required_reference_target_vocabulary_is_representable() {
    let targets = vec![
        HighAssuranceApprovalRequiredReferenceTarget::EvidenceReference {
            evidence_reference_id: EvidenceReferenceId::new("evidence/context")
                .expect("valid evidence reference"),
        },
        HighAssuranceApprovalRequiredReferenceTarget::PolicyDecision {
            event_id: EventId::new("event/policy-decision").expect("valid event id"),
        },
        HighAssuranceApprovalRequiredReferenceTarget::SideEffect {
            side_effect_id: SideEffectId::new("side-effect/proposed").expect("valid side effect"),
        },
        HighAssuranceApprovalRequiredReferenceTarget::ValidationReference {
            validation_reference_id: ValidationReferenceId::new("validation/spec")
                .expect("valid validation reference"),
        },
        HighAssuranceApprovalRequiredReferenceTarget::LocalCheckResult {
            local_check_result_id: LocalCheckResultId::new("local-check/docs")
                .expect("valid local check result"),
        },
        HighAssuranceApprovalRequiredReferenceTarget::WorkflowEvent {
            event_id: EventId::new("event/workflow").expect("valid workflow event id"),
        },
        HighAssuranceApprovalRequiredReferenceTarget::AuditEvent {
            event_id: EventId::new("event/audit").expect("valid audit event id"),
        },
        HighAssuranceApprovalRequiredReferenceTarget::WorkReport {
            work_report_id: WorkReportId::new("work-report/terminal").expect("valid report id"),
        },
        HighAssuranceApprovalRequiredReferenceTarget::AdapterTelemetry {
            reference: WorkReportStableReference::new("adapter-telemetry/read-context")
                .expect("valid stable reference"),
        },
    ];

    let names: Vec<_> = targets
        .iter()
        .map(HighAssuranceApprovalRequiredReferenceTarget::kind_name)
        .collect();
    assert_eq!(
        names,
        vec![
            "evidence_reference",
            "policy_decision",
            "side_effect",
            "validation_reference",
            "local_check_result",
            "workflow_event",
            "audit_event",
            "work_report",
            "adapter_telemetry",
        ]
    );
}

#[test]
fn required_reference_serde_round_trip_uses_validated_shape() {
    let reference = evidence_reference();
    let serialized = serde_json::to_string(&reference).expect("serialize reference");
    let deserialized: HighAssuranceApprovalRequiredReference =
        serde_json::from_str(&serialized).expect("deserialize reference");

    assert_eq!(deserialized, reference);
    assert_eq!(deserialized.name(), "evidence_reference");
    assert!(deserialized.required());
}

#[test]
fn invalid_serialized_required_reference_name_fails_closed_without_leaking() {
    let value = json!({
        "name": "bad name",
        "target": {
            "kind": "evidence_reference",
            "evidence_reference_id": "evidence/context"
        },
        "required": true
    });

    let error = serde_json::from_value::<HighAssuranceApprovalRequiredReference>(value)
        .expect_err("invalid reference name rejected");

    assert!(error.to_string().contains("invalid character"));
    assert!(!error.to_string().contains("bad name"));
}

#[test]
fn secret_like_serialized_required_reference_name_fails_closed_without_leaking() {
    let value = json!({
        "name": "authorization_token",
        "target": {
            "kind": "evidence_reference",
            "evidence_reference_id": "evidence/context"
        },
        "required": true
    });

    let error = serde_json::from_value::<HighAssuranceApprovalRequiredReference>(value)
        .expect_err("secret-like reference name rejected");

    assert!(error.to_string().contains("sensitive-looking text"));
    assert!(!error.to_string().contains("authorization_token"));
}

#[test]
fn protected_action_vocabulary_includes_future_write_terms_without_runtime_execution() {
    let mut definition = valid_definition();
    definition.protected_actions = vec![
        HighAssuranceProtectedActionKind::AdapterWrite,
        HighAssuranceProtectedActionKind::SideEffectAttempt,
        HighAssuranceProtectedActionKind::ReportArtifactWrite,
    ];

    let control =
        HighAssuranceApprovalControl::new(definition).expect("future vocabulary is model-only");

    assert_eq!(control.protected_actions().len(), 3);
}

#[test]
fn requester_approver_separation_rules_are_representable() {
    for requester_approver_rule in [
        HighAssuranceRequesterApproverRule::SameActorAllowed,
        HighAssuranceRequesterApproverRule::MustDiffer,
        HighAssuranceRequesterApproverRule::HumanApproverMustDiffer,
    ] {
        let mut definition = valid_definition();
        definition.requester_approver_rule = requester_approver_rule;

        let control = HighAssuranceApprovalControl::new(definition)
            .expect("requester approver rule representable");

        assert_eq!(control.requester_approver_rule(), requester_approver_rule);
    }
}

#[test]
fn expiration_and_revocation_policies_are_representable() {
    for expiration_policy in [
        HighAssuranceApprovalExpirationPolicy::NotRequired,
        HighAssuranceApprovalExpirationPolicy::RequiredOnRequest,
        HighAssuranceApprovalExpirationPolicy::MustBeUnexpiredAtDecision,
        HighAssuranceApprovalExpirationPolicy::MustBeUnexpiredAtUse,
    ] {
        let mut definition = valid_definition();
        definition.expiration_policy = expiration_policy;

        let control =
            HighAssuranceApprovalControl::new(definition).expect("expiration policy valid");

        assert_eq!(control.expiration_policy(), expiration_policy);
    }

    for revocation_policy in [
        HighAssuranceApprovalRevocationPolicy::Unsupported,
        HighAssuranceApprovalRevocationPolicy::ExplicitEventBeforeUse,
        HighAssuranceApprovalRevocationPolicy::ReportOnlyAfterUse,
    ] {
        let mut definition = valid_definition();
        definition.revocation_policy = revocation_policy;

        let control =
            HighAssuranceApprovalControl::new(definition).expect("revocation policy valid");

        assert_eq!(control.revocation_policy(), revocation_policy);
    }
}

#[test]
fn report_disclosures_require_nonempty_unique_values() {
    let mut definition = valid_definition();
    definition.report_disclosures.clear();

    let error =
        HighAssuranceApprovalControl::new(definition).expect_err("empty disclosures rejected");

    assert_eq!(
        error.code(),
        "high_assurance_approval.report_disclosures.required"
    );

    let mut definition = valid_definition();
    definition.report_disclosures = vec![
        HighAssuranceApprovalReportDisclosure::Requested,
        HighAssuranceApprovalReportDisclosure::Requested,
    ];

    let error =
        HighAssuranceApprovalControl::new(definition).expect_err("duplicate disclosures rejected");

    assert_eq!(
        error.code(),
        "high_assurance_approval.report_disclosures.duplicate"
    );
}

#[test]
fn all_report_disclosures_are_representable() {
    let mut definition = valid_definition();
    definition.report_disclosures = vec![
        HighAssuranceApprovalReportDisclosure::Requested,
        HighAssuranceApprovalReportDisclosure::Granted,
        HighAssuranceApprovalReportDisclosure::Denied,
        HighAssuranceApprovalReportDisclosure::Expired,
        HighAssuranceApprovalReportDisclosure::Revoked,
        HighAssuranceApprovalReportDisclosure::Skipped,
        HighAssuranceApprovalReportDisclosure::Deferred,
        HighAssuranceApprovalReportDisclosure::EvidenceConsidered,
        HighAssuranceApprovalReportDisclosure::SideEffectsAuthorized,
    ];

    let control = HighAssuranceApprovalControl::new(definition).expect("all disclosures valid");

    assert_eq!(control.report_disclosures().len(), 9);
}

#[test]
fn serde_round_trip_for_valid_control() {
    let control = valid_control();
    let serialized = serde_json::to_string(&control).expect("serialize control");
    let deserialized: HighAssuranceApprovalControl =
        serde_json::from_str(&serialized).expect("deserialize control");

    assert_eq!(deserialized, control);
}

#[test]
fn invalid_serialized_control_fails_closed() {
    let mut value = serde_json::to_value(valid_control()).expect("serialize control");
    value["minimum_approvals"] = json!(0);

    let error = serde_json::from_value::<HighAssuranceApprovalControl>(value)
        .expect_err("invalid serialized control");

    assert!(error
        .to_string()
        .contains("high-assurance approval controls require at least one approval"));
}

#[test]
fn secret_like_redaction_metadata_is_rejected_without_leakage() {
    let mut definition = valid_definition();
    definition.redaction = RedactionMetadata {
        redacted_fields: vec!["authorization_token".to_owned()],
        field_states: vec![RedactionFieldState {
            field: "approval_context".to_owned(),
            disposition: RedactionDisposition::Redacted,
            reason: "contains Bearer abc.def.ghi".to_owned(),
        }],
    };

    let error =
        HighAssuranceApprovalControl::new(definition).expect_err("secret-like redaction metadata");

    assert_eq!(error.code(), "high_assurance_approval.secret_like_value");
    assert!(!error.to_string().contains("authorization_token"));
    assert!(!error.to_string().contains("abc.def.ghi"));
}

#[test]
fn debug_output_does_not_leak_secret_like_values() {
    let control = valid_control();
    let debug = format!("{control:?}");

    assert!(!debug.contains("approval-control/nuclear-key"));
    assert!(!debug.contains("evidence/context"));
    assert!(!debug.contains("approval_context"));
    assert!(debug.contains("protected_action_count"));
    assert!(debug.contains("[REDACTED]"));
}

#[test]
fn serialization_does_not_include_forbidden_raw_payload_fields() {
    let serialized = serde_json::to_string(&valid_control()).expect("serialize control");

    for forbidden in [
        "raw_provider_payload",
        "raw_command_output",
        "raw_spec_contents",
        "authorization_header",
        "private_key",
        "api_token",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "serialized control should not include forbidden payload marker {forbidden}"
        );
    }
}
