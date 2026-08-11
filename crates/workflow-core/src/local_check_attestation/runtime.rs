use std::collections::BTreeMap;
use std::fmt;

use sha2::{Digest, Sha256};

use super::structural_coverage::{
    adapt_authoritative_docs_check_contribution, adapt_stored_canonical_local_check_declarations,
    authoritative_record_for_step, convert_authoritative_local_check_coverage,
    evaluate_local_check_structural_coverage, AuthoritativeLocalCheckEvidenceCheckFact,
    LocalCheckGovernanceObligationSetCandidate,
};
use super::verifier::{
    exit_posture, verify_local_check_attestation, KernelObservedLocalCheck,
    KernelObservedLocalCheckDefinition, LocalCheckAttestationVerificationInput,
};
use super::{
    AcceptedLocalCheckAttestation, LocalCheckAttestationAlgorithm, LocalCheckAttestationAssurance,
    LocalCheckAttestationBinding, LocalCheckAttestationBindingDefinition, LocalCheckAttestationId,
    LocalCheckAttestationRequirement, LocalCheckAttestationSource,
};
use crate::proportional_governance_immutable_bundle::preflight_immutable_bundle_governance;
use crate::{
    assess_immutable_bundle_governance, assess_immutable_bundle_governance_from_current_facts,
    compute_local_check_command_contract_fingerprint, local_check::AuthoritativeLocalCheckHandler,
    GovernanceAssessmentBinding, GovernanceRuntimeFactAssessmentRequest,
    GovernanceRuntimeFactObservation, GovernanceRuntimeFactObservationDefinition,
    GovernanceRuntimeFactSnapshot, GovernanceRuntimeFactSnapshotId, GovernanceRuntimeFactSource,
    GovernanceRuntimeFactSourceContractVersion, GovernanceRuntimeFactSourceId,
    GovernanceRuntimeFactSourceRegistration, GovernanceRuntimeFactSourceRegistrationDefinition,
    GovernanceRuntimeFactSourceRequest, GovernanceStrictnessProfile, IdempotencyKey,
    ImmutableBundleGovernanceAssessmentRequest, ImmutableBundleGovernanceAssessmentSet,
    ImmutableLocalCheckExecutionBinding, ImmutableLocalCheckExecutionBindingDefinition,
    ImmutableLocalCheckHandlerSelection, ImmutableRunBundleDefinitionKind,
    ImmutableRunBundleDefinitionRecord, ImmutableRunBundleId, ImmutableRunBundleVersion,
    LocalCheckResult, LocalCheckResultId, SkillId, SkillInvocationId, SkillVersion,
    SpecContentHash, StepGovernanceRuntimeFacts, StepId, StoredImmutableRunBundle, Timestamp,
    WorkflowId, WorkflowOsError, WorkflowRunId,
};

pub(crate) trait LocalCheckObservationClock: fmt::Debug {
    fn now(&self) -> Result<Timestamp, WorkflowOsError>;
}

#[derive(Debug)]
pub(crate) struct SystemLocalCheckObservationClock;

impl LocalCheckObservationClock for SystemLocalCheckObservationClock {
    fn now(&self) -> Result<Timestamp, WorkflowOsError> {
        Ok(Timestamp::now_utc())
    }
}

pub(crate) struct DocsCheckAttestationExecutionInput<'a> {
    pub stored_immutable_run_bundle: &'a StoredImmutableRunBundle,
    pub requirement: &'a LocalCheckAttestationRequirement,
    pub handler: &'a dyn AuthoritativeLocalCheckHandler,
    pub workflow_id: WorkflowId,
    pub run_id: WorkflowRunId,
    pub step_id: StepId,
    pub invocation_id: SkillInvocationId,
    pub idempotency_key: IdempotencyKey,
    pub result_id: LocalCheckResultId,
    pub attestation_id: LocalCheckAttestationId,
    pub clock: &'a dyn LocalCheckObservationClock,
}

impl fmt::Debug for DocsCheckAttestationExecutionInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DocsCheckAttestationExecutionInput")
            .field("stored_immutable_run_bundle", &"[REDACTED]")
            .field("requirement", &self.requirement)
            .field("handler", &self.handler)
            .field("identities", &"[REDACTED]")
            .field("clock", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct DocsCheckAttestationExecutionOutcome {
    result: LocalCheckResult,
    accepted_attestation: Option<AcceptedLocalCheckAttestation>,
}

impl DocsCheckAttestationExecutionOutcome {
    pub(crate) const fn result(&self) -> &LocalCheckResult {
        &self.result
    }

    pub(crate) const fn accepted_attestation(&self) -> Option<&AcceptedLocalCheckAttestation> {
        self.accepted_attestation.as_ref()
    }
}

impl fmt::Debug for DocsCheckAttestationExecutionOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DocsCheckAttestationExecutionOutcome")
            .field("result_status", &self.result.status())
            .field(
                "accepted_attestation_present",
                &self.accepted_attestation.is_some(),
            )
            .field("result", &"[REDACTED]")
            .field("accepted_attestation", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DocsCheckAttestationGateReason {
    ResultStatusNotAccepted,
    FreshnessExpired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DocsCheckAttestationGateDisposition {
    Satisfied,
    NotSatisfied(DocsCheckAttestationGateReason),
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct DocsCheckAttestationGateOutcome {
    result: LocalCheckResult,
    disposition: DocsCheckAttestationGateDisposition,
    proof_fingerprint: Option<SpecContentHash>,
}

impl DocsCheckAttestationGateOutcome {
    pub(crate) const fn result(&self) -> &LocalCheckResult {
        &self.result
    }

    pub(crate) const fn disposition(&self) -> DocsCheckAttestationGateDisposition {
        self.disposition
    }

    pub(crate) const fn proof_fingerprint(&self) -> Option<&SpecContentHash> {
        self.proof_fingerprint.as_ref()
    }
}

impl fmt::Debug for DocsCheckAttestationGateOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DocsCheckAttestationGateOutcome")
            .field("result_status", &self.result.status())
            .field("disposition", &self.disposition)
            .field("proof_present", &self.proof_fingerprint.is_some())
            .field("result", &"[REDACTED]")
            .field("proof_fingerprint", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GovernanceEvidenceCheckContributionPosture {
    Satisfied,
    RequiredUnavailable,
    Failed,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct DocsCheckGovernanceEvidenceCheckContribution {
    obligation_fingerprint: SpecContentHash,
    posture: GovernanceEvidenceCheckContributionPosture,
}

impl DocsCheckGovernanceEvidenceCheckContribution {
    pub(crate) const fn obligation_fingerprint(&self) -> &SpecContentHash {
        &self.obligation_fingerprint
    }

    pub(crate) const fn posture(&self) -> GovernanceEvidenceCheckContributionPosture {
        self.posture
    }
}

impl fmt::Debug for DocsCheckGovernanceEvidenceCheckContribution {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DocsCheckGovernanceEvidenceCheckContribution")
            .field("obligation_fingerprint", &"[REDACTED]")
            .field("posture", &self.posture)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct DocsCheckGovernanceContributionOutcome {
    result: LocalCheckResult,
    contribution: DocsCheckGovernanceEvidenceCheckContribution,
}

pub(crate) struct AuthoritativeDocsCheckCompositionInput<'a> {
    pub stored_immutable_run_bundle: &'a StoredImmutableRunBundle,
    pub step_id: &'a StepId,
    pub executions: &'a [DocsCheckAttestationExecutionInput<'a>],
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct AuthoritativeDocsCheckCompositionOutcome {
    results: Vec<LocalCheckResult>,
    fact: AuthoritativeLocalCheckEvidenceCheckFact,
}

impl AuthoritativeDocsCheckCompositionOutcome {
    pub(crate) fn results(&self) -> &[LocalCheckResult] {
        &self.results
    }

    pub(crate) const fn fact(&self) -> &AuthoritativeLocalCheckEvidenceCheckFact {
        &self.fact
    }
}

impl fmt::Debug for AuthoritativeDocsCheckCompositionOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthoritativeDocsCheckCompositionOutcome")
            .field("result_count", &self.results.len())
            .field("fact", &self.fact)
            .field("results", &"[REDACTED]")
            .finish()
    }
}

pub(crate) struct AuthoritativeLocalCheckReassessmentInput<'a> {
    pub local_check: AuthoritativeDocsCheckCompositionInput<'a>,
    pub profile: GovernanceStrictnessProfile,
    pub runtime_facts: &'a [StepGovernanceRuntimeFacts],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AuthoritativeLocalCheckReassessmentBindingAlgorithm {
    V1,
}

impl AuthoritativeLocalCheckReassessmentBindingAlgorithm {
    pub(crate) const fn identifier(self) -> &'static str {
        match self {
            Self::V1 => "workflow-os/authoritative-local-check-reassessment-binding/v1",
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct AuthoritativeLocalCheckBoundAssessment {
    algorithm: AuthoritativeLocalCheckReassessmentBindingAlgorithm,
    local_check_fact: AuthoritativeLocalCheckEvidenceCheckFact,
    assessment_set: ImmutableBundleGovernanceAssessmentSet,
    binding_fingerprint: SpecContentHash,
}

impl AuthoritativeLocalCheckBoundAssessment {
    pub(crate) const fn algorithm(&self) -> AuthoritativeLocalCheckReassessmentBindingAlgorithm {
        self.algorithm
    }

    pub(crate) const fn binding_fingerprint(&self) -> &SpecContentHash {
        &self.binding_fingerprint
    }

    pub(crate) const fn local_check_posture(
        &self,
    ) -> crate::GovernanceWorkloadEvidenceCheckPosture {
        self.local_check_fact.posture()
    }

    pub(crate) fn assessment_count(&self) -> usize {
        self.assessment_set.assessments().len()
    }

    pub(crate) fn into_governance_binding(
        self,
        bundle: &StoredImmutableRunBundle,
        selected_step_id: StepId,
    ) -> Result<crate::GovernanceAssessmentBinding, WorkflowOsError> {
        if !self
            .assessment_set
            .assessments()
            .iter()
            .any(|assessment| assessment.step_id() == &selected_step_id)
        {
            return Err(reassessment_binding_error(
                "selected_assessment_unresolved",
                "authoritative local check reassessment selected assessment is unavailable",
            ));
        }
        crate::GovernanceAssessmentBinding::from_authoritative_local_check_assessment(
            bundle,
            &self.assessment_set,
            selected_step_id,
            self.binding_fingerprint,
        )
    }
}

impl fmt::Debug for AuthoritativeLocalCheckBoundAssessment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthoritativeLocalCheckBoundAssessment")
            .field("algorithm", &self.algorithm())
            .field("local_check_posture", &self.local_check_posture())
            .field("assessment_count", &self.assessment_count())
            .field("local_check_fact", &"[REDACTED]")
            .field("assessment_set", &"[REDACTED]")
            .field("binding_fingerprint", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct AuthoritativeLocalCheckReassessmentOutcome {
    results: Vec<LocalCheckResult>,
    bound_assessment: AuthoritativeLocalCheckBoundAssessment,
}

pub(crate) struct AuthoritativeLocalCheckRuntimeFactSourceBridgeInput<'a> {
    pub reassessment: AuthoritativeLocalCheckReassessmentInput<'a>,
    pub evaluated_at: Timestamp,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct AuthoritativeLocalCheckRuntimeFactSourceBridgeOutcome {
    results: Vec<LocalCheckResult>,
    governance_binding: GovernanceAssessmentBinding,
    runtime_fact_snapshot: GovernanceRuntimeFactSnapshot,
    registration: GovernanceRuntimeFactSourceRegistration,
    source: CoreOwnedAuthoritativeLocalCheckRuntimeFactSource,
}

impl AuthoritativeLocalCheckRuntimeFactSourceBridgeOutcome {
    pub(crate) fn results(&self) -> &[LocalCheckResult] {
        &self.results
    }

    pub(crate) const fn governance_binding(&self) -> &GovernanceAssessmentBinding {
        &self.governance_binding
    }

    pub(crate) const fn runtime_fact_snapshot(&self) -> &GovernanceRuntimeFactSnapshot {
        &self.runtime_fact_snapshot
    }

    pub(crate) const fn registration(&self) -> &GovernanceRuntimeFactSourceRegistration {
        &self.registration
    }

    pub(crate) const fn source(&self) -> &CoreOwnedAuthoritativeLocalCheckRuntimeFactSource {
        &self.source
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        Vec<LocalCheckResult>,
        GovernanceAssessmentBinding,
        GovernanceRuntimeFactSnapshot,
    ) {
        (
            self.results,
            self.governance_binding,
            self.runtime_fact_snapshot,
        )
    }
}

impl fmt::Debug for AuthoritativeLocalCheckRuntimeFactSourceBridgeOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthoritativeLocalCheckRuntimeFactSourceBridgeOutcome")
            .field("result_count", &self.results.len())
            .field("governance_binding", &self.governance_binding)
            .field("runtime_fact_snapshot", &self.runtime_fact_snapshot)
            .field("registration", &"[REDACTED]")
            .field("source", &"[REDACTED]")
            .field("results", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct CoreOwnedAuthoritativeLocalCheckRuntimeFactSource {
    source_id: GovernanceRuntimeFactSourceId,
    contract_version: GovernanceRuntimeFactSourceContractVersion,
    snapshot_id: GovernanceRuntimeFactSnapshotId,
    bundle_binding: crate::ImmutableRunBundleBinding,
    runtime_facts: Vec<StepGovernanceRuntimeFacts>,
}

impl fmt::Debug for CoreOwnedAuthoritativeLocalCheckRuntimeFactSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CoreOwnedAuthoritativeLocalCheckRuntimeFactSource")
            .field("source_id", &"[REDACTED]")
            .field("contract_version", &"[REDACTED]")
            .field("snapshot_id", &"[REDACTED]")
            .field("bundle_binding", &"[REDACTED]")
            .field("runtime_fact_count", &self.runtime_facts.len())
            .finish()
    }
}

impl GovernanceRuntimeFactSource for CoreOwnedAuthoritativeLocalCheckRuntimeFactSource {
    fn observe(
        &self,
        request: &GovernanceRuntimeFactSourceRequest<'_>,
    ) -> Result<GovernanceRuntimeFactObservation, WorkflowOsError> {
        if request.bundle_binding() != &self.bundle_binding {
            return Err(runtime_fact_source_bridge_error(
                "bundle_binding_mismatch",
                "Core-owned runtime fact source request does not match the immutable bundle",
            ));
        }
        GovernanceRuntimeFactObservation::new(GovernanceRuntimeFactObservationDefinition {
            source_id: self.source_id.clone(),
            contract_version: self.contract_version.clone(),
            snapshot_id: self.snapshot_id.clone(),
            bundle_binding: self.bundle_binding.clone(),
            observed_at: request.evaluated_at(),
            source_maximum_observation_age_seconds: 1,
            runtime_facts: self.runtime_facts.clone(),
        })
    }
}

pub(crate) fn compose_authoritative_local_check_runtime_fact_source_bridge(
    input: &AuthoritativeLocalCheckRuntimeFactSourceBridgeInput<'_>,
) -> Result<AuthoritativeLocalCheckRuntimeFactSourceBridgeOutcome, WorkflowOsError> {
    let reassessment = compose_authoritative_local_check_reassessment(&input.reassessment)?;
    let selected_fact_index = input
        .reassessment
        .runtime_facts
        .iter()
        .position(|fact| fact.step_id() == input.reassessment.local_check.step_id)
        .ok_or_else(|| {
            runtime_fact_source_bridge_error(
                "selected_runtime_fact_unresolved",
                "Core-owned runtime fact source selected fact is unavailable",
            )
        })?;
    let mut runtime_facts = input.reassessment.runtime_facts.to_vec();
    runtime_facts[selected_fact_index] = runtime_facts[selected_fact_index]
        .with_authoritative_evidence_and_checks(
            reassessment.bound_assessment().local_check_posture(),
        );

    let registration = core_owned_authoritative_local_check_runtime_fact_source_registration()?;
    let source = CoreOwnedAuthoritativeLocalCheckRuntimeFactSource {
        source_id: registration.source_id().clone(),
        contract_version: registration.contract_version().clone(),
        snapshot_id: GovernanceRuntimeFactSnapshotId::new(format!(
            "snapshot/{}",
            reassessment
                .bound_assessment()
                .binding_fingerprint()
                .as_str()
        ))?,
        bundle_binding: input
            .reassessment
            .local_check
            .stored_immutable_run_bundle
            .manifest()
            .run_binding(),
        runtime_facts,
    };
    let current = assess_immutable_bundle_governance_from_current_facts(
        &GovernanceRuntimeFactAssessmentRequest {
            bundle: input.reassessment.local_check.stored_immutable_run_bundle,
            profile: input.reassessment.profile,
            registration: &registration,
            source: &source,
            evaluated_at: input.evaluated_at,
        },
    )?;
    let (runtime_fact_snapshot, assessment_set) = current.into_parts();
    if assessment_set != reassessment.bound_assessment.assessment_set {
        return Err(runtime_fact_source_bridge_error(
            "equivalence_mismatch",
            "Core-owned runtime fact source assessment does not match authoritative reassessment",
        ));
    }
    let governance_binding = GovernanceAssessmentBinding::from_current_runtime_fact_assessment(
        input.reassessment.local_check.stored_immutable_run_bundle,
        &assessment_set,
        &runtime_fact_snapshot,
    )?;

    Ok(AuthoritativeLocalCheckRuntimeFactSourceBridgeOutcome {
        results: reassessment.results,
        governance_binding,
        runtime_fact_snapshot,
        registration,
        source,
    })
}

pub(crate) fn core_owned_authoritative_local_check_runtime_fact_source_registration(
) -> Result<GovernanceRuntimeFactSourceRegistration, WorkflowOsError> {
    GovernanceRuntimeFactSourceRegistration::new(
        GovernanceRuntimeFactSourceRegistrationDefinition {
            source_id: GovernanceRuntimeFactSourceId::new(
                "source/core-owned-authoritative-project-validation",
            )?,
            contract_version: GovernanceRuntimeFactSourceContractVersion::new("v1")?,
            configuration_commitment: SpecContentHash::from_text(
                "workflow-os/core-owned-authoritative-project-validation-source/v1",
            ),
            core_maximum_observation_age_seconds: 1,
        },
    )
}

impl AuthoritativeLocalCheckReassessmentOutcome {
    pub(crate) fn results(&self) -> &[LocalCheckResult] {
        &self.results
    }

    pub(crate) const fn bound_assessment(&self) -> &AuthoritativeLocalCheckBoundAssessment {
        &self.bound_assessment
    }

    pub(crate) fn into_parts(
        self,
        bundle: &StoredImmutableRunBundle,
        selected_step_id: StepId,
    ) -> Result<(Vec<LocalCheckResult>, crate::GovernanceAssessmentBinding), WorkflowOsError> {
        let binding = self
            .bound_assessment
            .into_governance_binding(bundle, selected_step_id)?;
        Ok((self.results, binding))
    }
}

impl fmt::Debug for AuthoritativeLocalCheckReassessmentOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthoritativeLocalCheckReassessmentOutcome")
            .field("result_count", &self.results.len())
            .field("bound_assessment", &self.bound_assessment)
            .field("results", &"[REDACTED]")
            .finish()
    }
}

pub(crate) struct AuthoritativeLocalCheckReassessmentPreflight {
    selected_fact_index: usize,
    candidate_set_fingerprint: SpecContentHash,
}

pub(crate) fn compose_authoritative_local_check_reassessment(
    input: &AuthoritativeLocalCheckReassessmentInput<'_>,
) -> Result<AuthoritativeLocalCheckReassessmentOutcome, WorkflowOsError> {
    let preflight = preflight_authoritative_local_check_reassessment(input)?;
    let local_check = compose_authoritative_docs_check_evidence_check_fact(&input.local_check)?;
    if local_check.fact().candidate_set_fingerprint() != &preflight.candidate_set_fingerprint {
        return Err(reassessment_binding_error(
            "candidate_mismatch",
            "authoritative local check reassessment candidate does not match preflight",
        ));
    }

    let mut runtime_facts = input.runtime_facts.to_vec();
    runtime_facts[preflight.selected_fact_index] = runtime_facts[preflight.selected_fact_index]
        .with_authoritative_evidence_and_checks(local_check.fact().posture());
    let assessment_set =
        assess_immutable_bundle_governance(&ImmutableBundleGovernanceAssessmentRequest {
            bundle: input.local_check.stored_immutable_run_bundle,
            profile: input.profile,
            runtime_facts: &runtime_facts,
        })?;
    let selected_assessment = assessment_set
        .assessments()
        .iter()
        .find(|assessment| assessment.step_id() == input.local_check.step_id)
        .ok_or_else(|| {
            reassessment_binding_error(
                "selected_assessment_unresolved",
                "authoritative local check reassessment selected assessment is unavailable",
            )
        })?;
    let binding_fingerprint = authoritative_local_check_reassessment_binding_fingerprint(
        input.local_check.stored_immutable_run_bundle,
        input.local_check.step_id,
        local_check.fact(),
        &assessment_set,
        selected_assessment.assessment(),
    );

    Ok(AuthoritativeLocalCheckReassessmentOutcome {
        results: local_check.results().to_vec(),
        bound_assessment: AuthoritativeLocalCheckBoundAssessment {
            algorithm: AuthoritativeLocalCheckReassessmentBindingAlgorithm::V1,
            local_check_fact: local_check.fact().clone(),
            assessment_set,
            binding_fingerprint,
        },
    })
}

pub(crate) fn preflight_authoritative_local_check_reassessment(
    input: &AuthoritativeLocalCheckReassessmentInput<'_>,
) -> Result<AuthoritativeLocalCheckReassessmentPreflight, WorkflowOsError> {
    preflight_immutable_bundle_governance(&ImmutableBundleGovernanceAssessmentRequest {
        bundle: input.local_check.stored_immutable_run_bundle,
        profile: input.profile,
        runtime_facts: input.runtime_facts,
    })?;
    let selected_fact_index = input
        .runtime_facts
        .iter()
        .position(|fact| fact.step_id() == input.local_check.step_id)
        .ok_or_else(|| {
            reassessment_binding_error(
                "selected_runtime_fact_unresolved",
                "authoritative local check reassessment selected runtime fact is unavailable",
            )
        })?;
    if input.runtime_facts[selected_fact_index]
        .evidence_and_checks()
        .is_some()
    {
        return Err(reassessment_binding_error(
            "selected_evidence_check_posture_supplied",
            "authoritative local check reassessment requires Core-derived check posture",
        ));
    }

    let candidate = adapt_stored_canonical_local_check_declarations(
        input.local_check.stored_immutable_run_bundle,
        input.local_check.step_id,
    )?;
    preflight_authoritative_docs_check_composition(&input.local_check, &candidate)?;
    Ok(AuthoritativeLocalCheckReassessmentPreflight {
        selected_fact_index,
        candidate_set_fingerprint: candidate.candidate_set_fingerprint().clone(),
    })
}

fn authoritative_local_check_reassessment_binding_fingerprint(
    bundle: &StoredImmutableRunBundle,
    step_id: &StepId,
    fact: &AuthoritativeLocalCheckEvidenceCheckFact,
    assessment_set: &ImmutableBundleGovernanceAssessmentSet,
    selected_assessment: &crate::ProportionalGovernanceWorkloadAssessment,
) -> SpecContentHash {
    let manifest = bundle.manifest();
    let binding = manifest.run_binding();
    let mut hasher = Sha256::new();
    for (label, value) in [
        (
            "domain",
            AuthoritativeLocalCheckReassessmentBindingAlgorithm::V1.identifier(),
        ),
        ("bundle_id", binding.bundle_id().as_str()),
        ("bundle_version", binding.bundle_version().as_str()),
        ("bundle_root", binding.root_hash().as_str()),
        ("workflow_id", manifest.workflow_id().as_str()),
        ("run_id", manifest.run_id().as_str()),
        ("step_id", step_id.as_str()),
        ("local_check_fact_algorithm", fact.algorithm().identifier()),
        (
            "local_check_fact_fingerprint",
            fact.fact_fingerprint().as_str(),
        ),
        (
            "local_check_candidate_set_fingerprint",
            fact.candidate_set_fingerprint().as_str(),
        ),
        (
            "local_check_structural_coverage_fingerprint",
            fact.structural_coverage_fingerprint().as_str(),
        ),
        (
            "selected_assessment_algorithm",
            selected_assessment.algorithm().identifier(),
        ),
        (
            "selected_assessment_input_fingerprint",
            selected_assessment.input_fingerprint().as_str(),
        ),
        (
            "assessment_set_algorithm",
            assessment_set.algorithm().identifier(),
        ),
        (
            "assessment_set_aggregate_fingerprint",
            assessment_set.aggregate_fingerprint().as_str(),
        ),
    ] {
        hash_reassessment_binding_field(&mut hasher, label, value);
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_reassessment_binding_field(hasher: &mut Sha256, label: &str, value: &str) {
    for part in [label.as_bytes(), value.as_bytes()] {
        let length = u64::try_from(part.len()).unwrap_or(u64::MAX);
        hasher.update(length.to_be_bytes());
        hasher.update(part);
    }
}

impl DocsCheckGovernanceContributionOutcome {
    pub(crate) const fn result(&self) -> &LocalCheckResult {
        &self.result
    }

    pub(crate) const fn contribution(&self) -> &DocsCheckGovernanceEvidenceCheckContribution {
        &self.contribution
    }
}

impl fmt::Debug for DocsCheckGovernanceContributionOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DocsCheckGovernanceContributionOutcome")
            .field("result_status", &self.result.status())
            .field("contribution", &self.contribution)
            .field("result", &"[REDACTED]")
            .finish()
    }
}

// Keep the authority-sensitive binding, execution, observation, and verification
// order linear so reviewers can audit that no process starts before binding.
#[allow(clippy::too_many_lines)]
pub(crate) fn execute_docs_check_with_attestation(
    input: &DocsCheckAttestationExecutionInput<'_>,
) -> Result<DocsCheckAttestationExecutionOutcome, WorkflowOsError> {
    execute_docs_check_with_attestation_inner(input)
}

#[allow(clippy::too_many_lines)]
fn execute_docs_check_with_attestation_inner(
    input: &DocsCheckAttestationExecutionInput<'_>,
) -> Result<DocsCheckAttestationExecutionOutcome, WorkflowOsError> {
    let manifest = input.stored_immutable_run_bundle.manifest();
    if &input.workflow_id != manifest.workflow_id() || &input.run_id != manifest.run_id() {
        return Err(runtime_error(
            "bundle_identity_mismatch",
            "local check attestation execution identity does not match the stored bundle",
        ));
    }

    resolve_stored_step_skill(input.stored_immutable_run_bundle, &input.step_id)?;
    let resolved_skill_id = SkillId::new(input.handler.skill_id())?;
    let resolved_skill_version = SkillVersion::new(input.handler.skill_version())?;

    let contract = input.handler.contract();
    contract.validate()?;
    if input.requirement.command_id() != contract.command_id() {
        return Err(runtime_error(
            "requirement_command_mismatch",
            "local check attestation requirement does not match the docs check command",
        ));
    }

    let handler_selection = ImmutableLocalCheckHandlerSelection::registered_unattested(
        contract.command_kind(),
        resolved_skill_id.clone(),
        resolved_skill_version.clone(),
    );
    let binding_created_at = input.clock.now()?;
    let execution_binding =
        ImmutableLocalCheckExecutionBinding::new(ImmutableLocalCheckExecutionBindingDefinition {
            immutable_run_bundle: manifest.run_binding(),
            workflow_id: input.workflow_id.clone(),
            run_id: input.run_id.clone(),
            step_id: input.step_id.clone(),
            skill_id: resolved_skill_id,
            skill_version: resolved_skill_version,
            command_contract: contract,
            handler_selection,
            created_at: binding_created_at,
        })?;

    let request = input.handler.build_process_request()?;
    let started_at = input.clock.now()?;
    ensure_clock_order(binding_created_at, started_at)?;
    let output = input.handler.run_process(&request)?;
    let completed_at = input.clock.now()?;
    ensure_clock_order(started_at, completed_at)?;
    let result = LocalCheckResult::from_process_output(contract, &output)?;
    let result_exit_posture = exit_posture(&result);
    let observation = KernelObservedLocalCheck::new(KernelObservedLocalCheckDefinition {
        workflow_id: input.workflow_id.clone(),
        run_id: input.run_id.clone(),
        step_id: input.step_id.clone(),
        invocation_id: input.invocation_id.clone(),
        idempotency_key: input.idempotency_key.clone(),
        immutable_run_bundle: manifest.run_binding(),
        command_contract_fingerprint: execution_binding.command_contract_fingerprint().clone(),
        handler_selection_fingerprint: execution_binding
            .handler_selection()
            .selection_fingerprint()
            .clone(),
        effective_policy_fingerprint: execution_binding.effective_policy_fingerprint().clone(),
        result_id: input.result_id.clone(),
        result_status: result.status(),
        exit_code_posture: result_exit_posture,
        duration_ms: result.duration_ms(),
        timed_out: matches!(result.status(), crate::LocalCheckResultStatus::TimedOut),
        stdout_truncated: result.stdout_truncated(),
        stderr_truncated: result.stderr_truncated(),
        started_at,
        completed_at,
    })?;
    let candidate = LocalCheckAttestationBinding::new(LocalCheckAttestationBindingDefinition {
        attestation_id: input.attestation_id.clone(),
        algorithm: LocalCheckAttestationAlgorithm::V1,
        assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
        source: LocalCheckAttestationSource::KernelLocalProcessRunner,
        command_id: contract.command_id().clone(),
        command_contract_fingerprint: execution_binding.command_contract_fingerprint().clone(),
        requirement_fingerprint: input.requirement.requirement_fingerprint().clone(),
        immutable_run_bundle: manifest.run_binding(),
        workflow_id: input.workflow_id.clone(),
        run_id: input.run_id.clone(),
        step_id: input.step_id.clone(),
        invocation_id: input.invocation_id.clone(),
        idempotency_key: input.idempotency_key.clone(),
        handler_fingerprint: execution_binding
            .handler_selection()
            .selection_fingerprint()
            .clone(),
        result_id: input.result_id.clone(),
        result_status: result.status(),
        exit_code_posture: result_exit_posture,
        stdout_truncated: result.stdout_truncated(),
        stderr_truncated: result.stderr_truncated(),
        observed_started_at: started_at,
        observed_completed_at: completed_at,
        freshness: input.requirement.freshness(),
    })?;

    if !input
        .requirement
        .accepted_statuses()
        .contains(&result.status())
    {
        return Ok(DocsCheckAttestationExecutionOutcome {
            result,
            accepted_attestation: None,
        });
    }

    let evaluated_at = input.clock.now()?;
    ensure_clock_order(completed_at, evaluated_at)?;
    let accepted_attestation =
        verify_local_check_attestation(&LocalCheckAttestationVerificationInput {
            requirement: input.requirement,
            candidate: &candidate,
            stored_immutable_run_bundle: input.stored_immutable_run_bundle,
            execution_binding: &execution_binding,
            command_contract: contract,
            observation: &observation,
            result: &result,
            evaluated_at,
        })?;

    Ok(DocsCheckAttestationExecutionOutcome {
        result,
        accepted_attestation: Some(accepted_attestation),
    })
}

pub(crate) fn execute_docs_check_attestation_gate(
    input: &DocsCheckAttestationExecutionInput<'_>,
) -> Result<DocsCheckAttestationGateOutcome, WorkflowOsError> {
    let execution = execute_docs_check_with_attestation_inner(input)?;
    let DocsCheckAttestationExecutionOutcome {
        result,
        accepted_attestation,
    } = execution;

    if !input
        .requirement
        .accepted_statuses()
        .contains(&result.status())
    {
        if accepted_attestation.is_some() {
            return Err(gate_error(
                "proof_unexpected",
                "local check attestation gate proof posture is invalid",
            ));
        }
        return Ok(DocsCheckAttestationGateOutcome {
            result,
            disposition: DocsCheckAttestationGateDisposition::NotSatisfied(
                DocsCheckAttestationGateReason::ResultStatusNotAccepted,
            ),
            proof_fingerprint: None,
        });
    }

    let accepted = accepted_attestation.as_ref().ok_or_else(|| {
        gate_error(
            "proof_unavailable",
            "local check attestation gate proof is unavailable",
        )
    })?;
    ensure_gate_context(input, &result, accepted)?;

    let consumed_at = input.clock.now()?;
    if consumed_at < *accepted.verified_at() || consumed_at < *accepted.observed_completed_at() {
        return Err(gate_error(
            "clock_order_invalid",
            "local check attestation gate clock ordering is invalid",
        ));
    }

    if let super::LocalCheckAttestationFreshnessPolicy::MaxAgeSeconds { seconds } =
        input.requirement.freshness()
    {
        let age = consumed_at.as_offset_date_time()
            - accepted.observed_completed_at().as_offset_date_time();
        if age > time::Duration::seconds(i64::from(seconds)) {
            return Ok(DocsCheckAttestationGateOutcome {
                result,
                disposition: DocsCheckAttestationGateDisposition::NotSatisfied(
                    DocsCheckAttestationGateReason::FreshnessExpired,
                ),
                proof_fingerprint: None,
            });
        }
    }

    Ok(DocsCheckAttestationGateOutcome {
        result,
        disposition: DocsCheckAttestationGateDisposition::Satisfied,
        proof_fingerprint: Some(accepted.proof_fingerprint().clone()),
    })
}

pub(crate) fn execute_docs_check_governance_contribution(
    input: &DocsCheckAttestationExecutionInput<'_>,
) -> Result<DocsCheckGovernanceContributionOutcome, WorkflowOsError> {
    let gate = execute_docs_check_attestation_gate(input)?;
    let posture = match gate.disposition() {
        DocsCheckAttestationGateDisposition::Satisfied => {
            GovernanceEvidenceCheckContributionPosture::Satisfied
        }
        DocsCheckAttestationGateDisposition::NotSatisfied(
            DocsCheckAttestationGateReason::ResultStatusNotAccepted,
        ) => GovernanceEvidenceCheckContributionPosture::Failed,
        DocsCheckAttestationGateDisposition::NotSatisfied(
            DocsCheckAttestationGateReason::FreshnessExpired,
        ) => GovernanceEvidenceCheckContributionPosture::RequiredUnavailable,
    };
    let obligation_fingerprint = governance_obligation_fingerprint(input);

    Ok(DocsCheckGovernanceContributionOutcome {
        result: gate.result,
        contribution: DocsCheckGovernanceEvidenceCheckContribution {
            obligation_fingerprint,
            posture,
        },
    })
}

pub(crate) fn compose_authoritative_docs_check_evidence_check_fact(
    input: &AuthoritativeDocsCheckCompositionInput<'_>,
) -> Result<AuthoritativeDocsCheckCompositionOutcome, WorkflowOsError> {
    let candidate = adapt_stored_canonical_local_check_declarations(
        input.stored_immutable_run_bundle,
        input.step_id,
    )?;
    let executions = preflight_authoritative_docs_check_composition(input, &candidate)?;
    let mut results = Vec::with_capacity(executions.len());
    let mut contributions = Vec::with_capacity(executions.len());
    for execution in executions {
        let outcome = execute_docs_check_governance_contribution(execution)?;
        results.push(outcome.result().clone());
        contributions.push(adapt_authoritative_docs_check_contribution(
            &candidate,
            outcome.contribution(),
        )?);
    }
    let coverage = evaluate_local_check_structural_coverage(&candidate, &contributions)?;
    let fact = convert_authoritative_local_check_coverage(&coverage)?;
    Ok(AuthoritativeDocsCheckCompositionOutcome { results, fact })
}

fn preflight_authoritative_docs_check_composition<'a>(
    input: &'a AuthoritativeDocsCheckCompositionInput<'a>,
    candidate: &LocalCheckGovernanceObligationSetCandidate,
) -> Result<Vec<&'a DocsCheckAttestationExecutionInput<'a>>, WorkflowOsError> {
    let manifest = input.stored_immutable_run_bundle.manifest();
    let record = authoritative_record_for_step(input.stored_immutable_run_bundle, input.step_id)?;
    let mut by_obligation = BTreeMap::new();

    for execution in input.executions {
        if execution.stored_immutable_run_bundle != input.stored_immutable_run_bundle {
            return Err(composition_error(
                "bundle_mismatch",
                "local check composition input does not match the authoritative bundle",
            ));
        }
        if &execution.workflow_id != manifest.workflow_id()
            || &execution.run_id != manifest.run_id()
            || &execution.step_id != input.step_id
        {
            return Err(composition_error(
                "execution_context_mismatch",
                "local check composition execution context does not match",
            ));
        }

        let requirement_fingerprint = execution.requirement.requirement_fingerprint();
        let declaration = record
            .declarations()
            .iter()
            .find(|declaration| {
                declaration.attestation_requirement_fingerprint() == requirement_fingerprint
            })
            .ok_or_else(|| {
                composition_error(
                    "requirement_unexpected",
                    "local check composition contains an unexpected requirement",
                )
            })?;
        let contract = execution.handler.contract();
        contract.validate().map_err(|_| {
            composition_error(
                "command_contract_invalid",
                "local check composition contains an invalid command contract",
            )
        })?;
        if declaration.command_id() != execution.requirement.command_id()
            || declaration.command_id() != contract.command_id()
            || declaration.command_kind() != contract.command_kind()
            || declaration.command_contract_fingerprint()
                != &compute_local_check_command_contract_fingerprint(contract)
        {
            return Err(composition_error(
                "command_contract_mismatch",
                "local check composition command contract does not match",
            ));
        }

        let obligation = candidate
            .obligations()
            .iter()
            .find(|obligation| obligation.requirement_fingerprint() == requirement_fingerprint)
            .ok_or_else(|| {
                composition_error(
                    "obligation_unexpected",
                    "local check composition contains an unexpected obligation",
                )
            })?;
        if by_obligation
            .insert(obligation.obligation_fingerprint().clone(), execution)
            .is_some()
        {
            return Err(composition_error(
                "execution_duplicate",
                "local check composition repeats an obligation execution",
            ));
        }
    }

    Ok(candidate
        .obligations()
        .iter()
        .filter_map(|obligation| by_obligation.remove(obligation.obligation_fingerprint()))
        .collect())
}

fn governance_obligation_fingerprint(
    input: &DocsCheckAttestationExecutionInput<'_>,
) -> SpecContentHash {
    let binding = input.stored_immutable_run_bundle.manifest().run_binding();
    docs_check_governance_obligation_fingerprint(
        binding.bundle_id(),
        binding.bundle_version(),
        binding.root_hash(),
        &input.step_id,
        input.requirement.requirement_fingerprint(),
    )
}

pub(super) fn docs_check_governance_obligation_fingerprint(
    bundle_id: &ImmutableRunBundleId,
    bundle_version: &ImmutableRunBundleVersion,
    bundle_root: &SpecContentHash,
    step_id: &StepId,
    requirement_fingerprint: &SpecContentHash,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_governance_obligation_field(
        &mut hasher,
        "algorithm",
        "workflow-os/docs-check-governance-contribution/v1",
    );
    hash_governance_obligation_field(&mut hasher, "bundle_id", bundle_id.as_str());
    hash_governance_obligation_field(&mut hasher, "bundle_version", bundle_version.as_str());
    hash_governance_obligation_field(&mut hasher, "bundle_root", bundle_root.as_str());
    hash_governance_obligation_field(&mut hasher, "step_id", step_id.as_str());
    hash_governance_obligation_field(
        &mut hasher,
        "requirement_fingerprint",
        requirement_fingerprint.as_str(),
    );
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_governance_obligation_field(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update(u64::try_from(label.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(value.as_bytes());
}

fn ensure_gate_context(
    input: &DocsCheckAttestationExecutionInput<'_>,
    result: &LocalCheckResult,
    accepted: &AcceptedLocalCheckAttestation,
) -> Result<(), WorkflowOsError> {
    let manifest = input.stored_immutable_run_bundle.manifest();
    let skill_id = SkillId::new(input.handler.skill_id())?;
    let skill_version = SkillVersion::new(input.handler.skill_version())?;
    let expected_handler = ImmutableLocalCheckHandlerSelection::registered_unattested(
        input.handler.contract().command_kind(),
        skill_id,
        skill_version,
    );
    if accepted.requirement_fingerprint() != input.requirement.requirement_fingerprint()
        || accepted.assurance() < input.requirement.minimum_assurance()
        || accepted.immutable_run_bundle() != &manifest.run_binding()
        || accepted.workflow_id() != &input.workflow_id
        || accepted.run_id() != &input.run_id
        || accepted.step_id() != &input.step_id
        || accepted.invocation_id() != &input.invocation_id
        || accepted.result_id() != &input.result_id
        || accepted.result_status() != result.status()
        || accepted.handler_selection_fingerprint() != expected_handler.selection_fingerprint()
        || accepted.freshness() != input.requirement.freshness()
        || (!input.requirement.truncation_allowed()
            && (accepted.stdout_truncated() || accepted.stderr_truncated()))
    {
        return Err(gate_error(
            "proof_context_mismatch",
            "local check attestation gate proof context does not match",
        ));
    }
    Ok(())
}

fn ensure_clock_order(earlier: Timestamp, later: Timestamp) -> Result<(), WorkflowOsError> {
    if earlier > later {
        return Err(runtime_error(
            "clock_order_invalid",
            "local check attestation clock ordering is invalid",
        ));
    }
    Ok(())
}

fn resolve_stored_step_skill(
    bundle: &StoredImmutableRunBundle,
    step_id: &StepId,
) -> Result<(SkillId, SkillVersion), WorkflowOsError> {
    let manifest = bundle.manifest();
    let workflow_record = exactly_one_record(
        bundle
            .definition_records()
            .iter()
            .filter(|record| record.kind() == ImmutableRunBundleDefinitionKind::Workflow)
            .filter(|record| record.definition_id() == manifest.workflow_id().as_str())
            .filter(|record| record.source_content_hash() == manifest.workflow_content_hash()),
        "workflow_unresolved",
        "local check attestation stored workflow could not be resolved",
    )?;
    let workflow = workflow_record
        .canonical_definition()
        .as_workflow()
        .ok_or_else(|| {
            runtime_error(
                "workflow_unresolved",
                "local check attestation stored workflow could not be resolved",
            )
        })?;
    if &workflow.version != manifest.workflow_version() {
        return Err(runtime_error(
            "workflow_mismatch",
            "local check attestation stored workflow does not match the manifest",
        ));
    }

    let mut matching_steps = workflow.steps.iter().filter(|step| &step.id == step_id);
    let step = matching_steps.next().ok_or_else(|| {
        runtime_error(
            "step_unresolved",
            "local check attestation step could not be resolved from the stored workflow",
        )
    })?;
    if matching_steps.next().is_some() {
        return Err(runtime_error(
            "step_unresolved",
            "local check attestation step could not be resolved from the stored workflow",
        ));
    }

    let definition_reference = {
        let mut references = manifest
            .definitions()
            .iter()
            .filter(|reference| reference.kind() == ImmutableRunBundleDefinitionKind::Skill)
            .filter(|reference| reference.step_id() == Some(step_id));
        let reference = references.next().ok_or_else(|| {
            runtime_error(
                "skill_unresolved",
                "local check attestation skill could not be resolved from the stored bundle",
            )
        })?;
        if references.next().is_some() {
            return Err(runtime_error(
                "skill_unresolved",
                "local check attestation skill could not be resolved from the stored bundle",
            ));
        }
        reference
    };
    let skill_record = exactly_one_record(
        bundle
            .definition_records()
            .iter()
            .filter(|record| record.kind() == ImmutableRunBundleDefinitionKind::Skill)
            .filter(|record| record.definition_id() == definition_reference.definition_id())
            .filter(|record| record.source_content_hash() == definition_reference.content_hash()),
        "skill_unresolved",
        "local check attestation skill could not be resolved from the stored bundle",
    )?;
    let skill = skill_record
        .canonical_definition()
        .as_skill()
        .ok_or_else(|| {
            runtime_error(
                "skill_unresolved",
                "local check attestation skill could not be resolved from the stored bundle",
            )
        })?;
    if skill.id != step.skill_ref.id
        || step
            .skill_ref
            .version
            .as_ref()
            .is_some_and(|version| version != &skill.version)
        || skill.id.as_str() != definition_reference.definition_id()
        || definition_reference
            .definition_version()
            .is_some_and(|version| version != skill.version.as_str())
    {
        return Err(runtime_error(
            "skill_mismatch",
            "local check attestation stored skill does not match the selected workflow step",
        ));
    }

    Ok((skill.id.clone(), skill.version.clone()))
}

fn exactly_one_record<'a>(
    mut records: impl Iterator<Item = &'a ImmutableRunBundleDefinitionRecord>,
    suffix: &str,
    message: &'static str,
) -> Result<&'a ImmutableRunBundleDefinitionRecord, WorkflowOsError> {
    let record = records
        .next()
        .ok_or_else(|| runtime_error(suffix, message))?;
    if records.next().is_some() {
        return Err(runtime_error(suffix, message));
    }
    Ok(record)
}

fn runtime_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("local_check_attestation.runtime.{suffix}"), message)
}

fn gate_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("local_check_attestation.gate.{suffix}"), message)
}

fn composition_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("local_check_attestation.composition.{suffix}"),
        message,
    )
}

fn reassessment_binding_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("local_check_attestation.reassessment_binding.{suffix}"),
        message,
    )
}

fn runtime_fact_source_bridge_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("local_check_attestation.runtime_fact_source_bridge.{suffix}"),
        message,
    )
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::too_many_lines)]
mod tests {
    use std::collections::VecDeque;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::local_check_attestation::structural_coverage::{
        adapt_docs_check_contribution, LocalCheckGovernanceContributionPosture,
        LocalCheckGovernanceObligationDefinition, LocalCheckGovernanceObligationSetCandidate,
        LocalCheckGovernanceObligationSetCandidateDefinition, LocalCheckGovernanceRequirementLevel,
    };
    use crate::{
        build_immutable_run_bundle_with_local_check_declarations, load_project, ActorId,
        DocsCheckLocalHandler, GovernanceDisclosureRequirement, GovernanceExecutionDisposition,
        GovernanceStrictnessProfile, GovernanceWorkloadAuthorityPosture,
        GovernanceWorkloadEvidenceCheckPosture, GovernanceWorkloadSideEffectPosture,
        ImmutableRunBundleBuildRequest, ImmutableRunBundleExecutionPosture,
        ImmutableRunBundleHandlerPosture, ImmutableRunBundleHandlerReference, ImmutableRunBundleId,
        ImmutableRunBundleReferencePosture, ImmutableRunBundleSensitivity,
        ImmutableRunBundleVersion, LocalCheckAttestationFreshnessPolicy,
        LocalCheckAttestationRequirementDefinition, LocalCheckCommandContract,
        LocalCheckCommandContractInventory, LocalCheckProcessOutput, LocalCheckProcessRequest,
        LocalCheckProcessRunner, LocalCheckResultStatus, LocalImmutableRunBundleStore,
        WorkflowOsErrorKind, SUPPORTED_SCHEMA_VERSION,
    };

    static NEXT_TEST_ROOT: AtomicUsize = AtomicUsize::new(1);

    struct TestRoot {
        path: PathBuf,
    }

    impl TestRoot {
        fn new(label: &str) -> Self {
            let sequence = NEXT_TEST_ROOT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "workflow-os-attestation-runtime-{label}-{}-{}-{sequence}",
                std::process::id(),
                Timestamp::now_utc()
                    .as_offset_date_time()
                    .unix_timestamp_nanos()
            ));
            fs::create_dir_all(&path).expect("test root created");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn write(&self, relative: &str, content: &str) {
            let path = self.path.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("parent created");
            }
            fs::write(path, content).expect("fixture written");
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[derive(Debug)]
    struct ScriptedClock {
        samples: Mutex<VecDeque<Result<Timestamp, WorkflowOsError>>>,
        calls: Arc<AtomicUsize>,
    }

    impl ScriptedClock {
        fn new(samples: Vec<Result<Timestamp, WorkflowOsError>>) -> (Self, Arc<AtomicUsize>) {
            let calls = Arc::new(AtomicUsize::new(0));
            (
                Self {
                    samples: Mutex::new(samples.into()),
                    calls: calls.clone(),
                },
                calls,
            )
        }
    }

    impl LocalCheckObservationClock for ScriptedClock {
        fn now(&self) -> Result<Timestamp, WorkflowOsError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.samples
                .lock()
                .expect("clock lock")
                .pop_front()
                .expect("scripted clock sample")
        }
    }

    #[derive(Debug)]
    struct RecordingRunner {
        output: Result<LocalCheckProcessOutput, WorkflowOsError>,
        calls: Arc<AtomicUsize>,
        clock_calls: Arc<AtomicUsize>,
    }

    impl LocalCheckProcessRunner for RecordingRunner {
        fn run(
            &self,
            _request: &LocalCheckProcessRequest,
        ) -> Result<LocalCheckProcessOutput, WorkflowOsError> {
            assert_eq!(
                self.clock_calls.load(Ordering::SeqCst),
                2,
                "binding creation and process-start time must precede the runner"
            );
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.output.clone()
        }
    }

    struct Fixture {
        project_root: TestRoot,
        handler: DocsCheckLocalHandler,
        stored_bundle: StoredImmutableRunBundle,
        requirement: LocalCheckAttestationRequirement,
        runner_calls: Arc<AtomicUsize>,
    }

    fn timestamp(value: &str) -> Result<Timestamp, WorkflowOsError> {
        Timestamp::parse_rfc3339(value)
    }

    fn fixture(
        output: Result<LocalCheckProcessOutput, WorkflowOsError>,
        clock_calls: Arc<AtomicUsize>,
    ) -> Fixture {
        fixture_with_level(output, clock_calls, "required")
    }

    fn fixture_with_level(
        output: Result<LocalCheckProcessOutput, WorkflowOsError>,
        clock_calls: Arc<AtomicUsize>,
        requirement_level: &str,
    ) -> Fixture {
        fixture_with_requirement_level(output, clock_calls, Some(requirement_level))
    }

    fn fixture_without_requirements(
        output: Result<LocalCheckProcessOutput, WorkflowOsError>,
        clock_calls: Arc<AtomicUsize>,
    ) -> Fixture {
        fixture_with_requirement_level(output, clock_calls, None)
    }

    fn fixture_with_requirement_level(
        output: Result<LocalCheckProcessOutput, WorkflowOsError>,
        clock_calls: Arc<AtomicUsize>,
        requirement_level: Option<&str>,
    ) -> Fixture {
        let project = TestRoot::new("project");
        let storage = TestRoot::new("storage");
        let local_check_requirements = requirement_level.map_or_else(String::new, |level| {
            format!(
                "    local_check_requirements:\n      - id: docs-required\n        command_id: local-check/docs\n        requirement_level: {level}\n        minimum_assurance: kernel_observed_local_process\n        accepted_statuses: [passed]\n        freshness:\n          mode: no_reuse\n        exact_immutable_run_binding_required: true\n        truncation_allowed: false\n        network_maximum: disabled\n        side_effect_maximum: no_source_writes\n"
            )
        });
        project.write(
            "workflow-os.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nproject:\n  id: runtime/project\n  name: Runtime Project\n"
            ),
        );
        project.write(
            "workflows/check.workflow.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: workflow/test\nversion: v1\ndisplay_name: Check Workflow\ntriggers:\n  - id: manual-start\n    kind: manual\nsteps:\n  - id: check-docs\n    skill_ref:\n      id: local/check-docs\n      version: v0\n    policy_requirements:\n      - id: local/read-only\n{local_check_requirements}    terminal_behavior: fail_workflow\ncancellation_behavior: stop\naudit_requirements:\n  required: true\n  events: [RunCreated, RunCompleted]\n  store_references_only: true\nobservability_requirements:\n  metrics: [workflow_latency]\n  tracing: true\n  latency_tracking: true\n"
            ),
        );
        project.write(
            "skills/check.skill.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/check-docs\nversion: v0\ndisplay_name: Check Docs\nallowed_capabilities:\n  - name: local.read\ninput_contract:\n  fields:\n    - name: request\n      field_type: string\noutput_contract:\n  fields:\n    - name: summary\n      field_type: string\nfailure_modes:\n  - code: check_failed\n    description: Check failed.\n    retryable: false\naudit_requirements:\n  required: true\n  events: [SkillInvocationRequested]\n  store_references_only: true\nobservability_requirements:\n  metrics: [skill_latency]\n  tracing: true\n  latency_tracking: true\n"
            ),
        );
        project.write(
            "policies/read-only.policy.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/read-only\nname: Read Only\nrules:\n  - id: allow-local\n    effect: allow_local\n"
            ),
        );
        project.write("package.json", "{}\n");
        project.write("scripts/check-docs.mjs", "// fixture\n");
        project.write("bin/npm", "#!/bin/sh\n");

        let loaded = load_project(project.path());
        assert!(!loaded.has_errors(), "{:?}", loaded.diagnostics);
        let project_bundle = loaded.bundle.expect("loaded project");
        let request = ImmutableRunBundleBuildRequest {
            project: &project_bundle,
            workflow_id: &WorkflowId::new("workflow/test").expect("workflow id"),
            bundle_id: ImmutableRunBundleId::new("bundle/test").expect("bundle id"),
            bundle_version: ImmutableRunBundleVersion::new("v1").expect("bundle version"),
            run_id: WorkflowRunId::new("run-test").expect("run id"),
            resolved_execution_context_hash: SpecContentHash::from_text("resolved context"),
            execution_posture: ImmutableRunBundleExecutionPosture::new(
                Vec::new(),
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::NotSupplied,
            )
            .expect("execution posture"),
            handlers: vec![ImmutableRunBundleHandlerReference {
                skill_id: SkillId::new("local/check-docs").expect("skill id"),
                skill_version: SkillVersion::new("v0").expect("skill version"),
                posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
            }],
            created_at: timestamp("2026-07-19T11:59:59Z").expect("bundle time"),
            created_by: ActorId::new("system/kernel").expect("actor"),
            sensitivity: ImmutableRunBundleSensitivity::Internal,
            redaction_required: true,
        };
        let inventory = LocalCheckCommandContractInventory::new(vec![
            LocalCheckCommandContract::docs_check_model_only().expect("contract"),
        ])
        .expect("inventory");
        let built = build_immutable_run_bundle_with_local_check_declarations(request, &inventory)
            .expect("bundle built");
        let store = LocalImmutableRunBundleStore::new(storage.path());
        store.write_bundle(&built).expect("bundle written");
        let stored_bundle = store
            .read_bundle(built.manifest().run_id(), built.manifest().bundle_id())
            .expect("bundle read");

        let runner_calls = Arc::new(AtomicUsize::new(0));
        let handler = DocsCheckLocalHandler::new_with_process_runner(
            LocalCheckCommandContract::docs_check_model_only().expect("contract"),
            project.path().join("bin/npm"),
            project.path().to_path_buf(),
            None,
            Arc::new(RecordingRunner {
                output,
                calls: runner_calls.clone(),
                clock_calls,
            }),
        )
        .expect("handler");
        let requirement =
            LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
                command_id: handler.contract().command_id().clone(),
                minimum_assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
                accepted_statuses: vec![LocalCheckResultStatus::Passed],
                freshness: LocalCheckAttestationFreshnessPolicy::NoReuse,
                exact_immutable_run_binding_required: true,
                truncation_allowed: false,
            })
            .expect("requirement");

        Fixture {
            project_root: project,
            handler,
            stored_bundle,
            requirement,
            runner_calls,
        }
    }

    fn input<'a>(
        fixture: &'a Fixture,
        clock: &'a dyn LocalCheckObservationClock,
    ) -> DocsCheckAttestationExecutionInput<'a> {
        DocsCheckAttestationExecutionInput {
            stored_immutable_run_bundle: &fixture.stored_bundle,
            requirement: &fixture.requirement,
            handler: &fixture.handler,
            workflow_id: WorkflowId::new("workflow/test").expect("workflow id"),
            run_id: WorkflowRunId::new("run-test").expect("run id"),
            step_id: StepId::new("check-docs").expect("step id"),
            invocation_id: SkillInvocationId::new("invocation/check-docs").expect("invocation id"),
            idempotency_key: IdempotencyKey::new("idempotency/check-docs")
                .expect("idempotency key"),
            result_id: LocalCheckResultId::new("result/check-docs").expect("result id"),
            attestation_id: LocalCheckAttestationId::new("attestation/check-docs")
                .expect("attestation id"),
            clock,
        }
    }

    fn four_samples() -> Vec<Result<Timestamp, WorkflowOsError>> {
        [
            "2026-07-19T12:00:00Z",
            "2026-07-19T12:00:01Z",
            "2026-07-19T12:00:02Z",
            "2026-07-19T12:00:03Z",
        ]
        .into_iter()
        .map(timestamp)
        .collect()
    }

    fn five_samples() -> Vec<Result<Timestamp, WorkflowOsError>> {
        [
            "2026-07-19T12:00:00Z",
            "2026-07-19T12:00:01Z",
            "2026-07-19T12:00:02Z",
            "2026-07-19T12:00:03Z",
            "2026-07-19T12:00:04Z",
        ]
        .into_iter()
        .map(timestamp)
        .collect()
    }

    fn reassessment_fact(
        evidence_and_checks: Option<GovernanceWorkloadEvidenceCheckPosture>,
    ) -> StepGovernanceRuntimeFacts {
        StepGovernanceRuntimeFacts::new(
            StepId::new("check-docs").expect("step id"),
            Some(GovernanceWorkloadAuthorityPosture::Sufficient),
            evidence_and_checks,
            Some(GovernanceWorkloadSideEffectPosture::None),
            None,
            None,
            None,
        )
    }

    #[test]
    fn passed_docs_check_returns_structured_result_and_accepted_proof() {
        let (clock, clock_calls) = ScriptedClock::new(four_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                b"documentation check passed".to_vec(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );

        let outcome = execute_docs_check_with_attestation(&input(&fixture, &clock))
            .expect("composition succeeds");

        assert_eq!(outcome.result().status(), LocalCheckResultStatus::Passed);
        let accepted = outcome.accepted_attestation().expect("accepted proof");
        assert_eq!(
            accepted.workflow_id(),
            fixture.stored_bundle.manifest().workflow_id()
        );
        assert_eq!(accepted.run_id(), fixture.stored_bundle.manifest().run_id());
        assert_eq!(accepted.result_status(), LocalCheckResultStatus::Passed);
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 1);
        assert_eq!(clock_calls.load(Ordering::SeqCst), 4);
    }

    #[test]
    fn passed_current_invocation_satisfies_gate_without_exposing_proof() {
        let (clock, clock_calls) = ScriptedClock::new(five_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                b"documentation check passed".to_vec(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );

        let outcome = execute_docs_check_attestation_gate(&input(&fixture, &clock))
            .expect("gate evaluation succeeds");

        assert_eq!(outcome.result().status(), LocalCheckResultStatus::Passed);
        assert_eq!(
            outcome.disposition(),
            DocsCheckAttestationGateDisposition::Satisfied
        );
        assert!(outcome.proof_fingerprint().is_some());
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 1);
        assert_eq!(clock_calls.load(Ordering::SeqCst), 5);

        let debug = format!("{outcome:?}");
        assert!(debug.contains("Satisfied"));
        assert!(debug.contains("proof_present: true"));
        assert!(!debug.contains("workflow/test"));
        assert!(!debug.contains("run-test"));
        assert!(!debug.contains("invocation/check-docs"));
        assert!(!debug.contains(
            outcome
                .proof_fingerprint()
                .expect("proof fingerprint")
                .as_str()
        ));
    }

    #[test]
    fn unaccepted_result_status_is_typed_not_satisfied_without_proof() {
        for output in [
            LocalCheckProcessOutput::completed(
                Some(1),
                false,
                1_000,
                Vec::new(),
                b"check failed".to_vec(),
            ),
            LocalCheckProcessOutput::timed_out(1_000, Vec::new(), Vec::new()),
        ] {
            let samples = five_samples().into_iter().take(3).collect();
            let (clock, clock_calls) = ScriptedClock::new(samples);
            let fixture = fixture(Ok(output), clock_calls.clone());

            let outcome = execute_docs_check_attestation_gate(&input(&fixture, &clock))
                .expect("gate outcome returned");

            assert_eq!(
                outcome.disposition(),
                DocsCheckAttestationGateDisposition::NotSatisfied(
                    DocsCheckAttestationGateReason::ResultStatusNotAccepted
                )
            );
            assert!(outcome.proof_fingerprint().is_none());
            assert_eq!(clock_calls.load(Ordering::SeqCst), 3);
            assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 1);
        }
    }

    #[test]
    fn maximum_age_is_reevaluated_at_gate_consumption() {
        let (clock, clock_calls) = ScriptedClock::new(
            [
                "2026-07-19T12:00:00Z",
                "2026-07-19T12:00:01Z",
                "2026-07-19T12:00:02Z",
                "2026-07-19T12:00:02Z",
                "2026-07-19T12:00:04Z",
            ]
            .into_iter()
            .map(timestamp)
            .collect(),
        );
        let mut fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );
        fixture.requirement =
            LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
                command_id: fixture.handler.contract().command_id().clone(),
                minimum_assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
                accepted_statuses: vec![LocalCheckResultStatus::Passed],
                freshness: LocalCheckAttestationFreshnessPolicy::max_age_seconds(1)
                    .expect("freshness"),
                exact_immutable_run_binding_required: true,
                truncation_allowed: false,
            })
            .expect("requirement");

        let outcome = execute_docs_check_attestation_gate(&input(&fixture, &clock))
            .expect("expired proof is typed gate outcome");

        assert_eq!(
            outcome.disposition(),
            DocsCheckAttestationGateDisposition::NotSatisfied(
                DocsCheckAttestationGateReason::FreshnessExpired
            )
        );
        assert!(outcome.proof_fingerprint().is_none());
        assert_eq!(clock_calls.load(Ordering::SeqCst), 5);
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn regressing_consumption_clock_fails_without_satisfied_outcome() {
        let (clock, clock_calls) = ScriptedClock::new(
            [
                "2026-07-19T12:00:00Z",
                "2026-07-19T12:00:01Z",
                "2026-07-19T12:00:02Z",
                "2026-07-19T12:00:03Z",
                "2026-07-19T12:00:02Z",
            ]
            .into_iter()
            .map(timestamp)
            .collect(),
        );
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );

        let error = execute_docs_check_attestation_gate(&input(&fixture, &clock))
            .expect_err("regressing consumption time rejected");

        assert_eq!(
            error.code(),
            "local_check_attestation.gate.clock_order_invalid"
        );
        assert!(!error.to_string().contains("workflow/test"));
        assert!(!error.to_string().contains("run-test"));
        assert_eq!(clock_calls.load(Ordering::SeqCst), 5);
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn passed_gate_maps_to_requirement_scoped_satisfied_contribution() {
        let (clock, clock_calls) = ScriptedClock::new(five_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                b"documentation check passed".to_vec(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );

        let outcome = execute_docs_check_governance_contribution(&input(&fixture, &clock))
            .expect("contribution succeeds");

        assert_eq!(outcome.result().status(), LocalCheckResultStatus::Passed);
        assert_eq!(
            outcome.contribution().posture(),
            GovernanceEvidenceCheckContributionPosture::Satisfied
        );
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 1);
        assert_eq!(clock_calls.load(Ordering::SeqCst), 5);

        let manifest = fixture.stored_bundle.manifest();
        let candidate_set = LocalCheckGovernanceObligationSetCandidate::new(
            LocalCheckGovernanceObligationSetCandidateDefinition {
                bundle_id: manifest.bundle_id().clone(),
                bundle_version: manifest.bundle_version().clone(),
                bundle_root: manifest.root_hash().clone(),
                workflow_id: manifest.workflow_id().clone(),
                workflow_version: manifest.workflow_version().clone(),
                run_id: manifest.run_id().clone(),
                step_id: StepId::new("check-docs").expect("step id"),
                obligations: vec![LocalCheckGovernanceObligationDefinition::new(
                    fixture.requirement.requirement_fingerprint().clone(),
                    LocalCheckGovernanceRequirementLevel::Required,
                )],
            },
        )
        .expect("candidate set");
        let adapted = adapt_docs_check_contribution(
            &candidate_set,
            outcome.contribution(),
            LocalCheckGovernanceRequirementLevel::Required,
        )
        .expect("same-call contribution adapts");
        assert_eq!(
            adapted.posture(),
            LocalCheckGovernanceContributionPosture::Satisfied
        );
        assert_eq!(
            adapted.obligation_fingerprint(),
            outcome.contribution().obligation_fingerprint()
        );

        let relabeled_candidate = LocalCheckGovernanceObligationSetCandidate::new(
            LocalCheckGovernanceObligationSetCandidateDefinition {
                bundle_id: ImmutableRunBundleId::new("bundle/relabeled").expect("bundle id"),
                bundle_version: manifest.bundle_version().clone(),
                bundle_root: SpecContentHash::from_text("relabeled bundle root"),
                workflow_id: manifest.workflow_id().clone(),
                workflow_version: manifest.workflow_version().clone(),
                run_id: manifest.run_id().clone(),
                step_id: StepId::new("check-docs").expect("step id"),
                obligations: vec![LocalCheckGovernanceObligationDefinition::new(
                    fixture.requirement.requirement_fingerprint().clone(),
                    LocalCheckGovernanceRequirementLevel::Required,
                )],
            },
        )
        .expect("relabeled candidate set");
        let relabel_error = adapt_docs_check_contribution(
            &relabeled_candidate,
            outcome.contribution(),
            LocalCheckGovernanceRequirementLevel::Required,
        )
        .expect_err("cross-bundle relabeling must fail");
        assert_eq!(
            relabel_error.code(),
            "local_check_attestation.structural_coverage.contribution_unexpected"
        );

        let debug = format!("{outcome:?}");
        assert!(debug.contains("Satisfied"));
        assert!(!debug.contains("workflow/test"));
        assert!(!debug.contains("run-test"));
        assert!(!debug.contains(outcome.contribution().obligation_fingerprint().as_str()));
    }

    #[test]
    fn unaccepted_gate_result_maps_to_failed_leaf_contribution() {
        for output in [
            LocalCheckProcessOutput::completed(
                Some(1),
                false,
                1_000,
                Vec::new(),
                b"check failed".to_vec(),
            ),
            LocalCheckProcessOutput::timed_out(1_000, Vec::new(), Vec::new()),
        ] {
            let samples = five_samples().into_iter().take(3).collect();
            let (clock, clock_calls) = ScriptedClock::new(samples);
            let fixture = fixture(Ok(output), clock_calls.clone());

            let outcome = execute_docs_check_governance_contribution(&input(&fixture, &clock))
                .expect("failed contribution returned");

            assert_eq!(
                outcome.contribution().posture(),
                GovernanceEvidenceCheckContributionPosture::Failed
            );
            assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 1);
            assert_eq!(clock_calls.load(Ordering::SeqCst), 3);
        }
    }

    #[test]
    fn stale_gate_maps_to_required_unavailable_leaf_contribution() {
        let (clock, clock_calls) = ScriptedClock::new(
            [
                "2026-07-19T12:00:00Z",
                "2026-07-19T12:00:01Z",
                "2026-07-19T12:00:02Z",
                "2026-07-19T12:00:02Z",
                "2026-07-19T12:00:04Z",
            ]
            .into_iter()
            .map(timestamp)
            .collect(),
        );
        let mut fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );
        fixture.requirement =
            LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
                command_id: fixture.handler.contract().command_id().clone(),
                minimum_assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
                accepted_statuses: vec![LocalCheckResultStatus::Passed],
                freshness: LocalCheckAttestationFreshnessPolicy::max_age_seconds(1)
                    .expect("freshness"),
                exact_immutable_run_binding_required: true,
                truncation_allowed: false,
            })
            .expect("requirement");

        let outcome = execute_docs_check_governance_contribution(&input(&fixture, &clock))
            .expect("stale contribution returned");

        assert_eq!(
            outcome.contribution().posture(),
            GovernanceEvidenceCheckContributionPosture::RequiredUnavailable
        );
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 1);
        assert_eq!(clock_calls.load(Ordering::SeqCst), 5);
    }

    #[test]
    fn contribution_identity_binds_step_and_requirement_deterministically() {
        let (clock, clock_calls) = ScriptedClock::new(Vec::new());
        let mut fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls,
        );
        let original_input = input(&fixture, &clock);
        let original = governance_obligation_fingerprint(&original_input);
        assert_eq!(original, governance_obligation_fingerprint(&original_input));

        let mut changed_step_input = input(&fixture, &clock);
        changed_step_input.step_id = StepId::new("other-step").expect("step id");
        assert_ne!(
            original,
            governance_obligation_fingerprint(&changed_step_input)
        );

        fixture.requirement =
            LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
                command_id: fixture.handler.contract().command_id().clone(),
                minimum_assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
                accepted_statuses: vec![LocalCheckResultStatus::Passed],
                freshness: LocalCheckAttestationFreshnessPolicy::NoReuse,
                exact_immutable_run_binding_required: true,
                truncation_allowed: true,
            })
            .expect("changed requirement");
        assert_ne!(
            original,
            governance_obligation_fingerprint(&input(&fixture, &clock))
        );
    }

    #[test]
    fn failed_and_timed_out_checks_return_honest_no_proof_outcomes() {
        for output in [
            LocalCheckProcessOutput::completed(
                Some(1),
                false,
                1_000,
                Vec::new(),
                b"check failed".to_vec(),
            ),
            LocalCheckProcessOutput::timed_out(1_000, Vec::new(), Vec::new()),
        ] {
            let samples = four_samples().into_iter().take(3).collect();
            let (clock, clock_calls) = ScriptedClock::new(samples);
            let fixture = fixture(Ok(output), clock_calls.clone());

            let outcome = execute_docs_check_with_attestation(&input(&fixture, &clock))
                .expect("check outcome returned");

            assert!(matches!(
                outcome.result().status(),
                LocalCheckResultStatus::Failed | LocalCheckResultStatus::TimedOut
            ));
            assert!(outcome.accepted_attestation().is_none());
            assert_eq!(clock_calls.load(Ordering::SeqCst), 3);
        }
    }

    #[test]
    fn invalid_identity_fails_before_clock_or_process_execution() {
        let (clock, clock_calls) = ScriptedClock::new(four_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );
        let mut execution_input = input(&fixture, &clock);
        execution_input.run_id = WorkflowRunId::new("run-other").expect("other run id");

        let error = execute_docs_check_with_attestation(&execution_input)
            .expect_err("identity mismatch rejected");

        assert_eq!(
            error.code(),
            "local_check_attestation.runtime.bundle_identity_mismatch"
        );
        assert_eq!(clock_calls.load(Ordering::SeqCst), 0);
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn unknown_step_fails_before_clock_or_process_execution() {
        let (clock, clock_calls) = ScriptedClock::new(four_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );
        let mut execution_input = input(&fixture, &clock);
        execution_input.step_id = StepId::new("other-step").expect("other step id");

        let error = execute_docs_check_with_attestation(&execution_input)
            .expect_err("unknown stored step rejected");

        assert_eq!(
            error.code(),
            "local_check_attestation.runtime.step_unresolved"
        );
        assert_eq!(clock_calls.load(Ordering::SeqCst), 0);
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn stored_step_derives_skill_identity_without_caller_authority() {
        let (_clock, clock_calls) = ScriptedClock::new(four_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls,
        );

        let (skill_id, skill_version) = resolve_stored_step_skill(
            &fixture.stored_bundle,
            &StepId::new("check-docs").expect("step id"),
        )
        .expect("stored step and skill resolve");

        assert_eq!(
            skill_id,
            SkillId::new("local/check-docs").expect("skill id")
        );
        assert_eq!(skill_version, SkillVersion::new("v0").expect("version"));
    }

    #[test]
    fn clock_and_runner_failures_return_no_partial_outcome() {
        let (backward_clock, backward_calls) = ScriptedClock::new(vec![
            timestamp("2026-07-19T12:00:01Z"),
            timestamp("2026-07-19T12:00:00Z"),
        ]);
        let backward_fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1,
                Vec::new(),
                Vec::new(),
            )),
            backward_calls.clone(),
        );
        let error = execute_docs_check_with_attestation(&input(&backward_fixture, &backward_clock))
            .expect_err("backward clock rejected");
        assert_eq!(
            error.code(),
            "local_check_attestation.runtime.clock_order_invalid"
        );
        assert_eq!(backward_fixture.runner_calls.load(Ordering::SeqCst), 0);

        let (runner_clock, runner_clock_calls) = ScriptedClock::new(four_samples());
        let runner_fixture = fixture(
            Err(WorkflowOsError::new(
                WorkflowOsErrorKind::Internal,
                "local_check.process.spawn_failed",
                "local check process could not be started",
            )),
            runner_clock_calls.clone(),
        );
        let error = execute_docs_check_with_attestation(&input(&runner_fixture, &runner_clock))
            .expect_err("runner error propagated");
        assert_eq!(error.code(), "local_check.process.spawn_failed");
        assert_eq!(runner_clock_calls.load(Ordering::SeqCst), 2);
        assert_eq!(runner_fixture.runner_calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn eligible_status_propagates_verifier_failure_and_debug_stays_bounded() {
        let (clock, clock_calls) = ScriptedClock::new(vec![
            timestamp("2026-07-19T12:00:00Z"),
            timestamp("2026-07-19T12:00:01Z"),
            timestamp("2026-07-19T12:00:02Z"),
            timestamp("2026-07-19T12:00:10Z"),
        ]);
        let mut fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                b"documentation check passed".to_vec(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );
        fixture.requirement =
            LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
                command_id: fixture.handler.contract().command_id().clone(),
                minimum_assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
                accepted_statuses: vec![LocalCheckResultStatus::Passed],
                freshness: LocalCheckAttestationFreshnessPolicy::max_age_seconds(1)
                    .expect("freshness"),
                exact_immutable_run_binding_required: true,
                truncation_allowed: false,
            })
            .expect("requirement");

        let execution_input = input(&fixture, &clock);
        let debug = format!("{execution_input:?}");
        assert!(!debug.contains("workflow/test"));
        assert!(!debug.contains("run-test"));
        assert!(!debug.contains("invocation/check-docs"));

        let error = execute_docs_check_with_attestation(&execution_input)
            .expect_err("stale eligible proof rejected");
        assert_eq!(
            error.code(),
            "local_check_attestation.verify.freshness_expired"
        );
        assert_eq!(clock_calls.load(Ordering::SeqCst), 4);
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn authoritative_same_call_composition_returns_satisfied_fact() {
        let (clock, clock_calls) = ScriptedClock::new(five_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                b"bounded output".to_vec(),
                Vec::new(),
            )),
            clock_calls,
        );
        let execution = input(&fixture, &clock);
        let executions = [execution];
        let step_id = StepId::new("check-docs").expect("step id");

        let outcome = compose_authoritative_docs_check_evidence_check_fact(
            &AuthoritativeDocsCheckCompositionInput {
                stored_immutable_run_bundle: &fixture.stored_bundle,
                step_id: &step_id,
                executions: &executions,
            },
        )
        .expect("authoritative composition");

        assert_eq!(outcome.results().len(), 1);
        assert_eq!(
            outcome.results()[0].status(),
            LocalCheckResultStatus::Passed
        );
        assert_eq!(
            outcome.fact().posture(),
            crate::GovernanceWorkloadEvidenceCheckPosture::Satisfied
        );
        assert_eq!(outcome.fact().expected_count(), 1);
        assert_eq!(outcome.fact().satisfied_count(), 1);
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 1);
        let debug = format!("{outcome:?}");
        assert!(!debug.contains("bounded output"));
        assert!(!debug.contains("workflow/test"));
        assert!(!debug.contains("run-test"));
    }

    #[test]
    fn authoritative_same_call_composition_accounts_for_omitted_levels() {
        for (level, expected) in [
            (
                "required",
                crate::GovernanceWorkloadEvidenceCheckPosture::RequiredUnavailable,
            ),
            (
                "optional",
                crate::GovernanceWorkloadEvidenceCheckPosture::OptionalUnavailable,
            ),
        ] {
            let (clock, clock_calls) = ScriptedClock::new(Vec::new());
            let fixture = fixture_with_level(
                Ok(LocalCheckProcessOutput::completed(
                    Some(0),
                    true,
                    1_000,
                    Vec::new(),
                    Vec::new(),
                )),
                clock_calls,
                level,
            );
            let executions = [];
            let step_id = StepId::new("check-docs").expect("step id");

            let outcome = compose_authoritative_docs_check_evidence_check_fact(
                &AuthoritativeDocsCheckCompositionInput {
                    stored_immutable_run_bundle: &fixture.stored_bundle,
                    step_id: &step_id,
                    executions: &executions,
                },
            )
            .expect("omission is represented");

            assert!(outcome.results().is_empty());
            assert_eq!(outcome.fact().posture(), expected);
            assert_eq!(outcome.fact().missing_count(), 1);
            assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 0);
            assert_eq!(clock.calls.load(Ordering::SeqCst), 0);
        }
    }

    #[test]
    fn canonical_empty_declarations_execute_nothing_and_are_satisfied() {
        let (_clock, clock_calls) = ScriptedClock::new(Vec::new());
        let fixture = fixture_without_requirements(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );
        let executions = [];
        let step_id = StepId::new("check-docs").expect("step id");

        let outcome = compose_authoritative_docs_check_evidence_check_fact(
            &AuthoritativeDocsCheckCompositionInput {
                stored_immutable_run_bundle: &fixture.stored_bundle,
                step_id: &step_id,
                executions: &executions,
            },
        )
        .expect("canonical empty declarations are authoritative");

        assert!(outcome.results().is_empty());
        assert_eq!(
            outcome.fact().posture(),
            crate::GovernanceWorkloadEvidenceCheckPosture::Satisfied
        );
        assert_eq!(outcome.fact().expected_count(), 0);
        assert_eq!(outcome.fact().satisfied_count(), 0);
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 0);
        assert_eq!(clock_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn full_batch_preflight_rejects_later_mismatch_before_execution() {
        let (clock, clock_calls) = ScriptedClock::new(five_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );
        let first = input(&fixture, &clock);
        let mut second = input(&fixture, &clock);
        second.run_id = WorkflowRunId::new("run-other").expect("run id");
        let executions = [first, second];
        let step_id = StepId::new("check-docs").expect("step id");

        let error = compose_authoritative_docs_check_evidence_check_fact(
            &AuthoritativeDocsCheckCompositionInput {
                stored_immutable_run_bundle: &fixture.stored_bundle,
                step_id: &step_id,
                executions: &executions,
            },
        )
        .expect_err("later mismatch rejects the full batch");

        assert_eq!(
            error.code(),
            "local_check_attestation.composition.execution_context_mismatch"
        );
        assert!(!error.to_string().contains("run-other"));
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 0);
        assert_eq!(clock_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn duplicate_authoritative_execution_fails_before_process_start() {
        let (clock, clock_calls) = ScriptedClock::new(five_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );
        let executions = [input(&fixture, &clock), input(&fixture, &clock)];
        let step_id = StepId::new("check-docs").expect("step id");

        let error = compose_authoritative_docs_check_evidence_check_fact(
            &AuthoritativeDocsCheckCompositionInput {
                stored_immutable_run_bundle: &fixture.stored_bundle,
                step_id: &step_id,
                executions: &executions,
            },
        )
        .expect_err("duplicate obligation rejected");

        assert_eq!(
            error.code(),
            "local_check_attestation.composition.execution_duplicate"
        );
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 0);
        assert_eq!(clock_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn executed_optional_failure_remains_failed() {
        let samples = five_samples().into_iter().take(3).collect();
        let (clock, clock_calls) = ScriptedClock::new(samples);
        let fixture = fixture_with_level(
            Ok(LocalCheckProcessOutput::completed(
                Some(1),
                false,
                1_000,
                Vec::new(),
                b"bounded failure".to_vec(),
            )),
            clock_calls,
            "optional",
        );
        let executions = [input(&fixture, &clock)];
        let step_id = StepId::new("check-docs").expect("step id");

        let outcome = compose_authoritative_docs_check_evidence_check_fact(
            &AuthoritativeDocsCheckCompositionInput {
                stored_immutable_run_bundle: &fixture.stored_bundle,
                step_id: &step_id,
                executions: &executions,
            },
        )
        .expect("executed optional failure is represented");

        assert_eq!(
            outcome.fact().posture(),
            crate::GovernanceWorkloadEvidenceCheckPosture::Failed
        );
        assert_eq!(outcome.fact().failed_count(), 1);
        assert_eq!(outcome.fact().optional_unavailable_count(), 0);
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 1);
        assert!(!format!("{outcome:?}").contains("bounded failure"));
    }

    #[test]
    fn unexpected_requirement_fails_preflight_without_execution() {
        let (clock, clock_calls) = ScriptedClock::new(five_samples());
        let mut fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );
        fixture.requirement =
            LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
                command_id: fixture.handler.contract().command_id().clone(),
                minimum_assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
                accepted_statuses: vec![LocalCheckResultStatus::Passed],
                freshness: LocalCheckAttestationFreshnessPolicy::NoReuse,
                exact_immutable_run_binding_required: true,
                truncation_allowed: true,
            })
            .expect("different requirement");
        let executions = [input(&fixture, &clock)];
        let step_id = StepId::new("check-docs").expect("step id");

        let error = compose_authoritative_docs_check_evidence_check_fact(
            &AuthoritativeDocsCheckCompositionInput {
                stored_immutable_run_bundle: &fixture.stored_bundle,
                step_id: &step_id,
                executions: &executions,
            },
        )
        .expect_err("unexpected requirement rejected");

        assert_eq!(
            error.code(),
            "local_check_attestation.composition.requirement_unexpected"
        );
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 0);
        assert_eq!(clock_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn command_contract_mismatch_fails_preflight_without_execution() {
        let (clock, clock_calls) = ScriptedClock::new(five_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );
        let mut contract_value =
            serde_json::to_value(fixture.handler.contract()).expect("contract serializes");
        contract_value["timeout_seconds"] = serde_json::json!(121);
        let mismatched_contract: LocalCheckCommandContract =
            serde_json::from_value(contract_value).expect("changed contract remains valid");
        let mismatched_handler = DocsCheckLocalHandler::new_with_process_runner(
            mismatched_contract,
            fixture.project_root.path().join("bin/npm"),
            fixture.project_root.path().to_path_buf(),
            None,
            Arc::new(RecordingRunner {
                output: Ok(LocalCheckProcessOutput::completed(
                    Some(0),
                    true,
                    1_000,
                    Vec::new(),
                    Vec::new(),
                )),
                calls: fixture.runner_calls.clone(),
                clock_calls: clock_calls.clone(),
            }),
        )
        .expect("mismatched handler");
        let mut execution = input(&fixture, &clock);
        execution.handler = &mismatched_handler;
        let executions = [execution];
        let step_id = StepId::new("check-docs").expect("step id");

        let error = compose_authoritative_docs_check_evidence_check_fact(
            &AuthoritativeDocsCheckCompositionInput {
                stored_immutable_run_bundle: &fixture.stored_bundle,
                step_id: &step_id,
                executions: &executions,
            },
        )
        .expect_err("command contract mismatch rejected");

        assert_eq!(
            error.code(),
            "local_check_attestation.composition.command_contract_mismatch"
        );
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 0);
        assert_eq!(clock_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn authoritative_reassessment_returns_one_fact_bound_assessment() {
        let (clock, clock_calls) = ScriptedClock::new(five_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                b"private-output-marker".to_vec(),
                Vec::new(),
            )),
            clock_calls,
        );
        let executions = [input(&fixture, &clock)];
        let step_id = StepId::new("check-docs").expect("step id");
        let runtime_facts = [reassessment_fact(None)];

        let outcome = compose_authoritative_local_check_reassessment(
            &AuthoritativeLocalCheckReassessmentInput {
                local_check: AuthoritativeDocsCheckCompositionInput {
                    stored_immutable_run_bundle: &fixture.stored_bundle,
                    step_id: &step_id,
                    executions: &executions,
                },
                profile: GovernanceStrictnessProfile::ObserveAndReport,
                runtime_facts: &runtime_facts,
            },
        )
        .expect("authoritative reassessment");

        assert_eq!(outcome.results().len(), 1);
        assert_eq!(
            outcome.bound_assessment().local_check_posture(),
            GovernanceWorkloadEvidenceCheckPosture::Satisfied
        );
        assert_eq!(outcome.bound_assessment().assessment_count(), 1);
        assert_eq!(
            outcome.bound_assessment.assessment_set.assessments()[0]
                .assessment()
                .decision()
                .execution(),
            GovernanceExecutionDisposition::Proceed
        );
        assert_eq!(
            outcome.bound_assessment().algorithm().identifier(),
            "workflow-os/authoritative-local-check-reassessment-binding/v1"
        );
        let debug = format!("{outcome:?}");
        assert!(!debug.contains("private-output-marker"));
        assert!(!debug.contains("workflow/test"));
        assert!(!debug.contains("run-test"));
        assert!(!debug.contains(outcome.bound_assessment().binding_fingerprint().as_str()));
    }

    #[test]
    fn runtime_fact_source_bridge_returns_one_call_source_backed_binding() {
        let (clock, clock_calls) = ScriptedClock::new(five_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                b"private-output-marker".to_vec(),
                Vec::new(),
            )),
            clock_calls,
        );
        let executions = [input(&fixture, &clock)];
        let step_id = StepId::new("check-docs").expect("step id");
        let runtime_facts = [reassessment_fact(None)];

        let outcome = compose_authoritative_local_check_runtime_fact_source_bridge(
            &AuthoritativeLocalCheckRuntimeFactSourceBridgeInput {
                reassessment: AuthoritativeLocalCheckReassessmentInput {
                    local_check: AuthoritativeDocsCheckCompositionInput {
                        stored_immutable_run_bundle: &fixture.stored_bundle,
                        step_id: &step_id,
                        executions: &executions,
                    },
                    profile: GovernanceStrictnessProfile::ObserveAndReport,
                    runtime_facts: &runtime_facts,
                },
                evaluated_at: timestamp("2026-07-19T12:00:04Z").expect("evaluation time"),
            },
        )
        .expect("Core-owned source bridge");

        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 1);
        assert_eq!(outcome.results().len(), 1);
        assert_eq!(outcome.runtime_fact_snapshot().runtime_fact_count(), 1);
        assert_eq!(
            outcome
                .runtime_fact_snapshot()
                .effective_maximum_observation_age_seconds(),
            1
        );
        assert!(outcome.governance_binding().source_binding().is_none());
        assert!(outcome
            .governance_binding()
            .runtime_fact_snapshot_binding()
            .is_some());

        let debug = format!("{outcome:?}");
        assert!(!debug.contains("private-output-marker"));
        assert!(!debug.contains("workflow/test"));
        assert!(!debug.contains("run-test"));
        assert!(!debug.contains(
            outcome
                .runtime_fact_snapshot()
                .snapshot_commitment()
                .as_str()
        ));
    }

    #[test]
    fn runtime_fact_source_bridge_matches_legacy_reassessment_matrix() {
        fn output(failed: bool) -> LocalCheckProcessOutput {
            if failed {
                LocalCheckProcessOutput::completed(
                    Some(1),
                    false,
                    1_000,
                    Vec::new(),
                    b"private-failure-marker".to_vec(),
                )
            } else {
                LocalCheckProcessOutput::completed(
                    Some(0),
                    true,
                    1_000,
                    b"private-success-marker".to_vec(),
                    Vec::new(),
                )
            }
        }

        fn compare(
            requirement_level: &str,
            execute_check: bool,
            failed: bool,
            authority: GovernanceWorkloadAuthorityPosture,
        ) -> SpecContentHash {
            let legacy_samples = if execute_check {
                five_samples()
            } else {
                Vec::new()
            };
            let (legacy_clock, legacy_clock_calls) = ScriptedClock::new(legacy_samples);
            let legacy_fixture =
                fixture_with_level(Ok(output(failed)), legacy_clock_calls, requirement_level);
            let legacy_executions = if execute_check {
                vec![input(&legacy_fixture, &legacy_clock)]
            } else {
                Vec::new()
            };
            let step_id = StepId::new("check-docs").expect("step id");
            let runtime_facts = [StepGovernanceRuntimeFacts::new(
                step_id.clone(),
                Some(authority),
                None,
                Some(GovernanceWorkloadSideEffectPosture::None),
                None,
                None,
                None,
            )];
            let legacy = compose_authoritative_local_check_reassessment(
                &AuthoritativeLocalCheckReassessmentInput {
                    local_check: AuthoritativeDocsCheckCompositionInput {
                        stored_immutable_run_bundle: &legacy_fixture.stored_bundle,
                        step_id: &step_id,
                        executions: &legacy_executions,
                    },
                    profile: GovernanceStrictnessProfile::ObserveAndReport,
                    runtime_facts: &runtime_facts,
                },
            )
            .expect("legacy reassessment");
            let legacy_result_count = legacy.results().len();
            let (_, legacy_binding) = legacy
                .into_parts(&legacy_fixture.stored_bundle, step_id.clone())
                .expect("legacy binding");

            let bridge_samples = if execute_check {
                five_samples()
            } else {
                Vec::new()
            };
            let (bridge_clock, bridge_clock_calls) = ScriptedClock::new(bridge_samples);
            let bridge_fixture =
                fixture_with_level(Ok(output(failed)), bridge_clock_calls, requirement_level);
            let bridge_executions = if execute_check {
                vec![input(&bridge_fixture, &bridge_clock)]
            } else {
                Vec::new()
            };
            let bridge = compose_authoritative_local_check_runtime_fact_source_bridge(
                &AuthoritativeLocalCheckRuntimeFactSourceBridgeInput {
                    reassessment: AuthoritativeLocalCheckReassessmentInput {
                        local_check: AuthoritativeDocsCheckCompositionInput {
                            stored_immutable_run_bundle: &bridge_fixture.stored_bundle,
                            step_id: &step_id,
                            executions: &bridge_executions,
                        },
                        profile: GovernanceStrictnessProfile::ObserveAndReport,
                        runtime_facts: &runtime_facts,
                    },
                    evaluated_at: timestamp("2026-07-19T12:00:04Z").expect("evaluation time"),
                },
            )
            .expect("source-backed reassessment");

            assert_eq!(legacy_result_count, bridge.results().len());
            assert_eq!(
                legacy_binding.aggregate_fingerprint(),
                bridge.governance_binding().aggregate_fingerprint()
            );
            assert_eq!(
                legacy_binding.execution(),
                bridge.governance_binding().execution()
            );
            assert_eq!(
                legacy_binding.disclosure(),
                bridge.governance_binding().disclosure()
            );
            assert_eq!(
                legacy_binding.completeness(),
                bridge.governance_binding().completeness()
            );
            assert_eq!(
                legacy_fixture.runner_calls.load(Ordering::SeqCst),
                usize::from(execute_check)
            );
            assert_eq!(
                bridge_fixture.runner_calls.load(Ordering::SeqCst),
                usize::from(execute_check)
            );
            bridge
                .governance_binding()
                .runtime_fact_snapshot_binding()
                .expect("source snapshot binding")
                .source_registration_commitment()
                .clone()
        }

        let registration_commitments = [
            compare(
                "required",
                true,
                false,
                GovernanceWorkloadAuthorityPosture::Sufficient,
            ),
            compare(
                "optional",
                false,
                false,
                GovernanceWorkloadAuthorityPosture::Sufficient,
            ),
            compare(
                "required",
                false,
                false,
                GovernanceWorkloadAuthorityPosture::Sufficient,
            ),
            compare(
                "required",
                true,
                false,
                GovernanceWorkloadAuthorityPosture::Unavailable,
            ),
            compare(
                "optional",
                true,
                true,
                GovernanceWorkloadAuthorityPosture::Sufficient,
            ),
        ];
        assert!(registration_commitments
            .iter()
            .all(|commitment| commitment == &registration_commitments[0]));
    }

    #[test]
    fn runtime_fact_source_bridge_rejects_caller_check_posture_before_execution() {
        let (clock, clock_calls) = ScriptedClock::new(five_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );
        let executions = [input(&fixture, &clock)];
        let step_id = StepId::new("check-docs").expect("step id");
        let runtime_facts = [reassessment_fact(Some(
            GovernanceWorkloadEvidenceCheckPosture::Satisfied,
        ))];

        let error = compose_authoritative_local_check_runtime_fact_source_bridge(
            &AuthoritativeLocalCheckRuntimeFactSourceBridgeInput {
                reassessment: AuthoritativeLocalCheckReassessmentInput {
                    local_check: AuthoritativeDocsCheckCompositionInput {
                        stored_immutable_run_bundle: &fixture.stored_bundle,
                        step_id: &step_id,
                        executions: &executions,
                    },
                    profile: GovernanceStrictnessProfile::ObserveAndReport,
                    runtime_facts: &runtime_facts,
                },
                evaluated_at: timestamp("2026-07-19T12:00:04Z").expect("evaluation time"),
            },
        )
        .expect_err("caller posture rejected");

        assert_eq!(
            error.code(),
            "local_check_attestation.reassessment_binding.selected_evidence_check_posture_supplied"
        );
        assert!(!error.to_string().contains("check-docs"));
        assert_eq!(clock_calls.load(Ordering::SeqCst), 0);
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn selected_caller_check_posture_fails_before_clock_or_process() {
        let (clock, clock_calls) = ScriptedClock::new(five_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );
        let executions = [input(&fixture, &clock)];
        let step_id = StepId::new("check-docs").expect("step id");
        let runtime_facts = [reassessment_fact(Some(
            GovernanceWorkloadEvidenceCheckPosture::Satisfied,
        ))];

        let error = compose_authoritative_local_check_reassessment(
            &AuthoritativeLocalCheckReassessmentInput {
                local_check: AuthoritativeDocsCheckCompositionInput {
                    stored_immutable_run_bundle: &fixture.stored_bundle,
                    step_id: &step_id,
                    executions: &executions,
                },
                profile: GovernanceStrictnessProfile::ObserveAndReport,
                runtime_facts: &runtime_facts,
            },
        )
        .expect_err("caller posture rejected");

        assert_eq!(
            error.code(),
            "local_check_attestation.reassessment_binding.selected_evidence_check_posture_supplied"
        );
        assert!(!error.to_string().contains("check-docs"));
        assert_eq!(clock_calls.load(Ordering::SeqCst), 0);
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn runtime_fact_shape_mismatch_fails_before_clock_or_process() {
        let (clock, clock_calls) = ScriptedClock::new(five_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls.clone(),
        );
        let executions = [input(&fixture, &clock)];
        let step_id = StepId::new("check-docs").expect("step id");
        let runtime_facts = [];

        let error = compose_authoritative_local_check_reassessment(
            &AuthoritativeLocalCheckReassessmentInput {
                local_check: AuthoritativeDocsCheckCompositionInput {
                    stored_immutable_run_bundle: &fixture.stored_bundle,
                    step_id: &step_id,
                    executions: &executions,
                },
                profile: GovernanceStrictnessProfile::ObserveAndReport,
                runtime_facts: &runtime_facts,
            },
        )
        .expect_err("runtime fact shape rejected");

        assert_eq!(
            error.code(),
            "governance.proportional.immutable_bundle.runtime_facts_count_mismatch"
        );
        assert_eq!(clock_calls.load(Ordering::SeqCst), 0);
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn satisfied_checks_cannot_weaken_unavailable_authority() {
        let (clock, clock_calls) = ScriptedClock::new(five_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls,
        );
        let executions = [input(&fixture, &clock)];
        let step_id = StepId::new("check-docs").expect("step id");
        let runtime_facts = [StepGovernanceRuntimeFacts::new(
            step_id.clone(),
            Some(GovernanceWorkloadAuthorityPosture::Unavailable),
            None,
            Some(GovernanceWorkloadSideEffectPosture::None),
            None,
            None,
            None,
        )];

        let outcome = compose_authoritative_local_check_reassessment(
            &AuthoritativeLocalCheckReassessmentInput {
                local_check: AuthoritativeDocsCheckCompositionInput {
                    stored_immutable_run_bundle: &fixture.stored_bundle,
                    step_id: &step_id,
                    executions: &executions,
                },
                profile: GovernanceStrictnessProfile::ObserveAndReport,
                runtime_facts: &runtime_facts,
            },
        )
        .expect("authoritative reassessment");

        assert_eq!(
            outcome.bound_assessment.assessment_set.assessments()[0]
                .assessment()
                .decision()
                .execution(),
            GovernanceExecutionDisposition::Denied
        );
    }

    #[test]
    fn same_posture_with_different_authoritative_fact_changes_binding() {
        fn bound_fingerprint(level: &str) -> SpecContentHash {
            let (clock, clock_calls) = ScriptedClock::new(five_samples());
            let fixture = fixture_with_level(
                Ok(LocalCheckProcessOutput::completed(
                    Some(0),
                    true,
                    1_000,
                    Vec::new(),
                    Vec::new(),
                )),
                clock_calls,
                level,
            );
            let executions = [input(&fixture, &clock)];
            let step_id = StepId::new("check-docs").expect("step id");
            let runtime_facts = [reassessment_fact(None)];
            compose_authoritative_local_check_reassessment(
                &AuthoritativeLocalCheckReassessmentInput {
                    local_check: AuthoritativeDocsCheckCompositionInput {
                        stored_immutable_run_bundle: &fixture.stored_bundle,
                        step_id: &step_id,
                        executions: &executions,
                    },
                    profile: GovernanceStrictnessProfile::ObserveAndReport,
                    runtime_facts: &runtime_facts,
                },
            )
            .expect("authoritative reassessment")
            .bound_assessment()
            .binding_fingerprint()
            .clone()
        }

        assert_ne!(bound_fingerprint("required"), bound_fingerprint("optional"));
    }

    #[test]
    fn reassessment_failure_returns_no_bound_value_after_check() {
        let (clock, clock_calls) = ScriptedClock::new(five_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls,
        );
        let executions = [input(&fixture, &clock)];
        let step_id = StepId::new("check-docs").expect("step id");
        let runtime_facts = [reassessment_fact(None)];

        let error = compose_authoritative_local_check_reassessment(
            &AuthoritativeLocalCheckReassessmentInput {
                local_check: AuthoritativeDocsCheckCompositionInput {
                    stored_immutable_run_bundle: &fixture.stored_bundle,
                    step_id: &step_id,
                    executions: &executions,
                },
                profile: GovernanceStrictnessProfile::StrictEnterprise,
                runtime_facts: &runtime_facts,
            },
        )
        .expect_err("missing steward minimum rejected");

        assert_eq!(
            error.code(),
            "governance.proportional.steward_minimum.required"
        );
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn reassessment_binding_framing_separates_ambiguous_fields() {
        let mut first = Sha256::new();
        hash_reassessment_binding_field(&mut first, "a", "bc");
        hash_reassessment_binding_field(&mut first, "d", "e");
        let mut second = Sha256::new();
        hash_reassessment_binding_field(&mut second, "ab", "c");
        hash_reassessment_binding_field(&mut second, "d", "e");

        assert_ne!(first.finalize(), second.finalize());
    }

    #[test]
    fn reassessment_binding_v1_known_vector_is_stable() {
        let (clock, clock_calls) = ScriptedClock::new(five_samples());
        let fixture = fixture(
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                1_000,
                Vec::new(),
                Vec::new(),
            )),
            clock_calls,
        );
        let executions = [input(&fixture, &clock)];
        let step_id = StepId::new("check-docs").expect("step id");
        let runtime_facts = [reassessment_fact(None)];

        let outcome = compose_authoritative_local_check_reassessment(
            &AuthoritativeLocalCheckReassessmentInput {
                local_check: AuthoritativeDocsCheckCompositionInput {
                    stored_immutable_run_bundle: &fixture.stored_bundle,
                    step_id: &step_id,
                    executions: &executions,
                },
                profile: GovernanceStrictnessProfile::ObserveAndReport,
                runtime_facts: &runtime_facts,
            },
        )
        .expect("authoritative reassessment");

        assert_eq!(
            outcome.bound_assessment().binding_fingerprint().as_str(),
            "fb984d445806a460e1b2d35cc97cbdd4dc322c680e3bb12dccebc24f45bc041d"
        );
    }

    #[test]
    fn aggregate_posture_preserves_visible_and_denied_semantics() {
        for (level, expected_execution, expected_disclosure) in [
            (
                "optional",
                GovernanceExecutionDisposition::Proceed,
                GovernanceDisclosureRequirement::Visible,
            ),
            (
                "required",
                GovernanceExecutionDisposition::Denied,
                GovernanceDisclosureRequirement::Visible,
            ),
        ] {
            let (_clock, clock_calls) = ScriptedClock::new(Vec::new());
            let fixture = fixture_with_level(
                Ok(LocalCheckProcessOutput::completed(
                    Some(0),
                    true,
                    1_000,
                    Vec::new(),
                    Vec::new(),
                )),
                clock_calls,
                level,
            );
            let executions = [];
            let step_id = StepId::new("check-docs").expect("step id");
            let runtime_facts = [reassessment_fact(None)];
            let outcome = compose_authoritative_local_check_reassessment(
                &AuthoritativeLocalCheckReassessmentInput {
                    local_check: AuthoritativeDocsCheckCompositionInput {
                        stored_immutable_run_bundle: &fixture.stored_bundle,
                        step_id: &step_id,
                        executions: &executions,
                    },
                    profile: GovernanceStrictnessProfile::ObserveAndReport,
                    runtime_facts: &runtime_facts,
                },
            )
            .expect("authoritative reassessment");
            let decision = outcome.bound_assessment.assessment_set.assessments()[0]
                .assessment()
                .decision();

            assert_eq!(decision.execution(), expected_execution);
            assert_eq!(decision.disclosure(), expected_disclosure);
        }
    }

    #[test]
    fn non_check_governance_axis_changes_binding() {
        fn bound_fingerprint(authority: GovernanceWorkloadAuthorityPosture) -> SpecContentHash {
            let (clock, clock_calls) = ScriptedClock::new(five_samples());
            let fixture = fixture(
                Ok(LocalCheckProcessOutput::completed(
                    Some(0),
                    true,
                    1_000,
                    Vec::new(),
                    Vec::new(),
                )),
                clock_calls,
            );
            let executions = [input(&fixture, &clock)];
            let step_id = StepId::new("check-docs").expect("step id");
            let runtime_facts = [StepGovernanceRuntimeFacts::new(
                step_id.clone(),
                Some(authority),
                None,
                Some(GovernanceWorkloadSideEffectPosture::None),
                None,
                None,
                None,
            )];
            compose_authoritative_local_check_reassessment(
                &AuthoritativeLocalCheckReassessmentInput {
                    local_check: AuthoritativeDocsCheckCompositionInput {
                        stored_immutable_run_bundle: &fixture.stored_bundle,
                        step_id: &step_id,
                        executions: &executions,
                    },
                    profile: GovernanceStrictnessProfile::ObserveAndReport,
                    runtime_facts: &runtime_facts,
                },
            )
            .expect("authoritative reassessment")
            .bound_assessment()
            .binding_fingerprint()
            .clone()
        }

        assert_ne!(
            bound_fingerprint(GovernanceWorkloadAuthorityPosture::Sufficient),
            bound_fingerprint(GovernanceWorkloadAuthorityPosture::Unavailable)
        );
    }

    #[test]
    fn execution_error_returns_no_composition_outcome() {
        let (clock, clock_calls) = ScriptedClock::new(five_samples());
        let fixture = fixture(
            Err(WorkflowOsError::new(
                WorkflowOsErrorKind::Internal,
                "local_check.process.spawn_failed",
                "local check process could not be started",
            )),
            clock_calls,
        );
        let executions = [input(&fixture, &clock)];
        let step_id = StepId::new("check-docs").expect("step id");

        let error = compose_authoritative_docs_check_evidence_check_fact(
            &AuthoritativeDocsCheckCompositionInput {
                stored_immutable_run_bundle: &fixture.stored_bundle,
                step_id: &step_id,
                executions: &executions,
            },
        )
        .expect_err("execution error returns no fact");

        assert_eq!(error.code(), "local_check.process.spawn_failed");
        assert_eq!(fixture.runner_calls.load(Ordering::SeqCst), 1);
    }
}
