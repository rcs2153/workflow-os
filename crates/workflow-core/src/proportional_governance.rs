use std::collections::BTreeSet;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{GovernanceStrictnessProfile, WorkflowOsError, WorkflowOsErrorKind};

/// Ordered execution disposition selected by proportional governance.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceExecutionDisposition {
    /// The governed action may proceed without an approval pause.
    Proceed,
    /// The governed action must pause for explicit approval.
    RequireApproval,
    /// The governed action must fail closed.
    Denied,
}

/// Ordered operator-disclosure requirement selected independently of execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDisclosureRequirement {
    /// Immediate operator disclosure is not required.
    Quiet,
    /// A bounded operator-visible disclosure is required.
    Visible,
}

/// Ordered risk classification derived from both accepted decision axes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceRiskClass {
    /// Bounded work may proceed without required immediate disclosure.
    BoundedObservation,
    /// Permitted work requires operator-visible disclosure.
    ReviewWorthy,
    /// Work must pause for approval.
    ApprovalRequired,
    /// Work must fail closed.
    Denied,
}

/// Execution requirement contributed by one validated governance concern.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceExecutionRequirement {
    /// The concern permits execution without an approval pause.
    Proceed,
    /// The concern requires blocking approval.
    RequireApproval,
    /// The concern denies the action.
    Denied,
    /// The concern declares behavior this model cannot enforce.
    Unsupported,
}

/// Disclosure obligation contributed by one validated governance concern.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDisclosureObligation {
    /// The concern permits quiet presentation.
    QuietAllowed,
    /// The concern requires bounded visible disclosure.
    VisibleRequired,
    /// The concern declares behavior this model cannot enforce.
    Unsupported,
}

/// Independent execution and disclosure requirements from one concern.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GovernancePostureRequirement {
    execution: GovernanceExecutionRequirement,
    disclosure: GovernanceDisclosureObligation,
}

impl GovernancePostureRequirement {
    /// Creates one explicit two-axis governance requirement.
    #[must_use]
    pub const fn new(
        execution: GovernanceExecutionRequirement,
        disclosure: GovernanceDisclosureObligation,
    ) -> Self {
        Self {
            execution,
            disclosure,
        }
    }

    /// Returns a requirement that permits quiet, uninterrupted execution.
    #[must_use]
    pub const fn quiet() -> Self {
        Self::new(
            GovernanceExecutionRequirement::Proceed,
            GovernanceDisclosureObligation::QuietAllowed,
        )
    }

    /// Returns a requirement that permits execution but requires disclosure.
    #[must_use]
    pub const fn visible() -> Self {
        Self::new(
            GovernanceExecutionRequirement::Proceed,
            GovernanceDisclosureObligation::VisibleRequired,
        )
    }

    /// Returns a requirement that blocks for approval and requires disclosure.
    #[must_use]
    pub const fn approval() -> Self {
        Self::new(
            GovernanceExecutionRequirement::RequireApproval,
            GovernanceDisclosureObligation::VisibleRequired,
        )
    }

    /// Returns a requirement that denies execution and requires disclosure.
    #[must_use]
    pub const fn denied() -> Self {
        Self::new(
            GovernanceExecutionRequirement::Denied,
            GovernanceDisclosureObligation::VisibleRequired,
        )
    }

    /// Returns the execution requirement.
    #[must_use]
    pub const fn execution(&self) -> GovernanceExecutionRequirement {
        self.execution
    }

    /// Returns the disclosure obligation.
    #[must_use]
    pub const fn disclosure(&self) -> GovernanceDisclosureObligation {
        self.disclosure
    }
}

/// Stable, payload-free reason for a proportional-governance decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecisionReason {
    /// The active governance profile established a minimum.
    ProfileMinimum,
    /// The workflow declaration contributed a requirement.
    WorkflowRequirement,
    /// A policy decision contributed a requirement.
    PolicyRequirement,
    /// Actor or delegated authority contributed a requirement.
    AuthorityRequirement,
    /// Evidence or check availability contributed a requirement.
    EvidenceOrCheckRequirement,
    /// Sensitivity classification contributed a requirement.
    SensitivityRequirement,
    /// Side-effect posture contributed a requirement.
    SideEffectRequirement,
    /// A validated runtime change required escalation.
    RuntimeEscalation,
    /// A prior decision prevented a downgrade.
    PriorDecisionMinimum,
    /// A steward-defined minimum contributed a requirement.
    StewardMinimum,
}

/// Operator action indicated by a read-only decision projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceActionRequirement {
    /// No blocking operator action is required.
    None,
    /// A future enforcing integration must obtain approval before proceeding.
    Approval,
    /// The accepted decision denies the governed action.
    Denied,
}

/// Enforcement posture of a decision projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecisionPosture {
    /// The projection reports an assessment but does not enforce it.
    AssessedNotEnforced,
}

/// Durability posture of a decision projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
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
    /// Workflow-declared requirements.
    pub workflow: GovernancePostureRequirement,
    /// Policy-derived requirements.
    pub policy: GovernancePostureRequirement,
    /// Actor or delegated-authority requirements.
    pub authority: GovernancePostureRequirement,
    /// Combined evidence and check requirements.
    pub evidence_and_checks: GovernancePostureRequirement,
    /// Sensitivity-derived requirements.
    pub sensitivity: GovernancePostureRequirement,
    /// Side-effect-derived requirements.
    pub side_effect: GovernancePostureRequirement,
    /// Requirements derived from validated runtime changes.
    pub runtime_escalation: GovernancePostureRequirement,
    /// Prior execution disposition for this governed action.
    pub prior_execution: Option<GovernanceExecutionDisposition>,
    /// Prior disclosure requirement for this governed action.
    pub prior_disclosure: Option<GovernanceDisclosureRequirement>,
    /// Explicit steward minimum, required for strict-enterprise evaluation.
    pub steward_minimum: Option<GovernancePostureRequirement>,
}

/// Deterministic result of proportional-governance selection.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProportionalGovernanceDecision {
    execution: GovernanceExecutionDisposition,
    disclosure: GovernanceDisclosureRequirement,
    risk_class: GovernanceRiskClass,
    reasons: BTreeSet<GovernanceDecisionReason>,
}

impl ProportionalGovernanceDecision {
    /// Returns the selected execution disposition.
    #[must_use]
    pub const fn execution(&self) -> GovernanceExecutionDisposition {
        self.execution
    }

    /// Returns the selected disclosure requirement.
    #[must_use]
    pub const fn disclosure(&self) -> GovernanceDisclosureRequirement {
        self.disclosure
    }

    /// Returns the corresponding ordered risk class.
    #[must_use]
    pub const fn risk_class(&self) -> GovernanceRiskClass {
        self.risk_class
    }

    /// Returns stable reasons that contributed above the quiet baseline.
    #[must_use]
    pub const fn reasons(&self) -> &BTreeSet<GovernanceDecisionReason> {
        &self.reasons
    }
}

/// Read-only, in-memory projection of an accepted decision.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProportionalGovernanceDecisionProjection {
    execution: GovernanceExecutionDisposition,
    disclosure: GovernanceDisclosureRequirement,
    risk_class: GovernanceRiskClass,
    reasons: BTreeSet<GovernanceDecisionReason>,
    action_requirement: GovernanceActionRequirement,
    decision_posture: GovernanceDecisionPosture,
    persistence_posture: GovernancePersistencePosture,
}

impl ProportionalGovernanceDecisionProjection {
    /// Returns the accepted execution disposition.
    #[must_use]
    pub const fn execution(&self) -> GovernanceExecutionDisposition {
        self.execution
    }

    /// Returns the accepted disclosure requirement.
    #[must_use]
    pub const fn disclosure(&self) -> GovernanceDisclosureRequirement {
        self.disclosure
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

    /// Returns the blocking operator action indicated by the decision.
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
        execution: decision.execution,
        disclosure: decision.disclosure,
        risk_class: decision.risk_class,
        reasons: decision.reasons.clone(),
        action_requirement: action_requirement(decision.execution),
        decision_posture: GovernanceDecisionPosture::AssessedNotEnforced,
        persistence_posture: GovernancePersistencePosture::NotPersisted,
    }
}

/// Selects the strictest applicable execution and disclosure requirements.
///
/// # Errors
///
/// Returns stable, non-leaking errors when a declared requirement is
/// unsupported or strict-enterprise evaluation lacks a steward minimum.
pub fn select_proportional_governance(
    input: ProportionalGovernanceDecisionInput,
) -> Result<ProportionalGovernanceDecision, WorkflowOsError> {
    let mut selected = profile_minimum(input.profile, input.steward_minimum)?;
    let mut reasons = BTreeSet::from([GovernanceDecisionReason::ProfileMinimum]);

    for (requirement, reason) in [
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
    ] {
        let posture = requirement_posture(requirement)?;
        if posture != RequirementPosture::quiet() {
            reasons.insert(reason);
        }
        selected = selected.max(posture);
    }

    if let Some(steward_minimum) = input.steward_minimum {
        let posture = requirement_posture(steward_minimum)?;
        if posture != RequirementPosture::quiet() {
            reasons.insert(GovernanceDecisionReason::StewardMinimum);
        }
        selected = selected.max(posture);
    }

    if let Some(prior_execution) = input.prior_execution {
        if prior_execution > GovernanceExecutionDisposition::Proceed {
            reasons.insert(GovernanceDecisionReason::PriorDecisionMinimum);
        }
        selected.execution = selected.execution.max(prior_execution);
    }

    if let Some(prior_disclosure) = input.prior_disclosure {
        if prior_disclosure > GovernanceDisclosureRequirement::Quiet {
            reasons.insert(GovernanceDecisionReason::PriorDecisionMinimum);
        }
        selected.disclosure = selected.disclosure.max(prior_disclosure);
    }

    selected = normalize_cross_axis_posture(selected);

    Ok(ProportionalGovernanceDecision {
        execution: selected.execution,
        disclosure: selected.disclosure,
        risk_class: risk_class(selected.execution, selected.disclosure),
        reasons,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RequirementPosture {
    execution: GovernanceExecutionDisposition,
    disclosure: GovernanceDisclosureRequirement,
}

impl RequirementPosture {
    const fn quiet() -> Self {
        Self {
            execution: GovernanceExecutionDisposition::Proceed,
            disclosure: GovernanceDisclosureRequirement::Quiet,
        }
    }

    fn max(self, other: Self) -> Self {
        Self {
            execution: self.execution.max(other.execution),
            disclosure: self.disclosure.max(other.disclosure),
        }
    }
}

fn profile_minimum(
    profile: GovernanceStrictnessProfile,
    steward_minimum: Option<GovernancePostureRequirement>,
) -> Result<RequirementPosture, WorkflowOsError> {
    match profile {
        GovernanceStrictnessProfile::ObserveAndReport
        | GovernanceStrictnessProfile::AgentAssistedGated => Ok(RequirementPosture::quiet()),
        GovernanceStrictnessProfile::HumanApprovalGated => Ok(RequirementPosture {
            execution: GovernanceExecutionDisposition::RequireApproval,
            disclosure: GovernanceDisclosureRequirement::Visible,
        }),
        GovernanceStrictnessProfile::StrictEnterprise if steward_minimum.is_some() => {
            Ok(RequirementPosture::quiet())
        }
        GovernanceStrictnessProfile::StrictEnterprise => Err(WorkflowOsError::invalid_state(
            "governance.proportional.steward_minimum.required",
            "strict enterprise governance requires an explicit steward minimum",
        )),
    }
}

fn requirement_posture(
    requirement: GovernancePostureRequirement,
) -> Result<RequirementPosture, WorkflowOsError> {
    let execution = match requirement.execution {
        GovernanceExecutionRequirement::Proceed => GovernanceExecutionDisposition::Proceed,
        GovernanceExecutionRequirement::RequireApproval => {
            GovernanceExecutionDisposition::RequireApproval
        }
        GovernanceExecutionRequirement::Denied => GovernanceExecutionDisposition::Denied,
        GovernanceExecutionRequirement::Unsupported => return Err(unsupported_requirement()),
    };
    let disclosure = match requirement.disclosure {
        GovernanceDisclosureObligation::QuietAllowed => GovernanceDisclosureRequirement::Quiet,
        GovernanceDisclosureObligation::VisibleRequired => GovernanceDisclosureRequirement::Visible,
        GovernanceDisclosureObligation::Unsupported => return Err(unsupported_requirement()),
    };
    Ok(normalize_cross_axis_posture(RequirementPosture {
        execution,
        disclosure,
    }))
}

const fn normalize_cross_axis_posture(mut posture: RequirementPosture) -> RequirementPosture {
    if !matches!(posture.execution, GovernanceExecutionDisposition::Proceed) {
        posture.disclosure = GovernanceDisclosureRequirement::Visible;
    }
    posture
}

fn unsupported_requirement() -> WorkflowOsError {
    WorkflowOsError::new(
        WorkflowOsErrorKind::Unsupported,
        "governance.proportional.requirement.unsupported",
        "a declared governance requirement is not supported",
    )
}

const fn risk_class(
    execution: GovernanceExecutionDisposition,
    disclosure: GovernanceDisclosureRequirement,
) -> GovernanceRiskClass {
    match (execution, disclosure) {
        (GovernanceExecutionDisposition::Denied, _) => GovernanceRiskClass::Denied,
        (GovernanceExecutionDisposition::RequireApproval, _) => {
            GovernanceRiskClass::ApprovalRequired
        }
        (GovernanceExecutionDisposition::Proceed, GovernanceDisclosureRequirement::Visible) => {
            GovernanceRiskClass::ReviewWorthy
        }
        (GovernanceExecutionDisposition::Proceed, GovernanceDisclosureRequirement::Quiet) => {
            GovernanceRiskClass::BoundedObservation
        }
    }
}

const fn action_requirement(
    execution: GovernanceExecutionDisposition,
) -> GovernanceActionRequirement {
    match execution {
        GovernanceExecutionDisposition::Proceed => GovernanceActionRequirement::None,
        GovernanceExecutionDisposition::RequireApproval => GovernanceActionRequirement::Approval,
        GovernanceExecutionDisposition::Denied => GovernanceActionRequirement::Denied,
    }
}

fn decision_parts_are_valid(
    execution: GovernanceExecutionDisposition,
    disclosure: GovernanceDisclosureRequirement,
    risk_class_value: GovernanceRiskClass,
    reasons: &BTreeSet<GovernanceDecisionReason>,
) -> bool {
    risk_class_value == risk_class(execution, disclosure)
        && reasons.contains(&GovernanceDecisionReason::ProfileMinimum)
        && (matches!(execution, GovernanceExecutionDisposition::Proceed)
            || matches!(disclosure, GovernanceDisclosureRequirement::Visible))
}

macro_rules! safe_wire_enum {
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
                        formatter.write_str("a supported proportional governance value")
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
                    where E: serde::de::Error { Err(E::custom($error)) }
                    fn visit_i64<E>(self, _value: i64) -> Result<Self::Value, E>
                    where E: serde::de::Error { Err(E::custom($error)) }
                    fn visit_u64<E>(self, _value: u64) -> Result<Self::Value, E>
                    where E: serde::de::Error { Err(E::custom($error)) }
                    fn visit_f64<E>(self, _value: f64) -> Result<Self::Value, E>
                    where E: serde::de::Error { Err(E::custom($error)) }
                    fn visit_unit<E>(self) -> Result<Self::Value, E>
                    where E: serde::de::Error { Err(E::custom($error)) }
                    fn visit_seq<A>(self, _sequence: A) -> Result<Self::Value, A::Error>
                    where A: serde::de::SeqAccess<'de> { Err(serde::de::Error::custom($error)) }
                    fn visit_map<A>(self, _map: A) -> Result<Self::Value, A::Error>
                    where A: serde::de::MapAccess<'de> { Err(serde::de::Error::custom($error)) }
                }

                deserializer.deserialize_any(SafeVisitor)
            }
        }
    };
}

safe_wire_enum!(SafeExecution, GovernanceExecutionDisposition,
    "invalid proportional governance execution disposition", {
        "proceed" => GovernanceExecutionDisposition::Proceed,
        "require_approval" => GovernanceExecutionDisposition::RequireApproval,
        "denied" => GovernanceExecutionDisposition::Denied,
    }
);
safe_wire_enum!(SafeDisclosure, GovernanceDisclosureRequirement,
    "invalid proportional governance disclosure requirement", {
        "quiet" => GovernanceDisclosureRequirement::Quiet,
        "visible" => GovernanceDisclosureRequirement::Visible,
    }
);
safe_wire_enum!(SafeRiskClass, GovernanceRiskClass,
    "invalid proportional governance risk class", {
        "bounded_observation" => GovernanceRiskClass::BoundedObservation,
        "review_worthy" => GovernanceRiskClass::ReviewWorthy,
        "approval_required" => GovernanceRiskClass::ApprovalRequired,
        "denied" => GovernanceRiskClass::Denied,
    }
);
safe_wire_enum!(SafeReason, GovernanceDecisionReason,
    "invalid proportional governance decision reason", {
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
safe_wire_enum!(SafeAction, GovernanceActionRequirement,
    "invalid proportional governance action requirement", {
        "none" => GovernanceActionRequirement::None,
        "approval" => GovernanceActionRequirement::Approval,
        "denied" => GovernanceActionRequirement::Denied,
    }
);
safe_wire_enum!(SafeDecisionPosture, GovernanceDecisionPosture,
    "invalid proportional governance decision posture", {
        "assessed_not_enforced" => GovernanceDecisionPosture::AssessedNotEnforced,
    }
);
safe_wire_enum!(SafePersistencePosture, GovernancePersistencePosture,
    "invalid proportional governance persistence posture", {
        "not_persisted" => GovernancePersistencePosture::NotPersisted,
    }
);

impl<'de> Deserialize<'de> for GovernanceExecutionDisposition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        SafeExecution::deserialize(deserializer).map(|value| value.0)
    }
}

impl<'de> Deserialize<'de> for GovernanceDisclosureRequirement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        SafeDisclosure::deserialize(deserializer).map(|value| value.0)
    }
}

fn collect_reasons(values: Vec<SafeReason>) -> Option<BTreeSet<GovernanceDecisionReason>> {
    let count = values.len();
    let reasons = values
        .into_iter()
        .map(|reason| reason.0)
        .collect::<BTreeSet<_>>();
    (reasons.len() == count).then_some(reasons)
}

impl<'de> Deserialize<'de> for ProportionalGovernanceDecision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            execution: SafeExecution,
            disclosure: SafeDisclosure,
            risk_class: SafeRiskClass,
            reasons: Vec<SafeReason>,
        }

        let wire = Wire::deserialize(deserializer)?;
        let Some(reasons) = collect_reasons(wire.reasons) else {
            return Err(serde::de::Error::custom(
                "invalid proportional governance decision",
            ));
        };
        if !decision_parts_are_valid(
            wire.execution.0,
            wire.disclosure.0,
            wire.risk_class.0,
            &reasons,
        ) {
            return Err(serde::de::Error::custom(
                "invalid proportional governance decision",
            ));
        }
        Ok(Self {
            execution: wire.execution.0,
            disclosure: wire.disclosure.0,
            risk_class: wire.risk_class.0,
            reasons,
        })
    }
}

impl<'de> Deserialize<'de> for ProportionalGovernanceDecisionProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            execution: SafeExecution,
            disclosure: SafeDisclosure,
            risk_class: SafeRiskClass,
            reasons: Vec<SafeReason>,
            action_requirement: SafeAction,
            decision_posture: SafeDecisionPosture,
            persistence_posture: SafePersistencePosture,
        }

        let wire = Wire::deserialize(deserializer)?;
        let Some(reasons) = collect_reasons(wire.reasons) else {
            return Err(serde::de::Error::custom(
                "invalid proportional governance decision projection",
            ));
        };
        if !decision_parts_are_valid(
            wire.execution.0,
            wire.disclosure.0,
            wire.risk_class.0,
            &reasons,
        ) || wire.action_requirement.0 != action_requirement(wire.execution.0)
            || wire.decision_posture.0 != GovernanceDecisionPosture::AssessedNotEnforced
            || wire.persistence_posture.0 != GovernancePersistencePosture::NotPersisted
        {
            return Err(serde::de::Error::custom(
                "invalid proportional governance decision projection",
            ));
        }

        Ok(Self {
            execution: wire.execution.0,
            disclosure: wire.disclosure.0,
            risk_class: wire.risk_class.0,
            reasons,
            action_requirement: wire.action_requirement.0,
            decision_posture: wire.decision_posture.0,
            persistence_posture: wire.persistence_posture.0,
        })
    }
}
