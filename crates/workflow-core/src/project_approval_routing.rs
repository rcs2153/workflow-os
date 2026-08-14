use std::fmt;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    ActorId, ApprovalRequest, EscalationRecord, HostedPrincipalBinding, HostedProjectCapability,
    HostedProjectResourceBinding, HostedProjectResourceBindingStatus, HostedProjectResourceKind,
    HostedProjectScope, OwnershipMetadata, Timestamp, WorkflowId, WorkflowOsError, WorkflowRunId,
};

const ROUTE_ID_PREFIX: &str = "project-approval-route-";
const MAX_PRINCIPALS: usize = 1_024;

/// Stable content-derived identity for one project approval route resolution.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ProjectApprovalRouteId(String);

impl ProjectApprovalRouteId {
    /// Creates a validated project approval route identity.
    ///
    /// # Errors
    ///
    /// Rejects values outside the content-derived route identity shape.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        let digest = value.strip_prefix(ROUTE_ID_PREFIX).ok_or_else(|| {
            route_error(
                "project_approval_route.id.invalid",
                "project approval route identity is invalid",
            )
        })?;
        if digest.len() != 64
            || !digest
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err(route_error(
                "project_approval_route.id.invalid",
                "project approval route identity is invalid",
            ));
        }
        Ok(Self(value))
    }

    /// Returns the stable route identity text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ProjectApprovalRouteId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ProjectApprovalRouteId([REDACTED])")
    }
}

impl fmt::Display for ProjectApprovalRouteId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<String> for ProjectApprovalRouteId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ProjectApprovalRouteId> for String {
    fn from(value: ProjectApprovalRouteId) -> Self {
        value.0
    }
}

/// Immutable workflow metadata field used to select a routing candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectApprovalRoutingReason {
    /// Route an ordinary approval to the configured workflow maintainer.
    WorkflowMaintainer,
    /// Route an escalation-related approval to the configured escalation contact.
    WorkflowEscalationContact,
}

impl ProjectApprovalRoutingReason {
    const fn label(self) -> &'static str {
        match self {
            Self::WorkflowMaintainer => "workflow_maintainer",
            Self::WorkflowEscalationContact => "workflow_escalation_contact",
        }
    }
}

/// Bounded outcome of deterministic project approval routing.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectApprovalRouteStatus {
    /// Candidate metadata intersected one exact-project authority binding.
    Routed,
    /// The selected immutable metadata field was absent.
    UnresolvedMissingMetadata,
    /// The selected actor lacks exact-project approval decision authority.
    UnresolvedAuthorityUnavailable,
}

impl ProjectApprovalRouteStatus {
    const fn label(self) -> &'static str {
        match self {
            Self::Routed => "routed",
            Self::UnresolvedMissingMetadata => "unresolved_missing_metadata",
            Self::UnresolvedAuthorityUnavailable => "unresolved_authority_unavailable",
        }
    }
}

/// Pull-based notification availability without a delivery or observation claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectApprovalNotificationPosture {
    /// A future project inbox may expose the routed reference to the recipient.
    AvailableForProjectInbox,
    /// No inbox entry can be addressed because route resolution was unresolved.
    UnavailableRouteUnresolved,
}

impl ProjectApprovalNotificationPosture {
    const fn label(self) -> &'static str {
        match self {
            Self::AvailableForProjectInbox => "available_for_project_inbox",
            Self::UnavailableRouteUnresolved => "unavailable_route_unresolved",
        }
    }
}

/// Payload-free, project-scoped routing result for one pending approval.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ProjectApprovalRoute {
    route_id: ProjectApprovalRouteId,
    scope: HostedProjectScope,
    run_id: WorkflowRunId,
    approval_id: String,
    workflow_id: WorkflowId,
    routing_reason: ProjectApprovalRoutingReason,
    escalation_id: Option<String>,
    status: ProjectApprovalRouteStatus,
    recipient: Option<ActorId>,
    notification_posture: ProjectApprovalNotificationPosture,
    resolved_at: Timestamp,
}

#[derive(Deserialize)]
struct ProjectApprovalRouteWire {
    route_id: ProjectApprovalRouteId,
    scope: HostedProjectScope,
    run_id: WorkflowRunId,
    approval_id: String,
    workflow_id: WorkflowId,
    routing_reason: ProjectApprovalRoutingReason,
    escalation_id: Option<String>,
    status: ProjectApprovalRouteStatus,
    recipient: Option<ActorId>,
    notification_posture: ProjectApprovalNotificationPosture,
    resolved_at: Timestamp,
}

impl ProjectApprovalRoute {
    /// Returns the stable content-derived route identity.
    #[must_use]
    pub const fn route_id(&self) -> &ProjectApprovalRouteId {
        &self.route_id
    }

    /// Returns the exact hosted project scope.
    #[must_use]
    pub const fn scope(&self) -> &HostedProjectScope {
        &self.scope
    }

    /// Returns the governed run identity.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the stable approval reference.
    #[must_use]
    pub fn approval_id(&self) -> &str {
        &self.approval_id
    }

    /// Returns the immutable workflow identity.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the immutable metadata field used for candidate selection.
    #[must_use]
    pub const fn routing_reason(&self) -> ProjectApprovalRoutingReason {
        self.routing_reason
    }

    /// Returns the stable escalation reference for escalation-contact routing.
    #[must_use]
    pub fn escalation_id(&self) -> Option<&str> {
        self.escalation_id.as_deref()
    }

    /// Returns the bounded route status.
    #[must_use]
    pub const fn status(&self) -> ProjectApprovalRouteStatus {
        self.status
    }

    /// Returns the authorized recipient only for a routed result.
    #[must_use]
    pub const fn recipient(&self) -> Option<&ActorId> {
        self.recipient.as_ref()
    }

    /// Returns pull-based notification availability.
    #[must_use]
    pub const fn notification_posture(&self) -> ProjectApprovalNotificationPosture {
        self.notification_posture
    }

    /// Returns when the immutable authority view was resolved.
    #[must_use]
    pub const fn resolved_at(&self) -> Timestamp {
        self.resolved_at
    }

    #[allow(clippy::too_many_arguments)]
    fn from_parts(
        scope: HostedProjectScope,
        run_id: WorkflowRunId,
        approval_id: String,
        workflow_id: WorkflowId,
        routing_reason: ProjectApprovalRoutingReason,
        escalation_id: Option<String>,
        status: ProjectApprovalRouteStatus,
        recipient: Option<ActorId>,
        notification_posture: ProjectApprovalNotificationPosture,
        resolved_at: Timestamp,
    ) -> Result<Self, WorkflowOsError> {
        validate_approval_reference(&approval_id)?;
        if let Some(escalation_id) = escalation_id.as_deref() {
            validate_escalation_reference(escalation_id)?;
        }
        validate_route_posture(
            routing_reason,
            escalation_id.as_deref(),
            status,
            recipient.as_ref(),
            notification_posture,
        )?;
        let route_id = derive_route_id(
            &scope,
            &run_id,
            &approval_id,
            &workflow_id,
            routing_reason,
            escalation_id.as_deref(),
            status,
            recipient.as_ref(),
            notification_posture,
        )?;
        Ok(Self {
            route_id,
            scope,
            run_id,
            approval_id,
            workflow_id,
            routing_reason,
            escalation_id,
            status,
            recipient,
            notification_posture,
            resolved_at,
        })
    }
}

impl fmt::Debug for ProjectApprovalRoute {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProjectApprovalRoute")
            .field("route_id", &"[REDACTED]")
            .field("scope", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("approval_id", &"[REDACTED]")
            .field("workflow_id", &"[REDACTED]")
            .field("routing_reason", &self.routing_reason)
            .field(
                "escalation_id",
                &self.escalation_id.as_ref().map(|_| "[REDACTED]"),
            )
            .field("status", &self.status)
            .field("recipient", &self.recipient.as_ref().map(|_| "[REDACTED]"))
            .field("notification_posture", &self.notification_posture)
            .field("resolved_at", &self.resolved_at)
            .finish()
    }
}

impl<'de> Deserialize<'de> for ProjectApprovalRoute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let wire = ProjectApprovalRouteWire::deserialize(deserializer)
            .map_err(|_| serde::de::Error::custom("invalid project approval route"))?;
        let route = Self::from_parts(
            wire.scope,
            wire.run_id,
            wire.approval_id,
            wire.workflow_id,
            wire.routing_reason,
            wire.escalation_id,
            wire.status,
            wire.recipient,
            wire.notification_posture,
            wire.resolved_at,
        )
        .map_err(|_| serde::de::Error::custom("invalid project approval route"))?;
        if route.route_id != wire.route_id {
            return Err(serde::de::Error::custom("invalid project approval route"));
        }
        Ok(route)
    }
}

/// Explicit immutable inputs for deterministic project approval routing.
pub struct ProjectApprovalRouteInput<'a> {
    /// Exact project scope for the governed run.
    pub scope: &'a HostedProjectScope,
    /// Active, project-bound run reference.
    pub run_binding: &'a HostedProjectResourceBinding,
    /// Pending approval with a validated subject.
    pub approval: &'a ApprovalRequest,
    /// Immutable workflow ownership metadata captured for the run.
    pub ownership: &'a OwnershipMetadata,
    /// Metadata field to use for candidate selection.
    pub routing_reason: ProjectApprovalRoutingReason,
    /// Exact run-bound escalation subject required for escalation routing.
    pub escalation: Option<&'a EscalationRecord>,
    /// Immutable deployment-owned authority view.
    pub principals: &'a [HostedPrincipalBinding],
    /// Resolution timestamp supplied by the caller.
    pub resolved_at: Timestamp,
}

/// Resolves a project approval recipient without creating or widening authority.
///
/// # Errors
///
/// Fails closed for malformed approval subjects, invalid project/run bindings,
/// decided approvals, oversized authority views, or ambiguous principal state.
pub fn resolve_project_approval_route(
    input: &ProjectApprovalRouteInput<'_>,
) -> Result<ProjectApprovalRoute, WorkflowOsError> {
    validate_input_boundary(input)?;

    let escalation_id = input
        .escalation
        .map(|escalation| escalation.escalation_id.clone());

    let candidate = match input.routing_reason {
        ProjectApprovalRoutingReason::WorkflowMaintainer => input.ownership.maintainer.as_ref(),
        ProjectApprovalRoutingReason::WorkflowEscalationContact => {
            input.ownership.escalation_contact.as_ref()
        }
    };

    let Some(candidate) = candidate else {
        return ProjectApprovalRoute::from_parts(
            input.scope.clone(),
            input.approval.run_id.clone(),
            input.approval.approval_id.clone(),
            input.approval.workflow_id.clone(),
            input.routing_reason,
            escalation_id,
            ProjectApprovalRouteStatus::UnresolvedMissingMetadata,
            None,
            ProjectApprovalNotificationPosture::UnavailableRouteUnresolved,
            input.resolved_at,
        );
    };

    let authorized = input
        .principals
        .iter()
        .filter(|principal| {
            principal.actor_id() == candidate
                && principal.organization_id() == input.scope.organization_id()
                && principal.allows(
                    input.scope.project_id(),
                    HostedProjectCapability::ApprovalDecide,
                )
        })
        .collect::<Vec<_>>();

    if authorized.len() > 1 {
        return Err(route_error(
            "project_approval_route.authority.ambiguous",
            "project approval routing authority is ambiguous",
        ));
    }

    let (status, recipient, notification_posture) = if authorized.is_empty() {
        (
            ProjectApprovalRouteStatus::UnresolvedAuthorityUnavailable,
            None,
            ProjectApprovalNotificationPosture::UnavailableRouteUnresolved,
        )
    } else {
        (
            ProjectApprovalRouteStatus::Routed,
            Some(candidate.clone()),
            ProjectApprovalNotificationPosture::AvailableForProjectInbox,
        )
    };

    ProjectApprovalRoute::from_parts(
        input.scope.clone(),
        input.approval.run_id.clone(),
        input.approval.approval_id.clone(),
        input.approval.workflow_id.clone(),
        input.routing_reason,
        escalation_id,
        status,
        recipient,
        notification_posture,
        input.resolved_at,
    )
}

fn validate_input_boundary(input: &ProjectApprovalRouteInput<'_>) -> Result<(), WorkflowOsError> {
    input.scope.validate()?;
    input.approval.validate_subject()?;
    validate_approval_reference(&input.approval.approval_id)?;
    if input.approval.decision.is_some() {
        return Err(route_error(
            "project_approval_route.approval.not_pending",
            "project approval route requires a pending approval",
        ));
    }
    validate_escalation_subject(input)?;
    if input.principals.len() > MAX_PRINCIPALS {
        return Err(route_error(
            "project_approval_route.authority_view.oversized",
            "project approval routing authority view is oversized",
        ));
    }
    if input.run_binding.scope() != input.scope
        || input.run_binding.resource_kind() != HostedProjectResourceKind::Run
        || input.run_binding.status() != HostedProjectResourceBindingStatus::Active
        || input.run_binding.resource_id() != input.approval.run_id.as_str()
    {
        return Err(route_error(
            "project_approval_route.run_binding.invalid",
            "project approval route run binding is invalid",
        ));
    }
    Ok(())
}

fn validate_route_posture(
    routing_reason: ProjectApprovalRoutingReason,
    escalation_id: Option<&str>,
    status: ProjectApprovalRouteStatus,
    recipient: Option<&ActorId>,
    notification_posture: ProjectApprovalNotificationPosture,
) -> Result<(), WorkflowOsError> {
    let subject_is_valid = matches!(
        (routing_reason, escalation_id),
        (ProjectApprovalRoutingReason::WorkflowMaintainer, None)
            | (
                ProjectApprovalRoutingReason::WorkflowEscalationContact,
                Some(_)
            )
    );
    if !subject_is_valid {
        return Err(route_error(
            "project_approval_route.escalation_subject.invalid",
            "project approval route escalation subject is invalid",
        ));
    }
    match (status, recipient, notification_posture) {
        (
            ProjectApprovalRouteStatus::Routed,
            Some(_),
            ProjectApprovalNotificationPosture::AvailableForProjectInbox,
        )
        | (
            ProjectApprovalRouteStatus::UnresolvedMissingMetadata
            | ProjectApprovalRouteStatus::UnresolvedAuthorityUnavailable,
            None,
            ProjectApprovalNotificationPosture::UnavailableRouteUnresolved,
        ) => Ok(()),
        _ => Err(route_error(
            "project_approval_route.posture.invalid",
            "project approval route posture is invalid",
        )),
    }
}

fn validate_escalation_subject(
    input: &ProjectApprovalRouteInput<'_>,
) -> Result<(), WorkflowOsError> {
    match (input.routing_reason, input.escalation) {
        (ProjectApprovalRoutingReason::WorkflowMaintainer, None) => Ok(()),
        (ProjectApprovalRoutingReason::WorkflowMaintainer, Some(_)) => Err(route_error(
            "project_approval_route.escalation_subject.unexpected",
            "ordinary project approval route cannot carry an escalation subject",
        )),
        (ProjectApprovalRoutingReason::WorkflowEscalationContact, None) => Err(route_error(
            "project_approval_route.escalation_subject.missing",
            "escalation contact routing requires an escalation subject",
        )),
        (ProjectApprovalRoutingReason::WorkflowEscalationContact, Some(escalation)) => {
            validate_escalation_reference(&escalation.escalation_id)?;
            if escalation.run_id != input.approval.run_id
                || escalation.contact.as_ref() != input.ownership.escalation_contact.as_ref()
            {
                return Err(route_error(
                    "project_approval_route.escalation_subject.mismatch",
                    "project approval route escalation subject is mismatched",
                ));
            }
            Ok(())
        }
    }
}

fn validate_approval_reference(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty()
        || value.len() > 256
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
    {
        return Err(route_error(
            "project_approval_route.approval_reference.invalid",
            "project approval route reference is invalid",
        ));
    }
    Ok(())
}

fn validate_escalation_reference(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
    {
        return Err(route_error(
            "project_approval_route.escalation_reference.invalid",
            "project approval route escalation reference is invalid",
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn derive_route_id(
    scope: &HostedProjectScope,
    run_id: &WorkflowRunId,
    approval_id: &str,
    workflow_id: &WorkflowId,
    routing_reason: ProjectApprovalRoutingReason,
    escalation_id: Option<&str>,
    status: ProjectApprovalRouteStatus,
    recipient: Option<&ActorId>,
    notification_posture: ProjectApprovalNotificationPosture,
) -> Result<ProjectApprovalRouteId, WorkflowOsError> {
    let mut hasher = Sha256::new();
    hash_field(
        &mut hasher,
        "domain",
        b"workflow-os.project-approval-route.v1",
    );
    hash_field(
        &mut hasher,
        "organization_id",
        scope.organization_id().as_str().as_bytes(),
    );
    hash_field(
        &mut hasher,
        "project_id",
        scope.project_id().as_str().as_bytes(),
    );
    hash_field(&mut hasher, "run_id", run_id.as_str().as_bytes());
    hash_field(&mut hasher, "approval_id", approval_id.as_bytes());
    hash_field(&mut hasher, "workflow_id", workflow_id.as_str().as_bytes());
    hash_field(
        &mut hasher,
        "routing_reason",
        routing_reason.label().as_bytes(),
    );
    hash_field(
        &mut hasher,
        "escalation_id",
        escalation_id.map_or(b"none".as_slice(), str::as_bytes),
    );
    hash_field(&mut hasher, "status", status.label().as_bytes());
    hash_field(
        &mut hasher,
        "recipient",
        recipient.map_or(b"none".as_slice(), |actor| actor.as_str().as_bytes()),
    );
    hash_field(
        &mut hasher,
        "notification_posture",
        notification_posture.label().as_bytes(),
    );
    ProjectApprovalRouteId::new(format!(
        "{ROUTE_ID_PREFIX}{}",
        hex_lower(hasher.finalize().as_slice())
    ))
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &[u8]) {
    hasher.update(label.len().to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(value.len().to_be_bytes());
    hasher.update(value);
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

fn route_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}
