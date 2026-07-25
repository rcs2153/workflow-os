use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use sha2::{Digest, Sha256};

use super::runtime::{
    docs_check_governance_obligation_fingerprint, DocsCheckGovernanceEvidenceCheckContribution,
    GovernanceEvidenceCheckContributionPosture,
};
use crate::{
    CanonicalLocalCheckDeclarationSetRecord, GovernanceWorkloadEvidenceCheckPosture,
    ImmutableRunBundleDefinitionKind, ImmutableRunBundleId, ImmutableRunBundleVersion,
    LocalCheckRequirementLevel, SpecContentHash, StepId, StoredImmutableRunBundle, WorkflowId,
    WorkflowOsError, WorkflowRunId, WorkflowVersion,
};

const CANDIDATE_ALGORITHM: &str =
    "workflow-os/local-check-governance-structural-coverage-candidate/v1";
const AGGREGATE_POSTURE_ALGORITHM: &str =
    "workflow-os/authoritative-local-check-aggregate-posture/v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum LocalCheckGovernanceRequirementLevel {
    Required,
    Optional,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum LocalCheckGovernanceContributionPosture {
    Satisfied,
    OptionalUnavailable,
    RequiredUnavailable,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LocalCheckGovernanceStructuralCoverageDisposition {
    Satisfied,
    OptionalUnavailable,
    RequiredUnavailable,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LocalCheckGovernanceDeclarationSourcePosture {
    Unresolved,
    CanonicalStoredBundle,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct LocalCheckGovernanceObligationDefinition {
    requirement_fingerprint: SpecContentHash,
    requirement_level: LocalCheckGovernanceRequirementLevel,
}

impl LocalCheckGovernanceObligationDefinition {
    pub(crate) const fn new(
        requirement_fingerprint: SpecContentHash,
        requirement_level: LocalCheckGovernanceRequirementLevel,
    ) -> Self {
        Self {
            requirement_fingerprint,
            requirement_level,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct LocalCheckGovernanceObligation {
    obligation_fingerprint: SpecContentHash,
    requirement_fingerprint: SpecContentHash,
    requirement_level: LocalCheckGovernanceRequirementLevel,
}

impl LocalCheckGovernanceObligation {
    pub(crate) const fn obligation_fingerprint(&self) -> &SpecContentHash {
        &self.obligation_fingerprint
    }

    pub(crate) const fn requirement_level(&self) -> LocalCheckGovernanceRequirementLevel {
        self.requirement_level
    }
}

impl fmt::Debug for LocalCheckGovernanceObligation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCheckGovernanceObligation")
            .field("obligation_fingerprint", &"[REDACTED]")
            .field("requirement_fingerprint", &"[REDACTED]")
            .field("requirement_level", &self.requirement_level)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct LocalCheckGovernanceContribution {
    candidate_set_fingerprint: SpecContentHash,
    obligation_fingerprint: SpecContentHash,
    posture: LocalCheckGovernanceContributionPosture,
}

impl LocalCheckGovernanceContribution {
    fn new(
        candidate_set_fingerprint: SpecContentHash,
        obligation_fingerprint: SpecContentHash,
        posture: LocalCheckGovernanceContributionPosture,
    ) -> Self {
        Self {
            candidate_set_fingerprint,
            obligation_fingerprint,
            posture,
        }
    }

    pub(crate) const fn obligation_fingerprint(&self) -> &SpecContentHash {
        &self.obligation_fingerprint
    }

    pub(crate) const fn posture(&self) -> LocalCheckGovernanceContributionPosture {
        self.posture
    }
}

impl fmt::Debug for LocalCheckGovernanceContribution {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCheckGovernanceContribution")
            .field("candidate_set_fingerprint", &"[REDACTED]")
            .field("obligation_fingerprint", &"[REDACTED]")
            .field("posture", &self.posture)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct LocalCheckGovernanceObligationSetCandidate {
    source_posture: LocalCheckGovernanceDeclarationSourcePosture,
    bundle_id: ImmutableRunBundleId,
    bundle_version: ImmutableRunBundleVersion,
    bundle_root: SpecContentHash,
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    run_id: WorkflowRunId,
    step_id: StepId,
    obligations: Vec<LocalCheckGovernanceObligation>,
    candidate_set_fingerprint: SpecContentHash,
}

pub(crate) struct LocalCheckGovernanceObligationSetCandidateDefinition {
    pub bundle_id: ImmutableRunBundleId,
    pub bundle_version: ImmutableRunBundleVersion,
    pub bundle_root: SpecContentHash,
    pub workflow_id: WorkflowId,
    pub workflow_version: WorkflowVersion,
    pub run_id: WorkflowRunId,
    pub step_id: StepId,
    pub obligations: Vec<LocalCheckGovernanceObligationDefinition>,
}

impl LocalCheckGovernanceObligationSetCandidate {
    pub(crate) fn new(
        definition: LocalCheckGovernanceObligationSetCandidateDefinition,
    ) -> Result<Self, WorkflowOsError> {
        Self::new_with_source(
            definition,
            LocalCheckGovernanceDeclarationSourcePosture::Unresolved,
        )
    }

    fn new_with_source(
        definition: LocalCheckGovernanceObligationSetCandidateDefinition,
        source_posture: LocalCheckGovernanceDeclarationSourcePosture,
    ) -> Result<Self, WorkflowOsError> {
        let mut obligations = definition
            .obligations
            .iter()
            .map(|obligation| LocalCheckGovernanceObligation {
                obligation_fingerprint: docs_check_governance_obligation_fingerprint(
                    &definition.bundle_id,
                    &definition.bundle_version,
                    &definition.bundle_root,
                    &definition.step_id,
                    &obligation.requirement_fingerprint,
                ),
                requirement_fingerprint: obligation.requirement_fingerprint.clone(),
                requirement_level: obligation.requirement_level,
            })
            .collect::<Vec<_>>();
        obligations.sort_by(|left, right| {
            left.obligation_fingerprint
                .cmp(&right.obligation_fingerprint)
        });
        if obligations
            .windows(2)
            .any(|pair| pair[0].obligation_fingerprint == pair[1].obligation_fingerprint)
        {
            return Err(coverage_error(
                "obligation_duplicate",
                "local check governance candidate contains a duplicate obligation",
            ));
        }

        let candidate_set_fingerprint =
            candidate_set_fingerprint(&definition, source_posture, &obligations);
        Ok(Self {
            source_posture,
            bundle_id: definition.bundle_id,
            bundle_version: definition.bundle_version,
            bundle_root: definition.bundle_root,
            workflow_id: definition.workflow_id,
            workflow_version: definition.workflow_version,
            run_id: definition.run_id,
            step_id: definition.step_id,
            obligations,
            candidate_set_fingerprint,
        })
    }

    pub(crate) fn obligations(&self) -> &[LocalCheckGovernanceObligation] {
        &self.obligations
    }

    pub(crate) const fn source_posture(&self) -> LocalCheckGovernanceDeclarationSourcePosture {
        self.source_posture
    }

    pub(crate) const fn candidate_set_fingerprint(&self) -> &SpecContentHash {
        &self.candidate_set_fingerprint
    }
}

impl fmt::Debug for LocalCheckGovernanceObligationSetCandidate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCheckGovernanceObligationSetCandidate")
            .field("source_posture", &self.source_posture)
            .field("obligation_count", &self.obligations.len())
            .field("binding", &"[REDACTED]")
            .field("candidate_set_fingerprint", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct LocalCheckGovernanceStructuralCoverageCandidate {
    source_posture: LocalCheckGovernanceDeclarationSourcePosture,
    disposition: LocalCheckGovernanceStructuralCoverageDisposition,
    expected_count: usize,
    satisfied_count: usize,
    failed_count: usize,
    required_unavailable_count: usize,
    optional_unavailable_count: usize,
    missing_count: usize,
    candidate_set_fingerprint: SpecContentHash,
    structural_coverage_fingerprint: SpecContentHash,
}

impl LocalCheckGovernanceStructuralCoverageCandidate {
    pub(crate) const fn source_posture(&self) -> LocalCheckGovernanceDeclarationSourcePosture {
        self.source_posture
    }

    pub(crate) const fn disposition(&self) -> LocalCheckGovernanceStructuralCoverageDisposition {
        self.disposition
    }

    pub(crate) const fn expected_count(&self) -> usize {
        self.expected_count
    }

    pub(crate) const fn satisfied_count(&self) -> usize {
        self.satisfied_count
    }

    pub(crate) const fn failed_count(&self) -> usize {
        self.failed_count
    }

    pub(crate) const fn required_unavailable_count(&self) -> usize {
        self.required_unavailable_count
    }

    pub(crate) const fn optional_unavailable_count(&self) -> usize {
        self.optional_unavailable_count
    }

    pub(crate) const fn missing_count(&self) -> usize {
        self.missing_count
    }

    pub(crate) const fn candidate_set_fingerprint(&self) -> &SpecContentHash {
        &self.candidate_set_fingerprint
    }

    pub(crate) const fn structural_coverage_fingerprint(&self) -> &SpecContentHash {
        &self.structural_coverage_fingerprint
    }
}

impl fmt::Debug for LocalCheckGovernanceStructuralCoverageCandidate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCheckGovernanceStructuralCoverageCandidate")
            .field("source_posture", &self.source_posture)
            .field("disposition", &self.disposition)
            .field("expected_count", &self.expected_count)
            .field("satisfied_count", &self.satisfied_count)
            .field("failed_count", &self.failed_count)
            .field(
                "required_unavailable_count",
                &self.required_unavailable_count,
            )
            .field(
                "optional_unavailable_count",
                &self.optional_unavailable_count,
            )
            .field("missing_count", &self.missing_count)
            .field("candidate_set_fingerprint", &"[REDACTED]")
            .field("structural_coverage_fingerprint", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AuthoritativeLocalCheckEvidenceCheckFactAlgorithm {
    V1,
}

impl AuthoritativeLocalCheckEvidenceCheckFactAlgorithm {
    pub(crate) const fn identifier(self) -> &'static str {
        match self {
            Self::V1 => AGGREGATE_POSTURE_ALGORITHM,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct AuthoritativeLocalCheckEvidenceCheckFact {
    algorithm: AuthoritativeLocalCheckEvidenceCheckFactAlgorithm,
    posture: GovernanceWorkloadEvidenceCheckPosture,
    expected_count: usize,
    satisfied_count: usize,
    failed_count: usize,
    required_unavailable_count: usize,
    optional_unavailable_count: usize,
    missing_count: usize,
    candidate_set_fingerprint: SpecContentHash,
    structural_coverage_fingerprint: SpecContentHash,
    fact_fingerprint: SpecContentHash,
}

impl AuthoritativeLocalCheckEvidenceCheckFact {
    pub(crate) const fn algorithm(&self) -> AuthoritativeLocalCheckEvidenceCheckFactAlgorithm {
        self.algorithm
    }

    pub(crate) const fn posture(&self) -> GovernanceWorkloadEvidenceCheckPosture {
        self.posture
    }

    pub(crate) const fn expected_count(&self) -> usize {
        self.expected_count
    }

    pub(crate) const fn satisfied_count(&self) -> usize {
        self.satisfied_count
    }

    pub(crate) const fn failed_count(&self) -> usize {
        self.failed_count
    }

    pub(crate) const fn required_unavailable_count(&self) -> usize {
        self.required_unavailable_count
    }

    pub(crate) const fn optional_unavailable_count(&self) -> usize {
        self.optional_unavailable_count
    }

    pub(crate) const fn missing_count(&self) -> usize {
        self.missing_count
    }

    pub(crate) const fn candidate_set_fingerprint(&self) -> &SpecContentHash {
        &self.candidate_set_fingerprint
    }

    pub(crate) const fn structural_coverage_fingerprint(&self) -> &SpecContentHash {
        &self.structural_coverage_fingerprint
    }

    pub(crate) const fn fact_fingerprint(&self) -> &SpecContentHash {
        &self.fact_fingerprint
    }
}

impl fmt::Debug for AuthoritativeLocalCheckEvidenceCheckFact {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthoritativeLocalCheckEvidenceCheckFact")
            .field("algorithm", &self.algorithm)
            .field("posture", &self.posture)
            .field("expected_count", &self.expected_count)
            .field("satisfied_count", &self.satisfied_count)
            .field("failed_count", &self.failed_count)
            .field(
                "required_unavailable_count",
                &self.required_unavailable_count,
            )
            .field(
                "optional_unavailable_count",
                &self.optional_unavailable_count,
            )
            .field("missing_count", &self.missing_count)
            .field("candidate_set_fingerprint", &"[REDACTED]")
            .field("structural_coverage_fingerprint", &"[REDACTED]")
            .field("fact_fingerprint", &"[REDACTED]")
            .finish()
    }
}

pub(crate) fn convert_authoritative_local_check_coverage(
    coverage: &LocalCheckGovernanceStructuralCoverageCandidate,
) -> Result<AuthoritativeLocalCheckEvidenceCheckFact, WorkflowOsError> {
    if coverage.source_posture
        != LocalCheckGovernanceDeclarationSourcePosture::CanonicalStoredBundle
    {
        return Err(aggregate_posture_error(
            "source_not_authoritative",
            "local check aggregate posture requires canonical stored coverage",
        ));
    }
    validate_authoritative_coverage(coverage)?;
    let posture = match coverage.disposition {
        LocalCheckGovernanceStructuralCoverageDisposition::Satisfied => {
            GovernanceWorkloadEvidenceCheckPosture::Satisfied
        }
        LocalCheckGovernanceStructuralCoverageDisposition::OptionalUnavailable => {
            GovernanceWorkloadEvidenceCheckPosture::OptionalUnavailable
        }
        LocalCheckGovernanceStructuralCoverageDisposition::RequiredUnavailable => {
            GovernanceWorkloadEvidenceCheckPosture::RequiredUnavailable
        }
        LocalCheckGovernanceStructuralCoverageDisposition::Failed => {
            GovernanceWorkloadEvidenceCheckPosture::Failed
        }
    };
    let algorithm = AuthoritativeLocalCheckEvidenceCheckFactAlgorithm::V1;
    let fact_fingerprint = aggregate_fact_fingerprint(algorithm, posture, coverage);
    Ok(AuthoritativeLocalCheckEvidenceCheckFact {
        algorithm,
        posture,
        expected_count: coverage.expected_count,
        satisfied_count: coverage.satisfied_count,
        failed_count: coverage.failed_count,
        required_unavailable_count: coverage.required_unavailable_count,
        optional_unavailable_count: coverage.optional_unavailable_count,
        missing_count: coverage.missing_count,
        candidate_set_fingerprint: coverage.candidate_set_fingerprint.clone(),
        structural_coverage_fingerprint: coverage.structural_coverage_fingerprint.clone(),
        fact_fingerprint,
    })
}

fn validate_authoritative_coverage(
    coverage: &LocalCheckGovernanceStructuralCoverageCandidate,
) -> Result<(), WorkflowOsError> {
    let terminal_count = coverage
        .satisfied_count
        .checked_add(coverage.failed_count)
        .and_then(|count| count.checked_add(coverage.required_unavailable_count))
        .and_then(|count| count.checked_add(coverage.optional_unavailable_count))
        .ok_or_else(|| {
            aggregate_posture_error(
                "counts_invalid",
                "local check aggregate posture coverage counts are invalid",
            )
        })?;
    if terminal_count != coverage.expected_count {
        return Err(aggregate_posture_error(
            "counts_invalid",
            "local check aggregate posture coverage counts are invalid",
        ));
    }
    let unavailable_count = coverage
        .required_unavailable_count
        .checked_add(coverage.optional_unavailable_count)
        .ok_or_else(|| {
            aggregate_posture_error(
                "counts_invalid",
                "local check aggregate posture coverage counts are invalid",
            )
        })?;
    if coverage.missing_count > unavailable_count {
        return Err(aggregate_posture_error(
            "missing_count_invalid",
            "local check aggregate posture missing coverage count is invalid",
        ));
    }
    let expected_disposition = if coverage.failed_count > 0 {
        LocalCheckGovernanceStructuralCoverageDisposition::Failed
    } else if coverage.required_unavailable_count > 0 {
        LocalCheckGovernanceStructuralCoverageDisposition::RequiredUnavailable
    } else if coverage.optional_unavailable_count > 0 {
        LocalCheckGovernanceStructuralCoverageDisposition::OptionalUnavailable
    } else {
        LocalCheckGovernanceStructuralCoverageDisposition::Satisfied
    };
    if expected_disposition != coverage.disposition {
        return Err(aggregate_posture_error(
            "disposition_mismatch",
            "local check aggregate posture disposition does not match coverage",
        ));
    }
    Ok(())
}

pub(crate) fn adapt_stored_canonical_local_check_declarations(
    stored_bundle: &StoredImmutableRunBundle,
    step_id: &StepId,
) -> Result<LocalCheckGovernanceObligationSetCandidate, WorkflowOsError> {
    let manifest = stored_bundle.manifest();
    let record = authoritative_record_for_step(stored_bundle, step_id)?;
    if record.workflow_id() != manifest.workflow_id()
        || record.workflow_version() != manifest.workflow_version()
        || record.immutable_bundle_version() != manifest.bundle_version()
    {
        return Err(coverage_error(
            "authoritative_source_mismatch",
            "stored immutable run bundle declaration identity does not match",
        ));
    }

    let obligations = record
        .declarations()
        .iter()
        .map(|declaration| {
            LocalCheckGovernanceObligationDefinition::new(
                declaration.attestation_requirement_fingerprint().clone(),
                match declaration.requirement_level() {
                    LocalCheckRequirementLevel::Required => {
                        LocalCheckGovernanceRequirementLevel::Required
                    }
                    LocalCheckRequirementLevel::Optional => {
                        LocalCheckGovernanceRequirementLevel::Optional
                    }
                },
            )
        })
        .collect();
    LocalCheckGovernanceObligationSetCandidate::new_with_source(
        LocalCheckGovernanceObligationSetCandidateDefinition {
            bundle_id: manifest.bundle_id().clone(),
            bundle_version: manifest.bundle_version().clone(),
            bundle_root: manifest.root_hash().clone(),
            workflow_id: manifest.workflow_id().clone(),
            workflow_version: manifest.workflow_version().clone(),
            run_id: manifest.run_id().clone(),
            step_id: step_id.clone(),
            obligations,
        },
        LocalCheckGovernanceDeclarationSourcePosture::CanonicalStoredBundle,
    )
}

fn authoritative_record_for_step<'a>(
    stored_bundle: &'a StoredImmutableRunBundle,
    step_id: &StepId,
) -> Result<&'a CanonicalLocalCheckDeclarationSetRecord, WorkflowOsError> {
    let manifest = stored_bundle.manifest();
    let references = manifest.local_check_declaration_sets();
    if references.is_empty() {
        return Err(coverage_error(
            "authoritative_source_missing",
            "stored immutable run bundle has no authoritative local check declaration source",
        ));
    }
    let skill_references = manifest
        .definitions()
        .iter()
        .filter(|reference| reference.kind() == ImmutableRunBundleDefinitionKind::Skill)
        .collect::<Vec<_>>();
    let expected_steps = skill_references
        .iter()
        .map(|reference| {
            reference.step_id().cloned().ok_or_else(|| {
                coverage_error(
                    "authoritative_source_mismatch",
                    "stored immutable run bundle has an invalid step binding",
                )
            })
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if expected_steps.len() != skill_references.len() {
        return Err(coverage_error(
            "authoritative_source_duplicate",
            "stored immutable run bundle repeats an authoritative step binding",
        ));
    }
    if !expected_steps.contains(step_id) {
        return Err(coverage_error(
            "authoritative_step_missing",
            "requested step is not present in the stored immutable run bundle",
        ));
    }
    let referenced_steps = references
        .iter()
        .map(|reference| reference.step_id().clone())
        .collect::<BTreeSet<_>>();
    if referenced_steps.len() != references.len() {
        return Err(coverage_error(
            "authoritative_source_duplicate",
            "stored immutable run bundle repeats an authoritative declaration source",
        ));
    }
    if referenced_steps != expected_steps {
        return Err(coverage_error(
            "authoritative_source_incomplete",
            "stored immutable run bundle does not cover every workflow step",
        ));
    }
    let records = stored_bundle.local_check_declaration_set_records();
    let recorded_steps = records
        .iter()
        .map(|record| record.step_id().clone())
        .collect::<BTreeSet<_>>();
    if recorded_steps.len() != records.len() {
        return Err(coverage_error(
            "authoritative_source_duplicate",
            "stored immutable run bundle repeats an authoritative declaration record",
        ));
    }
    if recorded_steps != expected_steps || records.len() != references.len() {
        return Err(coverage_error(
            "authoritative_source_incomplete",
            "stored immutable run bundle is missing an authoritative declaration record",
        ));
    }
    if references.iter().any(|reference| {
        records
            .iter()
            .filter(|record| {
                record.step_id() == reference.step_id()
                    && record.declaration_set_fingerprint()
                        == reference.declaration_set_fingerprint()
            })
            .count()
            != 1
    }) {
        return Err(coverage_error(
            "authoritative_source_mismatch",
            "stored immutable run bundle declaration binding does not match",
        ));
    }
    records
        .iter()
        .find(|record| record.step_id() == step_id)
        .ok_or_else(|| {
            coverage_error(
                "authoritative_step_missing",
                "requested step has no authoritative declaration record",
            )
        })
}

pub(crate) fn adapt_docs_check_contribution(
    candidate_set: &LocalCheckGovernanceObligationSetCandidate,
    contribution: &DocsCheckGovernanceEvidenceCheckContribution,
    requirement_level: LocalCheckGovernanceRequirementLevel,
) -> Result<LocalCheckGovernanceContribution, WorkflowOsError> {
    let obligation = candidate_set
        .obligations
        .iter()
        .find(|obligation| {
            obligation.obligation_fingerprint == *contribution.obligation_fingerprint()
        })
        .ok_or_else(|| {
            coverage_error(
                "contribution_unexpected",
                "local check governance coverage contains an unexpected contribution",
            )
        })?;
    if obligation.requirement_level != requirement_level {
        return Err(coverage_error(
            "contribution_level_mismatch",
            "local check governance contribution level does not match the candidate obligation",
        ));
    }
    let posture = match (requirement_level, contribution.posture()) {
        (_, GovernanceEvidenceCheckContributionPosture::Satisfied) => {
            LocalCheckGovernanceContributionPosture::Satisfied
        }
        (_, GovernanceEvidenceCheckContributionPosture::Failed) => {
            LocalCheckGovernanceContributionPosture::Failed
        }
        (
            LocalCheckGovernanceRequirementLevel::Required,
            GovernanceEvidenceCheckContributionPosture::RequiredUnavailable,
        ) => LocalCheckGovernanceContributionPosture::RequiredUnavailable,
        (
            LocalCheckGovernanceRequirementLevel::Optional,
            GovernanceEvidenceCheckContributionPosture::RequiredUnavailable,
        ) => LocalCheckGovernanceContributionPosture::OptionalUnavailable,
    };
    Ok(LocalCheckGovernanceContribution::new(
        candidate_set.candidate_set_fingerprint.clone(),
        contribution.obligation_fingerprint().clone(),
        posture,
    ))
}

pub(crate) fn evaluate_local_check_structural_coverage(
    candidate_set: &LocalCheckGovernanceObligationSetCandidate,
    contributions: &[LocalCheckGovernanceContribution],
) -> Result<LocalCheckGovernanceStructuralCoverageCandidate, WorkflowOsError> {
    let mut supplied = BTreeMap::new();
    for contribution in contributions {
        if contribution.candidate_set_fingerprint != candidate_set.candidate_set_fingerprint {
            return Err(coverage_error(
                "contribution_binding_mismatch",
                "local check governance contribution binding does not match the candidate set",
            ));
        }
        if supplied
            .insert(
                contribution.obligation_fingerprint.clone(),
                contribution.posture,
            )
            .is_some()
        {
            return Err(coverage_error(
                "contribution_duplicate",
                "local check governance coverage contains a duplicate contribution",
            ));
        }
    }

    let expected = candidate_set
        .obligations
        .iter()
        .map(|obligation| obligation.obligation_fingerprint.clone())
        .collect::<BTreeSet<_>>();
    if supplied.keys().any(|identity| !expected.contains(identity)) {
        return Err(coverage_error(
            "contribution_unexpected",
            "local check governance coverage contains an unexpected contribution",
        ));
    }

    let mut counts = CoverageCounts::default();
    for obligation in &candidate_set.obligations {
        if let Some(posture) = supplied.get(&obligation.obligation_fingerprint).copied() {
            validate_posture_for_level(obligation.requirement_level, posture)?;
            counts.record(posture, false);
        } else {
            let posture = match obligation.requirement_level {
                LocalCheckGovernanceRequirementLevel::Required => {
                    LocalCheckGovernanceContributionPosture::RequiredUnavailable
                }
                LocalCheckGovernanceRequirementLevel::Optional => {
                    LocalCheckGovernanceContributionPosture::OptionalUnavailable
                }
            };
            counts.record(posture, true);
        }
    }

    let disposition = counts.disposition();
    let structural_coverage_fingerprint = structural_coverage_fingerprint(
        candidate_set.candidate_set_fingerprint(),
        contributions,
        &counts,
        disposition,
    );
    Ok(LocalCheckGovernanceStructuralCoverageCandidate {
        source_posture: candidate_set.source_posture,
        disposition,
        expected_count: candidate_set.obligations.len(),
        satisfied_count: counts.satisfied,
        failed_count: counts.failed,
        required_unavailable_count: counts.required_unavailable,
        optional_unavailable_count: counts.optional_unavailable,
        missing_count: counts.missing,
        candidate_set_fingerprint: candidate_set.candidate_set_fingerprint.clone(),
        structural_coverage_fingerprint,
    })
}

#[derive(Default)]
struct CoverageCounts {
    satisfied: usize,
    failed: usize,
    required_unavailable: usize,
    optional_unavailable: usize,
    missing: usize,
}

impl CoverageCounts {
    fn record(&mut self, posture: LocalCheckGovernanceContributionPosture, missing: bool) {
        match posture {
            LocalCheckGovernanceContributionPosture::Satisfied => self.satisfied += 1,
            LocalCheckGovernanceContributionPosture::Failed => self.failed += 1,
            LocalCheckGovernanceContributionPosture::RequiredUnavailable => {
                self.required_unavailable += 1;
            }
            LocalCheckGovernanceContributionPosture::OptionalUnavailable => {
                self.optional_unavailable += 1;
            }
        }
        if missing {
            self.missing += 1;
        }
    }

    const fn disposition(&self) -> LocalCheckGovernanceStructuralCoverageDisposition {
        if self.failed > 0 {
            LocalCheckGovernanceStructuralCoverageDisposition::Failed
        } else if self.required_unavailable > 0 {
            LocalCheckGovernanceStructuralCoverageDisposition::RequiredUnavailable
        } else if self.optional_unavailable > 0 {
            LocalCheckGovernanceStructuralCoverageDisposition::OptionalUnavailable
        } else {
            LocalCheckGovernanceStructuralCoverageDisposition::Satisfied
        }
    }
}

fn validate_posture_for_level(
    requirement_level: LocalCheckGovernanceRequirementLevel,
    posture: LocalCheckGovernanceContributionPosture,
) -> Result<(), WorkflowOsError> {
    let invalid = matches!(
        (requirement_level, posture),
        (
            LocalCheckGovernanceRequirementLevel::Required,
            LocalCheckGovernanceContributionPosture::OptionalUnavailable
        ) | (
            LocalCheckGovernanceRequirementLevel::Optional,
            LocalCheckGovernanceContributionPosture::RequiredUnavailable
        )
    );
    if invalid {
        return Err(coverage_error(
            "contribution_posture_mismatch",
            "local check governance contribution posture does not match its requirement level",
        ));
    }
    Ok(())
}

fn candidate_set_fingerprint(
    definition: &LocalCheckGovernanceObligationSetCandidateDefinition,
    source_posture: LocalCheckGovernanceDeclarationSourcePosture,
    obligations: &[LocalCheckGovernanceObligation],
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "algorithm", CANDIDATE_ALGORITHM);
    hash_field(
        &mut hasher,
        "source_posture",
        source_posture_label(source_posture),
    );
    hash_field(&mut hasher, "bundle_id", definition.bundle_id.as_str());
    hash_field(
        &mut hasher,
        "bundle_version",
        definition.bundle_version.as_str(),
    );
    hash_field(&mut hasher, "bundle_root", definition.bundle_root.as_str());
    hash_field(&mut hasher, "workflow_id", definition.workflow_id.as_str());
    hash_field(
        &mut hasher,
        "workflow_version",
        definition.workflow_version.as_str(),
    );
    hash_field(&mut hasher, "run_id", definition.run_id.as_str());
    hash_field(&mut hasher, "step_id", definition.step_id.as_str());
    for obligation in obligations {
        hash_field(
            &mut hasher,
            "obligation",
            obligation.obligation_fingerprint.as_str(),
        );
        hash_field(
            &mut hasher,
            "requirement",
            obligation.requirement_fingerprint.as_str(),
        );
        hash_field(
            &mut hasher,
            "requirement_level",
            requirement_level_label(obligation.requirement_level),
        );
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

const fn source_posture_label(
    posture: LocalCheckGovernanceDeclarationSourcePosture,
) -> &'static str {
    match posture {
        LocalCheckGovernanceDeclarationSourcePosture::Unresolved => "unresolved",
        LocalCheckGovernanceDeclarationSourcePosture::CanonicalStoredBundle => {
            "canonical_stored_bundle"
        }
    }
}

fn structural_coverage_fingerprint(
    candidate_set_fingerprint: &SpecContentHash,
    contributions: &[LocalCheckGovernanceContribution],
    counts: &CoverageCounts,
    disposition: LocalCheckGovernanceStructuralCoverageDisposition,
) -> SpecContentHash {
    let mut ordered = contributions.to_vec();
    ordered.sort_by(|left, right| {
        left.obligation_fingerprint
            .cmp(&right.obligation_fingerprint)
    });
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "algorithm", CANDIDATE_ALGORITHM);
    hash_field(
        &mut hasher,
        "candidate_set_fingerprint",
        candidate_set_fingerprint.as_str(),
    );
    for contribution in ordered {
        hash_field(
            &mut hasher,
            "contribution",
            contribution.obligation_fingerprint.as_str(),
        );
        hash_field(
            &mut hasher,
            "posture",
            contribution_posture_label(contribution.posture),
        );
    }
    hash_field(&mut hasher, "disposition", disposition_label(disposition));
    hash_field(&mut hasher, "missing_count", &counts.missing.to_string());
    SpecContentHash::from_bytes(hasher.finalize())
}

fn aggregate_fact_fingerprint(
    algorithm: AuthoritativeLocalCheckEvidenceCheckFactAlgorithm,
    posture: GovernanceWorkloadEvidenceCheckPosture,
    coverage: &LocalCheckGovernanceStructuralCoverageCandidate,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "algorithm", algorithm.identifier());
    hash_field(&mut hasher, "scope", "canonical_local_checks");
    hash_field(&mut hasher, "posture", aggregate_posture_label(posture));
    hash_field(
        &mut hasher,
        "expected_count",
        &coverage.expected_count.to_string(),
    );
    hash_field(
        &mut hasher,
        "satisfied_count",
        &coverage.satisfied_count.to_string(),
    );
    hash_field(
        &mut hasher,
        "failed_count",
        &coverage.failed_count.to_string(),
    );
    hash_field(
        &mut hasher,
        "required_unavailable_count",
        &coverage.required_unavailable_count.to_string(),
    );
    hash_field(
        &mut hasher,
        "optional_unavailable_count",
        &coverage.optional_unavailable_count.to_string(),
    );
    hash_field(
        &mut hasher,
        "missing_count",
        &coverage.missing_count.to_string(),
    );
    hash_field(
        &mut hasher,
        "candidate_set_fingerprint",
        coverage.candidate_set_fingerprint.as_str(),
    );
    hash_field(
        &mut hasher,
        "structural_coverage_fingerprint",
        coverage.structural_coverage_fingerprint.as_str(),
    );
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update(u64::try_from(label.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(value.as_bytes());
}

const fn requirement_level_label(level: LocalCheckGovernanceRequirementLevel) -> &'static str {
    match level {
        LocalCheckGovernanceRequirementLevel::Required => "required",
        LocalCheckGovernanceRequirementLevel::Optional => "optional",
    }
}

const fn contribution_posture_label(
    posture: LocalCheckGovernanceContributionPosture,
) -> &'static str {
    match posture {
        LocalCheckGovernanceContributionPosture::Satisfied => "satisfied",
        LocalCheckGovernanceContributionPosture::OptionalUnavailable => "optional_unavailable",
        LocalCheckGovernanceContributionPosture::RequiredUnavailable => "required_unavailable",
        LocalCheckGovernanceContributionPosture::Failed => "failed",
    }
}

const fn disposition_label(
    disposition: LocalCheckGovernanceStructuralCoverageDisposition,
) -> &'static str {
    match disposition {
        LocalCheckGovernanceStructuralCoverageDisposition::Satisfied => "satisfied",
        LocalCheckGovernanceStructuralCoverageDisposition::OptionalUnavailable => {
            "optional_unavailable"
        }
        LocalCheckGovernanceStructuralCoverageDisposition::RequiredUnavailable => {
            "required_unavailable"
        }
        LocalCheckGovernanceStructuralCoverageDisposition::Failed => "failed",
    }
}

const fn aggregate_posture_label(posture: GovernanceWorkloadEvidenceCheckPosture) -> &'static str {
    match posture {
        GovernanceWorkloadEvidenceCheckPosture::Satisfied => "satisfied",
        GovernanceWorkloadEvidenceCheckPosture::OptionalUnavailable => "optional_unavailable",
        GovernanceWorkloadEvidenceCheckPosture::RequiredUnavailable => "required_unavailable",
        GovernanceWorkloadEvidenceCheckPosture::Failed => "failed",
        GovernanceWorkloadEvidenceCheckPosture::Unknown => "unknown",
    }
}

fn coverage_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("local_check_attestation.structural_coverage.{suffix}"),
        message,
    )
}

fn aggregate_posture_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("local_check_attestation.aggregate_posture.{suffix}"),
        message,
    )
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;
    use crate::{
        build_immutable_run_bundle, build_immutable_run_bundle_with_local_check_declarations,
        canonical_yaml_content_hash, load_project, parse_skill_spec_yaml, validate_project_bundle,
        ActorId, ImmutableRunBundleBuildRequest, ImmutableRunBundleDefinitionRecord,
        ImmutableRunBundleExecutionPosture, ImmutableRunBundleHandlerPosture,
        ImmutableRunBundleHandlerReference, ImmutableRunBundleManifest,
        ImmutableRunBundleReferencePosture, ImmutableRunBundleSensitivity,
        LocalCheckCommandContract, LocalCheckCommandContractInventory,
        LocalImmutableRunBundleStore, SkillId, SkillVersion, Timestamp, SUPPORTED_SCHEMA_VERSION,
    };

    static NEXT_TEST_PROJECT: AtomicU64 = AtomicU64::new(1);

    struct TestProject {
        root: PathBuf,
    }

    impl TestProject {
        fn new() -> Self {
            let id = NEXT_TEST_PROJECT.fetch_add(1, Ordering::Relaxed);
            let root = std::env::temp_dir().join(format!(
                "workflow-os-structural-coverage-adapter-{}-{id}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&root);
            fs::create_dir_all(&root).expect("test root");
            let project = Self { root };
            project.write_valid_project();
            project
        }

        fn path(&self) -> &Path {
            &self.root
        }

        fn write(&self, relative: &str, content: &str) {
            let path = self.root.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("parent");
            }
            fs::write(path, content).expect("fixture");
        }

        fn write_valid_project(&self) {
            self.write(
                "workflow-os.yml",
                &format!(
                    "schema_version: {SUPPORTED_SCHEMA_VERSION}\nproject:\n  id: coverage/project\n  name: Coverage Project\n"
                ),
            );
            self.write(
                "workflows/build.workflow.yml",
                &format!(
                    r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: coverage/build
version: v1
display_name: Coverage Build
triggers:
  - id: manual-start
    kind: manual
steps:
  - id: inspect
    skill_ref:
      id: local/check
      version: v1
    policy_requirements:
      - id: local/read-only
    local_check_requirements:
      - id: docs-required
        command_id: local-check/docs
        requirement_level: required
        minimum_assurance: kernel_observed_local_process
        accepted_statuses: [passed]
        freshness:
          mode: no_reuse
        exact_immutable_run_binding_required: true
        truncation_allowed: false
        network_maximum: disabled
        side_effect_maximum: no_source_writes
    terminal_behavior: fail_workflow
  - id: verify
    skill_ref:
      id: local/check
      version: v1
    policy_requirements:
      - id: local/read-only
    terminal_behavior: fail_workflow
cancellation_behavior: stop
audit_requirements:
  required: true
  events: [RunCreated, RunCompleted]
  store_references_only: true
observability_requirements:
  metrics: [workflow_latency]
  tracing: true
  latency_tracking: true
"
                ),
            );
            self.write(
                "skills/check.skill.yml",
                &format!(
                    r"
schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/check
version: v1
display_name: Local Check
input_contract:
  fields:
    - name: request
      field_type: string
output_contract:
  fields:
    - name: summary
      field_type: string
failure_modes:
  - code: check_failed
    description: Local check failed.
    retryable: false
audit_requirements:
  required: true
  events: [SkillInvocationRequested]
  store_references_only: true
observability_requirements:
  metrics: [skill_latency]
  tracing: true
  latency_tracking: true
"
                ),
            );
            self.write(
                "policies/read-only.policy.yml",
                &format!(
                    "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/read-only\nname: Read Only\nrules:\n  - id: allow-local\n    effect: allow_local\n"
                ),
            );
        }
    }

    impl Drop for TestProject {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    fn stored_bundle(authoritative: bool) -> StoredImmutableRunBundle {
        let project = TestProject::new();
        let loaded = load_project(project.path());
        assert!(!loaded.has_errors(), "{:?}", loaded.diagnostics);
        let bundle = loaded.bundle.expect("bundle");
        let validation = validate_project_bundle(&bundle);
        assert!(!validation.has_errors(), "{:?}", validation.diagnostics);
        let workflow_id = WorkflowId::new("coverage/build").expect("workflow");
        let request = ImmutableRunBundleBuildRequest {
            project: &bundle,
            workflow_id: &workflow_id,
            bundle_id: ImmutableRunBundleId::new("bundle/coverage").expect("bundle id"),
            bundle_version: ImmutableRunBundleVersion::new("v1").expect("bundle version"),
            run_id: WorkflowRunId::new("run-coverage").expect("run"),
            resolved_execution_context_hash: SpecContentHash::from_text("context"),
            execution_posture: ImmutableRunBundleExecutionPosture::new(
                vec![StepId::new("inspect").expect("step")],
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::CommittedReference,
            )
            .expect("posture"),
            handlers: vec![ImmutableRunBundleHandlerReference {
                skill_id: SkillId::new("local/check").expect("skill"),
                skill_version: SkillVersion::new("v1").expect("skill version"),
                posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
            }],
            created_at: Timestamp::parse_rfc3339("2026-07-25T12:00:00Z").expect("timestamp"),
            created_by: ActorId::new("system/kernel").expect("actor"),
            sensitivity: ImmutableRunBundleSensitivity::Internal,
            redaction_required: true,
        };
        let built = if authoritative {
            let inventory = LocalCheckCommandContractInventory::new(vec![
                LocalCheckCommandContract::docs_check_model_only().expect("contract"),
            ])
            .expect("inventory");
            build_immutable_run_bundle_with_local_check_declarations(request, &inventory)
                .expect("authoritative bundle")
        } else {
            build_immutable_run_bundle(request).expect("legacy bundle")
        };
        let store = LocalImmutableRunBundleStore::new(project.path().join("bundle-store"));
        store.write_bundle(&built).expect("store bundle");
        store
            .read_bundle(built.manifest().run_id(), built.manifest().bundle_id())
            .expect("read stored bundle")
    }

    fn stored_bundle_with_duplicate_skill_step_binding() -> StoredImmutableRunBundle {
        const SECOND_SKILL_YAML: &str = r"
schema_version: workflowos.dev/v0
id: local/second-check
version: v1
display_name: Second Local Check
input_contract:
  fields:
    - name: request
      field_type: string
output_contract:
  fields:
    - name: summary
      field_type: string
failure_modes:
  - code: check_failed
    description: Local check failed.
    retryable: false
audit_requirements:
  required: true
  events: [SkillInvocationRequested]
  store_references_only: true
observability_requirements:
  metrics: [skill_latency]
  tracing: true
  latency_tracking: true
";

        let stored = stored_bundle(true);
        let manifest = stored.manifest();
        let second_record = ImmutableRunBundleDefinitionRecord::from_skill(
            manifest.bundle_version().clone(),
            parse_skill_spec_yaml(SECOND_SKILL_YAML).expect("second skill"),
            canonical_yaml_content_hash(SECOND_SKILL_YAML).expect("second skill hash"),
            manifest.sensitivity(),
            manifest.redaction_required(),
        )
        .expect("second skill record");
        let mut definitions = manifest.definitions().to_vec();
        definitions.push(
            second_record
                .definition_reference(Some(StepId::new("inspect").expect("step")))
                .expect("second skill reference"),
        );
        let mut handlers = manifest.handlers().to_vec();
        handlers.push(ImmutableRunBundleHandlerReference {
            skill_id: SkillId::new("local/second-check").expect("skill"),
            skill_version: SkillVersion::new("v1").expect("skill version"),
            posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
        });
        let forged_manifest = ImmutableRunBundleManifest::new_with_local_check_declaration_sets(
            manifest.bundle_id().clone(),
            manifest.bundle_version().clone(),
            manifest.run_id().clone(),
            manifest.workflow_id().clone(),
            manifest.workflow_version().clone(),
            manifest.schema_version().clone(),
            manifest.workflow_content_hash().clone(),
            manifest.resolved_execution_context_hash().clone(),
            definitions,
            manifest.local_check_declaration_sets().to_vec(),
            manifest.execution_posture().clone(),
            handlers,
            *manifest.created_at(),
            manifest.created_by().clone(),
            manifest.sensitivity(),
            manifest.redaction_required(),
        )
        .expect("manifest accepts distinct skill references with one step binding");
        let mut records = stored.definition_records().to_vec();
        records.push(second_record);
        StoredImmutableRunBundle::from_validated_parts_for_test(
            forged_manifest,
            records,
            stored.local_check_declaration_set_records().to_vec(),
        )
    }

    fn obligation(
        label: &str,
        level: LocalCheckGovernanceRequirementLevel,
    ) -> LocalCheckGovernanceObligationDefinition {
        LocalCheckGovernanceObligationDefinition::new(SpecContentHash::from_text(label), level)
    }

    fn candidate(
        obligations: Vec<LocalCheckGovernanceObligationDefinition>,
    ) -> LocalCheckGovernanceObligationSetCandidate {
        candidate_with_source(
            obligations,
            LocalCheckGovernanceDeclarationSourcePosture::Unresolved,
        )
    }

    fn authoritative_candidate(
        obligations: Vec<LocalCheckGovernanceObligationDefinition>,
    ) -> LocalCheckGovernanceObligationSetCandidate {
        candidate_with_source(
            obligations,
            LocalCheckGovernanceDeclarationSourcePosture::CanonicalStoredBundle,
        )
    }

    fn candidate_with_source(
        obligations: Vec<LocalCheckGovernanceObligationDefinition>,
        source_posture: LocalCheckGovernanceDeclarationSourcePosture,
    ) -> LocalCheckGovernanceObligationSetCandidate {
        LocalCheckGovernanceObligationSetCandidate::new_with_source(
            LocalCheckGovernanceObligationSetCandidateDefinition {
                bundle_id: ImmutableRunBundleId::new("bundle/test").expect("bundle id"),
                bundle_version: ImmutableRunBundleVersion::new("v1").expect("bundle version"),
                bundle_root: SpecContentHash::from_text("bundle root"),
                workflow_id: WorkflowId::new("workflow/test").expect("workflow id"),
                workflow_version: WorkflowVersion::new("v1").expect("workflow version"),
                run_id: WorkflowRunId::new("run-test").expect("run id"),
                step_id: StepId::new("check-docs").expect("step id"),
                obligations,
            },
            source_posture,
        )
        .expect("candidate set")
    }

    fn contribution(
        candidate_set: &LocalCheckGovernanceObligationSetCandidate,
        obligation: &LocalCheckGovernanceObligationDefinition,
        posture: LocalCheckGovernanceContributionPosture,
    ) -> LocalCheckGovernanceContribution {
        let bound_obligation = candidate_set
            .obligations()
            .iter()
            .find(|bound| {
                bound.requirement_fingerprint == obligation.requirement_fingerprint
                    && bound.requirement_level == obligation.requirement_level
            })
            .expect("bound obligation");
        LocalCheckGovernanceContribution::new(
            candidate_set.candidate_set_fingerprint().clone(),
            bound_obligation.obligation_fingerprint().clone(),
            posture,
        )
    }

    #[test]
    fn complete_candidate_coverage_is_structurally_satisfied_and_unresolved() {
        let required = obligation("required", LocalCheckGovernanceRequirementLevel::Required);
        let set = candidate(vec![required.clone()]);
        let result = evaluate_local_check_structural_coverage(
            &set,
            &[contribution(
                &set,
                &required,
                LocalCheckGovernanceContributionPosture::Satisfied,
            )],
        )
        .expect("coverage");

        assert_eq!(
            result.disposition(),
            LocalCheckGovernanceStructuralCoverageDisposition::Satisfied
        );
        assert_eq!(result.expected_count(), 1);
        assert_eq!(result.satisfied_count(), 1);
        assert!(format!("{result:?}").contains("Unresolved"));
    }

    #[test]
    fn missing_required_and_optional_obligations_remain_distinct() {
        let required = obligation("required", LocalCheckGovernanceRequirementLevel::Required);
        let optional = obligation("optional", LocalCheckGovernanceRequirementLevel::Optional);
        let required_result =
            evaluate_local_check_structural_coverage(&candidate(vec![required]), &[])
                .expect("required coverage");
        let optional_result =
            evaluate_local_check_structural_coverage(&candidate(vec![optional]), &[])
                .expect("optional coverage");

        assert_eq!(
            required_result.disposition(),
            LocalCheckGovernanceStructuralCoverageDisposition::RequiredUnavailable
        );
        assert_eq!(required_result.required_unavailable_count(), 1);
        assert_eq!(required_result.missing_count(), 1);
        assert_eq!(
            optional_result.disposition(),
            LocalCheckGovernanceStructuralCoverageDisposition::OptionalUnavailable
        );
        assert_eq!(optional_result.optional_unavailable_count(), 1);
    }

    #[test]
    fn failed_optional_check_remains_failed() {
        let optional = obligation("optional", LocalCheckGovernanceRequirementLevel::Optional);
        let set = candidate(vec![optional.clone()]);
        let result = evaluate_local_check_structural_coverage(
            &set,
            &[contribution(
                &set,
                &optional,
                LocalCheckGovernanceContributionPosture::Failed,
            )],
        )
        .expect("coverage");

        assert_eq!(
            result.disposition(),
            LocalCheckGovernanceStructuralCoverageDisposition::Failed
        );
        assert_eq!(result.failed_count(), 1);
    }

    #[test]
    fn failure_cannot_be_masked_by_satisfied_or_missing_optional_coverage() {
        let failed = obligation("failed", LocalCheckGovernanceRequirementLevel::Required);
        let satisfied = obligation("satisfied", LocalCheckGovernanceRequirementLevel::Required);
        let optional = obligation("optional", LocalCheckGovernanceRequirementLevel::Optional);
        let set = candidate(vec![failed.clone(), satisfied.clone(), optional]);
        let result = evaluate_local_check_structural_coverage(
            &set,
            &[
                contribution(
                    &set,
                    &failed,
                    LocalCheckGovernanceContributionPosture::Failed,
                ),
                contribution(
                    &set,
                    &satisfied,
                    LocalCheckGovernanceContributionPosture::Satisfied,
                ),
            ],
        )
        .expect("coverage");

        assert_eq!(
            result.disposition(),
            LocalCheckGovernanceStructuralCoverageDisposition::Failed
        );
        assert_eq!(result.failed_count(), 1);
        assert_eq!(result.optional_unavailable_count(), 1);
    }

    #[test]
    fn duplicate_expected_and_supplied_identities_fail_closed() {
        let required = obligation("duplicate", LocalCheckGovernanceRequirementLevel::Required);
        let set_error = LocalCheckGovernanceObligationSetCandidate::new(
            LocalCheckGovernanceObligationSetCandidateDefinition {
                bundle_id: ImmutableRunBundleId::new("bundle/test").expect("bundle id"),
                bundle_version: ImmutableRunBundleVersion::new("v1").expect("bundle version"),
                bundle_root: SpecContentHash::from_text("bundle root"),
                workflow_id: WorkflowId::new("workflow/test").expect("workflow id"),
                workflow_version: WorkflowVersion::new("v1").expect("workflow version"),
                run_id: WorkflowRunId::new("run-test").expect("run id"),
                step_id: StepId::new("check-docs").expect("step id"),
                obligations: vec![required.clone(), required.clone()],
            },
        )
        .expect_err("duplicate set rejected");
        assert_eq!(
            set_error.code(),
            "local_check_attestation.structural_coverage.obligation_duplicate"
        );

        let set = candidate(vec![required.clone()]);
        let duplicate = contribution(
            &set,
            &required,
            LocalCheckGovernanceContributionPosture::Satisfied,
        );
        let contribution_error =
            evaluate_local_check_structural_coverage(&set, &[duplicate.clone(), duplicate])
                .expect_err("duplicate contribution rejected");
        assert_eq!(
            contribution_error.code(),
            "local_check_attestation.structural_coverage.contribution_duplicate"
        );
    }

    #[test]
    fn unexpected_and_requirement_level_mismatched_contributions_fail_closed() {
        let required = obligation("required", LocalCheckGovernanceRequirementLevel::Required);
        let set = candidate(vec![required.clone()]);
        let unexpected = LocalCheckGovernanceContribution::new(
            set.candidate_set_fingerprint().clone(),
            SpecContentHash::from_text("unexpected"),
            LocalCheckGovernanceContributionPosture::Satisfied,
        );
        assert_eq!(
            evaluate_local_check_structural_coverage(&set, &[unexpected])
                .expect_err("unexpected rejected")
                .code(),
            "local_check_attestation.structural_coverage.contribution_unexpected"
        );
        assert_eq!(
            evaluate_local_check_structural_coverage(
                &set,
                &[contribution(
                    &set,
                    &required,
                    LocalCheckGovernanceContributionPosture::OptionalUnavailable,
                )],
            )
            .expect_err("mismatch rejected")
            .code(),
            "local_check_attestation.structural_coverage.contribution_posture_mismatch"
        );
    }

    #[test]
    fn order_does_not_change_set_or_structural_coverage_fingerprints() {
        let first = obligation("first", LocalCheckGovernanceRequirementLevel::Required);
        let second = obligation("second", LocalCheckGovernanceRequirementLevel::Optional);
        let first_set = candidate(vec![first.clone(), second.clone()]);
        let second_set = candidate(vec![second.clone(), first.clone()]);
        assert_eq!(
            first_set.candidate_set_fingerprint(),
            second_set.candidate_set_fingerprint()
        );

        let first_contribution = contribution(
            &first_set,
            &first,
            LocalCheckGovernanceContributionPosture::Satisfied,
        );
        let second_contribution = contribution(
            &first_set,
            &second,
            LocalCheckGovernanceContributionPosture::OptionalUnavailable,
        );
        let first_result = evaluate_local_check_structural_coverage(
            &first_set,
            &[first_contribution.clone(), second_contribution.clone()],
        )
        .expect("coverage");
        let second_contributions = [
            contribution(
                &second_set,
                &second,
                LocalCheckGovernanceContributionPosture::OptionalUnavailable,
            ),
            contribution(
                &second_set,
                &first,
                LocalCheckGovernanceContributionPosture::Satisfied,
            ),
        ];
        let second_result =
            evaluate_local_check_structural_coverage(&second_set, &second_contributions)
                .expect("coverage");
        assert_eq!(
            first_result.structural_coverage_fingerprint(),
            second_result.structural_coverage_fingerprint()
        );
    }

    #[test]
    fn relevant_binding_or_obligation_changes_invalidate_candidate_identity() {
        let required = obligation("required", LocalCheckGovernanceRequirementLevel::Required);
        let baseline = candidate(vec![required]);
        let changed = candidate(vec![obligation(
            "changed",
            LocalCheckGovernanceRequirementLevel::Required,
        )]);
        assert_ne!(
            baseline.candidate_set_fingerprint(),
            changed.candidate_set_fingerprint()
        );

        let changed_step = LocalCheckGovernanceObligationSetCandidate::new(
            LocalCheckGovernanceObligationSetCandidateDefinition {
                bundle_id: ImmutableRunBundleId::new("bundle/test").expect("bundle id"),
                bundle_version: ImmutableRunBundleVersion::new("v1").expect("bundle version"),
                bundle_root: SpecContentHash::from_text("bundle root"),
                workflow_id: WorkflowId::new("workflow/test").expect("workflow id"),
                workflow_version: WorkflowVersion::new("v1").expect("workflow version"),
                run_id: WorkflowRunId::new("run-test").expect("run id"),
                step_id: StepId::new("other-step").expect("step id"),
                obligations: vec![obligation(
                    "required",
                    LocalCheckGovernanceRequirementLevel::Required,
                )],
            },
        )
        .expect("candidate");
        assert_ne!(
            baseline.candidate_set_fingerprint(),
            changed_step.candidate_set_fingerprint()
        );
    }

    #[test]
    fn contribution_from_another_candidate_binding_fails_closed() {
        let required = obligation("required", LocalCheckGovernanceRequirementLevel::Required);
        let first_set = candidate(vec![required.clone()]);
        let cross_bound = contribution(
            &first_set,
            &required,
            LocalCheckGovernanceContributionPosture::Satisfied,
        );
        let second_set = LocalCheckGovernanceObligationSetCandidate::new(
            LocalCheckGovernanceObligationSetCandidateDefinition {
                bundle_id: ImmutableRunBundleId::new("bundle/other").expect("bundle id"),
                bundle_version: ImmutableRunBundleVersion::new("v1").expect("bundle version"),
                bundle_root: SpecContentHash::from_text("other bundle root"),
                workflow_id: WorkflowId::new("workflow/test").expect("workflow id"),
                workflow_version: WorkflowVersion::new("v1").expect("workflow version"),
                run_id: WorkflowRunId::new("run-test").expect("run id"),
                step_id: StepId::new("check-docs").expect("step id"),
                obligations: vec![required],
            },
        )
        .expect("candidate");

        let error = evaluate_local_check_structural_coverage(&second_set, &[cross_bound])
            .expect_err("cross-bound contribution rejected");
        assert_eq!(
            error.code(),
            "local_check_attestation.structural_coverage.contribution_binding_mismatch"
        );
    }

    #[test]
    fn empty_candidate_is_vacuously_structural_not_authoritative() {
        let result = evaluate_local_check_structural_coverage(&candidate(Vec::new()), &[])
            .expect("empty structural coverage");
        assert_eq!(
            result.disposition(),
            LocalCheckGovernanceStructuralCoverageDisposition::Satisfied
        );
        assert_eq!(result.expected_count(), 0);
        assert!(format!("{result:?}").contains("source_posture: Unresolved"));
    }

    #[test]
    fn stored_canonical_declarations_create_authoritative_candidate() {
        let stored = stored_bundle(true);
        let candidate = adapt_stored_canonical_local_check_declarations(
            &stored,
            &StepId::new("inspect").expect("step"),
        )
        .expect("authoritative candidate");

        assert_eq!(
            candidate.source_posture(),
            LocalCheckGovernanceDeclarationSourcePosture::CanonicalStoredBundle
        );
        assert_eq!(candidate.obligations().len(), 1);
        assert_eq!(
            candidate.obligations()[0].requirement_level(),
            LocalCheckGovernanceRequirementLevel::Required
        );
        let result =
            evaluate_local_check_structural_coverage(&candidate, &[]).expect("coverage candidate");
        assert_eq!(
            result.source_posture(),
            LocalCheckGovernanceDeclarationSourcePosture::CanonicalStoredBundle
        );
        assert_eq!(
            result.disposition(),
            LocalCheckGovernanceStructuralCoverageDisposition::RequiredUnavailable
        );
        assert!(format!("{candidate:?} {result:?}").contains("CanonicalStoredBundle"));
    }

    #[test]
    fn stored_canonical_empty_set_is_authoritative_and_distinct_from_unresolved() {
        let stored = stored_bundle(true);
        let authoritative = adapt_stored_canonical_local_check_declarations(
            &stored,
            &StepId::new("verify").expect("step"),
        )
        .expect("authoritative empty candidate");
        let unresolved = candidate(Vec::new());
        let result = evaluate_local_check_structural_coverage(&authoritative, &[])
            .expect("authoritative empty coverage");

        assert!(authoritative.obligations().is_empty());
        assert_eq!(
            result.disposition(),
            LocalCheckGovernanceStructuralCoverageDisposition::Satisfied
        );
        assert_eq!(
            result.source_posture(),
            LocalCheckGovernanceDeclarationSourcePosture::CanonicalStoredBundle
        );
        assert_ne!(
            authoritative.candidate_set_fingerprint(),
            unresolved.candidate_set_fingerprint()
        );
    }

    #[test]
    fn legacy_bundle_and_unknown_step_fail_closed_without_leaking_identity() {
        let legacy = stored_bundle(false);
        let missing_source = adapt_stored_canonical_local_check_declarations(
            &legacy,
            &StepId::new("inspect").expect("step"),
        )
        .expect_err("legacy bundle rejected");
        assert_eq!(
            missing_source.code(),
            "local_check_attestation.structural_coverage.authoritative_source_missing"
        );

        let authoritative = stored_bundle(true);
        let secret_step = "token-sk-not-a-step";
        let missing_step = adapt_stored_canonical_local_check_declarations(
            &authoritative,
            &StepId::new(secret_step).expect("step"),
        )
        .expect_err("unknown step rejected");
        assert_eq!(
            missing_step.code(),
            "local_check_attestation.structural_coverage.authoritative_step_missing"
        );
        assert!(!missing_step.to_string().contains(secret_step));
        assert!(!format!("{missing_step:?}").contains(secret_step));
    }

    #[test]
    fn duplicate_skill_step_binding_fails_closed_before_step_deduplication() {
        let stored = stored_bundle_with_duplicate_skill_step_binding();
        let error = adapt_stored_canonical_local_check_declarations(
            &stored,
            &StepId::new("inspect").expect("step"),
        )
        .expect_err("duplicate skill step binding rejected");

        assert_eq!(
            error.code(),
            "local_check_attestation.structural_coverage.authoritative_source_duplicate"
        );
    }

    #[test]
    fn authoritative_coverage_maps_each_structural_disposition_exactly() {
        let cases = [
            (
                LocalCheckGovernanceRequirementLevel::Required,
                LocalCheckGovernanceContributionPosture::Satisfied,
                GovernanceWorkloadEvidenceCheckPosture::Satisfied,
            ),
            (
                LocalCheckGovernanceRequirementLevel::Optional,
                LocalCheckGovernanceContributionPosture::OptionalUnavailable,
                GovernanceWorkloadEvidenceCheckPosture::OptionalUnavailable,
            ),
            (
                LocalCheckGovernanceRequirementLevel::Required,
                LocalCheckGovernanceContributionPosture::RequiredUnavailable,
                GovernanceWorkloadEvidenceCheckPosture::RequiredUnavailable,
            ),
            (
                LocalCheckGovernanceRequirementLevel::Optional,
                LocalCheckGovernanceContributionPosture::Failed,
                GovernanceWorkloadEvidenceCheckPosture::Failed,
            ),
        ];

        for (index, (level, contribution_posture, expected_posture)) in
            cases.into_iter().enumerate()
        {
            let obligation = obligation(&format!("case-{index}"), level);
            let set = authoritative_candidate(vec![obligation.clone()]);
            let coverage = evaluate_local_check_structural_coverage(
                &set,
                &[contribution(&set, &obligation, contribution_posture)],
            )
            .expect("authoritative coverage");
            let fact = convert_authoritative_local_check_coverage(&coverage)
                .expect("authoritative conversion");

            assert_eq!(fact.posture(), expected_posture);
            assert_eq!(fact.expected_count(), 1);
            assert_eq!(fact.algorithm().identifier(), AGGREGATE_POSTURE_ALGORITHM);
            assert_eq!(
                fact.candidate_set_fingerprint(),
                coverage.candidate_set_fingerprint()
            );
            assert_eq!(
                fact.structural_coverage_fingerprint(),
                coverage.structural_coverage_fingerprint()
            );
        }
    }

    #[test]
    fn canonical_stored_empty_set_converts_but_unresolved_empty_set_does_not() {
        let stored = stored_bundle(true);
        let canonical = adapt_stored_canonical_local_check_declarations(
            &stored,
            &StepId::new("verify").expect("step"),
        )
        .expect("canonical empty candidate");
        let canonical_coverage = evaluate_local_check_structural_coverage(&canonical, &[])
            .expect("canonical empty coverage");
        let fact = convert_authoritative_local_check_coverage(&canonical_coverage)
            .expect("canonical empty conversion");
        assert_eq!(
            fact.posture(),
            GovernanceWorkloadEvidenceCheckPosture::Satisfied
        );
        assert_eq!(fact.expected_count(), 0);
        assert_eq!(fact.satisfied_count(), 0);
        assert_eq!(fact.failed_count(), 0);
        assert_eq!(fact.required_unavailable_count(), 0);
        assert_eq!(fact.optional_unavailable_count(), 0);
        assert_eq!(fact.missing_count(), 0);

        let unresolved_coverage =
            evaluate_local_check_structural_coverage(&candidate(Vec::new()), &[])
                .expect("unresolved empty coverage");
        let error = convert_authoritative_local_check_coverage(&unresolved_coverage)
            .expect_err("unresolved empty coverage rejected");
        assert_eq!(
            error.code(),
            "local_check_attestation.aggregate_posture.source_not_authoritative"
        );
    }

    #[test]
    fn unresolved_populated_coverage_cannot_be_relabelled_as_authoritative() {
        let secret = "token-sk-unresolved-aggregate";
        let required = obligation(secret, LocalCheckGovernanceRequirementLevel::Required);
        let set = candidate(vec![required.clone()]);
        let coverage = evaluate_local_check_structural_coverage(
            &set,
            &[contribution(
                &set,
                &required,
                LocalCheckGovernanceContributionPosture::Satisfied,
            )],
        )
        .expect("unresolved coverage");

        let error = convert_authoritative_local_check_coverage(&coverage)
            .expect_err("unresolved coverage rejected");
        assert_eq!(
            error.code(),
            "local_check_attestation.aggregate_posture.source_not_authoritative"
        );
        assert!(!error.to_string().contains(secret));
        assert!(!format!("{error:?}").contains(secret));
    }

    #[test]
    fn contradictory_authoritative_coverage_fails_closed() {
        let mut coverage = LocalCheckGovernanceStructuralCoverageCandidate {
            source_posture: LocalCheckGovernanceDeclarationSourcePosture::CanonicalStoredBundle,
            disposition: LocalCheckGovernanceStructuralCoverageDisposition::Satisfied,
            expected_count: 1,
            satisfied_count: 0,
            failed_count: 1,
            required_unavailable_count: 0,
            optional_unavailable_count: 0,
            missing_count: 0,
            candidate_set_fingerprint: SpecContentHash::from_text("candidate"),
            structural_coverage_fingerprint: SpecContentHash::from_text("coverage"),
        };
        let disposition_error = convert_authoritative_local_check_coverage(&coverage)
            .expect_err("contradictory disposition rejected");
        assert_eq!(
            disposition_error.code(),
            "local_check_attestation.aggregate_posture.disposition_mismatch"
        );

        coverage.expected_count = 2;
        let count_error = convert_authoritative_local_check_coverage(&coverage)
            .expect_err("contradictory counts rejected");
        assert_eq!(
            count_error.code(),
            "local_check_attestation.aggregate_posture.counts_invalid"
        );

        coverage.expected_count = 1;
        coverage.disposition = LocalCheckGovernanceStructuralCoverageDisposition::Failed;
        coverage.missing_count = 1;
        let missing_error = convert_authoritative_local_check_coverage(&coverage)
            .expect_err("invalid missing count rejected");
        assert_eq!(
            missing_error.code(),
            "local_check_attestation.aggregate_posture.missing_count_invalid"
        );
    }

    #[test]
    fn aggregate_fact_identity_is_deterministic_and_debug_is_redaction_safe() {
        let required = obligation(
            "token-sk-aggregate-debug",
            LocalCheckGovernanceRequirementLevel::Required,
        );
        let set = authoritative_candidate(vec![required.clone()]);
        let coverage = evaluate_local_check_structural_coverage(
            &set,
            &[contribution(
                &set,
                &required,
                LocalCheckGovernanceContributionPosture::Satisfied,
            )],
        )
        .expect("coverage");
        let first =
            convert_authoritative_local_check_coverage(&coverage).expect("first conversion");
        let second =
            convert_authoritative_local_check_coverage(&coverage).expect("second conversion");

        assert_eq!(first.fact_fingerprint(), second.fact_fingerprint());
        assert_eq!(
            first.fact_fingerprint().as_str(),
            "f12fd20e99cfe4dc9ea4fdacb1b5526418961b0466f0ed20f53fb848316a7f04"
        );
        let debug = format!("{first:?}");
        assert!(debug.contains("Satisfied"));
        assert!(debug.contains("expected_count: 1"));
        assert!(!debug.contains("token-sk-aggregate-debug"));
        assert!(!debug.contains(first.candidate_set_fingerprint().as_str()));
        assert!(!debug.contains(first.structural_coverage_fingerprint().as_str()));
        assert!(!debug.contains(first.fact_fingerprint().as_str()));
    }

    #[test]
    fn aggregate_fact_identity_binds_every_decision_relevant_input() {
        let algorithm = AuthoritativeLocalCheckEvidenceCheckFactAlgorithm::V1;
        let baseline = LocalCheckGovernanceStructuralCoverageCandidate {
            source_posture: LocalCheckGovernanceDeclarationSourcePosture::CanonicalStoredBundle,
            disposition: LocalCheckGovernanceStructuralCoverageDisposition::Failed,
            expected_count: 4,
            satisfied_count: 1,
            failed_count: 1,
            required_unavailable_count: 1,
            optional_unavailable_count: 1,
            missing_count: 1,
            candidate_set_fingerprint: SpecContentHash::from_text("candidate-baseline"),
            structural_coverage_fingerprint: SpecContentHash::from_text("coverage-baseline"),
        };
        let baseline_fact =
            convert_authoritative_local_check_coverage(&baseline).expect("baseline fact");

        let mut posture_variant = baseline.clone();
        posture_variant.disposition =
            LocalCheckGovernanceStructuralCoverageDisposition::RequiredUnavailable;
        posture_variant.failed_count = 0;
        posture_variant.expected_count = 3;
        let posture_fact =
            convert_authoritative_local_check_coverage(&posture_variant).expect("posture fact");

        let mut count_variant = baseline.clone();
        count_variant.expected_count = 5;
        count_variant.satisfied_count = 2;
        let count_fact =
            convert_authoritative_local_check_coverage(&count_variant).expect("count fact");

        let mut candidate_variant = baseline.clone();
        candidate_variant.candidate_set_fingerprint =
            SpecContentHash::from_text("candidate-variant");
        let candidate_fact =
            convert_authoritative_local_check_coverage(&candidate_variant).expect("candidate fact");

        let mut coverage_variant = baseline.clone();
        coverage_variant.structural_coverage_fingerprint =
            SpecContentHash::from_text("coverage-variant");
        let coverage_fact =
            convert_authoritative_local_check_coverage(&coverage_variant).expect("coverage fact");

        for changed in [
            posture_fact.fact_fingerprint(),
            count_fact.fact_fingerprint(),
            candidate_fact.fact_fingerprint(),
            coverage_fact.fact_fingerprint(),
        ] {
            assert_ne!(baseline_fact.fact_fingerprint(), changed);
        }

        let baseline_hash = aggregate_fact_fingerprint(
            algorithm,
            GovernanceWorkloadEvidenceCheckPosture::Failed,
            &baseline,
        );
        let mut one_field_variants = Vec::new();
        let mut expected = baseline.clone();
        expected.expected_count += 1;
        one_field_variants.push(expected);
        let mut satisfied = baseline.clone();
        satisfied.satisfied_count += 1;
        one_field_variants.push(satisfied);
        let mut failed = baseline.clone();
        failed.failed_count += 1;
        one_field_variants.push(failed);
        let mut required_unavailable = baseline.clone();
        required_unavailable.required_unavailable_count += 1;
        one_field_variants.push(required_unavailable);
        let mut optional_unavailable = baseline.clone();
        optional_unavailable.optional_unavailable_count += 1;
        one_field_variants.push(optional_unavailable);
        let mut missing = baseline.clone();
        missing.missing_count += 1;
        one_field_variants.push(missing);
        let mut candidate = baseline.clone();
        candidate.candidate_set_fingerprint = SpecContentHash::from_text("candidate-field");
        one_field_variants.push(candidate);
        let mut coverage = baseline.clone();
        coverage.structural_coverage_fingerprint = SpecContentHash::from_text("coverage-field");
        one_field_variants.push(coverage);

        for variant in &one_field_variants {
            assert_ne!(
                baseline_hash,
                aggregate_fact_fingerprint(
                    algorithm,
                    GovernanceWorkloadEvidenceCheckPosture::Failed,
                    variant,
                )
            );
        }
        assert_ne!(
            baseline_hash,
            aggregate_fact_fingerprint(
                algorithm,
                GovernanceWorkloadEvidenceCheckPosture::RequiredUnavailable,
                &baseline,
            )
        );
    }

    #[test]
    fn aggregate_fingerprint_framing_separates_ambiguous_field_pairs() {
        let mut first = Sha256::new();
        hash_field(&mut first, "field", "a:b");
        hash_field(&mut first, "next", "c");
        let mut second = Sha256::new();
        hash_field(&mut second, "field", "a");
        hash_field(&mut second, "next", "b:c");

        let first_digest: [u8; 32] = first.finalize().into();
        let second_digest: [u8; 32] = second.finalize().into();
        assert_ne!(first_digest, second_digest);
    }

    #[test]
    fn debug_and_errors_do_not_expose_obligation_or_binding_values() {
        let secret = "token-sk-structural-coverage";
        let required = obligation(secret, LocalCheckGovernanceRequirementLevel::Required);
        let set = candidate(vec![required.clone()]);
        let result = evaluate_local_check_structural_coverage(
            &set,
            &[contribution(
                &set,
                &required,
                LocalCheckGovernanceContributionPosture::Satisfied,
            )],
        )
        .expect("coverage");
        let debug = format!("{set:?} {result:?}");
        assert!(!debug.contains(secret));
        assert!(!debug.contains(set.candidate_set_fingerprint().as_str()));
    }
}
