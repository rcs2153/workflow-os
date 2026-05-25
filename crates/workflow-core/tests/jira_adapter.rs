#![allow(clippy::expect_used)]
//! Jira read-only adapter tests use fixture clients by default.

use std::collections::BTreeMap;

use workflow_core::{
    jira_actions, jira_read_request, ActorId, AdapterCapability, AdapterErrorKind,
    AdapterOperationMode, AdapterPolicyPrecheck, AdapterPolicyPrecheckProvenance,
    AdapterReadOperation, AdapterResponseStatus, CorrelationId, JiraFixtureClient,
    JiraReadOnlyAdapter, JiraReadOnlyConfig,
};

fn actor() -> ActorId {
    ActorId::new("system/jira-adapter-test").expect("actor")
}

fn correlation() -> CorrelationId {
    CorrelationId::new("correlation/jira-adapter-test").expect("correlation")
}

fn issue_metadata(issue_key: &str) -> BTreeMap<String, String> {
    BTreeMap::from([("issue_key".to_owned(), issue_key.to_owned())])
}

fn project_metadata(project_key: &str) -> BTreeMap<String, String> {
    BTreeMap::from([("project_key".to_owned(), project_key.to_owned())])
}

fn request(action: &str, metadata: BTreeMap<String, String>) -> workflow_core::AdapterRequest {
    jira_read_request(
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
    jira_read_request(
        action,
        actor(),
        correlation(),
        metadata,
        AdapterOperationMode::Fixture,
        AdapterPolicyPrecheck::runtime_allowed(vec!["policy.allow.jira_read".to_owned()]),
    )
    .expect("request")
}

fn adapter(client: JiraFixtureClient) -> JiraReadOnlyAdapter<JiraFixtureClient> {
    JiraReadOnlyAdapter::new(JiraReadOnlyConfig::fixture().expect("config"), client)
}

fn issue_body() -> &'static str {
    r#"{
        "key":"OPS-42",
        "self":"https://example.atlassian.net/rest/api/3/issue/10042",
        "fields":{
            "summary":"Review vendor access request",
            "description":{"type":"doc","content":[{"text":"secret access rationale"}]},
            "status":{"name":"In Review"},
            "priority":{"name":"High"},
            "labels":["access","vendor"],
            "assignee":{"displayName":"Avery Approver"},
            "reporter":{"displayName":"Riley Requester"},
            "project":{"key":"OPS","name":"Operations"}
        }
    }"#
}

#[test]
fn jira_read_request_records_explicit_fixture_policy_provenance() {
    let request = request(jira_actions::ISSUE_METADATA, issue_metadata("OPS-42"));

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
    let adapter = adapter(JiraFixtureClient::new());
    let mut request = request(jira_actions::ISSUE_METADATA, issue_metadata("OPS-42"));
    request.policy_precheck = AdapterPolicyPrecheck::Missing;

    let error = adapter.read(&request).expect_err("missing policy denied");

    assert_eq!(error.kind, AdapterErrorKind::PolicyDenied);
    assert_eq!(error.code, "adapter.policy.required");
}

#[test]
fn denied_policy_precheck_prevents_jira_invocation() {
    let adapter = adapter(JiraFixtureClient::new());
    let mut request = request(jira_actions::ISSUE_METADATA, issue_metadata("OPS-42"));
    request.policy_precheck =
        AdapterPolicyPrecheck::runtime_denied(vec!["policy.deny.test".to_owned()]);

    let error = adapter
        .read(&request)
        .expect_err("denied policy blocks read");

    assert_eq!(error.kind, AdapterErrorKind::PolicyDenied);
    assert_eq!(error.code, "adapter.policy.required");
}

#[test]
fn runtime_policy_allowed_precheck_permits_jira_read() {
    let client = JiraFixtureClient::new().with_json("/rest/api/3/issue/OPS-42", issue_body());
    let adapter = adapter(client);
    let request =
        runtime_authorized_request(jira_actions::ISSUE_METADATA, issue_metadata("OPS-42"));

    let response = adapter.read(&request).expect("runtime policy allowed read");

    assert_eq!(response.status, AdapterResponseStatus::Success);
}

#[test]
fn fixture_issue_metadata_read() {
    let client = JiraFixtureClient::new().with_json("/rest/api/3/issue/OPS-42", issue_body());
    let adapter = adapter(client);
    let request = request(jira_actions::ISSUE_METADATA, issue_metadata("OPS-42"));

    let outcome = adapter
        .read_issue_metadata(&request, "OPS-42")
        .expect("issue metadata");

    assert_eq!(outcome.response.status, AdapterResponseStatus::Success);
    assert!(outcome.response.summary.contains("Jira issue OPS-42"));
    assert!(outcome.response.summary.contains("status=In Review"));
    assert!(outcome.response.summary.contains("priority=High"));
    assert!(outcome.response.summary.contains("labels=access,vendor"));
    assert!(outcome
        .response
        .summary
        .contains("description=reference-only"));
    assert!(!outcome.response.summary.contains("secret access rationale"));
}

#[test]
fn fixture_issue_description_is_reference_only() {
    let client = JiraFixtureClient::new().with_json("/rest/api/3/issue/OPS-42", issue_body());
    let adapter = adapter(client);
    let request = request(jira_actions::ISSUE_DESCRIPTION, issue_metadata("OPS-42"));

    let response = adapter.read(&request).expect("issue description");

    assert!(response.summary.contains("description_present=true"));
    assert!(response.summary.contains("description=reference-only"));
    assert!(!response.summary.contains("secret access rationale"));
}

#[test]
fn fixture_comments_read_is_reference_only() {
    let client = JiraFixtureClient::new().with_json(
        "/rest/api/3/issue/OPS-42/comment",
        r#"{"comments":[{"self":"https://example.atlassian.net/rest/api/3/issue/10042/comment/1","body":{"content":[{"text":"secret discussion"}]}}]}"#,
    );
    let adapter = adapter(client);
    let request = request(jira_actions::ISSUE_COMMENTS, issue_metadata("OPS-42"));

    let response = adapter.read(&request).expect("issue comments");

    assert!(response.summary.contains("comments=1"));
    assert!(response.summary.contains("bodies=reference-only"));
    assert!(!response.summary.contains("secret discussion"));
    assert_eq!(
        response.external_references,
        vec!["https://example.atlassian.net/rest/api/3/issue/10042/comment/1"]
    );
}

#[test]
fn fixture_status_priority_labels_and_people_read() {
    let client = JiraFixtureClient::new().with_json("/rest/api/3/issue/OPS-42", issue_body());
    let adapter = adapter(client);

    let status = adapter
        .read(&request(
            jira_actions::ISSUE_STATUS,
            issue_metadata("OPS-42"),
        ))
        .expect("status");
    let priority = adapter
        .read(&request(
            jira_actions::ISSUE_PRIORITY,
            issue_metadata("OPS-42"),
        ))
        .expect("priority");
    let labels = adapter
        .read(&request(
            jira_actions::ISSUE_LABELS,
            issue_metadata("OPS-42"),
        ))
        .expect("labels");
    let people = adapter
        .read(&request(
            jira_actions::ISSUE_PEOPLE,
            issue_metadata("OPS-42"),
        ))
        .expect("people");

    assert!(status.summary.contains("status=In Review"));
    assert!(priority.summary.contains("priority=High"));
    assert!(labels.summary.contains("labels=access,vendor"));
    assert!(people.summary.contains("assignee=Avery Approver"));
    assert!(people.summary.contains("reporter=Riley Requester"));
}

#[test]
fn fixture_project_metadata_read() {
    let client = JiraFixtureClient::new().with_json(
        "/rest/api/3/project/OPS",
        r#"{"key":"OPS","name":"Operations","projectTypeKey":"business","self":"https://example.atlassian.net/rest/api/3/project/OPS"}"#,
    );
    let adapter = adapter(client);
    let request = request(jira_actions::PROJECT_METADATA, project_metadata("OPS"));

    let response = adapter.read(&request).expect("project metadata");

    assert!(response.summary.contains("Jira project OPS"));
    assert!(response.summary.contains("project_type=business"));
}

#[test]
fn missing_credentials_health_check_does_not_expose_token() {
    let adapter = adapter(JiraFixtureClient::new());

    let health = adapter.health();
    let debug = format!("{health:?}");

    assert!(health.configured);
    assert!(!health.credential_present);
    assert!(!debug.contains("jira_secret"));
    assert!(health
        .warnings
        .iter()
        .any(|warning| warning.contains("Jira auth not configured")));
}

#[test]
fn configured_credentials_health_check_without_exposing_token() {
    let config = JiraReadOnlyConfig::live_read_only_with_basic_auth(
        "https://example.atlassian.net".to_owned(),
        "person@example.com".to_owned(),
        "jira_secret_token".to_owned(),
    )
    .expect("config");
    let adapter = JiraReadOnlyAdapter::new(config, JiraFixtureClient::new());

    let health = adapter.health();
    let debug = format!("{health:?}");

    assert!(health.configured);
    assert!(health.credential_present);
    assert!(!debug.contains("jira_secret_token"));
    assert!(!debug.contains("person@example.com"));
}

#[test]
fn configured_auth_mode_builds_expected_redacted_request_metadata() {
    let config = JiraReadOnlyConfig::live_read_only_with_basic_auth(
        "https://example.atlassian.net".to_owned(),
        "person@example.com".to_owned(),
        "jira_secret_token".to_owned(),
    )
    .expect("config");

    let metadata = config.redacted_auth_metadata();
    let debug = format!("{config:?}{metadata:?}");

    assert_eq!(metadata.get("auth_mode").map(String::as_str), Some("basic"));
    assert_eq!(
        metadata.get("basic_email_configured").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        metadata
            .get("basic_api_token_configured")
            .map(String::as_str),
        Some("true")
    );
    assert!(!debug.contains("person@example.com"));
    assert!(!debug.contains("jira_secret_token"));
}

#[test]
fn auth_failure_classified() {
    let client = JiraFixtureClient::new().with_error(
        "/rest/api/3/issue/OPS-42",
        401,
        r#"{"errorMessages":["Unauthorized"]}"#,
        BTreeMap::new(),
    );
    let adapter = adapter(client);
    let request = request(jira_actions::ISSUE_METADATA, issue_metadata("OPS-42"));

    let error = adapter.read(&request).expect_err("auth failure");

    assert_eq!(error.kind, AdapterErrorKind::AuthFailure);
}

#[test]
fn permission_failure_classified() {
    let client = JiraFixtureClient::new().with_error(
        "/rest/api/3/issue/OPS-42",
        403,
        r#"{"errorMessages":["Forbidden"]}"#,
        BTreeMap::new(),
    );
    let adapter = adapter(client);
    let request = request(jira_actions::ISSUE_METADATA, issue_metadata("OPS-42"));

    let error = adapter.read(&request).expect_err("permission failure");

    assert_eq!(error.kind, AdapterErrorKind::PermissionFailure);
}

#[test]
fn not_found_classified() {
    let client = JiraFixtureClient::new().with_error(
        "/rest/api/3/issue/OPS-99",
        404,
        r#"{"errorMessages":["Issue does not exist"]}"#,
        BTreeMap::new(),
    );
    let adapter = adapter(client);
    let request = request(jira_actions::ISSUE_METADATA, issue_metadata("OPS-99"));

    let error = adapter.read(&request).expect_err("not found");

    assert_eq!(error.kind, AdapterErrorKind::NotFound);
}

#[test]
fn rate_limit_classified() {
    let client = JiraFixtureClient::new().with_error(
        "/rest/api/3/issue/OPS-42",
        429,
        r#"{"errorMessages":["Rate limit exceeded"]}"#,
        BTreeMap::from([("retry-after".to_owned(), "60".to_owned())]),
    );
    let adapter = adapter(client);
    let request = request(jira_actions::ISSUE_METADATA, issue_metadata("OPS-42"));

    let error = adapter.read(&request).expect_err("rate limit");

    assert_eq!(error.kind, AdapterErrorKind::RateLimited);
}

#[test]
fn write_operation_unavailable_and_denied() {
    let client = JiraFixtureClient::new();
    let adapter = adapter(client);
    let mut request = request("jira.issue.update", issue_metadata("OPS-42"));
    request.capability = AdapterCapability::JiraWrite;
    request.action.side_effecting = true;
    request.action.required_capabilities = vec![AdapterCapability::JiraWrite];

    let error = adapter.read(&request).expect_err("write denied");

    assert_eq!(error.kind, AdapterErrorKind::PermissionFailure);
    assert_eq!(error.code, "jira.capability.read_required");
}

#[test]
fn adapter_emits_audit_and_observability_record() {
    let client = JiraFixtureClient::new().with_json("/rest/api/3/issue/OPS-42", issue_body());
    let adapter = adapter(client);
    let request = request(jira_actions::ISSUE_METADATA, issue_metadata("OPS-42"));

    let outcome = adapter
        .read_issue_metadata(&request, "OPS-42")
        .expect("read");

    assert_eq!(
        outcome.invocation.adapter_kind,
        workflow_core::AdapterKind::Jira
    );
    assert_eq!(outcome.invocation.capability, AdapterCapability::JiraRead);
    assert_eq!(outcome.observability.status, AdapterResponseStatus::Success);
}

#[test]
fn no_token_appears_in_debug_audit_or_health_output() {
    let config = JiraReadOnlyConfig::live_read_only_with_basic_auth(
        "https://example.atlassian.net".to_owned(),
        "person@example.com".to_owned(),
        "jira_secret_token".to_owned(),
    )
    .expect("config");
    let adapter = JiraReadOnlyAdapter::new(
        config,
        JiraFixtureClient::new().with_json("/rest/api/3/issue/OPS-42", issue_body()),
    );
    let request = request(jira_actions::ISSUE_METADATA, issue_metadata("OPS-42"));
    let outcome = adapter
        .read_issue_metadata(&request, "OPS-42")
        .expect("read");

    let combined = format!(
        "{:?}{:?}{:?}",
        adapter.health(),
        outcome.invocation,
        outcome.observability
    );

    assert!(!combined.contains("jira_secret_token"));
    assert!(!combined.contains("jira_secret"));
    assert!(!combined.contains("person@example.com"));
}

#[test]
fn fixture_tests_do_not_require_live_jira_credentials() {
    let config = JiraReadOnlyConfig::fixture().expect("fixture config");

    assert!(!config.credential_present());
}

#[test]
#[ignore = "opt-in live Jira read-only test; requires WORKFLOW_OS_LIVE_JIRA_TESTS=1, WORKFLOW_OS_JIRA_BASE_URL, and read-only Jira auth"]
fn live_jira_issue_metadata_read_is_opt_in() {
    if std::env::var("WORKFLOW_OS_LIVE_JIRA_TESTS").ok().as_deref() != Some("1") {
        return;
    }
    let config = JiraReadOnlyConfig::from_env().expect("live config");
    let client = workflow_core::JiraLiveReadOnlyClient::new(config.clone());
    let adapter = JiraReadOnlyAdapter::new(config, client);
    let issue_key = std::env::var("WORKFLOW_OS_JIRA_TEST_ISSUE_KEY").expect("test issue key");
    let request = request(jira_actions::ISSUE_METADATA, issue_metadata(&issue_key));

    let outcome = adapter
        .read_issue_metadata(&request, &issue_key)
        .expect("live read");

    assert!(outcome.response.summary.contains(&issue_key));
}
