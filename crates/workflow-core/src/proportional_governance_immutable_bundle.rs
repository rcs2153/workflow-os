use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::proportional_governance_workflow_derivation::{
    derive_resolved_workflow_step_governance_assessment_input,
    ResolvedWorkflowStepGovernanceDerivationRequest,
};
use crate::{
    assess_proportional_governance_workload, GovernanceAssessmentSetAlgorithm,
    GovernanceDisclosureRequirement, GovernanceExecutionDisposition, GovernancePostureRequirement,
    GovernanceStrictnessProfile, GovernanceWorkloadAuthorityPosture,
    GovernanceWorkloadEvidenceCheckPosture, GovernanceWorkloadSideEffectPosture,
    ImmutableRunBundleBinding, ImmutableRunBundleDefinitionKind,
    ImmutableRunBundleDefinitionRecord, PolicyId, ProportionalGovernanceWorkloadAssessment,
    SpecContentHash, StepDefinition, StepId, StoredImmutableRunBundle, WorkflowDefinition,
    WorkflowId, WorkflowOsError, WorkflowRunId,
};

/// Exact explicit runtime facts for one immutable workflow step.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct StepGovernanceRuntimeFacts {
    step_id: StepId,
    authority: Option<GovernanceWorkloadAuthorityPosture>,
    evidence_and_checks: Option<GovernanceWorkloadEvidenceCheckPosture>,
    side_effect: Option<GovernanceWorkloadSideEffectPosture>,
    runtime_escalation: Option<GovernancePostureRequirement>,
    prior_execution: Option<GovernanceExecutionDisposition>,
    prior_disclosure: Option<GovernanceDisclosureRequirement>,
    steward_minimum: Option<GovernancePostureRequirement>,
}

impl StepGovernanceRuntimeFacts {
    /// Creates one bounded runtime-fact record for an exact step.
    #[must_use]
    pub const fn new(
        step_id: StepId,
        authority: Option<GovernanceWorkloadAuthorityPosture>,
        evidence_and_checks: Option<GovernanceWorkloadEvidenceCheckPosture>,
        side_effect: Option<GovernanceWorkloadSideEffectPosture>,
        prior_execution: Option<GovernanceExecutionDisposition>,
        prior_disclosure: Option<GovernanceDisclosureRequirement>,
        steward_minimum: Option<GovernancePostureRequirement>,
    ) -> Self {
        Self {
            step_id,
            authority,
            evidence_and_checks,
            side_effect,
            runtime_escalation: None,
            prior_execution,
            prior_disclosure,
            steward_minimum,
        }
    }

    /// Returns the exact step identity.
    #[must_use]
    pub const fn step_id(&self) -> &StepId {
        &self.step_id
    }

    /// Adds one validated runtime escalation that may only hold or raise posture.
    #[must_use]
    pub const fn with_runtime_escalation(
        mut self,
        runtime_escalation: GovernancePostureRequirement,
    ) -> Self {
        self.runtime_escalation = Some(runtime_escalation);
        self
    }
}

impl fmt::Debug for StepGovernanceRuntimeFacts {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StepGovernanceRuntimeFacts")
            .field("step_id", &"<redacted>")
            .field("authority", &self.authority)
            .field("evidence_and_checks", &self.evidence_and_checks)
            .field("side_effect", &self.side_effect)
            .field("runtime_escalation", &self.runtime_escalation)
            .field("prior_execution", &self.prior_execution)
            .field("prior_disclosure", &self.prior_disclosure)
            .field("steward_minimum", &self.steward_minimum)
            .finish()
    }
}

/// Pure reassessment request over one validated stored immutable run bundle.
pub struct ImmutableBundleGovernanceAssessmentRequest<'a> {
    /// Validated bundle returned by the immutable run-bundle store boundary.
    pub bundle: &'a StoredImmutableRunBundle,
    /// Active proportional-governance profile.
    pub profile: GovernanceStrictnessProfile,
    /// Exactly one runtime-fact record for every ordered workflow step.
    pub runtime_facts: &'a [StepGovernanceRuntimeFacts],
}

impl fmt::Debug for ImmutableBundleGovernanceAssessmentRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmutableBundleGovernanceAssessmentRequest")
            .field("bundle", &"<redacted>")
            .field("profile", &self.profile)
            .field("runtime_fact_count", &self.runtime_facts.len())
            .finish()
    }
}

/// Review-only proportional-governance assessment for one immutable step.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ImmutableBundleStepGovernanceAssessment {
    step_id: StepId,
    assessment: ProportionalGovernanceWorkloadAssessment,
}

impl ImmutableBundleStepGovernanceAssessment {
    /// Returns the immutable workflow step identity.
    #[must_use]
    pub const fn step_id(&self) -> &StepId {
        &self.step_id
    }

    /// Returns the accepted review-only workload assessment.
    #[must_use]
    pub const fn assessment(&self) -> &ProportionalGovernanceWorkloadAssessment {
        &self.assessment
    }
}

impl fmt::Debug for ImmutableBundleStepGovernanceAssessment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmutableBundleStepGovernanceAssessment")
            .field("step_id", &"<redacted>")
            .field("assessment", &self.assessment)
            .finish()
    }
}

/// Deterministic ordered assessment set derived without execution or persistence.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ImmutableBundleGovernanceAssessmentSet {
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    immutable_run_bundle: ImmutableRunBundleBinding,
    assessments: Vec<ImmutableBundleStepGovernanceAssessment>,
    aggregate_fingerprint: SpecContentHash,
}

impl ImmutableBundleGovernanceAssessmentSet {
    /// Returns the versioned assessment-set algorithm.
    #[must_use]
    pub const fn algorithm(&self) -> GovernanceAssessmentSetAlgorithm {
        GovernanceAssessmentSetAlgorithm::V1
    }

    /// Returns the immutable workflow identity.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the immutable run identity.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the exact immutable bundle used to derive this assessment set.
    #[must_use]
    pub const fn immutable_run_bundle(&self) -> &ImmutableRunBundleBinding {
        &self.immutable_run_bundle
    }

    /// Returns assessments in workflow step order.
    #[must_use]
    pub fn assessments(&self) -> &[ImmutableBundleStepGovernanceAssessment] {
        &self.assessments
    }

    /// Returns the versioned aggregate assessment-set fingerprint.
    #[must_use]
    pub const fn aggregate_fingerprint(&self) -> &SpecContentHash {
        &self.aggregate_fingerprint
    }
}

impl fmt::Debug for ImmutableBundleGovernanceAssessmentSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmutableBundleGovernanceAssessmentSet")
            .field("workflow_id", &"<redacted>")
            .field("run_id", &"<redacted>")
            .field("immutable_run_bundle", &"<redacted>")
            .field("assessment_count", &self.assessments.len())
            .field("aggregate_fingerprint", &"<redacted>")
            .finish()
    }
}

/// Reassesses every immutable workflow step from canonical definitions and exact facts.
///
/// This helper is pure and review-only. It does not prove fact freshness, bind
/// results durably, enforce a decision, append events, or invoke a handler.
///
/// # Errors
///
/// Returns stable bounded validation errors when canonical definitions or exact
/// runtime facts cannot be resolved unambiguously, or when assessment fails.
pub fn assess_immutable_bundle_governance(
    request: &ImmutableBundleGovernanceAssessmentRequest<'_>,
) -> Result<ImmutableBundleGovernanceAssessmentSet, WorkflowOsError> {
    let manifest = request.bundle.manifest();
    let records = request.bundle.definition_records();
    let workflow_record = resolve_workflow_record(records, manifest.workflow_id())?;
    let workflow = workflow_record
        .canonical_definition()
        .as_workflow()
        .ok_or_else(|| assessment_error("workflow_unresolved"))?;
    if workflow.version != *manifest.workflow_version()
        || workflow_record.source_content_hash() != manifest.workflow_content_hash()
    {
        return Err(assessment_error("workflow_mismatch"));
    }

    let facts = exact_runtime_facts(workflow, request.runtime_facts)?;
    let mut assessments = Vec::with_capacity(workflow.steps.len());
    for step in &workflow.steps {
        let fact = facts
            .get(&step.id)
            .copied()
            .ok_or_else(|| assessment_error("runtime_facts_missing"))?;
        let skill_record = resolve_step_skill_record(request.bundle, step)?;
        let skill = skill_record
            .canonical_definition()
            .as_skill()
            .ok_or_else(|| assessment_error("skill_unresolved"))?;
        if skill.id != step.skill_ref.id
            || step
                .skill_ref
                .version
                .as_ref()
                .is_some_and(|version| version != &skill.version)
        {
            return Err(assessment_error("skill_mismatch"));
        }
        let policies = resolve_step_policy_records(request.bundle, workflow, step)?;
        let resolved_policies = policies
            .iter()
            .map(|record| {
                record
                    .canonical_definition()
                    .as_policy()
                    .map(|policy| (policy, record.source_content_hash()))
                    .ok_or_else(|| assessment_error("policy_unresolved"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let input = derive_resolved_workflow_step_governance_assessment_input(
            &ResolvedWorkflowStepGovernanceDerivationRequest {
                workflow,
                workflow_hash: workflow_record.source_content_hash(),
                step,
                skill,
                skill_hash: skill_record.source_content_hash(),
                policies: &resolved_policies,
                profile: request.profile,
                authority: fact.authority,
                evidence_and_checks: fact.evidence_and_checks,
                side_effect: fact.side_effect,
                runtime_escalation: fact.runtime_escalation,
                prior_execution: fact.prior_execution,
                prior_disclosure: fact.prior_disclosure,
                steward_minimum: fact.steward_minimum,
            },
        )?;
        assessments.push(ImmutableBundleStepGovernanceAssessment {
            step_id: step.id.clone(),
            assessment: assess_proportional_governance_workload(&input)?,
        });
    }

    let aggregate_fingerprint = aggregate_fingerprint(request.bundle, &assessments);
    Ok(ImmutableBundleGovernanceAssessmentSet {
        workflow_id: manifest.workflow_id().clone(),
        run_id: manifest.run_id().clone(),
        immutable_run_bundle: manifest.run_binding(),
        assessments,
        aggregate_fingerprint,
    })
}

fn exact_runtime_facts<'a>(
    workflow: &WorkflowDefinition,
    supplied: &'a [StepGovernanceRuntimeFacts],
) -> Result<BTreeMap<StepId, &'a StepGovernanceRuntimeFacts>, WorkflowOsError> {
    if supplied.len() != workflow.steps.len() {
        return Err(assessment_error("runtime_facts_count_mismatch"));
    }
    let expected = workflow
        .steps
        .iter()
        .map(|step| step.id.clone())
        .collect::<BTreeSet<_>>();
    let mut facts = BTreeMap::new();
    for fact in supplied {
        if !expected.contains(&fact.step_id) {
            return Err(assessment_error("runtime_facts_step_mismatch"));
        }
        if facts.insert(fact.step_id.clone(), fact).is_some() {
            return Err(assessment_error("runtime_facts_duplicate"));
        }
    }
    if facts.len() != expected.len() {
        return Err(assessment_error("runtime_facts_missing"));
    }
    Ok(facts)
}

fn resolve_workflow_record<'a>(
    records: &'a [ImmutableRunBundleDefinitionRecord],
    workflow_id: &WorkflowId,
) -> Result<&'a ImmutableRunBundleDefinitionRecord, WorkflowOsError> {
    exactly_one(
        records
            .iter()
            .filter(|record| record.kind() == ImmutableRunBundleDefinitionKind::Workflow)
            .filter(|record| record.definition_id() == workflow_id.as_str()),
        "workflow_unresolved",
    )
}

fn resolve_step_skill_record<'a>(
    bundle: &'a StoredImmutableRunBundle,
    step: &StepDefinition,
) -> Result<&'a ImmutableRunBundleDefinitionRecord, WorkflowOsError> {
    let reference = exactly_one(
        bundle
            .manifest()
            .definitions()
            .iter()
            .filter(|reference| reference.kind() == ImmutableRunBundleDefinitionKind::Skill)
            .filter(|reference| reference.step_id() == Some(&step.id)),
        "skill_reference_unresolved",
    )?;
    exactly_one(
        bundle
            .definition_records()
            .iter()
            .filter(|record| record.kind() == ImmutableRunBundleDefinitionKind::Skill)
            .filter(|record| record.definition_id() == reference.definition_id())
            .filter(|record| record.source_content_hash() == reference.content_hash()),
        "skill_unresolved",
    )
}

fn resolve_step_policy_records<'a>(
    bundle: &'a StoredImmutableRunBundle,
    workflow: &WorkflowDefinition,
    step: &StepDefinition,
) -> Result<Vec<&'a ImmutableRunBundleDefinitionRecord>, WorkflowOsError> {
    let ids = referenced_policy_ids(workflow, step);
    ids.into_iter()
        .map(|id| {
            let reference = exactly_one(
                bundle
                    .manifest()
                    .definitions()
                    .iter()
                    .filter(|reference| {
                        reference.kind() == ImmutableRunBundleDefinitionKind::Policy
                    })
                    .filter(|reference| reference.definition_id() == id.as_str()),
                "policy_reference_unresolved",
            )?;
            exactly_one(
                bundle
                    .definition_records()
                    .iter()
                    .filter(|record| record.kind() == ImmutableRunBundleDefinitionKind::Policy)
                    .filter(|record| record.definition_id() == reference.definition_id())
                    .filter(|record| record.source_content_hash() == reference.content_hash()),
                "policy_unresolved",
            )
        })
        .collect()
}

fn referenced_policy_ids(
    workflow: &WorkflowDefinition,
    step: &StepDefinition,
) -> BTreeSet<PolicyId> {
    let mut ids = BTreeSet::new();
    ids.extend(
        workflow
            .retry_policy_refs
            .iter()
            .map(|reference| reference.id.clone()),
    );
    ids.extend(
        workflow
            .escalation_policy_refs
            .iter()
            .map(|reference| reference.id.clone()),
    );
    ids.extend(
        step.policy_requirements
            .iter()
            .map(|reference| reference.id.clone()),
    );
    ids.extend(
        step.approval_policy
            .iter()
            .map(|reference| reference.policy.id.clone()),
    );
    ids.extend(
        step.retry_policy
            .iter()
            .map(|reference| reference.policy.id.clone()),
    );
    ids.extend(
        step.escalation_policy
            .iter()
            .map(|reference| reference.policy.id.clone()),
    );
    ids
}

fn exactly_one<T>(
    values: impl Iterator<Item = T>,
    error_suffix: &'static str,
) -> Result<T, WorkflowOsError> {
    let mut values = values.take(2);
    let value = values
        .next()
        .ok_or_else(|| assessment_error(error_suffix))?;
    if values.next().is_some() {
        return Err(assessment_error(error_suffix));
    }
    Ok(value)
}

fn aggregate_fingerprint(
    bundle: &StoredImmutableRunBundle,
    assessments: &[ImmutableBundleStepGovernanceAssessment],
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(
        &mut hasher,
        "domain",
        GovernanceAssessmentSetAlgorithm::V1.identifier(),
    );
    hash_field(
        &mut hasher,
        "bundle_root",
        bundle.manifest().root_hash().as_str(),
    );
    hash_field(
        &mut hasher,
        "workflow_id",
        bundle.manifest().workflow_id().as_str(),
    );
    hash_field(&mut hasher, "run_id", bundle.manifest().run_id().as_str());
    for item in assessments {
        hash_field(&mut hasher, "step_id", item.step_id.as_str());
        hash_field(
            &mut hasher,
            "assessment_algorithm",
            item.assessment.algorithm().identifier(),
        );
        hash_field(
            &mut hasher,
            "assessment_fingerprint",
            item.assessment.input_fingerprint().as_str(),
        );
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &str) {
    for part in [label.as_bytes(), value.as_bytes()] {
        let length = u64::try_from(part.len()).unwrap_or(u64::MAX);
        hasher.update(length.to_be_bytes());
        hasher.update(part);
    }
}

fn assessment_error(suffix: &'static str) -> WorkflowOsError {
    let code = match suffix {
        "workflow_unresolved" => "governance.proportional.immutable_bundle.workflow_unresolved",
        "workflow_mismatch" => "governance.proportional.immutable_bundle.workflow_mismatch",
        "skill_reference_unresolved" => {
            "governance.proportional.immutable_bundle.skill_reference_unresolved"
        }
        "skill_unresolved" => "governance.proportional.immutable_bundle.skill_unresolved",
        "skill_mismatch" => "governance.proportional.immutable_bundle.skill_mismatch",
        "policy_reference_unresolved" => {
            "governance.proportional.immutable_bundle.policy_reference_unresolved"
        }
        "policy_unresolved" => "governance.proportional.immutable_bundle.policy_unresolved",
        "runtime_facts_count_mismatch" => {
            "governance.proportional.immutable_bundle.runtime_facts_count_mismatch"
        }
        "runtime_facts_step_mismatch" => {
            "governance.proportional.immutable_bundle.runtime_facts_step_mismatch"
        }
        "runtime_facts_duplicate" => {
            "governance.proportional.immutable_bundle.runtime_facts_duplicate"
        }
        "runtime_facts_missing" => "governance.proportional.immutable_bundle.runtime_facts_missing",
        _ => "governance.proportional.immutable_bundle.invalid",
    };
    WorkflowOsError::validation(code, "immutable bundle governance assessment failed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_width_framing_distinguishes_delimiter_ambiguous_values() {
        let mut left = Sha256::new();
        hash_field(&mut left, "a", "bc");
        let mut right = Sha256::new();
        hash_field(&mut right, "ab", "c");

        assert_ne!(left.finalize(), right.finalize());
    }
}
