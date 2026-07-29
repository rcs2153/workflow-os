#![allow(clippy::expect_used)]

//! Writer-guard capability and immutable migration-attempt model tests.

use workflow_core::{
    DurableStateBackendKind, StateMigrationAttempt, StateMigrationDestinationId,
    StateMigrationDigest, StateMigrationGuardProtocolVersion, StateMigrationId,
    StateMigrationImporterTransactionVersion, StateMigrationInventory, StateMigrationPlan,
    StateMigrationRecordCount, StateMigrationRecordFamily, StateMigrationWriterCompatibility,
    StateMigrationWriterCompatibilityPosture, StateMigrationWriterGuardAcquisitionOutcome,
    StateMigrationWriterGuardBoundary, StateMigrationWriterGuardCapability,
    StateMigrationWriterGuardMode, StateMigrationWriterGuardReleasePolicy,
    StateMigrationWriterProtocolVersion,
};

fn inventory(seed: char) -> StateMigrationInventory {
    let records = StateMigrationRecordFamily::all()
        .iter()
        .copied()
        .map(|family| {
            StateMigrationRecordCount::new(
                family,
                family.disposition(),
                0,
                Some(StateMigrationDigest::new(seed.to_string().repeat(64)).expect("valid digest")),
            )
            .expect("valid record")
        })
        .collect();
    StateMigrationInventory::new(records, Vec::new(), true).expect("compatible inventory")
}

fn plan(seed: char, schema_version: u32) -> StateMigrationPlan {
    StateMigrationPlan::new(
        StateMigrationId::new("migration/writer-guard-v1").expect("migration id"),
        &inventory(seed),
        StateMigrationDestinationId::new("sqlite/writer-guard-staging").expect("destination id"),
        schema_version,
    )
    .expect("valid plan")
}

fn capability() -> StateMigrationWriterGuardCapability {
    StateMigrationWriterGuardCapability::local_filesystem_v1()
}

fn compatible_writer() -> StateMigrationWriterCompatibility {
    StateMigrationWriterCompatibility::assess(
        DurableStateBackendKind::LocalFilesystemPreview,
        Some(StateMigrationWriterProtocolVersion::V1),
        &capability(),
        true,
    )
}

fn attempt(seed: char, schema_version: u32) -> StateMigrationAttempt {
    StateMigrationAttempt::new(
        &plan(seed, schema_version),
        &capability(),
        &compatible_writer(),
        StateMigrationImporterTransactionVersion::V1,
    )
    .expect("valid attempt")
}

#[test]
fn v1_capability_is_local_cooperative_cross_process_contract_only() {
    let capability = capability();

    assert_eq!(
        capability.source_backend(),
        DurableStateBackendKind::LocalFilesystemPreview
    );
    assert_eq!(
        capability.writer_protocol_version(),
        StateMigrationWriterProtocolVersion::V1
    );
    assert_eq!(
        capability.guard_protocol_version(),
        StateMigrationGuardProtocolVersion::V1
    );
    assert_eq!(
        capability.supported_modes(),
        StateMigrationWriterGuardMode::all()
    );
    assert!(capability.local_only());
    assert!(capability.cooperating_writers_only());
    assert!(capability.cross_process_required());
    assert_eq!(
        capability.boundary(),
        StateMigrationWriterGuardBoundary::LocalCooperatingProcesses
    );
    assert_eq!(
        capability.release_policy(),
        StateMigrationWriterGuardReleasePolicy::OnProcessExit
    );
}

#[test]
fn guard_modes_and_bounded_acquisition_outcomes_are_representable() {
    assert_eq!(
        StateMigrationWriterGuardMode::all(),
        &[
            StateMigrationWriterGuardMode::SharedWriter,
            StateMigrationWriterGuardMode::ExclusiveMigration,
        ]
    );
    let outcomes = [
        StateMigrationWriterGuardAcquisitionOutcome::Acquired,
        StateMigrationWriterGuardAcquisitionOutcome::Contended,
        StateMigrationWriterGuardAcquisitionOutcome::IncompatibleWriterProtocol,
        StateMigrationWriterGuardAcquisitionOutcome::Unavailable,
    ];
    assert_eq!(outcomes.len(), 4);
}

#[test]
fn compatibility_requires_exact_protocol_and_older_writer_assertion() {
    let capability = capability();
    let compatible = compatible_writer();
    let missing_marker = StateMigrationWriterCompatibility::assess(
        DurableStateBackendKind::LocalFilesystemPreview,
        None,
        &capability,
        true,
    );
    let missing_assertion = StateMigrationWriterCompatibility::assess(
        DurableStateBackendKind::LocalFilesystemPreview,
        Some(StateMigrationWriterProtocolVersion::V1),
        &capability,
        false,
    );
    let wrong_backend = StateMigrationWriterCompatibility::assess(
        DurableStateBackendKind::EmbeddedSqlite,
        Some(StateMigrationWriterProtocolVersion::V1),
        &capability,
        true,
    );

    assert_eq!(
        compatible.posture(),
        StateMigrationWriterCompatibilityPosture::Compatible
    );
    assert_eq!(
        missing_marker.posture(),
        StateMigrationWriterCompatibilityPosture::Unverified
    );
    assert_eq!(
        missing_assertion.posture(),
        StateMigrationWriterCompatibilityPosture::Unverified
    );
    assert_eq!(
        wrong_backend.posture(),
        StateMigrationWriterCompatibilityPosture::Incompatible
    );
}

#[test]
fn migration_attempt_binds_plan_source_destination_schema_and_protocols() {
    let plan = plan('a', 7);
    let attempt = StateMigrationAttempt::new(
        &plan,
        &capability(),
        &compatible_writer(),
        StateMigrationImporterTransactionVersion::V1,
    )
    .expect("valid attempt");

    assert_eq!(attempt.migration_id(), plan.migration_id());
    assert_eq!(attempt.plan_version(), plan.version());
    assert_eq!(attempt.plan_fingerprint(), plan.plan_fingerprint());
    assert_eq!(
        attempt.source_fingerprint(),
        plan.source().source_fingerprint()
    );
    assert_eq!(
        attempt.destination_id(),
        plan.destination().destination_id()
    );
    assert_eq!(attempt.adapter_schema_version(), 7);
    assert_eq!(
        attempt.writer_protocol_version(),
        StateMigrationWriterProtocolVersion::V1
    );
    assert_eq!(
        attempt.guard_protocol_version(),
        StateMigrationGuardProtocolVersion::V1
    );
    assert_eq!(
        attempt.importer_transaction_version(),
        StateMigrationImporterTransactionVersion::V1
    );
    assert_eq!(
        attempt.guard_mode(),
        StateMigrationWriterGuardMode::ExclusiveMigration
    );
}

#[test]
fn incompatible_or_unverified_writer_posture_rejects_attempt_without_leakage() {
    let plan = plan('a', 1);
    let unverified = StateMigrationWriterCompatibility::assess(
        DurableStateBackendKind::LocalFilesystemPreview,
        Some(StateMigrationWriterProtocolVersion::V1),
        &capability(),
        false,
    );
    let error = StateMigrationAttempt::new(
        &plan,
        &capability(),
        &unverified,
        StateMigrationImporterTransactionVersion::V1,
    )
    .expect_err("unverified writer rejected");

    assert_eq!(error.code(), "state.migration.writer.compatibility.invalid");
    assert!(!error.to_string().contains(plan.migration_id().as_str()));
    assert!(!error
        .to_string()
        .contains(plan.source().source_fingerprint().as_str()));
}

#[test]
fn attempt_fingerprint_is_deterministic_and_changes_with_bound_plan_facts() {
    let baseline = attempt('a', 1);
    let same = attempt('a', 1);
    let changed_source = attempt('b', 1);
    let changed_schema = attempt('a', 2);

    assert_eq!(baseline.attempt_fingerprint(), same.attempt_fingerprint());
    assert_ne!(
        baseline.attempt_fingerprint(),
        changed_source.attempt_fingerprint()
    );
    assert_ne!(
        baseline.attempt_fingerprint(),
        changed_schema.attempt_fingerprint()
    );
}

#[test]
fn capability_compatibility_and_attempt_round_trip_through_serde() {
    let capability = capability();
    let compatibility = compatible_writer();
    let attempt = attempt('a', 1);

    let capability_round_trip: StateMigrationWriterGuardCapability =
        serde_json::from_str(&serde_json::to_string(&capability).expect("serialize capability"))
            .expect("deserialize capability");
    let compatibility_round_trip: StateMigrationWriterCompatibility = serde_json::from_str(
        &serde_json::to_string(&compatibility).expect("serialize compatibility"),
    )
    .expect("deserialize compatibility");
    let attempt_round_trip: StateMigrationAttempt =
        serde_json::from_str(&serde_json::to_string(&attempt).expect("serialize attempt"))
            .expect("deserialize attempt");

    assert_eq!(capability_round_trip, capability);
    assert_eq!(compatibility_round_trip, compatibility);
    assert_eq!(attempt_round_trip, attempt);
}

#[test]
fn serialized_derived_posture_and_fingerprint_tampering_fails_closed() {
    let mut capability = serde_json::to_value(capability()).expect("serialize capability");
    capability["boundary"] = serde_json::Value::String("distributed_workers".to_owned());
    serde_json::from_value::<StateMigrationWriterGuardCapability>(capability)
        .expect_err("capability posture tamper rejected");

    let mut compatibility =
        serde_json::to_value(compatible_writer()).expect("serialize compatibility");
    compatibility["posture"] = serde_json::Value::String("unverified".to_owned());
    serde_json::from_value::<StateMigrationWriterCompatibility>(compatibility)
        .expect_err("compatibility posture tamper rejected");

    let mut attempt = serde_json::to_value(attempt('a', 1)).expect("serialize attempt");
    attempt["attempt_fingerprint"] = serde_json::Value::String("f".repeat(64));
    serde_json::from_value::<StateMigrationAttempt>(attempt)
        .expect_err("attempt fingerprint tamper rejected");
}

#[test]
fn invalid_serialized_protocol_and_guard_mode_fail_closed() {
    let serialized = serde_json::to_value(attempt('a', 1)).expect("serialize attempt");

    for (field, value) in [
        (
            "writer_protocol_version",
            serde_json::Value::String("v2".to_owned()),
        ),
        (
            "guard_protocol_version",
            serde_json::Value::String("v2".to_owned()),
        ),
        (
            "importer_transaction_version",
            serde_json::Value::String("v2".to_owned()),
        ),
        (
            "guard_mode",
            serde_json::Value::String("shared_writer".to_owned()),
        ),
    ] {
        let mut tampered = serialized.clone();
        tampered[field] = value;
        serde_json::from_value::<StateMigrationAttempt>(tampered)
            .expect_err("protocol or mode tamper rejected");
    }
}

#[test]
fn debug_redacts_all_attempt_identity_and_fingerprint_values() {
    let attempt = attempt('a', 1);
    let debug = format!("{attempt:?}");

    for secret_like_identity in [
        attempt.migration_id().as_str(),
        attempt.plan_fingerprint().as_str(),
        attempt.source_fingerprint().as_str(),
        attempt.destination_id().as_str(),
        attempt.attempt_fingerprint().as_str(),
    ] {
        assert!(!debug.contains(secret_like_identity));
    }
    assert!(debug.contains("<redacted>"));
}

#[test]
fn serialized_models_are_path_and_payload_free() {
    let serialized = serde_json::to_string(&(
        capability(),
        compatible_writer(),
        attempt('a', 1),
        StateMigrationWriterGuardAcquisitionOutcome::Contended,
    ))
    .expect("serialize models");

    for forbidden in [
        "source_path",
        "destination_path",
        "lock_path",
        "raw_payload",
        "command_output",
        "provider_payload",
        "environment_value",
        "credential",
        "authorization_header",
        "private_key",
    ] {
        assert!(!serialized.contains(forbidden));
    }
}
