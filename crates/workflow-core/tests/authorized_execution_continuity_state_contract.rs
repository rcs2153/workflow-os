#![allow(clippy::expect_used, clippy::panic)]
#![doc = "Public compatibility tests for authorized-execution continuity state support."]

use std::sync::Arc;

use postgres::Client;
use workflow_core::{
    AuthorizedExecutionContinuityOperationKind, AuthorizedExecutionContinuityOperationSupport,
    AuthorizedExecutionContinuityOperationSupportEntry, AuthorizedExecutionContinuityStateContract,
    AuthorizedExecutionContinuityStateContractProvider,
    AuthorizedExecutionContinuityStateContractVersion, LocalStateBackend,
    PostgresConnectionFactory, PostgresStateBackend, SqliteStateBackend, WorkflowOsError,
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
