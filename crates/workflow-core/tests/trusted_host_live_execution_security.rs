#![allow(clippy::expect_used)]

//! Behavioral tests for the non-authorizing trusted-host live-security model.

use workflow_core::{
    assess_trusted_host_live_execution_security, BuildTimeInputCompletenessPosture,
    BuildTimeInputDiscoveryMethod, BuildTimeInputKind, BuildTimeInputKindCount,
    BuildTimeInputManifest, BuildTimeInputManifestDefinition, HostedExecutionCleanupPosture,
    HostedExecutionControlPosture, HostedExecutionEnforcementMode,
    HostedExecutionObservationSummary, HostedExecutionProviderId, HostedExecutionProviderVersion,
    HostedExecutionReference, HostedExecutionReferenceKind, SpecContentHash,
    TrustedHostAttemptStartPosture, TrustedHostLiveExecutionBinding,
    TrustedHostLiveExecutionSecurityAssessment, TrustedHostLiveExecutionSecurityDisposition,
    TrustedHostLiveExecutionSecurityObservations,
    TrustedHostLiveExecutionSecurityObservationsDefinition,
    TrustedHostLiveExecutionSecurityRequirements,
    TrustedHostLiveExecutionSecurityRequirementsDefinition, TrustedHostObservationPosture,
    TrustedHostProcessTreeObservation, TrustedHostProcessTreeTerminality,
    TrustedHostResourceBudgetObservation, TrustedHostRootAccess, TrustedHostRootBindingRequirement,
    TrustedHostRootObservation, TrustedHostRootObservationDefinition,
    TrustedHostRootObservationMethod, TrustedHostRootRequirement, TrustedHostRootRole,
    TrustedHostSecurityAffirmation, TrustedHostTransientWriteObservation,
    TrustedHostTransientWriteObservationDefinition, TrustedHostTransientWriteObservationPosture,
};

fn hash(label: &str) -> SpecContentHash {
    SpecContentHash::from_text(label)
}

fn reference(kind: HostedExecutionReferenceKind, value: &str) -> HostedExecutionReference {
    HostedExecutionReference::new(kind, value).expect("reference")
}

fn binding() -> TrustedHostLiveExecutionBinding {
    TrustedHostLiveExecutionBinding::new(
        hash("run-bundle"),
        hash("command-contract"),
        hash("attempt"),
        hash("request"),
        hash("provider-selection"),
    )
}

fn root_requirements() -> Vec<TrustedHostRootRequirement> {
    use TrustedHostRootRole::{
        BuildOutputWrite, CacheWrite, DependencyInput, RepositoryInput, TemporaryWrite,
        ToolchainInput,
    };
    vec![
        TrustedHostRootRequirement::new(
            RepositoryInput,
            TrustedHostRootAccess::ReadOnly,
            TrustedHostRootBindingRequirement::ProviderImmutableProjection,
            false,
        ),
        TrustedHostRootRequirement::new(
            ToolchainInput,
            TrustedHostRootAccess::ReadExecute,
            TrustedHostRootBindingRequirement::AdmissionToTerminal,
            false,
        ),
        TrustedHostRootRequirement::new(
            DependencyInput,
            TrustedHostRootAccess::ReadOnly,
            TrustedHostRootBindingRequirement::ProviderImmutableProjection,
            false,
        ),
        TrustedHostRootRequirement::new(
            CacheWrite,
            TrustedHostRootAccess::ReadWrite,
            TrustedHostRootBindingRequirement::AdmissionToTerminal,
            true,
        ),
        TrustedHostRootRequirement::new(
            BuildOutputWrite,
            TrustedHostRootAccess::ReadWrite,
            TrustedHostRootBindingRequirement::AdmissionToTerminal,
            true,
        ),
        TrustedHostRootRequirement::new(
            TemporaryWrite,
            TrustedHostRootAccess::ReadWrite,
            TrustedHostRootBindingRequirement::AdmissionToTerminal,
            true,
        ),
    ]
}

fn requirements() -> TrustedHostLiveExecutionSecurityRequirements {
    TrustedHostLiveExecutionSecurityRequirements::new(
        TrustedHostLiveExecutionSecurityRequirementsDefinition {
            binding: binding(),
            requested_policy_hash: hash("policy"),
            root_requirements: root_requirements(),
            denied_egress_probe_required: false,
        },
    )
    .expect("requirements")
}

fn input_manifest(completeness: BuildTimeInputCompletenessPosture) -> BuildTimeInputManifest {
    BuildTimeInputManifest::new(BuildTimeInputManifestDefinition {
        fingerprint: hash("input-manifest"),
        total_count: 3,
        kind_counts: vec![
            BuildTimeInputKindCount::new(BuildTimeInputKind::RepositoryContent, 1).expect("count"),
            BuildTimeInputKindCount::new(BuildTimeInputKind::ToolchainExecutable, 1)
                .expect("count"),
            BuildTimeInputKindCount::new(BuildTimeInputKind::DependencyPackage, 1).expect("count"),
        ],
        discovery_method: BuildTimeInputDiscoveryMethod::CoreAndProvider,
        completeness,
        observation_reference: Some(reference(
            HostedExecutionReferenceKind::Telemetry,
            "telemetry/input-manifest",
        )),
    })
    .expect("manifest")
}

fn root_observations() -> Vec<TrustedHostRootObservation> {
    root_requirements()
        .into_iter()
        .map(|required| {
            TrustedHostRootObservation::new(TrustedHostRootObservationDefinition {
                role: required.role(),
                access: required.access(),
                object_binding: TrustedHostObservationPosture::Proven,
                continuity: TrustedHostObservationPosture::Proven,
                method: match required.binding() {
                    TrustedHostRootBindingRequirement::AdmissionToTerminal => {
                        TrustedHostRootObservationMethod::ProviderNativeIdentity
                    }
                    TrustedHostRootBindingRequirement::ProviderImmutableProjection => {
                        TrustedHostRootObservationMethod::ImmutableProjection
                    }
                },
                invocation_scoped: required.invocation_scoped_creation(),
                commitment: hash(&format!("root-{:?}", required.role())),
                reference: reference(
                    HostedExecutionReferenceKind::Telemetry,
                    &format!("telemetry/root-{:?}", required.role()).to_ascii_lowercase(),
                ),
            })
            .expect("root observation")
        })
        .collect()
}

fn observations_definition(
    requirements: &TrustedHostLiveExecutionSecurityRequirements,
) -> TrustedHostLiveExecutionSecurityObservationsDefinition {
    TrustedHostLiveExecutionSecurityObservationsDefinition {
        binding: binding(),
        requirements_fingerprint: requirements.fingerprint().clone(),
        provider_id: HostedExecutionProviderId::new("provider/sandbox").expect("provider"),
        provider_version: HostedExecutionProviderVersion::new("v1").expect("version"),
        input_manifest: input_manifest(BuildTimeInputCompletenessPosture::Complete),
        root_observations: root_observations(),
        effective_policy_hash: hash("policy"),
        policy_revision_stable: TrustedHostSecurityAffirmation::Affirmed,
        enforcement_mode: HostedExecutionEnforcementMode::Enforce,
        filesystem_control: HostedExecutionControlPosture::Enforced,
        process_control: HostedExecutionControlPosture::Enforced,
        network_control: HostedExecutionControlPosture::Enforced,
        default_deny_filesystem: TrustedHostSecurityAffirmation::Affirmed,
        host_home_excluded: TrustedHostSecurityAffirmation::Affirmed,
        credential_store_excluded: TrustedHostSecurityAffirmation::Affirmed,
        ambient_temp_excluded: TrustedHostSecurityAffirmation::Affirmed,
        default_deny_network: TrustedHostSecurityAffirmation::Affirmed,
        denied_egress_probe_observed: TrustedHostSecurityAffirmation::NotAffirmed,
        observation_channel: TrustedHostObservationPosture::Proven,
        summary: HostedExecutionObservationSummary::new(
            0,
            0,
            1,
            1,
            0,
            0,
            reference(
                HostedExecutionReferenceKind::Telemetry,
                "telemetry/security-observations",
            ),
        )
        .expect("summary"),
        process_tree: TrustedHostProcessTreeObservation::new(
            TrustedHostAttemptStartPosture::Started,
            true,
            TrustedHostProcessTreeTerminality::Terminal,
            false,
            true,
        )
        .expect("process tree"),
        resource_budgets: TrustedHostResourceBudgetObservation {
            cpu: HostedExecutionControlPosture::Enforced,
            memory: HostedExecutionControlPosture::Enforced,
            process_count: HostedExecutionControlPosture::Enforced,
            file_descriptors: HostedExecutionControlPosture::Enforced,
            disk: HostedExecutionControlPosture::Enforced,
            wall_clock: HostedExecutionControlPosture::Enforced,
        },
        transient_writes: TrustedHostTransientWriteObservation::new(
            TrustedHostTransientWriteObservationDefinition {
                protected_roots: HostedExecutionControlPosture::Enforced,
                posture: TrustedHostTransientWriteObservationPosture::NoAttemptObserved,
                denied_attempt_count: 0,
                outputs_quiescent: true,
                retained_artifact_commitment: TrustedHostObservationPosture::Proven,
                reference: reference(
                    HostedExecutionReferenceKind::Telemetry,
                    "telemetry/transient-write",
                ),
            },
        )
        .expect("transient writes"),
        cleanup_posture: HostedExecutionCleanupPosture::Completed,
        cleanup_reference: reference(HostedExecutionReferenceKind::Telemetry, "telemetry/cleanup"),
    }
}

fn observations(
    requirements: &TrustedHostLiveExecutionSecurityRequirements,
) -> TrustedHostLiveExecutionSecurityObservations {
    TrustedHostLiveExecutionSecurityObservations::new(observations_definition(requirements))
        .expect("observations")
}

#[test]
fn exact_complete_observations_satisfy_without_granting_authority() {
    let requirements = requirements();
    let observations = observations(&requirements);

    let assessment = assess_trusted_host_live_execution_security(&requirements, &observations)
        .expect("assessment");

    assert_eq!(
        assessment.disposition(),
        TrustedHostLiveExecutionSecurityDisposition::Satisfied
    );
    assert!(assessment.is_satisfied());
    assert_eq!(assessment.reasons(), []);
    let serialized = serde_json::to_string(&assessment).expect("serialize");
    assert!(!serialized.contains("authorize"));
    assert!(!serialized.contains("runtime_image_digest"));
}

#[test]
fn incomplete_input_after_start_requires_reconciliation() {
    let requirements = requirements();
    let mut definition = observations_definition(&requirements);
    definition.input_manifest = input_manifest(BuildTimeInputCompletenessPosture::Incomplete);
    let observations =
        TrustedHostLiveExecutionSecurityObservations::new(definition).expect("observations");

    let assessment = assess_trusted_host_live_execution_security(&requirements, &observations)
        .expect("assessment");

    assert_eq!(
        assessment.disposition(),
        TrustedHostLiveExecutionSecurityDisposition::ReconciliationRequired
    );
}

#[test]
fn unmet_controls_with_affirmative_not_started_evidence_deny_before_start() {
    let requirements = requirements();
    let mut definition = observations_definition(&requirements);
    definition.filesystem_control = HostedExecutionControlPosture::Unavailable;
    definition.summary = HostedExecutionObservationSummary::new(
        0,
        0,
        0,
        0,
        0,
        0,
        reference(
            HostedExecutionReferenceKind::Telemetry,
            "telemetry/pre-start-denial",
        ),
    )
    .expect("summary");
    definition.process_tree = TrustedHostProcessTreeObservation::new(
        TrustedHostAttemptStartPosture::NotStarted,
        false,
        TrustedHostProcessTreeTerminality::NotObserved,
        false,
        false,
    )
    .expect("process tree");
    let observations =
        TrustedHostLiveExecutionSecurityObservations::new(definition).expect("observations");

    let assessment = assess_trusted_host_live_execution_security(&requirements, &observations)
        .expect("assessment");
    assert_eq!(
        assessment.disposition(),
        TrustedHostLiveExecutionSecurityDisposition::DeniedBeforeStart
    );
}

#[test]
fn requirement_or_provider_selection_substitution_fails_closed() {
    let requirements = requirements();
    let mut definition = observations_definition(&requirements);
    definition.requirements_fingerprint = hash("other-requirements");
    let error = assess_trusted_host_live_execution_security(
        &requirements,
        &TrustedHostLiveExecutionSecurityObservations::new(definition).expect("observations"),
    )
    .expect_err("substitution");
    assert_eq!(
        error.code(),
        "trusted_host.live_security.assessment.requirements.substituted"
    );

    let mut definition = observations_definition(&requirements);
    definition.binding = TrustedHostLiveExecutionBinding::new(
        hash("run-bundle"),
        hash("command-contract"),
        hash("attempt"),
        hash("request"),
        hash("different-provider-selection"),
    );
    let error = assess_trusted_host_live_execution_security(
        &requirements,
        &TrustedHostLiveExecutionSecurityObservations::new(definition).expect("observations"),
    )
    .expect_err("binding substitution");
    assert_eq!(
        error.code(),
        "trusted_host.live_security.assessment.binding.substituted"
    );
}

#[test]
fn duplicate_roots_and_incoherent_attempt_posture_fail_construction() {
    let mut roots = root_requirements();
    roots[1] = roots[0];
    let error = TrustedHostLiveExecutionSecurityRequirements::new(
        TrustedHostLiveExecutionSecurityRequirementsDefinition {
            binding: binding(),
            requested_policy_hash: hash("policy"),
            root_requirements: roots,
            denied_egress_probe_required: false,
        },
    )
    .expect_err("duplicate root");
    assert_eq!(
        error.code(),
        "trusted_host.live_security.requirements.root_binding.invalid"
    );

    let error = TrustedHostProcessTreeObservation::new(
        TrustedHostAttemptStartPosture::NotStarted,
        true,
        TrustedHostProcessTreeTerminality::Terminal,
        false,
        true,
    )
    .expect_err("incoherent process tree");
    assert_eq!(
        error.code(),
        "trusted_host.live_security.observation.process_tree.invalid"
    );
}

#[test]
fn serde_revalidates_bounds_and_debug_redacts_commitments() {
    let requirements = requirements();
    let observations = observations(&requirements);
    let serialized = serde_json::to_string(&observations).expect("serialize");
    let restored: TrustedHostLiveExecutionSecurityObservations =
        serde_json::from_str(&serialized).expect("deserialize");
    assert_eq!(observations, restored);

    let debug = format!("{observations:?}");
    assert!(!debug.contains(hash("policy").as_str()));
    assert!(!debug.contains("provider/sandbox"));
    assert!(!debug.contains("telemetry/cleanup"));

    let mut value =
        serde_json::to_value(input_manifest(BuildTimeInputCompletenessPosture::Complete))
            .expect("value");
    value["total_count"] = serde_json::json!(16_385);
    let error =
        serde_json::from_value::<BuildTimeInputManifest>(value).expect_err("oversized manifest");
    assert!(!error.to_string().contains("16_385"));
}

#[test]
fn all_input_kinds_and_ambiguity_states_are_representable() {
    let kinds = [
        BuildTimeInputKind::RepositoryContent,
        BuildTimeInputKind::ManifestOrConfiguration,
        BuildTimeInputKind::BuildScript,
        BuildTimeInputKind::ProceduralMacro,
        BuildTimeInputKind::DependencyPackage,
        BuildTimeInputKind::ToolchainExecutable,
        BuildTimeInputKind::LinkerOrArchiver,
        BuildTimeInputKind::SdkOrSysroot,
        BuildTimeInputKind::DynamicLibrary,
        BuildTimeInputKind::RuntimeBase,
    ];
    assert_eq!(kinds.len(), 10);
    assert_ne!(
        TrustedHostAttemptStartPosture::NotStarted,
        TrustedHostAttemptStartPosture::MayHaveStarted
    );
    assert_ne!(
        TrustedHostObservationPosture::Unavailable,
        TrustedHostObservationPosture::Uncertain
    );
}

#[test]
fn nested_process_tree_and_assessment_deserialization_fail_closed() {
    let requirements = requirements();
    let observations = observations(&requirements);
    let mut observation_value = serde_json::to_value(observations).expect("observation value");
    observation_value["process_tree"]["attempt_start"] = serde_json::json!("not_started");
    let observation_error =
        serde_json::from_value::<TrustedHostLiveExecutionSecurityObservations>(observation_value)
            .expect_err("incoherent process tree");
    assert!(!observation_error.to_string().contains("telemetry"));

    let assessment_error = serde_json::from_value::<TrustedHostLiveExecutionSecurityAssessment>(
        serde_json::json!({"disposition": "satisfied", "reasons": ["policy_drift"]}),
    )
    .expect_err("incoherent assessment");
    assert!(!assessment_error.to_string().contains("policy_drift"));
}

#[test]
fn policy_change_count_and_unproven_observation_channel_fail_satisfaction() {
    let requirements = requirements();
    let mut definition = observations_definition(&requirements);
    definition.summary = HostedExecutionObservationSummary::new(
        0,
        0,
        1,
        1,
        1,
        0,
        reference(
            HostedExecutionReferenceKind::Telemetry,
            "telemetry/policy-change",
        ),
    )
    .expect("summary");
    let changed = TrustedHostLiveExecutionSecurityObservations::new(definition)
        .expect("changed observations");
    let assessment = assess_trusted_host_live_execution_security(&requirements, &changed)
        .expect("changed assessment");
    assert!(!assessment.is_satisfied());

    let mut definition = observations_definition(&requirements);
    definition.observation_channel = TrustedHostObservationPosture::Unavailable;
    let unavailable = TrustedHostLiveExecutionSecurityObservations::new(definition)
        .expect("unavailable observations");
    let assessment = assess_trusted_host_live_execution_security(&requirements, &unavailable)
        .expect("unavailable assessment");
    assert!(!assessment.is_satisfied());
}

#[test]
fn required_denied_egress_probe_is_fail_closed() {
    let requirements = TrustedHostLiveExecutionSecurityRequirements::new(
        TrustedHostLiveExecutionSecurityRequirementsDefinition {
            binding: binding(),
            requested_policy_hash: hash("policy"),
            root_requirements: root_requirements(),
            denied_egress_probe_required: true,
        },
    )
    .expect("requirements");
    let observations = observations(&requirements);
    let assessment = assess_trusted_host_live_execution_security(&requirements, &observations)
        .expect("assessment");
    assert!(!assessment.is_satisfied());
}

#[test]
fn requirements_fingerprint_is_canonical_across_root_order() {
    let first = requirements();
    let mut reversed = root_requirements();
    reversed.reverse();
    let second = TrustedHostLiveExecutionSecurityRequirements::new(
        TrustedHostLiveExecutionSecurityRequirementsDefinition {
            binding: binding(),
            requested_policy_hash: hash("policy"),
            root_requirements: reversed,
            denied_egress_probe_required: false,
        },
    )
    .expect("requirements");

    assert_eq!(first.fingerprint(), second.fingerprint());
    let serialized = serde_json::to_string(&first).expect("requirements serialization");
    let restored: TrustedHostLiveExecutionSecurityRequirements =
        serde_json::from_str(&serialized).expect("requirements deserialization");
    assert_eq!(first.fingerprint(), restored.fingerprint());
}

#[test]
fn manifest_and_root_observation_semantics_are_canonical_and_fail_closed() {
    let canonical = input_manifest(BuildTimeInputCompletenessPosture::Complete);
    let mut reversed_counts = canonical.kind_counts().to_vec();
    reversed_counts.reverse();
    let reordered = BuildTimeInputManifest::new(BuildTimeInputManifestDefinition {
        fingerprint: canonical.fingerprint().clone(),
        total_count: canonical.total_count(),
        kind_counts: reversed_counts,
        discovery_method: canonical.discovery_method(),
        completeness: canonical.completeness(),
        observation_reference: Some(reference(
            HostedExecutionReferenceKind::Telemetry,
            "telemetry/input-manifest",
        )),
    })
    .expect("reordered manifest");
    assert_eq!(canonical.kind_counts(), reordered.kind_counts());

    let error = TrustedHostRootObservation::new(TrustedHostRootObservationDefinition {
        role: TrustedHostRootRole::RepositoryInput,
        access: TrustedHostRootAccess::ReadOnly,
        object_binding: TrustedHostObservationPosture::Unavailable,
        continuity: TrustedHostObservationPosture::Proven,
        method: TrustedHostRootObservationMethod::Unavailable,
        invocation_scoped: false,
        commitment: hash("unavailable-root"),
        reference: reference(
            HostedExecutionReferenceKind::Telemetry,
            "telemetry/unavailable-root",
        ),
    })
    .expect_err("unavailable method cannot prove continuity");
    assert_eq!(
        error.code(),
        "trusted_host.live_security.root_binding.posture.invalid"
    );

    let requirements = requirements();
    let mut definition = observations_definition(&requirements);
    let repository = definition
        .root_observations
        .iter_mut()
        .find(|root| root.role() == TrustedHostRootRole::RepositoryInput)
        .expect("repository observation");
    *repository = TrustedHostRootObservation::new(TrustedHostRootObservationDefinition {
        role: TrustedHostRootRole::RepositoryInput,
        access: TrustedHostRootAccess::ReadOnly,
        object_binding: TrustedHostObservationPosture::Proven,
        continuity: TrustedHostObservationPosture::Proven,
        method: TrustedHostRootObservationMethod::ProviderNativeIdentity,
        invocation_scoped: false,
        commitment: hash("repository-root"),
        reference: reference(
            HostedExecutionReferenceKind::Telemetry,
            "telemetry/repository-root",
        ),
    })
    .expect("repository observation");
    let observations =
        TrustedHostLiveExecutionSecurityObservations::new(definition).expect("observations");
    let assessment = assess_trusted_host_live_execution_security(&requirements, &observations)
        .expect("assessment");
    assert!(!assessment.is_satisfied());
}

#[test]
fn transient_write_posture_rejects_incoherent_denied_counts() {
    let error =
        TrustedHostTransientWriteObservation::new(TrustedHostTransientWriteObservationDefinition {
            protected_roots: HostedExecutionControlPosture::Enforced,
            posture: TrustedHostTransientWriteObservationPosture::NoAttemptObserved,
            denied_attempt_count: 1,
            outputs_quiescent: true,
            retained_artifact_commitment: TrustedHostObservationPosture::Proven,
            reference: reference(
                HostedExecutionReferenceKind::Telemetry,
                "telemetry/transient-write",
            ),
        })
        .expect_err("no-attempt posture cannot include denied attempts");
    assert_eq!(
        error.code(),
        "trusted_host.live_security.observation.transient_write.invalid"
    );
}
