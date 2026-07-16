use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ActorId, ApprovalReferenceId, EvidenceReferenceId, HarnessContractId, LocalCheckResultId,
    PolicyId, RedactionMetadata, StepId, Timestamp, WorkReportSensitivity, WorkflowId,
    WorkflowOsError, WorkflowRunId,
};

const IDENTIFIER_MAX_BYTES: usize = 128;
const RESOURCE_REFERENCE_MAX_BYTES: usize = 256;
const REQUIREMENT_MAX_COUNT: usize = 64;
const REDACTION_FIELD_MAX_BYTES: usize = 128;
const REDACTION_REASON_MAX_BYTES: usize = 512;
const REDACTION_MAX_ENTRIES: usize = 64;
const MAX_DELEGATION_DEPTH: u8 = 8;

/// Identifier for one scoped capability grant.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CapabilityGrantId(String);

impl CapabilityGrantId {
    /// Creates a validated capability-grant identifier.
    ///
    /// # Errors
    ///
    /// Returns an error when the identifier is empty, too long, malformed, or secret-like.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("capability grant id", &value)?;
        Ok(Self(value))
    }

    /// Returns the identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CapabilityGrantId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for CapabilityGrantId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CapabilityGrantId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<CapabilityGrantId> for String {
    fn from(value: CapabilityGrantId) -> Self {
        value.0
    }
}

impl TryFrom<String> for CapabilityGrantId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for CapabilityGrantId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Stable capability identifier independent of current availability or authority.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CapabilityReference(String);

impl CapabilityReference {
    /// Creates a validated capability reference.
    ///
    /// # Errors
    ///
    /// Returns an error when the reference is empty, too long, malformed, or secret-like.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier("capability reference", &value)?;
        Ok(Self(value))
    }

    /// Returns the capability reference text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CapabilityReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for CapabilityReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CapabilityReference")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<CapabilityReference> for String {
    fn from(value: CapabilityReference) -> Self {
        value.0
    }
}

impl TryFrom<String> for CapabilityReference {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for CapabilityReference {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Domain-neutral resource class constrained by a capability grant.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityResourceKind {
    /// Repository resource.
    Repository,
    /// Workflow-owned resource.
    Workflow,
    /// Local project resource.
    LocalProject,
    /// Adapter-addressed resource.
    AdapterResource,
    /// External provider resource.
    ExternalResource,
    /// Unknown resource class. Valid grants reject this state.
    Unknown,
}

/// Bounded resource scope attached to a capability grant.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CapabilityResourceScope {
    kind: CapabilityResourceKind,
    reference: String,
}

impl CapabilityResourceScope {
    /// Creates a validated resource scope.
    ///
    /// # Errors
    ///
    /// Returns an error for unknown kinds, raw paths, malformed references, or secret-like text.
    pub fn new(
        kind: CapabilityResourceKind,
        reference: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let scope = Self {
            kind,
            reference: reference.into(),
        };
        scope.validate()?;
        Ok(scope)
    }

    /// Validates this resource scope.
    ///
    /// # Errors
    ///
    /// Returns an error when the resource scope is not bounded and canonical.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.kind == CapabilityResourceKind::Unknown {
            return Err(validation_error(
                "capability_authority.resource.kind_unknown",
                "capability resource kind must be known",
            ));
        }
        validate_resource_reference(&self.reference)
    }

    /// Returns the resource kind.
    #[must_use]
    pub const fn kind(&self) -> CapabilityResourceKind {
        self.kind
    }

    /// Returns the bounded resource reference.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }
}

impl fmt::Debug for CapabilityResourceScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CapabilityResourceScope")
            .field("kind", &self.kind)
            .field("reference", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for CapabilityResourceScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            kind: CapabilityResourceKind,
            reference: String,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(wire.kind, wire.reference).map_err(serde::de::Error::custom)
    }
}

/// Exact workflow/run/step/harness boundary for one grant.
#[allow(clippy::struct_field_names)]
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CapabilityGrantScope {
    workflow_id: WorkflowId,
    run_id: Option<WorkflowRunId>,
    step_id: Option<StepId>,
    harness_contract_id: Option<HarnessContractId>,
}

impl CapabilityGrantScope {
    /// Creates a validated scoped grant boundary.
    ///
    /// # Errors
    ///
    /// Returns an error when a step is supplied without an exact run binding.
    pub fn new(
        workflow_id: WorkflowId,
        run_id: Option<WorkflowRunId>,
        step_id: Option<StepId>,
        harness_contract_id: Option<HarnessContractId>,
    ) -> Result<Self, WorkflowOsError> {
        validate_not_secret_like("capability scope workflow id", workflow_id.as_str())?;
        if let Some(run_id) = &run_id {
            validate_not_secret_like("capability scope run id", run_id.as_str())?;
        }
        if let Some(step_id) = &step_id {
            validate_not_secret_like("capability scope step id", step_id.as_str())?;
        }
        if let Some(harness_contract_id) = &harness_contract_id {
            validate_not_secret_like(
                "capability scope harness contract id",
                harness_contract_id.as_str(),
            )?;
        }
        if step_id.is_some() && run_id.is_none() {
            return Err(validation_error(
                "capability_authority.scope.step_requires_run",
                "step-scoped capability grants require an exact run binding",
            ));
        }
        Ok(Self {
            workflow_id,
            run_id,
            step_id,
            harness_contract_id,
        })
    }

    /// Returns the workflow boundary.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the exact run boundary, when present.
    #[must_use]
    pub const fn run_id(&self) -> Option<&WorkflowRunId> {
        self.run_id.as_ref()
    }

    /// Returns the exact step boundary, when present.
    #[must_use]
    pub const fn step_id(&self) -> Option<&StepId> {
        self.step_id.as_ref()
    }

    /// Returns the harness-contract boundary, when present.
    #[must_use]
    pub const fn harness_contract_id(&self) -> Option<&HarnessContractId> {
        self.harness_contract_id.as_ref()
    }
}

impl fmt::Debug for CapabilityGrantScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CapabilityGrantScope")
            .field("workflow_id", &"[REDACTED]")
            .field("run_id", &self.run_id.as_ref().map(|_| "[REDACTED]"))
            .field("step_id", &self.step_id.as_ref().map(|_| "[REDACTED]"))
            .field(
                "harness_contract_id",
                &self.harness_contract_id.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

impl<'de> Deserialize<'de> for CapabilityGrantScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[allow(clippy::struct_field_names)]
        #[derive(Deserialize)]
        struct Wire {
            workflow_id: WorkflowId,
            run_id: Option<WorkflowRunId>,
            step_id: Option<StepId>,
            harness_contract_id: Option<HarnessContractId>,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.workflow_id,
            wire.run_id,
            wire.step_id,
            wire.harness_contract_id,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// Lifecycle posture for a capability grant.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityGrantLifecycle {
    /// Grant is active subject to expiry and independent policy/approval checks.
    Active,
    /// Grant has been revoked and cannot authorize future work.
    Revoked,
}

/// Delegation posture for a capability grant.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "mode")]
pub enum CapabilityDelegationPosture {
    /// Delegation is disabled.
    #[default]
    Disabled,
    /// Delegation is explicitly bounded by a maximum depth.
    Allowed {
        /// Maximum number of downstream delegations.
        max_depth: u8,
    },
}

impl CapabilityDelegationPosture {
    fn validate(self) -> Result<(), WorkflowOsError> {
        if let Self::Allowed { max_depth } = self {
            if max_depth == 0 || max_depth > MAX_DELEGATION_DEPTH {
                return Err(validation_error(
                    "capability_authority.delegation.depth_invalid",
                    "delegation depth must be within the supported bounded range",
                ));
            }
        }
        Ok(())
    }
}

/// Stable prerequisite references that remain independent of grant existence.
#[allow(clippy::struct_field_names)]
#[derive(Clone, Default, Eq, PartialEq, Serialize)]
pub struct CapabilityGrantRequirements {
    /// Policy definitions required for future invocation evaluation.
    policy_ids: Vec<PolicyId>,
    /// Approval decisions required by the grant declaration.
    approval_ids: Vec<ApprovalReferenceId>,
    /// Evidence references required by the grant declaration.
    evidence_ids: Vec<EvidenceReferenceId>,
    /// Local-check result references required by the grant declaration.
    check_ids: Vec<LocalCheckResultId>,
}

impl fmt::Debug for CapabilityGrantRequirements {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CapabilityGrantRequirements")
            .field("policy_ids", &self.policy_ids.len())
            .field("approval_ids", &self.approval_ids.len())
            .field("evidence_ids", &self.evidence_ids.len())
            .field("check_ids", &self.check_ids.len())
            .finish()
    }
}

impl CapabilityGrantRequirements {
    /// Creates validated prerequisite references.
    ///
    /// # Errors
    ///
    /// Returns an error for excessive, duplicate, or secret-like references.
    pub fn new(
        policy_ids: Vec<PolicyId>,
        approval_ids: Vec<ApprovalReferenceId>,
        evidence_ids: Vec<EvidenceReferenceId>,
        check_ids: Vec<LocalCheckResultId>,
    ) -> Result<Self, WorkflowOsError> {
        let requirements = Self {
            policy_ids,
            approval_ids,
            evidence_ids,
            check_ids,
        };
        requirements.validate()?;
        Ok(requirements)
    }

    /// Returns required policy references.
    #[must_use]
    pub fn policy_ids(&self) -> &[PolicyId] {
        &self.policy_ids
    }

    /// Returns required approval references.
    #[must_use]
    pub fn approval_ids(&self) -> &[ApprovalReferenceId] {
        &self.approval_ids
    }

    /// Returns required evidence references.
    #[must_use]
    pub fn evidence_ids(&self) -> &[EvidenceReferenceId] {
        &self.evidence_ids
    }

    /// Returns required local-check references.
    #[must_use]
    pub fn check_ids(&self) -> &[LocalCheckResultId] {
        &self.check_ids
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_unique_references("policy", &self.policy_ids, PolicyId::as_str)?;
        validate_unique_references("approval", &self.approval_ids, ApprovalReferenceId::as_str)?;
        validate_unique_references("evidence", &self.evidence_ids, EvidenceReferenceId::as_str)?;
        validate_unique_references("check", &self.check_ids, LocalCheckResultId::as_str)
    }
}

impl<'de> Deserialize<'de> for CapabilityGrantRequirements {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[allow(clippy::struct_field_names)]
        #[derive(Deserialize)]
        struct Wire {
            policy_ids: Vec<PolicyId>,
            approval_ids: Vec<ApprovalReferenceId>,
            evidence_ids: Vec<EvidenceReferenceId>,
            check_ids: Vec<LocalCheckResultId>,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.policy_ids,
            wire.approval_ids,
            wire.evidence_ids,
            wire.check_ids,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// Public construction definition for a capability grant.
#[derive(Clone, Eq, PartialEq)]
pub struct CapabilityGrantDefinition {
    /// Grant identity.
    pub grant_id: CapabilityGrantId,
    /// Actor receiving authority.
    pub subject: ActorId,
    /// Stable capability reference.
    pub capability: CapabilityReference,
    /// Bounded resource scope.
    pub resource: CapabilityResourceScope,
    /// Workflow/run/step/harness scope.
    pub scope: CapabilityGrantScope,
    /// Actor issuing the grant.
    pub issuer: ActorId,
    /// Grant issuance timestamp.
    pub issued_at: Timestamp,
    /// Optional expiry timestamp.
    pub expires_at: Option<Timestamp>,
    /// Active or revoked lifecycle.
    pub lifecycle: CapabilityGrantLifecycle,
    /// Required bounded revocation reference when revoked.
    pub revocation_reference: Option<String>,
    /// Explicit delegation posture.
    pub delegation: CapabilityDelegationPosture,
    /// Independent policy, approval, evidence, and check prerequisites.
    pub requirements: CapabilityGrantRequirements,
    /// Maximum sensitivity permitted by the grant.
    pub sensitivity_ceiling: WorkReportSensitivity,
    /// Required redaction metadata.
    pub redaction: RedactionMetadata,
}

impl fmt::Debug for CapabilityGrantDefinition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CapabilityGrantDefinition")
            .field("grant_id", &"[REDACTED]")
            .field("subject", &"[REDACTED]")
            .field("capability", &"[REDACTED]")
            .field("resource", &self.resource)
            .field("scope", &self.scope)
            .field("issuer", &"[REDACTED]")
            .field("issued_at", &self.issued_at)
            .field("expires_at", &self.expires_at)
            .field("lifecycle", &self.lifecycle)
            .field(
                "revocation_reference",
                &self.revocation_reference.as_ref().map(|_| "[REDACTED]"),
            )
            .field("delegation", &self.delegation)
            .field("requirements", &self.requirements)
            .field("sensitivity_ceiling", &self.sensitivity_ceiling)
            .field(
                "redaction",
                &RedactedRedactionMetadataDebug(&self.redaction),
            )
            .finish()
    }
}

/// Validated, domain-neutral scoped authority record.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CapabilityGrant {
    grant_id: CapabilityGrantId,
    subject: ActorId,
    capability: CapabilityReference,
    resource: CapabilityResourceScope,
    scope: CapabilityGrantScope,
    issuer: ActorId,
    issued_at: Timestamp,
    expires_at: Option<Timestamp>,
    lifecycle: CapabilityGrantLifecycle,
    revocation_reference: Option<String>,
    delegation: CapabilityDelegationPosture,
    requirements: CapabilityGrantRequirements,
    sensitivity_ceiling: WorkReportSensitivity,
    redaction: RedactionMetadata,
}

impl CapabilityGrant {
    /// Creates a validated capability grant.
    ///
    /// # Errors
    ///
    /// Returns a stable, non-leaking validation error for malformed or inconsistent input.
    pub fn new(definition: CapabilityGrantDefinition) -> Result<Self, WorkflowOsError> {
        let grant = Self {
            grant_id: definition.grant_id,
            subject: definition.subject,
            capability: definition.capability,
            resource: definition.resource,
            scope: definition.scope,
            issuer: definition.issuer,
            issued_at: definition.issued_at,
            expires_at: definition.expires_at,
            lifecycle: definition.lifecycle,
            revocation_reference: definition.revocation_reference,
            delegation: definition.delegation,
            requirements: definition.requirements,
            sensitivity_ceiling: definition.sensitivity_ceiling,
            redaction: definition.redaction,
        };
        grant.validate()?;
        Ok(grant)
    }

    /// Validates all capability-grant invariants.
    ///
    /// # Errors
    ///
    /// Returns a stable, non-leaking error for invalid grant state.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_not_secret_like("capability grant subject", self.subject.as_str())?;
        validate_not_secret_like("capability grant issuer", self.issuer.as_str())?;
        self.resource.validate()?;
        if let Some(expires_at) = self.expires_at {
            if expires_at <= self.issued_at {
                return Err(validation_error(
                    "capability_authority.expiry.not_after_issuance",
                    "capability grant expiry must be after issuance",
                ));
            }
        }
        match (self.lifecycle, self.revocation_reference.as_deref()) {
            (CapabilityGrantLifecycle::Active, None) => {}
            (CapabilityGrantLifecycle::Active, Some(_)) => {
                return Err(validation_error(
                    "capability_authority.revocation.active_has_reference",
                    "active capability grants cannot carry a revocation reference",
                ));
            }
            (CapabilityGrantLifecycle::Revoked, Some(reference)) => {
                validate_reference("revocation reference", reference)?;
            }
            (CapabilityGrantLifecycle::Revoked, None) => {
                return Err(validation_error(
                    "capability_authority.revocation.reference_required",
                    "revoked capability grants require a revocation reference",
                ));
            }
        }
        self.delegation.validate()?;
        self.requirements.validate()?;
        if self.sensitivity_ceiling == WorkReportSensitivity::Unknown {
            return Err(validation_error(
                "capability_authority.sensitivity.unknown",
                "capability grant sensitivity ceiling must be known",
            ));
        }
        validate_redaction_metadata(&self.redaction)
    }

    /// Returns the grant identity.
    #[must_use]
    pub const fn grant_id(&self) -> &CapabilityGrantId {
        &self.grant_id
    }

    /// Returns the subject actor.
    #[must_use]
    pub const fn subject(&self) -> &ActorId {
        &self.subject
    }

    /// Returns the capability reference.
    #[must_use]
    pub const fn capability(&self) -> &CapabilityReference {
        &self.capability
    }

    /// Returns the bounded resource scope.
    #[must_use]
    pub const fn resource(&self) -> &CapabilityResourceScope {
        &self.resource
    }

    /// Returns the workflow/run/step/harness scope.
    #[must_use]
    pub const fn scope(&self) -> &CapabilityGrantScope {
        &self.scope
    }

    /// Returns the issuer actor.
    #[must_use]
    pub const fn issuer(&self) -> &ActorId {
        &self.issuer
    }

    /// Returns the issuance timestamp.
    #[must_use]
    pub const fn issued_at(&self) -> Timestamp {
        self.issued_at
    }

    /// Returns the expiry timestamp, when present.
    #[must_use]
    pub const fn expires_at(&self) -> Option<Timestamp> {
        self.expires_at
    }

    /// Returns the lifecycle posture.
    #[must_use]
    pub const fn lifecycle(&self) -> CapabilityGrantLifecycle {
        self.lifecycle
    }

    /// Returns the bounded revocation reference, when present.
    #[must_use]
    pub fn revocation_reference(&self) -> Option<&str> {
        self.revocation_reference.as_deref()
    }

    /// Returns the delegation posture.
    #[must_use]
    pub const fn delegation(&self) -> CapabilityDelegationPosture {
        self.delegation
    }

    /// Returns prerequisite references.
    #[must_use]
    pub const fn requirements(&self) -> &CapabilityGrantRequirements {
        &self.requirements
    }

    /// Returns the sensitivity ceiling.
    #[must_use]
    pub const fn sensitivity_ceiling(&self) -> WorkReportSensitivity {
        self.sensitivity_ceiling
    }

    /// Returns redaction metadata.
    #[must_use]
    pub const fn redaction(&self) -> &RedactionMetadata {
        &self.redaction
    }
}

impl fmt::Debug for CapabilityGrant {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CapabilityGrant")
            .field("grant_id", &"[REDACTED]")
            .field("subject", &"[REDACTED]")
            .field("capability", &"[REDACTED]")
            .field("resource", &self.resource)
            .field("scope", &"[REDACTED]")
            .field("issuer", &"[REDACTED]")
            .field("issued_at", &self.issued_at)
            .field("expires_at", &self.expires_at)
            .field("lifecycle", &self.lifecycle)
            .field(
                "revocation_reference",
                &self.revocation_reference.as_ref().map(|_| "[REDACTED]"),
            )
            .field("delegation", &self.delegation)
            .field("requirements", &"[REDACTED]")
            .field("sensitivity_ceiling", &self.sensitivity_ceiling)
            .field(
                "redaction",
                &RedactedRedactionMetadataDebug(&self.redaction),
            )
            .finish()
    }
}

impl<'de> Deserialize<'de> for CapabilityGrant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            grant_id: CapabilityGrantId,
            subject: ActorId,
            capability: CapabilityReference,
            resource: CapabilityResourceScope,
            scope: CapabilityGrantScope,
            issuer: ActorId,
            issued_at: Timestamp,
            expires_at: Option<Timestamp>,
            lifecycle: CapabilityGrantLifecycle,
            revocation_reference: Option<String>,
            delegation: CapabilityDelegationPosture,
            requirements: CapabilityGrantRequirements,
            sensitivity_ceiling: WorkReportSensitivity,
            redaction: RedactionMetadata,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(CapabilityGrantDefinition {
            grant_id: wire.grant_id,
            subject: wire.subject,
            capability: wire.capability,
            resource: wire.resource,
            scope: wire.scope,
            issuer: wire.issuer,
            issued_at: wire.issued_at,
            expires_at: wire.expires_at,
            lifecycle: wire.lifecycle,
            revocation_reference: wire.revocation_reference,
            delegation: wire.delegation,
            requirements: wire.requirements,
            sensitivity_ceiling: wire.sensitivity_ceiling,
            redaction: wire.redaction,
        })
        .map_err(serde::de::Error::custom)
    }
}

/// Explicit current inventory/connectivity posture for one bounded resource.
///
/// Availability does not express authority, denial, expiry, revocation, or
/// invocation readiness. Those outcomes require independent validated inputs.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityAvailability {
    /// Capability is present in the current bounded inventory.
    Available,
    /// Capability is declared but no connection is configured.
    DeclaredNotConnected,
    /// Capability is known to be unsupported by the current build or adapter.
    KnownUnsupported,
    /// Capability posture is unknown and must fail closed before invocation.
    Unknown,
}

/// Bounded observation of capability availability without runtime activation.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CapabilityAvailabilityRecord {
    capability: CapabilityReference,
    resource: CapabilityResourceScope,
    availability: CapabilityAvailability,
    observed_at: Timestamp,
    redaction: RedactionMetadata,
}

impl CapabilityAvailabilityRecord {
    /// Creates a validated capability availability record.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid resource or redaction metadata.
    pub fn new(
        capability: CapabilityReference,
        resource: CapabilityResourceScope,
        availability: CapabilityAvailability,
        observed_at: Timestamp,
        redaction: RedactionMetadata,
    ) -> Result<Self, WorkflowOsError> {
        resource.validate()?;
        validate_redaction_metadata(&redaction)?;
        Ok(Self {
            capability,
            resource,
            availability,
            observed_at,
            redaction,
        })
    }

    /// Returns the capability reference.
    #[must_use]
    pub const fn capability(&self) -> &CapabilityReference {
        &self.capability
    }

    /// Returns the resource scope.
    #[must_use]
    pub const fn resource(&self) -> &CapabilityResourceScope {
        &self.resource
    }

    /// Returns the observed availability posture.
    #[must_use]
    pub const fn availability(&self) -> CapabilityAvailability {
        self.availability
    }

    /// Returns the observation timestamp.
    #[must_use]
    pub const fn observed_at(&self) -> Timestamp {
        self.observed_at
    }

    /// Returns redaction metadata.
    #[must_use]
    pub const fn redaction(&self) -> &RedactionMetadata {
        &self.redaction
    }
}

impl fmt::Debug for CapabilityAvailabilityRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CapabilityAvailabilityRecord")
            .field("capability", &"[REDACTED]")
            .field("resource", &self.resource)
            .field("availability", &self.availability)
            .field("observed_at", &self.observed_at)
            .field(
                "redaction",
                &RedactedRedactionMetadataDebug(&self.redaction),
            )
            .finish()
    }
}

impl<'de> Deserialize<'de> for CapabilityAvailabilityRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            capability: CapabilityReference,
            resource: CapabilityResourceScope,
            availability: CapabilityAvailability,
            observed_at: Timestamp,
            redaction: RedactionMetadata,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.capability,
            wire.resource,
            wire.availability,
            wire.observed_at,
            wire.redaction,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// Terminal posture from pure capability resolution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityResolutionPosture {
    /// One active, current, scope-matching grant authorizes the request.
    Authorized,
    /// A matching grant exists, but independent prerequisite evaluation remains.
    RequiresIndependentEvaluation,
    /// The request is not authorized by the supplied validated inputs.
    NotAuthorized,
}

/// Stable, payload-free reasons produced by pure capability resolution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityResolutionReason {
    /// An active matching grant authorizes the request.
    ActiveGrantMatched,
    /// No current availability record matched the requested capability and resource.
    AvailabilityRecordMissing,
    /// The capability is declared but not connected.
    CapabilityNotConnected,
    /// The capability is unsupported by the current bounded inventory.
    CapabilityUnsupported,
    /// Capability availability is unknown.
    CapabilityAvailabilityUnknown,
    /// No grant matched actor, capability, resource, workflow, run, step, and harness scope.
    NoMatchingGrant,
    /// A matching grant is revoked.
    MatchingGrantRevoked,
    /// A matching grant is expired at the explicit evaluation time.
    MatchingGrantExpired,
    /// Requested sensitivity exceeds a matching grant ceiling.
    SensitivityExceedsGrant,
    /// A matching grant requires independent policy evaluation.
    PolicyEvaluationRequired,
    /// A matching grant requires an independent approval decision.
    ApprovalEvaluationRequired,
    /// A matching grant requires independent evidence validation.
    EvidenceEvaluationRequired,
    /// A matching grant requires independent local-check validation.
    CheckEvaluationRequired,
}

/// Explicit, borrowed inputs for pure capability resolution.
pub struct CapabilityResolutionInput<'a> {
    /// Requested capability.
    pub capability: &'a CapabilityReference,
    /// Requested bounded resource.
    pub resource: &'a CapabilityResourceScope,
    /// Actor requesting authority.
    pub actor: &'a ActorId,
    /// Workflow boundary.
    pub workflow_id: &'a WorkflowId,
    /// Exact run boundary.
    pub run_id: &'a WorkflowRunId,
    /// Exact step boundary.
    pub step_id: &'a StepId,
    /// Optional harness-contract boundary.
    pub harness_contract_id: Option<&'a HarnessContractId>,
    /// Sensitivity requested for the invocation context.
    pub requested_sensitivity: WorkReportSensitivity,
    /// Explicit evaluation timestamp.
    pub evaluated_at: Timestamp,
    /// Current bounded inventory observations.
    pub availability_records: &'a [CapabilityAvailabilityRecord],
    /// Candidate validated grants.
    pub grants: &'a [CapabilityGrant],
}

impl fmt::Debug for CapabilityResolutionInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CapabilityResolutionInput")
            .field("capability", &"[REDACTED]")
            .field("resource", &"[REDACTED]")
            .field("actor", &"[REDACTED]")
            .field("workflow_id", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("step_id", &"[REDACTED]")
            .field(
                "harness_contract_id",
                &self.harness_contract_id.map(|_| "[REDACTED]"),
            )
            .field("requested_sensitivity", &self.requested_sensitivity)
            .field("evaluated_at", &self.evaluated_at)
            .field("availability_records", &self.availability_records.len())
            .field("grants", &self.grants.len())
            .finish()
    }
}

/// Deterministic, payload-free capability resolution result.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CapabilityResolution {
    posture: CapabilityResolutionPosture,
    availability: Option<CapabilityAvailability>,
    selected_grant_id: Option<CapabilityGrantId>,
    reasons: Vec<CapabilityResolutionReason>,
    evaluated_at: Timestamp,
}

impl CapabilityResolution {
    /// Validates the internal consistency of this resolution result.
    ///
    /// # Errors
    ///
    /// Returns a stable validation error for impossible posture, availability,
    /// grant, or reason combinations.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.reasons.is_empty() {
            return Err(validation_error(
                "capability_authority.resolution.reasons_empty",
                "capability resolution requires at least one stable reason",
            ));
        }
        if self.reasons.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(validation_error(
                "capability_authority.resolution.reasons_invalid",
                "capability resolution reasons must be unique and ordered",
            ));
        }

        let has_authorized_reason = self
            .reasons
            .contains(&CapabilityResolutionReason::ActiveGrantMatched);
        let has_prerequisite_reason = self.reasons.iter().any(|reason| {
            matches!(
                reason,
                CapabilityResolutionReason::PolicyEvaluationRequired
                    | CapabilityResolutionReason::ApprovalEvaluationRequired
                    | CapabilityResolutionReason::EvidenceEvaluationRequired
                    | CapabilityResolutionReason::CheckEvaluationRequired
            )
        });
        match self.posture {
            CapabilityResolutionPosture::Authorized
                if self.availability == Some(CapabilityAvailability::Available)
                    && self.selected_grant_id.is_some()
                    && self.reasons == [CapabilityResolutionReason::ActiveGrantMatched] =>
            {
                Ok(())
            }
            CapabilityResolutionPosture::RequiresIndependentEvaluation
                if self.availability == Some(CapabilityAvailability::Available)
                    && self.selected_grant_id.is_some()
                    && has_prerequisite_reason
                    && !has_authorized_reason
                    && self.reasons.iter().all(|reason| {
                        matches!(
                            reason,
                            CapabilityResolutionReason::PolicyEvaluationRequired
                                | CapabilityResolutionReason::ApprovalEvaluationRequired
                                | CapabilityResolutionReason::EvidenceEvaluationRequired
                                | CapabilityResolutionReason::CheckEvaluationRequired
                        )
                    }) =>
            {
                Ok(())
            }
            CapabilityResolutionPosture::NotAuthorized
                if self.selected_grant_id.is_none()
                    && !has_authorized_reason
                    && !has_prerequisite_reason
                    && valid_not_authorized_reasons(self.availability, &self.reasons) =>
            {
                Ok(())
            }
            _ => Err(validation_error(
                "capability_authority.resolution.inconsistent",
                "capability resolution posture is inconsistent with its bounded inputs",
            )),
        }
    }

    /// Returns the resolution posture.
    #[must_use]
    pub const fn posture(&self) -> CapabilityResolutionPosture {
        self.posture
    }

    /// Returns the matched inventory posture, when one unambiguous record exists.
    #[must_use]
    pub const fn availability(&self) -> Option<CapabilityAvailability> {
        self.availability
    }

    /// Returns the selected matching grant identity, when applicable.
    #[must_use]
    pub const fn selected_grant_id(&self) -> Option<&CapabilityGrantId> {
        self.selected_grant_id.as_ref()
    }

    /// Returns stable reasons for the result.
    #[must_use]
    pub fn reasons(&self) -> &[CapabilityResolutionReason] {
        &self.reasons
    }

    /// Returns the explicit evaluation timestamp.
    #[must_use]
    pub const fn evaluated_at(&self) -> Timestamp {
        self.evaluated_at
    }
}

fn valid_not_authorized_reasons(
    availability: Option<CapabilityAvailability>,
    reasons: &[CapabilityResolutionReason],
) -> bool {
    match availability {
        None => reasons == [CapabilityResolutionReason::AvailabilityRecordMissing],
        Some(CapabilityAvailability::DeclaredNotConnected) => {
            reasons == [CapabilityResolutionReason::CapabilityNotConnected]
        }
        Some(CapabilityAvailability::KnownUnsupported) => {
            reasons == [CapabilityResolutionReason::CapabilityUnsupported]
        }
        Some(CapabilityAvailability::Unknown) => {
            reasons == [CapabilityResolutionReason::CapabilityAvailabilityUnknown]
        }
        Some(CapabilityAvailability::Available) => {
            reasons == [CapabilityResolutionReason::NoMatchingGrant]
                || reasons.iter().all(|reason| {
                    matches!(
                        reason,
                        CapabilityResolutionReason::MatchingGrantRevoked
                            | CapabilityResolutionReason::MatchingGrantExpired
                            | CapabilityResolutionReason::SensitivityExceedsGrant
                    )
                })
        }
    }
}

impl fmt::Debug for CapabilityResolution {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CapabilityResolution")
            .field("posture", &self.posture)
            .field("availability", &self.availability)
            .field(
                "selected_grant_id",
                &self.selected_grant_id.as_ref().map(|_| "[REDACTED]"),
            )
            .field("reasons", &self.reasons)
            .field("evaluated_at", &self.evaluated_at)
            .finish()
    }
}

impl<'de> Deserialize<'de> for CapabilityResolution {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            posture: CapabilityResolutionPosture,
            availability: Option<CapabilityAvailability>,
            selected_grant_id: Option<CapabilityGrantId>,
            reasons: Vec<CapabilityResolutionReason>,
            evaluated_at: Timestamp,
        }

        let wire = Wire::deserialize(deserializer)?;
        let resolution = Self {
            posture: wire.posture,
            availability: wire.availability,
            selected_grant_id: wire.selected_grant_id,
            reasons: wire.reasons,
            evaluated_at: wire.evaluated_at,
        };
        resolution.validate().map_err(serde::de::Error::custom)?;
        Ok(resolution)
    }
}

/// Resolves current capability authority from explicit validated inputs.
///
/// This helper is pure and non-mutating. Availability alone never authorizes
/// invocation, and declared grant prerequisites remain independent evaluation
/// obligations rather than being inferred as satisfied.
///
/// # Errors
///
/// Returns stable validation errors for ambiguous inventory, duplicate grant
/// identities, future observations, or unknown requested sensitivity.
pub fn resolve_capability_authority(
    input: &CapabilityResolutionInput<'_>,
) -> Result<CapabilityResolution, WorkflowOsError> {
    if input.requested_sensitivity == WorkReportSensitivity::Unknown {
        return Err(validation_error(
            "capability_authority.resolution.sensitivity_unknown",
            "capability resolution requires known requested sensitivity",
        ));
    }

    let Some(availability_record) = find_matching_availability(input)? else {
        return Ok(resolution(
            CapabilityResolutionPosture::NotAuthorized,
            None,
            None,
            [CapabilityResolutionReason::AvailabilityRecordMissing],
            input.evaluated_at,
        ));
    };
    let availability = availability_record.availability();
    let unavailable_reason = match availability {
        CapabilityAvailability::Available => None,
        CapabilityAvailability::DeclaredNotConnected => {
            Some(CapabilityResolutionReason::CapabilityNotConnected)
        }
        CapabilityAvailability::KnownUnsupported => {
            Some(CapabilityResolutionReason::CapabilityUnsupported)
        }
        CapabilityAvailability::Unknown => {
            Some(CapabilityResolutionReason::CapabilityAvailabilityUnknown)
        }
    };
    if let Some(reason) = unavailable_reason {
        return Ok(resolution(
            CapabilityResolutionPosture::NotAuthorized,
            Some(availability),
            None,
            [reason],
            input.evaluated_at,
        ));
    }

    let matching_grants = find_matching_grants(input)?;
    if matching_grants.is_empty() {
        return Ok(resolution(
            CapabilityResolutionPosture::NotAuthorized,
            Some(availability),
            None,
            [CapabilityResolutionReason::NoMatchingGrant],
            input.evaluated_at,
        ));
    }

    let mut deferred: Option<(&CapabilityGrant, BTreeSet<CapabilityResolutionReason>)> = None;
    let mut rejected_reasons = BTreeSet::new();
    for grant in matching_grants {
        if grant.lifecycle() == CapabilityGrantLifecycle::Revoked {
            rejected_reasons.insert(CapabilityResolutionReason::MatchingGrantRevoked);
            continue;
        }
        if grant
            .expires_at()
            .is_some_and(|expires_at| expires_at <= input.evaluated_at)
        {
            rejected_reasons.insert(CapabilityResolutionReason::MatchingGrantExpired);
            continue;
        }
        if input.requested_sensitivity > grant.sensitivity_ceiling() {
            rejected_reasons.insert(CapabilityResolutionReason::SensitivityExceedsGrant);
            continue;
        }

        let prerequisite_reasons = prerequisite_reasons(grant.requirements());
        if prerequisite_reasons.is_empty() {
            return Ok(resolution(
                CapabilityResolutionPosture::Authorized,
                Some(availability),
                Some(grant.grant_id().clone()),
                [CapabilityResolutionReason::ActiveGrantMatched],
                input.evaluated_at,
            ));
        }
        if deferred.is_none() {
            deferred = Some((grant, prerequisite_reasons));
        }
    }

    if let Some((grant, reasons)) = deferred {
        return Ok(CapabilityResolution {
            posture: CapabilityResolutionPosture::RequiresIndependentEvaluation,
            availability: Some(availability),
            selected_grant_id: Some(grant.grant_id().clone()),
            reasons: reasons.into_iter().collect(),
            evaluated_at: input.evaluated_at,
        });
    }

    Ok(CapabilityResolution {
        posture: CapabilityResolutionPosture::NotAuthorized,
        availability: Some(availability),
        selected_grant_id: None,
        reasons: rejected_reasons.into_iter().collect(),
        evaluated_at: input.evaluated_at,
    })
}

fn find_matching_availability<'a>(
    input: &'a CapabilityResolutionInput<'_>,
) -> Result<Option<&'a CapabilityAvailabilityRecord>, WorkflowOsError> {
    let matching = input
        .availability_records
        .iter()
        .filter(|record| {
            record.capability() == input.capability && record.resource() == input.resource
        })
        .collect::<Vec<_>>();
    if matching.len() > 1 {
        return Err(validation_error(
            "capability_authority.resolution.availability_ambiguous",
            "capability resolution requires one current availability record",
        ));
    }
    let record = matching.first().copied();
    if record.is_some_and(|record| record.observed_at() > input.evaluated_at) {
        return Err(validation_error(
            "capability_authority.resolution.observation_in_future",
            "capability availability observation cannot follow evaluation time",
        ));
    }
    Ok(record)
}

fn find_matching_grants<'a>(
    input: &'a CapabilityResolutionInput<'_>,
) -> Result<Vec<&'a CapabilityGrant>, WorkflowOsError> {
    let mut grant_ids = BTreeSet::new();
    for grant in input.grants {
        grant.validate()?;
        if !grant_ids.insert(grant.grant_id()) {
            return Err(validation_error(
                "capability_authority.resolution.duplicate_grant",
                "capability resolution cannot accept duplicate grant identities",
            ));
        }
    }

    let mut matching = input
        .grants
        .iter()
        .filter(|grant| grant_matches_request(grant, input))
        .collect::<Vec<_>>();
    matching.sort_by(|left, right| {
        grant_specificity(right)
            .cmp(&grant_specificity(left))
            .then_with(|| left.grant_id().cmp(right.grant_id()))
    });
    if let Some(highest_specificity) = matching.first().map(|grant| grant_specificity(grant)) {
        matching.retain(|grant| grant_specificity(grant) == highest_specificity);
    }
    Ok(matching)
}

fn grant_matches_request(grant: &CapabilityGrant, input: &CapabilityResolutionInput<'_>) -> bool {
    let scope = grant.scope();
    grant.subject() == input.actor
        && grant.capability() == input.capability
        && grant.resource() == input.resource
        && scope.workflow_id() == input.workflow_id
        && scope.run_id().map_or(true, |run_id| run_id == input.run_id)
        && scope
            .step_id()
            .map_or(true, |step_id| step_id == input.step_id)
        && scope.harness_contract_id().map_or(true, |harness_id| {
            Some(harness_id) == input.harness_contract_id
        })
}

fn grant_specificity(grant: &CapabilityGrant) -> u8 {
    u8::from(grant.scope().run_id().is_some())
        + u8::from(grant.scope().step_id().is_some())
        + u8::from(grant.scope().harness_contract_id().is_some())
}

fn prerequisite_reasons(
    requirements: &CapabilityGrantRequirements,
) -> BTreeSet<CapabilityResolutionReason> {
    let mut reasons = BTreeSet::new();
    if !requirements.policy_ids().is_empty() {
        reasons.insert(CapabilityResolutionReason::PolicyEvaluationRequired);
    }
    if !requirements.approval_ids().is_empty() {
        reasons.insert(CapabilityResolutionReason::ApprovalEvaluationRequired);
    }
    if !requirements.evidence_ids().is_empty() {
        reasons.insert(CapabilityResolutionReason::EvidenceEvaluationRequired);
    }
    if !requirements.check_ids().is_empty() {
        reasons.insert(CapabilityResolutionReason::CheckEvaluationRequired);
    }
    reasons
}

fn resolution(
    posture: CapabilityResolutionPosture,
    availability: Option<CapabilityAvailability>,
    selected_grant_id: Option<CapabilityGrantId>,
    reasons: impl IntoIterator<Item = CapabilityResolutionReason>,
    evaluated_at: Timestamp,
) -> CapabilityResolution {
    let resolution = CapabilityResolution {
        posture,
        availability,
        selected_grant_id,
        reasons: reasons.into_iter().collect(),
        evaluated_at,
    };
    debug_assert!(resolution.validate().is_ok());
    resolution
}

fn validate_unique_references<T, F>(
    kind: &'static str,
    values: &[T],
    as_str: F,
) -> Result<(), WorkflowOsError>
where
    T: Ord,
    F: Fn(&T) -> &str,
{
    if values.len() > REQUIREMENT_MAX_COUNT {
        return Err(validation_error(
            "capability_authority.requirements.too_many",
            "capability grant contains too many prerequisite references",
        ));
    }
    let mut seen = BTreeSet::new();
    for value in values {
        validate_not_secret_like("capability prerequisite reference", as_str(value))?;
        if !seen.insert(value) {
            return Err(validation_error(
                "capability_authority.requirements.duplicate",
                format!("capability grant contains duplicate {kind} references"),
            ));
        }
    }
    Ok(())
}

fn validate_identifier(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "capability_authority.identifier.empty",
            format!("{type_name} cannot be empty"),
        ));
    }
    if value.len() > IDENTIFIER_MAX_BYTES {
        return Err(validation_error(
            "capability_authority.identifier.too_long",
            format!("{type_name} exceeds the supported length"),
        ));
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
    {
        return Err(validation_error(
            "capability_authority.identifier.invalid_character",
            format!("{type_name} contains an invalid character"),
        ));
    }
    validate_not_secret_like(type_name, value)
}

fn validate_resource_reference(value: &str) -> Result<(), WorkflowOsError> {
    validate_reference("capability resource reference", value)?;
    if value.starts_with('/')
        || value.contains("..")
        || value.contains("//")
        || value.contains('\\')
        || value.contains(':')
    {
        return Err(validation_error(
            "capability_authority.resource.not_canonical",
            "capability resource references must be bounded canonical references, not raw paths or URLs",
        ));
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
    {
        return Err(validation_error(
            "capability_authority.resource.invalid_character",
            "capability resource reference contains an invalid character",
        ));
    }
    Ok(())
}

fn validate_reference(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(validation_error(
            "capability_authority.reference.empty",
            format!("{type_name} cannot be empty"),
        ));
    }
    if value.len() > RESOURCE_REFERENCE_MAX_BYTES {
        return Err(validation_error(
            "capability_authority.reference.too_long",
            format!("{type_name} exceeds the supported length"),
        ));
    }
    validate_not_secret_like(type_name, value)
}

fn validate_redaction_metadata(redaction: &RedactionMetadata) -> Result<(), WorkflowOsError> {
    if redaction.redacted_fields.len() > REDACTION_MAX_ENTRIES
        || redaction.field_states.len() > REDACTION_MAX_ENTRIES
    {
        return Err(validation_error(
            "capability_authority.redaction.too_many_entries",
            "capability authority redaction metadata contains too many entries",
        ));
    }
    for field in &redaction.redacted_fields {
        validate_redaction_field(field)?;
    }
    for state in &redaction.field_states {
        validate_redaction_field(&state.field)?;
        if state.reason.is_empty() || state.reason.len() > REDACTION_REASON_MAX_BYTES {
            return Err(validation_error(
                "capability_authority.redaction.reason_invalid",
                "capability authority redaction reason is invalid",
            ));
        }
        validate_not_secret_like("capability authority redaction reason", &state.reason)?;
    }
    Ok(())
}

fn validate_redaction_field(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() || value.len() > REDACTION_FIELD_MAX_BYTES {
        return Err(validation_error(
            "capability_authority.redaction.field_invalid",
            "capability authority redaction field is invalid",
        ));
    }
    validate_not_secret_like("capability authority redaction field", value)
}

fn validate_not_secret_like(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    let lowercase = value.to_ascii_lowercase();
    let is_secret_like = lowercase.contains("authorization")
        || lowercase.contains("bearer")
        || lowercase.contains("private_key")
        || lowercase.contains("private-key")
        || lowercase.contains("api_token")
        || lowercase.contains("api-token")
        || lowercase.contains("secret")
        || lowercase.contains("token");
    if is_secret_like {
        return Err(validation_error(
            "capability_authority.secret_like_value",
            format!("{type_name} contains sensitive-looking text"),
        ));
    }
    Ok(())
}

fn validation_error(code: &'static str, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}

struct RedactedRedactionMetadataDebug<'a>(&'a RedactionMetadata);

impl fmt::Debug for RedactedRedactionMetadataDebug<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RedactionMetadata")
            .field("redacted_fields", &self.0.redacted_fields.len())
            .field("field_states", &self.0.field_states.len())
            .finish()
    }
}
