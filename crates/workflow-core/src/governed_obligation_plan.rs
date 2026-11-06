use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use sha2::{Digest, Sha256};

use crate::{SpecContentHash, WorkflowOsError};

const SOURCE_ID_MAX_BYTES: usize = 256;
const SOURCE_CLOSURE_DOMAIN: &str = "workflow-os/governed-obligation-source-closure/v1";
const OBLIGATION_ID_DOMAIN: &str = "workflow-os/governed-obligation-id/v1";
const INVENTORY_DOMAIN: &str = "workflow-os/governed-obligation-inventory/v1";

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

        let obligation_id = compute_obligation_id(&obligation.source, obligation.kind);
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
    let source_closure_commitment = compute_source_closure_commitment(&source_commitments);
    let inventory_commitment =
        compute_inventory_commitment(algorithm, &source_closure_commitment, &entries);

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
) -> SpecContentHash {
    hash_segments([
        OBLIGATION_ID_DOMAIN,
        source.kind.as_str(),
        &source.identity,
        source.content_commitment.as_str(),
        kind.as_str(),
    ])
}

fn compute_source_closure_commitment(
    sources: &BTreeMap<(GovernedObligationSourceKind, String), SpecContentHash>,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    update_segment(&mut hasher, SOURCE_CLOSURE_DOMAIN);
    for ((kind, identity), commitment) in sources {
        update_segment(&mut hasher, kind.as_str());
        update_segment(&mut hasher, identity);
        update_segment(&mut hasher, commitment.as_str());
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn compute_inventory_commitment(
    algorithm: GovernedObligationPlanAlgorithm,
    source_closure_commitment: &SpecContentHash,
    entries: &[GovernedObligationPlanEntry],
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    update_segment(&mut hasher, INVENTORY_DOMAIN);
    update_segment(&mut hasher, algorithm.as_str());
    update_segment(&mut hasher, source_closure_commitment.as_str());
    for entry in entries {
        update_segment(&mut hasher, entry.obligation_id.as_str());
        update_segment(&mut hasher, entry.kind.as_str());
        update_segment(&mut hasher, entry.posture.as_str());
        update_segment(
            &mut hasher,
            entry
                .evidence_class
                .map_or("none", GovernedObligationEvidenceClass::as_str),
        );
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_segments<'a>(segments: impl IntoIterator<Item = &'a str>) -> SpecContentHash {
    let mut hasher = Sha256::new();
    for segment in segments {
        update_segment(&mut hasher, segment);
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn update_segment(hasher: &mut Sha256, value: &str) {
    hasher.update(value.len().to_be_bytes());
    hasher.update(value.as_bytes());
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
}
