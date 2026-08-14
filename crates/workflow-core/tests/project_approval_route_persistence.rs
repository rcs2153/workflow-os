#![allow(clippy::expect_used)]

//! Project-scoped approval route persistence model and store-contract tests.

use serde_json::Value;
use std::sync::{Arc, Barrier};
use std::thread;
use workflow_core::{
    resolve_project_approval_route, ActorId, ApprovalReferenceId, ApprovalRequest, CorrelationId,
    EventId, GovernanceApprovalBinding, GovernanceApprovalBindingId, GovernanceAssessmentBinding,
    HostedPrincipalBinding, HostedPrincipalKind, HostedPrincipalRegistry, HostedProjectCapability,
    HostedProjectGrant, HostedProjectResourceBinding, HostedProjectResourceBindingStatus,
    HostedProjectResourceKind, HostedProjectScope, IdempotencyKey,
    ImmutableRunBundleDefinitionKind, ImmutableRunBundleDefinitionReference,
    ImmutableRunBundleExecutionPosture, ImmutableRunBundleId, ImmutableRunBundleManifest,
    ImmutableRunBundleReferencePosture, ImmutableRunBundleSensitivity, ImmutableRunBundleVersion,
    InMemoryProjectApprovalRouteStoreFixture, LifecycleStatus, OrganizationId, OwnershipMetadata,
    ProjectApprovalAuthorityViewCommitment, ProjectApprovalRoute, ProjectApprovalRouteCreateResult,
    ProjectApprovalRouteInput, ProjectApprovalRouteRecord, ProjectApprovalRouteSourceCommitment,
    ProjectApprovalRouteSourceCommitmentInput, ProjectApprovalRouteStatus,
    ProjectApprovalRouteStore, ProjectApprovalRoutingReason, ProjectId, SchemaVersion, SkillId,
    SkillVersion, SpecContentHash, StepId, Timestamp, WorkflowId, WorkflowRunId, WorkflowVersion,
};

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).expect("valid timestamp")
}

fn scope(project: &str) -> HostedProjectScope {
    HostedProjectScope::new(
        OrganizationId::new("org/test").expect("organization id"),
        ProjectId::new(project).expect("project id"),
    )
}

fn run_id() -> WorkflowRunId {
    WorkflowRunId::new("run-route-store-test").expect("run id")
}

fn approval() -> ApprovalRequest {
    ApprovalRequest {
        approval_id: "approval/run-route-store-test/step-one".to_owned(),
        run_id: run_id(),
        workflow_id: WorkflowId::new("workflow/route-store-test").expect("workflow id"),
        schema_version: SchemaVersion::new("workflowos.dev/v0").expect("schema version"),
        workflow_version: WorkflowVersion::new("v1").expect("workflow version"),
        spec_content_hash: SpecContentHash::from_text("workflow definition"),
        resolved_execution_context_hash: Some(SpecContentHash::from_text("resolved context")),
        step_id: Some(StepId::new("step-one").expect("step id")),
        skill_id: Some(SkillId::new("skill/route-store").expect("skill id")),
        skill_version: Some(SkillVersion::new("v1").expect("skill version")),
        governance_approval_binding: None,
        requested_by: ActorId::new("system/kernel").expect("requester"),
        correlation_id: CorrelationId::new("correlation/route-store-test").expect("correlation"),
        idempotency_key: Some(IdempotencyKey::new("route-store-key").expect("idempotency key")),
        reason: "reason must never enter the route record".to_owned(),
        requested_at: timestamp("2026-08-13T12:00:00Z"),
        expires_after: None,
        expires_at: None,
        decision: None,
    }
}

fn run_binding(scope: &HostedProjectScope) -> HostedProjectResourceBinding {
    HostedProjectResourceBinding::new(
        scope.clone(),
        HostedProjectResourceKind::Run,
        run_id().as_str(),
        HostedProjectResourceBindingStatus::Active,
        timestamp("2026-08-13T11:00:00Z"),
    )
    .expect("run binding")
}

fn ownership() -> OwnershipMetadata {
    OwnershipMetadata {
        owning_team: Some("descriptive-only".to_owned()),
        maintainer: Some(ActorId::new("user/maintainer").expect("maintainer")),
        escalation_contact: None,
        lifecycle_status: LifecycleStatus::Experimental,
    }
}

fn principal(actor: &str, project: &str) -> HostedPrincipalBinding {
    HostedPrincipalBinding::new(
        ActorId::new(actor).expect("actor"),
        OrganizationId::new("org/test").expect("organization"),
        HostedPrincipalKind::Human,
        vec![HostedProjectGrant::new(
            ProjectId::new(project).expect("project"),
            vec![
                HostedProjectCapability::ApprovalRead,
                HostedProjectCapability::ApprovalDecide,
            ],
        )
        .expect("grant")],
    )
    .expect("principal")
}

fn principal_registry(
    scope: &HostedProjectScope,
    principals: Vec<HostedPrincipalBinding>,
) -> HostedPrincipalRegistry {
    HostedPrincipalRegistry::new(scope.organization_id().clone(), principals)
        .expect("principal registry")
}

fn authority_commitment(
    scope: &HostedProjectScope,
    principals: &[HostedPrincipalBinding],
) -> ProjectApprovalAuthorityViewCommitment {
    let registry = principal_registry(scope, principals.to_vec());
    ProjectApprovalAuthorityViewCommitment::from_registry(scope, &registry)
        .expect("authority commitment")
}

fn bundle_manifest(approval: &ApprovalRequest) -> ImmutableRunBundleManifest {
    let workflow_reference = ImmutableRunBundleDefinitionReference::new(
        ImmutableRunBundleDefinitionKind::Workflow,
        approval.workflow_id.as_str(),
        Some(approval.workflow_version.as_str().to_owned()),
        approval.schema_version.clone(),
        approval.spec_content_hash.clone(),
        None,
    )
    .expect("workflow reference");
    let posture = ImmutableRunBundleExecutionPosture::new(
        Vec::new(),
        ImmutableRunBundleReferencePosture::NotSupplied,
        ImmutableRunBundleReferencePosture::NotSupplied,
        ImmutableRunBundleReferencePosture::NotSupplied,
    )
    .expect("execution posture");
    ImmutableRunBundleManifest::new(
        ImmutableRunBundleId::new("bundle/route-store-test").expect("bundle id"),
        ImmutableRunBundleVersion::new("v1").expect("bundle version"),
        approval.run_id.clone(),
        approval.workflow_id.clone(),
        approval.workflow_version.clone(),
        approval.schema_version.clone(),
        approval.spec_content_hash.clone(),
        approval
            .resolved_execution_context_hash
            .clone()
            .unwrap_or_else(|| SpecContentHash::from_text("missing resolved context")),
        vec![workflow_reference],
        posture,
        Vec::new(),
        timestamp("2026-08-13T11:30:00Z"),
        ActorId::new("system/kernel").expect("bundle actor"),
        ImmutableRunBundleSensitivity::Internal,
        true,
    )
    .expect("bundle manifest")
}

fn aggregate_approval(aggregate_material: &str, source_material: &str) -> ApprovalRequest {
    let mut approval = approval();
    let assessment: GovernanceAssessmentBinding = serde_json::from_value(serde_json::json!({
        "binding_version": "v2",
        "assessment_set_algorithm": "v1",
        "workflow_id": approval.workflow_id.as_str(),
        "run_id": approval.run_id.as_str(),
        "immutable_run_bundle": {
            "bundle_id": "bundle/aggregate-route-store",
            "bundle_version": "v1",
            "root_hash": SpecContentHash::from_text("aggregate route bundle").as_str(),
        },
        "aggregate_fingerprint": SpecContentHash::from_text(aggregate_material).as_str(),
        "step_count": 3,
        "execution": "require_approval",
        "disclosure": "visible",
        "completeness": "complete",
        "source_binding": {
            "kind": "authoritative_local_check_reassessment",
            "algorithm": "v1",
            "fingerprint": SpecContentHash::from_text(source_material).as_str(),
            "selected_step_id": "step-one",
        },
    }))
    .expect("aggregate assessment");
    approval.step_id = None;
    approval.skill_id = None;
    approval.skill_version = None;
    approval.idempotency_key = None;
    approval.governance_approval_binding = Some(
        GovernanceApprovalBinding::new(
            GovernanceApprovalBindingId::new("approval-binding/route-store").expect("binding id"),
            assessment,
        )
        .expect("approval binding"),
    );
    approval
}

fn route(
    scope: &HostedProjectScope,
    approval: &ApprovalRequest,
    principals: &[HostedPrincipalBinding],
    resolved_at: &str,
) -> ProjectApprovalRoute {
    resolve_project_approval_route(&ProjectApprovalRouteInput {
        scope,
        run_binding: &run_binding(scope),
        approval,
        ownership: &ownership(),
        routing_reason: ProjectApprovalRoutingReason::WorkflowMaintainer,
        escalation: None,
        principals,
        resolved_at: timestamp(resolved_at),
    })
    .expect("route")
}

fn source_commitment(
    route: &ProjectApprovalRoute,
    approval: &ApprovalRequest,
    run_binding: &HostedProjectResourceBinding,
    authority: &ProjectApprovalAuthorityViewCommitment,
    approval_event: &str,
) -> ProjectApprovalRouteSourceCommitment {
    let event_id = EventId::new(approval_event).expect("event id");
    let bundle = bundle_manifest(approval);
    ProjectApprovalRouteSourceCommitment::new(
        route,
        &ProjectApprovalRouteSourceCommitmentInput {
            approval,
            approval_request_event_id: &event_id,
            immutable_run_bundle: &bundle,
            run_binding,
            escalation_event_id: None,
            authority_view: authority,
        },
    )
    .expect("source commitment")
}

fn record(
    scope: &HostedProjectScope,
    approval: &ApprovalRequest,
    principals: &[HostedPrincipalBinding],
    resolved_at: &str,
    created_at: &str,
    approval_event: &str,
) -> ProjectApprovalRouteRecord {
    let route = route(scope, approval, principals, resolved_at);
    let binding = run_binding(scope);
    let authority = authority_commitment(scope, principals);
    let source = source_commitment(&route, approval, &binding, &authority, approval_event);
    ProjectApprovalRouteRecord::new(route, source, timestamp(created_at)).expect("record")
}

#[test]
fn creates_reads_and_round_trips_valid_record() {
    let scope = scope("project/alpha");
    let approval = approval();
    let principals = vec![principal("user/maintainer", "project/alpha")];
    let record = record(
        &scope,
        &approval,
        &principals,
        "2026-08-13T12:00:00Z",
        "2026-08-13T12:00:01Z",
        "event/approval-requested",
    );
    let store = InMemoryProjectApprovalRouteStoreFixture::default();

    let result = store
        .create_project_approval_route(record.clone())
        .expect("record creates");
    assert!(matches!(
        result,
        ProjectApprovalRouteCreateResult::Created(_)
    ));
    assert_eq!(
        store
            .read_project_approval_route(record.logical_subject_id())
            .expect("record reads"),
        Some(record.clone())
    );

    let json = serde_json::to_string(&record).expect("record serializes");
    let restored: ProjectApprovalRouteRecord =
        serde_json::from_str(&json).expect("record deserializes");
    assert_eq!(restored, record);
}

#[test]
fn exact_retry_preserves_first_resolved_and_created_timestamps() {
    let scope = scope("project/alpha");
    let approval = approval();
    let principals = vec![principal("user/maintainer", "project/alpha")];
    let first = record(
        &scope,
        &approval,
        &principals,
        "2026-08-13T12:00:00Z",
        "2026-08-13T12:00:01Z",
        "event/approval-requested",
    );
    let later = record(
        &scope,
        &approval,
        &principals,
        "2026-08-13T12:30:00Z",
        "2026-08-13T12:30:01Z",
        "event/approval-requested",
    );
    assert!(first.is_decision_equivalent(&later));
    let store = InMemoryProjectApprovalRouteStoreFixture::default();
    store
        .create_project_approval_route(first.clone())
        .expect("first creates");

    let result = store
        .create_project_approval_route(later)
        .expect("retry reconciles");
    assert!(matches!(
        result,
        ProjectApprovalRouteCreateResult::ReconciledExisting(_)
    ));
    assert_eq!(result.record().created_at(), first.created_at());
    assert_eq!(
        result.record().route().resolved_at(),
        first.route().resolved_at()
    );
}

#[test]
fn changed_source_for_same_logical_subject_conflicts_without_leakage() {
    let scope = scope("project/alpha");
    let approval = approval();
    let principals = vec![principal("user/maintainer", "project/alpha")];
    let first = record(
        &scope,
        &approval,
        &principals,
        "2026-08-13T12:00:00Z",
        "2026-08-13T12:00:01Z",
        "event/secret-first",
    );
    let changed = record(
        &scope,
        &approval,
        &principals,
        "2026-08-13T12:00:00Z",
        "2026-08-13T12:00:01Z",
        "event/secret-second",
    );
    let store = InMemoryProjectApprovalRouteStoreFixture::default();
    store
        .create_project_approval_route(first)
        .expect("first creates");
    let error = store
        .create_project_approval_route(changed)
        .expect_err("source drift conflicts");
    assert_eq!(error.code(), "project_approval_route_store.create.conflict");
    assert!(!error.to_string().contains("secret"));
}

#[test]
fn record_rejects_source_commitment_bound_to_a_different_route_decision() {
    let scope = scope("project/alpha");
    let approval = approval();
    let routed_principals = vec![principal("user/maintainer", "project/alpha")];
    let unresolved_principals = vec![principal("user/other", "project/alpha")];
    let routed = route(
        &scope,
        &approval,
        &routed_principals,
        "2026-08-13T12:00:00Z",
    );
    let unresolved = route(
        &scope,
        &approval,
        &unresolved_principals,
        "2026-08-13T12:00:00Z",
    );
    let binding = run_binding(&scope);
    let authority = authority_commitment(&scope, &routed_principals);
    let source = source_commitment(&routed, &approval, &binding, &authority, "event/request");

    let record =
        ProjectApprovalRouteRecord::new(unresolved, source, timestamp("2026-08-13T12:00:01Z"))
            .expect_err("mismatched route source fails closed");
    assert_eq!(record.code(), "project_approval_route_store.record.invalid");
}

#[test]
fn concurrent_identical_writers_create_once_and_reconcile_once() {
    let scope = scope("project/alpha");
    let approval = approval();
    let principals = vec![principal("user/maintainer", "project/alpha")];
    let record = record(
        &scope,
        &approval,
        &principals,
        "2026-08-13T12:00:00Z",
        "2026-08-13T12:00:01Z",
        "event/concurrent",
    );
    let store = InMemoryProjectApprovalRouteStoreFixture::default();
    let barrier = Arc::new(Barrier::new(2));
    let handles = (0..2)
        .map(|_| {
            let store = store.clone();
            let barrier = Arc::clone(&barrier);
            let record = record.clone();
            thread::spawn(move || {
                barrier.wait();
                store
                    .create_project_approval_route(record)
                    .expect("concurrent write")
            })
        })
        .collect::<Vec<_>>();
    let results = handles
        .into_iter()
        .map(|handle| handle.join().expect("writer joins"))
        .collect::<Vec<_>>();
    assert_eq!(
        results.iter().filter(|result| result.was_created()).count(),
        1
    );
    assert_eq!(
        results
            .iter()
            .filter(|result| !result.was_created())
            .count(),
        1
    );
}

#[test]
fn concurrent_conflicting_writers_cannot_both_commit() {
    let scope = scope("project/alpha");
    let approval = approval();
    let principals = vec![principal("user/maintainer", "project/alpha")];
    let candidates = ["event/concurrent-a", "event/concurrent-b"].map(|event| {
        record(
            &scope,
            &approval,
            &principals,
            "2026-08-13T12:00:00Z",
            "2026-08-13T12:00:01Z",
            event,
        )
    });
    let store = InMemoryProjectApprovalRouteStoreFixture::default();
    let barrier = Arc::new(Barrier::new(2));
    let handles = candidates.map(|record| {
        let store = store.clone();
        let barrier = Arc::clone(&barrier);
        thread::spawn(move || {
            barrier.wait();
            store.create_project_approval_route(record)
        })
    });
    let results = handles.map(|handle| handle.join().expect("writer joins"));
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    let error = results
        .iter()
        .find_map(|result| result.as_ref().err())
        .expect("one conflict");
    assert_eq!(error.code(), "project_approval_route_store.create.conflict");
}

#[test]
fn authority_commitment_is_order_independent_and_changes_with_grants() {
    let scope = scope("project/alpha");
    let alpha = principal("user/alpha", "project/alpha");
    let beta = principal("user/beta", "project/alpha");
    let left = authority_commitment(&scope, &[alpha.clone(), beta.clone()]);
    let right = authority_commitment(&scope, &[beta, alpha]);
    assert_eq!(left, right);

    let changed = authority_commitment(&scope, &[principal("user/alpha", "project/beta")]);
    assert_ne!(left, changed);
}

#[test]
fn authority_registry_rejects_duplicate_and_cross_organization_principals() {
    let scope = scope("project/alpha");
    let duplicate = principal("user/alpha", "project/alpha");
    let error = HostedPrincipalRegistry::new(
        scope.organization_id().clone(),
        vec![duplicate.clone(), duplicate],
    )
    .expect_err("duplicate actors fail closed");
    assert_eq!(error.code(), "hosted_project.principal_registry.duplicate");

    let error = HostedPrincipalRegistry::new(
        OrganizationId::new("org/other").expect("other organization"),
        vec![principal("user/alpha", "project/alpha")],
    )
    .expect_err("cross-organization principals fail closed");
    assert_eq!(error.code(), "hosted_project.principal_registry.invalid");
}

#[test]
fn coherent_bundle_manifest_must_match_the_exact_approval_context() {
    let scope = scope("project/alpha");
    let principals = vec![principal("user/maintainer", "project/alpha")];
    let approval = approval();
    let route = route(&scope, &approval, &principals, "2026-08-13T12:00:00Z");
    let authority = authority_commitment(&scope, &principals);
    let mut changed = approval.clone();
    changed.spec_content_hash = SpecContentHash::from_text("changed workflow definition");
    let mismatched_bundle = bundle_manifest(&changed);
    let event = EventId::new("event/request").expect("event");
    let binding = run_binding(&scope);

    let error = ProjectApprovalRouteSourceCommitment::new(
        &route,
        &ProjectApprovalRouteSourceCommitmentInput {
            approval: &approval,
            approval_request_event_id: &event,
            immutable_run_bundle: &mismatched_bundle,
            run_binding: &binding,
            escalation_event_id: None,
            authority_view: &authority,
        },
    )
    .expect_err("mismatched coherent bundle fails closed");
    assert_eq!(error.code(), "project_approval_route_store.source.mismatch");
}

#[test]
fn aggregate_approval_subject_commits_nested_assessment_provenance() {
    let scope = scope("project/alpha");
    let principals = vec![principal("user/maintainer", "project/alpha")];
    let first_approval =
        aggregate_approval("aggregate assessment", "authoritative aggregate source one");
    let changed_approval =
        aggregate_approval("aggregate assessment", "authoritative aggregate source two");
    let first_route = route(&scope, &first_approval, &principals, "2026-08-13T12:00:00Z");
    let changed_route = route(
        &scope,
        &changed_approval,
        &principals,
        "2026-08-13T12:00:00Z",
    );
    assert_eq!(first_route.route_id(), changed_route.route_id());
    let authority = authority_commitment(&scope, &principals);
    let binding = run_binding(&scope);
    let first_source = source_commitment(
        &first_route,
        &first_approval,
        &binding,
        &authority,
        "event/aggregate-request",
    );
    let changed_source = source_commitment(
        &changed_route,
        &changed_approval,
        &binding,
        &authority,
        "event/aggregate-request",
    );

    assert_ne!(first_source, changed_source);
}

#[test]
fn source_commitment_rejects_decided_missing_context_and_inactive_binding() {
    let scope = scope("project/alpha");
    let principals = vec![principal("user/maintainer", "project/alpha")];
    let approval = approval();
    let route = route(&scope, &approval, &principals, "2026-08-13T12:00:00Z");
    let authority = authority_commitment(&scope, &principals);
    let mut no_context = approval.clone();
    no_context.resolved_execution_context_hash = None;
    let no_context_bundle = bundle_manifest(&approval);
    let no_context_binding = run_binding(&scope);
    let no_context_event = EventId::new("event/request").expect("event");
    let error = ProjectApprovalRouteSourceCommitment::new(
        &route,
        &ProjectApprovalRouteSourceCommitmentInput {
            approval: &no_context,
            approval_request_event_id: &no_context_event,
            immutable_run_bundle: &no_context_bundle,
            run_binding: &no_context_binding,
            escalation_event_id: None,
            authority_view: &authority,
        },
    )
    .expect_err("missing context fails");
    assert_eq!(error.code(), "project_approval_route_store.source.mismatch");

    let reserved = HostedProjectResourceBinding::new(
        scope.clone(),
        HostedProjectResourceKind::Run,
        run_id().as_str(),
        HostedProjectResourceBindingStatus::Reserved,
        timestamp("2026-08-13T11:00:00Z"),
    )
    .expect("reserved binding");
    let event_id = EventId::new("event/request").expect("event");
    let bundle = bundle_manifest(&approval);
    let error = ProjectApprovalRouteSourceCommitment::new(
        &route,
        &ProjectApprovalRouteSourceCommitmentInput {
            approval: &approval,
            approval_request_event_id: &event_id,
            immutable_run_bundle: &bundle,
            run_binding: &reserved,
            escalation_event_id: None,
            authority_view: &authority,
        },
    )
    .expect_err("reserved binding fails");
    assert_eq!(error.code(), "project_approval_route_store.source.mismatch");
}

#[test]
fn routed_recipient_enumeration_is_exact_bounded_and_excludes_unresolved() {
    let alpha = scope("project/alpha");
    let beta = scope("project/beta");
    let approval = approval();
    let mut unresolved_approval = approval.clone();
    unresolved_approval.approval_id = "approval/run-route-store-test/step-two".to_owned();
    let alpha_principals = vec![principal("user/maintainer", "project/alpha")];
    let beta_principals = vec![principal("user/maintainer", "project/beta")];
    let unresolved_principals = vec![principal("user/other", "project/alpha")];
    let store = InMemoryProjectApprovalRouteStoreFixture::default();
    for item in [
        record(
            &alpha,
            &approval,
            &alpha_principals,
            "2026-08-13T12:00:00Z",
            "2026-08-13T12:00:01Z",
            "event/alpha",
        ),
        record(
            &beta,
            &approval,
            &beta_principals,
            "2026-08-13T12:00:00Z",
            "2026-08-13T12:00:01Z",
            "event/beta",
        ),
        record(
            &alpha,
            &unresolved_approval,
            &unresolved_principals,
            "2026-08-13T12:00:00Z",
            "2026-08-13T12:00:01Z",
            "event/unresolved",
        ),
    ] {
        store
            .create_project_approval_route(item)
            .expect("record creates");
    }

    let recipient = ActorId::new("user/maintainer").expect("recipient");
    let listed = store
        .list_project_approval_routes_for_recipient(&alpha, &recipient, 10)
        .expect("routes list");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].route().scope(), &alpha);
    assert_eq!(
        listed[0].route().status(),
        ProjectApprovalRouteStatus::Routed
    );

    let error = store
        .list_project_approval_routes_for_recipient(&alpha, &recipient, 0)
        .expect_err("zero limit fails");
    assert_eq!(
        error.code(),
        "project_approval_route_store.list.limit.invalid"
    );
}

#[test]
fn approval_enumeration_includes_unresolved_but_never_crosses_project() {
    let alpha = scope("project/alpha");
    let beta = scope("project/beta");
    let approval = approval();
    let unresolved = vec![principal("user/other", "project/alpha")];
    let store = InMemoryProjectApprovalRouteStoreFixture::default();
    store
        .create_project_approval_route(record(
            &alpha,
            &approval,
            &unresolved,
            "2026-08-13T12:00:00Z",
            "2026-08-13T12:00:01Z",
            "event/alpha",
        ))
        .expect("alpha creates");
    let approval_reference =
        ApprovalReferenceId::new(approval.approval_id.clone()).expect("approval reference");
    assert_eq!(
        store
            .list_project_approval_routes_for_approval(&alpha, &run_id(), &approval_reference, 10,)
            .expect("approval list")
            .len(),
        1
    );
    assert!(store
        .list_project_approval_routes_for_approval(&beta, &run_id(), &approval_reference, 10,)
        .expect("cross-project list")
        .is_empty());
}

#[test]
fn approval_enumeration_requires_a_valid_bounded_reference_id() {
    let empty = ApprovalReferenceId::new("").expect_err("empty id fails");
    assert_eq!(empty.code(), "evidence.identifier.empty");
    let oversized = ApprovalReferenceId::new("x".repeat(513)).expect_err("oversized id fails");
    assert_eq!(oversized.code(), "evidence.identifier.too_long");
}

#[test]
fn deserialization_rejects_tampered_logical_subject_without_echoing_values() {
    let scope = scope("project/secret-alpha");
    let approval = approval();
    let principals = vec![principal("user/maintainer", "project/secret-alpha")];
    let record = record(
        &scope,
        &approval,
        &principals,
        "2026-08-13T12:00:00Z",
        "2026-08-13T12:00:01Z",
        "event/secret-request",
    );
    let mut value: Value = serde_json::to_value(record).expect("record json");
    value["logical_subject_id"] =
        Value::String(format!("project-approval-route-subject-{}", "0".repeat(64)));
    let error = serde_json::from_value::<ProjectApprovalRouteRecord>(value)
        .expect_err("tampering fails closed");
    assert_eq!(error.to_string(), "invalid project approval route record");
    assert!(!error.to_string().contains("secret"));
}

#[test]
fn debug_and_serialization_do_not_copy_forbidden_approval_reason() {
    let scope = scope("project/alpha");
    let approval = approval();
    let principals = vec![principal("user/maintainer", "project/alpha")];
    let record = record(
        &scope,
        &approval,
        &principals,
        "2026-08-13T12:00:00Z",
        "2026-08-13T12:00:01Z",
        "event/request",
    );
    let debug = format!("{record:?}");
    let json = serde_json::to_string(&record).expect("record json");
    for forbidden in [
        "reason must never enter the route record",
        "workflow/route-store-test",
        "user/maintainer",
        "project/alpha",
    ] {
        assert!(!debug.contains(forbidden));
    }
    assert!(!json.contains("reason must never enter the route record"));
    assert!(!json.contains("principal"));
    assert!(!json.contains("grant"));
}
