use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::Serialize;

use super::{
    hash_serializable, source_error, CurrentAuthorityFactFamily,
    CurrentAuthoritySourceCompleteness, CurrentAuthoritySourceConsistency,
    CurrentAuthoritySourceContractVersion, CurrentAuthoritySourceFactCount,
    CurrentAuthoritySourceFailure, CurrentAuthoritySourceFailureKind,
    CurrentAuthoritySourceFailurePosture, CurrentAuthoritySourceFreshness,
    CurrentAuthoritySourceGeneration, CurrentAuthoritySourceId, CurrentAuthoritySourceKind,
    CurrentAuthoritySourceReadWindow, CurrentAuthoritySourceRegistration,
    CurrentAuthoritySourceRegistrationInput, CurrentAuthoritySourceRequest,
    CurrentAuthoritySourceRequestInput, CurrentAuthoritySourceSnapshot,
    CurrentAuthoritySourceSnapshotId, CurrentAuthoritySourceSnapshotInput,
    CurrentAuthoritySourceWatermark,
};
use crate::{
    capability_authority::grant_matches_execution_scope, consume_required_context,
    project_step_scoped_context, resolve_capability_authority, AuthorityFactCompletenessPosture,
    AuthorityFactSourceKind, CapabilityAvailabilityRecord, CapabilityGrant,
    CapabilityResolutionInput, CapabilityResolutionReason, CurrentAuthorityFactSet,
    CurrentAuthorityFactSetInput, CurrentAuthorityQuerySet, GovernedContextAccessLevel,
    GovernedContextProjectionCandidate, GovernedContextProjectionInput, GovernedContextReference,
    RedactionMetadata, RequiredContextConsumptionContext, RequiredContextConsumptionInput,
    RequiredContextConsumptionPosture, RequiredContextContractBinding,
    RequiredContextExecutionBinding, RequiredContextObligation, SpecContentHash, Timestamp,
    WorkReportArtifactStore, WorkReportId, WorkReportSensitivity, WorkReportStatus,
    WorkflowOsError, WorkflowRunId,
};

const REGISTERED_FACT_FAMILIES: [CurrentAuthorityFactFamily; 3] = [
    CurrentAuthorityFactFamily::CapabilityGrants,
    CurrentAuthorityFactFamily::CapabilityAvailability,
    CurrentAuthorityFactFamily::GovernedContextReferences,
];

pub(super) struct RegisteredInMemoryCurrentAuthoritySourceInput {
    pub(super) source_id: CurrentAuthoritySourceId,
    pub(super) contract_version: CurrentAuthoritySourceContractVersion,
    pub(super) configuration_commitment: SpecContentHash,
    pub(super) core_maximum_observation_age_seconds: u32,
    pub(super) sensitivity: WorkReportSensitivity,
    pub(super) observed_at: Timestamp,
    pub(super) source_valid_through: Option<Timestamp>,
    pub(super) generation: Option<CurrentAuthoritySourceGeneration>,
    pub(super) complete_grant_inventory: Vec<CapabilityGrant>,
    pub(super) complete_availability_inventory: Vec<CapabilityAvailabilityRecord>,
    pub(super) complete_context_reference_inventory: Vec<GovernedContextReference>,
}

pub(super) struct RegisteredCurrentAuthoritySourceReadInput<'a> {
    pub(super) execution_binding: &'a RequiredContextExecutionBinding,
    pub(super) contract: &'a RequiredContextContractBinding,
    pub(super) evaluated_at: Timestamp,
}

pub(super) enum RegisteredCurrentAuthoritySourceReadOutcome {
    Snapshot(Box<CurrentAuthoritySourceSnapshot>),
    Failure(CurrentAuthoritySourceFailure),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum RegisteredCurrentAuthorityResolutionPosture {
    Ready,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum RegisteredCurrentAuthorityResolutionReason {
    Ready,
    RequiredContextGap,
    OptionalContextGap,
    IndependentPolicyRequired,
    IndependentApprovalRequired,
    IndependentEvidenceRequired,
    IndependentCheckRequired,
}

pub(super) struct RegisteredCurrentAuthorityResolutionInput<'a> {
    pub(super) execution_binding: &'a RequiredContextExecutionBinding,
    pub(super) contract: &'a RequiredContextContractBinding,
    pub(super) evaluated_at: Timestamp,
    pub(super) redaction: &'a RedactionMetadata,
}

pub(super) enum RegisteredCurrentAuthorityResolutionOutcome {
    Assessment(Box<RegisteredCurrentAuthorityResolutionAssessment>),
    SourceFailure(CurrentAuthoritySourceFailure),
}

pub(super) struct RegisteredCurrentAuthorityUseInput<'a> {
    pub(super) execution_binding: &'a RequiredContextExecutionBinding,
    pub(super) contract: &'a RequiredContextContractBinding,
    pub(super) evaluated_at: Timestamp,
    pub(super) redaction: &'a RedactionMetadata,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RegisteredCurrentAuthorityUsePosture {
    BlockedBeforeUse,
    ConsumerSucceeded,
    ConsumerFailed,
    ConsumerOutcomeAmbiguous,
    SourceFailure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RegisteredCurrentAuthorityConsumerResult {
    Succeeded,
    Failed,
    OutcomeAmbiguous,
}

pub(super) struct RegisteredCurrentAuthorityUseOutcome {
    posture: RegisteredCurrentAuthorityUsePosture,
    reasons: Vec<RegisteredCurrentAuthorityResolutionReason>,
    source_failure_kind: Option<CurrentAuthoritySourceFailureKind>,
    source_failure_posture: Option<CurrentAuthoritySourceFailurePosture>,
}

pub(super) struct RegisteredCurrentAuthorityUseCapability<'call> {
    assessment: &'call RegisteredCurrentAuthorityResolutionAssessment,
}

pub(super) struct CurrentAuthorityWorkReportMetadataReadInput<'a> {
    pub(super) execution_binding: &'a RequiredContextExecutionBinding,
    pub(super) contract: &'a RequiredContextContractBinding,
    pub(super) report_id: &'a WorkReportId,
    pub(super) evaluated_at: Timestamp,
    pub(super) redaction: &'a RedactionMetadata,
}

#[derive(Clone, Eq, PartialEq)]
pub(super) struct CurrentAuthorityWorkReportMetadataView {
    report_id: WorkReportId,
    run_id: WorkflowRunId,
    terminal_run_status: WorkReportStatus,
    sensitivity: WorkReportSensitivity,
}

pub(super) enum CurrentAuthorityWorkReportMetadataReadOutcome {
    Found(CurrentAuthorityWorkReportMetadataView),
    NotFound,
    Blocked(Vec<RegisteredCurrentAuthorityResolutionReason>),
    SourceFailure {
        kind: CurrentAuthoritySourceFailureKind,
        posture: CurrentAuthoritySourceFailurePosture,
    },
    StoreFailure,
}

enum CapturedWorkReportMetadataRead {
    Found(CurrentAuthorityWorkReportMetadataView),
    NotFound,
    StoreFailure,
    InvariantFailure,
}

pub(super) struct RegisteredCurrentAuthorityResolutionAssessment {
    posture: RegisteredCurrentAuthorityResolutionPosture,
    reasons: Vec<RegisteredCurrentAuthorityResolutionReason>,
    consumption: crate::RequiredContextConsumptionResult,
    source_snapshot_commitment: SpecContentHash,
    fact_set_commitment: SpecContentHash,
    evaluated_at: Timestamp,
    assessment_commitment: SpecContentHash,
}

struct RegisteredCurrentAuthoritySourceSelection {
    snapshot: Box<CurrentAuthoritySourceSnapshot>,
    grants: Vec<CapabilityGrant>,
    availability_records: Vec<CapabilityAvailabilityRecord>,
    context_references: Vec<GovernedContextReference>,
}

enum RegisteredCurrentAuthoritySourceSelectionOutcome {
    Selection(RegisteredCurrentAuthoritySourceSelection),
    Failure(CurrentAuthoritySourceFailure),
}

struct RegisteredResolvedProjectionCandidates {
    by_access: BTreeMap<GovernedContextAccessLevel, Vec<GovernedContextProjectionCandidate>>,
    reasons: BTreeSet<RegisteredCurrentAuthorityResolutionReason>,
}

pub(super) struct RegisteredInMemoryCurrentAuthoritySource {
    registration: CurrentAuthoritySourceRegistration,
    observed_at: Timestamp,
    source_valid_through: Option<Timestamp>,
    generation: Option<CurrentAuthoritySourceGeneration>,
    grants: Vec<CapabilityGrant>,
    availability_records: Vec<CapabilityAvailabilityRecord>,
    context_references: Vec<GovernedContextReference>,
    inventory_commitment: SpecContentHash,
}

impl RegisteredCurrentAuthorityResolutionAssessment {
    pub(super) const fn posture(&self) -> RegisteredCurrentAuthorityResolutionPosture {
        self.posture
    }

    pub(super) fn reasons(&self) -> &[RegisteredCurrentAuthorityResolutionReason] {
        &self.reasons
    }

    pub(super) const fn consumption(&self) -> &crate::RequiredContextConsumptionResult {
        &self.consumption
    }

    pub(super) const fn source_snapshot_commitment(&self) -> &SpecContentHash {
        &self.source_snapshot_commitment
    }

    pub(super) const fn fact_set_commitment(&self) -> &SpecContentHash {
        &self.fact_set_commitment
    }

    pub(super) const fn assessment_commitment(&self) -> &SpecContentHash {
        &self.assessment_commitment
    }
}

impl RegisteredCurrentAuthorityUseOutcome {
    pub(super) const fn posture(&self) -> RegisteredCurrentAuthorityUsePosture {
        self.posture
    }

    pub(super) fn reasons(&self) -> &[RegisteredCurrentAuthorityResolutionReason] {
        &self.reasons
    }

    pub(super) const fn source_failure_kind(&self) -> Option<CurrentAuthoritySourceFailureKind> {
        self.source_failure_kind
    }

    pub(super) const fn source_failure_posture(
        &self,
    ) -> Option<CurrentAuthoritySourceFailurePosture> {
        self.source_failure_posture
    }

    fn from_assessment(
        posture: RegisteredCurrentAuthorityUsePosture,
        assessment: &RegisteredCurrentAuthorityResolutionAssessment,
    ) -> Self {
        Self {
            posture,
            reasons: assessment.reasons.clone(),
            source_failure_kind: None,
            source_failure_posture: None,
        }
    }

    fn from_source_failure(failure: &CurrentAuthoritySourceFailure) -> Self {
        Self {
            posture: RegisteredCurrentAuthorityUsePosture::SourceFailure,
            reasons: Vec::new(),
            source_failure_kind: Some(failure.kind()),
            source_failure_posture: Some(failure.posture()),
        }
    }
}

impl fmt::Debug for RegisteredCurrentAuthorityUseOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RegisteredCurrentAuthorityUseOutcome")
            .field("posture", &self.posture)
            .field("reasons", &self.reasons)
            .field("source_failure_kind", &self.source_failure_kind)
            .field("source_failure_posture", &self.source_failure_posture)
            .finish()
    }
}

impl fmt::Debug for RegisteredCurrentAuthorityUseCapability<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RegisteredCurrentAuthorityUseCapability")
            .field("posture", &self.assessment.posture)
            .field("reason_count", &self.assessment.reasons.len())
            .field("assessment_commitment", &"[REDACTED]")
            .finish()
    }
}

impl RegisteredCurrentAuthorityUseCapability<'_> {
    fn work_report_metadata_sensitivity_ceiling(
        &self,
        report_id: &WorkReportId,
    ) -> Option<WorkReportSensitivity> {
        let consumption = self.assessment.consumption();
        let requirement = consumption
            .contract()
            .requirements()
            .iter()
            .find(|requirement| {
                requirement.target()
                    == &crate::GovernedContextReferenceTarget::WorkReport(report_id.clone())
            });
        let requirement = requirement?;
        (requirement.access_level() == GovernedContextAccessLevel::BoundedMetadata
            && requirement.obligation() == RequiredContextObligation::Required
            && consumption.satisfactions().iter().any(|satisfaction| {
                satisfaction.requirement_id() == requirement.requirement_id()
                    && satisfaction.access_level() == GovernedContextAccessLevel::BoundedMetadata
            }))
        .then_some(requirement.maximum_sensitivity())
    }
}

impl CurrentAuthorityWorkReportMetadataView {
    pub(super) const fn report_id(&self) -> &WorkReportId {
        &self.report_id
    }

    pub(super) const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    pub(super) const fn terminal_run_status(&self) -> WorkReportStatus {
        self.terminal_run_status
    }

    pub(super) const fn sensitivity(&self) -> WorkReportSensitivity {
        self.sensitivity
    }
}

impl fmt::Debug for CurrentAuthorityWorkReportMetadataView {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CurrentAuthorityWorkReportMetadataView")
            .field("report_id", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("terminal_run_status", &self.terminal_run_status)
            .field("sensitivity", &self.sensitivity)
            .finish()
    }
}

impl fmt::Debug for CurrentAuthorityWorkReportMetadataReadOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Found(view) => formatter.debug_tuple("Found").field(view).finish(),
            Self::NotFound => formatter.write_str("NotFound"),
            Self::Blocked(reasons) => formatter.debug_tuple("Blocked").field(reasons).finish(),
            Self::SourceFailure { kind, posture } => formatter
                .debug_struct("SourceFailure")
                .field("kind", kind)
                .field("posture", posture)
                .finish(),
            Self::StoreFailure => formatter.write_str("StoreFailure"),
        }
    }
}

impl fmt::Debug for RegisteredCurrentAuthorityResolutionAssessment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RegisteredCurrentAuthorityResolutionAssessment")
            .field("posture", &self.posture)
            .field("reasons", &self.reasons)
            .field("consumption_posture", &self.consumption.posture())
            .field("source_snapshot_commitment", &"[REDACTED]")
            .field("fact_set_commitment", &"[REDACTED]")
            .field("evaluated_at", &"[REDACTED]")
            .field("assessment_commitment", &"[REDACTED]")
            .finish()
    }
}

impl RegisteredInMemoryCurrentAuthoritySource {
    pub(super) fn register(
        input: RegisteredInMemoryCurrentAuthoritySourceInput,
    ) -> Result<Self, WorkflowOsError> {
        let registration =
            CurrentAuthoritySourceRegistration::new(CurrentAuthoritySourceRegistrationInput {
                source_id: input.source_id,
                contract_version: input.contract_version,
                source_kind: CurrentAuthoritySourceKind::LocalAggregate,
                configuration_commitment: input.configuration_commitment,
                supported_fact_families: REGISTERED_FACT_FAMILIES.to_vec(),
                consistency: CurrentAuthoritySourceConsistency::AtomicSnapshot,
                core_maximum_observation_age_seconds: input.core_maximum_observation_age_seconds,
                sensitivity: input.sensitivity,
                redaction_required: true,
            })?;

        validate_source_validity(input.observed_at, input.source_valid_through)?;
        let grants = canonical_grants(input.complete_grant_inventory, input.observed_at)?;
        let availability_records =
            canonical_availability(input.complete_availability_inventory, input.observed_at)?;
        let context_references =
            canonical_context_references(input.complete_context_reference_inventory)?;

        let inventory_commitment = hash_serializable(
            "registered-in-memory-inventory",
            &(
                input.observed_at,
                input.source_valid_through,
                input.generation,
                &grants,
                &availability_records,
                &context_references,
            ),
        )?;

        Ok(Self {
            registration,
            observed_at: input.observed_at,
            source_valid_through: input.source_valid_through,
            generation: input.generation,
            grants,
            availability_records,
            context_references,
            inventory_commitment,
        })
    }

    pub(super) fn read(
        &self,
        input: &RegisteredCurrentAuthoritySourceReadInput<'_>,
    ) -> Result<RegisteredCurrentAuthoritySourceReadOutcome, WorkflowOsError> {
        Ok(match self.read_selection(input)? {
            RegisteredCurrentAuthoritySourceSelectionOutcome::Selection(selection) => {
                RegisteredCurrentAuthoritySourceReadOutcome::Snapshot(selection.snapshot)
            }
            RegisteredCurrentAuthoritySourceSelectionOutcome::Failure(failure) => {
                RegisteredCurrentAuthoritySourceReadOutcome::Failure(failure)
            }
        })
    }

    pub(super) fn resolve_current_authority(
        &self,
        input: &RegisteredCurrentAuthorityResolutionInput<'_>,
    ) -> Result<RegisteredCurrentAuthorityResolutionOutcome, WorkflowOsError> {
        let selection = match self.read_selection(&RegisteredCurrentAuthoritySourceReadInput {
            execution_binding: input.execution_binding,
            contract: input.contract,
            evaluated_at: input.evaluated_at,
        })? {
            RegisteredCurrentAuthoritySourceSelectionOutcome::Selection(selection) => selection,
            RegisteredCurrentAuthoritySourceSelectionOutcome::Failure(failure) => {
                return Ok(RegisteredCurrentAuthorityResolutionOutcome::SourceFailure(
                    failure,
                ));
            }
        };

        let fact_set = CurrentAuthorityFactSet::new(CurrentAuthorityFactSetInput {
            execution_binding: input.execution_binding,
            contract: input.contract,
            source_kind: AuthorityFactSourceKind::InMemoryInventorySnapshot,
            source_snapshot_hash: selection.snapshot.snapshot_commitment().clone(),
            source_observed_at: self.observed_at,
            completeness: AuthorityFactCompletenessPosture::CompleteForExactQuery,
            evaluated_at: input.evaluated_at,
            grants: selection.grants,
            availability_records: selection.availability_records,
        })
        .map_err(|_| {
            registered_source_error(
                "resolution.fact_set_invalid",
                "registered current authority resolution fact set is invalid",
            )
        })?;
        let resolved = resolve_registered_projection_candidates(
            input,
            &fact_set,
            &selection.context_references,
            self.observed_at,
        )?;
        let projections = project_registered_current_context(input, resolved.by_access)?;
        let mut reasons = resolved.reasons;
        let context = RequiredContextConsumptionContext::new(
            input.execution_binding.actor().clone(),
            input.execution_binding.workflow_id().clone(),
            input.execution_binding.run_id().clone(),
            input.execution_binding.step_id().clone(),
            input.execution_binding.harness_contract_id().clone(),
            input.evaluated_at,
        );
        let consumption = consume_required_context(&RequiredContextConsumptionInput {
            contract: input.contract,
            context: &context,
            projections: &projections,
        })
        .map_err(|_| {
            registered_source_error(
                "resolution.consumption_failed",
                "registered current authority required-context consumption failed",
            )
        })?;
        for gap in consumption.gaps() {
            reasons.insert(match gap.obligation() {
                RequiredContextObligation::Required => {
                    RegisteredCurrentAuthorityResolutionReason::RequiredContextGap
                }
                RequiredContextObligation::Optional => {
                    RegisteredCurrentAuthorityResolutionReason::OptionalContextGap
                }
            });
        }
        let assessment = build_registered_resolution_assessment(
            input,
            selection.snapshot.snapshot_commitment().clone(),
            &fact_set,
            consumption,
            reasons,
        )?;
        Ok(RegisteredCurrentAuthorityResolutionOutcome::Assessment(
            Box::new(assessment),
        ))
    }

    pub(super) fn use_current_authority<F>(
        &self,
        input: &RegisteredCurrentAuthorityUseInput<'_>,
        consumer: F,
    ) -> Result<RegisteredCurrentAuthorityUseOutcome, WorkflowOsError>
    where
        F: FnOnce(
            &RegisteredCurrentAuthorityUseCapability<'_>,
        ) -> RegisteredCurrentAuthorityConsumerResult,
    {
        let outcome =
            self.resolve_current_authority(&RegisteredCurrentAuthorityResolutionInput {
                execution_binding: input.execution_binding,
                contract: input.contract,
                evaluated_at: input.evaluated_at,
                redaction: input.redaction,
            })?;
        let assessment = match outcome {
            RegisteredCurrentAuthorityResolutionOutcome::SourceFailure(failure) => {
                return Ok(RegisteredCurrentAuthorityUseOutcome::from_source_failure(
                    &failure,
                ));
            }
            RegisteredCurrentAuthorityResolutionOutcome::Assessment(assessment) => assessment,
        };
        if assessment.posture != RegisteredCurrentAuthorityResolutionPosture::Ready {
            return Ok(RegisteredCurrentAuthorityUseOutcome::from_assessment(
                RegisteredCurrentAuthorityUsePosture::BlockedBeforeUse,
                &assessment,
            ));
        }

        let capability = RegisteredCurrentAuthorityUseCapability {
            assessment: &assessment,
        };
        let posture = match consumer(&capability) {
            RegisteredCurrentAuthorityConsumerResult::Succeeded => {
                RegisteredCurrentAuthorityUsePosture::ConsumerSucceeded
            }
            RegisteredCurrentAuthorityConsumerResult::Failed => {
                RegisteredCurrentAuthorityUsePosture::ConsumerFailed
            }
            RegisteredCurrentAuthorityConsumerResult::OutcomeAmbiguous => {
                RegisteredCurrentAuthorityUsePosture::ConsumerOutcomeAmbiguous
            }
        };
        Ok(RegisteredCurrentAuthorityUseOutcome::from_assessment(
            posture,
            &assessment,
        ))
    }

    pub(super) fn read_work_report_metadata_with_current_authority(
        &self,
        input: &CurrentAuthorityWorkReportMetadataReadInput<'_>,
        store: &dyn WorkReportArtifactStore,
    ) -> Result<CurrentAuthorityWorkReportMetadataReadOutcome, WorkflowOsError> {
        validate_work_report_metadata_read_input(input)?;
        let mut captured = None;
        let use_outcome = self.use_current_authority(
            &RegisteredCurrentAuthorityUseInput {
                execution_binding: input.execution_binding,
                contract: input.contract,
                evaluated_at: input.evaluated_at,
                redaction: input.redaction,
            },
            |capability| {
                let read = capture_work_report_metadata_read(capability, input, store);
                let consumer_result = consumer_result_for_metadata_read(&read);
                captured = Some(read);
                consumer_result
            },
        )?;
        reconcile_work_report_metadata_read(&use_outcome, captured)
    }

    fn read_selection(
        &self,
        input: &RegisteredCurrentAuthoritySourceReadInput<'_>,
    ) -> Result<RegisteredCurrentAuthoritySourceSelectionOutcome, WorkflowOsError> {
        let request = CurrentAuthoritySourceRequest::new(CurrentAuthoritySourceRequestInput {
            registration: &self.registration,
            execution_binding: input.execution_binding,
            contract: input.contract,
            requested_fact_families: REGISTERED_FACT_FAMILIES.to_vec(),
            evaluated_at: input.evaluated_at,
        })?;

        if self.observed_at > input.evaluated_at {
            return Ok(RegisteredCurrentAuthoritySourceSelectionOutcome::Failure(
                self.failure(
                    &request,
                    CurrentAuthoritySourceFailureKind::FutureDated,
                    CurrentAuthoritySourceFailurePosture::RetryableAfterSourceChange,
                ),
            ));
        }

        let query_set = CurrentAuthorityQuerySet::from_contract(input.contract).map_err(|_| {
            registered_source_error(
                "query.invalid",
                "registered current authority source request is invalid",
            )
        })?;
        let grants = self.matching_grants(&query_set, input.execution_binding);
        let (availability_records, context_references) =
            match self.exact_target_records(&query_set, input.contract) {
                Ok(records) => records,
                Err(kind) => {
                    let posture = if kind == CurrentAuthoritySourceFailureKind::QueryMismatch {
                        CurrentAuthoritySourceFailurePosture::Terminal
                    } else {
                        CurrentAuthoritySourceFailurePosture::RetryableAfterSourceChange
                    };
                    return Ok(RegisteredCurrentAuthoritySourceSelectionOutcome::Failure(
                        self.failure(&request, kind, posture),
                    ));
                }
            };
        let snapshot = self.snapshot(
            &request,
            &grants,
            &availability_records,
            &context_references,
        )?;

        match snapshot.freshness() {
            CurrentAuthoritySourceFreshness::Fresh => {
                Ok(RegisteredCurrentAuthoritySourceSelectionOutcome::Selection(
                    RegisteredCurrentAuthoritySourceSelection {
                        snapshot: Box::new(snapshot),
                        grants,
                        availability_records,
                        context_references,
                    },
                ))
            }
            CurrentAuthoritySourceFreshness::Stale => Ok(
                RegisteredCurrentAuthoritySourceSelectionOutcome::Failure(self.failure(
                    &request,
                    CurrentAuthoritySourceFailureKind::Stale,
                    CurrentAuthoritySourceFailurePosture::RetryableAfterSourceChange,
                )),
            ),
            CurrentAuthoritySourceFreshness::FutureDated => Ok(
                RegisteredCurrentAuthoritySourceSelectionOutcome::Failure(self.failure(
                    &request,
                    CurrentAuthoritySourceFailureKind::FutureDated,
                    CurrentAuthoritySourceFailurePosture::RetryableAfterSourceChange,
                )),
            ),
        }
    }

    fn matching_grants(
        &self,
        query_set: &CurrentAuthorityQuerySet,
        binding: &RequiredContextExecutionBinding,
    ) -> Vec<CapabilityGrant> {
        self.grants
            .iter()
            .filter(|grant| {
                query_set.queries().iter().any(|query| {
                    grant.capability() == query.capability()
                        && grant.resource() == query.resource()
                        && grant_matches_execution_scope(
                            grant,
                            binding.actor(),
                            binding.workflow_id(),
                            binding.run_id(),
                            binding.step_id(),
                            Some(binding.harness_contract_id()),
                        )
                })
            })
            .cloned()
            .collect()
    }

    fn exact_target_records(
        &self,
        query_set: &CurrentAuthorityQuerySet,
        contract: &RequiredContextContractBinding,
    ) -> Result<
        (
            Vec<CapabilityAvailabilityRecord>,
            Vec<GovernedContextReference>,
        ),
        CurrentAuthoritySourceFailureKind,
    > {
        let mut availability_records = Vec::with_capacity(query_set.queries().len());
        let mut context_references = Vec::with_capacity(query_set.queries().len());
        for query in query_set.queries() {
            let requirement = contract
                .requirements()
                .iter()
                .find(|requirement| requirement.requirement_id() == query.requirement_id())
                .ok_or(CurrentAuthoritySourceFailureKind::QueryMismatch)?;
            let availability = self
                .availability_records
                .iter()
                .find(|record| {
                    record.capability() == query.capability()
                        && record.resource() == query.resource()
                })
                .cloned()
                .ok_or(CurrentAuthoritySourceFailureKind::Incomplete)?;
            availability_records.push(availability);
            let reference = self
                .context_references
                .iter()
                .find(|reference| reference.target() == requirement.target())
                .cloned()
                .ok_or(CurrentAuthoritySourceFailureKind::Incomplete)?;
            context_references.push(reference);
        }
        Ok((availability_records, context_references))
    }

    fn snapshot(
        &self,
        request: &CurrentAuthoritySourceRequest,
        grants: &[CapabilityGrant],
        availability_records: &[CapabilityAvailabilityRecord],
        context_references: &[GovernedContextReference],
    ) -> Result<CurrentAuthoritySourceSnapshot, WorkflowOsError> {
        let records_commitment = hash_serializable(
            "registered-in-memory-exact-records",
            &(
                request.request_commitment(),
                grants,
                availability_records,
                context_references,
            ),
        )?;
        CurrentAuthoritySourceSnapshot::new(CurrentAuthoritySourceSnapshotInput {
            request,
            registration: &self.registration,
            snapshot_id: CurrentAuthoritySourceSnapshotId::new(format!(
                "snapshot/{}",
                records_commitment.as_str()
            ))?,
            watermark: CurrentAuthoritySourceWatermark::new(format!(
                "watermark/{}",
                self.inventory_commitment.as_str()
            ))?,
            generation: self.generation,
            read_window: CurrentAuthoritySourceReadWindow::new(
                self.observed_at,
                self.observed_at,
                self.observed_at,
            )?,
            completeness: CurrentAuthoritySourceCompleteness::CompleteForExactQuery,
            consistency: CurrentAuthoritySourceConsistency::AtomicSnapshot,
            source_valid_through: self.source_valid_through,
            returned_fact_families: REGISTERED_FACT_FAMILIES.to_vec(),
            fact_counts: vec![
                CurrentAuthoritySourceFactCount::new(
                    CurrentAuthorityFactFamily::CapabilityGrants,
                    u64::try_from(grants.len()).unwrap_or(u64::MAX),
                ),
                CurrentAuthoritySourceFactCount::new(
                    CurrentAuthorityFactFamily::CapabilityAvailability,
                    u64::try_from(availability_records.len()).unwrap_or(u64::MAX),
                ),
                CurrentAuthoritySourceFactCount::new(
                    CurrentAuthorityFactFamily::GovernedContextReferences,
                    u64::try_from(context_references.len()).unwrap_or(u64::MAX),
                ),
            ],
            records_commitment,
        })
    }

    fn failure(
        &self,
        request: &CurrentAuthoritySourceRequest,
        kind: CurrentAuthoritySourceFailureKind,
        posture: CurrentAuthoritySourceFailurePosture,
    ) -> CurrentAuthoritySourceFailure {
        CurrentAuthoritySourceFailure::new(
            self.registration.registration_commitment().clone(),
            request.request_commitment().clone(),
            kind,
            posture,
        )
    }
}

fn resolve_registered_projection_candidates(
    input: &RegisteredCurrentAuthorityResolutionInput<'_>,
    fact_set: &CurrentAuthorityFactSet,
    references: &[GovernedContextReference],
    source_observed_at: Timestamp,
) -> Result<RegisteredResolvedProjectionCandidates, WorkflowOsError> {
    let mut by_access =
        BTreeMap::<GovernedContextAccessLevel, Vec<GovernedContextProjectionCandidate>>::new();
    let mut reasons = BTreeSet::new();
    for requirement in input.contract.requirements() {
        let reference = references
            .iter()
            .find(|reference| reference.target() == requirement.target())
            .cloned()
            .ok_or_else(|| {
                registered_source_error(
                    "resolution.reference_missing",
                    "registered current authority resolution is missing an exact reference",
                )
            })?;
        let capability = requirement.access_level().required_capability()?;
        let resource = requirement.target().capability_resource()?;
        let resolution = resolve_capability_authority(&CapabilityResolutionInput {
            capability: &capability,
            resource: &resource,
            actor: input.execution_binding.actor(),
            workflow_id: input.execution_binding.workflow_id(),
            run_id: input.execution_binding.run_id(),
            step_id: input.execution_binding.step_id(),
            harness_contract_id: Some(input.execution_binding.harness_contract_id()),
            requested_sensitivity: reference.sensitivity(),
            evaluated_at: input.evaluated_at,
            availability_records: fact_set.availability_records(),
            grants: fact_set.grants(),
        })
        .map_err(|_| {
            registered_source_error(
                "resolution.capability_failed",
                "registered current authority capability resolution failed",
            )
        })?;
        add_registered_prerequisite_reasons(&mut reasons, resolution.reasons());
        let candidate = GovernedContextProjectionCandidate::new(
            reference,
            source_observed_at,
            requirement.access_level(),
            resolution,
        )
        .map_err(|_| {
            registered_source_error(
                "resolution.projection_candidate_invalid",
                "registered current authority projection candidate is invalid",
            )
        })?;
        by_access
            .entry(requirement.access_level())
            .or_default()
            .push(candidate);
    }
    Ok(RegisteredResolvedProjectionCandidates { by_access, reasons })
}

fn project_registered_current_context(
    input: &RegisteredCurrentAuthorityResolutionInput<'_>,
    candidates_by_access: BTreeMap<
        GovernedContextAccessLevel,
        Vec<GovernedContextProjectionCandidate>,
    >,
) -> Result<Vec<crate::GovernedContextProjection>, WorkflowOsError> {
    let mut projections = Vec::with_capacity(candidates_by_access.len());
    for (access_level, candidates) in candidates_by_access {
        projections.push(
            project_step_scoped_context(&GovernedContextProjectionInput {
                actor: input.execution_binding.actor(),
                workflow_id: input.execution_binding.workflow_id(),
                run_id: input.execution_binding.run_id(),
                step_id: input.execution_binding.step_id(),
                harness_contract_id: Some(input.execution_binding.harness_contract_id()),
                projected_at: input.evaluated_at,
                maximum_allowed_sensitivity: input.execution_binding.maximum_sensitivity(),
                requested_access_level: access_level,
                candidates: &candidates,
                redaction: input.redaction,
            })
            .map_err(|_| {
                registered_source_error(
                    "resolution.projection_failed",
                    "registered current authority context projection failed",
                )
            })?,
        );
    }
    Ok(projections)
}

fn build_registered_resolution_assessment(
    input: &RegisteredCurrentAuthorityResolutionInput<'_>,
    source_snapshot_commitment: SpecContentHash,
    fact_set: &CurrentAuthorityFactSet,
    consumption: crate::RequiredContextConsumptionResult,
    mut reasons: BTreeSet<RegisteredCurrentAuthorityResolutionReason>,
) -> Result<RegisteredCurrentAuthorityResolutionAssessment, WorkflowOsError> {
    let posture = match consumption.posture() {
        RequiredContextConsumptionPosture::Satisfied => {
            RegisteredCurrentAuthorityResolutionPosture::Ready
        }
        RequiredContextConsumptionPosture::Blocked => {
            RegisteredCurrentAuthorityResolutionPosture::Blocked
        }
    };
    if reasons.is_empty() {
        reasons.insert(RegisteredCurrentAuthorityResolutionReason::Ready);
    }
    let reasons = reasons.into_iter().collect::<Vec<_>>();
    let fact_set_commitment = fact_set.fact_set_hash().clone();
    let assessment_commitment = hash_serializable(
        "registered-current-authority-resolution",
        &(
            input.execution_binding.binding_hash(),
            input.contract.content_hash(),
            &source_snapshot_commitment,
            &fact_set_commitment,
            input.evaluated_at,
            posture,
            &reasons,
            &consumption,
        ),
    )?;
    Ok(RegisteredCurrentAuthorityResolutionAssessment {
        posture,
        reasons,
        consumption,
        source_snapshot_commitment,
        fact_set_commitment,
        evaluated_at: input.evaluated_at,
        assessment_commitment,
    })
}

fn add_registered_prerequisite_reasons(
    reasons: &mut BTreeSet<RegisteredCurrentAuthorityResolutionReason>,
    resolution_reasons: &[CapabilityResolutionReason],
) {
    for reason in resolution_reasons {
        let mapped = match reason {
            CapabilityResolutionReason::PolicyEvaluationRequired => {
                Some(RegisteredCurrentAuthorityResolutionReason::IndependentPolicyRequired)
            }
            CapabilityResolutionReason::ApprovalEvaluationRequired => {
                Some(RegisteredCurrentAuthorityResolutionReason::IndependentApprovalRequired)
            }
            CapabilityResolutionReason::EvidenceEvaluationRequired => {
                Some(RegisteredCurrentAuthorityResolutionReason::IndependentEvidenceRequired)
            }
            CapabilityResolutionReason::CheckEvaluationRequired => {
                Some(RegisteredCurrentAuthorityResolutionReason::IndependentCheckRequired)
            }
            _ => None,
        };
        if let Some(mapped) = mapped {
            reasons.insert(mapped);
        }
    }
}

fn validate_source_validity(
    observed_at: Timestamp,
    source_valid_through: Option<Timestamp>,
) -> Result<(), WorkflowOsError> {
    if source_valid_through.is_some_and(|valid_through| valid_through < observed_at) {
        return Err(registered_source_error(
            "inventory.validity_invalid",
            "registered current authority source validity is invalid",
        ));
    }
    Ok(())
}

fn canonical_grants(
    mut grants: Vec<CapabilityGrant>,
    observed_at: Timestamp,
) -> Result<Vec<CapabilityGrant>, WorkflowOsError> {
    for grant in &grants {
        grant.validate().map_err(|_| {
            registered_source_error(
                "inventory.grant_invalid",
                "registered current authority source contains an invalid grant",
            )
        })?;
        if grant.issued_at() > observed_at {
            return Err(registered_source_error(
                "inventory.time_invalid",
                "registered current authority source inventory time is invalid",
            ));
        }
    }
    grants.sort_by(|left, right| left.grant_id().as_str().cmp(right.grant_id().as_str()));
    if grants
        .windows(2)
        .any(|pair| pair[0].grant_id() == pair[1].grant_id())
    {
        return Err(registered_source_error(
            "inventory.grant_duplicate",
            "registered current authority source contains duplicate grants",
        ));
    }
    Ok(grants)
}

fn canonical_availability(
    mut records: Vec<CapabilityAvailabilityRecord>,
    observed_at: Timestamp,
) -> Result<Vec<CapabilityAvailabilityRecord>, WorkflowOsError> {
    if records
        .iter()
        .any(|record| record.observed_at() > observed_at)
    {
        return Err(registered_source_error(
            "inventory.time_invalid",
            "registered current authority source inventory time is invalid",
        ));
    }
    sort_serializable(&mut records)?;
    if records.windows(2).any(|pair| {
        pair[0].capability() == pair[1].capability() && pair[0].resource() == pair[1].resource()
    }) {
        return Err(registered_source_error(
            "inventory.availability_duplicate",
            "registered current authority source contains duplicate availability",
        ));
    }
    Ok(records)
}

fn canonical_context_references(
    mut references: Vec<GovernedContextReference>,
) -> Result<Vec<GovernedContextReference>, WorkflowOsError> {
    for reference in &references {
        reference.validate().map_err(|_| {
            registered_source_error(
                "inventory.context_reference_invalid",
                "registered current authority source contains an invalid context reference",
            )
        })?;
    }
    sort_serializable(&mut references)?;
    if references
        .windows(2)
        .any(|pair| pair[0].target() == pair[1].target())
    {
        return Err(registered_source_error(
            "inventory.context_reference_duplicate",
            "registered current authority source contains duplicate context references",
        ));
    }
    Ok(references)
}

impl fmt::Debug for RegisteredInMemoryCurrentAuthoritySource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RegisteredInMemoryCurrentAuthoritySource")
            .field("registration", &self.registration)
            .field("observed_at", &"[REDACTED]")
            .field(
                "source_valid_through",
                &self.source_valid_through.map(|_| "[REDACTED]"),
            )
            .field("generation", &self.generation)
            .field("grant_count", &self.grants.len())
            .field("availability_count", &self.availability_records.len())
            .field("context_reference_count", &self.context_references.len())
            .field("inventory_commitment", &"[REDACTED]")
            .finish()
    }
}

fn sort_serializable<T: Serialize>(values: &mut [T]) -> Result<(), WorkflowOsError> {
    let mut keyed = values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            serde_json::to_vec(value)
                .map(|bytes| (bytes, index))
                .map_err(|_| {
                    registered_source_error(
                        "inventory.canonicalization_failed",
                        "registered current authority source inventory canonicalization failed",
                    )
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    keyed.sort_by(|left, right| left.0.cmp(&right.0));
    let mut positions = vec![0usize; values.len()];
    for (new_position, (_, old_position)) in keyed.into_iter().enumerate() {
        positions[old_position] = new_position;
    }
    for index in 0..values.len() {
        while positions[index] != index {
            let other = positions[index];
            values.swap(index, other);
            positions.swap(index, other);
        }
    }
    Ok(())
}

fn registered_source_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    source_error(&format!("registered.{suffix}"), message)
}

fn capture_work_report_metadata_read(
    capability: &RegisteredCurrentAuthorityUseCapability<'_>,
    input: &CurrentAuthorityWorkReportMetadataReadInput<'_>,
    store: &dyn WorkReportArtifactStore,
) -> CapturedWorkReportMetadataRead {
    let Some(requirement_sensitivity_ceiling) =
        capability.work_report_metadata_sensitivity_ceiling(input.report_id)
    else {
        return CapturedWorkReportMetadataRead::InvariantFailure;
    };
    match store.read_work_report_artifact(input.execution_binding.run_id(), input.report_id) {
        Ok(Some(artifact)) => {
            capture_valid_work_report_metadata(&artifact, input, requirement_sensitivity_ceiling)
        }
        Ok(None) => CapturedWorkReportMetadataRead::NotFound,
        Err(_) => CapturedWorkReportMetadataRead::StoreFailure,
    }
}

fn capture_valid_work_report_metadata(
    artifact: &crate::WorkReportArtifactRecord,
    input: &CurrentAuthorityWorkReportMetadataReadInput<'_>,
    requirement_sensitivity_ceiling: WorkReportSensitivity,
) -> CapturedWorkReportMetadataRead {
    if artifact.validate().is_err()
        || artifact.report_id() != input.report_id
        || artifact.run_id() != input.execution_binding.run_id()
    {
        return CapturedWorkReportMetadataRead::StoreFailure;
    }
    let metadata = artifact.metadata();
    if metadata.sensitivity() > requirement_sensitivity_ceiling
        || metadata.sensitivity() > input.execution_binding.maximum_sensitivity()
    {
        return CapturedWorkReportMetadataRead::StoreFailure;
    }
    CapturedWorkReportMetadataRead::Found(CurrentAuthorityWorkReportMetadataView {
        report_id: metadata.report_id().clone(),
        run_id: metadata.run_id().clone(),
        terminal_run_status: metadata.terminal_run_status(),
        sensitivity: metadata.sensitivity(),
    })
}

const fn consumer_result_for_metadata_read(
    read: &CapturedWorkReportMetadataRead,
) -> RegisteredCurrentAuthorityConsumerResult {
    match read {
        CapturedWorkReportMetadataRead::Found(_) | CapturedWorkReportMetadataRead::NotFound => {
            RegisteredCurrentAuthorityConsumerResult::Succeeded
        }
        CapturedWorkReportMetadataRead::StoreFailure
        | CapturedWorkReportMetadataRead::InvariantFailure => {
            RegisteredCurrentAuthorityConsumerResult::Failed
        }
    }
}

fn reconcile_work_report_metadata_read(
    use_outcome: &RegisteredCurrentAuthorityUseOutcome,
    captured: Option<CapturedWorkReportMetadataRead>,
) -> Result<CurrentAuthorityWorkReportMetadataReadOutcome, WorkflowOsError> {
    match use_outcome.posture() {
        RegisteredCurrentAuthorityUsePosture::BlockedBeforeUse => {
            ensure_no_captured_read(captured.as_ref())?;
            Ok(CurrentAuthorityWorkReportMetadataReadOutcome::Blocked(
                use_outcome.reasons().to_vec(),
            ))
        }
        RegisteredCurrentAuthorityUsePosture::SourceFailure => {
            ensure_no_captured_read(captured.as_ref())?;
            Ok(
                CurrentAuthorityWorkReportMetadataReadOutcome::SourceFailure {
                    kind: use_outcome.source_failure_kind().ok_or_else(|| {
                        metadata_read_error(
                            "source_failure_missing",
                            "current-authority metadata read source failure is incomplete",
                        )
                    })?,
                    posture: use_outcome.source_failure_posture().ok_or_else(|| {
                        metadata_read_error(
                            "source_failure_posture_missing",
                            "current-authority metadata read source failure posture is incomplete",
                        )
                    })?,
                },
            )
        }
        RegisteredCurrentAuthorityUsePosture::ConsumerSucceeded => match captured {
            Some(CapturedWorkReportMetadataRead::Found(view)) => {
                Ok(CurrentAuthorityWorkReportMetadataReadOutcome::Found(view))
            }
            Some(CapturedWorkReportMetadataRead::NotFound) => {
                Ok(CurrentAuthorityWorkReportMetadataReadOutcome::NotFound)
            }
            _ => Err(metadata_read_error(
                "consumer_result_inconsistent",
                "current-authority metadata read result is inconsistent",
            )),
        },
        RegisteredCurrentAuthorityUsePosture::ConsumerFailed => match captured {
            Some(CapturedWorkReportMetadataRead::StoreFailure) => {
                Ok(CurrentAuthorityWorkReportMetadataReadOutcome::StoreFailure)
            }
            Some(CapturedWorkReportMetadataRead::InvariantFailure) => Err(metadata_read_error(
                "authority_inconsistent",
                "current-authority metadata read authority is inconsistent",
            )),
            _ => Err(metadata_read_error(
                "consumer_result_inconsistent",
                "current-authority metadata read result is inconsistent",
            )),
        },
        RegisteredCurrentAuthorityUsePosture::ConsumerOutcomeAmbiguous => Err(metadata_read_error(
            "consumer_outcome_ambiguous",
            "current-authority metadata read outcome is ambiguous",
        )),
    }
}

fn metadata_read_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    registered_source_error(&format!("work_report_metadata.{suffix}"), message)
}

fn validate_work_report_metadata_read_input(
    input: &CurrentAuthorityWorkReportMetadataReadInput<'_>,
) -> Result<(), WorkflowOsError> {
    input.execution_binding.validate()?;
    if input.execution_binding.harness_contract_id() != input.contract.contract_id()
        || input.execution_binding.harness_contract_version() != input.contract.contract_version()
        || input.execution_binding.contract_content_hash() != input.contract.content_hash()
    {
        return Err(metadata_read_error(
            "contract_mismatch",
            "current-authority metadata read contract does not match execution binding",
        ));
    }
    let requirement = input.contract.requirements().iter().find(|requirement| {
        requirement.target()
            == &crate::GovernedContextReferenceTarget::WorkReport(input.report_id.clone())
    });
    let Some(requirement) = requirement else {
        return Err(metadata_read_error(
            "target_missing",
            "current-authority metadata read target is not declared",
        ));
    };
    if requirement.access_level() != GovernedContextAccessLevel::BoundedMetadata {
        return Err(metadata_read_error(
            "access_insufficient",
            "current-authority metadata read needs bounded metadata access",
        ));
    }
    if requirement.obligation() != RequiredContextObligation::Required {
        return Err(metadata_read_error(
            "obligation_insufficient",
            "current-authority metadata read needs required context",
        ));
    }
    Ok(())
}

fn ensure_no_captured_read(
    captured: Option<&CapturedWorkReportMetadataRead>,
) -> Result<(), WorkflowOsError> {
    if captured.is_some() {
        return Err(metadata_read_error(
            "blocked_result_inconsistent",
            "blocked current-authority metadata read captured an unexpected result",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

    use super::*;
    use crate::{
        build_immutable_run_bundle, load_project, ActorId, ApprovalReferenceId,
        CapabilityAvailability, CapabilityDelegationPosture, CapabilityGrantDefinition,
        CapabilityGrantId, CapabilityGrantLifecycle, CapabilityGrantRequirements,
        CapabilityGrantScope, CorrelationId, EvidenceReferenceId, GovernedContextAccessLevel,
        GovernedContextAvailability, GovernedContextReferenceTarget, HarnessContractId,
        HarnessContractVersion, ImmutableRunBundleBuildRequest, ImmutableRunBundleExecutionPosture,
        ImmutableRunBundleHandlerPosture, ImmutableRunBundleHandlerReference, ImmutableRunBundleId,
        ImmutableRunBundleReferencePosture, ImmutableRunBundleSensitivity,
        ImmutableRunBundleVersion, LocalCheckResultId, LocalImmutableRunBundleStore, PolicyId,
        RedactionMetadata, RequiredContextExecutionBindingInput, RequiredContextObligation,
        RequiredContextRequirement, RequiredContextRequirementId, SchemaVersion, SkillId,
        SkillVersion, StepId, WorkReport, WorkReportArtifactRecord, WorkReportContractId,
        WorkReportContractVersion, WorkReportDefinition, WorkReportGenerationContext,
        WorkReportHandoffNote, WorkReportIncompleteWorkDisclosure, WorkReportKnownLimitation,
        WorkReportRisk, WorkReportSection, WorkReportSectionKind, WorkReportStatus, WorkflowId,
        WorkflowRunId, WorkflowVersion, SUPPORTED_SCHEMA_VERSION,
    };

    static NEXT_ROOT: AtomicU64 = AtomicU64::new(1);

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new(name: &str) -> Self {
            let id = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "workflow-os-registered-authority-source-{name}-{}-{id}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&path);
            fs::create_dir_all(&path).expect("test root");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn write(&self, relative: &str, content: &str) {
            let path = self.0.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("parent");
            }
            fs::write(path, content).expect("fixture");
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn timestamp(value: &str) -> Timestamp {
        Timestamp::parse_rfc3339(value).expect("timestamp")
    }

    fn fixture() -> (
        RequiredContextContractBinding,
        RequiredContextExecutionBinding,
    ) {
        fixture_with_contract_id("harness/context")
    }

    fn fixture_with_contract_id(
        contract_id: &str,
    ) -> (
        RequiredContextContractBinding,
        RequiredContextExecutionBinding,
    ) {
        fixture_with_shape(
            contract_id,
            GovernedContextAccessLevel::BoundedMetadata,
            RequiredContextObligation::Required,
            "run-authority",
        )
    }

    fn fixture_with_shape(
        contract_id: &str,
        metadata_access_level: GovernedContextAccessLevel,
        metadata_obligation: RequiredContextObligation,
        run_id: &str,
    ) -> (
        RequiredContextContractBinding,
        RequiredContextExecutionBinding,
    ) {
        let project_root = TestRoot::new("project");
        let store_root = TestRoot::new("store");
        project_root.write(
            "workflow-os.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nproject:\n  id: authority/project\n  name: Authority Project\n"
            ),
        );
        project_root.write(
            "workflows/build.workflow.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: authority/build\nversion: v1\ndisplay_name: Authority Build\ntriggers:\n  - id: manual\n    kind: manual\nsteps:\n  - id: consume\n    skill_ref:\n      id: local/check\n      version: v1\n    policy_requirements:\n      - id: local/read-only\n    terminal_behavior: fail_workflow\ncancellation_behavior: stop\naudit_requirements:\n  required: true\n  events: [RunCreated]\n  store_references_only: true\nobservability_requirements:\n  metrics: [workflow_latency]\n  tracing: true\n  latency_tracking: true\n"
            ),
        );
        project_root.write(
            "skills/check.skill.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/check\nversion: v1\ndisplay_name: Check\nallowed_capabilities:\n  - name: local.read\ninput_contract:\n  fields:\n    - name: request\n      field_type: string\noutput_contract:\n  fields:\n    - name: summary\n      field_type: string\nfailure_modes:\n  - code: failed\n    description: Failed.\n    retryable: false\naudit_requirements:\n  required: true\n  events: [SkillInvocationRequested]\n  store_references_only: true\nobservability_requirements:\n  metrics: [skill_latency]\n  tracing: true\n  latency_tracking: true\n"
            ),
        );
        project_root.write(
            "policies/read-only.policy.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/read-only\nname: Read only\nrules:\n  - id: allow\n    effect: allow_local\n"
            ),
        );
        let loaded = load_project(project_root.path());
        assert!(!loaded.has_errors(), "{:?}", loaded.diagnostics);
        let project = loaded.bundle.expect("project");
        let built = build_immutable_run_bundle(ImmutableRunBundleBuildRequest {
            project: &project,
            workflow_id: &WorkflowId::new("authority/build").expect("workflow"),
            bundle_id: ImmutableRunBundleId::new("bundle/authority").expect("bundle"),
            bundle_version: ImmutableRunBundleVersion::new("v1").expect("version"),
            run_id: WorkflowRunId::new(run_id).expect("run"),
            resolved_execution_context_hash: SpecContentHash::from_text("context"),
            execution_posture: ImmutableRunBundleExecutionPosture::new(
                Vec::new(),
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::NotSupplied,
            )
            .expect("posture"),
            handlers: vec![ImmutableRunBundleHandlerReference {
                skill_id: SkillId::new("local/check").expect("skill"),
                skill_version: SkillVersion::new("v1").expect("skill version"),
                posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
            }],
            created_at: timestamp("2026-07-26T10:00:00Z"),
            created_by: ActorId::new("system/kernel").expect("actor"),
            sensitivity: ImmutableRunBundleSensitivity::Internal,
            redaction_required: true,
        })
        .expect("built");
        let store = LocalImmutableRunBundleStore::new(store_root.path());
        store.write_bundle(&built).expect("write");
        let stored = store
            .read_bundle(built.manifest().run_id(), built.manifest().bundle_id())
            .expect("read");
        let contract = RequiredContextContractBinding::new(
            HarnessContractId::new(contract_id).expect("contract"),
            HarnessContractVersion::new("v1").expect("contract version"),
            vec![
                RequiredContextRequirement::new(
                    RequiredContextRequirementId::new("required/report-reference")
                        .expect("requirement"),
                    GovernedContextReferenceTarget::WorkReport(
                        WorkReportId::new("report/current").expect("report"),
                    ),
                    GovernedContextAccessLevel::ReferenceOnly,
                    RequiredContextObligation::Required,
                    WorkReportSensitivity::Internal,
                )
                .expect("requirement"),
                RequiredContextRequirement::new(
                    RequiredContextRequirementId::new("required/report-metadata")
                        .expect("requirement"),
                    GovernedContextReferenceTarget::WorkReport(
                        WorkReportId::new("report/metadata").expect("report"),
                    ),
                    metadata_access_level,
                    metadata_obligation,
                    WorkReportSensitivity::Internal,
                )
                .expect("requirement"),
            ],
        )
        .expect("contract");
        let binding = RequiredContextExecutionBinding::new(RequiredContextExecutionBindingInput {
            bundle: &stored,
            contract: &contract,
            actor: ActorId::new("agent/consumer").expect("actor"),
            step_id: StepId::new("consume").expect("step"),
            maximum_sensitivity: WorkReportSensitivity::Internal,
            bound_at: timestamp("2026-07-26T10:10:00Z"),
        })
        .expect("binding");
        (contract, binding)
    }

    fn availability(
        contract: &RequiredContextContractBinding,
    ) -> Vec<CapabilityAvailabilityRecord> {
        contract
            .requirements()
            .iter()
            .map(|requirement| {
                CapabilityAvailabilityRecord::new(
                    requirement
                        .access_level()
                        .required_capability()
                        .expect("capability"),
                    requirement
                        .target()
                        .capability_resource()
                        .expect("resource"),
                    CapabilityAvailability::Available,
                    timestamp("2026-07-26T10:20:00Z"),
                    RedactionMetadata::empty(),
                )
                .expect("availability")
            })
            .collect()
    }

    fn references(contract: &RequiredContextContractBinding) -> Vec<GovernedContextReference> {
        contract
            .requirements()
            .iter()
            .map(|requirement| {
                GovernedContextReference::new(
                    requirement.target().clone(),
                    WorkReportSensitivity::Internal,
                    GovernedContextAvailability::Available,
                    RedactionMetadata::empty(),
                )
                .expect("reference")
            })
            .collect()
    }

    fn grant(contract: &RequiredContextContractBinding) -> CapabilityGrant {
        grant_for(
            contract,
            0,
            CapabilityGrantLifecycle::Active,
            CapabilityGrantRequirements::default(),
        )
    }

    fn grant_for(
        contract: &RequiredContextContractBinding,
        index: usize,
        lifecycle: CapabilityGrantLifecycle,
        requirements: CapabilityGrantRequirements,
    ) -> CapabilityGrant {
        grant_for_with_expiry(contract, index, lifecycle, requirements, None)
    }

    fn grant_for_with_expiry(
        contract: &RequiredContextContractBinding,
        index: usize,
        lifecycle: CapabilityGrantLifecycle,
        requirements: CapabilityGrantRequirements,
        expires_at: Option<Timestamp>,
    ) -> CapabilityGrant {
        let requirement = &contract.requirements()[index];
        CapabilityGrant::new(CapabilityGrantDefinition {
            grant_id: CapabilityGrantId::new(format!("grant/exact-{index}")).expect("grant id"),
            subject: ActorId::new("agent/consumer").expect("actor"),
            capability: requirement
                .access_level()
                .required_capability()
                .expect("capability"),
            resource: requirement
                .target()
                .capability_resource()
                .expect("resource"),
            scope: CapabilityGrantScope::new(
                WorkflowId::new("authority/build").expect("workflow"),
                Some(WorkflowRunId::new("run-authority").expect("run")),
                Some(StepId::new("consume").expect("step")),
                Some(HarnessContractId::new("harness/context").expect("harness")),
            )
            .expect("scope"),
            issuer: ActorId::new("system/authority").expect("issuer"),
            issued_at: timestamp("2026-07-26T10:05:00Z"),
            expires_at,
            lifecycle,
            revocation_reference: (lifecycle == CapabilityGrantLifecycle::Revoked)
                .then(|| "revocation/current".to_owned()),
            delegation: CapabilityDelegationPosture::Disabled,
            requirements,
            sensitivity_ceiling: WorkReportSensitivity::Internal,
            redaction: RedactionMetadata::empty(),
        })
        .expect("grant")
    }

    fn grants_with_first_lifecycle(
        contract: &RequiredContextContractBinding,
        lifecycle: CapabilityGrantLifecycle,
    ) -> Vec<CapabilityGrant> {
        vec![
            grant_for(
                contract,
                0,
                lifecycle,
                CapabilityGrantRequirements::default(),
            ),
            grant_for(
                contract,
                1,
                CapabilityGrantLifecycle::Active,
                CapabilityGrantRequirements::default(),
            ),
        ]
    }

    fn source(
        contract: &RequiredContextContractBinding,
        observed_at: &str,
    ) -> RegisteredInMemoryCurrentAuthoritySource {
        source_with_inventory(
            contract,
            observed_at,
            vec![grant(contract)],
            availability(contract),
            references(contract),
        )
    }

    fn source_with_inventory(
        _contract: &RequiredContextContractBinding,
        observed_at: &str,
        grants: Vec<CapabilityGrant>,
        availability_records: Vec<CapabilityAvailabilityRecord>,
        context_references: Vec<GovernedContextReference>,
    ) -> RegisteredInMemoryCurrentAuthoritySource {
        RegisteredInMemoryCurrentAuthoritySource::register(
            RegisteredInMemoryCurrentAuthoritySourceInput {
                source_id: CurrentAuthoritySourceId::new("authority/local").expect("source"),
                contract_version: CurrentAuthoritySourceContractVersion::new("v1")
                    .expect("version"),
                configuration_commitment: SpecContentHash::from_text("safe configuration"),
                core_maximum_observation_age_seconds: 600,
                sensitivity: WorkReportSensitivity::Internal,
                observed_at: timestamp(observed_at),
                source_valid_through: None,
                generation: Some(CurrentAuthoritySourceGeneration::new(1).expect("generation")),
                complete_grant_inventory: grants,
                complete_availability_inventory: availability_records,
                complete_context_reference_inventory: context_references,
            },
        )
        .expect("source")
    }

    struct InstrumentedWorkReportArtifactStore {
        artifact: Option<WorkReportArtifactRecord>,
        fail_read: bool,
        enforce_identity: bool,
        reads: AtomicUsize,
        writes: AtomicUsize,
        lists: AtomicUsize,
    }

    impl InstrumentedWorkReportArtifactStore {
        fn with_artifact(artifact: WorkReportArtifactRecord) -> Self {
            Self {
                artifact: Some(artifact),
                fail_read: false,
                enforce_identity: true,
                reads: AtomicUsize::new(0),
                writes: AtomicUsize::new(0),
                lists: AtomicUsize::new(0),
            }
        }

        fn with_mismatched_artifact(artifact: WorkReportArtifactRecord) -> Self {
            Self {
                artifact: Some(artifact),
                fail_read: false,
                enforce_identity: false,
                reads: AtomicUsize::new(0),
                writes: AtomicUsize::new(0),
                lists: AtomicUsize::new(0),
            }
        }

        fn empty() -> Self {
            Self {
                artifact: None,
                fail_read: false,
                enforce_identity: true,
                reads: AtomicUsize::new(0),
                writes: AtomicUsize::new(0),
                lists: AtomicUsize::new(0),
            }
        }

        fn failing() -> Self {
            Self {
                artifact: None,
                fail_read: true,
                enforce_identity: true,
                reads: AtomicUsize::new(0),
                writes: AtomicUsize::new(0),
                lists: AtomicUsize::new(0),
            }
        }

        fn read_count(&self) -> usize {
            self.reads.load(Ordering::Relaxed)
        }

        fn write_count(&self) -> usize {
            self.writes.load(Ordering::Relaxed)
        }

        fn list_count(&self) -> usize {
            self.lists.load(Ordering::Relaxed)
        }
    }

    impl WorkReportArtifactStore for InstrumentedWorkReportArtifactStore {
        fn write_work_report_artifact(
            &self,
            _artifact: &WorkReportArtifactRecord,
        ) -> Result<(), WorkflowOsError> {
            self.writes.fetch_add(1, Ordering::Relaxed);
            Err(WorkflowOsError::invalid_state(
                "test.store.write_forbidden",
                "test store write is forbidden",
            ))
        }

        fn read_work_report_artifact(
            &self,
            run_id: &WorkflowRunId,
            report_id: &WorkReportId,
        ) -> Result<Option<WorkReportArtifactRecord>, WorkflowOsError> {
            self.reads.fetch_add(1, Ordering::Relaxed);
            if self.fail_read {
                return Err(WorkflowOsError::invalid_state(
                    "test.store.secret_failure",
                    "store failed for token=secret-like-value",
                ));
            }
            Ok(self.artifact.as_ref().and_then(|artifact| {
                (!self.enforce_identity
                    || (artifact.run_id() == run_id && artifact.report_id() == report_id))
                    .then(|| artifact.clone())
            }))
        }

        fn list_work_report_artifacts(
            &self,
            _run_id: &WorkflowRunId,
        ) -> Result<Vec<WorkReportArtifactRecord>, WorkflowOsError> {
            self.lists.fetch_add(1, Ordering::Relaxed);
            Ok(Vec::new())
        }
    }

    fn work_report_artifact(
        report_id: &str,
        run_id: &str,
        sensitivity: WorkReportSensitivity,
    ) -> WorkReportArtifactRecord {
        let sections = WorkReportSectionKind::v1_required_kinds()
            .into_iter()
            .map(|kind| {
                WorkReportSection::new(
                    kind,
                    Some("bounded metadata fixture".to_owned()),
                    Vec::new(),
                )
                .expect("section")
            })
            .collect();
        let report = WorkReport::new(WorkReportDefinition {
            report_id: WorkReportId::new(report_id).expect("report"),
            report_contract_id: WorkReportContractId::new("report/contract").expect("contract"),
            report_contract_version: WorkReportContractVersion::new("v1").expect("version"),
            generation_context: WorkReportGenerationContext {
                workflow_id: WorkflowId::new("authority/build").expect("workflow"),
                workflow_version: WorkflowVersion::new("v1").expect("workflow version"),
                schema_version: SchemaVersion::new(SUPPORTED_SCHEMA_VERSION).expect("schema"),
                spec_hash: SpecContentHash::from_text("workflow spec"),
                run_id: WorkflowRunId::new(run_id).expect("run"),
                terminal_run_status: WorkReportStatus::Completed,
                generated_at: timestamp("2026-07-26T10:22:00Z"),
                generated_by: ActorId::new("system/report").expect("actor"),
                correlation_id: Some(
                    CorrelationId::new("correlation/authority").expect("correlation"),
                ),
            },
            sections,
            incomplete_work: vec![WorkReportIncompleteWorkDisclosure::new("none", Vec::new())
                .expect("incomplete work")],
            known_limitations: vec![
                WorkReportKnownLimitation::new("none", Vec::new()).expect("known limitation")
            ],
            risks: vec![WorkReportRisk::new("none", Vec::new()).expect("risk")],
            handoff_notes: vec![WorkReportHandoffNote::new("none", Vec::new()).expect("handoff")],
            high_assurance_approval: None,
            sensitivity,
            redaction: RedactionMetadata::empty(),
        })
        .expect("work report");
        WorkReportArtifactRecord::new(report).expect("artifact")
    }

    fn ready_source(
        contract: &RequiredContextContractBinding,
    ) -> RegisteredInMemoryCurrentAuthoritySource {
        source_with_inventory(
            contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for(
                    contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant_for(
                    contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(contract),
            references(contract),
        )
    }

    fn read_work_report_metadata(
        source: &RegisteredInMemoryCurrentAuthoritySource,
        binding: &RequiredContextExecutionBinding,
        contract: &RequiredContextContractBinding,
        report_id: &WorkReportId,
        evaluated_at: &str,
        store: &dyn WorkReportArtifactStore,
    ) -> Result<CurrentAuthorityWorkReportMetadataReadOutcome, WorkflowOsError> {
        source.read_work_report_metadata_with_current_authority(
            &CurrentAuthorityWorkReportMetadataReadInput {
                execution_binding: binding,
                contract,
                report_id,
                evaluated_at: timestamp(evaluated_at),
                redaction: &RedactionMetadata::empty(),
            },
            store,
        )
    }

    fn read(
        source: &RegisteredInMemoryCurrentAuthoritySource,
        binding: &RequiredContextExecutionBinding,
        contract: &RequiredContextContractBinding,
        evaluated_at: &str,
    ) -> RegisteredCurrentAuthoritySourceReadOutcome {
        source
            .read(&RegisteredCurrentAuthoritySourceReadInput {
                execution_binding: binding,
                contract,
                evaluated_at: timestamp(evaluated_at),
            })
            .expect("read")
    }

    fn resolve(
        source: &RegisteredInMemoryCurrentAuthoritySource,
        binding: &RequiredContextExecutionBinding,
        contract: &RequiredContextContractBinding,
        evaluated_at: &str,
    ) -> RegisteredCurrentAuthorityResolutionOutcome {
        source
            .resolve_current_authority(&RegisteredCurrentAuthorityResolutionInput {
                execution_binding: binding,
                contract,
                evaluated_at: timestamp(evaluated_at),
                redaction: &RedactionMetadata::empty(),
            })
            .expect("resolve")
    }

    fn use_authority<F>(
        source: &RegisteredInMemoryCurrentAuthoritySource,
        binding: &RequiredContextExecutionBinding,
        contract: &RequiredContextContractBinding,
        evaluated_at: &str,
        consumer: F,
    ) -> RegisteredCurrentAuthorityUseOutcome
    where
        F: FnOnce(
            &RegisteredCurrentAuthorityUseCapability<'_>,
        ) -> RegisteredCurrentAuthorityConsumerResult,
    {
        source
            .use_current_authority(
                &RegisteredCurrentAuthorityUseInput {
                    execution_binding: binding,
                    contract,
                    evaluated_at: timestamp(evaluated_at),
                    redaction: &RedactionMetadata::empty(),
                },
                consumer,
            )
            .expect("use current authority")
    }

    fn snapshot(
        outcome: RegisteredCurrentAuthoritySourceReadOutcome,
    ) -> Result<Box<CurrentAuthoritySourceSnapshot>, &'static str> {
        match outcome {
            RegisteredCurrentAuthoritySourceReadOutcome::Snapshot(snapshot) => Ok(snapshot),
            RegisteredCurrentAuthoritySourceReadOutcome::Failure(_) => {
                Err("expected source snapshot")
            }
        }
    }

    fn failure(
        outcome: RegisteredCurrentAuthoritySourceReadOutcome,
    ) -> Result<CurrentAuthoritySourceFailure, &'static str> {
        match outcome {
            RegisteredCurrentAuthoritySourceReadOutcome::Snapshot(_) => {
                Err("expected source failure")
            }
            RegisteredCurrentAuthoritySourceReadOutcome::Failure(failure) => Ok(failure),
        }
    }

    fn assessment(
        outcome: RegisteredCurrentAuthorityResolutionOutcome,
    ) -> Result<Box<RegisteredCurrentAuthorityResolutionAssessment>, &'static str> {
        match outcome {
            RegisteredCurrentAuthorityResolutionOutcome::Assessment(assessment) => Ok(assessment),
            RegisteredCurrentAuthorityResolutionOutcome::SourceFailure(_) => {
                Err("expected resolution assessment")
            }
        }
    }

    fn resolution_failure(
        outcome: RegisteredCurrentAuthorityResolutionOutcome,
    ) -> Result<CurrentAuthoritySourceFailure, &'static str> {
        match outcome {
            RegisteredCurrentAuthorityResolutionOutcome::Assessment(_) => {
                Err("expected resolution source failure")
            }
            RegisteredCurrentAuthorityResolutionOutcome::SourceFailure(failure) => Ok(failure),
        }
    }

    #[test]
    fn core_owned_registration_returns_one_complete_exact_snapshot() {
        let (contract, binding) = fixture();
        let source = source(&contract, "2026-07-26T10:20:00Z");
        let snapshot =
            snapshot(read(&source, &binding, &contract, "2026-07-26T10:25:00Z")).expect("snapshot");

        assert_eq!(
            snapshot.completeness(),
            CurrentAuthoritySourceCompleteness::CompleteForExactQuery
        );
        assert_eq!(snapshot.freshness(), CurrentAuthoritySourceFreshness::Fresh);
        assert_eq!(snapshot.generation().expect("generation").get(), 1);
    }

    #[test]
    fn canonical_inventory_order_produces_the_same_snapshot_commitment() {
        let (contract, binding) = fixture();
        let first = source(&contract, "2026-07-26T10:20:00Z");
        let mut reversed_availability = availability(&contract);
        reversed_availability.reverse();
        let mut reversed_references = references(&contract);
        reversed_references.reverse();
        let second = RegisteredInMemoryCurrentAuthoritySource::register(
            RegisteredInMemoryCurrentAuthoritySourceInput {
                source_id: CurrentAuthoritySourceId::new("authority/local").expect("source"),
                contract_version: CurrentAuthoritySourceContractVersion::new("v1")
                    .expect("version"),
                configuration_commitment: SpecContentHash::from_text("safe configuration"),
                core_maximum_observation_age_seconds: 600,
                sensitivity: WorkReportSensitivity::Internal,
                observed_at: timestamp("2026-07-26T10:20:00Z"),
                source_valid_through: None,
                generation: Some(CurrentAuthoritySourceGeneration::new(1).expect("generation")),
                complete_grant_inventory: vec![grant(&contract)],
                complete_availability_inventory: reversed_availability,
                complete_context_reference_inventory: reversed_references,
            },
        )
        .expect("source");

        let first = snapshot(read(&first, &binding, &contract, "2026-07-26T10:25:00Z"))
            .expect("first snapshot");
        let second = snapshot(read(&second, &binding, &contract, "2026-07-26T10:25:00Z"))
            .expect("second snapshot");
        assert_eq!(first.snapshot_commitment(), second.snapshot_commitment());
    }

    #[test]
    fn missing_exact_family_record_returns_bounded_incomplete_failure() {
        let (contract, binding) = fixture();
        let mut availability = availability(&contract);
        availability.pop();
        let source = RegisteredInMemoryCurrentAuthoritySource::register(
            RegisteredInMemoryCurrentAuthoritySourceInput {
                source_id: CurrentAuthoritySourceId::new("authority/local").expect("source"),
                contract_version: CurrentAuthoritySourceContractVersion::new("v1")
                    .expect("version"),
                configuration_commitment: SpecContentHash::from_text("safe configuration"),
                core_maximum_observation_age_seconds: 600,
                sensitivity: WorkReportSensitivity::Internal,
                observed_at: timestamp("2026-07-26T10:20:00Z"),
                source_valid_through: None,
                generation: None,
                complete_grant_inventory: Vec::new(),
                complete_availability_inventory: availability,
                complete_context_reference_inventory: references(&contract),
            },
        )
        .expect("source");

        let failure =
            failure(read(&source, &binding, &contract, "2026-07-26T10:25:00Z")).expect("failure");
        assert_eq!(
            failure.kind(),
            CurrentAuthoritySourceFailureKind::Incomplete
        );
        assert_eq!(
            failure.posture(),
            CurrentAuthoritySourceFailurePosture::RetryableAfterSourceChange
        );
        assert!(!format!("{failure:?}").contains("report/current"));
    }

    #[test]
    fn stale_and_future_observations_fail_closed_without_raw_values() {
        let (contract, binding) = fixture();
        let stale = source(&contract, "2026-07-26T10:20:00Z");
        let future = source(&contract, "2026-07-26T10:40:00Z");

        let stale = failure(read(&stale, &binding, &contract, "2026-07-26T10:31:00Z"))
            .expect("stale failure");
        let future = failure(read(&future, &binding, &contract, "2026-07-26T10:30:00Z"))
            .expect("future failure");
        assert_eq!(stale.kind(), CurrentAuthoritySourceFailureKind::Stale);
        assert_eq!(
            future.kind(),
            CurrentAuthoritySourceFailureKind::FutureDated
        );
        assert!(!format!("{stale:?}{future:?}").contains("2026-07-26"));
    }

    #[test]
    fn duplicate_inventory_is_rejected_without_leaking_identifiers() {
        let (contract, _binding) = fixture();
        let duplicate = availability(&contract)[0].clone();
        let error = RegisteredInMemoryCurrentAuthoritySource::register(
            RegisteredInMemoryCurrentAuthoritySourceInput {
                source_id: CurrentAuthoritySourceId::new("authority/local").expect("source"),
                contract_version: CurrentAuthoritySourceContractVersion::new("v1")
                    .expect("version"),
                configuration_commitment: SpecContentHash::from_text("safe configuration"),
                core_maximum_observation_age_seconds: 600,
                sensitivity: WorkReportSensitivity::Internal,
                observed_at: timestamp("2026-07-26T10:20:00Z"),
                source_valid_through: None,
                generation: None,
                complete_grant_inventory: Vec::new(),
                complete_availability_inventory: vec![duplicate.clone(), duplicate],
                complete_context_reference_inventory: references(&contract),
            },
        )
        .expect_err("duplicate must fail");
        assert_eq!(
            error.code(),
            "current_authority.source.registered.inventory.availability_duplicate"
        );
        assert!(!error.to_string().contains("report/current"));
    }

    #[test]
    fn registered_source_and_same_call_resolver_produce_ready_assessment() {
        let (contract, binding) = fixture();
        let source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant_for(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract),
            references(&contract),
        );
        let assessment = assessment(resolve(
            &source,
            &binding,
            &contract,
            "2026-07-26T10:25:00Z",
        ))
        .expect("assessment");

        assert_eq!(
            assessment.posture(),
            RegisteredCurrentAuthorityResolutionPosture::Ready
        );
        assert_eq!(
            assessment.reasons(),
            [RegisteredCurrentAuthorityResolutionReason::Ready]
        );
        assert_eq!(
            assessment.consumption().posture(),
            RequiredContextConsumptionPosture::Satisfied
        );
        assert_ne!(
            assessment.source_snapshot_commitment(),
            assessment.fact_set_commitment()
        );
    }

    #[test]
    fn ready_authority_invokes_one_bounded_consumer_once() {
        let (contract, binding) = fixture();
        let source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant_for(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract),
            references(&contract),
        );
        let invocations = AtomicUsize::new(0);

        let outcome = use_authority(&source, &binding, &contract, "2026-07-26T10:25:00Z", |_| {
            invocations.fetch_add(1, Ordering::Relaxed);
            RegisteredCurrentAuthorityConsumerResult::Succeeded
        });

        assert_eq!(invocations.load(Ordering::Relaxed), 1);
        assert_eq!(
            outcome.posture(),
            RegisteredCurrentAuthorityUsePosture::ConsumerSucceeded
        );
        assert_eq!(
            outcome.reasons(),
            [RegisteredCurrentAuthorityResolutionReason::Ready]
        );
    }

    #[test]
    fn blocked_authority_never_invokes_the_consumer() {
        let (contract, binding) = fixture();
        let approval_requirements = CapabilityGrantRequirements::new(
            Vec::new(),
            vec![ApprovalReferenceId::new("approval/current").expect("approval")],
            Vec::new(),
            Vec::new(),
        )
        .expect("requirements");
        let source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    approval_requirements,
                ),
                grant_for(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract),
            references(&contract),
        );
        let invocations = AtomicUsize::new(0);

        let outcome = use_authority(&source, &binding, &contract, "2026-07-26T10:25:00Z", |_| {
            invocations.fetch_add(1, Ordering::Relaxed);
            RegisteredCurrentAuthorityConsumerResult::Succeeded
        });

        assert_eq!(invocations.load(Ordering::Relaxed), 0);
        assert_eq!(
            outcome.posture(),
            RegisteredCurrentAuthorityUsePosture::BlockedBeforeUse
        );
        assert!(outcome
            .reasons()
            .contains(&RegisteredCurrentAuthorityResolutionReason::IndependentApprovalRequired));
    }

    #[test]
    fn source_failure_never_invokes_the_consumer() {
        let (contract, binding) = fixture();
        let mut context_references = references(&contract);
        context_references.pop();
        let source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            Vec::new(),
            availability(&contract),
            context_references,
        );
        let invocations = AtomicUsize::new(0);

        let outcome = use_authority(&source, &binding, &contract, "2026-07-26T10:25:00Z", |_| {
            invocations.fetch_add(1, Ordering::Relaxed);
            RegisteredCurrentAuthorityConsumerResult::Succeeded
        });

        assert_eq!(invocations.load(Ordering::Relaxed), 0);
        assert_eq!(
            outcome.posture(),
            RegisteredCurrentAuthorityUsePosture::SourceFailure
        );
        assert_eq!(
            outcome.source_failure_kind(),
            Some(CurrentAuthoritySourceFailureKind::Incomplete)
        );
        assert_eq!(
            outcome.source_failure_posture(),
            Some(CurrentAuthoritySourceFailurePosture::RetryableAfterSourceChange)
        );
        assert!(outcome.reasons().is_empty());
    }

    #[test]
    fn consumer_failure_and_ambiguity_remain_explicit() {
        let (contract, binding) = fixture();
        let grants = vec![
            grant_for(
                &contract,
                0,
                CapabilityGrantLifecycle::Active,
                CapabilityGrantRequirements::default(),
            ),
            grant_for(
                &contract,
                1,
                CapabilityGrantLifecycle::Active,
                CapabilityGrantRequirements::default(),
            ),
        ];
        let source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            grants,
            availability(&contract),
            references(&contract),
        );

        let failed = use_authority(&source, &binding, &contract, "2026-07-26T10:25:00Z", |_| {
            RegisteredCurrentAuthorityConsumerResult::Failed
        });
        let ambiguous = use_authority(&source, &binding, &contract, "2026-07-26T10:26:00Z", |_| {
            RegisteredCurrentAuthorityConsumerResult::OutcomeAmbiguous
        });

        assert_eq!(
            failed.posture(),
            RegisteredCurrentAuthorityUsePosture::ConsumerFailed
        );
        assert_eq!(
            ambiguous.posture(),
            RegisteredCurrentAuthorityUsePosture::ConsumerOutcomeAmbiguous
        );
    }

    #[test]
    fn repeated_calls_each_resolve_and_invoke_one_fresh_consumer() {
        let (contract, binding) = fixture();
        let source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant_for(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract),
            references(&contract),
        );
        let invocations = AtomicUsize::new(0);

        for evaluated_at in ["2026-07-26T10:25:00Z", "2026-07-26T10:26:00Z"] {
            let outcome = use_authority(&source, &binding, &contract, evaluated_at, |_| {
                invocations.fetch_add(1, Ordering::Relaxed);
                RegisteredCurrentAuthorityConsumerResult::Succeeded
            });
            assert_eq!(
                outcome.posture(),
                RegisteredCurrentAuthorityUsePosture::ConsumerSucceeded
            );
        }

        assert_eq!(invocations.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn expired_grant_blocks_use_before_consumer_invocation() {
        let (contract, binding) = fixture();
        let source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for_with_expiry(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                    Some(timestamp("2026-07-26T10:24:00Z")),
                ),
                grant_for(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract),
            references(&contract),
        );
        let invocations = AtomicUsize::new(0);

        let outcome = use_authority(&source, &binding, &contract, "2026-07-26T10:25:00Z", |_| {
            invocations.fetch_add(1, Ordering::Relaxed);
            RegisteredCurrentAuthorityConsumerResult::Succeeded
        });

        assert_eq!(invocations.load(Ordering::Relaxed), 0);
        assert_eq!(
            outcome.posture(),
            RegisteredCurrentAuthorityUsePosture::BlockedBeforeUse
        );
        assert_eq!(
            outcome.reasons(),
            [RegisteredCurrentAuthorityResolutionReason::RequiredContextGap]
        );
    }

    #[test]
    fn revoked_grant_blocks_use_before_consumer_invocation() {
        let (contract, binding) = fixture();
        let source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Revoked,
                    CapabilityGrantRequirements::default(),
                ),
                grant_for(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract),
            references(&contract),
        );
        let invocations = AtomicUsize::new(0);

        let outcome = use_authority(&source, &binding, &contract, "2026-07-26T10:25:00Z", |_| {
            invocations.fetch_add(1, Ordering::Relaxed);
            RegisteredCurrentAuthorityConsumerResult::Succeeded
        });

        assert_eq!(invocations.load(Ordering::Relaxed), 0);
        assert_eq!(
            outcome.posture(),
            RegisteredCurrentAuthorityUsePosture::BlockedBeforeUse
        );
        assert_eq!(
            outcome.reasons(),
            [RegisteredCurrentAuthorityResolutionReason::RequiredContextGap]
        );
    }

    #[test]
    fn changed_contract_and_binding_cannot_reuse_prior_source_authority() {
        let (original_contract, _) = fixture();
        let (changed_contract, changed_binding) =
            fixture_with_contract_id("harness/context-changed");
        let source = source_with_inventory(
            &original_contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for(
                    &original_contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant_for(
                    &original_contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&original_contract),
            references(&original_contract),
        );
        let invocations = AtomicUsize::new(0);

        let outcome = use_authority(
            &source,
            &changed_binding,
            &changed_contract,
            "2026-07-26T10:25:00Z",
            |_| {
                invocations.fetch_add(1, Ordering::Relaxed);
                RegisteredCurrentAuthorityConsumerResult::Succeeded
            },
        );

        assert_eq!(invocations.load(Ordering::Relaxed), 0);
        assert_eq!(
            outcome.posture(),
            RegisteredCurrentAuthorityUsePosture::BlockedBeforeUse
        );
        assert_eq!(
            outcome.reasons(),
            [RegisteredCurrentAuthorityResolutionReason::RequiredContextGap]
        );
    }

    #[test]
    fn mismatched_contract_is_rejected_before_consumer_with_stable_error() {
        let (contract, binding) = fixture();
        let (changed_contract, _) = fixture_with_contract_id("harness/context-changed");
        let source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant_for(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract),
            references(&contract),
        );
        let invocations = AtomicUsize::new(0);

        let error = source
            .use_current_authority(
                &RegisteredCurrentAuthorityUseInput {
                    execution_binding: &binding,
                    contract: &changed_contract,
                    evaluated_at: timestamp("2026-07-26T10:25:00Z"),
                    redaction: &RedactionMetadata::empty(),
                },
                |_| {
                    invocations.fetch_add(1, Ordering::Relaxed);
                    RegisteredCurrentAuthorityConsumerResult::Succeeded
                },
            )
            .expect_err("mismatched contract");

        assert_eq!(invocations.load(Ordering::Relaxed), 0);
        assert_eq!(
            error.code(),
            "current_authority.source.request.contract_mismatch"
        );
        assert!(!format!("{error:?}").contains("harness/context"));
    }

    #[test]
    fn all_unresolved_prerequisites_block_use_with_fixed_reason_vector() {
        let (contract, binding) = fixture();
        let requirements = CapabilityGrantRequirements::new(
            vec![PolicyId::new("policy/current").expect("policy")],
            vec![ApprovalReferenceId::new("approval/current").expect("approval")],
            vec![EvidenceReferenceId::new("evidence/current").expect("evidence")],
            vec![LocalCheckResultId::new("check/current").expect("check")],
        )
        .expect("requirements");
        let source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for(&contract, 0, CapabilityGrantLifecycle::Active, requirements),
                grant_for(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract),
            references(&contract),
        );
        let invocations = AtomicUsize::new(0);

        let outcome = use_authority(&source, &binding, &contract, "2026-07-26T10:25:00Z", |_| {
            invocations.fetch_add(1, Ordering::Relaxed);
            RegisteredCurrentAuthorityConsumerResult::Succeeded
        });

        assert_eq!(invocations.load(Ordering::Relaxed), 0);
        assert_eq!(
            outcome.posture(),
            RegisteredCurrentAuthorityUsePosture::BlockedBeforeUse
        );
        assert_eq!(
            outcome.reasons(),
            [
                RegisteredCurrentAuthorityResolutionReason::RequiredContextGap,
                RegisteredCurrentAuthorityResolutionReason::IndependentPolicyRequired,
                RegisteredCurrentAuthorityResolutionReason::IndependentApprovalRequired,
                RegisteredCurrentAuthorityResolutionReason::IndependentEvidenceRequired,
                RegisteredCurrentAuthorityResolutionReason::IndependentCheckRequired,
            ]
        );
    }

    #[test]
    fn bounded_use_outcome_vector_is_stable() {
        let (contract, binding) = fixture();
        let ready_source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            grants_with_first_lifecycle(&contract, CapabilityGrantLifecycle::Active),
            availability(&contract),
            references(&contract),
        );
        let revoked_source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            grants_with_first_lifecycle(&contract, CapabilityGrantLifecycle::Revoked),
            availability(&contract),
            references(&contract),
        );

        let succeeded = use_authority(
            &ready_source,
            &binding,
            &contract,
            "2026-07-26T10:25:00Z",
            |_| RegisteredCurrentAuthorityConsumerResult::Succeeded,
        );
        let blocked = use_authority(
            &revoked_source,
            &binding,
            &contract,
            "2026-07-26T10:25:00Z",
            |_| RegisteredCurrentAuthorityConsumerResult::Succeeded,
        );
        let stale = use_authority(
            &ready_source,
            &binding,
            &contract,
            "2026-07-26T10:31:00Z",
            |_| RegisteredCurrentAuthorityConsumerResult::Succeeded,
        );
        let ambiguous = use_authority(
            &ready_source,
            &binding,
            &contract,
            "2026-07-26T10:26:00Z",
            |_| RegisteredCurrentAuthorityConsumerResult::OutcomeAmbiguous,
        );

        let vector = [&succeeded, &blocked, &stale, &ambiguous].map(|outcome| {
            (
                outcome.posture(),
                outcome.reasons().to_vec(),
                outcome.source_failure_kind(),
                outcome.source_failure_posture(),
            )
        });
        assert_eq!(
            vector,
            [
                (
                    RegisteredCurrentAuthorityUsePosture::ConsumerSucceeded,
                    vec![RegisteredCurrentAuthorityResolutionReason::Ready],
                    None,
                    None,
                ),
                (
                    RegisteredCurrentAuthorityUsePosture::BlockedBeforeUse,
                    vec![RegisteredCurrentAuthorityResolutionReason::RequiredContextGap],
                    None,
                    None,
                ),
                (
                    RegisteredCurrentAuthorityUsePosture::SourceFailure,
                    Vec::new(),
                    Some(CurrentAuthoritySourceFailureKind::Stale),
                    Some(CurrentAuthoritySourceFailurePosture::RetryableAfterSourceChange),
                ),
                (
                    RegisteredCurrentAuthorityUsePosture::ConsumerOutcomeAmbiguous,
                    vec![RegisteredCurrentAuthorityResolutionReason::Ready],
                    None,
                    None,
                ),
            ]
        );
    }

    #[test]
    fn use_capability_and_outcome_debug_are_redaction_safe() {
        let (contract, binding) = fixture();
        let source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant_for(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract),
            references(&contract),
        );
        let mut capability_debug = String::new();

        let outcome = use_authority(
            &source,
            &binding,
            &contract,
            "2026-07-26T10:25:00Z",
            |capability| {
                capability_debug = format!("{capability:?}");
                RegisteredCurrentAuthorityConsumerResult::Succeeded
            },
        );
        let outcome_debug = format!("{outcome:?}");

        for marker in [
            "authority/local",
            "report/current",
            "run-authority",
            "agent/consumer",
            "2026-07-26",
        ] {
            assert!(!capability_debug.contains(marker));
            assert!(!outcome_debug.contains(marker));
        }
        assert!(capability_debug.contains("[REDACTED]"));
    }

    #[test]
    fn source_failure_prevents_same_call_resolution() {
        let (contract, binding) = fixture();
        let mut context_references = references(&contract);
        context_references.pop();
        let source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            Vec::new(),
            availability(&contract),
            context_references,
        );
        let failure = resolution_failure(resolve(
            &source,
            &binding,
            &contract,
            "2026-07-26T10:25:00Z",
        ))
        .expect("source failure");

        assert_eq!(
            failure.kind(),
            CurrentAuthoritySourceFailureKind::Incomplete
        );
        assert!(!format!("{failure:?}").contains("report/metadata"));
    }

    #[test]
    fn unresolved_approval_prerequisite_blocks_required_context() {
        let (contract, binding) = fixture();
        let approval_requirements = CapabilityGrantRequirements::new(
            Vec::new(),
            vec![ApprovalReferenceId::new("approval/current").expect("approval")],
            Vec::new(),
            Vec::new(),
        )
        .expect("requirements");
        let source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    approval_requirements,
                ),
                grant_for(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract),
            references(&contract),
        );
        let assessment = assessment(resolve(
            &source,
            &binding,
            &contract,
            "2026-07-26T10:25:00Z",
        ))
        .expect("assessment");

        assert_eq!(
            assessment.posture(),
            RegisteredCurrentAuthorityResolutionPosture::Blocked
        );
        assert!(assessment
            .reasons()
            .contains(&RegisteredCurrentAuthorityResolutionReason::IndependentApprovalRequired));
        assert!(assessment
            .reasons()
            .contains(&RegisteredCurrentAuthorityResolutionReason::RequiredContextGap));
    }

    #[test]
    fn revoked_grant_cannot_produce_ready_assessment() {
        let (contract, binding) = fixture();
        let source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Revoked,
                    CapabilityGrantRequirements::default(),
                ),
                grant_for(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract),
            references(&contract),
        );
        let assessment = assessment(resolve(
            &source,
            &binding,
            &contract,
            "2026-07-26T10:25:00Z",
        ))
        .expect("assessment");

        assert_eq!(
            assessment.posture(),
            RegisteredCurrentAuthorityResolutionPosture::Blocked
        );
        assert!(assessment
            .reasons()
            .contains(&RegisteredCurrentAuthorityResolutionReason::RequiredContextGap));
    }

    #[test]
    fn canonical_inventory_order_produces_same_resolution_commitment() {
        let (contract, binding) = fixture();
        let mut grants = vec![
            grant_for(
                &contract,
                0,
                CapabilityGrantLifecycle::Active,
                CapabilityGrantRequirements::default(),
            ),
            grant_for(
                &contract,
                1,
                CapabilityGrantLifecycle::Active,
                CapabilityGrantRequirements::default(),
            ),
        ];
        let mut availability_records = availability(&contract);
        let mut context_references = references(&contract);
        let first = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            grants.clone(),
            availability_records.clone(),
            context_references.clone(),
        );
        grants.reverse();
        availability_records.reverse();
        context_references.reverse();
        let second = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            grants,
            availability_records,
            context_references,
        );

        let first = assessment(resolve(&first, &binding, &contract, "2026-07-26T10:25:00Z"))
            .expect("first");
        let second = assessment(resolve(
            &second,
            &binding,
            &contract,
            "2026-07-26T10:25:00Z",
        ))
        .expect("second");
        assert_eq!(
            first.assessment_commitment(),
            second.assessment_commitment()
        );
    }

    #[test]
    fn ready_current_authority_reads_exact_work_report_metadata_once() {
        let (contract, binding) = fixture();
        let source = ready_source(&contract);
        let store = InstrumentedWorkReportArtifactStore::with_artifact(work_report_artifact(
            "report/metadata",
            "run-authority",
            WorkReportSensitivity::Internal,
        ));
        let report_id = WorkReportId::new("report/metadata").expect("report");

        let outcome = read_work_report_metadata(
            &source,
            &binding,
            &contract,
            &report_id,
            "2026-07-26T10:25:00Z",
            &store,
        )
        .expect("metadata read");

        assert!(matches!(
            outcome,
            CurrentAuthorityWorkReportMetadataReadOutcome::Found(_)
        ));
        if let CurrentAuthorityWorkReportMetadataReadOutcome::Found(view) = outcome {
            assert_eq!(view.report_id(), &report_id);
            assert_eq!(view.run_id(), binding.run_id());
            assert_eq!(view.terminal_run_status(), WorkReportStatus::Completed);
            assert_eq!(view.sensitivity(), WorkReportSensitivity::Internal);

            let debug = format!("{view:?}");
            assert!(!debug.contains("report/metadata"));
            assert!(!debug.contains("run-authority"));
            assert!(!debug.contains("bounded metadata fixture"));
            assert!(debug.contains("[REDACTED]"));
        }
        assert_eq!(store.read_count(), 1);
        assert_eq!(store.write_count(), 0);
        assert_eq!(store.list_count(), 0);
    }

    #[test]
    fn absent_work_report_is_explicit_after_one_exact_read() {
        let (contract, binding) = fixture();
        let source = ready_source(&contract);
        let store = InstrumentedWorkReportArtifactStore::empty();
        let report_id = WorkReportId::new("report/metadata").expect("report");

        let outcome = read_work_report_metadata(
            &source,
            &binding,
            &contract,
            &report_id,
            "2026-07-26T10:25:00Z",
            &store,
        )
        .expect("metadata read");

        assert!(matches!(
            outcome,
            CurrentAuthorityWorkReportMetadataReadOutcome::NotFound
        ));
        assert_eq!(store.read_count(), 1);
        assert_eq!(store.write_count(), 0);
        assert_eq!(store.list_count(), 0);
    }

    #[test]
    fn store_failure_is_bounded_and_does_not_leak_source_error() {
        let (contract, binding) = fixture();
        let source = ready_source(&contract);
        let store = InstrumentedWorkReportArtifactStore::failing();
        let report_id = WorkReportId::new("report/metadata").expect("report");

        let outcome = read_work_report_metadata(
            &source,
            &binding,
            &contract,
            &report_id,
            "2026-07-26T10:25:00Z",
            &store,
        )
        .expect("bounded store failure");
        let debug = format!("{outcome:?}");

        assert!(matches!(
            outcome,
            CurrentAuthorityWorkReportMetadataReadOutcome::StoreFailure
        ));
        assert_eq!(store.read_count(), 1);
        assert!(!debug.contains("secret-like-value"));
        assert!(!debug.contains("test.store.secret_failure"));
        assert_eq!(debug, "StoreFailure");
    }

    #[test]
    fn reference_only_and_optional_targets_are_rejected_before_store_read() {
        let (contract, binding) = fixture();
        let source = ready_source(&contract);
        let store = InstrumentedWorkReportArtifactStore::empty();
        let reference_only = WorkReportId::new("report/current").expect("report");

        let reference_error = read_work_report_metadata(
            &source,
            &binding,
            &contract,
            &reference_only,
            "2026-07-26T10:25:00Z",
            &store,
        )
        .expect_err("reference-only target");
        assert_eq!(
            reference_error.code(),
            "current_authority.source.registered.work_report_metadata.access_insufficient"
        );

        let (optional_contract, optional_binding) = fixture_with_shape(
            "harness/context",
            GovernedContextAccessLevel::BoundedMetadata,
            RequiredContextObligation::Optional,
            "run-authority",
        );
        let optional_source = ready_source(&optional_contract);
        let metadata = WorkReportId::new("report/metadata").expect("report");
        let optional_error = read_work_report_metadata(
            &optional_source,
            &optional_binding,
            &optional_contract,
            &metadata,
            "2026-07-26T10:25:00Z",
            &store,
        )
        .expect_err("optional target");
        assert_eq!(
            optional_error.code(),
            "current_authority.source.registered.work_report_metadata.obligation_insufficient"
        );
        assert_eq!(store.read_count(), 0);
    }

    #[test]
    fn undeclared_target_is_rejected_without_leaking_id_or_reading_store() {
        let (contract, binding) = fixture();
        let source = ready_source(&contract);
        let store = InstrumentedWorkReportArtifactStore::empty();
        let report_id = WorkReportId::new("report/unlisted-target").expect("report target");

        let error = read_work_report_metadata(
            &source,
            &binding,
            &contract,
            &report_id,
            "2026-07-26T10:25:00Z",
            &store,
        )
        .expect_err("undeclared target");
        let debug = format!("{error:?}");

        assert_eq!(
            error.code(),
            "current_authority.source.registered.work_report_metadata.target_missing"
        );
        assert!(!debug.contains("unlisted-target"));
        assert_eq!(store.read_count(), 0);
    }

    #[test]
    fn revoked_target_grant_and_missing_prerequisite_block_before_store_read() {
        let (contract, binding) = fixture();
        let store = InstrumentedWorkReportArtifactStore::empty();
        let report_id = WorkReportId::new("report/metadata").expect("report");
        let revoked_source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant_for(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Revoked,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract),
            references(&contract),
        );

        let revoked = read_work_report_metadata(
            &revoked_source,
            &binding,
            &contract,
            &report_id,
            "2026-07-26T10:25:00Z",
            &store,
        )
        .expect("blocked");
        assert!(matches!(
            revoked,
            CurrentAuthorityWorkReportMetadataReadOutcome::Blocked(_)
        ));

        let approval_requirements = CapabilityGrantRequirements::new(
            Vec::new(),
            vec![ApprovalReferenceId::new("approval/current").expect("approval")],
            Vec::new(),
            Vec::new(),
        )
        .expect("requirements");
        let prerequisite_source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant_for(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    approval_requirements,
                ),
            ],
            availability(&contract),
            references(&contract),
        );
        let prerequisite = read_work_report_metadata(
            &prerequisite_source,
            &binding,
            &contract,
            &report_id,
            "2026-07-26T10:25:00Z",
            &store,
        )
        .expect("blocked");
        assert!(matches!(
            prerequisite,
            CurrentAuthorityWorkReportMetadataReadOutcome::Blocked(_)
        ));
        if let CurrentAuthorityWorkReportMetadataReadOutcome::Blocked(reasons) = prerequisite {
            assert!(reasons.contains(
                &RegisteredCurrentAuthorityResolutionReason::IndependentApprovalRequired
            ));
        }
        assert_eq!(store.read_count(), 0);
    }

    #[test]
    fn stale_source_and_changed_run_binding_block_before_store_read() {
        let (contract, binding) = fixture();
        let source = ready_source(&contract);
        let store = InstrumentedWorkReportArtifactStore::empty();
        let report_id = WorkReportId::new("report/metadata").expect("report");

        let stale = read_work_report_metadata(
            &source,
            &binding,
            &contract,
            &report_id,
            "2026-07-26T10:31:00Z",
            &store,
        )
        .expect("source failure");
        assert!(matches!(
            stale,
            CurrentAuthorityWorkReportMetadataReadOutcome::SourceFailure {
                kind: CurrentAuthoritySourceFailureKind::Stale,
                posture: CurrentAuthoritySourceFailurePosture::RetryableAfterSourceChange,
            }
        ));

        let (same_contract, changed_binding) = fixture_with_shape(
            "harness/context",
            GovernedContextAccessLevel::BoundedMetadata,
            RequiredContextObligation::Required,
            "run-changed",
        );
        assert_eq!(same_contract.content_hash(), contract.content_hash());
        let changed = read_work_report_metadata(
            &source,
            &changed_binding,
            &contract,
            &report_id,
            "2026-07-26T10:25:00Z",
            &store,
        )
        .expect("blocked");
        assert!(matches!(
            changed,
            CurrentAuthorityWorkReportMetadataReadOutcome::Blocked(_)
        ));
        assert_eq!(store.read_count(), 0);
    }

    #[test]
    fn declared_reference_sensitivity_mismatch_blocks_before_store_read() {
        let (contract, binding) = fixture();
        let mut context_references = references(&contract);
        context_references[1] = GovernedContextReference::new(
            contract.requirements()[1].target().clone(),
            WorkReportSensitivity::Confidential,
            GovernedContextAvailability::Available,
            RedactionMetadata::empty(),
        )
        .expect("reference");
        let source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant_for(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract),
            context_references,
        );
        let store = InstrumentedWorkReportArtifactStore::empty();
        let report_id = WorkReportId::new("report/metadata").expect("report");

        let outcome = read_work_report_metadata(
            &source,
            &binding,
            &contract,
            &report_id,
            "2026-07-26T10:25:00Z",
            &store,
        )
        .expect("blocked");

        assert!(matches!(
            outcome,
            CurrentAuthorityWorkReportMetadataReadOutcome::Blocked(_)
        ));
        assert_eq!(store.read_count(), 0);
    }

    #[test]
    fn artifact_identity_or_sensitivity_mismatch_fails_after_one_bounded_read() {
        let (contract, binding) = fixture();
        let source = ready_source(&contract);
        let report_id = WorkReportId::new("report/metadata").expect("report");
        let mismatched_store =
            InstrumentedWorkReportArtifactStore::with_mismatched_artifact(work_report_artifact(
                "report/different",
                "run-authority",
                WorkReportSensitivity::Internal,
            ));

        let mismatched = read_work_report_metadata(
            &source,
            &binding,
            &contract,
            &report_id,
            "2026-07-26T10:25:00Z",
            &mismatched_store,
        )
        .expect("bounded failure");
        assert!(matches!(
            mismatched,
            CurrentAuthorityWorkReportMetadataReadOutcome::StoreFailure
        ));
        assert_eq!(mismatched_store.read_count(), 1);

        let sensitive_store =
            InstrumentedWorkReportArtifactStore::with_artifact(work_report_artifact(
                "report/metadata",
                "run-authority",
                WorkReportSensitivity::Confidential,
            ));
        let sensitive = read_work_report_metadata(
            &source,
            &binding,
            &contract,
            &report_id,
            "2026-07-26T10:25:00Z",
            &sensitive_store,
        )
        .expect("bounded failure");
        assert!(matches!(
            sensitive,
            CurrentAuthorityWorkReportMetadataReadOutcome::StoreFailure
        ));
        assert_eq!(sensitive_store.read_count(), 1);
    }

    #[test]
    fn repeated_metadata_reads_each_reresolve_and_read_once() {
        let (contract, binding) = fixture();
        let source = ready_source(&contract);
        let store = InstrumentedWorkReportArtifactStore::with_artifact(work_report_artifact(
            "report/metadata",
            "run-authority",
            WorkReportSensitivity::Internal,
        ));
        let report_id = WorkReportId::new("report/metadata").expect("report");

        for evaluated_at in ["2026-07-26T10:25:00Z", "2026-07-26T10:26:00Z"] {
            let outcome = read_work_report_metadata(
                &source,
                &binding,
                &contract,
                &report_id,
                evaluated_at,
                &store,
            )
            .expect("metadata read");
            assert!(matches!(
                outcome,
                CurrentAuthorityWorkReportMetadataReadOutcome::Found(_)
            ));
        }

        assert_eq!(store.read_count(), 2);
        assert_eq!(store.write_count(), 0);
        assert_eq!(store.list_count(), 0);
    }

    #[test]
    fn resolution_debug_redacts_commitments_and_source_values() {
        let (contract, binding) = fixture();
        let source = source_with_inventory(
            &contract,
            "2026-07-26T10:20:00Z",
            vec![
                grant_for(
                    &contract,
                    0,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
                grant_for(
                    &contract,
                    1,
                    CapabilityGrantLifecycle::Active,
                    CapabilityGrantRequirements::default(),
                ),
            ],
            availability(&contract),
            references(&contract),
        );
        let assessment = assessment(resolve(
            &source,
            &binding,
            &contract,
            "2026-07-26T10:25:00Z",
        ))
        .expect("assessment");
        let debug = format!("{assessment:?}");

        assert!(!debug.contains("authority/local"));
        assert!(!debug.contains("report/current"));
        assert!(!debug.contains(assessment.assessment_commitment().as_str()));
        assert!(debug.contains("[REDACTED]"));
    }

    #[test]
    fn private_source_debug_is_redaction_safe() {
        let (contract, _binding) = fixture();
        let source = source(&contract, "2026-07-26T10:20:00Z");
        let debug = format!("{source:?}");

        assert!(!debug.contains("authority/local"));
        assert!(!debug.contains("report/current"));
        assert!(!debug.contains("safe configuration"));
        assert!(debug.contains("[REDACTED]"));
    }
}
