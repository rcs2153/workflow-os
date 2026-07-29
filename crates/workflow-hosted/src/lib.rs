//! Single-tenant hosted alpha transport and no-write worker.
//!
//! This crate is intentionally deployment-bound, local to one trust domain,
//! and not a production, multi-tenant, or general agent runtime.

use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{DefaultBodyLimit, Path, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use workflow_core::{
    invoke_hosted_execution_provider, ActorId, HostedExecutionAttemptPosture,
    HostedExecutionErrorCategory, HostedExecutionId, HostedExecutionInvocationError,
    HostedExecutionProvider, HostedExecutionProviderId, HostedExecutionProviderVersion,
    HostedExecutionReceipt, HostedExecutionReference, HostedExecutionReferenceKind,
    HostedExecutionRequest, HostedExecutionStatus, HostedWorkItem, HostedWorkItemId,
    HostedWorkItemStatus, PostgresClaimHostedWorkItemRequest, PostgresCommitHostedReceiptRequest,
    PostgresStateBackend, PostgresTransitionHostedWorkItemRequest, SpecContentHash, Timestamp,
    WorkflowOsError,
};

const MAX_API_BODY_BYTES: usize = 64 * 1024;

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
        Ok(Self {
            backend,
            auth,
            build_id,
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
            .finish()
    }
}

/// Builds the single-tenant hosted alpha router.
pub fn hosted_router(state: HostedApiState) -> Router {
    Router::new()
        .route("/health/live", get(liveness))
        .route("/health/ready", get(readiness))
        .route("/version", get(version))
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
        let now = Timestamp::now_utc();
        HostedExecutionReceipt::new(
            HostedExecutionId::new(format!(
                "execution-{}",
                request.fingerprint().as_hash().as_str()
            ))
            .map_err(|_| {
                HostedExecutionInvocationError::new(
                    HostedExecutionErrorCategory::Protocol,
                    HostedExecutionAttemptPosture::NotStarted,
                )
            })?,
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

    /// Claims and records a receipt for at most one no-write hosted work item.
    ///
    /// # Errors
    ///
    /// Fails closed when the provider request is unsafe, the receipt is
    /// invalid, or the fenced durable commit fails. This proof does not append
    /// workflow events or mutate the governed run projection.
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
        let work_item = claimed.work_item().value();
        let receipt = match invoke_hosted_execution_provider(
            self.provider.as_ref(),
            work_item.execution_request(),
        ) {
            Ok(receipt) => receipt,
            Err(error) if error.attempt_posture() == HostedExecutionAttemptPosture::NotStarted => {
                let failed =
                    work_item.transition(HostedWorkItemStatus::Failed, Timestamp::now_utc())?;
                self.backend.transition_hosted_work_item(
                    PostgresTransitionHostedWorkItemRequest {
                        expected_revision: claimed.work_item().revision(),
                        work_item: &failed,
                        lease: Some(claimed.lease()),
                    },
                )?;
                return Ok(Some(HostedWorkerOutcome::RejectedBeforeStart));
            }
            Err(error) => return Err(HostedExecutionInvocationError::into_workflow_error(error)),
        };
        let completed =
            work_item.transition(HostedWorkItemStatus::Completed, receipt.terminal_at())?;
        self.backend
            .commit_hosted_receipt(PostgresCommitHostedReceiptRequest {
                expected_work_item_revision: claimed.work_item().revision(),
                work_item: &completed,
                receipt: &receipt,
                lease: claimed.lease(),
            })?;
        Ok(Some(HostedWorkerOutcome::Receipt(Box::new(receipt))))
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
        )
        .unwrap_or_else(|error| panic!("{error}"))
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
    async fn remote_creation_and_run_projection_routes_are_not_exposed() {
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
        assert_eq!(run.status(), StatusCode::NOT_FOUND);
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
