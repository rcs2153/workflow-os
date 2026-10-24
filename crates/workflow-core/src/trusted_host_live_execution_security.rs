//! Provider-neutral, non-authorizing security facts for future trusted-host execution.
//!
//! These models do not run a process, discover inputs, select a provider, grant
//! authority, or prove that caller-supplied observations are true. They only
//! validate bounded requirements and observations and compare them purely.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    HostedExecutionCleanupPosture, HostedExecutionControlPosture, HostedExecutionEnforcementMode,
    HostedExecutionObservationSummary, HostedExecutionProviderId, HostedExecutionProviderVersion,
    HostedExecutionReference, HostedExecutionReferenceKind, SpecContentHash, WorkflowOsError,
};

const ROOT_ROLE_COUNT: usize = 6;
const INPUT_KIND_MAX_COUNT: usize = 10;
const INPUT_TOTAL_MAX_COUNT: u32 = 16_384;
const SECURITY_EVENT_MAX_COUNT: u32 = 1_000_000;
const REQUIREMENTS_DOMAIN: &str = "workflow-os/trusted-host-live-security-requirements/v1";

/// Closed, versioned set of Core-owned security requirements.
///
/// `StrictBuildExecutionV1` requires enforce mode; enforced filesystem,
/// process, network, protected-root, and all resource-budget controls;
/// default-deny filesystem and network policy; exclusion of host home,
/// credential stores, and ambient temporary paths; complete observation;
/// zero allowed network events; terminal process-tree proof; quiescent outputs;
/// a retained-artifact commitment; and completed cleanup.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostLiveExecutionSecurityProfile {
    /// Strict no-network build execution contract, version one.
    StrictBuildExecutionV1,
}

impl<'de> Deserialize<'de> for TrustedHostLiveExecutionSecurityProfile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SecurityProfileVisitor;

        impl serde::de::Visitor<'_> for SecurityProfileVisitor {
            type Value = TrustedHostLiveExecutionSecurityProfile;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a supported trusted-host live-execution security profile")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "strict_build_execution_v1" => {
                        Ok(TrustedHostLiveExecutionSecurityProfile::StrictBuildExecutionV1)
                    }
                    _ => Err(E::custom(
                        "unsupported trusted-host live-execution security profile",
                    )),
                }
            }
        }

        deserializer.deserialize_str(SecurityProfileVisitor)
    }
}

/// Closed build-time input categories. No path or payload is stored.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildTimeInputKind {
    /// Repository-controlled source and included content.
    RepositoryContent,
    /// Build manifests and configuration.
    ManifestOrConfiguration,
    /// Executable build-script input.
    BuildScript,
    /// Procedural macro input.
    ProceduralMacro,
    /// Selected dependency package input.
    DependencyPackage,
    /// Compiler or other toolchain executable.
    ToolchainExecutable,
    /// Linker, archiver, or equivalent platform tool.
    LinkerOrArchiver,
    /// Target SDK or sysroot input.
    SdkOrSysroot,
    /// Dynamically loaded library input.
    DynamicLibrary,
    /// Runtime or sandbox base identity.
    RuntimeBase,
}

/// How a future component claims it derived an input manifest.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildTimeInputDiscoveryMethod {
    /// Core validated a closed manifest claim.
    CoreValidatedManifest,
    /// A future provider reported the observation.
    ProviderObserved,
    /// Core and provider observations were combined.
    CoreAndProvider,
    /// Discovery evidence was unavailable.
    Unavailable,
    /// The boundary does not support discovery.
    Unsupported,
}

/// Claimed completeness of a build-time input manifest.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildTimeInputCompletenessPosture {
    /// The producer claims the input set is complete.
    Complete,
    /// The producer knows the input set is incomplete.
    Incomplete,
    /// Completeness evidence is unavailable.
    Unavailable,
    /// Completeness cannot be observed by this boundary.
    Unsupported,
    /// Completeness cannot be established safely.
    Uncertain,
}

/// Bounded count for one input category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct BuildTimeInputKindCount {
    kind: BuildTimeInputKind,
    count: u32,
}

impl BuildTimeInputKindCount {
    /// Creates a bounded category count.
    ///
    /// # Errors
    ///
    /// Rejects counts above the model limit.
    pub fn new(kind: BuildTimeInputKind, count: u32) -> Result<Self, WorkflowOsError> {
        if count > INPUT_TOTAL_MAX_COUNT {
            return Err(live_security_error(
                "input_manifest.kind_count.invalid",
                "trusted-host input manifest kind count is invalid",
            ));
        }
        Ok(Self { kind, count })
    }

    #[must_use]
    /// Returns the closed input kind.
    pub const fn kind(&self) -> BuildTimeInputKind {
        self.kind
    }

    #[must_use]
    /// Returns the bounded count.
    pub const fn count(&self) -> u32 {
        self.count
    }
}

/// Payload-free claimed manifest for build-time inputs.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "BuildTimeInputManifestDefinition")]
pub struct BuildTimeInputManifest {
    fingerprint: SpecContentHash,
    total_count: u32,
    kind_counts: Vec<BuildTimeInputKindCount>,
    discovery_method: BuildTimeInputDiscoveryMethod,
    completeness: BuildTimeInputCompletenessPosture,
    observation_reference: Option<HostedExecutionReference>,
}

/// Deserializable definition for a build-time input manifest.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BuildTimeInputManifestDefinition {
    /// Deterministic commitment to the claimed manifest.
    pub fingerprint: SpecContentHash,
    /// Total claimed input count.
    pub total_count: u32,
    /// Per-kind counts.
    pub kind_counts: Vec<BuildTimeInputKindCount>,
    /// Claimed discovery method.
    pub discovery_method: BuildTimeInputDiscoveryMethod,
    /// Claimed completeness posture.
    pub completeness: BuildTimeInputCompletenessPosture,
    /// Optional stable telemetry reference.
    pub observation_reference: Option<HostedExecutionReference>,
}

impl BuildTimeInputManifest {
    /// Creates a bounded manifest claim. Validation does not prove completeness.
    ///
    /// # Errors
    ///
    /// Rejects oversized, duplicate, inconsistent, or semantically impossible claims.
    pub fn new(mut definition: BuildTimeInputManifestDefinition) -> Result<Self, WorkflowOsError> {
        if definition.total_count > INPUT_TOTAL_MAX_COUNT
            || definition.kind_counts.len() > INPUT_KIND_MAX_COUNT
        {
            return Err(live_security_error(
                "input_manifest.count.invalid",
                "trusted-host input manifest count is invalid",
            ));
        }
        let mut kinds = BTreeSet::new();
        let mut sum = 0_u32;
        for entry in &definition.kind_counts {
            if !kinds.insert(entry.kind) {
                return Err(live_security_error(
                    "input_manifest.kind.duplicate",
                    "trusted-host input manifest contains a duplicate kind",
                ));
            }
            sum = sum.checked_add(entry.count).ok_or_else(|| {
                live_security_error(
                    "input_manifest.count.invalid",
                    "trusted-host input manifest count is invalid",
                )
            })?;
        }
        if sum != definition.total_count {
            return Err(live_security_error(
                "input_manifest.count.mismatch",
                "trusted-host input manifest counts do not reconcile",
            ));
        }
        definition
            .kind_counts
            .sort_unstable_by_key(|entry| entry.kind);
        if definition.completeness == BuildTimeInputCompletenessPosture::Complete
            && (definition.total_count == 0
                || matches!(
                    definition.discovery_method,
                    BuildTimeInputDiscoveryMethod::Unavailable
                        | BuildTimeInputDiscoveryMethod::Unsupported
                ))
        {
            return Err(live_security_error(
                "input_manifest.completeness.invalid",
                "trusted-host input manifest completeness posture is invalid",
            ));
        }
        if let Some(reference) = &definition.observation_reference {
            if reference.kind() != HostedExecutionReferenceKind::Telemetry {
                return Err(live_security_error(
                    "input_manifest.reference.invalid",
                    "trusted-host input manifest reference is invalid",
                ));
            }
        }
        Ok(Self {
            fingerprint: definition.fingerprint,
            total_count: definition.total_count,
            kind_counts: definition.kind_counts,
            discovery_method: definition.discovery_method,
            completeness: definition.completeness,
            observation_reference: definition.observation_reference,
        })
    }

    #[must_use]
    /// Returns the manifest commitment.
    pub const fn fingerprint(&self) -> &SpecContentHash {
        &self.fingerprint
    }

    #[must_use]
    /// Returns the total input count.
    pub const fn total_count(&self) -> u32 {
        self.total_count
    }

    #[must_use]
    /// Returns the canonical per-kind counts.
    pub fn kind_counts(&self) -> &[BuildTimeInputKindCount] {
        &self.kind_counts
    }

    #[must_use]
    /// Returns the discovery method.
    pub const fn discovery_method(&self) -> BuildTimeInputDiscoveryMethod {
        self.discovery_method
    }

    #[must_use]
    /// Returns the completeness claim.
    pub const fn completeness(&self) -> BuildTimeInputCompletenessPosture {
        self.completeness
    }
}

impl TryFrom<BuildTimeInputManifestDefinition> for BuildTimeInputManifest {
    type Error = WorkflowOsError;

    fn try_from(value: BuildTimeInputManifestDefinition) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Debug for BuildTimeInputManifest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BuildTimeInputManifest")
            .field("fingerprint", &"[REDACTED]")
            .field("total_count", &self.total_count)
            .field("kind_count", &self.kind_counts.len())
            .field("discovery_method", &self.discovery_method)
            .field("completeness", &self.completeness)
            .field("observation_reference", &"[REDACTED]")
            .finish()
    }
}

/// Typed root roles required by the accepted Cargo clippy boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostRootRole {
    /// Immutable repository input.
    RepositoryInput,
    /// Immutable toolchain input.
    ToolchainInput,
    /// Immutable dependency input.
    DependencyInput,
    /// Invocation-scoped Cargo cache output.
    CacheWrite,
    /// Invocation-scoped build output.
    BuildOutputWrite,
    /// Invocation-scoped temporary output.
    TemporaryWrite,
}

/// Required or observed access class for a root.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostRootAccess {
    /// Read access only.
    ReadOnly,
    /// Read and execute access.
    ReadExecute,
    /// Read and write access.
    ReadWrite,
}

/// Required object-binding assurance.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostRootBindingRequirement {
    /// Preserve object identity from admission through terminal observation.
    AdmissionToTerminal,
    /// Project an immutable provider-owned root.
    ProviderImmutableProjection,
}

/// Observed certainty for a control fact not covered by hosted posture vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostObservationPosture {
    /// The fact was affirmatively observed.
    Proven,
    /// The fact was observed with weaker behavior.
    Degraded,
    /// The observation was unavailable.
    Unavailable,
    /// The observation is unsupported.
    Unsupported,
    /// The fact cannot be established safely.
    Uncertain,
}

/// Closed affirmation for one independently observed binary security fact.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostSecurityAffirmation {
    /// The fact was affirmatively observed.
    Affirmed,
    /// The fact was not affirmatively observed.
    NotAffirmed,
}

/// Provider-neutral method used to observe root identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostRootObservationMethod {
    /// Object identity was held through a protected handle.
    ProtectedHandle,
    /// The provider used an immutable projection.
    ImmutableProjection,
    /// A provider-native stable identity was observed.
    ProviderNativeIdentity,
    /// No observation method was available.
    Unavailable,
    /// No supported observation method exists.
    Unsupported,
}

/// Required posture for one typed root.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct TrustedHostRootRequirement {
    role: TrustedHostRootRole,
    access: TrustedHostRootAccess,
    binding: TrustedHostRootBindingRequirement,
    invocation_scoped_creation: bool,
}

impl TrustedHostRootRequirement {
    /// Creates one typed root requirement.
    #[must_use]
    pub const fn new(
        role: TrustedHostRootRole,
        access: TrustedHostRootAccess,
        binding: TrustedHostRootBindingRequirement,
        invocation_scoped_creation: bool,
    ) -> Self {
        Self {
            role,
            access,
            binding,
            invocation_scoped_creation,
        }
    }

    #[must_use]
    /// Returns the root role.
    pub const fn role(&self) -> TrustedHostRootRole {
        self.role
    }

    #[must_use]
    /// Returns the required access class.
    pub const fn access(&self) -> TrustedHostRootAccess {
        self.access
    }

    #[must_use]
    /// Returns the required binding assurance.
    pub const fn binding(&self) -> TrustedHostRootBindingRequirement {
        self.binding
    }

    #[must_use]
    /// Returns whether invocation-scoped creation is mandatory.
    pub const fn invocation_scoped_creation(&self) -> bool {
        self.invocation_scoped_creation
    }
}

/// Observed posture for one typed root. It stores no path or native object id.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "TrustedHostRootObservationDefinition")]
pub struct TrustedHostRootObservation {
    role: TrustedHostRootRole,
    access: TrustedHostRootAccess,
    object_binding: TrustedHostObservationPosture,
    continuity: TrustedHostObservationPosture,
    method: TrustedHostRootObservationMethod,
    invocation_scoped: bool,
    commitment: SpecContentHash,
    reference: HostedExecutionReference,
}

/// Deserializable definition for one root observation.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TrustedHostRootObservationDefinition {
    /// Observed root role.
    pub role: TrustedHostRootRole,
    /// Observed access class.
    pub access: TrustedHostRootAccess,
    /// Observed object-binding posture.
    pub object_binding: TrustedHostObservationPosture,
    /// Admission-to-terminal continuity posture.
    pub continuity: TrustedHostObservationPosture,
    /// Observation method.
    pub method: TrustedHostRootObservationMethod,
    /// Whether the root was invocation scoped.
    pub invocation_scoped: bool,
    /// Payload-free root commitment.
    pub commitment: SpecContentHash,
    /// Stable telemetry reference.
    pub reference: HostedExecutionReference,
}

impl TrustedHostRootObservation {
    /// Creates a bounded root observation.
    ///
    /// # Errors
    ///
    /// Rejects invalid telemetry references and incoherent method/posture combinations.
    pub fn new(definition: TrustedHostRootObservationDefinition) -> Result<Self, WorkflowOsError> {
        if definition.reference.kind() != HostedExecutionReferenceKind::Telemetry {
            return Err(live_security_error(
                "root_binding.reference.invalid",
                "trusted-host root observation reference is invalid",
            ));
        }
        if matches!(
            definition.method,
            TrustedHostRootObservationMethod::Unavailable
                | TrustedHostRootObservationMethod::Unsupported
        ) && (definition.object_binding == TrustedHostObservationPosture::Proven
            || definition.continuity == TrustedHostObservationPosture::Proven)
        {
            return Err(live_security_error(
                "root_binding.posture.invalid",
                "trusted-host root observation posture is invalid",
            ));
        }
        Ok(Self {
            role: definition.role,
            access: definition.access,
            object_binding: definition.object_binding,
            continuity: definition.continuity,
            method: definition.method,
            invocation_scoped: definition.invocation_scoped,
            commitment: definition.commitment,
            reference: definition.reference,
        })
    }

    #[must_use]
    /// Returns the observed root role.
    pub const fn role(&self) -> TrustedHostRootRole {
        self.role
    }

    #[must_use]
    /// Returns the observed access class.
    pub const fn access(&self) -> TrustedHostRootAccess {
        self.access
    }

    #[must_use]
    /// Returns the object-binding posture.
    pub const fn object_binding(&self) -> TrustedHostObservationPosture {
        self.object_binding
    }

    #[must_use]
    /// Returns the continuity posture.
    pub const fn continuity(&self) -> TrustedHostObservationPosture {
        self.continuity
    }

    #[must_use]
    /// Returns whether the observed root was invocation scoped.
    pub const fn invocation_scoped(&self) -> bool {
        self.invocation_scoped
    }
}

impl TryFrom<TrustedHostRootObservationDefinition> for TrustedHostRootObservation {
    type Error = WorkflowOsError;

    fn try_from(value: TrustedHostRootObservationDefinition) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Debug for TrustedHostRootObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostRootObservation")
            .field("role", &self.role)
            .field("access", &self.access)
            .field("object_binding", &self.object_binding)
            .field("continuity", &self.continuity)
            .field("method", &self.method)
            .field("invocation_scoped", &self.invocation_scoped)
            .field("commitment", &"[REDACTED]")
            .field("reference", &"[REDACTED]")
            .finish()
    }
}

/// Exact payload-free binding for one future attempt.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TrustedHostLiveExecutionBinding {
    run_bundle: SpecContentHash,
    command_contract: SpecContentHash,
    attempt: SpecContentHash,
    request: SpecContentHash,
    provider_selection: SpecContentHash,
}

impl TrustedHostLiveExecutionBinding {
    /// Creates an exact payload-free attempt binding.
    #[must_use]
    pub const fn new(
        run_bundle: SpecContentHash,
        command_contract: SpecContentHash,
        attempt: SpecContentHash,
        request: SpecContentHash,
        provider_selection: SpecContentHash,
    ) -> Self {
        Self {
            run_bundle,
            command_contract,
            attempt,
            request,
            provider_selection,
        }
    }
}

impl fmt::Debug for TrustedHostLiveExecutionBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostLiveExecutionBinding")
            .field("run_bundle", &"[REDACTED]")
            .field("command_contract", &"[REDACTED]")
            .field("attempt", &"[REDACTED]")
            .field("request", &"[REDACTED]")
            .field("provider_selection", &"[REDACTED]")
            .finish()
    }
}

/// Explicit process-start posture for safe-denial versus reconciliation decisions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostAttemptStartPosture {
    /// The provider affirmatively observed that execution did not start.
    NotStarted,
    /// The provider affirmatively observed execution start.
    Started,
    /// Execution may have started and requires reconciliation.
    MayHaveStarted,
}

/// Complete-process-tree terminality, separate from parent exit and cleanup.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostProcessTreeTerminality {
    /// No process tree existed because execution did not start.
    NotObserved,
    /// The complete process tree is terminal.
    Terminal,
    /// At least one process remains non-terminal.
    NonTerminal,
    /// Complete-tree terminality cannot be established.
    Uncertain,
}

/// Observed full-process-tree posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "TrustedHostProcessTreeObservationDefinition")]
pub struct TrustedHostProcessTreeObservation {
    attempt_start: TrustedHostAttemptStartPosture,
    parent_exited: bool,
    terminality: TrustedHostProcessTreeTerminality,
    cancellation_or_timeout_started: bool,
    termination_completed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct TrustedHostProcessTreeObservationDefinition {
    attempt_start: TrustedHostAttemptStartPosture,
    parent_exited: bool,
    terminality: TrustedHostProcessTreeTerminality,
    cancellation_or_timeout_started: bool,
    termination_completed: bool,
}

impl TrustedHostProcessTreeObservation {
    /// Creates a coherent process-tree observation.
    ///
    /// # Errors
    ///
    /// Rejects impossible start, parent-exit, terminality, and termination combinations.
    pub fn new(
        attempt_start: TrustedHostAttemptStartPosture,
        parent_exited: bool,
        terminality: TrustedHostProcessTreeTerminality,
        cancellation_or_timeout_started: bool,
        termination_completed: bool,
    ) -> Result<Self, WorkflowOsError> {
        let coherent = match attempt_start {
            TrustedHostAttemptStartPosture::NotStarted => {
                !parent_exited
                    && terminality == TrustedHostProcessTreeTerminality::NotObserved
                    && !cancellation_or_timeout_started
                    && !termination_completed
            }
            TrustedHostAttemptStartPosture::Started => {
                terminality != TrustedHostProcessTreeTerminality::NotObserved
                    && match terminality {
                        TrustedHostProcessTreeTerminality::Terminal => {
                            parent_exited && termination_completed
                        }
                        TrustedHostProcessTreeTerminality::NonTerminal
                        | TrustedHostProcessTreeTerminality::Uncertain => !termination_completed,
                        TrustedHostProcessTreeTerminality::NotObserved => false,
                    }
            }
            TrustedHostAttemptStartPosture::MayHaveStarted => {
                matches!(
                    terminality,
                    TrustedHostProcessTreeTerminality::NonTerminal
                        | TrustedHostProcessTreeTerminality::Uncertain
                ) && !termination_completed
            }
        };
        if !coherent {
            return Err(live_security_error(
                "observation.process_tree.invalid",
                "trusted-host process-tree observation is invalid",
            ));
        }
        Ok(Self {
            attempt_start,
            parent_exited,
            terminality,
            cancellation_or_timeout_started,
            termination_completed,
        })
    }

    #[must_use]
    /// Returns the attempt-start posture.
    pub const fn attempt_start(&self) -> TrustedHostAttemptStartPosture {
        self.attempt_start
    }

    #[must_use]
    /// Returns complete-tree terminality.
    pub const fn terminality(&self) -> TrustedHostProcessTreeTerminality {
        self.terminality
    }
}

impl TryFrom<TrustedHostProcessTreeObservationDefinition> for TrustedHostProcessTreeObservation {
    type Error = WorkflowOsError;

    fn try_from(value: TrustedHostProcessTreeObservationDefinition) -> Result<Self, Self::Error> {
        Self::new(
            value.attempt_start,
            value.parent_exited,
            value.terminality,
            value.cancellation_or_timeout_started,
            value.termination_completed,
        )
    }
}

/// Hard resource-budget observation for all mandatory categories.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TrustedHostResourceBudgetObservation {
    /// CPU-budget enforcement posture.
    pub cpu: HostedExecutionControlPosture,
    /// Memory-budget enforcement posture.
    pub memory: HostedExecutionControlPosture,
    /// Process-count enforcement posture.
    pub process_count: HostedExecutionControlPosture,
    /// File-descriptor enforcement posture.
    pub file_descriptors: HostedExecutionControlPosture,
    /// Disk-budget enforcement posture.
    pub disk: HostedExecutionControlPosture,
    /// Wall-clock enforcement posture.
    pub wall_clock: HostedExecutionControlPosture,
}

impl TrustedHostResourceBudgetObservation {
    #[must_use]
    /// Returns whether every mandatory budget was enforced.
    pub fn all_enforced(&self) -> bool {
        self.cpu == HostedExecutionControlPosture::Enforced
            && self.memory == HostedExecutionControlPosture::Enforced
            && self.process_count == HostedExecutionControlPosture::Enforced
            && self.file_descriptors == HostedExecutionControlPosture::Enforced
            && self.disk == HostedExecutionControlPosture::Enforced
            && self.wall_clock == HostedExecutionControlPosture::Enforced
    }
}

/// Observation availability for transient protected-state mutation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostTransientWriteObservationPosture {
    /// At least one protected write was observed and denied.
    ObservedAndDenied,
    /// Complete observation found no protected-write attempt.
    NoAttemptObserved,
    /// Protected-write observation was unavailable.
    Unavailable,
    /// Protected-write observation is unsupported.
    Unsupported,
    /// Protected-write behavior cannot be established safely.
    Uncertain,
}

/// Payload-free transient-write and output-quiescence observation.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "TrustedHostTransientWriteObservationDefinition")]
pub struct TrustedHostTransientWriteObservation {
    protected_roots: HostedExecutionControlPosture,
    posture: TrustedHostTransientWriteObservationPosture,
    denied_attempt_count: u32,
    outputs_quiescent: bool,
    retained_artifact_commitment: TrustedHostObservationPosture,
    reference: HostedExecutionReference,
}

/// Deserializable definition for transient-write observations.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TrustedHostTransientWriteObservationDefinition {
    /// Protected-root enforcement posture.
    pub protected_roots: HostedExecutionControlPosture,
    /// Transient-write observation posture.
    pub posture: TrustedHostTransientWriteObservationPosture,
    /// Bounded denied attempt count.
    pub denied_attempt_count: u32,
    /// Whether writable outputs were quiescent after terminality.
    pub outputs_quiescent: bool,
    /// Retained-artifact commitment posture.
    pub retained_artifact_commitment: TrustedHostObservationPosture,
    /// Stable telemetry or denied-action reference.
    pub reference: HostedExecutionReference,
}

impl TrustedHostTransientWriteObservation {
    /// Creates a validated transient-write observation.
    ///
    /// # Errors
    ///
    /// Rejects oversized counts, invalid references, and incoherent denied posture.
    pub fn new(
        definition: TrustedHostTransientWriteObservationDefinition,
    ) -> Result<Self, WorkflowOsError> {
        if definition.denied_attempt_count > SECURITY_EVENT_MAX_COUNT
            || !matches!(
                definition.reference.kind(),
                HostedExecutionReferenceKind::Telemetry
                    | HostedExecutionReferenceKind::DeniedAction
            )
            || match definition.posture {
                TrustedHostTransientWriteObservationPosture::ObservedAndDenied => {
                    definition.denied_attempt_count == 0
                }
                TrustedHostTransientWriteObservationPosture::NoAttemptObserved
                | TrustedHostTransientWriteObservationPosture::Unavailable
                | TrustedHostTransientWriteObservationPosture::Unsupported
                | TrustedHostTransientWriteObservationPosture::Uncertain => {
                    definition.denied_attempt_count != 0
                }
            }
        {
            return Err(live_security_error(
                "observation.transient_write.invalid",
                "trusted-host transient-write observation is invalid",
            ));
        }
        Ok(Self {
            protected_roots: definition.protected_roots,
            posture: definition.posture,
            denied_attempt_count: definition.denied_attempt_count,
            outputs_quiescent: definition.outputs_quiescent,
            retained_artifact_commitment: definition.retained_artifact_commitment,
            reference: definition.reference,
        })
    }
}

impl TryFrom<TrustedHostTransientWriteObservationDefinition>
    for TrustedHostTransientWriteObservation
{
    type Error = WorkflowOsError;

    fn try_from(
        value: TrustedHostTransientWriteObservationDefinition,
    ) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Debug for TrustedHostTransientWriteObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostTransientWriteObservation")
            .field("protected_roots", &self.protected_roots)
            .field("posture", &self.posture)
            .field("denied_attempt_count", &self.denied_attempt_count)
            .field("outputs_quiescent", &self.outputs_quiescent)
            .field(
                "retained_artifact_commitment",
                &self.retained_artifact_commitment,
            )
            .field("reference", &"[REDACTED]")
            .finish()
    }
}

/// Core-owned security requirements for one exact future attempt.
///
/// Assessment orchestration remains private until a separately reviewed
/// composition phase proves an external consumer.
///
/// ```compile_fail
/// use workflow_core::assess_trusted_host_live_execution_security;
/// ```
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "TrustedHostLiveExecutionSecurityRequirementsDefinition")]
pub struct TrustedHostLiveExecutionSecurityRequirements {
    binding: TrustedHostLiveExecutionBinding,
    profile: TrustedHostLiveExecutionSecurityProfile,
    requested_policy_hash: SpecContentHash,
    root_requirements: Vec<TrustedHostRootRequirement>,
    denied_egress_probe_required: bool,
    #[serde(skip_serializing)]
    fingerprint: SpecContentHash,
}

/// Deserializable Core-owned requirements definition.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TrustedHostLiveExecutionSecurityRequirementsDefinition {
    /// Exact future-attempt binding.
    pub binding: TrustedHostLiveExecutionBinding,
    /// Closed versioned security requirements profile.
    pub profile: TrustedHostLiveExecutionSecurityProfile,
    /// Requested policy commitment.
    pub requested_policy_hash: SpecContentHash,
    /// Closed typed root requirements.
    pub root_requirements: Vec<TrustedHostRootRequirement>,
    /// Whether a separate denied-egress proof probe is required.
    pub denied_egress_probe_required: bool,
}

impl TrustedHostLiveExecutionSecurityRequirements {
    /// Creates validated, canonical requirements and their fingerprint.
    ///
    /// # Errors
    ///
    /// Rejects missing, duplicate, or incoherent root requirements.
    pub fn new(
        mut definition: TrustedHostLiveExecutionSecurityRequirementsDefinition,
    ) -> Result<Self, WorkflowOsError> {
        canonicalize_root_requirements(&mut definition.root_requirements)?;
        let fingerprint = requirements_fingerprint(&definition);
        Ok(Self {
            binding: definition.binding,
            profile: definition.profile,
            requested_policy_hash: definition.requested_policy_hash,
            root_requirements: definition.root_requirements,
            denied_egress_probe_required: definition.denied_egress_probe_required,
            fingerprint,
        })
    }

    #[must_use]
    /// Returns the domain-separated requirements fingerprint.
    pub const fn fingerprint(&self) -> &SpecContentHash {
        &self.fingerprint
    }

    #[must_use]
    /// Returns the complete closed requirements profile.
    pub const fn profile(&self) -> TrustedHostLiveExecutionSecurityProfile {
        self.profile
    }

    #[must_use]
    /// Returns canonical typed root requirements.
    pub fn root_requirements(&self) -> &[TrustedHostRootRequirement] {
        &self.root_requirements
    }
}

impl TryFrom<TrustedHostLiveExecutionSecurityRequirementsDefinition>
    for TrustedHostLiveExecutionSecurityRequirements
{
    type Error = WorkflowOsError;

    fn try_from(
        value: TrustedHostLiveExecutionSecurityRequirementsDefinition,
    ) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Debug for TrustedHostLiveExecutionSecurityRequirements {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostLiveExecutionSecurityRequirements")
            .field("binding", &self.binding)
            .field("profile", &self.profile)
            .field("requested_policy_hash", &"[REDACTED]")
            .field("root_requirement_count", &self.root_requirements.len())
            .field(
                "denied_egress_probe_required",
                &self.denied_egress_probe_required,
            )
            .field("fingerprint", &"[REDACTED]")
            .finish()
    }
}

/// Payload-free observations for one exact future provider attempt.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "TrustedHostLiveExecutionSecurityObservationsDefinition")]
pub struct TrustedHostLiveExecutionSecurityObservations {
    binding: TrustedHostLiveExecutionBinding,
    requirements_fingerprint: SpecContentHash,
    provider_id: HostedExecutionProviderId,
    provider_version: HostedExecutionProviderVersion,
    input_manifest: BuildTimeInputManifest,
    root_observations: Vec<TrustedHostRootObservation>,
    effective_policy_hash: SpecContentHash,
    policy_revision_stable: TrustedHostSecurityAffirmation,
    enforcement_mode: HostedExecutionEnforcementMode,
    filesystem_control: HostedExecutionControlPosture,
    process_control: HostedExecutionControlPosture,
    network_control: HostedExecutionControlPosture,
    default_deny_filesystem: TrustedHostSecurityAffirmation,
    host_home_excluded: TrustedHostSecurityAffirmation,
    credential_store_excluded: TrustedHostSecurityAffirmation,
    ambient_temp_excluded: TrustedHostSecurityAffirmation,
    default_deny_network: TrustedHostSecurityAffirmation,
    denied_egress_probe_observed: TrustedHostSecurityAffirmation,
    observation_channel: TrustedHostObservationPosture,
    summary: HostedExecutionObservationSummary,
    process_tree: TrustedHostProcessTreeObservation,
    resource_budgets: TrustedHostResourceBudgetObservation,
    transient_writes: TrustedHostTransientWriteObservation,
    cleanup_posture: HostedExecutionCleanupPosture,
    cleanup_reference: HostedExecutionReference,
}

/// Deserializable payload-free observation definition.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TrustedHostLiveExecutionSecurityObservationsDefinition {
    /// Exact future-attempt binding.
    pub binding: TrustedHostLiveExecutionBinding,
    /// Accepted requirements fingerprint.
    pub requirements_fingerprint: SpecContentHash,
    /// Observed provider identity.
    pub provider_id: HostedExecutionProviderId,
    /// Observed provider version.
    pub provider_version: HostedExecutionProviderVersion,
    /// Claimed build-time input manifest.
    pub input_manifest: BuildTimeInputManifest,
    /// Typed root observations.
    pub root_observations: Vec<TrustedHostRootObservation>,
    /// Observed effective policy commitment.
    pub effective_policy_hash: SpecContentHash,
    /// Whether the policy revision remained stable.
    pub policy_revision_stable: TrustedHostSecurityAffirmation,
    /// Observed enforcement mode.
    pub enforcement_mode: HostedExecutionEnforcementMode,
    /// Filesystem control posture.
    pub filesystem_control: HostedExecutionControlPosture,
    /// Process control posture.
    pub process_control: HostedExecutionControlPosture,
    /// Network control posture.
    pub network_control: HostedExecutionControlPosture,
    /// Whether filesystem access was default deny.
    pub default_deny_filesystem: TrustedHostSecurityAffirmation,
    /// Whether host-home access was excluded.
    pub host_home_excluded: TrustedHostSecurityAffirmation,
    /// Whether credential-store access was excluded.
    pub credential_store_excluded: TrustedHostSecurityAffirmation,
    /// Whether ambient temporary paths were excluded.
    pub ambient_temp_excluded: TrustedHostSecurityAffirmation,
    /// Whether network access was default deny.
    pub default_deny_network: TrustedHostSecurityAffirmation,
    /// Whether the separate denied-egress probe was observed.
    pub denied_egress_probe_observed: TrustedHostSecurityAffirmation,
    /// Completeness and certainty of observation telemetry.
    pub observation_channel: TrustedHostObservationPosture,
    /// Existing bounded hosted observation summary.
    pub summary: HostedExecutionObservationSummary,
    /// Full-process-tree observation.
    pub process_tree: TrustedHostProcessTreeObservation,
    /// Resource-budget observations.
    pub resource_budgets: TrustedHostResourceBudgetObservation,
    /// Transient-write observations.
    pub transient_writes: TrustedHostTransientWriteObservation,
    /// Cleanup posture after terminality.
    pub cleanup_posture: HostedExecutionCleanupPosture,
    /// Stable cleanup telemetry reference.
    pub cleanup_reference: HostedExecutionReference,
}

impl TrustedHostLiveExecutionSecurityObservations {
    /// Creates validated observations without asserting their truth or satisfaction.
    ///
    /// # Errors
    ///
    /// Rejects duplicate roots, invalid references, oversized counts, and
    /// incoherent attempt-start facts.
    pub fn new(
        mut definition: TrustedHostLiveExecutionSecurityObservationsDefinition,
    ) -> Result<Self, WorkflowOsError> {
        canonicalize_root_observations(&mut definition.root_observations)?;
        if definition.cleanup_reference.kind() != HostedExecutionReferenceKind::Telemetry
            || security_counts(&definition.summary)
                .iter()
                .any(|count| *count > SECURITY_EVENT_MAX_COUNT)
        {
            return Err(live_security_error(
                "observation.summary.invalid",
                "trusted-host security observation summary is invalid",
            ));
        }
        match definition.process_tree.attempt_start {
            TrustedHostAttemptStartPosture::NotStarted
                if definition.summary.process_start_events() != 0
                    || definition.summary.process_terminal_events() != 0 =>
            {
                return Err(live_security_error(
                    "observation.attempt_start.invalid",
                    "trusted-host attempt-start observation is invalid",
                ));
            }
            TrustedHostAttemptStartPosture::Started
                if definition.summary.process_start_events() == 0
                    || (definition.process_tree.terminality
                        == TrustedHostProcessTreeTerminality::Terminal
                        && definition.summary.process_terminal_events() == 0) =>
            {
                return Err(live_security_error(
                    "observation.attempt_start.invalid",
                    "trusted-host attempt-start observation is invalid",
                ));
            }
            _ => {}
        }
        Ok(Self {
            binding: definition.binding,
            requirements_fingerprint: definition.requirements_fingerprint,
            provider_id: definition.provider_id,
            provider_version: definition.provider_version,
            input_manifest: definition.input_manifest,
            root_observations: definition.root_observations,
            effective_policy_hash: definition.effective_policy_hash,
            policy_revision_stable: definition.policy_revision_stable,
            enforcement_mode: definition.enforcement_mode,
            filesystem_control: definition.filesystem_control,
            process_control: definition.process_control,
            network_control: definition.network_control,
            default_deny_filesystem: definition.default_deny_filesystem,
            host_home_excluded: definition.host_home_excluded,
            credential_store_excluded: definition.credential_store_excluded,
            ambient_temp_excluded: definition.ambient_temp_excluded,
            default_deny_network: definition.default_deny_network,
            denied_egress_probe_observed: definition.denied_egress_probe_observed,
            observation_channel: definition.observation_channel,
            summary: definition.summary,
            process_tree: definition.process_tree,
            resource_budgets: definition.resource_budgets,
            transient_writes: definition.transient_writes,
            cleanup_posture: definition.cleanup_posture,
            cleanup_reference: definition.cleanup_reference,
        })
    }
}

impl TryFrom<TrustedHostLiveExecutionSecurityObservationsDefinition>
    for TrustedHostLiveExecutionSecurityObservations
{
    type Error = WorkflowOsError;

    fn try_from(
        value: TrustedHostLiveExecutionSecurityObservationsDefinition,
    ) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Debug for TrustedHostLiveExecutionSecurityObservations {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostLiveExecutionSecurityObservations")
            .field("binding", &self.binding)
            .field("requirements_fingerprint", &"[REDACTED]")
            .field("provider_id", &"[REDACTED]")
            .field("provider_version", &"[REDACTED]")
            .field("input_manifest", &self.input_manifest)
            .field("root_observation_count", &self.root_observations.len())
            .field("effective_policy_hash", &"[REDACTED]")
            .field("attempt_start", &self.process_tree.attempt_start)
            .field("cleanup_posture", &self.cleanup_posture)
            .field("cleanup_reference", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

/// Stable reasons an exact observation set does not satisfy requirements.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostLiveExecutionSecurityReason {
    /// Build-time input closure was not complete.
    InputClosure,
    /// Root observations did not satisfy the typed requirements.
    RootBinding,
    /// Requested and effective policy commitments differed.
    PolicyMismatch,
    /// The effective policy revision changed during the attempt.
    PolicyDrift,
    /// The provider did not use enforce mode.
    EnforcementMode,
    /// Filesystem containment was insufficient.
    FilesystemControl,
    /// Process containment was insufficient.
    ProcessControl,
    /// Network containment was insufficient.
    NetworkControl,
    /// Required observation telemetry was not proven complete.
    ObservationUnavailable,
    /// Allowed network activity was observed.
    NetworkActivity,
    /// A required denied-egress proof probe was missing.
    EgressProbeMissing,
    /// The complete process tree was not terminal.
    ProcessTreeNotTerminal,
    /// At least one mandatory resource budget was not enforced.
    ResourceBudget,
    /// Transient-write or output-quiescence posture was insufficient.
    TransientWrite,
    /// Cleanup did not complete deterministically.
    CleanupIncomplete,
}

/// Closed assessment disposition. A satisfied result grants no authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedHostLiveExecutionSecurityDisposition {
    /// Every mandatory requirement was satisfied for the exact attempt.
    Satisfied,
    /// Requirements failed with affirmative evidence that execution did not start.
    DeniedBeforeStart,
    /// Execution started or may have started and requires reconciliation.
    ReconciliationRequired,
}

/// Pure, payload-free comparison result for one exact attempt.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "TrustedHostLiveExecutionSecurityAssessmentDefinition")]
pub struct TrustedHostLiveExecutionSecurityAssessment {
    disposition: TrustedHostLiveExecutionSecurityDisposition,
    reasons: Vec<TrustedHostLiveExecutionSecurityReason>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct TrustedHostLiveExecutionSecurityAssessmentDefinition {
    disposition: TrustedHostLiveExecutionSecurityDisposition,
    reasons: Vec<TrustedHostLiveExecutionSecurityReason>,
}

impl TrustedHostLiveExecutionSecurityAssessment {
    fn new(
        disposition: TrustedHostLiveExecutionSecurityDisposition,
        mut reasons: Vec<TrustedHostLiveExecutionSecurityReason>,
    ) -> Result<Self, WorkflowOsError> {
        reasons.sort_unstable();
        let had_duplicates = reasons.windows(2).any(|pair| pair[0] == pair[1]);
        let coherent = match disposition {
            TrustedHostLiveExecutionSecurityDisposition::Satisfied => reasons.is_empty(),
            TrustedHostLiveExecutionSecurityDisposition::DeniedBeforeStart
            | TrustedHostLiveExecutionSecurityDisposition::ReconciliationRequired => {
                !reasons.is_empty()
            }
        };
        if had_duplicates || !coherent {
            return Err(live_security_error(
                "assessment.invalid",
                "trusted-host live-security assessment is invalid",
            ));
        }
        Ok(Self {
            disposition,
            reasons,
        })
    }

    #[must_use]
    /// Returns the assessment disposition.
    pub const fn disposition(&self) -> TrustedHostLiveExecutionSecurityDisposition {
        self.disposition
    }

    #[must_use]
    /// Returns canonical stable reasons.
    pub fn reasons(&self) -> &[TrustedHostLiveExecutionSecurityReason] {
        &self.reasons
    }

    /// Returns whether observations satisfy requirements. This is evidence, not authority.
    #[must_use]
    pub fn is_satisfied(&self) -> bool {
        self.disposition == TrustedHostLiveExecutionSecurityDisposition::Satisfied
    }
}

impl TryFrom<TrustedHostLiveExecutionSecurityAssessmentDefinition>
    for TrustedHostLiveExecutionSecurityAssessment
{
    type Error = WorkflowOsError;

    fn try_from(
        value: TrustedHostLiveExecutionSecurityAssessmentDefinition,
    ) -> Result<Self, Self::Error> {
        Self::new(value.disposition, value.reasons)
    }
}

/// Compares one exact observation set with Core-owned requirements.
///
/// This pure function performs no I/O and grants no execution authority.
///
/// # Errors
///
/// Rejects request, attempt, provider-selection, or requirements substitution.
#[allow(dead_code)]
pub(crate) fn assess_trusted_host_live_execution_security(
    requirements: &TrustedHostLiveExecutionSecurityRequirements,
    observations: &TrustedHostLiveExecutionSecurityObservations,
) -> Result<TrustedHostLiveExecutionSecurityAssessment, WorkflowOsError> {
    if requirements.binding != observations.binding {
        return Err(live_security_error(
            "assessment.binding.substituted",
            "trusted-host live-security binding does not match",
        ));
    }
    if requirements.fingerprint != observations.requirements_fingerprint {
        return Err(live_security_error(
            "assessment.requirements.substituted",
            "trusted-host live-security requirements do not match",
        ));
    }

    match requirements.profile {
        TrustedHostLiveExecutionSecurityProfile::StrictBuildExecutionV1 => {}
    }

    let mut reasons = BTreeSet::new();
    if observations.input_manifest.completeness != BuildTimeInputCompletenessPosture::Complete {
        reasons.insert(TrustedHostLiveExecutionSecurityReason::InputClosure);
    }
    assess_roots(requirements, observations, &mut reasons);
    if requirements.requested_policy_hash != observations.effective_policy_hash {
        reasons.insert(TrustedHostLiveExecutionSecurityReason::PolicyMismatch);
    }
    if observations.policy_revision_stable != TrustedHostSecurityAffirmation::Affirmed
        || observations.summary.policy_change_events() != 0
    {
        reasons.insert(TrustedHostLiveExecutionSecurityReason::PolicyDrift);
    }
    if observations.enforcement_mode != HostedExecutionEnforcementMode::Enforce {
        reasons.insert(TrustedHostLiveExecutionSecurityReason::EnforcementMode);
    }
    if observations.filesystem_control != HostedExecutionControlPosture::Enforced
        || observations.default_deny_filesystem != TrustedHostSecurityAffirmation::Affirmed
        || observations.host_home_excluded != TrustedHostSecurityAffirmation::Affirmed
        || observations.credential_store_excluded != TrustedHostSecurityAffirmation::Affirmed
        || observations.ambient_temp_excluded != TrustedHostSecurityAffirmation::Affirmed
    {
        reasons.insert(TrustedHostLiveExecutionSecurityReason::FilesystemControl);
    }
    if observations.process_control != HostedExecutionControlPosture::Enforced {
        reasons.insert(TrustedHostLiveExecutionSecurityReason::ProcessControl);
    }
    if observations.network_control != HostedExecutionControlPosture::Enforced
        || observations.default_deny_network != TrustedHostSecurityAffirmation::Affirmed
    {
        reasons.insert(TrustedHostLiveExecutionSecurityReason::NetworkControl);
    }
    if observations.observation_channel != TrustedHostObservationPosture::Proven {
        reasons.insert(TrustedHostLiveExecutionSecurityReason::ObservationUnavailable);
    }
    if observations.summary.allowed_network_events() != 0 {
        reasons.insert(TrustedHostLiveExecutionSecurityReason::NetworkActivity);
    }
    if requirements.denied_egress_probe_required
        && observations.denied_egress_probe_observed != TrustedHostSecurityAffirmation::Affirmed
    {
        reasons.insert(TrustedHostLiveExecutionSecurityReason::EgressProbeMissing);
    }
    if observations.process_tree.terminality != TrustedHostProcessTreeTerminality::Terminal {
        reasons.insert(TrustedHostLiveExecutionSecurityReason::ProcessTreeNotTerminal);
    }
    if !observations.resource_budgets.all_enforced() {
        reasons.insert(TrustedHostLiveExecutionSecurityReason::ResourceBudget);
    }
    if observations.transient_writes.protected_roots != HostedExecutionControlPosture::Enforced
        || !matches!(
            observations.transient_writes.posture,
            TrustedHostTransientWriteObservationPosture::ObservedAndDenied
                | TrustedHostTransientWriteObservationPosture::NoAttemptObserved
        )
        || !observations.transient_writes.outputs_quiescent
        || observations.transient_writes.retained_artifact_commitment
            != TrustedHostObservationPosture::Proven
    {
        reasons.insert(TrustedHostLiveExecutionSecurityReason::TransientWrite);
    }
    if observations.cleanup_posture != HostedExecutionCleanupPosture::Completed {
        reasons.insert(TrustedHostLiveExecutionSecurityReason::CleanupIncomplete);
    }

    let reasons: Vec<_> = reasons.into_iter().collect();
    let disposition = if reasons.is_empty() {
        TrustedHostLiveExecutionSecurityDisposition::Satisfied
    } else {
        match observations.process_tree.attempt_start {
            TrustedHostAttemptStartPosture::NotStarted => {
                TrustedHostLiveExecutionSecurityDisposition::DeniedBeforeStart
            }
            TrustedHostAttemptStartPosture::Started
            | TrustedHostAttemptStartPosture::MayHaveStarted => {
                TrustedHostLiveExecutionSecurityDisposition::ReconciliationRequired
            }
        }
    };
    TrustedHostLiveExecutionSecurityAssessment::new(disposition, reasons)
}

#[allow(dead_code)]
fn assess_roots(
    requirements: &TrustedHostLiveExecutionSecurityRequirements,
    observations: &TrustedHostLiveExecutionSecurityObservations,
    reasons: &mut BTreeSet<TrustedHostLiveExecutionSecurityReason>,
) {
    for required in &requirements.root_requirements {
        let Some(observed) = observations
            .root_observations
            .iter()
            .find(|value| value.role == required.role)
        else {
            reasons.insert(TrustedHostLiveExecutionSecurityReason::RootBinding);
            continue;
        };
        if observed.access != required.access
            || observed.object_binding != TrustedHostObservationPosture::Proven
            || observed.continuity != TrustedHostObservationPosture::Proven
            || (required.binding == TrustedHostRootBindingRequirement::ProviderImmutableProjection
                && observed.method != TrustedHostRootObservationMethod::ImmutableProjection)
            || (required.invocation_scoped_creation && !observed.invocation_scoped)
        {
            reasons.insert(TrustedHostLiveExecutionSecurityReason::RootBinding);
        }
    }
    if observations.root_observations.len() != requirements.root_requirements.len() {
        reasons.insert(TrustedHostLiveExecutionSecurityReason::RootBinding);
    }
}

fn canonicalize_root_requirements(
    values: &mut [TrustedHostRootRequirement],
) -> Result<(), WorkflowOsError> {
    values.sort_by_key(|value| value.role);
    if values.len() != ROOT_ROLE_COUNT
        || values.windows(2).any(|pair| pair[0].role == pair[1].role)
        || values.iter().any(|value| {
            let writable = matches!(
                value.role,
                TrustedHostRootRole::CacheWrite
                    | TrustedHostRootRole::BuildOutputWrite
                    | TrustedHostRootRole::TemporaryWrite
            );
            (writable && value.access != TrustedHostRootAccess::ReadWrite)
                || (!writable && value.access == TrustedHostRootAccess::ReadWrite)
                || (value.invocation_scoped_creation != writable)
        })
    {
        return Err(live_security_error(
            "requirements.root_binding.invalid",
            "trusted-host root requirements are invalid",
        ));
    }
    Ok(())
}

fn canonicalize_root_observations(
    values: &mut [TrustedHostRootObservation],
) -> Result<(), WorkflowOsError> {
    values.sort_by_key(|value| value.role);
    let unique_commitments: BTreeSet<_> = values
        .iter()
        .map(|value| value.commitment.as_str())
        .collect();
    if values.len() > ROOT_ROLE_COUNT
        || values.windows(2).any(|pair| pair[0].role == pair[1].role)
        || unique_commitments.len() != values.len()
    {
        return Err(live_security_error(
            "observation.root_binding.invalid",
            "trusted-host root observations are invalid",
        ));
    }
    Ok(())
}

fn requirements_fingerprint(
    definition: &TrustedHostLiveExecutionSecurityRequirementsDefinition,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "domain", REQUIREMENTS_DOMAIN);
    hash_hash(&mut hasher, "run_bundle", &definition.binding.run_bundle);
    hash_field(
        &mut hasher,
        "security_profile",
        security_profile_tag(definition.profile),
    );
    hash_hash(
        &mut hasher,
        "command_contract",
        &definition.binding.command_contract,
    );
    hash_hash(&mut hasher, "attempt", &definition.binding.attempt);
    hash_hash(&mut hasher, "request", &definition.binding.request);
    hash_hash(
        &mut hasher,
        "provider_selection",
        &definition.binding.provider_selection,
    );
    hash_hash(
        &mut hasher,
        "requested_policy_hash",
        &definition.requested_policy_hash,
    );
    hash_field(
        &mut hasher,
        "denied_egress_probe_required",
        if definition.denied_egress_probe_required {
            "true"
        } else {
            "false"
        },
    );
    for requirement in &definition.root_requirements {
        hash_field(&mut hasher, "root_role", root_role_tag(requirement.role));
        hash_field(
            &mut hasher,
            "root_access",
            root_access_tag(requirement.access),
        );
        hash_field(
            &mut hasher,
            "root_binding",
            root_binding_tag(requirement.binding),
        );
        hash_field(
            &mut hasher,
            "root_invocation_scoped",
            if requirement.invocation_scoped_creation {
                "true"
            } else {
                "false"
            },
        );
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_hash(hasher: &mut Sha256, label: &str, value: &SpecContentHash) {
    hash_field(hasher, label, value.as_str());
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update(label.len().to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(value.len().to_be_bytes());
    hasher.update(value.as_bytes());
}

const fn root_role_tag(value: TrustedHostRootRole) -> &'static str {
    match value {
        TrustedHostRootRole::RepositoryInput => "repository_input",
        TrustedHostRootRole::ToolchainInput => "toolchain_input",
        TrustedHostRootRole::DependencyInput => "dependency_input",
        TrustedHostRootRole::CacheWrite => "cache_write",
        TrustedHostRootRole::BuildOutputWrite => "build_output_write",
        TrustedHostRootRole::TemporaryWrite => "temporary_write",
    }
}

const fn security_profile_tag(value: TrustedHostLiveExecutionSecurityProfile) -> &'static str {
    match value {
        TrustedHostLiveExecutionSecurityProfile::StrictBuildExecutionV1 => {
            "strict_build_execution_v1"
        }
    }
}

const fn root_access_tag(value: TrustedHostRootAccess) -> &'static str {
    match value {
        TrustedHostRootAccess::ReadOnly => "read_only",
        TrustedHostRootAccess::ReadExecute => "read_execute",
        TrustedHostRootAccess::ReadWrite => "read_write",
    }
}

const fn root_binding_tag(value: TrustedHostRootBindingRequirement) -> &'static str {
    match value {
        TrustedHostRootBindingRequirement::AdmissionToTerminal => "admission_to_terminal",
        TrustedHostRootBindingRequirement::ProviderImmutableProjection => {
            "provider_immutable_projection"
        }
    }
}

fn security_counts(summary: &HostedExecutionObservationSummary) -> [u32; 6] {
    [
        summary.allowed_network_events(),
        summary.denied_network_events(),
        summary.process_start_events(),
        summary.process_terminal_events(),
        summary.policy_change_events(),
        summary.security_findings(),
    ]
}

fn live_security_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("trusted_host.live_security.{suffix}"), message)
}

#[cfg(test)]
#[path = "trusted_host_live_execution_security_tests.rs"]
mod tests;
