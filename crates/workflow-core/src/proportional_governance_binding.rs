use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    GovernanceAssessmentCompleteness, GovernanceDisclosureRequirement,
    GovernanceExecutionDisposition, ImmutableBundleGovernanceAssessmentSet,
    ImmutableRunBundleBinding, SpecContentHash, StoredImmutableRunBundle, WorkflowId,
    WorkflowOsError, WorkflowRunId,
};

const MAX_BOUND_STEP_COUNT: u32 = 1_024;

/// Version of the durable proportional-governance assessment-binding model.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceAssessmentBindingVersion {
    /// Initial immutable-bundle assessment binding.
    V1,
}

impl<'de> Deserialize<'de> for GovernanceAssessmentBindingVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "v1" => Ok(Self::V1),
            _ => Err(serde::de::Error::custom(
                "governance assessment binding version is invalid",
            )),
        }
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
            binding_version: GovernanceAssessmentBindingVersion::V1,
            assessment_set_algorithm: assessment_set.algorithm(),
            workflow_id: assessment_set.workflow_id().clone(),
            run_id: assessment_set.run_id().clone(),
            immutable_run_bundle: assessment_set.immutable_run_bundle().clone(),
            aggregate_fingerprint: assessment_set.aggregate_fingerprint().clone(),
            step_count,
            execution,
            disclosure,
            completeness,
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

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_step_count(self.step_count)?;
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
