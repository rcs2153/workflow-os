//! Non-authorizing model for deterministic trusted-host static input discovery.
//!
//! These values describe one exact static metadata discovery request and its
//! payload-free result. They do not read files, parse manifests, run build
//! code, select a provider, grant authority, or prove that observations are
//! true.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::de::IgnoredAny;
use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    BuildTimeInputKind, BuildTimeInputKindCount, HostedExecutionReference,
    HostedExecutionReferenceKind, SpecContentHash, WorkflowOsError,
};

const INPUT_KIND_MAX_COUNT: usize = 10;
const INPUT_TOTAL_MAX_COUNT: u32 = 16_384;
const HOST_OBLIGATION_COUNT: usize = 9;
const UNSUPPORTED_REASON_MAX_COUNT: usize = 16;
const REQUEST_DOMAIN: &str = "workflow-os/trusted-host-build-input-discovery-request/v1";
const RESULT_DOMAIN: &str = "workflow-os/trusted-host-build-input-discovery-result/v1";

struct IgnoredWireField {
    _marker: u8,
}

impl<'de> Deserialize<'de> for IgnoredWireField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        IgnoredAny::deserialize(deserializer)?;
        Ok(Self { _marker: 0 })
    }
}

macro_rules! impl_closed_string_deserialize {
    ($type:ty, $expecting:literal, $error:literal, { $($tag:literal => $variant:path),+ $(,)? }) => {
        impl<'de> Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
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

/// Closed version of the static discovery contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostBuildInputDiscoveryContractVersion {
    /// Static Cargo graph contract, version one.
    StaticCargoGraphV1,
}

impl<'de> Deserialize<'de> for TrustedHostBuildInputDiscoveryContractVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = TrustedHostBuildInputDiscoveryContractVersion;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a supported trusted-host build-input discovery version")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "static_cargo_graph_v1" => {
                        Ok(TrustedHostBuildInputDiscoveryContractVersion::StaticCargoGraphV1)
                    }
                    _ => Err(E::custom(
                        "unsupported trusted-host build-input discovery version",
                    )),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

/// How the static dependency selection was bounded.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostBuildInputSelectionPosture {
    /// Target, feature, profile, and package selection is explicitly committed.
    Explicit,
    /// Every statically reachable potential edge is conservatively included.
    ConservativeAllPotentialEdges,
}

impl<'de> Deserialize<'de> for TrustedHostBuildInputSelectionPosture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = TrustedHostBuildInputSelectionPosture;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a supported static build-input selection posture")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "explicit" => Ok(TrustedHostBuildInputSelectionPosture::Explicit),
                    "conservative_all_potential_edges" => {
                        Ok(TrustedHostBuildInputSelectionPosture::ConservativeAllPotentialEdges)
                    }
                    _ => Err(E::custom(
                        "unsupported static build-input selection posture",
                    )),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

/// Completeness of the approved static metadata graph only.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostStaticGraphCompleteness {
    /// Every approved static source was resolved under the closed contract.
    Complete,
    /// The producer knows the approved static graph is incomplete.
    Incomplete,
    /// The graph shape is unsupported by the closed contract.
    Unsupported,
    /// The graph cannot be established without ambiguity.
    Uncertain,
}

impl<'de> Deserialize<'de> for TrustedHostStaticGraphCompleteness {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = TrustedHostStaticGraphCompleteness;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a supported static graph completeness posture")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "complete" => Ok(TrustedHostStaticGraphCompleteness::Complete),
                    "incomplete" => Ok(TrustedHostStaticGraphCompleteness::Incomplete),
                    "unsupported" => Ok(TrustedHostStaticGraphCompleteness::Unsupported),
                    "uncertain" => Ok(TrustedHostStaticGraphCompleteness::Uncertain),
                    _ => Err(E::custom("unsupported static graph completeness posture")),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

/// Host fact that static repository discovery cannot prove.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostBuildInputHostObligationKind {
    /// Actual compiler and component object identities.
    ToolchainExecutable,
    /// Effective compilation target specification.
    TargetSpecification,
    /// Effective SDK or sysroot object identity.
    SdkOrSysroot,
    /// Effective linker and archiver object identities.
    LinkerOrArchiver,
    /// Native compiler or build-tool object identities.
    NativeCompilerTools,
    /// Dynamically loaded library object identities.
    DynamicLibraries,
    /// Registry, index, and package-source object identities.
    RegistryAndPackageSources,
    /// Effective environment inputs.
    Environment,
    /// Runtime or base-image object identity.
    RuntimeBase,
}

impl_closed_string_deserialize!(
    TrustedHostBuildInputHostObligationKind,
    "a supported trusted-host build-input host obligation",
    "unsupported trusted-host build-input host obligation",
    {
        "toolchain_executable" => TrustedHostBuildInputHostObligationKind::ToolchainExecutable,
        "target_specification" => TrustedHostBuildInputHostObligationKind::TargetSpecification,
        "sdk_or_sysroot" => TrustedHostBuildInputHostObligationKind::SdkOrSysroot,
        "linker_or_archiver" => TrustedHostBuildInputHostObligationKind::LinkerOrArchiver,
        "native_compiler_tools" => TrustedHostBuildInputHostObligationKind::NativeCompilerTools,
        "dynamic_libraries" => TrustedHostBuildInputHostObligationKind::DynamicLibraries,
        "registry_and_package_sources" => TrustedHostBuildInputHostObligationKind::RegistryAndPackageSources,
        "environment" => TrustedHostBuildInputHostObligationKind::Environment,
        "runtime_base" => TrustedHostBuildInputHostObligationKind::RuntimeBase,
    }
);

/// Static discovery posture for one host obligation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostBuildInputHostObligationPosture {
    /// A later observer may support the fact, but no observation is available.
    Unavailable,
    /// Static discovery cannot observe this fact.
    Unsupported,
}

impl_closed_string_deserialize!(
    TrustedHostBuildInputHostObligationPosture,
    "a supported trusted-host build-input host obligation posture",
    "unsupported trusted-host build-input host obligation posture",
    {
        "unavailable" => TrustedHostBuildInputHostObligationPosture::Unavailable,
        "unsupported" => TrustedHostBuildInputHostObligationPosture::Unsupported,
    }
);

/// Closed posture for one required host fact.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct TrustedHostBuildInputHostObligation {
    kind: TrustedHostBuildInputHostObligationKind,
    posture: TrustedHostBuildInputHostObligationPosture,
}

impl<'de> Deserialize<'de> for TrustedHostBuildInputHostObligation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            kind: TrustedHostBuildInputHostObligationKind,
            posture: TrustedHostBuildInputHostObligationPosture,
            #[serde(flatten)]
            extra: BTreeMap<String, IgnoredWireField>,
        }

        let wire = Wire::deserialize(deserializer)?;
        if !wire.extra.is_empty() {
            return Err(serde::de::Error::custom(
                "trusted-host static discovery host obligation contains an unknown field",
            ));
        }
        Ok(Self::new(wire.kind, wire.posture))
    }
}

impl TrustedHostBuildInputHostObligation {
    /// Creates one non-satisfied host obligation.
    #[must_use]
    pub const fn new(
        kind: TrustedHostBuildInputHostObligationKind,
        posture: TrustedHostBuildInputHostObligationPosture,
    ) -> Self {
        Self { kind, posture }
    }

    /// Returns the closed host obligation kind.
    #[must_use]
    pub const fn kind(&self) -> TrustedHostBuildInputHostObligationKind {
        self.kind
    }

    /// Returns the unsupported or unavailable posture.
    #[must_use]
    pub const fn posture(&self) -> TrustedHostBuildInputHostObligationPosture {
        self.posture
    }
}

/// Stable, payload-free reason that a static graph is not complete.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostBuildInputUnsupportedReason {
    /// Repository object binding is unavailable.
    RepositoryObjectBindingUnavailable,
    /// Feature, target, package, or profile selection is ambiguous.
    SelectionAmbiguous,
    /// Implicit target discovery is unsupported.
    ImplicitTargetUnsupported,
    /// Implicit build-script discovery is unsupported.
    ImplicitBuildScriptUnsupported,
    /// Cargo configuration semantics are unsupported.
    CargoConfigurationUnsupported,
    /// Package source replacement is unsupported.
    SourceReplacementUnsupported,
    /// A symbolic link is unsupported.
    SymlinkUnsupported,
    /// A submodule or nested repository is unsupported.
    NestedRepositoryUnsupported,
    /// A repository-local path escapes the committed root.
    RootEscape,
    /// The lockfile is missing or inconsistent.
    LockfileInconsistent,
    /// Metadata changed during discovery.
    MetadataChanged,
    /// A hard discovery bound was exceeded.
    BoundExceeded,
    /// Supported metadata could not be parsed deterministically.
    MetadataInvalid,
}

impl_closed_string_deserialize!(
    TrustedHostBuildInputUnsupportedReason,
    "a supported trusted-host static discovery reason",
    "unsupported trusted-host static discovery reason",
    {
        "repository_object_binding_unavailable" => TrustedHostBuildInputUnsupportedReason::RepositoryObjectBindingUnavailable,
        "selection_ambiguous" => TrustedHostBuildInputUnsupportedReason::SelectionAmbiguous,
        "implicit_target_unsupported" => TrustedHostBuildInputUnsupportedReason::ImplicitTargetUnsupported,
        "implicit_build_script_unsupported" => TrustedHostBuildInputUnsupportedReason::ImplicitBuildScriptUnsupported,
        "cargo_configuration_unsupported" => TrustedHostBuildInputUnsupportedReason::CargoConfigurationUnsupported,
        "source_replacement_unsupported" => TrustedHostBuildInputUnsupportedReason::SourceReplacementUnsupported,
        "symlink_unsupported" => TrustedHostBuildInputUnsupportedReason::SymlinkUnsupported,
        "nested_repository_unsupported" => TrustedHostBuildInputUnsupportedReason::NestedRepositoryUnsupported,
        "root_escape" => TrustedHostBuildInputUnsupportedReason::RootEscape,
        "lockfile_inconsistent" => TrustedHostBuildInputUnsupportedReason::LockfileInconsistent,
        "metadata_changed" => TrustedHostBuildInputUnsupportedReason::MetadataChanged,
        "bound_exceeded" => TrustedHostBuildInputUnsupportedReason::BoundExceeded,
        "metadata_invalid" => TrustedHostBuildInputUnsupportedReason::MetadataInvalid,
    }
);

/// Exact non-authorizing request for one future static discovery operation.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "TrustedHostBuildInputDiscoveryRequestDefinition")]
pub struct TrustedHostBuildInputDiscoveryRequest {
    version: TrustedHostBuildInputDiscoveryContractVersion,
    repository_tree_commitment: SpecContentHash,
    selection_posture: TrustedHostBuildInputSelectionPosture,
    selection_commitment: SpecContentHash,
    metadata_source_set_commitment: SpecContentHash,
    discovery_policy_commitment: SpecContentHash,
    #[serde(skip_serializing)]
    request_fingerprint: SpecContentHash,
}

/// Authoritative fields for a static discovery request.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct TrustedHostBuildInputDiscoveryRequestDefinition {
    /// Closed discovery contract version.
    pub version: TrustedHostBuildInputDiscoveryContractVersion,
    /// Commitment to the exact repository snapshot.
    pub repository_tree_commitment: SpecContentHash,
    /// Closed selection posture.
    pub selection_posture: TrustedHostBuildInputSelectionPosture,
    /// Commitment to package, feature, target, and profile selection.
    pub selection_commitment: SpecContentHash,
    /// Commitment to the exact approved metadata source set.
    pub metadata_source_set_commitment: SpecContentHash,
    /// Commitment to bounds and supported discovery semantics.
    pub discovery_policy_commitment: SpecContentHash,
}

impl<'de> Deserialize<'de> for TrustedHostBuildInputDiscoveryRequestDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            version: TrustedHostBuildInputDiscoveryContractVersion,
            repository_tree_commitment: SpecContentHash,
            selection_posture: TrustedHostBuildInputSelectionPosture,
            selection_commitment: SpecContentHash,
            metadata_source_set_commitment: SpecContentHash,
            discovery_policy_commitment: SpecContentHash,
            #[serde(flatten)]
            extra: BTreeMap<String, IgnoredWireField>,
        }

        let wire = Wire::deserialize(deserializer)?;
        if !wire.extra.is_empty() {
            return Err(serde::de::Error::custom(
                "trusted-host static discovery request contains an unknown field",
            ));
        }
        Ok(Self {
            version: wire.version,
            repository_tree_commitment: wire.repository_tree_commitment,
            selection_posture: wire.selection_posture,
            selection_commitment: wire.selection_commitment,
            metadata_source_set_commitment: wire.metadata_source_set_commitment,
            discovery_policy_commitment: wire.discovery_policy_commitment,
        })
    }
}

impl TrustedHostBuildInputDiscoveryRequest {
    /// Creates an exact non-authorizing discovery request.
    #[must_use]
    pub fn new(definition: TrustedHostBuildInputDiscoveryRequestDefinition) -> Self {
        let request_fingerprint = request_fingerprint(&definition);
        Self {
            version: definition.version,
            repository_tree_commitment: definition.repository_tree_commitment,
            selection_posture: definition.selection_posture,
            selection_commitment: definition.selection_commitment,
            metadata_source_set_commitment: definition.metadata_source_set_commitment,
            discovery_policy_commitment: definition.discovery_policy_commitment,
            request_fingerprint,
        }
    }

    /// Returns the closed contract version.
    #[must_use]
    pub const fn version(&self) -> TrustedHostBuildInputDiscoveryContractVersion {
        self.version
    }

    /// Returns the committed repository snapshot.
    #[must_use]
    pub const fn repository_tree_commitment(&self) -> &SpecContentHash {
        &self.repository_tree_commitment
    }

    /// Returns the closed selection posture.
    #[must_use]
    pub const fn selection_posture(&self) -> TrustedHostBuildInputSelectionPosture {
        self.selection_posture
    }

    /// Returns the committed selection.
    #[must_use]
    pub const fn selection_commitment(&self) -> &SpecContentHash {
        &self.selection_commitment
    }

    /// Returns the approved metadata source-set commitment.
    #[must_use]
    pub const fn metadata_source_set_commitment(&self) -> &SpecContentHash {
        &self.metadata_source_set_commitment
    }

    /// Returns the discovery policy commitment.
    #[must_use]
    pub const fn discovery_policy_commitment(&self) -> &SpecContentHash {
        &self.discovery_policy_commitment
    }

    /// Returns the derived canonical request fingerprint.
    #[must_use]
    pub const fn request_fingerprint(&self) -> &SpecContentHash {
        &self.request_fingerprint
    }
}

impl From<TrustedHostBuildInputDiscoveryRequestDefinition>
    for TrustedHostBuildInputDiscoveryRequest
{
    fn from(value: TrustedHostBuildInputDiscoveryRequestDefinition) -> Self {
        Self::new(value)
    }
}

impl fmt::Debug for TrustedHostBuildInputDiscoveryRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostBuildInputDiscoveryRequest")
            .field("version", &self.version)
            .field("repository_tree_commitment", &"[REDACTED]")
            .field("selection_posture", &self.selection_posture)
            .field("selection_commitment", &"[REDACTED]")
            .field("metadata_source_set_commitment", &"[REDACTED]")
            .field("discovery_policy_commitment", &"[REDACTED]")
            .field("request_fingerprint", &"[REDACTED]")
            .finish()
    }
}

/// Payload-free result for one exact static discovery request.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "TrustedHostBuildInputDiscoveryResultDefinition")]
pub struct TrustedHostBuildInputDiscoveryResult {
    request_fingerprint: SpecContentHash,
    resolved_graph_commitment: SpecContentHash,
    static_completeness: TrustedHostStaticGraphCompleteness,
    total_count: u32,
    kind_counts: Vec<BuildTimeInputKindCount>,
    host_obligations: Vec<TrustedHostBuildInputHostObligation>,
    unsupported_reasons: Vec<TrustedHostBuildInputUnsupportedReason>,
    observation_reference: Option<HostedExecutionReference>,
    #[serde(skip_serializing)]
    graph_commitment: SpecContentHash,
}

/// Authoritative fields for a static discovery result.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct TrustedHostBuildInputDiscoveryResultDefinition {
    /// Exact request fingerprint this result answers.
    pub request_fingerprint: SpecContentHash,
    /// Commitment to canonical resolved package nodes and dependency edges.
    pub resolved_graph_commitment: SpecContentHash,
    /// Completeness of the approved static graph only.
    pub static_completeness: TrustedHostStaticGraphCompleteness,
    /// Total bounded static input count.
    pub total_count: u32,
    /// Bounded counts by closed input kind.
    pub kind_counts: Vec<BuildTimeInputKindCount>,
    /// Every host fact that static discovery cannot satisfy.
    pub host_obligations: Vec<TrustedHostBuildInputHostObligation>,
    /// Stable reasons for non-complete static posture.
    pub unsupported_reasons: Vec<TrustedHostBuildInputUnsupportedReason>,
    /// Optional payload-free telemetry reference.
    pub observation_reference: Option<HostedExecutionReference>,
}

#[derive(Deserialize)]
struct StrictBuildTimeInputKindCountWire {
    kind: BuildTimeInputKind,
    count: u32,
    #[serde(flatten)]
    extra: BTreeMap<String, IgnoredWireField>,
}

#[derive(Deserialize)]
struct StrictHostedExecutionReferenceWire {
    kind: HostedExecutionReferenceKind,
    value: String,
    #[serde(flatten)]
    extra: BTreeMap<String, IgnoredWireField>,
}

impl<'de> Deserialize<'de> for TrustedHostBuildInputDiscoveryResultDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            request_fingerprint: SpecContentHash,
            resolved_graph_commitment: SpecContentHash,
            static_completeness: TrustedHostStaticGraphCompleteness,
            total_count: u32,
            kind_counts: Vec<StrictBuildTimeInputKindCountWire>,
            host_obligations: Vec<TrustedHostBuildInputHostObligation>,
            unsupported_reasons: Vec<TrustedHostBuildInputUnsupportedReason>,
            observation_reference: Option<StrictHostedExecutionReferenceWire>,
            #[serde(flatten)]
            extra: BTreeMap<String, IgnoredWireField>,
        }

        let wire = Wire::deserialize(deserializer)?;
        if !wire.extra.is_empty()
            || wire.kind_counts.iter().any(|entry| !entry.extra.is_empty())
            || wire
                .observation_reference
                .as_ref()
                .is_some_and(|reference| !reference.extra.is_empty())
        {
            return Err(serde::de::Error::custom(
                "trusted-host static discovery result contains an unknown field",
            ));
        }
        let kind_counts = wire
            .kind_counts
            .into_iter()
            .map(|entry| BuildTimeInputKindCount::new(entry.kind, entry.count))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| {
                serde::de::Error::custom(
                    "trusted-host static discovery input kind count is invalid",
                )
            })?;
        let observation_reference = wire
            .observation_reference
            .map(|reference| HostedExecutionReference::new(reference.kind, reference.value))
            .transpose()
            .map_err(|_| {
                serde::de::Error::custom("trusted-host static discovery reference is invalid")
            })?;
        Ok(Self {
            request_fingerprint: wire.request_fingerprint,
            resolved_graph_commitment: wire.resolved_graph_commitment,
            static_completeness: wire.static_completeness,
            total_count: wire.total_count,
            kind_counts,
            host_obligations: wire.host_obligations,
            unsupported_reasons: wire.unsupported_reasons,
            observation_reference,
        })
    }
}

impl TrustedHostBuildInputDiscoveryResult {
    /// Creates and canonically commits a payload-free static discovery result.
    ///
    /// # Errors
    ///
    /// Rejects incoherent counts, incomplete host obligations, invalid reason
    /// posture, oversized collections, or an invalid reference kind.
    pub fn new(
        mut definition: TrustedHostBuildInputDiscoveryResultDefinition,
    ) -> Result<Self, WorkflowOsError> {
        validate_counts(definition.total_count, &definition.kind_counts)?;
        if definition.static_completeness == TrustedHostStaticGraphCompleteness::Complete
            && definition.total_count == 0
        {
            return Err(discovery_error(
                "result.completeness.invalid",
                "trusted-host static discovery completeness posture is invalid",
            ));
        }
        validate_host_obligations(&definition.host_obligations)?;
        validate_reasons(
            definition.static_completeness,
            &definition.unsupported_reasons,
        )?;
        if let Some(reference) = &definition.observation_reference {
            if reference.kind() != HostedExecutionReferenceKind::Telemetry {
                return Err(discovery_error(
                    "result.reference.invalid",
                    "trusted-host static discovery reference is invalid",
                ));
            }
        }

        definition
            .kind_counts
            .sort_unstable_by_key(BuildTimeInputKindCount::kind);
        definition
            .host_obligations
            .sort_unstable_by_key(TrustedHostBuildInputHostObligation::kind);
        definition.unsupported_reasons.sort_unstable();
        let graph_commitment = result_commitment(&definition);

        Ok(Self {
            request_fingerprint: definition.request_fingerprint,
            resolved_graph_commitment: definition.resolved_graph_commitment,
            static_completeness: definition.static_completeness,
            total_count: definition.total_count,
            kind_counts: definition.kind_counts,
            host_obligations: definition.host_obligations,
            unsupported_reasons: definition.unsupported_reasons,
            observation_reference: definition.observation_reference,
            graph_commitment,
        })
    }

    /// Returns the exact request fingerprint.
    #[must_use]
    pub const fn request_fingerprint(&self) -> &SpecContentHash {
        &self.request_fingerprint
    }

    /// Returns the canonical resolved package-graph commitment.
    #[must_use]
    pub const fn resolved_graph_commitment(&self) -> &SpecContentHash {
        &self.resolved_graph_commitment
    }

    /// Returns static graph completeness, not live build-input completeness.
    #[must_use]
    pub const fn static_completeness(&self) -> TrustedHostStaticGraphCompleteness {
        self.static_completeness
    }

    /// Returns the bounded total static input count.
    #[must_use]
    pub const fn total_count(&self) -> u32 {
        self.total_count
    }

    /// Returns canonical counts by closed input kind.
    #[must_use]
    pub fn kind_counts(&self) -> &[BuildTimeInputKindCount] {
        &self.kind_counts
    }

    /// Returns the complete canonical host-obligation set.
    #[must_use]
    pub fn host_obligations(&self) -> &[TrustedHostBuildInputHostObligation] {
        &self.host_obligations
    }

    /// Returns canonical non-completeness reasons.
    #[must_use]
    pub fn unsupported_reasons(&self) -> &[TrustedHostBuildInputUnsupportedReason] {
        &self.unsupported_reasons
    }

    /// Returns the optional payload-free observation reference.
    #[must_use]
    pub const fn observation_reference(&self) -> Option<&HostedExecutionReference> {
        self.observation_reference.as_ref()
    }

    /// Returns the derived canonical static graph commitment.
    #[must_use]
    pub const fn graph_commitment(&self) -> &SpecContentHash {
        &self.graph_commitment
    }

    /// Verifies this result answers the exact request.
    ///
    /// # Errors
    ///
    /// Rejects request substitution without exposing either commitment.
    pub fn validate_for_request(
        &self,
        request: &TrustedHostBuildInputDiscoveryRequest,
    ) -> Result<(), WorkflowOsError> {
        if self.request_fingerprint != request.request_fingerprint {
            return Err(discovery_error(
                "result.request_mismatch",
                "trusted-host static discovery result does not match request",
            ));
        }
        Ok(())
    }
}

impl TryFrom<TrustedHostBuildInputDiscoveryResultDefinition>
    for TrustedHostBuildInputDiscoveryResult
{
    type Error = WorkflowOsError;

    fn try_from(
        value: TrustedHostBuildInputDiscoveryResultDefinition,
    ) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Debug for TrustedHostBuildInputDiscoveryResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostBuildInputDiscoveryResult")
            .field("request_fingerprint", &"[REDACTED]")
            .field("resolved_graph_commitment", &"[REDACTED]")
            .field("static_completeness", &self.static_completeness)
            .field("total_count", &self.total_count)
            .field("kind_counts", &self.kind_counts)
            .field("host_obligations", &self.host_obligations)
            .field("unsupported_reasons", &self.unsupported_reasons)
            .field(
                "observation_reference",
                &self.observation_reference.as_ref().map(|_| "[REDACTED]"),
            )
            .field("graph_commitment", &"[REDACTED]")
            .finish()
    }
}

fn validate_counts(
    total_count: u32,
    kind_counts: &[BuildTimeInputKindCount],
) -> Result<(), WorkflowOsError> {
    if total_count > INPUT_TOTAL_MAX_COUNT || kind_counts.len() > INPUT_KIND_MAX_COUNT {
        return Err(discovery_error(
            "result.count.invalid",
            "trusted-host static discovery count is invalid",
        ));
    }
    let mut kinds = BTreeSet::new();
    let mut sum = 0_u32;
    for entry in kind_counts {
        if entry.count() == 0 {
            return Err(discovery_error(
                "result.kind_count.invalid",
                "trusted-host static discovery input kind count is invalid",
            ));
        }
        if !kinds.insert(entry.kind()) {
            return Err(discovery_error(
                "result.kind.duplicate",
                "trusted-host static discovery contains a duplicate input kind",
            ));
        }
        sum = sum.checked_add(entry.count()).ok_or_else(|| {
            discovery_error(
                "result.count.invalid",
                "trusted-host static discovery count is invalid",
            )
        })?;
    }
    if sum != total_count {
        return Err(discovery_error(
            "result.count.mismatch",
            "trusted-host static discovery counts do not reconcile",
        ));
    }
    Ok(())
}

fn validate_host_obligations(
    obligations: &[TrustedHostBuildInputHostObligation],
) -> Result<(), WorkflowOsError> {
    if obligations.len() != HOST_OBLIGATION_COUNT {
        return Err(discovery_error(
            "result.host_obligations.invalid",
            "trusted-host static discovery host obligations are invalid",
        ));
    }
    let kinds: BTreeSet<_> = obligations
        .iter()
        .map(TrustedHostBuildInputHostObligation::kind)
        .collect();
    if kinds.len() != HOST_OBLIGATION_COUNT {
        return Err(discovery_error(
            "result.host_obligations.invalid",
            "trusted-host static discovery host obligations are invalid",
        ));
    }
    Ok(())
}

fn validate_reasons(
    completeness: TrustedHostStaticGraphCompleteness,
    reasons: &[TrustedHostBuildInputUnsupportedReason],
) -> Result<(), WorkflowOsError> {
    if reasons.len() > UNSUPPORTED_REASON_MAX_COUNT
        || reasons.iter().copied().collect::<BTreeSet<_>>().len() != reasons.len()
        || (completeness == TrustedHostStaticGraphCompleteness::Complete && !reasons.is_empty())
        || (completeness != TrustedHostStaticGraphCompleteness::Complete && reasons.is_empty())
    {
        return Err(discovery_error(
            "result.reasons.invalid",
            "trusted-host static discovery reasons are invalid",
        ));
    }
    Ok(())
}

fn request_fingerprint(
    definition: &TrustedHostBuildInputDiscoveryRequestDefinition,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "domain", REQUEST_DOMAIN);
    hash_field(
        &mut hasher,
        "version",
        contract_version_tag(definition.version),
    );
    hash_hash(
        &mut hasher,
        "repository_tree",
        &definition.repository_tree_commitment,
    );
    hash_field(
        &mut hasher,
        "selection_posture",
        selection_posture_tag(definition.selection_posture),
    );
    hash_hash(&mut hasher, "selection", &definition.selection_commitment);
    hash_hash(
        &mut hasher,
        "metadata_source_set",
        &definition.metadata_source_set_commitment,
    );
    hash_hash(
        &mut hasher,
        "discovery_policy",
        &definition.discovery_policy_commitment,
    );
    SpecContentHash::from_bytes(hasher.finalize())
}

fn result_commitment(
    definition: &TrustedHostBuildInputDiscoveryResultDefinition,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "domain", RESULT_DOMAIN);
    hash_hash(
        &mut hasher,
        "request_fingerprint",
        &definition.request_fingerprint,
    );
    hash_hash(
        &mut hasher,
        "resolved_graph",
        &definition.resolved_graph_commitment,
    );
    hash_field(
        &mut hasher,
        "static_completeness",
        completeness_tag(definition.static_completeness),
    );
    hash_field(
        &mut hasher,
        "total_count",
        &definition.total_count.to_string(),
    );
    for entry in &definition.kind_counts {
        hash_field(&mut hasher, "input_kind", input_kind_tag(entry.kind()));
        hash_field(&mut hasher, "input_count", &entry.count().to_string());
    }
    for obligation in &definition.host_obligations {
        hash_field(
            &mut hasher,
            "host_obligation",
            host_obligation_kind_tag(obligation.kind()),
        );
        hash_field(
            &mut hasher,
            "host_obligation_posture",
            host_obligation_posture_tag(obligation.posture()),
        );
    }
    for reason in &definition.unsupported_reasons {
        hash_field(
            &mut hasher,
            "unsupported_reason",
            unsupported_reason_tag(*reason),
        );
    }
    if let Some(reference) = &definition.observation_reference {
        hash_field(&mut hasher, "reference_kind", "telemetry");
        hash_field(&mut hasher, "reference_value", reference.value());
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_hash(hasher: &mut Sha256, label: &str, value: &SpecContentHash) {
    hash_field(hasher, label, value.as_str());
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update(u64::try_from(label.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(value.as_bytes());
}

const fn contract_version_tag(
    value: TrustedHostBuildInputDiscoveryContractVersion,
) -> &'static str {
    match value {
        TrustedHostBuildInputDiscoveryContractVersion::StaticCargoGraphV1 => {
            "static_cargo_graph_v1"
        }
    }
}

const fn selection_posture_tag(value: TrustedHostBuildInputSelectionPosture) -> &'static str {
    match value {
        TrustedHostBuildInputSelectionPosture::Explicit => "explicit",
        TrustedHostBuildInputSelectionPosture::ConservativeAllPotentialEdges => {
            "conservative_all_potential_edges"
        }
    }
}

const fn completeness_tag(value: TrustedHostStaticGraphCompleteness) -> &'static str {
    match value {
        TrustedHostStaticGraphCompleteness::Complete => "complete",
        TrustedHostStaticGraphCompleteness::Incomplete => "incomplete",
        TrustedHostStaticGraphCompleteness::Unsupported => "unsupported",
        TrustedHostStaticGraphCompleteness::Uncertain => "uncertain",
    }
}

const fn input_kind_tag(value: BuildTimeInputKind) -> &'static str {
    match value {
        BuildTimeInputKind::RepositoryContent => "repository_content",
        BuildTimeInputKind::ManifestOrConfiguration => "manifest_or_configuration",
        BuildTimeInputKind::BuildScript => "build_script",
        BuildTimeInputKind::ProceduralMacro => "procedural_macro",
        BuildTimeInputKind::DependencyPackage => "dependency_package",
        BuildTimeInputKind::ToolchainExecutable => "toolchain_executable",
        BuildTimeInputKind::LinkerOrArchiver => "linker_or_archiver",
        BuildTimeInputKind::SdkOrSysroot => "sdk_or_sysroot",
        BuildTimeInputKind::DynamicLibrary => "dynamic_library",
        BuildTimeInputKind::RuntimeBase => "runtime_base",
    }
}

const fn host_obligation_kind_tag(value: TrustedHostBuildInputHostObligationKind) -> &'static str {
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

const fn host_obligation_posture_tag(
    value: TrustedHostBuildInputHostObligationPosture,
) -> &'static str {
    match value {
        TrustedHostBuildInputHostObligationPosture::Unavailable => "unavailable",
        TrustedHostBuildInputHostObligationPosture::Unsupported => "unsupported",
    }
}

const fn unsupported_reason_tag(value: TrustedHostBuildInputUnsupportedReason) -> &'static str {
    match value {
        TrustedHostBuildInputUnsupportedReason::RepositoryObjectBindingUnavailable => {
            "repository_object_binding_unavailable"
        }
        TrustedHostBuildInputUnsupportedReason::SelectionAmbiguous => "selection_ambiguous",
        TrustedHostBuildInputUnsupportedReason::ImplicitTargetUnsupported => {
            "implicit_target_unsupported"
        }
        TrustedHostBuildInputUnsupportedReason::ImplicitBuildScriptUnsupported => {
            "implicit_build_script_unsupported"
        }
        TrustedHostBuildInputUnsupportedReason::CargoConfigurationUnsupported => {
            "cargo_configuration_unsupported"
        }
        TrustedHostBuildInputUnsupportedReason::SourceReplacementUnsupported => {
            "source_replacement_unsupported"
        }
        TrustedHostBuildInputUnsupportedReason::SymlinkUnsupported => "symlink_unsupported",
        TrustedHostBuildInputUnsupportedReason::NestedRepositoryUnsupported => {
            "nested_repository_unsupported"
        }
        TrustedHostBuildInputUnsupportedReason::RootEscape => "root_escape",
        TrustedHostBuildInputUnsupportedReason::LockfileInconsistent => "lockfile_inconsistent",
        TrustedHostBuildInputUnsupportedReason::MetadataChanged => "metadata_changed",
        TrustedHostBuildInputUnsupportedReason::BoundExceeded => "bound_exceeded",
        TrustedHostBuildInputUnsupportedReason::MetadataInvalid => "metadata_invalid",
    }
}

fn discovery_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("trusted_host.build_input_discovery.{suffix}"),
        message,
    )
}

#[cfg(test)]
#[path = "trusted_host_build_input_discovery_tests.rs"]
mod tests;
