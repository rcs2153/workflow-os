//! Single-tenant hosted alpha transport and no-write worker.
//!
//! This crate is intentionally deployment-bound, local to one trust domain,
//! and not a production, multi-tenant, or general agent runtime.

mod openshell;
mod openshell_cli;

pub use openshell::{
    OpenShellFixedOperationOutcome, OpenShellNoWriteClient, OpenShellNoWriteExecutionProvider,
    OpenShellSandboxSnapshot,
};
pub use openshell_cli::{
    OpenShellCliEffectivePolicy, OpenShellCliReconciledSnapshot, OpenShellCliSandboxState,
    OpenShellCliTransport, OpenShellCliTransportConfig, OPENSHELL_CLI_VERSION,
    OPENSHELL_UPSTREAM_COMMIT,
};

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path as FsPath, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{DefaultBodyLimit, Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use workflow_core::{
    decide_hosted_dispatch_approval_with_presentation, execute_with_hosted_no_write_dispatch,
    invoke_hosted_execution_provider, load_project, ActorId, ApprovalDecisionKind,
    ApprovalPresentationRecord, ApprovalPresentationRecordStore, ApprovalStore, CorrelationId,
    EventLogStore, HostedCatalogEntryId, HostedExecutionAttemptPosture, HostedExecutionBudget,
    HostedExecutionErrorCategory, HostedExecutionId, HostedExecutionInvocationError,
    HostedExecutionPolicyBinding, HostedExecutionPolicyId, HostedExecutionProvider,
    HostedExecutionProviderId, HostedExecutionProviderVersion, HostedExecutionReceipt,
    HostedExecutionReference, HostedExecutionReferenceKind, HostedExecutionRequest,
    HostedExecutionStatus, HostedNoWriteDispatchInputs, HostedPrincipalBinding,
    HostedProjectCapability, HostedProjectCatalogVersion, HostedProjectResourceBinding,
    HostedProjectResourceBindingStatus, HostedProjectResourceKind, HostedProjectScope,
    HostedTerminalReportArtifact, HostedTerminalResultProjection, HostedUnreceiptedOutcome,
    HostedUnreceiptedResultProjection, HostedWorkItem, HostedWorkItemId, HostedWorkItemStatus,
    IdempotencyKey, IdempotencyResult, IdempotencyStore, IdempotencyWrite, ImmutableRunBundleId,
    ImmutableRunBundleSensitivity, ImmutableRunBundleVersion, LocalApprovalDecisionRequest,
    LocalApprovalPresentationDecisionRequest, LocalApprovalPresentationProof,
    LocalCancellationRequest, LocalExecutionBeforeSkillInvocationCheckpointInputs,
    LocalExecutionImmutableRunBundleInputs, LocalExecutionRequest,
    LocalExecutionWithHostedDispatchRequest, LocalExecutionWithImmutableRunBundleRequest,
    LocalExecutor, LocalSkillRegistry, OrganizationId, PostgresClaimHostedWorkItemRequest,
    PostgresClaimedHostedWorkItem, PostgresCommitHostedReceiptProjectionRequest,
    PostgresCommitHostedReceiptRequest, PostgresCommitHostedUnreceiptedProjectionRequest,
    PostgresStateBackend, PostgresTransitionHostedWorkItemRequest, ProjectId, SpecContentHash,
    StateBackend, Timestamp, WorkReportArtifactMetadata, WorkReportArtifactStore, WorkReportId,
    WorkReportStatus, WorkflowId, WorkflowOsError, WorkflowRun, WorkflowRunEvent, WorkflowRunId,
    WorkflowRunStatus,
};

const MAX_API_BODY_BYTES: usize = 64 * 1024;
const MAX_EVENT_PAGE_SIZE: usize = 100;
const DEFAULT_PRESENTATION_MAX_AGE_SECONDS: u64 = 900;

/// Deployment-bound authentication token digest.
#[derive(Clone)]
pub struct HostedAuthTokenDigest([u8; 32]);

impl HostedAuthTokenDigest {
    /// Hashes one externally supplied token.
    ///
    /// # Errors
    ///
    /// Rejects empty, oversized, or control-character token material.
    pub fn from_token(token: &str) -> Result<Self, WorkflowOsError> {
        if token.is_empty() || token.len() > 4096 || token.chars().any(char::is_control) {
            return Err(WorkflowOsError::security(
                "hosted.auth.configuration.invalid",
                "hosted authentication configuration is invalid",
            ));
        }
        Ok(Self(Sha256::digest(token.as_bytes()).into()))
    }

    fn verify(&self, candidate: &str) -> bool {
        let candidate: [u8; 32] = Sha256::digest(candidate.as_bytes()).into();
        self.0.ct_eq(&candidate).into()
    }
}

impl fmt::Debug for HostedAuthTokenDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("HostedAuthTokenDigest([REDACTED])")
    }
}

/// Authenticated API configuration for one single-tenant deployment.
#[derive(Clone)]
pub struct HostedApiAuth {
    token_digest: HostedAuthTokenDigest,
    actor: ActorId,
}

impl HostedApiAuth {
    /// Creates one deployment-bound API authority.
    #[must_use]
    pub const fn new(token_digest: HostedAuthTokenDigest, actor: ActorId) -> Self {
        Self {
            token_digest,
            actor,
        }
    }

    fn authorize(&self, headers: &HeaderMap) -> Result<&ActorId, HostedApiError> {
        let value = headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .ok_or_else(HostedApiError::unauthorized)?;
        if !self.token_digest.verify(value) {
            return Err(HostedApiError::unauthorized());
        }
        Ok(&self.actor)
    }
}

impl fmt::Debug for HostedApiAuth {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedApiAuth")
            .field("token", &"[REDACTED]")
            .field("actor", &"[REDACTED]")
            .finish()
    }
}

/// Shared API state for one single-tenant deployment.
#[derive(Clone)]
pub struct HostedApiState {
    backend: PostgresStateBackend,
    auth: HostedApiAuth,
    build_id: String,
    project_root: PathBuf,
}

/// One deployment-owned project registration.
#[derive(Clone)]
pub struct HostedProjectRegistration {
    project_id: ProjectId,
    root: PathBuf,
}

impl HostedProjectRegistration {
    /// Creates one project registration with a canonical server-owned root.
    ///
    /// # Errors
    ///
    /// Rejects route-unsafe identities and missing or invalid roots.
    pub fn new(project_id: ProjectId, root: impl Into<PathBuf>) -> Result<Self, WorkflowOsError> {
        if project_id.as_str().contains('/') {
            return Err(WorkflowOsError::validation(
                "hosted.project_registry.id.path_unsafe",
                "hosted project identity is not route-safe",
            ));
        }
        let root = root.into().canonicalize().map_err(|_| {
            WorkflowOsError::validation(
                "hosted.project_registry.root.invalid",
                "hosted project root configuration is invalid",
            )
        })?;
        if !root.is_dir() {
            return Err(WorkflowOsError::validation(
                "hosted.project_registry.root.invalid",
                "hosted project root configuration is invalid",
            ));
        }
        Ok(Self { project_id, root })
    }
}

impl fmt::Debug for HostedProjectRegistration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedProjectRegistration")
            .field("project", &"[REDACTED]")
            .field("root", &"[REDACTED]")
            .finish()
    }
}

/// Immutable deployment project registry.
#[derive(Clone)]
pub struct HostedProjectRegistry {
    projects: BTreeMap<ProjectId, HostedProjectRegistration>,
}

impl HostedProjectRegistry {
    /// Creates a registry while rejecting duplicate, aliased, or nested roots.
    ///
    /// # Errors
    ///
    /// Rejects empty, oversized, duplicate, aliased, or nested registrations.
    pub fn new(registrations: Vec<HostedProjectRegistration>) -> Result<Self, WorkflowOsError> {
        if registrations.is_empty() || registrations.len() > 128 {
            return Err(WorkflowOsError::validation(
                "hosted.project_registry.invalid",
                "hosted project registry is invalid",
            ));
        }
        for (index, left) in registrations.iter().enumerate() {
            for right in registrations.iter().skip(index + 1) {
                if left.project_id == right.project_id || paths_overlap(&left.root, &right.root) {
                    return Err(WorkflowOsError::validation(
                        "hosted.project_registry.conflict",
                        "hosted project registry contains conflicting entries",
                    ));
                }
            }
        }
        Ok(Self {
            projects: registrations
                .into_iter()
                .map(|registration| (registration.project_id.clone(), registration))
                .collect(),
        })
    }

    fn get(&self, project_id: &ProjectId) -> Option<&HostedProjectRegistration> {
        self.projects.get(project_id)
    }

    fn contains(&self, project_id: &ProjectId) -> bool {
        self.projects.contains_key(project_id)
    }
}

impl fmt::Debug for HostedProjectRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedProjectRegistry")
            .field("project_count", &self.projects.len())
            .finish()
    }
}

fn paths_overlap(left: &FsPath, right: &FsPath) -> bool {
    left == right || left.starts_with(right) || right.starts_with(left)
}

/// One pre-provisioned principal authentication entry.
#[derive(Clone)]
pub struct HostedPrincipalCredential {
    digest: HostedAuthTokenDigest,
    binding: HostedPrincipalBinding,
}

impl HostedPrincipalCredential {
    /// Creates one immutable credential-to-principal binding.
    #[must_use]
    pub const fn new(digest: HostedAuthTokenDigest, binding: HostedPrincipalBinding) -> Self {
        Self { digest, binding }
    }
}

/// Immutable pre-provisioned principal registry.
#[derive(Clone)]
pub struct HostedPrincipalRegistry {
    principals: Vec<HostedPrincipalCredential>,
}

impl HostedPrincipalRegistry {
    /// Validates deployment principals against one organization and registry.
    ///
    /// # Errors
    ///
    /// Rejects empty, oversized, conflicting, or unknown-project bindings.
    pub fn new(
        organization_id: &OrganizationId,
        projects: &HostedProjectRegistry,
        principals: Vec<HostedPrincipalCredential>,
    ) -> Result<Self, WorkflowOsError> {
        if principals.is_empty() || principals.len() > 256 {
            return Err(WorkflowOsError::validation(
                "hosted.principal_registry.invalid",
                "hosted principal registry is invalid",
            ));
        }
        for (index, principal) in principals.iter().enumerate() {
            if principal.binding.organization_id() != organization_id
                || principal
                    .binding
                    .grants()
                    .iter()
                    .any(|grant| !projects.contains(grant.project_id()))
                || principals.iter().skip(index + 1).any(|other| {
                    other.binding.actor_id() == principal.binding.actor_id()
                        || other.digest.0 == principal.digest.0
                })
            {
                return Err(WorkflowOsError::validation(
                    "hosted.principal_registry.conflict",
                    "hosted principal registry contains conflicting entries",
                ));
            }
        }
        Ok(Self { principals })
    }

    fn authenticate(&self, headers: &HeaderMap) -> Result<&HostedPrincipalBinding, HostedApiError> {
        let value = headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .ok_or_else(HostedApiError::unauthorized)?;
        self.principals
            .iter()
            .find(|principal| principal.digest.verify(value))
            .map(|principal| &principal.binding)
            .ok_or_else(HostedApiError::unauthorized)
    }
}

impl fmt::Debug for HostedPrincipalRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedPrincipalRegistry")
            .field("principal_count", &self.principals.len())
            .finish()
    }
}

/// Shared state for the explicitly project-scoped collaborative beta router.
#[derive(Clone)]
pub struct CollaborativeHostedApiState {
    backend: PostgresStateBackend,
    organization_id: OrganizationId,
    projects: HostedProjectRegistry,
    principals: HostedPrincipalRegistry,
    build_id: String,
}

impl CollaborativeHostedApiState {
    /// Creates collaborative hosted API state from immutable deployment registries.
    ///
    /// # Errors
    ///
    /// Rejects route-unsafe organization identity or invalid build identity.
    pub fn new(
        backend: PostgresStateBackend,
        organization_id: OrganizationId,
        projects: HostedProjectRegistry,
        principals: HostedPrincipalRegistry,
        build_id: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let build_id = build_id.into();
        if organization_id.as_str().contains('/') {
            return Err(WorkflowOsError::validation(
                "hosted.organization.id.path_unsafe",
                "hosted organization identity is not route-safe",
            ));
        }
        if build_id.is_empty() || build_id.len() > 128 || looks_secret_like(&build_id) {
            return Err(WorkflowOsError::validation(
                "hosted.build_id.invalid",
                "hosted build identity is invalid",
            ));
        }
        Ok(Self {
            backend,
            organization_id,
            projects,
            principals,
            build_id,
        })
    }
}

impl fmt::Debug for CollaborativeHostedApiState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CollaborativeHostedApiState")
            .field("backend", &"postgresql")
            .field("organization", &"[REDACTED]")
            .field("projects", &self.projects)
            .field("principals", &self.principals)
            .field("build_id", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl HostedApiState {
    /// Creates hosted API state.
    ///
    /// # Errors
    ///
    /// Rejects empty, oversized, or secret-like build identities.
    pub fn new(
        backend: PostgresStateBackend,
        auth: HostedApiAuth,
        build_id: impl Into<String>,
        project_root: impl Into<PathBuf>,
    ) -> Result<Self, WorkflowOsError> {
        let build_id = build_id.into();
        if build_id.is_empty()
            || build_id.len() > 128
            || build_id.chars().any(char::is_control)
            || looks_secret_like(&build_id)
        {
            return Err(WorkflowOsError::validation(
                "hosted.build_id.invalid",
                "hosted build identity is invalid",
            ));
        }
        let project_root = project_root.into();
        if !project_root.is_dir() {
            return Err(WorkflowOsError::validation(
                "hosted.project_root.invalid",
                "hosted project root configuration is invalid",
            ));
        }
        Ok(Self {
            backend,
            auth,
            build_id,
            project_root,
        })
    }
}

impl fmt::Debug for HostedApiState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedApiState")
            .field("backend", &"postgresql")
            .field("auth", &"[REDACTED]")
            .field("build_id", &"[REDACTED]")
            .field("project_root", &"[REDACTED]")
            .finish()
    }
}

/// Builds the single-tenant hosted alpha router.
pub fn hosted_router(state: HostedApiState) -> Router {
    Router::new()
        .route("/health/live", get(liveness))
        .route("/health/ready", get(readiness))
        .route("/version", get(version))
        .route("/api/v0alpha1/runs", post(create_run))
        .route("/api/v0alpha1/runs/:run_id", get(read_run))
        .route("/api/v0alpha1/runs/:run_id/events", get(read_run_events))
        .route(
            "/api/v0alpha1/runs/:run_id/report",
            get(read_terminal_report_metadata),
        )
        .route(
            "/api/v0alpha1/runs/:run_id/approvals/:approval_id",
            get(read_approval).post(decide_approval),
        )
        .route("/api/v0alpha1/runs/:run_id/cancel", post(cancel_run))
        .route(
            "/api/v0alpha1/runs/:run_id/reports/:report_id",
            get(read_report_metadata),
        )
        .route("/api/v0alpha1/metrics", get(read_metrics))
        .route(
            "/api/v0alpha1/work-items/:work_item_id",
            get(read_work_item),
        )
        .route(
            "/api/v0alpha1/work-items/:work_item_id/executions/:execution_id",
            get(read_execution_receipt),
        )
        .layer(DefaultBodyLimit::max(MAX_API_BODY_BYTES))
        .with_state(state)
}

/// Builds the project-scoped collaborative beta router.
pub fn collaborative_hosted_router(state: CollaborativeHostedApiState) -> Router {
    Router::new()
        .route("/health/live", get(liveness))
        .route("/version", get(collaborative_version))
        .route(
            "/api/v0alpha1/organizations/:organization_id/projects/:project_id/runs",
            post(collaborative_create_run),
        )
        .route(
            "/api/v0alpha1/organizations/:organization_id/projects/:project_id/runs/:run_id",
            get(collaborative_read_run),
        )
        .route(
            "/api/v0alpha1/organizations/:organization_id/projects/:project_id/runs/:run_id/events",
            get(collaborative_read_run_events),
        )
        .route(
            "/api/v0alpha1/organizations/:organization_id/projects/:project_id/runs/:run_id/approvals/:approval_id",
            get(collaborative_read_approval).post(collaborative_decide_approval),
        )
        .route(
            "/api/v0alpha1/organizations/:organization_id/projects/:project_id/runs/:run_id/cancel",
            post(collaborative_cancel_run),
        )
        .route(
            "/api/v0alpha1/organizations/:organization_id/projects/:project_id/runs/:run_id/report",
            get(collaborative_read_terminal_report),
        )
        .route(
            "/api/v0alpha1/organizations/:organization_id/projects/:project_id/runs/:run_id/reports/:report_id",
            get(collaborative_read_report),
        )
        .route(
            "/api/v0alpha1/organizations/:organization_id/projects/:project_id/runs/:run_id/work-items/:work_item_id",
            get(collaborative_read_work_item),
        )
        .route(
            "/api/v0alpha1/organizations/:organization_id/projects/:project_id/runs/:run_id/work-items/:work_item_id/executions/:execution_id",
            get(collaborative_read_execution_receipt),
        )
        .route(
            "/api/v0alpha1/organizations/:organization_id/projects/:project_id/catalog",
            get(collaborative_list_catalog),
        )
        .route(
            "/api/v0alpha1/organizations/:organization_id/projects/:project_id/catalog/:workflow_id/versions/:workflow_version",
            get(collaborative_read_catalog_version),
        )
        .route(
            "/api/v0alpha1/organizations/:organization_id/projects/:project_id/catalog/:workflow_id/versions",
            post(collaborative_publish_catalog_version),
        )
        .layer(DefaultBodyLimit::max(MAX_API_BODY_BYTES))
        .with_state(state)
}

async fn collaborative_version(
    State(state): State<CollaborativeHostedApiState>,
    headers: HeaderMap,
) -> Result<Json<VersionResponse>, HostedApiError> {
    state.principals.authenticate(&headers)?;
    Ok(Json(VersionResponse {
        api_version: "v0alpha1",
        build_id: state.build_id,
        posture: "collaborative_team_beta_project_boundary",
    }))
}

fn require_legacy_unbound_resource(
    backend: &PostgresStateBackend,
    kind: HostedProjectResourceKind,
    resource_id: &str,
) -> Result<(), WorkflowOsError> {
    if backend
        .read_hosted_project_resource_binding(kind, resource_id)?
        .is_some()
    {
        return Err(WorkflowOsError::invalid_state(
            "hosted.resource.not_found",
            "hosted resource was not found",
        ));
    }
    Ok(())
}

fn collaborative_authorize<'a>(
    state: &'a CollaborativeHostedApiState,
    headers: &HeaderMap,
    organization_id: &str,
    project_id: &str,
    capability: HostedProjectCapability,
    target_kind: HostedProjectResourceKind,
    target_reference: &str,
) -> Result<(&'a HostedPrincipalBinding, HostedProjectScope, &'a FsPath), HostedApiError> {
    let principal = state.principals.authenticate(headers)?;
    let organization_id =
        OrganizationId::new(organization_id).map_err(|_| HostedApiError::not_found())?;
    let project_id = ProjectId::new(project_id).map_err(|_| HostedApiError::not_found())?;
    let attempted_scope = HostedProjectScope::new(organization_id.clone(), project_id.clone());
    if organization_id != state.organization_id || principal.organization_id() != &organization_id {
        record_collaborative_access_decision(
            state,
            principal,
            attempted_scope,
            capability,
            false,
            "hosted_project.scope.denied",
            target_kind,
            target_reference,
        )?;
        return Err(HostedApiError::not_found());
    }
    let Some(registration) = state.projects.get(&project_id) else {
        record_collaborative_access_decision(
            state,
            principal,
            attempted_scope,
            capability,
            false,
            "hosted_project.scope.denied",
            target_kind,
            target_reference,
        )?;
        return Err(HostedApiError::not_found());
    };
    let scope = HostedProjectScope::new(organization_id, project_id);
    if !principal.allows(scope.project_id(), capability) {
        record_collaborative_access_decision(
            state,
            principal,
            scope.clone(),
            capability,
            false,
            "hosted_project.capability.denied",
            target_kind,
            target_reference,
        )?;
        return Err(HostedApiError::forbidden());
    }
    record_collaborative_access_decision(
        state,
        principal,
        scope.clone(),
        capability,
        true,
        "hosted_project.capability.allowed",
        target_kind,
        target_reference,
    )?;
    Ok((principal, scope, &registration.root))
}

#[allow(clippy::too_many_arguments)]
fn record_collaborative_access_decision(
    state: &CollaborativeHostedApiState,
    principal: &HostedPrincipalBinding,
    scope: HostedProjectScope,
    capability: HostedProjectCapability,
    allowed: bool,
    reason: &'static str,
    target_kind: HostedProjectResourceKind,
    target_reference: &str,
) -> Result<(), HostedApiError> {
    let decision = workflow_core::HostedProjectAccessDecision::new(
        workflow_core::EventId::generate(),
        principal.actor_id().clone(),
        principal.principal_kind(),
        scope,
        capability,
        allowed,
        reason,
        target_kind,
        target_reference,
        None,
        Timestamp::now_utc(),
    )
    .map_err(|error| HostedApiError::from_core(&error))?;
    state
        .backend
        .write_hosted_project_access_decision(&decision)
        .map_err(|error| HostedApiError::from_core(&error))
}

fn require_collaborative_resource(
    state: &CollaborativeHostedApiState,
    scope: &HostedProjectScope,
    kind: HostedProjectResourceKind,
    resource_id: &str,
) -> Result<(), HostedApiError> {
    let binding = state
        .backend
        .read_hosted_project_resource_binding(kind, resource_id)
        .map_err(|error| HostedApiError::from_core(&error))?
        .ok_or_else(HostedApiError::not_found)?;
    if binding.value().scope() != scope
        || binding.value().status() != HostedProjectResourceBindingStatus::Active
    {
        return Err(HostedApiError::not_found());
    }
    Ok(())
}

type ProjectPath = Path<(String, String)>;
type ProjectRunPath = Path<(String, String, String)>;

async fn collaborative_create_run(
    State(state): State<CollaborativeHostedApiState>,
    headers: HeaderMap,
    Path((organization_id, project_id)): ProjectPath,
    Json(request): Json<HostedRunCreateRequest>,
) -> Result<(StatusCode, Json<WorkflowRun>), HostedApiError> {
    let (principal, scope, root) = collaborative_authorize(
        &state,
        &headers,
        &organization_id,
        &project_id,
        HostedProjectCapability::RunCreate,
        HostedProjectResourceKind::Run,
        request.run_id.as_str(),
    )?;
    let actor = principal.actor_id().clone();
    let project_root = root.to_path_buf();
    let loaded = load_project(&project_root);
    if loaded.has_errors()
        || loaded
            .bundle
            .as_ref()
            .map(|bundle| &bundle.manifest.definition.project.id)
            != Some(scope.project_id())
    {
        return Err(HostedApiError::bad_request());
    }
    let backend = state.backend.clone();
    let reservation = HostedProjectResourceBinding::new(
        scope.clone(),
        HostedProjectResourceKind::Run,
        request.run_id.as_str(),
        HostedProjectResourceBindingStatus::Reserved,
        Timestamp::now_utc(),
    )
    .map_err(|error| HostedApiError::from_core(&error))?;
    let run_id = request.run_id.clone();
    let run = tokio::task::spawn_blocking(move || {
        backend.reserve_hosted_project_resource(&reservation)?;
        let expected_result = IdempotencyResult {
            result_ref: format!(
                "hosted-project-run:{}:{}:{}:{}",
                scope.organization_id(),
                scope.project_id(),
                actor,
                run_id
            ),
        };
        match backend
            .record_idempotency_result(&request.idempotency_key, expected_result.clone())?
        {
            IdempotencyWrite::FirstWrite(result) | IdempotencyWrite::Duplicate(result)
                if result == expected_result => {}
            _ => {
                return Err(WorkflowOsError::invalid_state(
                    "hosted.project_run.idempotency.conflict",
                    "hosted project run idempotency conflicts with durable state",
                ))
            }
        }
        let registry = LocalSkillRegistry::new();
        let executor = LocalExecutor::new(&backend, &registry);
        let execution = LocalExecutionRequest {
            project_root,
            workflow_id: request.workflow_id,
            run_id: Some(request.run_id),
            correlation_id: request.correlation_id,
            actor,
            before_skill_invocation_checkpoints:
                LocalExecutionBeforeSkillInvocationCheckpointInputs::default(),
            before_skill_invocation_hook: None,
            side_effect_events: Vec::new(),
            side_effect_lifecycle_events: Vec::new(),
        };
        let execution = LocalExecutionWithImmutableRunBundleRequest {
            execution,
            bundle: LocalExecutionImmutableRunBundleInputs {
                bundle_id: request.bundle_id,
                bundle_version: request.bundle_version,
                created_at: request.created_at,
                sensitivity: request.sensitivity,
                redaction_required: request.redaction_required,
            },
        };
        let mut dispatch = no_write_dispatch_inputs()?;
        dispatch.project_scope = Some(scope.clone());
        let result = execute_with_hosted_no_write_dispatch(
            &executor,
            &LocalExecutionWithHostedDispatchRequest {
                execution,
                dispatch,
            },
        )?;
        backend.activate_hosted_project_resource(
            &scope,
            HostedProjectResourceKind::Run,
            run_id.as_str(),
        )?;
        Ok(result.into_parts().0)
    })
    .await
    .map_err(|_| HostedApiError::internal())?
    .map_err(|error| HostedApiError::from_core(&error))?;
    Ok((StatusCode::CREATED, Json(run)))
}

async fn collaborative_read_run(
    State(state): State<CollaborativeHostedApiState>,
    headers: HeaderMap,
    Path((organization_id, project_id, run_id)): ProjectRunPath,
) -> Result<Json<WorkflowRun>, HostedApiError> {
    let (_, scope, _) = collaborative_authorize(
        &state,
        &headers,
        &organization_id,
        &project_id,
        HostedProjectCapability::RunRead,
        HostedProjectResourceKind::Run,
        &run_id,
    )?;
    require_collaborative_resource(&state, &scope, HostedProjectResourceKind::Run, &run_id)?;
    let run_id = WorkflowRunId::new(run_id).map_err(|_| HostedApiError::not_found())?;
    let backend = state.backend.clone();
    let run = tokio::task::spawn_blocking(move || backend.rehydrate_run(&run_id))
        .await
        .map_err(|_| HostedApiError::internal())?
        .map_err(|_| HostedApiError::not_found())?;
    Ok(Json(run))
}

async fn collaborative_read_run_events(
    State(state): State<CollaborativeHostedApiState>,
    headers: HeaderMap,
    Path((organization_id, project_id, run_id)): ProjectRunPath,
    Query(query): Query<EventPageQuery>,
) -> Result<Json<EventPageResponse>, HostedApiError> {
    let (_, scope, _) = collaborative_authorize(
        &state,
        &headers,
        &organization_id,
        &project_id,
        HostedProjectCapability::RunRead,
        HostedProjectResourceKind::Run,
        &run_id,
    )?;
    require_collaborative_resource(&state, &scope, HostedProjectResourceKind::Run, &run_id)?;
    if query.limit == 0 || query.limit > MAX_EVENT_PAGE_SIZE {
        return Err(HostedApiError::bad_request());
    }
    let run_id = WorkflowRunId::new(run_id).map_err(|_| HostedApiError::not_found())?;
    let backend = state.backend.clone();
    let events = tokio::task::spawn_blocking(move || backend.read_events(&run_id))
        .await
        .map_err(|_| HostedApiError::internal())?
        .map_err(|_| HostedApiError::not_found())?;
    let mut selected = events
        .into_iter()
        .filter(|event| event.sequence_number.get() > query.after_sequence)
        .take(query.limit.saturating_add(1))
        .collect::<Vec<_>>();
    let has_more = selected.len() > query.limit;
    selected.truncate(query.limit);
    Ok(Json(EventPageResponse {
        events: selected,
        has_more,
    }))
}

async fn collaborative_read_approval(
    State(state): State<CollaborativeHostedApiState>,
    headers: HeaderMap,
    Path((organization_id, project_id, run_id, approval_id)): Path<(
        String,
        String,
        String,
        String,
    )>,
) -> Result<Json<workflow_core::ApprovalRequest>, HostedApiError> {
    let (_, scope, _) = collaborative_authorize(
        &state,
        &headers,
        &organization_id,
        &project_id,
        HostedProjectCapability::ApprovalRead,
        HostedProjectResourceKind::Run,
        &run_id,
    )?;
    require_collaborative_resource(&state, &scope, HostedProjectResourceKind::Run, &run_id)?;
    let run_id = WorkflowRunId::new(run_id).map_err(|_| HostedApiError::not_found())?;
    let backend = state.backend.clone();
    let approval = tokio::task::spawn_blocking(move || backend.load_approval_request(&approval_id))
        .await
        .map_err(|_| HostedApiError::internal())?
        .map_err(|error| HostedApiError::from_core(&error))?
        .ok_or_else(HostedApiError::not_found)?;
    if approval.run_id != run_id {
        return Err(HostedApiError::not_found());
    }
    Ok(Json(approval))
}

async fn collaborative_decide_approval(
    State(state): State<CollaborativeHostedApiState>,
    headers: HeaderMap,
    Path((organization_id, project_id, run_id, approval_id)): Path<(
        String,
        String,
        String,
        String,
    )>,
    Json(request): Json<HostedApprovalDecisionRequest>,
) -> Result<Json<WorkflowRun>, HostedApiError> {
    let (principal, scope, root) = collaborative_authorize(
        &state,
        &headers,
        &organization_id,
        &project_id,
        HostedProjectCapability::ApprovalDecide,
        HostedProjectResourceKind::Run,
        &run_id,
    )?;
    require_collaborative_resource(&state, &scope, HostedProjectResourceKind::Run, &run_id)?;
    let actor = principal.actor_id().clone();
    let project_root = root.to_path_buf();
    let run_id = WorkflowRunId::new(run_id).map_err(|_| HostedApiError::not_found())?;
    if request.presentation.run_id() != &run_id
        || request.presentation.approval_id() != approval_id
        || request.presentation.presented_by() != &actor
        || request.max_presentation_age_seconds == 0
        || request.max_presentation_age_seconds > 86_400
    {
        return Err(HostedApiError::bad_request());
    }
    let backend = state.backend.clone();
    let result = tokio::task::spawn_blocking(move || {
        reserve_scoped_hosted_mutation(
            &backend,
            &scope,
            &actor,
            &request.idempotency_key,
            "approval-decision",
            &request,
        )?;
        match backend.read_approval_presentation_record(request.presentation.presentation_id())? {
            Some(existing) if existing == request.presentation => {}
            Some(_) => {
                return Err(WorkflowOsError::invalid_state(
                    "hosted.approval.presentation.conflict",
                    "hosted approval presentation conflicts with durable proof",
                ))
            }
            None => backend.write_approval_presentation_record(&request.presentation)?,
        }
        let registry = LocalSkillRegistry::new();
        let executor = LocalExecutor::new(&backend, &registry);
        let mut dispatch = no_write_dispatch_inputs()?;
        dispatch.project_scope = Some(scope);
        decide_hosted_dispatch_approval_with_presentation(
            &executor,
            LocalApprovalPresentationDecisionRequest {
                approval: LocalApprovalDecisionRequest {
                    project_root,
                    run_id,
                    approval_id,
                    decision: request.decision,
                    actor,
                    reason: request.reason,
                    correlation_id: request.correlation_id,
                },
                proof: LocalApprovalPresentationProof::PresentationId(
                    request.presentation.presentation_id().clone(),
                ),
                max_presentation_age: Some(Duration::from_secs(
                    request.max_presentation_age_seconds,
                )),
            },
            &dispatch,
        )
    })
    .await
    .map_err(|_| HostedApiError::internal())?
    .map_err(|error| HostedApiError::from_core(&error))?;
    Ok(Json(result))
}

async fn collaborative_cancel_run(
    State(state): State<CollaborativeHostedApiState>,
    headers: HeaderMap,
    Path((organization_id, project_id, run_id)): ProjectRunPath,
    Json(request): Json<HostedCancellationRequest>,
) -> Result<Json<WorkflowRun>, HostedApiError> {
    let (principal, scope, _) = collaborative_authorize(
        &state,
        &headers,
        &organization_id,
        &project_id,
        HostedProjectCapability::RunCancel,
        HostedProjectResourceKind::Run,
        &run_id,
    )?;
    require_collaborative_resource(&state, &scope, HostedProjectResourceKind::Run, &run_id)?;
    let actor = principal.actor_id().clone();
    let run_id = WorkflowRunId::new(run_id).map_err(|_| HostedApiError::not_found())?;
    let backend = state.backend.clone();
    let result = tokio::task::spawn_blocking(move || {
        reserve_scoped_hosted_mutation(
            &backend,
            &scope,
            &actor,
            &request.idempotency_key,
            "cancellation",
            &request,
        )?;
        let registry = LocalSkillRegistry::new();
        LocalExecutor::new(&backend, &registry).cancel_run(LocalCancellationRequest {
            run_id,
            actor,
            reason: request.reason,
            correlation_id: request.correlation_id,
        })
    })
    .await
    .map_err(|_| HostedApiError::internal())?
    .map_err(|error| HostedApiError::from_core(&error))?;
    Ok(Json(result))
}

fn reserve_scoped_hosted_mutation<T: Serialize>(
    backend: &PostgresStateBackend,
    scope: &HostedProjectScope,
    actor: &ActorId,
    idempotency_key: &IdempotencyKey,
    operation: &str,
    request: &T,
) -> Result<(), WorkflowOsError> {
    let intent = hosted_mutation_intent(
        &format!(
            "{}:{}:{}:{}",
            scope.organization_id(),
            scope.project_id(),
            actor,
            operation
        ),
        request,
    )?;
    let expected = IdempotencyResult {
        result_ref: format!("hosted-project-mutation:{intent}"),
    };
    match backend.record_idempotency_result(idempotency_key, expected.clone())? {
        IdempotencyWrite::FirstWrite(result) if result == expected => Ok(()),
        IdempotencyWrite::Duplicate(result) if result == expected => {
            Err(WorkflowOsError::invalid_state(
                "hosted.project_mutation.idempotency.replay_deferred",
                "hosted project mutation replay requires resource inspection",
            ))
        }
        _ => Err(WorkflowOsError::invalid_state(
            "hosted.project_mutation.idempotency.conflict",
            "hosted project mutation idempotency conflicts with durable state",
        )),
    }
}

async fn collaborative_read_terminal_report(
    State(state): State<CollaborativeHostedApiState>,
    headers: HeaderMap,
    Path((organization_id, project_id, run_id)): ProjectRunPath,
) -> Result<Json<WorkReportArtifactMetadata>, HostedApiError> {
    let (_, scope, _) = collaborative_authorize(
        &state,
        &headers,
        &organization_id,
        &project_id,
        HostedProjectCapability::ReportRead,
        HostedProjectResourceKind::Run,
        &run_id,
    )?;
    require_collaborative_resource(&state, &scope, HostedProjectResourceKind::Run, &run_id)?;
    let run_id = WorkflowRunId::new(run_id).map_err(|_| HostedApiError::not_found())?;
    let backend = state.backend.clone();
    let stored_run_id = run_id.clone();
    let artifacts =
        tokio::task::spawn_blocking(move || backend.list_work_report_artifacts(&stored_run_id))
            .await
            .map_err(|_| HostedApiError::internal())?
            .map_err(|error| HostedApiError::from_core(&error))?;
    let mut artifacts = artifacts.into_iter();
    let artifact = artifacts.next().ok_or_else(HostedApiError::not_found)?;
    if artifacts.next().is_some() {
        return Err(HostedApiError::internal());
    }
    require_collaborative_resource(
        &state,
        &scope,
        HostedProjectResourceKind::Report,
        artifact.metadata().report_id().as_str(),
    )?;
    Ok(Json(artifact.metadata().clone()))
}

async fn collaborative_read_report(
    State(state): State<CollaborativeHostedApiState>,
    headers: HeaderMap,
    Path((organization_id, project_id, run_id, report_id)): Path<(String, String, String, String)>,
) -> Result<Json<WorkReportArtifactMetadata>, HostedApiError> {
    let (_, scope, _) = collaborative_authorize(
        &state,
        &headers,
        &organization_id,
        &project_id,
        HostedProjectCapability::ReportRead,
        HostedProjectResourceKind::Report,
        &report_id,
    )?;
    require_collaborative_resource(&state, &scope, HostedProjectResourceKind::Run, &run_id)?;
    let run_id = WorkflowRunId::new(run_id).map_err(|_| HostedApiError::not_found())?;
    let report_id = WorkReportId::new(report_id).map_err(|_| HostedApiError::not_found())?;
    let backend = state.backend.clone();
    let stored_run_id = run_id.clone();
    let stored_report_id = report_id.clone();
    let artifact = tokio::task::spawn_blocking(move || {
        backend.read_work_report_artifact(&stored_run_id, &stored_report_id)
    })
    .await
    .map_err(|_| HostedApiError::internal())?
    .map_err(|error| HostedApiError::from_core(&error))?
    .ok_or_else(HostedApiError::not_found)?;
    if artifact.metadata().run_id() != &run_id {
        return Err(HostedApiError::not_found());
    }
    require_collaborative_resource(
        &state,
        &scope,
        HostedProjectResourceKind::Report,
        report_id.as_str(),
    )?;
    Ok(Json(artifact.metadata().clone()))
}

async fn collaborative_read_work_item(
    State(state): State<CollaborativeHostedApiState>,
    headers: HeaderMap,
    Path((organization_id, project_id, run_id, work_item_id)): Path<(
        String,
        String,
        String,
        String,
    )>,
) -> Result<Json<HostedWorkItem>, HostedApiError> {
    let (_, scope, _) = collaborative_authorize(
        &state,
        &headers,
        &organization_id,
        &project_id,
        HostedProjectCapability::RunRead,
        HostedProjectResourceKind::WorkItem,
        &work_item_id,
    )?;
    require_collaborative_resource(&state, &scope, HostedProjectResourceKind::Run, &run_id)?;
    require_collaborative_resource(
        &state,
        &scope,
        HostedProjectResourceKind::WorkItem,
        &work_item_id,
    )?;
    let work_item_id =
        HostedWorkItemId::new(work_item_id).map_err(|_| HostedApiError::not_found())?;
    let backend = state.backend.clone();
    let record = tokio::task::spawn_blocking(move || {
        backend.read_revisioned_hosted_work_item(&work_item_id)
    })
    .await
    .map_err(|_| HostedApiError::internal())?
    .map_err(|error| HostedApiError::from_core(&error))?
    .ok_or_else(HostedApiError::not_found)?;
    if record.value().run_id().as_str() != run_id {
        return Err(HostedApiError::not_found());
    }
    Ok(Json(record.into_parts().0))
}

async fn collaborative_read_execution_receipt(
    State(state): State<CollaborativeHostedApiState>,
    headers: HeaderMap,
    Path((organization_id, project_id, run_id, work_item_id, execution_id)): Path<(
        String,
        String,
        String,
        String,
        String,
    )>,
) -> Result<Json<HostedExecutionReceipt>, HostedApiError> {
    let (_, scope, _) = collaborative_authorize(
        &state,
        &headers,
        &organization_id,
        &project_id,
        HostedProjectCapability::RunRead,
        HostedProjectResourceKind::ExecutionReceipt,
        &execution_id,
    )?;
    require_collaborative_resource(&state, &scope, HostedProjectResourceKind::Run, &run_id)?;
    require_collaborative_resource(
        &state,
        &scope,
        HostedProjectResourceKind::WorkItem,
        &work_item_id,
    )?;
    let work_item_id =
        HostedWorkItemId::new(work_item_id).map_err(|_| HostedApiError::not_found())?;
    let execution_id =
        HostedExecutionId::new(execution_id).map_err(|_| HostedApiError::not_found())?;
    let stored_execution_id = execution_id.clone();
    let backend = state.backend.clone();
    let receipt = tokio::task::spawn_blocking(move || {
        let work_item = backend
            .read_revisioned_hosted_work_item(&work_item_id)?
            .ok_or_else(|| {
                WorkflowOsError::invalid_state(
                    "hosted.project_resource.missing",
                    "hosted project resource is missing",
                )
            })?;
        if work_item.value().run_id().as_str() != run_id {
            return Ok(None);
        }
        backend.read_hosted_execution_receipt(&work_item_id, &stored_execution_id)
    })
    .await
    .map_err(|_| HostedApiError::internal())?
    .map_err(|error| HostedApiError::from_core(&error))?
    .ok_or_else(HostedApiError::not_found)?;
    require_collaborative_resource(
        &state,
        &scope,
        HostedProjectResourceKind::ExecutionReceipt,
        execution_id.as_str(),
    )?;
    Ok(Json(receipt))
}

async fn collaborative_list_catalog(
    State(state): State<CollaborativeHostedApiState>,
    headers: HeaderMap,
    Path((organization_id, project_id)): ProjectPath,
) -> Result<Json<Vec<HostedProjectCatalogVersion>>, HostedApiError> {
    let (_, scope, _) = collaborative_authorize(
        &state,
        &headers,
        &organization_id,
        &project_id,
        HostedProjectCapability::CatalogRead,
        HostedProjectResourceKind::CatalogRecord,
        "catalog",
    )?;
    let versions = state
        .backend
        .list_hosted_project_catalog_versions(&scope)
        .map_err(|error| HostedApiError::from_core(&error))?;
    Ok(Json(versions))
}

async fn collaborative_read_catalog_version(
    State(state): State<CollaborativeHostedApiState>,
    headers: HeaderMap,
    Path((organization_id, project_id, workflow_id, workflow_version)): Path<(
        String,
        String,
        String,
        String,
    )>,
) -> Result<Json<HostedProjectCatalogVersion>, HostedApiError> {
    let (_, scope, _) = collaborative_authorize(
        &state,
        &headers,
        &organization_id,
        &project_id,
        HostedProjectCapability::CatalogRead,
        HostedProjectResourceKind::CatalogRecord,
        &format!("{workflow_id}/{workflow_version}"),
    )?;
    let workflow_id = WorkflowId::new(workflow_id).map_err(|_| HostedApiError::not_found())?;
    let workflow_version = workflow_core::WorkflowVersion::new(workflow_version)
        .map_err(|_| HostedApiError::not_found())?;
    let version = state
        .backend
        .read_hosted_project_catalog_version(&scope, &workflow_id, &workflow_version)
        .map_err(|error| HostedApiError::from_core(&error))?
        .ok_or_else(HostedApiError::not_found)?;
    Ok(Json(version))
}

#[derive(Deserialize)]
struct HostedCatalogPublishRequest {
    version: HostedProjectCatalogVersion,
    stewardship: workflow_core::WorkflowStewardshipRecord,
    idempotency_key: IdempotencyKey,
}

async fn collaborative_publish_catalog_version(
    State(state): State<CollaborativeHostedApiState>,
    headers: HeaderMap,
    Path((organization_id, project_id, workflow_id)): Path<(String, String, String)>,
    Json(request): Json<HostedCatalogPublishRequest>,
) -> Result<(StatusCode, Json<HostedProjectCatalogVersion>), HostedApiError> {
    let (principal, scope, _) = collaborative_authorize(
        &state,
        &headers,
        &organization_id,
        &project_id,
        HostedProjectCapability::CatalogPublishVersion,
        HostedProjectResourceKind::CatalogRecord,
        &workflow_id,
    )?;
    if request.version.scope() != &scope
        || request.version.workflow_id().as_str() != workflow_id
        || request.version.published_by() != principal.actor_id()
    {
        return Err(HostedApiError::bad_request());
    }
    reserve_scoped_hosted_mutation(
        &state.backend,
        &scope,
        principal.actor_id(),
        &request.idempotency_key,
        "catalog-publish",
        &request.version,
    )
    .map_err(|error| HostedApiError::from_core(&error))?;
    state
        .backend
        .publish_hosted_project_catalog_version(&request.version, &request.stewardship)
        .map_err(|error| HostedApiError::from_core(&error))?;
    Ok((StatusCode::CREATED, Json(request.version)))
}

/// Exact explicit inputs for one remote governed-run creation.
#[derive(Deserialize)]
pub struct HostedRunCreateRequest {
    run_id: WorkflowRunId,
    workflow_id: WorkflowId,
    bundle_id: ImmutableRunBundleId,
    bundle_version: ImmutableRunBundleVersion,
    created_at: Timestamp,
    correlation_id: CorrelationId,
    idempotency_key: IdempotencyKey,
    #[serde(default)]
    sensitivity: ImmutableRunBundleSensitivity,
    #[serde(default = "default_redaction_required")]
    redaction_required: bool,
}

impl fmt::Debug for HostedRunCreateRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedRunCreateRequest")
            .field("identity", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .field("redaction_required", &self.redaction_required)
            .finish_non_exhaustive()
    }
}

const fn default_redaction_required() -> bool {
    true
}

async fn create_run(
    State(state): State<HostedApiState>,
    headers: HeaderMap,
    Json(request): Json<HostedRunCreateRequest>,
) -> Result<(StatusCode, Json<WorkflowRun>), HostedApiError> {
    let actor = state.auth.authorize(&headers)?.clone();
    let backend = state.backend.clone();
    let project_root = state.project_root.clone();
    let result = tokio::task::spawn_blocking(move || {
        require_legacy_unbound_resource(
            &backend,
            HostedProjectResourceKind::Run,
            request.run_id.as_str(),
        )?;
        let expected_result = IdempotencyResult {
            result_ref: format!("hosted-run:{}", request.run_id.as_str()),
        };
        match backend
            .record_idempotency_result(&request.idempotency_key, expected_result.clone())?
        {
            IdempotencyWrite::FirstWrite(result) | IdempotencyWrite::Duplicate(result)
                if result == expected_result => {}
            IdempotencyWrite::FirstWrite(_) | IdempotencyWrite::Duplicate(_) => {
                return Err(WorkflowOsError::invalid_state(
                    "hosted.run.idempotency.conflict",
                    "hosted run idempotency identity conflicts with durable state",
                ));
            }
        }
        let registry = LocalSkillRegistry::new();
        let executor = LocalExecutor::new(&backend, &registry);
        let execution = LocalExecutionRequest {
            project_root,
            workflow_id: request.workflow_id,
            run_id: Some(request.run_id),
            correlation_id: request.correlation_id,
            actor,
            before_skill_invocation_checkpoints:
                LocalExecutionBeforeSkillInvocationCheckpointInputs::default(),
            before_skill_invocation_hook: None,
            side_effect_events: Vec::new(),
            side_effect_lifecycle_events: Vec::new(),
        };
        let execution = LocalExecutionWithImmutableRunBundleRequest {
            execution,
            bundle: LocalExecutionImmutableRunBundleInputs {
                bundle_id: request.bundle_id,
                bundle_version: request.bundle_version,
                created_at: request.created_at,
                sensitivity: request.sensitivity,
                redaction_required: request.redaction_required,
            },
        };
        let request = LocalExecutionWithHostedDispatchRequest {
            execution,
            dispatch: no_write_dispatch_inputs()?,
        };
        let result = execute_with_hosted_no_write_dispatch(&executor, &request)?;
        Ok(result.into_parts().0)
    })
    .await
    .map_err(|_| HostedApiError::internal())?
    .map_err(|error| HostedApiError::from_core(&error))?;
    Ok((StatusCode::CREATED, Json(result)))
}

async fn read_run(
    State(state): State<HostedApiState>,
    headers: HeaderMap,
    Path(run_id): Path<String>,
) -> Result<Json<WorkflowRun>, HostedApiError> {
    state.auth.authorize(&headers)?;
    let run_id = WorkflowRunId::new(run_id).map_err(|error| HostedApiError::from_core(&error))?;
    let backend = state.backend.clone();
    let run = tokio::task::spawn_blocking(move || {
        require_legacy_unbound_resource(&backend, HostedProjectResourceKind::Run, run_id.as_str())?;
        backend.rehydrate_run(&run_id)
    })
    .await
    .map_err(|_| HostedApiError::internal())?
    .map_err(|error| HostedApiError::from_core(&error))?;
    Ok(Json(run))
}

#[derive(Deserialize)]
struct EventPageQuery {
    #[serde(default = "default_event_page_limit")]
    limit: usize,
    #[serde(default)]
    after_sequence: u64,
}

const fn default_event_page_limit() -> usize {
    50
}

#[derive(Serialize)]
struct EventPageResponse {
    events: Vec<WorkflowRunEvent>,
    has_more: bool,
}

async fn read_run_events(
    State(state): State<HostedApiState>,
    headers: HeaderMap,
    Path(run_id): Path<String>,
    Query(query): Query<EventPageQuery>,
) -> Result<Json<EventPageResponse>, HostedApiError> {
    state.auth.authorize(&headers)?;
    if query.limit == 0 || query.limit > MAX_EVENT_PAGE_SIZE {
        return Err(HostedApiError::bad_request());
    }
    let run_id = WorkflowRunId::new(run_id).map_err(|error| HostedApiError::from_core(&error))?;
    let backend = state.backend.clone();
    let events = tokio::task::spawn_blocking(move || {
        require_legacy_unbound_resource(&backend, HostedProjectResourceKind::Run, run_id.as_str())?;
        backend.read_events(&run_id)
    })
    .await
    .map_err(|_| HostedApiError::internal())?
    .map_err(|error| HostedApiError::from_core(&error))?;
    let mut selected = events
        .into_iter()
        .filter(|event| event.sequence_number.get() > query.after_sequence)
        .take(query.limit.saturating_add(1))
        .collect::<Vec<_>>();
    let has_more = selected.len() > query.limit;
    selected.truncate(query.limit);
    Ok(Json(EventPageResponse {
        events: selected,
        has_more,
    }))
}

async fn read_approval(
    State(state): State<HostedApiState>,
    headers: HeaderMap,
    Path((run_id, approval_id)): Path<(String, String)>,
) -> Result<Json<workflow_core::ApprovalRequest>, HostedApiError> {
    state.auth.authorize(&headers)?;
    let run_id = WorkflowRunId::new(run_id).map_err(|error| HostedApiError::from_core(&error))?;
    let backend = state.backend.clone();
    let guarded_run_id = run_id.clone();
    let approval = tokio::task::spawn_blocking(move || {
        require_legacy_unbound_resource(
            &backend,
            HostedProjectResourceKind::Run,
            guarded_run_id.as_str(),
        )?;
        backend.load_approval_request(&approval_id)
    })
    .await
    .map_err(|_| HostedApiError::internal())?
    .map_err(|error| HostedApiError::from_core(&error))?
    .ok_or_else(HostedApiError::not_found)?;
    if approval.run_id != run_id {
        return Err(HostedApiError::not_found());
    }
    Ok(Json(approval))
}

#[derive(Deserialize, Serialize)]
struct HostedApprovalDecisionRequest {
    decision: ApprovalDecisionKind,
    reason: String,
    correlation_id: CorrelationId,
    idempotency_key: IdempotencyKey,
    presentation: ApprovalPresentationRecord,
    #[serde(default = "default_presentation_max_age_seconds")]
    max_presentation_age_seconds: u64,
}

const fn default_presentation_max_age_seconds() -> u64 {
    DEFAULT_PRESENTATION_MAX_AGE_SECONDS
}

async fn decide_approval(
    State(state): State<HostedApiState>,
    headers: HeaderMap,
    Path((run_id, approval_id)): Path<(String, String)>,
    Json(request): Json<HostedApprovalDecisionRequest>,
) -> Result<Json<WorkflowRun>, HostedApiError> {
    let actor = state.auth.authorize(&headers)?.clone();
    let run_id = WorkflowRunId::new(run_id).map_err(|error| HostedApiError::from_core(&error))?;
    if request.presentation.run_id() != &run_id
        || request.presentation.approval_id() != approval_id
        || request.presentation.presented_by() != &actor
        || request.max_presentation_age_seconds == 0
        || request.max_presentation_age_seconds > 86_400
    {
        return Err(HostedApiError::bad_request());
    }
    let backend = state.backend.clone();
    let project_root = state.project_root.clone();
    let result = tokio::task::spawn_blocking(move || {
        require_legacy_unbound_resource(&backend, HostedProjectResourceKind::Run, run_id.as_str())?;
        reserve_hosted_mutation(
            &backend,
            &request.idempotency_key,
            "approval-decision",
            &request,
        )?;
        match backend.read_approval_presentation_record(request.presentation.presentation_id())? {
            Some(existing) if existing == request.presentation => {}
            Some(_) => {
                return Err(WorkflowOsError::invalid_state(
                    "hosted.approval.presentation.conflict",
                    "hosted approval presentation conflicts with durable proof",
                ));
            }
            None => backend.write_approval_presentation_record(&request.presentation)?,
        }
        let registry = LocalSkillRegistry::new();
        let executor = LocalExecutor::new(&backend, &registry);
        decide_hosted_dispatch_approval_with_presentation(
            &executor,
            LocalApprovalPresentationDecisionRequest {
                approval: LocalApprovalDecisionRequest {
                    project_root,
                    run_id,
                    approval_id,
                    decision: request.decision,
                    actor,
                    reason: request.reason,
                    correlation_id: request.correlation_id,
                },
                proof: LocalApprovalPresentationProof::PresentationId(
                    request.presentation.presentation_id().clone(),
                ),
                max_presentation_age: Some(Duration::from_secs(
                    request.max_presentation_age_seconds,
                )),
            },
            &no_write_dispatch_inputs()?,
        )
    })
    .await
    .map_err(|_| HostedApiError::internal())?
    .map_err(|error| HostedApiError::from_core(&error))?;
    Ok(Json(result))
}

#[derive(Deserialize, Serialize)]
struct HostedCancellationRequest {
    reason: String,
    correlation_id: CorrelationId,
    idempotency_key: IdempotencyKey,
}

async fn cancel_run(
    State(state): State<HostedApiState>,
    headers: HeaderMap,
    Path(run_id): Path<String>,
    Json(request): Json<HostedCancellationRequest>,
) -> Result<Json<WorkflowRun>, HostedApiError> {
    let actor = state.auth.authorize(&headers)?.clone();
    let run_id = WorkflowRunId::new(run_id).map_err(|error| HostedApiError::from_core(&error))?;
    let backend = state.backend.clone();
    let result = tokio::task::spawn_blocking(move || {
        require_legacy_unbound_resource(&backend, HostedProjectResourceKind::Run, run_id.as_str())?;
        reserve_hosted_mutation(&backend, &request.idempotency_key, "cancellation", &request)?;
        let registry = LocalSkillRegistry::new();
        LocalExecutor::new(&backend, &registry).cancel_run(LocalCancellationRequest {
            run_id,
            actor,
            reason: request.reason,
            correlation_id: request.correlation_id,
        })
    })
    .await
    .map_err(|_| HostedApiError::internal())?
    .map_err(|error| HostedApiError::from_core(&error))?;
    Ok(Json(result))
}

fn reserve_hosted_mutation<T: Serialize>(
    backend: &PostgresStateBackend,
    idempotency_key: &IdempotencyKey,
    operation: &str,
    request: &T,
) -> Result<(), WorkflowOsError> {
    let intent = hosted_mutation_intent(operation, request)?;
    let expected_result = IdempotencyResult {
        result_ref: format!("hosted-mutation:{}", intent.as_str()),
    };
    match backend.record_idempotency_result(idempotency_key, expected_result.clone())? {
        IdempotencyWrite::FirstWrite(result) | IdempotencyWrite::Duplicate(result)
            if result == expected_result =>
        {
            Ok(())
        }
        IdempotencyWrite::FirstWrite(_) | IdempotencyWrite::Duplicate(_) => {
            Err(WorkflowOsError::invalid_state(
                "hosted.mutation.idempotency.conflict",
                "hosted mutation idempotency identity conflicts with durable state",
            ))
        }
    }
}

fn hosted_mutation_intent<T: Serialize>(
    operation: &str,
    request: &T,
) -> Result<SpecContentHash, WorkflowOsError> {
    let mut canonical = operation.as_bytes().to_vec();
    canonical.push(0);
    canonical.extend(serde_json::to_vec(request).map_err(|_| {
        WorkflowOsError::validation(
            "hosted.mutation.intent.invalid",
            "hosted mutation intent is invalid",
        )
    })?);
    Ok(SpecContentHash::from_bytes(canonical))
}

async fn read_terminal_report_metadata(
    State(state): State<HostedApiState>,
    headers: HeaderMap,
    Path(run_id): Path<String>,
) -> Result<Json<WorkReportArtifactMetadata>, HostedApiError> {
    state.auth.authorize(&headers)?;
    let run_id = WorkflowRunId::new(run_id).map_err(|error| HostedApiError::from_core(&error))?;
    let backend = state.backend.clone();
    let stored_run_id = run_id.clone();
    let artifacts = tokio::task::spawn_blocking(move || {
        require_legacy_unbound_resource(
            &backend,
            HostedProjectResourceKind::Run,
            stored_run_id.as_str(),
        )?;
        backend.list_work_report_artifacts(&stored_run_id)
    })
    .await
    .map_err(|_| HostedApiError::internal())?
    .map_err(|error| HostedApiError::from_core(&error))?;
    let mut artifacts = artifacts.into_iter();
    let artifact = artifacts.next().ok_or_else(HostedApiError::not_found)?;
    if artifacts.next().is_some() {
        return Err(HostedApiError::internal());
    }
    let backend = state.backend.clone();
    let stored_run_id = run_id.clone();
    let run = tokio::task::spawn_blocking(move || backend.rehydrate_run(&stored_run_id))
        .await
        .map_err(|_| HostedApiError::internal())?
        .map_err(|error| HostedApiError::from_core(&error))?;
    let metadata = artifact.metadata();
    let terminal_status = match run.snapshot.status {
        WorkflowRunStatus::Completed => WorkReportStatus::Completed,
        WorkflowRunStatus::Failed => WorkReportStatus::Failed,
        WorkflowRunStatus::Canceled => WorkReportStatus::Canceled,
        _ => return Err(HostedApiError::internal()),
    };
    let identity = &run.snapshot.identity;
    if metadata.run_id() != &run_id
        || metadata.workflow_id() != &identity.workflow_id
        || metadata.workflow_version() != &identity.workflow_version
        || metadata.schema_version() != &identity.schema_version
        || metadata.spec_hash() != &identity.spec_content_hash
        || metadata.terminal_run_status() != terminal_status
    {
        return Err(HostedApiError::internal());
    }
    Ok(Json(metadata.clone()))
}

async fn read_report_metadata(
    State(state): State<HostedApiState>,
    headers: HeaderMap,
    Path((run_id, report_id)): Path<(String, String)>,
) -> Result<Json<WorkReportArtifactMetadata>, HostedApiError> {
    state.auth.authorize(&headers)?;
    let run_id = WorkflowRunId::new(run_id).map_err(|error| HostedApiError::from_core(&error))?;
    let report_id =
        WorkReportId::new(report_id).map_err(|error| HostedApiError::from_core(&error))?;
    let backend = state.backend.clone();
    let stored_run_id = run_id.clone();
    let stored_report_id = report_id.clone();
    let artifact = tokio::task::spawn_blocking(move || {
        require_legacy_unbound_resource(
            &backend,
            HostedProjectResourceKind::Run,
            stored_run_id.as_str(),
        )?;
        backend.read_work_report_artifact(&stored_run_id, &stored_report_id)
    })
    .await
    .map_err(|_| HostedApiError::internal())?
    .map_err(|error| HostedApiError::from_core(&error))?
    .ok_or_else(HostedApiError::not_found)?;
    if artifact.metadata().run_id() != &run_id || artifact.metadata().report_id() != &report_id {
        return Err(HostedApiError::internal());
    }
    Ok(Json(artifact.metadata().clone()))
}

#[derive(Serialize)]
struct HostedMetricsResponse {
    posture: &'static str,
    queued_work_items: u64,
    running_work_items: u64,
    waiting_work_items: u64,
    completed_work_items: u64,
    failed_work_items: u64,
    canceled_work_items: u64,
    ambiguous_work_items: u64,
    prepared_attempts: u64,
    invoking_attempts: u64,
    reconciliation_required_attempts: u64,
    terminal_attempts: u64,
    oldest_queued_age_ms: Option<u64>,
    observed_at_epoch_ms: i64,
}

async fn read_metrics(
    State(state): State<HostedApiState>,
    headers: HeaderMap,
) -> Result<Json<HostedMetricsResponse>, HostedApiError> {
    state.auth.authorize(&headers)?;
    let backend = state.backend.clone();
    let metrics = tokio::task::spawn_blocking(move || backend.hosted_queue_metrics_snapshot())
        .await
        .map_err(|_| HostedApiError::internal())?
        .map_err(|error| HostedApiError::from_core(&error))?;
    Ok(Json(HostedMetricsResponse {
        posture: "bounded_alpha",
        queued_work_items: metrics.queued_work_items(),
        running_work_items: metrics.running_work_items(),
        waiting_work_items: metrics.waiting_work_items(),
        completed_work_items: metrics.completed_work_items(),
        failed_work_items: metrics.failed_work_items(),
        canceled_work_items: metrics.canceled_work_items(),
        ambiguous_work_items: metrics.ambiguous_work_items(),
        prepared_attempts: metrics.prepared_attempts(),
        invoking_attempts: metrics.invoking_attempts(),
        reconciliation_required_attempts: metrics.reconciliation_required_attempts(),
        terminal_attempts: metrics.terminal_attempts(),
        oldest_queued_age_ms: metrics.oldest_queued_age_ms(),
        observed_at_epoch_ms: metrics.observed_at_epoch_ms(),
    }))
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    dependency: &'static str,
}

async fn liveness() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        dependency: "process",
    })
}

async fn readiness(
    State(state): State<HostedApiState>,
    headers: HeaderMap,
) -> Result<Json<HealthResponse>, HostedApiError> {
    state.auth.authorize(&headers)?;
    let backend = state.backend.clone();
    let report = tokio::task::spawn_blocking(move || backend.detailed_health_check())
        .await
        .map_err(|_| HostedApiError::internal())?
        .map_err(|error| HostedApiError::from_core(&error))?;
    if !report.healthy() {
        return Err(HostedApiError::unavailable());
    }
    Ok(Json(HealthResponse {
        status: "ok",
        dependency: "postgresql",
    }))
}

#[derive(Serialize)]
struct VersionResponse {
    api_version: &'static str,
    build_id: String,
    posture: &'static str,
}

async fn version(
    State(state): State<HostedApiState>,
    headers: HeaderMap,
) -> Result<Json<VersionResponse>, HostedApiError> {
    state.auth.authorize(&headers)?;
    Ok(Json(VersionResponse {
        api_version: "v0alpha1",
        build_id: state.build_id,
        posture: "single_tenant_hosted_alpha",
    }))
}

async fn read_work_item(
    State(state): State<HostedApiState>,
    headers: HeaderMap,
    Path(work_item_id): Path<String>,
) -> Result<Json<HostedWorkItem>, HostedApiError> {
    state.auth.authorize(&headers)?;
    let work_item_id =
        HostedWorkItemId::new(work_item_id).map_err(|error| HostedApiError::from_core(&error))?;
    let backend = state.backend.clone();
    let record = tokio::task::spawn_blocking(move || {
        let record = backend.read_revisioned_hosted_work_item(&work_item_id)?;
        if let Some(record) = &record {
            require_legacy_unbound_resource(
                &backend,
                HostedProjectResourceKind::Run,
                record.value().run_id().as_str(),
            )?;
        }
        Ok(record)
    })
    .await
    .map_err(|_| HostedApiError::internal())?
    .map_err(|error| HostedApiError::from_core(&error))?
    .ok_or_else(HostedApiError::not_found)?;
    Ok(Json(record.into_parts().0))
}

async fn read_execution_receipt(
    State(state): State<HostedApiState>,
    headers: HeaderMap,
    Path((work_item_id, execution_id)): Path<(String, String)>,
) -> Result<Json<HostedExecutionReceipt>, HostedApiError> {
    state.auth.authorize(&headers)?;
    let work_item_id =
        HostedWorkItemId::new(work_item_id).map_err(|error| HostedApiError::from_core(&error))?;
    let execution_id =
        HostedExecutionId::new(execution_id).map_err(|error| HostedApiError::from_core(&error))?;
    let backend = state.backend.clone();
    let receipt = tokio::task::spawn_blocking(move || {
        let work_item = backend
            .read_revisioned_hosted_work_item(&work_item_id)?
            .ok_or_else(|| {
                WorkflowOsError::invalid_state(
                    "hosted.resource.not_found",
                    "hosted resource was not found",
                )
            })?;
        require_legacy_unbound_resource(
            &backend,
            HostedProjectResourceKind::Run,
            work_item.value().run_id().as_str(),
        )?;
        backend.read_hosted_execution_receipt(&work_item_id, &execution_id)
    })
    .await
    .map_err(|_| HostedApiError::internal())?
    .map_err(|error| HostedApiError::from_core(&error))?
    .ok_or_else(HostedApiError::not_found)?;
    Ok(Json(receipt))
}

#[derive(Serialize)]
struct ErrorResponse {
    code: String,
    message: &'static str,
}

struct HostedApiError {
    status: StatusCode,
    code: String,
    message: &'static str,
}

impl HostedApiError {
    fn bad_request() -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "hosted.request.invalid".to_owned(),
            message: "hosted request is invalid",
        }
    }

    fn unauthorized() -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: "hosted.auth.unauthorized".to_owned(),
            message: "hosted API authentication failed",
        }
    }

    fn not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "hosted.resource.not_found".to_owned(),
            message: "hosted resource was not found",
        }
    }

    fn forbidden() -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code: "hosted.project.capability.denied".to_owned(),
            message: "hosted project capability is denied",
        }
    }

    fn unavailable() -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "hosted.dependency.unavailable".to_owned(),
            message: "hosted dependency is unavailable",
        }
    }

    fn internal() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "hosted.internal".to_owned(),
            message: "hosted request failed",
        }
    }

    fn from_core(error: &WorkflowOsError) -> Self {
        let status = if error.code() == "hosted.resource.not_found" {
            StatusCode::NOT_FOUND
        } else {
            match error.kind() {
                workflow_core::WorkflowOsErrorKind::Parse
                | workflow_core::WorkflowOsErrorKind::Validation
                | workflow_core::WorkflowOsErrorKind::Security => StatusCode::BAD_REQUEST,
                workflow_core::WorkflowOsErrorKind::PolicyDenied => StatusCode::FORBIDDEN,
                workflow_core::WorkflowOsErrorKind::Unsupported => StatusCode::NOT_IMPLEMENTED,
                workflow_core::WorkflowOsErrorKind::InvalidState => StatusCode::CONFLICT,
                workflow_core::WorkflowOsErrorKind::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            }
        };
        let (code, message) = if status == StatusCode::INTERNAL_SERVER_ERROR {
            ("hosted.internal".to_owned(), "hosted request failed")
        } else {
            (error.code().to_owned(), "hosted request is invalid")
        };
        Self {
            status,
            code,
            message,
        }
    }
}

impl IntoResponse for HostedApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ErrorResponse {
                code: self.code,
                message: self.message,
            }),
        )
            .into_response()
    }
}

/// Deterministic no-write provider used for the first hosted alpha proof.
pub struct NoWriteHostedExecutionProvider {
    provider_id: HostedExecutionProviderId,
    provider_version: HostedExecutionProviderVersion,
    configuration_hash: SpecContentHash,
}

impl NoWriteHostedExecutionProvider {
    /// Creates the fixed no-write provider.
    ///
    /// # Errors
    ///
    /// Returns an error only if the built-in identifiers violate Core rules.
    pub fn new() -> Result<Self, WorkflowOsError> {
        Ok(Self {
            provider_id: HostedExecutionProviderId::new("provider/no-write-alpha")?,
            provider_version: HostedExecutionProviderVersion::new("v1")?,
            configuration_hash: SpecContentHash::from_text(
                "workflow-os.no-write-hosted-provider.v1",
            ),
        })
    }

    fn validate_no_write_request(
        request: &HostedExecutionRequest,
    ) -> Result<(), HostedExecutionInvocationError> {
        if !request.approved_side_effects().is_empty()
            || !request.access_material_references().is_empty()
            || request
                .authorized_capabilities()
                .iter()
                .any(|capability| capability.as_str().strip_suffix(".read").is_none())
        {
            return Err(HostedExecutionInvocationError::new(
                HostedExecutionErrorCategory::Policy,
                HostedExecutionAttemptPosture::NotStarted,
            ));
        }
        Ok(())
    }

    fn no_write_execution_id(
        request: &HostedExecutionRequest,
    ) -> Result<HostedExecutionId, HostedExecutionInvocationError> {
        HostedExecutionId::new(format!(
            "execution-{}",
            request.fingerprint().as_hash().as_str()
        ))
        .map_err(|_| {
            HostedExecutionInvocationError::new(
                HostedExecutionErrorCategory::Protocol,
                HostedExecutionAttemptPosture::NotStarted,
            )
        })
    }
}

impl fmt::Debug for NoWriteHostedExecutionProvider {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NoWriteHostedExecutionProvider")
            .field("identity", &"[REDACTED]")
            .finish()
    }
}

impl HostedExecutionProvider for NoWriteHostedExecutionProvider {
    fn provider_id(&self) -> &HostedExecutionProviderId {
        &self.provider_id
    }

    fn provider_version(&self) -> &HostedExecutionProviderVersion {
        &self.provider_version
    }

    fn configuration_hash(&self) -> &SpecContentHash {
        &self.configuration_hash
    }

    fn validate_request(
        &self,
        request: &HostedExecutionRequest,
    ) -> Result<(), HostedExecutionInvocationError> {
        Self::validate_no_write_request(request)
    }

    fn execution_id(
        &self,
        request: &HostedExecutionRequest,
    ) -> Result<HostedExecutionId, HostedExecutionInvocationError> {
        Self::no_write_execution_id(request)
    }

    fn execute(
        &self,
        request: &HostedExecutionRequest,
    ) -> Result<HostedExecutionReceipt, HostedExecutionInvocationError> {
        self.validate_request(request)?;
        let now = Timestamp::now_utc();
        HostedExecutionReceipt::new(
            self.execution_id(request)?,
            self.provider_id.clone(),
            self.provider_version.clone(),
            self.configuration_hash.clone(),
            request.fingerprint(),
            HostedExecutionReference::new(
                HostedExecutionReferenceKind::Telemetry,
                "environment/no-write-alpha",
            )
            .map_err(|_| {
                HostedExecutionInvocationError::new(
                    HostedExecutionErrorCategory::Protocol,
                    HostedExecutionAttemptPosture::NotStarted,
                )
            })?,
            request.policy().policy_hash().clone(),
            now,
            now,
            HostedExecutionStatus::Completed,
            None,
            Some(0),
            vec![HostedExecutionReference::new(
                HostedExecutionReferenceKind::Telemetry,
                "telemetry/no-write-validation",
            )
            .map_err(|_| {
                HostedExecutionInvocationError::new(
                    HostedExecutionErrorCategory::Protocol,
                    HostedExecutionAttemptPosture::NotStarted,
                )
            })?],
        )
        .map_err(|_| {
            HostedExecutionInvocationError::new(
                HostedExecutionErrorCategory::Protocol,
                HostedExecutionAttemptPosture::NotStarted,
            )
        })
    }
}

fn no_write_dispatch_inputs() -> Result<HostedNoWriteDispatchInputs, WorkflowOsError> {
    Ok(HostedNoWriteDispatchInputs {
        catalog_entry_id: HostedCatalogEntryId::new("catalog/no-write-alpha")?,
        policy: HostedExecutionPolicyBinding::new(
            HostedExecutionPolicyId::new("policy/no-write-alpha")?,
            SpecContentHash::from_text("workflow-os.no-write-hosted-policy.v1"),
        ),
        budget: HostedExecutionBudget::new(60, 1024 * 1024)?,
        project_scope: None,
    })
}

/// Stateless fenced worker for one explicitly injected hosted provider.
pub struct HostedWorker {
    backend: PostgresStateBackend,
    worker: ActorId,
    provider: Arc<dyn HostedExecutionProvider>,
    lease_ttl: Duration,
    require_project_binding: bool,
}

/// Bounded outcome from processing one hosted work item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HostedWorkerOutcome {
    /// The inert provider produced an exactly bound receipt.
    Receipt(Box<HostedExecutionReceipt>),
    /// The request was rejected before any provider action started.
    RejectedBeforeStart,
    /// The provider may have started and operator reconciliation is required.
    ReconciliationRequired,
    /// The authoritative run was canceled before provider invocation.
    CanceledBeforeStart,
}

impl HostedWorker {
    /// Creates a stateless hosted worker.
    #[must_use]
    pub fn new(
        backend: PostgresStateBackend,
        worker: ActorId,
        provider: Arc<dyn HostedExecutionProvider>,
        lease_ttl: Duration,
    ) -> Self {
        Self {
            backend,
            worker,
            provider,
            lease_ttl,
            require_project_binding: false,
        }
    }

    /// Creates a collaborative worker that rejects every unbound work item.
    #[must_use]
    pub fn new_collaborative(
        backend: PostgresStateBackend,
        worker: ActorId,
        provider: Arc<dyn HostedExecutionProvider>,
        lease_ttl: Duration,
    ) -> Self {
        Self {
            backend,
            worker,
            provider,
            lease_ttl,
            require_project_binding: true,
        }
    }

    /// Claims and projects at most one no-write hosted work item.
    ///
    /// # Errors
    ///
    /// Fails closed when the provider request is unsafe, the receipt is
    /// invalid, or the fenced atomic receipt/run commit fails.
    pub fn run_once(&self) -> Result<Option<HostedWorkerOutcome>, WorkflowOsError> {
        let Some(claimed) =
            self.backend
                .claim_next_hosted_work_item(PostgresClaimHostedWorkItemRequest {
                    worker: &self.worker,
                    lease_ttl: self.lease_ttl,
                })?
        else {
            return Ok(None);
        };
        if let Some(outcome) = self.validate_claimed_context(&claimed)? {
            return Ok(Some(outcome));
        }
        if let Some(outcome) = self.reject_unsafe_request(&claimed)? {
            return Ok(Some(outcome));
        }
        self.invoke_claimed(&claimed).map(Some)
    }

    fn validate_claimed_context(
        &self,
        claimed: &PostgresClaimedHostedWorkItem,
    ) -> Result<Option<HostedWorkerOutcome>, WorkflowOsError> {
        let work_item = claimed.work_item().value();
        let work_item_binding = self.backend.read_hosted_project_resource_binding(
            HostedProjectResourceKind::WorkItem,
            work_item.work_item_id().as_str(),
        )?;
        if self.require_project_binding && work_item_binding.is_none() {
            return Err(WorkflowOsError::invalid_state(
                "hosted.worker.project_binding.missing",
                "collaborative hosted work item project binding is missing",
            ));
        }
        if let Some(work_item_binding) = work_item_binding {
            if work_item_binding.value().status() != HostedProjectResourceBindingStatus::Active {
                return Err(WorkflowOsError::invalid_state(
                    "hosted.worker.project_binding.inactive",
                    "hosted work item project binding is not active",
                ));
            }
            let run_binding = self
                .backend
                .read_hosted_project_resource_binding(
                    HostedProjectResourceKind::Run,
                    work_item.run_id().as_str(),
                )?
                .ok_or_else(|| {
                    WorkflowOsError::invalid_state(
                        "hosted.worker.project_binding.missing",
                        "hosted run project binding is missing",
                    )
                })?;
            if run_binding.value().status() != HostedProjectResourceBindingStatus::Active
                || run_binding.value().scope() != work_item_binding.value().scope()
            {
                return Err(WorkflowOsError::invalid_state(
                    "hosted.worker.project_binding.mismatch",
                    "hosted run and work item project bindings do not match",
                ));
            }
        }
        let run = self.backend.rehydrate_run(work_item.run_id())?;
        if run.snapshot.status != workflow_core::WorkflowRunStatus::Running {
            if run.snapshot.status == workflow_core::WorkflowRunStatus::Canceled {
                let canceled =
                    work_item.transition(HostedWorkItemStatus::Canceled, Timestamp::now_utc())?;
                self.backend.transition_hosted_work_item(
                    PostgresTransitionHostedWorkItemRequest {
                        expected_revision: claimed.work_item().revision(),
                        work_item: &canceled,
                        lease: Some(claimed.lease()),
                    },
                )?;
                return Ok(Some(HostedWorkerOutcome::CanceledBeforeStart));
            }
            return Err(WorkflowOsError::invalid_state(
                "hosted.worker.run.not_eligible",
                "hosted governed run is not eligible for provider invocation",
            ));
        }
        let binding = run
            .snapshot
            .identity
            .immutable_run_bundle
            .as_ref()
            .ok_or_else(|| {
                WorkflowOsError::invalid_state(
                    "hosted.worker.bundle_binding.missing",
                    "hosted governed run is missing immutable bundle identity",
                )
            })?;
        if binding.bundle_id() != work_item.bundle_id()
            || binding.bundle_version() != work_item.bundle_version()
            || binding.root_hash() != work_item.bundle_root_hash()
        {
            return Err(WorkflowOsError::invalid_state(
                "hosted.worker.bundle_binding.invalid",
                "hosted governed run bundle identity is invalid",
            ));
        }
        Ok(None)
    }

    fn reject_unsafe_request(
        &self,
        claimed: &PostgresClaimedHostedWorkItem,
    ) -> Result<Option<HostedWorkerOutcome>, WorkflowOsError> {
        let work_item = claimed.work_item().value();
        if let Err(error) = self
            .provider
            .validate_request(work_item.execution_request())
        {
            if error.attempt_posture() != HostedExecutionAttemptPosture::NotStarted {
                return Err(HostedExecutionInvocationError::into_workflow_error(error));
            }
            let occurred_at = Timestamp::now_utc();
            let failed = work_item.transition(HostedWorkItemStatus::Failed, occurred_at)?;
            let current_run = self.backend.rehydrate_run(work_item.run_id())?;
            let projection = HostedUnreceiptedResultProjection::new(
                &current_run,
                work_item,
                HostedUnreceiptedOutcome::RejectedBeforeStart,
                self.worker.clone(),
                occurred_at,
            )?;
            self.backend.commit_hosted_unreceipted_projection(
                PostgresCommitHostedUnreceiptedProjectionRequest {
                    expected_work_item_revision: claimed.work_item().revision(),
                    work_item: &failed,
                    expected_attempt_revision: None,
                    lease: claimed.lease(),
                    projection: &projection,
                },
            )?;
            return Ok(Some(HostedWorkerOutcome::RejectedBeforeStart));
        }
        Ok(None)
    }

    fn invoke_claimed(
        &self,
        claimed: &PostgresClaimedHostedWorkItem,
    ) -> Result<HostedWorkerOutcome, WorkflowOsError> {
        let work_item = claimed.work_item().value();
        let execution_id = self
            .provider
            .execution_id(work_item.execution_request())
            .map_err(HostedExecutionInvocationError::into_workflow_error)?;
        let prepared = self.backend.prepare_hosted_execution_attempt(
            claimed.work_item().revision(),
            work_item.work_item_id(),
            &execution_id,
            self.provider.provider_id(),
            self.provider.provider_version(),
            self.provider.configuration_hash(),
            claimed.lease(),
        )?;
        let invoking = self.backend.mark_hosted_execution_attempt_invoking(
            work_item.work_item_id(),
            prepared.revision(),
            claimed.lease(),
        )?;
        let Ok(receipt) =
            invoke_hosted_execution_provider(self.provider.as_ref(), work_item.execution_request())
        else {
            let occurred_at = Timestamp::now_utc();
            let ambiguous = work_item.transition(HostedWorkItemStatus::Ambiguous, occurred_at)?;
            let current_run = self.backend.rehydrate_run(work_item.run_id())?;
            let projection = HostedUnreceiptedResultProjection::new(
                &current_run,
                work_item,
                HostedUnreceiptedOutcome::ReconciliationRequired,
                self.worker.clone(),
                occurred_at,
            )?;
            self.backend.commit_hosted_unreceipted_projection(
                PostgresCommitHostedUnreceiptedProjectionRequest {
                    expected_work_item_revision: claimed.work_item().revision(),
                    work_item: &ambiguous,
                    expected_attempt_revision: Some(invoking.revision()),
                    lease: claimed.lease(),
                    projection: &projection,
                },
            )?;
            return Ok(HostedWorkerOutcome::ReconciliationRequired);
        };
        let terminal = work_item.transition(
            hosted_terminal_work_item_status(receipt.status()),
            receipt.terminal_at(),
        )?;
        let current_run = self.backend.rehydrate_run(work_item.run_id())?;
        let projection = HostedTerminalResultProjection::new(
            &current_run,
            work_item,
            receipt.clone(),
            self.worker.clone(),
        )?;
        let report_artifact =
            HostedTerminalReportArtifact::derive(&projection, work_item, self.worker.clone())?;
        self.backend.commit_hosted_receipt_and_projection(
            PostgresCommitHostedReceiptProjectionRequest {
                receipt_commit: PostgresCommitHostedReceiptRequest {
                    expected_work_item_revision: claimed.work_item().revision(),
                    work_item: &terminal,
                    receipt: &receipt,
                    lease: claimed.lease(),
                },
                expected_attempt_revision: invoking.revision(),
                projection: &projection,
                report_artifact: &report_artifact,
            },
        )?;
        Ok(HostedWorkerOutcome::Receipt(Box::new(receipt)))
    }
}

const fn hosted_terminal_work_item_status(status: HostedExecutionStatus) -> HostedWorkItemStatus {
    match status {
        HostedExecutionStatus::Completed => HostedWorkItemStatus::Completed,
        HostedExecutionStatus::Failed => HostedWorkItemStatus::Failed,
        HostedExecutionStatus::Canceled => HostedWorkItemStatus::Canceled,
        HostedExecutionStatus::Ambiguous => HostedWorkItemStatus::Ambiguous,
    }
}

fn looks_secret_like(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
        "password",
        "private_key",
        "private-key",
        "api_key",
        "api-key",
        "secret",
        "token",
        "credential",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::panic)]

    use super::*;
    use axum::body::Body;
    use axum::http::{Method, Request};
    use tower::ServiceExt;
    use workflow_core::{
        compute_approval_presentation_content_hash, ApprovalPresentationChannel,
        ApprovalPresentationId, ApprovalPresentationRecordDefinition,
        ApprovalPresentationSensitivity, CorrelationId, HostedExecutionBudget,
        HostedExecutionPolicyBinding, HostedExecutionPolicyId, HostedPrincipalKind,
        HostedProjectCapability, HostedProjectGrant, HostedProjectResourceKind, IdempotencyKey,
        ImmutableRunBundleId, ImmutableRunBundleVersion, OrganizationId, ProjectId,
        RedactionDisposition, RedactionFieldState, RedactionMetadata, SchemaVersion, StepId,
        WorkflowId, WorkflowRunId, WorkflowVersion,
    };

    struct RejectingFactory;

    impl workflow_core::PostgresConnectionFactory for RejectingFactory {
        fn connect(&self) -> Result<postgres::Client, WorkflowOsError> {
            Err(WorkflowOsError::invalid_state(
                "test.postgres.unavailable",
                "test PostgreSQL is unavailable",
            ))
        }
    }

    fn state() -> HostedApiState {
        HostedApiState::new(
            PostgresStateBackend::new(Arc::new(RejectingFactory)),
            HostedApiAuth::new(
                HostedAuthTokenDigest::from_token("test-value-123")
                    .unwrap_or_else(|error| panic!("{error}")),
                ActorId::new("user/test").unwrap_or_else(|error| panic!("{error}")),
            ),
            "test-build",
            ".",
        )
        .unwrap_or_else(|error| panic!("{error}"))
    }

    fn collaborative_state(
        backend: PostgresStateBackend,
    ) -> (CollaborativeHostedApiState, String, String) {
        let organization =
            OrganizationId::new("collaborative-test").unwrap_or_else(|error| panic!("{error}"));
        let project_a = ProjectId::new("collaborative-a").unwrap_or_else(|error| panic!("{error}"));
        let project_b = ProjectId::new("collaborative-b").unwrap_or_else(|error| panic!("{error}"));
        let source_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/vertical-slice-approval");
        let root = std::env::temp_dir().join("workflow-os-collaborative-project-a");
        if root.exists() {
            std::fs::remove_dir_all(&root).unwrap_or_else(|error| panic!("{error}"));
        }
        copy_test_project(&source_root, &root);
        let manifest_path = root.join("workflow-os.yml");
        let manifest = std::fs::read_to_string(&manifest_path)
            .unwrap_or_else(|error| panic!("{error}"))
            .replace("examples/vertical-slice-approval", "collaborative-a");
        std::fs::write(manifest_path, manifest).unwrap_or_else(|error| panic!("{error}"));
        let shadow_root = std::env::temp_dir().join("workflow-os-collaborative-project-b");
        std::fs::create_dir_all(&shadow_root).unwrap_or_else(|error| panic!("{error}"));
        let projects = HostedProjectRegistry::new(vec![
            HostedProjectRegistration::new(project_a.clone(), root)
                .unwrap_or_else(|error| panic!("{error}")),
            HostedProjectRegistration::new(project_b.clone(), shadow_root)
                .unwrap_or_else(|error| panic!("{error}")),
        ])
        .unwrap_or_else(|error| panic!("{error}"));
        let runner_token = "collaborative-runner-test-value".to_owned();
        let reviewer_token = "collaborative-reviewer-test-value".to_owned();
        let runner = HostedPrincipalBinding::new(
            ActorId::new("user/collaborative-runner").unwrap_or_else(|error| panic!("{error}")),
            organization.clone(),
            HostedPrincipalKind::Human,
            vec![HostedProjectGrant::new(
                project_a.clone(),
                vec![
                    HostedProjectCapability::RunCreate,
                    HostedProjectCapability::RunRead,
                ],
            )
            .unwrap_or_else(|error| panic!("{error}"))],
        )
        .unwrap_or_else(|error| panic!("{error}"));
        let reviewer = HostedPrincipalBinding::new(
            ActorId::new("user/collaborative-reviewer").unwrap_or_else(|error| panic!("{error}")),
            organization.clone(),
            HostedPrincipalKind::Human,
            vec![
                HostedProjectGrant::new(
                    project_a,
                    vec![
                        HostedProjectCapability::RunRead,
                        HostedProjectCapability::ApprovalRead,
                        HostedProjectCapability::ApprovalDecide,
                    ],
                )
                .unwrap_or_else(|error| panic!("{error}")),
                HostedProjectGrant::new(project_b, vec![HostedProjectCapability::RunRead])
                    .unwrap_or_else(|error| panic!("{error}")),
            ],
        )
        .unwrap_or_else(|error| panic!("{error}"));
        let principals = HostedPrincipalRegistry::new(
            &organization,
            &projects,
            vec![
                HostedPrincipalCredential::new(
                    HostedAuthTokenDigest::from_token(&runner_token)
                        .unwrap_or_else(|error| panic!("{error}")),
                    runner,
                ),
                HostedPrincipalCredential::new(
                    HostedAuthTokenDigest::from_token(&reviewer_token)
                        .unwrap_or_else(|error| panic!("{error}")),
                    reviewer,
                ),
            ],
        )
        .unwrap_or_else(|error| panic!("{error}"));
        (
            CollaborativeHostedApiState::new(
                backend,
                organization,
                projects,
                principals,
                "collaborative-test-build",
            )
            .unwrap_or_else(|error| panic!("{error}")),
            runner_token,
            reviewer_token,
        )
    }

    fn copy_test_project(source: &FsPath, destination: &FsPath) {
        std::fs::create_dir_all(destination).unwrap_or_else(|error| panic!("{error}"));
        for entry in std::fs::read_dir(source).unwrap_or_else(|error| panic!("{error}")) {
            let entry = entry.unwrap_or_else(|error| panic!("{error}"));
            let target = destination.join(entry.file_name());
            if entry.path().is_dir() {
                copy_test_project(&entry.path(), &target);
            } else {
                std::fs::copy(entry.path(), target).unwrap_or_else(|error| panic!("{error}"));
            }
        }
    }

    #[tokio::test]
    #[allow(clippy::too_many_lines)]
    async fn collaborative_project_boundary() {
        let Ok(value) = std::env::var("WORKFLOW_OS_TEST_POSTGRES_URL") else {
            assert!(
                std::env::var_os("WORKFLOW_OS_REQUIRE_POSTGRES_TESTS").is_none(),
                "WORKFLOW_OS_TEST_POSTGRES_URL is required"
            );
            return;
        };
        let config: postgres::Config = value
            .parse()
            .unwrap_or_else(|error| panic!("invalid test PostgreSQL URL: {error}"));
        let mut client = config
            .connect(postgres::NoTls)
            .unwrap_or_else(|error| panic!("{error}"));
        client
            .batch_execute("DROP SCHEMA IF EXISTS workflow_os CASCADE")
            .unwrap_or_else(|error| panic!("{error}"));
        let backend = PostgresStateBackend::new(Arc::new(
            workflow_core::PostgresNoTlsConnectionFactory::new(config),
        ));
        backend
            .initialize_schema()
            .unwrap_or_else(|error| panic!("{error}"));
        let (state, runner_token, reviewer_token) = collaborative_state(backend.clone());
        let app = collaborative_hosted_router(state);
        let run_id = WorkflowRunId::new("run-collaborative-boundary")
            .unwrap_or_else(|error| panic!("{error}"));
        let create_body = serde_json::json!({
            "run_id": run_id,
            "workflow_id": "ex/review",
            "bundle_id": "bundle-collaborative-boundary",
            "bundle_version": "v1",
            "created_at": "2026-08-13T00:00:00Z",
            "correlation_id": "correlation-collaborative-boundary",
            "idempotency_key": "collaborative-boundary-create",
            "sensitivity": "internal",
            "redaction_required": true
        });
        let create = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/v0alpha1/organizations/collaborative-test/projects/collaborative-a/runs")
                    .header(header::AUTHORIZATION, format!("Bearer {runner_token}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(create_body.to_string()))
                    .unwrap_or_else(|error| panic!("{error}")),
            )
            .await
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(create.status(), StatusCode::CREATED);
        let create_bytes = axum::body::to_bytes(create.into_body(), MAX_API_BODY_BYTES)
            .await
            .unwrap_or_else(|error| panic!("{error}"));
        let run: WorkflowRun =
            serde_json::from_slice(&create_bytes).unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(
            run.snapshot.status,
            workflow_core::WorkflowRunStatus::WaitingForApproval
        );
        let approval = run
            .snapshot
            .approval_requests
            .first()
            .unwrap_or_else(|| panic!("approval is required"));

        let approval_uri = format!(
            "/api/v0alpha1/organizations/collaborative-test/projects/collaborative-a/runs/{}/approvals/{}",
            run_id, approval.approval_id
        );
        let read = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri(&approval_uri)
                    .header(header::AUTHORIZATION, format!("Bearer {reviewer_token}"))
                    .body(Body::empty())
                    .unwrap_or_else(|error| panic!("{error}")),
            )
            .await
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(read.status(), StatusCode::OK);

        let strict_non_goals = vec!["no provider writes".to_owned()];
        let touched_surfaces = vec!["project-a run state".to_owned()];
        let validation_expectations = vec!["project scope remains exact".to_owned()];
        let channel = ApprovalPresentationChannel::Terminal;
        let sensitivity = ApprovalPresentationSensitivity::Internal;
        let content_hash = compute_approval_presentation_content_hash(
            &approval.run_id,
            &approval.approval_id,
            &approval.workflow_id,
            Some(&approval.workflow_version),
            Some(&approval.schema_version),
            approval.step_id.as_ref(),
            "approve collaborative project step",
            "review project-scoped execution",
            "project A approval only",
            &strict_non_goals,
            &touched_surfaces,
            &validation_expectations,
            "prove two-actor collaboration",
            "dispatch one no-write work item",
            &channel,
            sensitivity,
        )
        .unwrap_or_else(|error| panic!("{error}"));
        let presented_at = Timestamp::now_utc();
        let presentation = ApprovalPresentationRecord::new(ApprovalPresentationRecordDefinition {
            presentation_id: ApprovalPresentationId::new("presentation/collaborative-boundary")
                .unwrap_or_else(|error| panic!("{error}")),
            run_id: approval.run_id.clone(),
            approval_id: approval.approval_id.clone(),
            workflow_id: approval.workflow_id.clone(),
            workflow_version: Some(approval.workflow_version.clone()),
            schema_version: Some(approval.schema_version.clone()),
            step_id: approval.step_id.clone(),
            requested_action: "approve collaborative project step".to_owned(),
            work_summary: "review project-scoped execution".to_owned(),
            approved_scope: "project A approval only".to_owned(),
            strict_non_goals,
            expected_touched_surfaces: touched_surfaces,
            validation_expectations,
            why_now: "prove two-actor collaboration".to_owned(),
            next_action: "dispatch one no-write work item".to_owned(),
            presented_at,
            presented_by: ActorId::new("user/collaborative-reviewer")
                .unwrap_or_else(|error| panic!("{error}")),
            channel,
            content_hash,
            redaction: RedactionMetadata {
                redacted_fields: vec!["approval_context".to_owned()],
                field_states: vec![RedactionFieldState {
                    field: "approval_context".to_owned(),
                    disposition: RedactionDisposition::ReferenceOnly,
                    reason: "bounded project approval context".to_owned(),
                }],
            },
            sensitivity,
        })
        .unwrap_or_else(|error| panic!("{error}"));
        let decision = HostedApprovalDecisionRequest {
            decision: ApprovalDecisionKind::Granted,
            reason: "approved collaborative boundary test".to_owned(),
            correlation_id: CorrelationId::new("correlation-collaborative-decision")
                .unwrap_or_else(|error| panic!("{error}")),
            idempotency_key: IdempotencyKey::new("collaborative-boundary-decision")
                .unwrap_or_else(|error| panic!("{error}")),
            presentation,
            max_presentation_age_seconds: 86_400,
        };
        let decided = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(&approval_uri)
                    .header(header::AUTHORIZATION, format!("Bearer {reviewer_token}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&decision).unwrap_or_else(|error| panic!("{error}")),
                    ))
                    .unwrap_or_else(|error| panic!("{error}")),
            )
            .await
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(decided.status(), StatusCode::OK);

        let wrong_project = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri(format!(
                        "/api/v0alpha1/organizations/collaborative-test/projects/collaborative-b/runs/{run_id}"
                    ))
                    .header(header::AUTHORIZATION, format!("Bearer {reviewer_token}"))
                    .body(Body::empty())
                    .unwrap_or_else(|error| panic!("{error}")),
            )
            .await
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(wrong_project.status(), StatusCode::NOT_FOUND);

        let decisions = backend
            .list_hosted_project_access_decisions(&HostedProjectScope::new(
                OrganizationId::new("collaborative-test").unwrap_or_else(|error| panic!("{error}")),
                ProjectId::new("collaborative-b").unwrap_or_else(|error| panic!("{error}")),
            ))
            .unwrap_or_else(|error| panic!("{error}"));
        assert!(decisions.iter().any(|decision| !decision.allowed()));

        let legacy_token = "legacy-alpha-test-value";
        let legacy_state = HostedApiState::new(
            backend.clone(),
            HostedApiAuth::new(
                HostedAuthTokenDigest::from_token(legacy_token)
                    .unwrap_or_else(|error| panic!("{error}")),
                ActorId::new("user/legacy-alpha").unwrap_or_else(|error| panic!("{error}")),
            ),
            "legacy-alpha-test-build",
            std::env::temp_dir().join("workflow-os-collaborative-project-a"),
        )
        .unwrap_or_else(|error| panic!("{error}"));
        let legacy_read = hosted_router(legacy_state)
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri(format!("/api/v0alpha1/runs/{run_id}"))
                    .header(header::AUTHORIZATION, format!("Bearer {legacy_token}"))
                    .body(Body::empty())
                    .unwrap_or_else(|error| panic!("{error}")),
            )
            .await
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(legacy_read.status(), StatusCode::NOT_FOUND);

        let run_binding = backend
            .read_hosted_project_resource_binding(HostedProjectResourceKind::Run, run_id.as_str())
            .unwrap_or_else(|error| panic!("{error}"))
            .unwrap_or_else(|| panic!("run binding exists"));
        assert_eq!(
            run_binding.value().scope().project_id().as_str(),
            "collaborative-a"
        );
    }

    #[test]
    fn hosted_state_rejects_missing_server_project_root_without_leaking_path() {
        let path = std::env::temp_dir().join("workflow-os-hosted-private-missing-root");
        let error = HostedApiState::new(
            PostgresStateBackend::new(Arc::new(RejectingFactory)),
            HostedApiAuth::new(
                HostedAuthTokenDigest::from_token("test-value-123")
                    .unwrap_or_else(|error| panic!("{error}")),
                ActorId::new("user/test").unwrap_or_else(|error| panic!("{error}")),
            ),
            "test-build",
            &path,
        )
        .expect_err("missing project root rejected");
        assert_eq!(error.code(), "hosted.project_root.invalid");
        assert!(!error.to_string().contains("private-missing-root"));
    }

    #[test]
    fn hosted_mutation_intent_is_stable_operation_bound_and_payload_free() {
        let request = ("run-test", "private-reason-value");
        let first = hosted_mutation_intent("cancellation", &request)
            .unwrap_or_else(|error| panic!("{error}"));
        let replay = hosted_mutation_intent("cancellation", &request)
            .unwrap_or_else(|error| panic!("{error}"));
        let other_operation = hosted_mutation_intent("approval-decision", &request)
            .unwrap_or_else(|error| panic!("{error}"));

        assert_eq!(first, replay);
        assert_ne!(first, other_operation);
        assert!(!first.as_str().contains("private-reason-value"));
    }

    fn request_with_side_effects(
        approved_side_effects: Vec<workflow_core::SideEffectId>,
    ) -> HostedExecutionRequest {
        HostedExecutionRequest::new(
            WorkflowRunId::new("run-hosted-test").unwrap_or_else(|error| panic!("{error}")),
            WorkflowId::new("hosted/test").unwrap_or_else(|error| panic!("{error}")),
            WorkflowVersion::new("v1").unwrap_or_else(|error| panic!("{error}")),
            SchemaVersion::new("workflowos.dev/v0").unwrap_or_else(|error| panic!("{error}")),
            StepId::new("verify").unwrap_or_else(|error| panic!("{error}")),
            ImmutableRunBundleId::new("bundle-hosted-test")
                .unwrap_or_else(|error| panic!("{error}")),
            ImmutableRunBundleVersion::new("v1").unwrap_or_else(|error| panic!("{error}")),
            SpecContentHash::from_text("bundle"),
            Vec::new(),
            Vec::new(),
            approved_side_effects,
            HostedExecutionPolicyBinding::new(
                HostedExecutionPolicyId::new("policy/no-write")
                    .unwrap_or_else(|error| panic!("{error}")),
                SpecContentHash::from_text("policy"),
            ),
            HostedExecutionBudget::new(30, 1024).unwrap_or_else(|error| panic!("{error}")),
            CorrelationId::new("correlation-hosted-test").unwrap_or_else(|error| panic!("{error}")),
            IdempotencyKey::new("hosted-test").unwrap_or_else(|error| panic!("{error}")),
            Vec::new(),
        )
        .unwrap_or_else(|error| panic!("{error}"))
    }

    fn request() -> HostedExecutionRequest {
        request_with_side_effects(Vec::new())
    }

    #[tokio::test]
    async fn only_liveness_is_public() {
        let app = hosted_router(state());
        let live = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/health/live")
                    .body(Body::empty())
                    .unwrap_or_else(|error| panic!("{error}")),
            )
            .await
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(live.status(), StatusCode::OK);
        let readiness = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/health/ready")
                    .body(Body::empty())
                    .unwrap_or_else(|error| panic!("{error}")),
            )
            .await
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(readiness.status(), StatusCode::UNAUTHORIZED);
        let version = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/version")
                    .body(Body::empty())
                    .unwrap_or_else(|error| panic!("{error}")),
            )
            .await
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(version.status(), StatusCode::UNAUTHORIZED);
        let terminal_report = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/api/v0alpha1/runs/run-hosted-test/report")
                    .body(Body::empty())
                    .unwrap_or_else(|error| panic!("{error}")),
            )
            .await
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(terminal_report.status(), StatusCode::UNAUTHORIZED);
        let authorized = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/version")
                    .header(header::AUTHORIZATION, "Bearer test-value-123")
                    .body(Body::empty())
                    .unwrap_or_else(|error| panic!("{error}")),
            )
            .await
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(authorized.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn caller_authored_work_items_remain_absent_and_run_projection_is_exposed() {
        let app = hosted_router(state());
        let create = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/v0alpha1/work-items")
                    .header(header::AUTHORIZATION, "Bearer test-value-123")
                    .body(Body::empty())
                    .unwrap_or_else(|error| panic!("{error}")),
            )
            .await
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(create.status(), StatusCode::NOT_FOUND);
        let run = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/api/v0alpha1/runs/run-hosted-test")
                    .header(header::AUTHORIZATION, "Bearer test-value-123")
                    .body(Body::empty())
                    .unwrap_or_else(|error| panic!("{error}")),
            )
            .await
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(run.status(), StatusCode::CONFLICT);
        let report = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/api/v0alpha1/runs/run-hosted-test/report")
                    .header(header::AUTHORIZATION, "Bearer test-value-123")
                    .body(Body::empty())
                    .unwrap_or_else(|error| panic!("{error}")),
            )
            .await
            .unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(report.status(), StatusCode::CONFLICT);
    }

    #[test]
    fn no_write_provider_returns_exact_bound_receipt() {
        let provider =
            NoWriteHostedExecutionProvider::new().unwrap_or_else(|error| panic!("{error}"));
        let request = request();
        let receipt = invoke_hosted_execution_provider(&provider, &request)
            .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(receipt.status(), HostedExecutionStatus::Completed);
        assert_eq!(receipt.request_fingerprint(), &request.fingerprint());
        assert!(receipt
            .references()
            .iter()
            .all(|reference| reference.kind() == HostedExecutionReferenceKind::Telemetry));
    }

    #[test]
    fn no_write_provider_rejects_side_effects_before_invocation() {
        let request =
            request_with_side_effects(vec![workflow_core::SideEffectId::new("side-effect/write")
                .unwrap_or_else(|error| panic!("{error}"))]);
        let provider =
            NoWriteHostedExecutionProvider::new().unwrap_or_else(|error| panic!("{error}"));
        let error = provider
            .execute(&request)
            .expect_err("write-capable request must fail");
        assert_eq!(error.category(), HostedExecutionErrorCategory::Policy);
        assert_eq!(
            error.attempt_posture(),
            HostedExecutionAttemptPosture::NotStarted
        );
    }

    #[test]
    fn receipt_status_maps_to_exact_durable_work_item_posture() {
        assert_eq!(
            hosted_terminal_work_item_status(HostedExecutionStatus::Completed),
            HostedWorkItemStatus::Completed
        );
        assert_eq!(
            hosted_terminal_work_item_status(HostedExecutionStatus::Failed),
            HostedWorkItemStatus::Failed
        );
        assert_eq!(
            hosted_terminal_work_item_status(HostedExecutionStatus::Canceled),
            HostedWorkItemStatus::Canceled
        );
        assert_eq!(
            hosted_terminal_work_item_status(HostedExecutionStatus::Ambiguous),
            HostedWorkItemStatus::Ambiguous
        );
    }

    #[test]
    fn auth_debug_and_errors_do_not_leak_tokens() {
        let token = "test-value-123";
        let digest =
            HostedAuthTokenDigest::from_token(token).unwrap_or_else(|error| panic!("{error}"));
        let auth = HostedApiAuth::new(
            digest,
            ActorId::new("user/test").unwrap_or_else(|error| panic!("{error}")),
        );
        assert!(!format!("{auth:?}").contains(token));
        let headers = HeaderMap::new();
        let error = auth
            .authorize(&headers)
            .expect_err("missing auth must fail");
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn hosted_errors_expose_only_stable_core_codes() {
        let secret = "private-provider-payload-value";
        let error = WorkflowOsError::invalid_state(
            "executor.hosted_no_write.test_conflict",
            format!("request failed for {secret}"),
        );
        let hosted = HostedApiError::from_core(&error);

        assert_eq!(hosted.status, StatusCode::CONFLICT);
        assert_eq!(hosted.code, "executor.hosted_no_write.test_conflict");
        assert_eq!(hosted.message, "hosted request is invalid");
        assert!(!hosted.code.contains(secret));
        assert!(!hosted.message.contains(secret));
    }
}
