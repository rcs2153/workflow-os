#![doc = "Fixture-backed proof for trusted-host input observation."]
#![allow(clippy::expect_used, clippy::unwrap_used)]

use workflow_core::{
    observe_trusted_host_inputs_from_fixture, HostedExecutionReference,
    HostedExecutionReferenceKind, SpecContentHash, TrustedHostBuildInputHostObligationKind,
    TrustedHostInputObservationContractVersion, TrustedHostInputObservationFact,
    TrustedHostInputObservationFactDefinition, TrustedHostInputObservationFixture,
    TrustedHostInputObservationMethod, TrustedHostInputObservationPosture,
    TrustedHostInputObservationProducer, TrustedHostInputObservationProducerAssurance,
    TrustedHostInputObservationProducerDefinition, TrustedHostInputObservationProducerId,
    TrustedHostInputObservationProducerKind, TrustedHostInputObservationProducerVersion,
    TrustedHostInputObservationRequest, TrustedHostInputObservationRequestDefinition,
    TrustedHostObservationPosture, TrustedHostRootAccess, TrustedHostRootObservation,
    TrustedHostRootObservationDefinition, TrustedHostRootObservationMethod, TrustedHostRootRole,
};

fn hash(label: &str) -> SpecContentHash {
    SpecContentHash::from_text(label)
}

fn producer_id() -> TrustedHostInputObservationProducerId {
    TrustedHostInputObservationProducerId::new("observer/fixture-proof").unwrap()
}

fn producer_version() -> TrustedHostInputObservationProducerVersion {
    TrustedHostInputObservationProducerVersion::new("v1").unwrap()
}

fn producer() -> TrustedHostInputObservationProducer {
    TrustedHostInputObservationProducer::new(TrustedHostInputObservationProducerDefinition {
        id: producer_id(),
        version: producer_version(),
        kind: TrustedHostInputObservationProducerKind::Fixture,
        assurance: TrustedHostInputObservationProducerAssurance::FixtureOnly,
        implementation: hash("fixture-implementation"),
        configuration: hash("fixture-configuration"),
    })
    .unwrap()
}

fn request(
    expected_id: TrustedHostInputObservationProducerId,
) -> TrustedHostInputObservationRequest {
    TrustedHostInputObservationRequest::new(TrustedHostInputObservationRequestDefinition {
        version: TrustedHostInputObservationContractVersion::CargoHostInputsV2,
        run_bundle: hash("run-bundle"),
        operation_contract: hash("operation-contract"),
        static_request: hash("static-request"),
        static_result: hash("static-result"),
        ordering_authority: hash("ordering-authority"),
        static_discovery_completed_revision: 40,
        repository_tree: hash("repository-tree"),
        security_requirements: hash("security-requirements"),
        selection: hash("selection"),
        attempt: hash("attempt"),
        expected_producer_id: expected_id,
        expected_producer_version: producer_version(),
    })
    .unwrap()
}

fn telemetry(label: &str) -> HostedExecutionReference {
    HostedExecutionReference::new(HostedExecutionReferenceKind::Telemetry, label).unwrap()
}

fn root(repository_tree: SpecContentHash) -> TrustedHostRootObservation {
    TrustedHostRootObservation::new(TrustedHostRootObservationDefinition {
        role: TrustedHostRootRole::RepositoryInput,
        access: TrustedHostRootAccess::ReadOnly,
        object_binding: TrustedHostObservationPosture::Proven,
        continuity: TrustedHostObservationPosture::Proven,
        method: TrustedHostRootObservationMethod::ImmutableProjection,
        invocation_scoped: false,
        commitment: repository_tree,
        reference: telemetry("fixture/root-observation"),
    })
    .unwrap()
}

fn facts() -> Vec<TrustedHostInputObservationFact> {
    use TrustedHostBuildInputHostObligationKind::{
        DynamicLibraries, Environment, LinkerOrArchiver, NativeCompilerTools,
        RegistryAndPackageSources, RuntimeBase, SdkOrSysroot, TargetSpecification,
        ToolchainExecutable,
    };

    [
        RuntimeBase,
        Environment,
        RegistryAndPackageSources,
        DynamicLibraries,
        NativeCompilerTools,
        LinkerOrArchiver,
        SdkOrSysroot,
        TargetSpecification,
        ToolchainExecutable,
    ]
    .into_iter()
    .map(|kind| {
        TrustedHostInputObservationFact::new(TrustedHostInputObservationFactDefinition {
            producer_id: producer_id(),
            producer_version: producer_version(),
            kind,
            posture: TrustedHostInputObservationPosture::Observed,
            reason: None,
            method: TrustedHostInputObservationMethod::Fixture,
            object_binding: TrustedHostObservationPosture::Proven,
            continuity: TrustedHostObservationPosture::Proven,
            commitment: hash(&format!("raw-provider-payload-marker-{kind:?}")),
            item_count: 1,
            reference: Some(telemetry(&format!("fixture/fact/{kind:?}"))),
        })
        .unwrap()
    })
    .collect()
}

fn fixture(
    started_revision: u64,
    facts: Vec<TrustedHostInputObservationFact>,
) -> TrustedHostInputObservationFixture {
    TrustedHostInputObservationFixture::new(
        producer(),
        started_revision,
        started_revision + 1,
        root(hash("repository-tree")),
        facts,
    )
    .unwrap()
}

#[test]
fn explicit_fixture_produces_the_same_complete_result_twice() {
    let request = request(producer_id());
    let fixture = fixture(41, facts());

    let first = observe_trusted_host_inputs_from_fixture(&request, &fixture).unwrap();
    let second = observe_trusted_host_inputs_from_fixture(&request, &fixture).unwrap();

    assert_eq!(first, second);
    assert_eq!(first.fingerprint(), second.fingerprint());
    assert_eq!(first.facts().len(), 9);
    assert!(first
        .facts()
        .windows(2)
        .all(|pair| pair[0].kind() < pair[1].kind()));
}

#[test]
fn fixture_uses_the_exact_request_bindings() {
    let fixture = fixture(41, facts());
    let substituted_request = request(
        TrustedHostInputObservationProducerId::new("observer/substituted-fixture").unwrap(),
    );

    let error = observe_trusted_host_inputs_from_fixture(&substituted_request, &fixture)
        .expect_err("producer substitution must fail");

    assert_eq!(
        error.code(),
        "trusted_host_input_observer.result.request_mismatch"
    );
}

#[test]
fn stale_fixture_revision_fails_closed() {
    let request = request(producer_id());
    let fixture = fixture(40, facts());

    let error = observe_trusted_host_inputs_from_fixture(&request, &fixture)
        .expect_err("stale fixture must fail");

    assert_eq!(
        error.code(),
        "trusted_host_input_observer.result.request_mismatch"
    );
}

#[test]
fn incomplete_fixture_fails_through_the_core_result_constructor() {
    let request = request(producer_id());
    let mut incomplete = facts();
    incomplete.pop();
    let fixture = fixture(41, incomplete);

    let error = observe_trusted_host_inputs_from_fixture(&request, &fixture)
        .expect_err("incomplete facts must fail");

    assert_eq!(
        error.code(),
        "trusted_host_input_observer.result.binding.invalid"
    );
}

#[test]
fn fixture_source_rejects_non_fixture_producer() {
    let host_producer =
        TrustedHostInputObservationProducer::new(TrustedHostInputObservationProducerDefinition {
            id: TrustedHostInputObservationProducerId::new("observer/host").unwrap(),
            version: producer_version(),
            kind: TrustedHostInputObservationProducerKind::HostObserver,
            assurance: TrustedHostInputObservationProducerAssurance::IndependentlyReviewed,
            implementation: hash("host-implementation"),
            configuration: hash("host-configuration"),
        })
        .unwrap();

    let error = TrustedHostInputObservationFixture::new(
        host_producer,
        41,
        42,
        root(hash("repository-tree")),
        facts(),
    )
    .expect_err("live producer cannot enter fixture path");

    assert_eq!(
        error.code(),
        "trusted_host_input_observer.fixture.producer.invalid"
    );
}

#[test]
fn fixture_debug_and_result_serialization_do_not_copy_payload_markers() {
    let request = request(producer_id());
    let fixture = fixture(41, facts());
    let debug = format!("{fixture:?}");
    let result = observe_trusted_host_inputs_from_fixture(&request, &fixture).unwrap();
    let serialized = serde_json::to_string(&result).unwrap();

    assert!(!debug.contains("fixture/root-observation"));
    assert!(!debug.contains("raw-provider-payload-marker"));
    assert!(!serialized.contains("raw-provider-payload-marker"));
}
