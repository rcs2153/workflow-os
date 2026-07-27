#![allow(clippy::expect_used)]
//! Production current-authority source-boundary model tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use workflow_core::{
    build_immutable_run_bundle, load_project, ActorId, CurrentAuthorityFactFamily,
    CurrentAuthoritySourceCompleteness, CurrentAuthoritySourceConsistency,
    CurrentAuthoritySourceContractVersion, CurrentAuthoritySourceFactCount,
    CurrentAuthoritySourceFailure, CurrentAuthoritySourceFailureKind,
    CurrentAuthoritySourceFailurePosture, CurrentAuthoritySourceFreshness,
    CurrentAuthoritySourceGeneration, CurrentAuthoritySourceId, CurrentAuthoritySourceKind,
    CurrentAuthoritySourceReadWindow, CurrentAuthoritySourceRegistration,
    CurrentAuthoritySourceRegistrationInput, CurrentAuthoritySourceRequest,
    CurrentAuthoritySourceRequestInput, CurrentAuthoritySourceSnapshot,
    CurrentAuthoritySourceSnapshotId, CurrentAuthoritySourceSnapshotInput,
    CurrentAuthoritySourceWatermark, GovernedContextAccessLevel, GovernedContextReferenceTarget,
    HarnessContractId, HarnessContractVersion, ImmutableRunBundleBuildRequest,
    ImmutableRunBundleExecutionPosture, ImmutableRunBundleHandlerPosture,
    ImmutableRunBundleHandlerReference, ImmutableRunBundleId, ImmutableRunBundleReferencePosture,
    ImmutableRunBundleSensitivity, ImmutableRunBundleVersion, LocalImmutableRunBundleStore,
    RequiredContextContractBinding, RequiredContextExecutionBinding,
    RequiredContextExecutionBindingInput, RequiredContextObligation, RequiredContextRequirement,
    RequiredContextRequirementId, SkillId, SkillVersion, SpecContentHash, StepId, Timestamp,
    WorkReportId, WorkReportSensitivity, WorkflowId, WorkflowRunId, SUPPORTED_SCHEMA_VERSION,
};

static NEXT_ROOT: AtomicU64 = AtomicU64::new(1);

struct TestRoot(PathBuf);

impl TestRoot {
    fn new(name: &str) -> Self {
        let id = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "workflow-os-current-authority-source-{name}-{}-{id}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("test root");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn write(&self, relative: &str, content: &str) {
        let path = self.0.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent");
        }
        fs::write(path, content).expect("fixture");
    }
}

impl Drop for TestRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).expect("timestamp")
}

fn fixture() -> (
    workflow_core::StoredImmutableRunBundle,
    RequiredContextContractBinding,
    RequiredContextExecutionBinding,
) {
    let project_root = TestRoot::new("project");
    let store_root = TestRoot::new("store");
    project_root.write(
        "workflow-os.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nproject:\n  id: authority/project\n  name: Authority Project\n"
        ),
    );
    project_root.write(
        "workflows/build.workflow.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: authority/build\nversion: v1\ndisplay_name: Authority Build\ntriggers:\n  - id: manual\n    kind: manual\nsteps:\n  - id: consume\n    skill_ref:\n      id: local/check\n      version: v1\n    policy_requirements:\n      - id: local/read-only\n    terminal_behavior: fail_workflow\ncancellation_behavior: stop\naudit_requirements:\n  required: true\n  events: [RunCreated]\n  store_references_only: true\nobservability_requirements:\n  metrics: [workflow_latency]\n  tracing: true\n  latency_tracking: true\n"
        ),
    );
    project_root.write(
        "skills/check.skill.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/check\nversion: v1\ndisplay_name: Check\nallowed_capabilities:\n  - name: local.read\ninput_contract:\n  fields:\n    - name: request\n      field_type: string\noutput_contract:\n  fields:\n    - name: summary\n      field_type: string\nfailure_modes:\n  - code: failed\n    description: Failed.\n    retryable: false\naudit_requirements:\n  required: true\n  events: [SkillInvocationRequested]\n  store_references_only: true\nobservability_requirements:\n  metrics: [skill_latency]\n  tracing: true\n  latency_tracking: true\n"
        ),
    );
    project_root.write(
        "policies/read-only.policy.yml",
        &format!(
            "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/read-only\nname: Read only\nrules:\n  - id: allow\n    effect: allow_local\n"
        ),
    );
    let loaded = load_project(project_root.path());
    assert!(!loaded.has_errors(), "{:?}", loaded.diagnostics);
    let project = loaded.bundle.expect("project");
    let built = build_immutable_run_bundle(ImmutableRunBundleBuildRequest {
        project: &project,
        workflow_id: &WorkflowId::new("authority/build").expect("workflow"),
        bundle_id: ImmutableRunBundleId::new("bundle/authority").expect("bundle"),
        bundle_version: ImmutableRunBundleVersion::new("v1").expect("version"),
        run_id: WorkflowRunId::new("run-authority").expect("run"),
        resolved_execution_context_hash: SpecContentHash::from_text("context"),
        execution_posture: ImmutableRunBundleExecutionPosture::new(
            Vec::new(),
            ImmutableRunBundleReferencePosture::NotSupplied,
            ImmutableRunBundleReferencePosture::NotSupplied,
            ImmutableRunBundleReferencePosture::NotSupplied,
        )
        .expect("posture"),
        handlers: vec![ImmutableRunBundleHandlerReference {
            skill_id: SkillId::new("local/check").expect("skill"),
            skill_version: SkillVersion::new("v1").expect("skill version"),
            posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
        }],
        created_at: timestamp("2026-07-26T10:00:00Z"),
        created_by: ActorId::new("system/kernel").expect("actor"),
        sensitivity: ImmutableRunBundleSensitivity::Internal,
        redaction_required: true,
    })
    .expect("built");
    let store = LocalImmutableRunBundleStore::new(store_root.path());
    store.write_bundle(&built).expect("write");
    let stored = store
        .read_bundle(built.manifest().run_id(), built.manifest().bundle_id())
        .expect("read");
    let contract = RequiredContextContractBinding::new(
        HarnessContractId::new("harness/context").expect("contract"),
        HarnessContractVersion::new("v1").expect("contract version"),
        vec![
            RequiredContextRequirement::new(
                RequiredContextRequirementId::new("required/report-reference")
                    .expect("requirement"),
                GovernedContextReferenceTarget::WorkReport(
                    WorkReportId::new("report/current").expect("report"),
                ),
                GovernedContextAccessLevel::ReferenceOnly,
                RequiredContextObligation::Required,
                WorkReportSensitivity::Internal,
            )
            .expect("requirement"),
            RequiredContextRequirement::new(
                RequiredContextRequirementId::new("required/report-metadata").expect("requirement"),
                GovernedContextReferenceTarget::WorkReport(
                    WorkReportId::new("report/metadata").expect("report"),
                ),
                GovernedContextAccessLevel::BoundedMetadata,
                RequiredContextObligation::Required,
                WorkReportSensitivity::Internal,
            )
            .expect("requirement"),
        ],
    )
    .expect("contract");
    let binding = RequiredContextExecutionBinding::new(RequiredContextExecutionBindingInput {
        bundle: &stored,
        contract: &contract,
        actor: ActorId::new("agent/consumer").expect("actor"),
        step_id: StepId::new("consume").expect("step"),
        maximum_sensitivity: WorkReportSensitivity::Internal,
        bound_at: timestamp("2026-07-26T10:10:00Z"),
    })
    .expect("binding");
    (stored, contract, binding)
}

fn families() -> Vec<CurrentAuthorityFactFamily> {
    vec![
        CurrentAuthorityFactFamily::GovernedContextReferences,
        CurrentAuthorityFactFamily::CapabilityGrants,
        CurrentAuthorityFactFamily::CapabilityAvailability,
    ]
}

fn registration() -> CurrentAuthoritySourceRegistration {
    CurrentAuthoritySourceRegistration::new(CurrentAuthoritySourceRegistrationInput {
        source_id: CurrentAuthoritySourceId::new("authority/local").expect("source"),
        contract_version: CurrentAuthoritySourceContractVersion::new("v1").expect("version"),
        source_kind: CurrentAuthoritySourceKind::LocalAggregate,
        configuration_commitment: SpecContentHash::from_text("safe-normalized-configuration"),
        supported_fact_families: families(),
        consistency: CurrentAuthoritySourceConsistency::AtomicSnapshot,
        core_maximum_observation_age_seconds: 600,
        sensitivity: WorkReportSensitivity::Internal,
        redaction_required: true,
    })
    .expect("registration")
}

fn request<'a>(
    registration: &'a CurrentAuthoritySourceRegistration,
    binding: &'a RequiredContextExecutionBinding,
    contract: &'a RequiredContextContractBinding,
) -> CurrentAuthoritySourceRequest {
    CurrentAuthoritySourceRequest::new(CurrentAuthoritySourceRequestInput {
        registration,
        execution_binding: binding,
        contract,
        requested_fact_families: families(),
        evaluated_at: timestamp("2026-07-26T10:25:00Z"),
    })
    .expect("request")
}

fn snapshot(
    request: &CurrentAuthoritySourceRequest,
    registration: &CurrentAuthoritySourceRegistration,
    observed_at: Timestamp,
    source_valid_through: Option<Timestamp>,
    completeness: CurrentAuthoritySourceCompleteness,
    returned_fact_families: Vec<CurrentAuthorityFactFamily>,
) -> Result<CurrentAuthoritySourceSnapshot, workflow_core::WorkflowOsError> {
    let fact_counts = returned_fact_families
        .iter()
        .copied()
        .map(|family| {
            let count = match family {
                CurrentAuthorityFactFamily::CapabilityGrants => 0,
                CurrentAuthorityFactFamily::CapabilityAvailability
                | CurrentAuthorityFactFamily::GovernedContextReferences => 2,
            };
            CurrentAuthoritySourceFactCount::new(family, count)
        })
        .collect();
    CurrentAuthoritySourceSnapshot::new(CurrentAuthoritySourceSnapshotInput {
        request,
        registration,
        snapshot_id: CurrentAuthoritySourceSnapshotId::new("snapshot/current").expect("snapshot"),
        watermark: CurrentAuthoritySourceWatermark::new("watermark/current").expect("watermark"),
        generation: Some(CurrentAuthoritySourceGeneration::new(7).expect("generation")),
        read_window: CurrentAuthoritySourceReadWindow::new(observed_at, observed_at, observed_at)
            .expect("window"),
        completeness,
        consistency: CurrentAuthoritySourceConsistency::AtomicSnapshot,
        source_valid_through,
        returned_fact_families,
        fact_counts,
        records_commitment: SpecContentHash::from_text("bounded-records"),
    })
}

#[test]
fn valid_registration_request_and_snapshot_round_trip() {
    let (_bundle, contract, binding) = fixture();
    let registration = registration();
    let request = request(&registration, &binding, &contract);
    let snapshot = snapshot(
        &request,
        &registration,
        timestamp("2026-07-26T10:20:00Z"),
        Some(timestamp("2026-07-26T10:28:00Z")),
        CurrentAuthoritySourceCompleteness::CompleteForExactQuery,
        families(),
    )
    .expect("snapshot");

    assert_eq!(request.query_count(), 2);
    assert_eq!(
        snapshot.completeness(),
        CurrentAuthoritySourceCompleteness::CompleteForExactQuery
    );
    assert_eq!(snapshot.freshness(), CurrentAuthoritySourceFreshness::Fresh);
    assert_eq!(snapshot.generation().expect("generation").get(), 7);

    let registration_wire = serde_json::to_string(&registration).expect("serialize registration");
    let request_wire = serde_json::to_string(&request).expect("serialize request");
    let snapshot_wire = serde_json::to_string(&snapshot).expect("serialize snapshot");
    assert_eq!(
        serde_json::from_str::<CurrentAuthoritySourceRegistration>(&registration_wire)
            .expect("registration round trip"),
        registration
    );
    assert_eq!(
        serde_json::from_str::<CurrentAuthoritySourceRequest>(&request_wire)
            .expect("request round trip"),
        request
    );
    assert_eq!(
        serde_json::from_str::<CurrentAuthoritySourceSnapshot>(&snapshot_wire)
            .expect("snapshot round trip"),
        snapshot
    );
}

#[test]
fn registration_canonicalizes_families_and_rejects_invalid_posture() {
    let registration = registration();
    assert_eq!(
        registration.supported_fact_families(),
        &[
            CurrentAuthorityFactFamily::CapabilityGrants,
            CurrentAuthorityFactFamily::CapabilityAvailability,
            CurrentAuthorityFactFamily::GovernedContextReferences,
        ]
    );
    let canonical_equivalent =
        CurrentAuthoritySourceRegistration::new(CurrentAuthoritySourceRegistrationInput {
            source_id: CurrentAuthoritySourceId::new("authority/local").expect("source"),
            contract_version: CurrentAuthoritySourceContractVersion::new("v1").expect("version"),
            source_kind: CurrentAuthoritySourceKind::LocalAggregate,
            configuration_commitment: SpecContentHash::from_text("safe-normalized-configuration"),
            supported_fact_families: vec![
                CurrentAuthorityFactFamily::CapabilityAvailability,
                CurrentAuthorityFactFamily::CapabilityGrants,
                CurrentAuthorityFactFamily::GovernedContextReferences,
            ],
            consistency: CurrentAuthoritySourceConsistency::AtomicSnapshot,
            core_maximum_observation_age_seconds: 600,
            sensitivity: WorkReportSensitivity::Internal,
            redaction_required: true,
        })
        .expect("canonical equivalent");
    assert_eq!(
        canonical_equivalent.registration_commitment(),
        registration.registration_commitment()
    );

    let duplicate =
        CurrentAuthoritySourceRegistration::new(CurrentAuthoritySourceRegistrationInput {
            source_id: CurrentAuthoritySourceId::new("authority/local").expect("source"),
            contract_version: CurrentAuthoritySourceContractVersion::new("v1").expect("version"),
            source_kind: CurrentAuthoritySourceKind::LocalAggregate,
            configuration_commitment: SpecContentHash::from_text("config"),
            supported_fact_families: vec![
                CurrentAuthorityFactFamily::CapabilityGrants,
                CurrentAuthorityFactFamily::CapabilityGrants,
            ],
            consistency: CurrentAuthoritySourceConsistency::AtomicSnapshot,
            core_maximum_observation_age_seconds: 600,
            sensitivity: WorkReportSensitivity::Internal,
            redaction_required: true,
        })
        .expect_err("duplicate");
    assert_eq!(
        duplicate.code(),
        "current_authority.source.fact_family.duplicate"
    );

    let unredacted =
        CurrentAuthoritySourceRegistration::new(CurrentAuthoritySourceRegistrationInput {
            source_id: CurrentAuthoritySourceId::new("authority/unredacted").expect("source"),
            contract_version: CurrentAuthoritySourceContractVersion::new("v1").expect("version"),
            source_kind: CurrentAuthoritySourceKind::LocalAggregate,
            configuration_commitment: SpecContentHash::from_text("config"),
            supported_fact_families: families(),
            consistency: CurrentAuthoritySourceConsistency::AtomicSnapshot,
            core_maximum_observation_age_seconds: 600,
            sensitivity: WorkReportSensitivity::Internal,
            redaction_required: false,
        })
        .expect_err("unredacted registration");
    assert_eq!(
        unredacted.code(),
        "current_authority.source.registration.redaction_required"
    );
}

#[test]
fn request_rejects_unsupported_family_and_contract_substitution() {
    let (_bundle, contract, binding) = fixture();
    let limited =
        CurrentAuthoritySourceRegistration::new(CurrentAuthoritySourceRegistrationInput {
            source_id: CurrentAuthoritySourceId::new("authority/limited").expect("source"),
            contract_version: CurrentAuthoritySourceContractVersion::new("v1").expect("version"),
            source_kind: CurrentAuthoritySourceKind::LocalAggregate,
            configuration_commitment: SpecContentHash::from_text("config"),
            supported_fact_families: vec![CurrentAuthorityFactFamily::CapabilityGrants],
            consistency: CurrentAuthoritySourceConsistency::AtomicSnapshot,
            core_maximum_observation_age_seconds: 600,
            sensitivity: WorkReportSensitivity::Internal,
            redaction_required: true,
        })
        .expect("registration");
    let unsupported = CurrentAuthoritySourceRequest::new(CurrentAuthoritySourceRequestInput {
        registration: &limited,
        execution_binding: &binding,
        contract: &contract,
        requested_fact_families: families(),
        evaluated_at: timestamp("2026-07-26T10:25:00Z"),
    })
    .expect_err("unsupported");
    assert_eq!(
        unsupported.code(),
        "current_authority.source.request.family_unsupported"
    );

    let public_only =
        CurrentAuthoritySourceRegistration::new(CurrentAuthoritySourceRegistrationInput {
            source_id: CurrentAuthoritySourceId::new("authority/public-only").expect("source"),
            contract_version: CurrentAuthoritySourceContractVersion::new("v1").expect("version"),
            source_kind: CurrentAuthoritySourceKind::LocalAggregate,
            configuration_commitment: SpecContentHash::from_text("config"),
            supported_fact_families: families(),
            consistency: CurrentAuthoritySourceConsistency::AtomicSnapshot,
            core_maximum_observation_age_seconds: 600,
            sensitivity: WorkReportSensitivity::Public,
            redaction_required: true,
        })
        .expect("public-only registration");
    let sensitivity = CurrentAuthoritySourceRequest::new(CurrentAuthoritySourceRequestInput {
        registration: &public_only,
        execution_binding: &binding,
        contract: &contract,
        requested_fact_families: families(),
        evaluated_at: timestamp("2026-07-26T10:25:00Z"),
    })
    .expect_err("sensitivity exceeds source");
    assert_eq!(
        sensitivity.code(),
        "current_authority.source.request.sensitivity_exceeds_source"
    );

    let substituted = RequiredContextContractBinding::new(
        HarnessContractId::new("harness/substituted").expect("id"),
        HarnessContractVersion::new("v1").expect("version"),
        contract.requirements().to_vec(),
    )
    .expect("substituted");
    let mismatch = CurrentAuthoritySourceRequest::new(CurrentAuthoritySourceRequestInput {
        registration: &registration(),
        execution_binding: &binding,
        contract: &substituted,
        requested_fact_families: families(),
        evaluated_at: timestamp("2026-07-26T10:25:00Z"),
    })
    .expect_err("mismatch");
    assert_eq!(
        mismatch.code(),
        "current_authority.source.request.contract_mismatch"
    );
}

#[test]
fn complete_snapshot_requires_exact_family_and_target_coverage() {
    let (_bundle, contract, binding) = fixture();
    let registration = registration();
    let request = request(&registration, &binding, &contract);
    let missing_family = snapshot(
        &request,
        &registration,
        timestamp("2026-07-26T10:20:00Z"),
        None,
        CurrentAuthoritySourceCompleteness::CompleteForExactQuery,
        vec![CurrentAuthorityFactFamily::CapabilityGrants],
    )
    .expect_err("missing family");
    assert_eq!(
        missing_family.code(),
        "current_authority.source.snapshot.family_coverage_incomplete"
    );

    let incomplete = snapshot(
        &request,
        &registration,
        timestamp("2026-07-26T10:20:00Z"),
        None,
        CurrentAuthoritySourceCompleteness::Incomplete,
        families(),
    )
    .expect("explicit incomplete snapshot");
    assert_eq!(
        incomplete.completeness(),
        CurrentAuthoritySourceCompleteness::Incomplete
    );

    for posture in [
        CurrentAuthoritySourceCompleteness::Unsupported,
        CurrentAuthoritySourceCompleteness::Unavailable,
        CurrentAuthoritySourceCompleteness::Unknown,
    ] {
        let bounded = snapshot(
            &request,
            &registration,
            timestamp("2026-07-26T10:20:00Z"),
            None,
            posture,
            Vec::new(),
        )
        .expect("bounded non-complete posture");
        assert_eq!(bounded.completeness(), posture);
    }
}

#[test]
fn stable_watermark_consistency_and_empty_grants_are_representable() {
    let (_bundle, contract, binding) = fixture();
    let registration =
        CurrentAuthoritySourceRegistration::new(CurrentAuthoritySourceRegistrationInput {
            source_id: CurrentAuthoritySourceId::new("authority/stable").expect("source"),
            contract_version: CurrentAuthoritySourceContractVersion::new("v1").expect("version"),
            source_kind: CurrentAuthoritySourceKind::LocalAggregate,
            configuration_commitment: SpecContentHash::from_text("stable-config"),
            supported_fact_families: families(),
            consistency: CurrentAuthoritySourceConsistency::StableWatermark,
            core_maximum_observation_age_seconds: 600,
            sensitivity: WorkReportSensitivity::Internal,
            redaction_required: true,
        })
        .expect("registration");
    let request = request(&registration, &binding, &contract);
    let snapshot = CurrentAuthoritySourceSnapshot::new(CurrentAuthoritySourceSnapshotInput {
        request: &request,
        registration: &registration,
        snapshot_id: CurrentAuthoritySourceSnapshotId::new("snapshot/stable").expect("snapshot"),
        watermark: CurrentAuthoritySourceWatermark::new("watermark/stable").expect("watermark"),
        generation: None,
        read_window: CurrentAuthoritySourceReadWindow::new(
            timestamp("2026-07-26T10:20:00Z"),
            timestamp("2026-07-26T10:20:00Z"),
            timestamp("2026-07-26T10:20:00Z"),
        )
        .expect("window"),
        completeness: CurrentAuthoritySourceCompleteness::CompleteForExactQuery,
        consistency: CurrentAuthoritySourceConsistency::StableWatermark,
        source_valid_through: None,
        returned_fact_families: families(),
        fact_counts: vec![
            CurrentAuthoritySourceFactCount::new(CurrentAuthorityFactFamily::CapabilityGrants, 0),
            CurrentAuthoritySourceFactCount::new(
                CurrentAuthorityFactFamily::CapabilityAvailability,
                request.query_count(),
            ),
            CurrentAuthoritySourceFactCount::new(
                CurrentAuthorityFactFamily::GovernedContextReferences,
                request.query_count(),
            ),
        ],
        records_commitment: SpecContentHash::from_text("empty-grants-complete"),
    })
    .expect("stable snapshot");

    assert_eq!(
        snapshot.completeness(),
        CurrentAuthoritySourceCompleteness::CompleteForExactQuery
    );
    assert_eq!(snapshot.generation(), None);
}

#[test]
fn freshness_uses_stricter_source_and_core_bounds() {
    let (_bundle, contract, binding) = fixture();
    let registration = registration();
    let request = request(&registration, &binding, &contract);
    let source_expired = snapshot(
        &request,
        &registration,
        timestamp("2026-07-26T10:20:00Z"),
        Some(timestamp("2026-07-26T10:24:00Z")),
        CurrentAuthoritySourceCompleteness::CompleteForExactQuery,
        families(),
    )
    .expect("stale snapshot");
    assert_eq!(
        source_expired.freshness(),
        CurrentAuthoritySourceFreshness::Stale
    );

    let future = snapshot(
        &request,
        &registration,
        timestamp("2026-07-26T10:26:00Z"),
        None,
        CurrentAuthoritySourceCompleteness::CompleteForExactQuery,
        families(),
    )
    .expect("future snapshot vocabulary");
    assert_eq!(
        future.freshness(),
        CurrentAuthoritySourceFreshness::FutureDated
    );
}

#[test]
fn snapshot_rejects_registration_and_consistency_substitution() {
    let (_bundle, contract, binding) = fixture();
    let registration = registration();
    let request = request(&registration, &binding, &contract);
    let other = CurrentAuthoritySourceRegistration::new(CurrentAuthoritySourceRegistrationInput {
        source_id: CurrentAuthoritySourceId::new("authority/other").expect("source"),
        contract_version: CurrentAuthoritySourceContractVersion::new("v1").expect("version"),
        source_kind: CurrentAuthoritySourceKind::LocalAggregate,
        configuration_commitment: SpecContentHash::from_text("other"),
        supported_fact_families: families(),
        consistency: CurrentAuthoritySourceConsistency::AtomicSnapshot,
        core_maximum_observation_age_seconds: 600,
        sensitivity: WorkReportSensitivity::Internal,
        redaction_required: true,
    })
    .expect("other");
    let mismatch = snapshot(
        &request,
        &other,
        timestamp("2026-07-26T10:20:00Z"),
        None,
        CurrentAuthoritySourceCompleteness::CompleteForExactQuery,
        families(),
    )
    .expect_err("registration mismatch");
    assert_eq!(
        mismatch.code(),
        "current_authority.source.snapshot.registration_mismatch"
    );

    let consistency = CurrentAuthoritySourceSnapshot::new(CurrentAuthoritySourceSnapshotInput {
        request: &request,
        registration: &registration,
        snapshot_id: CurrentAuthoritySourceSnapshotId::new("snapshot/current").expect("snapshot"),
        watermark: CurrentAuthoritySourceWatermark::new("watermark/current").expect("watermark"),
        generation: None,
        read_window: CurrentAuthoritySourceReadWindow::new(
            timestamp("2026-07-26T10:20:00Z"),
            timestamp("2026-07-26T10:20:00Z"),
            timestamp("2026-07-26T10:20:00Z"),
        )
        .expect("window"),
        completeness: CurrentAuthoritySourceCompleteness::Incomplete,
        consistency: CurrentAuthoritySourceConsistency::StableWatermark,
        source_valid_through: None,
        returned_fact_families: families(),
        fact_counts: vec![
            CurrentAuthoritySourceFactCount::new(CurrentAuthorityFactFamily::CapabilityGrants, 0),
            CurrentAuthoritySourceFactCount::new(
                CurrentAuthorityFactFamily::CapabilityAvailability,
                2,
            ),
            CurrentAuthoritySourceFactCount::new(
                CurrentAuthorityFactFamily::GovernedContextReferences,
                2,
            ),
        ],
        records_commitment: SpecContentHash::from_text("records"),
    })
    .expect_err("consistency mismatch");
    assert_eq!(
        consistency.code(),
        "current_authority.source.snapshot.consistency_mismatch"
    );
}

#[test]
fn wire_tampering_fails_closed_without_leaking_values() {
    let (_bundle, contract, binding) = fixture();
    let registration = registration();
    let request = request(&registration, &binding, &contract);
    let snapshot = snapshot(
        &request,
        &registration,
        timestamp("2026-07-26T10:20:00Z"),
        None,
        CurrentAuthoritySourceCompleteness::CompleteForExactQuery,
        families(),
    )
    .expect("snapshot");
    let secret = "token-super-sensitive";
    let mut wire = serde_json::to_value(&snapshot).expect("wire");
    wire["snapshot_commitment"] = serde_json::Value::String(secret.to_owned());
    let error = serde_json::from_value::<CurrentAuthoritySourceSnapshot>(wire).expect_err("tamper");

    assert!(!error.to_string().contains(secret));
}

#[test]
fn debug_and_serialization_remain_payload_free() {
    let (_bundle, contract, binding) = fixture();
    let registration = registration();
    let request = request(&registration, &binding, &contract);
    let snapshot = snapshot(
        &request,
        &registration,
        timestamp("2026-07-26T10:20:00Z"),
        None,
        CurrentAuthoritySourceCompleteness::CompleteForExactQuery,
        families(),
    )
    .expect("snapshot");
    let debug = format!("{snapshot:?}");
    assert!(!debug.contains("snapshot/current"));
    assert!(!debug.contains("watermark/current"));

    let serialized = serde_json::to_string(&snapshot).expect("serialize");
    for forbidden in [
        "provider_payload",
        "command_output",
        "raw_spec",
        "environment_value",
        "authorization_header",
        "credential",
        "private_key",
    ] {
        assert!(!serialized.contains(forbidden));
    }
}

#[test]
fn failure_vocabulary_is_payload_free_and_round_trips() {
    for kind in [
        CurrentAuthoritySourceFailureKind::Unavailable,
        CurrentAuthoritySourceFailureKind::Unsupported,
        CurrentAuthoritySourceFailureKind::Incomplete,
        CurrentAuthoritySourceFailureKind::Stale,
        CurrentAuthoritySourceFailureKind::FutureDated,
        CurrentAuthoritySourceFailureKind::ConcurrentChange,
        CurrentAuthoritySourceFailureKind::Ambiguous,
        CurrentAuthoritySourceFailureKind::Corrupt,
        CurrentAuthoritySourceFailureKind::RegistrationMismatch,
        CurrentAuthoritySourceFailureKind::QueryMismatch,
        CurrentAuthoritySourceFailureKind::Transport,
        CurrentAuthoritySourceFailureKind::Internal,
    ] {
        let failure = CurrentAuthoritySourceFailure::new(
            SpecContentHash::from_text("registration"),
            SpecContentHash::from_text("request"),
            kind,
            CurrentAuthoritySourceFailurePosture::Terminal,
        );
        let wire = serde_json::to_string(&failure).expect("serialize");
        let decoded: CurrentAuthoritySourceFailure =
            serde_json::from_str(&wire).expect("deserialize");
        assert_eq!(decoded, failure);
    }
}
