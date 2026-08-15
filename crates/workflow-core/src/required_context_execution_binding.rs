use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    ActorId, HarnessContractId, HarnessContractVersion, ImmutableRunBundleBinding,
    RequiredContextContractBinding, SpecContentHash, StepId, StoredImmutableRunBundle, Timestamp,
    WorkReportSensitivity, WorkflowId, WorkflowOsError, WorkflowRunId,
};

/// Versioned algorithm for required-context execution bindings.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredContextExecutionBindingVersion {
    /// Initial fixed-width framed SHA-256 binding.
    V1,
}

impl RequiredContextExecutionBindingVersion {
    /// Returns the domain-separated algorithm identifier.
    #[must_use]
    pub const fn identifier(self) -> &'static str {
        match self {
            Self::V1 => "workflow-os/required-context-execution-binding/v1",
        }
    }
}

impl<'de> Deserialize<'de> for RequiredContextExecutionBindingVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "v1" => Ok(Self::V1),
            _ => Err(serde::de::Error::custom(
                "required context execution binding version is invalid",
            )),
        }
    }
}

/// Explicit validated inputs for one immutable required-context execution binding.
pub struct RequiredContextExecutionBindingInput<'a> {
    /// Validated bundle loaded from the create-only immutable run-bundle store.
    pub bundle: &'a StoredImmutableRunBundle,
    /// Exact content-addressed required-context contract selected for the step.
    pub contract: &'a RequiredContextContractBinding,
    /// Actor expected to consume the governed context.
    pub actor: ActorId,
    /// Exact immutable workflow step.
    pub step_id: StepId,
    /// Maximum sensitivity accepted by the future consumer.
    pub maximum_sensitivity: WorkReportSensitivity,
    /// Time at which the immutable pre-consumption commitment is created.
    pub bound_at: Timestamp,
}

impl fmt::Debug for RequiredContextExecutionBindingInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequiredContextExecutionBindingInput")
            .field("bundle", &self.bundle)
            .field("contract", &self.contract)
            .field("actor", &"[REDACTED]")
            .field("step_id", &"[REDACTED]")
            .field("maximum_sensitivity", &self.maximum_sensitivity)
            .field("bound_at", &"[REDACTED]")
            .finish()
    }
}

/// Payload-free immutable commitment for one future required-context consumption boundary.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct RequiredContextExecutionBinding {
    binding_version: RequiredContextExecutionBindingVersion,
    immutable_run_bundle: ImmutableRunBundleBinding,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    actor: ActorId,
    harness_contract_id: HarnessContractId,
    harness_contract_version: HarnessContractVersion,
    contract_content_hash: SpecContentHash,
    maximum_sensitivity: WorkReportSensitivity,
    bound_at: Timestamp,
    binding_hash: SpecContentHash,
}

#[cfg(test)]
pub(crate) enum RequiredContextExecutionBindingTestSubstitution {
    ImmutableRunBundle(ImmutableRunBundleBinding),
    WorkflowId(WorkflowId),
    RunId(WorkflowRunId),
    StepId(StepId),
    Actor(ActorId),
    HarnessContractId(HarnessContractId),
    HarnessContractVersion(HarnessContractVersion),
    ContractContentHash(SpecContentHash),
}

impl RequiredContextExecutionBinding {
    /// Creates an immutable required-context execution commitment from validated sources.
    ///
    /// This proves exact pre-consumption identity. It does not authorize target
    /// access, resolve current capabilities, dereference payloads, or execute work.
    ///
    /// # Errors
    ///
    /// Returns stable non-leaking validation errors when the immutable workflow
    /// record is unavailable, the step is absent, sensitivity is unknown, or the
    /// binding timestamp predates the bundle.
    pub fn new(input: RequiredContextExecutionBindingInput<'_>) -> Result<Self, WorkflowOsError> {
        let manifest = input.bundle.manifest();
        let workflow_record = canonical_workflow_record(input.bundle)?;
        let workflow = workflow_record
            .canonical_definition()
            .as_workflow()
            .ok_or_else(|| {
                binding_error(
                    "bundle.workflow_missing",
                    "stored immutable run bundle is missing its canonical workflow record",
                )
            })?;
        if workflow.id != *manifest.workflow_id()
            || workflow.version != *manifest.workflow_version()
            || workflow.schema_version != *manifest.schema_version()
            || workflow_record.source_content_hash() != manifest.workflow_content_hash()
        {
            return Err(binding_error(
                "bundle.workflow_mismatch",
                "stored immutable workflow record does not match the bundle manifest",
            ));
        }
        if !workflow.steps.iter().any(|step| step.id == input.step_id) {
            return Err(binding_error(
                "step.not_found",
                "required context execution step is not present in the immutable workflow",
            ));
        }
        if input.maximum_sensitivity == WorkReportSensitivity::Unknown {
            return Err(binding_error(
                "sensitivity.unknown",
                "required context execution binding needs known maximum sensitivity",
            ));
        }
        if input.bound_at < *manifest.created_at() {
            return Err(binding_error(
                "bound_at.before_bundle",
                "required context execution binding cannot predate the immutable run bundle",
            ));
        }

        let mut binding = Self {
            binding_version: RequiredContextExecutionBindingVersion::V1,
            immutable_run_bundle: manifest.run_binding(),
            workflow_id: manifest.workflow_id().clone(),
            run_id: manifest.run_id().clone(),
            step_id: input.step_id,
            actor: input.actor,
            harness_contract_id: input.contract.contract_id().clone(),
            harness_contract_version: input.contract.contract_version().clone(),
            contract_content_hash: input.contract.content_hash().clone(),
            maximum_sensitivity: input.maximum_sensitivity,
            bound_at: input.bound_at,
            binding_hash: SpecContentHash::from_text("pending"),
        };
        binding.binding_hash = compute_binding_hash(&binding);
        binding.validate()?;
        Ok(binding)
    }

    /// Validates all internal commitments.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when serialized fields are inconsistent.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.maximum_sensitivity == WorkReportSensitivity::Unknown {
            return Err(binding_error(
                "sensitivity.unknown",
                "required context execution binding needs known maximum sensitivity",
            ));
        }
        if self.binding_hash != compute_binding_hash(self) {
            return Err(binding_error(
                "content_hash.mismatch",
                "required context execution binding content hash is invalid",
            ));
        }
        Ok(())
    }

    /// Returns the binding model version.
    #[must_use]
    pub const fn binding_version(&self) -> RequiredContextExecutionBindingVersion {
        self.binding_version
    }

    /// Returns the exact immutable run-bundle binding.
    #[must_use]
    pub const fn immutable_run_bundle(&self) -> &ImmutableRunBundleBinding {
        &self.immutable_run_bundle
    }

    /// Returns the immutable workflow identity.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the immutable run identity.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the exact immutable step identity.
    #[must_use]
    pub const fn step_id(&self) -> &StepId {
        &self.step_id
    }

    /// Returns the actor committed for future consumption.
    #[must_use]
    pub const fn actor(&self) -> &ActorId {
        &self.actor
    }

    /// Returns the exact harness contract identity.
    #[must_use]
    pub const fn harness_contract_id(&self) -> &HarnessContractId {
        &self.harness_contract_id
    }

    /// Returns the exact harness contract version.
    #[must_use]
    pub const fn harness_contract_version(&self) -> &HarnessContractVersion {
        &self.harness_contract_version
    }

    /// Returns the exact required-context contract content hash.
    #[must_use]
    pub const fn contract_content_hash(&self) -> &SpecContentHash {
        &self.contract_content_hash
    }

    /// Returns the committed sensitivity ceiling.
    #[must_use]
    pub const fn maximum_sensitivity(&self) -> WorkReportSensitivity {
        self.maximum_sensitivity
    }

    /// Returns the pre-consumption binding time.
    #[must_use]
    pub const fn bound_at(&self) -> Timestamp {
        self.bound_at
    }

    /// Returns the deterministic content hash over the complete binding.
    #[must_use]
    pub const fn binding_hash(&self) -> &SpecContentHash {
        &self.binding_hash
    }

    #[cfg(test)]
    pub(crate) fn with_test_substitution(
        &self,
        substitution: RequiredContextExecutionBindingTestSubstitution,
    ) -> Self {
        let mut binding = self.clone();
        match substitution {
            RequiredContextExecutionBindingTestSubstitution::ImmutableRunBundle(value) => {
                binding.immutable_run_bundle = value;
            }
            RequiredContextExecutionBindingTestSubstitution::WorkflowId(value) => {
                binding.workflow_id = value;
            }
            RequiredContextExecutionBindingTestSubstitution::RunId(value) => {
                binding.run_id = value;
            }
            RequiredContextExecutionBindingTestSubstitution::StepId(value) => {
                binding.step_id = value;
            }
            RequiredContextExecutionBindingTestSubstitution::Actor(value) => {
                binding.actor = value;
            }
            RequiredContextExecutionBindingTestSubstitution::HarnessContractId(value) => {
                binding.harness_contract_id = value;
            }
            RequiredContextExecutionBindingTestSubstitution::HarnessContractVersion(value) => {
                binding.harness_contract_version = value;
            }
            RequiredContextExecutionBindingTestSubstitution::ContractContentHash(value) => {
                binding.contract_content_hash = value;
            }
        }
        binding.binding_hash = compute_binding_hash(&binding);
        binding
    }
}

impl fmt::Debug for RequiredContextExecutionBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequiredContextExecutionBinding")
            .field("binding_version", &self.binding_version)
            .field("immutable_run_bundle", &self.immutable_run_bundle)
            .field("workflow_identity", &"[REDACTED]")
            .field("run_identity", &"[REDACTED]")
            .field("step_identity", &"[REDACTED]")
            .field("actor", &"[REDACTED]")
            .field("harness_contract_identity", &"[REDACTED]")
            .field("contract_content_hash", &"[REDACTED]")
            .field("maximum_sensitivity", &self.maximum_sensitivity)
            .field("bound_at", &"[REDACTED]")
            .field("binding_hash", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[derive(Deserialize)]
struct RequiredContextExecutionBindingWire {
    binding_version: RequiredContextExecutionBindingVersion,
    immutable_run_bundle: ImmutableRunBundleBinding,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    actor: ActorId,
    harness_contract_id: HarnessContractId,
    harness_contract_version: HarnessContractVersion,
    contract_content_hash: SpecContentHash,
    maximum_sensitivity: WorkReportSensitivity,
    bound_at: Timestamp,
    binding_hash: SpecContentHash,
}

impl<'de> Deserialize<'de> for RequiredContextExecutionBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = RequiredContextExecutionBindingWire::deserialize(deserializer)?;
        let binding = Self {
            binding_version: wire.binding_version,
            immutable_run_bundle: wire.immutable_run_bundle,
            workflow_id: wire.workflow_id,
            run_id: wire.run_id,
            step_id: wire.step_id,
            actor: wire.actor,
            harness_contract_id: wire.harness_contract_id,
            harness_contract_version: wire.harness_contract_version,
            contract_content_hash: wire.contract_content_hash,
            maximum_sensitivity: wire.maximum_sensitivity,
            bound_at: wire.bound_at,
            binding_hash: wire.binding_hash,
        };
        binding
            .validate()
            .map_err(|_| serde::de::Error::custom("invalid required context execution binding"))?;
        Ok(binding)
    }
}

fn canonical_workflow_record(
    bundle: &StoredImmutableRunBundle,
) -> Result<&crate::ImmutableRunBundleDefinitionRecord, WorkflowOsError> {
    let mut matches = bundle.definition_records().iter().filter(|record| {
        record
            .canonical_definition()
            .as_workflow()
            .is_some_and(|workflow| workflow.id == *bundle.manifest().workflow_id())
    });
    let workflow = matches.next().ok_or_else(|| {
        binding_error(
            "bundle.workflow_missing",
            "stored immutable run bundle is missing its canonical workflow record",
        )
    })?;
    if matches.next().is_some() {
        return Err(binding_error(
            "bundle.workflow_duplicate",
            "stored immutable run bundle contains duplicate canonical workflow records",
        ));
    }
    Ok(workflow)
}

fn compute_binding_hash(binding: &RequiredContextExecutionBinding) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(
        &mut hasher,
        "binding_version",
        binding.binding_version.identifier(),
    );
    hash_field(
        &mut hasher,
        "bundle_id",
        binding.immutable_run_bundle.bundle_id().as_str(),
    );
    hash_field(
        &mut hasher,
        "bundle_version",
        binding.immutable_run_bundle.bundle_version().as_str(),
    );
    hash_field(
        &mut hasher,
        "bundle_root_hash",
        binding.immutable_run_bundle.root_hash().as_str(),
    );
    hash_field(&mut hasher, "workflow_id", binding.workflow_id.as_str());
    hash_field(&mut hasher, "run_id", binding.run_id.as_str());
    hash_field(&mut hasher, "step_id", binding.step_id.as_str());
    hash_field(&mut hasher, "actor", binding.actor.as_str());
    hash_field(
        &mut hasher,
        "harness_contract_id",
        binding.harness_contract_id.as_str(),
    );
    hash_field(
        &mut hasher,
        "harness_contract_version",
        binding.harness_contract_version.as_str(),
    );
    hash_field(
        &mut hasher,
        "contract_content_hash",
        binding.contract_content_hash.as_str(),
    );
    hash_field(
        &mut hasher,
        "maximum_sensitivity",
        sensitivity_label(binding.maximum_sensitivity),
    );
    hash_field(&mut hasher, "bound_at", &binding.bound_at.to_rfc3339());
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update(u64::try_from(label.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(value.as_bytes());
}

const fn sensitivity_label(value: WorkReportSensitivity) -> &'static str {
    match value {
        WorkReportSensitivity::Public => "public",
        WorkReportSensitivity::Internal => "internal",
        WorkReportSensitivity::Confidential => "confidential",
        WorkReportSensitivity::Regulated => "regulated",
        WorkReportSensitivity::Secret => "secret",
        WorkReportSensitivity::Unknown => "unknown",
    }
}

fn binding_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("required_context.execution_binding.{suffix}"),
        message,
    )
}
