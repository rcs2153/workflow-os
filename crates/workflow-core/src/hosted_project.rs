use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    ActorId, CorrelationId, EventId, OrganizationId, ProjectId, Timestamp, WorkflowCatalogRecord,
    WorkflowId, WorkflowOsError, WorkflowStewardshipDecisionId, WorkflowVersion,
};

const MAX_GRANTS: usize = 128;
const MAX_CAPABILITIES: usize = 32;
const MAX_PRINCIPALS: usize = 1_024;

/// Exact organization and project boundary for one collaborative hosted resource.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "HostedProjectScopeWire")]
pub struct HostedProjectScope {
    organization_id: OrganizationId,
    project_id: ProjectId,
}

impl fmt::Debug for HostedProjectScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedProjectScope")
            .field("organization", &"[REDACTED]")
            .field("project", &"[REDACTED]")
            .finish()
    }
}

#[derive(Deserialize)]
struct HostedProjectScopeWire {
    organization_id: OrganizationId,
    project_id: ProjectId,
}

impl HostedProjectScope {
    /// Creates one exact hosted project scope.
    #[must_use]
    pub const fn new(organization_id: OrganizationId, project_id: ProjectId) -> Self {
        Self {
            organization_id,
            project_id,
        }
    }

    /// Returns the organization identity.
    #[must_use]
    pub const fn organization_id(&self) -> &OrganizationId {
        &self.organization_id
    }

    /// Returns the project identity.
    #[must_use]
    pub const fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    /// Validates the scope.
    ///
    /// # Errors
    ///
    /// Rejects invalid organization or project identities.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        OrganizationId::new(self.organization_id.as_str())?;
        ProjectId::new(self.project_id.as_str())?;
        Ok(())
    }
}

impl TryFrom<HostedProjectScopeWire> for HostedProjectScope {
    type Error = WorkflowOsError;

    fn try_from(value: HostedProjectScopeWire) -> Result<Self, Self::Error> {
        let scope = Self::new(value.organization_id, value.project_id);
        scope.validate()?;
        Ok(scope)
    }
}

/// Closed collaborative hosted capability vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostedProjectCapability {
    /// Read immutable workflow catalog records within the project.
    CatalogRead,
    /// Publish one immutable workflow catalog version within the project.
    CatalogPublishVersion,
    /// Create a governed run within the project.
    RunCreate,
    /// Read a governed run and its bounded execution records.
    RunRead,
    /// Read approval requests for a project-bound run.
    ApprovalRead,
    /// Decide approval requests for a project-bound run.
    ApprovalDecide,
    /// Cancel a project-bound run.
    RunCancel,
    /// Read report metadata for a project-bound run.
    ReportRead,
}

/// Explicit capabilities granted for exactly one project.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct HostedProjectGrant {
    project_id: ProjectId,
    capabilities: Vec<HostedProjectCapability>,
}

impl fmt::Debug for HostedProjectGrant {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedProjectGrant")
            .field("project", &"[REDACTED]")
            .field("capabilities", &self.capabilities)
            .finish_non_exhaustive()
    }
}

#[derive(Deserialize)]
struct HostedProjectGrantWire {
    project_id: ProjectId,
    capabilities: Vec<HostedProjectCapability>,
}

impl HostedProjectGrant {
    /// Creates one validated project grant.
    ///
    /// # Errors
    ///
    /// Rejects empty, oversized, or duplicate capability sets.
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(
        project_id: ProjectId,
        capabilities: Vec<HostedProjectCapability>,
    ) -> Result<Self, WorkflowOsError> {
        if capabilities.is_empty() || capabilities.len() > MAX_CAPABILITIES {
            return Err(project_boundary_error(
                "hosted_project.grant.capabilities.invalid",
                "hosted project grant capabilities are invalid",
            ));
        }
        let unique = capabilities.iter().copied().collect::<BTreeSet<_>>();
        if unique.len() != capabilities.len() {
            return Err(project_boundary_error(
                "hosted_project.grant.capabilities.duplicate",
                "hosted project grant contains duplicate capabilities",
            ));
        }
        Ok(Self {
            project_id,
            capabilities: unique.into_iter().collect(),
        })
    }

    /// Returns the exactly granted project.
    #[must_use]
    pub const fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    /// Returns the sorted closed capability set.
    #[must_use]
    pub fn capabilities(&self) -> &[HostedProjectCapability] {
        &self.capabilities
    }

    /// Returns whether the grant contains the capability.
    #[must_use]
    pub fn allows(&self, capability: HostedProjectCapability) -> bool {
        self.capabilities.binary_search(&capability).is_ok()
    }
}

impl<'de> Deserialize<'de> for HostedProjectGrant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let wire = HostedProjectGrantWire::deserialize(deserializer)
            .map_err(|_| serde::de::Error::custom("invalid hosted project grant"))?;
        Self::new(wire.project_id, wire.capabilities)
            .map_err(|_| serde::de::Error::custom("invalid hosted project grant"))
    }
}

/// Bounded principal kind used only for audit posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostedPrincipalKind {
    /// A pre-provisioned human operator.
    Human,
    /// A pre-provisioned service actor.
    Service,
}

/// Deployment-owned principal binding for one organization.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct HostedPrincipalBinding {
    actor_id: ActorId,
    organization_id: OrganizationId,
    principal_kind: HostedPrincipalKind,
    grants: Vec<HostedProjectGrant>,
}

#[derive(Deserialize)]
struct HostedPrincipalBindingWire {
    actor_id: ActorId,
    organization_id: OrganizationId,
    principal_kind: HostedPrincipalKind,
    grants: Vec<HostedProjectGrant>,
}

impl HostedPrincipalBinding {
    /// Creates one validated deployment principal binding.
    ///
    /// # Errors
    ///
    /// Rejects empty, oversized, or duplicate project grants.
    pub fn new(
        actor_id: ActorId,
        organization_id: OrganizationId,
        principal_kind: HostedPrincipalKind,
        mut grants: Vec<HostedProjectGrant>,
    ) -> Result<Self, WorkflowOsError> {
        if grants.is_empty() || grants.len() > MAX_GRANTS {
            return Err(project_boundary_error(
                "hosted_project.principal.grants.invalid",
                "hosted principal grants are invalid",
            ));
        }
        grants.sort_by(|left, right| left.project_id.cmp(&right.project_id));
        if grants
            .windows(2)
            .any(|pair| pair[0].project_id == pair[1].project_id)
        {
            return Err(project_boundary_error(
                "hosted_project.principal.grants.duplicate",
                "hosted principal contains duplicate project grants",
            ));
        }
        Ok(Self {
            actor_id,
            organization_id,
            principal_kind,
            grants,
        })
    }

    /// Returns the stable actor identity.
    #[must_use]
    pub const fn actor_id(&self) -> &ActorId {
        &self.actor_id
    }

    /// Returns the deployment organization identity.
    #[must_use]
    pub const fn organization_id(&self) -> &OrganizationId {
        &self.organization_id
    }

    /// Returns the bounded principal kind.
    #[must_use]
    pub const fn principal_kind(&self) -> HostedPrincipalKind {
        self.principal_kind
    }

    /// Returns the sorted project grants.
    #[must_use]
    pub fn grants(&self) -> &[HostedProjectGrant] {
        &self.grants
    }

    /// Returns whether the principal has the capability for the exact project.
    #[must_use]
    pub fn allows(&self, project_id: &ProjectId, capability: HostedProjectCapability) -> bool {
        self.grants
            .binary_search_by(|grant| grant.project_id.cmp(project_id))
            .ok()
            .is_some_and(|index| self.grants[index].allows(capability))
    }
}

impl fmt::Debug for HostedPrincipalBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedPrincipalBinding")
            .field("actor", &"[REDACTED]")
            .field("organization", &"[REDACTED]")
            .field("principal_kind", &self.principal_kind)
            .field("grant_count", &self.grants.len())
            .finish_non_exhaustive()
    }
}

impl<'de> Deserialize<'de> for HostedPrincipalBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let wire = HostedPrincipalBindingWire::deserialize(deserializer)
            .map_err(|_| serde::de::Error::custom("invalid hosted principal binding"))?;
        Self::new(
            wire.actor_id,
            wire.organization_id,
            wire.principal_kind,
            wire.grants,
        )
        .map_err(|_| serde::de::Error::custom("invalid hosted principal binding"))
    }
}

/// Immutable deployment-owned authority registry for one organization.
///
/// The closed registry is the minimum type that can truthfully represent a complete
/// authority view. A caller-provided principal slice is not equivalent to this boundary.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct HostedPrincipalRegistry {
    organization_id: OrganizationId,
    principals: Vec<HostedPrincipalBinding>,
}

#[derive(Deserialize)]
struct HostedPrincipalRegistryWire {
    organization_id: OrganizationId,
    principals: Vec<HostedPrincipalBinding>,
}

impl HostedPrincipalRegistry {
    /// Creates one complete immutable organization authority registry.
    ///
    /// # Errors
    ///
    /// Rejects oversized registries, duplicate actor bindings, and principals outside the
    /// registry organization. An empty registry is a complete view with no principals.
    pub fn new(
        organization_id: OrganizationId,
        mut principals: Vec<HostedPrincipalBinding>,
    ) -> Result<Self, WorkflowOsError> {
        if principals.len() > MAX_PRINCIPALS
            || principals
                .iter()
                .any(|principal| principal.organization_id() != &organization_id)
        {
            return Err(project_boundary_error(
                "hosted_project.principal_registry.invalid",
                "hosted principal registry is invalid",
            ));
        }
        principals.sort_by(|left, right| left.actor_id().as_str().cmp(right.actor_id().as_str()));
        if principals
            .windows(2)
            .any(|pair| pair[0].actor_id() == pair[1].actor_id())
        {
            return Err(project_boundary_error(
                "hosted_project.principal_registry.duplicate",
                "hosted principal registry contains duplicate actors",
            ));
        }
        Ok(Self {
            organization_id,
            principals,
        })
    }

    /// Returns the exact organization governed by this complete view.
    #[must_use]
    pub const fn organization_id(&self) -> &OrganizationId {
        &self.organization_id
    }

    /// Returns the complete deterministically ordered principal view.
    #[must_use]
    pub fn principals(&self) -> &[HostedPrincipalBinding] {
        &self.principals
    }
}

impl fmt::Debug for HostedPrincipalRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedPrincipalRegistry")
            .field("organization", &"[REDACTED]")
            .field("principal_count", &self.principals.len())
            .finish_non_exhaustive()
    }
}

impl<'de> Deserialize<'de> for HostedPrincipalRegistry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let wire = HostedPrincipalRegistryWire::deserialize(deserializer)
            .map_err(|_| serde::de::Error::custom("invalid hosted principal registry"))?;
        Self::new(wire.organization_id, wire.principals)
            .map_err(|_| serde::de::Error::custom("invalid hosted principal registry"))
    }
}

/// Durable resource families protected by a hosted project scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostedProjectResourceKind {
    /// Governed workflow run identity.
    Run,
    /// Durable hosted execution work item.
    WorkItem,
    /// Payload-free hosted execution receipt.
    ExecutionReceipt,
    /// Terminal work report artifact metadata.
    Report,
    /// Immutable project workflow catalog record.
    CatalogRecord,
}

impl HostedProjectResourceKind {
    /// Returns the stable storage discriminator.
    #[must_use]
    pub const fn storage_key(self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::WorkItem => "work_item",
            Self::ExecutionReceipt => "execution_receipt",
            Self::Report => "report",
            Self::CatalogRecord => "catalog_record",
        }
    }
}

/// Reservation posture for a project-bound resource.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostedProjectResourceBindingStatus {
    /// Identity is claimed but is not externally readable yet.
    Reserved,
    /// Identity is committed and may be exposed within its exact scope.
    Active,
}

/// Payload-free durable commitment between a resource and one exact project.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "HostedProjectResourceBindingWire")]
pub struct HostedProjectResourceBinding {
    scope: HostedProjectScope,
    resource_kind: HostedProjectResourceKind,
    resource_id: String,
    status: HostedProjectResourceBindingStatus,
    bound_at: Timestamp,
}

#[derive(Deserialize)]
struct HostedProjectResourceBindingWire {
    scope: HostedProjectScope,
    resource_kind: HostedProjectResourceKind,
    resource_id: String,
    status: HostedProjectResourceBindingStatus,
    bound_at: Timestamp,
}

impl HostedProjectResourceBinding {
    /// Creates a validated payload-free resource binding.
    ///
    /// # Errors
    ///
    /// Rejects invalid scope or resource references.
    pub fn new(
        scope: HostedProjectScope,
        resource_kind: HostedProjectResourceKind,
        resource_id: impl Into<String>,
        status: HostedProjectResourceBindingStatus,
        bound_at: Timestamp,
    ) -> Result<Self, WorkflowOsError> {
        let resource_id = resource_id.into();
        validate_reference(&resource_id)?;
        scope.validate()?;
        Ok(Self {
            scope,
            resource_kind,
            resource_id,
            status,
            bound_at,
        })
    }

    /// Returns the exact organization and project scope.
    #[must_use]
    pub const fn scope(&self) -> &HostedProjectScope {
        &self.scope
    }

    /// Returns the durable resource family.
    #[must_use]
    pub const fn resource_kind(&self) -> HostedProjectResourceKind {
        self.resource_kind
    }

    /// Returns the bounded resource identity.
    #[must_use]
    pub fn resource_id(&self) -> &str {
        &self.resource_id
    }

    /// Returns reservation posture.
    #[must_use]
    pub const fn status(&self) -> HostedProjectResourceBindingStatus {
        self.status
    }

    /// Returns when the resource was first bound to the project.
    #[must_use]
    pub const fn bound_at(&self) -> Timestamp {
        self.bound_at
    }

    /// Returns an active copy without changing the original binding time.
    ///
    /// # Errors
    ///
    /// Fails closed if the persisted binding cannot be reconstructed.
    pub fn activate(&self) -> Result<Self, WorkflowOsError> {
        if self.status == HostedProjectResourceBindingStatus::Active {
            return Ok(self.clone());
        }
        Self::new(
            self.scope.clone(),
            self.resource_kind,
            self.resource_id.clone(),
            HostedProjectResourceBindingStatus::Active,
            self.bound_at,
        )
    }
}

impl fmt::Debug for HostedProjectResourceBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedProjectResourceBinding")
            .field("scope", &"[REDACTED]")
            .field("resource_kind", &self.resource_kind)
            .field("resource_id", &"[REDACTED]")
            .field("status", &self.status)
            .finish_non_exhaustive()
    }
}

impl TryFrom<HostedProjectResourceBindingWire> for HostedProjectResourceBinding {
    type Error = WorkflowOsError;

    fn try_from(value: HostedProjectResourceBindingWire) -> Result<Self, Self::Error> {
        Self::new(
            value.scope,
            value.resource_kind,
            value.resource_id,
            value.status,
            value.bound_at,
        )
    }
}

/// Stable allowed/denied authorization result suitable for bounded audit projection.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct HostedProjectAccessDecision {
    decision_id: EventId,
    actor_id: ActorId,
    principal_kind: HostedPrincipalKind,
    scope: HostedProjectScope,
    capability: HostedProjectCapability,
    allowed: bool,
    reason_code: String,
    target_kind: HostedProjectResourceKind,
    target_reference: String,
    correlation_id: Option<CorrelationId>,
    decided_at: Timestamp,
}

#[derive(Deserialize)]
struct HostedProjectAccessDecisionWire {
    decision_id: EventId,
    actor_id: ActorId,
    principal_kind: HostedPrincipalKind,
    scope: HostedProjectScope,
    capability: HostedProjectCapability,
    allowed: bool,
    reason_code: String,
    target_kind: HostedProjectResourceKind,
    target_reference: String,
    correlation_id: Option<CorrelationId>,
    decided_at: Timestamp,
}

impl HostedProjectAccessDecision {
    /// Creates one validated payload-free authorization decision.
    ///
    /// # Errors
    ///
    /// Rejects invalid scope, reason, or target reference posture.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        decision_id: EventId,
        actor_id: ActorId,
        principal_kind: HostedPrincipalKind,
        scope: HostedProjectScope,
        capability: HostedProjectCapability,
        allowed: bool,
        reason_code: impl Into<String>,
        target_kind: HostedProjectResourceKind,
        target_reference: impl Into<String>,
        correlation_id: Option<CorrelationId>,
        decided_at: Timestamp,
    ) -> Result<Self, WorkflowOsError> {
        let reason_code = reason_code.into();
        let target_reference = target_reference.into();
        validate_reference(&reason_code)?;
        validate_reference(&target_reference)?;
        scope.validate()?;
        Ok(Self {
            decision_id,
            actor_id,
            principal_kind,
            scope,
            capability,
            allowed,
            reason_code,
            target_kind,
            target_reference,
            correlation_id,
            decided_at,
        })
    }

    /// Returns the stable decision identity.
    #[must_use]
    pub const fn decision_id(&self) -> &EventId {
        &self.decision_id
    }
    /// Returns the exact authorization scope.
    #[must_use]
    pub const fn scope(&self) -> &HostedProjectScope {
        &self.scope
    }

    /// Returns whether the authorization decision allowed the request.
    #[must_use]
    pub const fn allowed(&self) -> bool {
        self.allowed
    }

    /// Returns the evaluated capability.
    #[must_use]
    pub const fn capability(&self) -> HostedProjectCapability {
        self.capability
    }
}

impl fmt::Debug for HostedProjectAccessDecision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedProjectAccessDecision")
            .field("decision", &"[REDACTED]")
            .field("actor", &"[REDACTED]")
            .field("scope", &"[REDACTED]")
            .field("capability", &self.capability)
            .field("allowed", &self.allowed)
            .field("target_kind", &self.target_kind)
            .finish_non_exhaustive()
    }
}

impl<'de> Deserialize<'de> for HostedProjectAccessDecision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let wire = HostedProjectAccessDecisionWire::deserialize(deserializer)
            .map_err(|_| serde::de::Error::custom("invalid hosted project access decision"))?;
        Self::new(
            wire.decision_id,
            wire.actor_id,
            wire.principal_kind,
            wire.scope,
            wire.capability,
            wire.allowed,
            wire.reason_code,
            wire.target_kind,
            wire.target_reference,
            wire.correlation_id,
            wire.decided_at,
        )
        .map_err(|_| serde::de::Error::custom("invalid hosted project access decision"))
    }
}

/// Project-scoped immutable workflow catalog version.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "HostedProjectCatalogVersionWire")]
pub struct HostedProjectCatalogVersion {
    scope: HostedProjectScope,
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    record: WorkflowCatalogRecord,
    published_by: ActorId,
    stewardship_decision_id: WorkflowStewardshipDecisionId,
    published_at: Timestamp,
}

#[derive(Deserialize)]
struct HostedProjectCatalogVersionWire {
    scope: HostedProjectScope,
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    record: WorkflowCatalogRecord,
    published_by: ActorId,
    stewardship_decision_id: WorkflowStewardshipDecisionId,
    published_at: Timestamp,
}

impl HostedProjectCatalogVersion {
    /// Creates one validated immutable project catalog version.
    ///
    /// # Errors
    ///
    /// Rejects missing ownership, escalation, or stewardship linkage and any
    /// mismatch between the record and supplied catalog identity.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        scope: HostedProjectScope,
        workflow_id: WorkflowId,
        workflow_version: WorkflowVersion,
        record: WorkflowCatalogRecord,
        published_by: ActorId,
        stewardship_decision_id: WorkflowStewardshipDecisionId,
        published_at: Timestamp,
    ) -> Result<Self, WorkflowOsError> {
        scope.validate()?;
        record.validate()?;
        if record.workflow_id() != &workflow_id
            || record.owner().is_none()
            || record.escalation_contact().is_none()
            || record.latest_stewardship_decision_id() != Some(&stewardship_decision_id)
        {
            return Err(project_boundary_error(
                "hosted_project.catalog.governance.invalid",
                "hosted project catalog governance metadata is invalid",
            ));
        }
        Ok(Self {
            scope,
            workflow_id,
            workflow_version,
            record,
            published_by,
            stewardship_decision_id,
            published_at,
        })
    }

    /// Returns the exact project scope.
    #[must_use]
    pub const fn scope(&self) -> &HostedProjectScope {
        &self.scope
    }
    /// Returns the workflow identity.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }
    /// Returns the immutable workflow version.
    #[must_use]
    pub const fn workflow_version(&self) -> &WorkflowVersion {
        &self.workflow_version
    }
    /// Returns the governed catalog record.
    #[must_use]
    pub const fn record(&self) -> &WorkflowCatalogRecord {
        &self.record
    }

    /// Returns the actor that published this version.
    #[must_use]
    pub const fn published_by(&self) -> &ActorId {
        &self.published_by
    }

    /// Returns the durable stewardship decision required for publication.
    #[must_use]
    pub const fn stewardship_decision_id(&self) -> &WorkflowStewardshipDecisionId {
        &self.stewardship_decision_id
    }
}

impl fmt::Debug for HostedProjectCatalogVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostedProjectCatalogVersion")
            .field("scope", &"[REDACTED]")
            .field("workflow", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl TryFrom<HostedProjectCatalogVersionWire> for HostedProjectCatalogVersion {
    type Error = WorkflowOsError;
    fn try_from(value: HostedProjectCatalogVersionWire) -> Result<Self, Self::Error> {
        Self::new(
            value.scope,
            value.workflow_id,
            value.workflow_version,
            value.record,
            value.published_by,
            value.stewardship_decision_id,
            value.published_at,
        )
    }
}

fn validate_reference(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() || value.len() > 256 || value.chars().any(char::is_control) {
        return Err(project_boundary_error(
            "hosted_project.resource.reference.invalid",
            "hosted project resource reference is invalid",
        ));
    }
    Ok(())
}

fn project_boundary_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}
