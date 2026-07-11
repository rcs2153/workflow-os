use std::collections::BTreeSet;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{GovernanceStrictnessProfile, WorkflowOsError, WorkflowOsErrorKind};

/// Ordered interaction posture selected by proportional governance.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceInteractionMode {
    /// Record the governed result without interrupting eligible work.
    QuietCapture,
    /// Proceed while presenting a bounded, non-blocking disclosure.
    VisibleDisclosure,
    /// Pause before the governed action for explicit approval.
    BlockingApproval,
    /// Fail closed because the governed action is prohibited or unsafe.
    Denied,
}

/// Ordered risk classification corresponding to an interaction mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceRiskClass {
    /// Read-only or otherwise bounded work with complete required posture.
    BoundedObservation,
    /// Permitted work that warrants operator awareness.
    ReviewWorthy,
    /// Work that must pause for approval.
    ApprovalRequired,
    /// Work that must fail closed.
    Denied,
}

/// Typed requirement contributed by one validated governance concern.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernancePostureRequirement {
    /// The concern adds no interruption beyond quiet capture.
    QuietCapture,
    /// The concern requires visible disclosure.
    VisibleDisclosure,
    /// The concern requires blocking approval.
    BlockingApproval,
    /// The concern denies the action.
    Denied,
    /// The concern declares behavior this model cannot enforce.
    Unsupported,
}

/// Stable, payload-free reason for a proportional-governance decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecisionReason {
    /// The active governance profile established the minimum posture.
    ProfileMinimum,
    /// The workflow declaration required this posture.
    WorkflowRequirement,
    /// A policy decision required this posture.
    PolicyRequirement,
    /// Actor or delegated authority required this posture.
    AuthorityRequirement,
    /// Evidence or check availability required this posture.
    EvidenceOrCheckRequirement,
    /// Sensitivity classification required this posture.
    SensitivityRequirement,
    /// Side-effect posture required this posture.
    SideEffectRequirement,
    /// A validated runtime change required escalation.
    RuntimeEscalation,
    /// A prior decision prevented a posture downgrade.
    PriorDecisionMinimum,
    /// A steward-defined minimum established the posture.
    StewardMinimum,
}

/// Validated, bounded inputs to the pure proportional-governance selector.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProportionalGovernanceDecisionInput {
    /// Active governance profile.
    pub profile: GovernanceStrictnessProfile,
    /// Workflow-declared minimum posture.
    pub workflow: GovernancePostureRequirement,
    /// Policy-derived minimum posture.
    pub policy: GovernancePostureRequirement,
    /// Actor or delegated-authority minimum posture.
    pub authority: GovernancePostureRequirement,
    /// Combined required evidence and check posture.
    pub evidence_and_checks: GovernancePostureRequirement,
    /// Sensitivity-derived minimum posture.
    pub sensitivity: GovernancePostureRequirement,
    /// Side-effect-derived minimum posture.
    pub side_effect: GovernancePostureRequirement,
    /// Minimum posture derived from validated runtime changes.
    pub runtime_escalation: GovernancePostureRequirement,
    /// Prior selected mode, when reassessing an active governed action.
    pub prior_mode: Option<GovernanceInteractionMode>,
    /// Explicit steward minimum, required for strict-enterprise evaluation.
    pub steward_minimum: Option<GovernancePostureRequirement>,
}

/// Deterministic result of proportional-governance selection.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProportionalGovernanceDecision {
    mode: GovernanceInteractionMode,
    risk_class: GovernanceRiskClass,
    reasons: BTreeSet<GovernanceDecisionReason>,
}

impl ProportionalGovernanceDecision {
    /// Returns the selected interaction mode.
    #[must_use]
    pub const fn mode(&self) -> GovernanceInteractionMode {
        self.mode
    }

    /// Returns the corresponding ordered risk class.
    #[must_use]
    pub const fn risk_class(&self) -> GovernanceRiskClass {
        self.risk_class
    }

    /// Returns stable reasons that contributed above quiet capture.
    #[must_use]
    pub const fn reasons(&self) -> &BTreeSet<GovernanceDecisionReason> {
        &self.reasons
    }
}

/// Selects the strictest applicable posture without runtime side effects.
///
/// # Errors
///
/// Returns a stable, non-leaking unsupported error when any declared
/// requirement cannot be enforced by this model.
pub fn select_proportional_governance(
    input: ProportionalGovernanceDecisionInput,
) -> Result<ProportionalGovernanceDecision, WorkflowOsError> {
    let mut selected = profile_minimum(input.profile, input.steward_minimum)?;
    let mut reasons = BTreeSet::from([GovernanceDecisionReason::ProfileMinimum]);

    let requirements = [
        (
            input.workflow,
            GovernanceDecisionReason::WorkflowRequirement,
        ),
        (input.policy, GovernanceDecisionReason::PolicyRequirement),
        (
            input.authority,
            GovernanceDecisionReason::AuthorityRequirement,
        ),
        (
            input.evidence_and_checks,
            GovernanceDecisionReason::EvidenceOrCheckRequirement,
        ),
        (
            input.sensitivity,
            GovernanceDecisionReason::SensitivityRequirement,
        ),
        (
            input.side_effect,
            GovernanceDecisionReason::SideEffectRequirement,
        ),
        (
            input.runtime_escalation,
            GovernanceDecisionReason::RuntimeEscalation,
        ),
    ];

    for (requirement, reason) in requirements {
        let mode = requirement_mode(requirement)?;
        if mode > GovernanceInteractionMode::QuietCapture {
            reasons.insert(reason);
        }
        selected = selected.max(mode);
    }

    if let Some(steward_minimum) = input.steward_minimum {
        let mode = requirement_mode(steward_minimum)?;
        if mode > GovernanceInteractionMode::QuietCapture {
            reasons.insert(GovernanceDecisionReason::StewardMinimum);
        }
        selected = selected.max(mode);
    }

    if let Some(prior_mode) = input.prior_mode {
        if prior_mode > GovernanceInteractionMode::QuietCapture {
            reasons.insert(GovernanceDecisionReason::PriorDecisionMinimum);
        }
        selected = selected.max(prior_mode);
    }

    Ok(ProportionalGovernanceDecision {
        mode: selected,
        risk_class: risk_class(selected),
        reasons,
    })
}

fn profile_minimum(
    profile: GovernanceStrictnessProfile,
    steward_minimum: Option<GovernancePostureRequirement>,
) -> Result<GovernanceInteractionMode, WorkflowOsError> {
    match profile {
        GovernanceStrictnessProfile::ObserveAndReport
        | GovernanceStrictnessProfile::AgentAssistedGated => {
            Ok(GovernanceInteractionMode::QuietCapture)
        }
        GovernanceStrictnessProfile::HumanApprovalGated => {
            Ok(GovernanceInteractionMode::BlockingApproval)
        }
        GovernanceStrictnessProfile::StrictEnterprise if steward_minimum.is_some() => {
            Ok(GovernanceInteractionMode::QuietCapture)
        }
        GovernanceStrictnessProfile::StrictEnterprise => Err(WorkflowOsError::invalid_state(
            "governance.proportional.steward_minimum.required",
            "strict enterprise governance requires an explicit steward minimum",
        )),
    }
}

fn requirement_mode(
    requirement: GovernancePostureRequirement,
) -> Result<GovernanceInteractionMode, WorkflowOsError> {
    match requirement {
        GovernancePostureRequirement::QuietCapture => Ok(GovernanceInteractionMode::QuietCapture),
        GovernancePostureRequirement::VisibleDisclosure => {
            Ok(GovernanceInteractionMode::VisibleDisclosure)
        }
        GovernancePostureRequirement::BlockingApproval => {
            Ok(GovernanceInteractionMode::BlockingApproval)
        }
        GovernancePostureRequirement::Denied => Ok(GovernanceInteractionMode::Denied),
        GovernancePostureRequirement::Unsupported => Err(WorkflowOsError::new(
            WorkflowOsErrorKind::Unsupported,
            "governance.proportional.requirement.unsupported",
            "a declared governance requirement is not supported",
        )),
    }
}

const fn risk_class(mode: GovernanceInteractionMode) -> GovernanceRiskClass {
    match mode {
        GovernanceInteractionMode::QuietCapture => GovernanceRiskClass::BoundedObservation,
        GovernanceInteractionMode::VisibleDisclosure => GovernanceRiskClass::ReviewWorthy,
        GovernanceInteractionMode::BlockingApproval => GovernanceRiskClass::ApprovalRequired,
        GovernanceInteractionMode::Denied => GovernanceRiskClass::Denied,
    }
}

impl<'de> Deserialize<'de> for ProportionalGovernanceDecision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            mode: GovernanceInteractionMode,
            risk_class: GovernanceRiskClass,
            reasons: BTreeSet<GovernanceDecisionReason>,
        }

        let wire = Wire::deserialize(deserializer)?;
        if wire.risk_class != risk_class(wire.mode)
            || !wire
                .reasons
                .contains(&GovernanceDecisionReason::ProfileMinimum)
        {
            return Err(serde::de::Error::custom(
                "invalid proportional governance decision",
            ));
        }
        Ok(Self {
            mode: wire.mode,
            risk_class: wire.risk_class,
            reasons: wire.reasons,
        })
    }
}
