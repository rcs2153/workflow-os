//! Private, process-free contracts for the local Cargo input observer.
//!
//! The module owns no concrete filesystem or process implementation. Callers
//! must inject closed sources, and the observer returns non-authorizing Core
//! observations plus an exact in-memory evidence bundle.

use std::collections::BTreeSet;
use std::fmt;

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

    fn request() -> TrustedHostInputObservationRequest {
        let producer = producer();
        TrustedHostInputObservationRequest::new(TrustedHostInputObservationRequestDefinition {
            version: TrustedHostInputObservationContractVersion::CargoHostInputsV2,
            run_bundle: hash("run-bundle"),
            operation_contract: hash("operation-contract"),
            static_request: hash("static-request"),
            static_result: hash("static-result"),
            ordering_authority: hash("ordering-authority"),
            static_discovery_completed_revision: 10,
            repository_tree: hash("repository-tree"),
            security_requirements: hash("security-requirements"),
            selection: hash("selection"),
            attempt: hash("attempt"),
            expected_producer_id: producer.id().clone(),
            expected_producer_version: producer.version().clone(),
        })
        .expect("request")
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
}
