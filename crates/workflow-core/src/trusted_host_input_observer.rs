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
const REQUEST_DOMAIN: &str = "workflow-os/trusted-host-input-observation-request/v1";
const RESULT_DOMAIN: &str = "workflow-os/trusted-host-input-observation-result/v1";

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
    /// Cargo/Rust host-input observation contract, version one.
    CargoHostInputsV1,
}

impl_closed_string_deserialize!(
    TrustedHostInputObservationContractVersion,
    "a supported trusted-host input observation contract version",
    "unsupported trusted-host input observation contract version",
    { "cargo_host_inputs_v1" => TrustedHostInputObservationContractVersion::CargoHostInputsV1 }
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
    kind: TrustedHostInputObservationProducerKind,
    assurance: TrustedHostInputObservationProducerAssurance,
    implementation: SpecContentHash,
    configuration: SpecContentHash,
}

/// Authoritative producer fields.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrustedHostInputObservationProducerDefinition {
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
        if (definition.kind == TrustedHostInputObservationProducerKind::Fixture)
            != (definition.assurance == TrustedHostInputObservationProducerAssurance::FixtureOnly)
        {
            return Err(observer_error(
                "producer.assurance.invalid",
                "trusted-host input observation producer assurance is invalid",
            ));
        }
        Ok(Self {
            kind: definition.kind,
            assurance: definition.assurance,
            implementation: definition.implementation,
            configuration: definition.configuration,
        })
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
    kind: TrustedHostBuildInputHostObligationKind,
    posture: TrustedHostInputObservationPosture,
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
    /// Closed host-input obligation category.
    pub kind: TrustedHostBuildInputHostObligationKind,
    /// Observation posture.
    pub posture: TrustedHostInputObservationPosture,
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
        let observed = definition.posture == TrustedHostInputObservationPosture::Observed;
        let observed_method = !matches!(
            definition.method,
            TrustedHostInputObservationMethod::DeclaredConfiguration
                | TrustedHostInputObservationMethod::Unavailable
                | TrustedHostInputObservationMethod::Unsupported
        );
        let proven = definition.object_binding == TrustedHostObservationPosture::Proven
            && definition.continuity == TrustedHostObservationPosture::Proven;
        if observed != (observed_method && proven)
            || (!observed && proven)
            || (definition.posture == TrustedHostInputObservationPosture::DeclaredOnly
                && definition.method != TrustedHostInputObservationMethod::DeclaredConfiguration)
            || (definition.posture == TrustedHostInputObservationPosture::Unavailable
                && definition.method != TrustedHostInputObservationMethod::Unavailable)
            || (definition.posture == TrustedHostInputObservationPosture::Unsupported
                && definition.method != TrustedHostInputObservationMethod::Unsupported)
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
            kind: definition.kind,
            posture: definition.posture,
            method: definition.method,
            object_binding: definition.object_binding,
            continuity: definition.continuity,
            commitment: definition.commitment,
            item_count: definition.item_count,
            reference: definition.reference,
        })
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
            .field("kind", &self.kind)
            .field("posture", &self.posture)
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
    repository_tree: SpecContentHash,
    security_requirements: SpecContentHash,
    selection: SpecContentHash,
    attempt: SpecContentHash,
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
    /// Immutable repository-tree commitment.
    pub repository_tree: SpecContentHash,
    /// Security-requirements commitment.
    pub security_requirements: SpecContentHash,
    /// Provider-selection commitment.
    pub selection: SpecContentHash,
    /// Attempt commitment.
    pub attempt: SpecContentHash,
}

impl TrustedHostInputObservationRequest {
    #[must_use]
    /// Creates an exact non-authorizing observation request.
    pub fn new(definition: TrustedHostInputObservationRequestDefinition) -> Self {
        let fingerprint = request_fingerprint(&definition);
        Self {
            version: definition.version,
            run_bundle: definition.run_bundle,
            operation_contract: definition.operation_contract,
            static_request: definition.static_request,
            static_result: definition.static_result,
            repository_tree: definition.repository_tree,
            security_requirements: definition.security_requirements,
            selection: definition.selection,
            attempt: definition.attempt,
            fingerprint,
        }
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
}

impl From<TrustedHostInputObservationRequestDefinition> for TrustedHostInputObservationRequest {
    fn from(value: TrustedHostInputObservationRequestDefinition) -> Self {
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
            .field("repository_tree", &"[REDACTED]")
            .field("security_requirements", &"[REDACTED]")
            .field("selection", &"[REDACTED]")
            .field("attempt", &"[REDACTED]")
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
    /// Validated producer claim.
    pub producer: TrustedHostInputObservationProducer,
    /// Producer-local starting revision.
    pub started_revision: u64,
    /// Producer-local terminal revision.
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
    hash_field(
        &mut hasher,
        "producer_kind",
        producer_kind_tag(definition.producer.kind),
    );
    hash_field(
        &mut hasher,
        "producer_assurance",
        producer_assurance_tag(definition.producer.assurance),
    );
    hash_hash(
        &mut hasher,
        "producer_implementation",
        &definition.producer.implementation,
    );
    hash_hash(
        &mut hasher,
        "producer_configuration",
        &definition.producer.configuration,
    );
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
    hash_field(
        &mut hasher,
        "root_role",
        root_role_tag(definition.repository_root.role()),
    );
    hash_field(
        &mut hasher,
        "root_access",
        root_access_tag(definition.repository_root.access()),
    );
    hash_field(
        &mut hasher,
        "root_object_binding",
        observation_posture_tag(definition.repository_root.object_binding()),
    );
    hash_field(
        &mut hasher,
        "root_continuity",
        observation_posture_tag(definition.repository_root.continuity()),
    );
    hash_field(
        &mut hasher,
        "root_method",
        root_method_tag(definition.repository_root.method()),
    );
    hash_field(
        &mut hasher,
        "root_invocation_scoped",
        bool_tag(definition.repository_root.invocation_scoped()),
    );
    hash_hash(
        &mut hasher,
        "root_commitment",
        definition.repository_root.commitment(),
    );
    hash_reference(
        &mut hasher,
        "root_reference",
        definition.repository_root.reference(),
    );
    for fact in &definition.facts {
        hash_field(&mut hasher, "kind", obligation_kind_tag(fact.kind));
        hash_field(&mut hasher, "posture", input_posture_tag(fact.posture));
        hash_field(&mut hasher, "method", input_method_tag(fact.method));
        hash_field(
            &mut hasher,
            "object_binding",
            observation_posture_tag(fact.object_binding),
        );
        hash_field(
            &mut hasher,
            "continuity",
            observation_posture_tag(fact.continuity),
        );
        hash_hash(&mut hasher, "commitment", &fact.commitment);
        hash_field(&mut hasher, "count", &fact.item_count.to_string());
        match &fact.reference {
            Some(reference) => hash_reference(&mut hasher, "reference", reference),
            None => hash_field(&mut hasher, "reference", "none"),
        }
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn contract_version_tag(value: TrustedHostInputObservationContractVersion) -> &'static str {
    match value {
        TrustedHostInputObservationContractVersion::CargoHostInputsV1 => "cargo_host_inputs_v1",
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

fn observer_error(suffix: &str, message: &str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("trusted_host_input_observer.{suffix}"), message)
}

#[cfg(test)]
mod tests;
