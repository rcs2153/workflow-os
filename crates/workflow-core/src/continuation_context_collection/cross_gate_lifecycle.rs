//! Private cross-gate lifecycle binding for replacement-executor authority.
//!
//! This model composes current authoritative observations from the three
//! continuity-of-authority gates. It remains payload-free and non-authorizing;
//! the replacement executor may use its commitment only as an input to a
//! separately resolved fresh-authority request.

use serde::Serialize;

use super::{collection_commitment, collection_error};
use crate::{
    ImmutableRunBundleStore, RunSnapshotStore, SpecContentHash,
    WorkReportArtifactLocationExpectation, WorkReportArtifactLocatorClass, WorkReportArtifactStore,
    WorkReportId, WorkflowOsError, WorkflowRunId,
};

const BINDING_VERSION: &str = "workflow-os/cross-gate-lifecycle-binding/v1";

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct AcceptedDispatchObservation {
    run_id: WorkflowRunId,
    immutable_context_commitment: SpecContentHash,
    dispatch_commitments: Vec<SpecContentHash>,
}

impl AcceptedDispatchObservation {
    pub(super) fn new(
        run_id: WorkflowRunId,
        immutable_context_commitment: SpecContentHash,
        mut dispatch_commitments: Vec<SpecContentHash>,
    ) -> Result<Self, WorkflowOsError> {
        if dispatch_commitments.is_empty() {
            return Err(binding_error(
                "dispatch.missing",
                "cross-gate lifecycle binding requires accepted durable dispatch evidence",
            ));
        }
        dispatch_commitments.sort_by(|left, right| left.as_str().cmp(right.as_str()));
        if dispatch_commitments
            .windows(2)
            .any(|pair| pair[0] == pair[1])
        {
            return Err(binding_error(
                "dispatch.duplicate",
                "cross-gate lifecycle binding contains duplicate dispatch evidence",
            ));
        }
        Ok(Self {
            run_id,
            immutable_context_commitment,
            dispatch_commitments,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct VerifiedReportObservation {
    run_id: WorkflowRunId,
    immutable_context_commitment: SpecContentHash,
    artifact_location_commitment: SpecContentHash,
}

impl VerifiedReportObservation {
    pub(super) fn new(
        run_id: WorkflowRunId,
        immutable_context_commitment: SpecContentHash,
        artifact_location_commitment: SpecContentHash,
    ) -> Self {
        Self {
            run_id,
            immutable_context_commitment,
            artifact_location_commitment,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CurrentResourceAuthorityObservation {
    run_id: WorkflowRunId,
    immutable_context_commitment: SpecContentHash,
    resource_restriction_commitment: SpecContentHash,
}

impl CurrentResourceAuthorityObservation {
    pub(super) fn new(
        run_id: WorkflowRunId,
        immutable_context_commitment: SpecContentHash,
        resource_restriction_commitment: SpecContentHash,
    ) -> Self {
        Self {
            run_id,
            immutable_context_commitment,
            resource_restriction_commitment,
        }
    }
}

/// One revision-fenced observation from an authoritative current-state reader.
#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CrossGateLifecycleObservation {
    revision: SpecContentHash,
    lawful_next_operation_identity: SpecContentHash,
    dispatch: AcceptedDispatchObservation,
    report: VerifiedReportObservation,
    resource_authority: CurrentResourceAuthorityObservation,
}

impl CrossGateLifecycleObservation {
    pub(super) fn new(
        revision: SpecContentHash,
        lawful_next_operation_identity: SpecContentHash,
        dispatch: AcceptedDispatchObservation,
        report: VerifiedReportObservation,
        resource_authority: CurrentResourceAuthorityObservation,
    ) -> Self {
        Self {
            revision,
            lawful_next_operation_identity,
            dispatch,
            report,
            resource_authority,
        }
    }
}

/// Read-only source that must reconstruct all three gate observations from the
/// same current authoritative state read.
pub(super) trait CrossGateLifecycleSource {
    fn observe_current(&self) -> Result<CrossGateLifecycleObservation, WorkflowOsError>;

    /// Rereads the source revision after composition so drift fails closed.
    fn current_revision(&self) -> Result<SpecContentHash, WorkflowOsError>;
}

#[derive(Clone, Eq, PartialEq)]
pub(super) struct CurrentProjectResourceAuthorityRead {
    run_id: WorkflowRunId,
    immutable_context_commitment: SpecContentHash,
    resource_restriction_commitment: SpecContentHash,
    revision: SpecContentHash,
}

impl CurrentProjectResourceAuthorityRead {
    pub(super) fn new(
        run_id: WorkflowRunId,
        immutable_context_commitment: SpecContentHash,
        resource_restriction_commitment: SpecContentHash,
        revision: SpecContentHash,
    ) -> Self {
        Self {
            run_id,
            immutable_context_commitment,
            resource_restriction_commitment,
            revision,
        }
    }
}

/// Current project-resource reader. Implementations own reconstruction of the
/// exact project restriction and must not return cached approval-time facts.
pub(super) trait CurrentProjectResourceAuthoritySource {
    fn read_current(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<CurrentProjectResourceAuthorityRead, WorkflowOsError>;
}

#[derive(Clone, Eq, PartialEq)]
pub(super) struct CurrentAcceptedDispatchRead {
    run_id: WorkflowRunId,
    immutable_context_commitment: SpecContentHash,
    dispatch_commitments: Vec<SpecContentHash>,
    revision: SpecContentHash,
}

impl CurrentAcceptedDispatchRead {
    pub(super) fn new(
        run_id: WorkflowRunId,
        immutable_context_commitment: SpecContentHash,
        dispatch_commitments: Vec<SpecContentHash>,
        revision: SpecContentHash,
    ) -> Self {
        Self {
            run_id,
            immutable_context_commitment,
            dispatch_commitments,
            revision,
        }
    }
}

/// Current Gate 1 reader. Implementations must derive accepted dispatch from
/// the complete durable hosted posture, including queue/work-item evidence,
/// rather than from run events alone.
pub(super) trait CurrentAcceptedDispatchSource {
    fn read_current(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<CurrentAcceptedDispatchRead, WorkflowOsError>;
}

#[derive(Clone, Eq, PartialEq)]
pub(super) struct CurrentLawfulNextOperationRead {
    run_id: WorkflowRunId,
    operation_identity: SpecContentHash,
    revision: SpecContentHash,
}

impl CurrentLawfulNextOperationRead {
    pub(super) fn new(
        run_id: WorkflowRunId,
        operation_identity: SpecContentHash,
        revision: SpecContentHash,
    ) -> Self {
        Self {
            run_id,
            operation_identity,
            revision,
        }
    }
}

/// Current accepted-intent reader for the single lawful-next operation.
pub(super) trait CurrentLawfulNextOperationSource {
    fn read_current(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<CurrentLawfulNextOperationRead, WorkflowOsError>;
}

/// Store-backed source that reconstructs dispatch and report facts from
/// current durable state and joins them to one current project-resource read.
pub(super) struct CrossGateReportArtifactInput {
    report_id: WorkReportId,
    locator_class: WorkReportArtifactLocatorClass,
}

impl CrossGateReportArtifactInput {
    pub(super) const fn new(
        report_id: WorkReportId,
        locator_class: WorkReportArtifactLocatorClass,
    ) -> Self {
        Self {
            report_id,
            locator_class,
        }
    }
}

pub(super) struct StoreBackedCrossGateLifecycleSource<'a, S, B, D, O, R> {
    state: &'a S,
    bundles: &'a B,
    dispatches: &'a D,
    operations: &'a O,
    resources: &'a R,
    run_id: WorkflowRunId,
    report_id: WorkReportId,
    locator_class: WorkReportArtifactLocatorClass,
}

impl<'a, S, B, D, O, R> StoreBackedCrossGateLifecycleSource<'a, S, B, D, O, R>
where
    S: RunSnapshotStore + WorkReportArtifactStore,
    B: ImmutableRunBundleStore,
    D: CurrentAcceptedDispatchSource,
    O: CurrentLawfulNextOperationSource,
    R: CurrentProjectResourceAuthoritySource,
{
    pub(super) fn new(
        state: &'a S,
        bundles: &'a B,
        dispatches: &'a D,
        operations: &'a O,
        resources: &'a R,
        run_id: WorkflowRunId,
        report_artifact: CrossGateReportArtifactInput,
    ) -> Self {
        Self {
            state,
            bundles,
            dispatches,
            operations,
            resources,
            run_id,
            report_id: report_artifact.report_id,
            locator_class: report_artifact.locator_class,
        }
    }

    #[allow(clippy::too_many_lines)]
    fn read_current_observation(&self) -> Result<CrossGateLifecycleObservation, WorkflowOsError> {
        let snapshot = self
            .state
            .load_snapshot(&self.run_id)?
            .ok_or_else(|| binding_error("run.missing", "cross-gate run state is unavailable"))?;
        let bundle_binding = snapshot
            .identity
            .immutable_run_bundle
            .as_ref()
            .ok_or_else(|| {
                binding_error(
                    "immutable_bundle.missing",
                    "cross-gate run is not bound to an immutable run bundle",
                )
            })?;
        let stored = self
            .bundles
            .read_exact_bundle(&self.run_id, bundle_binding.bundle_id())?;
        let manifest = stored.manifest();
        if manifest.run_binding() != *bundle_binding
            || manifest.workflow_id() != &snapshot.identity.workflow_id
            || manifest.workflow_version() != &snapshot.identity.workflow_version
            || manifest.schema_version() != &snapshot.identity.schema_version
            || manifest.workflow_content_hash() != &snapshot.identity.spec_content_hash
        {
            return Err(binding_error(
                "immutable_bundle.mismatch",
                "cross-gate immutable run bundle does not match current run identity",
            ));
        }
        let dispatch = self.dispatches.read_current(&self.run_id)?;
        if dispatch.run_id != self.run_id
            || dispatch.immutable_context_commitment != *manifest.resolved_execution_context_hash()
        {
            return Err(binding_error(
                "dispatch.mismatch",
                "current accepted dispatch does not match the immutable run context",
            ));
        }
        let operation = self.operations.read_current(&self.run_id)?;
        if operation.run_id != self.run_id {
            return Err(binding_error(
                "operation.mismatch",
                "current lawful-next operation does not match the cross-gate run",
            ));
        }
        let expectation = WorkReportArtifactLocationExpectation::for_report(
            self.run_id.clone(),
            self.report_id.clone(),
            self.locator_class,
        )
        .with_run_identity(&snapshot.identity);
        let artifact = self
            .state
            .read_work_report_artifact_with_expectation(&expectation)?
            .ok_or_else(|| {
                binding_error(
                    "report.missing",
                    "cross-gate verified work report is unavailable",
                )
            })?;
        let body_commitment = artifact.metadata().body_commitment().ok_or_else(|| {
            binding_error(
                "report.commitment_missing",
                "cross-gate work report requires an external artifact body commitment",
            )
        })?;
        let report_binding = collection_commitment(
            "cross-gate-report-location",
            &(
                artifact.run_id(),
                artifact.report_id(),
                artifact.metadata().workflow_id(),
                artifact.metadata().workflow_version(),
                artifact.metadata().schema_version(),
                artifact.metadata().spec_hash(),
                body_commitment.commitment(),
                locator_class_name(self.locator_class),
            ),
        )?;
        let resource = self.resources.read_current(&self.run_id)?;
        if resource.run_id != self.run_id
            || resource.immutable_context_commitment != *manifest.resolved_execution_context_hash()
        {
            return Err(binding_error(
                "resource_authority.mismatch",
                "current project-resource authority does not match the immutable run context",
            ));
        }
        let revision = collection_commitment(
            "cross-gate-current-source-revision",
            &(
                manifest.root_hash(),
                &dispatch.dispatch_commitments,
                &dispatch.revision,
                &operation.operation_identity,
                &operation.revision,
                &report_binding,
                &resource.revision,
                &resource.resource_restriction_commitment,
            ),
        )?;
        Ok(CrossGateLifecycleObservation::new(
            revision,
            operation.operation_identity,
            AcceptedDispatchObservation::new(
                self.run_id.clone(),
                manifest.resolved_execution_context_hash().clone(),
                dispatch.dispatch_commitments,
            )?,
            VerifiedReportObservation::new(
                self.run_id.clone(),
                manifest.resolved_execution_context_hash().clone(),
                report_binding,
            ),
            CurrentResourceAuthorityObservation::new(
                self.run_id.clone(),
                resource.immutable_context_commitment,
                resource.resource_restriction_commitment,
            ),
        ))
    }
}

impl<S, B, D, O, R> CrossGateLifecycleSource
    for StoreBackedCrossGateLifecycleSource<'_, S, B, D, O, R>
where
    S: RunSnapshotStore + WorkReportArtifactStore,
    B: ImmutableRunBundleStore,
    D: CurrentAcceptedDispatchSource,
    O: CurrentLawfulNextOperationSource,
    R: CurrentProjectResourceAuthoritySource,
{
    fn observe_current(&self) -> Result<CrossGateLifecycleObservation, WorkflowOsError> {
        self.read_current_observation()
    }

    fn current_revision(&self) -> Result<SpecContentHash, WorkflowOsError> {
        Ok(self.read_current_observation()?.revision)
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CrossGateLifecycleBinding {
    version: &'static str,
    run_id: WorkflowRunId,
    immutable_context_commitment: SpecContentHash,
    lawful_next_operation_identity: SpecContentHash,
    dispatch_commitments: Vec<SpecContentHash>,
    report_artifact_location_commitment: SpecContentHash,
    resource_restriction_commitment: SpecContentHash,
    source_revision: SpecContentHash,
    commitment: SpecContentHash,
    non_authorizing: bool,
}

impl CrossGateLifecycleBinding {
    pub(super) fn compose(source: &impl CrossGateLifecycleSource) -> Result<Self, WorkflowOsError> {
        let observation = source.observe_current()?;
        let run_id = &observation.dispatch.run_id;
        let immutable_context = &observation.dispatch.immutable_context_commitment;
        if &observation.report.run_id != run_id || &observation.resource_authority.run_id != run_id
        {
            return Err(binding_error(
                "run.mismatch",
                "cross-gate lifecycle observations do not identify one exact run",
            ));
        }
        if &observation.report.immutable_context_commitment != immutable_context
            || &observation.resource_authority.immutable_context_commitment != immutable_context
        {
            return Err(binding_error(
                "immutable_context.mismatch",
                "cross-gate lifecycle observations do not share one immutable execution context",
            ));
        }
        let current_revision = source.current_revision()?;
        if current_revision != observation.revision {
            return Err(binding_error(
                "source.drifted",
                "cross-gate lifecycle source changed during composition",
            ));
        }
        let commitment = collection_commitment(
            "cross-gate-lifecycle-binding",
            &(
                BINDING_VERSION,
                run_id,
                immutable_context,
                &observation.lawful_next_operation_identity,
                &observation.dispatch.dispatch_commitments,
                &observation.report.artifact_location_commitment,
                &observation
                    .resource_authority
                    .resource_restriction_commitment,
                &observation.revision,
                true,
            ),
        )?;
        Ok(Self {
            version: BINDING_VERSION,
            run_id: run_id.clone(),
            immutable_context_commitment: immutable_context.clone(),
            lawful_next_operation_identity: observation.lawful_next_operation_identity,
            dispatch_commitments: observation.dispatch.dispatch_commitments,
            report_artifact_location_commitment: observation.report.artifact_location_commitment,
            resource_restriction_commitment: observation
                .resource_authority
                .resource_restriction_commitment,
            source_revision: observation.revision,
            commitment,
            non_authorizing: true,
        })
    }

    pub(super) const fn commitment(&self) -> &SpecContentHash {
        &self.commitment
    }

    pub(super) const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    pub(super) const fn lawful_next_operation_identity(&self) -> &SpecContentHash {
        &self.lawful_next_operation_identity
    }

    pub(super) const fn is_non_authorizing(&self) -> bool {
        self.non_authorizing
    }
}

fn binding_error(code: &str, message: impl Into<String>) -> WorkflowOsError {
    collection_error(&format!("cross_gate_lifecycle.{code}"), message)
}

const fn locator_class_name(locator: WorkReportArtifactLocatorClass) -> &'static str {
    match locator {
        WorkReportArtifactLocatorClass::LocalFilesystem => "local_filesystem",
        WorkReportArtifactLocatorClass::Sqlite => "sqlite",
        WorkReportArtifactLocatorClass::Postgres => "postgres",
        WorkReportArtifactLocatorClass::PostgresTransaction => "postgres_transaction",
        WorkReportArtifactLocatorClass::InjectedStore => "injected_store",
    }
}
