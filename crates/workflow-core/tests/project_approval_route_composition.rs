//! Authenticated project approval route composition tests.

#![allow(clippy::expect_used)]

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use workflow_core::{
    build_immutable_run_bundle, compose_authenticated_project_approval_route, load_project,
    ActorId, ApprovalReferenceId, ApprovalRequest, CorrelationId, EscalationRecord, EventId,
    EventLogStore, EventSequenceNumber, FailureClass, HostedAuthorityRegistryRevision,
    HostedPrincipalBinding, HostedPrincipalKind, HostedPrincipalRegistry, HostedProjectCapability,
    HostedProjectGrant, HostedProjectResourceBinding, HostedProjectResourceBindingReader,
    HostedProjectResourceBindingStatus, HostedProjectResourceKind, HostedProjectScope,
    IdempotencyKey, ImmutableRunBundleBuildRequest, ImmutableRunBundleExecutionPosture,
    ImmutableRunBundleHandlerPosture, ImmutableRunBundleHandlerReference, ImmutableRunBundleId,
    ImmutableRunBundlePublishOutcome, ImmutableRunBundleReferencePosture,
    ImmutableRunBundleSensitivity, ImmutableRunBundleStore, ImmutableRunBundleVersion,
    InMemoryProjectApprovalRouteStoreFixture, LocalImmutableRunBundleStore, OrganizationId,
    ProjectApprovalAuthoritySnapshotCommitment, ProjectApprovalAuthorityViewCommitment,
    ProjectApprovalRouteAuthenticatedCompositionRequest, ProjectApprovalRouteCreateResult,
    ProjectApprovalRoutingReason, ProjectId, SkillId, SkillVersion, SpecContentHash, StepId,
    Timestamp, WorkflowOsError, WorkflowRunEvent, WorkflowRunEventKind, WorkflowRunId,
};

static NEXT_ROOT: AtomicU64 = AtomicU64::new(1);

struct TestRoot(PathBuf);

impl TestRoot {
    fn new() -> Self {
        let id = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "workflow-os-project-route-composition-{}-{id}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("test root");
        Self(path)
    }
}

impl Drop for TestRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[derive(Clone)]
struct EventFixture(Vec<WorkflowRunEvent>);

impl EventLogStore for EventFixture {
    fn append_event(&self, _event: &WorkflowRunEvent) -> Result<(), WorkflowOsError> {
        Err(WorkflowOsError::invalid_state(
            "test.fixture.read_only",
            "composition fixture is read-only",
        ))
    }

    fn read_events(
        &self,
        _run_id: &WorkflowRunId,
    ) -> Result<Vec<WorkflowRunEvent>, WorkflowOsError> {
        Ok(self.0.clone())
    }
}

#[derive(Clone)]
struct BindingFixture(Option<HostedProjectResourceBinding>);

impl HostedProjectResourceBindingReader for BindingFixture {
    fn read_project_resource_binding(
        &self,
        _kind: HostedProjectResourceKind,
        _resource_id: &str,
    ) -> Result<Option<HostedProjectResourceBinding>, WorkflowOsError> {
        Ok(self.0.clone())
    }
}

struct CompositionFixture {
    _storage: TestRoot,
    event_log: EventFixture,
    bundle_store: LocalImmutableRunBundleStore,
    binding: BindingFixture,
    route_store: InMemoryProjectApprovalRouteStoreFixture,
    scope: HostedProjectScope,
    run_id: WorkflowRunId,
    bundle_id: ImmutableRunBundleId,
    approval_id: ApprovalReferenceId,
    registry: HostedPrincipalRegistry,
    authority: ProjectApprovalAuthoritySnapshotCommitment,
}

impl CompositionFixture {
    fn new() -> Self {
        let storage = TestRoot::new();
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/vertical-slice-approval");
        let loaded = load_project(project_root);
        assert!(!loaded.has_errors(), "{:?}", loaded.diagnostics);
        let project = loaded.bundle.expect("project");
        let workflow_id = workflow_core::WorkflowId::new("ex/review").expect("workflow");
        let run_id = WorkflowRunId::new("run-project-route-composition").expect("run");
        let bundle_id =
            ImmutableRunBundleId::new("bundle/project-route-composition").expect("bundle");
        let context_hash = SpecContentHash::from_text("project route composition context");
        let build = build_immutable_run_bundle(ImmutableRunBundleBuildRequest {
            project: &project,
            workflow_id: &workflow_id,
            bundle_id: bundle_id.clone(),
            bundle_version: ImmutableRunBundleVersion::new("v1").expect("bundle version"),
            run_id: run_id.clone(),
            resolved_execution_context_hash: context_hash.clone(),
            execution_posture: ImmutableRunBundleExecutionPosture::new(
                Vec::new(),
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::NotSupplied,
            )
            .expect("posture"),
            handlers: vec![ImmutableRunBundleHandlerReference {
                skill_id: SkillId::new("local/rec").expect("skill"),
                skill_version: SkillVersion::new("v0").expect("skill version"),
                posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
            }],
            created_at: time("2026-08-13T10:00:00Z"),
            created_by: ActorId::new("system/kernel").expect("actor"),
            sensitivity: ImmutableRunBundleSensitivity::Internal,
            redaction_required: true,
        })
        .expect("bundle");
        let bundle_store = LocalImmutableRunBundleStore::new(&storage.0);
        assert_eq!(
            bundle_store
                .publish_bundle_create_only(&build)
                .expect("bundle published"),
            ImmutableRunBundlePublishOutcome::Published
        );
        let approval_id = ApprovalReferenceId::new("approval/run-project-route-composition/rec")
            .expect("approval id");
        let approval = ApprovalRequest {
            approval_id: approval_id.as_str().to_owned(),
            run_id: run_id.clone(),
            workflow_id: workflow_id.clone(),
            schema_version: build.manifest().schema_version().clone(),
            workflow_version: build.manifest().workflow_version().clone(),
            spec_content_hash: build.manifest().workflow_content_hash().clone(),
            resolved_execution_context_hash: Some(context_hash),
            step_id: Some(StepId::new("rec").expect("step")),
            skill_id: Some(SkillId::new("local/rec").expect("skill")),
            skill_version: Some(SkillVersion::new("v0").expect("skill version")),
            governance_approval_binding: None,
            requested_by: ActorId::new("system/kernel").expect("actor"),
            correlation_id: CorrelationId::new("correlation/project-route").expect("correlation"),
            idempotency_key: Some(IdempotencyKey::new("project-route-key").expect("key")),
            reason: "bounded approval reason".to_owned(),
            requested_at: time("2026-08-13T10:01:00Z"),
            expires_after: None,
            expires_at: None,
            decision: None,
        };
        let events = vec![
            event(
                &build,
                1,
                "event/project-route-created",
                WorkflowRunEventKind::RunCreated {
                    summary: None,
                    immutable_run_bundle: Some(build.manifest().run_binding()),
                },
            ),
            event(
                &build,
                2,
                "event/project-route-approval",
                WorkflowRunEventKind::ApprovalRequested(Box::new(approval)),
            ),
        ];
        let scope = project_scope();
        let (registry, authority) = authority_fixture(&scope);
        let binding = active_binding(&scope, &run_id);
        Self {
            _storage: storage,
            event_log: EventFixture(events),
            bundle_store,
            binding: BindingFixture(Some(binding)),
            route_store: InMemoryProjectApprovalRouteStoreFixture::default(),
            scope,
            run_id,
            bundle_id,
            approval_id,
            registry,
            authority,
        }
    }

    fn request(&self) -> ProjectApprovalRouteAuthenticatedCompositionRequest<'_> {
        ProjectApprovalRouteAuthenticatedCompositionRequest {
            scope: &self.scope,
            run_id: &self.run_id,
            bundle_id: &self.bundle_id,
            approval_id: &self.approval_id,
            routing_reason: ProjectApprovalRoutingReason::WorkflowMaintainer,
            escalation_id: None,
            authority_registry: &self.registry,
            authority_snapshot: &self.authority,
            resolved_at: time("2026-08-13T10:02:00Z"),
        }
    }
}

fn project_scope() -> HostedProjectScope {
    HostedProjectScope::new(
        OrganizationId::new("org/project-route").expect("organization"),
        ProjectId::new("project/route").expect("project"),
    )
}

fn active_binding(
    scope: &HostedProjectScope,
    run_id: &WorkflowRunId,
) -> HostedProjectResourceBinding {
    HostedProjectResourceBinding::new(
        scope.clone(),
        HostedProjectResourceKind::Run,
        run_id.as_str(),
        HostedProjectResourceBindingStatus::Active,
        time("2026-08-13T10:00:00Z"),
    )
    .expect("binding")
}

fn authority_fixture(
    scope: &HostedProjectScope,
) -> (
    HostedPrincipalRegistry,
    ProjectApprovalAuthoritySnapshotCommitment,
) {
    let principal = HostedPrincipalBinding::new(
        ActorId::new("workflow-os").expect("maintainer"),
        scope.organization_id().clone(),
        HostedPrincipalKind::Human,
        vec![HostedProjectGrant::new(
            scope.project_id().clone(),
            vec![
                HostedProjectCapability::ApprovalRead,
                HostedProjectCapability::ApprovalDecide,
            ],
        )
        .expect("grant")],
    )
    .expect("principal");
    let registry = HostedPrincipalRegistry::new(scope.organization_id().clone(), vec![principal])
        .expect("registry");
    let view =
        ProjectApprovalAuthorityViewCommitment::from_registry(scope, &registry).expect("view");
    let authority = ProjectApprovalAuthoritySnapshotCommitment::new(
        HostedAuthorityRegistryRevision::new(7).expect("revision"),
        view,
    );
    (registry, authority)
}

fn event(
    build: &workflow_core::ImmutableRunBundleBuildResult,
    sequence: u64,
    event_id: &str,
    kind: WorkflowRunEventKind,
) -> WorkflowRunEvent {
    WorkflowRunEvent {
        sequence_number: EventSequenceNumber::new(sequence).expect("sequence"),
        event_id: EventId::new(event_id).expect("event"),
        timestamp: time("2026-08-13T10:01:00Z"),
        run_id: build.manifest().run_id().clone(),
        workflow_id: build.manifest().workflow_id().clone(),
        schema_version: build.manifest().schema_version().clone(),
        workflow_version: build.manifest().workflow_version().clone(),
        spec_content_hash: build.manifest().workflow_content_hash().clone(),
        correlation_id: Some(CorrelationId::new("correlation/project-route").expect("correlation")),
        actor: Some(ActorId::new("system/kernel").expect("actor")),
        idempotency_key: None,
        kind,
    }
}

fn time(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).expect("timestamp")
}

#[test]
fn composes_ordinary_route_from_durable_frozen_sources() {
    let fixture = CompositionFixture::new();
    let result = compose_authenticated_project_approval_route(
        &fixture.event_log,
        &fixture.bundle_store,
        &fixture.binding,
        &fixture.route_store,
        &fixture.request(),
    )
    .expect("route composed");

    assert!(matches!(
        result,
        ProjectApprovalRouteCreateResult::Created(_)
    ));
    assert_eq!(
        result.record().route().recipient().map(ActorId::as_str),
        Some("workflow-os")
    );
    assert_eq!(
        result
            .record()
            .source_commitment()
            .authority_registry_revision()
            .get(),
        7
    );
}

#[test]
fn composes_escalation_route_only_from_exact_durable_escalation_event() {
    let mut fixture = CompositionFixture::new();
    let mut escalation_event = fixture.event_log.0[0].clone();
    escalation_event.sequence_number = EventSequenceNumber::new(3).expect("sequence");
    escalation_event.event_id = EventId::new("event/project-route-escalation").expect("event id");
    escalation_event.kind = WorkflowRunEventKind::EscalationTriggered(EscalationRecord {
        escalation_id: "escalation/project-route".to_owned(),
        run_id: fixture.run_id.clone(),
        step_id: Some(StepId::new("rec").expect("step")),
        skill_id: Some(SkillId::new("local/rec").expect("skill")),
        skill_version: Some(SkillVersion::new("v0").expect("skill version")),
        attempts: 1,
        last_error: "runtime.failure".to_owned(),
        failure_class: FailureClass::Unknown,
        suggested_next_action: "manual review".to_owned(),
        reason: "bounded escalation".to_owned(),
        contact: Some(ActorId::new("workflow-os").expect("contact")),
    });
    fixture.event_log.0.push(escalation_event);
    let mut request = fixture.request();
    request.routing_reason = ProjectApprovalRoutingReason::WorkflowEscalationContact;
    request.escalation_id = Some("escalation/project-route");

    let result = compose_authenticated_project_approval_route(
        &fixture.event_log,
        &fixture.bundle_store,
        &fixture.binding,
        &fixture.route_store,
        &request,
    )
    .expect("escalation route composed");

    assert_eq!(
        result.record().route().escalation_id(),
        Some("escalation/project-route")
    );
    assert_eq!(
        result.record().route().recipient().map(ActorId::as_str),
        Some("workflow-os")
    );
}

#[test]
fn missing_approval_event_fails_closed_without_route() {
    let mut fixture = CompositionFixture::new();
    fixture.event_log.0.truncate(1);
    let error = compose_authenticated_project_approval_route(
        &fixture.event_log,
        &fixture.bundle_store,
        &fixture.binding,
        &fixture.route_store,
        &fixture.request(),
    )
    .expect_err("missing approval event rejected");

    assert_eq!(
        error.code(),
        "project_approval_route_composition.approval_event.missing"
    );
}

#[test]
fn inactive_or_cross_project_binding_fails_closed() {
    let mut fixture = CompositionFixture::new();
    fixture.binding = BindingFixture(Some(
        HostedProjectResourceBinding::new(
            HostedProjectScope::new(
                fixture.scope.organization_id().clone(),
                ProjectId::new("project/other").expect("project"),
            ),
            HostedProjectResourceKind::Run,
            fixture.run_id.as_str(),
            HostedProjectResourceBindingStatus::Active,
            time("2026-08-13T10:00:00Z"),
        )
        .expect("binding"),
    ));
    let error = compose_authenticated_project_approval_route(
        &fixture.event_log,
        &fixture.bundle_store,
        &fixture.binding,
        &fixture.route_store,
        &fixture.request(),
    )
    .expect_err("cross-project binding rejected");

    assert_eq!(
        error.code(),
        "project_approval_route_composition.run_binding.invalid"
    );
}

#[test]
fn authority_registry_and_snapshot_cannot_diverge() {
    let mut fixture = CompositionFixture::new();
    fixture.authority = ProjectApprovalAuthoritySnapshotCommitment::new(
        HostedAuthorityRegistryRevision::new(7).expect("revision"),
        ProjectApprovalAuthorityViewCommitment::from_registry(
            &fixture.scope,
            &HostedPrincipalRegistry::new(fixture.scope.organization_id().clone(), Vec::new())
                .expect("empty registry"),
        )
        .expect("view"),
    );
    let error = compose_authenticated_project_approval_route(
        &fixture.event_log,
        &fixture.bundle_store,
        &fixture.binding,
        &fixture.route_store,
        &fixture.request(),
    )
    .expect_err("divergent authority rejected");

    assert_eq!(
        error.code(),
        "project_approval_route_composition.authority.invalid"
    );
}
