use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ActorId, CapabilityResolutionPosture, CapabilityResolutionReason, GovernedContextAccessLevel,
    GovernedContextAvailability, GovernedContextProjection, GovernedContextProjectionEntry,
    GovernedContextReferenceKind, GovernedContextReferenceTarget, HarnessContractId,
    HarnessContractVersion, SpecContentHash, StepId, Timestamp, WorkReportSensitivity, WorkflowId,
    WorkflowOsError, WorkflowRunId,
};

const REQUIREMENT_ID_MAX_BYTES: usize = 128;

/// Stable identifier for one typed required-context obligation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub struct RequiredContextRequirementId(String);

impl RequiredContextRequirementId {
    /// Creates a bounded requirement identifier.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error for invalid identifiers.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_requirement_id(&value)?;
        Ok(Self(value))
    }

    /// Returns the validated identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for RequiredContextRequirementId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("RequiredContextRequirementId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for RequiredContextRequirementId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::new(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// Whether a declared context reference is mandatory or optional.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredContextObligation {
    /// Missing or unsatisfied context blocks consumption.
    Required,
    /// Missing or unsatisfied context remains an explicit non-blocking gap.
    Optional,
}

impl<'de> Deserialize<'de> for RequiredContextObligation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "required" => Ok(Self::Required),
            "optional" => Ok(Self::Optional),
            _ => Err(serde::de::Error::custom(validation_error(
                "required_context.obligation.invalid",
                "required context obligation is invalid",
            ))),
        }
    }
}

/// One exact typed required-context declaration.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct RequiredContextRequirement {
    requirement_id: RequiredContextRequirementId,
    target: GovernedContextReferenceTarget,
    access_level: GovernedContextAccessLevel,
    obligation: RequiredContextObligation,
    maximum_sensitivity: WorkReportSensitivity,
}

impl RequiredContextRequirement {
    /// Creates a validated typed context requirement.
    ///
    /// # Errors
    ///
    /// Returns a stable error when the sensitivity is unknown.
    pub fn new(
        requirement_id: RequiredContextRequirementId,
        target: GovernedContextReferenceTarget,
        access_level: GovernedContextAccessLevel,
        obligation: RequiredContextObligation,
        maximum_sensitivity: WorkReportSensitivity,
    ) -> Result<Self, WorkflowOsError> {
        let requirement = Self {
            requirement_id,
            target,
            access_level,
            obligation,
            maximum_sensitivity,
        };
        requirement.validate()?;
        Ok(requirement)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_requirement_id(self.requirement_id.as_str())?;
        self.access_level.required_capability()?;
        if self.maximum_sensitivity == WorkReportSensitivity::Unknown {
            return Err(validation_error(
                "required_context.requirement.sensitivity_unknown",
                "required context requirement needs known maximum sensitivity",
            ));
        }
        Ok(())
    }

    /// Returns the stable requirement ID.
    #[must_use]
    pub const fn requirement_id(&self) -> &RequiredContextRequirementId {
        &self.requirement_id
    }

    /// Returns the exact typed target.
    #[must_use]
    pub const fn target(&self) -> &GovernedContextReferenceTarget {
        &self.target
    }

    /// Returns the exact requested access level.
    #[must_use]
    pub const fn access_level(&self) -> GovernedContextAccessLevel {
        self.access_level
    }

    /// Returns whether this requirement is mandatory.
    #[must_use]
    pub const fn obligation(&self) -> RequiredContextObligation {
        self.obligation
    }

    /// Returns the requirement sensitivity ceiling.
    #[must_use]
    pub const fn maximum_sensitivity(&self) -> WorkReportSensitivity {
        self.maximum_sensitivity
    }
}

impl fmt::Debug for RequiredContextRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequiredContextRequirement")
            .field("requirement_id", &"[REDACTED]")
            .field("target_kind", &self.target.kind())
            .field("target_id", &"[REDACTED]")
            .field("access_level", &self.access_level)
            .field("obligation", &self.obligation)
            .field("maximum_sensitivity", &self.maximum_sensitivity)
            .finish()
    }
}

impl<'de> Deserialize<'de> for RequiredContextRequirement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            requirement_id: RequiredContextRequirementId,
            target: GovernedContextReferenceTarget,
            access_level: GovernedContextAccessLevel,
            obligation: RequiredContextObligation,
            maximum_sensitivity: WorkReportSensitivity,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.requirement_id,
            wire.target,
            wire.access_level,
            wire.obligation,
            wire.maximum_sensitivity,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// Immutable, content-addressed required-context contract binding.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct RequiredContextContractBinding {
    contract_id: HarnessContractId,
    contract_version: HarnessContractVersion,
    content_hash: SpecContentHash,
    requirements: Vec<RequiredContextRequirement>,
}

impl RequiredContextContractBinding {
    /// Creates a canonical binding and computes its content hash.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error for empty or duplicate requirements.
    pub fn new(
        contract_id: HarnessContractId,
        contract_version: HarnessContractVersion,
        mut requirements: Vec<RequiredContextRequirement>,
    ) -> Result<Self, WorkflowOsError> {
        requirements.sort_by_key(requirement_key);
        validate_requirements(&requirements)?;
        let content_hash = compute_contract_hash(&contract_id, &contract_version, &requirements);
        Ok(Self {
            contract_id,
            contract_version,
            content_hash,
            requirements,
        })
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_requirements(&self.requirements)?;
        if !is_canonically_ordered(&self.requirements) {
            return Err(validation_error(
                "required_context.contract.requirements_unordered",
                "required context requirements must use canonical order",
            ));
        }
        let expected = compute_contract_hash(
            &self.contract_id,
            &self.contract_version,
            &self.requirements,
        );
        if self.content_hash != expected {
            return Err(validation_error(
                "required_context.contract.content_hash_mismatch",
                "required context contract content hash does not match its requirements",
            ));
        }
        Ok(())
    }

    /// Returns the harness contract ID.
    #[must_use]
    pub const fn contract_id(&self) -> &HarnessContractId {
        &self.contract_id
    }

    /// Returns the harness contract version.
    #[must_use]
    pub const fn contract_version(&self) -> &HarnessContractVersion {
        &self.contract_version
    }

    /// Returns the canonical content hash.
    #[must_use]
    pub const fn content_hash(&self) -> &SpecContentHash {
        &self.content_hash
    }

    /// Returns the canonical typed requirements.
    #[must_use]
    pub fn requirements(&self) -> &[RequiredContextRequirement] {
        &self.requirements
    }
}

impl fmt::Debug for RequiredContextContractBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequiredContextContractBinding")
            .field("contract_id", &"[REDACTED]")
            .field("contract_version", &"[REDACTED]")
            .field("content_hash", &"[REDACTED]")
            .field("requirement_count", &self.requirements.len())
            .finish()
    }
}

impl<'de> Deserialize<'de> for RequiredContextContractBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            contract_id: HarnessContractId,
            contract_version: HarnessContractVersion,
            content_hash: SpecContentHash,
            requirements: Vec<RequiredContextRequirement>,
        }

        let wire = Wire::deserialize(deserializer)?;
        let binding = Self {
            contract_id: wire.contract_id,
            contract_version: wire.contract_version,
            content_hash: wire.content_hash,
            requirements: wire.requirements,
        };
        binding.validate().map_err(serde::de::Error::custom)?;
        Ok(binding)
    }
}

/// Exact execution context for one required-context consumption decision.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RequiredContextConsumptionContext {
    actor: ActorId,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    harness_contract_id: HarnessContractId,
    evaluated_at: Timestamp,
}

impl RequiredContextConsumptionContext {
    /// Creates an explicit payload-free execution context.
    #[must_use]
    pub const fn new(
        actor: ActorId,
        workflow_id: WorkflowId,
        run_id: WorkflowRunId,
        step_id: StepId,
        harness_contract_id: HarnessContractId,
        evaluated_at: Timestamp,
    ) -> Self {
        Self {
            actor,
            workflow_id,
            run_id,
            step_id,
            harness_contract_id,
            evaluated_at,
        }
    }

    /// Returns the actor consuming context.
    #[must_use]
    pub const fn actor(&self) -> &ActorId {
        &self.actor
    }

    /// Returns the exact workflow boundary.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the exact run boundary.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the exact step boundary.
    #[must_use]
    pub const fn step_id(&self) -> &StepId {
        &self.step_id
    }

    /// Returns the exact harness contract boundary.
    #[must_use]
    pub const fn harness_contract_id(&self) -> &HarnessContractId {
        &self.harness_contract_id
    }

    /// Returns the exact projection evaluation time.
    #[must_use]
    pub const fn evaluated_at(&self) -> Timestamp {
        self.evaluated_at
    }
}

impl fmt::Debug for RequiredContextConsumptionContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequiredContextConsumptionContext")
            .field("actor", &"[REDACTED]")
            .field("workflow_id", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("step_id", &"[REDACTED]")
            .field("harness_contract_id", &"[REDACTED]")
            .field("evaluated_at", &self.evaluated_at)
            .finish()
    }
}

/// Overall contract-consumption posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredContextConsumptionPosture {
    /// Every required requirement is satisfied.
    Satisfied,
    /// At least one required requirement is unsatisfied.
    Blocked,
}

impl<'de> Deserialize<'de> for RequiredContextConsumptionPosture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "satisfied" => Ok(Self::Satisfied),
            "blocked" => Ok(Self::Blocked),
            _ => Err(serde::de::Error::custom(validation_error(
                "required_context.consumption.posture_invalid",
                "required context consumption posture is invalid",
            ))),
        }
    }
}

/// Stable bounded reason that a requirement was not satisfied.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredContextGapReason {
    /// The target was declared unavailable.
    Unavailable,
    /// Current target availability is unknown.
    UnknownAvailability,
    /// No exact current authority authorizes the target.
    NoMatchingAuthority,
    /// Independent policy evaluation remains required.
    IndependentPolicyEvaluationRequired,
    /// Independent approval evaluation remains required.
    IndependentApprovalEvaluationRequired,
    /// Independent evidence or check evaluation remains required.
    IndependentEvidenceOrCheckEvaluationRequired,
    /// The target exceeds the projection sensitivity ceiling.
    ProjectionSensitivityCeilingExceeded,
    /// The target exceeds the contract requirement sensitivity ceiling.
    RequirementSensitivityCeilingExceeded,
    /// The requested access level is unavailable or unsupported.
    AccessLevelNotAuthorized,
}

impl<'de> Deserialize<'de> for RequiredContextGapReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "unavailable" => Ok(Self::Unavailable),
            "unknown_availability" => Ok(Self::UnknownAvailability),
            "no_matching_authority" => Ok(Self::NoMatchingAuthority),
            "independent_policy_evaluation_required" => {
                Ok(Self::IndependentPolicyEvaluationRequired)
            }
            "independent_approval_evaluation_required" => {
                Ok(Self::IndependentApprovalEvaluationRequired)
            }
            "independent_evidence_or_check_evaluation_required" => {
                Ok(Self::IndependentEvidenceOrCheckEvaluationRequired)
            }
            "projection_sensitivity_ceiling_exceeded" => {
                Ok(Self::ProjectionSensitivityCeilingExceeded)
            }
            "requirement_sensitivity_ceiling_exceeded" => {
                Ok(Self::RequirementSensitivityCeilingExceeded)
            }
            "access_level_not_authorized" => Ok(Self::AccessLevelNotAuthorized),
            _ => Err(serde::de::Error::custom(validation_error(
                "required_context.gap.reason_invalid",
                "required context gap reason is invalid",
            ))),
        }
    }
}

/// Payload-free record that one requirement was satisfied.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RequiredContextSatisfaction {
    requirement_id: RequiredContextRequirementId,
    target_kind: GovernedContextReferenceKind,
    access_level: GovernedContextAccessLevel,
}

impl RequiredContextSatisfaction {
    /// Returns the requirement ID.
    #[must_use]
    pub const fn requirement_id(&self) -> &RequiredContextRequirementId {
        &self.requirement_id
    }

    /// Returns the stable target kind.
    #[must_use]
    pub const fn target_kind(&self) -> GovernedContextReferenceKind {
        self.target_kind
    }

    /// Returns the exact satisfied access level.
    #[must_use]
    pub const fn access_level(&self) -> GovernedContextAccessLevel {
        self.access_level
    }
}

impl fmt::Debug for RequiredContextSatisfaction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequiredContextSatisfaction")
            .field("requirement_id", &"[REDACTED]")
            .field("target_kind", &self.target_kind)
            .field("access_level", &self.access_level)
            .finish()
    }
}

/// Payload-free record that one requirement was not satisfied.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RequiredContextGap {
    requirement_id: RequiredContextRequirementId,
    target_kind: GovernedContextReferenceKind,
    obligation: RequiredContextObligation,
    reason: RequiredContextGapReason,
}

impl RequiredContextGap {
    /// Returns the requirement ID.
    #[must_use]
    pub const fn requirement_id(&self) -> &RequiredContextRequirementId {
        &self.requirement_id
    }

    /// Returns the stable target kind.
    #[must_use]
    pub const fn target_kind(&self) -> GovernedContextReferenceKind {
        self.target_kind
    }

    /// Returns whether the gap blocks consumption.
    #[must_use]
    pub const fn obligation(&self) -> RequiredContextObligation {
        self.obligation
    }

    /// Returns the bounded gap reason.
    #[must_use]
    pub const fn reason(&self) -> RequiredContextGapReason {
        self.reason
    }
}

impl fmt::Debug for RequiredContextGap {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequiredContextGap")
            .field("requirement_id", &"[REDACTED]")
            .field("target_kind", &self.target_kind)
            .field("obligation", &self.obligation)
            .field("reason", &self.reason)
            .finish()
    }
}

/// Borrowed inputs for pure required-context contract consumption.
pub struct RequiredContextConsumptionInput<'a> {
    /// Immutable typed contract binding.
    pub contract: &'a RequiredContextContractBinding,
    /// Independently declared exact execution context.
    pub context: &'a RequiredContextConsumptionContext,
    /// Complete projections, at most one per requested access level.
    pub projections: &'a [GovernedContextProjection],
}

impl fmt::Debug for RequiredContextConsumptionInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequiredContextConsumptionInput")
            .field("contract", &self.contract)
            .field("context", &self.context)
            .field("projection_count", &self.projections.len())
            .finish()
    }
}

/// Deterministic payload-free required-context consumption result.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct RequiredContextConsumptionResult {
    contract: RequiredContextContractBinding,
    context: RequiredContextConsumptionContext,
    projections: Vec<GovernedContextProjection>,
    satisfactions: Vec<RequiredContextSatisfaction>,
    gaps: Vec<RequiredContextGap>,
    posture: RequiredContextConsumptionPosture,
}

impl RequiredContextConsumptionResult {
    /// Validates and recomputes the exact result.
    ///
    /// # Errors
    ///
    /// Returns a stable error when source contract, projections, or derived
    /// output are inconsistent.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.contract.validate()?;
        validate_projection_set(&self.contract, &self.context, &self.projections)?;
        let (satisfactions, gaps, posture) = derive_consumption(&self.contract, &self.projections)?;
        if self.satisfactions != satisfactions || self.gaps != gaps || self.posture != posture {
            return Err(validation_error(
                "required_context.consumption.derivation_inconsistent",
                "required context consumption result must match retained sources",
            ));
        }
        Ok(())
    }

    /// Returns the immutable source contract.
    #[must_use]
    pub const fn contract(&self) -> &RequiredContextContractBinding {
        &self.contract
    }

    /// Returns the independently declared exact execution context.
    #[must_use]
    pub const fn context(&self) -> &RequiredContextConsumptionContext {
        &self.context
    }

    /// Returns the canonical source projections.
    #[must_use]
    pub fn projections(&self) -> &[GovernedContextProjection] {
        &self.projections
    }

    /// Returns satisfied requirements.
    #[must_use]
    pub fn satisfactions(&self) -> &[RequiredContextSatisfaction] {
        &self.satisfactions
    }

    /// Returns required and optional gaps.
    #[must_use]
    pub fn gaps(&self) -> &[RequiredContextGap] {
        &self.gaps
    }

    /// Returns the overall consumption posture.
    #[must_use]
    pub const fn posture(&self) -> RequiredContextConsumptionPosture {
        self.posture
    }
}

impl fmt::Debug for RequiredContextConsumptionResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequiredContextConsumptionResult")
            .field("contract", &self.contract)
            .field("context", &self.context)
            .field("projection_count", &self.projections.len())
            .field("satisfaction_count", &self.satisfactions.len())
            .field("gaps", &self.gaps)
            .field("posture", &self.posture)
            .finish()
    }
}

impl<'de> Deserialize<'de> for RequiredContextConsumptionResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            contract: RequiredContextContractBinding,
            context: RequiredContextConsumptionContext,
            projections: Vec<GovernedContextProjection>,
            satisfactions: Vec<RequiredContextSatisfaction>,
            gaps: Vec<RequiredContextGap>,
            posture: RequiredContextConsumptionPosture,
        }

        let wire = Wire::deserialize(deserializer)?;
        let result = Self {
            contract: wire.contract,
            context: wire.context,
            projections: wire.projections,
            satisfactions: wire.satisfactions,
            gaps: wire.gaps,
            posture: wire.posture,
        };
        result.validate().map_err(serde::de::Error::custom)?;
        Ok(result)
    }
}

/// Consumes exact typed required-context declarations against governed projections.
///
/// This helper is pure and payload-free. It does not dereference targets,
/// inspect repositories, mutate runtime state, emit events, or invoke tools,
/// providers, sandboxes, or writes.
///
/// # Errors
///
/// Returns a stable non-leaking error for incomplete, overbroad, duplicate,
/// mismatched, or non-canonical source inputs.
pub fn consume_required_context(
    input: &RequiredContextConsumptionInput<'_>,
) -> Result<RequiredContextConsumptionResult, WorkflowOsError> {
    input.contract.validate()?;
    let mut projections = input.projections.to_vec();
    projections.sort_by_key(GovernedContextProjection::requested_access_level);
    validate_projection_set(input.contract, input.context, &projections)?;
    let (satisfactions, gaps, posture) = derive_consumption(input.contract, &projections)?;
    let result = RequiredContextConsumptionResult {
        contract: input.contract.clone(),
        context: input.context.clone(),
        projections,
        satisfactions,
        gaps,
        posture,
    };
    result.validate()?;
    Ok(result)
}

fn validate_requirements(
    requirements: &[RequiredContextRequirement],
) -> Result<(), WorkflowOsError> {
    if requirements.is_empty() {
        return Err(validation_error(
            "required_context.contract.requirements_empty",
            "required context contract needs at least one requirement",
        ));
    }
    let mut ids = BTreeSet::new();
    let mut targets = BTreeSet::new();
    for requirement in requirements {
        requirement.validate()?;
        if !ids.insert(requirement.requirement_id.as_str()) {
            return Err(validation_error(
                "required_context.contract.requirement_id_duplicate",
                "required context contract cannot contain duplicate requirement IDs",
            ));
        }
        if !targets.insert(target_key(&requirement.target)) {
            return Err(validation_error(
                "required_context.contract.target_duplicate",
                "required context contract cannot contain duplicate targets",
            ));
        }
    }
    Ok(())
}

fn validate_projection_set(
    contract: &RequiredContextContractBinding,
    context: &RequiredContextConsumptionContext,
    projections: &[GovernedContextProjection],
) -> Result<(), WorkflowOsError> {
    if context.harness_contract_id() != contract.contract_id() {
        return Err(validation_error(
            "required_context.consumption.contract_context_mismatch",
            "required context contract does not match the expected execution context",
        ));
    }
    if projections.is_empty() {
        return Err(validation_error(
            "required_context.consumption.projections_empty",
            "required context consumption needs at least one projection",
        ));
    }
    let mut access_levels = BTreeSet::new();
    for projection in projections {
        projection.validate()?;
        if !access_levels.insert(projection.requested_access_level()) {
            return Err(validation_error(
                "required_context.consumption.projection_access_duplicate",
                "required context consumption allows one projection per access level",
            ));
        }
        if projection.actor() != context.actor()
            || projection.workflow_id() != context.workflow_id()
            || projection.run_id() != context.run_id()
            || projection.step_id() != context.step_id()
            || projection.harness_contract_id() != Some(context.harness_contract_id())
            || projection.projected_at() != context.evaluated_at()
        {
            return Err(validation_error(
                "required_context.consumption.projection_context_mismatch",
                "required context projection does not match expected execution context",
            ));
        }
    }
    if !projections
        .windows(2)
        .all(|pair| pair[0].requested_access_level() < pair[1].requested_access_level())
    {
        return Err(validation_error(
            "required_context.consumption.projections_unordered",
            "required context projections must use canonical access order",
        ));
    }

    let mut candidate_targets = Vec::new();
    for projection in projections {
        for candidate in projection.candidates() {
            candidate_targets.push((
                target_key(candidate.reference().target()),
                candidate.requested_access_level(),
            ));
        }
    }
    candidate_targets.sort();
    if candidate_targets
        .windows(2)
        .any(|pair| pair[0].0 == pair[1].0)
    {
        return Err(validation_error(
            "required_context.consumption.candidate_target_duplicate",
            "required context projections cannot repeat a target",
        ));
    }
    let mut requirement_targets = contract
        .requirements
        .iter()
        .map(|requirement| (target_key(&requirement.target), requirement.access_level))
        .collect::<Vec<_>>();
    requirement_targets.sort();
    if candidate_targets != requirement_targets {
        return Err(validation_error(
            "required_context.consumption.target_set_mismatch",
            "required context projections must exactly match declared targets and access levels",
        ));
    }
    Ok(())
}

fn derive_consumption(
    contract: &RequiredContextContractBinding,
    projections: &[GovernedContextProjection],
) -> Result<
    (
        Vec<RequiredContextSatisfaction>,
        Vec<RequiredContextGap>,
        RequiredContextConsumptionPosture,
    ),
    WorkflowOsError,
> {
    let mut satisfactions = Vec::new();
    let mut gaps = Vec::new();
    for requirement in &contract.requirements {
        let projection = projections
            .iter()
            .find(|projection| projection.requested_access_level() == requirement.access_level)
            .ok_or_else(|| {
                validation_error(
                    "required_context.consumption.projection_missing",
                    "required context projection is missing",
                )
            })?;
        let candidate = projection
            .candidates()
            .iter()
            .find(|candidate| candidate.reference().target() == &requirement.target)
            .ok_or_else(|| {
                validation_error(
                    "required_context.consumption.candidate_missing",
                    "required context candidate is missing",
                )
            })?;
        let entry = projection
            .entries()
            .iter()
            .find(|entry| entry.reference().target() == &requirement.target);
        if candidate.reference().sensitivity() > requirement.maximum_sensitivity {
            gaps.push(gap(
                requirement,
                RequiredContextGapReason::RequirementSensitivityCeilingExceeded,
            ));
        } else if let Some(entry) = entry {
            validate_exact_entry(requirement, entry)?;
            satisfactions.push(RequiredContextSatisfaction {
                requirement_id: requirement.requirement_id.clone(),
                target_kind: requirement.target.kind(),
                access_level: requirement.access_level,
            });
        } else {
            gaps.push(gap(
                requirement,
                map_projection_gap_reason(candidate, projection.maximum_allowed_sensitivity()),
            ));
        }
    }
    let posture = if gaps
        .iter()
        .any(|gap| gap.obligation == RequiredContextObligation::Required)
    {
        RequiredContextConsumptionPosture::Blocked
    } else {
        RequiredContextConsumptionPosture::Satisfied
    };
    Ok((satisfactions, gaps, posture))
}

fn validate_exact_entry(
    requirement: &RequiredContextRequirement,
    entry: &GovernedContextProjectionEntry,
) -> Result<(), WorkflowOsError> {
    if entry.reference().target() != &requirement.target
        || entry.access_level() != requirement.access_level
    {
        return Err(validation_error(
            "required_context.consumption.entry_mismatch",
            "required context entry does not exactly match its requirement",
        ));
    }
    Ok(())
}

fn gap(
    requirement: &RequiredContextRequirement,
    reason: RequiredContextGapReason,
) -> RequiredContextGap {
    RequiredContextGap {
        requirement_id: requirement.requirement_id.clone(),
        target_kind: requirement.target.kind(),
        obligation: requirement.obligation,
        reason,
    }
}

fn map_projection_gap_reason(
    candidate: &crate::GovernedContextProjectionCandidate,
    sensitivity_ceiling: WorkReportSensitivity,
) -> RequiredContextGapReason {
    match candidate.reference().availability() {
        GovernedContextAvailability::Unavailable => return RequiredContextGapReason::Unavailable,
        GovernedContextAvailability::Unknown => {
            return RequiredContextGapReason::UnknownAvailability;
        }
        GovernedContextAvailability::Available => {}
    }
    if candidate.reference().sensitivity() > sensitivity_ceiling {
        return RequiredContextGapReason::ProjectionSensitivityCeilingExceeded;
    }
    match candidate.source_resolution().posture() {
        CapabilityResolutionPosture::Authorized => {
            RequiredContextGapReason::AccessLevelNotAuthorized
        }
        CapabilityResolutionPosture::RequiresIndependentEvaluation => {
            if candidate
                .source_resolution()
                .reasons()
                .contains(&CapabilityResolutionReason::PolicyEvaluationRequired)
            {
                RequiredContextGapReason::IndependentPolicyEvaluationRequired
            } else if candidate
                .source_resolution()
                .reasons()
                .contains(&CapabilityResolutionReason::ApprovalEvaluationRequired)
            {
                RequiredContextGapReason::IndependentApprovalEvaluationRequired
            } else {
                RequiredContextGapReason::IndependentEvidenceOrCheckEvaluationRequired
            }
        }
        CapabilityResolutionPosture::NotAuthorized => {
            if candidate
                .source_resolution()
                .reasons()
                .iter()
                .any(|reason| {
                    matches!(
                        reason,
                        CapabilityResolutionReason::CapabilityNotConnected
                            | CapabilityResolutionReason::CapabilityUnsupported
                            | CapabilityResolutionReason::CapabilityAvailabilityUnknown
                    )
                })
            {
                RequiredContextGapReason::AccessLevelNotAuthorized
            } else {
                RequiredContextGapReason::NoMatchingAuthority
            }
        }
    }
}

fn requirement_key(requirement: &RequiredContextRequirement) -> (String, String) {
    (
        target_key(&requirement.target),
        requirement.requirement_id.as_str().to_owned(),
    )
}

fn is_canonically_ordered(requirements: &[RequiredContextRequirement]) -> bool {
    requirements
        .windows(2)
        .all(|pair| requirement_key(&pair[0]) < requirement_key(&pair[1]))
}

fn target_key(target: &GovernedContextReferenceTarget) -> String {
    let (kind, id) = match target {
        GovernedContextReferenceTarget::EvidenceReference(value) => {
            ("evidence-reference", value.as_str())
        }
        GovernedContextReferenceTarget::WorkflowEvent(value) => ("workflow-event", value.as_str()),
        GovernedContextReferenceTarget::AuditEvent(value) => ("audit-event", value.as_str()),
        GovernedContextReferenceTarget::ValidationDiagnostic(value) => {
            ("validation-diagnostic", value.as_str())
        }
        GovernedContextReferenceTarget::ApprovalDecision(value) => {
            ("approval-decision", value.as_str())
        }
        GovernedContextReferenceTarget::PolicyDecision(value) => {
            ("policy-decision", value.as_str())
        }
        GovernedContextReferenceTarget::SideEffect(value) => ("side-effect", value.as_str()),
        GovernedContextReferenceTarget::TypedHandoff(value) => ("typed-handoff", value.as_str()),
        GovernedContextReferenceTarget::WorkReport(value) => ("work-report", value.as_str()),
    };
    format!("{kind}/{id}")
}

fn compute_contract_hash(
    contract_id: &HarnessContractId,
    contract_version: &HarnessContractVersion,
    requirements: &[RequiredContextRequirement],
) -> SpecContentHash {
    let mut bytes = Vec::new();
    append_framed(&mut bytes, b"required-context-contract-v1");
    append_framed(&mut bytes, contract_id.as_str().as_bytes());
    append_framed(&mut bytes, contract_version.as_str().as_bytes());
    append_framed(&mut bytes, &(requirements.len() as u64).to_be_bytes());
    for requirement in requirements {
        append_framed(&mut bytes, requirement.requirement_id.as_str().as_bytes());
        append_framed(&mut bytes, target_key(&requirement.target).as_bytes());
        append_framed(
            &mut bytes,
            match requirement.access_level {
                GovernedContextAccessLevel::ReferenceOnly => b"reference-only",
                GovernedContextAccessLevel::BoundedMetadata => b"bounded-metadata",
            },
        );
        append_framed(
            &mut bytes,
            match requirement.obligation {
                RequiredContextObligation::Required => b"required",
                RequiredContextObligation::Optional => b"optional",
            },
        );
        append_framed(
            &mut bytes,
            sensitivity_name(requirement.maximum_sensitivity).as_bytes(),
        );
    }
    SpecContentHash::from_bytes(bytes)
}

fn append_framed(buffer: &mut Vec<u8>, value: &[u8]) {
    buffer.extend_from_slice(&(value.len() as u64).to_be_bytes());
    buffer.extend_from_slice(value);
}

fn sensitivity_name(sensitivity: WorkReportSensitivity) -> &'static str {
    match sensitivity {
        WorkReportSensitivity::Public => "public",
        WorkReportSensitivity::Internal => "internal",
        WorkReportSensitivity::Confidential => "confidential",
        WorkReportSensitivity::Regulated => "regulated",
        WorkReportSensitivity::Secret => "secret",
        WorkReportSensitivity::Unknown => "unknown",
    }
}

fn validate_requirement_id(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty()
        || value.len() > REQUIREMENT_ID_MAX_BYTES
        || !value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'.' | b'_' | b'-' | b':')
        })
    {
        return Err(validation_error(
            "required_context.requirement.id_invalid",
            "required context requirement ID is invalid",
        ));
    }
    validate_not_secret_like(value)
}

fn validate_not_secret_like(value: &str) -> Result<(), WorkflowOsError> {
    let normalized = value.to_ascii_lowercase();
    if [
        "authorization",
        "bearer",
        "credential",
        "password",
        "private_key",
        "private-key",
        "secret",
        "token",
        "api_key",
        "api-key",
    ]
    .iter()
    .any(|needle| normalized.contains(needle))
    {
        return Err(validation_error(
            "required_context.value.secret_like",
            "required context value is not allowed",
        ));
    }
    Ok(())
}

fn validation_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}
