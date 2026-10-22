use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    ActorId, HostedAuthorityRegistryRevision, HostedPrincipalKind, HostedProjectCapability,
    OrganizationId, ProjectId, SpecContentHash, Timestamp, WorkflowOsError,
};

const IDENTIFIER_MAX_BYTES: usize = 128;
const MAX_AUDIENCES: usize = 16;
const MAX_CAPABILITIES: usize = 16;
const MAX_BINDINGS: usize = 128;
const MAX_GRANTS: usize = 128;
const MAX_REVOCATIONS: usize = 128;
const PROVIDER_COMMITMENT_DOMAIN: &str = "workflow-os/collaborative-identity-provider/v1";
const GRANT_COMMITMENT_DOMAIN: &str = "workflow-os/collaborative-project-grant/v1";
const REVOCATION_COMMITMENT_DOMAIN: &str = "workflow-os/collaborative-project-grant-revocation/v1";
const PROJECTION_CLAIM_COMMITMENT_DOMAIN: &str =
    "workflow-os/collaborative-authority-projection-claim/v1";

macro_rules! redacted_string_id {
    ($(#[$meta:meta])* $name:ident, $label:literal) => {
        $(#[$meta])*
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            /// Creates one validated identifier.
            ///
            /// # Errors
            ///
            /// Rejects empty, oversized, malformed, or sensitive-looking values.
            pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
                let value = value.into();
                validate_identifier($label, &value)?;
                Ok(Self(value))
            }

            /// Returns the identifier text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.debug_tuple(stringify!($name)).field(&"[REDACTED]").finish()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = WorkflowOsError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl FromStr for $name {
            type Err = WorkflowOsError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }
    };
}

macro_rules! positive_revision {
    ($(#[$meta:meta])* $name:ident, $code:literal, $message:literal) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
        #[serde(into = "u64")]
        pub struct $name(u64);

        impl $name {
            /// Creates one positive revision.
            ///
            /// # Errors
            ///
            /// Rejects zero or values outside the signed durable storage bound.
            pub fn new(value: u64) -> Result<Self, WorkflowOsError> {
                if value == 0 || value > i64::MAX as u64 {
                    return Err(identity_error($code, $message));
                }
                Ok(Self(value))
            }

            /// Returns the positive revision value.
            #[must_use]
            pub const fn get(self) -> u64 {
                self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(concat!(stringify!($name), "([REDACTED])"))
            }
        }

        impl TryFrom<u64> for $name {
            type Error = WorkflowOsError;

            fn try_from(value: u64) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl From<$name> for u64 {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = u64::deserialize(deserializer)
                    .map_err(|_| serde::de::Error::custom($message))?;
                Self::new(value).map_err(|_| serde::de::Error::custom($message))
            }
        }
    };
}

macro_rules! hash_commitment {
    ($(#[$meta:meta])* $name:ident, $code:literal, $message:literal) => {
        $(#[$meta])*
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(SpecContentHash);

        impl $name {
            /// Creates one validated lowercase SHA-256 commitment.
            ///
            /// # Errors
            ///
            /// Rejects values outside the expected commitment shape.
            pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
                SpecContentHash::new(value.into())
                    .map(Self)
                    .map_err(|_| identity_error($code, $message))
            }

            #[allow(dead_code)]
            fn from_hasher(hasher: Sha256) -> Self {
                Self(content_hash_from_hasher(hasher))
            }

            /// Returns the commitment digest.
            #[must_use]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.debug_tuple(stringify!($name)).field(&"[REDACTED]").finish()
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0.as_str().to_owned()
            }
        }

        impl TryFrom<String> for $name {
            type Error = WorkflowOsError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

redacted_string_id!(
    /// Identifier for one configured collaborative identity provider.
    CollaborativeIdentityProviderId,
    "collaborative identity provider id"
);
redacted_string_id!(
    /// Identifier for one local binding from an opaque external subject to a principal.
    CollaborativePrincipalIdentityBindingId,
    "collaborative principal identity binding id"
);
redacted_string_id!(
    /// Identifier for one immutable collaborative project grant.
    CollaborativeProjectGrantId,
    "collaborative project grant id"
);
redacted_string_id!(
    /// Identifier for one create-only collaborative project-grant revocation.
    CollaborativeProjectGrantRevocationId,
    "collaborative project grant revocation id"
);

positive_revision!(
    /// Version of one collaborative identity-provider registration.
    CollaborativeIdentityProviderRegistrationVersion,
    "collaborative_identity.provider.version.invalid",
    "collaborative identity provider version is invalid"
);
positive_revision!(
    /// Positive lifecycle revision of one collaborative principal.
    CollaborativePrincipalRevision,
    "collaborative_identity.principal.revision.invalid",
    "collaborative principal revision is invalid"
);
positive_revision!(
    /// Positive revision of one immutable collaborative project grant.
    CollaborativeProjectGrantRevision,
    "collaborative_identity.grant.revision.invalid",
    "collaborative project grant revision is invalid"
);

hash_commitment!(
    /// Opaque commitment to one canonical external identity-provider issuer.
    CollaborativeIdentityIssuerCommitment,
    "collaborative_identity.provider.issuer_commitment.invalid",
    "collaborative identity issuer commitment is invalid"
);
hash_commitment!(
    /// Opaque commitment to one accepted identity-provider audience.
    CollaborativeIdentityAudienceCommitment,
    "collaborative_identity.provider.audience_commitment.invalid",
    "collaborative identity audience commitment is invalid"
);
hash_commitment!(
    /// Opaque commitment to one external identity-provider subject.
    CollaborativeExternalSubjectCommitment,
    "collaborative_identity.binding.subject_commitment.invalid",
    "collaborative external subject commitment is invalid"
);
hash_commitment!(
    /// Canonical commitment to one immutable collaborative project grant.
    CollaborativeProjectGrantCommitment,
    "collaborative_identity.grant.commitment.invalid",
    "collaborative project grant commitment is invalid"
);
hash_commitment!(
    /// Canonical commitment to one additive grant revocation.
    CollaborativeProjectGrantRevocationCommitment,
    "collaborative_identity.revocation.commitment.invalid",
    "collaborative project grant revocation commitment is invalid"
);
hash_commitment!(
    /// Canonical commitment to one validated collaborative authority projection claim.
    CollaborativeAuthorityProjectionClaimCommitment,
    "collaborative_identity.projection.commitment.invalid",
    "collaborative authority projection claim commitment is invalid"
);

/// Lifecycle of one configured identity-provider trust registration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborativeIdentityProviderLifecycle {
    /// The provider may participate in current authority resolution.
    Active,
    /// The provider remains historical evidence but cannot establish current authority.
    Disabled,
}

/// Lifecycle of one external-subject-to-principal binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborativePrincipalIdentityBindingLifecycle {
    /// The binding may participate in current authority resolution.
    Active,
    /// The binding is disabled without deleting history.
    Disabled,
    /// A later governed binding superseded this binding.
    Superseded,
}

/// Lifecycle of one collaborative principal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborativePrincipalLifecycle {
    /// The principal may hold current authority.
    Active,
    /// The principal remains historical evidence but cannot hold current authority.
    Disabled,
}

/// Bounded reason category for an additive grant revocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborativeProjectGrantRevocationReason {
    /// Access is no longer required.
    AccessRemoved,
    /// The principal has been disabled.
    PrincipalDisabled,
    /// The grant was replaced by a later governed grant.
    Superseded,
    /// A security response removed the grant.
    SecurityResponse,
    /// An administrative governance decision removed the grant.
    AdministrativeDecision,
}

/// One trusted identity-provider registration inside an organization trust domain.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CollaborativeIdentityProviderRegistration {
    provider_id: CollaborativeIdentityProviderId,
    version: CollaborativeIdentityProviderRegistrationVersion,
    revision: HostedAuthorityRegistryRevision,
    organization_id: OrganizationId,
    issuer_commitment: CollaborativeIdentityIssuerCommitment,
    audience_commitments: Vec<CollaborativeIdentityAudienceCommitment>,
    lifecycle: CollaborativeIdentityProviderLifecycle,
    commitment: SpecContentHash,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CollaborativeIdentityProviderRegistrationWire {
    provider_id: CollaborativeIdentityProviderId,
    version: CollaborativeIdentityProviderRegistrationVersion,
    revision: HostedAuthorityRegistryRevision,
    organization_id: OrganizationId,
    issuer_commitment: CollaborativeIdentityIssuerCommitment,
    audience_commitments: Vec<CollaborativeIdentityAudienceCommitment>,
    lifecycle: CollaborativeIdentityProviderLifecycle,
    commitment: SpecContentHash,
}

impl CollaborativeIdentityProviderRegistration {
    /// Creates one canonical provider registration.
    ///
    /// # Errors
    ///
    /// Rejects empty, oversized, or duplicate audience commitments and sensitive-looking scope.
    pub fn new(
        provider_id: CollaborativeIdentityProviderId,
        version: CollaborativeIdentityProviderRegistrationVersion,
        revision: HostedAuthorityRegistryRevision,
        organization_id: OrganizationId,
        issuer_commitment: CollaborativeIdentityIssuerCommitment,
        mut audience_commitments: Vec<CollaborativeIdentityAudienceCommitment>,
        lifecycle: CollaborativeIdentityProviderLifecycle,
    ) -> Result<Self, WorkflowOsError> {
        validate_not_secret_like(
            "collaborative identity organization",
            organization_id.as_str(),
        )?;
        if audience_commitments.is_empty() || audience_commitments.len() > MAX_AUDIENCES {
            return Err(identity_error(
                "collaborative_identity.provider.audiences.invalid",
                "collaborative identity provider audiences are invalid",
            ));
        }
        audience_commitments.sort();
        if audience_commitments
            .windows(2)
            .any(|pair| pair[0] == pair[1])
        {
            return Err(identity_error(
                "collaborative_identity.provider.audiences.duplicate",
                "collaborative identity provider audiences contain duplicates",
            ));
        }
        let mut registration = Self {
            provider_id,
            version,
            revision,
            organization_id,
            issuer_commitment,
            audience_commitments,
            lifecycle,
            commitment: SpecContentHash::from_text("uninitialized"),
        };
        registration.commitment = registration.compute_commitment();
        Ok(registration)
    }

    /// Returns the provider identity.
    #[must_use]
    pub const fn provider_id(&self) -> &CollaborativeIdentityProviderId {
        &self.provider_id
    }

    /// Returns the provider registration version.
    #[must_use]
    pub const fn version(&self) -> CollaborativeIdentityProviderRegistrationVersion {
        self.version
    }

    /// Returns the authority-registry revision carrying this registration.
    #[must_use]
    pub const fn revision(&self) -> HostedAuthorityRegistryRevision {
        self.revision
    }

    /// Returns the exact organization trust domain.
    #[must_use]
    pub const fn organization_id(&self) -> &OrganizationId {
        &self.organization_id
    }

    /// Returns the opaque issuer commitment.
    #[must_use]
    pub const fn issuer_commitment(&self) -> &CollaborativeIdentityIssuerCommitment {
        &self.issuer_commitment
    }

    /// Returns the sorted accepted-audience commitments.
    #[must_use]
    pub fn audience_commitments(&self) -> &[CollaborativeIdentityAudienceCommitment] {
        &self.audience_commitments
    }

    /// Returns the provider lifecycle.
    #[must_use]
    pub const fn lifecycle(&self) -> CollaborativeIdentityProviderLifecycle {
        self.lifecycle
    }

    /// Returns the canonical registration commitment.
    #[must_use]
    pub const fn commitment(&self) -> &SpecContentHash {
        &self.commitment
    }

    fn compute_commitment(&self) -> SpecContentHash {
        let mut hasher = domain_hasher(PROVIDER_COMMITMENT_DOMAIN);
        hash_text(&mut hasher, "provider_id", self.provider_id.as_str());
        hash_u64(&mut hasher, "version", self.version.get());
        hash_u64(&mut hasher, "revision", self.revision.get());
        hash_text(
            &mut hasher,
            "organization_id",
            self.organization_id.as_str(),
        );
        hash_text(
            &mut hasher,
            "issuer_commitment",
            self.issuer_commitment.as_str(),
        );
        hash_usize(
            &mut hasher,
            "audience_count",
            self.audience_commitments.len(),
        );
        for audience in &self.audience_commitments {
            hash_text(&mut hasher, "audience_commitment", audience.as_str());
        }
        hash_text(
            &mut hasher,
            "lifecycle",
            provider_lifecycle_label(self.lifecycle),
        );
        content_hash_from_hasher(hasher)
    }
}

impl fmt::Debug for CollaborativeIdentityProviderRegistration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CollaborativeIdentityProviderRegistration")
            .field("provider", &"[REDACTED]")
            .field("organization", &"[REDACTED]")
            .field("version", &self.version)
            .field("revision", &self.revision)
            .field("audience_count", &self.audience_commitments.len())
            .field("lifecycle", &self.lifecycle)
            .finish_non_exhaustive()
    }
}

impl<'de> Deserialize<'de> for CollaborativeIdentityProviderRegistration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = CollaborativeIdentityProviderRegistrationWire::deserialize(deserializer)
            .map_err(|_| serde::de::Error::custom("collaborative identity provider is invalid"))?;
        let registration = Self::new(
            wire.provider_id,
            wire.version,
            wire.revision,
            wire.organization_id,
            wire.issuer_commitment,
            wire.audience_commitments,
            wire.lifecycle,
        )
        .map_err(|_| serde::de::Error::custom("collaborative identity provider is invalid"))?;
        if registration.commitment != wire.commitment {
            return Err(serde::de::Error::custom(
                "collaborative identity provider is invalid",
            ));
        }
        Ok(registration)
    }
}

/// Immutable binding from one opaque external subject to one local principal.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CollaborativePrincipalIdentityBinding {
    binding_id: CollaborativePrincipalIdentityBindingId,
    provider_id: CollaborativeIdentityProviderId,
    provider_revision: HostedAuthorityRegistryRevision,
    organization_id: OrganizationId,
    principal_id: ActorId,
    subject_commitment: CollaborativeExternalSubjectCommitment,
    issued_at: Timestamp,
    lifecycle: CollaborativePrincipalIdentityBindingLifecycle,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CollaborativePrincipalIdentityBindingWire {
    binding_id: CollaborativePrincipalIdentityBindingId,
    provider_id: CollaborativeIdentityProviderId,
    provider_revision: HostedAuthorityRegistryRevision,
    organization_id: OrganizationId,
    principal_id: ActorId,
    subject_commitment: CollaborativeExternalSubjectCommitment,
    issued_at: Timestamp,
    lifecycle: CollaborativePrincipalIdentityBindingLifecycle,
}

impl CollaborativePrincipalIdentityBinding {
    /// Creates one opaque subject binding.
    ///
    /// # Errors
    ///
    /// Rejects sensitive-looking identity and scope identifiers.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        binding_id: CollaborativePrincipalIdentityBindingId,
        provider_id: CollaborativeIdentityProviderId,
        provider_revision: HostedAuthorityRegistryRevision,
        organization_id: OrganizationId,
        principal_id: ActorId,
        subject_commitment: CollaborativeExternalSubjectCommitment,
        issued_at: Timestamp,
        lifecycle: CollaborativePrincipalIdentityBindingLifecycle,
    ) -> Result<Self, WorkflowOsError> {
        validate_identity_scope(&organization_id, None, &principal_id)?;
        Ok(Self {
            binding_id,
            provider_id,
            provider_revision,
            organization_id,
            principal_id,
            subject_commitment,
            issued_at,
            lifecycle,
        })
    }

    /// Returns the binding identity.
    #[must_use]
    pub const fn binding_id(&self) -> &CollaborativePrincipalIdentityBindingId {
        &self.binding_id
    }

    /// Returns the provider identity.
    #[must_use]
    pub const fn provider_id(&self) -> &CollaborativeIdentityProviderId {
        &self.provider_id
    }

    /// Returns the provider authority revision used by the binding.
    #[must_use]
    pub const fn provider_revision(&self) -> HostedAuthorityRegistryRevision {
        self.provider_revision
    }

    /// Returns the exact organization.
    #[must_use]
    pub const fn organization_id(&self) -> &OrganizationId {
        &self.organization_id
    }

    /// Returns the local principal identity.
    #[must_use]
    pub const fn principal_id(&self) -> &ActorId {
        &self.principal_id
    }

    /// Returns the opaque external-subject commitment.
    #[must_use]
    pub const fn subject_commitment(&self) -> &CollaborativeExternalSubjectCommitment {
        &self.subject_commitment
    }

    /// Returns when the binding was issued.
    #[must_use]
    pub const fn issued_at(&self) -> Timestamp {
        self.issued_at
    }

    /// Returns the binding lifecycle.
    #[must_use]
    pub const fn lifecycle(&self) -> CollaborativePrincipalIdentityBindingLifecycle {
        self.lifecycle
    }
}

impl fmt::Debug for CollaborativePrincipalIdentityBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CollaborativePrincipalIdentityBinding")
            .field("binding", &"[REDACTED]")
            .field("provider", &"[REDACTED]")
            .field("organization", &"[REDACTED]")
            .field("principal", &"[REDACTED]")
            .field("provider_revision", &self.provider_revision)
            .field("issued_at", &self.issued_at)
            .field("lifecycle", &self.lifecycle)
            .finish_non_exhaustive()
    }
}

impl<'de> Deserialize<'de> for CollaborativePrincipalIdentityBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = CollaborativePrincipalIdentityBindingWire::deserialize(deserializer)
            .map_err(|_| serde::de::Error::custom("collaborative identity binding is invalid"))?;
        Self::new(
            wire.binding_id,
            wire.provider_id,
            wire.provider_revision,
            wire.organization_id,
            wire.principal_id,
            wire.subject_commitment,
            wire.issued_at,
            wire.lifecycle,
        )
        .map_err(|_| serde::de::Error::custom("collaborative identity binding is invalid"))
    }
}

/// Validates one complete set of collaborative identity bindings.
///
/// Active bindings are intentionally one-to-one: one provider subject may identify only one
/// principal, and one principal may have only one active external identity. Historical disabled
/// or superseded bindings remain representable without granting current authority.
///
/// # Errors
///
/// Rejects oversized sets, duplicate binding identities, ambiguous active subjects, or ambiguous
/// active principals.
pub fn validate_collaborative_identity_binding_set(
    bindings: &[CollaborativePrincipalIdentityBinding],
) -> Result<(), WorkflowOsError> {
    if bindings.len() > MAX_BINDINGS {
        return Err(identity_error(
            "collaborative_identity.binding_set.too_large",
            "collaborative identity binding set is too large",
        ));
    }

    let mut binding_ids = BTreeSet::new();
    let mut active_subjects = BTreeSet::new();
    let mut active_principals = BTreeSet::new();
    for binding in bindings {
        if !binding_ids.insert(binding.binding_id().as_str()) {
            return Err(identity_error(
                "collaborative_identity.binding_set.duplicate_binding",
                "collaborative identity binding set contains a duplicate binding",
            ));
        }
        if binding.lifecycle() != CollaborativePrincipalIdentityBindingLifecycle::Active {
            continue;
        }
        if !active_subjects.insert((
            binding.provider_id().as_str(),
            binding.subject_commitment().as_str(),
        )) {
            return Err(identity_error(
                "collaborative_identity.binding_set.subject_ambiguous",
                "collaborative identity binding set contains an ambiguous active subject",
            ));
        }
        if !active_principals.insert((
            binding.organization_id().as_str(),
            binding.principal_id().as_str(),
        )) {
            return Err(identity_error(
                "collaborative_identity.binding_set.principal_ambiguous",
                "collaborative identity binding set contains an ambiguous active principal",
            ));
        }
    }
    Ok(())
}

/// One create-only lifecycle revision for a collaborative principal.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CollaborativePrincipalRecord {
    organization_id: OrganizationId,
    principal_id: ActorId,
    principal_kind: HostedPrincipalKind,
    revision: CollaborativePrincipalRevision,
    effective_at: Timestamp,
    lifecycle: CollaborativePrincipalLifecycle,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CollaborativePrincipalRecordWire {
    organization_id: OrganizationId,
    principal_id: ActorId,
    principal_kind: HostedPrincipalKind,
    revision: CollaborativePrincipalRevision,
    effective_at: Timestamp,
    lifecycle: CollaborativePrincipalLifecycle,
}

impl CollaborativePrincipalRecord {
    /// Creates one principal lifecycle record.
    ///
    /// # Errors
    ///
    /// Rejects sensitive-looking identity and scope identifiers.
    pub fn new(
        organization_id: OrganizationId,
        principal_id: ActorId,
        principal_kind: HostedPrincipalKind,
        revision: CollaborativePrincipalRevision,
        effective_at: Timestamp,
        lifecycle: CollaborativePrincipalLifecycle,
    ) -> Result<Self, WorkflowOsError> {
        validate_identity_scope(&organization_id, None, &principal_id)?;
        Ok(Self {
            organization_id,
            principal_id,
            principal_kind,
            revision,
            effective_at,
            lifecycle,
        })
    }

    /// Returns the organization.
    #[must_use]
    pub const fn organization_id(&self) -> &OrganizationId {
        &self.organization_id
    }

    /// Returns the principal identity.
    #[must_use]
    pub const fn principal_id(&self) -> &ActorId {
        &self.principal_id
    }

    /// Returns the principal kind.
    #[must_use]
    pub const fn principal_kind(&self) -> HostedPrincipalKind {
        self.principal_kind
    }

    /// Returns the lifecycle revision.
    #[must_use]
    pub const fn revision(&self) -> CollaborativePrincipalRevision {
        self.revision
    }

    /// Returns when the lifecycle revision became effective.
    #[must_use]
    pub const fn effective_at(&self) -> Timestamp {
        self.effective_at
    }

    /// Returns the lifecycle.
    #[must_use]
    pub const fn lifecycle(&self) -> CollaborativePrincipalLifecycle {
        self.lifecycle
    }
}

impl fmt::Debug for CollaborativePrincipalRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CollaborativePrincipalRecord")
            .field("organization", &"[REDACTED]")
            .field("principal", &"[REDACTED]")
            .field("principal_kind", &self.principal_kind)
            .field("revision", &self.revision)
            .field("effective_at", &self.effective_at)
            .field("lifecycle", &self.lifecycle)
            .finish_non_exhaustive()
    }
}

impl<'de> Deserialize<'de> for CollaborativePrincipalRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = CollaborativePrincipalRecordWire::deserialize(deserializer)
            .map_err(|_| serde::de::Error::custom("collaborative principal is invalid"))?;
        Self::new(
            wire.organization_id,
            wire.principal_id,
            wire.principal_kind,
            wire.revision,
            wire.effective_at,
            wire.lifecycle,
        )
        .map_err(|_| serde::de::Error::custom("collaborative principal is invalid"))
    }
}

/// Definition used to construct one immutable exact-project grant.
pub struct CollaborativeProjectGrantDefinition {
    /// Stable grant identity.
    pub grant_id: CollaborativeProjectGrantId,
    /// Exact organization scope.
    pub organization_id: OrganizationId,
    /// Exact project scope.
    pub project_id: ProjectId,
    /// Principal receiving authority.
    pub principal_id: ActorId,
    /// Closed project capability set.
    pub capabilities: Vec<HostedProjectCapability>,
    /// Actor that issued the governed grant.
    pub issued_by: ActorId,
    /// Positive grant revision.
    pub revision: CollaborativeProjectGrantRevision,
    /// Deployment authority revision at issuance.
    pub authority_registry_revision: HostedAuthorityRegistryRevision,
    /// Time when the grant becomes effective.
    pub effective_at: Timestamp,
    /// Optional exclusive expiry boundary.
    pub expires_at: Option<Timestamp>,
}

/// One immutable exact-project collaborative authority grant.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CollaborativeProjectGrant {
    grant_id: CollaborativeProjectGrantId,
    organization_id: OrganizationId,
    project_id: ProjectId,
    principal_id: ActorId,
    capabilities: Vec<HostedProjectCapability>,
    issued_by: ActorId,
    revision: CollaborativeProjectGrantRevision,
    authority_registry_revision: HostedAuthorityRegistryRevision,
    effective_at: Timestamp,
    expires_at: Option<Timestamp>,
    commitment: CollaborativeProjectGrantCommitment,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CollaborativeProjectGrantWire {
    grant_id: CollaborativeProjectGrantId,
    organization_id: OrganizationId,
    project_id: ProjectId,
    principal_id: ActorId,
    capabilities: Vec<HostedProjectCapability>,
    issued_by: ActorId,
    revision: CollaborativeProjectGrantRevision,
    authority_registry_revision: HostedAuthorityRegistryRevision,
    effective_at: Timestamp,
    expires_at: Option<Timestamp>,
    commitment: CollaborativeProjectGrantCommitment,
}

impl CollaborativeProjectGrant {
    /// Creates one canonical immutable exact-project grant.
    ///
    /// # Errors
    ///
    /// Rejects invalid scope, empty or duplicate capabilities, and invalid time windows.
    pub fn new(
        mut definition: CollaborativeProjectGrantDefinition,
    ) -> Result<Self, WorkflowOsError> {
        validate_identity_scope(
            &definition.organization_id,
            Some(&definition.project_id),
            &definition.principal_id,
        )?;
        validate_not_secret_like("collaborative grant issuer", definition.issued_by.as_str())?;
        if definition.capabilities.is_empty() || definition.capabilities.len() > MAX_CAPABILITIES {
            return Err(identity_error(
                "collaborative_identity.grant.capabilities.invalid",
                "collaborative project grant capabilities are invalid",
            ));
        }
        definition.capabilities.sort();
        if definition
            .capabilities
            .windows(2)
            .any(|pair| pair[0] == pair[1])
        {
            return Err(identity_error(
                "collaborative_identity.grant.capabilities.duplicate",
                "collaborative project grant contains duplicate capabilities",
            ));
        }
        if definition
            .expires_at
            .is_some_and(|expires_at| expires_at <= definition.effective_at)
        {
            return Err(identity_error(
                "collaborative_identity.grant.time_window.invalid",
                "collaborative project grant time window is invalid",
            ));
        }
        let mut grant = Self {
            grant_id: definition.grant_id,
            organization_id: definition.organization_id,
            project_id: definition.project_id,
            principal_id: definition.principal_id,
            capabilities: definition.capabilities,
            issued_by: definition.issued_by,
            revision: definition.revision,
            authority_registry_revision: definition.authority_registry_revision,
            effective_at: definition.effective_at,
            expires_at: definition.expires_at,
            commitment: CollaborativeProjectGrantCommitment::new(
                SpecContentHash::from_text("uninitialized").as_str(),
            )?,
        };
        grant.commitment = grant.compute_commitment();
        Ok(grant)
    }

    /// Returns the grant identity.
    #[must_use]
    pub const fn grant_id(&self) -> &CollaborativeProjectGrantId {
        &self.grant_id
    }

    /// Returns the organization.
    #[must_use]
    pub const fn organization_id(&self) -> &OrganizationId {
        &self.organization_id
    }

    /// Returns the exact project.
    #[must_use]
    pub const fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    /// Returns the principal.
    #[must_use]
    pub const fn principal_id(&self) -> &ActorId {
        &self.principal_id
    }

    /// Returns the sorted closed capability set.
    #[must_use]
    pub fn capabilities(&self) -> &[HostedProjectCapability] {
        &self.capabilities
    }

    /// Returns the issuing actor reference.
    #[must_use]
    pub const fn issued_by(&self) -> &ActorId {
        &self.issued_by
    }

    /// Returns the grant revision.
    #[must_use]
    pub const fn revision(&self) -> CollaborativeProjectGrantRevision {
        self.revision
    }

    /// Returns the authority registry revision at issuance.
    #[must_use]
    pub const fn authority_registry_revision(&self) -> HostedAuthorityRegistryRevision {
        self.authority_registry_revision
    }

    /// Returns the effective time.
    #[must_use]
    pub const fn effective_at(&self) -> Timestamp {
        self.effective_at
    }

    /// Returns the optional expiry time.
    #[must_use]
    pub const fn expires_at(&self) -> Option<Timestamp> {
        self.expires_at
    }

    /// Returns the canonical grant commitment.
    #[must_use]
    pub const fn commitment(&self) -> &CollaborativeProjectGrantCommitment {
        &self.commitment
    }

    fn compute_commitment(&self) -> CollaborativeProjectGrantCommitment {
        let mut hasher = domain_hasher(GRANT_COMMITMENT_DOMAIN);
        hash_text(&mut hasher, "grant_id", self.grant_id.as_str());
        hash_text(
            &mut hasher,
            "organization_id",
            self.organization_id.as_str(),
        );
        hash_text(&mut hasher, "project_id", self.project_id.as_str());
        hash_text(&mut hasher, "principal_id", self.principal_id.as_str());
        hash_usize(&mut hasher, "capability_count", self.capabilities.len());
        for capability in &self.capabilities {
            hash_text(&mut hasher, "capability", capability_label(*capability));
        }
        hash_text(&mut hasher, "issued_by", self.issued_by.as_str());
        hash_u64(&mut hasher, "grant_revision", self.revision.get());
        hash_u64(
            &mut hasher,
            "authority_registry_revision",
            self.authority_registry_revision.get(),
        );
        hash_text(&mut hasher, "effective_at", &self.effective_at.to_rfc3339());
        hash_optional_timestamp(&mut hasher, "expires_at", self.expires_at);
        CollaborativeProjectGrantCommitment::from_hasher(hasher)
    }
}

impl fmt::Debug for CollaborativeProjectGrant {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CollaborativeProjectGrant")
            .field("grant", &"[REDACTED]")
            .field("organization", &"[REDACTED]")
            .field("project", &"[REDACTED]")
            .field("principal", &"[REDACTED]")
            .field("capabilities", &self.capabilities)
            .field("revision", &self.revision)
            .field(
                "authority_registry_revision",
                &self.authority_registry_revision,
            )
            .field("effective_at", &self.effective_at)
            .field("expires_at", &self.expires_at)
            .finish_non_exhaustive()
    }
}

impl<'de> Deserialize<'de> for CollaborativeProjectGrant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = CollaborativeProjectGrantWire::deserialize(deserializer)
            .map_err(|_| serde::de::Error::custom("collaborative project grant is invalid"))?;
        let expected_commitment = wire.commitment;
        let grant = Self::new(CollaborativeProjectGrantDefinition {
            grant_id: wire.grant_id,
            organization_id: wire.organization_id,
            project_id: wire.project_id,
            principal_id: wire.principal_id,
            capabilities: wire.capabilities,
            issued_by: wire.issued_by,
            revision: wire.revision,
            authority_registry_revision: wire.authority_registry_revision,
            effective_at: wire.effective_at,
            expires_at: wire.expires_at,
        })
        .map_err(|_| serde::de::Error::custom("collaborative project grant is invalid"))?;
        if grant.commitment != expected_commitment {
            return Err(serde::de::Error::custom(
                "collaborative project grant is invalid",
            ));
        }
        Ok(grant)
    }
}

/// Definition used to construct one additive project-grant revocation.
pub struct CollaborativeProjectGrantRevocationDefinition {
    /// Stable revocation identity.
    pub revocation_id: CollaborativeProjectGrantRevocationId,
    /// Exact grant identity being revoked.
    pub grant_id: CollaborativeProjectGrantId,
    /// Exact canonical commitment of the revoked grant.
    pub grant_commitment: CollaborativeProjectGrantCommitment,
    /// Exact organization scope.
    pub organization_id: OrganizationId,
    /// Exact project scope.
    pub project_id: ProjectId,
    /// Exact principal that held the grant.
    pub principal_id: ActorId,
    /// Trusted time when revocation became effective.
    pub revoked_at: Timestamp,
    /// Bounded revocation reason.
    pub reason: CollaborativeProjectGrantRevocationReason,
    /// Actor that performed the governed revocation.
    pub revoked_by: ActorId,
}

/// One create-only revocation of an immutable collaborative project grant.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CollaborativeProjectGrantRevocation {
    revocation_id: CollaborativeProjectGrantRevocationId,
    grant_id: CollaborativeProjectGrantId,
    grant_commitment: CollaborativeProjectGrantCommitment,
    organization_id: OrganizationId,
    project_id: ProjectId,
    principal_id: ActorId,
    revoked_at: Timestamp,
    reason: CollaborativeProjectGrantRevocationReason,
    revoked_by: ActorId,
    commitment: CollaborativeProjectGrantRevocationCommitment,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CollaborativeProjectGrantRevocationWire {
    revocation_id: CollaborativeProjectGrantRevocationId,
    grant_id: CollaborativeProjectGrantId,
    grant_commitment: CollaborativeProjectGrantCommitment,
    organization_id: OrganizationId,
    project_id: ProjectId,
    principal_id: ActorId,
    revoked_at: Timestamp,
    reason: CollaborativeProjectGrantRevocationReason,
    revoked_by: ActorId,
    commitment: CollaborativeProjectGrantRevocationCommitment,
}

impl CollaborativeProjectGrantRevocation {
    /// Creates one canonical additive revocation.
    ///
    /// # Errors
    ///
    /// Rejects invalid or sensitive-looking scope identifiers.
    pub fn new(
        definition: CollaborativeProjectGrantRevocationDefinition,
    ) -> Result<Self, WorkflowOsError> {
        validate_identity_scope(
            &definition.organization_id,
            Some(&definition.project_id),
            &definition.principal_id,
        )?;
        validate_not_secret_like(
            "collaborative revoking actor",
            definition.revoked_by.as_str(),
        )?;
        let mut revocation = Self {
            revocation_id: definition.revocation_id,
            grant_id: definition.grant_id,
            grant_commitment: definition.grant_commitment,
            organization_id: definition.organization_id,
            project_id: definition.project_id,
            principal_id: definition.principal_id,
            revoked_at: definition.revoked_at,
            reason: definition.reason,
            revoked_by: definition.revoked_by,
            commitment: CollaborativeProjectGrantRevocationCommitment::new(
                SpecContentHash::from_text("uninitialized").as_str(),
            )?,
        };
        revocation.commitment = revocation.compute_commitment();
        Ok(revocation)
    }

    /// Returns the revocation identity.
    #[must_use]
    pub const fn revocation_id(&self) -> &CollaborativeProjectGrantRevocationId {
        &self.revocation_id
    }

    /// Returns the exact revoked grant identity.
    #[must_use]
    pub const fn grant_id(&self) -> &CollaborativeProjectGrantId {
        &self.grant_id
    }

    /// Returns the exact revoked grant commitment.
    #[must_use]
    pub const fn grant_commitment(&self) -> &CollaborativeProjectGrantCommitment {
        &self.grant_commitment
    }

    /// Returns the organization.
    #[must_use]
    pub const fn organization_id(&self) -> &OrganizationId {
        &self.organization_id
    }

    /// Returns the project.
    #[must_use]
    pub const fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    /// Returns the principal.
    #[must_use]
    pub const fn principal_id(&self) -> &ActorId {
        &self.principal_id
    }

    /// Returns the revocation time.
    #[must_use]
    pub const fn revoked_at(&self) -> Timestamp {
        self.revoked_at
    }

    /// Returns the bounded revocation reason.
    #[must_use]
    pub const fn reason(&self) -> CollaborativeProjectGrantRevocationReason {
        self.reason
    }

    /// Returns the revoking actor reference.
    #[must_use]
    pub const fn revoked_by(&self) -> &ActorId {
        &self.revoked_by
    }

    /// Returns the canonical revocation commitment.
    #[must_use]
    pub const fn commitment(&self) -> &CollaborativeProjectGrantRevocationCommitment {
        &self.commitment
    }

    fn compute_commitment(&self) -> CollaborativeProjectGrantRevocationCommitment {
        let mut hasher = domain_hasher(REVOCATION_COMMITMENT_DOMAIN);
        hash_text(&mut hasher, "revocation_id", self.revocation_id.as_str());
        hash_text(&mut hasher, "grant_id", self.grant_id.as_str());
        hash_text(
            &mut hasher,
            "grant_commitment",
            self.grant_commitment.as_str(),
        );
        hash_text(
            &mut hasher,
            "organization_id",
            self.organization_id.as_str(),
        );
        hash_text(&mut hasher, "project_id", self.project_id.as_str());
        hash_text(&mut hasher, "principal_id", self.principal_id.as_str());
        hash_text(&mut hasher, "revoked_at", &self.revoked_at.to_rfc3339());
        hash_text(&mut hasher, "reason", revocation_reason_label(self.reason));
        hash_text(&mut hasher, "revoked_by", self.revoked_by.as_str());
        CollaborativeProjectGrantRevocationCommitment::from_hasher(hasher)
    }
}

impl fmt::Debug for CollaborativeProjectGrantRevocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CollaborativeProjectGrantRevocation")
            .field("revocation", &"[REDACTED]")
            .field("grant", &"[REDACTED]")
            .field("organization", &"[REDACTED]")
            .field("project", &"[REDACTED]")
            .field("principal", &"[REDACTED]")
            .field("revoked_at", &self.revoked_at)
            .field("reason", &self.reason)
            .finish_non_exhaustive()
    }
}

impl<'de> Deserialize<'de> for CollaborativeProjectGrantRevocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = CollaborativeProjectGrantRevocationWire::deserialize(deserializer)
            .map_err(|_| serde::de::Error::custom("collaborative grant revocation is invalid"))?;
        let expected_commitment = wire.commitment;
        let revocation = Self::new(CollaborativeProjectGrantRevocationDefinition {
            revocation_id: wire.revocation_id,
            grant_id: wire.grant_id,
            grant_commitment: wire.grant_commitment,
            organization_id: wire.organization_id,
            project_id: wire.project_id,
            principal_id: wire.principal_id,
            revoked_at: wire.revoked_at,
            reason: wire.reason,
            revoked_by: wire.revoked_by,
        })
        .map_err(|_| serde::de::Error::custom("collaborative grant revocation is invalid"))?;
        if revocation.commitment != expected_commitment {
            return Err(serde::de::Error::custom(
                "collaborative grant revocation is invalid",
            ));
        }
        Ok(revocation)
    }
}

/// Exact replay outcome for an additive grant revocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollaborativeProjectGrantRevocationReconciliation {
    /// The candidate is exactly the existing canonical revocation.
    ExactReplay,
}

/// Reconciles an additive revocation without allowing replacement.
///
/// # Errors
///
/// Returns a conflict when the same revocation or grant identity carries different content.
pub fn reconcile_collaborative_project_grant_revocation(
    existing: &CollaborativeProjectGrantRevocation,
    candidate: &CollaborativeProjectGrantRevocation,
) -> Result<CollaborativeProjectGrantRevocationReconciliation, WorkflowOsError> {
    if existing == candidate {
        return Ok(CollaborativeProjectGrantRevocationReconciliation::ExactReplay);
    }
    Err(identity_error(
        "collaborative_identity.revocation.conflict",
        "collaborative project grant revocation conflicts with existing state",
    ))
}

/// Stable active-grant reference inside a validated non-authorizing claim.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollaborativeActiveGrantReference {
    grant_id: CollaborativeProjectGrantId,
    commitment: CollaborativeProjectGrantCommitment,
}

impl CollaborativeActiveGrantReference {
    /// Returns the grant identity.
    #[must_use]
    pub const fn grant_id(&self) -> &CollaborativeProjectGrantId {
        &self.grant_id
    }

    /// Returns the grant commitment.
    #[must_use]
    pub const fn commitment(&self) -> &CollaborativeProjectGrantCommitment {
        &self.commitment
    }
}

impl fmt::Debug for CollaborativeActiveGrantReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CollaborativeActiveGrantReference")
            .field("grant", &"[REDACTED]")
            .field("commitment", &"[REDACTED]")
            .finish()
    }
}

/// Caller-supplied inputs for validating one collaborative authority projection claim.
pub struct CollaborativeAuthorityProjectionClaimInput<'a> {
    /// Current provider registration.
    pub provider: &'a CollaborativeIdentityProviderRegistration,
    /// Current opaque subject binding.
    pub binding: &'a CollaborativePrincipalIdentityBinding,
    /// Current principal lifecycle record.
    pub principal: &'a CollaborativePrincipalRecord,
    /// Exact organization requested by the authority consumer.
    pub organization_id: &'a OrganizationId,
    /// Exact project requested by the authority consumer.
    pub project_id: &'a ProjectId,
    /// Authority revision asserted by the caller.
    pub authority_registry_revision: HostedAuthorityRegistryRevision,
    /// Bounded grant facts asserted by the caller.
    pub grants: &'a [CollaborativeProjectGrant],
    /// Bounded revocation facts asserted by the caller.
    pub revocations: &'a [CollaborativeProjectGrantRevocation],
    /// Point-in-time evaluation value asserted by the caller.
    pub evaluated_at: Timestamp,
}

/// Fixed authority posture for a caller-supplied projection claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborativeAuthorityProjectionClaimPosture {
    /// The claim is internally valid but cannot authorize an operation.
    NonAuthorizing,
}

/// Internally validated, explicitly non-authorizing authority projection claim.
///
/// The claim proves only that the supplied facts are mutually coherent. It does not prove that
/// they came from an authoritative source, form a complete current fact set, or use trusted time.
/// Runtime authorization must not consume this value as current authority.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CollaborativeAuthorityProjectionClaim {
    posture: CollaborativeAuthorityProjectionClaimPosture,
    provider_id: CollaborativeIdentityProviderId,
    provider_revision: HostedAuthorityRegistryRevision,
    binding_id: CollaborativePrincipalIdentityBindingId,
    principal_id: ActorId,
    principal_revision: CollaborativePrincipalRevision,
    organization_id: OrganizationId,
    project_id: ProjectId,
    authority_registry_revision: HostedAuthorityRegistryRevision,
    active_grants: Vec<CollaborativeActiveGrantReference>,
    capabilities: Vec<HostedProjectCapability>,
    evaluated_at: Timestamp,
    commitment: CollaborativeAuthorityProjectionClaimCommitment,
}

/// Untrusted serialized shape corresponding to an authority projection claim.
///
/// This type intentionally has no conversion into authoritative runtime state. Callers may only
/// validate asserted facts into a non-authorizing claim through
/// [`validate_collaborative_authority_projection_claim`].
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnverifiedCollaborativeAuthorityProjectionClaim {
    posture: CollaborativeAuthorityProjectionClaimPosture,
    provider_id: CollaborativeIdentityProviderId,
    provider_revision: HostedAuthorityRegistryRevision,
    binding_id: CollaborativePrincipalIdentityBindingId,
    principal_id: ActorId,
    principal_revision: CollaborativePrincipalRevision,
    organization_id: OrganizationId,
    project_id: ProjectId,
    authority_registry_revision: HostedAuthorityRegistryRevision,
    active_grants: Vec<CollaborativeActiveGrantReference>,
    capabilities: Vec<HostedProjectCapability>,
    evaluated_at: Timestamp,
    commitment: CollaborativeAuthorityProjectionClaimCommitment,
}

impl CollaborativeAuthorityProjectionClaim {
    /// Returns the fixed non-authorizing posture of this caller-supplied claim.
    #[must_use]
    pub const fn posture(&self) -> CollaborativeAuthorityProjectionClaimPosture {
        self.posture
    }

    /// Returns the provider identity.
    #[must_use]
    pub const fn provider_id(&self) -> &CollaborativeIdentityProviderId {
        &self.provider_id
    }

    /// Returns the principal identity.
    #[must_use]
    pub const fn principal_id(&self) -> &ActorId {
        &self.principal_id
    }

    /// Returns the exact organization.
    #[must_use]
    pub const fn organization_id(&self) -> &OrganizationId {
        &self.organization_id
    }

    /// Returns the exact project.
    #[must_use]
    pub const fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    /// Returns the current authority-registry revision.
    #[must_use]
    pub const fn authority_registry_revision(&self) -> HostedAuthorityRegistryRevision {
        self.authority_registry_revision
    }

    /// Returns the canonical active-grant references.
    #[must_use]
    pub fn active_grants(&self) -> &[CollaborativeActiveGrantReference] {
        &self.active_grants
    }

    /// Returns the effective closed project capability set.
    #[must_use]
    pub fn capabilities(&self) -> &[HostedProjectCapability] {
        &self.capabilities
    }

    /// Returns whether the validated claim contains one project capability assertion.
    ///
    /// Membership is descriptive only and does not authorize an operation.
    #[must_use]
    pub fn contains_capability(&self, capability: HostedProjectCapability) -> bool {
        self.capabilities.binary_search(&capability).is_ok()
    }

    /// Returns when the point-in-time projection was evaluated.
    #[must_use]
    pub const fn evaluated_at(&self) -> Timestamp {
        self.evaluated_at
    }

    /// Returns the canonical projection commitment.
    #[must_use]
    pub const fn commitment(&self) -> &CollaborativeAuthorityProjectionClaimCommitment {
        &self.commitment
    }

    fn compute_commitment(&self) -> CollaborativeAuthorityProjectionClaimCommitment {
        let mut hasher = domain_hasher(PROJECTION_CLAIM_COMMITMENT_DOMAIN);
        hash_text(&mut hasher, "posture", "non_authorizing");
        hash_text(&mut hasher, "provider_id", self.provider_id.as_str());
        hash_u64(
            &mut hasher,
            "provider_revision",
            self.provider_revision.get(),
        );
        hash_text(&mut hasher, "binding_id", self.binding_id.as_str());
        hash_text(&mut hasher, "principal_id", self.principal_id.as_str());
        hash_u64(
            &mut hasher,
            "principal_revision",
            self.principal_revision.get(),
        );
        hash_text(
            &mut hasher,
            "organization_id",
            self.organization_id.as_str(),
        );
        hash_text(&mut hasher, "project_id", self.project_id.as_str());
        hash_u64(
            &mut hasher,
            "authority_registry_revision",
            self.authority_registry_revision.get(),
        );
        hash_usize(&mut hasher, "active_grant_count", self.active_grants.len());
        for grant in &self.active_grants {
            hash_text(&mut hasher, "grant_id", grant.grant_id.as_str());
            hash_text(&mut hasher, "grant_commitment", grant.commitment.as_str());
        }
        hash_usize(&mut hasher, "capability_count", self.capabilities.len());
        for capability in &self.capabilities {
            hash_text(&mut hasher, "capability", capability_label(*capability));
        }
        hash_text(&mut hasher, "evaluated_at", &self.evaluated_at.to_rfc3339());
        CollaborativeAuthorityProjectionClaimCommitment::from_hasher(hasher)
    }
}

impl fmt::Debug for CollaborativeAuthorityProjectionClaim {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CollaborativeAuthorityProjectionClaim")
            .field("posture", &self.posture)
            .field("provider", &"[REDACTED]")
            .field("binding", &"[REDACTED]")
            .field("principal", &"[REDACTED]")
            .field("organization", &"[REDACTED]")
            .field("project", &"[REDACTED]")
            .field(
                "authority_registry_revision",
                &self.authority_registry_revision,
            )
            .field("active_grant_count", &self.active_grants.len())
            .field("capabilities", &self.capabilities)
            .field("evaluated_at", &self.evaluated_at)
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for UnverifiedCollaborativeAuthorityProjectionClaim {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UnverifiedCollaborativeAuthorityProjectionClaim")
            .field("posture", &self.posture)
            .field("provider", &"[REDACTED]")
            .field("binding", &"[REDACTED]")
            .field("principal", &"[REDACTED]")
            .field("organization", &"[REDACTED]")
            .field("project", &"[REDACTED]")
            .field("active_grant_count", &self.active_grants.len())
            .field("capability_count", &self.capabilities.len())
            .finish_non_exhaustive()
    }
}

/// Validates one caller-supplied, non-authorizing authority projection claim.
///
/// This function establishes internal coherence only. It does not establish authoritative source
/// provenance, completeness of current facts, or trusted time. Its result must not authorize a
/// runtime operation.
///
/// # Errors
///
/// Fails closed for inactive or mismatched identity facts, stale authority revisions, ambiguous
/// records, invalid revocations, or absence of one active exact-project grant.
pub fn validate_collaborative_authority_projection_claim(
    input: &CollaborativeAuthorityProjectionClaimInput<'_>,
) -> Result<CollaborativeAuthorityProjectionClaim, WorkflowOsError> {
    validate_projection_identity(input)?;
    if input.grants.len() > MAX_GRANTS || input.revocations.len() > MAX_REVOCATIONS {
        return Err(identity_error(
            "collaborative_identity.projection.input_too_large",
            "collaborative authority projection claim input is too large",
        ));
    }

    let mut grants_by_id = BTreeMap::new();
    for grant in input.grants {
        if grants_by_id.insert(grant.grant_id(), grant).is_some() {
            return Err(identity_error(
                "collaborative_identity.projection.duplicate_grant",
                "collaborative authority projection claim contains duplicate grants",
            ));
        }
        if grant.authority_registry_revision() > input.authority_registry_revision {
            return Err(identity_error(
                "collaborative_identity.projection.authority_revision.stale",
                "collaborative authority projection claim revision is stale",
            ));
        }
        validate_projection_grant_scope(input, grant)?;
    }

    let mut revoked_grants = BTreeSet::new();
    let mut revocation_ids = BTreeSet::new();
    for revocation in input.revocations {
        if !revocation_ids.insert(revocation.revocation_id())
            || !revoked_grants.insert(revocation.grant_id())
        {
            return Err(identity_error(
                "collaborative_identity.projection.revocation_ambiguous",
                "collaborative authority projection claim contains ambiguous revocation state",
            ));
        }
        let Some(grant) = grants_by_id.get(revocation.grant_id()) else {
            return Err(identity_error(
                "collaborative_identity.projection.revocation_orphaned",
                "collaborative authority projection claim contains an orphaned revocation",
            ));
        };
        if revocation.grant_commitment() != grant.commitment()
            || revocation.organization_id() != grant.organization_id()
            || revocation.project_id() != grant.project_id()
            || revocation.principal_id() != grant.principal_id()
            || revocation.revoked_at() < grant.effective_at()
            || revocation.revoked_at() > input.evaluated_at
        {
            return Err(identity_error(
                "collaborative_identity.projection.revocation_invalid",
                "collaborative authority projection claim contains an invalid revocation",
            ));
        }
    }

    let mut active_grants = Vec::new();
    let mut capabilities = BTreeSet::new();
    for grant in input.grants {
        if grant.effective_at() > input.evaluated_at
            || grant
                .expires_at()
                .is_some_and(|expires_at| expires_at <= input.evaluated_at)
            || revoked_grants.contains(grant.grant_id())
        {
            continue;
        }
        active_grants.push(CollaborativeActiveGrantReference {
            grant_id: grant.grant_id().clone(),
            commitment: grant.commitment().clone(),
        });
        capabilities.extend(grant.capabilities().iter().copied());
    }
    if active_grants.is_empty() {
        return Err(identity_error(
            "collaborative_identity.projection.active_grant_missing",
            "collaborative authority projection claim requires an active exact-project grant",
        ));
    }
    active_grants.sort_by(|left, right| left.grant_id.cmp(&right.grant_id));
    let mut projection = CollaborativeAuthorityProjectionClaim {
        posture: CollaborativeAuthorityProjectionClaimPosture::NonAuthorizing,
        provider_id: input.provider.provider_id().clone(),
        provider_revision: input.provider.revision(),
        binding_id: input.binding.binding_id().clone(),
        principal_id: input.principal.principal_id().clone(),
        principal_revision: input.principal.revision(),
        organization_id: input.organization_id.clone(),
        project_id: input.project_id.clone(),
        authority_registry_revision: input.authority_registry_revision,
        active_grants,
        capabilities: capabilities.into_iter().collect(),
        evaluated_at: input.evaluated_at,
        commitment: CollaborativeAuthorityProjectionClaimCommitment::new(
            SpecContentHash::from_text("uninitialized").as_str(),
        )?,
    };
    projection.commitment = projection.compute_commitment();
    Ok(projection)
}

fn validate_projection_identity(
    input: &CollaborativeAuthorityProjectionClaimInput<'_>,
) -> Result<(), WorkflowOsError> {
    if input.provider.lifecycle() != CollaborativeIdentityProviderLifecycle::Active {
        return Err(identity_error(
            "collaborative_identity.projection.provider_inactive",
            "collaborative authority projection claim requires an active provider",
        ));
    }
    if input.binding.lifecycle() != CollaborativePrincipalIdentityBindingLifecycle::Active {
        return Err(identity_error(
            "collaborative_identity.projection.binding_inactive",
            "collaborative authority projection claim requires an active identity binding",
        ));
    }
    if input.principal.lifecycle() != CollaborativePrincipalLifecycle::Active {
        return Err(identity_error(
            "collaborative_identity.projection.principal_inactive",
            "collaborative authority projection claim requires an active principal",
        ));
    }
    if input.provider.organization_id() != input.organization_id
        || input.binding.organization_id() != input.organization_id
        || input.principal.organization_id() != input.organization_id
        || input.binding.provider_id() != input.provider.provider_id()
        || input.binding.provider_revision() != input.provider.revision()
        || input.binding.principal_id() != input.principal.principal_id()
    {
        return Err(identity_error(
            "collaborative_identity.projection.identity_mismatch",
            "collaborative authority projection claim identity context does not match",
        ));
    }
    if input.provider.revision() > input.authority_registry_revision
        || input.binding.provider_revision() > input.authority_registry_revision
    {
        return Err(identity_error(
            "collaborative_identity.projection.authority_revision.stale",
            "collaborative authority projection claim revision is stale",
        ));
    }
    if input.binding.issued_at() > input.evaluated_at
        || input.principal.effective_at() > input.evaluated_at
    {
        return Err(identity_error(
            "collaborative_identity.projection.fact_in_future",
            "collaborative authority projection claim contains a future identity fact",
        ));
    }
    Ok(())
}

fn validate_projection_grant_scope(
    input: &CollaborativeAuthorityProjectionClaimInput<'_>,
    grant: &CollaborativeProjectGrant,
) -> Result<(), WorkflowOsError> {
    if grant.organization_id() != input.organization_id
        || grant.project_id() != input.project_id
        || grant.principal_id() != input.principal.principal_id()
    {
        return Err(identity_error(
            "collaborative_identity.projection.grant_scope_mismatch",
            "collaborative authority projection claim contains a mismatched grant scope",
        ));
    }
    Ok(())
}

fn validate_identity_scope(
    organization_id: &OrganizationId,
    project_id: Option<&ProjectId>,
    principal_id: &ActorId,
) -> Result<(), WorkflowOsError> {
    validate_not_secret_like(
        "collaborative identity organization",
        organization_id.as_str(),
    )?;
    if let Some(project_id) = project_id {
        validate_not_secret_like("collaborative identity project", project_id.as_str())?;
    }
    validate_not_secret_like("collaborative identity principal", principal_id.as_str())
}

fn validate_identifier(label: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() || value.len() > IDENTIFIER_MAX_BYTES {
        return Err(identity_error(
            "collaborative_identity.identifier.invalid",
            "collaborative identity identifier is invalid",
        ));
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
    {
        return Err(identity_error(
            "collaborative_identity.identifier.invalid",
            "collaborative identity identifier is invalid",
        ));
    }
    validate_not_secret_like(label, value)
}

fn validate_not_secret_like(_label: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    let lowercase = value.to_ascii_lowercase();
    if lowercase.contains("authorization")
        || lowercase.contains("bearer")
        || lowercase.contains("private_key")
        || lowercase.contains("private-key")
        || lowercase.contains("api_token")
        || lowercase.contains("api-token")
        || lowercase.contains("secret")
        || lowercase.contains("token")
    {
        return Err(WorkflowOsError::security(
            "collaborative_identity.secret_like_value",
            "collaborative identity input contains sensitive-looking text",
        ));
    }
    Ok(())
}

fn identity_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}

fn provider_lifecycle_label(lifecycle: CollaborativeIdentityProviderLifecycle) -> &'static str {
    match lifecycle {
        CollaborativeIdentityProviderLifecycle::Active => "active",
        CollaborativeIdentityProviderLifecycle::Disabled => "disabled",
    }
}

fn revocation_reason_label(reason: CollaborativeProjectGrantRevocationReason) -> &'static str {
    match reason {
        CollaborativeProjectGrantRevocationReason::AccessRemoved => "access_removed",
        CollaborativeProjectGrantRevocationReason::PrincipalDisabled => "principal_disabled",
        CollaborativeProjectGrantRevocationReason::Superseded => "superseded",
        CollaborativeProjectGrantRevocationReason::SecurityResponse => "security_response",
        CollaborativeProjectGrantRevocationReason::AdministrativeDecision => {
            "administrative_decision"
        }
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

fn domain_hasher(domain: &str) -> Sha256 {
    let mut hasher = Sha256::new();
    hash_text(&mut hasher, "domain", domain);
    hasher
}

fn hash_text(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update(label.len().to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(value.len().to_be_bytes());
    hasher.update(value.as_bytes());
}

fn hash_u64(hasher: &mut Sha256, label: &str, value: u64) {
    hash_text(hasher, label, &value.to_string());
}

fn hash_usize(hasher: &mut Sha256, label: &str, value: usize) {
    hash_text(hasher, label, &value.to_string());
}

fn hash_optional_timestamp(hasher: &mut Sha256, label: &str, value: Option<Timestamp>) {
    match value {
        Some(value) => {
            hash_text(hasher, &format!("{label}_posture"), "present");
            hash_text(hasher, label, &value.to_rfc3339());
        }
        None => hash_text(hasher, &format!("{label}_posture"), "absent"),
    }
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

fn content_hash_from_hasher(hasher: Sha256) -> SpecContentHash {
    match SpecContentHash::new(hex_lower(&hasher.finalize())) {
        Ok(hash) => hash,
        Err(_) => unreachable!("sha256 digest always has the validated content-hash shape"),
    }
}
