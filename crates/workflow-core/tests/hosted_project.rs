#![allow(clippy::expect_used)]

//! Hosted collaborative project boundary model tests.

use serde_json::{json, Value};
use workflow_core::{
    ActorId, EventId, HostedPrincipalBinding, HostedPrincipalKind, HostedProjectAccessDecision,
    HostedProjectCapability, HostedProjectCatalogVersion, HostedProjectGrant,
    HostedProjectResourceBinding, HostedProjectResourceBindingStatus, HostedProjectResourceKind,
    HostedProjectScope, OrganizationId, ProjectId, RedactionMetadata, SchemaVersion,
    SpecContentHash, Timestamp, WorkReportSensitivity, WorkflowArchiveRecordId,
    WorkflowCatalogRecord, WorkflowCatalogRecordDefinition, WorkflowCatalogRecordId, WorkflowId,
    WorkflowLifecycleStatus, WorkflowStewardshipDecisionId, WorkflowVersion,
};

fn organization_id() -> OrganizationId {
    OrganizationId::new("org/hosted-test").expect("valid organization id")
}

fn project_id() -> ProjectId {
    ProjectId::new("project/hosted-test").expect("valid project id")
}

fn actor_id() -> ActorId {
    ActorId::new("user/hosted-operator").expect("valid actor id")
}

fn scope() -> HostedProjectScope {
    HostedProjectScope::new(organization_id(), project_id())
}

fn timestamp() -> Timestamp {
    Timestamp::parse_rfc3339("2026-08-13T00:00:00Z").expect("valid timestamp")
}

fn grant(project_id: ProjectId, capabilities: Vec<HostedProjectCapability>) -> HostedProjectGrant {
    HostedProjectGrant::new(project_id, capabilities).expect("valid project grant")
}

fn stewardship_id() -> WorkflowStewardshipDecisionId {
    WorkflowStewardshipDecisionId::new("stewardship/hosted/approved").expect("valid stewardship id")
}

fn workflow_id() -> WorkflowId {
    WorkflowId::new("hosted/catalog-workflow").expect("valid workflow id")
}

fn catalog_record_definition() -> WorkflowCatalogRecordDefinition {
    WorkflowCatalogRecordDefinition {
        record_id: WorkflowCatalogRecordId::new("catalog/hosted/workflow")
            .expect("valid catalog record id"),
        workflow_id: workflow_id(),
        workflow_path: "workflows/hosted.workflow.yml".to_owned(),
        workflow_content_hash: SpecContentHash::from_text("hosted workflow"),
        schema_version: SchemaVersion::new("workflowos.dev/v0").expect("valid schema version"),
        lifecycle_status: WorkflowLifecycleStatus::Active,
        source_recommendation_id: None,
        source_draft_path: None,
        archived_draft_path: None,
        owner: Some(actor_id()),
        escalation_contact: Some(
            ActorId::new("user/hosted-escalation").expect("valid escalation actor"),
        ),
        authority_scope: Some("governs hosted project workflow".to_owned()),
        evidence_check_report_posture: Some("requires bounded evidence".to_owned()),
        side_effect_posture: Some("none_skipped_unsupported".to_owned()),
        latest_stewardship_decision_id: Some(stewardship_id()),
        latest_promotion_decision_id: Some(stewardship_id()),
        latest_archive_record_id: Some(
            WorkflowArchiveRecordId::new("archive/hosted/workflow")
                .expect("valid archive record id"),
        ),
        created_at: timestamp(),
        updated_at: timestamp(),
        sensitivity: WorkReportSensitivity::Confidential,
        redaction: RedactionMetadata::empty(),
    }
}

fn catalog_version() -> HostedProjectCatalogVersion {
    HostedProjectCatalogVersion::new(
        scope(),
        workflow_id(),
        WorkflowVersion::new("v1").expect("valid workflow version"),
        WorkflowCatalogRecord::new(catalog_record_definition()).expect("valid catalog record"),
        actor_id(),
        stewardship_id(),
        timestamp(),
    )
    .expect("valid hosted catalog version")
}

#[test]
fn scope_preserves_exact_project_boundary_and_round_trips() {
    let scope = scope();

    scope.validate().expect("scope remains valid");
    assert_eq!(scope.organization_id().as_str(), "org/hosted-test");
    assert_eq!(scope.project_id().as_str(), "project/hosted-test");

    let serialized = serde_json::to_string(&scope).expect("scope serializes");
    let restored: HostedProjectScope =
        serde_json::from_str(&serialized).expect("valid scope deserializes");
    assert_eq!(restored, scope);
}

#[test]
fn grants_sort_capabilities_and_authorize_only_declared_capabilities() {
    let grant = grant(
        project_id(),
        vec![
            HostedProjectCapability::RunRead,
            HostedProjectCapability::CatalogRead,
            HostedProjectCapability::ApprovalDecide,
        ],
    );

    assert_eq!(grant.project_id(), &project_id());
    assert_eq!(
        grant.capabilities(),
        &[
            HostedProjectCapability::CatalogRead,
            HostedProjectCapability::RunRead,
            HostedProjectCapability::ApprovalDecide,
        ]
    );
    assert!(grant.allows(HostedProjectCapability::RunRead));
    assert!(!grant.allows(HostedProjectCapability::RunCancel));
}

#[test]
fn grant_rejects_empty_and_duplicate_capability_sets() {
    let empty = HostedProjectGrant::new(project_id(), Vec::new()).expect_err("empty rejected");
    assert_eq!(empty.code(), "hosted_project.grant.capabilities.invalid");

    let duplicate = HostedProjectGrant::new(
        project_id(),
        vec![
            HostedProjectCapability::RunRead,
            HostedProjectCapability::RunRead,
        ],
    )
    .expect_err("duplicate rejected");
    assert_eq!(
        duplicate.code(),
        "hosted_project.grant.capabilities.duplicate"
    );
}

#[test]
fn principal_binding_is_project_exact_and_rejects_duplicate_grants() {
    let other_project = ProjectId::new("project/other").expect("valid project id");
    let binding = HostedPrincipalBinding::new(
        actor_id(),
        organization_id(),
        HostedPrincipalKind::Human,
        vec![
            grant(
                other_project.clone(),
                vec![HostedProjectCapability::CatalogRead],
            ),
            grant(project_id(), vec![HostedProjectCapability::RunCreate]),
        ],
    )
    .expect("valid principal binding");

    assert_eq!(binding.principal_kind(), HostedPrincipalKind::Human);
    assert!(binding
        .grants()
        .windows(2)
        .all(|pair| pair[0].project_id() < pair[1].project_id()));
    assert!(binding.allows(&project_id(), HostedProjectCapability::RunCreate));
    assert!(!binding.allows(&project_id(), HostedProjectCapability::CatalogRead));
    assert!(!binding.allows(&other_project, HostedProjectCapability::RunCreate));

    let duplicate = HostedPrincipalBinding::new(
        actor_id(),
        organization_id(),
        HostedPrincipalKind::Service,
        vec![
            grant(project_id(), vec![HostedProjectCapability::RunRead]),
            grant(project_id(), vec![HostedProjectCapability::RunCancel]),
        ],
    )
    .expect_err("duplicate project grants rejected");
    assert_eq!(
        duplicate.code(),
        "hosted_project.principal.grants.duplicate"
    );
}

#[test]
fn resource_binding_reserves_then_activates_without_changing_identity() {
    let reserved = HostedProjectResourceBinding::new(
        scope(),
        HostedProjectResourceKind::Run,
        "run/hosted-1",
        HostedProjectResourceBindingStatus::Reserved,
        timestamp(),
    )
    .expect("valid resource binding");

    let active = reserved.activate().expect("binding activates");
    assert_eq!(
        reserved.status(),
        HostedProjectResourceBindingStatus::Reserved
    );
    assert_eq!(active.status(), HostedProjectResourceBindingStatus::Active);
    assert_eq!(active.scope(), reserved.scope());
    assert_eq!(active.resource_kind(), HostedProjectResourceKind::Run);
    assert_eq!(active.resource_id(), "run/hosted-1");
    assert_eq!(
        HostedProjectResourceKind::WorkItem.storage_key(),
        "work_item"
    );
}

#[test]
fn resource_binding_rejects_unbounded_or_control_character_references() {
    for invalid in [
        String::new(),
        "run/secret\nvalue".to_owned(),
        "x".repeat(257),
    ] {
        let error = HostedProjectResourceBinding::new(
            scope(),
            HostedProjectResourceKind::ExecutionReceipt,
            invalid,
            HostedProjectResourceBindingStatus::Active,
            timestamp(),
        )
        .expect_err("invalid reference rejected");
        assert_eq!(error.code(), "hosted_project.resource.reference.invalid");
        assert!(!error.to_string().contains("secret"));
    }
}

#[test]
fn access_decision_preserves_bounded_audit_identity_and_round_trips() {
    let decision = HostedProjectAccessDecision::new(
        EventId::new("event/hosted-access-1").expect("valid event id"),
        actor_id(),
        HostedPrincipalKind::Service,
        scope(),
        HostedProjectCapability::ReportRead,
        false,
        "capability_not_granted",
        HostedProjectResourceKind::Report,
        "report/hosted-1",
        None,
        timestamp(),
    )
    .expect("valid access decision");

    assert_eq!(decision.decision_id().as_str(), "event/hosted-access-1");
    assert_eq!(decision.scope(), &scope());

    let serialized = serde_json::to_string(&decision).expect("decision serializes");
    let restored: HostedProjectAccessDecision =
        serde_json::from_str(&serialized).expect("valid decision deserializes");
    assert_eq!(restored, decision);
}

#[test]
fn access_decision_rejects_invalid_reason_and_target_without_leakage() {
    for (reason, target) in [("", "report/1"), ("allowed", "report/secret\nvalue")] {
        let error = HostedProjectAccessDecision::new(
            EventId::new("event/hosted-access-invalid").expect("valid event id"),
            actor_id(),
            HostedPrincipalKind::Human,
            scope(),
            HostedProjectCapability::ReportRead,
            false,
            reason,
            HostedProjectResourceKind::Report,
            target,
            None,
            timestamp(),
        )
        .expect_err("invalid decision rejected");
        assert_eq!(error.code(), "hosted_project.resource.reference.invalid");
        assert!(!error.to_string().contains("secret"));
    }
}

#[test]
fn catalog_version_requires_matching_identity_and_governance_metadata() {
    let valid = catalog_version();
    assert_eq!(valid.scope(), &scope());
    assert_eq!(valid.workflow_id(), &workflow_id());
    assert_eq!(valid.workflow_version().as_str(), "v1");
    assert_eq!(valid.record().workflow_id(), valid.workflow_id());
    assert_eq!(valid.published_by(), &actor_id());

    let mismatched = HostedProjectCatalogVersion::new(
        scope(),
        WorkflowId::new("hosted/different-workflow").expect("valid workflow id"),
        WorkflowVersion::new("v1").expect("valid workflow version"),
        WorkflowCatalogRecord::new(catalog_record_definition()).expect("valid catalog record"),
        actor_id(),
        stewardship_id(),
        timestamp(),
    )
    .expect_err("mismatched workflow identity rejected");
    assert_eq!(
        mismatched.code(),
        "hosted_project.catalog.governance.invalid"
    );

    let mut missing_owner = catalog_record_definition();
    missing_owner.owner = None;
    let missing_owner = HostedProjectCatalogVersion::new(
        scope(),
        workflow_id(),
        WorkflowVersion::new("v1").expect("valid workflow version"),
        WorkflowCatalogRecord::new(missing_owner).expect("catalog permits absent owner"),
        actor_id(),
        stewardship_id(),
        timestamp(),
    )
    .expect_err("missing hosted governance owner rejected");
    assert_eq!(
        missing_owner.code(),
        "hosted_project.catalog.governance.invalid"
    );
}

#[test]
fn serde_reconstruction_fails_closed_for_invalid_and_duplicate_state() {
    let duplicate_grant = json!({
        "project_id": "project/hosted-test",
        "capabilities": ["run_read", "run_read"]
    });
    let grant_error = serde_json::from_value::<HostedProjectGrant>(duplicate_grant)
        .expect_err("duplicate serialized capability rejected");
    assert_eq!(grant_error.to_string(), "invalid hosted project grant");

    let duplicate_principal = json!({
        "actor_id": "user/hosted-operator",
        "organization_id": "org/hosted-test",
        "principal_kind": "human",
        "grants": [
            {"project_id": "project/hosted-test", "capabilities": ["run_read"]},
            {"project_id": "project/hosted-test", "capabilities": ["run_cancel"]}
        ]
    });
    let principal_error = serde_json::from_value::<HostedPrincipalBinding>(duplicate_principal)
        .expect_err("duplicate serialized project grant rejected");
    assert_eq!(
        principal_error.to_string(),
        "invalid hosted principal binding"
    );

    let mut resource = serde_json::to_value(
        HostedProjectResourceBinding::new(
            scope(),
            HostedProjectResourceKind::Report,
            "report/hosted-1",
            HostedProjectResourceBindingStatus::Active,
            timestamp(),
        )
        .expect("valid resource binding"),
    )
    .expect("resource serializes");
    resource["resource_id"] = Value::String(String::new());
    assert!(serde_json::from_value::<HostedProjectResourceBinding>(resource).is_err());

    let mut decision = serde_json::to_value(
        HostedProjectAccessDecision::new(
            EventId::new("event/hosted-access-serde").expect("valid event id"),
            actor_id(),
            HostedPrincipalKind::Human,
            scope(),
            HostedProjectCapability::RunRead,
            true,
            "capability_granted",
            HostedProjectResourceKind::Run,
            "run/hosted-1",
            None,
            timestamp(),
        )
        .expect("valid decision"),
    )
    .expect("decision serializes");
    decision["reason_code"] = Value::String("secret\nreason".to_owned());
    let decision_error = serde_json::from_value::<HostedProjectAccessDecision>(decision)
        .expect_err("invalid serialized decision rejected");
    assert_eq!(
        decision_error.to_string(),
        "invalid hosted project access decision"
    );
    assert!(!decision_error.to_string().contains("secret"));
}

#[test]
fn serialized_catalog_version_fails_closed_when_governance_is_removed() {
    let mut serialized = serde_json::to_value(catalog_version()).expect("catalog serializes");
    serialized["record"]["owner"] = Value::Null;

    let error = serde_json::from_value::<HostedProjectCatalogVersion>(serialized)
        .expect_err("catalog without owner rejected");
    assert!(!error.to_string().contains("hosted-operator"));
}

#[test]
fn debug_output_redacts_hosted_boundary_identities_and_references() {
    let grant = grant(project_id(), vec![HostedProjectCapability::RunRead]);
    let principal = HostedPrincipalBinding::new(
        actor_id(),
        organization_id(),
        HostedPrincipalKind::Human,
        vec![grant.clone()],
    )
    .expect("valid principal");
    let resource = HostedProjectResourceBinding::new(
        scope(),
        HostedProjectResourceKind::Run,
        "run/secret-target",
        HostedProjectResourceBindingStatus::Active,
        timestamp(),
    )
    .expect("valid resource");
    let decision = HostedProjectAccessDecision::new(
        EventId::new("event/secret-decision").expect("valid event id"),
        actor_id(),
        HostedPrincipalKind::Human,
        scope(),
        HostedProjectCapability::RunRead,
        true,
        "secret_reason_reference",
        HostedProjectResourceKind::Run,
        "run/secret-target",
        None,
        timestamp(),
    )
    .expect("valid decision");

    for debug in [
        format!("{:?}", scope()),
        format!("{grant:?}"),
        format!("{principal:?}"),
        format!("{resource:?}"),
        format!("{decision:?}"),
        format!("{:?}", catalog_version()),
    ] {
        assert!(debug.contains("[REDACTED]"), "missing redaction: {debug}");
        assert!(!debug.contains("hosted-test"), "scope leaked: {debug}");
        assert!(!debug.contains("secret"), "reference leaked: {debug}");
        assert!(!debug.contains("hosted-operator"), "actor leaked: {debug}");
    }
}
