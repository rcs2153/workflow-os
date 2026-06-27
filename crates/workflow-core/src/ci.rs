use std::collections::BTreeMap;
use std::env;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    ActorId, AdapterCapability, AdapterError, AdapterErrorKind, AdapterHealth, AdapterId,
    AdapterInvocationRecord, AdapterKind, AdapterObservabilityRecord, AdapterOperationMode,
    AdapterPolicyPrecheck, AdapterReadOperation, AdapterRedactionPolicy, AdapterRequest,
    AdapterResponse, AdapterTimeoutPolicy, CorrelationId, RedactedValue, Timestamp,
};

const DEFAULT_BASE_URL: &str = "https://api.github.com";
const DEFAULT_TOKEN_ENV: &str = "WORKFLOW_OS_GITHUB_ACTIONS_TOKEN";
const FALLBACK_TOKEN_ENV: &str = "GITHUB_TOKEN";
const MAX_LOG_EXCERPT_BYTES: usize = 2_048;

/// Generic CI read-only adapter action names.
pub mod ci_actions {
    /// Read workflow run metadata.
    pub const WORKFLOW_RUN_METADATA: &str = "ci.workflow_run.metadata";
    /// Read workflow job summaries.
    pub const JOB_STATUS_SUMMARY: &str = "ci.jobs.summary";
    /// Read check status summaries.
    pub const CHECK_STATUS_SUMMARY: &str = "ci.checks.summary";
    /// Read normalized failure context summary.
    pub const FAILURE_SUMMARY: &str = "ci.failure.summary";
    /// Read log download references.
    pub const LOG_REFERENCE: &str = "ci.logs.reference";
    /// Read a limited, redacted log excerpt when explicitly requested.
    pub const LOG_EXCERPT: &str = "ci.logs.excerpt";
}

/// GitHub Actions read-only adapter configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubActionsReadOnlyConfig {
    /// Stable adapter ID.
    pub adapter_id: AdapterId,
    /// GitHub REST API base URL.
    pub base_url: String,
    /// Environment variable checked for the token.
    pub token_env_var: String,
    /// Optional redacted token value.
    pub token: Option<RedactedValue<String>>,
    /// Adapter operation mode.
    pub operation_mode: AdapterOperationMode,
}

impl GitHubActionsReadOnlyConfig {
    /// Loads GitHub Actions read-only configuration from environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error if the static adapter ID is invalid.
    pub fn from_env() -> Result<Self, AdapterError> {
        let token = env::var(DEFAULT_TOKEN_ENV)
            .or_else(|_| env::var(FALLBACK_TOKEN_ENV))
            .ok()
            .map(RedactedValue::new);

        Self::new(
            AdapterOperationMode::LiveReadOnly,
            token,
            DEFAULT_BASE_URL.to_owned(),
            DEFAULT_TOKEN_ENV.to_owned(),
        )
    }

    /// Creates a fixture-mode GitHub Actions configuration for tests.
    ///
    /// # Errors
    ///
    /// Returns an error if the static adapter ID is invalid.
    pub fn fixture() -> Result<Self, AdapterError> {
        Self::new(
            AdapterOperationMode::Fixture,
            None,
            DEFAULT_BASE_URL.to_owned(),
            DEFAULT_TOKEN_ENV.to_owned(),
        )
    }

    /// Creates a live read-only configuration with an already loaded token.
    ///
    /// The token is wrapped in `RedactedValue` and must not be logged.
    ///
    /// # Errors
    ///
    /// Returns an error if the static adapter ID is invalid.
    pub fn live_read_only_with_token(token: String) -> Result<Self, AdapterError> {
        Self::new(
            AdapterOperationMode::LiveReadOnly,
            Some(RedactedValue::new(token)),
            DEFAULT_BASE_URL.to_owned(),
            DEFAULT_TOKEN_ENV.to_owned(),
        )
    }

    fn new(
        operation_mode: AdapterOperationMode,
        token: Option<RedactedValue<String>>,
        base_url: String,
        token_env_var: String,
    ) -> Result<Self, AdapterError> {
        let adapter_id = AdapterId::new("adapter/github-actions-read-only").map_err(|error| {
            AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                "ci.github_actions.adapter_id.invalid",
                error.to_string(),
            )
        })?;

        Ok(Self {
            adapter_id,
            base_url,
            token_env_var,
            token,
            operation_mode,
        })
    }

    /// Returns true when a token is configured without exposing its value.
    #[must_use]
    pub fn credential_present(&self) -> bool {
        self.token.is_some()
    }
}

/// Minimal HTTP response returned by the GitHub Actions client abstraction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubActionsHttpResponse {
    /// HTTP status code.
    pub status: u16,
    /// Response body.
    pub body: String,
    /// Non-secret response headers.
    pub headers: BTreeMap<String, String>,
}

/// GitHub Actions HTTP client abstraction used by live and fixture clients.
pub trait GitHubActionsReadOnlyClient {
    /// Performs a read-only GET request.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the request cannot complete.
    fn get(&self, path: &str, accept: &str) -> Result<GitHubActionsHttpResponse, AdapterError>;
}

/// Live GitHub Actions REST client. Live use is opt-in through environment configuration.
#[derive(Clone, Debug)]
pub struct GitHubActionsLiveReadOnlyClient {
    config: GitHubActionsReadOnlyConfig,
}

impl GitHubActionsLiveReadOnlyClient {
    /// Creates a live GitHub Actions read-only client.
    #[must_use]
    pub const fn new(config: GitHubActionsReadOnlyConfig) -> Self {
        Self { config }
    }
}

impl GitHubActionsReadOnlyClient for GitHubActionsLiveReadOnlyClient {
    fn get(&self, path: &str, accept: &str) -> Result<GitHubActionsHttpResponse, AdapterError> {
        let url = format!("{}{}", self.config.base_url.trim_end_matches('/'), path);
        let mut request = ureq::get(&url)
            .set("accept", accept)
            .set("user-agent", "workflow-os/0.1.0-preview.1");

        if let Some(token) = &self.config.token {
            request = request.set(
                "authorization",
                &format!("Bearer {}", token.expose_secret()),
            );
        }

        match request.call() {
            Ok(response) => response_to_http(response),
            Err(ureq::Error::Status(status, response)) => {
                let headers = headers_from_response(&response);
                let body = response.into_string().unwrap_or_default();
                Err(classify_http_error(status, &headers, &body))
            }
            Err(ureq::Error::Transport(error)) => Err(AdapterError::new(
                AdapterErrorKind::TransientNetworkFailure,
                "ci.github_actions.network.transient",
                error.to_string(),
            )),
        }
    }
}

/// Fixture-backed GitHub Actions client for offline tests.
#[derive(Clone, Debug, Default)]
pub struct GitHubActionsFixtureClient {
    responses: BTreeMap<String, GitHubActionsHttpResponse>,
}

impl GitHubActionsFixtureClient {
    /// Creates an empty fixture client.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a fixture response for a path.
    #[must_use]
    pub fn with_json(mut self, path: impl Into<String>, body: impl Into<String>) -> Self {
        self.responses.insert(
            path.into(),
            GitHubActionsHttpResponse {
                status: 200,
                body: body.into(),
                headers: BTreeMap::new(),
            },
        );
        self
    }

    /// Adds a fixture text response for a path.
    #[must_use]
    pub fn with_text(mut self, path: impl Into<String>, body: impl Into<String>) -> Self {
        self.responses.insert(
            path.into(),
            GitHubActionsHttpResponse {
                status: 200,
                body: body.into(),
                headers: BTreeMap::new(),
            },
        );
        self
    }

    /// Adds a fixture error for a path.
    #[must_use]
    pub fn with_error(
        mut self,
        path: impl Into<String>,
        status: u16,
        body: impl Into<String>,
        headers: BTreeMap<String, String>,
    ) -> Self {
        self.responses.insert(
            path.into(),
            GitHubActionsHttpResponse {
                status,
                body: body.into(),
                headers,
            },
        );
        self
    }
}

impl GitHubActionsReadOnlyClient for GitHubActionsFixtureClient {
    fn get(&self, path: &str, _accept: &str) -> Result<GitHubActionsHttpResponse, AdapterError> {
        let Some(response) = self.responses.get(path).cloned() else {
            return Err(AdapterError::new(
                AdapterErrorKind::NotFound,
                "ci.github_actions.fixture.not_found",
                "fixture response was not found",
            ));
        };

        if (200..300).contains(&response.status) {
            Ok(response)
        } else {
            Err(classify_http_error(
                response.status,
                &response.headers,
                &response.body,
            ))
        }
    }
}

/// GitHub Actions read-only adapter.
#[derive(Clone, Debug)]
pub struct GitHubActionsReadOnlyAdapter<C> {
    config: GitHubActionsReadOnlyConfig,
    client: C,
}

impl<C> GitHubActionsReadOnlyAdapter<C>
where
    C: GitHubActionsReadOnlyClient,
{
    /// Creates a GitHub Actions read-only adapter from config and client.
    #[must_use]
    pub const fn new(config: GitHubActionsReadOnlyConfig, client: C) -> Self {
        Self { config, client }
    }

    /// Returns adapter health without exposing credential values.
    #[must_use]
    pub fn health(&self) -> AdapterHealth {
        AdapterHealth {
            adapter_id: self.config.adapter_id.clone(),
            adapter_kind: AdapterKind::Ci,
            operation_mode: self.config.operation_mode,
            configured: matches!(
                self.config.operation_mode,
                AdapterOperationMode::Fixture
                    | AdapterOperationMode::Mock
                    | AdapterOperationMode::Local
            ) || self.config.credential_present(),
            reachable: None,
            credential_present: self.config.credential_present(),
            last_checked_at: Timestamp::now_utc(),
            warnings: health_warnings(&self.config),
        }
    }

    /// Reads workflow run metadata.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when policy, fixture, parsing, or HTTP read fails.
    pub fn read_workflow_run_metadata(
        &self,
        request: &AdapterRequest,
        owner: &str,
        repo: &str,
        run_id: &str,
    ) -> Result<GitHubActionsReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/repos/{owner}/{repo}/actions/runs/{run_id}"),
            summarize_workflow_run,
        )
    }

    /// Reads workflow job status summaries.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_workflow_jobs(
        &self,
        request: &AdapterRequest,
        owner: &str,
        repo: &str,
        run_id: &str,
    ) -> Result<GitHubActionsReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/repos/{owner}/{repo}/actions/runs/{run_id}/jobs"),
            summarize_jobs,
        )
    }

    /// Reads check run status summaries for a commit ref.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_check_status_summary(
        &self,
        request: &AdapterRequest,
        owner: &str,
        repo: &str,
        reference: &str,
    ) -> Result<GitHubActionsReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/repos/{owner}/{repo}/commits/{reference}/check-runs"),
            summarize_checks,
        )
    }

    /// Produces a normalized CI failure summary.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_failure_summary(
        &self,
        request: &AdapterRequest,
        owner: &str,
        repo: &str,
        run_id: &str,
    ) -> Result<GitHubActionsReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/repos/{owner}/{repo}/actions/runs/{run_id}/jobs"),
            summarize_failures,
        )
    }

    /// Returns a log download reference without downloading logs.
    ///
    /// # Errors
    ///
    /// Returns an adapter error when the request is not a valid read request.
    pub fn read_log_reference(
        &self,
        request: &AdapterRequest,
        owner: &str,
        repo: &str,
        run_id: &str,
    ) -> Result<GitHubActionsReadOutcome, AdapterError> {
        Self::ensure_read_request(request)?;
        let summary = format!(
            "GitHub Actions run {run_id} logs=reference-only; download_requires_explicit_read=false"
        );
        Ok(Self::outcome(
            request,
            summary,
            vec![format!(
                "github-actions://{owner}/{repo}/actions/runs/{run_id}/logs"
            )],
            0,
        ))
    }

    /// Reads a limited, redacted job log excerpt.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_log_excerpt(
        &self,
        request: &AdapterRequest,
        owner: &str,
        repo: &str,
        job_id: &str,
    ) -> Result<GitHubActionsReadOutcome, AdapterError> {
        Self::ensure_read_request(request)?;
        let started = Instant::now();
        let response = self.client.get(
            &format!("/repos/{owner}/{repo}/actions/jobs/{job_id}/logs"),
            "text/plain",
        )?;
        let duration_ms = elapsed_ms(started);
        let (excerpt, truncated) = redacted_log_excerpt(&response.body);
        let summary = format!(
            "GitHub Actions job {job_id} log_excerpt={excerpt}; truncated={truncated}; full_log=reference-only"
        );
        Ok(Self::outcome(
            request,
            summary,
            vec![format!(
                "github-actions://{owner}/{repo}/actions/jobs/{job_id}/logs"
            )],
            duration_ms,
        ))
    }

    fn read_json_summary(
        &self,
        request: &AdapterRequest,
        path: &str,
        summarize: fn(&Value) -> CiSummary,
    ) -> Result<GitHubActionsReadOutcome, AdapterError> {
        Self::ensure_read_request(request)?;
        let started = Instant::now();
        let response = self.client.get(path, "application/vnd.github+json")?;
        let duration_ms = elapsed_ms(started);
        let value: Value = serde_json::from_str(&response.body).map_err(|error| {
            AdapterError::new(
                AdapterErrorKind::MalformedResponse,
                "ci.github_actions.response.malformed",
                error.to_string(),
            )
        })?;
        let summary = summarize(&value);
        Ok(Self::outcome(
            request,
            summary.summary,
            summary.references,
            duration_ms,
        ))
    }

    fn outcome(
        request: &AdapterRequest,
        summary: String,
        references: Vec<String>,
        duration_ms: u64,
    ) -> GitHubActionsReadOutcome {
        let response = AdapterResponse::redacted_summary(
            request.adapter_id.clone(),
            request.action.name.clone(),
            request.correlation_id.clone(),
            summary,
            references,
            duration_ms,
        );
        let invoked_at = Timestamp::now_utc();
        let invocation = AdapterInvocationRecord::from_response(request, &response, invoked_at);
        let observability = AdapterObservabilityRecord::from_invocation(&invocation);
        GitHubActionsReadOutcome {
            response,
            invocation,
            observability,
        }
    }

    fn ensure_read_request(request: &AdapterRequest) -> Result<(), AdapterError> {
        if request.adapter_kind != AdapterKind::Ci {
            return Err(AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                "ci.adapter_kind.invalid",
                "CI read-only adapter requires adapter kind ci",
            ));
        }
        if request.capability != AdapterCapability::CiRead {
            return Err(AdapterError::new(
                AdapterErrorKind::PermissionFailure,
                "ci.capability.read_required",
                "CI read-only adapter requires ci.read capability",
            ));
        }
        request.validate_preconditions()
    }
}

impl<C> AdapterReadOperation for GitHubActionsReadOnlyAdapter<C>
where
    C: GitHubActionsReadOnlyClient,
{
    fn read(&self, request: &AdapterRequest) -> Result<AdapterResponse, AdapterError> {
        Self::ensure_read_request(request)?;
        let owner = metadata_value(request, "owner")?;
        let repo = metadata_value(request, "repo")?;
        let outcome = match request.action.name.as_str() {
            ci_actions::WORKFLOW_RUN_METADATA => self.read_workflow_run_metadata(
                request,
                owner,
                repo,
                metadata_value(request, "run_id")?,
            )?,
            ci_actions::JOB_STATUS_SUMMARY => {
                self.read_workflow_jobs(request, owner, repo, metadata_value(request, "run_id")?)?
            }
            ci_actions::CHECK_STATUS_SUMMARY => self.read_check_status_summary(
                request,
                owner,
                repo,
                metadata_value(request, "ref")?,
            )?,
            ci_actions::FAILURE_SUMMARY => {
                self.read_failure_summary(request, owner, repo, metadata_value(request, "run_id")?)?
            }
            ci_actions::LOG_REFERENCE => {
                self.read_log_reference(request, owner, repo, metadata_value(request, "run_id")?)?
            }
            ci_actions::LOG_EXCERPT => {
                self.read_log_excerpt(request, owner, repo, metadata_value(request, "job_id")?)?
            }
            _ => {
                return Err(AdapterError::new(
                    AdapterErrorKind::UnsupportedOperation,
                    "ci.github_actions.action.unsupported",
                    "unsupported GitHub Actions read-only adapter action",
                ));
            }
        };
        Ok(outcome.response)
    }
}

/// Result of a GitHub Actions read including adapter response and derived telemetry records.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GitHubActionsReadOutcome {
    /// Normalized adapter response.
    pub response: AdapterResponse,
    /// Audit-safe adapter invocation record.
    pub invocation: AdapterInvocationRecord,
    /// Adapter observability record.
    pub observability: AdapterObservabilityRecord,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CiSummary {
    summary: String,
    references: Vec<String>,
}

fn summarize_workflow_run(value: &Value) -> CiSummary {
    let run_id = value.get("id").and_then(Value::as_u64).unwrap_or(0);
    let name = string_field(value, "name").unwrap_or("unknown");
    let status = string_field(value, "status").unwrap_or("unknown");
    let conclusion = string_field(value, "conclusion").unwrap_or("unknown");
    let head_branch = string_field(value, "head_branch").unwrap_or("unknown");
    CiSummary {
        summary: format!(
            "GitHub Actions run {run_id}; name={name}; status={status}; conclusion={conclusion}; head_branch={head_branch}"
        ),
        references: html_url(value).into_iter().collect(),
    }
}

fn summarize_jobs(value: &Value) -> CiSummary {
    let jobs = items(value, "jobs");
    let count = jobs.len();
    let status_counts = status_counts(&jobs);
    let sample = jobs
        .iter()
        .filter_map(|job| string_field(job, "name"))
        .take(10)
        .collect::<Vec<_>>()
        .join(",");
    CiSummary {
        summary: format!(
            "GitHub Actions jobs={count}; statuses={status_counts:?}; sample={sample}"
        ),
        references: jobs.iter().filter_map(|job| html_url(job)).collect(),
    }
}

fn summarize_checks(value: &Value) -> CiSummary {
    let runs = items(value, "check_runs");
    let count = runs.len();
    let status_counts = status_counts(&runs);
    CiSummary {
        summary: format!("GitHub Actions check_runs={count}; statuses={status_counts:?}"),
        references: runs.iter().filter_map(|run| html_url(run)).collect(),
    }
}

fn summarize_failures(value: &Value) -> CiSummary {
    let jobs = items(value, "jobs");
    let failed = jobs
        .iter()
        .filter(|job| {
            matches!(
                string_field(job, "conclusion"),
                Some("failure" | "timed_out" | "cancelled" | "action_required")
            )
        })
        .collect::<Vec<_>>();
    let names = failed
        .iter()
        .filter_map(|job| string_field(job, "name"))
        .take(10)
        .collect::<Vec<_>>()
        .join(",");
    CiSummary {
        summary: format!(
            "GitHub Actions failure_summary; failed_jobs={}; sample={names}; logs=reference-only",
            failed.len()
        ),
        references: failed.iter().filter_map(|job| html_url(job)).collect(),
    }
}

fn response_to_http(response: ureq::Response) -> Result<GitHubActionsHttpResponse, AdapterError> {
    let status = response.status();
    let headers = headers_from_response(&response);
    let body = response.into_string().map_err(|error| {
        AdapterError::new(
            AdapterErrorKind::MalformedResponse,
            "ci.github_actions.response.read_failed",
            error.to_string(),
        )
    })?;
    Ok(GitHubActionsHttpResponse {
        status,
        body,
        headers,
    })
}

fn headers_from_response(response: &ureq::Response) -> BTreeMap<String, String> {
    response
        .headers_names()
        .into_iter()
        .filter_map(|name| {
            response
                .header(&name)
                .map(|value| (name.to_ascii_lowercase(), value.to_owned()))
        })
        .collect()
}

fn classify_http_error(
    status: u16,
    headers: &BTreeMap<String, String>,
    body: &str,
) -> AdapterError {
    let kind = match status {
        401 => AdapterErrorKind::AuthFailure,
        403 if headers
            .get("x-ratelimit-remaining")
            .is_some_and(|remaining| remaining == "0") =>
        {
            AdapterErrorKind::RateLimited
        }
        403 => AdapterErrorKind::PermissionFailure,
        404 => AdapterErrorKind::NotFound,
        408 | 504 => AdapterErrorKind::Timeout,
        429 => AdapterErrorKind::RateLimited,
        400..=499 => AdapterErrorKind::ValidationFailure,
        500..=599 => AdapterErrorKind::TransientNetworkFailure,
        _ => AdapterErrorKind::Unknown,
    };
    AdapterError::new(
        kind,
        format!("ci.github_actions.http.{status}"),
        non_secret_error_message(body),
    )
}

fn non_secret_error_message(body: &str) -> String {
    serde_json::from_str::<Value>(body)
        .ok()
        .and_then(|value| {
            value
                .get("message")
                .and_then(Value::as_str)
                .map(str::to_owned)
        })
        .unwrap_or_else(|| "GitHub Actions read-only request failed".to_owned())
}

fn metadata_value<'a>(request: &'a AdapterRequest, key: &str) -> Result<&'a str, AdapterError> {
    request
        .metadata
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| {
            AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                format!("ci.metadata.{key}.required"),
                "required CI adapter metadata is missing",
            )
        })
}

fn string_field<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}

fn html_url(value: &Value) -> Option<String> {
    string_field(value, "html_url").map(str::to_owned)
}

fn items<'a>(value: &'a Value, key: &str) -> Vec<&'a Value> {
    value
        .get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .collect()
}

fn status_counts(items: &[&Value]) -> BTreeMap<String, u64> {
    items.iter().fold(BTreeMap::new(), |mut counts, item| {
        let status = string_field(item, "conclusion")
            .or_else(|| string_field(item, "status"))
            .unwrap_or("unknown");
        *counts.entry(status.to_owned()).or_default() += 1;
        counts
    })
}

fn redacted_log_excerpt(log: &str) -> (String, bool) {
    let mut excerpt = String::new();
    let mut truncated = false;
    for line in log.lines() {
        let safe_line = redact_log_line(line);
        let next_len = excerpt.len() + safe_line.len() + usize::from(!excerpt.is_empty());
        if next_len > MAX_LOG_EXCERPT_BYTES {
            truncated = true;
            break;
        }
        if !excerpt.is_empty() {
            excerpt.push_str("\\n");
        }
        excerpt.push_str(&safe_line);
    }
    if log.len() > MAX_LOG_EXCERPT_BYTES {
        truncated = true;
    }
    (excerpt, truncated)
}

fn redact_log_line(line: &str) -> String {
    let lower = line.to_ascii_lowercase();
    if lower.contains("secret")
        || lower.contains("token")
        || lower.contains("password")
        || lower.contains("credential")
        || lower.contains("api_key")
        || lower.contains("authorization")
        || lower.contains("database_url")
        || contains_sensitive_url_userinfo(&lower)
    {
        "[REDACTED_LOG_LINE]".to_owned()
    } else {
        line.to_owned()
    }
}

fn contains_sensitive_url_userinfo(lower: &str) -> bool {
    const SENSITIVE_SCHEMES: &[&str] = &["postgres://", "postgresql://"];

    SENSITIVE_SCHEMES.iter().any(|scheme| {
        let mut offset = 0;
        while let Some(relative_start) = lower[offset..].find(scheme) {
            let authority_start = offset + relative_start + scheme.len();
            let after_scheme = &lower[authority_start..];
            let authority_len = after_scheme
                .find(|c: char| matches!(c, '/' | '?' | '#' | ' ' | '\t'))
                .unwrap_or(after_scheme.len());
            let authority = &after_scheme[..authority_len];
            if authority.contains('@') {
                return true;
            }
            offset = authority_start + authority_len.max(1);
            if offset >= lower.len() {
                break;
            }
        }
        false
    })
}

fn elapsed_ms(started: Instant) -> u64 {
    u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX)
}

fn health_warnings(config: &GitHubActionsReadOnlyConfig) -> Vec<String> {
    if config.credential_present() {
        Vec::new()
    } else {
        vec![format!(
            "GitHub Actions token not configured; set {} or {} for live read-only mode",
            config.token_env_var, FALLBACK_TOKEN_ENV
        )]
    }
}

/// Builds a standard GitHub Actions read-only adapter request for tests and callers.
///
/// # Errors
///
/// Returns an error when identifier construction fails.
pub fn github_actions_read_request(
    action: impl Into<String>,
    actor: ActorId,
    correlation_id: CorrelationId,
    metadata: BTreeMap<String, String>,
    operation_mode: AdapterOperationMode,
    policy_precheck: AdapterPolicyPrecheck,
) -> Result<AdapterRequest, AdapterError> {
    Ok(AdapterRequest {
        adapter_id: AdapterId::new("adapter/github-actions-read-only").map_err(|error| {
            AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                "ci.github_actions.adapter_id.invalid",
                error.to_string(),
            )
        })?,
        adapter_kind: AdapterKind::Ci,
        action: crate::AdapterAction {
            name: action.into(),
            side_effecting: false,
            required_capabilities: vec![AdapterCapability::CiRead],
        },
        capability: AdapterCapability::CiRead,
        operation_mode,
        correlation_id,
        actor,
        run_scope: None,
        idempotency_key: None,
        input_reference: None,
        idempotency_strategy: crate::AdapterIdempotencyStrategy::NotRequiredForReadOnly,
        redaction_policy: AdapterRedactionPolicy {
            strategy: crate::AdapterRedactionStrategy::ReferenceOnly,
            sensitive_fields: vec![
                "log".to_owned(),
                "logs".to_owned(),
                "token".to_owned(),
                "authorization".to_owned(),
            ],
        },
        timeout_policy: AdapterTimeoutPolicy { timeout_ms: 10_000 },
        metadata,
        policy_precheck,
    })
}
