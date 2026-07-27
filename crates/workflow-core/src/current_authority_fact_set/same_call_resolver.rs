use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::Serialize;
use sha2::{Digest, Sha256};

use super::hash_field;
use super::in_memory_source::{
    CurrentAuthoritySourceQueryInput, InMemoryCurrentAuthoritySource,
    InMemoryCurrentAuthoritySourceInput,
};
use crate::{
    consume_required_context, project_step_scoped_context, resolve_capability_authority,
    CapabilityResolutionInput, CapabilityResolutionReason, CurrentAuthorityFactSet,
    GovernedContextAccessLevel, GovernedContextProjectionCandidate, GovernedContextProjectionInput,
    GovernedContextReference, RedactionMetadata, RequiredContextConsumptionContext,
    RequiredContextConsumptionInput, RequiredContextConsumptionPosture,
    RequiredContextContractBinding, RequiredContextExecutionBinding, RequiredContextObligation,
    SpecContentHash, Timestamp, WorkflowOsError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum CurrentAuthorityTimeOfUseVersion {
    V1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum CurrentAuthorityTimeOfUsePosture {
    Ready,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum CurrentAuthorityTimeOfUseReason {
    Ready,
    RequiredContextGap,
    OptionalContextGap,
    IndependentPolicyRequired,
    IndependentApprovalRequired,
    IndependentEvidenceRequired,
    IndependentCheckRequired,
}

struct InMemoryCurrentContextReferenceSourceInput {
    observed_at: Timestamp,
    complete_reference_inventory: Vec<GovernedContextReference>,
}

struct InMemoryCurrentContextReferenceSource {
    observed_at: Timestamp,
    references: Vec<GovernedContextReference>,
    inventory_hash: SpecContentHash,
}

impl InMemoryCurrentContextReferenceSource {
    fn new(input: InMemoryCurrentContextReferenceSourceInput) -> Result<Self, WorkflowOsError> {
        let mut references = input.complete_reference_inventory;
        for reference in &references {
            reference.validate().map_err(|_| {
                resolver_error(
                    "reference.inventory_invalid",
                    "current context reference source inventory is invalid",
                )
            })?;
        }
        references.sort_by(|left, right| left.target().cmp(right.target()));
        if references
            .windows(2)
            .any(|pair| pair[0].target() == pair[1].target())
        {
            return Err(resolver_error(
                "reference.duplicate",
                "current context reference source contains duplicate targets",
            ));
        }
        let inventory_hash = resolver_hash(
            "current-context-reference-inventory-v1",
            &(input.observed_at, &references),
        )?;
        Ok(Self {
            observed_at: input.observed_at,
            references,
            inventory_hash,
        })
    }

    const fn observed_at(&self) -> Timestamp {
        self.observed_at
    }

    fn query(
        &self,
        contract: &RequiredContextContractBinding,
        evaluated_at: Timestamp,
    ) -> Result<Vec<GovernedContextReference>, WorkflowOsError> {
        if self.observed_at > evaluated_at {
            return Err(resolver_error(
                "reference.time_invalid",
                "current context reference source time is invalid",
            ));
        }
        let mut selected = Vec::with_capacity(contract.requirements().len());
        for requirement in contract.requirements() {
            let reference = self
                .references
                .iter()
                .find(|reference| reference.target() == requirement.target())
                .ok_or_else(|| {
                    resolver_error(
                        "reference.missing",
                        "current context reference source is missing an exact target",
                    )
                })?;
            selected.push(reference.clone());
        }
        Ok(selected)
    }
}

impl fmt::Debug for InMemoryCurrentContextReferenceSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InMemoryCurrentContextReferenceSource")
            .field("observed_at", &"[REDACTED]")
            .field("reference_count", &self.references.len())
            .field("inventory_hash", &"[REDACTED]")
            .finish()
    }
}

struct CurrentAuthorityTimeOfUseInput<'a> {
    execution_binding: &'a RequiredContextExecutionBinding,
    contract: &'a RequiredContextContractBinding,
    authority_source: &'a InMemoryCurrentAuthoritySource,
    reference_source: &'a InMemoryCurrentContextReferenceSource,
    evaluated_at: Timestamp,
    redaction: &'a RedactionMetadata,
}

struct CurrentAuthorityTimeOfUseAssessment {
    version: CurrentAuthorityTimeOfUseVersion,
    posture: CurrentAuthorityTimeOfUsePosture,
    reasons: Vec<CurrentAuthorityTimeOfUseReason>,
    consumption: crate::RequiredContextConsumptionResult,
    authority_source_hash: SpecContentHash,
    reference_source_hash: SpecContentHash,
    fact_set_hash: SpecContentHash,
    evaluated_at: Timestamp,
    assessment_hash: SpecContentHash,
}

struct ResolvedProjectionCandidates {
    by_access: BTreeMap<GovernedContextAccessLevel, Vec<GovernedContextProjectionCandidate>>,
    reasons: BTreeSet<CurrentAuthorityTimeOfUseReason>,
}

impl CurrentAuthorityTimeOfUseAssessment {
    fn posture(&self) -> CurrentAuthorityTimeOfUsePosture {
        self.posture
    }

    fn reasons(&self) -> &[CurrentAuthorityTimeOfUseReason] {
        &self.reasons
    }

    fn consumption(&self) -> &crate::RequiredContextConsumptionResult {
        &self.consumption
    }

    fn assessment_hash(&self) -> &SpecContentHash {
        &self.assessment_hash
    }

    fn authority_source_hash(&self) -> &SpecContentHash {
        &self.authority_source_hash
    }

    fn reference_source_hash(&self) -> &SpecContentHash {
        &self.reference_source_hash
    }

    fn fact_set_hash(&self) -> &SpecContentHash {
        &self.fact_set_hash
    }

    fn evaluated_at(&self) -> Timestamp {
        self.evaluated_at
    }
}

impl fmt::Debug for CurrentAuthorityTimeOfUseAssessment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CurrentAuthorityTimeOfUseAssessment")
            .field("version", &self.version)
            .field("posture", &self.posture)
            .field("reasons", &self.reasons)
            .field("consumption_posture", &self.consumption.posture())
            .field("authority_source_hash", &"[REDACTED]")
            .field("reference_source_hash", &"[REDACTED]")
            .field("fact_set_hash", &"[REDACTED]")
            .field("evaluated_at", &"[REDACTED]")
            .field("assessment_hash", &"[REDACTED]")
            .finish()
    }
}

fn resolve_current_authority_time_of_use(
    input: &CurrentAuthorityTimeOfUseInput<'_>,
) -> Result<CurrentAuthorityTimeOfUseAssessment, WorkflowOsError> {
    validate_time_of_use_input(input)?;
    let fact_set = input
        .authority_source
        .query(&CurrentAuthoritySourceQueryInput {
            execution_binding: input.execution_binding,
            contract: input.contract,
            evaluated_at: input.evaluated_at,
        })
        .map_err(|_| {
            resolver_error(
                "source.query_failed",
                "current authority source query failed",
            )
        })?;
    let references = input
        .reference_source
        .query(input.contract, input.evaluated_at)?;
    let resolved_candidates = resolve_projection_candidates(input, &fact_set, references)?;
    let projections = project_current_context(input, resolved_candidates.by_access)?;
    let mut reasons = resolved_candidates.reasons;

    let context = RequiredContextConsumptionContext::new(
        input.execution_binding.actor().clone(),
        input.execution_binding.workflow_id().clone(),
        input.execution_binding.run_id().clone(),
        input.execution_binding.step_id().clone(),
        input.execution_binding.harness_contract_id().clone(),
        input.evaluated_at,
    );
    let consumption = consume_required_context(&RequiredContextConsumptionInput {
        contract: input.contract,
        context: &context,
        projections: &projections,
    })
    .map_err(|_| {
        resolver_error(
            "consumption.failed",
            "current required-context consumption failed",
        )
    })?;
    for gap in consumption.gaps() {
        reasons.insert(match gap.obligation() {
            RequiredContextObligation::Required => {
                CurrentAuthorityTimeOfUseReason::RequiredContextGap
            }
            RequiredContextObligation::Optional => {
                CurrentAuthorityTimeOfUseReason::OptionalContextGap
            }
        });
    }
    build_assessment(input, &fact_set, consumption, reasons)
}

fn validate_time_of_use_input(
    input: &CurrentAuthorityTimeOfUseInput<'_>,
) -> Result<(), WorkflowOsError> {
    input.execution_binding.validate().map_err(|_| {
        resolver_error(
            "binding.invalid",
            "current authority time-of-use binding is invalid",
        )
    })?;
    if input.execution_binding.contract_content_hash() != input.contract.content_hash()
        || input.execution_binding.harness_contract_id() != input.contract.contract_id()
        || input.execution_binding.harness_contract_version() != input.contract.contract_version()
    {
        return Err(resolver_error(
            "contract.mismatch",
            "current authority time-of-use contract does not match its binding",
        ));
    }
    if input.evaluated_at < input.execution_binding.bound_at()
        || input.authority_source.observed_at() > input.evaluated_at
    {
        return Err(resolver_error(
            "time.invalid",
            "current authority time-of-use timestamps are inconsistent",
        ));
    }
    Ok(())
}

fn resolve_projection_candidates(
    input: &CurrentAuthorityTimeOfUseInput<'_>,
    fact_set: &CurrentAuthorityFactSet,
    references: Vec<GovernedContextReference>,
) -> Result<ResolvedProjectionCandidates, WorkflowOsError> {
    let mut candidates_by_access =
        BTreeMap::<GovernedContextAccessLevel, Vec<GovernedContextProjectionCandidate>>::new();
    let mut reasons = BTreeSet::new();
    for (requirement, reference) in input.contract.requirements().iter().zip(references) {
        let capability = requirement.access_level().required_capability()?;
        let resource = requirement.target().capability_resource()?;
        fact_set
            .availability_records()
            .iter()
            .find(|record| record.capability() == &capability && record.resource() == &resource)
            .ok_or_else(|| {
                resolver_error(
                    "source.availability_missing",
                    "current authority fact set is missing exact availability",
                )
            })?;
        let resolution = resolve_capability_authority(&CapabilityResolutionInput {
            capability: &capability,
            resource: &resource,
            actor: input.execution_binding.actor(),
            workflow_id: input.execution_binding.workflow_id(),
            run_id: input.execution_binding.run_id(),
            step_id: input.execution_binding.step_id(),
            harness_contract_id: Some(input.execution_binding.harness_contract_id()),
            requested_sensitivity: reference.sensitivity(),
            evaluated_at: input.evaluated_at,
            availability_records: fact_set.availability_records(),
            grants: fact_set.grants(),
        })
        .map_err(|_| {
            resolver_error(
                "capability.resolution_failed",
                "current capability resolution failed",
            )
        })?;
        add_prerequisite_reasons(&mut reasons, resolution.reasons());
        let candidate = GovernedContextProjectionCandidate::new(
            reference,
            input.reference_source.observed_at(),
            requirement.access_level(),
            resolution,
        )
        .map_err(|_| {
            resolver_error(
                "projection.candidate_invalid",
                "current context projection candidate is invalid",
            )
        })?;
        candidates_by_access
            .entry(requirement.access_level())
            .or_default()
            .push(candidate);
    }
    Ok(ResolvedProjectionCandidates {
        by_access: candidates_by_access,
        reasons,
    })
}

fn project_current_context(
    input: &CurrentAuthorityTimeOfUseInput<'_>,
    candidates_by_access: BTreeMap<
        GovernedContextAccessLevel,
        Vec<GovernedContextProjectionCandidate>,
    >,
) -> Result<Vec<crate::GovernedContextProjection>, WorkflowOsError> {
    let mut projections = Vec::with_capacity(candidates_by_access.len());
    for (access_level, candidates) in candidates_by_access {
        projections.push(
            project_step_scoped_context(&GovernedContextProjectionInput {
                actor: input.execution_binding.actor(),
                workflow_id: input.execution_binding.workflow_id(),
                run_id: input.execution_binding.run_id(),
                step_id: input.execution_binding.step_id(),
                harness_contract_id: Some(input.execution_binding.harness_contract_id()),
                projected_at: input.evaluated_at,
                maximum_allowed_sensitivity: input.execution_binding.maximum_sensitivity(),
                requested_access_level: access_level,
                candidates: &candidates,
                redaction: input.redaction,
            })
            .map_err(|_| {
                resolver_error("projection.failed", "current context projection failed")
            })?,
        );
    }
    Ok(projections)
}

fn build_assessment(
    input: &CurrentAuthorityTimeOfUseInput<'_>,
    fact_set: &CurrentAuthorityFactSet,
    consumption: crate::RequiredContextConsumptionResult,
    mut reasons: BTreeSet<CurrentAuthorityTimeOfUseReason>,
) -> Result<CurrentAuthorityTimeOfUseAssessment, WorkflowOsError> {
    let posture = match consumption.posture() {
        RequiredContextConsumptionPosture::Satisfied => CurrentAuthorityTimeOfUsePosture::Ready,
        RequiredContextConsumptionPosture::Blocked => CurrentAuthorityTimeOfUsePosture::Blocked,
    };
    if reasons.is_empty() {
        reasons.insert(CurrentAuthorityTimeOfUseReason::Ready);
    }
    let reasons = reasons.into_iter().collect::<Vec<_>>();
    let version = CurrentAuthorityTimeOfUseVersion::V1;
    let authority_source_hash = input.authority_source.inventory_hash().clone();
    let reference_source_hash = input.reference_source.inventory_hash.clone();
    let fact_set_hash = fact_set.fact_set_hash().clone();
    let assessment_hash = resolver_hash(
        "current-authority-time-of-use-assessment-v1",
        &(
            version,
            input.execution_binding.binding_hash(),
            input.contract.content_hash(),
            fact_set.query_set().query_set_hash(),
            &authority_source_hash,
            &reference_source_hash,
            &fact_set_hash,
            input.evaluated_at,
            posture,
            &reasons,
            &consumption,
        ),
    )?;
    Ok(CurrentAuthorityTimeOfUseAssessment {
        version,
        posture,
        reasons,
        consumption,
        authority_source_hash,
        reference_source_hash,
        fact_set_hash,
        evaluated_at: input.evaluated_at,
        assessment_hash,
    })
}

fn add_prerequisite_reasons(
    reasons: &mut BTreeSet<CurrentAuthorityTimeOfUseReason>,
    resolution_reasons: &[CapabilityResolutionReason],
) {
    for reason in resolution_reasons {
        let mapped = match reason {
            CapabilityResolutionReason::PolicyEvaluationRequired => {
                Some(CurrentAuthorityTimeOfUseReason::IndependentPolicyRequired)
            }
            CapabilityResolutionReason::ApprovalEvaluationRequired => {
                Some(CurrentAuthorityTimeOfUseReason::IndependentApprovalRequired)
            }
            CapabilityResolutionReason::EvidenceEvaluationRequired => {
                Some(CurrentAuthorityTimeOfUseReason::IndependentEvidenceRequired)
            }
            CapabilityResolutionReason::CheckEvaluationRequired => {
                Some(CurrentAuthorityTimeOfUseReason::IndependentCheckRequired)
            }
            _ => None,
        };
        if let Some(mapped) = mapped {
            reasons.insert(mapped);
        }
    }
}

fn resolver_hash(domain: &str, value: &impl Serialize) -> Result<SpecContentHash, WorkflowOsError> {
    let bytes = serde_json::to_vec(value).map_err(|_| {
        resolver_error(
            "hash.serialization_failed",
            "current authority time-of-use hashing failed",
        )
    })?;
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "domain", domain.as_bytes());
    hash_field(&mut hasher, "value", &bytes);
    Ok(SpecContentHash::from_bytes(hasher.finalize()))
}

fn resolver_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("current_authority.time_of_use.{suffix}"), message)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;
    use crate::{
        build_immutable_run_bundle, load_project, ActorId, ApprovalReferenceId,
        CapabilityAvailability, CapabilityAvailabilityRecord, CapabilityDelegationPosture,
        CapabilityGrant, CapabilityGrantDefinition, CapabilityGrantId, CapabilityGrantLifecycle,
        CapabilityGrantRequirements, CapabilityGrantScope, EvidenceReferenceId,
        GovernedContextAvailability, GovernedContextReferenceTarget, HarnessContractId,
        HarnessContractVersion, ImmutableRunBundleBuildRequest, ImmutableRunBundleExecutionPosture,
        ImmutableRunBundleHandlerPosture, ImmutableRunBundleHandlerReference, ImmutableRunBundleId,
        ImmutableRunBundleReferencePosture, ImmutableRunBundleSensitivity,
        ImmutableRunBundleVersion, LocalCheckResultId, LocalImmutableRunBundleStore, PolicyId,
        RequiredContextExecutionBindingInput, RequiredContextRequirement,
        RequiredContextRequirementId, SkillId, SkillVersion, StepId, WorkReportId,
        WorkReportSensitivity, WorkflowId, WorkflowRunId, SUPPORTED_SCHEMA_VERSION,
    };

    static NEXT_ROOT: AtomicU64 = AtomicU64::new(1);

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new(name: &str) -> Self {
            let id = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "workflow-os-time-of-use-{name}-{}-{id}",
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

    fn fixture(
        second_obligation: RequiredContextObligation,
    ) -> (
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
                requirement(
                    "required/report-reference",
                    "report/current",
                    GovernedContextAccessLevel::ReferenceOnly,
                    RequiredContextObligation::Required,
                ),
                requirement(
                    "required/report-metadata",
                    "report/metadata",
                    GovernedContextAccessLevel::BoundedMetadata,
                    second_obligation,
                ),
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
        (contract, binding)
    }

    fn requirement(
        id: &str,
        report_id: &str,
        access_level: GovernedContextAccessLevel,
        obligation: RequiredContextObligation,
    ) -> RequiredContextRequirement {
        RequiredContextRequirement::new(
            RequiredContextRequirementId::new(id).expect("requirement"),
            GovernedContextReferenceTarget::WorkReport(
                WorkReportId::new(report_id).expect("report"),
            ),
            access_level,
            obligation,
            WorkReportSensitivity::Internal,
        )
        .expect("requirement")
    }

    fn availability(
        contract: &RequiredContextContractBinding,
        posture: CapabilityAvailability,
    ) -> Vec<CapabilityAvailabilityRecord> {
        contract
            .requirements()
            .iter()
            .map(|requirement| {
                CapabilityAvailabilityRecord::new(
                    requirement
                        .access_level()
                        .required_capability()
                        .expect("capability"),
                    requirement
                        .target()
                        .capability_resource()
                        .expect("resource"),
                    posture,
                    timestamp("2026-07-26T10:20:00Z"),
                    RedactionMetadata::empty(),
                )
                .expect("availability")
            })
            .collect()
    }

    fn grant(
        contract: &RequiredContextContractBinding,
        index: usize,
        lifecycle: CapabilityGrantLifecycle,
        requirements: CapabilityGrantRequirements,
    ) -> CapabilityGrant {
        grant_with_constraints(
            contract,
            index,
            lifecycle,
            requirements,
            None,
            WorkReportSensitivity::Internal,
        )
    }

    fn grant_with_constraints(
        contract: &RequiredContextContractBinding,
        index: usize,
        lifecycle: CapabilityGrantLifecycle,
        requirements: CapabilityGrantRequirements,
        expires_at: Option<Timestamp>,
        sensitivity_ceiling: WorkReportSensitivity,
    ) -> CapabilityGrant {
        let requirement = &contract.requirements()[index];
        CapabilityGrant::new(CapabilityGrantDefinition {
            grant_id: CapabilityGrantId::new(format!("grant/{index}")).expect("grant id"),
            subject: ActorId::new("agent/consumer").expect("actor"),
            capability: requirement
                .access_level()
                .required_capability()
                .expect("capability"),
            resource: requirement
                .target()
                .capability_resource()
                .expect("resource"),
            scope: CapabilityGrantScope::new(
                WorkflowId::new("authority/build").expect("workflow"),
                Some(WorkflowRunId::new("run-authority").expect("run")),
                Some(StepId::new("consume").expect("step")),
                Some(HarnessContractId::new("harness/context").expect("harness")),
            )
            .expect("scope"),
            issuer: ActorId::new("system/authority").expect("issuer"),
            issued_at: timestamp("2026-07-26T10:05:00Z"),
            expires_at,
            lifecycle,
            revocation_reference: (lifecycle == CapabilityGrantLifecycle::Revoked)
                .then(|| "revocation/record".to_owned()),
            delegation: CapabilityDelegationPosture::Disabled,
            requirements,
            sensitivity_ceiling,
            redaction: RedactionMetadata::empty(),
        })
        .expect("grant")
    }

    fn references(
        contract: &RequiredContextContractBinding,
        second_posture: GovernedContextAvailability,
    ) -> Vec<GovernedContextReference> {
        contract
            .requirements()
            .iter()
            .enumerate()
            .map(|(index, requirement)| {
                GovernedContextReference::new(
                    requirement.target().clone(),
                    WorkReportSensitivity::Internal,
                    if index == 1 {
                        second_posture
                    } else {
                        GovernedContextAvailability::Available
                    },
                    RedactionMetadata::empty(),
                )
                .expect("reference")
            })
            .collect()
    }

    fn resolve(
        contract: &RequiredContextContractBinding,
        binding: &RequiredContextExecutionBinding,
        grants: Vec<CapabilityGrant>,
        availability_records: Vec<CapabilityAvailabilityRecord>,
        references: Vec<GovernedContextReference>,
    ) -> Result<CurrentAuthorityTimeOfUseAssessment, WorkflowOsError> {
        let authority_source =
            InMemoryCurrentAuthoritySource::new(InMemoryCurrentAuthoritySourceInput {
                observed_at: timestamp("2026-07-26T10:20:00Z"),
                complete_grant_inventory: grants,
                complete_availability_inventory: availability_records,
            })?;
        let reference_source = InMemoryCurrentContextReferenceSource::new(
            InMemoryCurrentContextReferenceSourceInput {
                observed_at: timestamp("2026-07-26T10:20:00Z"),
                complete_reference_inventory: references,
            },
        )?;
        resolve_current_authority_time_of_use(&CurrentAuthorityTimeOfUseInput {
            execution_binding: binding,
            contract,
            authority_source: &authority_source,
            reference_source: &reference_source,
            evaluated_at: timestamp("2026-07-26T10:30:00Z"),
            redaction: &RedactionMetadata::empty(),
        })
    }

    #[test]
    fn complete_current_facts_produce_ready_assessment() {
        let (contract, binding) = fixture(RequiredContextObligation::Required);
        let result = resolve(
            &contract,
            &binding,
            vec![
                grant(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract, CapabilityAvailability::Available),
            references(&contract, GovernedContextAvailability::Available),
        )
        .expect("ready");

        assert_eq!(result.posture(), CurrentAuthorityTimeOfUsePosture::Ready);
        assert_eq!(result.reasons(), [CurrentAuthorityTimeOfUseReason::Ready]);
        assert_eq!(
            result.consumption().posture(),
            RequiredContextConsumptionPosture::Satisfied
        );
        assert_eq!(result.consumption().projections().len(), 2);
        assert_eq!(result.consumption().satisfactions().len(), 2);
        assert!(!result.authority_source_hash().as_str().is_empty());
        assert!(!result.reference_source_hash().as_str().is_empty());
        assert!(!result.fact_set_hash().as_str().is_empty());
        assert_eq!(result.evaluated_at(), timestamp("2026-07-26T10:30:00Z"));
        assert_eq!(
            result.assessment_hash().as_str(),
            "d6a2c046b7c0ea727756ad968f5866cd1dd00c438e5eafc1e76052c65c2a6c48"
        );
    }

    #[test]
    fn unresolved_required_approval_blocks() {
        let (contract, binding) = fixture(RequiredContextObligation::Required);
        let approval_requirements = CapabilityGrantRequirements::new(
            Vec::new(),
            vec![ApprovalReferenceId::new("approval/current").expect("approval")],
            Vec::new(),
            Vec::new(),
        )
        .expect("requirements");
        let result = resolve(
            &contract,
            &binding,
            vec![
                grant(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    approval_requirements,
                ),
            ],
            availability(&contract, CapabilityAvailability::Available),
            references(&contract, GovernedContextAvailability::Available),
        )
        .expect("blocked assessment");

        assert_eq!(result.posture(), CurrentAuthorityTimeOfUsePosture::Blocked);
        assert!(result
            .reasons()
            .contains(&CurrentAuthorityTimeOfUseReason::IndependentApprovalRequired));
        assert!(result
            .reasons()
            .contains(&CurrentAuthorityTimeOfUseReason::RequiredContextGap));
    }

    #[test]
    fn all_independent_prerequisites_remain_blocking() {
        let (contract, binding) = fixture(RequiredContextObligation::Required);
        let requirements = CapabilityGrantRequirements::new(
            vec![PolicyId::new("policy/current").expect("policy")],
            vec![ApprovalReferenceId::new("approval/current").expect("approval")],
            vec![EvidenceReferenceId::new("evidence/current").expect("evidence")],
            vec![LocalCheckResultId::new("check/current").expect("check")],
        )
        .expect("requirements");
        let result = resolve(
            &contract,
            &binding,
            vec![
                grant(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant(&contract, 1, CapabilityGrantLifecycle::Active, requirements),
            ],
            availability(&contract, CapabilityAvailability::Available),
            references(&contract, GovernedContextAvailability::Available),
        )
        .expect("blocked assessment");

        assert_eq!(result.posture(), CurrentAuthorityTimeOfUsePosture::Blocked);
        for reason in [
            CurrentAuthorityTimeOfUseReason::IndependentPolicyRequired,
            CurrentAuthorityTimeOfUseReason::IndependentApprovalRequired,
            CurrentAuthorityTimeOfUseReason::IndependentEvidenceRequired,
            CurrentAuthorityTimeOfUseReason::IndependentCheckRequired,
        ] {
            assert!(result.reasons().contains(&reason));
        }
    }

    #[test]
    fn unresolved_optional_approval_remains_explicit_non_blocking_gap() {
        let (contract, binding) = fixture(RequiredContextObligation::Optional);
        let approval_requirements = CapabilityGrantRequirements::new(
            Vec::new(),
            vec![ApprovalReferenceId::new("approval/current").expect("approval")],
            Vec::new(),
            Vec::new(),
        )
        .expect("requirements");
        let result = resolve(
            &contract,
            &binding,
            vec![
                grant(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    approval_requirements,
                ),
            ],
            availability(&contract, CapabilityAvailability::Available),
            references(&contract, GovernedContextAvailability::Available),
        )
        .expect("ready with optional gap");

        assert_eq!(result.posture(), CurrentAuthorityTimeOfUsePosture::Ready);
        assert!(result
            .reasons()
            .contains(&CurrentAuthorityTimeOfUseReason::IndependentApprovalRequired));
        assert!(result
            .reasons()
            .contains(&CurrentAuthorityTimeOfUseReason::OptionalContextGap));
        assert_eq!(result.consumption().gaps().len(), 1);
    }

    #[test]
    fn revoked_grant_blocks_without_error() {
        let (contract, binding) = fixture(RequiredContextObligation::Required);
        let result = resolve(
            &contract,
            &binding,
            vec![
                grant(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Revoked,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract, CapabilityAvailability::Available),
            references(&contract, GovernedContextAvailability::Available),
        )
        .expect("blocked assessment");

        assert_eq!(result.posture(), CurrentAuthorityTimeOfUsePosture::Blocked);
        assert!(result
            .reasons()
            .contains(&CurrentAuthorityTimeOfUseReason::RequiredContextGap));
    }

    #[test]
    fn expired_grant_blocks_without_reusing_stale_authority() {
        let (contract, binding) = fixture(RequiredContextObligation::Required);
        let result = resolve(
            &contract,
            &binding,
            vec![
                grant(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant_with_constraints(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                    Some(timestamp("2026-07-26T10:25:00Z")),
                    WorkReportSensitivity::Internal,
                ),
            ],
            availability(&contract, CapabilityAvailability::Available),
            references(&contract, GovernedContextAvailability::Available),
        )
        .expect("blocked assessment");

        assert_eq!(result.posture(), CurrentAuthorityTimeOfUsePosture::Blocked);
        assert!(result
            .reasons()
            .contains(&CurrentAuthorityTimeOfUseReason::RequiredContextGap));
    }

    #[test]
    fn sensitivity_ceiling_blocks_required_context() {
        let (contract, binding) = fixture(RequiredContextObligation::Required);
        let result = resolve(
            &contract,
            &binding,
            vec![
                grant(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant_with_constraints(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                    None,
                    WorkReportSensitivity::Public,
                ),
            ],
            availability(&contract, CapabilityAvailability::Available),
            references(&contract, GovernedContextAvailability::Available),
        )
        .expect("blocked assessment");

        assert_eq!(result.posture(), CurrentAuthorityTimeOfUsePosture::Blocked);
        assert!(result
            .reasons()
            .contains(&CurrentAuthorityTimeOfUseReason::RequiredContextGap));
    }

    #[test]
    fn unavailable_optional_reference_is_explicit_and_non_blocking() {
        let (contract, binding) = fixture(RequiredContextObligation::Optional);
        let result = resolve(
            &contract,
            &binding,
            vec![
                grant(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract, CapabilityAvailability::Available),
            references(&contract, GovernedContextAvailability::Unavailable),
        )
        .expect("ready with optional gap");

        assert_eq!(result.posture(), CurrentAuthorityTimeOfUsePosture::Ready);
        assert!(result
            .reasons()
            .contains(&CurrentAuthorityTimeOfUseReason::OptionalContextGap));
    }

    #[test]
    fn disconnected_capability_blocks_required_context() {
        let (contract, binding) = fixture(RequiredContextObligation::Required);
        let result = resolve(
            &contract,
            &binding,
            vec![
                grant(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract, CapabilityAvailability::DeclaredNotConnected),
            references(&contract, GovernedContextAvailability::Available),
        )
        .expect("blocked assessment");

        assert_eq!(result.posture(), CurrentAuthorityTimeOfUsePosture::Blocked);
        assert_eq!(result.consumption().gaps().len(), 2);
    }

    #[test]
    fn changed_contract_fails_before_resolution() {
        let (original_contract, binding) = fixture(RequiredContextObligation::Required);
        let (changed_contract, _) = fixture(RequiredContextObligation::Optional);
        let error = resolve(
            &changed_contract,
            &binding,
            vec![
                grant(
                    &changed_contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant(
                    &changed_contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&changed_contract, CapabilityAvailability::Available),
            references(&changed_contract, GovernedContextAvailability::Available),
        )
        .expect_err("contract substitution");
        let rendered = error.to_string();

        assert_ne!(
            original_contract.content_hash(),
            changed_contract.content_hash()
        );
        assert!(rendered.contains("current_authority.time_of_use.contract.mismatch"));
        assert!(!rendered.contains("report/current"));
    }

    #[test]
    fn duplicate_reference_inventory_fails_closed() {
        let (contract, _) = fixture(RequiredContextObligation::Required);
        let mut complete_references = references(&contract, GovernedContextAvailability::Available);
        complete_references.push(complete_references[0].clone());
        let error = InMemoryCurrentContextReferenceSource::new(
            InMemoryCurrentContextReferenceSourceInput {
                observed_at: timestamp("2026-07-26T10:20:00Z"),
                complete_reference_inventory: complete_references,
            },
        )
        .expect_err("duplicate reference");

        assert!(error
            .to_string()
            .contains("current_authority.time_of_use.reference.duplicate"));
    }

    #[test]
    fn input_order_does_not_change_assessment_hash() {
        let (contract, binding) = fixture(RequiredContextObligation::Required);
        let mut grants = vec![
            grant(
                &contract,
                0,
                CapabilityGrantLifecycle::Active,
                CapabilityGrantRequirements::default(),
            ),
            grant(
                &contract,
                1,
                CapabilityGrantLifecycle::Active,
                CapabilityGrantRequirements::default(),
            ),
        ];
        let mut availability_records = availability(&contract, CapabilityAvailability::Available);
        let mut complete_references = references(&contract, GovernedContextAvailability::Available);
        let first = resolve(
            &contract,
            &binding,
            grants.clone(),
            availability_records.clone(),
            complete_references.clone(),
        )
        .expect("first");
        grants.reverse();
        availability_records.reverse();
        complete_references.reverse();
        let second = resolve(
            &contract,
            &binding,
            grants,
            availability_records,
            complete_references,
        )
        .expect("second");

        assert_eq!(first.assessment_hash(), second.assessment_hash());
    }

    #[test]
    fn missing_reference_fails_closed_without_leaking_values() {
        let (contract, binding) = fixture(RequiredContextObligation::Required);
        let error = resolve(
            &contract,
            &binding,
            vec![
                grant(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract, CapabilityAvailability::Available),
            references(&contract, GovernedContextAvailability::Available)
                .into_iter()
                .take(1)
                .collect(),
        )
        .expect_err("missing reference");
        let rendered = error.to_string();

        assert!(rendered.contains("current_authority.time_of_use.reference.missing"));
        assert!(!rendered.contains("report/metadata"));
        assert!(!rendered.contains("token"));
    }

    #[test]
    fn debug_redacts_source_commitments() {
        let (contract, binding) = fixture(RequiredContextObligation::Required);
        let result = resolve(
            &contract,
            &binding,
            vec![
                grant(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract, CapabilityAvailability::Available),
            references(&contract, GovernedContextAvailability::Available),
        )
        .expect("ready");
        let rendered = format!("{result:?}");

        assert!(!rendered.contains(result.assessment_hash().as_str()));
        assert!(!rendered.contains("report/current"));
        assert!(!rendered.contains("run-authority"));
        assert!(rendered.contains("[REDACTED]"));
    }
}
