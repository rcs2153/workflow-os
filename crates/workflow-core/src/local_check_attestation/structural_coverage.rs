use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use sha2::{Digest, Sha256};

use super::runtime::{
    docs_check_governance_obligation_fingerprint, DocsCheckGovernanceEvidenceCheckContribution,
    GovernanceEvidenceCheckContributionPosture,
};
use crate::{
    ImmutableRunBundleId, ImmutableRunBundleVersion, SpecContentHash, StepId, WorkflowId,
    WorkflowOsError, WorkflowRunId, WorkflowVersion,
};

const CANDIDATE_ALGORITHM: &str =
    "workflow-os/local-check-governance-structural-coverage-candidate/v1";

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

        let candidate_set_fingerprint = candidate_set_fingerprint(&definition, &obligations);
        Ok(Self {
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

    pub(crate) const fn candidate_set_fingerprint(&self) -> &SpecContentHash {
        &self.candidate_set_fingerprint
    }
}

impl fmt::Debug for LocalCheckGovernanceObligationSetCandidate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCheckGovernanceObligationSetCandidate")
            .field("source_posture", &"unresolved")
            .field("obligation_count", &self.obligations.len())
            .field("binding", &"[REDACTED]")
            .field("candidate_set_fingerprint", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct LocalCheckGovernanceStructuralCoverageCandidate {
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
            .field("source_posture", &"unresolved")
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
    obligations: &[LocalCheckGovernanceObligation],
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "algorithm", CANDIDATE_ALGORITHM);
    hash_field(&mut hasher, "source_posture", "unresolved");
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

fn coverage_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("local_check_attestation.structural_coverage.{suffix}"),
        message,
    )
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    fn obligation(
        label: &str,
        level: LocalCheckGovernanceRequirementLevel,
    ) -> LocalCheckGovernanceObligationDefinition {
        LocalCheckGovernanceObligationDefinition::new(SpecContentHash::from_text(label), level)
    }

    fn candidate(
        obligations: Vec<LocalCheckGovernanceObligationDefinition>,
    ) -> LocalCheckGovernanceObligationSetCandidate {
        LocalCheckGovernanceObligationSetCandidate::new(
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
        assert!(format!("{result:?}").contains("unresolved"));
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
        assert!(format!("{result:?}").contains("source_posture: \"unresolved\""));
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
