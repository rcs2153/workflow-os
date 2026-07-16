use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use sha2::{Digest, Sha256};

use crate::{
    validate_project_bundle, ApprovalSensitivity, AutonomyLevel, Capability,
    GovernanceDisclosureRequirement, GovernanceExecutionDisposition, GovernancePostureRequirement,
    GovernanceStrictnessProfile, GovernanceWorkloadActionClass, GovernanceWorkloadAuthorityPosture,
    GovernanceWorkloadEvidenceCheckPosture, GovernanceWorkloadSensitivity,
    GovernanceWorkloadSideEffectPosture, LoadedSpec, PolicyEffect, PolicyId, PolicySpecDocument,
    ProjectBundle, ProportionalGovernanceWorkloadAssessmentInput, SkillDefinition, SpecContentHash,
    StepDefinition, StepId, WorkflowDefinition, WorkflowId, WorkflowOsError,
};

const DERIVATION_ALGORITHM: &str =
    "workflow-os/proportional-governance-workflow-step-derivation/v1";

/// Explicit request for deriving one assessment input from existing declarations.
pub struct WorkflowStepGovernanceDerivationRequest<'a> {
    /// Already loaded project whose validation is rechecked before derivation.
    pub project: &'a ProjectBundle,
    /// Workflow containing the assessed step.
    pub workflow_id: &'a WorkflowId,
    /// Step whose resolved skill and policy declarations are assessed.
    pub step_id: &'a StepId,
    /// Active governance profile.
    pub profile: GovernanceStrictnessProfile,
    /// Authority known outside static declarations, or `None` when unresolved.
    pub authority: Option<GovernanceWorkloadAuthorityPosture>,
    /// Check/evidence result known outside static declarations, or `None` when unresolved.
    pub evidence_and_checks: Option<GovernanceWorkloadEvidenceCheckPosture>,
    /// Write reversibility known outside static declarations, or `None` when unresolved.
    pub side_effect: Option<GovernanceWorkloadSideEffectPosture>,
    /// Prior accepted execution posture, when one exists.
    pub prior_execution: Option<GovernanceExecutionDisposition>,
    /// Prior accepted disclosure posture, when one exists.
    pub prior_disclosure: Option<GovernanceDisclosureRequirement>,
    /// Explicit enterprise steward minimum, when required by the profile.
    pub steward_minimum: Option<GovernancePostureRequirement>,
}

impl fmt::Debug for WorkflowStepGovernanceDerivationRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowStepGovernanceDerivationRequest")
            .field("project", &"<redacted>")
            .field("workflow_id", &"<redacted>")
            .field("step_id", &"<redacted>")
            .field("profile", &self.profile)
            .field("authority", &self.authority)
            .field("evidence_and_checks", &self.evidence_and_checks)
            .field("side_effect", &self.side_effect)
            .field("prior_execution", &self.prior_execution)
            .field("prior_disclosure", &self.prior_disclosure)
            .field("steward_minimum", &self.steward_minimum)
            .finish()
    }
}

/// Derives one bounded assessment input from validated workflow declarations.
///
/// Static declarations determine action class, sensitivity, declared minima,
/// and the relevant-definition root. Authority, executed checks, and write
/// reversibility remain explicit unknowns unless supplied by the caller.
///
/// # Errors
///
/// Returns a stable bounded error when the project is invalid, workflow, step,
/// skill, or policy resolution is ambiguous, or a supplied `SideEffect` posture
/// contradicts the declared capability class.
pub fn derive_workflow_step_governance_assessment_input(
    request: &WorkflowStepGovernanceDerivationRequest<'_>,
) -> Result<ProportionalGovernanceWorkloadAssessmentInput, WorkflowOsError> {
    if validate_project_bundle(request.project).has_errors() {
        return Err(derivation_error(
            "governance.proportional.derivation.project_invalid",
            "workflow governance derivation requires a valid project",
        ));
    }

    let workflow = resolve_workflow(request.project, request.workflow_id)?;
    let step = resolve_step(&workflow.definition, request.step_id)?;
    let skill = resolve_skill(&request.project.skills, step)?;
    let policies = resolve_step_policies(request.project, &workflow.definition, step)?;
    let resolved_policies = policies
        .iter()
        .map(|policy| (&policy.definition, &policy.content_hash))
        .collect::<Vec<_>>();
    derive_resolved_workflow_step_governance_assessment_input(
        &ResolvedWorkflowStepGovernanceDerivationRequest {
            workflow: &workflow.definition,
            workflow_hash: &workflow.content_hash,
            step,
            skill: &skill.definition,
            skill_hash: &skill.content_hash,
            policies: &resolved_policies,
            profile: request.profile,
            authority: request.authority,
            evidence_and_checks: request.evidence_and_checks,
            side_effect: request.side_effect,
            runtime_escalation: None,
            prior_execution: request.prior_execution,
            prior_disclosure: request.prior_disclosure,
            steward_minimum: request.steward_minimum,
        },
    )
}

pub(crate) struct ResolvedWorkflowStepGovernanceDerivationRequest<'a> {
    pub(crate) workflow: &'a WorkflowDefinition,
    pub(crate) workflow_hash: &'a SpecContentHash,
    pub(crate) step: &'a StepDefinition,
    pub(crate) skill: &'a SkillDefinition,
    pub(crate) skill_hash: &'a SpecContentHash,
    pub(crate) policies: &'a [(&'a PolicySpecDocument, &'a SpecContentHash)],
    pub(crate) profile: GovernanceStrictnessProfile,
    pub(crate) authority: Option<GovernanceWorkloadAuthorityPosture>,
    pub(crate) evidence_and_checks: Option<GovernanceWorkloadEvidenceCheckPosture>,
    pub(crate) side_effect: Option<GovernanceWorkloadSideEffectPosture>,
    pub(crate) runtime_escalation: Option<GovernancePostureRequirement>,
    pub(crate) prior_execution: Option<GovernanceExecutionDisposition>,
    pub(crate) prior_disclosure: Option<GovernanceDisclosureRequirement>,
    pub(crate) steward_minimum: Option<GovernancePostureRequirement>,
}

pub(crate) fn derive_resolved_workflow_step_governance_assessment_input(
    request: &ResolvedWorkflowStepGovernanceDerivationRequest<'_>,
) -> Result<ProportionalGovernanceWorkloadAssessmentInput, WorkflowOsError> {
    let capabilities = declared_capabilities(request.skill);
    let action_class = derive_action_class(&capabilities, request.skill);
    let inferred_side_effect = derive_side_effect(action_class, request.side_effect)?;

    Ok(ProportionalGovernanceWorkloadAssessmentInput {
        definition_root: resolved_definition_root(request),
        profile: request.profile,
        workflow_minimum: workflow_minimum(request.workflow),
        policy_minimum: resolved_policy_minimum(request.step, request.policies),
        action_class,
        authority: request
            .authority
            .unwrap_or(GovernanceWorkloadAuthorityPosture::Unknown),
        authority_minimum: GovernancePostureRequirement::quiet(),
        evidence_and_checks: request
            .evidence_and_checks
            .unwrap_or(GovernanceWorkloadEvidenceCheckPosture::Unknown),
        evidence_and_check_minimum: GovernancePostureRequirement::quiet(),
        sensitivity: derive_sensitivity(request.skill, &capabilities),
        sensitivity_minimum: GovernancePostureRequirement::quiet(),
        side_effect: inferred_side_effect,
        side_effect_minimum: GovernancePostureRequirement::quiet(),
        runtime_escalation: strictest_requirement(
            runtime_escalation(request.workflow, request.step),
            request
                .runtime_escalation
                .unwrap_or_else(GovernancePostureRequirement::quiet),
        ),
        prior_execution: request.prior_execution,
        prior_disclosure: request.prior_disclosure,
        steward_minimum: request.steward_minimum,
    })
}

fn resolve_workflow<'a>(
    project: &'a ProjectBundle,
    workflow_id: &WorkflowId,
) -> Result<&'a LoadedSpec<WorkflowDefinition>, WorkflowOsError> {
    project
        .workflows
        .iter()
        .find(|workflow| &workflow.definition.id == workflow_id)
        .ok_or_else(|| {
            derivation_error(
                "governance.proportional.derivation.workflow_unresolved",
                "workflow governance derivation could not resolve the workflow",
            )
        })
}

fn resolve_step<'a>(
    workflow: &'a WorkflowDefinition,
    step_id: &StepId,
) -> Result<&'a StepDefinition, WorkflowOsError> {
    workflow
        .steps
        .iter()
        .find(|step| &step.id == step_id)
        .ok_or_else(|| {
            derivation_error(
                "governance.proportional.derivation.step_unresolved",
                "workflow governance derivation could not resolve the step",
            )
        })
}

fn resolve_skill<'a>(
    skills: &'a [LoadedSpec<SkillDefinition>],
    step: &StepDefinition,
) -> Result<&'a LoadedSpec<SkillDefinition>, WorkflowOsError> {
    let matches = skills
        .iter()
        .filter(|skill| skill.definition.id == step.skill_ref.id)
        .filter(|skill| {
            step.skill_ref
                .version
                .as_ref()
                .map_or(true, |version| &skill.definition.version == version)
        })
        .collect::<Vec<_>>();
    let [skill] = matches.as_slice() else {
        return Err(derivation_error(
            "governance.proportional.derivation.skill_unresolved",
            "workflow governance derivation could not resolve one skill",
        ));
    };
    Ok(*skill)
}

fn resolve_step_policies<'a>(
    project: &'a ProjectBundle,
    workflow: &WorkflowDefinition,
    step: &StepDefinition,
) -> Result<Vec<&'a LoadedSpec<PolicySpecDocument>>, WorkflowOsError> {
    let mut ids = BTreeSet::<PolicyId>::new();
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

    ids.into_iter()
        .map(|id| {
            project
                .policies
                .iter()
                .find(|policy| policy.definition.id == id)
                .ok_or_else(|| {
                    derivation_error(
                        "governance.proportional.derivation.policy_unresolved",
                        "workflow governance derivation could not resolve a policy",
                    )
                })
        })
        .collect()
}

fn declared_capabilities(skill: &SkillDefinition) -> Vec<Capability> {
    if skill.allowed_capabilities.is_empty() && skill.adapter_requirements.is_empty() {
        return vec![Capability::LocalRead, Capability::LocalWrite];
    }
    skill
        .allowed_capabilities
        .iter()
        .map(|capability| Capability::from_declared_name(&capability.name))
        .collect()
}

fn derive_action_class(
    capabilities: &[Capability],
    skill: &SkillDefinition,
) -> GovernanceWorkloadActionClass {
    if capabilities
        .iter()
        .any(|capability| matches!(capability, Capability::Unknown(_)))
    {
        return GovernanceWorkloadActionClass::Unsupported;
    }
    if capabilities.contains(&Capability::ExternalWrite) {
        return GovernanceWorkloadActionClass::ExternalMutation;
    }
    if capabilities.contains(&Capability::LocalWrite) {
        return GovernanceWorkloadActionClass::LocalMutation;
    }
    if capabilities.contains(&Capability::AdapterInvoke)
        && !capabilities.contains(&Capability::ExternalRead)
        && !skill.adapter_requirements.is_empty()
    {
        return GovernanceWorkloadActionClass::Unknown;
    }
    GovernanceWorkloadActionClass::ReadOnly
}

fn derive_side_effect(
    action_class: GovernanceWorkloadActionClass,
    supplied: Option<GovernanceWorkloadSideEffectPosture>,
) -> Result<GovernanceWorkloadSideEffectPosture, WorkflowOsError> {
    let default = match action_class {
        GovernanceWorkloadActionClass::ReadOnly => GovernanceWorkloadSideEffectPosture::None,
        GovernanceWorkloadActionClass::LocalMutation
        | GovernanceWorkloadActionClass::ExternalMutation
        | GovernanceWorkloadActionClass::Unknown => GovernanceWorkloadSideEffectPosture::Unknown,
        GovernanceWorkloadActionClass::Unsupported => {
            GovernanceWorkloadSideEffectPosture::Unsupported
        }
    };
    let selected = supplied.unwrap_or(default);
    let valid = match action_class {
        GovernanceWorkloadActionClass::ReadOnly => {
            selected == GovernanceWorkloadSideEffectPosture::None
        }
        GovernanceWorkloadActionClass::LocalMutation => matches!(
            selected,
            GovernanceWorkloadSideEffectPosture::LocalReversible
                | GovernanceWorkloadSideEffectPosture::Ambiguous
                | GovernanceWorkloadSideEffectPosture::Unsupported
                | GovernanceWorkloadSideEffectPosture::Unknown
        ),
        GovernanceWorkloadActionClass::ExternalMutation => matches!(
            selected,
            GovernanceWorkloadSideEffectPosture::ExternalReversible
                | GovernanceWorkloadSideEffectPosture::ExternalIrreversible
                | GovernanceWorkloadSideEffectPosture::Ambiguous
                | GovernanceWorkloadSideEffectPosture::Unsupported
                | GovernanceWorkloadSideEffectPosture::Unknown
        ),
        GovernanceWorkloadActionClass::Unknown => matches!(
            selected,
            GovernanceWorkloadSideEffectPosture::Ambiguous
                | GovernanceWorkloadSideEffectPosture::Unsupported
                | GovernanceWorkloadSideEffectPosture::Unknown
        ),
        GovernanceWorkloadActionClass::Unsupported => {
            selected == GovernanceWorkloadSideEffectPosture::Unsupported
        }
    };
    if !valid {
        return Err(derivation_error(
            "governance.proportional.derivation.side_effect_mismatch",
            "supplied SideEffect posture contradicts declared capabilities",
        ));
    }
    Ok(selected)
}

fn derive_sensitivity(
    skill: &SkillDefinition,
    capabilities: &[Capability],
) -> GovernanceWorkloadSensitivity {
    if capabilities.contains(&Capability::SecretRead)
        || skill.approval_sensitivity == ApprovalSensitivity::High
    {
        GovernanceWorkloadSensitivity::Restricted
    } else if skill.approval_sensitivity == ApprovalSensitivity::Medium {
        GovernanceWorkloadSensitivity::Elevated
    } else {
        GovernanceWorkloadSensitivity::Routine
    }
}

fn workflow_minimum(workflow: &WorkflowDefinition) -> GovernancePostureRequirement {
    if workflow.disabled_by_default
        || matches!(
            workflow.autonomy_level,
            AutonomyLevel::Level3ConditionalAutonomy | AutonomyLevel::Level4ScaledAutomation
        )
    {
        GovernancePostureRequirement::denied()
    } else if !workflow.approval_requirements.is_empty()
        || workflow.autonomy_level == AutonomyLevel::Level2GuidedWithApproval
    {
        GovernancePostureRequirement::approval()
    } else {
        GovernancePostureRequirement::quiet()
    }
}

fn resolved_policy_minimum(
    step: &StepDefinition,
    policies: &[(&PolicySpecDocument, &SpecContentHash)],
) -> GovernancePostureRequirement {
    let requires_approval = step.approval_policy.is_some()
        || policies.iter().any(|(policy, _)| {
            policy
                .rules
                .iter()
                .filter_map(|rule| PolicyEffect::parse(&rule.effect).ok())
                .any(|effect| effect == PolicyEffect::RequireApproval)
        });
    if requires_approval {
        GovernancePostureRequirement::approval()
    } else {
        GovernancePostureRequirement::quiet()
    }
}

fn runtime_escalation(
    workflow: &WorkflowDefinition,
    step: &StepDefinition,
) -> GovernancePostureRequirement {
    if step.escalation_policy.is_some() || !workflow.escalation_policy_refs.is_empty() {
        GovernancePostureRequirement::visible()
    } else {
        GovernancePostureRequirement::quiet()
    }
}

fn strictest_requirement(
    left: GovernancePostureRequirement,
    right: GovernancePostureRequirement,
) -> GovernancePostureRequirement {
    GovernancePostureRequirement::new(
        left.execution().max(right.execution()),
        left.disclosure().max(right.disclosure()),
    )
}

fn resolved_definition_root(
    request: &ResolvedWorkflowStepGovernanceDerivationRequest<'_>,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "algorithm", DERIVATION_ALGORITHM);
    hash_field(&mut hasher, "workflow_hash", request.workflow_hash.as_str());
    hash_field(&mut hasher, "step_id", request.step.id.as_str());
    hash_field(&mut hasher, "skill_hash", request.skill_hash.as_str());
    let ordered = request
        .policies
        .iter()
        .map(|(policy, hash)| (policy.id.as_str(), hash.as_str()))
        .collect::<BTreeMap<_, _>>();
    for (id, hash) in ordered {
        hash_field(&mut hasher, "policy_id", id);
        hash_field(&mut hasher, "policy_hash", hash);
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

fn derivation_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}
