// Tests are kept beside the private model to verify authority and privacy boundaries.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use super::*;

fn hash(label: &str) -> SpecContentHash {
    SpecContentHash::from_text(label)
}

fn telemetry(label: &str) -> HostedExecutionReference {
    HostedExecutionReference::new(HostedExecutionReferenceKind::Telemetry, label).unwrap()
}

fn request() -> TrustedHostInputObservationRequest {
    TrustedHostInputObservationRequest::new(TrustedHostInputObservationRequestDefinition {
        version: TrustedHostInputObservationContractVersion::CargoHostInputsV1,
        run_bundle: hash("run"),
        operation_contract: hash("operation"),
        static_request: hash("static-request"),
        static_result: hash("static-result"),
        repository_tree: hash("repository"),
        security_requirements: hash("security"),
        selection: hash("selection"),
        attempt: hash("attempt"),
    })
}

fn fact(kind: TrustedHostBuildInputHostObligationKind) -> TrustedHostInputObservationFact {
    TrustedHostInputObservationFact::new(TrustedHostInputObservationFactDefinition {
        kind,
        posture: TrustedHostInputObservationPosture::Observed,
        method: TrustedHostInputObservationMethod::ContentAddressedObject,
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
                kind: TrustedHostInputObservationProducerKind::Fixture,
                assurance: TrustedHostInputObservationProducerAssurance::FixtureOnly,
                implementation: hash("implementation"),
                configuration: hash("configuration"),
            },
        )
        .unwrap(),
        started_revision: 1,
        completed_revision: 2,
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
        "75131a0c1ac7146ae1219a40967385ad55d131963b16b7a2da0b9fefda076821"
    );
    assert_eq!(
        first.fingerprint().as_str(),
        "e211dd8f8caf69a799494b82f738022bd6d0ec782eb3ad14f5e0321f7f3a1625"
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
            kind: fact_changed.facts[0].kind(),
            posture: TrustedHostInputObservationPosture::Uncertain,
            method: TrustedHostInputObservationMethod::ContentAddressedObject,
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
        kind: TrustedHostBuildInputHostObligationKind::Environment,
        posture: TrustedHostInputObservationPosture::DeclaredOnly,
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
    for (posture, method, binding) in [
        (
            TrustedHostInputObservationPosture::DeclaredOnly,
            TrustedHostInputObservationMethod::DeclaredConfiguration,
            TrustedHostObservationPosture::Uncertain,
        ),
        (
            TrustedHostInputObservationPosture::Unavailable,
            TrustedHostInputObservationMethod::Unavailable,
            TrustedHostObservationPosture::Unavailable,
        ),
        (
            TrustedHostInputObservationPosture::Unsupported,
            TrustedHostInputObservationMethod::Unsupported,
            TrustedHostObservationPosture::Unsupported,
        ),
        (
            TrustedHostInputObservationPosture::Uncertain,
            TrustedHostInputObservationMethod::ProviderNativeIdentity,
            TrustedHostObservationPosture::Uncertain,
        ),
    ] {
        let fact =
            TrustedHostInputObservationFact::new(TrustedHostInputObservationFactDefinition {
                kind: TrustedHostBuildInputHostObligationKind::Environment,
                posture,
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
            kind: TrustedHostBuildInputHostObligationKind::Environment,
            posture: TrustedHostInputObservationPosture::Observed,
            method: TrustedHostInputObservationMethod::ContentAddressedObject,
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
