//! Persistent collaborative identity authority-source request and evidence models.

use std::collections::BTreeSet;
use std::fmt;

use serde::Serialize;

use crate::{
    validate_collaborative_identity_binding_set, ActorId,
    CollaborativeIdentityProviderRegistration, CollaborativePrincipalIdentityBinding,
    CollaborativePrincipalRecord, CollaborativeProjectGrant, CollaborativeProjectGrantRevocation,
    CurrentAuthorityFactFamily, CurrentAuthoritySourceCompleteness,
    CurrentAuthoritySourceConsistency, CurrentAuthoritySourceContractVersion,
    CurrentAuthoritySourceFactCount, CurrentAuthoritySourceGeneration, CurrentAuthoritySourceId,
    CurrentAuthoritySourceKind, CurrentAuthoritySourceReadWindow,
    CurrentAuthoritySourceRegistration, CurrentAuthoritySourceRegistrationInput,
    CurrentAuthoritySourceSnapshot, CurrentAuthoritySourceSnapshotId,
    CurrentAuthoritySourceWatermark, HostedAuthorityRegistryRevision, HostedProjectCapability,
    OrganizationId, ProjectId, SpecContentHash, Timestamp, WorkReportSensitivity, WorkflowOsError,
    WorkflowOsErrorKind,
};

const MAX_AUTHORITY_FACTS_PER_FAMILY: usize = 10_000;
const POSTGRES_SOURCE_ID: &str = "core/postgres/collaborative-identity";
pub(crate) const POSTGRES_SOURCE_VERSION: &str = "v1";

/// Fact families required for one complete collaborative identity authority read.
pub const COLLABORATIVE_IDENTITY_AUTHORITY_FACT_FAMILIES: [CurrentAuthorityFactFamily; 5] = [
    CurrentAuthorityFactFamily::CapabilityGrants,
    CurrentAuthorityFactFamily::IdentityProviderRegistrations,
    CurrentAuthorityFactFamily::PrincipalIdentityBindings,
    CurrentAuthorityFactFamily::PrincipalLifecycle,
    CurrentAuthorityFactFamily::CapabilityGrantRevocations,
];

/// Version of the collaborative exact-query request commitment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborativeIdentityAuthoritySourceRequestVersion {
    /// Initial closed organization/project/principal/capability query.
    V1,
}

/// Exact collaborative authority query constructed from trusted route context.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CollaborativeIdentityAuthorityQuery {
    version: CollaborativeIdentityAuthoritySourceRequestVersion,
    organization_id: OrganizationId,
    project_id: ProjectId,
    principal_id: ActorId,
    capability: HostedProjectCapability,
    query_commitment: SpecContentHash,
}

impl CollaborativeIdentityAuthorityQuery {
    /// Creates one exact project-capability authority query.
    ///
    /// Construction does not authorize an action. The Core-owned source must resolve it.
    ///
    /// # Errors
    ///
    /// Returns a stable error when the query cannot be committed.
    pub fn new(
        organization_id: OrganizationId,
        project_id: ProjectId,
        principal_id: ActorId,
        capability: HostedProjectCapability,
    ) -> Result<Self, WorkflowOsError> {
        let mut value = Self {
            version: CollaborativeIdentityAuthoritySourceRequestVersion::V1,
            organization_id,
            project_id,
            principal_id,
            capability,
            query_commitment: SpecContentHash::from_text("uninitialized"),
        };
        value.query_commitment = hash_serializable(
            "collaborative-identity-query-v1",
            &(
                value.version,
                &value.organization_id,
                &value.project_id,
                &value.principal_id,
                value.capability,
            ),
        )?;
        Ok(value)
    }

    /// Returns the organization subject.
    #[must_use]
    pub const fn organization_id(&self) -> &OrganizationId {
        &self.organization_id
    }

    /// Returns the exact project subject.
    #[must_use]
    pub const fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    /// Returns the local principal subject.
    #[must_use]
    pub const fn principal_id(&self) -> &ActorId {
        &self.principal_id
    }

    /// Returns the requested closed capability.
    #[must_use]
    pub const fn capability(&self) -> HostedProjectCapability {
        self.capability
    }

    /// Returns the payload-free exact-query commitment.
    #[must_use]
    pub const fn query_commitment(&self) -> &SpecContentHash {
        &self.query_commitment
    }
}

impl fmt::Debug for CollaborativeIdentityAuthorityQuery {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CollaborativeIdentityAuthorityQuery")
            .field("version", &self.version)
            .field("organization", &"[REDACTED]")
            .field("project", &"[REDACTED]")
            .field("principal", &"[REDACTED]")
            .field("capability", &self.capability)
            .field("query_commitment", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

/// Complete caller-proposed identity fact inventory for one authority revision.
///
/// This value is validated input to the Core-owned persistent source. It is not current
/// authority and cannot authorize an operation by itself.
#[derive(Clone, Eq, PartialEq)]
pub struct CollaborativeIdentityAuthorityRevisionCandidate {
    authority_revision: HostedAuthorityRegistryRevision,
    providers: Vec<CollaborativeIdentityProviderRegistration>,
    bindings: Vec<CollaborativePrincipalIdentityBinding>,
    principals: Vec<CollaborativePrincipalRecord>,
    grants: Vec<CollaborativeProjectGrant>,
    revocations: Vec<CollaborativeProjectGrantRevocation>,
    commitment: SpecContentHash,
}

impl CollaborativeIdentityAuthorityRevisionCandidate {
    /// Validates and canonicalizes one complete proposed authority inventory.
    ///
    /// # Errors
    ///
    /// Rejects oversized, ambiguous, cross-organization, dangling, or duplicate facts.
    #[allow(clippy::too_many_lines)]
    pub fn new(
        authority_revision: HostedAuthorityRegistryRevision,
        mut providers: Vec<CollaborativeIdentityProviderRegistration>,
        mut bindings: Vec<CollaborativePrincipalIdentityBinding>,
        mut principals: Vec<CollaborativePrincipalRecord>,
        mut grants: Vec<CollaborativeProjectGrant>,
        mut revocations: Vec<CollaborativeProjectGrantRevocation>,
    ) -> Result<Self, WorkflowOsError> {
        validate_family_size(providers.len())?;
        validate_family_size(bindings.len())?;
        validate_family_size(principals.len())?;
        validate_family_size(grants.len())?;
        validate_family_size(revocations.len())?;
        if providers.is_empty() || bindings.is_empty() || principals.is_empty() {
            return Err(source_error(
                "candidate.identity_facts.missing",
                "collaborative identity authority candidate is missing required identity facts",
            ));
        }

        providers.sort_by(|left, right| left.provider_id().cmp(right.provider_id()));
        bindings.sort_by(|left, right| left.binding_id().cmp(right.binding_id()));
        principals.sort_by(|left, right| left.principal_id().cmp(right.principal_id()));
        grants.sort_by(|left, right| left.grant_id().cmp(right.grant_id()));
        revocations.sort_by(|left, right| left.revocation_id().cmp(right.revocation_id()));

        validate_collaborative_identity_binding_set(&bindings)?;
        let organization_id = providers[0].organization_id();
        if providers.iter().any(|provider| {
            provider.organization_id() != organization_id
                || provider.revision() > authority_revision
        }) || bindings
            .iter()
            .any(|binding| binding.organization_id() != organization_id)
            || principals
                .iter()
                .any(|principal| principal.organization_id() != organization_id)
            || grants.iter().any(|grant| {
                grant.organization_id() != organization_id
                    || grant.authority_registry_revision() > authority_revision
            })
            || revocations
                .iter()
                .any(|revocation| revocation.organization_id() != organization_id)
        {
            return Err(source_error(
                "candidate.scope.invalid",
                "collaborative identity authority candidate scope is invalid",
            ));
        }

        reject_duplicate_by(providers.iter().map(|value| value.provider_id().as_str()))?;
        reject_duplicate_by(bindings.iter().map(|value| value.binding_id().as_str()))?;
        reject_duplicate_by(principals.iter().map(|value| value.principal_id().as_str()))?;
        reject_duplicate_by(grants.iter().map(|value| value.grant_id().as_str()))?;
        reject_duplicate_by(
            revocations
                .iter()
                .map(|value| value.revocation_id().as_str()),
        )?;
        reject_duplicate_by(revocations.iter().map(|value| value.grant_id().as_str()))?;

        for binding in &bindings {
            let provider = providers
                .iter()
                .find(|provider| provider.provider_id() == binding.provider_id())
                .ok_or_else(|| {
                    source_error(
                        "candidate.binding.provider_missing",
                        "collaborative identity authority candidate has a dangling binding",
                    )
                })?;
            if provider.revision() != binding.provider_revision()
                || !principals
                    .iter()
                    .any(|principal| principal.principal_id() == binding.principal_id())
            {
                return Err(source_error(
                    "candidate.binding.invalid",
                    "collaborative identity authority candidate binding is invalid",
                ));
            }
        }
        for grant in &grants {
            if !principals
                .iter()
                .any(|principal| principal.principal_id() == grant.principal_id())
            {
                return Err(source_error(
                    "candidate.grant.principal_missing",
                    "collaborative identity authority candidate has a dangling grant",
                ));
            }
        }
        for revocation in &revocations {
            let grant = grants
                .iter()
                .find(|grant| grant.grant_id() == revocation.grant_id())
                .ok_or_else(|| {
                    source_error(
                        "candidate.revocation.grant_missing",
                        "collaborative identity authority candidate has a dangling revocation",
                    )
                })?;
            if grant.commitment() != revocation.grant_commitment()
                || grant.project_id() != revocation.project_id()
                || grant.principal_id() != revocation.principal_id()
            {
                return Err(source_error(
                    "candidate.revocation.invalid",
                    "collaborative identity authority candidate revocation is invalid",
                ));
            }
        }

        let commitment = hash_serializable(
            "collaborative-identity-authority-revision-v1",
            &(
                authority_revision,
                &providers,
                &bindings,
                &principals,
                &grants,
                &revocations,
            ),
        )?;
        Ok(Self {
            authority_revision,
            providers,
            bindings,
            principals,
            grants,
            revocations,
            commitment,
        })
    }

    #[must_use]
    /// Returns the candidate authority revision.
    pub const fn authority_revision(&self) -> HostedAuthorityRegistryRevision {
        self.authority_revision
    }

    #[must_use]
    /// Returns the organization covered by this complete revision.
    pub fn organization_id(&self) -> &OrganizationId {
        self.providers[0].organization_id()
    }

    #[must_use]
    /// Returns canonical provider registrations.
    pub fn providers(&self) -> &[CollaborativeIdentityProviderRegistration] {
        &self.providers
    }

    #[must_use]
    /// Returns canonical principal bindings.
    pub fn bindings(&self) -> &[CollaborativePrincipalIdentityBinding] {
        &self.bindings
    }

    #[must_use]
    /// Returns canonical principal lifecycle records.
    pub fn principals(&self) -> &[CollaborativePrincipalRecord] {
        &self.principals
    }

    #[must_use]
    /// Returns canonical project grants.
    pub fn grants(&self) -> &[CollaborativeProjectGrant] {
        &self.grants
    }

    #[must_use]
    /// Returns canonical additive revocations.
    pub fn revocations(&self) -> &[CollaborativeProjectGrantRevocation] {
        &self.revocations
    }

    #[must_use]
    /// Returns the commitment to the complete canonical revision.
    pub const fn commitment(&self) -> &SpecContentHash {
        &self.commitment
    }

    /// Evaluates the exact query against this validated candidate at a supplied trusted time.
    ///
    /// This helper does not make the candidate authoritative and does not authorize an action.
    #[must_use]
    pub fn active_matching_grant_count(
        &self,
        query: &CollaborativeIdentityAuthorityQuery,
        evaluated_at: Timestamp,
    ) -> u64 {
        self.active_matching_grants(query, evaluated_at).count() as u64
    }

    pub(crate) fn single_active_matching_grant_commitment(
        &self,
        query: &CollaborativeIdentityAuthorityQuery,
        evaluated_at: Timestamp,
    ) -> Option<SpecContentHash> {
        let mut matches = self.active_matching_grants(query, evaluated_at);
        let commitment = SpecContentHash::new(matches.next()?.commitment().as_str()).ok()?;
        matches.next().is_none().then_some(commitment)
    }

    fn active_matching_grants<'a>(
        &'a self,
        query: &'a CollaborativeIdentityAuthorityQuery,
        evaluated_at: Timestamp,
    ) -> impl Iterator<Item = &'a CollaborativeProjectGrant> + 'a {
        let organization_matches = self.organization_id() == query.organization_id();
        let principal_active = self.principals.iter().any(|principal| {
            principal.principal_id() == query.principal_id()
                && principal.effective_at() <= evaluated_at
                && principal.lifecycle() == crate::CollaborativePrincipalLifecycle::Active
        });
        let binding_active = self.bindings.iter().any(|binding| {
            binding.principal_id() == query.principal_id()
                && binding.issued_at() <= evaluated_at
                && binding.lifecycle()
                    == crate::CollaborativePrincipalIdentityBindingLifecycle::Active
                && self.providers.iter().any(|provider| {
                    provider.provider_id() == binding.provider_id()
                        && provider.revision() == binding.provider_revision()
                        && provider.lifecycle()
                            == crate::CollaborativeIdentityProviderLifecycle::Active
                })
        });
        self.grants.iter().filter(move |grant| {
            organization_matches
                && principal_active
                && binding_active
                && grant.project_id() == query.project_id()
                && grant.principal_id() == query.principal_id()
                && grant.capabilities().contains(&query.capability())
                && grant.effective_at() <= evaluated_at
                && grant
                    .expires_at()
                    .map_or(true, |expiry| expiry > evaluated_at)
                && !self.revocations.iter().any(|revocation| {
                    revocation.grant_id() == grant.grant_id()
                        && revocation.grant_commitment() == grant.commitment()
                        && revocation.revoked_at() <= evaluated_at
                })
        })
    }

    fn exact_query_fact_counts(&self, query: &CollaborativeIdentityAuthorityQuery) -> [u64; 5] {
        let bindings = self
            .bindings
            .iter()
            .filter(|binding| binding.principal_id() == query.principal_id())
            .collect::<Vec<_>>();
        let provider_ids = bindings
            .iter()
            .map(|binding| binding.provider_id().as_str())
            .collect::<BTreeSet<_>>();
        let grants = self
            .grants
            .iter()
            .filter(|grant| {
                grant.project_id() == query.project_id()
                    && grant.principal_id() == query.principal_id()
            })
            .collect::<Vec<_>>();
        let grant_ids = grants
            .iter()
            .map(|grant| grant.grant_id().as_str())
            .collect::<BTreeSet<_>>();
        [
            grants.len() as u64,
            self.providers
                .iter()
                .filter(|provider| provider_ids.contains(provider.provider_id().as_str()))
                .count() as u64,
            bindings.len() as u64,
            self.principals
                .iter()
                .filter(|principal| principal.principal_id() == query.principal_id())
                .count() as u64,
            self.revocations
                .iter()
                .filter(|revocation| grant_ids.contains(revocation.grant_id().as_str()))
                .count() as u64,
        ]
    }
}

impl fmt::Debug for CollaborativeIdentityAuthorityRevisionCandidate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CollaborativeIdentityAuthorityRevisionCandidate")
            .field("authority_revision", &self.authority_revision)
            .field("provider_count", &self.providers.len())
            .field("binding_count", &self.bindings.len())
            .field("principal_count", &self.principals.len())
            .field("grant_count", &self.grants.len())
            .field("revocation_count", &self.revocations.len())
            .field("commitment", &"[REDACTED]")
            .finish()
    }
}

/// Result of activating one complete authority revision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollaborativeIdentityAuthorityRevisionActivation {
    /// This call atomically activated a higher revision.
    Activated,
    /// The exact already-active revision was reconciled.
    ReconciledExisting,
}

/// Whether a complete source read found an active matching grant.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborativeIdentityAuthorityMatchPosture {
    /// At least one active exact-project grant contains the requested capability.
    MatchingGrantPresent,
    /// The complete exact query contains no active matching grant.
    NoMatchingGrant,
}

/// Payload-free evidence from one complete Core-owned collaborative authority source read.
///
/// This record is not a reusable authorization token and no runtime consumer accepts it in the
/// current phase.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CollaborativeIdentityAuthoritySourceRead {
    query_commitment: SpecContentHash,
    authority_revision: HostedAuthorityRegistryRevision,
    match_posture: CollaborativeIdentityAuthorityMatchPosture,
    matching_active_grant_count: u64,
    source_snapshot: CurrentAuthoritySourceSnapshot,
}

impl CollaborativeIdentityAuthoritySourceRead {
    pub(crate) fn new(
        query_commitment: SpecContentHash,
        authority_revision: HostedAuthorityRegistryRevision,
        matching_active_grant_count: u64,
        source_snapshot: CurrentAuthoritySourceSnapshot,
    ) -> Result<Self, WorkflowOsError> {
        let match_posture = match_posture_for_count(matching_active_grant_count)?;
        Ok(Self {
            query_commitment,
            authority_revision,
            match_posture,
            matching_active_grant_count,
            source_snapshot,
        })
    }

    #[must_use]
    /// Returns the exact query commitment.
    pub const fn query_commitment(&self) -> &SpecContentHash {
        &self.query_commitment
    }

    #[must_use]
    /// Returns the authoritative revision observed by this read.
    pub const fn authority_revision(&self) -> HostedAuthorityRegistryRevision {
        self.authority_revision
    }

    #[must_use]
    /// Returns whether an active exact matching grant was present.
    pub const fn match_posture(&self) -> CollaborativeIdentityAuthorityMatchPosture {
        self.match_posture
    }

    #[must_use]
    /// Returns the count of active exact matching grants.
    pub const fn matching_active_grant_count(&self) -> u64 {
        self.matching_active_grant_count
    }

    #[must_use]
    /// Returns the payload-free source snapshot evidence.
    pub const fn source_snapshot(&self) -> &CurrentAuthoritySourceSnapshot {
        &self.source_snapshot
    }
}

impl fmt::Debug for CollaborativeIdentityAuthoritySourceRead {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CollaborativeIdentityAuthoritySourceRead")
            .field("query_commitment", &"[REDACTED]")
            .field("authority_revision", &self.authority_revision)
            .field("match_posture", &self.match_posture)
            .field(
                "matching_active_grant_count",
                &self.matching_active_grant_count,
            )
            .field("source_snapshot", &"[REDACTED]")
            .finish()
    }
}

pub(crate) struct CollaborativeIdentityAuthoritySourceRequest {
    registration_commitment: SpecContentHash,
    query_commitment: SpecContentHash,
    requested_fact_families: Vec<CurrentAuthorityFactFamily>,
    evaluated_at: Timestamp,
    request_commitment: SpecContentHash,
}

impl CollaborativeIdentityAuthoritySourceRequest {
    pub(crate) fn new(
        registration: &CurrentAuthoritySourceRegistration,
        query: &CollaborativeIdentityAuthorityQuery,
        evaluated_at: Timestamp,
    ) -> Result<Self, WorkflowOsError> {
        registration.validate()?;
        if registration.supported_fact_families() != COLLABORATIVE_IDENTITY_AUTHORITY_FACT_FAMILIES
        {
            return Err(source_error(
                "request.registration.invalid",
                "collaborative identity authority source registration is invalid",
            ));
        }
        let mut value = Self {
            registration_commitment: registration.registration_commitment().clone(),
            query_commitment: query.query_commitment().clone(),
            requested_fact_families: COLLABORATIVE_IDENTITY_AUTHORITY_FACT_FAMILIES.to_vec(),
            evaluated_at,
            request_commitment: SpecContentHash::from_text("uninitialized"),
        };
        value.request_commitment = hash_serializable(
            "collaborative-identity-source-request-v1",
            &(
                CollaborativeIdentityAuthoritySourceRequestVersion::V1,
                &value.registration_commitment,
                &value.query_commitment,
                &value.requested_fact_families,
                value.evaluated_at,
            ),
        )?;
        Ok(value)
    }

    pub(crate) const fn registration_commitment(&self) -> &SpecContentHash {
        &self.registration_commitment
    }

    pub(crate) const fn request_commitment(&self) -> &SpecContentHash {
        &self.request_commitment
    }

    pub(crate) fn requested_fact_families(&self) -> &[CurrentAuthorityFactFamily] {
        &self.requested_fact_families
    }

    pub(crate) const fn evaluated_at(&self) -> Timestamp {
        self.evaluated_at
    }
}

pub(crate) fn postgres_collaborative_identity_source_registration(
) -> Result<CurrentAuthoritySourceRegistration, WorkflowOsError> {
    CurrentAuthoritySourceRegistration::new(CurrentAuthoritySourceRegistrationInput {
        source_id: CurrentAuthoritySourceId::new(POSTGRES_SOURCE_ID)?,
        contract_version: CurrentAuthoritySourceContractVersion::new(POSTGRES_SOURCE_VERSION)?,
        source_kind: CurrentAuthoritySourceKind::LocalAggregate,
        configuration_commitment: SpecContentHash::from_text(
            "postgres-collaborative-identity-authority-source-v1",
        ),
        supported_fact_families: COLLABORATIVE_IDENTITY_AUTHORITY_FACT_FAMILIES.to_vec(),
        consistency: crate::CurrentAuthoritySourceConsistency::AtomicSnapshot,
        core_maximum_observation_age_seconds: 1,
        sensitivity: WorkReportSensitivity::Confidential,
        redaction_required: true,
    })
}

pub(crate) fn build_postgres_collaborative_identity_source_read(
    query: &CollaborativeIdentityAuthorityQuery,
    candidate: &CollaborativeIdentityAuthorityRevisionCandidate,
    evaluated_at: Timestamp,
) -> Result<CollaborativeIdentityAuthoritySourceRead, WorkflowOsError> {
    let matching_active_grant_count = candidate.active_matching_grant_count(query, evaluated_at);
    let exact_fact_counts = candidate.exact_query_fact_counts(query);
    let registration = postgres_collaborative_identity_source_registration()?;
    let request =
        CollaborativeIdentityAuthoritySourceRequest::new(&registration, query, evaluated_at)?;
    let revision = candidate.authority_revision().get();
    let snapshot = CurrentAuthoritySourceSnapshot::new_from_committed_request(
        crate::current_authority_source::CurrentAuthoritySourceCommittedSnapshotInput {
            request: crate::current_authority_source::CurrentAuthoritySourceSnapshotRequest {
                registration_commitment: request.registration_commitment(),
                request_commitment: request.request_commitment(),
                requested_fact_families: request.requested_fact_families(),
                query_count: 1,
                evaluated_at: request.evaluated_at(),
            },
            registration: &registration,
            snapshot_id: CurrentAuthoritySourceSnapshotId::new(format!(
                "collaborative-identity-{revision}"
            ))?,
            watermark: CurrentAuthoritySourceWatermark::new(format!(
                "revision-{revision}-{}",
                candidate.commitment().as_str()
            ))?,
            generation: Some(CurrentAuthoritySourceGeneration::new(revision)?),
            read_window: CurrentAuthoritySourceReadWindow::new(
                evaluated_at,
                evaluated_at,
                evaluated_at,
            )?,
            completeness: CurrentAuthoritySourceCompleteness::CompleteForExactQuery,
            consistency: CurrentAuthoritySourceConsistency::AtomicSnapshot,
            source_valid_through: Some(evaluated_at),
            returned_fact_families: COLLABORATIVE_IDENTITY_AUTHORITY_FACT_FAMILIES.to_vec(),
            fact_counts: vec![
                CurrentAuthoritySourceFactCount::new(
                    CurrentAuthorityFactFamily::CapabilityGrants,
                    exact_fact_counts[0],
                ),
                CurrentAuthoritySourceFactCount::new(
                    CurrentAuthorityFactFamily::IdentityProviderRegistrations,
                    exact_fact_counts[1],
                ),
                CurrentAuthoritySourceFactCount::new(
                    CurrentAuthorityFactFamily::PrincipalIdentityBindings,
                    exact_fact_counts[2],
                ),
                CurrentAuthoritySourceFactCount::new(
                    CurrentAuthorityFactFamily::PrincipalLifecycle,
                    exact_fact_counts[3],
                ),
                CurrentAuthoritySourceFactCount::new(
                    CurrentAuthorityFactFamily::CapabilityGrantRevocations,
                    exact_fact_counts[4],
                ),
            ],
            records_commitment: candidate.commitment().clone(),
        },
    )?;
    CollaborativeIdentityAuthoritySourceRead::new(
        query.query_commitment().clone(),
        candidate.authority_revision(),
        matching_active_grant_count,
        snapshot,
    )
}

fn validate_family_size(size: usize) -> Result<(), WorkflowOsError> {
    if size > MAX_AUTHORITY_FACTS_PER_FAMILY {
        return Err(source_error(
            "candidate.family.too_large",
            "collaborative identity authority candidate fact family is too large",
        ));
    }
    Ok(())
}

fn reject_duplicate_by<'a>(values: impl Iterator<Item = &'a str>) -> Result<(), WorkflowOsError> {
    let mut unique = BTreeSet::new();
    if values.into_iter().any(|value| !unique.insert(value)) {
        return Err(source_error(
            "candidate.fact.duplicate",
            "collaborative identity authority candidate contains duplicate facts",
        ));
    }
    Ok(())
}

fn match_posture_for_count(
    matching_active_grant_count: u64,
) -> Result<CollaborativeIdentityAuthorityMatchPosture, WorkflowOsError> {
    match matching_active_grant_count {
        0 => Ok(CollaborativeIdentityAuthorityMatchPosture::NoMatchingGrant),
        1 => Ok(CollaborativeIdentityAuthorityMatchPosture::MatchingGrantPresent),
        _ => Err(source_error(
            "query.grant.ambiguous",
            "collaborative identity authority query has ambiguous matching grants",
        )),
    }
}

fn hash_serializable<T: Serialize>(
    domain: &str,
    value: &T,
) -> Result<SpecContentHash, WorkflowOsError> {
    let encoded = serde_json::to_string(&(domain, value)).map_err(|_| {
        source_error(
            "commitment.failed",
            "collaborative identity authority commitment failed",
        )
    })?;
    Ok(SpecContentHash::from_text(&encoded))
}

fn source_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::new(
        WorkflowOsErrorKind::InvalidState,
        concat_code(code),
        message,
    )
}

fn concat_code(code: &'static str) -> &'static str {
    match code {
        "candidate.identity_facts.missing" => {
            "collaborative_identity_authority_source.candidate.identity_facts.missing"
        }
        "candidate.scope.invalid" => {
            "collaborative_identity_authority_source.candidate.scope.invalid"
        }
        "candidate.binding.provider_missing" => {
            "collaborative_identity_authority_source.candidate.binding.provider_missing"
        }
        "candidate.binding.invalid" => {
            "collaborative_identity_authority_source.candidate.binding.invalid"
        }
        "candidate.grant.principal_missing" => {
            "collaborative_identity_authority_source.candidate.grant.principal_missing"
        }
        "candidate.revocation.grant_missing" => {
            "collaborative_identity_authority_source.candidate.revocation.grant_missing"
        }
        "candidate.revocation.invalid" => {
            "collaborative_identity_authority_source.candidate.revocation.invalid"
        }
        "candidate.family.too_large" => {
            "collaborative_identity_authority_source.candidate.family.too_large"
        }
        "candidate.fact.duplicate" => {
            "collaborative_identity_authority_source.candidate.fact.duplicate"
        }
        "request.registration.invalid" => {
            "collaborative_identity_authority_source.request.registration.invalid"
        }
        "query.grant.ambiguous" => "collaborative_identity_authority_source.query.grant.ambiguous",
        "commitment.failed" => "collaborative_identity_authority_source.commitment.failed",
        _ => "collaborative_identity_authority_source.invalid",
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::{
        match_posture_for_count, postgres_collaborative_identity_source_registration,
        CollaborativeIdentityAuthorityMatchPosture, COLLABORATIVE_IDENTITY_AUTHORITY_FACT_FAMILIES,
    };

    #[test]
    fn postgres_registration_matches_canonical_collaborative_family_contract() {
        let registration =
            postgres_collaborative_identity_source_registration().expect("valid registration");
        assert_eq!(
            registration.supported_fact_families(),
            COLLABORATIVE_IDENTITY_AUTHORITY_FACT_FAMILIES
        );
    }

    #[test]
    fn multiple_active_matching_grants_fail_closed() {
        assert_eq!(
            match_posture_for_count(0).expect("complete negative"),
            CollaborativeIdentityAuthorityMatchPosture::NoMatchingGrant
        );
        assert_eq!(
            match_posture_for_count(1).expect("one matching grant"),
            CollaborativeIdentityAuthorityMatchPosture::MatchingGrantPresent
        );
        let error = match_posture_for_count(2).expect_err("ambiguous grants must fail closed");
        assert_eq!(
            error.code(),
            "collaborative_identity_authority_source.query.grant.ambiguous"
        );
    }
}
