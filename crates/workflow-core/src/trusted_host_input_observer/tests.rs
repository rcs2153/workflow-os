// Tests are kept beside the private model to verify authority and privacy boundaries.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use super::*;

fn hash(label: &str) -> SpecContentHash {
    SpecContentHash::from_text(label)
}

fn telemetry(label: &str) -> HostedExecutionReference {
    HostedExecutionReference::new(HostedExecutionReferenceKind::Telemetry, label).unwrap()
}

fn producer_id() -> TrustedHostInputObservationProducerId {
    TrustedHostInputObservationProducerId::new("observer/fixture").unwrap()
}

fn producer_version() -> TrustedHostInputObservationProducerVersion {
    TrustedHostInputObservationProducerVersion::new("v2").unwrap()
}

fn request() -> TrustedHostInputObservationRequest {
    TrustedHostInputObservationRequest::new(TrustedHostInputObservationRequestDefinition {
        version: TrustedHostInputObservationContractVersion::CargoHostInputsV2,
        run_bundle: hash("run"),
        operation_contract: hash("operation"),
        static_request: hash("static-request"),
        static_result: hash("static-result"),
        static_discovery_completed_revision: 10,
        repository_tree: hash("repository"),
        security_requirements: hash("security"),
        selection: hash("selection"),
        attempt: hash("attempt"),
        expected_producer_id: producer_id(),
        expected_producer_version: producer_version(),
    })
    .unwrap()
}

fn fact(kind: TrustedHostBuildInputHostObligationKind) -> TrustedHostInputObservationFact {
    TrustedHostInputObservationFact::new(TrustedHostInputObservationFactDefinition {
        producer_id: producer_id(),
        producer_version: producer_version(),
        kind,
        posture: TrustedHostInputObservationPosture::Observed,
        reason: None,
        method: TrustedHostInputObservationMethod::Fixture,
        object_binding: TrustedHostObservationPosture::Proven,
        continuity: TrustedHostObservationPosture::Proven,
        commitment: hash(&format!("fact-{kind:?}")),
        item_count: 1,
        reference: Some(telemetry(&format!("telemetry/{kind:?}"))),
    })
    .unwrap()
}

fn all_facts() -> Vec<TrustedHostInputObservationFact> {
    use TrustedHostBuildInputHostObligationKind::*;
    [
        ToolchainExecutable,
        TargetSpecification,
        SdkOrSysroot,
        LinkerOrArchiver,
        NativeCompilerTools,
        DynamicLibraries,
        RegistryAndPackageSources,
        Environment,
        RuntimeBase,
    ]
    .into_iter()
    .map(fact)
    .collect()
}

fn root(commitment: SpecContentHash) -> TrustedHostRootObservation {
    TrustedHostRootObservation::new(crate::TrustedHostRootObservationDefinition {
        role: TrustedHostRootRole::RepositoryInput,
        access: TrustedHostRootAccess::ReadOnly,
        object_binding: TrustedHostObservationPosture::Proven,
        continuity: TrustedHostObservationPosture::Proven,
        method: TrustedHostRootObservationMethod::ImmutableProjection,
        invocation_scoped: false,
        commitment,
        reference: telemetry("telemetry/root"),
    })
    .unwrap()
}

fn result_definition(
    request: &TrustedHostInputObservationRequest,
) -> TrustedHostInputObservationResultDefinition {
    TrustedHostInputObservationResultDefinition {
        request_fingerprint: request.fingerprint().clone(),
        repository_tree: request.repository_tree().clone(),
        producer: TrustedHostInputObservationProducer::new(
            TrustedHostInputObservationProducerDefinition {
                id: producer_id(),
                version: producer_version(),
                kind: TrustedHostInputObservationProducerKind::Fixture,
                assurance: TrustedHostInputObservationProducerAssurance::FixtureOnly,
                implementation: hash("implementation"),
                configuration: hash("configuration"),
            },
        )
        .unwrap(),
        started_revision: 11,
        completed_revision: 12,
        repository_root: root(request.repository_tree().clone()),
        facts: all_facts(),
    }
}

#[test]
fn complete_result_is_deterministic_and_matches_exact_request() {
    let request = request();
    let first = TrustedHostInputObservationResult::new(result_definition(&request)).unwrap();
    let second = TrustedHostInputObservationResult::new(result_definition(&request)).unwrap();
    assert_eq!(first.fingerprint(), second.fingerprint());
    assert_eq!(first.facts().len(), 9);
    assert_eq!(
        request.fingerprint().as_str(),
        "d818624dacb8ea7e0a75e97c8ce6d0f1ab5867292011cf6667eb3a682ec71e04"
    );
    assert_eq!(
        first.fingerprint().as_str(),
        "3e507baeb1e8ba792bcebe0e0ea14744c6c0494a6dd0da5c6e9801c5f095149a"
    );
    first.validate_for_request(&request).unwrap();
}

#[test]
fn fact_order_does_not_change_the_canonical_result() {
    let request = request();
    let first = TrustedHostInputObservationResult::new(result_definition(&request)).unwrap();
    let mut reversed = result_definition(&request);
    reversed.facts.reverse();
    let second = TrustedHostInputObservationResult::new(reversed).unwrap();
    assert_eq!(first.fingerprint(), second.fingerprint());
    assert_eq!(first.facts(), second.facts());
}

#[test]
fn every_semantic_binding_changes_the_result_commitment() {
    let request = request();
    let baseline = TrustedHostInputObservationResult::new(result_definition(&request)).unwrap();

    let mut producer_changed = result_definition(&request);
    producer_changed.producer =
        TrustedHostInputObservationProducer::new(TrustedHostInputObservationProducerDefinition {
            id: producer_id(),
            version: producer_version(),
            kind: TrustedHostInputObservationProducerKind::Fixture,
            assurance: TrustedHostInputObservationProducerAssurance::FixtureOnly,
            implementation: hash("different-implementation"),
            configuration: hash("configuration"),
        })
        .unwrap();
    let producer_changed = TrustedHostInputObservationResult::new(producer_changed).unwrap();
    assert_ne!(baseline.fingerprint(), producer_changed.fingerprint());

    let mut fact_changed = result_definition(&request);
    fact_changed.facts[0] =
        TrustedHostInputObservationFact::new(TrustedHostInputObservationFactDefinition {
            producer_id: producer_id(),
            producer_version: producer_version(),
            kind: fact_changed.facts[0].kind(),
            posture: TrustedHostInputObservationPosture::Uncertain,
            reason: Some(TrustedHostInputObservationReason::IntegrityOrContinuityUncertain),
            method: TrustedHostInputObservationMethod::Fixture,
            object_binding: TrustedHostObservationPosture::Uncertain,
            continuity: TrustedHostObservationPosture::Uncertain,
            commitment: hash("uncertain-fact"),
            item_count: 0,
            reference: None,
        })
        .unwrap();
    let fact_changed = TrustedHostInputObservationResult::new(fact_changed).unwrap();
    assert_ne!(baseline.fingerprint(), fact_changed.fingerprint());

    let mut root_changed = result_definition(&request);
    root_changed.repository_root =
        TrustedHostRootObservation::new(crate::TrustedHostRootObservationDefinition {
            role: TrustedHostRootRole::RepositoryInput,
            access: TrustedHostRootAccess::ReadOnly,
            object_binding: TrustedHostObservationPosture::Proven,
            continuity: TrustedHostObservationPosture::Proven,
            method: TrustedHostRootObservationMethod::ProtectedHandle,
            invocation_scoped: false,
            commitment: request.repository_tree().clone(),
            reference: telemetry("telemetry/root-other"),
        })
        .unwrap();
    let root_changed = TrustedHostInputObservationResult::new(root_changed).unwrap();
    assert_ne!(baseline.fingerprint(), root_changed.fingerprint());
}

#[test]
fn missing_duplicate_and_substituted_bindings_fail_closed() {
    let request = request();
    let mut missing = result_definition(&request);
    missing.facts.pop();
    assert!(TrustedHostInputObservationResult::new(missing).is_err());
    let mut duplicate = result_definition(&request);
    duplicate.facts[8] = duplicate.facts[0].clone();
    assert!(TrustedHostInputObservationResult::new(duplicate).is_err());
    let mut substituted = result_definition(&request);
    substituted.repository_tree = hash("other");
    assert!(TrustedHostInputObservationResult::new(substituted).is_err());
}

#[test]
fn declared_only_cannot_claim_observed_binding() {
    let error = TrustedHostInputObservationFact::new(TrustedHostInputObservationFactDefinition {
        producer_id: producer_id(),
        producer_version: producer_version(),
        kind: TrustedHostBuildInputHostObligationKind::Environment,
        posture: TrustedHostInputObservationPosture::DeclaredOnly,
        reason: Some(TrustedHostInputObservationReason::NotIndependentlyObserved),
        method: TrustedHostInputObservationMethod::DeclaredConfiguration,
        object_binding: TrustedHostObservationPosture::Proven,
        continuity: TrustedHostObservationPosture::Proven,
        commitment: hash("environment"),
        item_count: 1,
        reference: None,
    })
    .unwrap_err();
    assert_eq!(
        error.code(),
        "trusted_host_input_observer.fact.posture.invalid"
    );
}

#[test]
fn incomplete_postures_are_explicit_and_non_authorizing() {
    for (posture, reason, method, binding) in [
        (
            TrustedHostInputObservationPosture::DeclaredOnly,
            TrustedHostInputObservationReason::NotIndependentlyObserved,
            TrustedHostInputObservationMethod::DeclaredConfiguration,
            TrustedHostObservationPosture::Uncertain,
        ),
        (
            TrustedHostInputObservationPosture::Unavailable,
            TrustedHostInputObservationReason::ObservationUnavailable,
            TrustedHostInputObservationMethod::Unavailable,
            TrustedHostObservationPosture::Unavailable,
        ),
        (
            TrustedHostInputObservationPosture::Unsupported,
            TrustedHostInputObservationReason::ObservationUnsupported,
            TrustedHostInputObservationMethod::Unsupported,
            TrustedHostObservationPosture::Unsupported,
        ),
        (
            TrustedHostInputObservationPosture::Uncertain,
            TrustedHostInputObservationReason::IntegrityOrContinuityUncertain,
            TrustedHostInputObservationMethod::Fixture,
            TrustedHostObservationPosture::Uncertain,
        ),
    ] {
        let fact =
            TrustedHostInputObservationFact::new(TrustedHostInputObservationFactDefinition {
                producer_id: producer_id(),
                producer_version: producer_version(),
                kind: TrustedHostBuildInputHostObligationKind::Environment,
                posture,
                reason: Some(reason),
                method,
                object_binding: binding,
                continuity: binding,
                commitment: hash("bounded-incomplete"),
                item_count: 0,
                reference: None,
            })
            .unwrap();
        assert_eq!(fact.posture(), posture);
    }
}

#[test]
fn invalid_producer_reference_and_repository_root_fail_closed() {
    let producer_error =
        TrustedHostInputObservationProducer::new(TrustedHostInputObservationProducerDefinition {
            id: producer_id(),
            version: producer_version(),
            kind: TrustedHostInputObservationProducerKind::Fixture,
            assurance: TrustedHostInputObservationProducerAssurance::CallerSupplied,
            implementation: hash("implementation"),
            configuration: hash("configuration"),
        })
        .unwrap_err();
    assert_eq!(
        producer_error.code(),
        "trusted_host_input_observer.producer.assurance.invalid"
    );

    let reference_error =
        TrustedHostInputObservationFact::new(TrustedHostInputObservationFactDefinition {
            producer_id: producer_id(),
            producer_version: producer_version(),
            kind: TrustedHostBuildInputHostObligationKind::Environment,
            posture: TrustedHostInputObservationPosture::Observed,
            reason: None,
            method: TrustedHostInputObservationMethod::Fixture,
            object_binding: TrustedHostObservationPosture::Proven,
            continuity: TrustedHostObservationPosture::Proven,
            commitment: hash("environment"),
            item_count: 1,
            reference: Some(
                HostedExecutionReference::new(HostedExecutionReferenceKind::Artifact, "artifact/1")
                    .unwrap(),
            ),
        })
        .unwrap_err();
    assert_eq!(
        reference_error.code(),
        "trusted_host_input_observer.fact.posture.invalid"
    );

    let request = request();
    let mut wrong_root = result_definition(&request);
    wrong_root.repository_root =
        TrustedHostRootObservation::new(crate::TrustedHostRootObservationDefinition {
            role: TrustedHostRootRole::ToolchainInput,
            access: TrustedHostRootAccess::ReadOnly,
            object_binding: TrustedHostObservationPosture::Proven,
            continuity: TrustedHostObservationPosture::Proven,
            method: TrustedHostRootObservationMethod::ImmutableProjection,
            invocation_scoped: false,
            commitment: request.repository_tree().clone(),
            reference: telemetry("telemetry/root"),
        })
        .unwrap();
    assert!(TrustedHostInputObservationResult::new(wrong_root).is_err());
}

#[test]
fn debug_redacts_commitments_and_serde_revalidates() {
    let request = request();
    let result = TrustedHostInputObservationResult::new(result_definition(&request)).unwrap();
    let debug = format!("{result:?}");
    assert!(!debug.contains(request.repository_tree().as_str()));
    let mut value = serde_json::to_value(result).unwrap();
    value["completed_revision"] = serde_json::json!(0);
    assert!(serde_json::from_value::<TrustedHostInputObservationResult>(value).is_err());
}

#[test]
fn strict_serde_rejects_unknown_fields_without_echoing_secret_values() {
    let request = request();
    let result = TrustedHostInputObservationResult::new(result_definition(&request)).unwrap();
    let mut value = serde_json::to_value(result).unwrap();
    value["repository_root"]["secret_field"] = serde_json::json!("TOKEN_DO_NOT_ECHO");
    let error = serde_json::from_value::<TrustedHostInputObservationResult>(value)
        .unwrap_err()
        .to_string();
    assert!(!error.contains("TOKEN_DO_NOT_ECHO"));

    let mut request_value = serde_json::to_value(request).unwrap();
    request_value["version"] = serde_json::json!("TOKEN_DO_NOT_ECHO");
    let error = serde_json::from_value::<TrustedHostInputObservationRequest>(request_value)
        .unwrap_err()
        .to_string();
    assert!(!error.contains("TOKEN_DO_NOT_ECHO"));
}

#[test]
fn request_binds_expected_producer_and_static_discovery_freshness() {
    let mut definition = TrustedHostInputObservationRequestDefinition {
        version: TrustedHostInputObservationContractVersion::CargoHostInputsV2,
        run_bundle: hash("run"),
        operation_contract: hash("operation"),
        static_request: hash("static-request"),
        static_result: hash("static-result"),
        static_discovery_completed_revision: 0,
        repository_tree: hash("repository"),
        security_requirements: hash("security"),
        selection: hash("selection"),
        attempt: hash("attempt"),
        expected_producer_id: producer_id(),
        expected_producer_version: producer_version(),
    };
    let error = TrustedHostInputObservationRequest::new(definition.clone()).unwrap_err();
    assert_eq!(
        error.code(),
        "trusted_host_input_observer.request.freshness.invalid"
    );

    definition.static_discovery_completed_revision = 10;
    definition.expected_producer_id =
        TrustedHostInputObservationProducerId::new("observer/other").unwrap();
    let mismatched_request = TrustedHostInputObservationRequest::new(definition).unwrap();
    let original_request = request();
    let result =
        TrustedHostInputObservationResult::new(result_definition(&original_request)).unwrap();
    assert!(result.validate_for_request(&mismatched_request).is_err());

    let mut stale = result_definition(&original_request);
    stale.started_revision = 10;
    stale.completed_revision = 10;
    let stale = TrustedHostInputObservationResult::new(stale).unwrap();
    assert!(stale.validate_for_request(&original_request).is_err());
}

#[test]
fn per_fact_producer_provenance_is_exact_and_homogeneous() {
    let request = request();
    let mut definition = result_definition(&request);
    definition.facts[0] =
        TrustedHostInputObservationFact::new(TrustedHostInputObservationFactDefinition {
            producer_id: TrustedHostInputObservationProducerId::new("observer/substituted")
                .unwrap(),
            producer_version: producer_version(),
            kind: definition.facts[0].kind(),
            posture: TrustedHostInputObservationPosture::Observed,
            reason: None,
            method: TrustedHostInputObservationMethod::Fixture,
            object_binding: TrustedHostObservationPosture::Proven,
            continuity: TrustedHostObservationPosture::Proven,
            commitment: hash("substituted"),
            item_count: 1,
            reference: None,
        })
        .unwrap();
    let error = TrustedHostInputObservationResult::new(definition).unwrap_err();
    assert_eq!(
        error.code(),
        "trusted_host_input_observer.result.binding.invalid"
    );
}

#[test]
fn non_proven_facts_require_exact_reason_and_never_accept_one_proven_axis() {
    let base = TrustedHostInputObservationFactDefinition {
        producer_id: producer_id(),
        producer_version: producer_version(),
        kind: TrustedHostBuildInputHostObligationKind::Environment,
        posture: TrustedHostInputObservationPosture::Unavailable,
        reason: None,
        method: TrustedHostInputObservationMethod::Unavailable,
        object_binding: TrustedHostObservationPosture::Unavailable,
        continuity: TrustedHostObservationPosture::Unavailable,
        commitment: hash("unavailable"),
        item_count: 0,
        reference: None,
    };
    assert!(TrustedHostInputObservationFact::new(base.clone()).is_err());

    let mut wrong_reason = base.clone();
    wrong_reason.reason = Some(TrustedHostInputObservationReason::ObservationUnsupported);
    assert!(TrustedHostInputObservationFact::new(wrong_reason).is_err());

    let mut partial_proof = base;
    partial_proof.reason = Some(TrustedHostInputObservationReason::ObservationUnavailable);
    partial_proof.object_binding = TrustedHostObservationPosture::Proven;
    assert!(TrustedHostInputObservationFact::new(partial_proof).is_err());
}

#[test]
fn posture_method_and_producer_method_combinations_fail_closed() {
    let uncertain_unavailable =
        TrustedHostInputObservationFact::new(TrustedHostInputObservationFactDefinition {
            producer_id: producer_id(),
            producer_version: producer_version(),
            kind: TrustedHostBuildInputHostObligationKind::Environment,
            posture: TrustedHostInputObservationPosture::Uncertain,
            reason: Some(TrustedHostInputObservationReason::IntegrityOrContinuityUncertain),
            method: TrustedHostInputObservationMethod::Unavailable,
            object_binding: TrustedHostObservationPosture::Uncertain,
            continuity: TrustedHostObservationPosture::Uncertain,
            commitment: hash("uncertain"),
            item_count: 0,
            reference: None,
        });
    assert!(uncertain_unavailable.is_err());

    let request = request();
    let mut definition = result_definition(&request);
    definition.producer =
        TrustedHostInputObservationProducer::new(TrustedHostInputObservationProducerDefinition {
            id: producer_id(),
            version: producer_version(),
            kind: TrustedHostInputObservationProducerKind::HostObserver,
            assurance: TrustedHostInputObservationProducerAssurance::IndependentlyReviewed,
            implementation: hash("host-observer"),
            configuration: hash("configuration"),
        })
        .unwrap();
    let error = TrustedHostInputObservationResult::new(definition).unwrap_err();
    assert_eq!(
        error.code(),
        "trusted_host_input_observer.result.binding.invalid"
    );
}

#[test]
fn nested_reused_enum_deserialization_never_echoes_unknown_values() {
    let request = request();
    let result = TrustedHostInputObservationResult::new(result_definition(&request)).unwrap();
    for path in [
        &["repository_root", "role"][..],
        &["repository_root", "access"][..],
        &["repository_root", "object_binding"][..],
        &["repository_root", "method"][..],
        &["facts", "0", "object_binding"][..],
        &["facts", "0", "reference", "kind"][..],
    ] {
        let mut value = serde_json::to_value(&result).unwrap();
        let mut current = &mut value;
        for segment in &path[..path.len() - 1] {
            current = if let Ok(index) = segment.parse::<usize>() {
                &mut current[index]
            } else {
                &mut current[*segment]
            };
        }
        current[path[path.len() - 1]] = serde_json::json!("TOKEN_DO_NOT_ECHO");
        let error = serde_json::from_value::<TrustedHostInputObservationResult>(value)
            .unwrap_err()
            .to_string();
        assert!(!error.contains("TOKEN_DO_NOT_ECHO"));
    }
}

#[test]
fn producer_identifiers_reject_secret_like_values_without_echoing_them() {
    let secret = "authorization_bearer_DO_NOT_ECHO";
    let error = TrustedHostInputObservationProducerId::new(secret)
        .unwrap_err()
        .to_string();
    assert!(!error.contains(secret));

    let error = serde_json::from_value::<TrustedHostInputObservationProducerVersion>(
        serde_json::json!(secret),
    )
    .unwrap_err()
    .to_string();
    assert!(!error.contains(secret));
}

#[test]
fn caller_supplied_producer_cannot_assert_observed_facts() {
    let request = request();
    let mut definition = result_definition(&request);
    definition.producer =
        TrustedHostInputObservationProducer::new(TrustedHostInputObservationProducerDefinition {
            id: producer_id(),
            version: producer_version(),
            kind: TrustedHostInputObservationProducerKind::HostObserver,
            assurance: TrustedHostInputObservationProducerAssurance::CallerSupplied,
            implementation: hash("caller-supplied-observer"),
            configuration: hash("configuration"),
        })
        .unwrap();
    let error = TrustedHostInputObservationResult::new(definition).unwrap_err();
    assert_eq!(
        error.code(),
        "trusted_host_input_observer.result.binding.invalid"
    );
}
