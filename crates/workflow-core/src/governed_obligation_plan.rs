use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use sha2::{Digest, Sha256};

use crate::{
    immutable_run_bundle::immutable_run_bundle_reference_max_count, ImmutableRunBundleBinding,
    LocalCheckRequirementLevel, PolicyEffect, RequiredContextContractBinding,
    RequiredContextExecutionBinding, RequiredContextObligation, SpecContentHash,
    StoredImmutableRunBundle, WorkReportArtifactApprovalProofMarkerRequirement,
    WorkReportArtifactHighAssuranceRequirement, WorkflowOsError, WorkflowRunId,
};

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
    ReportArtifactRequirements,
}

impl GovernedObligationSourceKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::WorkflowStep => "workflow_step",
            Self::PolicyRule => "policy_rule",
            Self::RequiredContextContract => "required_context_contract",
            Self::LocalCheckDeclarationSet => "local_check_declaration_set",
            Self::ReportArtifactRequirements => "report_artifact_requirements",
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

#[derive(Clone, Eq, PartialEq)]
struct GovernedRequiredContextBindingPair {
    contract: RequiredContextContractBinding,
    execution: RequiredContextExecutionBinding,
}

/// Complete private static commitment closure for one run-bound obligation plan.
///
/// This proves the exact immutable sources supplied to extraction. It does not
/// prove that a future runtime consumer selected or authorized the actor,
/// sensitivity ceiling, or binding time committed by those sources.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct GovernedObligationCanonicalCommitmentClosure {
    bundle: StoredImmutableRunBundle,
    required_context: Vec<GovernedRequiredContextBindingPair>,
    required_context_closure_commitment: SpecContentHash,
}

impl GovernedObligationCanonicalCommitmentClosure {
    pub(crate) fn new(
        bundle: &StoredImmutableRunBundle,
        required_context: Vec<(
            RequiredContextContractBinding,
            RequiredContextExecutionBinding,
        )>,
    ) -> Result<Self, WorkflowOsError> {
        let workflow = canonical_workflow_record(bundle)?
            .canonical_definition()
            .as_workflow()
            .ok_or_else(|| composition_error("source_closure_incomplete"))?;
        validate_required_context_pair_cardinality(required_context.len())?;

        let manifest = bundle.manifest();
        let mut seen_steps = BTreeSet::new();
        let mut pairs = Vec::with_capacity(required_context.len());
        for (contract, execution) in required_context {
            execution
                .validate()
                .map_err(|_| composition_error("required_context_binding_mismatched"))?;
            if execution.immutable_run_bundle() != &manifest.run_binding()
                || execution.workflow_id() != manifest.workflow_id()
                || execution.run_id() != manifest.run_id()
                || execution.harness_contract_id() != contract.contract_id()
                || execution.harness_contract_version() != contract.contract_version()
                || execution.contract_content_hash() != contract.content_hash()
                || execution.bound_at() < *manifest.created_at()
                || !workflow
                    .steps
                    .iter()
                    .any(|step| &step.id == execution.step_id())
                || !seen_steps.insert(execution.step_id().clone())
            {
                return Err(composition_error("required_context_binding_mismatched"));
            }
            pairs.push(GovernedRequiredContextBindingPair {
                contract,
                execution,
            });
        }
        if seen_steps.len() != workflow.steps.len() {
            return Err(composition_error("source_closure_incomplete"));
        }
        pairs.sort_by(|left, right| left.execution.step_id().cmp(right.execution.step_id()));
        let required_context_closure_commitment = compute_required_context_closure(&pairs)?;
        Ok(Self {
            bundle: bundle.clone(),
            required_context: pairs,
            required_context_closure_commitment,
        })
    }

    #[must_use]
    pub(crate) const fn bundle(&self) -> &StoredImmutableRunBundle {
        &self.bundle
    }
}

impl fmt::Debug for GovernedObligationCanonicalCommitmentClosure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernedObligationCanonicalCommitmentClosure")
            .field("bundle", &self.bundle)
            .field("required_context_count", &self.required_context.len())
            .field("required_context_closure_commitment", &"[REDACTED]")
            .finish()
    }
}

/// Private non-authorizing identity binding for one extracted static inventory.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct GovernedObligationPlanProjectionBinding {
    algorithm: GovernedObligationPlanAlgorithm,
    immutable_run_bundle: ImmutableRunBundleBinding,
    run_id: WorkflowRunId,
    required_context_closure_commitment: SpecContentHash,
    source_closure_commitment: SpecContentHash,
    inventory_commitment: SpecContentHash,
    obligation_count: u32,
    projection_commitment: SpecContentHash,
}

impl GovernedObligationPlanProjectionBinding {
    #[must_use]
    pub(crate) const fn immutable_run_bundle(&self) -> &ImmutableRunBundleBinding {
        &self.immutable_run_bundle
    }

    #[must_use]
    pub(crate) const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    #[must_use]
    pub(crate) const fn source_closure_commitment(&self) -> &SpecContentHash {
        &self.source_closure_commitment
    }

    #[must_use]
    pub(crate) const fn inventory_commitment(&self) -> &SpecContentHash {
        &self.inventory_commitment
    }

    #[must_use]
    pub(crate) const fn projection_commitment(&self) -> &SpecContentHash {
        &self.projection_commitment
    }

    #[must_use]
    pub(crate) const fn obligation_count(&self) -> u32 {
        self.obligation_count
    }
}

impl fmt::Debug for GovernedObligationPlanProjectionBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernedObligationPlanProjectionBinding")
            .field("algorithm", &self.algorithm)
            .field("immutable_run_bundle", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("required_context_closure_commitment", &"[REDACTED]")
            .field("source_closure_commitment", &"[REDACTED]")
            .field("inventory_commitment", &"[REDACTED]")
            .field("obligation_count", &self.obligation_count)
            .field("projection_commitment", &"[REDACTED]")
            .finish()
    }
}

// Keep the closed source-family walk linear so completeness review can compare
// every accepted family against the plan without cross-module dispatch.
#[allow(clippy::too_many_lines)]
pub(crate) fn extract_governed_obligation_plan(
    closure: &GovernedObligationCanonicalCommitmentClosure,
) -> Result<
    (
        GovernedObligationPlan,
        GovernedObligationPlanProjectionBinding,
    ),
    WorkflowOsError,
> {
    let bundle = closure.bundle();
    let workflow_record = canonical_workflow_record(bundle)?;
    let workflow = workflow_record
        .canonical_definition()
        .as_workflow()
        .ok_or_else(|| composition_error("source_closure_incomplete"))?;
    let (source_capacity, obligation_capacity) =
        validate_extraction_cardinality(closure, workflow)?;
    let mut sources = Vec::with_capacity(source_capacity);
    let mut obligations = Vec::with_capacity(obligation_capacity);

    for step in &workflow.steps {
        let skill_record = bundle
            .definition_records()
            .iter()
            .find(|record| {
                record
                    .canonical_definition()
                    .as_skill()
                    .is_some_and(|skill| {
                        skill.id == step.skill_ref.id
                            && step
                                .skill_ref
                                .version
                                .as_ref()
                                .map_or(true, |version| &skill.version == version)
                    })
            })
            .ok_or_else(|| composition_error("source_closure_incomplete"))?;
        let step_source = GovernedObligationSource::derive(
            GovernedObligationSourceKind::WorkflowStep,
            format!("workflow_step/{}", step.id.as_str()),
            hash_segments([
                "workflow-os/governed-obligation-workflow-step-source/v1",
                workflow_record.canonical_record_hash().as_str(),
                skill_record.canonical_record_hash().as_str(),
                step.id.as_str(),
            ])?,
        )?;
        sources.push(step_source.clone());
        if step.approval_policy.is_some() {
            obligations.push(DerivedGovernedObligation::derive(
                step_source,
                GovernedObligationKind::Approval,
                GovernedObligationPosture::Mandatory,
                Some(GovernedObligationEvidenceClass::ApprovalDecisionProof),
            ));
        }
    }

    if !workflow.approval_requirements.is_empty() {
        return Err(composition_error("unsupported_canonical_source"));
    }

    for record in bundle.definition_records() {
        let Some(policy) = record.canonical_definition().as_policy() else {
            continue;
        };
        for rule in &policy.rules {
            let effect = PolicyEffect::parse(&rule.effect)
                .map_err(|_| composition_error("unsupported_canonical_source"))?;
            let source = GovernedObligationSource::derive(
                GovernedObligationSourceKind::PolicyRule,
                format!("policy_rule/{}/{}", policy.id.as_str(), rule.id),
                hash_segments([
                    "workflow-os/governed-obligation-policy-rule-source/v1",
                    record.canonical_record_hash().as_str(),
                    &rule.id,
                    &rule.effect,
                ])?,
            )?;
            sources.push(source.clone());
            obligations.push(DerivedGovernedObligation::derive(
                source,
                if effect == PolicyEffect::RequireApproval {
                    GovernedObligationKind::Approval
                } else {
                    GovernedObligationKind::Policy
                },
                GovernedObligationPosture::Mandatory,
                Some(if effect == PolicyEffect::RequireApproval {
                    GovernedObligationEvidenceClass::ApprovalDecisionProof
                } else {
                    GovernedObligationEvidenceClass::PolicyDecision
                }),
            ));
        }
    }

    for pair in &closure.required_context {
        let step_id = pair.execution.step_id().as_str();
        if pair.contract.is_explicit_none() {
            sources.push(GovernedObligationSource::derive(
                GovernedObligationSourceKind::RequiredContextContract,
                format!("required_context/{step_id}/explicit_none"),
                hash_segments([
                    "workflow-os/governed-obligation-required-context-source/v1",
                    pair.contract.content_hash().as_str(),
                    pair.execution.binding_hash().as_str(),
                    "explicit_none",
                ])?,
            )?);
            continue;
        }
        for requirement in pair.contract.requirements() {
            let source = GovernedObligationSource::derive(
                GovernedObligationSourceKind::RequiredContextContract,
                format!(
                    "required_context/{step_id}/{}",
                    requirement.requirement_id().as_str()
                ),
                hash_segments([
                    "workflow-os/governed-obligation-required-context-source/v1",
                    pair.contract.content_hash().as_str(),
                    pair.execution.binding_hash().as_str(),
                    requirement.requirement_id().as_str(),
                ])?,
            )?;
            sources.push(source.clone());
            obligations.push(DerivedGovernedObligation::derive(
                source,
                GovernedObligationKind::RequiredContext,
                match requirement.obligation() {
                    RequiredContextObligation::Required => GovernedObligationPosture::Mandatory,
                    RequiredContextObligation::Optional => GovernedObligationPosture::Optional,
                },
                Some(GovernedObligationEvidenceClass::RequiredContextConsumption),
            ));
        }
    }

    for record in bundle.local_check_declaration_set_records() {
        for declaration in record.declarations() {
            let source = GovernedObligationSource::derive(
                GovernedObligationSourceKind::LocalCheckDeclarationSet,
                format!(
                    "local_check/{}/{}",
                    record.step_id().as_str(),
                    declaration.requirement_id().as_str()
                ),
                declaration.obligation_identity().clone(),
            )?;
            sources.push(source.clone());
            obligations.push(DerivedGovernedObligation::derive(
                source,
                GovernedObligationKind::LocalCheck,
                match declaration.requirement_level() {
                    LocalCheckRequirementLevel::Required => GovernedObligationPosture::Mandatory,
                    LocalCheckRequirementLevel::Optional => GovernedObligationPosture::Optional,
                },
                Some(GovernedObligationEvidenceClass::LocalCheckResult),
            ));
        }
    }

    let report_requirements = &workflow.report_artifact_requirements;
    let report_source = GovernedObligationSource::derive(
        GovernedObligationSourceKind::ReportArtifactRequirements,
        "report_artifact_requirements/workflow",
        hash_segments([
            "workflow-os/governed-obligation-report-artifact-source/v1",
            workflow_record.canonical_record_hash().as_str(),
            report_high_assurance_name(report_requirements.high_assurance_approval),
            report_proof_marker_name(report_requirements.approval_proof_markers),
        ])?,
    )?;
    sources.push(report_source.clone());
    if report_requirements.high_assurance_approval
        != WorkReportArtifactHighAssuranceRequirement::NotRequired
        || report_requirements.approval_proof_markers
            != WorkReportArtifactApprovalProofMarkerRequirement::NotRequired
    {
        obligations.push(DerivedGovernedObligation::derive(
            report_source,
            GovernedObligationKind::WorkReportArtifact,
            GovernedObligationPosture::Mandatory,
            Some(GovernedObligationEvidenceClass::WorkReportArtifact),
        ));
    }

    if sources.len() > immutable_run_bundle_reference_max_count()
        || obligations.len() > immutable_run_bundle_reference_max_count()
    {
        return Err(composition_error("cardinality_exceeded"));
    }
    let plan = build_governed_obligation_plan(sources, obligations)?;
    let obligation_count = u32::try_from(plan.entries().len())
        .map_err(|_| composition_error("cardinality_exceeded"))?;
    let manifest = bundle.manifest();
    let mut projection = GovernedObligationPlanProjectionBinding {
        algorithm: plan.algorithm(),
        immutable_run_bundle: manifest.run_binding(),
        run_id: manifest.run_id().clone(),
        required_context_closure_commitment: closure.required_context_closure_commitment.clone(),
        source_closure_commitment: plan.source_closure_commitment().clone(),
        inventory_commitment: plan.inventory_commitment().clone(),
        obligation_count,
        projection_commitment: SpecContentHash::from_bytes([]),
    };
    projection.projection_commitment = compute_projection_commitment(&projection)?;
    Ok((plan, projection))
}

fn canonical_workflow_record(
    bundle: &StoredImmutableRunBundle,
) -> Result<&crate::ImmutableRunBundleDefinitionRecord, WorkflowOsError> {
    let mut records = bundle
        .definition_records()
        .iter()
        .filter(|record| record.canonical_definition().as_workflow().is_some());
    let record = records
        .next()
        .ok_or_else(|| composition_error("source_closure_incomplete"))?;
    if records.next().is_some() {
        return Err(composition_error("source_closure_incomplete"));
    }
    Ok(record)
}

fn compute_required_context_closure(
    pairs: &[GovernedRequiredContextBindingPair],
) -> Result<SpecContentHash, WorkflowOsError> {
    let mut segments = vec!["workflow-os/governed-obligation-required-context-closure/v1"];
    for pair in pairs {
        segments.push(pair.contract.content_hash().as_str());
        segments.push(pair.execution.binding_hash().as_str());
    }
    hash_segments(segments)
}

fn validate_extraction_cardinality(
    closure: &GovernedObligationCanonicalCommitmentClosure,
    workflow: &crate::WorkflowDefinition,
) -> Result<(usize, usize), WorkflowOsError> {
    let mut source_count = workflow.steps.len();
    let mut obligation_count = workflow
        .steps
        .iter()
        .filter(|step| step.approval_policy.is_some())
        .count();
    for record in closure.bundle.definition_records() {
        if let Some(policy) = record.canonical_definition().as_policy() {
            source_count = checked_add_cardinality(source_count, policy.rules.len())?;
            obligation_count = checked_add_cardinality(obligation_count, policy.rules.len())?;
        }
    }
    for pair in &closure.required_context {
        let source_increment = if pair.contract.is_explicit_none() {
            1
        } else {
            pair.contract.requirements().len()
        };
        source_count = checked_add_cardinality(source_count, source_increment)?;
        obligation_count =
            checked_add_cardinality(obligation_count, pair.contract.requirements().len())?;
    }
    for record in closure.bundle.local_check_declaration_set_records() {
        source_count = checked_add_cardinality(source_count, record.declarations().len())?;
        obligation_count = checked_add_cardinality(obligation_count, record.declarations().len())?;
    }
    source_count = checked_add_cardinality(source_count, 1)?;
    if workflow
        .report_artifact_requirements
        .high_assurance_approval
        != WorkReportArtifactHighAssuranceRequirement::NotRequired
        || workflow.report_artifact_requirements.approval_proof_markers
            != WorkReportArtifactApprovalProofMarkerRequirement::NotRequired
    {
        obligation_count = checked_add_cardinality(obligation_count, 1)?;
    }
    validate_extraction_count_bounds(source_count, obligation_count)?;
    Ok((source_count, obligation_count))
}

fn validate_required_context_pair_cardinality(pair_count: usize) -> Result<(), WorkflowOsError> {
    if pair_count > immutable_run_bundle_reference_max_count() {
        return Err(composition_error("cardinality_exceeded"));
    }
    Ok(())
}

fn validate_extraction_count_bounds(
    source_count: usize,
    obligation_count: usize,
) -> Result<(), WorkflowOsError> {
    let maximum = immutable_run_bundle_reference_max_count();
    if source_count > maximum || obligation_count > maximum {
        return Err(composition_error("cardinality_exceeded"));
    }
    Ok(())
}

fn checked_add_cardinality(current: usize, increment: usize) -> Result<usize, WorkflowOsError> {
    current
        .checked_add(increment)
        .ok_or_else(|| composition_error("cardinality_exceeded"))
}

fn compute_projection_commitment(
    binding: &GovernedObligationPlanProjectionBinding,
) -> Result<SpecContentHash, WorkflowOsError> {
    let obligation_count = binding.obligation_count.to_string();
    hash_segments([
        "workflow-os/governed-obligation-plan-projection/v1",
        binding.algorithm.as_str(),
        binding.immutable_run_bundle.bundle_id().as_str(),
        binding.immutable_run_bundle.bundle_version().as_str(),
        binding.immutable_run_bundle.root_hash().as_str(),
        binding.run_id.as_str(),
        binding.required_context_closure_commitment.as_str(),
        binding.source_closure_commitment.as_str(),
        binding.inventory_commitment.as_str(),
        &obligation_count,
    ])
}

const fn report_high_assurance_name(
    requirement: WorkReportArtifactHighAssuranceRequirement,
) -> &'static str {
    match requirement {
        WorkReportArtifactHighAssuranceRequirement::NotRequired => "not_required",
        WorkReportArtifactHighAssuranceRequirement::DisclosureRequired => "disclosure_required",
        WorkReportArtifactHighAssuranceRequirement::ValidatedDisclosureRequired => {
            "validated_disclosure_required"
        }
        WorkReportArtifactHighAssuranceRequirement::ValidatedFailClosedDisclosureRequired => {
            "validated_fail_closed_disclosure_required"
        }
    }
}

const fn report_proof_marker_name(
    requirement: WorkReportArtifactApprovalProofMarkerRequirement,
) -> &'static str {
    match requirement {
        WorkReportArtifactApprovalProofMarkerRequirement::NotRequired => "not_required",
        WorkReportArtifactApprovalProofMarkerRequirement::ProjectionRequired => {
            "projection_required"
        }
        WorkReportArtifactApprovalProofMarkerRequirement::MarkerRequired => "marker_required",
    }
}

fn composition_error(code: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("governed_obligation_plan.runtime_composition.{code}"),
        "governed obligation runtime composition input is invalid",
    )
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
    #![allow(clippy::expect_used, clippy::panic, clippy::too_many_lines)]

    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;
    use crate::required_context_execution_binding::RequiredContextExecutionBindingTestSubstitution;
    use crate::{
        build_immutable_run_bundle, build_immutable_run_bundle_with_local_check_declarations,
        load_project, validate_project_bundle, ActorId, GovernedContextAccessLevel,
        GovernedContextReferenceTarget, HarnessContractId, HarnessContractVersion,
        ImmutableRunBundleBuildRequest, ImmutableRunBundleExecutionPosture,
        ImmutableRunBundleHandlerPosture, ImmutableRunBundleHandlerReference, ImmutableRunBundleId,
        ImmutableRunBundleReferencePosture, ImmutableRunBundleSensitivity,
        ImmutableRunBundleVersion, LocalCheckCommandContract, LocalCheckCommandContractInventory,
        LocalImmutableRunBundleStore, RequiredContextExecutionBindingInput,
        RequiredContextRequirement, RequiredContextRequirementId, SkillId, SkillVersion, StepId,
        Timestamp, WorkReportId, WorkReportSensitivity, WorkflowId, WorkflowRunId,
        SUPPORTED_SCHEMA_VERSION,
    };

    static NEXT_ROOT: AtomicU64 = AtomicU64::new(1);

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new(name: &str) -> Self {
            let id = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "workflow-os-obligation-plan-{name}-{}-{id}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&path);
            fs::create_dir_all(&path).expect("test root");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn write(&self, relative: &str, content: &str) {
            let path = self.0.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("fixture parent");
            }
            fs::write(path, content).expect("fixture write");
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn timestamp(value: &str) -> Timestamp {
        Timestamp::parse_rfc3339(value).expect("timestamp")
    }

    fn extraction_fixture() -> (
        TestRoot,
        StoredImmutableRunBundle,
        RequiredContextContractBinding,
        RequiredContextExecutionBinding,
    ) {
        let root = TestRoot::new("extraction");
        root.write(
            "workflow-os.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nproject:\n  id: obligation/project\n  name: Obligation Project\n"
            ),
        );
        root.write(
            "workflows/build.workflow.yml",
            &format!(
                r"schema_version: {SUPPORTED_SCHEMA_VERSION}
id: obligation/build
version: v1
display_name: Obligation Build
triggers:
  - id: manual
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
cancellation_behavior: stop
audit_requirements:
  required: true
  events: [RunCreated]
  store_references_only: true
observability_requirements:
  metrics: [workflow_latency]
  tracing: true
  latency_tracking: true
"
            ),
        );
        root.write(
            "skills/check.skill.yml",
            &format!(
                r"schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/check
version: v1
display_name: Check
allowed_capabilities:
  - name: local.read
input_contract:
  fields:
    - name: request
      field_type: string
output_contract:
  fields:
    - name: summary
      field_type: string
failure_modes:
  - code: failed
    description: Failed.
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
        root.write(
            "policies/read-only.policy.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/read-only\nname: Read Only\nrules:\n  - id: allow-local\n    effect: allow_local\n"
            ),
        );
        let loaded = load_project(root.path());
        assert!(!loaded.has_errors(), "{:?}", loaded.diagnostics);
        let project = loaded.bundle.expect("project");
        let validation = validate_project_bundle(&project);
        assert!(
            !validation.has_errors(),
            "project validation: {:?}",
            validation.diagnostics
        );
        let inventory = LocalCheckCommandContractInventory::new(vec![
            LocalCheckCommandContract::docs_check_model_only().expect("contract"),
        ])
        .expect("inventory");
        let built = build_immutable_run_bundle_with_local_check_declarations(
            ImmutableRunBundleBuildRequest {
                project: &project,
                workflow_id: &WorkflowId::new("obligation/build").expect("workflow"),
                bundle_id: ImmutableRunBundleId::new("bundle/obligation").expect("bundle"),
                bundle_version: ImmutableRunBundleVersion::new("v1").expect("version"),
                run_id: WorkflowRunId::new("run-obligation").expect("run"),
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
                created_at: timestamp("2026-11-06T10:00:00Z"),
                created_by: ActorId::new("system/kernel").expect("actor"),
                sensitivity: ImmutableRunBundleSensitivity::Internal,
                redaction_required: true,
            },
            &inventory,
        )
        .expect("bundle");
        let store = LocalImmutableRunBundleStore::new(root.path().join("bundle-store"));
        store.write_bundle(&built).expect("store");
        let stored = store
            .read_bundle(built.manifest().run_id(), built.manifest().bundle_id())
            .expect("stored");
        let contract = RequiredContextContractBinding::new(
            HarnessContractId::new("harness/obligation").expect("contract"),
            HarnessContractVersion::new("v1").expect("version"),
            vec![RequiredContextRequirement::new(
                RequiredContextRequirementId::new("report-current").expect("requirement"),
                GovernedContextReferenceTarget::WorkReport(
                    WorkReportId::new("report/current").expect("report"),
                ),
                GovernedContextAccessLevel::ReferenceOnly,
                RequiredContextObligation::Required,
                WorkReportSensitivity::Internal,
            )
            .expect("requirement")],
        )
        .expect("contract");
        let binding = RequiredContextExecutionBinding::new(RequiredContextExecutionBindingInput {
            bundle: &stored,
            contract: &contract,
            actor: ActorId::new("agent/executor").expect("actor"),
            step_id: StepId::new("inspect").expect("step"),
            maximum_sensitivity: WorkReportSensitivity::Internal,
            bound_at: timestamp("2026-11-06T10:05:00Z"),
        })
        .expect("binding");
        (root, stored, contract, binding)
    }

    fn multi_step_extraction_fixture() -> (
        TestRoot,
        StoredImmutableRunBundle,
        RequiredContextContractBinding,
        Vec<RequiredContextExecutionBinding>,
    ) {
        let root = TestRoot::new("multi-step-extraction");
        root.write(
            "workflow-os.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nproject:\n  id: obligation/multi\n  name: Obligation Multi\n"
            ),
        );
        root.write(
            "workflows/build.workflow.yml",
            &format!(
                r"schema_version: {SUPPORTED_SCHEMA_VERSION}
id: obligation/multi-build
version: v1
display_name: Obligation Multi Build
triggers:
  - id: manual
    kind: manual
steps:
  - id: first
    skill_ref:
      id: local/check
      version: v1
    policy_requirements:
      - id: local/read-only
    terminal_behavior: fail_workflow
  - id: second
    skill_ref:
      id: local/check
      version: v1
    policy_requirements:
      - id: local/read-only
    terminal_behavior: fail_workflow
cancellation_behavior: stop
audit_requirements:
  required: true
  events: [RunCreated]
  store_references_only: true
observability_requirements:
  metrics: [workflow_latency]
  tracing: true
  latency_tracking: true
"
            ),
        );
        root.write(
            "skills/check.skill.yml",
            &format!(
                r"schema_version: {SUPPORTED_SCHEMA_VERSION}
id: local/check
version: v1
display_name: Check
allowed_capabilities:
  - name: local.read
input_contract:
  fields:
    - name: request
      field_type: string
output_contract:
  fields:
    - name: summary
      field_type: string
failure_modes:
  - code: failed
    description: Failed.
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
        root.write(
            "policies/read-only.policy.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/read-only\nname: Read Only\nrules:\n  - id: allow-local\n    effect: allow_local\n"
            ),
        );
        let loaded = load_project(root.path());
        assert!(!loaded.has_errors(), "{:?}", loaded.diagnostics);
        let project = loaded.bundle.expect("project");
        let built = build_immutable_run_bundle(ImmutableRunBundleBuildRequest {
            project: &project,
            workflow_id: &WorkflowId::new("obligation/multi-build").expect("workflow"),
            bundle_id: ImmutableRunBundleId::new("bundle/multi-obligation").expect("bundle"),
            bundle_version: ImmutableRunBundleVersion::new("v1").expect("version"),
            run_id: WorkflowRunId::new("run-multi-obligation").expect("run"),
            resolved_execution_context_hash: SpecContentHash::from_text("context"),
            execution_posture: ImmutableRunBundleExecutionPosture::new(
                Vec::new(),
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::NotSupplied,
            )
            .expect("posture"),
            handlers: vec![ImmutableRunBundleHandlerReference {
                skill_id: SkillId::new("local/check").expect("skill"),
                skill_version: SkillVersion::new("v1").expect("skill version"),
                posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
            }],
            created_at: timestamp("2026-11-06T10:00:00Z"),
            created_by: ActorId::new("system/kernel").expect("actor"),
            sensitivity: ImmutableRunBundleSensitivity::Internal,
            redaction_required: true,
        })
        .expect("bundle");
        let store = LocalImmutableRunBundleStore::new(root.path().join("bundle-store"));
        store.write_bundle(&built).expect("store");
        let stored = store
            .read_bundle(built.manifest().run_id(), built.manifest().bundle_id())
            .expect("stored");
        let contract = RequiredContextContractBinding::explicit_none_for_test(
            HarnessContractId::new("harness/multi-obligation").expect("contract"),
            HarnessContractVersion::new("v1").expect("version"),
        )
        .expect("contract");
        let bindings = ["first", "second"]
            .into_iter()
            .map(|step| {
                RequiredContextExecutionBinding::new(RequiredContextExecutionBindingInput {
                    bundle: &stored,
                    contract: &contract,
                    actor: ActorId::new("agent/executor").expect("actor"),
                    step_id: StepId::new(step).expect("step"),
                    maximum_sensitivity: WorkReportSensitivity::Internal,
                    bound_at: timestamp("2026-11-06T10:05:00Z"),
                })
                .expect("binding")
            })
            .collect();
        (root, stored, contract, bindings)
    }

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
            GovernedObligationSourceKind::ReportArtifactRequirements,
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

    #[test]
    fn canonical_extraction_binds_every_supported_source_without_authority() {
        let (_root, bundle, contract, binding) = extraction_fixture();
        let closure = success(GovernedObligationCanonicalCommitmentClosure::new(
            &bundle,
            vec![(contract, binding)],
        ));
        let (plan, projection) = success(extract_governed_obligation_plan(&closure));

        assert_eq!(plan.entries().len(), 3);
        assert_eq!(projection.obligation_count(), 3);
        assert_eq!(
            projection.immutable_run_bundle(),
            &bundle.manifest().run_binding()
        );
        assert_eq!(projection.run_id(), bundle.manifest().run_id());
        assert_eq!(
            projection.source_closure_commitment(),
            plan.source_closure_commitment()
        );
        assert_eq!(
            projection.inventory_commitment(),
            plan.inventory_commitment()
        );
        assert!(plan.entries().iter().any(|entry| {
            entry.source.kind == GovernedObligationSourceKind::LocalCheckDeclarationSet
                && entry.kind() == GovernedObligationKind::LocalCheck
        }));
        assert!(plan.entries().iter().any(|entry| {
            entry.source.kind == GovernedObligationSourceKind::RequiredContextContract
                && entry.kind() == GovernedObligationKind::RequiredContext
        }));
        assert!(plan.entries().iter().any(|entry| {
            entry.source.kind == GovernedObligationSourceKind::PolicyRule
                && entry.kind() == GovernedObligationKind::Policy
        }));
    }

    #[test]
    fn canonical_extraction_is_deterministic_and_redacts_projection_identity() {
        let (_root, bundle, contract, binding) = extraction_fixture();
        let first = success(GovernedObligationCanonicalCommitmentClosure::new(
            &bundle,
            vec![(contract.clone(), binding.clone())],
        ));
        let second = success(GovernedObligationCanonicalCommitmentClosure::new(
            &bundle,
            vec![(contract, binding)],
        ));
        let (first_plan, first_projection) = success(extract_governed_obligation_plan(&first));
        let (second_plan, second_projection) = success(extract_governed_obligation_plan(&second));

        assert_eq!(first_plan, second_plan);
        assert_eq!(first_projection, second_projection);
        let debug = format!("{first:?} {first_projection:?}");
        assert!(!debug.contains(bundle.manifest().run_id().as_str()));
        assert!(!debug.contains(first_projection.projection_commitment().as_str()));
        assert!(!debug.contains(first_plan.inventory_commitment().as_str()));
        assert!(debug.contains("required_context_count"));
        assert!(debug.contains("obligation_count"));
    }

    #[test]
    fn multi_step_commitment_closure_is_independent_of_insertion_order() {
        let (_root, bundle, contract, bindings) = multi_step_extraction_fixture();
        let forward = success(GovernedObligationCanonicalCommitmentClosure::new(
            &bundle,
            vec![
                (contract.clone(), bindings[0].clone()),
                (contract.clone(), bindings[1].clone()),
            ],
        ));
        let reversed = success(GovernedObligationCanonicalCommitmentClosure::new(
            &bundle,
            vec![
                (contract.clone(), bindings[1].clone()),
                (contract, bindings[0].clone()),
            ],
        ));
        let (forward_plan, forward_projection) =
            success(extract_governed_obligation_plan(&forward));
        let (reversed_plan, reversed_projection) =
            success(extract_governed_obligation_plan(&reversed));
        assert_eq!(forward_plan, reversed_plan);
        assert_eq!(forward_projection, reversed_projection);
    }

    #[test]
    fn missing_or_substituted_required_context_binding_fails_closed() {
        let (_root, bundle, contract, binding) = extraction_fixture();
        let missing = failure(GovernedObligationCanonicalCommitmentClosure::new(
            &bundle,
            Vec::new(),
        ));
        assert_eq!(
            missing.code(),
            "governed_obligation_plan.runtime_composition.source_closure_incomplete"
        );

        let substituted = binding.with_test_substitution(
            RequiredContextExecutionBindingTestSubstitution::ContractContentHash(
                SpecContentHash::from_text("substituted"),
            ),
        );
        let error = failure(GovernedObligationCanonicalCommitmentClosure::new(
            &bundle,
            vec![(contract, substituted)],
        ));
        assert_eq!(
            error.code(),
            "governed_obligation_plan.runtime_composition.required_context_binding_mismatched"
        );
        assert!(!error.to_string().contains("substituted"));
    }

    #[test]
    fn duplicate_required_context_step_fails_as_mismatched() {
        let (_root, bundle, contract, binding) = extraction_fixture();
        let error = failure(GovernedObligationCanonicalCommitmentClosure::new(
            &bundle,
            vec![(contract.clone(), binding.clone()), (contract, binding)],
        ));
        assert_eq!(
            error.code(),
            "governed_obligation_plan.runtime_composition.required_context_binding_mismatched"
        );
    }

    #[test]
    fn wrong_bundle_workflow_run_and_step_bindings_fail_closed() {
        let (_root, bundle, contract, binding) = extraction_fixture();
        let substitutions = [
            RequiredContextExecutionBindingTestSubstitution::ImmutableRunBundle(
                bundle
                    .manifest()
                    .run_binding()
                    .with_test_root_hash(SpecContentHash::from_text("other-root")),
            ),
            RequiredContextExecutionBindingTestSubstitution::WorkflowId(
                WorkflowId::new("obligation/other").expect("workflow"),
            ),
            RequiredContextExecutionBindingTestSubstitution::RunId(
                WorkflowRunId::new("run-other").expect("run"),
            ),
            RequiredContextExecutionBindingTestSubstitution::StepId(
                StepId::new("other").expect("step"),
            ),
        ];

        for substitution in substitutions {
            let changed = binding.with_test_substitution(substitution);
            let error = failure(GovernedObligationCanonicalCommitmentClosure::new(
                &bundle,
                vec![(contract.clone(), changed)],
            ));
            assert_eq!(
                error.code(),
                "governed_obligation_plan.runtime_composition.required_context_binding_mismatched"
            );
        }
    }

    #[test]
    fn wrong_contract_identity_version_and_content_bindings_fail_closed() {
        let (_root, bundle, contract, binding) = extraction_fixture();
        let substitutions = [
            RequiredContextExecutionBindingTestSubstitution::HarnessContractId(
                HarnessContractId::new("harness/other").expect("contract"),
            ),
            RequiredContextExecutionBindingTestSubstitution::HarnessContractVersion(
                HarnessContractVersion::new("v2").expect("version"),
            ),
            RequiredContextExecutionBindingTestSubstitution::ContractContentHash(
                SpecContentHash::from_text("other-contract"),
            ),
        ];

        for substitution in substitutions {
            let changed = binding.with_test_substitution(substitution);
            let error = failure(GovernedObligationCanonicalCommitmentClosure::new(
                &bundle,
                vec![(contract.clone(), changed)],
            ));
            assert_eq!(
                error.code(),
                "governed_obligation_plan.runtime_composition.required_context_binding_mismatched"
            );
        }
    }

    #[test]
    fn alternate_actor_is_a_distinct_valid_commitment() {
        let (_root, bundle, contract, binding) = extraction_fixture();
        let baseline = success(GovernedObligationCanonicalCommitmentClosure::new(
            &bundle,
            vec![(contract.clone(), binding)],
        ));
        let alternate = success(RequiredContextExecutionBinding::new(
            RequiredContextExecutionBindingInput {
                bundle: &bundle,
                contract: &contract,
                actor: ActorId::new("agent/alternate").expect("actor"),
                step_id: StepId::new("inspect").expect("step"),
                maximum_sensitivity: WorkReportSensitivity::Internal,
                bound_at: timestamp("2026-11-06T10:05:00Z"),
            },
        ));
        let alternate = success(GovernedObligationCanonicalCommitmentClosure::new(
            &bundle,
            vec![(contract, alternate)],
        ));
        let (_, baseline_projection) = success(extract_governed_obligation_plan(&baseline));
        let (_, alternate_projection) = success(extract_governed_obligation_plan(&alternate));
        assert_ne!(
            baseline_projection.projection_commitment(),
            alternate_projection.projection_commitment()
        );
    }

    #[test]
    fn alternate_known_sensitivity_is_a_distinct_valid_commitment() {
        let (_root, bundle, contract, binding) = extraction_fixture();
        let baseline = success(GovernedObligationCanonicalCommitmentClosure::new(
            &bundle,
            vec![(contract.clone(), binding)],
        ));
        let alternate = success(RequiredContextExecutionBinding::new(
            RequiredContextExecutionBindingInput {
                bundle: &bundle,
                contract: &contract,
                actor: ActorId::new("agent/executor").expect("actor"),
                step_id: StepId::new("inspect").expect("step"),
                maximum_sensitivity: WorkReportSensitivity::Confidential,
                bound_at: timestamp("2026-11-06T10:05:00Z"),
            },
        ));
        let alternate = success(GovernedObligationCanonicalCommitmentClosure::new(
            &bundle,
            vec![(contract, alternate)],
        ));
        let (_, baseline_projection) = success(extract_governed_obligation_plan(&baseline));
        let (_, alternate_projection) = success(extract_governed_obligation_plan(&alternate));
        assert_ne!(
            baseline_projection.projection_commitment(),
            alternate_projection.projection_commitment()
        );
    }

    #[test]
    fn alternate_non_pre_bundle_time_is_a_distinct_valid_commitment() {
        let (_root, bundle, contract, binding) = extraction_fixture();
        let baseline = success(GovernedObligationCanonicalCommitmentClosure::new(
            &bundle,
            vec![(contract.clone(), binding)],
        ));
        let alternate = success(RequiredContextExecutionBinding::new(
            RequiredContextExecutionBindingInput {
                bundle: &bundle,
                contract: &contract,
                actor: ActorId::new("agent/executor").expect("actor"),
                step_id: StepId::new("inspect").expect("step"),
                maximum_sensitivity: WorkReportSensitivity::Internal,
                bound_at: timestamp("2026-11-06T10:06:00Z"),
            },
        ));
        let alternate = success(GovernedObligationCanonicalCommitmentClosure::new(
            &bundle,
            vec![(contract, alternate)],
        ));
        let (_, baseline_projection) = success(extract_governed_obligation_plan(&baseline));
        let (_, alternate_projection) = success(extract_governed_obligation_plan(&alternate));
        assert_ne!(
            baseline_projection.projection_commitment(),
            alternate_projection.projection_commitment()
        );
    }

    #[test]
    fn unknown_sensitivity_and_pre_bundle_time_fail_at_binding_boundary() {
        let (_root, bundle, contract, _binding) = extraction_fixture();
        let unknown = failure(RequiredContextExecutionBinding::new(
            RequiredContextExecutionBindingInput {
                bundle: &bundle,
                contract: &contract,
                actor: ActorId::new("agent/executor").expect("actor"),
                step_id: StepId::new("inspect").expect("step"),
                maximum_sensitivity: WorkReportSensitivity::Unknown,
                bound_at: timestamp("2026-11-06T10:05:00Z"),
            },
        ));
        assert_eq!(
            unknown.code(),
            "required_context.execution_binding.sensitivity.unknown"
        );

        let early = failure(RequiredContextExecutionBinding::new(
            RequiredContextExecutionBindingInput {
                bundle: &bundle,
                contract: &contract,
                actor: ActorId::new("agent/executor").expect("actor"),
                step_id: StepId::new("inspect").expect("step"),
                maximum_sensitivity: WorkReportSensitivity::Internal,
                bound_at: timestamp("2026-11-06T09:59:59Z"),
            },
        ));
        assert_eq!(
            early.code(),
            "required_context.execution_binding.bound_at.before_bundle"
        );
    }

    #[test]
    fn shared_cardinality_bounds_accept_exact_maxima_and_reject_max_plus_one() {
        let maximum = immutable_run_bundle_reference_max_count();
        success(validate_required_context_pair_cardinality(maximum));
        success(validate_extraction_count_bounds(maximum, maximum));

        let pair_error = failure(validate_required_context_pair_cardinality(maximum + 1));
        let source_error = failure(validate_extraction_count_bounds(maximum + 1, maximum));
        let obligation_error = failure(validate_extraction_count_bounds(maximum, maximum + 1));
        for error in [pair_error, source_error, obligation_error] {
            assert_eq!(
                error.code(),
                "governed_obligation_plan.runtime_composition.cardinality_exceeded"
            );
        }
    }

    #[test]
    fn checked_cardinality_arithmetic_rejects_overflow() {
        let error = failure(checked_add_cardinality(usize::MAX, 1));
        assert_eq!(
            error.code(),
            "governed_obligation_plan.runtime_composition.cardinality_exceeded"
        );
    }

    #[test]
    fn required_context_cardinality_rejects_shared_bundle_maximum_plus_one() {
        let (_root, bundle, contract, binding) = extraction_fixture();
        let pairs = (0..=immutable_run_bundle_reference_max_count())
            .map(|_| (contract.clone(), binding.clone()))
            .collect();
        let error = failure(GovernedObligationCanonicalCommitmentClosure::new(
            &bundle, pairs,
        ));
        assert_eq!(
            error.code(),
            "governed_obligation_plan.runtime_composition.cardinality_exceeded"
        );
    }

    #[test]
    fn explicit_none_required_context_is_a_canonical_source_not_an_obligation() {
        let (_root, bundle, _contract, _binding) = extraction_fixture();
        let contract = success(RequiredContextContractBinding::explicit_none_for_test(
            HarnessContractId::new("harness/none").expect("contract"),
            HarnessContractVersion::new("v1").expect("version"),
        ));
        let binding = success(RequiredContextExecutionBinding::new(
            RequiredContextExecutionBindingInput {
                bundle: &bundle,
                contract: &contract,
                actor: ActorId::new("agent/executor").expect("actor"),
                step_id: StepId::new("inspect").expect("step"),
                maximum_sensitivity: WorkReportSensitivity::Internal,
                bound_at: timestamp("2026-11-06T10:05:00Z"),
            },
        ));
        let closure = success(GovernedObligationCanonicalCommitmentClosure::new(
            &bundle,
            vec![(contract, binding)],
        ));
        let (plan, projection) = success(extract_governed_obligation_plan(&closure));

        assert_eq!(plan.entries().len(), 2);
        assert_eq!(projection.obligation_count(), 2);
        assert!(!plan
            .entries()
            .iter()
            .any(|entry| entry.kind() == GovernedObligationKind::RequiredContext));
    }
}
