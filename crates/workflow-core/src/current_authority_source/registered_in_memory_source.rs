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
    WorkReportSensitivity, WorkflowOsError,
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

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;
    use crate::{
        build_immutable_run_bundle, load_project, ActorId, ApprovalReferenceId,
        CapabilityAvailability, CapabilityDelegationPosture, CapabilityGrantDefinition,
        CapabilityGrantId, CapabilityGrantLifecycle, CapabilityGrantRequirements,
        CapabilityGrantScope, GovernedContextAccessLevel, GovernedContextAvailability,
        GovernedContextReferenceTarget, HarnessContractId, HarnessContractVersion,
        ImmutableRunBundleBuildRequest, ImmutableRunBundleExecutionPosture,
        ImmutableRunBundleHandlerPosture, ImmutableRunBundleHandlerReference, ImmutableRunBundleId,
        ImmutableRunBundleReferencePosture, ImmutableRunBundleSensitivity,
        ImmutableRunBundleVersion, LocalImmutableRunBundleStore, RedactionMetadata,
        RequiredContextExecutionBindingInput, RequiredContextObligation,
        RequiredContextRequirement, RequiredContextRequirementId, SkillId, SkillVersion, StepId,
        WorkReportId, WorkflowId, WorkflowRunId, SUPPORTED_SCHEMA_VERSION,
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
            run_id: WorkflowRunId::new("run-authority").expect("run"),
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
            HarnessContractId::new("harness/context").expect("contract"),
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
                    GovernedContextAccessLevel::BoundedMetadata,
                    RequiredContextObligation::Required,
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
            expires_at: None,
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
