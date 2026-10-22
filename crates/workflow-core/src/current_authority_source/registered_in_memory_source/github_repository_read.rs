use std::collections::BTreeMap;
use std::fmt;

use serde::Serialize;

use super::RegisteredInMemoryCurrentAuthoritySource;
use crate::{
    github_actions, github_read_request, project_step_scoped_capabilities,
    resolve_capability_authority, ActorId, AdapterError, AdapterOperationMode,
    AdapterPolicyPrecheck, AdapterPolicyPrecheckProvenance, AdapterRunScope,
    CapabilityAvailabilityRecord, CapabilityGrant, CapabilityGrantId, CapabilityReference,
    CapabilityResolutionInput, CapabilityResolutionPosture, CapabilityResourceKind,
    CapabilityResourceScope, CapabilityUseAuthorityReceipt, CorrelationId,
    CurrentAuthorityFactFamily, CurrentAuthoritySourceCompleteness,
    CurrentAuthoritySourceConsistency, CurrentAuthoritySourceFactCount,
    CurrentAuthoritySourceFreshness, CurrentAuthoritySourceGeneration,
    CurrentAuthoritySourceReadWindow, CurrentAuthoritySourceSnapshot,
    CurrentAuthoritySourceSnapshotId, CurrentAuthoritySourceWatermark, GitHubReadOnlyAdapter,
    GitHubReadOnlyClient, GitHubReadOutcome, HarnessContractId, HarnessContractVersion,
    RedactionMetadata, SchemaVersion, SpecContentHash, StepId, StepScopedCapabilityProjection,
    StepScopedCapabilityProjectionInput, Timestamp, WorkReportSensitivity, WorkflowId,
    WorkflowOsError, WorkflowRunId, WorkflowVersion,
};

const GITHUB_READ_CAPABILITY: &str = "github.read";

#[derive(Clone, Eq, PartialEq, Serialize)]
struct CapabilityUseCurrentAuthorityQuery {
    capability: CapabilityReference,
    resource: CapabilityResourceScope,
}

#[derive(Clone, Eq, PartialEq, Serialize)]
struct CapabilityUseCurrentAuthorityQuerySet {
    query: CapabilityUseCurrentAuthorityQuery,
}

#[derive(Clone, Eq, PartialEq, Serialize)]
struct CapabilityUseCurrentAuthorityFactSet {
    query_set: CapabilityUseCurrentAuthorityQuerySet,
    source_snapshot_commitment: SpecContentHash,
    source_observed_at: Timestamp,
    evaluated_at: Timestamp,
    grants: Vec<CapabilityGrant>,
    availability_records: Vec<CapabilityAvailabilityRecord>,
    fact_set_commitment: SpecContentHash,
}

impl CapabilityUseCurrentAuthorityFactSet {
    fn new(
        query_set: CapabilityUseCurrentAuthorityQuerySet,
        source_snapshot_commitment: SpecContentHash,
        source_observed_at: Timestamp,
        evaluated_at: Timestamp,
        grants: Vec<CapabilityGrant>,
        availability_records: Vec<CapabilityAvailabilityRecord>,
    ) -> Result<Self, WorkflowOsError> {
        if source_observed_at > evaluated_at
            || grants.iter().any(|grant| {
                grant.capability() != &query_set.query.capability
                    || grant.resource() != &query_set.query.resource
            })
            || availability_records.iter().any(|record| {
                record.capability() != &query_set.query.capability
                    || record.resource() != &query_set.query.resource
                    || record.observed_at() > evaluated_at
            })
        {
            return Err(integration_error(
                "fact_set.invalid",
                "capability-use current authority facts are invalid",
            ));
        }
        if grants
            .windows(2)
            .any(|pair| pair[0].grant_id() >= pair[1].grant_id())
            || availability_records.len() > 1
        {
            return Err(integration_error(
                "fact_set.ambiguous",
                "capability-use current authority facts are ambiguous",
            ));
        }
        let fact_set_commitment = super::super::hash_serializable(
            "capability-use-current-authority-fact-set-v1",
            &(
                &query_set,
                &source_snapshot_commitment,
                source_observed_at,
                evaluated_at,
                &grants,
                &availability_records,
            ),
        )?;
        Ok(Self {
            query_set,
            source_snapshot_commitment,
            source_observed_at,
            evaluated_at,
            grants,
            availability_records,
            fact_set_commitment,
        })
    }
}

pub(crate) struct CurrentAuthorityGitHubRepositoryReadInput<'a, C> {
    pub(crate) actor: &'a ActorId,
    pub(crate) workflow_id: &'a WorkflowId,
    pub(crate) workflow_version: &'a WorkflowVersion,
    pub(crate) schema_version: &'a SchemaVersion,
    pub(crate) spec_hash: &'a SpecContentHash,
    pub(crate) run_id: &'a WorkflowRunId,
    pub(crate) step_id: &'a StepId,
    pub(crate) harness_contract_id: &'a HarnessContractId,
    pub(crate) harness_contract_version: &'a HarnessContractVersion,
    pub(crate) owner: &'a str,
    pub(crate) repository: &'a str,
    pub(crate) requested_sensitivity: WorkReportSensitivity,
    pub(crate) evaluated_at: Timestamp,
    pub(crate) source_continuity: CapabilityUseSourceContinuity,
    pub(crate) correlation_id: &'a CorrelationId,
    pub(crate) redaction: &'a RedactionMetadata,
    pub(crate) policy_precheck: &'a AdapterPolicyPrecheck,
    pub(crate) adapter: &'a GitHubReadOnlyAdapter<C>,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) enum CapabilityUseSourceContinuity {
    FirstObservation,
    PriorObservation {
        generation: CurrentAuthoritySourceGeneration,
        watermark: CurrentAuthoritySourceWatermark,
    },
}

impl fmt::Debug for CapabilityUseSourceContinuity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FirstObservation => formatter.write_str("FirstObservation"),
            Self::PriorObservation { generation, .. } => formatter
                .debug_struct("PriorObservation")
                .field("generation", generation)
                .field("watermark", &"[REDACTED]")
                .finish(),
        }
    }
}

pub(crate) enum CurrentAuthorityGitHubRepositoryReadOutcome {
    Succeeded {
        outcome: GitHubReadOutcome,
        projection: Box<StepScopedCapabilityProjection>,
        receipt: Box<CapabilityUseAuthorityReceipt>,
    },
    ReadSucceededReceiptFailed {
        outcome: GitHubReadOutcome,
        projection: Box<StepScopedCapabilityProjection>,
        error: WorkflowOsError,
    },
    BlockedBeforeRead {
        error: WorkflowOsError,
    },
    ReadFailed {
        error: AdapterError,
    },
}

impl fmt::Debug for CurrentAuthorityGitHubRepositoryReadOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Succeeded { .. } => formatter.write_str("Succeeded([REDACTED])"),
            Self::ReadSucceededReceiptFailed { error, .. } => formatter
                .debug_struct("ReadSucceededReceiptFailed")
                .field("read_outcome", &"[REDACTED]")
                .field("projection", &"[REDACTED]")
                .field("error_code", &error.code())
                .finish(),
            Self::BlockedBeforeRead { error } => formatter
                .debug_struct("BlockedBeforeRead")
                .field("error_code", &error.code())
                .finish(),
            Self::ReadFailed { error } => formatter
                .debug_struct("ReadFailed")
                .field("error_kind", &error.kind)
                .field("error_code", &error.code)
                .finish(),
        }
    }
}

pub(crate) struct SuccessfulCapabilityUseProof {
    execution_binding_hash: SpecContentHash,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    actor: ActorId,
    harness_contract_id: HarnessContractId,
    harness_contract_version: HarnessContractVersion,
    requested_sensitivity: WorkReportSensitivity,
    capability: CapabilityReference,
    resource_kind: CapabilityResourceKind,
    resource_scope_commitment: SpecContentHash,
    grant_id: CapabilityGrantId,
    source_snapshot_commitment: SpecContentHash,
    fact_set_commitment: SpecContentHash,
    assessment_commitment: SpecContentHash,
    issued_at: Timestamp,
    operation_outcome_commitment: SpecContentHash,
}

impl SuccessfulCapabilityUseProof {
    pub(crate) const fn execution_binding_hash(&self) -> &SpecContentHash {
        &self.execution_binding_hash
    }
    pub(crate) const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }
    pub(crate) const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }
    pub(crate) const fn step_id(&self) -> &StepId {
        &self.step_id
    }
    pub(crate) const fn actor(&self) -> &ActorId {
        &self.actor
    }
    pub(crate) const fn harness_contract_id(&self) -> &HarnessContractId {
        &self.harness_contract_id
    }
    pub(crate) const fn harness_contract_version(&self) -> &HarnessContractVersion {
        &self.harness_contract_version
    }
    pub(crate) const fn requested_sensitivity(&self) -> WorkReportSensitivity {
        self.requested_sensitivity
    }
    pub(crate) const fn capability(&self) -> &CapabilityReference {
        &self.capability
    }
    pub(crate) const fn resource_kind(&self) -> CapabilityResourceKind {
        self.resource_kind
    }
    pub(crate) const fn resource_scope_commitment(&self) -> &SpecContentHash {
        &self.resource_scope_commitment
    }
    pub(crate) const fn grant_id(&self) -> &CapabilityGrantId {
        &self.grant_id
    }
    pub(crate) const fn source_snapshot_commitment(&self) -> &SpecContentHash {
        &self.source_snapshot_commitment
    }
    pub(crate) const fn fact_set_commitment(&self) -> &SpecContentHash {
        &self.fact_set_commitment
    }
    pub(crate) const fn assessment_commitment(&self) -> &SpecContentHash {
        &self.assessment_commitment
    }
    pub(crate) const fn issued_at(&self) -> Timestamp {
        self.issued_at
    }
    pub(crate) const fn operation_outcome_commitment(&self) -> &SpecContentHash {
        &self.operation_outcome_commitment
    }
}

impl RegisteredInMemoryCurrentAuthoritySource {
    pub(crate) fn read_github_repository_metadata<C: GitHubReadOnlyClient>(
        &self,
        input: &CurrentAuthorityGitHubRepositoryReadInput<'_, C>,
    ) -> CurrentAuthorityGitHubRepositoryReadOutcome {
        self.read_github_repository_metadata_with_receipt(input, |proof| {
            CapabilityUseAuthorityReceipt::from_successful_use(&proof)
        })
    }

    fn read_github_repository_metadata_with_receipt<C, F>(
        &self,
        input: &CurrentAuthorityGitHubRepositoryReadInput<'_, C>,
        receipt_producer: F,
    ) -> CurrentAuthorityGitHubRepositoryReadOutcome
    where
        C: GitHubReadOnlyClient,
        F: FnOnce(
            SuccessfulCapabilityUseProof,
        ) -> Result<CapabilityUseAuthorityReceipt, WorkflowOsError>,
    {
        let prepared = match self.prepare_github_repository_read(input) {
            Ok(prepared) => prepared,
            Err(error) => {
                return CurrentAuthorityGitHubRepositoryReadOutcome::BlockedBeforeRead { error };
            }
        };

        let outcome = match input.adapter.read_repository_metadata(
            &prepared.request,
            &prepared.owner,
            &prepared.repository,
        ) {
            Ok(outcome) => outcome,
            Err(error) => return CurrentAuthorityGitHubRepositoryReadOutcome::ReadFailed { error },
        };
        let operation_outcome_commitment = match super::super::hash_serializable(
            "github-repository-metadata-read-outcome-v1",
            &outcome,
        ) {
            Ok(commitment) => commitment,
            Err(error) => {
                return CurrentAuthorityGitHubRepositoryReadOutcome::ReadSucceededReceiptFailed {
                    outcome,
                    projection: Box::new(prepared.projection),
                    error,
                };
            }
        };
        let proof = SuccessfulCapabilityUseProof {
            execution_binding_hash: prepared.execution_binding_hash,
            workflow_id: input.workflow_id.clone(),
            run_id: input.run_id.clone(),
            step_id: input.step_id.clone(),
            actor: input.actor.clone(),
            harness_contract_id: input.harness_contract_id.clone(),
            harness_contract_version: input.harness_contract_version.clone(),
            requested_sensitivity: input.requested_sensitivity,
            capability: prepared.capability,
            resource_kind: CapabilityResourceKind::AdapterResource,
            resource_scope_commitment: prepared.resource_scope_commitment,
            grant_id: prepared.grant_id,
            source_snapshot_commitment: prepared.source_snapshot_commitment,
            fact_set_commitment: prepared.fact_set_commitment,
            assessment_commitment: prepared.assessment_commitment,
            issued_at: input.evaluated_at,
            operation_outcome_commitment,
        };
        match receipt_producer(proof) {
            Ok(receipt) => CurrentAuthorityGitHubRepositoryReadOutcome::Succeeded {
                outcome,
                projection: Box::new(prepared.projection),
                receipt: Box::new(receipt),
            },
            Err(error) => CurrentAuthorityGitHubRepositoryReadOutcome::ReadSucceededReceiptFailed {
                outcome,
                projection: Box::new(prepared.projection),
                error,
            },
        }
    }

    // This is intentionally one linear fail-closed preparation pipeline. Keeping
    // every gate before the adapter call makes the no-call-on-denial invariant auditable.
    #[allow(clippy::too_many_lines)]
    fn prepare_github_repository_read<C: GitHubReadOnlyClient>(
        &self,
        input: &CurrentAuthorityGitHubRepositoryReadInput<'_, C>,
    ) -> Result<PreparedGitHubRepositoryRead, WorkflowOsError> {
        validate_policy_precheck(input.adapter.operation_mode(), input.policy_precheck)?;
        let owner = canonical_owner(input.owner)?;
        let repository = canonical_repository(input.repository)?;
        let capability = CapabilityReference::new(GITHUB_READ_CAPABILITY)?;
        let resource = CapabilityResourceScope::new(
            CapabilityResourceKind::AdapterResource,
            format!("github/{owner}/{repository}"),
        )?;
        if input.requested_sensitivity == WorkReportSensitivity::Unknown
            || input.requested_sensitivity > self.registration.sensitivity
            || !self
                .registration
                .supported_fact_families
                .contains(&super::super::CurrentAuthorityFactFamily::CapabilityGrants)
            || !self
                .registration
                .supported_fact_families
                .contains(&super::super::CurrentAuthorityFactFamily::CapabilityAvailability)
        {
            return Err(integration_error(
                "source.registration_invalid",
                "registered capability-use authority source is insufficient",
            ));
        }
        let query_set = CapabilityUseCurrentAuthorityQuerySet {
            query: CapabilityUseCurrentAuthorityQuery {
                capability: capability.clone(),
                resource: resource.clone(),
            },
        };
        let grants = self
            .grants
            .iter()
            .filter(|grant| {
                grant.capability() == &capability
                    && grant.resource() == &resource
                    && crate::capability_authority::grant_matches_execution_scope(
                        grant,
                        input.actor,
                        input.workflow_id,
                        input.run_id,
                        input.step_id,
                        Some(input.harness_contract_id),
                    )
            })
            .cloned()
            .collect::<Vec<_>>();
        let availability_records = self
            .availability_records
            .iter()
            .filter(|record| record.capability() == &capability && record.resource() == &resource)
            .cloned()
            .collect::<Vec<_>>();
        let snapshot = self.capability_use_snapshot(
            &query_set,
            &grants,
            &availability_records,
            input.evaluated_at,
            &input.source_continuity,
        )?;
        let fact_set = CapabilityUseCurrentAuthorityFactSet::new(
            query_set,
            snapshot.snapshot_commitment().clone(),
            self.observed_at,
            input.evaluated_at,
            grants,
            availability_records,
        )?;
        if fact_set.grants.len() > 1 {
            return Err(integration_error(
                "authority.ambiguous",
                "capability-use authority resolution is ambiguous",
            ));
        }
        let resolution = resolve_capability_authority(&CapabilityResolutionInput {
            capability: &capability,
            resource: &resource,
            actor: input.actor,
            workflow_id: input.workflow_id,
            run_id: input.run_id,
            step_id: input.step_id,
            harness_contract_id: Some(input.harness_contract_id),
            requested_sensitivity: input.requested_sensitivity,
            evaluated_at: input.evaluated_at,
            availability_records: &fact_set.availability_records,
            grants: &fact_set.grants,
        })?;
        if resolution.posture() != CapabilityResolutionPosture::Authorized {
            return Err(integration_error(
                "authority.not_authorized",
                "capability-use authority did not authorize the operation",
            ));
        }
        let grant_id = resolution.selected_grant_id().cloned().ok_or_else(|| {
            integration_error(
                "authority.grant_missing",
                "authorized capability-use resolution is missing its grant",
            )
        })?;
        let selected = fact_set
            .grants
            .iter()
            .find(|grant| grant.grant_id() == &grant_id)
            .ok_or_else(|| {
                integration_error(
                    "authority.grant_missing",
                    "authorized capability-use resolution is missing its grant",
                )
            })?;
        if selected.scope().run_id() != Some(input.run_id)
            || selected.scope().step_id() != Some(input.step_id)
            || selected.scope().harness_contract_id() != Some(input.harness_contract_id)
        {
            return Err(integration_error(
                "authority.scope_insufficient",
                "capability-use grant scope is insufficient for this operation",
            ));
        }
        let projection = project_step_scoped_capabilities(&StepScopedCapabilityProjectionInput {
            actor: input.actor,
            workflow_id: input.workflow_id,
            run_id: input.run_id,
            step_id: input.step_id,
            harness_contract_id: Some(input.harness_contract_id),
            projected_at: input.evaluated_at,
            resolutions: std::slice::from_ref(&resolution),
            redaction: input.redaction,
        })?;
        let assessment_commitment = super::super::hash_serializable(
            "github-repository-metadata-authority-assessment-v1",
            &(
                &fact_set.fact_set_commitment,
                &resolution,
                &projection,
                &grant_id,
            ),
        )?;
        let execution_binding_hash = super::super::hash_serializable(
            "github-repository-metadata-execution-binding-v1",
            &(
                input.actor,
                input.workflow_id,
                input.workflow_version,
                input.schema_version,
                input.spec_hash,
                input.run_id,
                input.step_id,
                input.harness_contract_id,
                input.harness_contract_version,
            ),
        )?;
        let resource_scope_commitment =
            super::super::hash_serializable("github-repository-resource-v1", &resource)?;
        let mut metadata = BTreeMap::new();
        metadata.insert("owner".to_owned(), owner.clone());
        metadata.insert("repo".to_owned(), repository.clone());
        let mut request = github_read_request(
            github_actions::REPOSITORY_METADATA,
            input.actor.clone(),
            input.correlation_id.clone(),
            metadata,
            input.adapter.operation_mode(),
            input.policy_precheck.clone(),
        )
        .map_err(WorkflowOsError::from)?;
        request.run_scope = Some(AdapterRunScope {
            workflow_run_id: input.run_id.clone(),
            workflow_id: input.workflow_id.clone(),
            workflow_version: input.workflow_version.clone(),
            schema_version: input.schema_version.clone(),
            spec_hash: input.spec_hash.clone(),
        });
        request
            .validate_preconditions()
            .map_err(WorkflowOsError::from)?;
        Ok(PreparedGitHubRepositoryRead {
            owner,
            repository,
            request,
            projection,
            execution_binding_hash,
            capability,
            resource_scope_commitment,
            grant_id,
            source_snapshot_commitment: fact_set.source_snapshot_commitment,
            fact_set_commitment: fact_set.fact_set_commitment,
            assessment_commitment,
        })
    }

    fn capability_use_snapshot(
        &self,
        query_set: &CapabilityUseCurrentAuthorityQuerySet,
        grants: &[CapabilityGrant],
        availability_records: &[CapabilityAvailabilityRecord],
        evaluated_at: Timestamp,
        continuity: &CapabilityUseSourceContinuity,
    ) -> Result<CurrentAuthoritySourceSnapshot, WorkflowOsError> {
        let requested_fact_families = [
            CurrentAuthorityFactFamily::CapabilityGrants,
            CurrentAuthorityFactFamily::CapabilityAvailability,
        ];
        let request_commitment = super::super::hash_serializable(
            "capability-use-current-authority-request-v1",
            &(
                self.registration.registration_commitment(),
                query_set,
                requested_fact_families,
                evaluated_at,
            ),
        )?;
        let records_commitment = super::super::hash_serializable(
            "capability-use-current-authority-records-v1",
            &(request_commitment.clone(), grants, availability_records),
        )?;
        let watermark = CurrentAuthoritySourceWatermark::new(format!(
            "watermark/{}",
            self.inventory_commitment.as_str()
        ))?;
        validate_source_continuity(continuity, self.generation, &watermark)?;

        let snapshot = CurrentAuthoritySourceSnapshot::new_from_committed_request(
            super::super::CurrentAuthoritySourceCommittedSnapshotInput {
                request: super::super::CurrentAuthoritySourceSnapshotRequest {
                    registration_commitment: self.registration.registration_commitment(),
                    request_commitment: &request_commitment,
                    requested_fact_families: &requested_fact_families,
                    query_count: 1,
                    evaluated_at,
                },
                registration: &self.registration,
                snapshot_id: CurrentAuthoritySourceSnapshotId::new(format!(
                    "snapshot/{}",
                    records_commitment.as_str()
                ))?,
                watermark,
                generation: self.generation,
                read_window: CurrentAuthoritySourceReadWindow::new(
                    self.observed_at,
                    self.observed_at,
                    self.observed_at,
                )?,
                completeness: CurrentAuthoritySourceCompleteness::CompleteForExactQuery,
                consistency: CurrentAuthoritySourceConsistency::AtomicSnapshot,
                source_valid_through: self.source_valid_through,
                returned_fact_families: requested_fact_families.to_vec(),
                fact_counts: vec![
                    CurrentAuthoritySourceFactCount::new(
                        CurrentAuthorityFactFamily::CapabilityGrants,
                        u64::try_from(grants.len()).unwrap_or(u64::MAX),
                    ),
                    CurrentAuthoritySourceFactCount::new(
                        CurrentAuthorityFactFamily::CapabilityAvailability,
                        u64::try_from(availability_records.len()).unwrap_or(u64::MAX),
                    ),
                ],
                records_commitment,
            },
        )?;
        if snapshot.freshness() != CurrentAuthoritySourceFreshness::Fresh {
            return Err(integration_error(
                "source.not_fresh",
                "registered capability-use authority source is not fresh",
            ));
        }
        Ok(snapshot)
    }
}

struct PreparedGitHubRepositoryRead {
    owner: String,
    repository: String,
    request: crate::AdapterRequest,
    projection: StepScopedCapabilityProjection,
    execution_binding_hash: SpecContentHash,
    capability: CapabilityReference,
    resource_scope_commitment: SpecContentHash,
    grant_id: CapabilityGrantId,
    source_snapshot_commitment: SpecContentHash,
    fact_set_commitment: SpecContentHash,
    assessment_commitment: SpecContentHash,
}

fn validate_policy_precheck(
    mode: AdapterOperationMode,
    precheck: &AdapterPolicyPrecheck,
) -> Result<(), WorkflowOsError> {
    let valid = matches!(
        (mode, precheck),
        (
            AdapterOperationMode::Fixture,
            AdapterPolicyPrecheck::Allowed {
                provenance: AdapterPolicyPrecheckProvenance::FixtureTest,
                ..
            }
        ) | (
            AdapterOperationMode::LiveReadOnly,
            AdapterPolicyPrecheck::Allowed {
                provenance: AdapterPolicyPrecheckProvenance::RuntimePolicy,
                ..
            }
        )
    );
    if !valid {
        return Err(integration_error(
            "policy_precheck.invalid",
            "GitHub capability use requires an allowed policy precheck with matching provenance",
        ));
    }
    Ok(())
}

fn validate_source_continuity(
    expected: &CapabilityUseSourceContinuity,
    current_generation: Option<CurrentAuthoritySourceGeneration>,
    current_watermark: &CurrentAuthoritySourceWatermark,
) -> Result<(), WorkflowOsError> {
    let CapabilityUseSourceContinuity::PriorObservation {
        generation: prior_generation,
        watermark: prior_watermark,
    } = expected
    else {
        return Ok(());
    };
    let current_generation = current_generation.ok_or_else(|| {
        integration_error(
            "source.continuity_missing",
            "registered capability-use authority source continuity is unavailable",
        )
    })?;
    if current_generation < *prior_generation {
        return Err(integration_error(
            "source.generation_rollback",
            "registered capability-use authority source generation moved backward",
        ));
    }
    if current_generation == *prior_generation && current_watermark != prior_watermark {
        return Err(integration_error(
            "source.watermark_changed_without_generation",
            "registered capability-use authority source changed without a new generation",
        ));
    }
    Ok(())
}

fn canonical_owner(value: &str) -> Result<String, WorkflowOsError> {
    if value.is_empty()
        || value.len() > 39
        || value.starts_with('-')
        || value.ends_with('-')
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        || secret_like(value)
    {
        return Err(integration_error(
            "repository.owner_invalid",
            "GitHub repository owner is invalid",
        ));
    }
    Ok(value.to_ascii_lowercase())
}

fn canonical_repository(value: &str) -> Result<String, WorkflowOsError> {
    if value.is_empty()
        || value.len() > 100
        || value == "."
        || value == ".."
        || value.contains("..")
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        || secret_like(value)
    {
        return Err(integration_error(
            "repository.name_invalid",
            "GitHub repository name is invalid",
        ));
    }
    Ok(value.to_ascii_lowercase())
}

fn secret_like(value: &str) -> bool {
    let value = value.to_ascii_lowercase();
    [
        "authorization",
        "bearer",
        "credential",
        "password",
        "private-key",
        "private_key",
        "secret",
        "token",
        "api-key",
        "api_key",
    ]
    .iter()
    .any(|marker| value.contains(marker))
}

fn integration_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("current_authority.github_repository_read.{suffix}"),
        message,
    )
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::{
        AdapterErrorKind, CapabilityAvailability, CapabilityDelegationPosture,
        CapabilityGrantDefinition, CapabilityGrantLifecycle, CapabilityGrantRequirements,
        CapabilityGrantScope, CurrentAuthoritySourceContractVersion,
        CurrentAuthoritySourceGeneration, CurrentAuthoritySourceId, GitHubHttpResponse,
        GitHubReadOnlyConfig, PolicyId, UnverifiedCapabilityUseAuthorityReceipt,
    };

    #[derive(Clone, Default)]
    struct CountingGitHubClient {
        calls: Arc<AtomicUsize>,
        paths: Arc<Mutex<Vec<String>>>,
        fail: bool,
    }

    impl GitHubReadOnlyClient for CountingGitHubClient {
        fn get(&self, path: &str, _accept: &str) -> Result<GitHubHttpResponse, AdapterError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.paths.lock().expect("paths lock").push(path.to_owned());
            if self.fail {
                return Err(AdapterError::new(
                    AdapterErrorKind::TransientNetworkFailure,
                    "github.fixture.unavailable",
                    "fixture GitHub read failed",
                ));
            }
            Ok(GitHubHttpResponse {
                status: 200,
                body: r#"{"full_name":"acme/widgets","default_branch":"main","private":true,"html_url":"https://github.com/acme/widgets"}"#.to_owned(),
                headers: BTreeMap::new(),
            })
        }
    }

    struct Fixture {
        source: RegisteredInMemoryCurrentAuthoritySource,
        actor: ActorId,
        workflow_id: WorkflowId,
        workflow_version: WorkflowVersion,
        schema_version: SchemaVersion,
        spec_hash: SpecContentHash,
        run_id: WorkflowRunId,
        step_id: StepId,
        harness_contract_id: HarnessContractId,
        harness_contract_version: HarnessContractVersion,
        correlation_id: CorrelationId,
        redaction: RedactionMetadata,
        precheck: AdapterPolicyPrecheck,
    }

    impl Fixture {
        fn new(scope: CapabilityGrantScope) -> Self {
            Self::new_with(
                scope,
                CapabilityGrantRequirements::default(),
                CapabilityAvailability::Available,
            )
        }

        fn new_with(
            scope: CapabilityGrantScope,
            requirements: CapabilityGrantRequirements,
            availability_posture: CapabilityAvailability,
        ) -> Self {
            let capability = CapabilityReference::new(GITHUB_READ_CAPABILITY).expect("capability");
            let resource = CapabilityResourceScope::new(
                CapabilityResourceKind::AdapterResource,
                "github/acme/widgets",
            )
            .expect("resource");
            let observed_at = timestamp("2026-10-22T10:10:00Z");
            let grant = CapabilityGrant::new(CapabilityGrantDefinition {
                grant_id: CapabilityGrantId::new("grant/github-read").expect("grant"),
                subject: ActorId::new("agent/reader").expect("subject"),
                capability: capability.clone(),
                resource: resource.clone(),
                scope,
                issuer: ActorId::new("system/authority").expect("issuer"),
                issued_at: timestamp("2026-10-22T10:00:00Z"),
                expires_at: Some(timestamp("2026-10-22T11:00:00Z")),
                lifecycle: CapabilityGrantLifecycle::Active,
                revocation_reference: None,
                delegation: CapabilityDelegationPosture::Disabled,
                requirements,
                sensitivity_ceiling: WorkReportSensitivity::Internal,
                redaction: RedactionMetadata::empty(),
            })
            .expect("grant");
            let availability = CapabilityAvailabilityRecord::new(
                capability,
                resource,
                availability_posture,
                observed_at,
                RedactionMetadata::empty(),
            )
            .expect("availability");
            let source = RegisteredInMemoryCurrentAuthoritySource::register(
                super::super::RegisteredInMemoryCurrentAuthoritySourceInput {
                    source_id: CurrentAuthoritySourceId::new("source/github-authority")
                        .expect("source"),
                    contract_version: CurrentAuthoritySourceContractVersion::new("v1")
                        .expect("contract version"),
                    configuration_commitment: SpecContentHash::from_text("github authority"),
                    core_maximum_observation_age_seconds: 3_600,
                    sensitivity: WorkReportSensitivity::Internal,
                    observed_at,
                    source_valid_through: Some(timestamp("2026-10-22T10:30:00Z")),
                    generation: Some(CurrentAuthoritySourceGeneration::new(1).expect("generation")),
                    complete_grant_inventory: vec![grant],
                    complete_availability_inventory: vec![availability],
                    complete_context_reference_inventory: Vec::new(),
                },
            )
            .expect("source");
            Self {
                source,
                actor: ActorId::new("agent/reader").expect("actor"),
                workflow_id: WorkflowId::new("workflow/read").expect("workflow"),
                workflow_version: WorkflowVersion::new("v1").expect("workflow version"),
                schema_version: SchemaVersion::new("workflowos.dev/v0").expect("schema"),
                spec_hash: SpecContentHash::from_text("workflow spec"),
                run_id: WorkflowRunId::new("run-read").expect("run"),
                step_id: StepId::new("read").expect("step"),
                harness_contract_id: HarnessContractId::new("harness/read").expect("harness"),
                harness_contract_version: HarnessContractVersion::new("v1")
                    .expect("harness version"),
                correlation_id: CorrelationId::new("correlation/read").expect("correlation"),
                redaction: RedactionMetadata::empty(),
                precheck: AdapterPolicyPrecheck::fixture_test_allowed(vec![
                    "policy.github.read".to_owned()
                ]),
            }
        }

        fn exact() -> Self {
            Self::new(
                CapabilityGrantScope::new(
                    WorkflowId::new("workflow/read").expect("workflow"),
                    Some(WorkflowRunId::new("run-read").expect("run")),
                    Some(StepId::new("read").expect("step")),
                    Some(HarnessContractId::new("harness/read").expect("harness")),
                )
                .expect("scope"),
            )
        }

        fn input<'a, C>(
            &'a self,
            adapter: &'a GitHubReadOnlyAdapter<C>,
            owner: &'a str,
            repository: &'a str,
        ) -> CurrentAuthorityGitHubRepositoryReadInput<'a, C> {
            CurrentAuthorityGitHubRepositoryReadInput {
                actor: &self.actor,
                workflow_id: &self.workflow_id,
                workflow_version: &self.workflow_version,
                schema_version: &self.schema_version,
                spec_hash: &self.spec_hash,
                run_id: &self.run_id,
                step_id: &self.step_id,
                harness_contract_id: &self.harness_contract_id,
                harness_contract_version: &self.harness_contract_version,
                owner,
                repository,
                requested_sensitivity: WorkReportSensitivity::Internal,
                evaluated_at: timestamp("2026-10-22T10:15:00Z"),
                source_continuity: CapabilityUseSourceContinuity::FirstObservation,
                correlation_id: &self.correlation_id,
                redaction: &self.redaction,
                policy_precheck: &self.precheck,
                adapter,
            }
        }
    }

    fn timestamp(value: &str) -> Timestamp {
        Timestamp::parse_rfc3339(value).expect("timestamp")
    }

    fn adapter(client: CountingGitHubClient) -> GitHubReadOnlyAdapter<CountingGitHubClient> {
        GitHubReadOnlyAdapter::new(GitHubReadOnlyConfig::fixture().expect("config"), client)
    }

    #[test]
    fn exact_authority_performs_one_canonical_read_and_emits_payload_free_receipt() {
        let fixture = Fixture::exact();
        let client = CountingGitHubClient::default();
        let calls = Arc::clone(&client.calls);
        let paths = Arc::clone(&client.paths);
        let adapter = adapter(client);

        let outcome = fixture
            .source
            .read_github_repository_metadata(&fixture.input(&adapter, "Acme", "Widgets"));

        let CurrentAuthorityGitHubRepositoryReadOutcome::Succeeded {
            projection,
            receipt,
            ..
        } = outcome
        else {
            panic!("expected successful authorized read: {outcome:?}");
        };
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            paths.lock().expect("paths").as_slice(),
            ["/repos/acme/widgets"]
        );
        assert_eq!(projection.entries().len(), 1);
        receipt.validate().expect("receipt");
        let serialized = serde_json::to_string(&receipt).expect("serialize receipt");
        assert!(!serialized.contains("acme"));
        assert!(!serialized.contains("widgets"));
        let claim: UnverifiedCapabilityUseAuthorityReceipt =
            serde_json::from_str(&serialized).expect("unverified claim");
        claim.validate_claim().expect("claim");
    }

    #[test]
    fn mismatched_policy_provenance_blocks_before_adapter_call() {
        let mut fixture = Fixture::exact();
        fixture.precheck =
            AdapterPolicyPrecheck::runtime_allowed(vec!["policy.github.read".to_owned()]);
        let client = CountingGitHubClient::default();
        let calls = Arc::clone(&client.calls);
        let adapter = adapter(client);

        let outcome = fixture
            .source
            .read_github_repository_metadata(&fixture.input(&adapter, "acme", "widgets"));

        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert!(matches!(
            outcome,
            CurrentAuthorityGitHubRepositoryReadOutcome::BlockedBeforeRead { ref error }
                if error.code() == "current_authority.github_repository_read.policy_precheck.invalid"
        ));
    }

    #[test]
    fn workflow_wide_grant_is_insufficient_and_never_calls_adapter() {
        let fixture = Fixture::new(
            CapabilityGrantScope::new(
                WorkflowId::new("workflow/read").expect("workflow"),
                None,
                None,
                None,
            )
            .expect("scope"),
        );
        let client = CountingGitHubClient::default();
        let calls = Arc::clone(&client.calls);
        let adapter = adapter(client);

        let outcome = fixture
            .source
            .read_github_repository_metadata(&fixture.input(&adapter, "acme", "widgets"));

        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert!(matches!(
            outcome,
            CurrentAuthorityGitHubRepositoryReadOutcome::BlockedBeforeRead { .. }
        ));
    }

    #[test]
    fn adapter_failure_is_explicit_and_has_no_receipt() {
        let fixture = Fixture::exact();
        let client = CountingGitHubClient {
            fail: true,
            ..CountingGitHubClient::default()
        };
        let calls = Arc::clone(&client.calls);
        let adapter = adapter(client);

        let outcome = fixture
            .source
            .read_github_repository_metadata(&fixture.input(&adapter, "acme", "widgets"));

        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert!(matches!(
            outcome,
            CurrentAuthorityGitHubRepositoryReadOutcome::ReadFailed { .. }
        ));
    }

    #[test]
    fn stale_source_blocks_before_adapter_call() {
        let mut fixture = Fixture::exact();
        fixture.source.observed_at = timestamp("2026-10-22T08:00:00Z");
        fixture.source.source_valid_through = Some(timestamp("2026-10-22T08:30:00Z"));
        let client = CountingGitHubClient::default();
        let calls = Arc::clone(&client.calls);
        let adapter = adapter(client);

        let outcome = fixture
            .source
            .read_github_repository_metadata(&fixture.input(&adapter, "acme", "widgets"));

        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert!(matches!(
            outcome,
            CurrentAuthorityGitHubRepositoryReadOutcome::BlockedBeforeRead { ref error }
                if error.code() == "current_authority.github_repository_read.source.not_fresh"
        ));
    }

    #[test]
    fn incomplete_exact_query_snapshot_blocks_before_adapter_call() {
        let mut fixture = Fixture::exact();
        fixture.source.availability_records.clear();
        let client = CountingGitHubClient::default();
        let calls = Arc::clone(&client.calls);
        let adapter = adapter(client);

        let outcome = fixture
            .source
            .read_github_repository_metadata(&fixture.input(&adapter, "acme", "widgets"));

        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert!(matches!(
            outcome,
            CurrentAuthorityGitHubRepositoryReadOutcome::BlockedBeforeRead { ref error }
                if error.code() == "current_authority.source.snapshot.query_coverage_incomplete"
        ));
    }

    #[test]
    fn matching_prior_source_observation_allows_the_read() {
        let fixture = Fixture::exact();
        let client = CountingGitHubClient::default();
        let calls = Arc::clone(&client.calls);
        let adapter = adapter(client);
        let mut input = fixture.input(&adapter, "acme", "widgets");
        input.source_continuity = CapabilityUseSourceContinuity::PriorObservation {
            generation: CurrentAuthoritySourceGeneration::new(1).expect("generation"),
            watermark: CurrentAuthoritySourceWatermark::new(format!(
                "watermark/{}",
                fixture.source.inventory_commitment.as_str()
            ))
            .expect("watermark"),
        };

        let outcome = fixture.source.read_github_repository_metadata(&input);

        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert!(matches!(
            outcome,
            CurrentAuthorityGitHubRepositoryReadOutcome::Succeeded { .. }
        ));
    }

    #[test]
    fn source_generation_rollback_blocks_before_adapter_call() {
        let fixture = Fixture::exact();
        let client = CountingGitHubClient::default();
        let calls = Arc::clone(&client.calls);
        let adapter = adapter(client);
        let mut input = fixture.input(&adapter, "acme", "widgets");
        input.source_continuity = CapabilityUseSourceContinuity::PriorObservation {
            generation: CurrentAuthoritySourceGeneration::new(2).expect("generation"),
            watermark: CurrentAuthoritySourceWatermark::new("watermark/prior").expect("watermark"),
        };

        let outcome = fixture.source.read_github_repository_metadata(&input);

        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert!(matches!(
            outcome,
            CurrentAuthorityGitHubRepositoryReadOutcome::BlockedBeforeRead { ref error }
                if error.code()
                    == "current_authority.github_repository_read.source.generation_rollback"
        ));
    }

    #[test]
    fn same_generation_watermark_substitution_blocks_before_adapter_call() {
        let fixture = Fixture::exact();
        let client = CountingGitHubClient::default();
        let calls = Arc::clone(&client.calls);
        let adapter = adapter(client);
        let mut input = fixture.input(&adapter, "acme", "widgets");
        input.source_continuity = CapabilityUseSourceContinuity::PriorObservation {
            generation: CurrentAuthoritySourceGeneration::new(1).expect("generation"),
            watermark: CurrentAuthoritySourceWatermark::new("watermark/substituted")
                .expect("watermark"),
        };

        let outcome = fixture.source.read_github_repository_metadata(&input);

        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert!(matches!(
            outcome,
            CurrentAuthorityGitHubRepositoryReadOutcome::BlockedBeforeRead { ref error }
                if error.code()
                    == "current_authority.github_repository_read.source.watermark_changed_without_generation"
        ));
    }

    #[test]
    fn exact_execution_identity_mismatches_block_before_adapter_call() {
        let fixture = Fixture::exact();
        let client = CountingGitHubClient::default();
        let calls = Arc::clone(&client.calls);
        let adapter = adapter(client);
        let wrong_run = WorkflowRunId::new("run-other").expect("run");
        let wrong_step = StepId::new("other").expect("step");
        let wrong_harness = HarnessContractId::new("harness/other").expect("harness");

        let mut input = fixture.input(&adapter, "acme", "widgets");
        input.run_id = &wrong_run;
        assert!(matches!(
            fixture.source.read_github_repository_metadata(&input),
            CurrentAuthorityGitHubRepositoryReadOutcome::BlockedBeforeRead { .. }
        ));
        let mut input = fixture.input(&adapter, "acme", "widgets");
        input.step_id = &wrong_step;
        assert!(matches!(
            fixture.source.read_github_repository_metadata(&input),
            CurrentAuthorityGitHubRepositoryReadOutcome::BlockedBeforeRead { .. }
        ));
        let mut input = fixture.input(&adapter, "acme", "widgets");
        input.harness_contract_id = &wrong_harness;
        assert!(matches!(
            fixture.source.read_github_repository_metadata(&input),
            CurrentAuthorityGitHubRepositoryReadOutcome::BlockedBeforeRead { .. }
        ));

        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn invalid_repository_identity_boundaries_block_before_adapter_call() {
        let fixture = Fixture::exact();
        let client = CountingGitHubClient::default();
        let calls = Arc::clone(&client.calls);
        let adapter = adapter(client);

        for (owner, repository) in [
            ("-acme", "widgets"),
            ("acme-", "widgets"),
            ("acme/org", "widgets"),
            ("acme", ".."),
            ("acme", "widget/name"),
        ] {
            assert!(matches!(
                fixture
                    .source
                    .read_github_repository_metadata(&fixture.input(&adapter, owner, repository)),
                CurrentAuthorityGitHubRepositoryReadOutcome::BlockedBeforeRead { .. }
            ));
        }
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn unavailable_capability_blocks_before_adapter_call() {
        let scope = CapabilityGrantScope::new(
            WorkflowId::new("workflow/read").expect("workflow"),
            Some(WorkflowRunId::new("run-read").expect("run")),
            Some(StepId::new("read").expect("step")),
            Some(HarnessContractId::new("harness/read").expect("harness")),
        )
        .expect("scope");
        let fixture = Fixture::new_with(
            scope,
            CapabilityGrantRequirements::default(),
            CapabilityAvailability::DeclaredNotConnected,
        );
        let client = CountingGitHubClient::default();
        let calls = Arc::clone(&client.calls);
        let adapter = adapter(client);

        let outcome = fixture
            .source
            .read_github_repository_metadata(&fixture.input(&adapter, "acme", "widgets"));

        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert!(matches!(
            outcome,
            CurrentAuthorityGitHubRepositoryReadOutcome::BlockedBeforeRead { .. }
        ));
    }

    #[test]
    fn prerequisite_bearing_grant_blocks_before_adapter_call() {
        let scope = CapabilityGrantScope::new(
            WorkflowId::new("workflow/read").expect("workflow"),
            Some(WorkflowRunId::new("run-read").expect("run")),
            Some(StepId::new("read").expect("step")),
            Some(HarnessContractId::new("harness/read").expect("harness")),
        )
        .expect("scope");
        let fixture = Fixture::new_with(
            scope,
            CapabilityGrantRequirements::new(
                vec![PolicyId::new("policy/github-read").expect("policy")],
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )
            .expect("requirements"),
            CapabilityAvailability::Available,
        );
        let client = CountingGitHubClient::default();
        let calls = Arc::clone(&client.calls);
        let adapter = adapter(client);

        let outcome = fixture
            .source
            .read_github_repository_metadata(&fixture.input(&adapter, "acme", "widgets"));

        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert!(matches!(
            outcome,
            CurrentAuthorityGitHubRepositoryReadOutcome::BlockedBeforeRead { .. }
        ));
    }

    #[test]
    fn sensitivity_above_grant_ceiling_blocks_before_adapter_call() {
        let fixture = Fixture::exact();
        let client = CountingGitHubClient::default();
        let calls = Arc::clone(&client.calls);
        let adapter = adapter(client);
        let mut input = fixture.input(&adapter, "acme", "widgets");
        input.requested_sensitivity = WorkReportSensitivity::Confidential;

        let outcome = fixture.source.read_github_repository_metadata(&input);

        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert!(matches!(
            outcome,
            CurrentAuthorityGitHubRepositoryReadOutcome::BlockedBeforeRead { .. }
        ));
    }

    #[test]
    fn receipt_failure_is_distinct_from_read_failure() {
        let fixture = Fixture::exact();
        let client = CountingGitHubClient::default();
        let calls = Arc::clone(&client.calls);
        let adapter = adapter(client);
        let input = fixture.input(&adapter, "acme", "widgets");

        let outcome = fixture
            .source
            .read_github_repository_metadata_with_receipt(&input, |_| {
                Err(integration_error(
                    "receipt.test_failure",
                    "receipt construction failed",
                ))
            });

        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert!(matches!(
            outcome,
            CurrentAuthorityGitHubRepositoryReadOutcome::ReadSucceededReceiptFailed { .. }
        ));
    }

    #[test]
    fn invalid_secret_like_repository_is_rejected_without_leakage_or_adapter_call() {
        let fixture = Fixture::exact();
        let client = CountingGitHubClient::default();
        let calls = Arc::clone(&client.calls);
        let adapter = adapter(client);
        let secret = "token-ghp-super-secret";

        let outcome = fixture
            .source
            .read_github_repository_metadata(&fixture.input(&adapter, "acme", secret));

        assert_eq!(calls.load(Ordering::SeqCst), 0);
        let rendered = format!("{outcome:?}");
        assert!(!rendered.contains(secret));
        assert!(rendered.contains("repository.name_invalid"));
    }
}
