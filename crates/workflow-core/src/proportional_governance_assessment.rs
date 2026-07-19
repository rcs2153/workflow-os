use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    select_proportional_governance, GovernanceDecisionPosture, GovernanceDisclosureObligation,
    GovernanceDisclosureRequirement, GovernanceExecutionDisposition,
    GovernanceExecutionRequirement, GovernancePersistencePosture, GovernancePostureRequirement,
    GovernanceStrictnessProfile, ProportionalGovernanceDecision,
    ProportionalGovernanceDecisionInput, SpecContentHash, WorkflowOsError,
};

const ASSESSMENT_ALGORITHM: &str = "workflow-os/proportional-governance-workload-assessment/v1";

/// Versioned deterministic workload-assessment algorithm identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceWorkloadAssessmentAlgorithm {
    /// Initial bounded typed-fact classifier and fingerprint format.
    V1,
}

impl GovernanceWorkloadAssessmentAlgorithm {
    /// Returns the stable domain-separated algorithm identifier.
    #[must_use]
    pub const fn identifier(self) -> &'static str {
        match self {
            Self::V1 => ASSESSMENT_ALGORITHM,
        }
    }
}

/// Bounded action class derived from validated workload metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceWorkloadActionClass {
    /// Reads bounded local or provider state without mutation.
    ReadOnly,
    /// Mutates bounded local state with a defined rollback posture.
    LocalMutation,
    /// Mutates an external provider or externally visible resource.
    ExternalMutation,
    /// The action class cannot yet be established from validated facts.
    Unknown,
    /// The action class is outside the supported governance vocabulary.
    Unsupported,
}

/// Authority available to the assessed workload.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceWorkloadAuthorityPosture {
    /// Validated scoped authority is already sufficient.
    Sufficient,
    /// Explicit approval is required to establish sufficient authority.
    ApprovalRequired,
    /// Required authority is unavailable.
    Unavailable,
    /// Authority cannot yet be established from validated facts.
    Unknown,
}

/// Evidence and deterministic-check posture for the assessed workload.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceWorkloadEvidenceCheckPosture {
    /// Required evidence and checks are satisfied.
    Satisfied,
    /// Optional evidence or checks are unavailable and must be disclosed.
    OptionalUnavailable,
    /// Required evidence or checks are unavailable.
    RequiredUnavailable,
    /// A required deterministic check failed.
    Failed,
    /// Evidence or check posture cannot yet be established.
    Unknown,
}

/// Bounded sensitivity classification for the assessed workload.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceWorkloadSensitivity {
    /// Public or ordinary internal metadata.
    Routine,
    /// Elevated but bounded sensitivity requiring visible disclosure.
    Elevated,
    /// Restricted material requiring approval before execution.
    Restricted,
    /// Sensitivity cannot yet be established from validated facts.
    Unknown,
}

/// Side-effect posture derived from validated workload metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceWorkloadSideEffectPosture {
    /// No `SideEffect` is proposed.
    None,
    /// A bounded, reversible local `SideEffect` is proposed.
    LocalReversible,
    /// A bounded, reversible external `SideEffect` is proposed.
    ExternalReversible,
    /// An irreversible external `SideEffect` is proposed.
    ExternalIrreversible,
    /// The proposed `SideEffect` is ambiguous and must fail closed.
    Ambiguous,
    /// The `SideEffect` is outside the supported governance vocabulary.
    Unsupported,
    /// `SideEffect` posture cannot yet be established from validated facts.
    Unknown,
}

/// Safety-relevant fact category that remains unresolved.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceAssessmentUnknownFact {
    /// Action class remains unresolved.
    ActionClass,
    /// Authority posture remains unresolved.
    Authority,
    /// Evidence or check posture remains unresolved.
    EvidenceAndChecks,
    /// Sensitivity remains unresolved.
    Sensitivity,
    /// `SideEffect` posture remains unresolved.
    SideEffect,
}

/// Deterministic completeness posture, not probabilistic confidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceAssessmentCompleteness {
    /// Every modeled safety-relevant fact is known.
    Complete,
    /// One or more modeled safety-relevant facts remains unknown.
    Incomplete,
}

impl<'de> Deserialize<'de> for GovernanceAssessmentCompleteness {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "complete" => Ok(Self::Complete),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err(serde::de::Error::custom(
                "governance assessment completeness is invalid",
            )),
        }
    }
}

/// Explicit, typed input to review-only workload assessment.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ProportionalGovernanceWorkloadAssessmentInput {
    /// Canonical immutable definition root used for reassessment invalidation.
    pub definition_root: SpecContentHash,
    /// Active governance profile.
    pub profile: GovernanceStrictnessProfile,
    /// Explicit workflow minimum.
    pub workflow_minimum: GovernancePostureRequirement,
    /// Explicit policy minimum.
    pub policy_minimum: GovernancePostureRequirement,
    /// Validated action class.
    pub action_class: GovernanceWorkloadActionClass,
    /// Validated authority posture.
    pub authority: GovernanceWorkloadAuthorityPosture,
    /// Explicit authority minimum that inference may not weaken.
    pub authority_minimum: GovernancePostureRequirement,
    /// Validated evidence and check posture.
    pub evidence_and_checks: GovernanceWorkloadEvidenceCheckPosture,
    /// Explicit evidence/check minimum that inference may not weaken.
    pub evidence_and_check_minimum: GovernancePostureRequirement,
    /// Validated sensitivity posture.
    pub sensitivity: GovernanceWorkloadSensitivity,
    /// Explicit sensitivity minimum that inference may not weaken.
    pub sensitivity_minimum: GovernancePostureRequirement,
    /// Validated `SideEffect` posture.
    pub side_effect: GovernanceWorkloadSideEffectPosture,
    /// Explicit `SideEffect` minimum that inference may not weaken.
    pub side_effect_minimum: GovernancePostureRequirement,
    /// Validated runtime escalation minimum.
    pub runtime_escalation: GovernancePostureRequirement,
    /// Prior accepted execution disposition for this governed action.
    pub prior_execution: Option<GovernanceExecutionDisposition>,
    /// Prior accepted disclosure requirement for this governed action.
    pub prior_disclosure: Option<GovernanceDisclosureRequirement>,
    /// Explicit steward minimum, required by strict-enterprise selection.
    pub steward_minimum: Option<GovernancePostureRequirement>,
}

impl fmt::Debug for ProportionalGovernanceWorkloadAssessmentInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProportionalGovernanceWorkloadAssessmentInput")
            .field("definition_root", &"<redacted>")
            .field("profile", &self.profile)
            .field("workflow_minimum", &self.workflow_minimum)
            .field("policy_minimum", &self.policy_minimum)
            .field("action_class", &self.action_class)
            .field("authority", &self.authority)
            .field("authority_minimum", &self.authority_minimum)
            .field("evidence_and_checks", &self.evidence_and_checks)
            .field(
                "evidence_and_check_minimum",
                &self.evidence_and_check_minimum,
            )
            .field("sensitivity", &self.sensitivity)
            .field("sensitivity_minimum", &self.sensitivity_minimum)
            .field("side_effect", &self.side_effect)
            .field("side_effect_minimum", &self.side_effect_minimum)
            .field("runtime_escalation", &self.runtime_escalation)
            .field("prior_execution", &self.prior_execution)
            .field("prior_disclosure", &self.prior_disclosure)
            .field("steward_minimum", &self.steward_minimum)
            .finish()
    }
}

/// Review-only workload assessment and accepted proportional-governance result.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ProportionalGovernanceWorkloadAssessment {
    algorithm: GovernanceWorkloadAssessmentAlgorithm,
    decision: ProportionalGovernanceDecision,
    unknown_facts: BTreeSet<GovernanceAssessmentUnknownFact>,
    completeness: GovernanceAssessmentCompleteness,
    input_fingerprint: SpecContentHash,
}

impl ProportionalGovernanceWorkloadAssessment {
    /// Returns the versioned assessment algorithm identity.
    #[must_use]
    pub const fn algorithm(&self) -> GovernanceWorkloadAssessmentAlgorithm {
        self.algorithm
    }

    /// Returns the accepted decision after composing inferred and explicit minima.
    #[must_use]
    pub const fn decision(&self) -> &ProportionalGovernanceDecision {
        &self.decision
    }

    /// Returns unresolved safety-relevant fact categories.
    #[must_use]
    pub const fn unknown_facts(&self) -> &BTreeSet<GovernanceAssessmentUnknownFact> {
        &self.unknown_facts
    }

    /// Returns deterministic fact completeness, not model confidence.
    #[must_use]
    pub const fn completeness(&self) -> GovernanceAssessmentCompleteness {
        self.completeness
    }

    /// Returns the versioned payload-free input fingerprint.
    #[must_use]
    pub const fn input_fingerprint(&self) -> &SpecContentHash {
        &self.input_fingerprint
    }

    /// Returns the explicit non-enforcing posture of this recommendation.
    #[must_use]
    pub const fn decision_posture(&self) -> GovernanceDecisionPosture {
        GovernanceDecisionPosture::AssessedNotEnforced
    }

    /// Returns the explicit in-memory-only posture of this recommendation.
    #[must_use]
    pub const fn persistence_posture(&self) -> GovernancePersistencePosture {
        GovernancePersistencePosture::NotPersisted
    }
}

impl fmt::Debug for ProportionalGovernanceWorkloadAssessment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProportionalGovernanceWorkloadAssessment")
            .field("algorithm", &self.algorithm)
            .field("decision", &self.decision)
            .field("unknown_facts", &self.unknown_facts)
            .field("completeness", &self.completeness)
            .field("input_fingerprint", &"<redacted>")
            .finish()
    }
}

/// Derives a deterministic review-only recommendation from bounded workload facts.
///
/// Explicit workflow, policy, authority, evidence/check, sensitivity,
/// `SideEffect`, runtime, prior-decision, profile, and steward minima remain
/// authoritative. Inference may hold or raise posture, never weaken it.
///
/// # Errors
///
/// Returns the stable errors produced by the accepted proportional-governance
/// selector, including unsupported explicit requirements or a missing strict
/// enterprise steward minimum.
pub fn assess_proportional_governance_workload(
    input: &ProportionalGovernanceWorkloadAssessmentInput,
) -> Result<ProportionalGovernanceWorkloadAssessment, WorkflowOsError> {
    let unknown_facts = collect_unknown_facts(input);
    let completeness = if unknown_facts.is_empty() {
        GovernanceAssessmentCompleteness::Complete
    } else {
        GovernanceAssessmentCompleteness::Incomplete
    };
    let input_fingerprint = compute_input_fingerprint(input);

    let decision = select_proportional_governance(ProportionalGovernanceDecisionInput {
        profile: input.profile,
        workload_assessment: action_requirement(input.action_class),
        workflow: input.workflow_minimum,
        policy: input.policy_minimum,
        authority: strictest_requirement(
            input.authority_minimum,
            authority_requirement(input.authority),
        ),
        evidence_and_checks: strictest_requirement(
            input.evidence_and_check_minimum,
            evidence_requirement(input.evidence_and_checks),
        ),
        sensitivity: strictest_requirement(
            input.sensitivity_minimum,
            sensitivity_requirement(input.sensitivity),
        ),
        side_effect: strictest_requirement(
            input.side_effect_minimum,
            side_effect_requirement(input.side_effect),
        ),
        runtime_escalation: input.runtime_escalation,
        prior_execution: input.prior_execution,
        prior_disclosure: input.prior_disclosure,
        steward_minimum: input.steward_minimum,
    })?;

    Ok(ProportionalGovernanceWorkloadAssessment {
        algorithm: GovernanceWorkloadAssessmentAlgorithm::V1,
        decision,
        unknown_facts,
        completeness,
        input_fingerprint,
    })
}

fn action_requirement(action: GovernanceWorkloadActionClass) -> GovernancePostureRequirement {
    match action {
        GovernanceWorkloadActionClass::ReadOnly => GovernancePostureRequirement::quiet(),
        GovernanceWorkloadActionClass::LocalMutation => GovernancePostureRequirement::visible(),
        GovernanceWorkloadActionClass::ExternalMutation
        | GovernanceWorkloadActionClass::Unknown => GovernancePostureRequirement::approval(),
        GovernanceWorkloadActionClass::Unsupported => GovernancePostureRequirement::denied(),
    }
}

fn authority_requirement(
    authority: GovernanceWorkloadAuthorityPosture,
) -> GovernancePostureRequirement {
    match authority {
        GovernanceWorkloadAuthorityPosture::Sufficient => GovernancePostureRequirement::quiet(),
        GovernanceWorkloadAuthorityPosture::ApprovalRequired
        | GovernanceWorkloadAuthorityPosture::Unknown => GovernancePostureRequirement::approval(),
        GovernanceWorkloadAuthorityPosture::Unavailable => GovernancePostureRequirement::denied(),
    }
}

fn evidence_requirement(
    evidence: GovernanceWorkloadEvidenceCheckPosture,
) -> GovernancePostureRequirement {
    match evidence {
        GovernanceWorkloadEvidenceCheckPosture::Satisfied => GovernancePostureRequirement::quiet(),
        GovernanceWorkloadEvidenceCheckPosture::OptionalUnavailable => {
            GovernancePostureRequirement::visible()
        }
        GovernanceWorkloadEvidenceCheckPosture::Unknown => GovernancePostureRequirement::approval(),
        GovernanceWorkloadEvidenceCheckPosture::RequiredUnavailable
        | GovernanceWorkloadEvidenceCheckPosture::Failed => GovernancePostureRequirement::denied(),
    }
}

fn sensitivity_requirement(
    sensitivity: GovernanceWorkloadSensitivity,
) -> GovernancePostureRequirement {
    match sensitivity {
        GovernanceWorkloadSensitivity::Routine => GovernancePostureRequirement::quiet(),
        GovernanceWorkloadSensitivity::Elevated => GovernancePostureRequirement::visible(),
        GovernanceWorkloadSensitivity::Restricted | GovernanceWorkloadSensitivity::Unknown => {
            GovernancePostureRequirement::approval()
        }
    }
}

fn side_effect_requirement(
    side_effect: GovernanceWorkloadSideEffectPosture,
) -> GovernancePostureRequirement {
    match side_effect {
        GovernanceWorkloadSideEffectPosture::None => GovernancePostureRequirement::quiet(),
        GovernanceWorkloadSideEffectPosture::LocalReversible => {
            GovernancePostureRequirement::visible()
        }
        GovernanceWorkloadSideEffectPosture::ExternalReversible
        | GovernanceWorkloadSideEffectPosture::ExternalIrreversible
        | GovernanceWorkloadSideEffectPosture::Unknown => GovernancePostureRequirement::approval(),
        GovernanceWorkloadSideEffectPosture::Ambiguous
        | GovernanceWorkloadSideEffectPosture::Unsupported => {
            GovernancePostureRequirement::denied()
        }
    }
}

fn strictest_requirement(
    explicit: GovernancePostureRequirement,
    inferred: GovernancePostureRequirement,
) -> GovernancePostureRequirement {
    GovernancePostureRequirement::new(
        explicit.execution().max(inferred.execution()),
        explicit.disclosure().max(inferred.disclosure()),
    )
}

fn collect_unknown_facts(
    input: &ProportionalGovernanceWorkloadAssessmentInput,
) -> BTreeSet<GovernanceAssessmentUnknownFact> {
    let mut unknown = BTreeSet::new();
    if input.action_class == GovernanceWorkloadActionClass::Unknown {
        unknown.insert(GovernanceAssessmentUnknownFact::ActionClass);
    }
    if input.authority == GovernanceWorkloadAuthorityPosture::Unknown {
        unknown.insert(GovernanceAssessmentUnknownFact::Authority);
    }
    if input.evidence_and_checks == GovernanceWorkloadEvidenceCheckPosture::Unknown {
        unknown.insert(GovernanceAssessmentUnknownFact::EvidenceAndChecks);
    }
    if input.sensitivity == GovernanceWorkloadSensitivity::Unknown {
        unknown.insert(GovernanceAssessmentUnknownFact::Sensitivity);
    }
    if input.side_effect == GovernanceWorkloadSideEffectPosture::Unknown {
        unknown.insert(GovernanceAssessmentUnknownFact::SideEffect);
    }
    unknown
}

fn compute_input_fingerprint(
    input: &ProportionalGovernanceWorkloadAssessmentInput,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(
        &mut hasher,
        "algorithm",
        GovernanceWorkloadAssessmentAlgorithm::V1.identifier(),
    );
    hash_field(
        &mut hasher,
        "definition_root",
        input.definition_root.as_str(),
    );
    hash_field(&mut hasher, "profile", input.profile.label());
    hash_requirement(&mut hasher, "workflow_minimum", input.workflow_minimum);
    hash_requirement(&mut hasher, "policy_minimum", input.policy_minimum);
    hash_field(
        &mut hasher,
        "action_class",
        action_label(input.action_class),
    );
    hash_field(&mut hasher, "authority", authority_label(input.authority));
    hash_requirement(&mut hasher, "authority_minimum", input.authority_minimum);
    hash_field(
        &mut hasher,
        "evidence_and_checks",
        evidence_label(input.evidence_and_checks),
    );
    hash_requirement(
        &mut hasher,
        "evidence_and_check_minimum",
        input.evidence_and_check_minimum,
    );
    hash_field(
        &mut hasher,
        "sensitivity",
        sensitivity_label(input.sensitivity),
    );
    hash_requirement(
        &mut hasher,
        "sensitivity_minimum",
        input.sensitivity_minimum,
    );
    hash_field(
        &mut hasher,
        "side_effect",
        side_effect_label(input.side_effect),
    );
    hash_requirement(
        &mut hasher,
        "side_effect_minimum",
        input.side_effect_minimum,
    );
    hash_requirement(&mut hasher, "runtime_escalation", input.runtime_escalation);
    hash_field(
        &mut hasher,
        "prior_execution",
        optional_execution_label(input.prior_execution),
    );
    hash_field(
        &mut hasher,
        "prior_disclosure",
        optional_disclosure_label(input.prior_disclosure),
    );
    match input.steward_minimum {
        Some(requirement) => hash_requirement(&mut hasher, "steward_minimum", requirement),
        None => hash_field(&mut hasher, "steward_minimum", "none"),
    }

    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_requirement(hasher: &mut Sha256, label: &str, requirement: GovernancePostureRequirement) {
    hash_field(
        hasher,
        &format!("{label}.execution"),
        execution_requirement_label(requirement.execution()),
    );
    hash_field(
        hasher,
        &format!("{label}.disclosure"),
        disclosure_obligation_label(requirement.disclosure()),
    );
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &str) {
    for part in [label.as_bytes(), value.as_bytes()] {
        let length = u64::try_from(part.len()).unwrap_or(u64::MAX);
        hasher.update(length.to_be_bytes());
        hasher.update(part);
    }
}

const fn action_label(value: GovernanceWorkloadActionClass) -> &'static str {
    match value {
        GovernanceWorkloadActionClass::ReadOnly => "read_only",
        GovernanceWorkloadActionClass::LocalMutation => "local_mutation",
        GovernanceWorkloadActionClass::ExternalMutation => "external_mutation",
        GovernanceWorkloadActionClass::Unknown => "unknown",
        GovernanceWorkloadActionClass::Unsupported => "unsupported",
    }
}

const fn authority_label(value: GovernanceWorkloadAuthorityPosture) -> &'static str {
    match value {
        GovernanceWorkloadAuthorityPosture::Sufficient => "sufficient",
        GovernanceWorkloadAuthorityPosture::ApprovalRequired => "approval_required",
        GovernanceWorkloadAuthorityPosture::Unavailable => "unavailable",
        GovernanceWorkloadAuthorityPosture::Unknown => "unknown",
    }
}

const fn evidence_label(value: GovernanceWorkloadEvidenceCheckPosture) -> &'static str {
    match value {
        GovernanceWorkloadEvidenceCheckPosture::Satisfied => "satisfied",
        GovernanceWorkloadEvidenceCheckPosture::OptionalUnavailable => "optional_unavailable",
        GovernanceWorkloadEvidenceCheckPosture::RequiredUnavailable => "required_unavailable",
        GovernanceWorkloadEvidenceCheckPosture::Failed => "failed",
        GovernanceWorkloadEvidenceCheckPosture::Unknown => "unknown",
    }
}

const fn sensitivity_label(value: GovernanceWorkloadSensitivity) -> &'static str {
    match value {
        GovernanceWorkloadSensitivity::Routine => "routine",
        GovernanceWorkloadSensitivity::Elevated => "elevated",
        GovernanceWorkloadSensitivity::Restricted => "restricted",
        GovernanceWorkloadSensitivity::Unknown => "unknown",
    }
}

const fn side_effect_label(value: GovernanceWorkloadSideEffectPosture) -> &'static str {
    match value {
        GovernanceWorkloadSideEffectPosture::None => "none",
        GovernanceWorkloadSideEffectPosture::LocalReversible => "local_reversible",
        GovernanceWorkloadSideEffectPosture::ExternalReversible => "external_reversible",
        GovernanceWorkloadSideEffectPosture::ExternalIrreversible => "external_irreversible",
        GovernanceWorkloadSideEffectPosture::Ambiguous => "ambiguous",
        GovernanceWorkloadSideEffectPosture::Unsupported => "unsupported",
        GovernanceWorkloadSideEffectPosture::Unknown => "unknown",
    }
}

const fn execution_requirement_label(value: GovernanceExecutionRequirement) -> &'static str {
    match value {
        GovernanceExecutionRequirement::Proceed => "proceed",
        GovernanceExecutionRequirement::RequireApproval => "require_approval",
        GovernanceExecutionRequirement::Denied => "denied",
        GovernanceExecutionRequirement::Unsupported => "unsupported",
    }
}

const fn disclosure_obligation_label(value: GovernanceDisclosureObligation) -> &'static str {
    match value {
        GovernanceDisclosureObligation::QuietAllowed => "quiet_allowed",
        GovernanceDisclosureObligation::VisibleRequired => "visible_required",
        GovernanceDisclosureObligation::Unsupported => "unsupported",
    }
}

const fn optional_execution_label(value: Option<GovernanceExecutionDisposition>) -> &'static str {
    match value {
        None => "none",
        Some(GovernanceExecutionDisposition::Proceed) => "proceed",
        Some(GovernanceExecutionDisposition::RequireApproval) => "require_approval",
        Some(GovernanceExecutionDisposition::Denied) => "denied",
    }
}

const fn optional_disclosure_label(value: Option<GovernanceDisclosureRequirement>) -> &'static str {
    match value {
        None => "none",
        Some(GovernanceDisclosureRequirement::Quiet) => "quiet",
        Some(GovernanceDisclosureRequirement::Visible) => "visible",
    }
}
