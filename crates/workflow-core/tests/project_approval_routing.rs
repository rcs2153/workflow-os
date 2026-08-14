#![allow(clippy::expect_used)]

//! Deterministic project-scoped approval routing boundary tests.

use serde_json::Value;
use workflow_core::{
    resolve_project_approval_route, ActorId, ApprovalDecision, ApprovalDecisionKind,
    ApprovalRequest, CorrelationId, EscalationRecord, FailureClass, HostedPrincipalBinding,
    HostedPrincipalKind, HostedProjectCapability, HostedProjectGrant, HostedProjectResourceBinding,
    HostedProjectResourceBindingStatus, HostedProjectResourceKind, HostedProjectScope,
    IdempotencyKey, LifecycleStatus, OrganizationId, OwnershipMetadata,
    ProjectApprovalNotificationPosture, ProjectApprovalRoute, ProjectApprovalRouteInput,
    ProjectApprovalRouteStatus, ProjectApprovalRoutingReason, ProjectId, SchemaVersion, SkillId,
    SkillVersion, SpecContentHash, StepId, Timestamp, WorkflowId, WorkflowRunId, WorkflowVersion,
};

fn timestamp() -> Timestamp {
    Timestamp::parse_rfc3339("2026-08-13T12:00:00Z").expect("valid timestamp")
}

fn scope() -> HostedProjectScope {
    HostedProjectScope::new(
        OrganizationId::new("org/test").expect("organization id"),
        ProjectId::new("project/alpha").expect("project id"),
    )
}

fn run_id() -> WorkflowRunId {
    WorkflowRunId::new("run-routing-test").expect("run id")
}

fn approval() -> ApprovalRequest {
    ApprovalRequest {
        approval_id: "approval/run-routing-test/step-one".to_owned(),
        run_id: run_id(),
        workflow_id: WorkflowId::new("workflow/routing-test").expect("workflow id"),
        schema_version: SchemaVersion::new("workflowos.dev/v0").expect("schema version"),
        workflow_version: WorkflowVersion::new("v1").expect("workflow version"),
        spec_content_hash: SpecContentHash::from_text("routing workflow"),
        resolved_execution_context_hash: Some(SpecContentHash::from_text("resolved context")),
        step_id: Some(StepId::new("step-one").expect("step id")),
        skill_id: Some(SkillId::new("skill/routing").expect("skill id")),
        skill_version: Some(SkillVersion::new("v1").expect("skill version")),
        governance_approval_binding: None,
        requested_by: ActorId::new("system/kernel").expect("requester"),
        correlation_id: CorrelationId::new("correlation/routing-test").expect("correlation"),
        idempotency_key: Some(IdempotencyKey::new("routing-test-key").expect("idempotency key")),
        reason: "bounded routing test".to_owned(),
        requested_at: timestamp(),
        expires_after: None,
        expires_at: None,
        decision: None,
    }
}

fn active_run_binding(scope: HostedProjectScope) -> HostedProjectResourceBinding {
    HostedProjectResourceBinding::new(
        scope,
        HostedProjectResourceKind::Run,
        run_id().as_str(),
        HostedProjectResourceBindingStatus::Active,
        timestamp(),
    )
    .expect("active run binding")
}

fn ownership() -> OwnershipMetadata {
    OwnershipMetadata {
        owning_team: Some("descriptive-team-only".to_owned()),
        maintainer: Some(ActorId::new("user/maintainer").expect("maintainer")),
        escalation_contact: Some(ActorId::new("user/escalation").expect("escalation")),
        lifecycle_status: LifecycleStatus::Experimental,
    }
}

fn principal(actor: &str, project: ProjectId) -> HostedPrincipalBinding {
    HostedPrincipalBinding::new(
        ActorId::new(actor).expect("actor id"),
        OrganizationId::new("org/test").expect("organization id"),
        HostedPrincipalKind::Human,
        vec![
            HostedProjectGrant::new(project, vec![HostedProjectCapability::ApprovalDecide])
                .expect("project grant"),
        ],
    )
    .expect("principal binding")
}

fn resolve(
    scope: &HostedProjectScope,
    run_binding: &HostedProjectResourceBinding,
    approval: &ApprovalRequest,
    ownership: &OwnershipMetadata,
    reason: ProjectApprovalRoutingReason,
    principals: &[HostedPrincipalBinding],
) -> Result<ProjectApprovalRoute, workflow_core::WorkflowOsError> {
    resolve_project_approval_route(&ProjectApprovalRouteInput {
        scope,
        run_binding,
        approval,
        ownership,
        routing_reason: reason,
        escalation: None,
        principals,
        resolved_at: timestamp(),
    })
}

#[test]
fn routes_maintainer_only_when_exact_project_authority_exists() {
    let scope = scope();
    let run_binding = active_run_binding(scope.clone());
    let approval = approval();
    let ownership = ownership();
    let principals = vec![principal("user/maintainer", scope.project_id().clone())];

    let route = resolve(
        &scope,
        &run_binding,
        &approval,
        &ownership,
        ProjectApprovalRoutingReason::WorkflowMaintainer,
        &principals,
    )
    .expect("route resolves");

    assert_eq!(route.scope(), &scope);
    assert_eq!(route.run_id(), &approval.run_id);
    assert_eq!(route.approval_id(), approval.approval_id);
    assert_eq!(route.workflow_id(), &approval.workflow_id);
    assert_eq!(route.status(), ProjectApprovalRouteStatus::Routed);
    assert_eq!(
        route.recipient().map(ActorId::as_str),
        Some("user/maintainer")
    );
    assert_eq!(
        route.notification_posture(),
        ProjectApprovalNotificationPosture::AvailableForProjectInbox
    );
}

#[test]
fn metadata_never_grants_authority_and_owning_team_is_ignored() {
    let scope = scope();
    let run_binding = active_run_binding(scope.clone());
    let approval = approval();
    let ownership = ownership();

    let route = resolve(
        &scope,
        &run_binding,
        &approval,
        &ownership,
        ProjectApprovalRoutingReason::WorkflowMaintainer,
        &[],
    )
    .expect("unresolved route is valid");

    assert_eq!(
        route.status(),
        ProjectApprovalRouteStatus::UnresolvedAuthorityUnavailable
    );
    assert_eq!(route.recipient(), None);
    assert_eq!(
        route.notification_posture(),
        ProjectApprovalNotificationPosture::UnavailableRouteUnresolved
    );
}

#[test]
fn wrong_project_and_wrong_organization_authority_do_not_route() {
    let scope = scope();
    let run_binding = active_run_binding(scope.clone());
    let approval = approval();
    let ownership = ownership();
    let wrong_project = principal(
        "user/maintainer",
        ProjectId::new("project/beta").expect("project id"),
    );
    let wrong_organization = HostedPrincipalBinding::new(
        ActorId::new("user/maintainer").expect("actor"),
        OrganizationId::new("org/other").expect("organization"),
        HostedPrincipalKind::Human,
        vec![HostedProjectGrant::new(
            scope.project_id().clone(),
            vec![HostedProjectCapability::ApprovalDecide],
        )
        .expect("grant")],
    )
    .expect("principal");

    for principals in [vec![wrong_project], vec![wrong_organization]] {
        let route = resolve(
            &scope,
            &run_binding,
            &approval,
            &ownership,
            ProjectApprovalRoutingReason::WorkflowMaintainer,
            &principals,
        )
        .expect("unresolved route is valid");
        assert_eq!(
            route.status(),
            ProjectApprovalRouteStatus::UnresolvedAuthorityUnavailable
        );
    }
}

#[test]
fn missing_metadata_is_explicit_and_escalation_contact_is_representable() {
    let scope = scope();
    let run_binding = active_run_binding(scope.clone());
    let approval = approval();
    let mut ownership = ownership();
    ownership.maintainer = None;
    let escalation_principals = vec![principal("user/escalation", scope.project_id().clone())];

    let missing = resolve(
        &scope,
        &run_binding,
        &approval,
        &ownership,
        ProjectApprovalRoutingReason::WorkflowMaintainer,
        &escalation_principals,
    )
    .expect("missing metadata is explicit");
    assert_eq!(
        missing.status(),
        ProjectApprovalRouteStatus::UnresolvedMissingMetadata
    );

    let missing_subject = resolve(
        &scope,
        &run_binding,
        &approval,
        &ownership,
        ProjectApprovalRoutingReason::WorkflowEscalationContact,
        &escalation_principals,
    )
    .expect_err("ordinary approval cannot select escalation contact");
    assert_eq!(
        missing_subject.code(),
        "project_approval_route.escalation_subject.missing"
    );

    let escalation = escalation_record("user/escalation", run_id());
    let escalated = resolve_project_approval_route(&ProjectApprovalRouteInput {
        scope: &scope,
        run_binding: &run_binding,
        approval: &approval,
        ownership: &ownership,
        routing_reason: ProjectApprovalRoutingReason::WorkflowEscalationContact,
        escalation: Some(&escalation),
        principals: &escalation_principals,
        resolved_at: timestamp(),
    })
    .expect("run-bound escalation route resolves");
    assert_eq!(escalated.status(), ProjectApprovalRouteStatus::Routed);
    assert_eq!(escalated.escalation_id(), Some("escalation/routing-test"));
    assert_eq!(
        escalated.recipient().map(ActorId::as_str),
        Some("user/escalation")
    );

    let debug = format!("{escalated:?}");
    assert!(!debug.contains("escalation/routing-test"));
    let serialized = serde_json::to_string(&escalated).expect("escalation route serializes");
    let restored: ProjectApprovalRoute =
        serde_json::from_str(&serialized).expect("escalation route round trips");
    assert_eq!(restored, escalated);

    let mut missing_proof: Value = serde_json::from_str(&serialized).expect("route json");
    missing_proof["escalation_id"] = Value::Null;
    let error = serde_json::from_value::<ProjectApprovalRoute>(missing_proof)
        .expect_err("missing escalation proof fails closed");
    assert_eq!(error.to_string(), "invalid project approval route");
}

#[test]
fn escalation_subject_must_match_run_and_immutable_contact() {
    let scope = scope();
    let run_binding = active_run_binding(scope.clone());
    let approval = approval();
    let ownership = ownership();
    let principals = vec![principal("user/escalation", scope.project_id().clone())];

    for escalation in [
        escalation_record(
            "user/escalation",
            WorkflowRunId::new("run-other").expect("other run"),
        ),
        escalation_record("user/other-contact", run_id()),
    ] {
        let error = resolve_project_approval_route(&ProjectApprovalRouteInput {
            scope: &scope,
            run_binding: &run_binding,
            approval: &approval,
            ownership: &ownership,
            routing_reason: ProjectApprovalRoutingReason::WorkflowEscalationContact,
            escalation: Some(&escalation),
            principals: &principals,
            resolved_at: timestamp(),
        })
        .expect_err("mismatched escalation rejected");
        assert_eq!(
            error.code(),
            "project_approval_route.escalation_subject.mismatch"
        );
        assert!(!error.message().contains(escalation.escalation_id.as_str()));
    }
}

#[test]
fn ordinary_route_rejects_unexpected_escalation_subject() {
    let scope = scope();
    let run_binding = active_run_binding(scope.clone());
    let approval = approval();
    let ownership = ownership();
    let escalation = escalation_record("user/escalation", run_id());
    let principals = vec![principal("user/maintainer", scope.project_id().clone())];

    let error = resolve_project_approval_route(&ProjectApprovalRouteInput {
        scope: &scope,
        run_binding: &run_binding,
        approval: &approval,
        ownership: &ownership,
        routing_reason: ProjectApprovalRoutingReason::WorkflowMaintainer,
        escalation: Some(&escalation),
        principals: &principals,
        resolved_at: timestamp(),
    })
    .expect_err("ordinary route rejects escalation subject");
    assert_eq!(
        error.code(),
        "project_approval_route.escalation_subject.unexpected"
    );
}

#[test]
fn duplicate_authority_state_is_rejected_as_ambiguous_without_values() {
    let scope = scope();
    let run_binding = active_run_binding(scope.clone());
    let approval = approval();
    let ownership = ownership();
    let binding = principal("user/maintainer", scope.project_id().clone());
    let marker = "super-secret-route-marker";

    let error = resolve(
        &scope,
        &run_binding,
        &approval,
        &ownership,
        ProjectApprovalRoutingReason::WorkflowMaintainer,
        &[binding.clone(), binding],
    )
    .expect_err("duplicate authority rejected");

    assert_eq!(error.code(), "project_approval_route.authority.ambiguous");
    assert!(!format!("{error:?}").contains(marker));
    assert!(!error.message().contains("user/maintainer"));
}

#[test]
fn invalid_binding_and_decided_approval_fail_closed() {
    let scope = scope();
    let reserved = HostedProjectResourceBinding::new(
        scope.clone(),
        HostedProjectResourceKind::Run,
        run_id().as_str(),
        HostedProjectResourceBindingStatus::Reserved,
        timestamp(),
    )
    .expect("reserved binding");
    let approval = approval();
    let ownership = ownership();

    let binding_error = resolve(
        &scope,
        &reserved,
        &approval,
        &ownership,
        ProjectApprovalRoutingReason::WorkflowMaintainer,
        &[],
    )
    .expect_err("reserved binding rejected");
    assert_eq!(
        binding_error.code(),
        "project_approval_route.run_binding.invalid"
    );

    let active = active_run_binding(scope.clone());
    let mut decided = approval;
    decided.decision = Some(ApprovalDecision {
        approval_id: decided.approval_id.clone(),
        actor: ActorId::new("user/reviewer").expect("reviewer"),
        decided_at: timestamp(),
        decision: ApprovalDecisionKind::Denied,
        reason: "bounded denial".to_owned(),
        correlation_id: CorrelationId::new("correlation/routing-decision")
            .expect("decision correlation"),
        proof_marker: None,
    });
    let decision_error = resolve(
        &scope,
        &active,
        &decided,
        &ownership,
        ProjectApprovalRoutingReason::WorkflowMaintainer,
        &[],
    )
    .expect_err("decided approval rejected");
    assert_eq!(
        decision_error.code(),
        "project_approval_route.approval.not_pending"
    );
}

#[test]
fn route_identity_is_deterministic_and_deserialization_fails_closed_on_tampering() {
    let scope = scope();
    let run_binding = active_run_binding(scope.clone());
    let approval = approval();
    let ownership = ownership();
    let principals = vec![principal("user/maintainer", scope.project_id().clone())];

    let first = resolve(
        &scope,
        &run_binding,
        &approval,
        &ownership,
        ProjectApprovalRoutingReason::WorkflowMaintainer,
        &principals,
    )
    .expect("first route");
    let second = resolve(
        &scope,
        &run_binding,
        &approval,
        &ownership,
        ProjectApprovalRoutingReason::WorkflowMaintainer,
        &principals,
    )
    .expect("second route");
    assert_eq!(first.route_id(), second.route_id());

    let serialized = serde_json::to_string(&first).expect("route serializes");
    let restored: ProjectApprovalRoute =
        serde_json::from_str(&serialized).expect("route round trip");
    assert_eq!(restored, first);

    let mut tampered: Value = serde_json::from_str(&serialized).expect("route json");
    tampered["route_id"] = Value::String(format!("project-approval-route-{}", "0".repeat(64)));
    let error = serde_json::from_value::<ProjectApprovalRoute>(tampered)
        .expect_err("tampered route identity rejected");
    assert_eq!(error.to_string(), "invalid project approval route");
}

#[test]
fn debug_output_redacts_all_stable_route_references() {
    let scope = scope();
    let run_binding = active_run_binding(scope.clone());
    let approval = approval();
    let ownership = ownership();
    let principals = vec![principal("user/maintainer", scope.project_id().clone())];
    let route = resolve(
        &scope,
        &run_binding,
        &approval,
        &ownership,
        ProjectApprovalRoutingReason::WorkflowMaintainer,
        &principals,
    )
    .expect("route resolves");

    let debug = format!("{route:?}");
    for secret in [
        route.route_id().as_str(),
        scope.organization_id().as_str(),
        scope.project_id().as_str(),
        approval.run_id.as_str(),
        approval.approval_id.as_str(),
        approval.workflow_id.as_str(),
        "user/maintainer",
    ] {
        assert!(!debug.contains(secret));
    }
    assert!(debug.contains("Routed"));
    assert!(debug.contains("AvailableForProjectInbox"));
}

fn escalation_record(contact: &str, run_id: WorkflowRunId) -> EscalationRecord {
    EscalationRecord {
        escalation_id: "escalation/routing-test".to_owned(),
        run_id,
        step_id: Some(StepId::new("step-one").expect("step id")),
        skill_id: Some(SkillId::new("skill/routing").expect("skill id")),
        skill_version: Some(SkillVersion::new("v1").expect("skill version")),
        attempts: 2,
        last_error: "runtime.failure".to_owned(),
        failure_class: FailureClass::Permanent,
        suggested_next_action: "review escalation".to_owned(),
        reason: "bounded escalation".to_owned(),
        contact: Some(ActorId::new(contact).expect("contact")),
    }
}
