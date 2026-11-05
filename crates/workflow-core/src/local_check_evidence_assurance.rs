use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    AcceptedLocalCheckAttestation, ImmutableLocalCheckExecutionBinding, ImmutableRunBundleBinding,
    LocalCheckCommandId, LocalCheckCommandKind, LocalCheckResult, LocalCheckResultId,
    LocalCheckResultStatus, SpecContentHash, StepId, WorkflowId, WorkflowOsError, WorkflowRunId,
};

/// Versioned algorithm for payload-free local-check evidence assurance projections.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckEvidenceAssuranceProjectionAlgorithm {
    /// Initial fixed-width framed SHA-256 projection.
    V1,
}

impl LocalCheckEvidenceAssuranceProjectionAlgorithm {
    #[must_use]
    const fn identifier(self) -> &'static str {
        match self {
            Self::V1 => "workflow-os/local-check-evidence-assurance-projection/v1",
        }
    }
}

impl<'de> Deserialize<'de> for LocalCheckEvidenceAssuranceProjectionAlgorithm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "v1" => Ok(Self::V1),
            _ => Err(serde::de::Error::custom(
                "local check evidence assurance projection algorithm is invalid",
            )),
        }
    }
}

/// Producer provenance established for the selected local-check implementation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckEvidenceProducerPosture {
    /// An operator selected exact executable bytes that the trusted host content-bound.
    OperatorSelectedContentBound,
    /// Core selected a reviewed, registered producer identity and content binding.
    ///
    /// This is requirement vocabulary only until a separately reviewed producer
    /// registry and verifier can establish the posture.
    RegisteredContentBound,
}

impl<'de> Deserialize<'de> for LocalCheckEvidenceProducerPosture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "operator_selected_content_bound" => Ok(Self::OperatorSelectedContentBound),
            "registered_content_bound" => Ok(Self::RegisteredContentBound),
            _ => Err(serde::de::Error::custom(
                "local check evidence producer posture is invalid",
            )),
        }
    }
}

/// Execution observation established for one local-check result.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckEvidenceExecutionPosture {
    /// The trusted host observed one bounded process result.
    TrustedHostObserved,
}

impl<'de> Deserialize<'de> for LocalCheckEvidenceExecutionPosture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "trusted_host_observed" => Ok(Self::TrustedHostObserved),
            _ => Err(serde::de::Error::custom(
                "local check evidence execution posture is invalid",
            )),
        }
    }
}

/// Independent attestation established for one local-check result.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckEvidenceAttestationPosture {
    /// No accepted attestation proof is bound to the result.
    Unattested,
    /// Core accepted an exact local-process attestation proof.
    KernelAcceptedLocalProcess,
}

impl<'de> Deserialize<'de> for LocalCheckEvidenceAttestationPosture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "unattested" => Ok(Self::Unattested),
            "kernel_accepted_local_process" => Ok(Self::KernelAcceptedLocalProcess),
            _ => Err(serde::de::Error::custom(
                "local check evidence attestation posture is invalid",
            )),
        }
    }
}

/// External verifiability established for one local-check result.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckEvidenceExternalVerifiabilityPosture {
    /// No external witness, signature, or independently verifiable receipt exists.
    Unavailable,
}

impl<'de> Deserialize<'de> for LocalCheckEvidenceExternalVerifiabilityPosture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "unavailable" => Ok(Self::Unavailable),
            _ => Err(serde::de::Error::custom(
                "local check evidence external verifiability posture is invalid",
            )),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
struct AcceptedAttestationReference {
    proof_fingerprint: SpecContentHash,
    execution_binding_fingerprint: SpecContentHash,
    immutable_run_bundle: ImmutableRunBundleBinding,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    result_id: LocalCheckResultId,
    result_status: LocalCheckResultStatus,
    handler_selection_fingerprint: SpecContentHash,
}

impl AcceptedAttestationReference {
    fn from_accepted(proof: &AcceptedLocalCheckAttestation) -> Self {
        Self {
            proof_fingerprint: proof.proof_fingerprint().clone(),
            execution_binding_fingerprint: proof.execution_binding_fingerprint().clone(),
            immutable_run_bundle: proof.immutable_run_bundle().clone(),
            workflow_id: proof.workflow_id().clone(),
            run_id: proof.run_id().clone(),
            step_id: proof.step_id().clone(),
            result_id: proof.result_id().clone(),
            result_status: proof.result_status(),
            handler_selection_fingerprint: proof.handler_selection_fingerprint().clone(),
        }
    }
}

/// Payload-free result-level projection of local-check evidence assurance.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct LocalCheckEvidenceAssuranceProjection {
    algorithm: LocalCheckEvidenceAssuranceProjectionAlgorithm,
    immutable_run_bundle: ImmutableRunBundleBinding,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    command_id: LocalCheckCommandId,
    command_kind: LocalCheckCommandKind,
    execution_binding_fingerprint: SpecContentHash,
    handler_selection_fingerprint: SpecContentHash,
    resolved_execution_fingerprint: SpecContentHash,
    result_id: LocalCheckResultId,
    result_status: LocalCheckResultStatus,
    producer_posture: LocalCheckEvidenceProducerPosture,
    execution_posture: LocalCheckEvidenceExecutionPosture,
    attestation_posture: LocalCheckEvidenceAttestationPosture,
    accepted_attestation: Option<AcceptedAttestationReference>,
    external_verifiability: LocalCheckEvidenceExternalVerifiabilityPosture,
    projection_fingerprint: SpecContentHash,
}

/// Existing commitments used to construct an assurance projection.
///
/// This definition is model input, not evidence that Core observed execution.
/// A later runtime consumer must bind the projection to a Core-owned source
/// before using its represented posture as an authorization condition.
pub struct LocalCheckEvidenceAssuranceProjectionDefinition<'a> {
    /// Exact immutable pre-execution binding.
    pub execution_binding: &'a ImmutableLocalCheckExecutionBinding,
    /// Resolved-execution commitment identity supplied by the composing boundary.
    ///
    /// The projection binds this value but does not prove who produced it.
    pub resolved_execution_fingerprint: SpecContentHash,
    /// Stable result identity.
    pub result_id: LocalCheckResultId,
    /// Exact structured result.
    pub result: &'a LocalCheckResult,
    /// Optional proof accepted by the existing crate-private verifier.
    pub accepted_attestation: Option<&'a AcceptedLocalCheckAttestation>,
}

impl LocalCheckEvidenceAssuranceProjection {
    /// Creates a validated assurance projection without copying process payloads.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when execution, result, run, handler,
    /// or accepted-attestation commitments do not match exactly.
    pub fn new(
        definition: LocalCheckEvidenceAssuranceProjectionDefinition<'_>,
    ) -> Result<Self, WorkflowOsError> {
        definition.execution_binding.validate()?;
        definition.result.validate()?;
        if definition.execution_binding.command_id() != definition.result.command_id()
            || definition.execution_binding.command_kind() != definition.result.command_kind()
        {
            return Err(assurance_error(
                "result.command_mismatch",
                "local check evidence assurance result does not match the execution binding",
            ));
        }

        let accepted_attestation = definition
            .accepted_attestation
            .map(AcceptedAttestationReference::from_accepted);
        let attestation_posture = if accepted_attestation.is_some() {
            LocalCheckEvidenceAttestationPosture::KernelAcceptedLocalProcess
        } else {
            LocalCheckEvidenceAttestationPosture::Unattested
        };
        let mut projection = Self {
            algorithm: LocalCheckEvidenceAssuranceProjectionAlgorithm::V1,
            immutable_run_bundle: definition.execution_binding.immutable_run_bundle().clone(),
            workflow_id: definition.execution_binding.workflow_id().clone(),
            run_id: definition.execution_binding.run_id().clone(),
            step_id: definition.execution_binding.step_id().clone(),
            command_id: definition.execution_binding.command_id().clone(),
            command_kind: definition.execution_binding.command_kind(),
            execution_binding_fingerprint: definition
                .execution_binding
                .binding_fingerprint()
                .clone(),
            handler_selection_fingerprint: definition
                .execution_binding
                .handler_selection()
                .selection_fingerprint()
                .clone(),
            resolved_execution_fingerprint: definition.resolved_execution_fingerprint,
            result_id: definition.result_id,
            result_status: definition.result.status(),
            producer_posture: LocalCheckEvidenceProducerPosture::OperatorSelectedContentBound,
            execution_posture: LocalCheckEvidenceExecutionPosture::TrustedHostObserved,
            attestation_posture,
            accepted_attestation,
            external_verifiability: LocalCheckEvidenceExternalVerifiabilityPosture::Unavailable,
            projection_fingerprint: SpecContentHash::from_text("pending"),
        };
        projection.validate_commitments()?;
        projection.projection_fingerprint = compute_projection_fingerprint(&projection);
        projection.validate()?;
        Ok(projection)
    }

    /// Validates internal commitment alignment and the projection fingerprint.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error for contradictory or tampered data.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.validate_commitments()?;
        if self.projection_fingerprint != compute_projection_fingerprint(self) {
            return Err(assurance_error(
                "fingerprint_mismatch",
                "local check evidence assurance projection fingerprint is invalid",
            ));
        }
        Ok(())
    }

    fn validate_commitments(&self) -> Result<(), WorkflowOsError> {
        match (self.attestation_posture, &self.accepted_attestation) {
            (LocalCheckEvidenceAttestationPosture::Unattested, None) => {}
            (LocalCheckEvidenceAttestationPosture::KernelAcceptedLocalProcess, Some(proof)) => {
                if proof.execution_binding_fingerprint != self.execution_binding_fingerprint
                    || proof.immutable_run_bundle != self.immutable_run_bundle
                    || proof.workflow_id != self.workflow_id
                    || proof.run_id != self.run_id
                    || proof.step_id != self.step_id
                    || proof.result_id != self.result_id
                    || proof.result_status != self.result_status
                    || proof.handler_selection_fingerprint != self.handler_selection_fingerprint
                {
                    return Err(assurance_error(
                        "attestation.commitment_mismatch",
                        "local check evidence assurance attestation does not match the result",
                    ));
                }
            }
            _ => {
                return Err(assurance_error(
                    "attestation.posture_mismatch",
                    "local check evidence assurance attestation posture is invalid",
                ));
            }
        }
        Ok(())
    }

    /// Returns the projection algorithm.
    #[must_use]
    pub const fn algorithm(&self) -> LocalCheckEvidenceAssuranceProjectionAlgorithm {
        self.algorithm
    }

    /// Returns the exact immutable run binding.
    #[must_use]
    pub const fn immutable_run_bundle(&self) -> &ImmutableRunBundleBinding {
        &self.immutable_run_bundle
    }

    /// Returns the workflow identity.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the workflow run identity.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the workflow step identity.
    #[must_use]
    pub const fn step_id(&self) -> &StepId {
        &self.step_id
    }

    /// Returns the local-check command identity.
    #[must_use]
    pub const fn command_id(&self) -> &LocalCheckCommandId {
        &self.command_id
    }

    /// Returns the closed local-check command kind.
    #[must_use]
    pub const fn command_kind(&self) -> LocalCheckCommandKind {
        self.command_kind
    }

    /// Returns the exact immutable pre-execution binding commitment.
    #[must_use]
    pub const fn execution_binding_fingerprint(&self) -> &SpecContentHash {
        &self.execution_binding_fingerprint
    }

    /// Returns the exact resolved process-boundary commitment.
    #[must_use]
    pub const fn resolved_execution_fingerprint(&self) -> &SpecContentHash {
        &self.resolved_execution_fingerprint
    }

    /// Returns the exact selected-handler commitment.
    #[must_use]
    pub const fn handler_selection_fingerprint(&self) -> &SpecContentHash {
        &self.handler_selection_fingerprint
    }

    /// Returns the exact local-check result identity.
    #[must_use]
    pub const fn result_id(&self) -> &LocalCheckResultId {
        &self.result_id
    }

    /// Returns the structured local-check result status.
    #[must_use]
    pub const fn result_status(&self) -> LocalCheckResultStatus {
        self.result_status
    }

    /// Returns the independently modeled producer posture.
    #[must_use]
    pub const fn producer_posture(&self) -> LocalCheckEvidenceProducerPosture {
        self.producer_posture
    }

    /// Returns the independently modeled execution-observation posture.
    #[must_use]
    pub const fn execution_posture(&self) -> LocalCheckEvidenceExecutionPosture {
        self.execution_posture
    }

    /// Returns the independently modeled attestation posture.
    #[must_use]
    pub const fn attestation_posture(&self) -> LocalCheckEvidenceAttestationPosture {
        self.attestation_posture
    }

    /// Returns the explicit external-verifiability posture.
    #[must_use]
    pub const fn external_verifiability(&self) -> LocalCheckEvidenceExternalVerifiabilityPosture {
        self.external_verifiability
    }

    /// Returns the accepted proof reference when verifier-owned proof was supplied in memory.
    #[must_use]
    pub fn accepted_attestation_proof_fingerprint(&self) -> Option<&SpecContentHash> {
        self.accepted_attestation
            .as_ref()
            .map(|proof| &proof.proof_fingerprint)
    }

    /// Returns the deterministic projection fingerprint.
    #[must_use]
    pub const fn projection_fingerprint(&self) -> &SpecContentHash {
        &self.projection_fingerprint
    }

    /// Returns whether the represented posture satisfies a declared minimum.
    ///
    /// This is a dimension-specific model comparison, not execution authority or
    /// proof that Core produced the projection. A runtime gate must separately
    /// establish a Core-owned source and immutable context before relying on it.
    #[must_use]
    pub fn satisfies_minimum(&self, minimum: LocalCheckEvidenceAssuranceMinimum) -> bool {
        producer_satisfies(self.producer_posture, minimum.producer)
            && self.execution_posture == minimum.execution
            && attestation_satisfies(self.attestation_posture, minimum.attestation)
    }

    /// Returns whether two projections describe the same durable evidence subject
    /// and assurance posture.
    ///
    /// Observation-specific execution fingerprints are intentionally excluded so
    /// a later reassessment cannot replace the first persisted projection merely
    /// because it observed the same subject at a different time.
    #[must_use]
    pub(crate) fn same_durable_subject_and_posture(&self, other: &Self) -> bool {
        self.immutable_run_bundle == other.immutable_run_bundle
            && self.workflow_id == other.workflow_id
            && self.run_id == other.run_id
            && self.step_id == other.step_id
            && self.command_id == other.command_id
            && self.command_kind == other.command_kind
            && self.handler_selection_fingerprint == other.handler_selection_fingerprint
            && self.result_id == other.result_id
            && self.result_status == other.result_status
            && self.producer_posture == other.producer_posture
            && self.execution_posture == other.execution_posture
            && self.attestation_posture == other.attestation_posture
            && self.accepted_attestation == other.accepted_attestation
            && self.external_verifiability == other.external_verifiability
    }
}

impl fmt::Debug for LocalCheckEvidenceAssuranceProjection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCheckEvidenceAssuranceProjection")
            .field("algorithm", &self.algorithm)
            .field("identities", &"[REDACTED]")
            .field("commitments", &"[REDACTED]")
            .field("result_status", &self.result_status)
            .field("producer_posture", &self.producer_posture)
            .field("execution_posture", &self.execution_posture)
            .field("attestation_posture", &self.attestation_posture)
            .field("external_verifiability", &self.external_verifiability)
            .finish_non_exhaustive()
    }
}

#[derive(Deserialize)]
struct LocalCheckEvidenceAssuranceProjectionWire {
    algorithm: LocalCheckEvidenceAssuranceProjectionAlgorithm,
    immutable_run_bundle: ImmutableRunBundleBinding,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    command_id: LocalCheckCommandId,
    command_kind: LocalCheckCommandKind,
    execution_binding_fingerprint: SpecContentHash,
    handler_selection_fingerprint: SpecContentHash,
    resolved_execution_fingerprint: SpecContentHash,
    result_id: LocalCheckResultId,
    result_status: LocalCheckResultStatus,
    producer_posture: LocalCheckEvidenceProducerPosture,
    execution_posture: LocalCheckEvidenceExecutionPosture,
    attestation_posture: LocalCheckEvidenceAttestationPosture,
    accepted_attestation: Option<AcceptedAttestationReference>,
    external_verifiability: LocalCheckEvidenceExternalVerifiabilityPosture,
    projection_fingerprint: SpecContentHash,
}

impl<'de> Deserialize<'de> for LocalCheckEvidenceAssuranceProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = LocalCheckEvidenceAssuranceProjectionWire::deserialize(deserializer)?;
        if wire.attestation_posture
            == LocalCheckEvidenceAttestationPosture::KernelAcceptedLocalProcess
            || wire.accepted_attestation.is_some()
        {
            return Err(serde::de::Error::custom(
                "accepted local check evidence assurance requires verifier-owned proof",
            ));
        }
        let projection = Self {
            algorithm: wire.algorithm,
            immutable_run_bundle: wire.immutable_run_bundle,
            workflow_id: wire.workflow_id,
            run_id: wire.run_id,
            step_id: wire.step_id,
            command_id: wire.command_id,
            command_kind: wire.command_kind,
            execution_binding_fingerprint: wire.execution_binding_fingerprint,
            handler_selection_fingerprint: wire.handler_selection_fingerprint,
            resolved_execution_fingerprint: wire.resolved_execution_fingerprint,
            result_id: wire.result_id,
            result_status: wire.result_status,
            producer_posture: wire.producer_posture,
            execution_posture: wire.execution_posture,
            attestation_posture: wire.attestation_posture,
            accepted_attestation: wire.accepted_attestation,
            external_verifiability: wire.external_verifiability,
            projection_fingerprint: wire.projection_fingerprint,
        };
        projection.validate().map_err(|_| {
            serde::de::Error::custom("invalid local check evidence assurance projection")
        })?;
        Ok(projection)
    }
}

/// Dimension-specific minimum assurance required by a later narrow gate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalCheckEvidenceAssuranceMinimum {
    producer: LocalCheckEvidenceProducerPosture,
    execution: LocalCheckEvidenceExecutionPosture,
    attestation: LocalCheckEvidenceAttestationPosture,
}

impl LocalCheckEvidenceAssuranceMinimum {
    /// Creates a dimension-specific minimum without collapsing dimensions into one level.
    #[must_use]
    pub const fn new(
        producer: LocalCheckEvidenceProducerPosture,
        execution: LocalCheckEvidenceExecutionPosture,
        attestation: LocalCheckEvidenceAttestationPosture,
    ) -> Self {
        Self {
            producer,
            execution,
            attestation,
        }
    }

    /// Returns the required producer posture.
    #[must_use]
    pub const fn producer(&self) -> LocalCheckEvidenceProducerPosture {
        self.producer
    }

    /// Returns the required execution-observation posture.
    #[must_use]
    pub const fn execution(&self) -> LocalCheckEvidenceExecutionPosture {
        self.execution
    }

    /// Returns the required attestation posture.
    #[must_use]
    pub const fn attestation(&self) -> LocalCheckEvidenceAttestationPosture {
        self.attestation
    }
}

fn compute_projection_fingerprint(
    projection: &LocalCheckEvidenceAssuranceProjection,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "algorithm", projection.algorithm.identifier());
    hash_field(
        &mut hasher,
        "bundle_id",
        projection.immutable_run_bundle.bundle_id().as_str(),
    );
    hash_field(
        &mut hasher,
        "bundle_version",
        projection.immutable_run_bundle.bundle_version().as_str(),
    );
    hash_field(
        &mut hasher,
        "bundle_root",
        projection.immutable_run_bundle.root_hash().as_str(),
    );
    hash_field(&mut hasher, "workflow_id", projection.workflow_id.as_str());
    hash_field(&mut hasher, "run_id", projection.run_id.as_str());
    hash_field(&mut hasher, "step_id", projection.step_id.as_str());
    hash_field(&mut hasher, "command_id", projection.command_id.as_str());
    hash_field(
        &mut hasher,
        "command_kind",
        command_kind_label(projection.command_kind),
    );
    hash_field(
        &mut hasher,
        "execution_binding_fingerprint",
        projection.execution_binding_fingerprint.as_str(),
    );
    hash_field(
        &mut hasher,
        "handler_selection_fingerprint",
        projection.handler_selection_fingerprint.as_str(),
    );
    hash_field(
        &mut hasher,
        "resolved_execution_fingerprint",
        projection.resolved_execution_fingerprint.as_str(),
    );
    hash_field(&mut hasher, "result_id", projection.result_id.as_str());
    hash_field(
        &mut hasher,
        "result_status",
        &projection.result_status.to_string(),
    );
    hash_field(
        &mut hasher,
        "producer_posture",
        producer_label(projection.producer_posture),
    );
    hash_field(
        &mut hasher,
        "execution_posture",
        execution_label(projection.execution_posture),
    );
    hash_field(
        &mut hasher,
        "attestation_posture",
        attestation_label(projection.attestation_posture),
    );
    if let Some(proof) = &projection.accepted_attestation {
        hash_field(
            &mut hasher,
            "accepted_attestation_proof_fingerprint",
            proof.proof_fingerprint.as_str(),
        );
    } else {
        hash_field(
            &mut hasher,
            "accepted_attestation_proof_fingerprint",
            "none",
        );
    }
    hash_field(
        &mut hasher,
        "external_verifiability",
        external_verifiability_label(projection.external_verifiability),
    );
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update(u64::try_from(label.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(value.as_bytes());
}

const fn producer_label(value: LocalCheckEvidenceProducerPosture) -> &'static str {
    match value {
        LocalCheckEvidenceProducerPosture::OperatorSelectedContentBound => {
            "operator_selected_content_bound"
        }
        LocalCheckEvidenceProducerPosture::RegisteredContentBound => "registered_content_bound",
    }
}

const fn execution_label(value: LocalCheckEvidenceExecutionPosture) -> &'static str {
    match value {
        LocalCheckEvidenceExecutionPosture::TrustedHostObserved => "trusted_host_observed",
    }
}

const fn producer_satisfies(
    actual: LocalCheckEvidenceProducerPosture,
    minimum: LocalCheckEvidenceProducerPosture,
) -> bool {
    matches!(
        (actual, minimum),
        (
            LocalCheckEvidenceProducerPosture::OperatorSelectedContentBound,
            LocalCheckEvidenceProducerPosture::OperatorSelectedContentBound
        ) | (
            LocalCheckEvidenceProducerPosture::RegisteredContentBound,
            LocalCheckEvidenceProducerPosture::OperatorSelectedContentBound
                | LocalCheckEvidenceProducerPosture::RegisteredContentBound
        )
    )
}

const fn attestation_label(value: LocalCheckEvidenceAttestationPosture) -> &'static str {
    match value {
        LocalCheckEvidenceAttestationPosture::Unattested => "unattested",
        LocalCheckEvidenceAttestationPosture::KernelAcceptedLocalProcess => {
            "kernel_accepted_local_process"
        }
    }
}

const fn attestation_satisfies(
    actual: LocalCheckEvidenceAttestationPosture,
    minimum: LocalCheckEvidenceAttestationPosture,
) -> bool {
    matches!(
        (actual, minimum),
        (
            LocalCheckEvidenceAttestationPosture::Unattested
                | LocalCheckEvidenceAttestationPosture::KernelAcceptedLocalProcess,
            LocalCheckEvidenceAttestationPosture::Unattested
        ) | (
            LocalCheckEvidenceAttestationPosture::KernelAcceptedLocalProcess,
            LocalCheckEvidenceAttestationPosture::KernelAcceptedLocalProcess
        )
    )
}

const fn external_verifiability_label(
    value: LocalCheckEvidenceExternalVerifiabilityPosture,
) -> &'static str {
    match value {
        LocalCheckEvidenceExternalVerifiabilityPosture::Unavailable => "unavailable",
    }
}

const fn command_kind_label(value: LocalCheckCommandKind) -> &'static str {
    match value {
        LocalCheckCommandKind::WorkflowOsValidateDogfood => "workflow_os_validate_dogfood",
        LocalCheckCommandKind::WorkflowOsProjectValidation => "workflow_os_project_validation",
        LocalCheckCommandKind::DocsCheck => "docs_check",
        LocalCheckCommandKind::CargoFmtCheck => "cargo_fmt_check",
        LocalCheckCommandKind::CargoClippyWorkspace => "cargo_clippy_workspace",
        LocalCheckCommandKind::CargoTestWorkspace => "cargo_test_workspace",
        LocalCheckCommandKind::TypeScriptCheck => "typescript_check",
        LocalCheckCommandKind::ContractCheck => "contract_check",
        LocalCheckCommandKind::IntegrationCheck => "integration_check",
    }
}

fn assurance_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("local_check_evidence_assurance.{suffix}"), message)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use serde_json::json;

    use super::*;
    use crate::{
        ImmutableLocalCheckExecutionBindingDefinition, ImmutableLocalCheckHandlerSelection,
        LocalCheckCommandContract, LocalCheckResultDefinition, SkillId, SkillVersion, Timestamp,
    };

    fn immutable_run_bundle() -> ImmutableRunBundleBinding {
        serde_json::from_value(json!({
            "bundle_id": "bundle/test",
            "bundle_version": "v1",
            "root_hash": SpecContentHash::from_text("bundle root"),
        }))
        .expect("bundle binding")
    }

    fn execution_binding(
        command: &LocalCheckCommandContract,
        skill_id: &str,
    ) -> ImmutableLocalCheckExecutionBinding {
        execution_binding_with_bundle(command, skill_id, immutable_run_bundle())
    }

    fn execution_binding_with_bundle(
        command: &LocalCheckCommandContract,
        skill_id: &str,
        immutable_run_bundle: ImmutableRunBundleBinding,
    ) -> ImmutableLocalCheckExecutionBinding {
        let skill_id = SkillId::new(skill_id).expect("skill id");
        let skill_version = SkillVersion::new("v0").expect("skill version");
        ImmutableLocalCheckExecutionBinding::new(ImmutableLocalCheckExecutionBindingDefinition {
            immutable_run_bundle,
            workflow_id: WorkflowId::new("workflow/test").expect("workflow id"),
            run_id: WorkflowRunId::new("run-test").expect("run id"),
            step_id: StepId::new("check").expect("step id"),
            skill_id: skill_id.clone(),
            skill_version: skill_version.clone(),
            command_contract: command,
            handler_selection: ImmutableLocalCheckHandlerSelection::registered_unattested(
                command.command_kind(),
                skill_id,
                skill_version,
            ),
            created_at: Timestamp::parse_rfc3339("2026-11-04T20:00:00Z").expect("timestamp"),
        })
        .expect("execution binding")
    }

    fn result(
        command: &LocalCheckCommandContract,
        status: LocalCheckResultStatus,
        exit_code: Option<i32>,
    ) -> LocalCheckResult {
        LocalCheckResult::new(LocalCheckResultDefinition {
            command_id: command.command_id().clone(),
            command_kind: command.command_kind(),
            status,
            exit_code,
            duration_ms: 12,
            stdout_summary: "check complete".to_owned(),
            stderr_summary: String::new(),
            stdout_truncated: false,
            stderr_truncated: false,
            error_code: None,
        })
        .expect("result")
    }

    fn weak_projection(
        resolved_execution: &str,
        status: LocalCheckResultStatus,
        exit_code: Option<i32>,
    ) -> LocalCheckEvidenceAssuranceProjection {
        let command = LocalCheckCommandContract::docs_check_model_only().expect("command");
        let binding = execution_binding(&command, "local/check-docs");
        let result = result(&command, status, exit_code);
        LocalCheckEvidenceAssuranceProjection::new(
            LocalCheckEvidenceAssuranceProjectionDefinition {
                execution_binding: &binding,
                resolved_execution_fingerprint: SpecContentHash::from_text(resolved_execution),
                result_id: LocalCheckResultId::new("result/check").expect("result id"),
                result: &result,
                accepted_attestation: None,
            },
        )
        .expect("projection")
    }

    #[test]
    fn current_validator_posture_round_trips_without_assurance_upgrade() {
        let projection = weak_projection(
            "resolved execution",
            LocalCheckResultStatus::Passed,
            Some(0),
        );
        assert_eq!(
            projection.projection_fingerprint().as_str(),
            "55e3803c1e78ff5642ec8a54ec1cd7d1cc239ce4d7ba9b993a1f50024a491923"
        );
        assert_eq!(
            projection.producer_posture(),
            LocalCheckEvidenceProducerPosture::OperatorSelectedContentBound
        );
        assert_eq!(
            projection.execution_posture(),
            LocalCheckEvidenceExecutionPosture::TrustedHostObserved
        );
        assert_eq!(
            projection.attestation_posture(),
            LocalCheckEvidenceAttestationPosture::Unattested
        );
        assert_eq!(
            projection.external_verifiability(),
            LocalCheckEvidenceExternalVerifiabilityPosture::Unavailable
        );
        assert!(projection
            .accepted_attestation_proof_fingerprint()
            .is_none());

        let encoded = serde_json::to_string(&projection).expect("serialized");
        assert!(!encoded.contains("check complete"));
        assert!(!encoded.contains("stdout"));
        assert!(!encoded.contains("stderr"));
        let decoded: LocalCheckEvidenceAssuranceProjection =
            serde_json::from_str(&encoded).expect("deserialized");
        assert_eq!(decoded, projection);
    }

    #[test]
    fn zero_exit_changes_result_commitment_but_not_producer_or_attestation_posture() {
        let passed = weak_projection(
            "resolved execution",
            LocalCheckResultStatus::Passed,
            Some(0),
        );
        let failed = weak_projection(
            "resolved execution",
            LocalCheckResultStatus::Failed,
            Some(1),
        );
        assert_eq!(passed.producer_posture(), failed.producer_posture());
        assert_eq!(passed.attestation_posture(), failed.attestation_posture());
        assert_ne!(
            passed.projection_fingerprint(),
            failed.projection_fingerprint()
        );
    }

    #[test]
    fn resolved_execution_and_handler_selection_change_projection_commitment() {
        let original = weak_projection(
            "resolved execution",
            LocalCheckResultStatus::Passed,
            Some(0),
        );
        let changed_execution =
            weak_projection("changed execution", LocalCheckResultStatus::Passed, Some(0));
        assert_ne!(
            original.projection_fingerprint(),
            changed_execution.projection_fingerprint()
        );

        let command = LocalCheckCommandContract::docs_check_model_only().expect("command");
        let changed_binding = execution_binding(&command, "local/alternate-docs-check");
        let result = result(&command, LocalCheckResultStatus::Passed, Some(0));
        let changed_handler = LocalCheckEvidenceAssuranceProjection::new(
            LocalCheckEvidenceAssuranceProjectionDefinition {
                execution_binding: &changed_binding,
                resolved_execution_fingerprint: SpecContentHash::from_text("resolved execution"),
                result_id: LocalCheckResultId::new("result/check").expect("result id"),
                result: &result,
                accepted_attestation: None,
            },
        )
        .expect("projection");
        assert_ne!(
            original.projection_fingerprint(),
            changed_handler.projection_fingerprint()
        );
    }

    #[test]
    fn command_run_and_result_identity_change_projection_commitment() {
        let original = weak_projection(
            "resolved execution",
            LocalCheckResultStatus::Passed,
            Some(0),
        );

        let cargo_command =
            LocalCheckCommandContract::cargo_fmt_check_model_only().expect("command");
        let cargo_binding = execution_binding(&cargo_command, "local/cargo-fmt-check");
        let cargo_result = result(&cargo_command, LocalCheckResultStatus::Passed, Some(0));
        let changed_command = LocalCheckEvidenceAssuranceProjection::new(
            LocalCheckEvidenceAssuranceProjectionDefinition {
                execution_binding: &cargo_binding,
                resolved_execution_fingerprint: SpecContentHash::from_text("resolved execution"),
                result_id: LocalCheckResultId::new("result/check").expect("result id"),
                result: &cargo_result,
                accepted_attestation: None,
            },
        )
        .expect("projection");
        assert_ne!(
            original.projection_fingerprint(),
            changed_command.projection_fingerprint()
        );

        let command = LocalCheckCommandContract::docs_check_model_only().expect("command");
        let changed_bundle = immutable_run_bundle()
            .with_test_root_hash(SpecContentHash::from_text("changed bundle root"));
        let changed_bundle_binding =
            execution_binding_with_bundle(&command, "local/check-docs", changed_bundle);
        let docs_result = result(&command, LocalCheckResultStatus::Passed, Some(0));
        let changed_run = LocalCheckEvidenceAssuranceProjection::new(
            LocalCheckEvidenceAssuranceProjectionDefinition {
                execution_binding: &changed_bundle_binding,
                resolved_execution_fingerprint: SpecContentHash::from_text("resolved execution"),
                result_id: LocalCheckResultId::new("result/check").expect("result id"),
                result: &docs_result,
                accepted_attestation: None,
            },
        )
        .expect("projection");
        assert_ne!(
            original.projection_fingerprint(),
            changed_run.projection_fingerprint()
        );

        let original_binding = execution_binding(&command, "local/check-docs");
        let changed_result_id = LocalCheckEvidenceAssuranceProjection::new(
            LocalCheckEvidenceAssuranceProjectionDefinition {
                execution_binding: &original_binding,
                resolved_execution_fingerprint: SpecContentHash::from_text("resolved execution"),
                result_id: LocalCheckResultId::new("result/changed").expect("result id"),
                result: &docs_result,
                accepted_attestation: None,
            },
        )
        .expect("projection");
        assert_ne!(
            original.projection_fingerprint(),
            changed_result_id.projection_fingerprint()
        );
    }

    #[test]
    fn minimum_assurance_is_dimension_specific_and_monotonic() {
        let projection = weak_projection(
            "resolved execution",
            LocalCheckResultStatus::Passed,
            Some(0),
        );
        let weak = LocalCheckEvidenceAssuranceMinimum::new(
            LocalCheckEvidenceProducerPosture::OperatorSelectedContentBound,
            LocalCheckEvidenceExecutionPosture::TrustedHostObserved,
            LocalCheckEvidenceAttestationPosture::Unattested,
        );
        let attested = LocalCheckEvidenceAssuranceMinimum::new(
            LocalCheckEvidenceProducerPosture::OperatorSelectedContentBound,
            LocalCheckEvidenceExecutionPosture::TrustedHostObserved,
            LocalCheckEvidenceAttestationPosture::KernelAcceptedLocalProcess,
        );
        let registered = LocalCheckEvidenceAssuranceMinimum::new(
            LocalCheckEvidenceProducerPosture::RegisteredContentBound,
            LocalCheckEvidenceExecutionPosture::TrustedHostObserved,
            LocalCheckEvidenceAttestationPosture::Unattested,
        );
        assert!(projection.satisfies_minimum(weak));
        assert!(!projection.satisfies_minimum(attested));
        assert!(!projection.satisfies_minimum(registered));
        assert!(producer_satisfies(
            LocalCheckEvidenceProducerPosture::RegisteredContentBound,
            LocalCheckEvidenceProducerPosture::OperatorSelectedContentBound,
        ));
        assert!(producer_satisfies(
            LocalCheckEvidenceProducerPosture::RegisteredContentBound,
            LocalCheckEvidenceProducerPosture::RegisteredContentBound,
        ));
        assert!(!producer_satisfies(
            LocalCheckEvidenceProducerPosture::OperatorSelectedContentBound,
            LocalCheckEvidenceProducerPosture::RegisteredContentBound,
        ));
    }

    #[test]
    fn serde_tampering_and_unknown_vocabulary_fail_closed() {
        let projection = weak_projection(
            "resolved execution",
            LocalCheckResultStatus::Passed,
            Some(0),
        );
        let mut value = serde_json::to_value(&projection).expect("value");
        value["resolved_execution_fingerprint"] =
            serde_json::to_value(SpecContentHash::from_text("tampered")).expect("hash");
        assert!(serde_json::from_value::<LocalCheckEvidenceAssuranceProjection>(value).is_err());

        let mut value = serde_json::to_value(&projection).expect("value");
        value["producer_posture"] = json!("reviewed_because_path_looked_official");
        assert!(serde_json::from_value::<LocalCheckEvidenceAssuranceProjection>(value).is_err());
    }

    #[test]
    fn debug_and_errors_do_not_disclose_commitments_or_payloads() {
        let projection = weak_projection(
            "sensitive resolved execution",
            LocalCheckResultStatus::Passed,
            Some(0),
        );
        let debug = format!("{projection:?}");
        assert!(!debug.contains(projection.projection_fingerprint().as_str()));
        assert!(!debug.contains("sensitive"));
        assert!(!debug.contains("check complete"));

        let mut value = serde_json::to_value(&projection).expect("value");
        value["result_id"] = json!("result/changed");
        let error = serde_json::from_value::<LocalCheckEvidenceAssuranceProjection>(value)
            .expect_err("tampering must fail");
        assert!(!error
            .to_string()
            .contains(projection.projection_fingerprint().as_str()));
        assert!(!error.to_string().contains("result/changed"));
    }
}
