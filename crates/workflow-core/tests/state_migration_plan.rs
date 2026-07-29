#![allow(clippy::expect_used)]

//! Filesystem-to-SQLite migration plan model tests.

use std::collections::BTreeSet;

use workflow_core::{
    DurableStateBackendKind, StateMigrationDestinationId, StateMigrationDestinationPosture,
    StateMigrationDigest, StateMigrationDisposition, StateMigrationFindingCode,
    StateMigrationFindingSeverity, StateMigrationId, StateMigrationInventory, StateMigrationPlan,
    StateMigrationPlanVersion, StateMigrationRecordCount, StateMigrationRecordFamily,
    StateMigrationResumePolicy, StateMigrationVerificationRequirement,
};

fn inventory_with_digest(seed: char) -> StateMigrationInventory {
    let records = StateMigrationRecordFamily::all()
        .iter()
        .copied()
        .map(|family| {
            StateMigrationRecordCount::new(
                family,
                family.disposition(),
                0,
                Some(
                    StateMigrationDigest::new(seed.to_string().repeat(64))
                        .expect("valid family digest"),
                ),
            )
            .expect("valid family record")
        })
        .collect();
    StateMigrationInventory::new(records, Vec::new(), true).expect("compatible inventory")
}

fn plan() -> StateMigrationPlan {
    StateMigrationPlan::new(
        StateMigrationId::new("migration/local-preview-v1").expect("migration id"),
        &inventory_with_digest('a'),
        StateMigrationDestinationId::new("sqlite/staging-v1").expect("destination id"),
        1,
    )
    .expect("valid plan")
}

#[test]
fn valid_plan_binds_source_and_unreachable_sqlite_staging_destination() {
    let plan = plan();

    assert_eq!(plan.version(), StateMigrationPlanVersion::V1);
    assert_eq!(
        plan.source().backend_kind(),
        DurableStateBackendKind::LocalFilesystemPreview
    );
    assert!(plan.source().quiescence_required());
    assert_eq!(
        plan.destination().backend_kind(),
        DurableStateBackendKind::EmbeddedSqlite
    );
    assert_eq!(
        plan.destination().posture(),
        StateMigrationDestinationPosture::Staging
    );
    assert_eq!(plan.destination().adapter_schema_version(), 1);
    assert!(plan.destination().empty_required());
    assert!(!plan.destination().runtime_selectable());
}

#[test]
fn invalid_or_secret_like_identifiers_fail_without_leaking_values() {
    let secret = "ghp_secret_migration_token";
    let migration_error = StateMigrationId::new(secret).expect_err("secret-like migration id");
    let destination_error =
        StateMigrationDestinationId::new(secret).expect_err("secret-like destination id");
    let malformed_error = StateMigrationId::new("migration with spaces").expect_err("malformed");

    assert_eq!(migration_error.code(), "state.migration.plan.id.invalid");
    assert_eq!(
        destination_error.code(),
        "state.migration.destination.id.invalid"
    );
    assert!(!migration_error.to_string().contains(secret));
    assert!(!destination_error.to_string().contains(secret));
    assert!(!malformed_error
        .to_string()
        .contains("migration with spaces"));
}

#[test]
fn incompatible_inventory_and_zero_schema_version_fail_closed() {
    let records = StateMigrationRecordFamily::all()
        .iter()
        .copied()
        .map(|family| {
            StateMigrationRecordCount::new(
                family,
                family.disposition(),
                0,
                Some(StateMigrationDigest::new("c".repeat(64)).expect("digest")),
            )
            .expect("record")
        })
        .collect();
    let incompatible = StateMigrationInventory::new(
        records,
        vec![workflow_core::StateMigrationCompatibilityFinding::new(
            StateMigrationFindingSeverity::Blocker,
            StateMigrationFindingCode::SourceUnhealthy,
            None,
        )],
        true,
    )
    .expect("complete incompatible inventory");
    let source_error = StateMigrationPlan::new(
        StateMigrationId::new("migration/incompatible").expect("migration id"),
        &incompatible,
        StateMigrationDestinationId::new("sqlite/staging").expect("destination id"),
        1,
    )
    .expect_err("incompatible source");
    assert_eq!(source_error.code(), "state.migration.source.incompatible");

    let inventory = inventory_with_digest('b');
    let error = StateMigrationPlan::new(
        StateMigrationId::new("migration/v1").expect("migration id"),
        &inventory,
        StateMigrationDestinationId::new("sqlite/staging").expect("destination id"),
        0,
    )
    .expect_err("zero schema version");
    assert_eq!(
        error.code(),
        "state.migration.destination.schema_version.invalid"
    );
}

#[test]
fn every_family_appears_once_in_deterministic_dependency_order() {
    let plan = plan();
    let expected = [
        StateMigrationRecordFamily::WorkflowEvents,
        StateMigrationRecordFamily::EventIdIndexes,
        StateMigrationRecordFamily::RunSnapshots,
        StateMigrationRecordFamily::PendingApprovalProjections,
        StateMigrationRecordFamily::ApprovalPresentationRecords,
        StateMigrationRecordFamily::ApprovalPresentationIdIndexes,
        StateMigrationRecordFamily::IdempotencyResults,
        StateMigrationRecordFamily::ProjectStateRecords,
        StateMigrationRecordFamily::PolicyAuditRecords,
        StateMigrationRecordFamily::AdapterAuditRecords,
        StateMigrationRecordFamily::AdapterObservabilityRecords,
        StateMigrationRecordFamily::SideEffectRecords,
        StateMigrationRecordFamily::SideEffectIdIndexes,
        StateMigrationRecordFamily::WorkReportArtifacts,
        StateMigrationRecordFamily::LocalLocks,
        StateMigrationRecordFamily::ImmutableRunBundles,
    ];
    let actual = plan
        .steps()
        .iter()
        .map(|step| step.family())
        .collect::<Vec<_>>();
    let unique = actual.iter().copied().collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
    assert_eq!(unique.len(), StateMigrationRecordFamily::all().len());
    for (sequence, step) in (1_u16..).zip(plan.steps()) {
        assert_eq!(step.sequence(), sequence);
        assert_eq!(step.disposition(), step.family().disposition());
    }
}

#[test]
fn plan_distinguishes_import_rebuild_exclusion_and_companion_preservation() {
    let dispositions = plan()
        .steps()
        .iter()
        .map(|step| step.disposition())
        .collect::<BTreeSet<_>>();

    assert_eq!(
        dispositions,
        BTreeSet::from([
            StateMigrationDisposition::CanonicalImport,
            StateMigrationDisposition::ProjectionRebuild,
            StateMigrationDisposition::EphemeralExclude,
            StateMigrationDisposition::CompanionPreserve,
        ])
    );
}

#[test]
fn exact_resume_source_recheck_and_separate_activation_are_required() {
    let plan = plan();

    assert_eq!(
        plan.resume_policy(),
        StateMigrationResumePolicy::ExactPlanOnly
    );
    assert!(plan.source_recheck_required());
    assert!(plan.activation_separate());
}

#[test]
fn every_v1_verification_obligation_is_required_in_stable_order() {
    let plan = plan();

    assert_eq!(
        plan.verification_requirements(),
        StateMigrationVerificationRequirement::all()
    );
    assert!(plan
        .verification_requirements()
        .contains(&StateMigrationVerificationRequirement::NoLocksImported));
    assert!(plan
        .verification_requirements()
        .contains(&StateMigrationVerificationRequirement::SqliteSchemaAndQuickCheckHealthy));
}

#[test]
fn plan_fingerprint_changes_with_source_destination_or_schema_identity() {
    let baseline = plan();
    let changed_source = StateMigrationPlan::new(
        StateMigrationId::new("migration/local-preview-v1").expect("migration id"),
        &inventory_with_digest('b'),
        StateMigrationDestinationId::new("sqlite/staging-v1").expect("destination id"),
        1,
    )
    .expect("changed source plan");
    let changed_destination = StateMigrationPlan::new(
        StateMigrationId::new("migration/local-preview-v1").expect("migration id"),
        &inventory_with_digest('a'),
        StateMigrationDestinationId::new("sqlite/staging-v2").expect("destination id"),
        1,
    )
    .expect("changed destination plan");
    let changed_schema = StateMigrationPlan::new(
        StateMigrationId::new("migration/local-preview-v1").expect("migration id"),
        &inventory_with_digest('a'),
        StateMigrationDestinationId::new("sqlite/staging-v1").expect("destination id"),
        2,
    )
    .expect("changed schema plan");

    assert_ne!(
        baseline.plan_fingerprint(),
        changed_source.plan_fingerprint()
    );
    assert_ne!(
        baseline.plan_fingerprint(),
        changed_destination.plan_fingerprint()
    );
    assert_ne!(
        baseline.plan_fingerprint(),
        changed_schema.plan_fingerprint()
    );
}

#[test]
fn valid_plan_round_trips_through_serde() {
    let plan = plan();
    let serialized = serde_json::to_string(&plan).expect("serialize");
    let round_trip: StateMigrationPlan = serde_json::from_str(&serialized).expect("deserialize");

    assert_eq!(round_trip, plan);
}

#[test]
fn tampered_destination_and_plan_posture_fail_closed() {
    let serialized = serde_json::to_value(plan()).expect("serialize");

    for (pointer, replacement) in [
        (
            "/destination/runtime_selectable",
            serde_json::Value::Bool(true),
        ),
        (
            "/destination/empty_required",
            serde_json::Value::Bool(false),
        ),
        (
            "/steps/0/disposition",
            serde_json::Value::String("projection_rebuild".to_owned()),
        ),
        ("/source_recheck_required", serde_json::Value::Bool(false)),
        ("/activation_separate", serde_json::Value::Bool(false)),
    ] {
        let mut tampered = serialized.clone();
        *tampered.pointer_mut(pointer).expect("pointer") = replacement;
        serde_json::from_value::<StateMigrationPlan>(tampered).expect_err("tamper rejected");
    }
}

#[test]
fn missing_duplicate_or_reordered_plan_obligations_fail_closed() {
    let serialized = serde_json::to_value(plan()).expect("serialize");

    let mut missing_step = serialized.clone();
    missing_step["steps"].as_array_mut().expect("steps").pop();
    serde_json::from_value::<StateMigrationPlan>(missing_step).expect_err("missing step rejected");

    let mut duplicate_step = serialized.clone();
    let first = duplicate_step["steps"][0].clone();
    duplicate_step["steps"][1] = first;
    serde_json::from_value::<StateMigrationPlan>(duplicate_step)
        .expect_err("duplicate step rejected");

    let mut missing_verification = serialized;
    missing_verification["verification_requirements"]
        .as_array_mut()
        .expect("requirements")
        .pop();
    serde_json::from_value::<StateMigrationPlan>(missing_verification)
        .expect_err("missing verification rejected");
}

#[test]
fn invalid_serialized_secret_like_identity_fails_without_echoing_value() {
    let secret = "authorization_bearer_secret_value";
    let mut serialized = serde_json::to_value(plan()).expect("serialize");
    serialized["migration_id"] = serde_json::Value::String(secret.to_owned());

    let error = serde_json::from_value::<StateMigrationPlan>(serialized)
        .expect_err("secret identity rejected")
        .to_string();

    assert!(!error.contains(secret));
}

#[test]
fn debug_redacts_plan_id_destination_id_and_source_fingerprint() {
    let plan = plan();
    let debug = format!("{plan:?}");

    assert!(!debug.contains(plan.migration_id().as_str()));
    assert!(!debug.contains(plan.destination().destination_id().as_str()));
    assert!(!debug.contains(plan.source().source_fingerprint().as_str()));
    assert!(debug.contains("<redacted>"));
}

#[test]
fn serialized_plan_is_path_free_and_contains_no_forbidden_payload_fields() {
    let plan = plan();
    let serialized = serde_json::to_string(&plan).expect("serialize");

    for forbidden in [
        "source_path",
        "destination_path",
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
