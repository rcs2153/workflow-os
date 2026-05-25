use std::collections::BTreeMap;
use std::env;
use std::time::Instant;

use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    ActorId, AdapterCapability, AdapterError, AdapterErrorKind, AdapterHealth, AdapterId,
    AdapterInvocationRecord, AdapterKind, AdapterObservabilityRecord, AdapterOperationMode,
    AdapterPolicyPrecheck, AdapterReadOperation, AdapterRedactionPolicy, AdapterRequest,
    AdapterResponse, AdapterTimeoutPolicy, CorrelationId, RedactedValue, Timestamp,
};

const DEFAULT_BASE_URL_ENV: &str = "WORKFLOW_OS_JIRA_BASE_URL";
const DEFAULT_EMAIL_ENV: &str = "WORKFLOW_OS_JIRA_EMAIL";
const FALLBACK_EMAIL_ENV: &str = "JIRA_EMAIL";
const DEFAULT_API_TOKEN_ENV: &str = "WORKFLOW_OS_JIRA_API_TOKEN";
const FALLBACK_API_TOKEN_ENV: &str = "JIRA_API_TOKEN";
const DEFAULT_BEARER_TOKEN_ENV: &str = "WORKFLOW_OS_JIRA_BEARER_TOKEN";
const LEGACY_BEARER_TOKEN_ENV: &str = "WORKFLOW_OS_JIRA_TOKEN";

/// Jira read-only adapter action names.
pub mod jira_actions {
    /// Read issue metadata.
    pub const ISSUE_METADATA: &str = "jira.issue.metadata";
    /// Read issue summary.
    pub const ISSUE_SUMMARY: &str = "jira.issue.summary";
    /// Read issue description.
    pub const ISSUE_DESCRIPTION: &str = "jira.issue.description";
    /// Read issue comments.
    pub const ISSUE_COMMENTS: &str = "jira.issue.comments";
    /// Read issue status.
    pub const ISSUE_STATUS: &str = "jira.issue.status";
    /// Read issue priority.
    pub const ISSUE_PRIORITY: &str = "jira.issue.priority";
    /// Read issue labels.
    pub const ISSUE_LABELS: &str = "jira.issue.labels";
    /// Read assignee and reporter display metadata.
    pub const ISSUE_PEOPLE: &str = "jira.issue.people";
    /// Read Jira project metadata.
    pub const PROJECT_METADATA: &str = "jira.project.metadata";
}

/// Jira read-only adapter configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JiraReadOnlyConfig {
    /// Stable adapter ID.
    pub adapter_id: AdapterId,
    /// Jira REST API base URL, for example `https://example.atlassian.net`.
    pub base_url: String,
    /// Environment variable checked for the base URL.
    pub base_url_env_var: String,
    /// Environment variable checked for the Jira account email.
    pub email_env_var: String,
    /// Environment variable checked for the Jira API token.
    pub api_token_env_var: String,
    /// Environment variable checked for a bearer token.
    pub bearer_token_env_var: String,
    /// Optional redacted Jira account email for Atlassian Cloud Basic auth.
    pub email: Option<RedactedValue<String>>,
    /// Optional redacted Jira API token for Atlassian Cloud Basic auth.
    pub api_token: Option<RedactedValue<String>>,
    /// Optional redacted bearer token for non-Cloud deployments that support bearer auth.
    pub bearer_token: Option<RedactedValue<String>>,
    /// Adapter operation mode.
    pub operation_mode: AdapterOperationMode,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct JiraEnvVarNames {
    base_url: String,
    email: String,
    api_token: String,
    bearer_token: String,
}

impl JiraEnvVarNames {
    fn defaults() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL_ENV.to_owned(),
            email: DEFAULT_EMAIL_ENV.to_owned(),
            api_token: DEFAULT_API_TOKEN_ENV.to_owned(),
            bearer_token: DEFAULT_BEARER_TOKEN_ENV.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct JiraAuthConfig {
    email: Option<RedactedValue<String>>,
    api_token: Option<RedactedValue<String>>,
    bearer_token: Option<RedactedValue<String>>,
}

impl JiraAuthConfig {
    fn empty() -> Self {
        Self {
            email: None,
            api_token: None,
            bearer_token: None,
        }
    }
}

impl JiraReadOnlyConfig {
    /// Loads Jira read-only configuration from environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error if the static adapter ID is invalid.
    pub fn from_env() -> Result<Self, AdapterError> {
        let base_url = env::var(DEFAULT_BASE_URL_ENV).unwrap_or_default();
        let email = env::var(DEFAULT_EMAIL_ENV)
            .or_else(|_| env::var(FALLBACK_EMAIL_ENV))
            .ok()
            .map(RedactedValue::new);
        let api_token = env::var(DEFAULT_API_TOKEN_ENV)
            .or_else(|_| env::var(FALLBACK_API_TOKEN_ENV))
            .ok()
            .map(RedactedValue::new);
        let bearer_token = env::var(DEFAULT_BEARER_TOKEN_ENV)
            .or_else(|_| env::var(LEGACY_BEARER_TOKEN_ENV))
            .ok()
            .map(RedactedValue::new);

        Self::new(
            AdapterOperationMode::LiveReadOnly,
            base_url,
            JiraEnvVarNames::defaults(),
            JiraAuthConfig {
                email,
                api_token,
                bearer_token,
            },
        )
    }

    /// Creates a fixture-mode Jira configuration for tests.
    ///
    /// # Errors
    ///
    /// Returns an error if the static adapter ID is invalid.
    pub fn fixture() -> Result<Self, AdapterError> {
        Self::new(
            AdapterOperationMode::Fixture,
            "fixture://jira".to_owned(),
            JiraEnvVarNames::defaults(),
            JiraAuthConfig::empty(),
        )
    }

    /// Creates a live read-only configuration using Atlassian Cloud Basic auth.
    ///
    /// # Errors
    ///
    /// Returns an error if the static adapter ID is invalid.
    pub fn live_read_only_with_basic_auth(
        base_url: String,
        email: String,
        api_token: String,
    ) -> Result<Self, AdapterError> {
        Self::new(
            AdapterOperationMode::LiveReadOnly,
            base_url,
            JiraEnvVarNames::defaults(),
            JiraAuthConfig {
                email: Some(RedactedValue::new(email)),
                api_token: Some(RedactedValue::new(api_token)),
                bearer_token: None,
            },
        )
    }

    /// Creates a live read-only configuration using bearer auth.
    ///
    /// Bearer auth is supported only for Jira deployments that explicitly accept
    /// bearer tokens. Atlassian Cloud API-token usage should prefer
    /// [`Self::live_read_only_with_basic_auth`].
    ///
    /// # Errors
    ///
    /// Returns an error if the static adapter ID is invalid.
    pub fn live_read_only_with_bearer_token(
        base_url: String,
        bearer_token: String,
    ) -> Result<Self, AdapterError> {
        Self::new(
            AdapterOperationMode::LiveReadOnly,
            base_url,
            JiraEnvVarNames::defaults(),
            JiraAuthConfig {
                email: None,
                api_token: None,
                bearer_token: Some(RedactedValue::new(bearer_token)),
            },
        )
    }

    /// Creates a live read-only configuration with a legacy bearer token.
    ///
    /// The token is wrapped in `RedactedValue` and must not be logged.
    ///
    /// # Errors
    ///
    /// Returns an error if the static adapter ID is invalid.
    pub fn live_read_only_with_token(
        base_url: String,
        token: String,
    ) -> Result<Self, AdapterError> {
        Self::live_read_only_with_bearer_token(base_url, token)
    }

    fn new(
        operation_mode: AdapterOperationMode,
        base_url: String,
        env_vars: JiraEnvVarNames,
        auth: JiraAuthConfig,
    ) -> Result<Self, AdapterError> {
        let adapter_id = AdapterId::new("adapter/jira-read-only").map_err(|error| {
            AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                "jira.adapter_id.invalid",
                error.to_string(),
            )
        })?;

        Ok(Self {
            adapter_id,
            base_url,
            base_url_env_var: env_vars.base_url,
            email_env_var: env_vars.email,
            api_token_env_var: env_vars.api_token,
            bearer_token_env_var: env_vars.bearer_token,
            email: auth.email,
            api_token: auth.api_token,
            bearer_token: auth.bearer_token,
            operation_mode,
        })
    }

    /// Returns true when a complete supported auth mode is configured.
    #[must_use]
    pub fn credential_present(&self) -> bool {
        self.basic_auth_configured() || self.bearer_auth_configured()
    }

    /// Returns non-secret Jira auth configuration metadata.
    #[must_use]
    pub fn redacted_auth_metadata(&self) -> BTreeMap<String, String> {
        BTreeMap::from([
            (
                "auth_mode".to_owned(),
                self.auth_mode_name().unwrap_or("unconfigured").to_owned(),
            ),
            (
                "base_url_configured".to_owned(),
                self.base_url_present().to_string(),
            ),
            (
                "basic_email_configured".to_owned(),
                self.email.is_some().to_string(),
            ),
            (
                "basic_api_token_configured".to_owned(),
                self.api_token.is_some().to_string(),
            ),
            (
                "bearer_token_configured".to_owned(),
                self.bearer_token.is_some().to_string(),
            ),
        ])
    }

    fn base_url_present(&self) -> bool {
        !self.base_url.trim().is_empty()
    }

    fn basic_auth_configured(&self) -> bool {
        self.email.is_some() && self.api_token.is_some()
    }

    fn partial_basic_auth_configured(&self) -> bool {
        self.email.is_some() ^ self.api_token.is_some()
    }

    fn bearer_auth_configured(&self) -> bool {
        self.bearer_token.is_some()
    }

    fn auth_mode_name(&self) -> Option<&'static str> {
        if self.basic_auth_configured() {
            Some("basic")
        } else if self.bearer_auth_configured() {
            Some("bearer")
        } else {
            None
        }
    }

    fn authorization_header(&self) -> Result<String, AdapterError> {
        match (&self.email, &self.api_token, &self.bearer_token) {
            (Some(email), Some(api_token), _) => {
                let credential = format!("{}:{}", email.expose_secret(), api_token.expose_secret());
                Ok(format!(
                    "Basic {}",
                    BASE64_STANDARD.encode(credential.as_bytes())
                ))
            }
            (_, _, Some(token)) => Ok(format!("Bearer {}", token.expose_secret())),
            _ => Err(AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                "jira.config.auth_required",
                "Jira live read-only mode requires complete Basic auth (WORKFLOW_OS_JIRA_EMAIL plus WORKFLOW_OS_JIRA_API_TOKEN) or WORKFLOW_OS_JIRA_BEARER_TOKEN",
            )),
        }
    }
}

/// Minimal HTTP response returned by the Jira client abstraction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JiraHttpResponse {
    /// HTTP status code.
    pub status: u16,
    /// Response body.
    pub body: String,
    /// Non-secret response headers.
    pub headers: BTreeMap<String, String>,
}

/// Jira HTTP client abstraction used by live and fixture clients.
pub trait JiraReadOnlyClient {
    /// Performs a read-only GET request.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the request cannot complete.
    fn get(&self, path: &str, accept: &str) -> Result<JiraHttpResponse, AdapterError>;
}

/// Live Jira REST client. Live use is opt-in through environment configuration.
#[derive(Clone, Debug)]
pub struct JiraLiveReadOnlyClient {
    config: JiraReadOnlyConfig,
}

impl JiraLiveReadOnlyClient {
    /// Creates a live Jira read-only client.
    #[must_use]
    pub const fn new(config: JiraReadOnlyConfig) -> Self {
        Self { config }
    }
}

impl JiraReadOnlyClient for JiraLiveReadOnlyClient {
    fn get(&self, path: &str, accept: &str) -> Result<JiraHttpResponse, AdapterError> {
        if !self.config.base_url_present() {
            return Err(AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                "jira.config.base_url_required",
                "Jira live read-only mode requires WORKFLOW_OS_JIRA_BASE_URL",
            ));
        }

        let url = format!("{}{}", self.config.base_url.trim_end_matches('/'), path);
        let mut request = ureq::get(&url)
            .set("accept", accept)
            .set("user-agent", "workflow-os/0.1.0-preview.1");

        let authorization = self.config.authorization_header()?;
        request = request.set("authorization", &authorization);

        match request.call() {
            Ok(response) => response_to_http(response),
            Err(ureq::Error::Status(status, response)) => {
                let headers = headers_from_response(&response);
                let body = response.into_string().unwrap_or_default();
                Err(classify_http_error(status, &headers, &body))
            }
            Err(ureq::Error::Transport(error)) => Err(AdapterError::new(
                AdapterErrorKind::TransientNetworkFailure,
                "jira.network.transient",
                error.to_string(),
            )),
        }
    }
}

/// Fixture-backed Jira client for offline tests.
#[derive(Clone, Debug, Default)]
pub struct JiraFixtureClient {
    responses: BTreeMap<String, JiraHttpResponse>,
}

impl JiraFixtureClient {
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
            JiraHttpResponse {
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
            JiraHttpResponse {
                status,
                body: body.into(),
                headers,
            },
        );
        self
    }
}

impl JiraReadOnlyClient for JiraFixtureClient {
    fn get(&self, path: &str, _accept: &str) -> Result<JiraHttpResponse, AdapterError> {
        let Some(response) = self.responses.get(path).cloned() else {
            return Err(AdapterError::new(
                AdapterErrorKind::NotFound,
                "jira.fixture.not_found",
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

/// Jira read-only adapter.
#[derive(Clone, Debug)]
pub struct JiraReadOnlyAdapter<C> {
    config: JiraReadOnlyConfig,
    client: C,
}

impl<C> JiraReadOnlyAdapter<C>
where
    C: JiraReadOnlyClient,
{
    /// Creates a Jira read-only adapter from config and client.
    #[must_use]
    pub const fn new(config: JiraReadOnlyConfig, client: C) -> Self {
        Self { config, client }
    }

    /// Returns adapter health without exposing credential values.
    #[must_use]
    pub fn health(&self) -> AdapterHealth {
        AdapterHealth {
            adapter_id: self.config.adapter_id.clone(),
            adapter_kind: AdapterKind::Jira,
            operation_mode: self.config.operation_mode,
            configured: matches!(
                self.config.operation_mode,
                AdapterOperationMode::Fixture
                    | AdapterOperationMode::Mock
                    | AdapterOperationMode::Local
            ) || self.config.credential_present() && self.config.base_url_present(),
            reachable: None,
            credential_present: self.config.credential_present(),
            last_checked_at: Timestamp::now_utc(),
            warnings: health_warnings(&self.config),
        }
    }

    /// Reads Jira issue metadata.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when policy, fixture, parsing, or HTTP read fails.
    pub fn read_issue_metadata(
        &self,
        request: &AdapterRequest,
        issue_key: &str,
    ) -> Result<JiraReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/rest/api/3/issue/{issue_key}"),
            summarize_issue_metadata,
        )
    }

    /// Reads Jira issue summary.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_issue_summary(
        &self,
        request: &AdapterRequest,
        issue_key: &str,
    ) -> Result<JiraReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/rest/api/3/issue/{issue_key}"),
            summarize_issue_summary,
        )
    }

    /// Reads Jira issue description as reference-only summary metadata.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_issue_description(
        &self,
        request: &AdapterRequest,
        issue_key: &str,
    ) -> Result<JiraReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/rest/api/3/issue/{issue_key}"),
            summarize_issue_description,
        )
    }

    /// Reads Jira issue comments as reference-only summary metadata.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_issue_comments(
        &self,
        request: &AdapterRequest,
        issue_key: &str,
    ) -> Result<JiraReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/rest/api/3/issue/{issue_key}/comment"),
            summarize_issue_comments,
        )
    }

    /// Reads Jira issue status.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_issue_status(
        &self,
        request: &AdapterRequest,
        issue_key: &str,
    ) -> Result<JiraReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/rest/api/3/issue/{issue_key}"),
            summarize_issue_status,
        )
    }

    /// Reads Jira issue priority.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_issue_priority(
        &self,
        request: &AdapterRequest,
        issue_key: &str,
    ) -> Result<JiraReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/rest/api/3/issue/{issue_key}"),
            summarize_issue_priority,
        )
    }

    /// Reads Jira issue labels.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_issue_labels(
        &self,
        request: &AdapterRequest,
        issue_key: &str,
    ) -> Result<JiraReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/rest/api/3/issue/{issue_key}"),
            summarize_issue_labels,
        )
    }

    /// Reads Jira issue assignee and reporter display metadata.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_issue_people(
        &self,
        request: &AdapterRequest,
        issue_key: &str,
    ) -> Result<JiraReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/rest/api/3/issue/{issue_key}"),
            summarize_issue_people,
        )
    }

    /// Reads Jira project metadata.
    ///
    /// # Errors
    ///
    /// Returns a classified adapter error when the read fails.
    pub fn read_project_metadata(
        &self,
        request: &AdapterRequest,
        project_key: &str,
    ) -> Result<JiraReadOutcome, AdapterError> {
        self.read_json_summary(
            request,
            &format!("/rest/api/3/project/{project_key}"),
            summarize_project_metadata,
        )
    }

    fn read_json_summary(
        &self,
        request: &AdapterRequest,
        path: &str,
        summarize: fn(&Value) -> JiraSummary,
    ) -> Result<JiraReadOutcome, AdapterError> {
        Self::ensure_read_request(request)?;
        let started = Instant::now();
        let response = self.client.get(path, "application/json")?;
        let duration_ms = elapsed_ms(started);
        let value: Value = serde_json::from_str(&response.body).map_err(|error| {
            AdapterError::new(
                AdapterErrorKind::MalformedResponse,
                "jira.response.malformed",
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
    ) -> JiraReadOutcome {
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
        JiraReadOutcome {
            response,
            invocation,
            observability,
        }
    }

    fn ensure_read_request(request: &AdapterRequest) -> Result<(), AdapterError> {
        if request.adapter_kind != AdapterKind::Jira {
            return Err(AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                "jira.adapter_kind.invalid",
                "Jira read-only adapter requires adapter kind jira",
            ));
        }
        if request.capability != AdapterCapability::JiraRead {
            return Err(AdapterError::new(
                AdapterErrorKind::PermissionFailure,
                "jira.capability.read_required",
                "Jira read-only adapter requires jira.read capability",
            ));
        }
        request.validate_preconditions()
    }
}

impl<C> AdapterReadOperation for JiraReadOnlyAdapter<C>
where
    C: JiraReadOnlyClient,
{
    fn read(&self, request: &AdapterRequest) -> Result<AdapterResponse, AdapterError> {
        Self::ensure_read_request(request)?;
        let outcome = match request.action.name.as_str() {
            jira_actions::ISSUE_METADATA => {
                self.read_issue_metadata(request, metadata_value(request, "issue_key")?)?
            }
            jira_actions::ISSUE_SUMMARY => {
                self.read_issue_summary(request, metadata_value(request, "issue_key")?)?
            }
            jira_actions::ISSUE_DESCRIPTION => {
                self.read_issue_description(request, metadata_value(request, "issue_key")?)?
            }
            jira_actions::ISSUE_COMMENTS => {
                self.read_issue_comments(request, metadata_value(request, "issue_key")?)?
            }
            jira_actions::ISSUE_STATUS => {
                self.read_issue_status(request, metadata_value(request, "issue_key")?)?
            }
            jira_actions::ISSUE_PRIORITY => {
                self.read_issue_priority(request, metadata_value(request, "issue_key")?)?
            }
            jira_actions::ISSUE_LABELS => {
                self.read_issue_labels(request, metadata_value(request, "issue_key")?)?
            }
            jira_actions::ISSUE_PEOPLE => {
                self.read_issue_people(request, metadata_value(request, "issue_key")?)?
            }
            jira_actions::PROJECT_METADATA => {
                self.read_project_metadata(request, metadata_value(request, "project_key")?)?
            }
            _ => {
                return Err(AdapterError::new(
                    AdapterErrorKind::UnsupportedOperation,
                    "jira.action.unsupported",
                    "unsupported Jira read-only adapter action",
                ));
            }
        };
        Ok(outcome.response)
    }
}

/// Result of a Jira read including adapter response and derived telemetry records.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct JiraReadOutcome {
    /// Normalized adapter response.
    pub response: AdapterResponse,
    /// Audit-safe adapter invocation record.
    pub invocation: AdapterInvocationRecord,
    /// Adapter observability record.
    pub observability: AdapterObservabilityRecord,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct JiraSummary {
    summary: String,
    references: Vec<String>,
}

fn summarize_issue_metadata(value: &Value) -> JiraSummary {
    let key = issue_key(value);
    let fields = fields(value);
    let summary = nested_string(fields, &["summary"]).unwrap_or("unknown");
    let status = nested_string(fields, &["status", "name"]).unwrap_or("unknown");
    let priority = nested_string(fields, &["priority", "name"]).unwrap_or("unknown");
    let labels = labels(fields);
    let assignee = nested_string(fields, &["assignee", "displayName"]).unwrap_or("unassigned");
    let reporter = nested_string(fields, &["reporter", "displayName"]).unwrap_or("unknown");
    JiraSummary {
        summary: format!(
            "Jira issue {key}; summary={summary}; status={status}; priority={priority}; labels={labels}; assignee={assignee}; reporter={reporter}; description=reference-only"
        ),
        references: issue_references(value),
    }
}

fn summarize_issue_summary(value: &Value) -> JiraSummary {
    let key = issue_key(value);
    let summary = nested_string(fields(value), &["summary"]).unwrap_or("unknown");
    JiraSummary {
        summary: format!("Jira issue {key}; summary={summary}"),
        references: issue_references(value),
    }
}

fn summarize_issue_description(value: &Value) -> JiraSummary {
    let key = issue_key(value);
    let has_description = fields(value)
        .get("description")
        .is_some_and(|field| !field.is_null());
    JiraSummary {
        summary: format!(
            "Jira issue {key}; description_present={has_description}; description=reference-only"
        ),
        references: issue_references(value),
    }
}

fn summarize_issue_comments(value: &Value) -> JiraSummary {
    let comments = comment_items(value);
    JiraSummary {
        summary: format!(
            "Jira issue comments={}; bodies=reference-only",
            comments.len()
        ),
        references: comments
            .iter()
            .filter_map(|comment| self_url(comment))
            .collect(),
    }
}

fn summarize_issue_status(value: &Value) -> JiraSummary {
    let key = issue_key(value);
    let status = nested_string(fields(value), &["status", "name"]).unwrap_or("unknown");
    JiraSummary {
        summary: format!("Jira issue {key}; status={status}"),
        references: issue_references(value),
    }
}

fn summarize_issue_priority(value: &Value) -> JiraSummary {
    let key = issue_key(value);
    let priority = nested_string(fields(value), &["priority", "name"]).unwrap_or("unknown");
    JiraSummary {
        summary: format!("Jira issue {key}; priority={priority}"),
        references: issue_references(value),
    }
}

fn summarize_issue_labels(value: &Value) -> JiraSummary {
    let key = issue_key(value);
    let labels = labels(fields(value));
    JiraSummary {
        summary: format!("Jira issue {key}; labels={labels}"),
        references: issue_references(value),
    }
}

fn summarize_issue_people(value: &Value) -> JiraSummary {
    let key = issue_key(value);
    let fields = fields(value);
    let assignee = nested_string(fields, &["assignee", "displayName"]).unwrap_or("unassigned");
    let reporter = nested_string(fields, &["reporter", "displayName"]).unwrap_or("unknown");
    JiraSummary {
        summary: format!("Jira issue {key}; assignee={assignee}; reporter={reporter}"),
        references: issue_references(value),
    }
}

fn summarize_project_metadata(value: &Value) -> JiraSummary {
    let key = string_field(value, "key").unwrap_or("unknown");
    let name = string_field(value, "name").unwrap_or("unknown");
    let project_type = string_field(value, "projectTypeKey").unwrap_or("unknown");
    JiraSummary {
        summary: format!("Jira project {key}; name={name}; project_type={project_type}"),
        references: self_url(value).into_iter().collect(),
    }
}

fn response_to_http(response: ureq::Response) -> Result<JiraHttpResponse, AdapterError> {
    let status = response.status();
    let headers = headers_from_response(&response);
    let body = response.into_string().map_err(|error| {
        AdapterError::new(
            AdapterErrorKind::MalformedResponse,
            "jira.response.read_failed",
            error.to_string(),
        )
    })?;
    Ok(JiraHttpResponse {
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
        403 => AdapterErrorKind::PermissionFailure,
        404 => AdapterErrorKind::NotFound,
        408 | 504 => AdapterErrorKind::Timeout,
        429 => AdapterErrorKind::RateLimited,
        400..=499 => AdapterErrorKind::ValidationFailure,
        500..=599 => AdapterErrorKind::TransientNetworkFailure,
        _ if headers.contains_key("retry-after") => AdapterErrorKind::RateLimited,
        _ => AdapterErrorKind::Unknown,
    };
    AdapterError::new(
        kind,
        format!("jira.http.{status}"),
        non_secret_error_message(body),
    )
}

fn non_secret_error_message(body: &str) -> String {
    serde_json::from_str::<Value>(body)
        .ok()
        .and_then(|value| {
            value
                .get("errorMessages")
                .and_then(Value::as_array)
                .and_then(|messages| messages.first())
                .and_then(Value::as_str)
                .or_else(|| value.get("message").and_then(Value::as_str))
                .map(str::to_owned)
        })
        .unwrap_or_else(|| "Jira read-only request failed".to_owned())
}

fn metadata_value<'a>(request: &'a AdapterRequest, key: &str) -> Result<&'a str, AdapterError> {
    request
        .metadata
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| {
            AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                format!("jira.metadata.{key}.required"),
                "required Jira adapter metadata is missing",
            )
        })
}

fn issue_key(value: &Value) -> &str {
    string_field(value, "key").unwrap_or("unknown")
}

fn fields(value: &Value) -> &Value {
    value.get("fields").unwrap_or(&Value::Null)
}

fn labels(value: &Value) -> String {
    value
        .get("labels")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .take(10)
        .collect::<Vec<_>>()
        .join(",")
}

fn comment_items(value: &Value) -> Vec<&Value> {
    value
        .get("comments")
        .or_else(|| value.get("values"))
        .and_then(Value::as_array)
        .or_else(|| value.as_array())
        .map(|items| items.iter().collect())
        .unwrap_or_default()
}

fn issue_references(value: &Value) -> Vec<String> {
    self_url(value).into_iter().collect()
}

fn string_field<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}

fn nested_string<'a>(value: &'a Value, path: &[&str]) -> Option<&'a str> {
    let mut current = value;
    for key in path {
        current = current.get(*key)?;
    }
    current.as_str()
}

fn self_url(value: &Value) -> Option<String> {
    string_field(value, "self").map(str::to_owned)
}

fn elapsed_ms(started: Instant) -> u64 {
    u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX)
}

fn health_warnings(config: &JiraReadOnlyConfig) -> Vec<String> {
    let mut warnings = Vec::new();
    if !config.base_url_present() {
        warnings.push(format!(
            "Jira base URL not configured; set {} for live read-only mode",
            config.base_url_env_var
        ));
    }
    if config.partial_basic_auth_configured() {
        warnings.push(format!(
            "Jira Basic auth is partially configured; set both {} and {}",
            config.email_env_var, config.api_token_env_var
        ));
    }
    if !config.credential_present() {
        warnings.push(format!(
            "Jira auth not configured; set {} plus {}, or {} for bearer auth",
            config.email_env_var, config.api_token_env_var, config.bearer_token_env_var
        ));
    }
    warnings
}

/// Builds a standard Jira read-only adapter request for tests and callers.
///
/// # Errors
///
/// Returns an error when identifier construction fails.
pub fn jira_read_request(
    action: impl Into<String>,
    actor: ActorId,
    correlation_id: CorrelationId,
    metadata: BTreeMap<String, String>,
    operation_mode: AdapterOperationMode,
    policy_precheck: AdapterPolicyPrecheck,
) -> Result<AdapterRequest, AdapterError> {
    Ok(AdapterRequest {
        adapter_id: AdapterId::new("adapter/jira-read-only").map_err(|error| {
            AdapterError::new(
                AdapterErrorKind::ValidationFailure,
                "jira.adapter_id.invalid",
                error.to_string(),
            )
        })?,
        adapter_kind: AdapterKind::Jira,
        action: crate::AdapterAction {
            name: action.into(),
            side_effecting: false,
            required_capabilities: vec![AdapterCapability::JiraRead],
        },
        capability: AdapterCapability::JiraRead,
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
                "description".to_owned(),
                "comment.body".to_owned(),
                "body".to_owned(),
            ],
        },
        timeout_policy: AdapterTimeoutPolicy { timeout_ms: 10_000 },
        metadata,
        policy_precheck,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use super::*;

    #[test]
    fn basic_auth_header_is_constructed_without_debug_exposing_secret_parts() {
        let config = JiraReadOnlyConfig::live_read_only_with_basic_auth(
            "https://example.atlassian.net".to_owned(),
            "person@example.com".to_owned(),
            "api_token_value".to_owned(),
        )
        .expect("config");

        let header = config.authorization_header().expect("auth header");
        let debug = format!("{config:?}");
        let metadata = config.redacted_auth_metadata();

        assert!(header.starts_with("Basic "));
        assert!(!header.contains("person@example.com"));
        assert!(!header.contains("api_token_value"));
        assert_eq!(metadata.get("auth_mode").map(String::as_str), Some("basic"));
        assert!(!debug.contains("person@example.com"));
        assert!(!debug.contains("api_token_value"));
    }

    #[test]
    fn bearer_auth_header_is_supported_when_explicitly_configured() {
        let config = JiraReadOnlyConfig::live_read_only_with_bearer_token(
            "https://jira.example.com".to_owned(),
            "bearer_secret_value".to_owned(),
        )
        .expect("config");

        let header = config.authorization_header().expect("auth header");
        let debug = format!("{config:?}");

        assert!(header.starts_with("Bearer "));
        assert!(!debug.contains("bearer_secret_value"));
        assert_eq!(
            config
                .redacted_auth_metadata()
                .get("auth_mode")
                .map(String::as_str),
            Some("bearer")
        );
    }

    #[test]
    fn complete_basic_auth_takes_precedence_over_bearer_auth() {
        let config = JiraReadOnlyConfig::new(
            AdapterOperationMode::LiveReadOnly,
            "https://example.atlassian.net".to_owned(),
            JiraEnvVarNames::defaults(),
            JiraAuthConfig {
                email: Some(RedactedValue::new("person@example.com".to_owned())),
                api_token: Some(RedactedValue::new("api_token_value".to_owned())),
                bearer_token: Some(RedactedValue::new("bearer_secret_value".to_owned())),
            },
        )
        .expect("config");

        let header = config.authorization_header().expect("auth header");

        assert!(header.starts_with("Basic "));
        assert_eq!(
            config
                .redacted_auth_metadata()
                .get("auth_mode")
                .map(String::as_str),
            Some("basic")
        );
    }

    #[test]
    fn partial_basic_auth_fails_clearly_without_exposing_values() {
        let config = JiraReadOnlyConfig::new(
            AdapterOperationMode::LiveReadOnly,
            "https://example.atlassian.net".to_owned(),
            JiraEnvVarNames::defaults(),
            JiraAuthConfig {
                email: Some(RedactedValue::new("person@example.com".to_owned())),
                api_token: None,
                bearer_token: None,
            },
        )
        .expect("config");

        let error = config
            .authorization_header()
            .expect_err("partial auth fails");
        let warnings = health_warnings(&config).join("\n");
        let debug = format!("{config:?}{error:?}{warnings}");

        assert_eq!(error.kind, AdapterErrorKind::ValidationFailure);
        assert_eq!(error.code, "jira.config.auth_required");
        assert!(warnings.contains("Jira Basic auth is partially configured"));
        assert!(!debug.contains("person@example.com"));
    }

    #[test]
    fn missing_live_auth_health_reports_unconfigured() {
        let config = JiraReadOnlyConfig::new(
            AdapterOperationMode::LiveReadOnly,
            "https://example.atlassian.net".to_owned(),
            JiraEnvVarNames::defaults(),
            JiraAuthConfig::empty(),
        )
        .expect("config");
        let adapter = JiraReadOnlyAdapter::new(config, JiraFixtureClient::new());

        let health = adapter.health();

        assert!(!health.configured);
        assert!(!health.credential_present);
        assert!(health
            .warnings
            .iter()
            .any(|warning| warning.contains("Jira auth not configured")));
    }
}
