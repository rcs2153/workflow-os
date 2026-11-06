use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use sha2::{Digest, Sha256};

use crate::{SpecContentHash, WorkflowOsError};

const SOURCE_ID_MAX_BYTES: usize = 256;
const SOURCE_CLOSURE_DOMAIN: &str = "workflow-os/governed-obligation-source-closure/v1";
const OBLIGATION_ID_DOMAIN: &str = "workflow-os/governed-obligation-id/v1";
const INVENTORY_DOMAIN: &str = "workflow-os/governed-obligation-inventory/v1";
const SOURCE_TRACE_DOMAIN: &str = "workflow-os/governed-obligation-source-trace/v1";

/// Versioned private algorithm for the derived requirement inventory.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum GovernedObligationPlanAlgorithm {
    V1,
}

impl GovernedObligationPlanAlgorithm {
    const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

/// Closed source classes that may contribute one static requirement.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum GovernedObligationSourceKind {
    WorkflowStep,
    PolicyRule,
    RequiredContextContract,
    LocalCheckDeclarationSet,
    WorkReportContract,
}

impl GovernedObligationSourceKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::WorkflowStep => "workflow_step",
            Self::PolicyRule => "policy_rule",
            Self::RequiredContextContract => "required_context_contract",
            Self::LocalCheckDeclarationSet => "local_check_declaration_set",
            Self::WorkReportContract => "work_report_contract",
        }
    }
}

/// Closed static requirement classes. Runtime `SideEffects` are intentionally absent.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum GovernedObligationKind {
    Policy,
    Approval,
    RequiredContext,
    LocalCheck,
    WorkReportArtifact,
}

impl GovernedObligationKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Policy => "policy",
            Self::Approval => "approval",
            Self::RequiredContext => "required_context",
            Self::LocalCheck => "local_check",
            Self::WorkReportArtifact => "work_report_artifact",
        }
    }
}

/// Static requirement posture. These values never represent runtime satisfaction.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum GovernedObligationPosture {
    Mandatory,
    Optional,
    Unsupported,
    Unknown,
}

impl GovernedObligationPosture {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Mandatory => "mandatory",
            Self::Optional => "optional",
            Self::Unsupported => "unsupported",
            Self::Unknown => "unknown",
        }
    }
}

/// Closed references to existing authoritative evidence classes.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum GovernedObligationEvidenceClass {
    PolicyDecision,
    ApprovalDecisionProof,
    RequiredContextConsumption,
    LocalCheckResult,
    WorkReportArtifact,
}

impl GovernedObligationEvidenceClass {
    const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyDecision => "policy_decision",
            Self::ApprovalDecisionProof => "approval_decision_proof",
            Self::RequiredContextConsumption => "required_context_consumption",
            Self::LocalCheckResult => "local_check_result",
            Self::WorkReportArtifact => "work_report_artifact",
        }
    }
}

/// Private canonical source reference used only during deterministic derivation.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct GovernedObligationSource {
    kind: GovernedObligationSourceKind,
    identity: String,
    content_commitment: SpecContentHash,
}

impl GovernedObligationSource {
    fn derive(
        kind: GovernedObligationSourceKind,
        identity: impl Into<String>,
        content_commitment: SpecContentHash,
    ) -> Result<Self, WorkflowOsError> {
        let identity = identity.into();
        validate_source_identity(&identity)?;
        Ok(Self {
            kind,
            identity,
            content_commitment,
        })
    }
}

impl fmt::Debug for GovernedObligationSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernedObligationSource")
            .field("kind", &self.kind)
            .field("identity", &"[REDACTED]")
            .field("content_commitment", &"[REDACTED]")
            .finish()
    }
}

/// Private derivation input. Constructors are intentionally module-owned.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct DerivedGovernedObligation {
    source: GovernedObligationSource,
    kind: GovernedObligationKind,
    posture: GovernedObligationPosture,
    evidence_class: Option<GovernedObligationEvidenceClass>,
}

impl DerivedGovernedObligation {
    fn derive(
        source: GovernedObligationSource,
        kind: GovernedObligationKind,
        posture: GovernedObligationPosture,
        evidence_class: Option<GovernedObligationEvidenceClass>,
    ) -> Self {
        Self {
            source,
            kind,
            posture,
            evidence_class,
        }
    }
}

/// One deterministic non-authorizing requirement-index entry.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct GovernedObligationPlanEntry {
    obligation_id: SpecContentHash,
    source: GovernedObligationSource,
    kind: GovernedObligationKind,
    posture: GovernedObligationPosture,
    evidence_class: Option<GovernedObligationEvidenceClass>,
}

impl GovernedObligationPlanEntry {
    #[must_use]
    pub(crate) const fn obligation_id(&self) -> &SpecContentHash {
        &self.obligation_id
    }

    #[must_use]
    pub(crate) const fn kind(&self) -> GovernedObligationKind {
        self.kind
    }

    #[must_use]
    pub(crate) const fn posture(&self) -> GovernedObligationPosture {
        self.posture
    }

    #[must_use]
    pub(crate) const fn evidence_class(&self) -> Option<GovernedObligationEvidenceClass> {
        self.evidence_class
    }
}

impl fmt::Debug for GovernedObligationPlanEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernedObligationPlanEntry")
            .field("obligation_id", &"[REDACTED]")
            .field("source", &self.source)
            .field("kind", &self.kind)
            .field("posture", &self.posture)
            .field("evidence_class", &self.evidence_class)
            .finish()
    }
}

/// Private deterministic inventory of static requirements only.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct GovernedObligationPlan {
    algorithm: GovernedObligationPlanAlgorithm,
    source_closure_commitment: SpecContentHash,
    entries: Vec<GovernedObligationPlanEntry>,
    inventory_commitment: SpecContentHash,
}

impl GovernedObligationPlan {
    #[must_use]
    pub(crate) const fn algorithm(&self) -> GovernedObligationPlanAlgorithm {
        self.algorithm
    }

    #[must_use]
    pub(crate) const fn source_closure_commitment(&self) -> &SpecContentHash {
        &self.source_closure_commitment
    }

    #[must_use]
    pub(crate) fn entries(&self) -> &[GovernedObligationPlanEntry] {
        &self.entries
    }

    #[must_use]
    pub(crate) const fn inventory_commitment(&self) -> &SpecContentHash {
        &self.inventory_commitment
    }
}

impl fmt::Debug for GovernedObligationPlan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernedObligationPlan")
            .field("algorithm", &self.algorithm)
            .field("source_closure_commitment", &"[REDACTED]")
            .field("entry_count", &self.entries.len())
            .field("inventory_commitment", &"[REDACTED]")
            .finish()
    }
}

/// One redaction-safe trace from a derived obligation to its canonical source.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct GovernedObligationExplanationEntry {
    obligation_id: SpecContentHash,
    source_kind: GovernedObligationSourceKind,
    source_trace: SpecContentHash,
    kind: GovernedObligationKind,
    posture: GovernedObligationPosture,
    evidence_class: Option<GovernedObligationEvidenceClass>,
}

impl fmt::Debug for GovernedObligationExplanationEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernedObligationExplanationEntry")
            .field("obligation_id", &"[REDACTED]")
            .field("source_kind", &self.source_kind)
            .field("source_trace", &"[REDACTED]")
            .field("kind", &self.kind)
            .field("posture", &self.posture)
            .field("evidence_class", &self.evidence_class)
            .finish()
    }
}

/// Deterministic, non-authorizing explanation of one private obligation plan.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct GovernedObligationExplanation {
    algorithm: GovernedObligationPlanAlgorithm,
    source_closure_commitment: SpecContentHash,
    inventory_commitment: SpecContentHash,
    entries: Vec<GovernedObligationExplanationEntry>,
}

impl GovernedObligationExplanation {
    #[must_use]
    pub(crate) fn entries(&self) -> &[GovernedObligationExplanationEntry] {
        &self.entries
    }
}

impl fmt::Debug for GovernedObligationExplanation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernedObligationExplanation")
            .field("algorithm", &self.algorithm)
            .field("source_closure_commitment", &"[REDACTED]")
            .field("inventory_commitment", &"[REDACTED]")
            .field("entry_count", &self.entries.len())
            .finish()
    }
}

/// Independent oracle row. This is not constructed from a derived plan.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct GovernedObligationDifferentialExpectation {
    obligation_id: SpecContentHash,
    source_trace: SpecContentHash,
    kind: GovernedObligationKind,
    posture: GovernedObligationPosture,
    evidence_class: Option<GovernedObligationEvidenceClass>,
}

/// Existing authoritative Core decision used as the observed side of review.
///
/// The variants carry canonical decision types rather than restating their
/// semantics in a second outcome vocabulary.
#[derive(Clone, Eq, PartialEq)]
pub(crate) enum GovernedObligationAuthoritativeDecision {
    Policy(crate::PolicyDecision),
    WorkloadAssessment(crate::ProportionalGovernanceWorkloadAssessment),
    ValidationFailure(WorkflowOsError),
}

/// One independently obtained authoritative Core decision.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct GovernedObligationObservedOutcome {
    obligation_id: SpecContentHash,
    decision: GovernedObligationAuthoritativeDecision,
}

/// Independent review observation. Runtime `SideEffects` remain facts, not static
/// obligations, and therefore cannot add entries to the compared plan.
#[derive(Clone, Eq, PartialEq)]
pub(crate) enum GovernedObligationDifferentialObservation {
    AuthoritativeDecision(Box<GovernedObligationObservedOutcome>),
    RuntimeSideEffectObserved,
}

/// Non-authorizing evidence that independent expectations and observations agree.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GovernedObligationDifferentialValidation {
    compared_obligations: usize,
    observed_runtime_side_effects: usize,
}

/// Projects stable explanation rows without exposing raw source identities.
pub(crate) fn explain_governed_obligation_plan(
    plan: &GovernedObligationPlan,
) -> Result<GovernedObligationExplanation, WorkflowOsError> {
    let entries = plan
        .entries
        .iter()
        .map(|entry| {
            Ok(GovernedObligationExplanationEntry {
                obligation_id: entry.obligation_id.clone(),
                source_kind: entry.source.kind,
                source_trace: compute_source_trace(&entry.source)?,
                kind: entry.kind,
                posture: entry.posture,
                evidence_class: entry.evidence_class,
            })
        })
        .collect::<Result<Vec<_>, WorkflowOsError>>()?;

    Ok(GovernedObligationExplanation {
        algorithm: plan.algorithm,
        source_closure_commitment: plan.source_closure_commitment.clone(),
        inventory_commitment: plan.inventory_commitment.clone(),
        entries,
    })
}

/// Fails closed when a derived explanation, independent oracle, and observed
/// canonical outcomes disagree. It never computes or selects an outcome.
pub(crate) fn validate_governed_obligation_differential(
    plan: &GovernedObligationPlan,
    explanation: &GovernedObligationExplanation,
    expected_source_closure: &SpecContentHash,
    expected_inventory: &SpecContentHash,
    expectations: &[GovernedObligationDifferentialExpectation],
    observations: &[GovernedObligationDifferentialObservation],
) -> Result<GovernedObligationDifferentialValidation, WorkflowOsError> {
    if explanation.algorithm != plan.algorithm
        || explanation.source_closure_commitment != plan.source_closure_commitment
        || explanation.inventory_commitment != plan.inventory_commitment
    {
        return Err(model_error(
            "explanation_stale",
            "governed obligation explanation is not bound to the current plan",
        ));
    }
    if &plan.source_closure_commitment != expected_source_closure {
        return Err(model_error(
            "source_closure_disagreement",
            "governed obligation source closure disagrees with the independent expectation",
        ));
    }
    if &plan.inventory_commitment != expected_inventory {
        return Err(model_error(
            "inventory_disagreement",
            "governed obligation inventory disagrees with the independent expectation",
        ));
    }
    if !is_strictly_sorted_by_obligation_id(&explanation.entries) {
        return Err(model_error(
            "explanation_order_unstable",
            "governed obligation explanation ordering is not canonical",
        ));
    }

    let expected_by_id = unique_expectations(expectations)?;
    let observed_decisions = observations
        .iter()
        .filter_map(|observation| match observation {
            GovernedObligationDifferentialObservation::AuthoritativeDecision(outcome) => {
                Some(outcome.as_ref())
            }
            GovernedObligationDifferentialObservation::RuntimeSideEffectObserved => None,
        });
    let observed_by_id = unique_observed_decisions(observed_decisions)?;
    let observed_runtime_side_effects = observations
        .iter()
        .filter(|observation| {
            matches!(
                observation,
                GovernedObligationDifferentialObservation::RuntimeSideEffectObserved
            )
        })
        .count();
    if expected_by_id.len() != explanation.entries.len() {
        return Err(model_error(
            "obligation_set_disagreement",
            "governed obligation explanation and independent expectation sets differ",
        ));
    }

    for entry in &explanation.entries {
        let expected = expected_by_id.get(&entry.obligation_id).ok_or_else(|| {
            model_error(
                "obligation_set_disagreement",
                "governed obligation explanation contains an unexpected obligation",
            )
        })?;
        if expected.source_trace != entry.source_trace
            || expected.kind != entry.kind
            || expected.posture != entry.posture
            || expected.evidence_class != entry.evidence_class
        {
            return Err(model_error(
                "explanation_disagreement",
                "governed obligation explanation disagrees with the independent expectation",
            ));
        }
        let observed = observed_by_id.get(&entry.obligation_id).ok_or_else(|| {
            model_error(
                "authoritative_decision_missing",
                "governed obligation is missing an authoritative Core decision",
            )
        })?;
        validate_authoritative_observation(expected, observed)?;
    }
    if observed_by_id.len() != explanation.entries.len() {
        return Err(model_error(
            "authoritative_decision_unexpected",
            "authoritative Core decisions contain an obligation outside the derived plan",
        ));
    }

    Ok(GovernedObligationDifferentialValidation {
        compared_obligations: explanation.entries.len(),
        observed_runtime_side_effects,
    })
}

fn validate_authoritative_observation(
    expected: &GovernedObligationDifferentialExpectation,
    observed: &GovernedObligationObservedOutcome,
) -> Result<(), WorkflowOsError> {
    match (
        expected.kind,
        expected.posture,
        expected.evidence_class,
        &observed.decision,
    ) {
        (
            GovernedObligationKind::Approval,
            GovernedObligationPosture::Mandatory,
            Some(GovernedObligationEvidenceClass::ApprovalDecisionProof),
            GovernedObligationAuthoritativeDecision::Policy(decision),
        ) if decision.requires_approval && decision.allowed => Ok(()),
        (
            GovernedObligationKind::LocalCheck,
            GovernedObligationPosture::Mandatory,
            Some(GovernedObligationEvidenceClass::LocalCheckResult),
            GovernedObligationAuthoritativeDecision::WorkloadAssessment(assessment),
        ) if assessment.decision().execution() == crate::GovernanceExecutionDisposition::Denied
            && assessment
                .decision()
                .reasons()
                .contains(&crate::GovernanceDecisionReason::EvidenceOrCheckRequirement) =>
        {
            Ok(())
        }
        (
            GovernedObligationKind::LocalCheck,
            GovernedObligationPosture::Unsupported,
            Some(GovernedObligationEvidenceClass::LocalCheckResult),
            GovernedObligationAuthoritativeDecision::ValidationFailure(error),
        ) if error.kind() == crate::WorkflowOsErrorKind::Unsupported
            && error.code() == "governance.proportional.requirement.unsupported" =>
        {
            Ok(())
        }
        _ => Err(model_error(
            "authoritative_decision_disagreement",
            "authoritative Core decision does not satisfy the independently expected obligation",
        )),
    }
}

/// Builds the private static requirement index from already-derived canonical sources.
pub(crate) fn build_governed_obligation_plan(
    canonical_sources: Vec<GovernedObligationSource>,
    obligations: Vec<DerivedGovernedObligation>,
) -> Result<GovernedObligationPlan, WorkflowOsError> {
    if canonical_sources.is_empty() {
        return Err(model_error(
            "source_closure_empty",
            "governed obligation source closure cannot be empty",
        ));
    }
    if obligations.is_empty() {
        return Err(model_error(
            "empty",
            "governed obligation inventory cannot be empty",
        ));
    }

    let algorithm = GovernedObligationPlanAlgorithm::V1;
    let mut source_commitments =
        BTreeMap::<(GovernedObligationSourceKind, String), SpecContentHash>::new();
    let mut entries = Vec::with_capacity(obligations.len());
    let mut obligation_ids = BTreeSet::new();

    for source in canonical_sources {
        let source_key = (source.kind, source.identity);
        if let Some(existing) = source_commitments.get(&source_key) {
            if existing != &source.content_commitment {
                return Err(model_error(
                    "source_conflict",
                    "governed obligation source identity has conflicting commitments",
                ));
            }
        } else {
            source_commitments.insert(source_key, source.content_commitment);
        }
    }

    for obligation in obligations {
        let source_key = (obligation.source.kind, obligation.source.identity.clone());
        if source_commitments.get(&source_key) != Some(&obligation.source.content_commitment) {
            return Err(model_error(
                "source_outside_closure",
                "governed obligation source is not bound by the canonical source closure",
            ));
        }

        let obligation_id = compute_obligation_id(&obligation.source, obligation.kind)?;
        if !obligation_ids.insert(obligation_id.clone()) {
            return Err(model_error(
                "duplicate_obligation",
                "governed obligation inventory repeats one derived obligation",
            ));
        }
        entries.push(GovernedObligationPlanEntry {
            obligation_id,
            source: obligation.source,
            kind: obligation.kind,
            posture: obligation.posture,
            evidence_class: obligation.evidence_class,
        });
    }

    entries.sort_by(|left, right| left.obligation_id.cmp(&right.obligation_id));
    let source_closure_commitment = compute_source_closure_commitment(&source_commitments)?;
    let inventory_commitment =
        compute_inventory_commitment(algorithm, &source_closure_commitment, &entries)?;

    Ok(GovernedObligationPlan {
        algorithm,
        source_closure_commitment,
        entries,
        inventory_commitment,
    })
}

fn compute_obligation_id(
    source: &GovernedObligationSource,
    kind: GovernedObligationKind,
) -> Result<SpecContentHash, WorkflowOsError> {
    hash_segments([
        OBLIGATION_ID_DOMAIN,
        source.kind.as_str(),
        &source.identity,
        source.content_commitment.as_str(),
        kind.as_str(),
    ])
}

fn compute_source_trace(
    source: &GovernedObligationSource,
) -> Result<SpecContentHash, WorkflowOsError> {
    hash_segments([
        SOURCE_TRACE_DOMAIN,
        source.kind.as_str(),
        &source.identity,
        source.content_commitment.as_str(),
    ])
}

fn is_strictly_sorted_by_obligation_id(entries: &[GovernedObligationExplanationEntry]) -> bool {
    entries
        .windows(2)
        .all(|pair| pair[0].obligation_id < pair[1].obligation_id)
}

fn unique_expectations(
    expectations: &[GovernedObligationDifferentialExpectation],
) -> Result<BTreeMap<SpecContentHash, &GovernedObligationDifferentialExpectation>, WorkflowOsError>
{
    let mut by_id = BTreeMap::new();
    for expectation in expectations {
        if by_id
            .insert(expectation.obligation_id.clone(), expectation)
            .is_some()
        {
            return Err(model_error(
                "expectation_duplicate",
                "independent governed obligation expectations contain a duplicate identity",
            ));
        }
    }
    Ok(by_id)
}

fn unique_observed_decisions<'a>(
    outcomes: impl IntoIterator<Item = &'a GovernedObligationObservedOutcome>,
) -> Result<BTreeMap<SpecContentHash, &'a GovernedObligationObservedOutcome>, WorkflowOsError> {
    let mut by_id = BTreeMap::new();
    for outcome in outcomes {
        if by_id
            .insert(outcome.obligation_id.clone(), outcome)
            .is_some()
        {
            return Err(model_error(
                "authoritative_decision_duplicate",
                "authoritative Core decisions contain a duplicate obligation identity",
            ));
        }
    }
    Ok(by_id)
}

fn compute_source_closure_commitment(
    sources: &BTreeMap<(GovernedObligationSourceKind, String), SpecContentHash>,
) -> Result<SpecContentHash, WorkflowOsError> {
    let mut hasher = Sha256::new();
    update_segment(&mut hasher, SOURCE_CLOSURE_DOMAIN)?;
    for ((kind, identity), commitment) in sources {
        update_segment(&mut hasher, kind.as_str())?;
        update_segment(&mut hasher, identity)?;
        update_segment(&mut hasher, commitment.as_str())?;
    }
    Ok(SpecContentHash::from_bytes(hasher.finalize()))
}

fn compute_inventory_commitment(
    algorithm: GovernedObligationPlanAlgorithm,
    source_closure_commitment: &SpecContentHash,
    entries: &[GovernedObligationPlanEntry],
) -> Result<SpecContentHash, WorkflowOsError> {
    let mut hasher = Sha256::new();
    update_segment(&mut hasher, INVENTORY_DOMAIN)?;
    update_segment(&mut hasher, algorithm.as_str())?;
    update_segment(&mut hasher, source_closure_commitment.as_str())?;
    for entry in entries {
        update_segment(&mut hasher, entry.obligation_id.as_str())?;
        update_segment(&mut hasher, entry.kind.as_str())?;
        update_segment(&mut hasher, entry.posture.as_str())?;
        update_segment(
            &mut hasher,
            entry
                .evidence_class
                .map_or("none", GovernedObligationEvidenceClass::as_str),
        )?;
    }
    Ok(SpecContentHash::from_bytes(hasher.finalize()))
}

fn hash_segments<'a>(
    segments: impl IntoIterator<Item = &'a str>,
) -> Result<SpecContentHash, WorkflowOsError> {
    let mut hasher = Sha256::new();
    for segment in segments {
        update_segment(&mut hasher, segment)?;
    }
    Ok(SpecContentHash::from_bytes(hasher.finalize()))
}

fn update_segment(hasher: &mut Sha256, value: &str) -> Result<(), WorkflowOsError> {
    let length = u64::try_from(value.len()).map_err(|_| {
        model_error(
            "commitment_input_too_large",
            "governed obligation commitment input is too large",
        )
    })?;
    hasher.update(length.to_be_bytes());
    hasher.update(value.as_bytes());
    Ok(())
}

fn validate_source_identity(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() || value.len() > SOURCE_ID_MAX_BYTES {
        return Err(model_error(
            "source_identity_invalid",
            "governed obligation source identity is invalid",
        ));
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
    {
        return Err(model_error(
            "source_identity_invalid",
            "governed obligation source identity is invalid",
        ));
    }
    Ok(())
}

fn model_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("governed_obligation_plan.{code}"), message)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic)]

    use super::*;

    fn success<T>(result: Result<T, WorkflowOsError>) -> T {
        result.unwrap_or_else(|error| panic!("unexpected error: {error}"))
    }

    fn failure<T>(result: Result<T, WorkflowOsError>) -> WorkflowOsError {
        match result {
            Ok(_) => panic!("expected an error"),
            Err(error) => error,
        }
    }

    fn source(
        kind: GovernedObligationSourceKind,
        identity: &str,
        commitment_seed: &str,
    ) -> GovernedObligationSource {
        success(GovernedObligationSource::derive(
            kind,
            identity,
            SpecContentHash::from_text(commitment_seed),
        ))
    }

    fn obligation(
        source: GovernedObligationSource,
        kind: GovernedObligationKind,
        posture: GovernedObligationPosture,
        evidence_class: Option<GovernedObligationEvidenceClass>,
    ) -> DerivedGovernedObligation {
        DerivedGovernedObligation::derive(source, kind, posture, evidence_class)
    }

    fn fixture() -> (
        Vec<GovernedObligationSource>,
        Vec<DerivedGovernedObligation>,
    ) {
        let approval_source = source(
            GovernedObligationSourceKind::WorkflowStep,
            "workflow/build/step/review",
            "workflow-v1",
        );
        let check_source = source(
            GovernedObligationSourceKind::LocalCheckDeclarationSet,
            "workflow/build/step/test",
            "check-set-v1",
        );
        let obligations = vec![
            obligation(
                approval_source.clone(),
                GovernedObligationKind::Approval,
                GovernedObligationPosture::Mandatory,
                Some(GovernedObligationEvidenceClass::ApprovalDecisionProof),
            ),
            obligation(
                check_source.clone(),
                GovernedObligationKind::LocalCheck,
                GovernedObligationPosture::Mandatory,
                Some(GovernedObligationEvidenceClass::LocalCheckResult),
            ),
        ];
        (vec![approval_source, check_source], obligations)
    }

    fn fixed_hash(value: &str) -> SpecContentHash {
        success(SpecContentHash::new(value))
    }

    fn independent_expectations(
        check_posture: GovernedObligationPosture,
    ) -> Vec<GovernedObligationDifferentialExpectation> {
        vec![
            GovernedObligationDifferentialExpectation {
                obligation_id: fixed_hash(
                    "60071ad3666e43408c02ec79742f50a89218043a7a4fa438d133966a0ba1e9ba",
                ),
                source_trace: fixed_hash(
                    "cb27124410714454269f57dbfc47f46f65673d0bae2c44cf7612cfc8905447cc",
                ),
                kind: GovernedObligationKind::LocalCheck,
                posture: check_posture,
                evidence_class: Some(GovernedObligationEvidenceClass::LocalCheckResult),
            },
            GovernedObligationDifferentialExpectation {
                obligation_id: fixed_hash(
                    "6bd9aeb9bb3c9cbf70fcf7d9b5f1fe65163d1865f90f25cdbdc4ad940836d4b4",
                ),
                source_trace: fixed_hash(
                    "432788aea804b831b5aec3868c7b7ddeceb8e4d03394bf88e806a19bb0505f10",
                ),
                kind: GovernedObligationKind::Approval,
                posture: GovernedObligationPosture::Mandatory,
                evidence_class: Some(GovernedObligationEvidenceClass::ApprovalDecisionProof),
            },
        ]
    }

    fn authoritative_approval_decision() -> crate::PolicyDecision {
        let mut policy_effects = crate::PolicyEffectSet::default();
        policy_effects.insert(crate::PolicyEffect::AllowLocal);
        policy_effects.insert(crate::PolicyEffect::RequireApproval);
        crate::ConservativePolicyEngine::new().evaluate(&crate::PolicyEvaluationContext {
            action: crate::Action::InvokeSkill,
            capabilities: vec![crate::Capability::LocalRead],
            actor: Some(success(crate::ActorId::new("system/obligation-review"))),
            workflow_id: Some(success(crate::WorkflowId::new(
                "workflow/obligation-review",
            ))),
            run_id: Some(success(crate::WorkflowRunId::new("run/obligation-review"))),
            step_id: None,
            skill_id: None,
            autonomy_level: Some(crate::AutonomyLevel::Level1Assistive),
            approval_sensitivity: Some(crate::ApprovalSensitivity::Low),
            has_approval_policy: true,
            policy_effects,
            adapter_id: None,
            correlation_id: Some(success(crate::CorrelationId::new(
                "correlation/obligation-review",
            ))),
        })
    }

    fn authoritative_check_decision(
        check_posture: GovernedObligationPosture,
    ) -> GovernedObligationAuthoritativeDecision {
        let evidence_and_check_minimum = if check_posture == GovernedObligationPosture::Unsupported
        {
            crate::GovernancePostureRequirement::new(
                crate::GovernanceExecutionRequirement::Unsupported,
                crate::GovernanceDisclosureObligation::QuietAllowed,
            )
        } else {
            crate::GovernancePostureRequirement::quiet()
        };
        let result = crate::assess_proportional_governance_workload(
            &crate::ProportionalGovernanceWorkloadAssessmentInput {
                definition_root: SpecContentHash::from_text("obligation-review-definition"),
                profile: crate::GovernanceStrictnessProfile::ObserveAndReport,
                workflow_minimum: crate::GovernancePostureRequirement::quiet(),
                policy_minimum: crate::GovernancePostureRequirement::quiet(),
                action_class: crate::GovernanceWorkloadActionClass::ReadOnly,
                authority: crate::GovernanceWorkloadAuthorityPosture::Sufficient,
                authority_minimum: crate::GovernancePostureRequirement::quiet(),
                evidence_and_checks:
                    crate::GovernanceWorkloadEvidenceCheckPosture::RequiredUnavailable,
                evidence_and_check_minimum,
                sensitivity: crate::GovernanceWorkloadSensitivity::Routine,
                sensitivity_minimum: crate::GovernancePostureRequirement::quiet(),
                side_effect: crate::GovernanceWorkloadSideEffectPosture::None,
                side_effect_minimum: crate::GovernancePostureRequirement::quiet(),
                runtime_escalation: crate::GovernancePostureRequirement::quiet(),
                prior_execution: None,
                prior_disclosure: None,
                steward_minimum: None,
            },
        );
        match result {
            Ok(assessment) => {
                GovernedObligationAuthoritativeDecision::WorkloadAssessment(assessment)
            }
            Err(error) => GovernedObligationAuthoritativeDecision::ValidationFailure(error),
        }
    }

    fn independent_observations(
        check_posture: GovernedObligationPosture,
    ) -> Vec<GovernedObligationDifferentialObservation> {
        vec![
            GovernedObligationDifferentialObservation::AuthoritativeDecision(Box::new(
                GovernedObligationObservedOutcome {
                    obligation_id: fixed_hash(
                        "60071ad3666e43408c02ec79742f50a89218043a7a4fa438d133966a0ba1e9ba",
                    ),
                    decision: authoritative_check_decision(check_posture),
                },
            )),
            GovernedObligationDifferentialObservation::AuthoritativeDecision(Box::new(
                GovernedObligationObservedOutcome {
                    obligation_id: fixed_hash(
                        "6bd9aeb9bb3c9cbf70fcf7d9b5f1fe65163d1865f90f25cdbdc4ad940836d4b4",
                    ),
                    decision: GovernedObligationAuthoritativeDecision::Policy(
                        authoritative_approval_decision(),
                    ),
                },
            )),
        ]
    }

    #[test]
    fn inventory_is_deterministic_across_input_order() {
        let (sources, obligations) = fixture();
        let forward = success(build_governed_obligation_plan(
            sources.clone(),
            obligations.clone(),
        ));
        let mut reversed_sources = sources;
        reversed_sources.reverse();
        let mut reversed_obligations = obligations;
        reversed_obligations.reverse();
        let reversed = success(build_governed_obligation_plan(
            reversed_sources,
            reversed_obligations,
        ));

        assert_eq!(forward.algorithm(), GovernedObligationPlanAlgorithm::V1);
        assert_eq!(forward.entries(), reversed.entries());
        assert_eq!(
            forward.source_closure_commitment(),
            reversed.source_closure_commitment()
        );
        assert_eq!(
            forward.inventory_commitment(),
            reversed.inventory_commitment()
        );
    }

    #[test]
    fn v1_fixed_width_commitment_vectors_are_stable() {
        let (sources, obligations) = fixture();
        let plan = success(build_governed_obligation_plan(sources, obligations));

        assert_eq!(
            plan.source_closure_commitment().as_str(),
            "422d11ca31afbbebf82884f4f0998f175bc0fa54f70c5816e1dcf1621ab697e2"
        );
        assert_eq!(
            plan.entries()[0].obligation_id().as_str(),
            "60071ad3666e43408c02ec79742f50a89218043a7a4fa438d133966a0ba1e9ba"
        );
        assert_eq!(
            plan.entries()[1].obligation_id().as_str(),
            "6bd9aeb9bb3c9cbf70fcf7d9b5f1fe65163d1865f90f25cdbdc4ad940836d4b4"
        );
        assert_eq!(
            plan.inventory_commitment().as_str(),
            "39374d695aa0229dcaffb357d8f688a7c1993ba77c57c2b6fac7467866558a4e"
        );
    }

    #[test]
    fn omitted_obligation_changes_inventory_but_not_independent_source_closure() {
        let (sources, obligations) = fixture();
        let complete = success(build_governed_obligation_plan(
            sources.clone(),
            obligations.clone(),
        ));
        let incomplete = success(build_governed_obligation_plan(
            sources,
            vec![obligations[0].clone()],
        ));

        assert_eq!(
            complete.source_closure_commitment(),
            incomplete.source_closure_commitment()
        );
        assert_ne!(
            complete.inventory_commitment(),
            incomplete.inventory_commitment()
        );
    }

    #[test]
    fn omitted_canonical_source_changes_source_closure_commitment() {
        let (sources, obligations) = fixture();
        let complete = success(build_governed_obligation_plan(sources, obligations.clone()));
        let retained_source = obligations[0].source.clone();
        let reduced = success(build_governed_obligation_plan(
            vec![retained_source],
            vec![obligations[0].clone()],
        ));

        assert_ne!(
            complete.source_closure_commitment(),
            reduced.source_closure_commitment()
        );
    }

    #[test]
    fn changed_source_commitment_changes_derived_identity_and_roots() {
        let (sources, obligations) = fixture();
        let original = success(build_governed_obligation_plan(sources, obligations));
        let changed_source = source(
            GovernedObligationSourceKind::WorkflowStep,
            "workflow/build/step/review",
            "workflow-v2",
        );
        let changed = success(build_governed_obligation_plan(
            vec![changed_source.clone()],
            vec![obligation(
                changed_source,
                GovernedObligationKind::Approval,
                GovernedObligationPosture::Mandatory,
                Some(GovernedObligationEvidenceClass::ApprovalDecisionProof),
            )],
        ));

        assert_ne!(
            original.entries()[0].obligation_id(),
            changed.entries()[0].obligation_id()
        );
        assert_ne!(
            original.source_closure_commitment(),
            changed.source_closure_commitment()
        );
        assert_ne!(
            original.inventory_commitment(),
            changed.inventory_commitment()
        );
    }

    #[test]
    fn conflicting_commitments_for_one_source_fail_closed() {
        let obligations = vec![
            obligation(
                source(
                    GovernedObligationSourceKind::PolicyRule,
                    "policy/default/rule/approval",
                    "policy-v1",
                ),
                GovernedObligationKind::Policy,
                GovernedObligationPosture::Mandatory,
                Some(GovernedObligationEvidenceClass::PolicyDecision),
            ),
            obligation(
                source(
                    GovernedObligationSourceKind::PolicyRule,
                    "policy/default/rule/approval",
                    "policy-v2",
                ),
                GovernedObligationKind::Approval,
                GovernedObligationPosture::Mandatory,
                Some(GovernedObligationEvidenceClass::ApprovalDecisionProof),
            ),
        ];

        let error = failure(build_governed_obligation_plan(
            vec![obligations[0].source.clone(), obligations[1].source.clone()],
            obligations,
        ));
        assert_eq!(error.code(), "governed_obligation_plan.source_conflict");
    }

    #[test]
    fn repeated_derived_obligation_fails_closed() {
        let (sources, obligations) = fixture();
        let repeated = obligations[0].clone();
        let error = failure(build_governed_obligation_plan(
            sources,
            vec![repeated.clone(), repeated],
        ));
        assert_eq!(
            error.code(),
            "governed_obligation_plan.duplicate_obligation"
        );
    }

    #[test]
    fn empty_or_malformed_sources_cannot_form_a_plan() {
        let empty_sources = failure(build_governed_obligation_plan(Vec::new(), fixture().1));
        assert_eq!(
            empty_sources.code(),
            "governed_obligation_plan.source_closure_empty"
        );

        let empty_obligations = failure(build_governed_obligation_plan(fixture().0, Vec::new()));
        assert_eq!(empty_obligations.code(), "governed_obligation_plan.empty");

        let malformed = failure(GovernedObligationSource::derive(
            GovernedObligationSourceKind::RequiredContextContract,
            "contains spaces",
            SpecContentHash::from_text("context-v1"),
        ));
        assert_eq!(
            malformed.code(),
            "governed_obligation_plan.source_identity_invalid"
        );
    }

    #[test]
    fn obligation_outside_independent_source_closure_fails_closed() {
        let (sources, obligations) = fixture();
        let error = failure(build_governed_obligation_plan(
            vec![sources[0].clone()],
            vec![obligations[1].clone()],
        ));
        assert_eq!(
            error.code(),
            "governed_obligation_plan.source_outside_closure"
        );
    }

    #[test]
    fn debug_output_redacts_source_identity_and_commitments() {
        let report_source = source(
            GovernedObligationSourceKind::WorkReportContract,
            "sensitive/report/source",
            "sensitive-report-commitment",
        );
        let plan = success(build_governed_obligation_plan(
            vec![report_source.clone()],
            vec![obligation(
                report_source,
                GovernedObligationKind::WorkReportArtifact,
                GovernedObligationPosture::Optional,
                Some(GovernedObligationEvidenceClass::WorkReportArtifact),
            )],
        ));

        let debug = format!("{plan:?} {:?}", plan.entries()[0]);
        assert!(!debug.contains("sensitive/report/source"));
        assert!(!debug.contains(plan.source_closure_commitment().as_str()));
        assert!(!debug.contains(plan.inventory_commitment().as_str()));
        assert!(debug.contains("entry_count"));
        assert!(debug.contains("WorkReportArtifact"));
    }

    #[test]
    fn closed_static_postures_and_evidence_classes_are_preserved() {
        let context_source = source(
            GovernedObligationSourceKind::RequiredContextContract,
            "context/repository-metadata",
            "context-v1",
        );
        let plan = success(build_governed_obligation_plan(
            vec![context_source.clone()],
            vec![obligation(
                context_source,
                GovernedObligationKind::RequiredContext,
                GovernedObligationPosture::Unsupported,
                Some(GovernedObligationEvidenceClass::RequiredContextConsumption),
            )],
        ));

        assert_eq!(
            plan.entries()[0].kind(),
            GovernedObligationKind::RequiredContext
        );
        assert_eq!(
            plan.entries()[0].posture(),
            GovernedObligationPosture::Unsupported
        );
        assert_eq!(
            plan.entries()[0].evidence_class(),
            Some(GovernedObligationEvidenceClass::RequiredContextConsumption)
        );
    }

    #[test]
    fn explanation_is_deterministic_redacted_and_source_traceable() {
        let (sources, obligations) = fixture();
        let forward = success(build_governed_obligation_plan(
            sources.clone(),
            obligations.clone(),
        ));
        let mut reversed_sources = sources;
        reversed_sources.reverse();
        let mut reversed_obligations = obligations;
        reversed_obligations.reverse();
        let reversed = success(build_governed_obligation_plan(
            reversed_sources,
            reversed_obligations,
        ));

        let forward_explanation = success(explain_governed_obligation_plan(&forward));
        let reversed_explanation = success(explain_governed_obligation_plan(&reversed));
        assert_eq!(forward_explanation, reversed_explanation);
        assert_eq!(
            forward_explanation.entries()[0].source_trace.as_str(),
            "cb27124410714454269f57dbfc47f46f65673d0bae2c44cf7612cfc8905447cc"
        );
        assert_eq!(
            forward_explanation.entries()[1].source_trace.as_str(),
            "432788aea804b831b5aec3868c7b7ddeceb8e4d03394bf88e806a19bb0505f10"
        );
        let debug = format!(
            "{forward_explanation:?} {:?}",
            forward_explanation.entries()[0]
        );
        assert!(!debug.contains("workflow/build/step/review"));
        assert!(!debug.contains(forward_explanation.entries()[0].source_trace.as_str()));
    }

    #[test]
    fn independently_specified_obligations_and_outcomes_agree() {
        let (sources, obligations) = fixture();
        let plan = success(build_governed_obligation_plan(sources, obligations));
        let explanation = success(explain_governed_obligation_plan(&plan));
        let validation = success(validate_governed_obligation_differential(
            &plan,
            &explanation,
            &fixed_hash("422d11ca31afbbebf82884f4f0998f175bc0fa54f70c5816e1dcf1621ab697e2"),
            &fixed_hash("39374d695aa0229dcaffb357d8f688a7c1993ba77c57c2b6fac7467866558a4e"),
            &independent_expectations(GovernedObligationPosture::Mandatory),
            &independent_observations(GovernedObligationPosture::Mandatory),
        ));

        assert_eq!(validation.compared_obligations, 2);
        assert_eq!(validation.observed_runtime_side_effects, 0);
    }

    #[test]
    fn omitted_gate_is_detected_even_when_runtime_outcomes_are_complete() {
        let (sources, obligations) = fixture();
        let incomplete = success(build_governed_obligation_plan(
            sources,
            vec![obligations[0].clone()],
        ));
        let explanation = success(explain_governed_obligation_plan(&incomplete));
        let observations = independent_observations(GovernedObligationPosture::Mandatory);
        assert!(matches!(
            &observations[0],
            GovernedObligationDifferentialObservation::AuthoritativeDecision(outcome)
                if matches!(
                    &outcome.decision,
                    GovernedObligationAuthoritativeDecision::WorkloadAssessment(assessment)
                        if assessment.decision().execution()
                            == crate::GovernanceExecutionDisposition::Denied
                            && assessment.decision().reasons().contains(
                                &crate::GovernanceDecisionReason::EvidenceOrCheckRequirement
                            )
                )
        ));
        assert!(matches!(
            &observations[1],
            GovernedObligationDifferentialObservation::AuthoritativeDecision(outcome)
                if matches!(
                    &outcome.decision,
                    GovernedObligationAuthoritativeDecision::Policy(decision)
                        if decision.allowed && decision.requires_approval
                )
        ));
        let error = failure(validate_governed_obligation_differential(
            &incomplete,
            &explanation,
            &fixed_hash("422d11ca31afbbebf82884f4f0998f175bc0fa54f70c5816e1dcf1621ab697e2"),
            &fixed_hash("39374d695aa0229dcaffb357d8f688a7c1993ba77c57c2b6fac7467866558a4e"),
            &independent_expectations(GovernedObligationPosture::Mandatory),
            &observations,
        ));

        assert_eq!(
            error.code(),
            "governed_obligation_plan.inventory_disagreement"
        );
    }

    #[test]
    fn unsupported_requirement_remains_disclosed() {
        let (sources, mut obligations) = fixture();
        obligations[1].posture = GovernedObligationPosture::Unsupported;
        let plan = success(build_governed_obligation_plan(sources, obligations));
        let explanation = success(explain_governed_obligation_plan(&plan));
        let validation = success(validate_governed_obligation_differential(
            &plan,
            &explanation,
            &fixed_hash("422d11ca31afbbebf82884f4f0998f175bc0fa54f70c5816e1dcf1621ab697e2"),
            &fixed_hash("476fe69fb7bb27e0978caea3c01c5a56f4fafcb1b13e99ae97918271c029bb55"),
            &independent_expectations(GovernedObligationPosture::Unsupported),
            &independent_observations(GovernedObligationPosture::Unsupported),
        ));

        assert_eq!(validation.compared_obligations, 2);
        assert_eq!(
            explanation.entries()[0].posture,
            GovernedObligationPosture::Unsupported
        );
    }

    #[test]
    fn runtime_side_effect_observation_cannot_manufacture_static_obligation() {
        let (sources, obligations) = fixture();
        let plan = success(build_governed_obligation_plan(sources, obligations));
        let explanation = success(explain_governed_obligation_plan(&plan));
        let mut observations = independent_observations(GovernedObligationPosture::Mandatory);
        observations.push(GovernedObligationDifferentialObservation::RuntimeSideEffectObserved);
        let validation = success(validate_governed_obligation_differential(
            &plan,
            &explanation,
            &fixed_hash("422d11ca31afbbebf82884f4f0998f175bc0fa54f70c5816e1dcf1621ab697e2"),
            &fixed_hash("39374d695aa0229dcaffb357d8f688a7c1993ba77c57c2b6fac7467866558a4e"),
            &independent_expectations(GovernedObligationPosture::Mandatory),
            &observations,
        ));

        assert_eq!(validation.compared_obligations, 2);
        assert_eq!(validation.observed_runtime_side_effects, 1);
        assert_eq!(explanation.entries().len(), 2);
    }

    #[test]
    fn stale_closure_and_unstable_explanation_order_fail_closed() {
        let (sources, obligations) = fixture();
        let plan = success(build_governed_obligation_plan(sources, obligations));
        let explanation = success(explain_governed_obligation_plan(&plan));
        let stale_error = failure(validate_governed_obligation_differential(
            &plan,
            &explanation,
            &SpecContentHash::from_text("stale-source-closure"),
            plan.inventory_commitment(),
            &independent_expectations(GovernedObligationPosture::Mandatory),
            &independent_observations(GovernedObligationPosture::Mandatory),
        ));
        assert_eq!(
            stale_error.code(),
            "governed_obligation_plan.source_closure_disagreement"
        );

        let mut unstable = explanation;
        unstable.entries.reverse();
        let order_error = failure(validate_governed_obligation_differential(
            &plan,
            &unstable,
            plan.source_closure_commitment(),
            plan.inventory_commitment(),
            &independent_expectations(GovernedObligationPosture::Mandatory),
            &independent_observations(GovernedObligationPosture::Mandatory),
        ));
        assert_eq!(
            order_error.code(),
            "governed_obligation_plan.explanation_order_unstable"
        );
    }

    #[test]
    fn differential_disagreement_never_selects_permissive_result() {
        let (sources, obligations) = fixture();
        let plan = success(build_governed_obligation_plan(sources, obligations));
        let explanation = success(explain_governed_obligation_plan(&plan));
        let mut observations = independent_observations(GovernedObligationPosture::Mandatory);
        if let GovernedObligationDifferentialObservation::AuthoritativeDecision(outcome) =
            &mut observations[0]
        {
            outcome.decision =
                GovernedObligationAuthoritativeDecision::Policy(authoritative_approval_decision());
        }
        let error = failure(validate_governed_obligation_differential(
            &plan,
            &explanation,
            plan.source_closure_commitment(),
            plan.inventory_commitment(),
            &independent_expectations(GovernedObligationPosture::Mandatory),
            &observations,
        ));

        assert_eq!(
            error.code(),
            "governed_obligation_plan.authoritative_decision_disagreement"
        );
    }

    #[test]
    fn duplicate_independent_expectation_fails_closed() {
        let (sources, obligations) = fixture();
        let plan = success(build_governed_obligation_plan(sources, obligations));
        let explanation = success(explain_governed_obligation_plan(&plan));
        let mut expectations = independent_expectations(GovernedObligationPosture::Mandatory);
        expectations.push(expectations[0].clone());
        let error = failure(validate_governed_obligation_differential(
            &plan,
            &explanation,
            plan.source_closure_commitment(),
            plan.inventory_commitment(),
            &expectations,
            &independent_observations(GovernedObligationPosture::Mandatory),
        ));

        assert_eq!(
            error.code(),
            "governed_obligation_plan.expectation_duplicate"
        );
    }

    #[test]
    fn duplicate_authoritative_decision_fails_closed() {
        let (sources, obligations) = fixture();
        let plan = success(build_governed_obligation_plan(sources, obligations));
        let explanation = success(explain_governed_obligation_plan(&plan));
        let mut observations = independent_observations(GovernedObligationPosture::Mandatory);
        observations.push(observations[0].clone());
        let error = failure(validate_governed_obligation_differential(
            &plan,
            &explanation,
            plan.source_closure_commitment(),
            plan.inventory_commitment(),
            &independent_expectations(GovernedObligationPosture::Mandatory),
            &observations,
        ));

        assert_eq!(
            error.code(),
            "governed_obligation_plan.authoritative_decision_duplicate"
        );
    }

    #[test]
    fn authoritative_decision_outside_derived_plan_fails_closed() {
        let (sources, obligations) = fixture();
        let plan = success(build_governed_obligation_plan(sources, obligations));
        let explanation = success(explain_governed_obligation_plan(&plan));
        let mut observations = independent_observations(GovernedObligationPosture::Mandatory);
        observations.push(
            GovernedObligationDifferentialObservation::AuthoritativeDecision(Box::new(
                GovernedObligationObservedOutcome {
                    obligation_id: SpecContentHash::from_text("unexpected-obligation"),
                    decision: GovernedObligationAuthoritativeDecision::Policy(
                        authoritative_approval_decision(),
                    ),
                },
            )),
        );
        let error = failure(validate_governed_obligation_differential(
            &plan,
            &explanation,
            plan.source_closure_commitment(),
            plan.inventory_commitment(),
            &independent_expectations(GovernedObligationPosture::Mandatory),
            &observations,
        ));

        assert_eq!(
            error.code(),
            "governed_obligation_plan.authoritative_decision_unexpected"
        );
    }
}
