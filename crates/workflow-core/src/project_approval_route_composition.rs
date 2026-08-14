use crate::{
    resolve_project_approval_route, ApprovalDecisionKind, ApprovalReferenceId, ApprovalRequest,
    EventLogStore, HostedPrincipalRegistry, HostedProjectResourceBinding,
    HostedProjectResourceBindingStatus, HostedProjectResourceKind, HostedProjectScope,
    ImmutableRunBundleId, ImmutableRunBundleStore, ProjectApprovalAuthoritySnapshotCommitment,
    ProjectApprovalRouteCreateResult, ProjectApprovalRouteInput, ProjectApprovalRouteRecord,
    ProjectApprovalRouteSourceCommitment, ProjectApprovalRouteSourceCommitmentInput,
    ProjectApprovalRouteStore, ProjectApprovalRoutingReason, Timestamp, WorkflowOsError,
    WorkflowRunEvent, WorkflowRunEventKind, WorkflowRunId,
};

/// Trusted lookup boundary for one exact hosted project resource binding.
pub trait HostedProjectResourceBindingReader: Send + Sync {
    /// Reads one exact resource binding without widening its project scope.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when storage is unavailable or corrupt.
    fn read_project_resource_binding(
        &self,
        kind: HostedProjectResourceKind,
        resource_id: &str,
    ) -> Result<Option<HostedProjectResourceBinding>, WorkflowOsError>;
}

/// Stable subjects and canonical authority used to compose one approval route.
pub struct ProjectApprovalRouteAuthenticatedCompositionRequest<'a> {
    /// Exact deployment project scope.
    pub scope: &'a HostedProjectScope,
    /// Governed run identity.
    pub run_id: &'a WorkflowRunId,
    /// Immutable bundle identity bound when the run was created.
    pub bundle_id: &'a ImmutableRunBundleId,
    /// Pending approval identity.
    pub approval_id: &'a ApprovalReferenceId,
    /// Frozen ownership field to resolve.
    pub routing_reason: ProjectApprovalRoutingReason,
    /// Exact escalation identity for escalation-contact routing.
    pub escalation_id: Option<&'a str>,
    /// Complete deployment-owned authority registry.
    pub authority_registry: &'a HostedPrincipalRegistry,
    /// Revision-bound commitment to that same registry.
    pub authority_snapshot: &'a ProjectApprovalAuthoritySnapshotCommitment,
    /// Caller-supplied deterministic resolution timestamp.
    pub resolved_at: Timestamp,
}

/// Composes and persists one authenticated project approval route from durable facts.
///
/// The function accepts lookup subjects rather than caller-authored route content. The route store
/// remains responsible for atomically rechecking mutable approval, project-binding, and authority
/// facts before create-only persistence.
///
/// # Errors
///
/// Fails closed when durable history, immutable bundle content, project binding, escalation proof,
/// or canonical authority does not form one coherent pending approval context.
pub fn compose_authenticated_project_approval_route(
    event_log: &dyn EventLogStore,
    bundles: &dyn ImmutableRunBundleStore,
    bindings: &dyn HostedProjectResourceBindingReader,
    routes: &dyn ProjectApprovalRouteStore,
    request: &ProjectApprovalRouteAuthenticatedCompositionRequest<'_>,
) -> Result<ProjectApprovalRouteCreateResult, WorkflowOsError> {
    request.scope.validate()?;
    validate_authority(request)?;

    let events = event_log.read_events(request.run_id)?;
    let approval_context = reconstruct_pending_approval(&events, request.approval_id)?;
    let run_bundle_binding = immutable_bundle_binding(&events)?;
    if run_bundle_binding.bundle_id() != request.bundle_id {
        return Err(composition_error(
            "project_approval_route_composition.bundle_binding.mismatch",
            "project approval route immutable bundle binding is invalid",
        ));
    }
    let bundle = bundles.read_exact_bundle(request.run_id, request.bundle_id)?;
    if bundle.manifest().root_hash() != run_bundle_binding.root_hash() {
        return Err(composition_error(
            "project_approval_route_composition.bundle_binding.mismatch",
            "project approval route immutable bundle binding is invalid",
        ));
    }
    let workflow = bundle
        .definition_records()
        .iter()
        .filter_map(|record| record.canonical_definition().as_workflow())
        .find(|definition| {
            definition.id == approval_context.request.workflow_id
                && definition.version == approval_context.request.workflow_version
        })
        .ok_or_else(|| {
            composition_error(
                "project_approval_route_composition.workflow_definition.missing",
                "project approval route workflow definition is unavailable",
            )
        })?;
    let run_binding = bindings
        .read_project_resource_binding(HostedProjectResourceKind::Run, request.run_id.as_str())?
        .ok_or_else(|| {
            composition_error(
                "project_approval_route_composition.run_binding.missing",
                "project approval route run binding is unavailable",
            )
        })?;
    if run_binding.scope() != request.scope
        || run_binding.status() != HostedProjectResourceBindingStatus::Active
    {
        return Err(composition_error(
            "project_approval_route_composition.run_binding.invalid",
            "project approval route run binding is invalid",
        ));
    }

    let escalation_context =
        reconstruct_escalation(&events, request.routing_reason, request.escalation_id)?;
    let route = resolve_project_approval_route(&ProjectApprovalRouteInput {
        scope: request.scope,
        run_binding: &run_binding,
        approval: approval_context.request,
        ownership: &workflow.owner,
        routing_reason: request.routing_reason,
        escalation: escalation_context.as_ref().map(|context| context.record),
        principals: request.authority_registry.principals(),
        resolved_at: request.resolved_at,
    })?;
    let source = ProjectApprovalRouteSourceCommitment::new(
        &route,
        &ProjectApprovalRouteSourceCommitmentInput {
            approval: approval_context.request,
            approval_request_event_id: &approval_context.event.event_id,
            immutable_run_bundle: bundle.manifest(),
            run_binding: &run_binding,
            escalation_event_id: escalation_context
                .as_ref()
                .map(|context| &context.event.event_id),
            authority_snapshot: request.authority_snapshot,
        },
    )?;
    routes.create_project_approval_route(ProjectApprovalRouteRecord::new(
        route,
        source,
        request.resolved_at,
    )?)
}

struct ApprovalEventContext<'a> {
    event: &'a WorkflowRunEvent,
    request: &'a ApprovalRequest,
}

struct EscalationEventContext<'a> {
    event: &'a WorkflowRunEvent,
    record: &'a crate::EscalationRecord,
}

fn validate_authority(
    request: &ProjectApprovalRouteAuthenticatedCompositionRequest<'_>,
) -> Result<(), WorkflowOsError> {
    if request.authority_registry.organization_id() != request.scope.organization_id()
        || request
            .authority_snapshot
            .authority_view()
            .organization_id()
            != request.scope.organization_id()
        || request.authority_snapshot.authority_view()
            != &crate::ProjectApprovalAuthorityViewCommitment::from_registry(
                request.scope,
                request.authority_registry,
            )?
    {
        return Err(composition_error(
            "project_approval_route_composition.authority.invalid",
            "project approval route authority snapshot is invalid",
        ));
    }
    Ok(())
}

fn reconstruct_pending_approval<'a>(
    events: &'a [WorkflowRunEvent],
    approval_id: &ApprovalReferenceId,
) -> Result<ApprovalEventContext<'a>, WorkflowOsError> {
    let mut requested = None;
    let mut decided = false;
    for event in events {
        match &event.kind {
            WorkflowRunEventKind::ApprovalRequested(candidate)
                if candidate.approval_id == approval_id.as_str()
                    && requested.replace((event, candidate.as_ref())).is_some() =>
            {
                return Err(composition_error(
                    "project_approval_route_composition.approval_history.invalid",
                    "project approval route approval history is invalid",
                ));
            }
            WorkflowRunEventKind::ApprovalGranted(decision)
            | WorkflowRunEventKind::ApprovalDenied(decision)
                if decision.approval_id == approval_id.as_str() =>
            {
                decided = matches!(
                    decision.decision,
                    ApprovalDecisionKind::Granted | ApprovalDecisionKind::Denied
                );
            }
            _ => {}
        }
    }
    let (event, request) = requested.ok_or_else(|| {
        composition_error(
            "project_approval_route_composition.approval_event.missing",
            "project approval route approval request event is unavailable",
        )
    })?;
    if decided || request.decision.is_some() {
        return Err(composition_error(
            "project_approval_route_composition.approval.not_pending",
            "project approval route requires a pending approval",
        ));
    }
    Ok(ApprovalEventContext { event, request })
}

fn immutable_bundle_binding(
    events: &[WorkflowRunEvent],
) -> Result<&crate::ImmutableRunBundleBinding, WorkflowOsError> {
    events
        .iter()
        .find_map(|event| match &event.kind {
            WorkflowRunEventKind::RunCreated {
                immutable_run_bundle: Some(binding),
                ..
            } => Some(binding),
            _ => None,
        })
        .ok_or_else(|| {
            composition_error(
                "project_approval_route_composition.bundle_binding.missing",
                "project approval route immutable bundle binding is unavailable",
            )
        })
}

fn reconstruct_escalation<'a>(
    events: &'a [WorkflowRunEvent],
    reason: ProjectApprovalRoutingReason,
    escalation_id: Option<&str>,
) -> Result<Option<EscalationEventContext<'a>>, WorkflowOsError> {
    match (reason, escalation_id) {
        (ProjectApprovalRoutingReason::WorkflowMaintainer, None) => Ok(None),
        (ProjectApprovalRoutingReason::WorkflowEscalationContact, Some(escalation_id)) => {
            let mut found = events.iter().filter_map(|event| match &event.kind {
                WorkflowRunEventKind::EscalationTriggered(record)
                    if record.escalation_id == escalation_id =>
                {
                    Some(EscalationEventContext { event, record })
                }
                _ => None,
            });
            let context = found.next().ok_or_else(|| {
                composition_error(
                    "project_approval_route_composition.escalation_event.missing",
                    "project approval route escalation event is unavailable",
                )
            })?;
            if found.next().is_some() {
                return Err(composition_error(
                    "project_approval_route_composition.escalation_history.invalid",
                    "project approval route escalation history is invalid",
                ));
            }
            Ok(Some(context))
        }
        _ => Err(composition_error(
            "project_approval_route_composition.escalation.invalid",
            "project approval route escalation subject is invalid",
        )),
    }
}

fn composition_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::invalid_state(code, message)
}
