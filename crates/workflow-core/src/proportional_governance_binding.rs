use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    GovernanceAssessmentCompleteness, GovernanceDisclosureRequirement,
    GovernanceExecutionDisposition, GovernanceRuntimeFactSnapshot,
    GovernanceRuntimeFactSnapshotBinding, ImmutableBundleGovernanceAssessmentSet,
    ImmutableRunBundleBinding, SpecContentHash, StepId, StoredImmutableRunBundle, WorkflowId,
    WorkflowOsError, WorkflowRunId,
};

const MAX_BOUND_STEP_COUNT: u32 = 1_024;

/// Version of the durable proportional-governance assessment-binding model.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceAssessmentBindingVersion {
    /// Initial immutable-bundle assessment binding.
    V1,
    /// Assessment binding with an optional authoritative fact-source commitment.
    V2,
    /// Assessment binding with an initial current-runtime-fact snapshot commitment.
    V3,
}

impl<'de> Deserialize<'de> for GovernanceAssessmentBindingVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "v1" => Ok(Self::V1),
            "v2" => Ok(Self::V2),
            "v3" => Ok(Self::V3),
            _ => Err(serde::de::Error::custom(
                "governance assessment binding version is invalid",
            )),
        }
    }
}

/// Bounded kind of authoritative source committed by a governance binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceAssessmentSourceKind {
    /// Same-call authoritative local-check reassessment.
    AuthoritativeLocalCheckReassessment,
}

impl GovernanceAssessmentSourceKind {
    /// Returns the stable bounded source-kind identifier.
    #[must_use]
    pub const fn identifier(self) -> &'static str {
        match self {
            Self::AuthoritativeLocalCheckReassessment => "authoritative_local_check_reassessment",
        }
    }
}

impl<'de> Deserialize<'de> for GovernanceAssessmentSourceKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "authoritative_local_check_reassessment" => {
                Ok(Self::AuthoritativeLocalCheckReassessment)
            }
            _ => Err(serde::de::Error::custom(
                "governance assessment source kind is invalid",
            )),
        }
    }
}

/// Versioned algorithm for an authoritative assessment-source commitment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceAssessmentSourceAlgorithm {
    /// Initial same-call local-check reassessment commitment.
    V1,
}

impl GovernanceAssessmentSourceAlgorithm {
    /// Returns the stable domain identifier.
    #[must_use]
    pub const fn identifier(self) -> &'static str {
        match self {
            Self::V1 => "workflow-os/authoritative-local-check-reassessment-binding/v1",
        }
    }
}

impl<'de> Deserialize<'de> for GovernanceAssessmentSourceAlgorithm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "v1" => Ok(Self::V1),
            _ => Err(serde::de::Error::custom(
                "governance assessment source algorithm is invalid",
            )),
        }
    }
}

/// Payload-free commitment to the authoritative runtime source of an assessment.
///
/// This record is an integrity commitment, not standalone proof that a check
/// ran. Runtime authority exists only when Core derives it from the same-call
/// private reassessment and matches the exact create-only stored binding.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceAssessmentSourceBinding {
    kind: GovernanceAssessmentSourceKind,
    algorithm: GovernanceAssessmentSourceAlgorithm,
    fingerprint: SpecContentHash,
    selected_step_id: StepId,
}

impl GovernanceAssessmentSourceBinding {
    fn authoritative_local_check(selected_step_id: StepId, fingerprint: SpecContentHash) -> Self {
        Self {
            kind: GovernanceAssessmentSourceKind::AuthoritativeLocalCheckReassessment,
            algorithm: GovernanceAssessmentSourceAlgorithm::V1,
            fingerprint,
            selected_step_id,
        }
    }

    /// Returns the bounded source kind.
    #[must_use]
    pub const fn kind(&self) -> GovernanceAssessmentSourceKind {
        self.kind
    }

    /// Returns the source commitment algorithm.
    #[must_use]
    pub const fn algorithm(&self) -> GovernanceAssessmentSourceAlgorithm {
        self.algorithm
    }

    /// Returns the committed source fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> &SpecContentHash {
        &self.fingerprint
    }

    /// Returns the selected step whose fact source was committed.
    #[must_use]
    pub const fn selected_step_id(&self) -> &StepId {
        &self.selected_step_id
    }
}

impl fmt::Debug for GovernanceAssessmentSourceBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernanceAssessmentSourceBinding")
            .field("kind", &self.kind)
            .field("algorithm", &self.algorithm)
            .field("fingerprint", &"<redacted>")
            .field("selected_step_id", &"<redacted>")
            .finish()
    }
}

/// Versioned algorithm used to create an immutable-bundle assessment set.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceAssessmentSetAlgorithm {
    /// Initial workflow-ordered aggregate fingerprint algorithm.
    V1,
}

impl GovernanceAssessmentSetAlgorithm {
    /// Returns the stable algorithm identifier bound into the assessment set.
    #[must_use]
    pub const fn identifier(self) -> &'static str {
        match self {
            Self::V1 => "workflow-os/immutable-bundle-governance-assessment-set/v1",
        }
    }
}

impl<'de> Deserialize<'de> for GovernanceAssessmentSetAlgorithm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "v1" => Ok(Self::V1),
            _ => Err(serde::de::Error::custom(
                "governance assessment set algorithm is invalid",
            )),
        }
    }
}

/// Durable payload-free binding for one accepted immutable-bundle assessment set.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GovernanceAssessmentBinding {
    binding_version: GovernanceAssessmentBindingVersion,
    assessment_set_algorithm: GovernanceAssessmentSetAlgorithm,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    immutable_run_bundle: ImmutableRunBundleBinding,
    aggregate_fingerprint: SpecContentHash,
    step_count: u32,
    execution: GovernanceExecutionDisposition,
    disclosure: GovernanceDisclosureRequirement,
    completeness: GovernanceAssessmentCompleteness,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_binding: Option<GovernanceAssessmentSourceBinding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_fact_snapshot_binding: Option<GovernanceRuntimeFactSnapshotBinding>,
}

impl GovernanceAssessmentBinding {
    /// Builds a validated binding from one stored bundle and its accepted set.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when identity, count, or bundle
    /// relationships are inconsistent.
    pub fn from_assessment_set(
        bundle: &StoredImmutableRunBundle,
        assessment_set: &ImmutableBundleGovernanceAssessmentSet,
    ) -> Result<Self, WorkflowOsError> {
        Self::build(bundle, assessment_set, None, None)
    }

    pub(crate) fn from_authoritative_local_check_assessment(
        bundle: &StoredImmutableRunBundle,
        assessment_set: &ImmutableBundleGovernanceAssessmentSet,
        selected_step_id: StepId,
        source_fingerprint: SpecContentHash,
    ) -> Result<Self, WorkflowOsError> {
        Self::build(
            bundle,
            assessment_set,
            Some(
                GovernanceAssessmentSourceBinding::authoritative_local_check(
                    selected_step_id,
                    source_fingerprint,
                ),
            ),
            None,
        )
    }

    pub(crate) fn from_current_runtime_fact_assessment(
        bundle: &StoredImmutableRunBundle,
        assessment_set: &ImmutableBundleGovernanceAssessmentSet,
        snapshot: &GovernanceRuntimeFactSnapshot,
    ) -> Result<Self, WorkflowOsError> {
        let snapshot_binding = snapshot.commitment_binding()?;
        if snapshot_binding.immutable_run_bundle() != &bundle.manifest().run_binding()
            || snapshot_binding.assessment_aggregate_fingerprint()
                != assessment_set.aggregate_fingerprint()
        {
            return Err(binding_error("runtime_fact_snapshot_mismatch"));
        }
        Self::build(bundle, assessment_set, None, Some(snapshot_binding))
    }

    fn build(
        bundle: &StoredImmutableRunBundle,
        assessment_set: &ImmutableBundleGovernanceAssessmentSet,
        source_binding: Option<GovernanceAssessmentSourceBinding>,
        runtime_fact_snapshot_binding: Option<GovernanceRuntimeFactSnapshotBinding>,
    ) -> Result<Self, WorkflowOsError> {
        if assessment_set.workflow_id() != bundle.manifest().workflow_id()
            || assessment_set.run_id() != bundle.manifest().run_id()
        {
            return Err(binding_error("identity_mismatch"));
        }
        if assessment_set.immutable_run_bundle() != &bundle.manifest().run_binding() {
            return Err(binding_error("bundle_mismatch"));
        }

        let step_count = u32::try_from(assessment_set.assessments().len())
            .map_err(|_| binding_error("step_count_invalid"))?;
        validate_step_count(step_count)?;

        let execution = assessment_set
            .assessments()
            .iter()
            .map(|item| item.assessment().decision().execution())
            .max()
            .ok_or_else(|| binding_error("step_count_invalid"))?;
        let disclosure = assessment_set
            .assessments()
            .iter()
            .map(|item| item.assessment().decision().disclosure())
            .max()
            .ok_or_else(|| binding_error("step_count_invalid"))?;
        let completeness = if assessment_set.assessments().iter().any(|item| {
            item.assessment().completeness() == GovernanceAssessmentCompleteness::Incomplete
        }) {
            GovernanceAssessmentCompleteness::Incomplete
        } else {
            GovernanceAssessmentCompleteness::Complete
        };

        let binding = Self {
            binding_version: match (
                source_binding.is_some(),
                runtime_fact_snapshot_binding.is_some(),
            ) {
                (false, false) => GovernanceAssessmentBindingVersion::V1,
                (true, false) => GovernanceAssessmentBindingVersion::V2,
                (false, true) => GovernanceAssessmentBindingVersion::V3,
                (true, true) => return Err(binding_error("source_binding_conflict")),
            },
            assessment_set_algorithm: assessment_set.algorithm(),
            workflow_id: assessment_set.workflow_id().clone(),
            run_id: assessment_set.run_id().clone(),
            immutable_run_bundle: assessment_set.immutable_run_bundle().clone(),
            aggregate_fingerprint: assessment_set.aggregate_fingerprint().clone(),
            step_count,
            execution,
            disclosure,
            completeness,
            source_binding,
            runtime_fact_snapshot_binding,
        };
        binding.validate()?;
        Ok(binding)
    }

    /// Returns the binding model version.
    #[must_use]
    pub const fn binding_version(&self) -> GovernanceAssessmentBindingVersion {
        self.binding_version
    }

    /// Returns the assessment-set algorithm.
    #[must_use]
    pub const fn assessment_set_algorithm(&self) -> GovernanceAssessmentSetAlgorithm {
        self.assessment_set_algorithm
    }

    /// Returns the bound workflow identity.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the bound run identity.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the immutable bundle identity and integrity root.
    #[must_use]
    pub const fn immutable_run_bundle(&self) -> &ImmutableRunBundleBinding {
        &self.immutable_run_bundle
    }

    /// Returns the aggregate assessment-set fingerprint.
    #[must_use]
    pub const fn aggregate_fingerprint(&self) -> &SpecContentHash {
        &self.aggregate_fingerprint
    }

    /// Returns the number of ordered step assessments bound by this record.
    #[must_use]
    pub const fn step_count(&self) -> u32 {
        self.step_count
    }

    /// Returns the strictest execution disposition in the assessment set.
    #[must_use]
    pub const fn execution(&self) -> GovernanceExecutionDisposition {
        self.execution
    }

    /// Returns the strictest disclosure requirement in the assessment set.
    #[must_use]
    pub const fn disclosure(&self) -> GovernanceDisclosureRequirement {
        self.disclosure
    }

    /// Returns aggregate deterministic fact completeness.
    #[must_use]
    pub const fn completeness(&self) -> GovernanceAssessmentCompleteness {
        self.completeness
    }

    /// Returns the optional authoritative fact-source commitment.
    #[must_use]
    pub const fn source_binding(&self) -> Option<&GovernanceAssessmentSourceBinding> {
        self.source_binding.as_ref()
    }

    /// Returns the initial current-runtime-fact snapshot commitment, when present.
    #[must_use]
    pub const fn runtime_fact_snapshot_binding(
        &self,
    ) -> Option<&GovernanceRuntimeFactSnapshotBinding> {
        self.runtime_fact_snapshot_binding.as_ref()
    }

    /// Returns whether this binding carries an accepted authoritative fact commitment.
    ///
    /// V2 binds a fixed authoritative source assessment. V3 binds a current-runtime-fact
    /// snapshot. The two forms are mutually exclusive, but both are authoritative inputs.
    #[must_use]
    pub const fn has_authoritative_fact_commitment(&self) -> bool {
        self.source_binding.is_some() || self.runtime_fact_snapshot_binding.is_some()
    }

    pub(crate) fn validate_current_runtime_fact_reassessment(
        &self,
        bundle: &StoredImmutableRunBundle,
        assessment_set: &ImmutableBundleGovernanceAssessmentSet,
        snapshot: &GovernanceRuntimeFactSnapshot,
    ) -> Result<(), WorkflowOsError> {
        let current = Self::from_current_runtime_fact_assessment(bundle, assessment_set, snapshot)?;
        self.validate_current_runtime_fact_binding(&current)
    }

    pub(crate) fn validate_current_runtime_fact_binding(
        &self,
        current: &Self,
    ) -> Result<(), WorkflowOsError> {
        let initial_snapshot = self
            .runtime_fact_snapshot_binding
            .as_ref()
            .ok_or_else(|| binding_error("runtime_fact_snapshot_missing"))?;
        let current_snapshot = current
            .runtime_fact_snapshot_binding
            .as_ref()
            .ok_or_else(|| binding_error("runtime_fact_snapshot_missing"))?;
        if self.binding_version != GovernanceAssessmentBindingVersion::V3
            || current.binding_version != GovernanceAssessmentBindingVersion::V3
            || !self.same_assessment_core(current)
            || initial_snapshot.source_registration_commitment()
                != current_snapshot.source_registration_commitment()
            || initial_snapshot.runtime_fact_commitment()
                != current_snapshot.runtime_fact_commitment()
            || initial_snapshot.runtime_fact_count() != current_snapshot.runtime_fact_count()
            || initial_snapshot.assessment_aggregate_fingerprint()
                != current_snapshot.assessment_aggregate_fingerprint()
        {
            return Err(binding_error("runtime_fact_reassessment_mismatch"));
        }
        Ok(())
    }

    fn same_assessment_core(&self, other: &Self) -> bool {
        self.assessment_set_algorithm == other.assessment_set_algorithm
            && self.workflow_id == other.workflow_id
            && self.run_id == other.run_id
            && self.immutable_run_bundle == other.immutable_run_bundle
            && self.aggregate_fingerprint == other.aggregate_fingerprint
            && self.step_count == other.step_count
            && self.execution == other.execution
            && self.disclosure == other.disclosure
            && self.completeness == other.completeness
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_step_count(self.step_count)?;
        match (
            self.binding_version,
            self.source_binding.is_some(),
            self.runtime_fact_snapshot_binding.is_some(),
        ) {
            (GovernanceAssessmentBindingVersion::V1, false, false)
            | (GovernanceAssessmentBindingVersion::V2, true, false)
            | (GovernanceAssessmentBindingVersion::V3, false, true) => {}
            _ => return Err(binding_error("source_binding_version_mismatch")),
        }
        if let Some(snapshot) = &self.runtime_fact_snapshot_binding {
            if snapshot.immutable_run_bundle() != &self.immutable_run_bundle
                || snapshot.assessment_aggregate_fingerprint() != &self.aggregate_fingerprint
            {
                return Err(binding_error("runtime_fact_snapshot_mismatch"));
            }
        }
        if self.execution != GovernanceExecutionDisposition::Proceed
            && self.disclosure != GovernanceDisclosureRequirement::Visible
        {
            return Err(binding_error("posture_invalid"));
        }
        Ok(())
    }
}

impl fmt::Debug for GovernanceAssessmentBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernanceAssessmentBinding")
            .field("binding_version", &self.binding_version)
            .field("assessment_set_algorithm", &self.assessment_set_algorithm)
            .field("workflow_id", &"<redacted>")
            .field("run_id", &"<redacted>")
            .field("immutable_run_bundle", &self.immutable_run_bundle)
            .field("aggregate_fingerprint", &"<redacted>")
            .field("step_count", &self.step_count)
            .field("execution", &self.execution)
            .field("disclosure", &self.disclosure)
            .field("completeness", &self.completeness)
            .field(
                "source_binding_kind",
                &self
                    .source_binding
                    .as_ref()
                    .map(GovernanceAssessmentSourceBinding::kind),
            )
            .field(
                "has_runtime_fact_snapshot_binding",
                &self.runtime_fact_snapshot_binding.is_some(),
            )
            .finish()
    }
}

impl<'de> Deserialize<'de> for GovernanceAssessmentBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            binding_version: GovernanceAssessmentBindingVersion,
            assessment_set_algorithm: GovernanceAssessmentSetAlgorithm,
            workflow_id: WorkflowId,
            run_id: WorkflowRunId,
            immutable_run_bundle: ImmutableRunBundleBinding,
            aggregate_fingerprint: SpecContentHash,
            step_count: u32,
            execution: GovernanceExecutionDisposition,
            disclosure: GovernanceDisclosureRequirement,
            completeness: GovernanceAssessmentCompleteness,
            #[serde(default)]
            source_binding: Option<GovernanceAssessmentSourceBinding>,
            #[serde(default)]
            runtime_fact_snapshot_binding: Option<GovernanceRuntimeFactSnapshotBinding>,
        }

        let wire = Wire::deserialize(deserializer)?;
        let binding = Self {
            binding_version: wire.binding_version,
            assessment_set_algorithm: wire.assessment_set_algorithm,
            workflow_id: wire.workflow_id,
            run_id: wire.run_id,
            immutable_run_bundle: wire.immutable_run_bundle,
            aggregate_fingerprint: wire.aggregate_fingerprint,
            step_count: wire.step_count,
            execution: wire.execution,
            disclosure: wire.disclosure,
            completeness: wire.completeness,
            source_binding: wire.source_binding,
            runtime_fact_snapshot_binding: wire.runtime_fact_snapshot_binding,
        };
        binding.validate().map_err(serde::de::Error::custom)?;
        Ok(binding)
    }
}

fn validate_step_count(step_count: u32) -> Result<(), WorkflowOsError> {
    if step_count == 0 || step_count > MAX_BOUND_STEP_COUNT {
        return Err(binding_error("step_count_invalid"));
    }
    Ok(())
}

fn binding_error(suffix: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("governance.proportional.assessment_binding.{suffix}"),
        "proportional-governance assessment binding is invalid",
    )
}
