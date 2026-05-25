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
const DEFAULT_TOKEN_ENV: &str = "WORKFLOW_OS_GITHUB_TOKEN";
const FALLBACK_TOKEN_ENV: &str = "GITHUB_TOKEN";

/// GitHub read-only adapter action names.
pub mod github_actions {
    /// Read repository metadata.
    pub const REPOSITORY_METADATA: &str = "github.repository.metadata";
    /// Read repository default branch.
    pub const DEFAULT_BRANCH: &str = "github.repository.default_branch";
    /// Read file contents by path and ref.
    pub const FILE_CONTENTS: &str = "github.file.contents";
    /// Read pull request metadata.
    pub const PULL_REQUEST_METADATA: &str = "github.pull_request.metadata";
    /// Read pull request diff summary.
    pub const PULL_REQUEST_DIFF_SUMMARY: &str = "github.pull_request.diff_summary";
    /// Read pull request changed files.
    pub const PULL_REQUEST_CHANGED_FILES: &str = "github.pull_request.changed_files";
    /// Read pull request comments.
    pub const PULL_REQUEST_COMMENTS: &str = "github.pull_request.comments";
    /// Read check suite and check run summaries.
    pub const CHECK_STATUS_SUMMARY: &str = "github.checks.summary";
}

/// GitHub read-only adapter configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubReadOnlyConfig {
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

impl GitHubReadOnlyConfig {
    /// Loads GitHub read-only configuration from environment variables.
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

    /// Creates a fixture-mode GitHub configuration for tests.
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
        let adapter_id = AdapterId::new("adapter/github-read-only").map_err(|error| {
            AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                "github.adapter_id.invalid",
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

/// Minimal HTTP response returned by the GitHub client abstraction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubHttpResponse {
    /// HTTP status code.
    pub status: u16,
    /// Response body.
    pub body: String,
    /// Non-secret response headers.
    pub headers: BTreeMap<String, String>,
}

/// GitHub HTTP client abstraction used by live and fixture clients.
pub trait GitHubReadOnlyClient {
    /// Performs a read-only GET request.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the request cannot complete.
    fn get(&self, path: &str, accept: &str) -> Result<GitHubHttpResponse, AdapterError>;
}

/// Live GitHub REST client. Live use is opt-in through environment configuration.
#[derive(Clone, Debug)]
pub struct GitHubLiveReadOnlyClient {
    config: GitHubReadOnlyConfig,
}

impl GitHubLiveReadOnlyClient {
    /// Creates a live GitHub read-only client.
    #[must_use]
    pub const fn new(config: GitHubReadOnlyConfig) -> Self {
        Self { config }
    }
}

impl GitHubReadOnlyClient for GitHubLiveReadOnlyClient {
    fn get(&self, path: &str, accept: &str) -> Result<GitHubHttpResponse, AdapterError> {
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
                "github.network.transient",
                error.to_string(),
            )),
        }
    }
}

/// Fixture-backed GitHub client for offline tests.
#[derive(Clone, Debug, Default)]
pub struct GitHubFixtureClient {
    responses: BTreeMap<String, GitHubHttpResponse>,
}

impl GitHubFixtureClient {
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
            GitHubHttpResponse {
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
            GitHubHttpResponse {
                status,
                body: body.into(),
                headers,
            },
        );
        self
    }
}

impl GitHubReadOnlyClient for GitHubFixtureClient {
    fn get(&self, path: &str, _accept: &str) -> Result<GitHubHttpResponse, AdapterError> {
        let Some(response) = self.responses.get(path).cloned() else {
            return Err(AdapterError::new(
                AdapterErrorKind::NotFound,
                "github.fixture.not_found",
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

/// GitHub read-only adapter.
#[derive(Clone, Debug)]
pub struct GitHubReadOnlyAdapter<C> {
    config: GitHubReadOnlyConfig,
    client: C,
}

impl<C> GitHubReadOnlyAdapter<C>
where
    C: GitHubReadOnlyClient,
{
    /// Creates a GitHub read-only adapter from config and client.
    #[must_use]
    pub const fn new(config: GitHubReadOnlyConfig, client: C) -> Self {
        Self { config, client }
    }

    /// Returns adapter health without exposing credential values.
    #[must_use]
    pub fn health(&self) -> AdapterHealth {
        AdapterHealth {
            adapter_id: self.config.adapter_id.clone(),
            adapter_kind: AdapterKind::GitHub,
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

    /// Reads repository metadata.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when policy, fixture, parsing, or HTTP read fails.
    pub fn read_repository_metadata(
        &self,
        request: &AdapterRequest,
        owner: &str,
        repo: &str,
    ) -> Result<GitHubReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/repos/{owner}/{repo}"),
            "application/vnd.github+json",
            summarize_repository,
        )
    }

    /// Reads repository default branch.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_default_branch(
        &self,
        request: &AdapterRequest,
        owner: &str,
        repo: &str,
    ) -> Result<GitHubReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/repos/{owner}/{repo}"),
            "application/vnd.github+json",
            summarize_default_branch,
        )
    }

    /// Reads file contents metadata by path and ref.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_file_contents(
        &self,
        request: &AdapterRequest,
        owner: &str,
        repo: &str,
        path: &str,
        reference: &str,
    ) -> Result<GitHubReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/repos/{owner}/{repo}/contents/{path}?ref={reference}"),
            "application/vnd.github+json",
            summarize_file_contents,
        )
    }

    /// Reads pull request metadata.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_pull_request_metadata(
        &self,
        request: &AdapterRequest,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<GitHubReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/repos/{owner}/{repo}/pulls/{number}"),
            "application/vnd.github+json",
            summarize_pull_request,
        )
    }

    /// Reads pull request changed files.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_pull_request_changed_files(
        &self,
        request: &AdapterRequest,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<GitHubReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/repos/{owner}/{repo}/pulls/{number}/files"),
            "application/vnd.github+json",
            summarize_changed_files,
        )
    }

    /// Reads pull request comments as read-only data.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_pull_request_comments(
        &self,
        request: &AdapterRequest,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<GitHubReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/repos/{owner}/{repo}/issues/{number}/comments"),
            "application/vnd.github+json",
            summarize_comments,
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
    ) -> Result<GitHubReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/repos/{owner}/{repo}/commits/{reference}/check-runs"),
            "application/vnd.github+json",
            summarize_check_runs,
        )
    }

    /// Reads pull request diff and returns a normalized diff summary.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_pull_request_diff_summary(
        &self,
        request: &AdapterRequest,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<GitHubReadOutcome, AdapterError> {
        Self::ensure_read_request(request)?;
        let started = Instant::now();
        let response = self.client.get(
            &format!("/repos/{owner}/{repo}/pulls/{number}.diff"),
            "application/vnd.github.v3.diff",
        )?;
        let duration_ms = elapsed_ms(started);
        let summary = summarize_diff(&response.body);
        Ok(Self::outcome(
            request,
            summary,
            vec![format!("github://{owner}/{repo}/pull/{number}")],
            duration_ms,
        ))
    }

    fn read_json_summary(
        &self,
        request: &AdapterRequest,
        path: &str,
        accept: &str,
        summarize: fn(&Value) -> GitHubSummary,
    ) -> Result<GitHubReadOutcome, AdapterError> {
        Self::ensure_read_request(request)?;
        let started = Instant::now();
        let response = self.client.get(path, accept)?;
        let duration_ms = elapsed_ms(started);
        let value: Value = serde_json::from_str(&response.body).map_err(|error| {
            AdapterError::new(
                AdapterErrorKind::MalformedResponse,
                "github.response.malformed",
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
    ) -> GitHubReadOutcome {
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
        GitHubReadOutcome {
            response,
            invocation,
            observability,
        }
    }

    fn ensure_read_request(request: &AdapterRequest) -> Result<(), AdapterError> {
        if request.adapter_kind != AdapterKind::GitHub {
            return Err(AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                "github.adapter_kind.invalid",
                "GitHub read-only adapter requires adapter kind github",
            ));
        }
        if request.capability != AdapterCapability::GitHubRead {
            return Err(AdapterError::new(
                AdapterErrorKind::PermissionFailure,
                "github.capability.read_required",
                "GitHub read-only adapter requires github.read capability",
            ));
        }
        request.validate_preconditions()
    }
}

impl<C> AdapterReadOperation for GitHubReadOnlyAdapter<C>
where
    C: GitHubReadOnlyClient,
{
    fn read(&self, request: &AdapterRequest) -> Result<AdapterResponse, AdapterError> {
        Self::ensure_read_request(request)?;
        let owner = metadata_value(request, "owner")?;
        let repo = metadata_value(request, "repo")?;
        let outcome = match request.action.name.as_str() {
            github_actions::REPOSITORY_METADATA => {
                self.read_repository_metadata(request, owner, repo)?
            }
            github_actions::DEFAULT_BRANCH => self.read_default_branch(request, owner, repo)?,
            github_actions::FILE_CONTENTS => self.read_file_contents(
                request,
                owner,
                repo,
                metadata_value(request, "path")?,
                metadata_value(request, "ref")?,
            )?,
            github_actions::PULL_REQUEST_METADATA => self.read_pull_request_metadata(
                request,
                owner,
                repo,
                metadata_u64(request, "pull_number")?,
            )?,
            github_actions::PULL_REQUEST_DIFF_SUMMARY => self.read_pull_request_diff_summary(
                request,
                owner,
                repo,
                metadata_u64(request, "pull_number")?,
            )?,
            github_actions::PULL_REQUEST_CHANGED_FILES => self.read_pull_request_changed_files(
                request,
                owner,
                repo,
                metadata_u64(request, "pull_number")?,
            )?,
            github_actions::PULL_REQUEST_COMMENTS => self.read_pull_request_comments(
                request,
                owner,
                repo,
                metadata_u64(request, "pull_number")?,
            )?,
            github_actions::CHECK_STATUS_SUMMARY => self.read_check_status_summary(
                request,
                owner,
                repo,
                metadata_value(request, "ref")?,
            )?,
            _ => {
                return Err(AdapterError::new(
                    AdapterErrorKind::UnsupportedOperation,
                    "github.action.unsupported",
                    "unsupported GitHub read-only adapter action",
                ));
            }
        };
        Ok(outcome.response)
    }
}

/// Result of a GitHub read including adapter response and derived telemetry records.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GitHubReadOutcome {
    /// Normalized adapter response.
    pub response: AdapterResponse,
    /// Audit-safe adapter invocation record.
    pub invocation: AdapterInvocationRecord,
    /// Adapter observability record.
    pub observability: AdapterObservabilityRecord,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GitHubSummary {
    summary: String,
    references: Vec<String>,
}

fn summarize_repository(value: &Value) -> GitHubSummary {
    let full_name = string_field(value, "full_name").unwrap_or("unknown");
    let default_branch = string_field(value, "default_branch").unwrap_or("unknown");
    let private = value
        .get("private")
        .and_then(Value::as_bool)
        .map_or("unknown".to_owned(), |private| private.to_string());
    GitHubSummary {
        summary: format!(
            "GitHub repository {full_name}; default_branch={default_branch}; private={private}"
        ),
        references: html_url(value).into_iter().collect(),
    }
}

fn summarize_default_branch(value: &Value) -> GitHubSummary {
    let full_name = string_field(value, "full_name").unwrap_or("unknown");
    let default_branch = string_field(value, "default_branch").unwrap_or("unknown");
    GitHubSummary {
        summary: format!("GitHub repository {full_name} default branch is {default_branch}"),
        references: html_url(value).into_iter().collect(),
    }
}

fn summarize_file_contents(value: &Value) -> GitHubSummary {
    let path = string_field(value, "path").unwrap_or("unknown");
    let sha = string_field(value, "sha").unwrap_or("unknown");
    let size = value.get("size").and_then(Value::as_u64).unwrap_or(0);
    GitHubSummary {
        summary: format!(
            "GitHub file {path}; sha={sha}; size_bytes={size}; content=reference-only"
        ),
        references: html_url(value)
            .or_else(|| string_field(value, "download_url").map(str::to_owned))
            .into_iter()
            .collect(),
    }
}

fn summarize_pull_request(value: &Value) -> GitHubSummary {
    let number = value.get("number").and_then(Value::as_u64).unwrap_or(0);
    let state = string_field(value, "state").unwrap_or("unknown");
    let title = string_field(value, "title").unwrap_or("untitled");
    GitHubSummary {
        summary: format!("GitHub pull request #{number}; state={state}; title={title}"),
        references: html_url(value).into_iter().collect(),
    }
}

fn summarize_changed_files(value: &Value) -> GitHubSummary {
    let files = value.as_array().map_or(0, Vec::len);
    let names = value
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|item| string_field(item, "filename"))
        .take(10)
        .collect::<Vec<_>>()
        .join(",");
    GitHubSummary {
        summary: format!("GitHub pull request changed_files={files}; sample={names}"),
        references: Vec::new(),
    }
}

fn summarize_comments(value: &Value) -> GitHubSummary {
    let comments = value.as_array().map_or(0, Vec::len);
    GitHubSummary {
        summary: format!("GitHub pull request comments={comments}; bodies=reference-only"),
        references: value
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(html_url)
            .collect(),
    }
}

fn summarize_check_runs(value: &Value) -> GitHubSummary {
    let runs = value
        .get("check_runs")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let conclusion_counts = value
        .get("check_runs")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|run| string_field(run, "conclusion").or_else(|| string_field(run, "status")))
        .fold(BTreeMap::<String, u64>::new(), |mut counts, status| {
            *counts.entry(status.to_owned()).or_default() += 1;
            counts
        });
    GitHubSummary {
        summary: format!("GitHub check runs={runs}; statuses={conclusion_counts:?}"),
        references: Vec::new(),
    }
}

fn summarize_diff(diff: &str) -> String {
    let files = diff
        .lines()
        .filter(|line| line.starts_with("diff --git "))
        .count();
    let additions = diff
        .lines()
        .filter(|line| line.starts_with('+') && !line.starts_with("+++"))
        .count();
    let deletions = diff
        .lines()
        .filter(|line| line.starts_with('-') && !line.starts_with("---"))
        .count();
    format!("GitHub pull request diff summary; files={files}; additions={additions}; deletions={deletions}")
}

fn response_to_http(response: ureq::Response) -> Result<GitHubHttpResponse, AdapterError> {
    let status = response.status();
    let headers = headers_from_response(&response);
    let body = response.into_string().map_err(|error| {
        AdapterError::new(
            AdapterErrorKind::MalformedResponse,
            "github.response.read_failed",
            error.to_string(),
        )
    })?;
    Ok(GitHubHttpResponse {
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
        format!("github.http.{status}"),
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
        .unwrap_or_else(|| "GitHub read-only request failed".to_owned())
}

fn metadata_value<'a>(request: &'a AdapterRequest, key: &str) -> Result<&'a str, AdapterError> {
    request
        .metadata
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| {
            AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                format!("github.metadata.{key}.required"),
                "required GitHub adapter metadata is missing",
            )
        })
}

fn metadata_u64(request: &AdapterRequest, key: &str) -> Result<u64, AdapterError> {
    metadata_value(request, key)?
        .parse::<u64>()
        .map_err(|error| {
            AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                format!("github.metadata.{key}.invalid"),
                error.to_string(),
            )
        })
}

fn string_field<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}

fn html_url(value: &Value) -> Option<String> {
    string_field(value, "html_url").map(str::to_owned)
}

fn elapsed_ms(started: Instant) -> u64 {
    u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX)
}

fn health_warnings(config: &GitHubReadOnlyConfig) -> Vec<String> {
    if config.credential_present() {
        Vec::new()
    } else {
        vec![format!(
            "GitHub token not configured; set {} or {} for live read-only mode",
            config.token_env_var, FALLBACK_TOKEN_ENV
        )]
    }
}

/// Builds a standard GitHub read-only adapter request for tests and callers.
///
/// # Errors
///
/// Returns an error when identifier construction fails.
pub fn github_read_request(
    action: impl Into<String>,
    actor: ActorId,
    correlation_id: CorrelationId,
    metadata: BTreeMap<String, String>,
    operation_mode: AdapterOperationMode,
    policy_precheck: AdapterPolicyPrecheck,
) -> Result<AdapterRequest, AdapterError> {
    Ok(AdapterRequest {
        adapter_id: AdapterId::new("adapter/github-read-only").map_err(|error| {
            AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                "github.adapter_id.invalid",
                error.to_string(),
            )
        })?,
        adapter_kind: AdapterKind::GitHub,
        action: crate::AdapterAction {
            name: action.into(),
            side_effecting: false,
            required_capabilities: vec![AdapterCapability::GitHubRead],
        },
        capability: AdapterCapability::GitHubRead,
        operation_mode,
        correlation_id,
        actor,
        run_scope: None,
        idempotency_key: None,
        input_reference: None,
        idempotency_strategy: crate::AdapterIdempotencyStrategy::NotRequiredForReadOnly,
        redaction_policy: AdapterRedactionPolicy {
            strategy: crate::AdapterRedactionStrategy::ReferenceOnly,
            sensitive_fields: vec!["content".to_owned(), "body".to_owned()],
        },
        timeout_policy: AdapterTimeoutPolicy { timeout_ms: 10_000 },
        metadata,
        policy_precheck,
    })
}
