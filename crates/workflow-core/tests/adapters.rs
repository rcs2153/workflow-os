#![allow(clippy::expect_used)]
//! Adapter contract tests using mock-only adapters.

use workflow_core::{
    AdapterAction, AdapterCapability, AdapterCapabilityDiscovery, AdapterError, AdapterErrorKind,
    AdapterHealth, AdapterHealthCheck, AdapterId, AdapterIdempotencyStrategy, AdapterKind,
    AdapterPolicyPrecheck, AdapterReadOperation, AdapterRedactionStrategy, AdapterRequest,
    AdapterResponse, AdapterWriteOperation, CorrelationId, IdempotencyKey,
};

struct MockAdapter;

impl AdapterReadOperation for MockAdapter {
    fn read(&self, request: &AdapterRequest) -> Result<AdapterResponse, AdapterError> {
        request.validate_preconditions()?;
        Ok(AdapterResponse::redacted_summary(
            "read summary",
            Some("external/ref/1".to_owned()),
        ))
    }
}

impl AdapterWriteOperation for MockAdapter {
    fn write(&self, request: &AdapterRequest) -> Result<AdapterResponse, AdapterError> {
        request.validate_preconditions()?;
        Ok(AdapterResponse::redacted_summary(
            "write summary",
            Some("external/ref/2".to_owned()),
        ))
    }
}

impl AdapterHealthCheck for MockAdapter {
    fn health(&self) -> Result<AdapterHealth, AdapterError> {
        Ok(AdapterHealth {
            adapter_id: AdapterId::new("adapter/mock").expect("adapter id"),
            adapter_kind: AdapterKind::Local,
            healthy: true,
            message: "mock healthy".to_owned(),
        })
    }
}

impl AdapterCapabilityDiscovery for MockAdapter {
    fn capabilities(&self) -> Vec<AdapterCapability> {
        vec![
            AdapterCapability::ExternalRead,
            AdapterCapability::ExternalWrite,
        ]
    }
}

fn read_request() -> AdapterRequest {
    AdapterRequest {
        adapter_id: AdapterId::new("adapter/mock").expect("adapter id"),
        adapter_kind: AdapterKind::Local,
        action: AdapterAction::read("read-record"),
        correlation_id: CorrelationId::new("correlation/adapter-test").expect("correlation"),
        idempotency_key: None,
        input_reference: Some("input/ref".to_owned()),
        idempotency_strategy: AdapterIdempotencyStrategy::NotRequiredForReadOnly,
        redaction_strategy: AdapterRedactionStrategy::ReferenceOnly,
        policy_precheck: AdapterPolicyPrecheck::Missing,
    }
}

fn write_request() -> AdapterRequest {
    AdapterRequest {
        adapter_id: AdapterId::new("adapter/mock").expect("adapter id"),
        adapter_kind: AdapterKind::Local,
        action: AdapterAction::write("write-record"),
        correlation_id: CorrelationId::new("correlation/adapter-test").expect("correlation"),
        idempotency_key: Some(IdempotencyKey::new("idempotency/adapter-write").expect("key")),
        input_reference: Some("input/ref".to_owned()),
        idempotency_strategy: AdapterIdempotencyStrategy::RuntimeKey,
        redaction_strategy: AdapterRedactionStrategy::ReferenceOnly,
        policy_precheck: AdapterPolicyPrecheck::Allowed {
            reason_codes: vec!["policy.allow.test".to_owned()],
        },
    }
}

#[test]
fn adapter_contract_mock_read() {
    let adapter = MockAdapter;
    let response = adapter.read(&read_request()).expect("mock read succeeds");

    assert_eq!(response.summary, "read summary");
    assert_eq!(
        response.external_reference.as_deref(),
        Some("external/ref/1")
    );
}

#[test]
fn adapter_contract_mock_write_denied_without_capability() {
    let adapter = MockAdapter;
    let mut request = write_request();
    request.action.required_capabilities = Vec::new();

    let error = adapter.write(&request).expect_err("capability required");

    assert_eq!(error.code, "adapter.capability.external_write_missing");
    assert_eq!(error.kind, AdapterErrorKind::PermissionFailure);
}

#[test]
fn adapter_contract_mock_write_requires_policy() {
    let adapter = MockAdapter;
    let mut request = write_request();
    request.policy_precheck = AdapterPolicyPrecheck::Missing;

    let error = adapter.write(&request).expect_err("policy required");

    assert_eq!(error.code, "adapter.policy.required");
    assert_eq!(error.kind, AdapterErrorKind::PermissionFailure);
}

#[test]
fn adapter_error_classification() {
    let errors = [
        AdapterErrorKind::RateLimit,
        AdapterErrorKind::AuthFailure,
        AdapterErrorKind::PermissionFailure,
        AdapterErrorKind::NotFound,
        AdapterErrorKind::ValidationFailure,
        AdapterErrorKind::TransientFailure,
        AdapterErrorKind::UnknownFailure,
    ];

    assert_eq!(errors.len(), 7);
}

#[test]
fn idempotency_key_required_for_side_effecting_action() {
    let mut request = write_request();
    request.idempotency_key = None;

    let error = request
        .validate_preconditions()
        .expect_err("idempotency is required");

    assert_eq!(error.code, "adapter.idempotency.required");
}

#[test]
fn adapter_response_redacts_sensitive_summary() {
    let response = AdapterResponse::redacted_summary("token value", None);

    assert_eq!(response.summary, "[REDACTED]");
    assert_eq!(response.redaction.redacted_fields, vec!["summary"]);
}

#[test]
fn health_check_contract() {
    let adapter = MockAdapter;
    let health = adapter.health().expect("health succeeds");

    assert!(health.healthy);
    assert_eq!(health.adapter_kind, AdapterKind::Local);
    assert!(adapter
        .capabilities()
        .contains(&AdapterCapability::ExternalRead));
}
