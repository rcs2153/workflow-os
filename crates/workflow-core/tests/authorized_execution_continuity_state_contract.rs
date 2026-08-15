#![allow(clippy::expect_used, clippy::panic)]
#![doc = "Public compatibility tests for authorized-execution continuity state support."]

use std::sync::Arc;

use postgres::Client;
use workflow_core::{
    AuthorizedExecutionContinuityOperationKind, AuthorizedExecutionContinuityOperationSupport,
    AuthorizedExecutionContinuityOperationSupportEntry, AuthorizedExecutionContinuityStateContract,
    AuthorizedExecutionContinuityStateContractProvider,
    AuthorizedExecutionContinuityStateContractV2,
    AuthorizedExecutionContinuityStateContractV2Version,
    AuthorizedExecutionContinuityStateContractVersion, AuthorizedExecutionContinuitySupportScope,
    LocalStateBackend, PostgresConnectionFactory, PostgresStateBackend, SqliteStateBackend,
    WorkflowOsError,
};

struct UnexpectedConnectionFactory;

impl PostgresConnectionFactory for UnexpectedConnectionFactory {
    fn connect(&self) -> Result<Client, WorkflowOsError> {
        panic!("continuity support declaration must not connect to PostgreSQL");
    }
}

#[test]
fn contract_requires_one_declaration_for_every_operation() {
    let incomplete = AuthorizedExecutionContinuityStateContract::new(
        AuthorizedExecutionContinuityStateContractVersion::V1,
        vec![AuthorizedExecutionContinuityOperationSupportEntry::new(
            AuthorizedExecutionContinuityOperationKind::RegisterYield,
            AuthorizedExecutionContinuityOperationSupport::Unsupported,
        )],
    )
    .expect_err("incomplete contract must fail");
    assert_eq!(
        incomplete.code(),
        "authorized_execution_continuity_state.contract.invalid"
    );
}

#[test]
fn valid_contract_round_trips_and_invalid_wire_fails_closed() {
    let contract = AuthorizedExecutionContinuityStateContract::new(
        AuthorizedExecutionContinuityStateContractVersion::V1,
        AuthorizedExecutionContinuityOperationKind::all()
            .iter()
            .copied()
            .map(|kind| {
                AuthorizedExecutionContinuityOperationSupportEntry::new(
                    kind,
                    AuthorizedExecutionContinuityOperationSupport::Unsupported,
                )
            })
            .collect(),
    )
    .expect("complete contract");
    let encoded = serde_json::to_string(&contract).expect("serialize contract");
    let decoded: AuthorizedExecutionContinuityStateContract =
        serde_json::from_str(&encoded).expect("deserialize contract");
    assert_eq!(decoded, contract);

    let invalid = encoded.replacen("\"transition_wait\"", "\"register_yield\"", 1);
    let error = serde_json::from_str::<AuthorizedExecutionContinuityStateContract>(&invalid)
        .expect_err("duplicate operation must fail");
    assert!(!error.to_string().contains("repo/"));
}

#[test]
fn legacy_v1_contracts_remain_readable_and_canonical() {
    let legacy = serde_json::json!({
        "version": "v1",
        "operations": [
            { "kind": "recover_ambiguous_attempt", "support": "supported" },
            { "kind": "record_attempt_outcome", "support": "unsupported" },
            { "kind": "consume_directive", "support": "supported" },
            { "kind": "transition_wait", "support": "unsupported" },
            { "kind": "register_yield", "support": "supported" }
        ]
    });
    let decoded: AuthorizedExecutionContinuityStateContract =
        serde_json::from_value(legacy).expect("legacy V1 contract");
    assert_eq!(
        decoded
            .operations()
            .iter()
            .map(|entry| entry.kind())
            .collect::<Vec<_>>(),
        AuthorizedExecutionContinuityOperationKind::all()
    );

    let reconstructed = AuthorizedExecutionContinuityStateContract::new(
        AuthorizedExecutionContinuityStateContractVersion::V1,
        decoded.operations().to_vec(),
    )
    .expect("legacy constructor remains compatible");
    assert_eq!(decoded, reconstructed);
    assert_eq!(
        serde_json::to_string(&decoded).expect("serialize"),
        serde_json::to_string(&reconstructed).expect("serialize")
    );
}

#[test]
fn contract_wire_rejects_unknown_fields_and_secret_like_enum_values_safely() {
    let operations = AuthorizedExecutionContinuityOperationKind::all()
        .iter()
        .copied()
        .map(|kind| {
            serde_json::json!({
                "kind": kind,
                "support": "supported"
            })
        })
        .collect::<Vec<_>>();
    for invalid in [
        serde_json::json!({
            "version": "token-secret-contract-version",
            "operations": operations.clone(),
        }),
        serde_json::json!({
            "version": "v1",
            "operations": operations.clone(),
            "authorization-secret-token": "unsupported-claim"
        }),
    ] {
        let error = serde_json::from_value::<AuthorizedExecutionContinuityStateContract>(invalid)
            .expect_err("invalid contract wire");
        assert!(!error.to_string().contains("unsupported-claim"));
        assert!(!error.to_string().contains("token-secret"));
    }

    let mut entry_unknown = serde_json::json!({
        "version": "v1",
        "operations": operations,
    });
    entry_unknown["operations"][0]["authorization-secret-token"] =
        serde_json::json!("unsupported-claim");
    let error = serde_json::from_value::<AuthorizedExecutionContinuityStateContract>(entry_unknown)
        .expect_err("unknown operation field");
    assert!(!error.to_string().contains("unsupported-claim"));
    assert!(!error.to_string().contains("authorization-secret-token"));

    let error = serde_json::from_value::<AuthorizedExecutionContinuityOperationKind>(
        serde_json::json!("token-secret-operation"),
    )
    .expect_err("unknown operation kind");
    assert!(!error.to_string().contains("token-secret-operation"));
}

#[test]
fn v2_support_requires_the_local_live_state_scope_and_complete_support() {
    let supported = AuthorizedExecutionContinuityOperationKind::all()
        .iter()
        .copied()
        .map(|kind| {
            AuthorizedExecutionContinuityOperationSupportEntry::new(
                kind,
                AuthorizedExecutionContinuityOperationSupport::Supported,
            )
        })
        .collect::<Vec<_>>();
    let contract = AuthorizedExecutionContinuityStateContractV2::new(
        AuthorizedExecutionContinuitySupportScope::LocalLiveStateOnly,
        supported.clone(),
    )
    .expect("valid scoped V2 contract");
    assert_eq!(
        contract.support_scope(),
        AuthorizedExecutionContinuitySupportScope::LocalLiveStateOnly
    );
    assert_eq!(
        contract.version(),
        AuthorizedExecutionContinuityStateContractV2Version::V2
    );
    let encoded = serde_json::to_string(&contract).expect("serialize V2 contract");
    let decoded: AuthorizedExecutionContinuityStateContractV2 =
        serde_json::from_str(&encoded).expect("deserialize V2 contract");
    assert_eq!(decoded, contract);

    let mut mixed = supported;
    mixed[0] = AuthorizedExecutionContinuityOperationSupportEntry::new(
        mixed[0].kind(),
        AuthorizedExecutionContinuityOperationSupport::Unsupported,
    );
    let error = AuthorizedExecutionContinuityStateContractV2::new(
        AuthorizedExecutionContinuitySupportScope::LocalLiveStateOnly,
        mixed,
    )
    .expect_err("mixed V2 support must fail");
    assert_eq!(
        error.code(),
        "authorized_execution_continuity_state.contract_v2.invalid"
    );
}

#[test]
fn all_existing_backends_explicitly_declare_continuity_unsupported() {
    let root = std::env::temp_dir().join(format!(
        "workflow-os-continuity-contract-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("temporary directory");
    let local = LocalStateBackend::new(root.join("local")).expect("local backend");
    let sqlite = SqliteStateBackend::open(root.join("state.sqlite3")).expect("SQLite backend");
    let postgres = PostgresStateBackend::new(Arc::new(UnexpectedConnectionFactory));

    for contract in [
        local
            .authorized_execution_continuity_state_contract()
            .expect("local contract"),
        sqlite
            .authorized_execution_continuity_state_contract()
            .expect("SQLite contract"),
        postgres
            .authorized_execution_continuity_state_contract()
            .expect("PostgreSQL contract"),
    ] {
        assert_eq!(
            contract.version(),
            AuthorizedExecutionContinuityStateContractVersion::V1
        );
        for kind in AuthorizedExecutionContinuityOperationKind::all() {
            assert_eq!(
                contract.support(*kind),
                AuthorizedExecutionContinuityOperationSupport::Unsupported
            );
        }
    }
    std::fs::remove_dir_all(root).expect("remove temporary directory");
}
