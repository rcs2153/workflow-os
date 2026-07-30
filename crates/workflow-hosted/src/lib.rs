//! Single-tenant hosted alpha transport and no-write worker.
//!
//! This crate is intentionally deployment-bound, local to one trust domain,
//! and not a production, multi-tenant, or general agent runtime.

use std::fmt;
use std::path::PathBuf;
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
    invoke_hosted_execution_provider, ActorId, ApprovalDecisionKind, ApprovalPresentationRecord,
    ApprovalPresentationRecordStore, ApprovalStore, CorrelationId, EventLogStore,
    HostedCatalogEntryId, HostedExecutionAttemptPosture, HostedExecutionBudget,
    HostedExecutionErrorCategory, HostedExecutionId, HostedExecutionInvocationError,
    HostedExecutionPolicyBinding, HostedExecutionPolicyId, HostedExecutionProvider,
    HostedExecutionProviderId, HostedExecutionProviderVersion, HostedExecutionReceipt,
    HostedExecutionReference, HostedExecutionReferenceKind, HostedExecutionRequest,
    HostedExecutionStatus, HostedNoWriteDispatchInputs, HostedTerminalReportArtifact,
    HostedTerminalResultProjection, HostedUnreceiptedOutcome, HostedUnreceiptedResultProjection,
    HostedWorkItem, HostedWorkItemId, HostedWorkItemStatus, IdempotencyKey, IdempotencyResult,
    IdempotencyStore, IdempotencyWrite, ImmutableRunBundleId, ImmutableRunBundleSensitivity,
    ImmutableRunBundleVersion, LocalApprovalDecisionRequest,
    LocalApprovalPresentationDecisionRequest, LocalApprovalPresentationProof,
    LocalCancellationRequest, LocalExecutionBeforeSkillInvocationCheckpointInputs,
    LocalExecutionImmutableRunBundleInputs, LocalExecutionRequest,
    LocalExecutionWithHostedDispatchRequest, LocalExecutionWithImmutableRunBundleRequest,
    LocalExecutor, LocalSkillRegistry, PostgresClaimHostedWorkItemRequest,
    PostgresClaimedHostedWorkItem, PostgresCommitHostedReceiptProjectionRequest,
    PostgresCommitHostedReceiptRequest, PostgresCommitHostedUnreceiptedProjectionRequest,
    PostgresStateBackend, PostgresTransitionHostedWorkItemRequest, SpecContentHash, StateBackend,
    Timestamp, WorkReportArtifactMetadata, WorkReportArtifactStore, WorkReportId, WorkReportStatus,
    WorkflowId, WorkflowOsError, WorkflowRun, WorkflowRunEvent, WorkflowRunId, WorkflowRunStatus,
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
    let run = tokio::task::spawn_blocking(move || backend.rehydrate_run(&run_id))
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
    let events = tokio::task::spawn_blocking(move || backend.read_events(&run_id))
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
        backend.read_revisioned_hosted_work_item(&work_item_id)
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
    code: &'static str,
    message: &'static str,
}

struct HostedApiError {
    status: StatusCode,
    code: &'static str,
    message: &'static str,
}

impl HostedApiError {
    const fn bad_request() -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "hosted.request.invalid",
            message: "hosted request is invalid",
        }
    }

    const fn unauthorized() -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: "hosted.auth.unauthorized",
            message: "hosted API authentication failed",
        }
    }

    const fn not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "hosted.resource.not_found",
            message: "hosted resource was not found",
        }
    }

    const fn unavailable() -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "hosted.dependency.unavailable",
            message: "hosted dependency is unavailable",
        }
    }

    const fn internal() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "hosted.internal",
            message: "hosted request failed",
        }
    }

    fn from_core(error: &WorkflowOsError) -> Self {
        let status = match error.kind() {
            workflow_core::WorkflowOsErrorKind::Parse
            | workflow_core::WorkflowOsErrorKind::Validation
            | workflow_core::WorkflowOsErrorKind::Security => StatusCode::BAD_REQUEST,
            workflow_core::WorkflowOsErrorKind::PolicyDenied => StatusCode::FORBIDDEN,
            workflow_core::WorkflowOsErrorKind::Unsupported => StatusCode::NOT_IMPLEMENTED,
            workflow_core::WorkflowOsErrorKind::InvalidState => StatusCode::CONFLICT,
            workflow_core::WorkflowOsErrorKind::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let (code, message) = if status == StatusCode::INTERNAL_SERVER_ERROR {
            ("hosted.internal", "hosted request failed")
        } else {
            ("hosted.request.invalid", "hosted request is invalid")
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

    fn validate_request(
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

    fn execution_id(
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

    fn execute(
        &self,
        request: &HostedExecutionRequest,
    ) -> Result<HostedExecutionReceipt, HostedExecutionInvocationError> {
        Self::validate_request(request)?;
        let now = Timestamp::now_utc();
        HostedExecutionReceipt::new(
            Self::execution_id(request)?,
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
    })
}

/// Stateless fenced worker for the no-write hosted proof.
pub struct HostedWorker {
    backend: PostgresStateBackend,
    worker: ActorId,
    provider: Arc<NoWriteHostedExecutionProvider>,
    lease_ttl: Duration,
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
        provider: Arc<NoWriteHostedExecutionProvider>,
        lease_ttl: Duration,
    ) -> Self {
        Self {
            backend,
            worker,
            provider,
            lease_ttl,
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
        if let Err(error) =
            NoWriteHostedExecutionProvider::validate_request(work_item.execution_request())
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
        let execution_id =
            NoWriteHostedExecutionProvider::execution_id(work_item.execution_request())
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
        CorrelationId, HostedExecutionBudget, HostedExecutionPolicyBinding,
        HostedExecutionPolicyId, IdempotencyKey, ImmutableRunBundleId, ImmutableRunBundleVersion,
        SchemaVersion, StepId, WorkflowId, WorkflowRunId, WorkflowVersion,
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
}
