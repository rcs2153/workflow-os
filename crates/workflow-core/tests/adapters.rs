#![allow(clippy::expect_used)]
//! Adapter contract tests using mock-only adapters.

use std::collections::BTreeMap;

use workflow_core::{
    ActorId, AdapterAction, AdapterCapability, AdapterCapabilityDiscovery, AdapterError,
    AdapterErrorKind, AdapterHealth, AdapterHealthCheck, AdapterId, AdapterIdempotencyStrategy,
    AdapterInvocationRecord, AdapterKind, AdapterObservabilityRecord, AdapterOperationMode,
    AdapterPolicyPrecheck, AdapterReadOperation, AdapterRedactionPolicy, AdapterRedactionStrategy,
    AdapterRequest, AdapterResponse, AdapterResponseStatus, AdapterRunScope, AdapterTimeoutPolicy,
    AdapterWriteOperation, CorrelationId, EvidenceKind, EvidenceMetadata,
    EvidenceRedactionMetadata, EvidenceReference, EvidenceReferenceId,
    EvidenceReferenceRequiredFields, EvidenceReferenceTarget, EvidenceScope, EvidenceSensitivity,
    EvidenceSourceComponent, IdempotencyKey, SchemaVersion, SpecContentHash, StepId, Timestamp,
    WorkflowId, WorkflowRunId, WorkflowVersion,
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

fn evidence_redaction() -> EvidenceRedactionMetadata {
    EvidenceRedactionMetadata::reference_only("target", "adapter telemetry stores references")
        .expect("redaction metadata")
}

fn adapter_evidence(kind: EvidenceKind, id: &str) -> EvidenceReference {
    EvidenceReference::new(EvidenceReferenceRequiredFields {
        id: EvidenceReferenceId::new(id).expect("evidence id"),
        kind,
        title: "Adapter evidence".to_owned(),
        target: EvidenceReferenceTarget::external("github", "repo/ref").expect("target"),
        source_component: EvidenceSourceComponent::Adapter,
        scope: EvidenceScope::Adapter,
        created_at: Timestamp::now_utc(),
        redaction_metadata: evidence_redaction(),
        sensitivity: Some(EvidenceSensitivity::Confidential),
    })
    .expect("evidence")
    .with_adapter(
        AdapterId::new("adapter/mock").expect("adapter id"),
        AdapterKind::Local,
    )
}

fn invalid_adapter_evidence() -> EvidenceReference {
    let mut evidence = adapter_evidence(EvidenceKind::AdapterInvocation, "evidence/invalid");
    evidence.adapter_id = None;
    evidence
}

fn runtime_audit_record() -> workflow_core::AdapterRuntimeAuditRecord {
    let adapter = MockAdapter;
    let request = read_request();
    let response = adapter.read(&request).expect("read succeeds");
    let invocation =
        AdapterInvocationRecord::from_response(&request, &response, Timestamp::now_utc());
    workflow_core::AdapterRuntimeAuditRecord::from_invocation(
        &invocation,
        Some(StepId::new("step/adapter-test").expect("step")),
        None,
        None,
        "adapter-test",
    )
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
fn adapter_invocation_record_attaches_one_valid_evidence_reference() {
    let adapter = MockAdapter;
    let request = read_request();
    let response = adapter.read(&request).expect("read succeeds");
    let mut audit_record =
        AdapterInvocationRecord::from_response(&request, &response, Timestamp::now_utc());
    let mut evidence = adapter_evidence(EvidenceKind::AdapterInvocation, "evidence/invocation");
    evidence.correlation_id = Some(request.correlation_id.clone());

    audit_record
        .attach_evidence_reference(&evidence)
        .expect("valid evidence attaches");

    assert_eq!(audit_record.evidence_references().len(), 1);
    assert_eq!(
        audit_record.evidence_references()[0].kind,
        EvidenceKind::AdapterInvocation
    );
    assert_eq!(
        audit_record.evidence_references()[0].adapter_id,
        Some(AdapterId::new("adapter/mock").expect("adapter id"))
    );
    assert_eq!(
        audit_record.evidence_references()[0].adapter_kind,
        Some(AdapterKind::Local)
    );
    assert_eq!(
        audit_record.evidence_references()[0].correlation_id,
        Some(request.correlation_id)
    );
}

#[test]
fn adapter_invocation_record_attaches_multiple_evidence_references_atomically() {
    let adapter = MockAdapter;
    let request = read_request();
    let response = adapter.read(&request).expect("read succeeds");
    let audit_record =
        AdapterInvocationRecord::from_response(&request, &response, Timestamp::now_utc());

    let audit_record = audit_record
        .with_evidence_references(vec![
            adapter_evidence(EvidenceKind::AdapterInvocation, "evidence/invocation"),
            adapter_evidence(
                EvidenceKind::AdapterResponseSummary,
                "evidence/response-summary",
            ),
        ])
        .expect("valid evidence attaches");

    assert_eq!(audit_record.evidence_references().len(), 2);
    assert_eq!(
        audit_record.evidence_references()[0].kind,
        EvidenceKind::AdapterInvocation
    );
    assert_eq!(
        audit_record.evidence_references()[1].kind,
        EvidenceKind::AdapterResponseSummary
    );
}

#[test]
fn adapter_invocation_record_rejects_invalid_evidence_reference() {
    let adapter = MockAdapter;
    let request = read_request();
    let response = adapter.read(&request).expect("read succeeds");
    let mut audit_record =
        AdapterInvocationRecord::from_response(&request, &response, Timestamp::now_utc());

    let evidence = invalid_adapter_evidence();
    let error = audit_record
        .attach_evidence_reference(&evidence)
        .expect_err("invalid evidence rejected");

    assert_eq!(error.code(), "evidence.scope.adapter_id_required");
    assert!(audit_record.evidence_references().is_empty());
}

#[test]
fn adapter_invocation_multiple_attachment_fails_atomically_when_one_reference_is_invalid() {
    let adapter = MockAdapter;
    let request = read_request();
    let response = adapter.read(&request).expect("read succeeds");
    let mut audit_record =
        AdapterInvocationRecord::from_response(&request, &response, Timestamp::now_utc());

    let error = audit_record
        .attach_evidence_references(vec![
            adapter_evidence(EvidenceKind::AdapterInvocation, "evidence/valid"),
            invalid_adapter_evidence(),
        ])
        .expect_err("invalid evidence rejects whole batch");

    assert_eq!(error.code(), "evidence.scope.adapter_id_required");
    assert!(audit_record.evidence_references().is_empty());
}

#[test]
fn adapter_invocation_rejects_non_adapter_evidence_kind() {
    let adapter = MockAdapter;
    let request = read_request();
    let response = adapter.read(&request).expect("read succeeds");
    let mut audit_record =
        AdapterInvocationRecord::from_response(&request, &response, Timestamp::now_utc());
    let evidence = adapter_evidence(EvidenceKind::ValidationResult, "evidence/wrong-kind")
        .with_validation_result_id(
            workflow_core::ValidationReferenceId::new("validation/result").expect("validation"),
        );

    let error = audit_record
        .attach_evidence_reference(&evidence)
        .expect_err("wrong evidence kind rejected");

    assert_eq!(error.code(), "adapter.evidence.kind_unsupported");
    assert!(audit_record.evidence_references().is_empty());
}

#[test]
fn adapter_runtime_audit_record_attaches_valid_evidence_reference() {
    let mut record = runtime_audit_record();
    let evidence = adapter_evidence(
        EvidenceKind::AdapterResponseSummary,
        "evidence/runtime-response-summary",
    );

    record
        .attach_evidence_reference(&evidence)
        .expect("valid evidence attaches");

    assert_eq!(record.evidence_references().len(), 1);
    assert_eq!(record.adapter_id.to_string(), "adapter/mock");
    assert_eq!(
        record.evidence_references()[0].kind,
        EvidenceKind::AdapterResponseSummary
    );
}

#[test]
fn adapter_runtime_audit_record_rejects_invalid_evidence_reference() {
    let mut record = runtime_audit_record();

    let evidence = invalid_adapter_evidence();
    let error = record
        .attach_evidence_reference(&evidence)
        .expect_err("invalid evidence rejected");

    assert_eq!(error.code(), "evidence.scope.adapter_id_required");
    assert!(record.evidence_references().is_empty());
}

#[test]
fn public_field_mutation_after_prior_validation_cannot_bypass_attachment_validation() {
    let mut evidence = adapter_evidence(EvidenceKind::AdapterInvocation, "evidence/mutated");
    evidence.validate().expect("initial evidence is valid");
    evidence.adapter_kind = None;

    let mut record = runtime_audit_record();
    let error = record
        .attach_evidence_reference(&evidence)
        .expect_err("mutated evidence is revalidated");

    assert_eq!(error.code(), "evidence.scope.adapter_kind_required");
    assert!(record.evidence_references().is_empty());
}

#[test]
fn attached_adapter_evidence_debug_display_and_serialization_do_not_leak_secret_like_values() {
    let mut metadata = BTreeMap::new();
    metadata.insert(
        "provider".to_owned(),
        "Authorization: Bearer token-value raw provider payload".to_owned(),
    );
    let mut evidence = adapter_evidence(EvidenceKind::AdapterInvocation, "evidence/secret-safe");
    evidence.title = "github_pat_title-secret".to_owned();
    evidence.target = EvidenceReferenceTarget::external(
        "github",
        "raw CI log: Authorization: Bearer target-secret",
    )
    .expect("target");
    evidence
        .set_summary("Jira description: private body token=summary-secret")
        .expect("summary");
    evidence.set_metadata(EvidenceMetadata::new(metadata).expect("metadata"));

    let mut record = runtime_audit_record();
    record
        .attach_evidence_reference(&evidence)
        .expect("secret-like values are sanitized before attach");

    let debug = format!("{record:?}");
    let json = serde_json::to_string(&record).expect("serialize");
    for output in [debug, json] {
        assert!(!output.contains("title-secret"));
        assert!(!output.contains("target-secret"));
        assert!(!output.contains("summary-secret"));
        assert!(!output.contains("token-value"));
        assert!(!output.contains("Authorization: Bearer"));
        assert!(!output.contains("raw provider payload"));
        assert!(output.contains("[REDACTED]"));
    }
}

#[test]
fn adapter_runtime_audit_mapping_preserves_invocation_evidence() {
    let adapter = MockAdapter;
    let request = read_request();
    let response = adapter.read(&request).expect("read succeeds");
    let invocation =
        AdapterInvocationRecord::from_response(&request, &response, Timestamp::now_utc())
            .with_evidence_references(vec![adapter_evidence(
                EvidenceKind::AdapterInvocation,
                "evidence/mapped",
            )])
            .expect("valid evidence attaches");

    let runtime_record = workflow_core::AdapterRuntimeAuditRecord::from_invocation(
        &invocation,
        Some(StepId::new("step/adapter-test").expect("step")),
        None,
        None,
        "adapter-test",
    );

    assert_eq!(runtime_record.evidence_references().len(), 1);
    assert_eq!(
        runtime_record.evidence_references()[0].kind,
        EvidenceKind::AdapterInvocation
    );
}

#[test]
fn adapter_invocation_evidence_accessor_is_read_only_and_serializes() {
    let adapter = MockAdapter;
    let request = read_request();
    let response = adapter.read(&request).expect("read succeeds");
    let record = AdapterInvocationRecord::from_response(&request, &response, Timestamp::now_utc())
        .with_evidence_references(vec![adapter_evidence(
            EvidenceKind::AdapterInvocation,
            "evidence/serde-invocation",
        )])
        .expect("valid evidence attaches");

    // Compile-time privacy enforces that callers can only read attached
    // evidence through this slice and cannot push directly into the record.
    let evidence = record.evidence_references();
    assert_eq!(evidence.len(), 1);

    let json = serde_json::to_string(&record).expect("serialize");
    assert!(json.contains("\"evidence_references\""));

    let decoded: AdapterInvocationRecord =
        serde_json::from_str(&json).expect("valid evidence-bearing record deserializes");
    assert_eq!(decoded.evidence_references().len(), 1);
    assert_eq!(
        decoded.evidence_references()[0].kind,
        EvidenceKind::AdapterInvocation
    );
}

#[test]
fn invalid_adapter_invocation_evidence_payload_fails_deserialization_without_leaking_values() {
    let adapter = MockAdapter;
    let request = read_request();
    let response = adapter.read(&request).expect("read succeeds");
    let record = AdapterInvocationRecord::from_response(&request, &response, Timestamp::now_utc())
        .with_evidence_references(vec![adapter_evidence(
            EvidenceKind::AdapterInvocation,
            "evidence/invalid-serde-invocation",
        )])
        .expect("valid evidence attaches");
    let mut json = serde_json::to_value(&record).expect("serialize value");
    json["evidence_references"][0]["adapter_id"] = serde_json::Value::Null;
    json["evidence_references"][0]["title"] =
        serde_json::Value::String("Authorization: Bearer title-secret".to_owned());

    let error = serde_json::from_value::<AdapterInvocationRecord>(json)
        .expect_err("invalid evidence-bearing record is rejected");
    let error_text = error.to_string();

    assert!(error_text.contains("evidence.scope.adapter_id_required"));
    assert!(!error_text.contains("title-secret"));
    assert!(!error_text.contains("Authorization: Bearer"));
}

#[test]
fn adapter_runtime_audit_evidence_accessor_is_read_only_and_serializes() {
    let record = runtime_audit_record()
        .with_evidence_references(vec![adapter_evidence(
            EvidenceKind::AdapterResponseSummary,
            "evidence/serde-runtime-audit",
        )])
        .expect("valid evidence attaches");

    // Compile-time privacy enforces that callers can only read attached
    // evidence through this slice and cannot push directly into the record.
    let evidence = record.evidence_references();
    assert_eq!(evidence.len(), 1);

    let json = serde_json::to_string(&record).expect("serialize");
    assert!(json.contains("\"evidence_references\""));

    let decoded: workflow_core::AdapterRuntimeAuditRecord =
        serde_json::from_str(&json).expect("valid runtime audit evidence deserializes");
    assert_eq!(decoded.evidence_references().len(), 1);
    assert_eq!(
        decoded.evidence_references()[0].kind,
        EvidenceKind::AdapterResponseSummary
    );
}

#[test]
fn invalid_adapter_runtime_audit_evidence_payload_fails_deserialization_without_leaking_values() {
    let record = runtime_audit_record()
        .with_evidence_references(vec![adapter_evidence(
            EvidenceKind::AdapterResponseSummary,
            "evidence/invalid-serde-runtime-audit",
        )])
        .expect("valid evidence attaches");
    let mut json = serde_json::to_value(&record).expect("serialize value");
    json["evidence_references"][0]["adapter_kind"] = serde_json::Value::Null;
    json["evidence_references"][0]["target"] = serde_json::json!({
        "kind": "external",
        "system": "github",
        "reference": "raw provider payload Authorization: Bearer target-secret"
    });

    let error = serde_json::from_value::<workflow_core::AdapterRuntimeAuditRecord>(json)
        .expect_err("invalid runtime audit evidence is rejected");
    let error_text = error.to_string();

    assert!(error_text.contains("evidence.scope.adapter_kind_required"));
    assert!(!error_text.contains("target-secret"));
    assert!(!error_text.contains("Authorization: Bearer"));
    assert!(!error_text.contains("raw provider payload"));
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
