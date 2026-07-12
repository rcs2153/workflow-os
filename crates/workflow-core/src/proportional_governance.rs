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

/// Operator action indicated by a read-only proportional-governance projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceActionRequirement {
    /// No operator action is indicated by the accepted decision.
    None,
    /// Operator review is useful but is not a blocking approval.
    Review,
    /// A future enforcing integration must obtain approval before proceeding.
    Approval,
    /// The accepted decision denies the governed action.
    Denied,
}

/// Enforcement posture of a proportional-governance decision projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecisionPosture {
    /// The projection reports an assessment but does not enforce it.
    AssessedNotEnforced,
}

/// Durability posture of a proportional-governance decision projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernancePersistencePosture {
    /// The projection exists only in memory and is not a durable record.
    NotPersisted,
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

/// Read-only, in-memory projection of an accepted proportional-governance decision.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProportionalGovernanceDecisionProjection {
    mode: GovernanceInteractionMode,
    risk_class: GovernanceRiskClass,
    reasons: BTreeSet<GovernanceDecisionReason>,
    action_requirement: GovernanceActionRequirement,
    decision_posture: GovernanceDecisionPosture,
    persistence_posture: GovernancePersistencePosture,
}

impl ProportionalGovernanceDecisionProjection {
    /// Returns the accepted interaction mode.
    #[must_use]
    pub const fn mode(&self) -> GovernanceInteractionMode {
        self.mode
    }

    /// Returns the accepted risk class.
    #[must_use]
    pub const fn risk_class(&self) -> GovernanceRiskClass {
        self.risk_class
    }

    /// Returns the accepted stable decision reasons.
    #[must_use]
    pub const fn reasons(&self) -> &BTreeSet<GovernanceDecisionReason> {
        &self.reasons
    }

    /// Returns the operator action indicated by the accepted decision.
    #[must_use]
    pub const fn action_requirement(&self) -> GovernanceActionRequirement {
        self.action_requirement
    }

    /// Returns the projection's non-enforcing posture.
    #[must_use]
    pub const fn decision_posture(&self) -> GovernanceDecisionPosture {
        self.decision_posture
    }

    /// Returns the projection's non-persistent posture.
    #[must_use]
    pub const fn persistence_posture(&self) -> GovernancePersistencePosture {
        self.persistence_posture
    }
}

/// Projects an accepted decision without enforcing or persisting it.
#[must_use]
pub fn project_proportional_governance_decision(
    decision: &ProportionalGovernanceDecision,
) -> ProportionalGovernanceDecisionProjection {
    ProportionalGovernanceDecisionProjection {
        mode: decision.mode,
        risk_class: decision.risk_class,
        reasons: decision.reasons.clone(),
        action_requirement: action_requirement(decision.mode),
        decision_posture: GovernanceDecisionPosture::AssessedNotEnforced,
        persistence_posture: GovernancePersistencePosture::NotPersisted,
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

const fn action_requirement(mode: GovernanceInteractionMode) -> GovernanceActionRequirement {
    match mode {
        GovernanceInteractionMode::QuietCapture => GovernanceActionRequirement::None,
        GovernanceInteractionMode::VisibleDisclosure => GovernanceActionRequirement::Review,
        GovernanceInteractionMode::BlockingApproval => GovernanceActionRequirement::Approval,
        GovernanceInteractionMode::Denied => GovernanceActionRequirement::Denied,
    }
}

fn decision_parts_are_valid(
    mode: GovernanceInteractionMode,
    risk_class_value: GovernanceRiskClass,
    reasons: &BTreeSet<GovernanceDecisionReason>,
) -> bool {
    risk_class_value == risk_class(mode)
        && reasons.contains(&GovernanceDecisionReason::ProfileMinimum)
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
        if !decision_parts_are_valid(wire.mode, wire.risk_class, &wire.reasons) {
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

macro_rules! projection_wire_enum {
    ($wire:ident, $value:ty, $error:literal, {$($name:literal => $variant:path),+ $(,)?}) => {
        #[derive(Clone, Copy, Eq, PartialEq)]
        struct $wire($value);

        impl<'de> Deserialize<'de> for $wire {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct SafeVisitor;

                impl<'de> serde::de::Visitor<'de> for SafeVisitor {
                    type Value = $wire;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str("a supported projection value")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            $($name => Ok($wire($variant)),)+
                            _ => Err(E::custom($error)),
                        }
                    }

                    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        self.visit_str(&value)
                    }

                    fn visit_bool<E>(self, _value: bool) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(E::custom($error))
                    }

                    fn visit_i64<E>(self, _value: i64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(E::custom($error))
                    }

                    fn visit_u64<E>(self, _value: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(E::custom($error))
                    }

                    fn visit_f64<E>(self, _value: f64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(E::custom($error))
                    }

                    fn visit_unit<E>(self) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(E::custom($error))
                    }

                    fn visit_seq<A>(self, _sequence: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        Err(serde::de::Error::custom($error))
                    }

                    fn visit_map<A>(self, _map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        Err(serde::de::Error::custom($error))
                    }
                }

                deserializer.deserialize_any(SafeVisitor)
            }
        }
    };
}

projection_wire_enum!(SafeProjectionMode, GovernanceInteractionMode,
    "invalid proportional governance projection mode", {
        "quiet_capture" => GovernanceInteractionMode::QuietCapture,
        "visible_disclosure" => GovernanceInteractionMode::VisibleDisclosure,
        "blocking_approval" => GovernanceInteractionMode::BlockingApproval,
        "denied" => GovernanceInteractionMode::Denied,
    }
);
projection_wire_enum!(SafeProjectionRiskClass, GovernanceRiskClass,
    "invalid proportional governance projection risk class", {
        "bounded_observation" => GovernanceRiskClass::BoundedObservation,
        "review_worthy" => GovernanceRiskClass::ReviewWorthy,
        "approval_required" => GovernanceRiskClass::ApprovalRequired,
        "denied" => GovernanceRiskClass::Denied,
    }
);
projection_wire_enum!(SafeProjectionReason, GovernanceDecisionReason,
    "invalid proportional governance projection reason", {
        "profile_minimum" => GovernanceDecisionReason::ProfileMinimum,
        "workflow_requirement" => GovernanceDecisionReason::WorkflowRequirement,
        "policy_requirement" => GovernanceDecisionReason::PolicyRequirement,
        "authority_requirement" => GovernanceDecisionReason::AuthorityRequirement,
        "evidence_or_check_requirement" => GovernanceDecisionReason::EvidenceOrCheckRequirement,
        "sensitivity_requirement" => GovernanceDecisionReason::SensitivityRequirement,
        "side_effect_requirement" => GovernanceDecisionReason::SideEffectRequirement,
        "runtime_escalation" => GovernanceDecisionReason::RuntimeEscalation,
        "prior_decision_minimum" => GovernanceDecisionReason::PriorDecisionMinimum,
        "steward_minimum" => GovernanceDecisionReason::StewardMinimum,
    }
);
projection_wire_enum!(SafeProjectionAction, GovernanceActionRequirement,
    "invalid proportional governance projection action requirement", {
        "none" => GovernanceActionRequirement::None,
        "review" => GovernanceActionRequirement::Review,
        "approval" => GovernanceActionRequirement::Approval,
        "denied" => GovernanceActionRequirement::Denied,
    }
);
projection_wire_enum!(SafeProjectionDecisionPosture, GovernanceDecisionPosture,
    "invalid proportional governance projection decision posture", {
        "assessed_not_enforced" => GovernanceDecisionPosture::AssessedNotEnforced,
    }
);
projection_wire_enum!(SafeProjectionPersistencePosture, GovernancePersistencePosture,
    "invalid proportional governance projection persistence posture", {
        "not_persisted" => GovernancePersistencePosture::NotPersisted,
    }
);

impl<'de> Deserialize<'de> for ProportionalGovernanceDecisionProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            mode: SafeProjectionMode,
            risk_class: SafeProjectionRiskClass,
            reasons: Vec<SafeProjectionReason>,
            action_requirement: SafeProjectionAction,
            decision_posture: SafeProjectionDecisionPosture,
            persistence_posture: SafeProjectionPersistencePosture,
        }

        let wire = Wire::deserialize(deserializer)?;
        let reason_count = wire.reasons.len();
        let reasons = wire
            .reasons
            .into_iter()
            .map(|reason| reason.0)
            .collect::<BTreeSet<_>>();
        if reasons.len() != reason_count
            || !decision_parts_are_valid(wire.mode.0, wire.risk_class.0, &reasons)
            || wire.action_requirement.0 != action_requirement(wire.mode.0)
            || wire.decision_posture.0 != GovernanceDecisionPosture::AssessedNotEnforced
            || wire.persistence_posture.0 != GovernancePersistencePosture::NotPersisted
        {
            return Err(serde::de::Error::custom(
                "invalid proportional governance decision projection",
            ));
        }

        Ok(Self {
            mode: wire.mode.0,
            risk_class: wire.risk_class.0,
            reasons,
            action_requirement: wire.action_requirement.0,
            decision_posture: wire.decision_posture.0,
            persistence_posture: wire.persistence_posture.0,
        })
    }
}
