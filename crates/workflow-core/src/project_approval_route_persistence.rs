use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    ActorId, ApprovalReferenceId, ApprovalRequest, EventId, HostedPrincipalKind,
    HostedPrincipalRegistry, HostedProjectCapability, HostedProjectResourceBinding,
    HostedProjectResourceBindingStatus, HostedProjectResourceKind, HostedProjectScope,
    IdempotencyKey, ImmutableRunBundleManifest, ProjectApprovalRoute, ProjectApprovalRouteId,
    ProjectApprovalRouteStatus, ProjectApprovalRoutingReason, SpecContentHash, Timestamp,
    WorkflowOsError, WorkflowRunId,
};

const LOGICAL_SUBJECT_PREFIX: &str = "project-approval-route-subject-";
const LOGICAL_SUBJECT_DOMAIN: &str = "workflow-os/project-approval-route-subject/v1";
const SOURCE_COMMITMENT_DOMAIN: &str = "workflow-os/project-approval-route-source/v1";
const AUTHORITY_COMMITMENT_DOMAIN: &str = "workflow-os/project-approval-authority-view/v1";
const AUTHORITY_SNAPSHOT_COMMITMENT_DOMAIN: &str =
    "workflow-os/project-approval-authority-snapshot/v1";
const MAX_HOSTED_AUTHORITY_REGISTRY_REVISION: u64 = 9_223_372_036_854_775_807;
const MAX_LIST_LIMIT: usize = 1_000;

/// Version of the durable project approval route record.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectApprovalRouteRecordVersion {
    /// Initial create-only payload-free route record.
    V1,
}

/// Versioned algorithm for the complete route-source commitment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectApprovalRouteSourceCommitmentAlgorithm {
    /// Initial complete approval, bundle, project-binding, and authority commitment.
    V1,
}

/// Versioned algorithm for the immutable deployment authority-view commitment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectApprovalAuthorityViewCommitmentAlgorithm {
    /// Initial canonical organization/principal/project-capability commitment.
    V1,
}

/// Positive bounded revision of one deployment-owned hosted authority registry.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(into = "u64")]
pub struct HostedAuthorityRegistryRevision(u64);

impl HostedAuthorityRegistryRevision {
    /// Creates one validated authority registry revision.
    ///
    /// # Errors
    ///
    /// Rejects zero and values outside the supported durable storage bound.
    pub fn new(value: u64) -> Result<Self, WorkflowOsError> {
        if value == 0 || value > MAX_HOSTED_AUTHORITY_REGISTRY_REVISION {
            return Err(persistence_error(
                "project_approval_route_store.authority_registry_revision.invalid",
                "hosted authority registry revision is invalid",
            ));
        }
        Ok(Self(value))
    }

    /// Returns the positive revision value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Debug for HostedAuthorityRegistryRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("HostedAuthorityRegistryRevision([REDACTED])")
    }
}

impl TryFrom<u64> for HostedAuthorityRegistryRevision {
    type Error = WorkflowOsError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<HostedAuthorityRegistryRevision> for u64 {
    fn from(value: HostedAuthorityRegistryRevision) -> Self {
        value.0
    }
}

impl<'de> Deserialize<'de> for HostedAuthorityRegistryRevision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer).map_err(|_| {
            serde::de::Error::custom("hosted authority registry revision is invalid")
        })?;
        Self::new(value)
            .map_err(|_| serde::de::Error::custom("hosted authority registry revision is invalid"))
    }
}

macro_rules! hash_identity {
    ($name:ident, $prefix:expr, $code:literal, $label:literal) => {
        #[doc = $label]
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            /// Creates a validated content-derived identity.
            ///
            /// # Errors
            ///
            /// Rejects values outside the expected lowercase SHA-256 identity shape.
            pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
                let value = value.into();
                let digest = value
                    .strip_prefix($prefix)
                    .ok_or_else(|| persistence_error($code, concat!($label, " is invalid")))?;
                if digest.len() != 64
                    || !digest
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
                {
                    return Err(persistence_error($code, concat!($label, " is invalid")));
                }
                Ok(Self(value))
            }

            /// Returns the stable identity text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&"[REDACTED]")
                    .finish()
            }
        }

        impl TryFrom<String> for $name {
            type Error = WorkflowOsError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

hash_identity!(
    ProjectApprovalRouteLogicalSubjectId,
    LOGICAL_SUBJECT_PREFIX,
    "project_approval_route_store.logical_subject.invalid",
    "project approval route logical subject"
);

/// Payload-free commitment to the complete immutable deployment authority view.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectApprovalAuthorityViewCommitment {
    algorithm: ProjectApprovalAuthorityViewCommitmentAlgorithm,
    organization_id: crate::OrganizationId,
    fingerprint: SpecContentHash,
}

impl ProjectApprovalAuthorityViewCommitment {
    /// Derives a deterministic commitment from one bounded deployment authority view.
    ///
    /// Authentication-token digests are not represented by the input model and therefore
    /// cannot enter the commitment.
    ///
    /// # Errors
    ///
    /// Rejects a registry outside the exact organization scope.
    pub fn from_registry(
        scope: &HostedProjectScope,
        registry: &HostedPrincipalRegistry,
    ) -> Result<Self, WorkflowOsError> {
        scope.validate()?;
        if registry.organization_id() != scope.organization_id() {
            return Err(persistence_error(
                "project_approval_route_store.authority_view.invalid",
                "project approval authority view is invalid",
            ));
        }

        let mut hasher = domain_hasher(AUTHORITY_COMMITMENT_DOMAIN);
        hash_text(
            &mut hasher,
            "organization_id",
            scope.organization_id().as_str(),
        );
        hash_usize(&mut hasher, "principal_count", registry.principals().len());
        for principal in registry.principals() {
            hash_text(&mut hasher, "actor_id", principal.actor_id().as_str());
            hash_text(
                &mut hasher,
                "principal_kind",
                principal_kind_label(principal.principal_kind()),
            );
            hash_usize(&mut hasher, "grant_count", principal.grants().len());
            for grant in principal.grants() {
                hash_text(&mut hasher, "project_id", grant.project_id().as_str());
                hash_usize(&mut hasher, "capability_count", grant.capabilities().len());
                for capability in grant.capabilities() {
                    hash_text(&mut hasher, "capability", capability_label(*capability));
                }
            }
        }
        Ok(Self {
            algorithm: ProjectApprovalAuthorityViewCommitmentAlgorithm::V1,
            organization_id: scope.organization_id().clone(),
            fingerprint: SpecContentHash::from_bytes(hasher.finalize()),
        })
    }

    /// Returns the commitment algorithm.
    #[must_use]
    pub const fn algorithm(&self) -> ProjectApprovalAuthorityViewCommitmentAlgorithm {
        self.algorithm
    }

    /// Returns the canonical authority-view fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> &SpecContentHash {
        &self.fingerprint
    }

    /// Returns the exact organization represented by the complete authority view.
    #[must_use]
    pub const fn organization_id(&self) -> &crate::OrganizationId {
        &self.organization_id
    }
}

impl fmt::Debug for ProjectApprovalAuthorityViewCommitment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProjectApprovalAuthorityViewCommitment")
            .field("algorithm", &self.algorithm)
            .field("organization", &"[REDACTED]")
            .field("fingerprint", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

/// Complete commitment to one revision of the deployment-owned authority registry.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ProjectApprovalAuthoritySnapshotCommitment {
    revision: HostedAuthorityRegistryRevision,
    authority_view: ProjectApprovalAuthorityViewCommitment,
    fingerprint: SpecContentHash,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProjectApprovalAuthoritySnapshotCommitmentWire {
    revision: HostedAuthorityRegistryRevision,
    authority_view: ProjectApprovalAuthorityViewCommitment,
    fingerprint: SpecContentHash,
}

impl ProjectApprovalAuthoritySnapshotCommitment {
    /// Binds one validated registry revision to its complete canonical authority view.
    #[must_use]
    pub fn new(
        revision: HostedAuthorityRegistryRevision,
        authority_view: ProjectApprovalAuthorityViewCommitment,
    ) -> Self {
        let mut hasher = domain_hasher(AUTHORITY_SNAPSHOT_COMMITMENT_DOMAIN);
        hash_text(
            &mut hasher,
            "organization_id",
            authority_view.organization_id().as_str(),
        );
        hash_text(
            &mut hasher,
            "registry_revision",
            &revision.get().to_string(),
        );
        hash_text(
            &mut hasher,
            "authority_algorithm",
            authority_algorithm_label(authority_view.algorithm()),
        );
        hash_text(
            &mut hasher,
            "authority_fingerprint",
            authority_view.fingerprint().as_str(),
        );
        Self {
            revision,
            authority_view,
            fingerprint: SpecContentHash::from_bytes(hasher.finalize()),
        }
    }

    /// Returns the exact deployment authority registry revision.
    #[must_use]
    pub const fn revision(&self) -> HostedAuthorityRegistryRevision {
        self.revision
    }

    /// Returns the complete canonical authority-view commitment.
    #[must_use]
    pub const fn authority_view(&self) -> &ProjectApprovalAuthorityViewCommitment {
        &self.authority_view
    }

    /// Returns the revision-bound authority snapshot fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> &SpecContentHash {
        &self.fingerprint
    }
}

impl fmt::Debug for ProjectApprovalAuthoritySnapshotCommitment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProjectApprovalAuthoritySnapshotCommitment")
            .field("revision", &self.revision)
            .field("authority_view", &self.authority_view)
            .field("fingerprint", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for ProjectApprovalAuthoritySnapshotCommitment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ProjectApprovalAuthoritySnapshotCommitmentWire::deserialize(deserializer)
            .map_err(|_| {
                serde::de::Error::custom("invalid project approval authority snapshot commitment")
            })?;
        let commitment = Self::new(wire.revision, wire.authority_view);
        if commitment.fingerprint != wire.fingerprint {
            return Err(serde::de::Error::custom(
                "invalid project approval authority snapshot commitment",
            ));
        }
        Ok(commitment)
    }
}

/// Trusted typed inputs used to derive the complete payload-free source commitment.
pub struct ProjectApprovalRouteSourceCommitmentInput<'a> {
    /// Pending approval projection reconstructed from durable events.
    pub approval: &'a ApprovalRequest,
    /// Exact event that created the approval request.
    pub approval_request_event_id: &'a EventId,
    /// Validated coherent immutable run-bundle manifest.
    pub immutable_run_bundle: &'a ImmutableRunBundleManifest,
    /// Active exact-project binding for the run.
    pub run_binding: &'a HostedProjectResourceBinding,
    /// Exact escalation event for escalation-contact routing.
    pub escalation_event_id: Option<&'a EventId>,
    /// Complete revision-bound deployment authority snapshot commitment.
    pub authority_snapshot: &'a ProjectApprovalAuthoritySnapshotCommitment,
}

/// Payload-free commitment to all authenticated sources used for one route decision.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectApprovalRouteSourceCommitment {
    algorithm: ProjectApprovalRouteSourceCommitmentAlgorithm,
    route_id: ProjectApprovalRouteId,
    authority_snapshot: ProjectApprovalAuthoritySnapshotCommitment,
    fingerprint: SpecContentHash,
}

impl ProjectApprovalRouteSourceCommitment {
    /// Derives the complete source commitment and cross-checks it against the route.
    ///
    /// # Errors
    ///
    /// Fails closed for a decided or invalid approval, missing resolved-context binding,
    /// mismatched route identity, inactive project binding, or inconsistent escalation proof.
    pub fn new(
        route: &ProjectApprovalRoute,
        input: &ProjectApprovalRouteSourceCommitmentInput<'_>,
    ) -> Result<Self, WorkflowOsError> {
        validate_source_input(route, input)?;
        let approval = input.approval;
        let resolved_execution_context_hash = approval
            .resolved_execution_context_hash
            .as_ref()
            .ok_or_else(source_mismatch_error)?;
        let mut hasher = domain_hasher(SOURCE_COMMITMENT_DOMAIN);
        hash_text(&mut hasher, "route_id", route.route_id().as_str());
        hash_text(
            &mut hasher,
            "schema_version",
            approval.schema_version.as_str(),
        );
        hash_text(&mut hasher, "workflow_id", approval.workflow_id.as_str());
        hash_text(
            &mut hasher,
            "workflow_version",
            approval.workflow_version.as_str(),
        );
        hash_text(
            &mut hasher,
            "spec_content_hash",
            approval.spec_content_hash.as_str(),
        );
        hash_text(
            &mut hasher,
            "resolved_execution_context_hash",
            resolved_execution_context_hash.as_str(),
        );
        hash_text(&mut hasher, "run_id", approval.run_id.as_str());
        hash_text(&mut hasher, "approval_id", &approval.approval_id);
        hash_approval_subject(&mut hasher, approval)?;
        hash_text(
            &mut hasher,
            "approval_request_event_id",
            input.approval_request_event_id.as_str(),
        );
        hash_text(
            &mut hasher,
            "immutable_run_bundle_id",
            input.immutable_run_bundle.bundle_id().as_str(),
        );
        hash_text(
            &mut hasher,
            "immutable_run_bundle_version",
            input.immutable_run_bundle.bundle_version().as_str(),
        );
        hash_text(
            &mut hasher,
            "immutable_run_bundle_root_hash",
            input.immutable_run_bundle.root_hash().as_str(),
        );
        hash_run_binding(&mut hasher, input.run_binding);
        hash_optional_text(&mut hasher, "escalation_id", route.escalation_id());
        hash_optional_text(
            &mut hasher,
            "escalation_event_id",
            input.escalation_event_id.map(EventId::as_str),
        );
        hash_text(
            &mut hasher,
            "routing_reason",
            routing_reason_label(route.routing_reason()),
        );
        hash_text(
            &mut hasher,
            "authority_registry_revision",
            &input.authority_snapshot.revision().get().to_string(),
        );
        hash_text(
            &mut hasher,
            "authority_snapshot_fingerprint",
            input.authority_snapshot.fingerprint().as_str(),
        );
        Ok(Self {
            algorithm: ProjectApprovalRouteSourceCommitmentAlgorithm::V1,
            route_id: route.route_id().clone(),
            authority_snapshot: input.authority_snapshot.clone(),
            fingerprint: SpecContentHash::from_bytes(hasher.finalize()),
        })
    }

    /// Returns the source commitment algorithm.
    #[must_use]
    pub const fn algorithm(&self) -> ProjectApprovalRouteSourceCommitmentAlgorithm {
        self.algorithm
    }

    /// Returns the exact route decision identity committed by the sources.
    #[must_use]
    pub const fn route_id(&self) -> &ProjectApprovalRouteId {
        &self.route_id
    }

    /// Returns the exact deployment authority registry revision used for routing.
    #[must_use]
    pub const fn authority_registry_revision(&self) -> HostedAuthorityRegistryRevision {
        self.authority_snapshot.revision()
    }

    /// Returns the complete revision-bound deployment authority snapshot commitment.
    #[must_use]
    pub const fn authority_snapshot(&self) -> &ProjectApprovalAuthoritySnapshotCommitment {
        &self.authority_snapshot
    }

    /// Returns the revision-bound authority snapshot identity used for routing.
    #[must_use]
    pub const fn authority_snapshot_fingerprint(&self) -> &SpecContentHash {
        self.authority_snapshot.fingerprint()
    }

    /// Returns the complete source fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> &SpecContentHash {
        &self.fingerprint
    }
}

impl fmt::Debug for ProjectApprovalRouteSourceCommitment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProjectApprovalRouteSourceCommitment")
            .field("algorithm", &self.algorithm)
            .field("route_id", &"[REDACTED]")
            .field("authority_snapshot", &self.authority_snapshot)
            .field("fingerprint", &"[REDACTED]")
            .finish()
    }
}

/// Immutable payload-free route record suitable for a future hosted store.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ProjectApprovalRouteRecord {
    record_version: ProjectApprovalRouteRecordVersion,
    route: ProjectApprovalRoute,
    logical_subject_id: ProjectApprovalRouteLogicalSubjectId,
    source_commitment: ProjectApprovalRouteSourceCommitment,
    created_at: Timestamp,
}

#[derive(Deserialize)]
struct ProjectApprovalRouteRecordWire {
    record_version: ProjectApprovalRouteRecordVersion,
    route: ProjectApprovalRoute,
    logical_subject_id: ProjectApprovalRouteLogicalSubjectId,
    source_commitment: ProjectApprovalRouteSourceCommitment,
    created_at: Timestamp,
}

impl ProjectApprovalRouteRecord {
    /// Creates one validated immutable route record.
    ///
    /// # Errors
    ///
    /// Fails closed when the derived logical subject cannot be constructed.
    pub fn new(
        route: ProjectApprovalRoute,
        source_commitment: ProjectApprovalRouteSourceCommitment,
        created_at: Timestamp,
    ) -> Result<Self, WorkflowOsError> {
        let logical_subject_id = logical_subject_id(&route)?;
        let record = Self {
            record_version: ProjectApprovalRouteRecordVersion::V1,
            route,
            logical_subject_id,
            source_commitment,
            created_at,
        };
        record.validate()?;
        Ok(record)
    }

    /// Returns the durable record version.
    #[must_use]
    pub const fn record_version(&self) -> ProjectApprovalRouteRecordVersion {
        self.record_version
    }

    /// Returns the immutable routing decision.
    #[must_use]
    pub const fn route(&self) -> &ProjectApprovalRoute {
        &self.route
    }

    /// Returns the route slot identity independent of route outcome.
    #[must_use]
    pub const fn logical_subject_id(&self) -> &ProjectApprovalRouteLogicalSubjectId {
        &self.logical_subject_id
    }

    /// Returns the complete authenticated-source commitment.
    #[must_use]
    pub const fn source_commitment(&self) -> &ProjectApprovalRouteSourceCommitment {
        &self.source_commitment
    }

    /// Returns the first successful creation timestamp.
    #[must_use]
    pub const fn created_at(&self) -> Timestamp {
        self.created_at
    }

    /// Returns whether a retry is decision-equivalent to this canonical record.
    ///
    /// `resolved_at` and `created_at` are intentionally excluded. The first stored values
    /// remain canonical when route identity and complete source commitment match.
    #[must_use]
    pub fn is_decision_equivalent(&self, candidate: &Self) -> bool {
        self.logical_subject_id == candidate.logical_subject_id
            && self.route.route_id() == candidate.route.route_id()
            && self.source_commitment == candidate.source_commitment
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.record_version != ProjectApprovalRouteRecordVersion::V1
            || self.logical_subject_id != logical_subject_id(&self.route)?
            || self.source_commitment.route_id() != self.route.route_id()
            || self
                .source_commitment
                .authority_snapshot()
                .authority_view()
                .organization_id()
                != self.route.scope().organization_id()
        {
            return Err(persistence_error(
                "project_approval_route_store.record.invalid",
                "project approval route record is invalid",
            ));
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for ProjectApprovalRouteRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ProjectApprovalRouteRecordWire::deserialize(deserializer)
            .map_err(|_| serde::de::Error::custom("invalid project approval route record"))?;
        let record = Self {
            record_version: wire.record_version,
            route: wire.route,
            logical_subject_id: wire.logical_subject_id,
            source_commitment: wire.source_commitment,
            created_at: wire.created_at,
        };
        record
            .validate()
            .map_err(|_| serde::de::Error::custom("invalid project approval route record"))?;
        Ok(record)
    }
}

impl fmt::Debug for ProjectApprovalRouteRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProjectApprovalRouteRecord")
            .field("record_version", &self.record_version)
            .field("route", &"[REDACTED]")
            .field("logical_subject_id", &"[REDACTED]")
            .field("source_commitment", &self.source_commitment)
            .field("created_at", &"[REDACTED]")
            .finish()
    }
}

/// Outcome of one create-only route record write.
#[derive(Clone, Eq, PartialEq)]
pub enum ProjectApprovalRouteCreateResult {
    /// The record was created for the first time.
    Created(ProjectApprovalRouteRecord),
    /// An exact decision-equivalent retry returned the first canonical record.
    ReconciledExisting(ProjectApprovalRouteRecord),
}

impl ProjectApprovalRouteCreateResult {
    /// Returns the canonical stored record.
    #[must_use]
    pub const fn record(&self) -> &ProjectApprovalRouteRecord {
        match self {
            Self::Created(record) | Self::ReconciledExisting(record) => record,
        }
    }

    /// Returns whether this operation created the record.
    #[must_use]
    pub const fn was_created(&self) -> bool {
        matches!(self, Self::Created(_))
    }
}

impl fmt::Debug for ProjectApprovalRouteCreateResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProjectApprovalRouteCreateResult")
            .field(
                "posture",
                &if self.was_created() {
                    "created"
                } else {
                    "reconciled_existing"
                },
            )
            .field("record", &"[REDACTED]")
            .finish()
    }
}

/// Create-only, exact-scope storage primitive for project approval routes.
///
/// This trait is not an authentication boundary. A future hosted composer must reconstruct
/// and authenticate route sources before constructing the record passed to this store.
pub trait ProjectApprovalRouteStore: Send + Sync {
    /// Creates or reconciles one immutable record.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking conflict when one logical subject already has different
    /// route content or source provenance.
    fn create_project_approval_route(
        &self,
        record: ProjectApprovalRouteRecord,
    ) -> Result<ProjectApprovalRouteCreateResult, WorkflowOsError>;

    /// Reads one exact logical route subject.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the store is unavailable or stored data is
    /// invalid.
    fn read_project_approval_route(
        &self,
        logical_subject_id: &ProjectApprovalRouteLogicalSubjectId,
    ) -> Result<Option<ProjectApprovalRouteRecord>, WorkflowOsError>;

    /// Lists routed records for one exact project and exact recipient.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error for an invalid bound, invalid scope, unavailable
    /// store, or invalid stored record.
    fn list_project_approval_routes_for_recipient(
        &self,
        scope: &HostedProjectScope,
        recipient: &ActorId,
        limit: usize,
    ) -> Result<Vec<ProjectApprovalRouteRecord>, WorkflowOsError>;

    /// Lists records for one exact project, run, and approval reference.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error for an invalid bound, subject, or scope, an
    /// unavailable store, or an invalid stored record.
    fn list_project_approval_routes_for_approval(
        &self,
        scope: &HostedProjectScope,
        run_id: &WorkflowRunId,
        approval_id: &ApprovalReferenceId,
        limit: usize,
    ) -> Result<Vec<ProjectApprovalRouteRecord>, WorkflowOsError>;
}

/// In-memory contract fixture for route-store conformance and model integration tests.
///
/// This fixture is not durable and is not a production hosted backend.
#[derive(Clone, Default)]
pub struct InMemoryProjectApprovalRouteStoreFixture {
    records: Arc<Mutex<BTreeMap<ProjectApprovalRouteLogicalSubjectId, ProjectApprovalRouteRecord>>>,
}

impl fmt::Debug for InMemoryProjectApprovalRouteStoreFixture {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InMemoryProjectApprovalRouteStoreFixture")
            .field("records", &"[REDACTED]")
            .finish()
    }
}

impl ProjectApprovalRouteStore for InMemoryProjectApprovalRouteStoreFixture {
    fn create_project_approval_route(
        &self,
        record: ProjectApprovalRouteRecord,
    ) -> Result<ProjectApprovalRouteCreateResult, WorkflowOsError> {
        record.validate()?;
        let mut records = self.records.lock().map_err(|_| store_unavailable_error())?;
        if let Some(existing) = records.get(record.logical_subject_id()) {
            if existing.is_decision_equivalent(&record) {
                return Ok(ProjectApprovalRouteCreateResult::ReconciledExisting(
                    existing.clone(),
                ));
            }
            return Err(persistence_error(
                "project_approval_route_store.create.conflict",
                "project approval route record conflicts with existing state",
            ));
        }
        records.insert(record.logical_subject_id().clone(), record.clone());
        Ok(ProjectApprovalRouteCreateResult::Created(record))
    }

    fn read_project_approval_route(
        &self,
        logical_subject_id: &ProjectApprovalRouteLogicalSubjectId,
    ) -> Result<Option<ProjectApprovalRouteRecord>, WorkflowOsError> {
        let records = self.records.lock().map_err(|_| store_unavailable_error())?;
        Ok(records.get(logical_subject_id).cloned())
    }

    fn list_project_approval_routes_for_recipient(
        &self,
        scope: &HostedProjectScope,
        recipient: &ActorId,
        limit: usize,
    ) -> Result<Vec<ProjectApprovalRouteRecord>, WorkflowOsError> {
        validate_limit(limit)?;
        scope.validate()?;
        let records = self.records.lock().map_err(|_| store_unavailable_error())?;
        Ok(records
            .values()
            .filter(|record| {
                record.route().scope() == scope
                    && record.route().status() == ProjectApprovalRouteStatus::Routed
                    && record.route().recipient() == Some(recipient)
            })
            .take(limit)
            .cloned()
            .collect())
    }

    fn list_project_approval_routes_for_approval(
        &self,
        scope: &HostedProjectScope,
        run_id: &WorkflowRunId,
        approval_id: &ApprovalReferenceId,
        limit: usize,
    ) -> Result<Vec<ProjectApprovalRouteRecord>, WorkflowOsError> {
        validate_limit(limit)?;
        scope.validate()?;
        let records = self.records.lock().map_err(|_| store_unavailable_error())?;
        Ok(records
            .values()
            .filter(|record| {
                record.route().scope() == scope
                    && record.route().run_id() == run_id
                    && record.route().approval_id() == approval_id.as_str()
            })
            .take(limit)
            .cloned()
            .collect())
    }
}

fn logical_subject_id(
    route: &ProjectApprovalRoute,
) -> Result<ProjectApprovalRouteLogicalSubjectId, WorkflowOsError> {
    let mut hasher = domain_hasher(LOGICAL_SUBJECT_DOMAIN);
    hash_text(
        &mut hasher,
        "organization_id",
        route.scope().organization_id().as_str(),
    );
    hash_text(
        &mut hasher,
        "project_id",
        route.scope().project_id().as_str(),
    );
    hash_text(&mut hasher, "run_id", route.run_id().as_str());
    hash_text(&mut hasher, "approval_id", route.approval_id());
    hash_text(
        &mut hasher,
        "routing_reason",
        routing_reason_label(route.routing_reason()),
    );
    hash_optional_text(&mut hasher, "escalation_id", route.escalation_id());
    ProjectApprovalRouteLogicalSubjectId::new(format!(
        "{LOGICAL_SUBJECT_PREFIX}{}",
        hex_lower(&hasher.finalize())
    ))
}

fn validate_source_input(
    route: &ProjectApprovalRoute,
    input: &ProjectApprovalRouteSourceCommitmentInput<'_>,
) -> Result<(), WorkflowOsError> {
    let approval = input.approval;
    let manifest = input.immutable_run_bundle;
    approval
        .validate_subject()
        .map_err(|_| source_mismatch_error())?;
    if approval.decision.is_some()
        || approval.resolved_execution_context_hash.is_none()
        || &approval.run_id != route.run_id()
        || approval.approval_id != route.approval_id()
        || &approval.workflow_id != route.workflow_id()
        || input.run_binding.scope() != route.scope()
        || input.run_binding.resource_kind() != HostedProjectResourceKind::Run
        || input.run_binding.resource_id() != route.run_id().as_str()
        || input.run_binding.status() != HostedProjectResourceBindingStatus::Active
        || manifest.run_id() != &approval.run_id
        || manifest.workflow_id() != &approval.workflow_id
        || manifest.workflow_version() != &approval.workflow_version
        || manifest.schema_version() != &approval.schema_version
        || manifest.workflow_content_hash() != &approval.spec_content_hash
        || approval.resolved_execution_context_hash.as_ref()
            != Some(manifest.resolved_execution_context_hash())
        || input.authority_snapshot.authority_view().organization_id()
            != route.scope().organization_id()
    {
        return Err(source_mismatch_error());
    }
    match (
        route.routing_reason(),
        route.escalation_id(),
        input.escalation_event_id,
    ) {
        (ProjectApprovalRoutingReason::WorkflowMaintainer, None, None)
        | (ProjectApprovalRoutingReason::WorkflowEscalationContact, Some(_), Some(_)) => Ok(()),
        _ => Err(source_mismatch_error()),
    }
}

fn hash_approval_subject(
    hasher: &mut Sha256,
    approval: &ApprovalRequest,
) -> Result<(), WorkflowOsError> {
    if let Some(binding) = approval.governance_approval_binding.as_ref() {
        hash_text(hasher, "approval_subject_kind", "aggregate_governance");
        let canonical = serde_json::to_vec(binding).map_err(|_| {
            persistence_error(
                "project_approval_route_store.source_commitment.invalid",
                "project approval route source commitment is invalid",
            )
        })?;
        hash_bytes(hasher, "approval_subject", &canonical);
    } else {
        hash_text(hasher, "approval_subject_kind", "step_skill");
        hash_text(
            hasher,
            "step_id",
            approval
                .step_id
                .as_ref()
                .ok_or_else(source_mismatch_error)?
                .as_str(),
        );
        hash_text(
            hasher,
            "skill_id",
            approval
                .skill_id
                .as_ref()
                .ok_or_else(source_mismatch_error)?
                .as_str(),
        );
        hash_text(
            hasher,
            "skill_version",
            approval
                .skill_version
                .as_ref()
                .ok_or_else(source_mismatch_error)?
                .as_str(),
        );
        hash_optional_text(
            hasher,
            "idempotency_key",
            approval
                .idempotency_key
                .as_ref()
                .map(IdempotencyKey::as_str),
        );
    }
    Ok(())
}

fn hash_run_binding(hasher: &mut Sha256, binding: &HostedProjectResourceBinding) {
    hash_text(
        hasher,
        "binding_organization_id",
        binding.scope().organization_id().as_str(),
    );
    hash_text(
        hasher,
        "binding_project_id",
        binding.scope().project_id().as_str(),
    );
    hash_text(
        hasher,
        "binding_resource_kind",
        binding.resource_kind().storage_key(),
    );
    hash_text(hasher, "binding_resource_id", binding.resource_id());
    hash_text(
        hasher,
        "binding_status",
        match binding.status() {
            HostedProjectResourceBindingStatus::Reserved => "reserved",
            HostedProjectResourceBindingStatus::Active => "active",
        },
    );
    hash_text(hasher, "binding_bound_at", &binding.bound_at().to_rfc3339());
}

fn domain_hasher(domain: &str) -> Sha256 {
    let mut hasher = Sha256::new();
    hash_text(&mut hasher, "domain", domain);
    hasher
}

fn hash_text(hasher: &mut Sha256, label: &str, value: &str) {
    hash_bytes(hasher, label, value.as_bytes());
}

fn hash_bytes(hasher: &mut Sha256, label: &str, value: &[u8]) {
    hasher.update(label.len().to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(value.len().to_be_bytes());
    hasher.update(value);
}

fn hash_usize(hasher: &mut Sha256, label: &str, value: usize) {
    hash_text(hasher, label, &value.to_string());
}

fn hash_optional_text(hasher: &mut Sha256, label: &str, value: Option<&str>) {
    match value {
        Some(value) => {
            hash_text(hasher, &format!("{label}_posture"), "present");
            hash_text(hasher, label, value);
        }
        None => hash_text(hasher, &format!("{label}_posture"), "absent"),
    }
}

fn routing_reason_label(reason: ProjectApprovalRoutingReason) -> &'static str {
    match reason {
        ProjectApprovalRoutingReason::WorkflowMaintainer => "workflow_maintainer",
        ProjectApprovalRoutingReason::WorkflowEscalationContact => "workflow_escalation_contact",
    }
}

fn principal_kind_label(kind: HostedPrincipalKind) -> &'static str {
    match kind {
        HostedPrincipalKind::Human => "human",
        HostedPrincipalKind::Service => "service",
    }
}

fn capability_label(capability: HostedProjectCapability) -> &'static str {
    match capability {
        HostedProjectCapability::CatalogRead => "catalog_read",
        HostedProjectCapability::CatalogPublishVersion => "catalog_publish_version",
        HostedProjectCapability::RunCreate => "run_create",
        HostedProjectCapability::RunRead => "run_read",
        HostedProjectCapability::ApprovalRead => "approval_read",
        HostedProjectCapability::ApprovalDecide => "approval_decide",
        HostedProjectCapability::RunCancel => "run_cancel",
        HostedProjectCapability::ReportRead => "report_read",
    }
}

fn authority_algorithm_label(
    algorithm: ProjectApprovalAuthorityViewCommitmentAlgorithm,
) -> &'static str {
    match algorithm {
        ProjectApprovalAuthorityViewCommitmentAlgorithm::V1 => AUTHORITY_COMMITMENT_DOMAIN,
    }
}

fn validate_limit(limit: usize) -> Result<(), WorkflowOsError> {
    if limit == 0 || limit > MAX_LIST_LIMIT {
        return Err(persistence_error(
            "project_approval_route_store.list.limit.invalid",
            "project approval route list limit is invalid",
        ));
    }
    Ok(())
}

fn source_mismatch_error() -> WorkflowOsError {
    persistence_error(
        "project_approval_route_store.source.mismatch",
        "project approval route source does not match the route",
    )
}

fn store_unavailable_error() -> WorkflowOsError {
    persistence_error(
        "project_approval_route_store.unavailable",
        "project approval route store is unavailable",
    )
}

fn persistence_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::invalid_state(code, message)
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}
