#![allow(clippy::expect_used)]
//! GitHub Actions read-only adapter tests use fixture clients by default.

use std::collections::BTreeMap;

use workflow_core::{
    ci_actions, github_actions_read_request, ActorId, AdapterCapability, AdapterErrorKind,
    AdapterOperationMode, AdapterPolicyPrecheck, AdapterPolicyPrecheckProvenance,
    AdapterReadOperation, AdapterResponseStatus, CorrelationId, GitHubActionsFixtureClient,
    GitHubActionsReadOnlyAdapter, GitHubActionsReadOnlyConfig,
};

fn actor() -> ActorId {
    ActorId::new("system/ci-adapter-test").expect("actor")
}

fn correlation() -> CorrelationId {
    CorrelationId::new("correlation/ci-adapter-test").expect("correlation")
}

fn metadata(owner: &str, repo: &str) -> BTreeMap<String, String> {
    BTreeMap::from([
        ("owner".to_owned(), owner.to_owned()),
        ("repo".to_owned(), repo.to_owned()),
        ("run_id".to_owned(), "12345".to_owned()),
        ("ref".to_owned(), "abc123".to_owned()),
        ("job_id".to_owned(), "777".to_owned()),
    ])
}

fn request(action: &str, metadata: BTreeMap<String, String>) -> workflow_core::AdapterRequest {
    github_actions_read_request(
        action,
        actor(),
        correlation(),
        metadata,
        AdapterOperationMode::Fixture,
        AdapterPolicyPrecheck::fixture_test_allowed(vec!["policy.fixture.read".to_owned()]),
    )
    .expect("request")
}

fn runtime_authorized_request(
    action: &str,
    metadata: BTreeMap<String, String>,
) -> workflow_core::AdapterRequest {
    github_actions_read_request(
        action,
        actor(),
        correlation(),
        metadata,
        AdapterOperationMode::Fixture,
        AdapterPolicyPrecheck::runtime_allowed(vec!["policy.allow.ci_read".to_owned()]),
    )
    .expect("request")
}

fn adapter(
    client: GitHubActionsFixtureClient,
) -> GitHubActionsReadOnlyAdapter<GitHubActionsFixtureClient> {
    GitHubActionsReadOnlyAdapter::new(
        GitHubActionsReadOnlyConfig::fixture().expect("config"),
        client,
    )
}

fn workflow_run_body() -> &'static str {
    r#"{
        "id":12345,
        "name":"CI",
        "status":"completed",
        "conclusion":"failure",
        "head_branch":"main",
        "html_url":"https://github.com/acme/widgets/actions/runs/12345"
    }"#
}

fn jobs_body() -> &'static str {
    r#"{
        "total_count":2,
        "jobs":[
            {"id":777,"name":"test","status":"completed","conclusion":"failure","html_url":"https://github.com/acme/widgets/actions/runs/12345/job/777"},
            {"id":778,"name":"lint","status":"completed","conclusion":"success","html_url":"https://github.com/acme/widgets/actions/runs/12345/job/778"}
        ]
    }"#
}

#[test]
fn ci_read_request_records_explicit_fixture_policy_provenance() {
    let request = request(
        ci_actions::WORKFLOW_RUN_METADATA,
        metadata("acme", "widgets"),
    );

    assert_eq!(
        request.policy_precheck,
        AdapterPolicyPrecheck::Allowed {
            provenance: AdapterPolicyPrecheckProvenance::FixtureTest,
            reason_codes: vec!["policy.fixture.read".to_owned()]
        }
    );
}

#[test]
fn missing_policy_precheck_fails_closed() {
    let adapter = adapter(GitHubActionsFixtureClient::new());
    let mut request = request(
        ci_actions::WORKFLOW_RUN_METADATA,
        metadata("acme", "widgets"),
    );
    request.policy_precheck = AdapterPolicyPrecheck::Missing;

    let error = adapter.read(&request).expect_err("missing policy denied");

    assert_eq!(error.kind, AdapterErrorKind::PolicyDenied);
    assert_eq!(error.code, "adapter.policy.required");
}

#[test]
fn denied_policy_precheck_prevents_ci_invocation() {
    let adapter = adapter(GitHubActionsFixtureClient::new());
    let mut request = request(
        ci_actions::WORKFLOW_RUN_METADATA,
        metadata("acme", "widgets"),
    );
    request.policy_precheck =
        AdapterPolicyPrecheck::runtime_denied(vec!["policy.deny.test".to_owned()]);

    let error = adapter
        .read(&request)
        .expect_err("denied policy blocks read");

    assert_eq!(error.kind, AdapterErrorKind::PolicyDenied);
    assert_eq!(error.code, "adapter.policy.required");
}

#[test]
fn runtime_policy_allowed_precheck_permits_ci_read() {
    let client = GitHubActionsFixtureClient::new().with_json(
        "/repos/acme/widgets/actions/runs/12345",
        workflow_run_body(),
    );
    let adapter = adapter(client);
    let request = runtime_authorized_request(
        ci_actions::WORKFLOW_RUN_METADATA,
        metadata("acme", "widgets"),
    );

    let response = adapter.read(&request).expect("runtime policy allowed read");

    assert_eq!(response.status, AdapterResponseStatus::Success);
}

#[test]
fn fixture_workflow_run_metadata_read() {
    let client = GitHubActionsFixtureClient::new().with_json(
        "/repos/acme/widgets/actions/runs/12345",
        workflow_run_body(),
    );
    let adapter = adapter(client);
    let request = request(
        ci_actions::WORKFLOW_RUN_METADATA,
        metadata("acme", "widgets"),
    );

    let outcome = adapter
        .read_workflow_run_metadata(&request, "acme", "widgets", "12345")
        .expect("workflow run metadata");

    assert_eq!(outcome.response.status, AdapterResponseStatus::Success);
    assert!(outcome
        .response
        .summary
        .contains("GitHub Actions run 12345"));
    assert!(outcome.response.summary.contains("conclusion=failure"));
}

#[test]
fn fixture_jobs_read() {
    let client = GitHubActionsFixtureClient::new()
        .with_json("/repos/acme/widgets/actions/runs/12345/jobs", jobs_body());
    let adapter = adapter(client);
    let request = request(ci_actions::JOB_STATUS_SUMMARY, metadata("acme", "widgets"));

    let response = adapter.read(&request).expect("jobs");

    assert!(response.summary.contains("jobs=2"));
    assert!(response.summary.contains("failure"));
    assert!(response.summary.contains("test"));
}

#[test]
fn fixture_check_status_summary_read() {
    let client = GitHubActionsFixtureClient::new().with_json(
        "/repos/acme/widgets/commits/abc123/check-runs",
        r#"{"total_count":2,"check_runs":[{"name":"test","status":"completed","conclusion":"failure","html_url":"https://github.com/acme/widgets/runs/1"},{"name":"lint","status":"completed","conclusion":"success"}]}"#,
    );
    let adapter = adapter(client);
    let request = request(
        ci_actions::CHECK_STATUS_SUMMARY,
        metadata("acme", "widgets"),
    );

    let response = adapter.read(&request).expect("checks");

    assert!(response.summary.contains("check_runs=2"));
    assert!(response.summary.contains("failure"));
}

#[test]
fn fixture_failure_summary_read() {
    let client = GitHubActionsFixtureClient::new()
        .with_json("/repos/acme/widgets/actions/runs/12345/jobs", jobs_body());
    let adapter = adapter(client);
    let request = request(ci_actions::FAILURE_SUMMARY, metadata("acme", "widgets"));

    let response = adapter.read(&request).expect("failure summary");

    assert!(response.summary.contains("failed_jobs=1"));
    assert!(response.summary.contains("sample=test"));
    assert!(response.summary.contains("logs=reference-only"));
}

#[test]
fn fixture_log_reference_read_does_not_download_logs() {
    let adapter = adapter(GitHubActionsFixtureClient::new());
    let request = request(ci_actions::LOG_REFERENCE, metadata("acme", "widgets"));

    let response = adapter.read(&request).expect("log reference");

    assert!(response.summary.contains("logs=reference-only"));
    assert_eq!(
        response.external_references,
        vec!["github-actions://acme/widgets/actions/runs/12345/logs"]
    );
}

#[test]
fn log_excerpt_is_redacted_and_size_limited() {
    let long_safe = "a".repeat(3_000);
    let client = GitHubActionsFixtureClient::new().with_text(
        "/repos/acme/widgets/actions/jobs/777/logs",
        format!("building project\nTOKEN=ghs_secret_token\npassword=hunter2\n{long_safe}"),
    );
    let adapter = adapter(client);
    let request = request(ci_actions::LOG_EXCERPT, metadata("acme", "widgets"));

    let response = adapter.read(&request).expect("log excerpt");

    assert!(response.summary.contains("building project"));
    assert!(response.summary.contains("[REDACTED_LOG_LINE]"));
    assert!(response.summary.contains("truncated=true"));
    assert!(!response.summary.contains("ghs_secret_token"));
    assert!(!response.summary.contains("hunter2"));
    assert!(response.summary.len() < 2_500);
}

#[test]
fn missing_credentials_health_check_does_not_expose_token() {
    let adapter = adapter(GitHubActionsFixtureClient::new());

    let health = adapter.health();
    let debug = format!("{health:?}");

    assert!(health.configured);
    assert!(!health.credential_present);
    assert!(!debug.contains("ghs_"));
    assert!(health
        .warnings
        .iter()
        .any(|warning| warning.contains("GitHub Actions token not configured")));
}

#[test]
fn configured_credentials_health_check_without_exposing_token() {
    let config =
        GitHubActionsReadOnlyConfig::live_read_only_with_token("ghs_secret_token".to_owned())
            .expect("config");
    let adapter = GitHubActionsReadOnlyAdapter::new(config, GitHubActionsFixtureClient::new());

    let health = adapter.health();
    let debug = format!("{health:?}");

    assert!(health.configured);
    assert!(health.credential_present);
    assert!(!debug.contains("ghs_secret_token"));
}

#[test]
fn auth_failure_classified() {
    let client = GitHubActionsFixtureClient::new().with_error(
        "/repos/acme/widgets/actions/runs/12345",
        401,
        r#"{"message":"Bad credentials"}"#,
        BTreeMap::new(),
    );
    let adapter = adapter(client);
    let request = request(
        ci_actions::WORKFLOW_RUN_METADATA,
        metadata("acme", "widgets"),
    );

    let error = adapter.read(&request).expect_err("auth failure");

    assert_eq!(error.kind, AdapterErrorKind::AuthFailure);
}

#[test]
fn permission_failure_classified() {
    let client = GitHubActionsFixtureClient::new().with_error(
        "/repos/acme/widgets/actions/runs/12345",
        403,
        r#"{"message":"Resource not accessible"}"#,
        BTreeMap::new(),
    );
    let adapter = adapter(client);
    let request = request(
        ci_actions::WORKFLOW_RUN_METADATA,
        metadata("acme", "widgets"),
    );

    let error = adapter.read(&request).expect_err("permission failure");

    assert_eq!(error.kind, AdapterErrorKind::PermissionFailure);
}

#[test]
fn not_found_classified() {
    let client = GitHubActionsFixtureClient::new().with_error(
        "/repos/acme/widgets/actions/runs/12345",
        404,
        r#"{"message":"Not Found"}"#,
        BTreeMap::new(),
    );
    let adapter = adapter(client);
    let request = request(
        ci_actions::WORKFLOW_RUN_METADATA,
        metadata("acme", "widgets"),
    );

    let error = adapter.read(&request).expect_err("not found");

    assert_eq!(error.kind, AdapterErrorKind::NotFound);
}

#[test]
fn rate_limit_classified() {
    let client = GitHubActionsFixtureClient::new().with_error(
        "/repos/acme/widgets/actions/runs/12345",
        403,
        r#"{"message":"API rate limit exceeded"}"#,
        BTreeMap::from([("x-ratelimit-remaining".to_owned(), "0".to_owned())]),
    );
    let adapter = adapter(client);
    let request = request(
        ci_actions::WORKFLOW_RUN_METADATA,
        metadata("acme", "widgets"),
    );

    let error = adapter.read(&request).expect_err("rate limit");

    assert_eq!(error.kind, AdapterErrorKind::RateLimited);
}

#[test]
fn rerun_operation_unavailable_and_denied() {
    let client = GitHubActionsFixtureClient::new();
    let adapter = adapter(client);
    let mut request = request("ci.workflow.rerun", metadata("acme", "widgets"));
    request.capability = AdapterCapability::CiRerun;
    request.action.side_effecting = true;
    request.action.required_capabilities = vec![AdapterCapability::CiRerun];

    let error = adapter.read(&request).expect_err("rerun denied");

    assert_eq!(error.kind, AdapterErrorKind::PermissionFailure);
    assert_eq!(error.code, "ci.capability.read_required");
}

#[test]
fn workflow_dispatch_unavailable_and_denied() {
    let client = GitHubActionsFixtureClient::new();
    let adapter = adapter(client);
    let mut request = request("ci.workflow.dispatch", metadata("acme", "widgets"));
    request.capability = AdapterCapability::CiWrite;
    request.action.side_effecting = true;
    request.action.required_capabilities = vec![AdapterCapability::CiWrite];

    let error = adapter.read(&request).expect_err("dispatch denied");

    assert_eq!(error.kind, AdapterErrorKind::PermissionFailure);
    assert_eq!(error.code, "ci.capability.read_required");
}

#[test]
fn adapter_emits_audit_and_observability_record() {
    let client = GitHubActionsFixtureClient::new().with_json(
        "/repos/acme/widgets/actions/runs/12345",
        workflow_run_body(),
    );
    let adapter = adapter(client);
    let request = request(
        ci_actions::WORKFLOW_RUN_METADATA,
        metadata("acme", "widgets"),
    );

    let outcome = adapter
        .read_workflow_run_metadata(&request, "acme", "widgets", "12345")
        .expect("read");

    assert_eq!(
        outcome.invocation.adapter_kind,
        workflow_core::AdapterKind::Ci
    );
    assert_eq!(outcome.invocation.capability, AdapterCapability::CiRead);
    assert_eq!(outcome.observability.status, AdapterResponseStatus::Success);
}

#[test]
fn no_token_appears_in_debug_audit_or_health_output() {
    let config =
        GitHubActionsReadOnlyConfig::live_read_only_with_token("ghs_secret_token".to_owned())
            .expect("config");
    let adapter = GitHubActionsReadOnlyAdapter::new(
        config,
        GitHubActionsFixtureClient::new().with_json(
            "/repos/acme/widgets/actions/runs/12345",
            workflow_run_body(),
        ),
    );
    let request = request(
        ci_actions::WORKFLOW_RUN_METADATA,
        metadata("acme", "widgets"),
    );
    let outcome = adapter
        .read_workflow_run_metadata(&request, "acme", "widgets", "12345")
        .expect("read");

    let combined = format!(
        "{:?}{:?}{:?}",
        adapter.health(),
        outcome.invocation,
        outcome.observability
    );

    assert!(!combined.contains("ghs_secret_token"));
    assert!(!combined.contains("ghs_"));
}

#[test]
fn fixture_tests_do_not_require_live_ci_credentials() {
    let config = GitHubActionsReadOnlyConfig::fixture().expect("fixture config");

    assert!(!config.credential_present());
}

#[test]
#[ignore = "opt-in live GitHub Actions read-only test; requires WORKFLOW_OS_LIVE_GITHUB_ACTIONS_TESTS=1 and a read-only token"]
fn live_github_actions_workflow_run_read_is_opt_in() {
    if std::env::var("WORKFLOW_OS_LIVE_GITHUB_ACTIONS_TESTS")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }
    let config = GitHubActionsReadOnlyConfig::from_env().expect("live config");
    let client = workflow_core::GitHubActionsLiveReadOnlyClient::new(config.clone());
    let adapter = GitHubActionsReadOnlyAdapter::new(config, client);
    let owner = std::env::var("WORKFLOW_OS_GITHUB_ACTIONS_TEST_OWNER").expect("owner");
    let repo = std::env::var("WORKFLOW_OS_GITHUB_ACTIONS_TEST_REPO").expect("repo");
    let run_id = std::env::var("WORKFLOW_OS_GITHUB_ACTIONS_TEST_RUN_ID").expect("run id");
    let request = request(
        ci_actions::WORKFLOW_RUN_METADATA,
        BTreeMap::from([
            ("owner".to_owned(), owner.clone()),
            ("repo".to_owned(), repo.clone()),
            ("run_id".to_owned(), run_id.clone()),
        ]),
    );

    let outcome = adapter
        .read_workflow_run_metadata(&request, &owner, &repo, &run_id)
        .expect("live read");

    assert!(outcome.response.summary.contains(&run_id));
}
