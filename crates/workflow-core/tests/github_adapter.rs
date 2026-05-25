#![allow(clippy::expect_used)]
//! GitHub read-only adapter tests use fixture clients by default.

use std::collections::BTreeMap;

use workflow_core::{
    github_actions, github_read_request, ActorId, AdapterCapability, AdapterErrorKind,
    AdapterOperationMode, AdapterPolicyPrecheck, AdapterPolicyPrecheckProvenance,
    AdapterReadOperation, AdapterResponseStatus, CorrelationId, GitHubFixtureClient,
    GitHubReadOnlyAdapter, GitHubReadOnlyConfig,
};

fn actor() -> ActorId {
    ActorId::new("system/github-adapter-test").expect("actor")
}

fn correlation() -> CorrelationId {
    CorrelationId::new("correlation/github-adapter-test").expect("correlation")
}

fn metadata(owner: &str, repo: &str) -> BTreeMap<String, String> {
    BTreeMap::from([
        ("owner".to_owned(), owner.to_owned()),
        ("repo".to_owned(), repo.to_owned()),
    ])
}

fn request(action: &str, mut metadata: BTreeMap<String, String>) -> workflow_core::AdapterRequest {
    metadata
        .entry("ref".to_owned())
        .or_insert_with(|| "main".to_owned());
    github_read_request(
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
    mut metadata: BTreeMap<String, String>,
) -> workflow_core::AdapterRequest {
    metadata
        .entry("ref".to_owned())
        .or_insert_with(|| "main".to_owned());
    github_read_request(
        action,
        actor(),
        correlation(),
        metadata,
        AdapterOperationMode::Fixture,
        AdapterPolicyPrecheck::runtime_allowed(vec!["policy.allow.github_read".to_owned()]),
    )
    .expect("request")
}

fn adapter(client: GitHubFixtureClient) -> GitHubReadOnlyAdapter<GitHubFixtureClient> {
    GitHubReadOnlyAdapter::new(GitHubReadOnlyConfig::fixture().expect("config"), client)
}

#[test]
fn fixture_repo_metadata_read() {
    let client = GitHubFixtureClient::new().with_json(
        "/repos/acme/widgets",
        r#"{"full_name":"acme/widgets","default_branch":"main","private":true,"html_url":"https://github.com/acme/widgets"}"#,
    );
    let adapter = adapter(client);
    let request = request(
        github_actions::REPOSITORY_METADATA,
        metadata("acme", "widgets"),
    );

    let outcome = adapter
        .read_repository_metadata(&request, "acme", "widgets")
        .expect("repo metadata");

    assert_eq!(outcome.response.status, AdapterResponseStatus::Success);
    assert!(outcome.response.summary.contains("acme/widgets"));
    assert!(outcome.response.summary.contains("private=true"));
    assert_eq!(
        outcome.response.external_references,
        vec!["https://github.com/acme/widgets"]
    );
}

#[test]
fn github_read_request_records_explicit_fixture_policy_provenance() {
    let request = request(
        github_actions::REPOSITORY_METADATA,
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
    let adapter = adapter(GitHubFixtureClient::new());
    let mut request = request(
        github_actions::REPOSITORY_METADATA,
        metadata("acme", "widgets"),
    );
    request.policy_precheck = AdapterPolicyPrecheck::Missing;

    let error = adapter.read(&request).expect_err("missing policy denied");

    assert_eq!(error.kind, AdapterErrorKind::PolicyDenied);
    assert_eq!(error.code, "adapter.policy.required");
}

#[test]
fn denied_policy_precheck_prevents_github_invocation() {
    let adapter = adapter(GitHubFixtureClient::new());
    let mut request = request(
        github_actions::REPOSITORY_METADATA,
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
fn runtime_policy_allowed_precheck_permits_github_read() {
    let client = GitHubFixtureClient::new().with_json(
        "/repos/acme/widgets",
        r#"{"full_name":"acme/widgets","default_branch":"main","private":true}"#,
    );
    let adapter = adapter(client);
    let request = runtime_authorized_request(
        github_actions::REPOSITORY_METADATA,
        metadata("acme", "widgets"),
    );

    let response = adapter.read(&request).expect("runtime policy allowed read");

    assert_eq!(response.status, AdapterResponseStatus::Success);
}

#[test]
fn fixture_default_branch_read_through_trait_dispatch() {
    let client = GitHubFixtureClient::new().with_json(
        "/repos/acme/widgets",
        r#"{"full_name":"acme/widgets","default_branch":"trunk","html_url":"https://github.com/acme/widgets"}"#,
    );
    let adapter = adapter(client);
    let request = request(github_actions::DEFAULT_BRANCH, metadata("acme", "widgets"));

    let response = adapter.read(&request).expect("default branch");

    assert!(response.summary.contains("trunk"));
}

#[test]
fn fixture_file_read_is_reference_only() {
    let client = GitHubFixtureClient::new().with_json(
        "/repos/acme/widgets/contents/README.md?ref=main",
        r#"{"path":"README.md","sha":"abc123","size":42,"content":"token should not be summarized","html_url":"https://github.com/acme/widgets/blob/main/README.md"}"#,
    );
    let adapter = adapter(client);
    let mut meta = metadata("acme", "widgets");
    meta.insert("path".to_owned(), "README.md".to_owned());
    meta.insert("ref".to_owned(), "main".to_owned());
    let request = request(github_actions::FILE_CONTENTS, meta);

    let response = adapter.read(&request).expect("file contents");

    assert!(response.summary.contains("content=reference-only"));
    assert!(!response.summary.contains("token should not be summarized"));
}

#[test]
fn fixture_pr_metadata_read() {
    let client = GitHubFixtureClient::new().with_json(
        "/repos/acme/widgets/pulls/7",
        r#"{"number":7,"state":"open","title":"Improve widget","html_url":"https://github.com/acme/widgets/pull/7"}"#,
    );
    let adapter = adapter(client);
    let mut meta = metadata("acme", "widgets");
    meta.insert("pull_number".to_owned(), "7".to_owned());
    let request = request(github_actions::PULL_REQUEST_METADATA, meta);

    let response = adapter.read(&request).expect("pr metadata");

    assert!(response.summary.contains("#7"));
    assert!(response.summary.contains("state=open"));
}

#[test]
fn fixture_pr_changed_files_read() {
    let client = GitHubFixtureClient::new().with_json(
        "/repos/acme/widgets/pulls/7/files",
        r#"[{"filename":"src/lib.rs"},{"filename":"README.md"}]"#,
    );
    let adapter = adapter(client);
    let mut meta = metadata("acme", "widgets");
    meta.insert("pull_number".to_owned(), "7".to_owned());
    let request = request(github_actions::PULL_REQUEST_CHANGED_FILES, meta);

    let response = adapter.read(&request).expect("changed files");

    assert!(response.summary.contains("changed_files=2"));
    assert!(response.summary.contains("src/lib.rs"));
}

#[test]
fn fixture_pr_comments_read() {
    let client = GitHubFixtureClient::new().with_json(
        "/repos/acme/widgets/issues/7/comments",
        r#"[{"html_url":"https://github.com/acme/widgets/pull/7#issuecomment-1","body":"secret body"}]"#,
    );
    let adapter = adapter(client);
    let mut meta = metadata("acme", "widgets");
    meta.insert("pull_number".to_owned(), "7".to_owned());
    let request = request(github_actions::PULL_REQUEST_COMMENTS, meta);

    let response = adapter.read(&request).expect("comments");

    assert!(response.summary.contains("comments=1"));
    assert!(response.summary.contains("bodies=reference-only"));
    assert!(!response.summary.contains("secret body"));
}

#[test]
fn fixture_check_status_summary_read() {
    let client = GitHubFixtureClient::new().with_json(
        "/repos/acme/widgets/commits/main/check-runs",
        r#"{"total_count":2,"check_runs":[{"name":"test","status":"completed","conclusion":"success"},{"name":"lint","status":"queued","conclusion":null}]}"#,
    );
    let adapter = adapter(client);
    let request = request(
        github_actions::CHECK_STATUS_SUMMARY,
        metadata("acme", "widgets"),
    );

    let response = adapter.read(&request).expect("checks");

    assert!(response.summary.contains("check runs=2"));
    assert!(response.summary.contains("success"));
}

#[test]
fn fixture_pr_diff_summary_read() {
    let client = GitHubFixtureClient::new().with_json(
        "/repos/acme/widgets/pulls/7.diff",
        "diff --git a/a.txt b/a.txt\n+new\n-old\n",
    );
    let adapter = adapter(client);
    let mut meta = metadata("acme", "widgets");
    meta.insert("pull_number".to_owned(), "7".to_owned());
    let request = request(github_actions::PULL_REQUEST_DIFF_SUMMARY, meta);

    let response = adapter.read(&request).expect("diff summary");

    assert!(response.summary.contains("files=1"));
    assert!(response.summary.contains("additions=1"));
    assert!(response.summary.contains("deletions=1"));
}

#[test]
fn missing_credentials_health_check_does_not_expose_token() {
    let adapter = adapter(GitHubFixtureClient::new());

    let health = adapter.health();
    let debug = format!("{health:?}");

    assert!(health.configured);
    assert!(!health.credential_present);
    assert!(!debug.contains("ghp_"));
    assert!(health
        .warnings
        .iter()
        .any(|warning| warning.contains("GitHub token not configured")));
}

#[test]
fn configured_credentials_health_check_without_exposing_token() {
    let config = GitHubReadOnlyConfig::live_read_only_with_token("ghp_secret_token".to_owned())
        .expect("config");
    let adapter = GitHubReadOnlyAdapter::new(config, GitHubFixtureClient::new());

    let health = adapter.health();
    let debug = format!("{health:?}");

    assert!(health.configured);
    assert!(health.credential_present);
    assert!(!debug.contains("ghp_secret_token"));
}

#[test]
fn auth_failure_classified() {
    let client = GitHubFixtureClient::new().with_error(
        "/repos/acme/widgets",
        401,
        r#"{"message":"Bad credentials"}"#,
        BTreeMap::new(),
    );
    let adapter = adapter(client);
    let request = request(
        github_actions::REPOSITORY_METADATA,
        metadata("acme", "widgets"),
    );

    let error = adapter.read(&request).expect_err("auth failure");

    assert_eq!(error.kind, AdapterErrorKind::AuthFailure);
}

#[test]
fn permission_failure_classified() {
    let client = GitHubFixtureClient::new().with_error(
        "/repos/acme/widgets",
        403,
        r#"{"message":"Resource not accessible"}"#,
        BTreeMap::new(),
    );
    let adapter = adapter(client);
    let request = request(
        github_actions::REPOSITORY_METADATA,
        metadata("acme", "widgets"),
    );

    let error = adapter.read(&request).expect_err("permission failure");

    assert_eq!(error.kind, AdapterErrorKind::PermissionFailure);
}

#[test]
fn not_found_classified() {
    let client = GitHubFixtureClient::new().with_error(
        "/repos/acme/missing",
        404,
        r#"{"message":"Not Found"}"#,
        BTreeMap::new(),
    );
    let adapter = adapter(client);
    let request = request(
        github_actions::REPOSITORY_METADATA,
        metadata("acme", "missing"),
    );

    let error = adapter.read(&request).expect_err("not found");

    assert_eq!(error.kind, AdapterErrorKind::NotFound);
}

#[test]
fn rate_limit_classified() {
    let client = GitHubFixtureClient::new().with_error(
        "/repos/acme/widgets",
        403,
        r#"{"message":"API rate limit exceeded"}"#,
        BTreeMap::from([("x-ratelimit-remaining".to_owned(), "0".to_owned())]),
    );
    let adapter = adapter(client);
    let request = request(
        github_actions::REPOSITORY_METADATA,
        metadata("acme", "widgets"),
    );

    let error = adapter.read(&request).expect_err("rate limit");

    assert_eq!(error.kind, AdapterErrorKind::RateLimited);
}

#[test]
fn write_operation_unavailable_and_denied() {
    let client = GitHubFixtureClient::new();
    let adapter = adapter(client);
    let mut request = request("github.pull_request.merge", metadata("acme", "widgets"));
    request.capability = AdapterCapability::GitHubWrite;
    request.action.side_effecting = true;
    request.action.required_capabilities = vec![AdapterCapability::GitHubWrite];

    let error = adapter.read(&request).expect_err("write denied");

    assert_eq!(error.kind, AdapterErrorKind::PermissionFailure);
    assert_eq!(error.code, "github.capability.read_required");
}

#[test]
fn adapter_emits_audit_and_observability_record() {
    let client = GitHubFixtureClient::new().with_json(
        "/repos/acme/widgets",
        r#"{"full_name":"acme/widgets","default_branch":"main","private":false}"#,
    );
    let adapter = adapter(client);
    let request = request(
        github_actions::REPOSITORY_METADATA,
        metadata("acme", "widgets"),
    );

    let outcome = adapter
        .read_repository_metadata(&request, "acme", "widgets")
        .expect("read");

    assert_eq!(
        outcome.invocation.adapter_kind,
        workflow_core::AdapterKind::GitHub
    );
    assert_eq!(outcome.invocation.capability, AdapterCapability::GitHubRead);
    assert_eq!(outcome.observability.status, AdapterResponseStatus::Success);
}

#[test]
fn no_token_appears_in_debug_audit_or_health_output() {
    let config = GitHubReadOnlyConfig::live_read_only_with_token("ghp_secret_token".to_owned())
        .expect("config");
    let adapter = GitHubReadOnlyAdapter::new(
        config,
        GitHubFixtureClient::new().with_json(
            "/repos/acme/widgets",
            r#"{"full_name":"acme/widgets","default_branch":"main","private":true}"#,
        ),
    );
    let request = request(
        github_actions::REPOSITORY_METADATA,
        metadata("acme", "widgets"),
    );
    let outcome = adapter
        .read_repository_metadata(&request, "acme", "widgets")
        .expect("read");

    let combined = format!(
        "{:?}{:?}{:?}",
        adapter.health(),
        outcome.invocation,
        outcome.observability
    );

    assert!(!combined.contains("ghp_secret_token"));
    assert!(!combined.contains("ghp_"));
}

#[test]
fn fixture_tests_do_not_require_live_github_credentials() {
    let config = GitHubReadOnlyConfig::fixture().expect("fixture config");

    assert!(!config.credential_present());
}

#[test]
#[ignore = "opt-in live GitHub read-only test; requires WORKFLOW_OS_LIVE_GITHUB_TESTS=1 and a read-only token"]
fn live_github_repo_metadata_read_is_opt_in() {
    if std::env::var("WORKFLOW_OS_LIVE_GITHUB_TESTS")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }
    let config = GitHubReadOnlyConfig::from_env().expect("live config");
    let client = workflow_core::GitHubLiveReadOnlyClient::new(config.clone());
    let adapter = GitHubReadOnlyAdapter::new(config, client);
    let request = request(
        github_actions::REPOSITORY_METADATA,
        metadata("octocat", "Hello-World"),
    );

    let outcome = adapter
        .read_repository_metadata(&request, "octocat", "Hello-World")
        .expect("live read");

    assert!(outcome.response.summary.contains("octocat/Hello-World"));
}
