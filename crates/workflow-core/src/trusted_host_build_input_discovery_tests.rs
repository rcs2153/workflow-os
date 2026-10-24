#![allow(clippy::expect_used)]

use serde_json::json;

use super::{
    TrustedHostBuildInputDiscoveryContractVersion, TrustedHostBuildInputDiscoveryRequest,
    TrustedHostBuildInputDiscoveryRequestDefinition, TrustedHostBuildInputDiscoveryResult,
    TrustedHostBuildInputDiscoveryResultDefinition, TrustedHostBuildInputHostObligation,
    TrustedHostBuildInputHostObligationKind, TrustedHostBuildInputHostObligationPosture,
    TrustedHostBuildInputSelectionPosture, TrustedHostBuildInputUnsupportedReason,
    TrustedHostStaticGraphCompleteness,
};
use crate::{
    BuildTimeInputKind, BuildTimeInputKindCount, HostedExecutionReference,
    HostedExecutionReferenceKind, SpecContentHash,
};

fn hash(label: &str) -> SpecContentHash {
    SpecContentHash::from_text(label)
}

fn request() -> TrustedHostBuildInputDiscoveryRequest {
    TrustedHostBuildInputDiscoveryRequest::new(TrustedHostBuildInputDiscoveryRequestDefinition {
        version: TrustedHostBuildInputDiscoveryContractVersion::StaticCargoGraphV1,
        repository_tree_commitment: hash("repository"),
        selection_posture: TrustedHostBuildInputSelectionPosture::Explicit,
        selection_commitment: hash("selection"),
        metadata_source_set_commitment: hash("metadata-sources"),
        discovery_policy_commitment: hash("policy"),
    })
}

fn host_obligations() -> Vec<TrustedHostBuildInputHostObligation> {
    use TrustedHostBuildInputHostObligationKind as Kind;
    [
        Kind::ToolchainExecutable,
        Kind::TargetSpecification,
        Kind::SdkOrSysroot,
        Kind::LinkerOrArchiver,
        Kind::NativeCompilerTools,
        Kind::DynamicLibraries,
        Kind::RegistryAndPackageSources,
        Kind::Environment,
        Kind::RuntimeBase,
    ]
    .into_iter()
    .map(|kind| {
        TrustedHostBuildInputHostObligation::new(
            kind,
            TrustedHostBuildInputHostObligationPosture::Unavailable,
        )
    })
    .collect()
}

fn complete_definition(
    request: &TrustedHostBuildInputDiscoveryRequest,
) -> TrustedHostBuildInputDiscoveryResultDefinition {
    TrustedHostBuildInputDiscoveryResultDefinition {
        request_fingerprint: request.request_fingerprint().clone(),
        static_completeness: TrustedHostStaticGraphCompleteness::Complete,
        total_count: 9,
        kind_counts: vec![
            BuildTimeInputKindCount::new(BuildTimeInputKind::DependencyPackage, 5)
                .expect("dependency count"),
            BuildTimeInputKindCount::new(BuildTimeInputKind::RepositoryContent, 1)
                .expect("repository count"),
            BuildTimeInputKindCount::new(BuildTimeInputKind::ManifestOrConfiguration, 3)
                .expect("manifest count"),
        ],
        host_obligations: host_obligations(),
        unsupported_reasons: vec![],
        observation_reference: Some(
            HostedExecutionReference::new(
                HostedExecutionReferenceKind::Telemetry,
                "build-input-observation-1",
            )
            .expect("reference"),
        ),
    }
}

#[test]
fn valid_complete_static_graph_remains_distinct_from_live_input_completeness() {
    let request = request();
    let result = TrustedHostBuildInputDiscoveryResult::new(complete_definition(&request))
        .expect("valid result");

    assert_eq!(
        result.static_completeness(),
        TrustedHostStaticGraphCompleteness::Complete
    );
    assert_eq!(result.host_obligations().len(), 9);
    assert!(result
        .host_obligations()
        .iter()
        .all(|entry| entry.posture() == TrustedHostBuildInputHostObligationPosture::Unavailable));
    result
        .validate_for_request(&request)
        .expect("exact request");
}

#[test]
fn request_fingerprint_is_deterministic_and_selection_sensitive() {
    let first = request();
    let mut definition = TrustedHostBuildInputDiscoveryRequestDefinition {
        version: first.version(),
        repository_tree_commitment: first.repository_tree_commitment().clone(),
        selection_posture: first.selection_posture(),
        selection_commitment: first.selection_commitment().clone(),
        metadata_source_set_commitment: first.metadata_source_set_commitment().clone(),
        discovery_policy_commitment: first.discovery_policy_commitment().clone(),
    };
    let same = TrustedHostBuildInputDiscoveryRequest::new(definition.clone());
    assert_eq!(first.request_fingerprint(), same.request_fingerprint());

    definition.selection_commitment = hash("different-selection");
    let changed = TrustedHostBuildInputDiscoveryRequest::new(definition);
    assert_ne!(first.request_fingerprint(), changed.request_fingerprint());
}

#[test]
fn request_fingerprint_has_fixed_vector() {
    assert_eq!(
        request().request_fingerprint().as_str(),
        "c7af6033b90672844948848e25a063ef515b24126c947468b0d0ba6ee54c532f"
    );
}

#[test]
fn graph_commitment_is_order_independent() {
    let request = request();
    let first =
        TrustedHostBuildInputDiscoveryResult::new(complete_definition(&request)).expect("first");
    let mut reversed = complete_definition(&request);
    reversed.kind_counts.reverse();
    reversed.host_obligations.reverse();
    let second = TrustedHostBuildInputDiscoveryResult::new(reversed).expect("second");
    assert_eq!(first.graph_commitment(), second.graph_commitment());
}

#[test]
fn graph_commitment_has_fixed_vector() {
    let request = request();
    let result =
        TrustedHostBuildInputDiscoveryResult::new(complete_definition(&request)).expect("result");
    assert_eq!(
        result.graph_commitment().as_str(),
        "e3d3f32ad6d54f8ffec204b0b100d4b426cf19b7802369cb304afa3ccfae83de"
    );
}

#[test]
fn unknown_request_field_fails_closed_without_leakage() {
    let mut value = serde_json::to_value(request()).expect("request");
    value["secret-token-like-request-field"] = json!("secret-token-like-request-value");
    let error = serde_json::from_value::<TrustedHostBuildInputDiscoveryRequest>(value)
        .expect_err("unknown request field rejected")
        .to_string();
    assert!(!error.contains("secret-token-like-request-field"));
    assert!(!error.contains("secret-token-like-request-value"));
}

#[test]
fn unknown_result_field_fails_closed_without_leakage() {
    let request = request();
    let mut value = serde_json::to_value(complete_definition(&request)).expect("result");
    value["secret-token-like-result-field"] = json!("secret-token-like-result-value");
    let error = serde_json::from_value::<TrustedHostBuildInputDiscoveryResult>(value)
        .expect_err("unknown result field rejected")
        .to_string();
    assert!(!error.contains("secret-token-like-result-field"));
    assert!(!error.contains("secret-token-like-result-value"));
}

#[test]
fn unknown_nested_fields_fail_closed_without_leakage() {
    let request = request();

    let mut obligation = serde_json::to_value(complete_definition(&request)).expect("result");
    obligation["host_obligations"][0]["secret-token-like-obligation-field"] =
        json!("secret-token-like-obligation-value");
    let obligation_error =
        serde_json::from_value::<TrustedHostBuildInputDiscoveryResult>(obligation)
            .expect_err("unknown obligation field rejected")
            .to_string();
    assert!(!obligation_error.contains("secret-token-like-obligation-field"));
    assert!(!obligation_error.contains("secret-token-like-obligation-value"));

    let mut kind_count = serde_json::to_value(complete_definition(&request)).expect("result");
    kind_count["kind_counts"][0]["secret-token-like-count-field"] =
        json!("secret-token-like-count-value");
    let count_error = serde_json::from_value::<TrustedHostBuildInputDiscoveryResult>(kind_count)
        .expect_err("unknown count field rejected")
        .to_string();
    assert!(!count_error.contains("secret-token-like-count-field"));
    assert!(!count_error.contains("secret-token-like-count-value"));

    let mut reference = serde_json::to_value(complete_definition(&request)).expect("result");
    reference["observation_reference"]["secret-token-like-reference-field"] =
        json!("secret-token-like-reference-value");
    let reference_error = serde_json::from_value::<TrustedHostBuildInputDiscoveryResult>(reference)
        .expect_err("unknown reference field rejected")
        .to_string();
    assert!(!reference_error.contains("secret-token-like-reference-field"));
    assert!(!reference_error.contains("secret-token-like-reference-value"));
}

#[test]
fn request_and_result_serde_recompute_derived_commitments() {
    let request = request();
    let request_value = serde_json::to_value(&request).expect("request serialization");
    assert!(request_value.get("request_fingerprint").is_none());
    let restored_request: TrustedHostBuildInputDiscoveryRequest =
        serde_json::from_value(request_value).expect("request round trip");
    assert_eq!(request, restored_request);

    let result =
        TrustedHostBuildInputDiscoveryResult::new(complete_definition(&request)).expect("result");
    let result_value = serde_json::to_value(&result).expect("result serialization");
    assert!(result_value.get("graph_commitment").is_none());
    let restored_result: TrustedHostBuildInputDiscoveryResult =
        serde_json::from_value(result_value).expect("result round trip");
    assert_eq!(result, restored_result);
}

#[test]
fn result_rejects_request_substitution_without_leaking_commitments() {
    let request = request();
    let result =
        TrustedHostBuildInputDiscoveryResult::new(complete_definition(&request)).expect("result");
    let other = TrustedHostBuildInputDiscoveryRequest::new(
        TrustedHostBuildInputDiscoveryRequestDefinition {
            version: request.version(),
            repository_tree_commitment: hash("other-repository"),
            selection_posture: request.selection_posture(),
            selection_commitment: request.selection_commitment().clone(),
            metadata_source_set_commitment: request.metadata_source_set_commitment().clone(),
            discovery_policy_commitment: request.discovery_policy_commitment().clone(),
        },
    );
    let error = result
        .validate_for_request(&other)
        .expect_err("substitution rejected");
    assert_eq!(
        error.code(),
        "trusted_host.build_input_discovery.result.request_mismatch"
    );
    assert!(!error
        .to_string()
        .contains(request.request_fingerprint().as_str()));
    assert!(!error
        .to_string()
        .contains(other.request_fingerprint().as_str()));
}

#[test]
fn complete_graph_rejects_reasons_and_non_complete_graph_requires_them() {
    let request = request();
    let mut complete = complete_definition(&request);
    complete
        .unsupported_reasons
        .push(TrustedHostBuildInputUnsupportedReason::MetadataInvalid);
    assert!(TrustedHostBuildInputDiscoveryResult::new(complete).is_err());

    let mut incomplete = complete_definition(&request);
    incomplete.static_completeness = TrustedHostStaticGraphCompleteness::Incomplete;
    assert!(TrustedHostBuildInputDiscoveryResult::new(incomplete).is_err());
}

#[test]
fn duplicate_or_missing_host_obligations_fail_closed() {
    let request = request();
    let mut missing = complete_definition(&request);
    missing.host_obligations.pop();
    assert!(TrustedHostBuildInputDiscoveryResult::new(missing).is_err());

    let mut duplicate = complete_definition(&request);
    duplicate.host_obligations[1] = duplicate.host_obligations[0];
    assert!(TrustedHostBuildInputDiscoveryResult::new(duplicate).is_err());
}

#[test]
fn duplicate_and_mismatched_counts_fail_closed() {
    let request = request();
    let mut duplicate = complete_definition(&request);
    duplicate.kind_counts.push(
        BuildTimeInputKindCount::new(BuildTimeInputKind::RepositoryContent, 0).expect("duplicate"),
    );
    assert!(TrustedHostBuildInputDiscoveryResult::new(duplicate).is_err());

    let mut mismatch = complete_definition(&request);
    mismatch.total_count = 10;
    assert!(TrustedHostBuildInputDiscoveryResult::new(mismatch).is_err());
}

#[test]
fn zero_kind_count_and_empty_complete_graph_fail_closed() {
    let request = request();
    let mut zero_kind = complete_definition(&request);
    zero_kind.kind_counts[0] =
        BuildTimeInputKindCount::new(BuildTimeInputKind::RepositoryContent, 0)
            .expect("bounded zero");
    zero_kind.total_count = 4;
    assert!(TrustedHostBuildInputDiscoveryResult::new(zero_kind).is_err());

    let mut empty = complete_definition(&request);
    empty.kind_counts.clear();
    empty.total_count = 0;
    assert!(TrustedHostBuildInputDiscoveryResult::new(empty).is_err());
}

#[test]
fn invalid_reference_kind_fails_closed() {
    let request = request();
    let mut definition = complete_definition(&request);
    definition.observation_reference = Some(
        HostedExecutionReference::new(HostedExecutionReferenceKind::Log, "log-reference")
            .expect("reference"),
    );
    let error = TrustedHostBuildInputDiscoveryResult::new(definition)
        .expect_err("invalid reference rejected");
    assert_eq!(
        error.code(),
        "trusted_host.build_input_discovery.result.reference.invalid"
    );
}

#[test]
fn debug_redacts_commitments_and_references() {
    let request = request();
    let result =
        TrustedHostBuildInputDiscoveryResult::new(complete_definition(&request)).expect("result");
    let debug = format!("{request:?} {result:?}");
    assert!(!debug.contains(request.repository_tree_commitment().as_str()));
    assert!(!debug.contains(request.request_fingerprint().as_str()));
    assert!(!debug.contains(result.graph_commitment().as_str()));
    assert!(!debug.contains("build-input-observation-1"));
}

#[test]
fn unknown_version_deserialization_does_not_echo_secret_like_value() {
    let value = json!({
        "version": "secret-token-like-version",
        "repository_tree_commitment": hash("repository"),
        "selection_posture": "explicit",
        "selection_commitment": hash("selection"),
        "metadata_source_set_commitment": hash("metadata"),
        "discovery_policy_commitment": hash("policy")
    });
    let error = serde_json::from_value::<TrustedHostBuildInputDiscoveryRequest>(value)
        .expect_err("unknown version rejected")
        .to_string();
    assert!(!error.contains("secret-token-like-version"));
}

#[test]
fn unknown_nested_enum_deserialization_does_not_echo_secret_like_value() {
    let request = request();
    let mut value = serde_json::to_value(complete_definition(&request)).expect("definition");
    value["host_obligations"][0]["kind"] = json!("secret-token-like-obligation");
    let error = serde_json::from_value::<TrustedHostBuildInputDiscoveryResult>(value)
        .expect_err("unknown obligation rejected")
        .to_string();
    assert!(!error.contains("secret-token-like-obligation"));
}

#[test]
fn unknown_input_kind_deserialization_does_not_echo_secret_like_value() {
    let request = request();
    let mut value = serde_json::to_value(complete_definition(&request)).expect("definition");
    value["kind_counts"][0]["kind"] = json!("secret-token-like-input-kind");
    let error = serde_json::from_value::<TrustedHostBuildInputDiscoveryResult>(value)
        .expect_err("unknown input kind rejected")
        .to_string();
    assert!(!error.contains("secret-token-like-input-kind"));
}

#[test]
fn invalid_result_serde_reenters_validation() {
    let request = request();
    let mut value = serde_json::to_value(complete_definition(&request)).expect("definition");
    value["total_count"] = json!(99);
    let error = serde_json::from_value::<TrustedHostBuildInputDiscoveryResult>(value)
        .expect_err("invalid count rejected");
    assert!(error
        .to_string()
        .contains("trusted-host static discovery counts do not reconcile"));
}

#[test]
fn all_static_postures_and_unsupported_reasons_are_representable() {
    let postures = [
        TrustedHostStaticGraphCompleteness::Complete,
        TrustedHostStaticGraphCompleteness::Incomplete,
        TrustedHostStaticGraphCompleteness::Unsupported,
        TrustedHostStaticGraphCompleteness::Uncertain,
    ];
    assert_eq!(postures.len(), 4);

    let reasons = [
        TrustedHostBuildInputUnsupportedReason::RepositoryObjectBindingUnavailable,
        TrustedHostBuildInputUnsupportedReason::SelectionAmbiguous,
        TrustedHostBuildInputUnsupportedReason::ImplicitTargetUnsupported,
        TrustedHostBuildInputUnsupportedReason::ImplicitBuildScriptUnsupported,
        TrustedHostBuildInputUnsupportedReason::CargoConfigurationUnsupported,
        TrustedHostBuildInputUnsupportedReason::SourceReplacementUnsupported,
        TrustedHostBuildInputUnsupportedReason::SymlinkUnsupported,
        TrustedHostBuildInputUnsupportedReason::NestedRepositoryUnsupported,
        TrustedHostBuildInputUnsupportedReason::RootEscape,
        TrustedHostBuildInputUnsupportedReason::LockfileInconsistent,
        TrustedHostBuildInputUnsupportedReason::MetadataChanged,
        TrustedHostBuildInputUnsupportedReason::BoundExceeded,
        TrustedHostBuildInputUnsupportedReason::MetadataInvalid,
    ];
    assert_eq!(reasons.len(), 13);
}

#[test]
fn model_construction_requires_no_filesystem_process_provider_or_state_backend() {
    let request = request();
    let result = TrustedHostBuildInputDiscoveryResult::new(complete_definition(&request))
        .expect("pure model");
    assert_eq!(result.total_count(), 9);
}
