use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    IdempotencyKey, ImmutableRunBundleBinding, LocalCheckCommandId, LocalCheckResultId,
    LocalCheckResultStatus, SkillInvocationId, SpecContentHash, StepId, Timestamp, WorkflowId,
    WorkflowOsError, WorkflowRunId,
};

const ATTESTATION_ID_MAX_BYTES: usize = 128;
const MAX_FRESHNESS_SECONDS: u32 = 30 * 24 * 60 * 60;

/// Stable identifier for a local check attestation candidate.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct LocalCheckAttestationId(String);

impl LocalCheckAttestationId {
    /// Creates a validated attestation candidate identifier.
    ///
    /// # Errors
    ///
    /// Returns a stable validation error when the identifier is empty, too
    /// long, or outside the bounded identifier character set.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        if value.is_empty() {
            return Err(attestation_error(
                "id.empty",
                "local check attestation id cannot be empty",
            ));
        }
        if value.len() > ATTESTATION_ID_MAX_BYTES {
            return Err(attestation_error(
                "id.too_long",
                "local check attestation id exceeds the supported bound",
            ));
        }
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
        {
            return Err(attestation_error(
                "id.invalid_character",
                "local check attestation id contains an invalid character",
            ));
        }
        Ok(Self(value))
    }

    /// Returns the identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for LocalCheckAttestationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("LocalCheckAttestationId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl fmt::Display for LocalCheckAttestationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl From<LocalCheckAttestationId> for String {
    fn from(value: LocalCheckAttestationId) -> Self {
        value.0
    }
}

impl TryFrom<String> for LocalCheckAttestationId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for LocalCheckAttestationId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Versioned algorithm for payload-free attestation candidate bindings.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckAttestationAlgorithm {
    /// Initial fixed-width framed SHA-256 binding.
    V1,
}

impl LocalCheckAttestationAlgorithm {
    /// Returns the stable domain-separated algorithm identifier.
    #[must_use]
    pub const fn identifier(self) -> &'static str {
        match self {
            Self::V1 => "workflow-os/local-check-attestation-candidate/v1",
        }
    }
}

impl<'de> Deserialize<'de> for LocalCheckAttestationAlgorithm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "v1" => Ok(Self::V1),
            _ => Err(serde::de::Error::custom(
                "local check attestation algorithm is invalid",
            )),
        }
    }
}

/// Claimed assurance vocabulary. This model does not verify the claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckAttestationAssurance {
    /// A caller supplied the outcome posture.
    CallerAsserted,
    /// A preview or test mock produced the outcome posture.
    MockObserved,
    /// A future verifier may prove a kernel-owned local process observation.
    KernelObservedLocalProcess,
    /// Reserved future vocabulary for a separately governed verifier.
    ExternalVerifier,
}

impl LocalCheckAttestationAssurance {
    /// Returns whether this assurance is eligible for the future v0 verifier.
    ///
    /// Eligibility is not verification and does not satisfy a requirement.
    #[must_use]
    pub const fn eligible_for_v0_verification(self) -> bool {
        matches!(self, Self::KernelObservedLocalProcess)
    }
}

/// Source vocabulary for an attestation candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckAttestationSource {
    /// Caller-created model data.
    Caller,
    /// Preview or test mock handler.
    MockHandler,
    /// Future kernel-owned local process observation.
    KernelLocalProcessRunner,
    /// Future separately governed verifier.
    ExternalVerifier,
}

/// Freshness requirement for an independently verified local check.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum LocalCheckAttestationFreshnessPolicy {
    /// The future verifier must not reuse an earlier observation.
    NoReuse,
    /// A future verifier may accept an observation within this age.
    MaxAgeSeconds {
        /// Maximum observation age in seconds.
        seconds: u32,
    },
}

impl LocalCheckAttestationFreshnessPolicy {
    /// Creates a bounded maximum-age policy.
    ///
    /// # Errors
    ///
    /// Returns a stable validation error when the age is zero or above the
    /// supported model bound.
    pub fn max_age_seconds(seconds: u32) -> Result<Self, WorkflowOsError> {
        if seconds == 0 || seconds > MAX_FRESHNESS_SECONDS {
            return Err(attestation_error(
                "freshness.invalid_max_age",
                "local check attestation freshness age is invalid",
            ));
        }
        Ok(Self::MaxAgeSeconds { seconds })
    }

    /// Returns the configured maximum age, when reuse is modeled.
    #[must_use]
    pub const fn maximum_age_seconds(self) -> Option<u32> {
        match self {
            Self::NoReuse => None,
            Self::MaxAgeSeconds { seconds } => Some(seconds),
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
enum LocalCheckAttestationFreshnessPolicyWire {
    NoReuse,
    MaxAgeSeconds { seconds: u32 },
}

impl<'de> Deserialize<'de> for LocalCheckAttestationFreshnessPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match LocalCheckAttestationFreshnessPolicyWire::deserialize(deserializer)? {
            LocalCheckAttestationFreshnessPolicyWire::NoReuse => Ok(Self::NoReuse),
            LocalCheckAttestationFreshnessPolicyWire::MaxAgeSeconds { seconds } => {
                Self::max_age_seconds(seconds).map_err(|_| {
                    serde::de::Error::custom("local check attestation freshness policy is invalid")
                })
            }
        }
    }
}

/// Minimum independently verified local check posture required by a gate.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct LocalCheckAttestationRequirement {
    command_id: LocalCheckCommandId,
    minimum_assurance: LocalCheckAttestationAssurance,
    accepted_statuses: Vec<LocalCheckResultStatus>,
    freshness: LocalCheckAttestationFreshnessPolicy,
    exact_immutable_run_binding_required: bool,
    truncation_allowed: bool,
    requirement_fingerprint: SpecContentHash,
}

/// Input fields for a validated independent-check requirement.
pub struct LocalCheckAttestationRequirementDefinition {
    /// Required command contract identity.
    pub command_id: LocalCheckCommandId,
    /// Minimum independently verifiable assurance.
    pub minimum_assurance: LocalCheckAttestationAssurance,
    /// Result statuses the future gate may accept.
    pub accepted_statuses: Vec<LocalCheckResultStatus>,
    /// Time-of-use freshness requirement.
    pub freshness: LocalCheckAttestationFreshnessPolicy,
    /// Whether exact immutable-run binding is mandatory.
    pub exact_immutable_run_binding_required: bool,
    /// Whether a truncated bounded result may be considered.
    pub truncation_allowed: bool,
}

impl LocalCheckAttestationRequirement {
    /// Creates a validated independent-check requirement.
    ///
    /// # Errors
    ///
    /// Returns a stable validation error when the requirement could be
    /// satisfied by caller/mock posture, accepts no statuses, contains
    /// duplicates, or relaxes immutable-run binding.
    pub fn new(
        definition: LocalCheckAttestationRequirementDefinition,
    ) -> Result<Self, WorkflowOsError> {
        if definition.minimum_assurance
            != LocalCheckAttestationAssurance::KernelObservedLocalProcess
        {
            return Err(attestation_error(
                "requirement.assurance_unsupported",
                "independent local check requirements need kernel-observed assurance",
            ));
        }
        if definition.accepted_statuses.is_empty() {
            return Err(attestation_error(
                "requirement.statuses_empty",
                "independent local check requirement needs an accepted status",
            ));
        }
        let unique = definition.accepted_statuses.iter().collect::<BTreeSet<_>>();
        if unique.len() != definition.accepted_statuses.len() {
            return Err(attestation_error(
                "requirement.statuses_duplicate",
                "independent local check accepted statuses must be unique",
            ));
        }
        if !definition.exact_immutable_run_binding_required {
            return Err(attestation_error(
                "requirement.bundle_binding_required",
                "independent local check requires exact immutable run binding",
            ));
        }
        let mut requirement = Self {
            command_id: definition.command_id,
            minimum_assurance: definition.minimum_assurance,
            accepted_statuses: definition.accepted_statuses,
            freshness: definition.freshness,
            exact_immutable_run_binding_required: true,
            truncation_allowed: definition.truncation_allowed,
            requirement_fingerprint: SpecContentHash::from_bytes([]),
        };
        requirement.accepted_statuses.sort_unstable();
        requirement.requirement_fingerprint = compute_requirement_fingerprint(&requirement);
        Ok(requirement)
    }

    #[must_use]
    /// Returns the required command identity.
    pub const fn command_id(&self) -> &LocalCheckCommandId {
        &self.command_id
    }

    #[must_use]
    /// Returns the minimum required assurance.
    pub const fn minimum_assurance(&self) -> LocalCheckAttestationAssurance {
        self.minimum_assurance
    }

    #[must_use]
    /// Returns accepted result statuses.
    pub fn accepted_statuses(&self) -> &[LocalCheckResultStatus] {
        &self.accepted_statuses
    }

    #[must_use]
    /// Returns the freshness policy.
    pub const fn freshness(&self) -> LocalCheckAttestationFreshnessPolicy {
        self.freshness
    }

    #[must_use]
    /// Returns whether exact immutable-run binding is required.
    pub const fn exact_immutable_run_binding_required(&self) -> bool {
        self.exact_immutable_run_binding_required
    }

    #[must_use]
    /// Returns whether bounded truncation is allowed.
    pub const fn truncation_allowed(&self) -> bool {
        self.truncation_allowed
    }

    /// Returns the deterministic complete requirement fingerprint.
    #[must_use]
    pub const fn requirement_fingerprint(&self) -> &SpecContentHash {
        &self.requirement_fingerprint
    }
}

impl fmt::Debug for LocalCheckAttestationRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCheckAttestationRequirement")
            .field("command_id", &"[REDACTED]")
            .field("minimum_assurance", &self.minimum_assurance)
            .field("accepted_statuses", &self.accepted_statuses)
            .field("freshness", &self.freshness)
            .field(
                "exact_immutable_run_binding_required",
                &self.exact_immutable_run_binding_required,
            )
            .field("truncation_allowed", &self.truncation_allowed)
            .field("requirement_fingerprint", &"[REDACTED]")
            .finish()
    }
}

#[derive(Deserialize)]
struct LocalCheckAttestationRequirementWire {
    command_id: LocalCheckCommandId,
    minimum_assurance: LocalCheckAttestationAssurance,
    accepted_statuses: Vec<LocalCheckResultStatus>,
    freshness: LocalCheckAttestationFreshnessPolicy,
    exact_immutable_run_binding_required: bool,
    truncation_allowed: bool,
    requirement_fingerprint: SpecContentHash,
}

impl<'de> Deserialize<'de> for LocalCheckAttestationRequirement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = LocalCheckAttestationRequirementWire::deserialize(deserializer)?;
        let expected_fingerprint = wire.requirement_fingerprint;
        let requirement = Self::new(LocalCheckAttestationRequirementDefinition {
            command_id: wire.command_id,
            minimum_assurance: wire.minimum_assurance,
            accepted_statuses: wire.accepted_statuses,
            freshness: wire.freshness,
            exact_immutable_run_binding_required: wire.exact_immutable_run_binding_required,
            truncation_allowed: wire.truncation_allowed,
        })
        .map_err(|_| serde::de::Error::custom("invalid local check attestation requirement"))?;
        if requirement.requirement_fingerprint != expected_fingerprint {
            return Err(serde::de::Error::custom(
                "invalid local check attestation requirement",
            ));
        }
        Ok(requirement)
    }
}

/// Structured exit-code posture without raw process output.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckAttestationExitCodePosture {
    /// Process reported successful zero exit.
    Zero,
    /// Process reported a non-zero exit.
    NonZero,
    /// No meaningful process exit code was available.
    Unavailable,
}

/// Verification posture for a model-only attestation binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckAttestationVerificationPosture {
    /// The binding is a validated candidate and has not been independently verified.
    Unverified,
}

impl<'de> Deserialize<'de> for LocalCheckAttestationVerificationPosture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "unverified" => Ok(Self::Unverified),
            _ => Err(serde::de::Error::custom(
                "local check attestation verification posture is invalid",
            )),
        }
    }
}

/// Payload-free, explicitly unverified local check attestation candidate.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct LocalCheckAttestationBinding {
    attestation_id: LocalCheckAttestationId,
    algorithm: LocalCheckAttestationAlgorithm,
    assurance: LocalCheckAttestationAssurance,
    source: LocalCheckAttestationSource,
    verification_posture: LocalCheckAttestationVerificationPosture,
    command_id: LocalCheckCommandId,
    command_contract_fingerprint: SpecContentHash,
    requirement_fingerprint: SpecContentHash,
    immutable_run_bundle: ImmutableRunBundleBinding,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    invocation_id: SkillInvocationId,
    idempotency_key: IdempotencyKey,
    handler_fingerprint: SpecContentHash,
    result_id: LocalCheckResultId,
    result_status: LocalCheckResultStatus,
    exit_code_posture: LocalCheckAttestationExitCodePosture,
    stdout_truncated: bool,
    stderr_truncated: bool,
    observed_started_at: Timestamp,
    observed_completed_at: Timestamp,
    freshness: LocalCheckAttestationFreshnessPolicy,
    binding_fingerprint: SpecContentHash,
}

/// Explicit fields for constructing an unverified attestation candidate.
pub struct LocalCheckAttestationBindingDefinition {
    /// Candidate attestation identity.
    pub attestation_id: LocalCheckAttestationId,
    /// Candidate binding algorithm.
    pub algorithm: LocalCheckAttestationAlgorithm,
    /// Claimed assurance posture.
    pub assurance: LocalCheckAttestationAssurance,
    /// Claimed observation source.
    pub source: LocalCheckAttestationSource,
    /// Command contract identity.
    pub command_id: LocalCheckCommandId,
    /// Canonical validated command-contract fingerprint.
    pub command_contract_fingerprint: SpecContentHash,
    /// Complete independent-check requirement fingerprint.
    pub requirement_fingerprint: SpecContentHash,
    /// Exact immutable-run bundle binding.
    pub immutable_run_bundle: ImmutableRunBundleBinding,
    /// Workflow identity.
    pub workflow_id: WorkflowId,
    /// Run identity.
    pub run_id: WorkflowRunId,
    /// Step identity.
    pub step_id: StepId,
    /// Kernel invocation identity candidate.
    pub invocation_id: SkillInvocationId,
    /// Invocation idempotency reference.
    pub idempotency_key: IdempotencyKey,
    /// Handler implementation/posture fingerprint.
    pub handler_fingerprint: SpecContentHash,
    /// Structured result identity.
    pub result_id: LocalCheckResultId,
    /// Structured result status.
    pub result_status: LocalCheckResultStatus,
    /// Structured process exit-code posture.
    pub exit_code_posture: LocalCheckAttestationExitCodePosture,
    /// Whether the bounded stdout summary was truncated.
    pub stdout_truncated: bool,
    /// Whether the bounded stderr summary was truncated.
    pub stderr_truncated: bool,
    /// Claimed observation start time.
    pub observed_started_at: Timestamp,
    /// Claimed observation completion time.
    pub observed_completed_at: Timestamp,
    /// Freshness policy bound into the candidate.
    pub freshness: LocalCheckAttestationFreshnessPolicy,
}

impl LocalCheckAttestationBinding {
    /// Creates a validated but explicitly unverified attestation candidate.
    ///
    /// # Errors
    ///
    /// Returns a stable validation error for inconsistent source/assurance,
    /// result/exit posture, or observation ordering. This constructor never
    /// accepts or independently verifies the candidate.
    pub fn new(
        definition: LocalCheckAttestationBindingDefinition,
    ) -> Result<Self, WorkflowOsError> {
        validate_source_assurance(definition.source, definition.assurance)?;
        validate_result_exit_posture(definition.result_status, definition.exit_code_posture)?;
        if definition.observed_started_at > definition.observed_completed_at {
            return Err(attestation_error(
                "binding.observation_order_invalid",
                "local check attestation observation ordering is invalid",
            ));
        }

        let mut binding = Self {
            attestation_id: definition.attestation_id,
            algorithm: definition.algorithm,
            assurance: definition.assurance,
            source: definition.source,
            verification_posture: LocalCheckAttestationVerificationPosture::Unverified,
            command_id: definition.command_id,
            command_contract_fingerprint: definition.command_contract_fingerprint,
            requirement_fingerprint: definition.requirement_fingerprint,
            immutable_run_bundle: definition.immutable_run_bundle,
            workflow_id: definition.workflow_id,
            run_id: definition.run_id,
            step_id: definition.step_id,
            invocation_id: definition.invocation_id,
            idempotency_key: definition.idempotency_key,
            handler_fingerprint: definition.handler_fingerprint,
            result_id: definition.result_id,
            result_status: definition.result_status,
            exit_code_posture: definition.exit_code_posture,
            stdout_truncated: definition.stdout_truncated,
            stderr_truncated: definition.stderr_truncated,
            observed_started_at: definition.observed_started_at,
            observed_completed_at: definition.observed_completed_at,
            freshness: definition.freshness,
            binding_fingerprint: SpecContentHash::from_bytes([]),
        };
        binding.binding_fingerprint = compute_binding_fingerprint(&binding);
        Ok(binding)
    }

    #[must_use]
    /// Returns the explicitly unverified posture.
    pub const fn verification_posture(&self) -> LocalCheckAttestationVerificationPosture {
        self.verification_posture
    }

    #[must_use]
    /// Returns the claimed assurance.
    pub const fn assurance(&self) -> LocalCheckAttestationAssurance {
        self.assurance
    }

    #[must_use]
    /// Returns the claimed source.
    pub const fn source(&self) -> LocalCheckAttestationSource {
        self.source
    }

    #[must_use]
    /// Returns the deterministic candidate binding fingerprint.
    pub const fn binding_fingerprint(&self) -> &SpecContentHash {
        &self.binding_fingerprint
    }

    #[must_use]
    /// Returns the immutable-run binding.
    pub const fn immutable_run_bundle(&self) -> &ImmutableRunBundleBinding {
        &self.immutable_run_bundle
    }

    #[must_use]
    /// Returns the structured result status.
    pub const fn result_status(&self) -> LocalCheckResultStatus {
        self.result_status
    }

    /// Returns whether the declared posture may be submitted to the future v0
    /// verifier. This does not verify or satisfy a requirement.
    #[must_use]
    pub const fn eligible_for_v0_verification(&self) -> bool {
        self.assurance.eligible_for_v0_verification()
            && matches!(
                self.source,
                LocalCheckAttestationSource::KernelLocalProcessRunner
            )
    }
}

impl fmt::Debug for LocalCheckAttestationBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCheckAttestationBinding")
            .field("attestation_id", &"[REDACTED]")
            .field("algorithm", &self.algorithm)
            .field("assurance", &self.assurance)
            .field("source", &self.source)
            .field("verification_posture", &self.verification_posture)
            .field("identities", &"[REDACTED]")
            .field("fingerprints", &"[REDACTED]")
            .field("result_status", &self.result_status)
            .field("exit_code_posture", &self.exit_code_posture)
            .field("stdout_truncated", &self.stdout_truncated)
            .field("stderr_truncated", &self.stderr_truncated)
            .field("observation_times", &"[REDACTED]")
            .field("freshness", &self.freshness)
            .finish_non_exhaustive()
    }
}

#[derive(Deserialize)]
struct LocalCheckAttestationBindingWire {
    attestation_id: LocalCheckAttestationId,
    algorithm: LocalCheckAttestationAlgorithm,
    assurance: LocalCheckAttestationAssurance,
    source: LocalCheckAttestationSource,
    verification_posture: LocalCheckAttestationVerificationPosture,
    command_id: LocalCheckCommandId,
    command_contract_fingerprint: SpecContentHash,
    requirement_fingerprint: SpecContentHash,
    immutable_run_bundle: ImmutableRunBundleBinding,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    invocation_id: SkillInvocationId,
    idempotency_key: IdempotencyKey,
    handler_fingerprint: SpecContentHash,
    result_id: LocalCheckResultId,
    result_status: LocalCheckResultStatus,
    exit_code_posture: LocalCheckAttestationExitCodePosture,
    stdout_truncated: bool,
    stderr_truncated: bool,
    observed_started_at: Timestamp,
    observed_completed_at: Timestamp,
    freshness: LocalCheckAttestationFreshnessPolicy,
    binding_fingerprint: SpecContentHash,
}

impl<'de> Deserialize<'de> for LocalCheckAttestationBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = LocalCheckAttestationBindingWire::deserialize(deserializer)?;
        if wire.verification_posture != LocalCheckAttestationVerificationPosture::Unverified {
            return Err(serde::de::Error::custom(
                "invalid local check attestation binding",
            ));
        }
        let expected_fingerprint = wire.binding_fingerprint.clone();
        let binding = Self::new(LocalCheckAttestationBindingDefinition {
            attestation_id: wire.attestation_id,
            algorithm: wire.algorithm,
            assurance: wire.assurance,
            source: wire.source,
            command_id: wire.command_id,
            command_contract_fingerprint: wire.command_contract_fingerprint,
            requirement_fingerprint: wire.requirement_fingerprint,
            immutable_run_bundle: wire.immutable_run_bundle,
            workflow_id: wire.workflow_id,
            run_id: wire.run_id,
            step_id: wire.step_id,
            invocation_id: wire.invocation_id,
            idempotency_key: wire.idempotency_key,
            handler_fingerprint: wire.handler_fingerprint,
            result_id: wire.result_id,
            result_status: wire.result_status,
            exit_code_posture: wire.exit_code_posture,
            stdout_truncated: wire.stdout_truncated,
            stderr_truncated: wire.stderr_truncated,
            observed_started_at: wire.observed_started_at,
            observed_completed_at: wire.observed_completed_at,
            freshness: wire.freshness,
        })
        .map_err(|_| serde::de::Error::custom("invalid local check attestation binding"))?;
        if binding.binding_fingerprint != expected_fingerprint {
            return Err(serde::de::Error::custom(
                "invalid local check attestation binding",
            ));
        }
        Ok(binding)
    }
}

fn validate_source_assurance(
    source: LocalCheckAttestationSource,
    assurance: LocalCheckAttestationAssurance,
) -> Result<(), WorkflowOsError> {
    let aligned = matches!(
        (source, assurance),
        (
            LocalCheckAttestationSource::Caller,
            LocalCheckAttestationAssurance::CallerAsserted
        ) | (
            LocalCheckAttestationSource::MockHandler,
            LocalCheckAttestationAssurance::MockObserved
        ) | (
            LocalCheckAttestationSource::KernelLocalProcessRunner,
            LocalCheckAttestationAssurance::KernelObservedLocalProcess
        ) | (
            LocalCheckAttestationSource::ExternalVerifier,
            LocalCheckAttestationAssurance::ExternalVerifier
        )
    );
    if !aligned {
        return Err(attestation_error(
            "binding.source_assurance_mismatch",
            "local check attestation source and assurance do not align",
        ));
    }
    Ok(())
}

fn validate_result_exit_posture(
    status: LocalCheckResultStatus,
    exit: LocalCheckAttestationExitCodePosture,
) -> Result<(), WorkflowOsError> {
    let aligned = match status {
        LocalCheckResultStatus::Passed => exit == LocalCheckAttestationExitCodePosture::Zero,
        LocalCheckResultStatus::Failed => exit == LocalCheckAttestationExitCodePosture::NonZero,
        LocalCheckResultStatus::TimedOut
        | LocalCheckResultStatus::Skipped
        | LocalCheckResultStatus::NotAvailable
        | LocalCheckResultStatus::InternalError
        | LocalCheckResultStatus::PolicyDenied
        | LocalCheckResultStatus::RedactionFailed => {
            exit == LocalCheckAttestationExitCodePosture::Unavailable
        }
    };
    if !aligned {
        return Err(attestation_error(
            "binding.result_exit_mismatch",
            "local check attestation result and exit posture do not align",
        ));
    }
    Ok(())
}

fn compute_binding_fingerprint(binding: &LocalCheckAttestationBinding) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "algorithm", binding.algorithm.identifier());
    hash_field(&mut hasher, "assurance", assurance_label(binding.assurance));
    hash_field(&mut hasher, "source", source_label(binding.source));
    hash_field(&mut hasher, "command_id", binding.command_id.as_str());
    hash_field(
        &mut hasher,
        "command_contract_fingerprint",
        binding.command_contract_fingerprint.as_str(),
    );
    hash_field(
        &mut hasher,
        "requirement_fingerprint",
        binding.requirement_fingerprint.as_str(),
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
        "bundle_root",
        binding.immutable_run_bundle.root_hash().as_str(),
    );
    hash_field(&mut hasher, "workflow_id", binding.workflow_id.as_str());
    hash_field(&mut hasher, "run_id", binding.run_id.as_str());
    hash_field(&mut hasher, "step_id", binding.step_id.as_str());
    hash_field(&mut hasher, "invocation_id", binding.invocation_id.as_str());
    hash_field(
        &mut hasher,
        "idempotency_key",
        binding.idempotency_key.as_str(),
    );
    hash_field(
        &mut hasher,
        "handler_fingerprint",
        binding.handler_fingerprint.as_str(),
    );
    hash_field(&mut hasher, "result_id", binding.result_id.as_str());
    hash_field(
        &mut hasher,
        "result_status",
        &binding.result_status.to_string(),
    );
    hash_field(
        &mut hasher,
        "exit_code_posture",
        exit_code_label(binding.exit_code_posture),
    );
    hash_field(
        &mut hasher,
        "stdout_truncated",
        bool_label(binding.stdout_truncated),
    );
    hash_field(
        &mut hasher,
        "stderr_truncated",
        bool_label(binding.stderr_truncated),
    );
    hash_field(
        &mut hasher,
        "observed_started_at",
        &binding.observed_started_at.to_rfc3339(),
    );
    hash_field(
        &mut hasher,
        "observed_completed_at",
        &binding.observed_completed_at.to_rfc3339(),
    );
    match binding.freshness {
        LocalCheckAttestationFreshnessPolicy::NoReuse => {
            hash_field(&mut hasher, "freshness", "no_reuse");
        }
        LocalCheckAttestationFreshnessPolicy::MaxAgeSeconds { seconds } => {
            hash_field(&mut hasher, "freshness", "max_age_seconds");
            hash_field(&mut hasher, "freshness_seconds", &seconds.to_string());
        }
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn compute_requirement_fingerprint(
    requirement: &LocalCheckAttestationRequirement,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(
        &mut hasher,
        "algorithm",
        "workflow-os/local-check-attestation-requirement/v1",
    );
    hash_field(&mut hasher, "command_id", requirement.command_id.as_str());
    hash_field(
        &mut hasher,
        "minimum_assurance",
        assurance_label(requirement.minimum_assurance),
    );
    for status in &requirement.accepted_statuses {
        hash_field(&mut hasher, "accepted_status", &status.to_string());
    }
    match requirement.freshness {
        LocalCheckAttestationFreshnessPolicy::NoReuse => {
            hash_field(&mut hasher, "freshness", "no_reuse");
        }
        LocalCheckAttestationFreshnessPolicy::MaxAgeSeconds { seconds } => {
            hash_field(&mut hasher, "freshness", "max_age_seconds");
            hash_field(&mut hasher, "freshness_seconds", &seconds.to_string());
        }
    }
    hash_field(
        &mut hasher,
        "exact_immutable_run_binding_required",
        bool_label(requirement.exact_immutable_run_binding_required),
    );
    hash_field(
        &mut hasher,
        "truncation_allowed",
        bool_label(requirement.truncation_allowed),
    );
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update(u64::try_from(label.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(value.as_bytes());
}

const fn assurance_label(value: LocalCheckAttestationAssurance) -> &'static str {
    match value {
        LocalCheckAttestationAssurance::CallerAsserted => "caller_asserted",
        LocalCheckAttestationAssurance::MockObserved => "mock_observed",
        LocalCheckAttestationAssurance::KernelObservedLocalProcess => {
            "kernel_observed_local_process"
        }
        LocalCheckAttestationAssurance::ExternalVerifier => "external_verifier",
    }
}

const fn source_label(value: LocalCheckAttestationSource) -> &'static str {
    match value {
        LocalCheckAttestationSource::Caller => "caller",
        LocalCheckAttestationSource::MockHandler => "mock_handler",
        LocalCheckAttestationSource::KernelLocalProcessRunner => "kernel_local_process_runner",
        LocalCheckAttestationSource::ExternalVerifier => "external_verifier",
    }
}

const fn exit_code_label(value: LocalCheckAttestationExitCodePosture) -> &'static str {
    match value {
        LocalCheckAttestationExitCodePosture::Zero => "zero",
        LocalCheckAttestationExitCodePosture::NonZero => "non_zero",
        LocalCheckAttestationExitCodePosture::Unavailable => "unavailable",
    }
}

const fn bool_label(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn attestation_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("local_check_attestation.{suffix}"), message)
}
