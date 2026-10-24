//! Private, process-free contracts for the local Cargo input observer.
//!
//! The module owns no concrete filesystem or process implementation. Callers
//! must inject closed sources, and the observer returns non-authorizing Core
//! observations plus an exact in-memory evidence bundle.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fmt;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use sha2::{Digest, Sha256};

#[cfg(unix)]
use nix::sys::signal::{killpg, Signal};
#[cfg(unix)]
use nix::unistd::Pid;
#[cfg(unix)]
use std::os::unix::process::CommandExt;

use workflow_core::{
    HostedExecutionReference, HostedExecutionReferenceKind, SpecContentHash,
    TrustedHostBuildInputHostObligationKind, TrustedHostInputObservationFact,
    TrustedHostInputObservationFactDefinition, TrustedHostInputObservationMethod,
    TrustedHostInputObservationPosture, TrustedHostInputObservationProducer,
    TrustedHostInputObservationProducerAssurance, TrustedHostInputObservationProducerId,
    TrustedHostInputObservationProducerKind, TrustedHostInputObservationProducerVersion,
    TrustedHostInputObservationReason, TrustedHostInputObservationRequest,
    TrustedHostInputObservationResult, TrustedHostInputObservationResultDefinition,
    TrustedHostObservationPosture, TrustedHostRootAccess, TrustedHostRootObservation,
    TrustedHostRootObservationDefinition, TrustedHostRootObservationMethod, TrustedHostRootRole,
    WorkflowOsError,
};

const REQUIRED_FACT_KINDS: [TrustedHostBuildInputHostObligationKind; 9] = [
    TrustedHostBuildInputHostObligationKind::ToolchainExecutable,
    TrustedHostBuildInputHostObligationKind::TargetSpecification,
    TrustedHostBuildInputHostObligationKind::SdkOrSysroot,
    TrustedHostBuildInputHostObligationKind::LinkerOrArchiver,
    TrustedHostBuildInputHostObligationKind::NativeCompilerTools,
    TrustedHostBuildInputHostObligationKind::DynamicLibraries,
    TrustedHostBuildInputHostObligationKind::RegistryAndPackageSources,
    TrustedHostBuildInputHostObligationKind::Environment,
    TrustedHostBuildInputHostObligationKind::RuntimeBase,
];

const MAX_PROBE_STREAM_BYTES: usize = 256 * 1024;
const MAX_EXECUTABLE_BYTES: u64 = 256 * 1024 * 1024;
const MAX_ROOT_FILES: usize = 20_000;
const MAX_ROOT_BYTES: u64 = 512 * 1024 * 1024;
const FIXED_PROBE_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Clone, Eq, PartialEq)]
struct LocalCargoObservationDefinition {
    selection: SpecContentHash,
    operation_contract: SpecContentHash,
    security_requirements: SpecContentHash,
}

impl LocalCargoObservationDefinition {
    fn new(
        request: &TrustedHostInputObservationRequest,
        selection: SpecContentHash,
        operation_contract: SpecContentHash,
        security_requirements: SpecContentHash,
    ) -> Result<Self, WorkflowOsError> {
        if &selection != request.selection()
            || &operation_contract != request.operation_contract()
            || &security_requirements != request.security_requirements()
        {
            return Err(observer_error(
                "definition.request_mismatch",
                "local Cargo observation definition does not match its request",
            ));
        }
        Ok(Self {
            selection,
            operation_contract,
            security_requirements,
        })
    }
}

impl fmt::Debug for LocalCargoObservationDefinition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCargoObservationDefinition")
            .field("selection", &"[REDACTED]")
            .field("operation_contract", &"[REDACTED]")
            .field("security_requirements", &"[REDACTED]")
            .finish()
    }
}

trait LocalCargoFilesystemSource {
    fn repository_root(
        &self,
        request: &TrustedHostInputObservationRequest,
        definition: &LocalCargoObservationDefinition,
    ) -> Result<LocalCargoRootSourceObservation, WorkflowOsError>;
}

trait LocalCargoProcessSource {
    fn observe(
        &self,
        kind: TrustedHostBuildInputHostObligationKind,
        request: &TrustedHostInputObservationRequest,
        definition: &LocalCargoObservationDefinition,
    ) -> Result<LocalCargoFactSourceObservation, WorkflowOsError>;
}

trait LocalCargoRevisionSource {
    fn next_revision(
        &mut self,
        ordering_authority: &SpecContentHash,
    ) -> Result<u64, WorkflowOsError>;
}

#[derive(Clone, Eq, PartialEq)]
struct LocalCargoRootSourceObservation {
    object_binding: TrustedHostObservationPosture,
    continuity: TrustedHostObservationPosture,
    method: TrustedHostRootObservationMethod,
    commitment: SpecContentHash,
}

impl LocalCargoRootSourceObservation {
    fn new(
        object_binding: TrustedHostObservationPosture,
        continuity: TrustedHostObservationPosture,
        method: TrustedHostRootObservationMethod,
        commitment: SpecContentHash,
    ) -> Result<Self, WorkflowOsError> {
        if object_binding == TrustedHostObservationPosture::Proven
            || continuity == TrustedHostObservationPosture::Proven
        {
            return Err(observer_error(
                "root.posture.invalid",
                "local Cargo root observation exceeds the standalone posture ceiling",
            ));
        }
        Ok(Self {
            object_binding,
            continuity,
            method,
            commitment,
        })
    }
}

impl fmt::Debug for LocalCargoRootSourceObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCargoRootSourceObservation")
            .field("object_binding", &self.object_binding)
            .field("continuity", &self.continuity)
            .field("method", &self.method)
            .field("commitment", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
struct LocalCargoFactSourceObservation {
    kind: TrustedHostBuildInputHostObligationKind,
    posture: TrustedHostInputObservationPosture,
    reason: TrustedHostInputObservationReason,
    method: TrustedHostInputObservationMethod,
    object_binding: TrustedHostObservationPosture,
    continuity: TrustedHostObservationPosture,
    commitment: SpecContentHash,
    item_count: u32,
}

impl LocalCargoFactSourceObservation {
    #[allow(clippy::too_many_arguments)]
    fn new(
        kind: TrustedHostBuildInputHostObligationKind,
        posture: TrustedHostInputObservationPosture,
        reason: TrustedHostInputObservationReason,
        method: TrustedHostInputObservationMethod,
        object_binding: TrustedHostObservationPosture,
        continuity: TrustedHostObservationPosture,
        commitment: SpecContentHash,
        item_count: u32,
    ) -> Result<Self, WorkflowOsError> {
        if posture == TrustedHostInputObservationPosture::Observed
            || object_binding == TrustedHostObservationPosture::Proven
            || continuity == TrustedHostObservationPosture::Proven
            || !posture_allowed_for_kind(kind, posture)
        {
            return Err(observer_error(
                "fact.posture.invalid",
                "local Cargo fact exceeds its standalone posture ceiling",
            ));
        }
        Ok(Self {
            kind,
            posture,
            reason,
            method,
            object_binding,
            continuity,
            commitment,
            item_count,
        })
    }
}

impl fmt::Debug for LocalCargoFactSourceObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCargoFactSourceObservation")
            .field("kind", &self.kind)
            .field("posture", &self.posture)
            .field("reason", &self.reason)
            .field("method", &self.method)
            .field("object_binding", &self.object_binding)
            .field("continuity", &self.continuity)
            .field("commitment", &"[REDACTED]")
            .field("item_count", &self.item_count)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum LocalCargoEvidenceClass {
    RepositoryRoot,
    Fact(TrustedHostBuildInputHostObligationKind),
}

#[derive(Clone, Eq, PartialEq)]
struct LocalCargoObservationEvidenceEntry {
    reference: HostedExecutionReference,
    class: LocalCargoEvidenceClass,
    request_fingerprint: SpecContentHash,
    producer_id: TrustedHostInputObservationProducerId,
    producer_version: TrustedHostInputObservationProducerVersion,
    completed_revision: u64,
    commitment: SpecContentHash,
}

impl fmt::Debug for LocalCargoObservationEvidenceEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCargoObservationEvidenceEntry")
            .field("reference", &"[REDACTED]")
            .field("class", &self.class)
            .field("request_fingerprint", &"[REDACTED]")
            .field("producer_id", &self.producer_id)
            .field("producer_version", &self.producer_version)
            .field("completed_revision", &self.completed_revision)
            .field("commitment", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
struct LocalCargoObservationEvidenceBundle {
    request_fingerprint: SpecContentHash,
    result_fingerprint: SpecContentHash,
    completed_revision: u64,
    entries: Vec<LocalCargoObservationEvidenceEntry>,
}

impl LocalCargoObservationEvidenceBundle {
    fn new(
        request: &TrustedHostInputObservationRequest,
        result: &TrustedHostInputObservationResult,
        completed_revision: u64,
        mut entries: Vec<LocalCargoObservationEvidenceEntry>,
    ) -> Result<Self, WorkflowOsError> {
        entries.sort_unstable_by_key(|entry| entry.class);
        let bundle = Self {
            request_fingerprint: request.fingerprint().clone(),
            result_fingerprint: result.fingerprint().clone(),
            completed_revision,
            entries,
        };
        bundle.validate(request, result)?;
        Ok(bundle)
    }

    fn validate(
        &self,
        request: &TrustedHostInputObservationRequest,
        result: &TrustedHostInputObservationResult,
    ) -> Result<(), WorkflowOsError> {
        let expected = std::iter::once((
            LocalCargoEvidenceClass::RepositoryRoot,
            result.repository_root().reference(),
            result.repository_root().commitment(),
        ))
        .chain(result.facts().iter().filter_map(|fact| {
            fact.reference().map(|reference| {
                (
                    LocalCargoEvidenceClass::Fact(fact.kind()),
                    reference,
                    fact.commitment(),
                )
            })
        }))
        .collect::<Vec<_>>();

        let references = self
            .entries
            .iter()
            .map(|entry| entry.reference.value())
            .collect::<BTreeSet<_>>();
        let classes = self
            .entries
            .iter()
            .map(|entry| entry.class)
            .collect::<BTreeSet<_>>();
        if self.request_fingerprint != *request.fingerprint()
            || self.result_fingerprint != *result.fingerprint()
            || self.completed_revision != result.completed_revision()
            || self.entries.len() != expected.len()
            || references.len() != self.entries.len()
            || classes.len() != self.entries.len()
            || self.entries.iter().any(|entry| {
                entry.request_fingerprint != *request.fingerprint()
                    || entry.producer_id != *result.producer().id()
                    || entry.producer_version != *result.producer().version()
                    || entry.completed_revision != self.completed_revision
                    || !expected.iter().any(|(class, reference, commitment)| {
                        *class == entry.class
                            && *reference == &entry.reference
                            && *commitment == &entry.commitment
                    })
            })
        {
            return Err(observer_error(
                "evidence_bundle.invalid",
                "local Cargo observation evidence bundle is invalid",
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for LocalCargoObservationEvidenceBundle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCargoObservationEvidenceBundle")
            .field("request_fingerprint", &"[REDACTED]")
            .field("result_fingerprint", &"[REDACTED]")
            .field("completed_revision", &self.completed_revision)
            .field("entry_count", &self.entries.len())
            .finish()
    }
}

struct LocalCargoObservationOutput {
    result: TrustedHostInputObservationResult,
    evidence: LocalCargoObservationEvidenceBundle,
}

struct LocalCargoObservationAssembly {
    producer: TrustedHostInputObservationProducer,
    started_revision: u64,
    completed_revision: u64,
    root: LocalCargoRootSourceObservation,
    facts: Vec<LocalCargoFactSourceObservation>,
}

impl LocalCargoObservationAssembly {
    fn into_output(
        self,
        request: &TrustedHostInputObservationRequest,
    ) -> Result<LocalCargoObservationOutput, WorkflowOsError> {
        let root_entry = evidence_entry(
            request,
            &self.producer,
            self.completed_revision,
            LocalCargoEvidenceClass::RepositoryRoot,
            self.root.commitment.clone(),
        )?;
        let repository_root =
            TrustedHostRootObservation::new(TrustedHostRootObservationDefinition {
                role: TrustedHostRootRole::RepositoryInput,
                access: TrustedHostRootAccess::ReadOnly,
                object_binding: self.root.object_binding,
                continuity: self.root.continuity,
                method: self.root.method,
                invocation_scoped: false,
                commitment: self.root.commitment,
                reference: root_entry.reference.clone(),
            })?;

        let mut entries = vec![root_entry];
        let facts = self
            .facts
            .into_iter()
            .map(|source| {
                let entry = evidence_entry(
                    request,
                    &self.producer,
                    self.completed_revision,
                    LocalCargoEvidenceClass::Fact(source.kind),
                    source.commitment.clone(),
                )?;
                let fact = TrustedHostInputObservationFact::new(
                    TrustedHostInputObservationFactDefinition {
                        producer_id: self.producer.id().clone(),
                        producer_version: self.producer.version().clone(),
                        kind: source.kind,
                        posture: source.posture,
                        reason: Some(source.reason),
                        method: source.method,
                        object_binding: source.object_binding,
                        continuity: source.continuity,
                        commitment: source.commitment,
                        item_count: source.item_count,
                        reference: Some(entry.reference.clone()),
                    },
                )?;
                entries.push(entry);
                Ok(fact)
            })
            .collect::<Result<Vec<_>, WorkflowOsError>>()?;

        let result =
            TrustedHostInputObservationResult::new(TrustedHostInputObservationResultDefinition {
                request_fingerprint: request.fingerprint().clone(),
                repository_tree: request.repository_tree().clone(),
                ordering_authority: request.ordering_authority().clone(),
                producer: self.producer,
                started_revision: self.started_revision,
                completed_revision: self.completed_revision,
                repository_root,
                facts,
            })?;
        result.validate_for_request(request)?;
        let evidence = LocalCargoObservationEvidenceBundle::new(
            request,
            &result,
            self.completed_revision,
            entries,
        )?;
        Ok(LocalCargoObservationOutput { result, evidence })
    }
}

impl fmt::Debug for LocalCargoObservationOutput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCargoObservationOutput")
            .field("result", &self.result)
            .field("evidence", &self.evidence)
            .finish()
    }
}

struct LocalCargoHostObserver<F, P, R> {
    producer: TrustedHostInputObservationProducer,
    filesystem: F,
    process: P,
    revisions: R,
}

impl<F, P, R> LocalCargoHostObserver<F, P, R>
where
    F: LocalCargoFilesystemSource,
    P: LocalCargoProcessSource,
    R: LocalCargoRevisionSource,
{
    fn new(
        producer: TrustedHostInputObservationProducer,
        filesystem: F,
        process: P,
        revisions: R,
    ) -> Result<Self, WorkflowOsError> {
        if producer.kind() != TrustedHostInputObservationProducerKind::HostObserver
            || producer.assurance()
                != TrustedHostInputObservationProducerAssurance::IndependentlyReviewed
        {
            return Err(observer_error(
                "producer.invalid",
                "local Cargo observer producer is invalid",
            ));
        }
        Ok(Self {
            producer,
            filesystem,
            process,
            revisions,
        })
    }

    fn observe(
        &mut self,
        request: &TrustedHostInputObservationRequest,
        definition: &LocalCargoObservationDefinition,
    ) -> Result<LocalCargoObservationOutput, WorkflowOsError> {
        self.validate_binding(request, definition)?;

        let started_revision = self.revisions.next_revision(request.ordering_authority())?;
        if started_revision <= request.static_discovery_completed_revision() {
            return Err(observer_error(
                "revision.start.invalid",
                "local Cargo observation start revision is invalid",
            ));
        }

        let root = self.filesystem.repository_root(request, definition)?;
        if root.commitment != *request.repository_tree() {
            return Err(observer_error(
                "root.binding.invalid",
                "local Cargo repository root binding is invalid",
            ));
        }

        let source_facts = self.observe_facts(request, definition)?;

        let completed_revision = self.revisions.next_revision(request.ordering_authority())?;
        if completed_revision <= started_revision {
            return Err(observer_error(
                "revision.completion.invalid",
                "local Cargo observation completion revision is invalid",
            ));
        }

        LocalCargoObservationAssembly {
            producer: self.producer.clone(),
            started_revision,
            completed_revision,
            root,
            facts: source_facts,
        }
        .into_output(request)
    }

    fn validate_binding(
        &self,
        request: &TrustedHostInputObservationRequest,
        definition: &LocalCargoObservationDefinition,
    ) -> Result<(), WorkflowOsError> {
        if definition.selection != *request.selection()
            || definition.operation_contract != *request.operation_contract()
            || definition.security_requirements != *request.security_requirements()
            || self.producer.id() != request.expected_producer_id()
            || self.producer.version() != request.expected_producer_version()
        {
            return Err(observer_error(
                "request.binding.invalid",
                "local Cargo observer request binding is invalid",
            ));
        }
        Ok(())
    }

    fn observe_facts(
        &self,
        request: &TrustedHostInputObservationRequest,
        definition: &LocalCargoObservationDefinition,
    ) -> Result<Vec<LocalCargoFactSourceObservation>, WorkflowOsError> {
        REQUIRED_FACT_KINDS
            .iter()
            .copied()
            .map(|kind| {
                let fact = self.process.observe(kind, request, definition)?;
                if fact.kind != kind {
                    return Err(observer_error(
                        "fact.kind_substituted",
                        "local Cargo observation fact kind is substituted",
                    ));
                }
                Ok(fact)
            })
            .collect()
    }
}

fn posture_allowed_for_kind(
    kind: TrustedHostBuildInputHostObligationKind,
    posture: TrustedHostInputObservationPosture,
) -> bool {
    match kind {
        TrustedHostBuildInputHostObligationKind::ToolchainExecutable
        | TrustedHostBuildInputHostObligationKind::TargetSpecification
        | TrustedHostBuildInputHostObligationKind::SdkOrSysroot
        | TrustedHostBuildInputHostObligationKind::NativeCompilerTools
        | TrustedHostBuildInputHostObligationKind::RegistryAndPackageSources => matches!(
            posture,
            TrustedHostInputObservationPosture::Uncertain
                | TrustedHostInputObservationPosture::Unavailable
        ),
        TrustedHostBuildInputHostObligationKind::LinkerOrArchiver => matches!(
            posture,
            TrustedHostInputObservationPosture::DeclaredOnly
                | TrustedHostInputObservationPosture::Uncertain
                | TrustedHostInputObservationPosture::Unavailable
        ),
        TrustedHostBuildInputHostObligationKind::Environment => matches!(
            posture,
            TrustedHostInputObservationPosture::DeclaredOnly
                | TrustedHostInputObservationPosture::Unavailable
        ),
        TrustedHostBuildInputHostObligationKind::DynamicLibraries
        | TrustedHostBuildInputHostObligationKind::RuntimeBase => {
            posture == TrustedHostInputObservationPosture::Unsupported
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum LocalCargoExecutableRole {
    Cargo,
    Rustc,
    Rustdoc,
    CargoClippy,
    ClippyDriver,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LocalCargoFixedProbe {
    RustcVerboseVersion,
    RustcSysroot,
    RustcTargetLibdir,
    CargoMetadata,
}

#[derive(Clone)]
struct LocalCargoFixedProbeDefinition {
    repository_root: PathBuf,
    cargo_home: PathBuf,
    executables: BTreeMap<LocalCargoExecutableRole, PathBuf>,
    environment: BTreeMap<OsString, OsString>,
    selection: SpecContentHash,
}

impl LocalCargoFixedProbeDefinition {
    fn new(
        repository_root: &Path,
        cargo_home: &Path,
        executables: BTreeMap<LocalCargoExecutableRole, PathBuf>,
    ) -> Result<Self, WorkflowOsError> {
        let expected_roles = [
            LocalCargoExecutableRole::Cargo,
            LocalCargoExecutableRole::Rustc,
            LocalCargoExecutableRole::Rustdoc,
            LocalCargoExecutableRole::CargoClippy,
            LocalCargoExecutableRole::ClippyDriver,
        ];
        if expected_roles
            .iter()
            .any(|role| !executables.contains_key(role))
            || executables.len() != expected_roles.len()
        {
            return Err(observer_error(
                "fixed_definition.executables.invalid",
                "local Cargo fixed executable set is invalid",
            ));
        }
        let repository_root = canonical_directory(repository_root)?;
        let cargo_home = canonical_directory(cargo_home)?;
        let executables = executables
            .into_iter()
            .map(|(role, path)| canonical_regular_file(&path).map(|path| (role, path)))
            .collect::<Result<BTreeMap<_, _>, WorkflowOsError>>()?;
        let environment =
            BTreeMap::from([(OsString::from("CARGO_HOME"), cargo_home.clone().into())]);
        let selection =
            fixed_selection_commitment(&repository_root, &cargo_home, &executables, &environment);
        Ok(Self {
            repository_root,
            cargo_home,
            executables,
            environment,
            selection,
        })
    }

    fn selection(&self) -> &SpecContentHash {
        &self.selection
    }

    fn bind(
        &self,
        request: &TrustedHostInputObservationRequest,
    ) -> Result<LocalCargoObservationDefinition, WorkflowOsError> {
        LocalCargoObservationDefinition::new(
            request,
            self.selection.clone(),
            request.operation_contract().clone(),
            request.security_requirements().clone(),
        )
    }

    fn executable(&self, role: LocalCargoExecutableRole) -> Result<&Path, WorkflowOsError> {
        self.executables
            .get(&role)
            .map(PathBuf::as_path)
            .ok_or_else(|| {
                observer_error(
                    "fixed_definition.executables.invalid",
                    "local Cargo fixed executable set is invalid",
                )
            })
    }
}

impl fmt::Debug for LocalCargoFixedProbeDefinition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCargoFixedProbeDefinition")
            .field("repository_root", &"[REDACTED]")
            .field("cargo_home", &"[REDACTED]")
            .field("executable_count", &self.executables.len())
            .field("environment_count", &self.environment.len())
            .field("selection", &"[REDACTED]")
            .finish()
    }
}

trait LocalCargoFixedProbeRunner {
    fn run(
        &self,
        probe: LocalCargoFixedProbe,
        definition: &LocalCargoFixedProbeDefinition,
    ) -> Result<LocalCargoFixedProbeOutput, WorkflowOsError>;
}

#[derive(Clone, Eq, PartialEq)]
struct LocalCargoFixedProbeOutput {
    commitment: SpecContentHash,
    item_count: u32,
}

struct LocalCargoFixedFilesystemSource {
    definition: LocalCargoFixedProbeDefinition,
}

impl LocalCargoFilesystemSource for LocalCargoFixedFilesystemSource {
    fn repository_root(
        &self,
        request: &TrustedHostInputObservationRequest,
        _definition: &LocalCargoObservationDefinition,
    ) -> Result<LocalCargoRootSourceObservation, WorkflowOsError> {
        let commitment = repository_tree_commitment(&self.definition.repository_root)?;
        if &commitment != request.repository_tree() {
            return Err(observer_error(
                "fixed_source.repository_drift",
                "local Cargo repository root changed before observation",
            ));
        }
        LocalCargoRootSourceObservation::new(
            TrustedHostObservationPosture::Uncertain,
            TrustedHostObservationPosture::Uncertain,
            TrustedHostRootObservationMethod::ImmutableProjection,
            commitment,
        )
    }
}

struct LocalCargoFixedProcessSource<R> {
    definition: LocalCargoFixedProbeDefinition,
    runner: R,
    cached: RefCell<
        Option<BTreeMap<TrustedHostBuildInputHostObligationKind, LocalCargoFactSourceObservation>>,
    >,
}

impl<R> LocalCargoFixedProcessSource<R>
where
    R: LocalCargoFixedProbeRunner,
{
    fn new(definition: LocalCargoFixedProbeDefinition, runner: R) -> Self {
        Self {
            definition,
            runner,
            cached: RefCell::new(None),
        }
    }

    fn collect(
        &self,
    ) -> Result<
        BTreeMap<TrustedHostBuildInputHostObligationKind, LocalCargoFactSourceObservation>,
        WorkflowOsError,
    > {
        let before = executable_set_commitment(&self.definition.executables)?;
        let verbose = self
            .runner
            .run(LocalCargoFixedProbe::RustcVerboseVersion, &self.definition)?;
        let sysroot = self
            .runner
            .run(LocalCargoFixedProbe::RustcSysroot, &self.definition)?;
        let target_libdir = self
            .runner
            .run(LocalCargoFixedProbe::RustcTargetLibdir, &self.definition)?;
        let metadata = self
            .runner
            .run(LocalCargoFixedProbe::CargoMetadata, &self.definition)?;
        let after = executable_set_commitment(&self.definition.executables)?;
        if before != after {
            return Err(observer_error(
                "fixed_source.executable_drift",
                "local Cargo executable identity changed during observation",
            ));
        }

        let combined_sysroot =
            combine_commitments("sysroot", &[&sysroot.commitment, &target_libdir.commitment]);
        let environment = environment_commitment(&self.definition.environment);
        let linker = combine_commitments("linker-declaration", &[self.definition.selection()]);
        let facts = [
            uncertain_fact(
                TrustedHostBuildInputHostObligationKind::ToolchainExecutable,
                before,
                u32::try_from(self.definition.executables.len()).map_err(|_| {
                    observer_error(
                        "fixed_source.count.invalid",
                        "local Cargo item count is invalid",
                    )
                })?,
            )?,
            uncertain_fact(
                TrustedHostBuildInputHostObligationKind::TargetSpecification,
                verbose.commitment,
                verbose.item_count,
            )?,
            uncertain_fact(
                TrustedHostBuildInputHostObligationKind::SdkOrSysroot,
                combined_sysroot,
                sysroot.item_count.saturating_add(target_libdir.item_count),
            )?,
            declared_fact(
                TrustedHostBuildInputHostObligationKind::LinkerOrArchiver,
                linker,
                0,
            )?,
            uncertain_fact(
                TrustedHostBuildInputHostObligationKind::NativeCompilerTools,
                metadata.commitment.clone(),
                metadata.item_count,
            )?,
            unsupported_fact(TrustedHostBuildInputHostObligationKind::DynamicLibraries)?,
            uncertain_fact(
                TrustedHostBuildInputHostObligationKind::RegistryAndPackageSources,
                metadata.commitment,
                metadata.item_count,
            )?,
            declared_fact(
                TrustedHostBuildInputHostObligationKind::Environment,
                environment,
                u32::try_from(self.definition.environment.len()).map_err(|_| {
                    observer_error(
                        "fixed_source.count.invalid",
                        "local Cargo item count is invalid",
                    )
                })?,
            )?,
            unsupported_fact(TrustedHostBuildInputHostObligationKind::RuntimeBase)?,
        ];
        Ok(facts.into_iter().map(|fact| (fact.kind, fact)).collect())
    }
}

fn uncertain_fact(
    kind: TrustedHostBuildInputHostObligationKind,
    commitment: SpecContentHash,
    item_count: u32,
) -> Result<LocalCargoFactSourceObservation, WorkflowOsError> {
    LocalCargoFactSourceObservation::new(
        kind,
        TrustedHostInputObservationPosture::Uncertain,
        TrustedHostInputObservationReason::IntegrityOrContinuityUncertain,
        TrustedHostInputObservationMethod::PreAndPostObjectIdentity,
        TrustedHostObservationPosture::Uncertain,
        TrustedHostObservationPosture::Uncertain,
        commitment,
        item_count,
    )
}

fn declared_fact(
    kind: TrustedHostBuildInputHostObligationKind,
    commitment: SpecContentHash,
    item_count: u32,
) -> Result<LocalCargoFactSourceObservation, WorkflowOsError> {
    LocalCargoFactSourceObservation::new(
        kind,
        TrustedHostInputObservationPosture::DeclaredOnly,
        TrustedHostInputObservationReason::NotIndependentlyObserved,
        TrustedHostInputObservationMethod::DeclaredConfiguration,
        TrustedHostObservationPosture::Uncertain,
        TrustedHostObservationPosture::Uncertain,
        commitment,
        item_count,
    )
}

fn unsupported_fact(
    kind: TrustedHostBuildInputHostObligationKind,
) -> Result<LocalCargoFactSourceObservation, WorkflowOsError> {
    LocalCargoFactSourceObservation::new(
        kind,
        TrustedHostInputObservationPosture::Unsupported,
        TrustedHostInputObservationReason::ObservationUnsupported,
        TrustedHostInputObservationMethod::Unsupported,
        TrustedHostObservationPosture::Uncertain,
        TrustedHostObservationPosture::Uncertain,
        SpecContentHash::from_text(&format!("unsupported/{kind:?}")),
        0,
    )
}

impl<R> LocalCargoProcessSource for LocalCargoFixedProcessSource<R>
where
    R: LocalCargoFixedProbeRunner,
{
    fn observe(
        &self,
        kind: TrustedHostBuildInputHostObligationKind,
        _request: &TrustedHostInputObservationRequest,
        _definition: &LocalCargoObservationDefinition,
    ) -> Result<LocalCargoFactSourceObservation, WorkflowOsError> {
        if self.cached.borrow().is_none() {
            *self.cached.borrow_mut() = Some(self.collect()?);
        }
        self.cached
            .borrow()
            .as_ref()
            .and_then(|facts| facts.get(&kind))
            .cloned()
            .ok_or_else(|| {
                observer_error("fixed_source.fact_missing", "local Cargo fact is missing")
            })
    }
}

struct StandardLocalCargoProbeRunner;

impl LocalCargoFixedProbeRunner for StandardLocalCargoProbeRunner {
    fn run(
        &self,
        probe: LocalCargoFixedProbe,
        definition: &LocalCargoFixedProbeDefinition,
    ) -> Result<LocalCargoFixedProbeOutput, WorkflowOsError> {
        if !cfg!(all(target_os = "macos", target_arch = "aarch64")) {
            return Err(observer_error(
                "fixed_probe.platform_unsupported",
                "local Cargo fixed probes are unsupported on this platform",
            ));
        }
        let (role, arguments): (LocalCargoExecutableRole, &[&str]) = match probe {
            LocalCargoFixedProbe::RustcVerboseVersion => {
                (LocalCargoExecutableRole::Rustc, &["-vV"])
            }
            LocalCargoFixedProbe::RustcSysroot => {
                (LocalCargoExecutableRole::Rustc, &["--print", "sysroot"])
            }
            LocalCargoFixedProbe::RustcTargetLibdir => (
                LocalCargoExecutableRole::Rustc,
                &["--print", "target-libdir"],
            ),
            LocalCargoFixedProbe::CargoMetadata => (
                LocalCargoExecutableRole::Cargo,
                &[
                    "metadata",
                    "--locked",
                    "--offline",
                    "--format-version",
                    "1",
                    "--no-deps",
                ],
            ),
        };
        let executable = definition.executable(role)?;
        let before = regular_file_commitment(executable)?;
        let output = run_bounded_process(
            executable,
            arguments,
            &definition.repository_root,
            &definition.environment,
        )?;
        let after = regular_file_commitment(executable)?;
        if before != after {
            return Err(observer_error(
                "fixed_probe.executable_drift",
                "local Cargo probe executable changed during invocation",
            ));
        }
        parse_probe_output(probe, &output)
    }
}

fn run_bounded_process(
    executable: &Path,
    arguments: &[&str],
    working_directory: &Path,
    environment: &BTreeMap<OsString, OsString>,
) -> Result<Vec<u8>, WorkflowOsError> {
    let mut command = Command::new(executable);
    command
        .args(arguments)
        .current_dir(working_directory)
        .env_clear()
        .envs(environment)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(unix)]
    command.process_group(0);
    let mut child = command.spawn().map_err(|_| {
        observer_error(
            "fixed_probe.spawn_failed",
            "local Cargo probe could not start",
        )
    })?;
    let stdout = child.stdout.take().ok_or_else(|| {
        observer_error(
            "fixed_probe.stdout_missing",
            "local Cargo probe output is unavailable",
        )
    })?;
    let stderr = child.stderr.take().ok_or_else(|| {
        observer_error(
            "fixed_probe.stderr_missing",
            "local Cargo probe error output is unavailable",
        )
    })?;
    let stdout_reader = thread::spawn(move || read_bounded_stream(stdout));
    let stderr_reader = thread::spawn(move || read_bounded_stream(stderr));
    let started = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait().map_err(|_| {
            observer_error(
                "fixed_probe.wait_failed",
                "local Cargo probe status is unavailable",
            )
        })? {
            break status;
        }
        if started.elapsed() >= FIXED_PROBE_TIMEOUT {
            #[cfg(unix)]
            if let Ok(process_group) = i32::try_from(child.id()) {
                let _ = killpg(Pid::from_raw(process_group), Signal::SIGKILL);
            }
            #[cfg(not(unix))]
            let _ = child.kill();
            let _ = child.wait();
            let _ = join_bounded_stream(stdout_reader);
            let _ = join_bounded_stream(stderr_reader);
            return Err(observer_error(
                "fixed_probe.timeout",
                "local Cargo probe timed out",
            ));
        }
        thread::sleep(Duration::from_millis(10));
    };
    let stdout = join_bounded_stream(stdout_reader)?;
    let stderr = join_bounded_stream(stderr_reader)?;
    if !status.success() {
        return Err(observer_error(
            "fixed_probe.failed",
            "local Cargo probe failed",
        ));
    }
    if !stderr.is_empty() {
        return Err(observer_error(
            "fixed_probe.stderr_unexpected",
            "local Cargo probe produced unexpected error output",
        ));
    }
    Ok(stdout)
}

fn read_bounded_stream<R: Read>(reader: R) -> Result<Vec<u8>, WorkflowOsError> {
    let mut output = Vec::new();
    reader
        .take(u64::try_from(MAX_PROBE_STREAM_BYTES + 1).unwrap_or(u64::MAX))
        .read_to_end(&mut output)
        .map_err(|_| {
            observer_error("fixed_probe.output_read", "local Cargo probe output failed")
        })?;
    if output.len() > MAX_PROBE_STREAM_BYTES {
        return Err(observer_error(
            "fixed_probe.output_oversized",
            "local Cargo probe output exceeds its bound",
        ));
    }
    Ok(output)
}

fn join_bounded_stream(
    reader: thread::JoinHandle<Result<Vec<u8>, WorkflowOsError>>,
) -> Result<Vec<u8>, WorkflowOsError> {
    reader
        .join()
        .map_err(|_| observer_error("fixed_probe.output_join", "local Cargo probe output failed"))?
}

fn parse_probe_output(
    probe: LocalCargoFixedProbe,
    output: &[u8],
) -> Result<LocalCargoFixedProbeOutput, WorkflowOsError> {
    let text = std::str::from_utf8(output).map_err(|_| {
        observer_error(
            "fixed_probe.output_invalid",
            "local Cargo probe output is invalid",
        )
    })?;
    let normalized = text.trim();
    if normalized.is_empty() {
        return Err(observer_error(
            "fixed_probe.output_empty",
            "local Cargo probe output is empty",
        ));
    }
    let item_count = match probe {
        LocalCargoFixedProbe::RustcVerboseVersion => {
            for required in ["release:", "commit-hash:", "host:"] {
                if !normalized.lines().any(|line| line.starts_with(required)) {
                    return Err(observer_error(
                        "fixed_probe.rustc_verbose_invalid",
                        "local Cargo rustc version output is incomplete",
                    ));
                }
            }
            1
        }
        LocalCargoFixedProbe::RustcSysroot | LocalCargoFixedProbe::RustcTargetLibdir => {
            if normalized.lines().count() != 1 || !Path::new(normalized).is_absolute() {
                return Err(observer_error(
                    "fixed_probe.path_invalid",
                    "local Cargo rustc path output is invalid",
                ));
            }
            1
        }
        LocalCargoFixedProbe::CargoMetadata => {
            let value: serde_json::Value = serde_json::from_str(normalized).map_err(|_| {
                observer_error(
                    "fixed_probe.metadata_invalid",
                    "local Cargo metadata is invalid",
                )
            })?;
            let packages = value
                .get("packages")
                .and_then(serde_json::Value::as_array)
                .ok_or_else(|| {
                    observer_error(
                        "fixed_probe.metadata_invalid",
                        "local Cargo metadata is invalid",
                    )
                })?;
            u32::try_from(packages.len()).map_err(|_| {
                observer_error(
                    "fixed_probe.metadata_oversized",
                    "local Cargo metadata is oversized",
                )
            })?
        }
    };
    Ok(LocalCargoFixedProbeOutput {
        commitment: SpecContentHash::from_text(normalized),
        item_count,
    })
}

fn canonical_directory(path: &Path) -> Result<PathBuf, WorkflowOsError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| {
        observer_error(
            "fixed_definition.path_invalid",
            "local Cargo path is invalid",
        )
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(observer_error(
            "fixed_definition.path_invalid",
            "local Cargo path is invalid",
        ));
    }
    path.canonicalize().map_err(|_| {
        observer_error(
            "fixed_definition.path_invalid",
            "local Cargo path is invalid",
        )
    })
}

fn canonical_regular_file(path: &Path) -> Result<PathBuf, WorkflowOsError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| {
        observer_error(
            "fixed_definition.executable_invalid",
            "local Cargo executable is invalid",
        )
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(observer_error(
            "fixed_definition.executable_invalid",
            "local Cargo executable is invalid",
        ));
    }
    path.canonicalize().map_err(|_| {
        observer_error(
            "fixed_definition.executable_invalid",
            "local Cargo executable is invalid",
        )
    })
}

fn regular_file_commitment(path: &Path) -> Result<SpecContentHash, WorkflowOsError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| {
        observer_error(
            "fixed_source.file_unavailable",
            "local Cargo file is unavailable",
        )
    })?;
    if metadata.file_type().is_symlink()
        || !metadata.is_file()
        || metadata.len() > MAX_EXECUTABLE_BYTES
    {
        return Err(observer_error(
            "fixed_source.file_invalid",
            "local Cargo file is invalid",
        ));
    }
    let mut file = File::open(path).map_err(|_| {
        observer_error(
            "fixed_source.file_unavailable",
            "local Cargo file is unavailable",
        )
    })?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; 16 * 1024];
    loop {
        let read = file.read(&mut buffer).map_err(|_| {
            observer_error(
                "fixed_source.file_read",
                "local Cargo file could not be read",
            )
        })?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(SpecContentHash::from_text(&format!(
        "sha256:{:x}",
        hasher.finalize()
    )))
}

fn executable_set_commitment(
    executables: &BTreeMap<LocalCargoExecutableRole, PathBuf>,
) -> Result<SpecContentHash, WorkflowOsError> {
    let mut hasher = Sha256::new();
    for (role, path) in executables {
        hasher.update(format!("{role:?}\0").as_bytes());
        hasher.update(regular_file_commitment(path)?.as_str().as_bytes());
        hasher.update(b"\0");
    }
    Ok(SpecContentHash::from_text(&format!(
        "sha256:{:x}",
        hasher.finalize()
    )))
}

fn repository_tree_commitment(root: &Path) -> Result<SpecContentHash, WorkflowOsError> {
    let mut files = Vec::new();
    collect_regular_files(root, root, &mut files)?;
    files.sort();
    if files.len() > MAX_ROOT_FILES {
        return Err(observer_error(
            "fixed_source.root_oversized",
            "local Cargo repository root exceeds its file bound",
        ));
    }
    let mut total_bytes = 0_u64;
    let mut hasher = Sha256::new();
    for relative in files {
        let path = root.join(&relative);
        let metadata = fs::symlink_metadata(&path).map_err(|_| {
            observer_error(
                "fixed_source.root_drift",
                "local Cargo repository root changed",
            )
        })?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(observer_error(
                "fixed_source.root_invalid",
                "local Cargo repository root contains an unsupported object",
            ));
        }
        total_bytes = total_bytes.checked_add(metadata.len()).ok_or_else(|| {
            observer_error(
                "fixed_source.root_oversized",
                "local Cargo repository root is oversized",
            )
        })?;
        if total_bytes > MAX_ROOT_BYTES {
            return Err(observer_error(
                "fixed_source.root_oversized",
                "local Cargo repository root exceeds its byte bound",
            ));
        }
        hasher.update(relative.to_string_lossy().as_bytes());
        hasher.update(b"\0");
        let mut file = File::open(&path).map_err(|_| {
            observer_error(
                "fixed_source.root_read",
                "local Cargo repository root could not be read",
            )
        })?;
        let mut buffer = vec![0_u8; 16 * 1024];
        loop {
            let read = file.read(&mut buffer).map_err(|_| {
                observer_error(
                    "fixed_source.root_read",
                    "local Cargo repository root could not be read",
                )
            })?;
            if read == 0 {
                break;
            }
            hasher.update(&buffer[..read]);
        }
        hasher.update(b"\0");
    }
    Ok(SpecContentHash::from_text(&format!(
        "sha256:{:x}",
        hasher.finalize()
    )))
}

fn collect_regular_files(
    root: &Path,
    current: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), WorkflowOsError> {
    let mut entries = fs::read_dir(current)
        .map_err(|_| {
            observer_error(
                "fixed_source.root_read",
                "local Cargo repository root could not be read",
            )
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| {
            observer_error(
                "fixed_source.root_read",
                "local Cargo repository root could not be read",
            )
        })?;
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).map_err(|_| {
            observer_error(
                "fixed_source.root_read",
                "local Cargo repository root could not be read",
            )
        })?;
        if metadata.file_type().is_symlink() {
            return Err(observer_error(
                "fixed_source.root_symlink",
                "local Cargo repository root contains a symlink",
            ));
        }
        if metadata.is_dir() {
            collect_regular_files(root, &path, files)?;
        } else if metadata.is_file() {
            files.push(
                path.strip_prefix(root)
                    .map_err(|_| {
                        observer_error(
                            "fixed_source.root_invalid",
                            "local Cargo repository root is invalid",
                        )
                    })?
                    .to_path_buf(),
            );
        } else {
            return Err(observer_error(
                "fixed_source.root_invalid",
                "local Cargo repository root contains an unsupported object",
            ));
        }
    }
    Ok(())
}

fn fixed_selection_commitment(
    repository_root: &Path,
    cargo_home: &Path,
    executables: &BTreeMap<LocalCargoExecutableRole, PathBuf>,
    environment: &BTreeMap<OsString, OsString>,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hasher.update(b"workflow-os/local-cargo-fixed-probes/v1\0");
    hasher.update(repository_root.as_os_str().as_encoded_bytes());
    hasher.update(b"\0");
    hasher.update(cargo_home.as_os_str().as_encoded_bytes());
    hasher.update(b"\0");
    for (role, path) in executables {
        hasher.update(format!("{role:?}\0").as_bytes());
        hasher.update(path.as_os_str().as_encoded_bytes());
        hasher.update(b"\0");
    }
    for (name, value) in environment {
        hasher.update(name.as_encoded_bytes());
        hasher.update(b"\0");
        hasher.update(value.as_encoded_bytes());
        hasher.update(b"\0");
    }
    SpecContentHash::from_text(&format!("sha256:{:x}", hasher.finalize()))
}

fn environment_commitment(environment: &BTreeMap<OsString, OsString>) -> SpecContentHash {
    let mut hasher = Sha256::new();
    for (name, value) in environment {
        hasher.update(name.as_encoded_bytes());
        hasher.update(b"\0");
        hasher.update(value.as_encoded_bytes());
        hasher.update(b"\0");
    }
    SpecContentHash::from_text(&format!("sha256:{:x}", hasher.finalize()))
}

fn combine_commitments(label: &str, commitments: &[&SpecContentHash]) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hasher.update(label.as_bytes());
    hasher.update(b"\0");
    for commitment in commitments {
        hasher.update(commitment.as_str().as_bytes());
        hasher.update(b"\0");
    }
    SpecContentHash::from_text(&format!("sha256:{:x}", hasher.finalize()))
}

fn evidence_entry(
    request: &TrustedHostInputObservationRequest,
    producer: &TrustedHostInputObservationProducer,
    completed_revision: u64,
    class: LocalCargoEvidenceClass,
    commitment: SpecContentHash,
) -> Result<LocalCargoObservationEvidenceEntry, WorkflowOsError> {
    let class_tag = evidence_class_tag(class);
    let reference_commitment = SpecContentHash::from_text(&format!(
        "workflow-os/local-cargo-observation-evidence/v1\n{}\n{}\n{}\n{}\n{}\n{}",
        request.fingerprint(),
        producer.id().as_str(),
        producer.version().as_str(),
        completed_revision,
        class_tag,
        commitment,
    ));
    let reference = HostedExecutionReference::new(
        HostedExecutionReferenceKind::Telemetry,
        format!("trusted-host-observation/{}", reference_commitment.as_str()),
    )?;
    Ok(LocalCargoObservationEvidenceEntry {
        reference,
        class,
        request_fingerprint: request.fingerprint().clone(),
        producer_id: producer.id().clone(),
        producer_version: producer.version().clone(),
        completed_revision,
        commitment,
    })
}

fn evidence_class_tag(class: LocalCargoEvidenceClass) -> &'static str {
    match class {
        LocalCargoEvidenceClass::RepositoryRoot => "repository_root",
        LocalCargoEvidenceClass::Fact(
            TrustedHostBuildInputHostObligationKind::ToolchainExecutable,
        ) => "fact/toolchain_executable",
        LocalCargoEvidenceClass::Fact(
            TrustedHostBuildInputHostObligationKind::TargetSpecification,
        ) => "fact/target_specification",
        LocalCargoEvidenceClass::Fact(TrustedHostBuildInputHostObligationKind::SdkOrSysroot) => {
            "fact/sdk_or_sysroot"
        }
        LocalCargoEvidenceClass::Fact(
            TrustedHostBuildInputHostObligationKind::LinkerOrArchiver,
        ) => "fact/linker_or_archiver",
        LocalCargoEvidenceClass::Fact(
            TrustedHostBuildInputHostObligationKind::NativeCompilerTools,
        ) => "fact/native_compiler_tools",
        LocalCargoEvidenceClass::Fact(
            TrustedHostBuildInputHostObligationKind::DynamicLibraries,
        ) => "fact/dynamic_libraries",
        LocalCargoEvidenceClass::Fact(
            TrustedHostBuildInputHostObligationKind::RegistryAndPackageSources,
        ) => "fact/registry_and_package_sources",
        LocalCargoEvidenceClass::Fact(TrustedHostBuildInputHostObligationKind::Environment) => {
            "fact/environment"
        }
        LocalCargoEvidenceClass::Fact(TrustedHostBuildInputHostObligationKind::RuntimeBase) => {
            "fact/runtime_base"
        }
    }
}

fn observer_error(suffix: &str, message: &str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("trusted_host.local_cargo_input_observer.{suffix}"),
        message,
    )
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    use workflow_core::{
        TrustedHostInputObservationContractVersion, TrustedHostInputObservationProducerDefinition,
        TrustedHostInputObservationProducerId, TrustedHostInputObservationProducerVersion,
        TrustedHostInputObservationRequestDefinition,
    };

    #[derive(Clone)]
    struct FakeFilesystem {
        root: LocalCargoRootSourceObservation,
    }

    impl LocalCargoFilesystemSource for FakeFilesystem {
        fn repository_root(
            &self,
            _request: &TrustedHostInputObservationRequest,
            _definition: &LocalCargoObservationDefinition,
        ) -> Result<LocalCargoRootSourceObservation, WorkflowOsError> {
            Ok(self.root.clone())
        }
    }

    #[derive(Clone)]
    struct FakeProcess {
        facts: Vec<LocalCargoFactSourceObservation>,
        substitution: Option<(
            TrustedHostBuildInputHostObligationKind,
            TrustedHostBuildInputHostObligationKind,
        )>,
    }

    impl LocalCargoProcessSource for FakeProcess {
        fn observe(
            &self,
            kind: TrustedHostBuildInputHostObligationKind,
            _request: &TrustedHostInputObservationRequest,
            _definition: &LocalCargoObservationDefinition,
        ) -> Result<LocalCargoFactSourceObservation, WorkflowOsError> {
            if let Some((requested, returned)) = self.substitution {
                if requested == kind {
                    return self
                        .facts
                        .iter()
                        .find(|fact| fact.kind == returned)
                        .cloned()
                        .ok_or_else(|| {
                            observer_error("test.fact_missing", "test fact is missing")
                        });
                }
            }
            self.facts
                .iter()
                .find(|fact| fact.kind == kind)
                .cloned()
                .ok_or_else(|| observer_error("test.fact_missing", "test fact is missing"))
        }
    }

    struct FakeRevisions {
        revisions: Vec<u64>,
    }

    impl LocalCargoRevisionSource for FakeRevisions {
        fn next_revision(
            &mut self,
            _ordering_authority: &SpecContentHash,
        ) -> Result<u64, WorkflowOsError> {
            if self.revisions.is_empty() {
                return Err(observer_error(
                    "test.revision_missing",
                    "test revision is missing",
                ));
            }
            Ok(self.revisions.remove(0))
        }
    }

    fn hash(label: &str) -> SpecContentHash {
        SpecContentHash::from_text(label)
    }

    fn producer() -> TrustedHostInputObservationProducer {
        TrustedHostInputObservationProducer::new(TrustedHostInputObservationProducerDefinition {
            id: TrustedHostInputObservationProducerId::new("workflow-os/local-cargo-observer")
                .expect("producer id"),
            version: TrustedHostInputObservationProducerVersion::new("v1")
                .expect("producer version"),
            kind: TrustedHostInputObservationProducerKind::HostObserver,
            assurance: TrustedHostInputObservationProducerAssurance::IndependentlyReviewed,
            implementation: hash("implementation"),
            configuration: hash("configuration"),
        })
        .expect("producer")
    }

    fn request_with(
        selection: SpecContentHash,
        repository_tree: SpecContentHash,
    ) -> TrustedHostInputObservationRequest {
        let producer = producer();
        TrustedHostInputObservationRequest::new(TrustedHostInputObservationRequestDefinition {
            version: TrustedHostInputObservationContractVersion::CargoHostInputsV2,
            run_bundle: hash("run-bundle"),
            operation_contract: hash("operation-contract"),
            static_request: hash("static-request"),
            static_result: hash("static-result"),
            ordering_authority: hash("ordering-authority"),
            static_discovery_completed_revision: 10,
            repository_tree,
            security_requirements: hash("security-requirements"),
            selection,
            attempt: hash("attempt"),
            expected_producer_id: producer.id().clone(),
            expected_producer_version: producer.version().clone(),
        })
        .expect("request")
    }

    fn request() -> TrustedHostInputObservationRequest {
        request_with(hash("selection"), hash("repository-tree"))
    }

    fn definition(request: &TrustedHostInputObservationRequest) -> LocalCargoObservationDefinition {
        LocalCargoObservationDefinition::new(
            request,
            request.selection().clone(),
            request.operation_contract().clone(),
            request.security_requirements().clone(),
        )
        .expect("definition")
    }

    fn fact(kind: TrustedHostBuildInputHostObligationKind) -> LocalCargoFactSourceObservation {
        let (posture, reason, method) = match kind {
            TrustedHostBuildInputHostObligationKind::DynamicLibraries
            | TrustedHostBuildInputHostObligationKind::RuntimeBase => (
                TrustedHostInputObservationPosture::Unsupported,
                TrustedHostInputObservationReason::ObservationUnsupported,
                TrustedHostInputObservationMethod::Unsupported,
            ),
            TrustedHostBuildInputHostObligationKind::Environment => (
                TrustedHostInputObservationPosture::DeclaredOnly,
                TrustedHostInputObservationReason::NotIndependentlyObserved,
                TrustedHostInputObservationMethod::DeclaredConfiguration,
            ),
            _ => (
                TrustedHostInputObservationPosture::Uncertain,
                TrustedHostInputObservationReason::IntegrityOrContinuityUncertain,
                TrustedHostInputObservationMethod::PreAndPostObjectIdentity,
            ),
        };
        LocalCargoFactSourceObservation::new(
            kind,
            posture,
            reason,
            method,
            TrustedHostObservationPosture::Uncertain,
            TrustedHostObservationPosture::Uncertain,
            hash(&format!("fact-{kind:?}")),
            1,
        )
        .expect("fact")
    }

    fn observer() -> LocalCargoHostObserver<FakeFilesystem, FakeProcess, FakeRevisions> {
        LocalCargoHostObserver::new(
            producer(),
            FakeFilesystem {
                root: LocalCargoRootSourceObservation::new(
                    TrustedHostObservationPosture::Uncertain,
                    TrustedHostObservationPosture::Uncertain,
                    TrustedHostRootObservationMethod::ImmutableProjection,
                    hash("repository-tree"),
                )
                .expect("root"),
            },
            FakeProcess {
                facts: REQUIRED_FACT_KINDS.iter().copied().map(fact).collect(),
                substitution: None,
            },
            FakeRevisions {
                revisions: vec![11, 12],
            },
        )
        .expect("observer")
    }

    #[test]
    fn process_free_observer_returns_complete_exact_bound_result_and_bundle() {
        let request = request();
        let output = observer()
            .observe(&request, &definition(&request))
            .expect("observation");

        assert_eq!(output.result.facts().len(), 9);
        assert_eq!(output.evidence.entries.len(), 10);
        assert!(output.result.facts().iter().all(|fact| {
            fact.posture() != TrustedHostInputObservationPosture::Observed
                && fact.reference().is_some()
        }));
        output
            .evidence
            .validate(&request, &output.result)
            .expect("evidence bundle");
    }

    #[test]
    fn unchanged_inputs_produce_deterministic_result_and_evidence_references() {
        let request = request();
        let first = observer()
            .observe(&request, &definition(&request))
            .expect("first observation");
        let second = observer()
            .observe(&request, &definition(&request))
            .expect("second observation");

        assert_eq!(first.result.fingerprint(), second.result.fingerprint());
        assert_eq!(
            first
                .evidence
                .entries
                .iter()
                .map(|entry| entry.reference.value())
                .collect::<Vec<_>>(),
            second
                .evidence
                .entries
                .iter()
                .map(|entry| entry.reference.value())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn definition_rejects_selection_operation_or_security_substitution() {
        let request = request();
        for values in [
            (
                hash("other-selection"),
                request.operation_contract().clone(),
                request.security_requirements().clone(),
            ),
            (
                request.selection().clone(),
                hash("other-operation"),
                request.security_requirements().clone(),
            ),
            (
                request.selection().clone(),
                request.operation_contract().clone(),
                hash("other-security"),
            ),
        ] {
            let error =
                LocalCargoObservationDefinition::new(&request, values.0, values.1, values.2)
                    .expect_err("substitution must fail");
            assert_eq!(
                error.code(),
                "trusted_host.local_cargo_input_observer.definition.request_mismatch"
            );
        }
    }

    #[test]
    fn standalone_posture_ceiling_rejects_observed_facts_and_proven_root() {
        let observed = LocalCargoFactSourceObservation::new(
            TrustedHostBuildInputHostObligationKind::ToolchainExecutable,
            TrustedHostInputObservationPosture::Observed,
            TrustedHostInputObservationReason::IntegrityOrContinuityUncertain,
            TrustedHostInputObservationMethod::PreAndPostObjectIdentity,
            TrustedHostObservationPosture::Proven,
            TrustedHostObservationPosture::Proven,
            hash("toolchain"),
            1,
        );
        assert!(observed.is_err());
        assert!(LocalCargoRootSourceObservation::new(
            TrustedHostObservationPosture::Proven,
            TrustedHostObservationPosture::Uncertain,
            TrustedHostRootObservationMethod::ImmutableProjection,
            hash("root"),
        )
        .is_err());
    }

    #[test]
    fn evidence_bundle_rejects_missing_duplicate_extra_and_cross_request_entries() {
        let request = request();
        let output = observer()
            .observe(&request, &definition(&request))
            .expect("observation");

        let mut missing = output.evidence.clone();
        missing.entries.pop();
        assert!(missing.validate(&request, &output.result).is_err());

        let mut duplicate = output.evidence.clone();
        duplicate.entries[1] = duplicate.entries[0].clone();
        assert!(duplicate.validate(&request, &output.result).is_err());

        let mut extra = output.evidence.clone();
        extra.entries.push(extra.entries[0].clone());
        assert!(extra.validate(&request, &output.result).is_err());

        let mut cross_request = output.evidence.clone();
        cross_request.entries[0].request_fingerprint = hash("other-request");
        assert!(cross_request.validate(&request, &output.result).is_err());

        let mut producer_substitution = output.evidence.clone();
        producer_substitution.entries[0].producer_id =
            TrustedHostInputObservationProducerId::new("other/observer").expect("producer id");
        assert!(producer_substitution
            .validate(&request, &output.result)
            .is_err());

        let mut completion_substitution = output.evidence.clone();
        completion_substitution.completed_revision = output.result.completed_revision() + 1;
        assert!(completion_substitution
            .validate(&request, &output.result)
            .is_err());
    }

    #[test]
    fn source_cannot_substitute_one_obligation_for_another() {
        let request = request();
        let mut observer = observer();
        observer.process.substitution = Some((
            TrustedHostBuildInputHostObligationKind::ToolchainExecutable,
            TrustedHostBuildInputHostObligationKind::TargetSpecification,
        ));

        let error = observer
            .observe(&request, &definition(&request))
            .expect_err("kind substitution must fail");
        assert_eq!(
            error.code(),
            "trusted_host.local_cargo_input_observer.fact.kind_substituted"
        );
    }

    #[test]
    fn revisions_root_and_producer_fail_closed_without_source_retries() {
        let request = request();
        let mut stale = observer();
        stale.revisions.revisions = vec![10, 11];
        assert!(stale.observe(&request, &definition(&request)).is_err());

        let mut root_substitution = observer();
        root_substitution.filesystem.root.commitment = hash("other-root");
        assert!(root_substitution
            .observe(&request, &definition(&request))
            .is_err());

        let wrong_producer = TrustedHostInputObservationProducer::new(
            TrustedHostInputObservationProducerDefinition {
                id: TrustedHostInputObservationProducerId::new("other/observer")
                    .expect("producer id"),
                version: TrustedHostInputObservationProducerVersion::new("v1")
                    .expect("producer version"),
                kind: TrustedHostInputObservationProducerKind::HostObserver,
                assurance: TrustedHostInputObservationProducerAssurance::IndependentlyReviewed,
                implementation: hash("implementation"),
                configuration: hash("configuration"),
            },
        )
        .expect("producer");
        let mut observer = LocalCargoHostObserver::new(
            wrong_producer,
            observer().filesystem,
            observer().process,
            FakeRevisions {
                revisions: vec![11, 12],
            },
        )
        .expect("observer");
        assert!(observer.observe(&request, &definition(&request)).is_err());
    }

    #[test]
    fn debug_output_redacts_commitments_and_references() {
        let request = request();
        let output = observer()
            .observe(&request, &definition(&request))
            .expect("observation");
        let debug = format!("{:?}", output.evidence);

        assert!(!debug.contains(request.fingerprint().as_str()));
        assert!(!debug.contains(output.evidence.entries[0].reference.value()));
        assert!(!debug.contains("repository-tree"));
    }

    struct TempFixture {
        root: PathBuf,
    }

    impl TempFixture {
        fn new() -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock")
                .as_nanos();
            let root = std::env::temp_dir().join(format!(
                "workflow-os-local-cargo-source-{}-{nonce}",
                std::process::id()
            ));
            fs::create_dir_all(root.join("repository/src")).expect("repository");
            fs::create_dir_all(root.join("cargo-home")).expect("cargo home");
            fs::create_dir_all(root.join("bin")).expect("bin");
            fs::write(
                root.join("repository/Cargo.toml"),
                "[package]\nname='fixture'\nversion='0.1.0'\n",
            )
            .expect("manifest");
            fs::write(root.join("repository/src/lib.rs"), "pub fn fixture() {}\n").expect("source");
            for name in ["cargo", "rustc", "rustdoc", "cargo-clippy", "clippy-driver"] {
                fs::write(root.join("bin").join(name), format!("fixture-{name}"))
                    .expect("executable fixture");
            }
            Self { root }
        }

        fn definition(&self) -> LocalCargoFixedProbeDefinition {
            LocalCargoFixedProbeDefinition::new(
                &self.root.join("repository"),
                &self.root.join("cargo-home"),
                BTreeMap::from([
                    (LocalCargoExecutableRole::Cargo, self.root.join("bin/cargo")),
                    (LocalCargoExecutableRole::Rustc, self.root.join("bin/rustc")),
                    (
                        LocalCargoExecutableRole::Rustdoc,
                        self.root.join("bin/rustdoc"),
                    ),
                    (
                        LocalCargoExecutableRole::CargoClippy,
                        self.root.join("bin/cargo-clippy"),
                    ),
                    (
                        LocalCargoExecutableRole::ClippyDriver,
                        self.root.join("bin/clippy-driver"),
                    ),
                ]),
            )
            .expect("fixed definition")
        }
    }

    impl Drop for TempFixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    #[derive(Default)]
    struct ScriptedFixedRunner {
        probes: RefCell<Vec<LocalCargoFixedProbe>>,
    }

    impl LocalCargoFixedProbeRunner for ScriptedFixedRunner {
        fn run(
            &self,
            probe: LocalCargoFixedProbe,
            _definition: &LocalCargoFixedProbeDefinition,
        ) -> Result<LocalCargoFixedProbeOutput, WorkflowOsError> {
            self.probes.borrow_mut().push(probe);
            Ok(LocalCargoFixedProbeOutput {
                commitment: hash(&format!("probe-{probe:?}")),
                item_count: 1,
            })
        }
    }

    #[test]
    fn fixed_source_is_selection_bound_complete_and_process_free_under_injected_runner() {
        let fixture = TempFixture::new();
        let fixed = fixture.definition();
        let tree = repository_tree_commitment(&fixed.repository_root).expect("tree");
        let request = request_with(fixed.selection().clone(), tree);
        let definition = fixed.bind(&request).expect("binding");
        let runner = ScriptedFixedRunner::default();
        let process = LocalCargoFixedProcessSource::new(fixed.clone(), runner);
        let mut observer = LocalCargoHostObserver::new(
            producer(),
            LocalCargoFixedFilesystemSource { definition: fixed },
            process,
            FakeRevisions {
                revisions: vec![11, 12],
            },
        )
        .expect("observer");

        let output = observer
            .observe(&request, &definition)
            .expect("observation");

        assert_eq!(output.result.facts().len(), 9);
        assert!(output
            .result
            .facts()
            .iter()
            .all(|fact| { fact.posture() != TrustedHostInputObservationPosture::Observed }));
        assert_eq!(observer.process.runner.probes.borrow().len(), 4);
    }

    #[test]
    fn fixed_definition_rejects_missing_roles_and_symlinked_executables() {
        let fixture = TempFixture::new();
        let mut executables = fixture.definition().executables;
        executables.remove(&LocalCargoExecutableRole::CargoClippy);
        assert!(LocalCargoFixedProbeDefinition::new(
            &fixture.root.join("repository"),
            &fixture.root.join("cargo-home"),
            executables,
        )
        .is_err());

        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            let link = fixture.root.join("bin/cargo-link");
            symlink(fixture.root.join("bin/cargo"), &link).expect("symlink");
            let mut executables = fixture.definition().executables;
            executables.insert(LocalCargoExecutableRole::Cargo, link);
            assert!(LocalCargoFixedProbeDefinition::new(
                &fixture.root.join("repository"),
                &fixture.root.join("cargo-home"),
                executables,
            )
            .is_err());
        }
    }

    #[test]
    fn fixed_probe_parsers_fail_closed_and_accept_only_closed_shapes() {
        assert!(parse_probe_output(
            LocalCargoFixedProbe::RustcVerboseVersion,
            b"rustc 1.0\nrelease: 1.0\ncommit-hash: abc\nhost: aarch64-apple-darwin\n",
        )
        .is_ok());
        assert!(
            parse_probe_output(LocalCargoFixedProbe::RustcVerboseVersion, b"rustc 1.0\n").is_err()
        );
        assert!(
            parse_probe_output(LocalCargoFixedProbe::RustcSysroot, b"relative/path\n").is_err()
        );
        assert!(parse_probe_output(
            LocalCargoFixedProbe::CargoMetadata,
            br#"{"packages":[],"workspace_members":[]}"#,
        )
        .is_ok());
        assert!(parse_probe_output(LocalCargoFixedProbe::CargoMetadata, b"not-json").is_err());
    }

    #[test]
    fn fixed_source_detects_repository_and_executable_drift() {
        let fixture = TempFixture::new();
        let fixed = fixture.definition();
        let original_tree = repository_tree_commitment(&fixed.repository_root).expect("tree");
        let request = request_with(fixed.selection().clone(), original_tree);
        fs::write(
            fixed.repository_root.join("src/lib.rs"),
            "pub fn changed() {}\n",
        )
        .expect("change root");
        let source = LocalCargoFixedFilesystemSource {
            definition: fixed.clone(),
        };
        assert!(source
            .repository_root(&request, &fixed.bind(&request).expect("binding"))
            .is_err());

        let before = executable_set_commitment(&fixed.executables).expect("executables");
        fs::write(
            fixed
                .executable(LocalCargoExecutableRole::Rustdoc)
                .expect("rustdoc"),
            "changed",
        )
        .expect("change executable");
        let after = executable_set_commitment(&fixed.executables).expect("executables");
        assert_ne!(before, after);
    }

    #[test]
    fn fixed_definition_debug_redacts_paths_and_selection() {
        let fixture = TempFixture::new();
        let fixed = fixture.definition();
        let debug = format!("{fixed:?}");
        assert!(!debug.contains(fixture.root.to_string_lossy().as_ref()));
        assert!(!debug.contains(fixed.selection().as_str()));
    }

    #[test]
    #[ignore = "opt-in Apple Silicon live Cargo observation smoke"]
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    fn opt_in_apple_silicon_fixed_probe_smoke() {
        let fixture = TempFixture::new();
        fs::write(
            fixture.root.join("repository/Cargo.lock"),
            "# This file is automatically @generated by Cargo.\nversion = 3\n\n[[package]]\nname = \"fixture\"\nversion = \"0.1.0\"\n",
        )
        .expect("lockfile");
        let cargo = PathBuf::from(env!("CARGO"));
        let bin = cargo.parent().expect("cargo bin").to_path_buf();
        let cargo_home = bin.parent().expect("cargo home").to_path_buf();
        let fixed = LocalCargoFixedProbeDefinition::new(
            &fixture.root.join("repository"),
            &cargo_home,
            BTreeMap::from([
                (LocalCargoExecutableRole::Cargo, cargo),
                (LocalCargoExecutableRole::Rustc, bin.join("rustc")),
                (LocalCargoExecutableRole::Rustdoc, bin.join("rustdoc")),
                (
                    LocalCargoExecutableRole::CargoClippy,
                    bin.join("cargo-clippy"),
                ),
                (
                    LocalCargoExecutableRole::ClippyDriver,
                    bin.join("clippy-driver"),
                ),
            ]),
        )
        .expect("live definition");
        let tree = repository_tree_commitment(&fixed.repository_root).expect("tree");
        let request = request_with(fixed.selection().clone(), tree);
        let definition = fixed.bind(&request).expect("binding");
        let mut observer = LocalCargoHostObserver::new(
            producer(),
            LocalCargoFixedFilesystemSource {
                definition: fixed.clone(),
            },
            LocalCargoFixedProcessSource::new(fixed, StandardLocalCargoProbeRunner),
            FakeRevisions {
                revisions: vec![11, 12],
            },
        )
        .expect("observer");
        let output = observer
            .observe(&request, &definition)
            .expect("live observation");
        assert_eq!(output.result.facts().len(), 9);
        assert!(output
            .result
            .facts()
            .iter()
            .all(|fact| { fact.posture() != TrustedHostInputObservationPosture::Observed }));
    }
}
