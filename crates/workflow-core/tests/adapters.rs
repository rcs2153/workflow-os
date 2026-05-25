#![allow(clippy::expect_used)]
//! Adapter contract tests using mock-only adapters.

use std::collections::BTreeMap;

use workflow_core::{
    ActorId, AdapterAction, AdapterCapability, AdapterCapabilityDiscovery, AdapterError,
    AdapterErrorKind, AdapterHealth, AdapterHealthCheck, AdapterId, AdapterIdempotencyStrategy,
    AdapterInvocationRecord, AdapterKind, AdapterObservabilityRecord, AdapterOperationMode,
    AdapterPolicyPrecheck, AdapterReadOperation, AdapterRedactionPolicy, AdapterRedactionStrategy,
    AdapterRequest, AdapterResponse, AdapterResponseStatus, AdapterRunScope, AdapterTimeoutPolicy,
    AdapterWriteOperation, CorrelationId, IdempotencyKey, SchemaVersion, SpecContentHash,
    Timestamp, WorkflowId, WorkflowRunId, WorkflowVersion,
};

struct MockAdapter;

impl AdapterReadOperation for MockAdapter {
    fn read(&self, request: &AdapterRequest) -> Result<AdapterResponse, AdapterError> {
        request.validate_preconditions()?;
        Ok(AdapterResponse::redacted_summary(
            request.adapter_id.clone(),
            request.action.name.clone(),
            request.correlation_id.clone(),
            "read summary",
            vec!["external/ref/1".to_owned()],
            17,
        ))
    }
}

impl AdapterWriteOperation for MockAdapter {
    fn write(&self, request: &AdapterRequest) -> Result<AdapterResponse, AdapterError> {
        request.validate_preconditions()?;
        Ok(AdapterResponse::redacted_summary(
            request.adapter_id.clone(),
            request.action.name.clone(),
            request.correlation_id.clone(),
            "write summary",
            vec!["external/ref/2".to_owned()],
            19,
        ))
    }
}

impl AdapterHealthCheck for MockAdapter {
    fn health(&self) -> Result<AdapterHealth, AdapterError> {
        Ok(AdapterHealth {
            adapter_id: AdapterId::new("adapter/mock").expect("adapter id"),
            adapter_kind: AdapterKind::Local,
            operation_mode: AdapterOperationMode::Mock,
            configured: true,
            reachable: Some(true),
            credential_present: false,
            last_checked_at: Timestamp::now_utc(),
            warnings: vec!["mock health uses no credential".to_owned()],
        })
    }
}

impl AdapterCapabilityDiscovery for MockAdapter {
    fn capabilities(&self) -> Vec<AdapterCapability> {
        vec![
            AdapterCapability::ExternalRead,
            AdapterCapability::GitHubRead,
            AdapterCapability::ExternalWrite,
        ]
    }
}

fn read_request() -> AdapterRequest {
    let mut metadata = BTreeMap::new();
    metadata.insert("fixture".to_owned(), "adapter-contract".to_owned());
    AdapterRequest {
        adapter_id: AdapterId::new("adapter/mock").expect("adapter id"),
        adapter_kind: AdapterKind::Local,
        action: AdapterAction {
            name: "read-record".to_owned(),
            side_effecting: false,
            required_capabilities: vec![AdapterCapability::GitHubRead],
        },
        capability: AdapterCapability::GitHubRead,
        operation_mode: AdapterOperationMode::Mock,
        correlation_id: CorrelationId::new("correlation/adapter-test").expect("correlation"),
        actor: ActorId::new("system/adapter-test").expect("actor"),
        run_scope: Some(AdapterRunScope {
            workflow_run_id: WorkflowRunId::new("run/adapter-test").expect("run"),
            workflow_id: WorkflowId::new("workflow/adapter-test").expect("workflow"),
            workflow_version: WorkflowVersion::new("v1").expect("version"),
            schema_version: SchemaVersion::new("v0").expect("schema"),
            spec_hash: SpecContentHash::from_text("adapter-test"),
        }),
        idempotency_key: None,
        input_reference: Some("input/ref".to_owned()),
        idempotency_strategy: AdapterIdempotencyStrategy::NotRequiredForReadOnly,
        redaction_policy: AdapterRedactionPolicy {
            strategy: AdapterRedactionStrategy::ReferenceOnly,
            sensitive_fields: vec!["payload.token".to_owned()],
        },
        timeout_policy: AdapterTimeoutPolicy { timeout_ms: 5_000 },
        metadata,
        policy_precheck: AdapterPolicyPrecheck::runtime_allowed(vec![
            "policy.allow.adapter_read".to_owned()
        ]),
    }
}

fn write_request() -> AdapterRequest {
    let mut request = read_request();
    request.action = AdapterAction {
        name: "write-record".to_owned(),
        side_effecting: true,
        required_capabilities: vec![AdapterCapability::GitHubWrite],
    };
    request.capability = AdapterCapability::GitHubWrite;
    request.idempotency_key = Some(IdempotencyKey::new("idempotency/adapter-write").expect("key"));
    request.idempotency_strategy = AdapterIdempotencyStrategy::RuntimeKey;
    request
}

#[test]
fn read_only_adapter_request_includes_required_metadata() {
    let request = read_request();

    assert_eq!(request.adapter_id.to_string(), "adapter/mock");
    assert_eq!(request.adapter_kind, AdapterKind::Local);
    assert_eq!(request.capability, AdapterCapability::GitHubRead);
    assert_eq!(request.operation_mode, AdapterOperationMode::Mock);
    assert_eq!(request.actor.to_string(), "system/adapter-test");
    assert!(request.run_scope.is_some());
    assert_eq!(
        request.policy_precheck,
        AdapterPolicyPrecheck::runtime_allowed(vec!["policy.allow.adapter_read".to_owned()])
    );
    assert_eq!(request.timeout_policy.timeout_ms, 5_000);
    assert_eq!(
        request.metadata.get("fixture").map(String::as_str),
        Some("adapter-contract")
    );
}

#[test]
fn fixture_mock_live_read_only_modes_are_represented_clearly() {
    let modes = [
        AdapterOperationMode::Fixture,
        AdapterOperationMode::Mock,
        AdapterOperationMode::Local,
        AdapterOperationMode::LiveReadOnly,
        AdapterOperationMode::LiveWriteCapable,
    ];

    assert_eq!(modes.len(), 5);
    assert_eq!(
        serde_json::to_string(&AdapterOperationMode::LiveReadOnly).expect("serialize"),
        "\"live-read-only\""
    );
}

#[test]
fn adapter_action_constructors_preserve_generic_capabilities() {
    let read = AdapterAction::read("generic-read");
    let write = AdapterAction::write("generic-write");

    assert_eq!(
        read.required_capabilities,
        vec![AdapterCapability::ExternalRead]
    );
    assert_eq!(
        write.required_capabilities,
        vec![AdapterCapability::ExternalWrite]
    );
}

#[test]
fn adapter_contract_mock_read() {
    let adapter = MockAdapter;
    let response = adapter.read(&read_request()).expect("mock read succeeds");

    assert_eq!(response.adapter_id.to_string(), "adapter/mock");
    assert_eq!(response.action, "read-record");
    assert_eq!(response.status, AdapterResponseStatus::Success);
    assert_eq!(response.summary, "read summary");
    assert_eq!(response.external_references, vec!["external/ref/1"]);
    assert_eq!(response.duration_ms, 17);
}

#[test]
fn missing_capability_fails_closed() {
    let adapter = MockAdapter;
    let mut request = read_request();
    request.action.required_capabilities = Vec::new();

    let error = adapter.read(&request).expect_err("capability required");

    assert_eq!(error.code, "adapter.capability.required");
    assert_eq!(error.kind, AdapterErrorKind::PermissionFailure);
}

#[test]
fn read_action_without_policy_fails_closed() {
    let adapter = MockAdapter;
    let mut request = read_request();
    request.policy_precheck = AdapterPolicyPrecheck::Missing;

    let error = adapter.read(&request).expect_err("policy required");

    assert_eq!(error.code, "adapter.policy.required");
    assert_eq!(error.kind, AdapterErrorKind::PolicyDenied);
}

#[test]
fn write_operation_denied_in_phase_2() {
    let adapter = MockAdapter;
    let request = write_request();

    let error = adapter
        .write(&request)
        .expect_err("writes denied in phase 2");

    assert_eq!(error.code, "adapter.phase2.write_denied");
    assert_eq!(error.kind, AdapterErrorKind::PolicyDenied);
}

#[test]
fn live_write_capable_mode_denied_in_phase_2() {
    let mut request = read_request();
    request.operation_mode = AdapterOperationMode::LiveWriteCapable;

    let error = request
        .validate_preconditions()
        .expect_err("live write capable mode is denied");

    assert_eq!(error.code, "adapter.phase2.write_mode_denied");
    assert_eq!(error.kind, AdapterErrorKind::PolicyDenied);
}

#[test]
fn idempotency_strategy_mismatch_fails_closed() {
    let mut request = read_request();
    request.idempotency_strategy = AdapterIdempotencyStrategy::RuntimeKey;
    request.idempotency_key = None;

    let error = request
        .validate_preconditions()
        .expect_err("idempotency strategy requires key");

    assert_eq!(error.code, "adapter.idempotency.strategy_mismatch");
}

#[test]
fn adapter_error_classifications_serialize_deserialize() {
    let errors = [
        AdapterErrorKind::AuthFailure,
        AdapterErrorKind::PermissionFailure,
        AdapterErrorKind::NotFound,
        AdapterErrorKind::RateLimited,
        AdapterErrorKind::Timeout,
        AdapterErrorKind::ValidationFailure,
        AdapterErrorKind::MalformedResponse,
        AdapterErrorKind::TransientNetworkFailure,
        AdapterErrorKind::UnsupportedOperation,
        AdapterErrorKind::PolicyDenied,
        AdapterErrorKind::Unknown,
    ];

    for kind in errors {
        let encoded = serde_json::to_string(&kind).expect("serialize error kind");
        let decoded: AdapterErrorKind =
            serde_json::from_str(&encoded).expect("deserialize error kind");
        assert_eq!(decoded, kind);
    }
}

#[test]
fn adapter_response_redaction_metadata_works() {
    let response = AdapterResponse::redacted_summary(
        AdapterId::new("adapter/mock").expect("adapter id"),
        "read-record",
        CorrelationId::new("correlation/adapter-test").expect("correlation"),
        "token value",
        Vec::new(),
        3,
    );

    assert_eq!(response.summary, "[REDACTED]");
    assert_eq!(response.redaction.redacted_fields, vec!["summary"]);
    assert!(response
        .redaction
        .field_states
        .iter()
        .any(|field| field.field == "summary"));
}

#[test]
fn health_check_contract_does_not_expose_secrets() {
    let adapter = MockAdapter;
    let health = adapter.health().expect("health succeeds");
    let debug = format!("{health:?}");

    assert!(health.configured);
    assert_eq!(health.reachable, Some(true));
    assert!(!health.credential_present);
    assert_eq!(health.adapter_kind, AdapterKind::Local);
    assert!(!debug.contains("secret-value"));
    assert!(!debug.contains("token-value"));
    assert!(adapter
        .capabilities()
        .contains(&AdapterCapability::GitHubRead));
}

#[test]
fn adapter_invocation_emits_audit_and_observability_records() {
    let adapter = MockAdapter;
    let request = read_request();
    let response = adapter.read(&request).expect("read succeeds");
    let audit_record =
        AdapterInvocationRecord::from_response(&request, &response, Timestamp::now_utc());
    let observability = AdapterObservabilityRecord::from_invocation(&audit_record);

    assert_eq!(audit_record.adapter_id, request.adapter_id);
    assert_eq!(audit_record.status, AdapterResponseStatus::Success);
    assert_eq!(audit_record.capability, AdapterCapability::GitHubRead);
    assert_eq!(audit_record.actor.to_string(), "system/adapter-test");
    assert_eq!(
        audit_record.output_reference.as_deref(),
        Some("external/ref/1")
    );
    assert_eq!(observability.status, AdapterResponseStatus::Success);
    assert_eq!(observability.duration_ms, 17);
    assert_eq!(
        observability
            .attributes
            .get("workflow_id")
            .map(String::as_str),
        Some("workflow/adapter-test")
    );
}

#[test]
fn adapter_failure_emits_classified_audit_and_observability_records() {
    let request = read_request();
    let error = AdapterError::new(
        AdapterErrorKind::Timeout,
        "adapter.timeout",
        "adapter read timed out",
    );
    let audit_record =
        AdapterInvocationRecord::from_error(&request, &error, 5_000, Timestamp::now_utc());
    let observability = AdapterObservabilityRecord::from_invocation(&audit_record);

    assert_eq!(audit_record.status, AdapterResponseStatus::Failure);
    assert_eq!(audit_record.error_kind, Some(AdapterErrorKind::Timeout));
    assert_eq!(observability.error_kind, Some(AdapterErrorKind::Timeout));
}

#[test]
fn write_request_keeps_idempotency_metadata_even_though_denied() {
    let request = write_request();

    assert!(request.idempotency_key.is_some());
    assert_eq!(
        request.idempotency_strategy,
        AdapterIdempotencyStrategy::RuntimeKey
    );
}
