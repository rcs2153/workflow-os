use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};
use time::Duration;

use crate::{
    assess_immutable_bundle_governance, GovernanceStrictnessProfile,
    ImmutableBundleGovernanceAssessmentRequest, ImmutableBundleGovernanceAssessmentSet,
    ImmutableRunBundleBinding, SpecContentHash, StepGovernanceRuntimeFacts, StepId,
    StoredImmutableRunBundle, Timestamp, WorkflowOsError,
};

const SOURCE_ID_MAX_BYTES: usize = 128;
const SOURCE_VERSION_MAX_BYTES: usize = 64;
const SNAPSHOT_ID_MAX_BYTES: usize = 192;
const MAX_OBSERVATION_AGE_SECONDS: u32 = 31_536_000;
const MAX_RUNTIME_FACTS: usize = 1_024;

macro_rules! bounded_identifier {
    ($(#[$meta:meta])* $name:ident, $maximum:expr, $label:literal) => {
        $(#[$meta])*
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Creates one bounded, non-secret identifier.
            ///
            /// # Errors
            ///
            /// Returns a stable non-leaking error when the identifier is invalid.
            pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
                let value = value.into();
                validate_identifier($label, &value, $maximum)?;
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
    };
}

bounded_identifier!(
    /// Stable identity for one explicitly registered runtime-fact source.
    GovernanceRuntimeFactSourceId,
    SOURCE_ID_MAX_BYTES,
    "runtime fact source id"
);

bounded_identifier!(
    /// Version of the source contract implemented by a runtime-fact source.
    GovernanceRuntimeFactSourceContractVersion,
    SOURCE_VERSION_MAX_BYTES,
    "runtime fact source contract version"
);

bounded_identifier!(
    /// Opaque identity for one source observation.
    GovernanceRuntimeFactSnapshotId,
    SNAPSHOT_ID_MAX_BYTES,
    "runtime fact snapshot id"
);

/// Explicit definition for one registered runtime-fact source.
pub struct GovernanceRuntimeFactSourceRegistrationDefinition {
    /// Stable source identity expected from the injected implementation.
    pub source_id: GovernanceRuntimeFactSourceId,
    /// Exact source contract version.
    pub contract_version: GovernanceRuntimeFactSourceContractVersion,
    /// Credential-free normalized source configuration commitment.
    pub configuration_commitment: SpecContentHash,
    /// Core-owned maximum accepted observation age.
    pub core_maximum_observation_age_seconds: u32,
}

/// Payload-free registration commitment for one injected runtime-fact source.
///
/// Registration is an explicit trust choice by the embedding caller. It is not
/// source authentication, a signature, or reusable execution authority.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GovernanceRuntimeFactSourceRegistration {
    source_id: GovernanceRuntimeFactSourceId,
    contract_version: GovernanceRuntimeFactSourceContractVersion,
    configuration_commitment: SpecContentHash,
    core_maximum_observation_age_seconds: u32,
    registration_commitment: SpecContentHash,
}

impl GovernanceRuntimeFactSourceRegistration {
    /// Creates one validated source registration.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error for invalid freshness posture.
    pub fn new(
        definition: GovernanceRuntimeFactSourceRegistrationDefinition,
    ) -> Result<Self, WorkflowOsError> {
        validate_age_bound(definition.core_maximum_observation_age_seconds)?;
        let registration_commitment = hash_registration(&definition);
        Ok(Self {
            source_id: definition.source_id,
            contract_version: definition.contract_version,
            configuration_commitment: definition.configuration_commitment,
            core_maximum_observation_age_seconds: definition.core_maximum_observation_age_seconds,
            registration_commitment,
        })
    }

    /// Returns the registered source identity.
    #[must_use]
    pub const fn source_id(&self) -> &GovernanceRuntimeFactSourceId {
        &self.source_id
    }

    /// Returns the registered source contract version.
    #[must_use]
    pub const fn contract_version(&self) -> &GovernanceRuntimeFactSourceContractVersion {
        &self.contract_version
    }

    /// Returns the Core-owned maximum observation age.
    #[must_use]
    pub const fn core_maximum_observation_age_seconds(&self) -> u32 {
        self.core_maximum_observation_age_seconds
    }

    /// Returns the payload-free registration commitment.
    #[must_use]
    pub const fn registration_commitment(&self) -> &SpecContentHash {
        &self.registration_commitment
    }
}

impl fmt::Debug for GovernanceRuntimeFactSourceRegistration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernanceRuntimeFactSourceRegistration")
            .field("source_id", &"[REDACTED]")
            .field("contract_version", &"[REDACTED]")
            .field("configuration_commitment", &"[REDACTED]")
            .field(
                "core_maximum_observation_age_seconds",
                &self.core_maximum_observation_age_seconds,
            )
            .field("registration_commitment", &"[REDACTED]")
            .finish()
    }
}

/// Read-only request supplied to one injected runtime-fact source.
pub struct GovernanceRuntimeFactSourceRequest<'a> {
    bundle_binding: ImmutableRunBundleBinding,
    evaluated_at: Timestamp,
    bundle: &'a StoredImmutableRunBundle,
}

impl GovernanceRuntimeFactSourceRequest<'_> {
    /// Returns the exact immutable bundle binding being assessed.
    #[must_use]
    pub const fn bundle_binding(&self) -> &ImmutableRunBundleBinding {
        &self.bundle_binding
    }

    /// Returns the Core-selected evaluation time.
    #[must_use]
    pub const fn evaluated_at(&self) -> Timestamp {
        self.evaluated_at
    }

    /// Returns the validated stored immutable run bundle.
    #[must_use]
    pub const fn bundle(&self) -> &StoredImmutableRunBundle {
        self.bundle
    }
}

impl fmt::Debug for GovernanceRuntimeFactSourceRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernanceRuntimeFactSourceRequest")
            .field("bundle_binding", &"[REDACTED]")
            .field("evaluated_at", &"[REDACTED]")
            .field("bundle", &"[REDACTED]")
            .finish()
    }
}

/// Definition returned by an injected runtime-fact source.
pub struct GovernanceRuntimeFactObservationDefinition {
    /// Source identity claimed for this observation.
    pub source_id: GovernanceRuntimeFactSourceId,
    /// Source contract version claimed for this observation.
    pub contract_version: GovernanceRuntimeFactSourceContractVersion,
    /// Opaque identity for this observation.
    pub snapshot_id: GovernanceRuntimeFactSnapshotId,
    /// Exact immutable run bundle observed by the source.
    pub bundle_binding: ImmutableRunBundleBinding,
    /// Time at which the source observed the facts.
    pub observed_at: Timestamp,
    /// Source-owned maximum age for this observation.
    pub source_maximum_observation_age_seconds: u32,
    /// Exactly one fact record for every immutable workflow step.
    pub runtime_facts: Vec<StepGovernanceRuntimeFacts>,
}

/// Untrusted source output validated only inside the same-call assessment helper.
pub struct GovernanceRuntimeFactObservation {
    definition: GovernanceRuntimeFactObservationDefinition,
}

impl GovernanceRuntimeFactObservation {
    /// Creates one structurally bounded source observation.
    ///
    /// This constructor does not establish source trust, freshness, exact step
    /// coverage, or authority. Those checks belong to the same-call consumer.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error for an invalid age or fact count.
    pub fn new(
        definition: GovernanceRuntimeFactObservationDefinition,
    ) -> Result<Self, WorkflowOsError> {
        validate_age_bound(definition.source_maximum_observation_age_seconds)?;
        if definition.runtime_facts.is_empty() || definition.runtime_facts.len() > MAX_RUNTIME_FACTS
        {
            return Err(runtime_fact_error(
                "observation.fact_count_invalid",
                "runtime fact observation count is invalid",
            ));
        }
        Ok(Self { definition })
    }
}

impl fmt::Debug for GovernanceRuntimeFactObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernanceRuntimeFactObservation")
            .field("source_id", &"[REDACTED]")
            .field("contract_version", &"[REDACTED]")
            .field("snapshot_id", &"[REDACTED]")
            .field("bundle_binding", &"[REDACTED]")
            .field("observed_at", &"[REDACTED]")
            .field(
                "source_maximum_observation_age_seconds",
                &self.definition.source_maximum_observation_age_seconds,
            )
            .field("runtime_fact_count", &self.definition.runtime_facts.len())
            .finish()
    }
}

/// Injected read-only boundary for obtaining current runtime facts.
pub trait GovernanceRuntimeFactSource {
    /// Observes exact facts for the requested immutable run bundle.
    ///
    /// Source failures are replaced by a stable Core-owned error before they
    /// cross the public assessment boundary.
    ///
    /// # Errors
    ///
    /// Returns a source-local error when the observation cannot be produced.
    /// The same-call Core consumer replaces that error before exposing it.
    fn observe(
        &self,
        request: &GovernanceRuntimeFactSourceRequest<'_>,
    ) -> Result<GovernanceRuntimeFactObservation, WorkflowOsError>;
}

/// Same-call assessment request over one registered source.
pub struct GovernanceRuntimeFactAssessmentRequest<'a> {
    /// Validated stored immutable run bundle.
    pub bundle: &'a StoredImmutableRunBundle,
    /// Active governance profile.
    pub profile: GovernanceStrictnessProfile,
    /// Explicit trusted registration chosen by the embedding caller.
    pub registration: &'a GovernanceRuntimeFactSourceRegistration,
    /// Injected source implementation matching the registration.
    pub source: &'a dyn GovernanceRuntimeFactSource,
    /// Core-selected evaluation time used for freshness validation.
    pub evaluated_at: Timestamp,
}

impl fmt::Debug for GovernanceRuntimeFactAssessmentRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernanceRuntimeFactAssessmentRequest")
            .field("bundle", &"[REDACTED]")
            .field("profile", &self.profile)
            .field("registration", &self.registration)
            .field("source", &"[INJECTED]")
            .field("evaluated_at", &"[REDACTED]")
            .finish()
    }
}

/// Accepted payload-free snapshot of one fresh exact runtime-fact observation.
///
/// The snapshot intentionally has no deserialization path. It is evidence of
/// validation in this process, not reusable or self-authenticating authority.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GovernanceRuntimeFactSnapshot {
    source_id: GovernanceRuntimeFactSourceId,
    contract_version: GovernanceRuntimeFactSourceContractVersion,
    snapshot_id: GovernanceRuntimeFactSnapshotId,
    registration_commitment: SpecContentHash,
    bundle_binding: ImmutableRunBundleBinding,
    observed_at: Timestamp,
    evaluated_at: Timestamp,
    effective_maximum_observation_age_seconds: u32,
    runtime_fact_count: u32,
    runtime_fact_commitment: SpecContentHash,
    assessment_aggregate_fingerprint: SpecContentHash,
    snapshot_commitment: SpecContentHash,
}

impl GovernanceRuntimeFactSnapshot {
    /// Returns the exact immutable bundle binding validated for this snapshot.
    #[must_use]
    pub const fn bundle_binding(&self) -> &ImmutableRunBundleBinding {
        &self.bundle_binding
    }

    /// Returns the number of exact runtime facts committed by the snapshot.
    #[must_use]
    pub const fn runtime_fact_count(&self) -> u32 {
        self.runtime_fact_count
    }

    /// Returns the effective stricter-of-source-and-Core freshness bound.
    #[must_use]
    pub const fn effective_maximum_observation_age_seconds(&self) -> u32 {
        self.effective_maximum_observation_age_seconds
    }

    /// Returns the payload-free exact fact-set commitment.
    #[must_use]
    pub const fn runtime_fact_commitment(&self) -> &SpecContentHash {
        &self.runtime_fact_commitment
    }

    /// Returns the complete snapshot commitment.
    #[must_use]
    pub const fn snapshot_commitment(&self) -> &SpecContentHash {
        &self.snapshot_commitment
    }

    /// Creates the validated durable payload-free commitment for this accepted snapshot.
    ///
    /// The returned binding is provenance metadata only. It does not make the
    /// observation fresh or authoritative for a later operation.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when commitment construction fails.
    pub fn commitment_binding(
        &self,
    ) -> Result<GovernanceRuntimeFactSnapshotBinding, WorkflowOsError> {
        GovernanceRuntimeFactSnapshotBinding::from_snapshot(self)
    }
}

impl fmt::Debug for GovernanceRuntimeFactSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernanceRuntimeFactSnapshot")
            .field("source_id", &"[REDACTED]")
            .field("contract_version", &"[REDACTED]")
            .field("snapshot_id", &"[REDACTED]")
            .field("registration_commitment", &"[REDACTED]")
            .field("bundle_binding", &"[REDACTED]")
            .field("observed_at", &"[REDACTED]")
            .field("evaluated_at", &"[REDACTED]")
            .field(
                "effective_maximum_observation_age_seconds",
                &self.effective_maximum_observation_age_seconds,
            )
            .field("runtime_fact_count", &self.runtime_fact_count)
            .field("runtime_fact_commitment", &"[REDACTED]")
            .field("assessment_aggregate_fingerprint", &"[REDACTED]")
            .field("snapshot_commitment", &"[REDACTED]")
            .finish()
    }
}

/// Version of the durable runtime-fact snapshot commitment binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceRuntimeFactSnapshotBindingVersion {
    /// Initial payload-free snapshot commitment binding.
    V1,
}

impl<'de> Deserialize<'de> for GovernanceRuntimeFactSnapshotBindingVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "v1" => Ok(Self::V1),
            _ => Err(serde::de::Error::custom(
                "runtime fact snapshot binding version is invalid",
            )),
        }
    }
}

/// Durable payload-free commitment to the source observation that established a run assessment.
///
/// The binding preserves integrity and provenance metadata only. It contains no
/// runtime facts, source payload, reusable authority, or freshness claim for a
/// later operation. Every retry or future approval resume must resolve current
/// facts again.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GovernanceRuntimeFactSnapshotBinding {
    binding_version: GovernanceRuntimeFactSnapshotBindingVersion,
    source_registration_commitment: SpecContentHash,
    immutable_run_bundle: ImmutableRunBundleBinding,
    initial_snapshot_commitment: SpecContentHash,
    runtime_fact_commitment: SpecContentHash,
    runtime_fact_count: u32,
    observed_at: Timestamp,
    evaluated_at: Timestamp,
    effective_maximum_observation_age_seconds: u32,
    assessment_aggregate_fingerprint: SpecContentHash,
    binding_commitment: SpecContentHash,
}

impl GovernanceRuntimeFactSnapshotBinding {
    fn from_snapshot(snapshot: &GovernanceRuntimeFactSnapshot) -> Result<Self, WorkflowOsError> {
        let mut binding = Self {
            binding_version: GovernanceRuntimeFactSnapshotBindingVersion::V1,
            source_registration_commitment: snapshot.registration_commitment.clone(),
            immutable_run_bundle: snapshot.bundle_binding.clone(),
            initial_snapshot_commitment: snapshot.snapshot_commitment.clone(),
            runtime_fact_commitment: snapshot.runtime_fact_commitment.clone(),
            runtime_fact_count: snapshot.runtime_fact_count,
            observed_at: snapshot.observed_at,
            evaluated_at: snapshot.evaluated_at,
            effective_maximum_observation_age_seconds: snapshot
                .effective_maximum_observation_age_seconds,
            assessment_aggregate_fingerprint: snapshot.assessment_aggregate_fingerprint.clone(),
            binding_commitment: SpecContentHash::from_text("pending commitment"),
        };
        binding.binding_commitment = binding.calculate_commitment()?;
        binding.validate()?;
        Ok(binding)
    }

    /// Returns the binding model version.
    #[must_use]
    pub const fn binding_version(&self) -> GovernanceRuntimeFactSnapshotBindingVersion {
        self.binding_version
    }

    /// Returns the exact immutable bundle committed by the source observation.
    #[must_use]
    pub const fn immutable_run_bundle(&self) -> &ImmutableRunBundleBinding {
        &self.immutable_run_bundle
    }

    /// Returns the trusted source-registration commitment used for the observation.
    #[must_use]
    pub const fn source_registration_commitment(&self) -> &SpecContentHash {
        &self.source_registration_commitment
    }

    /// Returns the accepted initial source-snapshot commitment.
    #[must_use]
    pub const fn initial_snapshot_commitment(&self) -> &SpecContentHash {
        &self.initial_snapshot_commitment
    }

    /// Returns the exact initial runtime-fact-set commitment.
    #[must_use]
    pub const fn runtime_fact_commitment(&self) -> &SpecContentHash {
        &self.runtime_fact_commitment
    }

    /// Returns the number of committed runtime facts.
    #[must_use]
    pub const fn runtime_fact_count(&self) -> u32 {
        self.runtime_fact_count
    }

    /// Returns the evaluation time committed by the accepted source snapshot.
    #[must_use]
    pub const fn evaluated_at(&self) -> Timestamp {
        self.evaluated_at
    }

    /// Returns the assessment aggregate established from the committed facts.
    #[must_use]
    pub const fn assessment_aggregate_fingerprint(&self) -> &SpecContentHash {
        &self.assessment_aggregate_fingerprint
    }

    /// Returns the complete durable binding commitment.
    #[must_use]
    pub const fn binding_commitment(&self) -> &SpecContentHash {
        &self.binding_commitment
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.runtime_fact_count == 0 || self.runtime_fact_count as usize > MAX_RUNTIME_FACTS {
            return Err(runtime_fact_error(
                "snapshot_binding.fact_count_invalid",
                "runtime fact snapshot binding is invalid",
            ));
        }
        validate_age_bound(self.effective_maximum_observation_age_seconds)?;
        validate_freshness(
            self.observed_at,
            self.evaluated_at,
            self.effective_maximum_observation_age_seconds,
        )?;
        let expected = self.calculate_commitment()?;
        if expected != self.binding_commitment {
            return Err(runtime_fact_error(
                "snapshot_binding.commitment_mismatch",
                "runtime fact snapshot binding is invalid",
            ));
        }
        Ok(())
    }

    fn calculate_commitment(&self) -> Result<SpecContentHash, WorkflowOsError> {
        hash_serializable(
            "workflow-os/governance-runtime-fact-snapshot-binding/v1",
            &(
                self.binding_version,
                &self.source_registration_commitment,
                &self.immutable_run_bundle,
                &self.initial_snapshot_commitment,
                &self.runtime_fact_commitment,
                self.runtime_fact_count,
                self.observed_at,
                self.evaluated_at,
                self.effective_maximum_observation_age_seconds,
                &self.assessment_aggregate_fingerprint,
            ),
        )
    }
}

impl fmt::Debug for GovernanceRuntimeFactSnapshotBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernanceRuntimeFactSnapshotBinding")
            .field("binding_version", &self.binding_version)
            .field("source_registration_commitment", &"[REDACTED]")
            .field("immutable_run_bundle", &"[REDACTED]")
            .field("initial_snapshot_commitment", &"[REDACTED]")
            .field("runtime_fact_commitment", &"[REDACTED]")
            .field("runtime_fact_count", &self.runtime_fact_count)
            .field("observed_at", &"[REDACTED]")
            .field("evaluated_at", &"[REDACTED]")
            .field(
                "effective_maximum_observation_age_seconds",
                &self.effective_maximum_observation_age_seconds,
            )
            .field("assessment_aggregate_fingerprint", &"[REDACTED]")
            .field("binding_commitment", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for GovernanceRuntimeFactSnapshotBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            binding_version: GovernanceRuntimeFactSnapshotBindingVersion,
            source_registration_commitment: SpecContentHash,
            immutable_run_bundle: ImmutableRunBundleBinding,
            initial_snapshot_commitment: SpecContentHash,
            runtime_fact_commitment: SpecContentHash,
            runtime_fact_count: u32,
            observed_at: Timestamp,
            evaluated_at: Timestamp,
            effective_maximum_observation_age_seconds: u32,
            assessment_aggregate_fingerprint: SpecContentHash,
            binding_commitment: SpecContentHash,
        }

        let wire = Wire::deserialize(deserializer)?;
        let binding = Self {
            binding_version: wire.binding_version,
            source_registration_commitment: wire.source_registration_commitment,
            immutable_run_bundle: wire.immutable_run_bundle,
            initial_snapshot_commitment: wire.initial_snapshot_commitment,
            runtime_fact_commitment: wire.runtime_fact_commitment,
            runtime_fact_count: wire.runtime_fact_count,
            observed_at: wire.observed_at,
            evaluated_at: wire.evaluated_at,
            effective_maximum_observation_age_seconds: wire
                .effective_maximum_observation_age_seconds,
            assessment_aggregate_fingerprint: wire.assessment_aggregate_fingerprint,
            binding_commitment: wire.binding_commitment,
        };
        binding.validate().map_err(serde::de::Error::custom)?;
        Ok(binding)
    }
}

/// Same-call result that keeps the accepted source snapshot and assessment together.
pub struct GovernanceRuntimeFactAssessment {
    snapshot: GovernanceRuntimeFactSnapshot,
    assessment_set: ImmutableBundleGovernanceAssessmentSet,
}

impl GovernanceRuntimeFactAssessment {
    /// Returns the validated payload-free source snapshot.
    #[must_use]
    pub const fn snapshot(&self) -> &GovernanceRuntimeFactSnapshot {
        &self.snapshot
    }

    /// Returns the assessment derived in the same call from exact current facts.
    #[must_use]
    pub const fn assessment_set(&self) -> &ImmutableBundleGovernanceAssessmentSet {
        &self.assessment_set
    }

    /// Consumes the inseparable result into snapshot and assessment values.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        GovernanceRuntimeFactSnapshot,
        ImmutableBundleGovernanceAssessmentSet,
    ) {
        (self.snapshot, self.assessment_set)
    }
}

impl fmt::Debug for GovernanceRuntimeFactAssessment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernanceRuntimeFactAssessment")
            .field("snapshot", &self.snapshot)
            .field("assessment_set", &self.assessment_set)
            .finish()
    }
}

/// Resolves current facts from one registered source and assesses them in the same call.
///
/// The helper validates source identity, immutable-bundle identity, exact fact
/// coverage, and stricter-of-source-and-Core freshness before returning an
/// accepted payload-free snapshot. It does not persist or enforce the result.
///
/// # Errors
///
/// Returns stable non-leaking errors for source failure, identity mismatch,
/// stale or future-dated observations, invalid coverage, or assessment failure.
pub fn assess_immutable_bundle_governance_from_current_facts(
    request: &GovernanceRuntimeFactAssessmentRequest<'_>,
) -> Result<GovernanceRuntimeFactAssessment, WorkflowOsError> {
    let bundle_binding = request.bundle.manifest().run_binding();
    let source_request = GovernanceRuntimeFactSourceRequest {
        bundle_binding: bundle_binding.clone(),
        evaluated_at: request.evaluated_at,
        bundle: request.bundle,
    };
    let observation = request.source.observe(&source_request).map_err(|_| {
        runtime_fact_error("source_failed", "registered runtime fact source failed")
    })?;
    let definition = observation.definition;
    if &definition.source_id != request.registration.source_id()
        || &definition.contract_version != request.registration.contract_version()
    {
        return Err(runtime_fact_error(
            "source_identity_mismatch",
            "runtime fact source identity does not match registration",
        ));
    }
    if definition.bundle_binding != bundle_binding {
        return Err(runtime_fact_error(
            "bundle_binding_mismatch",
            "runtime fact observation does not match immutable bundle",
        ));
    }
    let effective_maximum_observation_age_seconds = definition
        .source_maximum_observation_age_seconds
        .min(request.registration.core_maximum_observation_age_seconds());
    validate_freshness(
        definition.observed_at,
        request.evaluated_at,
        effective_maximum_observation_age_seconds,
    )?;

    let assessment_set =
        assess_immutable_bundle_governance(&ImmutableBundleGovernanceAssessmentRequest {
            bundle: request.bundle,
            profile: request.profile,
            runtime_facts: &definition.runtime_facts,
        })?;
    let canonical_facts = canonical_facts(&definition.runtime_facts, &assessment_set)?;
    let runtime_fact_commitment = hash_serializable(
        "workflow-os/governance-runtime-fact-set/v1",
        &canonical_facts,
    )?;
    let runtime_fact_count = u32::try_from(canonical_facts.len()).map_err(|_| {
        runtime_fact_error(
            "observation.fact_count_invalid",
            "runtime fact observation count is invalid",
        )
    })?;
    let snapshot_commitment = hash_serializable(
        "workflow-os/governance-runtime-fact-snapshot/v1",
        &(
            &definition.source_id,
            &definition.contract_version,
            &definition.snapshot_id,
            request.registration.registration_commitment(),
            &bundle_binding,
            definition.observed_at,
            request.evaluated_at,
            effective_maximum_observation_age_seconds,
            runtime_fact_count,
            &runtime_fact_commitment,
            assessment_set.aggregate_fingerprint(),
        ),
    )?;
    let snapshot = GovernanceRuntimeFactSnapshot {
        source_id: definition.source_id,
        contract_version: definition.contract_version,
        snapshot_id: definition.snapshot_id,
        registration_commitment: request.registration.registration_commitment().clone(),
        bundle_binding,
        observed_at: definition.observed_at,
        evaluated_at: request.evaluated_at,
        effective_maximum_observation_age_seconds,
        runtime_fact_count,
        runtime_fact_commitment,
        assessment_aggregate_fingerprint: assessment_set.aggregate_fingerprint().clone(),
        snapshot_commitment,
    };
    Ok(GovernanceRuntimeFactAssessment {
        snapshot,
        assessment_set,
    })
}

fn canonical_facts<'a>(
    supplied: &'a [StepGovernanceRuntimeFacts],
    assessment_set: &ImmutableBundleGovernanceAssessmentSet,
) -> Result<Vec<&'a StepGovernanceRuntimeFacts>, WorkflowOsError> {
    let facts = supplied
        .iter()
        .map(|fact| (fact.step_id().clone(), fact))
        .collect::<BTreeMap<StepId, _>>();
    assessment_set
        .assessments()
        .iter()
        .map(|assessment| {
            facts.get(assessment.step_id()).copied().ok_or_else(|| {
                runtime_fact_error(
                    "observation.coverage_invalid",
                    "runtime fact observation coverage is invalid",
                )
            })
        })
        .collect()
}

fn validate_freshness(
    observed_at: Timestamp,
    evaluated_at: Timestamp,
    maximum_age_seconds: u32,
) -> Result<(), WorkflowOsError> {
    if observed_at > evaluated_at {
        return Err(runtime_fact_error(
            "observation.future_dated",
            "runtime fact observation is future dated",
        ));
    }
    let expires_at = observed_at
        .as_offset_date_time()
        .checked_add(Duration::seconds(i64::from(maximum_age_seconds)))
        .map(Timestamp::from_offset_date_time)
        .ok_or_else(|| {
            runtime_fact_error(
                "observation.time_overflow",
                "runtime fact observation time exceeds supported range",
            )
        })?;
    if evaluated_at > expires_at {
        return Err(runtime_fact_error(
            "observation.stale",
            "runtime fact observation is stale",
        ));
    }
    Ok(())
}

fn validate_age_bound(value: u32) -> Result<(), WorkflowOsError> {
    if value == 0 || value > MAX_OBSERVATION_AGE_SECONDS {
        return Err(runtime_fact_error(
            "freshness_bound_invalid",
            "runtime fact source freshness bound is invalid",
        ));
    }
    Ok(())
}

fn validate_identifier(
    _label: &'static str,
    value: &str,
    maximum_bytes: usize,
) -> Result<(), WorkflowOsError> {
    if value.is_empty() || value.len() > maximum_bytes {
        return Err(runtime_fact_error(
            "identifier.length_invalid",
            "runtime fact source identifier length is invalid",
        ));
    }
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
        "sk-",
    ]
    .iter()
    .any(|marker| normalized.contains(marker))
    {
        return Err(runtime_fact_error(
            "identifier.secret_like",
            "runtime fact source identifier must not contain secret-like text",
        ));
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
    {
        return Err(runtime_fact_error(
            "identifier.character_invalid",
            "runtime fact source identifier contains an invalid character",
        ));
    }
    Ok(())
}

fn hash_registration(
    definition: &GovernanceRuntimeFactSourceRegistrationDefinition,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    for (label, value) in [
        (
            "domain",
            "workflow-os/governance-runtime-fact-source-registration/v1",
        ),
        ("source_id", definition.source_id.as_str()),
        ("contract_version", definition.contract_version.as_str()),
        (
            "configuration_commitment",
            definition.configuration_commitment.as_str(),
        ),
    ] {
        hash_field(&mut hasher, label, value.as_bytes());
    }
    hash_field(
        &mut hasher,
        "core_maximum_observation_age_seconds",
        &definition
            .core_maximum_observation_age_seconds
            .to_be_bytes(),
    );
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_serializable(
    domain: &'static str,
    value: &impl Serialize,
) -> Result<SpecContentHash, WorkflowOsError> {
    let bytes = serde_json::to_vec(value).map_err(|_| {
        runtime_fact_error(
            "commitment.serialization_failed",
            "runtime fact commitment serialization failed",
        )
    })?;
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "domain", domain.as_bytes());
    hash_field(&mut hasher, "value", &bytes);
    Ok(SpecContentHash::from_bytes(hasher.finalize()))
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &[u8]) {
    for part in [label.as_bytes(), value] {
        hasher.update(u64::try_from(part.len()).unwrap_or(u64::MAX).to_be_bytes());
        hasher.update(part);
    }
}

fn runtime_fact_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("governance.proportional.runtime_fact_source.{suffix}"),
        message,
    )
}
