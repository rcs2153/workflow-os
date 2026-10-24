//! Non-authorizing model for provider-neutral trusted-host input observation.
//!
//! These values validate payload-free observation claims. They do not inspect
//! a host, run a process, select or trust a provider, admit execution, persist
//! state, append events, or grant authority.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    HostedExecutionReference, HostedExecutionReferenceKind, SpecContentHash,
    TrustedHostBuildInputHostObligationKind, TrustedHostObservationPosture, TrustedHostRootAccess,
    TrustedHostRootObservation, TrustedHostRootObservationMethod, TrustedHostRootRole,
    WorkflowOsError,
};

const HOST_OBLIGATION_COUNT: usize = 9;
const FACT_ITEM_MAX_COUNT: u32 = 16_384;
const PRODUCER_ID_MAX_BYTES: usize = 128;
const PRODUCER_VERSION_MAX_BYTES: usize = 64;
const REQUEST_DOMAIN: &str = "workflow-os/trusted-host-input-observation-request/v2";
const RESULT_DOMAIN: &str = "workflow-os/trusted-host-input-observation-result/v2";

macro_rules! impl_closed_string_deserialize {
    ($type:ty, $expecting:literal, $error:literal, { $($tag:literal => $variant:path),+ $(,)? }) => {
        impl<'de> Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Visitor;

                impl serde::de::Visitor<'_> for Visitor {
                    type Value = $type;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_str($expecting)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            $($tag => Ok($variant),)+
                            _ => Err(E::custom($error)),
                        }
                    }
                }

                deserializer.deserialize_str(Visitor)
            }
        }
    };
}

/// Closed version of the observer contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostInputObservationContractVersion {
    /// Cargo/Rust host-input observation contract, version two.
    CargoHostInputsV2,
}

impl_closed_string_deserialize!(
    TrustedHostInputObservationContractVersion,
    "a supported trusted-host input observation contract version",
    "unsupported trusted-host input observation contract version",
    { "cargo_host_inputs_v2" => TrustedHostInputObservationContractVersion::CargoHostInputsV2 }
);

macro_rules! producer_identifier {
    ($(#[$meta:meta])* $name:ident, $max_bytes:expr, $suffix:literal, $label:literal) => {
        $(#[$meta])*
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            /// Creates a bounded payload-free producer identifier.
            ///
            /// # Errors
            ///
            /// Rejects empty, oversized, malformed, or secret-like text.
            pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
                let value = value.into();
                let valid = !value.is_empty()
                    && value.len() <= $max_bytes
                    && value.bytes().all(|byte| {
                        byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/')
                    })
                    && !is_secret_like(&value);
                if !valid {
                    return Err(observer_error($suffix, concat!($label, " is invalid")));
                }
                Ok(Self(value))
            }

            /// Returns the validated identifier text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::new(value).map_err(|_| serde::de::Error::custom(concat!($label, " is invalid")))
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.debug_tuple(stringify!($name)).field(&"[REDACTED]").finish()
            }
        }
    };
}

producer_identifier!(
    /// Stable observer producer identity without authority semantics.
    TrustedHostInputObservationProducerId,
    PRODUCER_ID_MAX_BYTES,
    "producer.id.invalid",
    "trusted-host input observation producer id"
);
producer_identifier!(
    /// Stable observer producer version without authority semantics.
    TrustedHostInputObservationProducerVersion,
    PRODUCER_VERSION_MAX_BYTES,
    "producer.version.invalid",
    "trusted-host input observation producer version"
);

/// Closed producer category. This identifies a claimant; it does not trust it.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostInputObservationProducerKind {
    /// Explicit fixture producer for process-free tests.
    Fixture,
    /// Future independently reviewed host observer.
    HostObserver,
    /// Future execution-provider observation channel.
    ExecutionProvider,
}

impl_closed_string_deserialize!(
    TrustedHostInputObservationProducerKind,
    "a supported trusted-host input observation producer kind",
    "unsupported trusted-host input observation producer kind",
    {
        "fixture" => TrustedHostInputObservationProducerKind::Fixture,
        "host_observer" => TrustedHostInputObservationProducerKind::HostObserver,
        "execution_provider" => TrustedHostInputObservationProducerKind::ExecutionProvider
    }
);

/// Claimed assurance of the producer channel.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostInputObservationProducerAssurance {
    /// Caller supplied facts; never sufficient for live admission.
    CallerSupplied,
    /// Fixture-only facts; never evidence about a live host.
    FixtureOnly,
    /// Facts from a future independently reviewed channel.
    IndependentlyReviewed,
}

impl_closed_string_deserialize!(
    TrustedHostInputObservationProducerAssurance,
    "a supported trusted-host input observation producer assurance",
    "unsupported trusted-host input observation producer assurance",
    {
        "caller_supplied" => TrustedHostInputObservationProducerAssurance::CallerSupplied,
        "fixture_only" => TrustedHostInputObservationProducerAssurance::FixtureOnly,
        "independently_reviewed" => TrustedHostInputObservationProducerAssurance::IndependentlyReviewed
    }
);

/// Payload-free producer identity and configuration binding.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "TrustedHostInputObservationProducerDefinition")]
pub struct TrustedHostInputObservationProducer {
    id: TrustedHostInputObservationProducerId,
    version: TrustedHostInputObservationProducerVersion,
    kind: TrustedHostInputObservationProducerKind,
    assurance: TrustedHostInputObservationProducerAssurance,
    implementation: SpecContentHash,
    configuration: SpecContentHash,
}

/// Authoritative producer fields.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrustedHostInputObservationProducerDefinition {
    /// Stable observer identity.
    pub id: TrustedHostInputObservationProducerId,
    /// Stable observer version.
    pub version: TrustedHostInputObservationProducerVersion,
    /// Closed producer category.
    pub kind: TrustedHostInputObservationProducerKind,
    /// Claimed producer assurance.
    pub assurance: TrustedHostInputObservationProducerAssurance,
    /// Commitment to the producer implementation.
    pub implementation: SpecContentHash,
    /// Commitment to the producer configuration.
    pub configuration: SpecContentHash,
}

impl TrustedHostInputObservationProducer {
    /// Creates a validated payload-free producer claim.
    ///
    /// # Errors
    ///
    /// Rejects incompatible producer-kind and assurance combinations.
    pub fn new(
        definition: TrustedHostInputObservationProducerDefinition,
    ) -> Result<Self, WorkflowOsError> {
        let assurance_valid = match definition.kind {
            TrustedHostInputObservationProducerKind::Fixture => {
                definition.assurance == TrustedHostInputObservationProducerAssurance::FixtureOnly
            }
            TrustedHostInputObservationProducerKind::HostObserver
            | TrustedHostInputObservationProducerKind::ExecutionProvider => {
                definition.assurance != TrustedHostInputObservationProducerAssurance::FixtureOnly
            }
        };
        if !assurance_valid {
            return Err(observer_error(
                "producer.assurance.invalid",
                "trusted-host input observation producer assurance is invalid",
            ));
        }
        Ok(Self {
            id: definition.id,
            version: definition.version,
            kind: definition.kind,
            assurance: definition.assurance,
            implementation: definition.implementation,
            configuration: definition.configuration,
        })
    }

    /// Returns the stable producer identity.
    #[must_use]
    pub const fn id(&self) -> &TrustedHostInputObservationProducerId {
        &self.id
    }

    /// Returns the stable producer version.
    #[must_use]
    pub const fn version(&self) -> &TrustedHostInputObservationProducerVersion {
        &self.version
    }

    /// Returns the closed producer category.
    #[must_use]
    pub const fn kind(&self) -> TrustedHostInputObservationProducerKind {
        self.kind
    }

    /// Returns the claimed producer assurance.
    #[must_use]
    pub const fn assurance(&self) -> TrustedHostInputObservationProducerAssurance {
        self.assurance
    }
}

impl TryFrom<TrustedHostInputObservationProducerDefinition>
    for TrustedHostInputObservationProducer
{
    type Error = WorkflowOsError;

    fn try_from(value: TrustedHostInputObservationProducerDefinition) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Debug for TrustedHostInputObservationProducer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostInputObservationProducer")
            .field("id", &self.id)
            .field("version", &self.version)
            .field("kind", &self.kind)
            .field("assurance", &self.assurance)
            .field("implementation", &"[REDACTED]")
            .field("configuration", &"[REDACTED]")
            .finish()
    }
}

/// Whether one host obligation was independently observed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostInputObservationPosture {
    /// Independently observed with proven binding and continuity.
    Observed,
    /// Declared by configuration but not independently observed.
    DeclaredOnly,
    /// Observation was unavailable.
    Unavailable,
    /// Observation is unsupported.
    Unsupported,
    /// Observation could not be established safely.
    Uncertain,
}

impl_closed_string_deserialize!(
    TrustedHostInputObservationPosture,
    "a supported trusted-host input observation posture",
    "unsupported trusted-host input observation posture",
    {
        "observed" => TrustedHostInputObservationPosture::Observed,
        "declared_only" => TrustedHostInputObservationPosture::DeclaredOnly,
        "unavailable" => TrustedHostInputObservationPosture::Unavailable,
        "unsupported" => TrustedHostInputObservationPosture::Unsupported,
        "uncertain" => TrustedHostInputObservationPosture::Uncertain
    }
);

/// Closed reason explaining why a host-input fact is not proven.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostInputObservationReason {
    /// The fact was declared but not independently observed.
    NotIndependentlyObserved,
    /// The observation channel was unavailable.
    ObservationUnavailable,
    /// The observation method is unsupported.
    ObservationUnsupported,
    /// Object integrity or continuity could not be established.
    IntegrityOrContinuityUncertain,
}

impl_closed_string_deserialize!(
    TrustedHostInputObservationReason,
    "a supported trusted-host input observation reason",
    "unsupported trusted-host input observation reason",
    {
        "not_independently_observed" => TrustedHostInputObservationReason::NotIndependentlyObserved,
        "observation_unavailable" => TrustedHostInputObservationReason::ObservationUnavailable,
        "observation_unsupported" => TrustedHostInputObservationReason::ObservationUnsupported,
        "integrity_or_continuity_uncertain" => TrustedHostInputObservationReason::IntegrityOrContinuityUncertain
    }
);

/// Closed method used to derive one host-input fact.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostInputObservationMethod {
    /// Content-addressed object identity.
    ContentAddressedObject,
    /// Identity retained through a protected handle.
    ProtectedHandle,
    /// Immutable projection supplied by the host.
    ImmutableProjection,
    /// Stable provider-native identity.
    ProviderNativeIdentity,
    /// Pre- and post-observation identity comparison.
    PreAndPostObjectIdentity,
    /// Explicit process-free test fixture.
    Fixture,
    /// Caller-declared configuration only.
    DeclaredConfiguration,
    /// Observation method was unavailable.
    Unavailable,
    /// Observation method is unsupported.
    Unsupported,
}

impl_closed_string_deserialize!(
    TrustedHostInputObservationMethod,
    "a supported trusted-host input observation method",
    "unsupported trusted-host input observation method",
    {
        "content_addressed_object" => TrustedHostInputObservationMethod::ContentAddressedObject,
        "protected_handle" => TrustedHostInputObservationMethod::ProtectedHandle,
        "immutable_projection" => TrustedHostInputObservationMethod::ImmutableProjection,
        "provider_native_identity" => TrustedHostInputObservationMethod::ProviderNativeIdentity,
        "pre_and_post_object_identity" => TrustedHostInputObservationMethod::PreAndPostObjectIdentity,
        "fixture" => TrustedHostInputObservationMethod::Fixture,
        "declared_configuration" => TrustedHostInputObservationMethod::DeclaredConfiguration,
        "unavailable" => TrustedHostInputObservationMethod::Unavailable,
        "unsupported" => TrustedHostInputObservationMethod::Unsupported
    }
);

/// One bounded host-input observation claim.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "TrustedHostInputObservationFactDefinition")]
pub struct TrustedHostInputObservationFact {
    producer_id: TrustedHostInputObservationProducerId,
    producer_version: TrustedHostInputObservationProducerVersion,
    kind: TrustedHostBuildInputHostObligationKind,
    posture: TrustedHostInputObservationPosture,
    reason: Option<TrustedHostInputObservationReason>,
    method: TrustedHostInputObservationMethod,
    object_binding: TrustedHostObservationPosture,
    continuity: TrustedHostObservationPosture,
    commitment: SpecContentHash,
    item_count: u32,
    reference: Option<HostedExecutionReference>,
}

/// Authoritative fact fields.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrustedHostInputObservationFactDefinition {
    /// Identity of the component that produced this fact.
    pub producer_id: TrustedHostInputObservationProducerId,
    /// Version of the component that produced this fact.
    pub producer_version: TrustedHostInputObservationProducerVersion,
    /// Closed host-input obligation category.
    pub kind: TrustedHostBuildInputHostObligationKind,
    /// Observation posture.
    pub posture: TrustedHostInputObservationPosture,
    /// Closed reason required for every non-proven posture.
    pub reason: Option<TrustedHostInputObservationReason>,
    /// Observation method.
    pub method: TrustedHostInputObservationMethod,
    /// Object-binding posture.
    pub object_binding: TrustedHostObservationPosture,
    /// Observation continuity posture.
    pub continuity: TrustedHostObservationPosture,
    /// Payload-free commitment to the observed value set.
    pub commitment: SpecContentHash,
    /// Bounded number of observed items.
    pub item_count: u32,
    /// Optional stable telemetry reference.
    pub reference: Option<HostedExecutionReference>,
}

impl TrustedHostInputObservationFact {
    /// Creates one validated payload-free observation fact.
    ///
    /// # Errors
    ///
    /// Rejects oversized counts, incoherent posture or method combinations,
    /// false claims of proven observation, and non-telemetry references.
    pub fn new(
        definition: TrustedHostInputObservationFactDefinition,
    ) -> Result<Self, WorkflowOsError> {
        if definition.item_count > FACT_ITEM_MAX_COUNT {
            return Err(observer_error(
                "fact.count.invalid",
                "trusted-host input observation fact count is invalid",
            ));
        }
        let observed_method = !matches!(
            definition.method,
            TrustedHostInputObservationMethod::DeclaredConfiguration
                | TrustedHostInputObservationMethod::Unavailable
                | TrustedHostInputObservationMethod::Unsupported
        );
        let both_proven = definition.object_binding == TrustedHostObservationPosture::Proven
            && definition.continuity == TrustedHostObservationPosture::Proven;
        let either_proven = definition.object_binding == TrustedHostObservationPosture::Proven
            || definition.continuity == TrustedHostObservationPosture::Proven;
        let posture_details_valid = match definition.posture {
            TrustedHostInputObservationPosture::Observed => {
                definition.reason.is_none() && observed_method && both_proven
            }
            TrustedHostInputObservationPosture::DeclaredOnly => {
                definition.reason
                    == Some(TrustedHostInputObservationReason::NotIndependentlyObserved)
                    && definition.method == TrustedHostInputObservationMethod::DeclaredConfiguration
                    && !either_proven
            }
            TrustedHostInputObservationPosture::Unavailable => {
                definition.reason == Some(TrustedHostInputObservationReason::ObservationUnavailable)
                    && definition.method == TrustedHostInputObservationMethod::Unavailable
                    && !either_proven
            }
            TrustedHostInputObservationPosture::Unsupported => {
                definition.reason == Some(TrustedHostInputObservationReason::ObservationUnsupported)
                    && definition.method == TrustedHostInputObservationMethod::Unsupported
                    && !either_proven
            }
            TrustedHostInputObservationPosture::Uncertain => {
                definition.reason
                    == Some(TrustedHostInputObservationReason::IntegrityOrContinuityUncertain)
                    && observed_method
                    && !either_proven
            }
        };
        if !posture_details_valid
            || definition.reference.as_ref().is_some_and(|reference| {
                reference.kind() != HostedExecutionReferenceKind::Telemetry
            })
        {
            return Err(observer_error(
                "fact.posture.invalid",
                "trusted-host input observation fact posture is invalid",
            ));
        }
        Ok(Self {
            producer_id: definition.producer_id,
            producer_version: definition.producer_version,
            kind: definition.kind,
            posture: definition.posture,
            reason: definition.reason,
            method: definition.method,
            object_binding: definition.object_binding,
            continuity: definition.continuity,
            commitment: definition.commitment,
            item_count: definition.item_count,
            reference: definition.reference,
        })
    }

    /// Returns the component identity that produced this fact.
    #[must_use]
    pub const fn producer_id(&self) -> &TrustedHostInputObservationProducerId {
        &self.producer_id
    }

    /// Returns the component version that produced this fact.
    #[must_use]
    pub const fn producer_version(&self) -> &TrustedHostInputObservationProducerVersion {
        &self.producer_version
    }

    /// Returns the closed host-input obligation category.
    #[must_use]
    pub const fn kind(&self) -> TrustedHostBuildInputHostObligationKind {
        self.kind
    }

    /// Returns the observation posture.
    #[must_use]
    pub const fn posture(&self) -> TrustedHostInputObservationPosture {
        self.posture
    }

    /// Returns the payload-free commitment to the observed value set.
    #[must_use]
    pub const fn commitment(&self) -> &SpecContentHash {
        &self.commitment
    }

    /// Returns the optional stable telemetry reference.
    #[must_use]
    pub const fn reference(&self) -> Option<&HostedExecutionReference> {
        self.reference.as_ref()
    }
}

impl TryFrom<TrustedHostInputObservationFactDefinition> for TrustedHostInputObservationFact {
    type Error = WorkflowOsError;
    fn try_from(value: TrustedHostInputObservationFactDefinition) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Debug for TrustedHostInputObservationFact {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostInputObservationFact")
            .field("producer_id", &self.producer_id)
            .field("producer_version", &self.producer_version)
            .field("kind", &self.kind)
            .field("posture", &self.posture)
            .field("reason", &self.reason)
            .field("method", &self.method)
            .field("object_binding", &self.object_binding)
            .field("continuity", &self.continuity)
            .field("item_count", &self.item_count)
            .field("commitment", &"[REDACTED]")
            .field("reference", &"[REDACTED]")
            .finish()
    }
}

/// Exact non-authorizing request for host-input observations.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "TrustedHostInputObservationRequestDefinition")]
pub struct TrustedHostInputObservationRequest {
    version: TrustedHostInputObservationContractVersion,
    run_bundle: SpecContentHash,
    operation_contract: SpecContentHash,
    static_request: SpecContentHash,
    static_result: SpecContentHash,
    ordering_authority: SpecContentHash,
    static_discovery_completed_revision: u64,
    repository_tree: SpecContentHash,
    security_requirements: SpecContentHash,
    selection: SpecContentHash,
    attempt: SpecContentHash,
    expected_producer_id: TrustedHostInputObservationProducerId,
    expected_producer_version: TrustedHostInputObservationProducerVersion,
    #[serde(skip_serializing)]
    fingerprint: SpecContentHash,
}

/// Authoritative request fields.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrustedHostInputObservationRequestDefinition {
    /// Closed observer contract version.
    pub version: TrustedHostInputObservationContractVersion,
    /// Immutable run-bundle commitment.
    pub run_bundle: SpecContentHash,
    /// Operation-contract commitment.
    pub operation_contract: SpecContentHash,
    /// Static request commitment.
    pub static_request: SpecContentHash,
    /// Static result commitment.
    pub static_result: SpecContentHash,
    /// Commitment identifying the authority for every ordered revision.
    pub ordering_authority: SpecContentHash,
    /// Static-discovery completion revision in the shared ordering authority.
    pub static_discovery_completed_revision: u64,
    /// Immutable repository-tree commitment.
    pub repository_tree: SpecContentHash,
    /// Security-requirements commitment.
    pub security_requirements: SpecContentHash,
    /// Provider-selection commitment.
    pub selection: SpecContentHash,
    /// Attempt commitment.
    pub attempt: SpecContentHash,
    /// Exact observer identity expected to answer this request.
    pub expected_producer_id: TrustedHostInputObservationProducerId,
    /// Exact observer version expected to answer this request.
    pub expected_producer_version: TrustedHostInputObservationProducerVersion,
}

impl TrustedHostInputObservationRequest {
    /// Creates an exact non-authorizing observation request.
    ///
    /// # Errors
    ///
    /// Rejects a missing static-discovery completion revision.
    pub fn new(
        definition: TrustedHostInputObservationRequestDefinition,
    ) -> Result<Self, WorkflowOsError> {
        if definition.static_discovery_completed_revision == 0 {
            return Err(observer_error(
                "request.freshness.invalid",
                "trusted-host input observation request freshness is invalid",
            ));
        }
        let fingerprint = request_fingerprint(&definition);
        Ok(Self {
            version: definition.version,
            run_bundle: definition.run_bundle,
            operation_contract: definition.operation_contract,
            static_request: definition.static_request,
            static_result: definition.static_result,
            ordering_authority: definition.ordering_authority,
            static_discovery_completed_revision: definition.static_discovery_completed_revision,
            repository_tree: definition.repository_tree,
            security_requirements: definition.security_requirements,
            selection: definition.selection,
            attempt: definition.attempt,
            expected_producer_id: definition.expected_producer_id,
            expected_producer_version: definition.expected_producer_version,
            fingerprint,
        })
    }

    /// Returns the canonical request commitment.
    #[must_use]
    pub const fn fingerprint(&self) -> &SpecContentHash {
        &self.fingerprint
    }
    /// Returns the required immutable repository-tree commitment.
    #[must_use]
    pub const fn repository_tree(&self) -> &SpecContentHash {
        &self.repository_tree
    }

    /// Returns the exact operation-contract commitment.
    #[must_use]
    pub const fn operation_contract(&self) -> &SpecContentHash {
        &self.operation_contract
    }

    /// Returns the exact security-requirements commitment.
    #[must_use]
    pub const fn security_requirements(&self) -> &SpecContentHash {
        &self.security_requirements
    }

    /// Returns the exact observer-selection commitment.
    #[must_use]
    pub const fn selection(&self) -> &SpecContentHash {
        &self.selection
    }

    /// Returns the authority commitment shared by every ordered revision.
    #[must_use]
    pub const fn ordering_authority(&self) -> &SpecContentHash {
        &self.ordering_authority
    }

    /// Returns the static-discovery completion revision observation must follow.
    #[must_use]
    pub const fn static_discovery_completed_revision(&self) -> u64 {
        self.static_discovery_completed_revision
    }

    /// Returns the exact expected producer identity.
    #[must_use]
    pub const fn expected_producer_id(&self) -> &TrustedHostInputObservationProducerId {
        &self.expected_producer_id
    }

    /// Returns the exact expected producer version.
    #[must_use]
    pub const fn expected_producer_version(&self) -> &TrustedHostInputObservationProducerVersion {
        &self.expected_producer_version
    }
}

impl TryFrom<TrustedHostInputObservationRequestDefinition> for TrustedHostInputObservationRequest {
    type Error = WorkflowOsError;

    fn try_from(value: TrustedHostInputObservationRequestDefinition) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Debug for TrustedHostInputObservationRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostInputObservationRequest")
            .field("version", &self.version)
            .field("run_bundle", &"[REDACTED]")
            .field("operation_contract", &"[REDACTED]")
            .field("static_request", &"[REDACTED]")
            .field("static_result", &"[REDACTED]")
            .field("ordering_authority", &"[REDACTED]")
            .field(
                "static_discovery_completed_revision",
                &self.static_discovery_completed_revision,
            )
            .field("repository_tree", &"[REDACTED]")
            .field("security_requirements", &"[REDACTED]")
            .field("selection", &"[REDACTED]")
            .field("attempt", &"[REDACTED]")
            .field("expected_producer_id", &self.expected_producer_id)
            .field("expected_producer_version", &self.expected_producer_version)
            .field("fingerprint", &"[REDACTED]")
            .finish()
    }
}

/// Complete payload-free observations for one exact request.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "TrustedHostInputObservationResultDefinition")]
pub struct TrustedHostInputObservationResult {
    request_fingerprint: SpecContentHash,
    repository_tree: SpecContentHash,
    ordering_authority: SpecContentHash,
    producer: TrustedHostInputObservationProducer,
    started_revision: u64,
    completed_revision: u64,
    repository_root: TrustedHostRootObservation,
    facts: Vec<TrustedHostInputObservationFact>,
    #[serde(skip_serializing)]
    fingerprint: SpecContentHash,
}

/// Authoritative result fields.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrustedHostInputObservationResultDefinition {
    /// Exact request commitment answered by this result.
    pub request_fingerprint: SpecContentHash,
    /// Immutable repository-tree commitment.
    pub repository_tree: SpecContentHash,
    /// Commitment identifying the authority for every ordered revision.
    pub ordering_authority: SpecContentHash,
    /// Validated producer claim.
    pub producer: TrustedHostInputObservationProducer,
    /// Observation start revision in the shared ordering authority.
    pub started_revision: u64,
    /// Observation completion revision in the shared ordering authority.
    pub completed_revision: u64,
    /// Independently observed repository-root binding.
    pub repository_root: TrustedHostRootObservation,
    /// Exactly one fact for every required host-input obligation.
    pub facts: Vec<TrustedHostInputObservationFact>,
}

impl TrustedHostInputObservationResult {
    /// Creates a complete, exact-request-bound observation result.
    ///
    /// # Errors
    ///
    /// Rejects invalid revision ordering, substituted repository bindings,
    /// non-read-only or invocation-scoped repository roots, and incomplete or
    /// duplicate obligation sets.
    pub fn new(
        mut definition: TrustedHostInputObservationResultDefinition,
    ) -> Result<Self, WorkflowOsError> {
        if definition.started_revision == 0
            || definition.completed_revision < definition.started_revision
            || definition.repository_root.role() != TrustedHostRootRole::RepositoryInput
            || definition.repository_root.access() != TrustedHostRootAccess::ReadOnly
            || definition.repository_root.invocation_scoped()
            || definition.repository_root.commitment() != &definition.repository_tree
            || definition.facts.len() != HOST_OBLIGATION_COUNT
            || definition
                .facts
                .iter()
                .map(TrustedHostInputObservationFact::kind)
                .collect::<BTreeSet<_>>()
                .len()
                != HOST_OBLIGATION_COUNT
            || definition.facts.iter().any(|fact| {
                fact.producer_id() != definition.producer.id()
                    || fact.producer_version() != definition.producer.version()
                    || !producer_method_is_coherent(&definition.producer, fact)
            })
        {
            return Err(observer_error(
                "result.binding.invalid",
                "trusted-host input observation result binding is invalid",
            ));
        }
        definition
            .facts
            .sort_unstable_by_key(TrustedHostInputObservationFact::kind);
        let fingerprint = result_fingerprint(&definition);
        Ok(Self {
            request_fingerprint: definition.request_fingerprint,
            repository_tree: definition.repository_tree,
            ordering_authority: definition.ordering_authority,
            producer: definition.producer,
            started_revision: definition.started_revision,
            completed_revision: definition.completed_revision,
            repository_root: definition.repository_root,
            facts: definition.facts,
            fingerprint,
        })
    }

    /// Validates that this result answers the exact supplied request.
    ///
    /// # Errors
    ///
    /// Rejects any request or repository-tree substitution.
    pub fn validate_for_request(
        &self,
        request: &TrustedHostInputObservationRequest,
    ) -> Result<(), WorkflowOsError> {
        if &self.request_fingerprint != request.fingerprint()
            || &self.repository_tree != request.repository_tree()
            || &self.ordering_authority != request.ordering_authority()
            || self.started_revision <= request.static_discovery_completed_revision()
            || self.producer.id() != request.expected_producer_id()
            || self.producer.version() != request.expected_producer_version()
        {
            return Err(observer_error(
                "result.request_mismatch",
                "trusted-host input observation result does not match its request",
            ));
        }
        Ok(())
    }

    /// Returns the canonical result commitment.
    #[must_use]
    pub const fn fingerprint(&self) -> &SpecContentHash {
        &self.fingerprint
    }
    /// Returns the complete sorted observation facts.
    #[must_use]
    pub fn facts(&self) -> &[TrustedHostInputObservationFact] {
        &self.facts
    }
    /// Returns the independent repository-root observation.
    #[must_use]
    pub const fn repository_root(&self) -> &TrustedHostRootObservation {
        &self.repository_root
    }

    /// Returns the producer claim without granting it authority.
    #[must_use]
    pub const fn producer(&self) -> &TrustedHostInputObservationProducer {
        &self.producer
    }

    /// Returns the completion revision in the shared ordering authority.
    #[must_use]
    pub const fn completed_revision(&self) -> u64 {
        self.completed_revision
    }
}

impl TryFrom<TrustedHostInputObservationResultDefinition> for TrustedHostInputObservationResult {
    type Error = WorkflowOsError;
    fn try_from(value: TrustedHostInputObservationResultDefinition) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Debug for TrustedHostInputObservationResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostInputObservationResult")
            .field("request_fingerprint", &"[REDACTED]")
            .field("repository_tree", &"[REDACTED]")
            .field("ordering_authority", &"[REDACTED]")
            .field("producer", &self.producer)
            .field("started_revision", &self.started_revision)
            .field("completed_revision", &self.completed_revision)
            .field("repository_root", &self.repository_root)
            .field("fact_count", &self.facts.len())
            .field("fingerprint", &"[REDACTED]")
            .finish()
    }
}

fn request_fingerprint(
    definition: &TrustedHostInputObservationRequestDefinition,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "domain", REQUEST_DOMAIN);
    hash_field(
        &mut hasher,
        "version",
        contract_version_tag(definition.version),
    );
    hash_field(
        &mut hasher,
        "static_discovery_completed_revision",
        &definition.static_discovery_completed_revision.to_string(),
    );
    hash_hash(
        &mut hasher,
        "ordering_authority",
        &definition.ordering_authority,
    );
    hash_field(
        &mut hasher,
        "expected_producer_id",
        definition.expected_producer_id.as_str(),
    );
    hash_field(
        &mut hasher,
        "expected_producer_version",
        definition.expected_producer_version.as_str(),
    );
    for (label, value) in [
        ("run_bundle", &definition.run_bundle),
        ("operation_contract", &definition.operation_contract),
        ("static_request", &definition.static_request),
        ("static_result", &definition.static_result),
        ("repository_tree", &definition.repository_tree),
        ("security_requirements", &definition.security_requirements),
        ("selection", &definition.selection),
        ("attempt", &definition.attempt),
    ] {
        hash_hash(&mut hasher, label, value);
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn result_fingerprint(definition: &TrustedHostInputObservationResultDefinition) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "domain", RESULT_DOMAIN);
    hash_hash(&mut hasher, "request", &definition.request_fingerprint);
    hash_hash(&mut hasher, "repository", &definition.repository_tree);
    hash_hash(
        &mut hasher,
        "ordering_authority",
        &definition.ordering_authority,
    );
    hash_producer(&mut hasher, &definition.producer);
    hash_field(
        &mut hasher,
        "started",
        &definition.started_revision.to_string(),
    );
    hash_field(
        &mut hasher,
        "completed",
        &definition.completed_revision.to_string(),
    );
    hash_repository_root(&mut hasher, &definition.repository_root);
    for fact in &definition.facts {
        hash_observation_fact(&mut hasher, fact);
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_producer(hasher: &mut Sha256, producer: &TrustedHostInputObservationProducer) {
    hash_field(hasher, "producer_id", producer.id.as_str());
    hash_field(hasher, "producer_version", producer.version.as_str());
    hash_field(hasher, "producer_kind", producer_kind_tag(producer.kind));
    hash_field(
        hasher,
        "producer_assurance",
        producer_assurance_tag(producer.assurance),
    );
    hash_hash(hasher, "producer_implementation", &producer.implementation);
    hash_hash(hasher, "producer_configuration", &producer.configuration);
}

fn hash_repository_root(hasher: &mut Sha256, root: &TrustedHostRootObservation) {
    hash_field(hasher, "root_role", root_role_tag(root.role()));
    hash_field(hasher, "root_access", root_access_tag(root.access()));
    hash_field(
        hasher,
        "root_object_binding",
        observation_posture_tag(root.object_binding()),
    );
    hash_field(
        hasher,
        "root_continuity",
        observation_posture_tag(root.continuity()),
    );
    hash_field(hasher, "root_method", root_method_tag(root.method()));
    hash_field(
        hasher,
        "root_invocation_scoped",
        bool_tag(root.invocation_scoped()),
    );
    hash_hash(hasher, "root_commitment", root.commitment());
    hash_reference(hasher, "root_reference", root.reference());
}

fn hash_observation_fact(hasher: &mut Sha256, fact: &TrustedHostInputObservationFact) {
    hash_field(hasher, "fact_producer_id", fact.producer_id.as_str());
    hash_field(
        hasher,
        "fact_producer_version",
        fact.producer_version.as_str(),
    );
    hash_field(hasher, "kind", obligation_kind_tag(fact.kind));
    hash_field(hasher, "posture", input_posture_tag(fact.posture));
    match fact.reason {
        Some(reason) => hash_field(hasher, "reason", input_reason_tag(reason)),
        None => hash_field(hasher, "reason", "none"),
    }
    hash_field(hasher, "method", input_method_tag(fact.method));
    hash_field(
        hasher,
        "object_binding",
        observation_posture_tag(fact.object_binding),
    );
    hash_field(
        hasher,
        "continuity",
        observation_posture_tag(fact.continuity),
    );
    hash_hash(hasher, "commitment", &fact.commitment);
    hash_field(hasher, "count", &fact.item_count.to_string());
    match &fact.reference {
        Some(reference) => hash_reference(hasher, "reference", reference),
        None => hash_field(hasher, "reference", "none"),
    }
}

fn contract_version_tag(value: TrustedHostInputObservationContractVersion) -> &'static str {
    match value {
        TrustedHostInputObservationContractVersion::CargoHostInputsV2 => "cargo_host_inputs_v2",
    }
}

fn input_reason_tag(value: TrustedHostInputObservationReason) -> &'static str {
    match value {
        TrustedHostInputObservationReason::NotIndependentlyObserved => "not_independently_observed",
        TrustedHostInputObservationReason::ObservationUnavailable => "observation_unavailable",
        TrustedHostInputObservationReason::ObservationUnsupported => "observation_unsupported",
        TrustedHostInputObservationReason::IntegrityOrContinuityUncertain => {
            "integrity_or_continuity_uncertain"
        }
    }
}

fn producer_method_is_coherent(
    producer: &TrustedHostInputObservationProducer,
    fact: &TrustedHostInputObservationFact,
) -> bool {
    if fact.posture != TrustedHostInputObservationPosture::Observed {
        return true;
    }
    if producer.assurance == TrustedHostInputObservationProducerAssurance::CallerSupplied {
        return false;
    }
    match producer.kind {
        TrustedHostInputObservationProducerKind::Fixture => {
            fact.method == TrustedHostInputObservationMethod::Fixture
        }
        TrustedHostInputObservationProducerKind::HostObserver => matches!(
            fact.method,
            TrustedHostInputObservationMethod::ContentAddressedObject
                | TrustedHostInputObservationMethod::ProtectedHandle
                | TrustedHostInputObservationMethod::ImmutableProjection
                | TrustedHostInputObservationMethod::PreAndPostObjectIdentity
        ),
        TrustedHostInputObservationProducerKind::ExecutionProvider => matches!(
            fact.method,
            TrustedHostInputObservationMethod::ContentAddressedObject
                | TrustedHostInputObservationMethod::ProtectedHandle
                | TrustedHostInputObservationMethod::ImmutableProjection
                | TrustedHostInputObservationMethod::ProviderNativeIdentity
                | TrustedHostInputObservationMethod::PreAndPostObjectIdentity
        ),
    }
}

fn producer_kind_tag(value: TrustedHostInputObservationProducerKind) -> &'static str {
    match value {
        TrustedHostInputObservationProducerKind::Fixture => "fixture",
        TrustedHostInputObservationProducerKind::HostObserver => "host_observer",
        TrustedHostInputObservationProducerKind::ExecutionProvider => "execution_provider",
    }
}

fn producer_assurance_tag(value: TrustedHostInputObservationProducerAssurance) -> &'static str {
    match value {
        TrustedHostInputObservationProducerAssurance::CallerSupplied => "caller_supplied",
        TrustedHostInputObservationProducerAssurance::FixtureOnly => "fixture_only",
        TrustedHostInputObservationProducerAssurance::IndependentlyReviewed => {
            "independently_reviewed"
        }
    }
}

fn input_posture_tag(value: TrustedHostInputObservationPosture) -> &'static str {
    match value {
        TrustedHostInputObservationPosture::Observed => "observed",
        TrustedHostInputObservationPosture::DeclaredOnly => "declared_only",
        TrustedHostInputObservationPosture::Unavailable => "unavailable",
        TrustedHostInputObservationPosture::Unsupported => "unsupported",
        TrustedHostInputObservationPosture::Uncertain => "uncertain",
    }
}

fn input_method_tag(value: TrustedHostInputObservationMethod) -> &'static str {
    match value {
        TrustedHostInputObservationMethod::ContentAddressedObject => "content_addressed_object",
        TrustedHostInputObservationMethod::ProtectedHandle => "protected_handle",
        TrustedHostInputObservationMethod::ImmutableProjection => "immutable_projection",
        TrustedHostInputObservationMethod::ProviderNativeIdentity => "provider_native_identity",
        TrustedHostInputObservationMethod::PreAndPostObjectIdentity => {
            "pre_and_post_object_identity"
        }
        TrustedHostInputObservationMethod::Fixture => "fixture",
        TrustedHostInputObservationMethod::DeclaredConfiguration => "declared_configuration",
        TrustedHostInputObservationMethod::Unavailable => "unavailable",
        TrustedHostInputObservationMethod::Unsupported => "unsupported",
    }
}

fn obligation_kind_tag(value: TrustedHostBuildInputHostObligationKind) -> &'static str {
    match value {
        TrustedHostBuildInputHostObligationKind::ToolchainExecutable => "toolchain_executable",
        TrustedHostBuildInputHostObligationKind::TargetSpecification => "target_specification",
        TrustedHostBuildInputHostObligationKind::SdkOrSysroot => "sdk_or_sysroot",
        TrustedHostBuildInputHostObligationKind::LinkerOrArchiver => "linker_or_archiver",
        TrustedHostBuildInputHostObligationKind::NativeCompilerTools => "native_compiler_tools",
        TrustedHostBuildInputHostObligationKind::DynamicLibraries => "dynamic_libraries",
        TrustedHostBuildInputHostObligationKind::RegistryAndPackageSources => {
            "registry_and_package_sources"
        }
        TrustedHostBuildInputHostObligationKind::Environment => "environment",
        TrustedHostBuildInputHostObligationKind::RuntimeBase => "runtime_base",
    }
}

fn observation_posture_tag(value: TrustedHostObservationPosture) -> &'static str {
    match value {
        TrustedHostObservationPosture::Proven => "proven",
        TrustedHostObservationPosture::Degraded => "degraded",
        TrustedHostObservationPosture::Unavailable => "unavailable",
        TrustedHostObservationPosture::Unsupported => "unsupported",
        TrustedHostObservationPosture::Uncertain => "uncertain",
    }
}

fn root_role_tag(value: TrustedHostRootRole) -> &'static str {
    match value {
        TrustedHostRootRole::RepositoryInput => "repository_input",
        TrustedHostRootRole::ToolchainInput => "toolchain_input",
        TrustedHostRootRole::DependencyInput => "dependency_input",
        TrustedHostRootRole::CacheWrite => "cache_write",
        TrustedHostRootRole::BuildOutputWrite => "build_output_write",
        TrustedHostRootRole::TemporaryWrite => "temporary_write",
    }
}

fn root_access_tag(value: TrustedHostRootAccess) -> &'static str {
    match value {
        TrustedHostRootAccess::ReadOnly => "read_only",
        TrustedHostRootAccess::ReadExecute => "read_execute",
        TrustedHostRootAccess::ReadWrite => "read_write",
    }
}

fn root_method_tag(value: TrustedHostRootObservationMethod) -> &'static str {
    match value {
        TrustedHostRootObservationMethod::ProtectedHandle => "protected_handle",
        TrustedHostRootObservationMethod::ImmutableProjection => "immutable_projection",
        TrustedHostRootObservationMethod::ProviderNativeIdentity => "provider_native_identity",
        TrustedHostRootObservationMethod::Unavailable => "unavailable",
        TrustedHostRootObservationMethod::Unsupported => "unsupported",
    }
}

fn reference_kind_tag(value: HostedExecutionReferenceKind) -> &'static str {
    match value {
        HostedExecutionReferenceKind::Input => "input",
        HostedExecutionReferenceKind::Artifact => "artifact",
        HostedExecutionReferenceKind::Log => "log",
        HostedExecutionReferenceKind::DeniedAction => "denied_action",
        HostedExecutionReferenceKind::Telemetry => "telemetry",
        HostedExecutionReferenceKind::AccessMaterial => "access_material",
        HostedExecutionReferenceKind::SideEffectReconciliation => "side_effect_reconciliation",
    }
}

fn bool_tag(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn hash_reference(hasher: &mut Sha256, label: &str, value: &HostedExecutionReference) {
    hash_field(hasher, label, reference_kind_tag(value.kind()));
    hash_field(hasher, label, value.value());
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &str) {
    hash_bytes(hasher, label.as_bytes());
    hash_bytes(hasher, value.as_bytes());
}

fn hash_hash(hasher: &mut Sha256, label: &str, value: &SpecContentHash) {
    hash_field(hasher, label, value.as_str());
}

fn hash_bytes(hasher: &mut Sha256, value: &[u8]) {
    hasher.update(u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(value);
}

fn is_secret_like(value: &str) -> bool {
    let lowered = value.to_ascii_lowercase();
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
    .any(|marker| lowered.contains(marker))
}

fn observer_error(suffix: &str, message: &str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("trusted_host_input_observer.{suffix}"), message)
}

#[cfg(test)]
mod tests;
