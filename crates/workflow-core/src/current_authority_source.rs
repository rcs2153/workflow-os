use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};
use time::Duration;

use crate::{
    CurrentAuthorityQuerySet, RequiredContextContractBinding, RequiredContextExecutionBinding,
    SpecContentHash, Timestamp, WorkReportSensitivity, WorkflowOsError,
};

#[allow(dead_code)]
mod registered_in_memory_source;

pub(crate) use registered_in_memory_source::{
    RegisteredCurrentAuthorityConsumerResult, RegisteredCurrentAuthorityUseInput,
    RegisteredCurrentAuthorityUsePosture, RegisteredInMemoryCurrentAuthoritySource,
    SuccessfulWorkReportMetadataReadProof,
};

const SOURCE_ID_MAX_BYTES: usize = 128;
const SOURCE_VERSION_MAX_BYTES: usize = 64;
const SNAPSHOT_TOKEN_MAX_BYTES: usize = 192;
const MAX_OBSERVATION_AGE_SECONDS: u32 = 31_536_000;
const MAX_COMMITTED_FACT_COUNT: u64 = 1_000_000;

/// Versioned production current-authority source-boundary model.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentAuthoritySourceModelVersion {
    /// Initial payload-free source-boundary commitment model.
    V1,
}

impl<'de> Deserialize<'de> for CurrentAuthoritySourceModelVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "v1" => Ok(Self::V1),
            _ => Err(serde::de::Error::custom(
                "current authority source model version is invalid",
            )),
        }
    }
}

macro_rules! bounded_source_identifier {
    ($(#[$meta:meta])* $name:ident, $max_bytes:expr, $label:literal) => {
        $(#[$meta])*
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            /// Creates a bounded non-secret identifier.
            ///
            /// # Errors
            ///
            /// Returns a stable non-leaking validation error for invalid text.
            pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
                let value = value.into();
                validate_source_identifier($label, &value, $max_bytes)?;
                Ok(Self(value))
            }

            /// Returns the validated identifier.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&"[REDACTED]")
                    .finish()
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = WorkflowOsError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::new(value)
                    .map_err(|_| serde::de::Error::custom(concat!("invalid ", $label)))
            }
        }
    };
}

bounded_source_identifier!(
    /// Stable identity for a configured current-authority source.
    CurrentAuthoritySourceId,
    SOURCE_ID_MAX_BYTES,
    "current authority source id"
);

bounded_source_identifier!(
    /// Version of a current-authority source contract.
    CurrentAuthoritySourceContractVersion,
    SOURCE_VERSION_MAX_BYTES,
    "current authority source contract version"
);

bounded_source_identifier!(
    /// Opaque identity for one source snapshot.
    CurrentAuthoritySourceSnapshotId,
    SNAPSHOT_TOKEN_MAX_BYTES,
    "current authority source snapshot id"
);

/// Opaque equality token for one source snapshot.
///
/// This token intentionally does not implement ordering. It proves snapshot
/// identity or change, not monotonic progression.
#[derive(Clone, Eq, PartialEq, Hash, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct CurrentAuthoritySourceWatermark(String);

impl CurrentAuthoritySourceWatermark {
    /// Creates a bounded non-secret watermark.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error for invalid text.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_source_identifier(
            "current authority source watermark",
            &value,
            SNAPSHOT_TOKEN_MAX_BYTES,
        )?;
        Ok(Self(value))
    }

    /// Returns the opaque watermark.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for CurrentAuthoritySourceWatermark {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CurrentAuthoritySourceWatermark")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<CurrentAuthoritySourceWatermark> for String {
    fn from(value: CurrentAuthoritySourceWatermark) -> Self {
        value.0
    }
}

impl TryFrom<String> for CurrentAuthoritySourceWatermark {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'de> Deserialize<'de> for CurrentAuthoritySourceWatermark {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::new(String::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("invalid current authority source watermark"))
    }
}

/// Optional source-defined comparable generation.
///
/// Ordering is meaningful only when the accepted source contract defines the
/// generation semantics. This type does not authenticate that contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub struct CurrentAuthoritySourceGeneration(u64);

impl CurrentAuthoritySourceGeneration {
    /// Creates a non-zero source generation.
    ///
    /// # Errors
    ///
    /// Returns a stable error when the generation is zero.
    pub fn new(value: u64) -> Result<Self, WorkflowOsError> {
        if value == 0 {
            return Err(source_error(
                "generation.zero",
                "current authority source generation must be non-zero",
            ));
        }
        Ok(Self(value))
    }

    /// Returns the source-defined generation.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl<'de> Deserialize<'de> for CurrentAuthoritySourceGeneration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::new(u64::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("current authority source generation is invalid"))
    }
}

/// Domain-neutral source implementation posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentAuthoritySourceKind {
    /// One local aggregate source.
    LocalAggregate,
    /// One externally backed aggregate source.
    ExternalAggregate,
    /// A Core-owned coordinator over multiple registered sources.
    CoordinatedAggregate,
}

impl<'de> Deserialize<'de> for CurrentAuthoritySourceKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "local_aggregate" => Ok(Self::LocalAggregate),
            "external_aggregate" => Ok(Self::ExternalAggregate),
            "coordinated_aggregate" => Ok(Self::CoordinatedAggregate),
            _ => Err(serde::de::Error::custom(
                "current authority source kind is invalid",
            )),
        }
    }
}

/// Current-authority fact family supplied by a source boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentAuthorityFactFamily {
    /// Scoped capability grants.
    CapabilityGrants,
    /// Capability/resource availability observations.
    CapabilityAvailability,
    /// Governed context reference posture.
    GovernedContextReferences,
}

impl<'de> Deserialize<'de> for CurrentAuthorityFactFamily {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "capability_grants" => Ok(Self::CapabilityGrants),
            "capability_availability" => Ok(Self::CapabilityAvailability),
            "governed_context_references" => Ok(Self::GovernedContextReferences),
            _ => Err(serde::de::Error::custom(
                "current authority fact family is invalid",
            )),
        }
    }
}

/// Source consistency posture for one read.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentAuthoritySourceConsistency {
    /// All facts came from one atomic source snapshot.
    AtomicSnapshot,
    /// The source proved one stable watermark across the read.
    StableWatermark,
    /// The source made a bounded best-effort read without coherence proof.
    BestEffort,
    /// Consistency is unknown.
    Unknown,
}

impl<'de> Deserialize<'de> for CurrentAuthoritySourceConsistency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "atomic_snapshot" => Ok(Self::AtomicSnapshot),
            "stable_watermark" => Ok(Self::StableWatermark),
            "best_effort" => Ok(Self::BestEffort),
            "unknown" => Ok(Self::Unknown),
            _ => Err(serde::de::Error::custom(
                "current authority source consistency is invalid",
            )),
        }
    }
}

/// Exact-query completeness posture claimed by a source snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentAuthoritySourceCompleteness {
    /// Every requested fact family and exact target was evaluated.
    CompleteForExactQuery,
    /// Some requested facts are missing.
    Incomplete,
    /// The source does not support the requested query.
    Unsupported,
    /// The source could not be reached or read.
    Unavailable,
    /// Completeness is unknown.
    Unknown,
}

impl<'de> Deserialize<'de> for CurrentAuthoritySourceCompleteness {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "complete_for_exact_query" => Ok(Self::CompleteForExactQuery),
            "incomplete" => Ok(Self::Incomplete),
            "unsupported" => Ok(Self::Unsupported),
            "unavailable" => Ok(Self::Unavailable),
            "unknown" => Ok(Self::Unknown),
            _ => Err(serde::de::Error::custom(
                "current authority source completeness is invalid",
            )),
        }
    }
}

/// Deterministic freshness posture for one source snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentAuthoritySourceFreshness {
    /// Observation is within both source and Core freshness bounds.
    Fresh,
    /// Observation is older than the effective freshness bound.
    Stale,
    /// Observation is later than the requested evaluation time.
    FutureDated,
}

impl<'de> Deserialize<'de> for CurrentAuthoritySourceFreshness {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "fresh" => Ok(Self::Fresh),
            "stale" => Ok(Self::Stale),
            "future_dated" => Ok(Self::FutureDated),
            _ => Err(serde::de::Error::custom(
                "current authority source freshness is invalid",
            )),
        }
    }
}

/// Model-only registration input.
pub struct CurrentAuthoritySourceRegistrationInput {
    /// Stable source identity.
    pub source_id: CurrentAuthoritySourceId,
    /// Source contract version.
    pub contract_version: CurrentAuthoritySourceContractVersion,
    /// Source implementation posture.
    pub source_kind: CurrentAuthoritySourceKind,
    /// Hash of normalized, credential-free configuration.
    pub configuration_commitment: SpecContentHash,
    /// Fact families the source claims to support.
    pub supported_fact_families: Vec<CurrentAuthorityFactFamily>,
    /// Consistency posture the source claims to support.
    pub consistency: CurrentAuthoritySourceConsistency,
    /// Core-owned maximum accepted observation age.
    pub core_maximum_observation_age_seconds: u32,
    /// Maximum sensitivity accepted from this source.
    pub sensitivity: WorkReportSensitivity,
    /// Whether source output must remain redacted and payload-free.
    pub redaction_required: bool,
}

/// Payload-free source registration commitment.
///
/// Construction does not authenticate a source. Trusted registration remains
/// a future Core-owned runtime boundary.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CurrentAuthoritySourceRegistration {
    model_version: CurrentAuthoritySourceModelVersion,
    source_id: CurrentAuthoritySourceId,
    contract_version: CurrentAuthoritySourceContractVersion,
    source_kind: CurrentAuthoritySourceKind,
    configuration_commitment: SpecContentHash,
    supported_fact_families: Vec<CurrentAuthorityFactFamily>,
    consistency: CurrentAuthoritySourceConsistency,
    core_maximum_observation_age_seconds: u32,
    sensitivity: WorkReportSensitivity,
    redaction_required: bool,
    registration_commitment: SpecContentHash,
}

impl CurrentAuthoritySourceRegistration {
    /// Creates a model-only source registration commitment.
    ///
    /// # Errors
    ///
    /// Returns stable non-leaking errors for invalid or duplicate posture.
    pub fn new(input: CurrentAuthoritySourceRegistrationInput) -> Result<Self, WorkflowOsError> {
        let mut supported_fact_families = input.supported_fact_families;
        canonicalize_families(&mut supported_fact_families)?;
        if input.consistency == CurrentAuthoritySourceConsistency::Unknown {
            return Err(source_error(
                "registration.consistency_unknown",
                "current authority source registration needs known consistency",
            ));
        }
        if input.core_maximum_observation_age_seconds == 0
            || input.core_maximum_observation_age_seconds > MAX_OBSERVATION_AGE_SECONDS
        {
            return Err(source_error(
                "registration.freshness_bound_invalid",
                "current authority source freshness bound is invalid",
            ));
        }
        if input.sensitivity == WorkReportSensitivity::Unknown {
            return Err(source_error(
                "registration.sensitivity_unknown",
                "current authority source registration needs known sensitivity",
            ));
        }
        if !input.redaction_required {
            return Err(source_error(
                "registration.redaction_required",
                "current authority source registration requires redaction",
            ));
        }
        let mut value = Self {
            model_version: CurrentAuthoritySourceModelVersion::V1,
            source_id: input.source_id,
            contract_version: input.contract_version,
            source_kind: input.source_kind,
            configuration_commitment: input.configuration_commitment,
            supported_fact_families,
            consistency: input.consistency,
            core_maximum_observation_age_seconds: input.core_maximum_observation_age_seconds,
            sensitivity: input.sensitivity,
            redaction_required: input.redaction_required,
            registration_commitment: pending_hash(),
        };
        value.registration_commitment = value.compute_commitment()?;
        value.validate()?;
        Ok(value)
    }

    /// Validates the registration commitment.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error for inconsistent state.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_canonical_families(&self.supported_fact_families)?;
        if self.consistency == CurrentAuthoritySourceConsistency::Unknown
            || self.core_maximum_observation_age_seconds == 0
            || self.core_maximum_observation_age_seconds > MAX_OBSERVATION_AGE_SECONDS
            || self.sensitivity == WorkReportSensitivity::Unknown
            || !self.redaction_required
        {
            return Err(source_error(
                "registration.posture_invalid",
                "current authority source registration posture is invalid",
            ));
        }
        if self.registration_commitment != self.compute_commitment()? {
            return Err(source_error(
                "registration.commitment_mismatch",
                "current authority source registration commitment is invalid",
            ));
        }
        Ok(())
    }

    fn compute_commitment(&self) -> Result<SpecContentHash, WorkflowOsError> {
        hash_serializable(
            "registration",
            &(
                self.model_version,
                &self.source_id,
                &self.contract_version,
                self.source_kind,
                &self.configuration_commitment,
                &self.supported_fact_families,
                self.consistency,
                self.core_maximum_observation_age_seconds,
                self.sensitivity,
                self.redaction_required,
            ),
        )
    }

    /// Returns the stable source identity.
    #[must_use]
    pub const fn source_id(&self) -> &CurrentAuthoritySourceId {
        &self.source_id
    }

    /// Returns the registration commitment.
    #[must_use]
    pub const fn registration_commitment(&self) -> &SpecContentHash {
        &self.registration_commitment
    }

    /// Returns the supported fact families.
    #[must_use]
    pub fn supported_fact_families(&self) -> &[CurrentAuthorityFactFamily] {
        &self.supported_fact_families
    }

    /// Returns the Core-owned freshness cap.
    #[must_use]
    pub const fn core_maximum_observation_age_seconds(&self) -> u32 {
        self.core_maximum_observation_age_seconds
    }
}

impl fmt::Debug for CurrentAuthoritySourceRegistration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CurrentAuthoritySourceRegistration")
            .field("model_version", &self.model_version)
            .field("source_id", &"[REDACTED]")
            .field("contract_version", &"[REDACTED]")
            .field("source_kind", &self.source_kind)
            .field("configuration_commitment", &"[REDACTED]")
            .field("fact_family_count", &self.supported_fact_families.len())
            .field("consistency", &self.consistency)
            .field(
                "core_maximum_observation_age_seconds",
                &self.core_maximum_observation_age_seconds,
            )
            .field("sensitivity", &self.sensitivity)
            .field("redaction_required", &self.redaction_required)
            .field("registration_commitment", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for CurrentAuthoritySourceRegistration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            model_version: CurrentAuthoritySourceModelVersion,
            source_id: CurrentAuthoritySourceId,
            contract_version: CurrentAuthoritySourceContractVersion,
            source_kind: CurrentAuthoritySourceKind,
            configuration_commitment: SpecContentHash,
            supported_fact_families: Vec<CurrentAuthorityFactFamily>,
            consistency: CurrentAuthoritySourceConsistency,
            core_maximum_observation_age_seconds: u32,
            sensitivity: WorkReportSensitivity,
            redaction_required: bool,
            registration_commitment: SpecContentHash,
        }
        let wire = Wire::deserialize(deserializer)?;
        let value = Self {
            model_version: wire.model_version,
            source_id: wire.source_id,
            contract_version: wire.contract_version,
            source_kind: wire.source_kind,
            configuration_commitment: wire.configuration_commitment,
            supported_fact_families: wire.supported_fact_families,
            consistency: wire.consistency,
            core_maximum_observation_age_seconds: wire.core_maximum_observation_age_seconds,
            sensitivity: wire.sensitivity,
            redaction_required: wire.redaction_required,
            registration_commitment: wire.registration_commitment,
        };
        value.validate().map_err(|_| {
            serde::de::Error::custom("invalid current authority source registration")
        })?;
        Ok(value)
    }
}

/// Explicit request-construction input.
pub struct CurrentAuthoritySourceRequestInput<'a> {
    /// Model-only source registration.
    pub registration: &'a CurrentAuthoritySourceRegistration,
    /// Exact immutable execution binding.
    pub execution_binding: &'a RequiredContextExecutionBinding,
    /// Exact required-context contract.
    pub contract: &'a RequiredContextContractBinding,
    /// Fact families required from the source.
    pub requested_fact_families: Vec<CurrentAuthorityFactFamily>,
    /// Time at which current authority will be evaluated.
    pub evaluated_at: Timestamp,
}

/// Exact payload-free source request commitment.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CurrentAuthoritySourceRequest {
    model_version: CurrentAuthoritySourceModelVersion,
    registration_commitment: SpecContentHash,
    execution_binding_hash: SpecContentHash,
    contract_content_hash: SpecContentHash,
    query_set_hash: SpecContentHash,
    query_count: u64,
    requested_fact_families: Vec<CurrentAuthorityFactFamily>,
    maximum_sensitivity: WorkReportSensitivity,
    registered_sensitivity: WorkReportSensitivity,
    evaluated_at: Timestamp,
    request_commitment: SpecContentHash,
}

impl CurrentAuthoritySourceRequest {
    /// Creates an exact source request from validated immutable inputs.
    ///
    /// # Errors
    ///
    /// Returns stable non-leaking errors for mismatched bindings or unsupported
    /// fact families.
    pub fn new(input: CurrentAuthoritySourceRequestInput<'_>) -> Result<Self, WorkflowOsError> {
        input.registration.validate()?;
        input.execution_binding.validate()?;
        if input.execution_binding.contract_content_hash() != input.contract.content_hash() {
            return Err(source_error(
                "request.contract_mismatch",
                "current authority source request contract does not match execution binding",
            ));
        }
        if input.evaluated_at < input.execution_binding.bound_at() {
            return Err(source_error(
                "request.evaluation_before_binding",
                "current authority source request cannot predate its execution binding",
            ));
        }
        let mut requested_fact_families = input.requested_fact_families;
        canonicalize_families(&mut requested_fact_families)?;
        if requested_fact_families
            .iter()
            .any(|family| !input.registration.supported_fact_families.contains(family))
        {
            return Err(source_error(
                "request.family_unsupported",
                "current authority source does not support a requested fact family",
            ));
        }
        if input.execution_binding.maximum_sensitivity() == WorkReportSensitivity::Unknown {
            return Err(source_error(
                "request.sensitivity_unknown",
                "current authority source request needs known sensitivity",
            ));
        }
        if input.execution_binding.maximum_sensitivity() > input.registration.sensitivity {
            return Err(source_error(
                "request.sensitivity_exceeds_source",
                "current authority source request exceeds registered sensitivity",
            ));
        }
        let query_set = CurrentAuthorityQuerySet::from_contract(input.contract)?;
        let mut value = Self {
            model_version: CurrentAuthoritySourceModelVersion::V1,
            registration_commitment: input.registration.registration_commitment.clone(),
            execution_binding_hash: input.execution_binding.binding_hash().clone(),
            contract_content_hash: input.contract.content_hash().clone(),
            query_set_hash: query_set.query_set_hash().clone(),
            query_count: u64::try_from(query_set.queries().len()).unwrap_or(u64::MAX),
            requested_fact_families,
            maximum_sensitivity: input.execution_binding.maximum_sensitivity(),
            registered_sensitivity: input.registration.sensitivity,
            evaluated_at: input.evaluated_at,
            request_commitment: pending_hash(),
        };
        value.request_commitment = value.compute_commitment()?;
        value.validate()?;
        Ok(value)
    }

    /// Validates the exact request commitment.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error for inconsistent state.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_canonical_families(&self.requested_fact_families)?;
        if self.query_count == 0 || self.maximum_sensitivity == WorkReportSensitivity::Unknown {
            return Err(source_error(
                "request.posture_invalid",
                "current authority source request posture is invalid",
            ));
        }
        if self.registered_sensitivity == WorkReportSensitivity::Unknown
            || self.maximum_sensitivity > self.registered_sensitivity
        {
            return Err(source_error(
                "request.sensitivity_invalid",
                "current authority source request sensitivity is invalid",
            ));
        }
        if self.request_commitment != self.compute_commitment()? {
            return Err(source_error(
                "request.commitment_mismatch",
                "current authority source request commitment is invalid",
            ));
        }
        Ok(())
    }

    fn compute_commitment(&self) -> Result<SpecContentHash, WorkflowOsError> {
        hash_serializable(
            "request",
            &(
                self.model_version,
                &self.registration_commitment,
                &self.execution_binding_hash,
                &self.contract_content_hash,
                &self.query_set_hash,
                self.query_count,
                &self.requested_fact_families,
                self.maximum_sensitivity,
                self.registered_sensitivity,
                self.evaluated_at,
            ),
        )
    }

    /// Returns the registration commitment.
    #[must_use]
    pub const fn registration_commitment(&self) -> &SpecContentHash {
        &self.registration_commitment
    }

    /// Returns the exact request commitment.
    #[must_use]
    pub const fn request_commitment(&self) -> &SpecContentHash {
        &self.request_commitment
    }

    /// Returns the exact query count.
    #[must_use]
    pub const fn query_count(&self) -> u64 {
        self.query_count
    }

    /// Returns requested fact families.
    #[must_use]
    pub fn requested_fact_families(&self) -> &[CurrentAuthorityFactFamily] {
        &self.requested_fact_families
    }
}

impl fmt::Debug for CurrentAuthoritySourceRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CurrentAuthoritySourceRequest")
            .field("model_version", &self.model_version)
            .field("registration_commitment", &"[REDACTED]")
            .field("execution_binding_hash", &"[REDACTED]")
            .field("contract_content_hash", &"[REDACTED]")
            .field("query_set_hash", &"[REDACTED]")
            .field("query_count", &self.query_count)
            .field("fact_family_count", &self.requested_fact_families.len())
            .field("maximum_sensitivity", &self.maximum_sensitivity)
            .field("registered_sensitivity", &self.registered_sensitivity)
            .field("evaluated_at", &"[REDACTED]")
            .field("request_commitment", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for CurrentAuthoritySourceRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            model_version: CurrentAuthoritySourceModelVersion,
            registration_commitment: SpecContentHash,
            execution_binding_hash: SpecContentHash,
            contract_content_hash: SpecContentHash,
            query_set_hash: SpecContentHash,
            query_count: u64,
            requested_fact_families: Vec<CurrentAuthorityFactFamily>,
            maximum_sensitivity: WorkReportSensitivity,
            registered_sensitivity: WorkReportSensitivity,
            evaluated_at: Timestamp,
            request_commitment: SpecContentHash,
        }
        let wire = Wire::deserialize(deserializer)?;
        let value = Self {
            model_version: wire.model_version,
            registration_commitment: wire.registration_commitment,
            execution_binding_hash: wire.execution_binding_hash,
            contract_content_hash: wire.contract_content_hash,
            query_set_hash: wire.query_set_hash,
            query_count: wire.query_count,
            requested_fact_families: wire.requested_fact_families,
            maximum_sensitivity: wire.maximum_sensitivity,
            registered_sensitivity: wire.registered_sensitivity,
            evaluated_at: wire.evaluated_at,
            request_commitment: wire.request_commitment,
        };
        value
            .validate()
            .map_err(|_| serde::de::Error::custom("invalid current authority source request"))?;
        Ok(value)
    }
}

/// Validated temporal boundary for one source read.
#[allow(clippy::struct_field_names)]
#[derive(Clone, Copy, Eq, PartialEq, Serialize)]
pub struct CurrentAuthoritySourceReadWindow {
    started_at: Timestamp,
    observed_at: Timestamp,
    completed_at: Timestamp,
}

impl CurrentAuthoritySourceReadWindow {
    /// Creates a read window.
    ///
    /// # Errors
    ///
    /// Returns a stable error when timestamps are not ordered.
    pub fn new(
        started_at: Timestamp,
        observed_at: Timestamp,
        completed_at: Timestamp,
    ) -> Result<Self, WorkflowOsError> {
        let value = Self {
            started_at,
            observed_at,
            completed_at,
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.observed_at < self.started_at || self.completed_at < self.observed_at {
            return Err(source_error(
                "read_window.order_invalid",
                "current authority source read window is invalid",
            ));
        }
        Ok(())
    }

    /// Returns the source observation time.
    #[must_use]
    pub const fn observed_at(self) -> Timestamp {
        self.observed_at
    }
}

impl fmt::Debug for CurrentAuthoritySourceReadWindow {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CurrentAuthoritySourceReadWindow")
            .field("started_at", &"[REDACTED]")
            .field("observed_at", &"[REDACTED]")
            .field("completed_at", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for CurrentAuthoritySourceReadWindow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[allow(clippy::struct_field_names)]
        #[derive(Deserialize)]
        struct Wire {
            started_at: Timestamp,
            observed_at: Timestamp,
            completed_at: Timestamp,
        }
        let wire = Wire::deserialize(deserializer)?;
        Self::new(wire.started_at, wire.observed_at, wire.completed_at)
            .map_err(|_| serde::de::Error::custom("invalid current authority source read window"))
    }
}

/// Count of bounded records committed for one fact family.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct CurrentAuthoritySourceFactCount {
    family: CurrentAuthorityFactFamily,
    count: u64,
}

impl CurrentAuthoritySourceFactCount {
    /// Creates one family count.
    #[must_use]
    pub const fn new(family: CurrentAuthorityFactFamily, count: u64) -> Self {
        Self { family, count }
    }

    /// Returns the fact family.
    #[must_use]
    pub const fn family(self) -> CurrentAuthorityFactFamily {
        self.family
    }

    /// Returns the committed count.
    #[must_use]
    pub const fn count(self) -> u64 {
        self.count
    }
}

/// Explicit payload-free snapshot-construction input.
pub struct CurrentAuthoritySourceSnapshotInput<'a> {
    /// Exact source request.
    pub request: &'a CurrentAuthoritySourceRequest,
    /// Source registration used for freshness policy.
    pub registration: &'a CurrentAuthoritySourceRegistration,
    /// Opaque snapshot identity.
    pub snapshot_id: CurrentAuthoritySourceSnapshotId,
    /// Opaque snapshot equality token.
    pub watermark: CurrentAuthoritySourceWatermark,
    /// Optional source-defined comparable generation.
    pub generation: Option<CurrentAuthoritySourceGeneration>,
    /// Validated source read window.
    pub read_window: CurrentAuthoritySourceReadWindow,
    /// Exact-query completeness posture.
    pub completeness: CurrentAuthoritySourceCompleteness,
    /// Consistency observed for this read.
    pub consistency: CurrentAuthoritySourceConsistency,
    /// Optional source-supplied validity bound.
    pub source_valid_through: Option<Timestamp>,
    /// Fact families returned by the source.
    pub returned_fact_families: Vec<CurrentAuthorityFactFamily>,
    /// Bounded records committed per returned family.
    pub fact_counts: Vec<CurrentAuthoritySourceFactCount>,
    /// Hash of canonical bounded records held outside this commitment.
    pub records_commitment: SpecContentHash,
}

/// Payload-free commitment to one current-authority source snapshot.
///
/// This model cannot confer authority, readiness, or target access.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CurrentAuthoritySourceSnapshot {
    model_version: CurrentAuthoritySourceModelVersion,
    registration_commitment: SpecContentHash,
    request_commitment: SpecContentHash,
    requested_fact_families: Vec<CurrentAuthorityFactFamily>,
    query_count: u64,
    evaluated_at: Timestamp,
    snapshot_id: CurrentAuthoritySourceSnapshotId,
    watermark: CurrentAuthoritySourceWatermark,
    generation: Option<CurrentAuthoritySourceGeneration>,
    read_window: CurrentAuthoritySourceReadWindow,
    completeness: CurrentAuthoritySourceCompleteness,
    consistency: CurrentAuthoritySourceConsistency,
    registered_consistency: CurrentAuthoritySourceConsistency,
    source_valid_through: Option<Timestamp>,
    core_maximum_observation_age_seconds: u32,
    core_valid_through: Timestamp,
    effective_valid_through: Timestamp,
    freshness: CurrentAuthoritySourceFreshness,
    returned_fact_families: Vec<CurrentAuthorityFactFamily>,
    fact_counts: Vec<CurrentAuthoritySourceFactCount>,
    records_commitment: SpecContentHash,
    snapshot_commitment: SpecContentHash,
}

impl CurrentAuthoritySourceSnapshot {
    /// Creates a payload-free source snapshot commitment.
    ///
    /// # Errors
    ///
    /// Returns stable non-leaking errors for substitution, incomplete
    /// coverage, invalid time posture, or inconsistent commitments.
    #[allow(clippy::too_many_lines)]
    pub fn new(input: CurrentAuthoritySourceSnapshotInput<'_>) -> Result<Self, WorkflowOsError> {
        input.registration.validate()?;
        input.request.validate()?;
        if input.request.registration_commitment != *input.registration.registration_commitment() {
            return Err(source_error(
                "snapshot.registration_mismatch",
                "current authority source snapshot registration does not match request",
            ));
        }
        if input.consistency != input.registration.consistency {
            return Err(source_error(
                "snapshot.consistency_mismatch",
                "current authority source snapshot consistency does not match registration",
            ));
        }
        input.read_window.validate()?;
        let mut returned_fact_families = input.returned_fact_families;
        canonicalize_families_allow_empty(&mut returned_fact_families)?;
        let mut fact_counts = input.fact_counts;
        fact_counts.sort_by_key(|count| count.family);
        if fact_counts
            .windows(2)
            .any(|pair| pair[0].family == pair[1].family)
            || fact_counts
                .iter()
                .any(|count| !returned_fact_families.contains(&count.family))
            || fact_counts.len() != returned_fact_families.len()
        {
            return Err(source_error(
                "snapshot.fact_counts_invalid",
                "current authority source snapshot fact counts are invalid",
            ));
        }
        if input.completeness == CurrentAuthoritySourceCompleteness::CompleteForExactQuery {
            if returned_fact_families != input.request.requested_fact_families {
                return Err(source_error(
                    "snapshot.family_coverage_incomplete",
                    "complete current authority source snapshot needs exact family coverage",
                ));
            }
            for count in &fact_counts {
                if matches!(
                    count.family,
                    CurrentAuthorityFactFamily::CapabilityAvailability
                        | CurrentAuthorityFactFamily::GovernedContextReferences
                ) && count.count != input.request.query_count
                {
                    return Err(source_error(
                        "snapshot.query_coverage_incomplete",
                        "complete current authority source snapshot needs exact target coverage",
                    ));
                }
            }
        }

        let core_valid_through = add_seconds(
            input.read_window.observed_at,
            input.registration.core_maximum_observation_age_seconds,
        )?;
        let effective_valid_through = input
            .source_valid_through
            .map_or(core_valid_through, |source_bound| {
                source_bound.min(core_valid_through)
            });
        if input
            .source_valid_through
            .is_some_and(|bound| bound < input.read_window.observed_at)
        {
            return Err(source_error(
                "snapshot.source_validity_invalid",
                "current authority source validity bound is invalid",
            ));
        }
        let freshness = if input.read_window.observed_at > input.request.evaluated_at {
            CurrentAuthoritySourceFreshness::FutureDated
        } else if input.request.evaluated_at > effective_valid_through {
            CurrentAuthoritySourceFreshness::Stale
        } else {
            CurrentAuthoritySourceFreshness::Fresh
        };
        let mut value = Self {
            model_version: CurrentAuthoritySourceModelVersion::V1,
            registration_commitment: input.registration.registration_commitment.clone(),
            request_commitment: input.request.request_commitment.clone(),
            requested_fact_families: input.request.requested_fact_families.clone(),
            query_count: input.request.query_count,
            evaluated_at: input.request.evaluated_at,
            snapshot_id: input.snapshot_id,
            watermark: input.watermark,
            generation: input.generation,
            read_window: input.read_window,
            completeness: input.completeness,
            consistency: input.consistency,
            registered_consistency: input.registration.consistency,
            source_valid_through: input.source_valid_through,
            core_maximum_observation_age_seconds: input
                .registration
                .core_maximum_observation_age_seconds,
            core_valid_through,
            effective_valid_through,
            freshness,
            returned_fact_families,
            fact_counts,
            records_commitment: input.records_commitment,
            snapshot_commitment: pending_hash(),
        };
        value.snapshot_commitment = value.compute_commitment()?;
        value.validate()?;
        Ok(value)
    }

    /// Validates the aggregate source snapshot commitment.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error for inconsistent wire
    /// state.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.read_window.validate()?;
        validate_canonical_families(&self.requested_fact_families)?;
        validate_canonical_families_allow_empty(&self.returned_fact_families)?;
        if self
            .fact_counts
            .windows(2)
            .any(|pair| pair[0].family >= pair[1].family)
            || self.fact_counts.len() != self.returned_fact_families.len()
            || self
                .fact_counts
                .iter()
                .zip(&self.returned_fact_families)
                .any(|(count, family)| count.family != *family)
            || self
                .fact_counts
                .iter()
                .any(|count| count.count > MAX_COMMITTED_FACT_COUNT)
            || self.consistency != self.registered_consistency
            || self.registered_consistency == CurrentAuthoritySourceConsistency::Unknown
            || self.effective_valid_through > self.core_valid_through
            || self
                .source_valid_through
                .is_some_and(|bound| self.effective_valid_through > bound)
            || self
                .source_valid_through
                .is_some_and(|bound| bound < self.read_window.observed_at)
            || self.core_maximum_observation_age_seconds == 0
            || self.core_maximum_observation_age_seconds > MAX_OBSERVATION_AGE_SECONDS
            || self.core_valid_through
                != add_seconds(
                    self.read_window.observed_at,
                    self.core_maximum_observation_age_seconds,
                )?
            || self.effective_valid_through
                != self
                    .source_valid_through
                    .map_or(self.core_valid_through, |bound| {
                        bound.min(self.core_valid_through)
                    })
        {
            return Err(source_error(
                "snapshot.posture_invalid",
                "current authority source snapshot posture is invalid",
            ));
        }
        if self.completeness == CurrentAuthoritySourceCompleteness::CompleteForExactQuery {
            if self.returned_fact_families != self.requested_fact_families {
                return Err(source_error(
                    "snapshot.family_coverage_incomplete",
                    "complete current authority source snapshot needs exact family coverage",
                ));
            }
            for count in &self.fact_counts {
                if matches!(
                    count.family,
                    CurrentAuthorityFactFamily::CapabilityAvailability
                        | CurrentAuthorityFactFamily::GovernedContextReferences
                ) && count.count != self.query_count
                {
                    return Err(source_error(
                        "snapshot.query_coverage_incomplete",
                        "complete current authority source snapshot needs exact target coverage",
                    ));
                }
            }
        }
        let expected_freshness = if self.read_window.observed_at > self.evaluated_at {
            CurrentAuthoritySourceFreshness::FutureDated
        } else if self.evaluated_at > self.effective_valid_through {
            CurrentAuthoritySourceFreshness::Stale
        } else {
            CurrentAuthoritySourceFreshness::Fresh
        };
        if self.freshness != expected_freshness {
            return Err(source_error(
                "snapshot.freshness_mismatch",
                "current authority source snapshot freshness is invalid",
            ));
        }
        if self.snapshot_commitment != self.compute_commitment()? {
            return Err(source_error(
                "snapshot.commitment_mismatch",
                "current authority source snapshot commitment is invalid",
            ));
        }
        Ok(())
    }

    fn compute_commitment(&self) -> Result<SpecContentHash, WorkflowOsError> {
        #[derive(Serialize)]
        struct SnapshotCommitmentInput<'a> {
            model_version: CurrentAuthoritySourceModelVersion,
            registration_commitment: &'a SpecContentHash,
            request_commitment: &'a SpecContentHash,
            requested_fact_families: &'a [CurrentAuthorityFactFamily],
            query_count: u64,
            evaluated_at: Timestamp,
            snapshot_id: &'a CurrentAuthoritySourceSnapshotId,
            watermark: &'a CurrentAuthoritySourceWatermark,
            generation: Option<CurrentAuthoritySourceGeneration>,
            read_window: CurrentAuthoritySourceReadWindow,
            completeness: CurrentAuthoritySourceCompleteness,
            consistency: CurrentAuthoritySourceConsistency,
            registered_consistency: CurrentAuthoritySourceConsistency,
            source_valid_through: Option<Timestamp>,
            core_maximum_observation_age_seconds: u32,
            core_valid_through: Timestamp,
            effective_valid_through: Timestamp,
            freshness: CurrentAuthoritySourceFreshness,
            returned_fact_families: &'a [CurrentAuthorityFactFamily],
            fact_counts: &'a [CurrentAuthoritySourceFactCount],
            records_commitment: &'a SpecContentHash,
        }
        hash_serializable(
            "snapshot",
            &SnapshotCommitmentInput {
                model_version: self.model_version,
                registration_commitment: &self.registration_commitment,
                request_commitment: &self.request_commitment,
                requested_fact_families: &self.requested_fact_families,
                query_count: self.query_count,
                evaluated_at: self.evaluated_at,
                snapshot_id: &self.snapshot_id,
                watermark: &self.watermark,
                generation: self.generation,
                read_window: self.read_window,
                completeness: self.completeness,
                consistency: self.consistency,
                registered_consistency: self.registered_consistency,
                source_valid_through: self.source_valid_through,
                core_maximum_observation_age_seconds: self.core_maximum_observation_age_seconds,
                core_valid_through: self.core_valid_through,
                effective_valid_through: self.effective_valid_through,
                freshness: self.freshness,
                returned_fact_families: &self.returned_fact_families,
                fact_counts: &self.fact_counts,
                records_commitment: &self.records_commitment,
            },
        )
    }

    /// Returns the completeness posture.
    #[must_use]
    pub const fn completeness(&self) -> CurrentAuthoritySourceCompleteness {
        self.completeness
    }

    /// Returns the deterministic freshness posture.
    #[must_use]
    pub const fn freshness(&self) -> CurrentAuthoritySourceFreshness {
        self.freshness
    }

    /// Returns the optional source-defined generation.
    #[must_use]
    pub const fn generation(&self) -> Option<CurrentAuthoritySourceGeneration> {
        self.generation
    }

    /// Returns the aggregate snapshot commitment.
    #[must_use]
    pub const fn snapshot_commitment(&self) -> &SpecContentHash {
        &self.snapshot_commitment
    }
}

impl fmt::Debug for CurrentAuthoritySourceSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CurrentAuthoritySourceSnapshot")
            .field("model_version", &self.model_version)
            .field("registration_commitment", &"[REDACTED]")
            .field("request_commitment", &"[REDACTED]")
            .field(
                "requested_fact_family_count",
                &self.requested_fact_families.len(),
            )
            .field("query_count", &self.query_count)
            .field("evaluated_at", &"[REDACTED]")
            .field("snapshot_id", &"[REDACTED]")
            .field("watermark", &"[REDACTED]")
            .field("generation", &self.generation)
            .field("read_window", &self.read_window)
            .field("completeness", &self.completeness)
            .field("consistency", &self.consistency)
            .field("registered_consistency", &self.registered_consistency)
            .field("freshness", &self.freshness)
            .field("fact_family_count", &self.returned_fact_families.len())
            .field("records_commitment", &"[REDACTED]")
            .field("snapshot_commitment", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl<'de> Deserialize<'de> for CurrentAuthoritySourceSnapshot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            model_version: CurrentAuthoritySourceModelVersion,
            registration_commitment: SpecContentHash,
            request_commitment: SpecContentHash,
            requested_fact_families: Vec<CurrentAuthorityFactFamily>,
            query_count: u64,
            evaluated_at: Timestamp,
            snapshot_id: CurrentAuthoritySourceSnapshotId,
            watermark: CurrentAuthoritySourceWatermark,
            generation: Option<CurrentAuthoritySourceGeneration>,
            read_window: CurrentAuthoritySourceReadWindow,
            completeness: CurrentAuthoritySourceCompleteness,
            consistency: CurrentAuthoritySourceConsistency,
            registered_consistency: CurrentAuthoritySourceConsistency,
            source_valid_through: Option<Timestamp>,
            core_maximum_observation_age_seconds: u32,
            core_valid_through: Timestamp,
            effective_valid_through: Timestamp,
            freshness: CurrentAuthoritySourceFreshness,
            returned_fact_families: Vec<CurrentAuthorityFactFamily>,
            fact_counts: Vec<CurrentAuthoritySourceFactCount>,
            records_commitment: SpecContentHash,
            snapshot_commitment: SpecContentHash,
        }
        let wire = Wire::deserialize(deserializer)?;
        let value = Self {
            model_version: wire.model_version,
            registration_commitment: wire.registration_commitment,
            request_commitment: wire.request_commitment,
            requested_fact_families: wire.requested_fact_families,
            query_count: wire.query_count,
            evaluated_at: wire.evaluated_at,
            snapshot_id: wire.snapshot_id,
            watermark: wire.watermark,
            generation: wire.generation,
            read_window: wire.read_window,
            completeness: wire.completeness,
            consistency: wire.consistency,
            registered_consistency: wire.registered_consistency,
            source_valid_through: wire.source_valid_through,
            core_maximum_observation_age_seconds: wire.core_maximum_observation_age_seconds,
            core_valid_through: wire.core_valid_through,
            effective_valid_through: wire.effective_valid_through,
            freshness: wire.freshness,
            returned_fact_families: wire.returned_fact_families,
            fact_counts: wire.fact_counts,
            records_commitment: wire.records_commitment,
            snapshot_commitment: wire.snapshot_commitment,
        };
        value
            .validate()
            .map_err(|_| serde::de::Error::custom("invalid current authority source snapshot"))?;
        Ok(value)
    }
}

/// Stable source-boundary failure category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentAuthoritySourceFailureKind {
    /// Configured source was unavailable.
    Unavailable,
    /// Exact request is unsupported.
    Unsupported,
    /// Response did not cover the exact request.
    Incomplete,
    /// Response was stale.
    Stale,
    /// Observation was in the future.
    FutureDated,
    /// Source changed without a coherent snapshot.
    ConcurrentChange,
    /// Source returned ambiguous duplicate facts.
    Ambiguous,
    /// Source data was corrupt or invalid.
    Corrupt,
    /// Source registration did not match.
    RegistrationMismatch,
    /// Source request did not match.
    QueryMismatch,
    /// Bounded transport or operational failure.
    Transport,
    /// Internal source invariant failure.
    Internal,
}

impl<'de> Deserialize<'de> for CurrentAuthoritySourceFailureKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "unavailable" => Ok(Self::Unavailable),
            "unsupported" => Ok(Self::Unsupported),
            "incomplete" => Ok(Self::Incomplete),
            "stale" => Ok(Self::Stale),
            "future_dated" => Ok(Self::FutureDated),
            "concurrent_change" => Ok(Self::ConcurrentChange),
            "ambiguous" => Ok(Self::Ambiguous),
            "corrupt" => Ok(Self::Corrupt),
            "registration_mismatch" => Ok(Self::RegistrationMismatch),
            "query_mismatch" => Ok(Self::QueryMismatch),
            "transport" => Ok(Self::Transport),
            "internal" => Ok(Self::Internal),
            _ => Err(serde::de::Error::custom(
                "current authority source failure kind is invalid",
            )),
        }
    }
}

/// Stable future retry posture for a source failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentAuthoritySourceFailurePosture {
    /// Failure is deterministic for this exact request.
    Terminal,
    /// A bounded retry may use the unchanged request.
    RetryableUnchangedRequest,
    /// Retry requires source or configuration change.
    RetryableAfterSourceChange,
}

impl<'de> Deserialize<'de> for CurrentAuthoritySourceFailurePosture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "terminal" => Ok(Self::Terminal),
            "retryable_unchanged_request" => Ok(Self::RetryableUnchangedRequest),
            "retryable_after_source_change" => Ok(Self::RetryableAfterSourceChange),
            _ => Err(serde::de::Error::custom(
                "current authority source failure posture is invalid",
            )),
        }
    }
}

/// Payload-free source failure commitment.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrentAuthoritySourceFailure {
    registration_commitment: SpecContentHash,
    request_commitment: SpecContentHash,
    kind: CurrentAuthoritySourceFailureKind,
    posture: CurrentAuthoritySourceFailurePosture,
}

impl CurrentAuthoritySourceFailure {
    /// Creates a bounded source failure without raw error payloads.
    #[must_use]
    pub const fn new(
        registration_commitment: SpecContentHash,
        request_commitment: SpecContentHash,
        kind: CurrentAuthoritySourceFailureKind,
        posture: CurrentAuthoritySourceFailurePosture,
    ) -> Self {
        Self {
            registration_commitment,
            request_commitment,
            kind,
            posture,
        }
    }

    /// Returns the stable failure kind.
    #[must_use]
    pub const fn kind(&self) -> CurrentAuthoritySourceFailureKind {
        self.kind
    }

    /// Returns the future retry posture.
    #[must_use]
    pub const fn posture(&self) -> CurrentAuthoritySourceFailurePosture {
        self.posture
    }
}

impl fmt::Debug for CurrentAuthoritySourceFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CurrentAuthoritySourceFailure")
            .field("registration_commitment", &"[REDACTED]")
            .field("request_commitment", &"[REDACTED]")
            .field("kind", &self.kind)
            .field("posture", &self.posture)
            .finish()
    }
}

fn validate_source_identifier(
    label: &'static str,
    value: &str,
    maximum_bytes: usize,
) -> Result<(), WorkflowOsError> {
    if value.is_empty() || value.len() > maximum_bytes {
        return Err(source_error(
            "identifier.length_invalid",
            "current authority source identifier length is invalid",
        ));
    }
    if contains_secret_like(value) {
        return Err(source_error(
            "identifier.secret_like",
            "current authority source identifier must not contain secret-like text",
        ));
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
    {
        return Err(source_error(
            "identifier.character_invalid",
            "current authority source identifier contains an invalid character",
        ));
    }
    let _ = label;
    Ok(())
}

fn contains_secret_like(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();
    [
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
        "sk-",
    ]
    .iter()
    .any(|marker| normalized.contains(marker))
}

fn canonicalize_families(
    families: &mut [CurrentAuthorityFactFamily],
) -> Result<(), WorkflowOsError> {
    if families.is_empty() {
        return Err(source_error(
            "fact_family.empty",
            "current authority source fact families cannot be empty",
        ));
    }
    canonicalize_families_allow_empty(families)
}

fn canonicalize_families_allow_empty(
    families: &mut [CurrentAuthorityFactFamily],
) -> Result<(), WorkflowOsError> {
    families.sort_unstable();
    if families.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(source_error(
            "fact_family.duplicate",
            "current authority source fact families contain a duplicate",
        ));
    }
    Ok(())
}

fn validate_canonical_families(
    families: &[CurrentAuthorityFactFamily],
) -> Result<(), WorkflowOsError> {
    if families.is_empty() {
        return Err(source_error(
            "fact_family.empty",
            "current authority source fact families cannot be empty",
        ));
    }
    validate_canonical_families_allow_empty(families)
}

fn validate_canonical_families_allow_empty(
    families: &[CurrentAuthorityFactFamily],
) -> Result<(), WorkflowOsError> {
    if families.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(source_error(
            "fact_family.order_invalid",
            "current authority source fact families must be unique and ordered",
        ));
    }
    Ok(())
}

fn add_seconds(timestamp: Timestamp, seconds: u32) -> Result<Timestamp, WorkflowOsError> {
    timestamp
        .as_offset_date_time()
        .checked_add(Duration::seconds(i64::from(seconds)))
        .map(Timestamp::from_offset_date_time)
        .ok_or_else(|| {
            source_error(
                "time.overflow",
                "current authority source timestamp exceeds supported range",
            )
        })
}

fn pending_hash() -> SpecContentHash {
    SpecContentHash::from_text("pending")
}

fn hash_serializable(
    domain: &str,
    value: &impl Serialize,
) -> Result<SpecContentHash, WorkflowOsError> {
    let bytes = serde_json::to_vec(value).map_err(|_| {
        source_error(
            "hash.serialization_failed",
            "current authority source hashing failed",
        )
    })?;
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "domain", domain.as_bytes());
    hash_field(&mut hasher, "value", &bytes);
    Ok(SpecContentHash::from_bytes(hasher.finalize()))
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &[u8]) {
    hasher.update(u64::try_from(label.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(value);
}

fn source_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("current_authority.source.{suffix}"), message)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use super::*;

    #[test]
    fn source_identifiers_reject_secret_like_values_without_leaking() {
        let secret = "token-super-sensitive";
        let error = CurrentAuthoritySourceId::new(secret).expect_err("must fail closed");

        assert_eq!(
            error.code(),
            "current_authority.source.identifier.secret_like"
        );
        assert!(!format!("{error:?}").contains(secret));
        assert!(!error.to_string().contains(secret));
    }

    #[test]
    fn generation_must_be_non_zero() {
        let error = CurrentAuthoritySourceGeneration::new(0).expect_err("zero is invalid");
        assert_eq!(error.code(), "current_authority.source.generation.zero");
    }

    #[test]
    fn read_window_rejects_out_of_order_times() {
        let earlier = Timestamp::parse_rfc3339("2026-01-01T00:00:00Z").expect("valid");
        let later = Timestamp::parse_rfc3339("2026-01-01T00:01:00Z").expect("valid");
        let error =
            CurrentAuthoritySourceReadWindow::new(later, earlier, later).expect_err("invalid");
        assert_eq!(
            error.code(),
            "current_authority.source.read_window.order_invalid"
        );
    }

    #[test]
    fn enums_fail_closed_during_deserialization() {
        let error = serde_json::from_str::<CurrentAuthoritySourceCompleteness>("\"trusted\"")
            .expect_err("unknown posture must fail");
        assert!(!error.to_string().contains("trusted"));
    }

    #[test]
    fn failure_debug_redacts_commitments() {
        let secret = "secret-source-commitment";
        let failure = CurrentAuthoritySourceFailure::new(
            SpecContentHash::from_text(secret),
            SpecContentHash::from_text("request"),
            CurrentAuthoritySourceFailureKind::Transport,
            CurrentAuthoritySourceFailurePosture::RetryableUnchangedRequest,
        );
        let debug = format!("{failure:?}");

        assert!(!debug.contains(secret));
        assert!(debug.contains("[REDACTED]"));
    }
}
