#![allow(clippy::expect_used)]

//! Focused tests for collaborative identity lifecycle and point-in-time authority projection.

use serde_json::Value;
use workflow_core::{
    project_collaborative_current_authority, reconcile_collaborative_project_grant_revocation,
    validate_collaborative_identity_binding_set, ActorId,
    CollaborativeCurrentAuthorityProjectionInput, CollaborativeExternalSubjectCommitment,
    CollaborativeIdentityAudienceCommitment, CollaborativeIdentityIssuerCommitment,
    CollaborativeIdentityProviderId, CollaborativeIdentityProviderLifecycle,
    CollaborativeIdentityProviderRegistration, CollaborativeIdentityProviderRegistrationVersion,
    CollaborativePrincipalIdentityBinding, CollaborativePrincipalIdentityBindingId,
    CollaborativePrincipalIdentityBindingLifecycle, CollaborativePrincipalLifecycle,
    CollaborativePrincipalRecord, CollaborativePrincipalRevision, CollaborativeProjectGrant,
    CollaborativeProjectGrantDefinition, CollaborativeProjectGrantId,
    CollaborativeProjectGrantRevision, CollaborativeProjectGrantRevocation,
    CollaborativeProjectGrantRevocationDefinition, CollaborativeProjectGrantRevocationId,
    CollaborativeProjectGrantRevocationReason, CollaborativeProjectGrantRevocationReconciliation,
    HostedAuthorityRegistryRevision, HostedPrincipalKind, HostedProjectCapability, OrganizationId,
    ProjectId, SpecContentHash, Timestamp, UnverifiedCollaborativeCurrentAuthorityProjection,
    WorkflowOsErrorKind,
};

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).expect("timestamp")
}

fn hash(label: &str) -> String {
    SpecContentHash::from_text(label).as_str().to_owned()
}

fn provider(
    lifecycle: CollaborativeIdentityProviderLifecycle,
) -> CollaborativeIdentityProviderRegistration {
    CollaborativeIdentityProviderRegistration::new(
        CollaborativeIdentityProviderId::new("provider/corporate").expect("provider id"),
        CollaborativeIdentityProviderRegistrationVersion::new(1).expect("version"),
        HostedAuthorityRegistryRevision::new(7).expect("authority revision"),
        OrganizationId::new("org/acme").expect("organization"),
        CollaborativeIdentityIssuerCommitment::new(hash("issuer")).expect("issuer commitment"),
        vec![
            CollaborativeIdentityAudienceCommitment::new(hash("audience-b")).expect("audience"),
            CollaborativeIdentityAudienceCommitment::new(hash("audience-a")).expect("audience"),
        ],
        lifecycle,
    )
    .expect("provider")
}

fn binding(
    lifecycle: CollaborativePrincipalIdentityBindingLifecycle,
) -> CollaborativePrincipalIdentityBinding {
    CollaborativePrincipalIdentityBinding::new(
        CollaborativePrincipalIdentityBindingId::new("binding/alice").expect("binding id"),
        CollaborativeIdentityProviderId::new("provider/corporate").expect("provider id"),
        HostedAuthorityRegistryRevision::new(7).expect("authority revision"),
        OrganizationId::new("org/acme").expect("organization"),
        ActorId::new("user/alice").expect("principal"),
        CollaborativeExternalSubjectCommitment::new(hash("subject-alice"))
            .expect("subject commitment"),
        timestamp("2026-10-20T10:00:00Z"),
        lifecycle,
    )
    .expect("binding")
}

fn principal(lifecycle: CollaborativePrincipalLifecycle) -> CollaborativePrincipalRecord {
    CollaborativePrincipalRecord::new(
        OrganizationId::new("org/acme").expect("organization"),
        ActorId::new("user/alice").expect("principal"),
        HostedPrincipalKind::Human,
        CollaborativePrincipalRevision::new(3).expect("principal revision"),
        timestamp("2026-10-20T10:00:00Z"),
        lifecycle,
    )
    .expect("principal")
}

fn grant(
    id: &str,
    capabilities: Vec<HostedProjectCapability>,
    effective_at: &str,
    expires_at: Option<&str>,
) -> CollaborativeProjectGrant {
    CollaborativeProjectGrant::new(CollaborativeProjectGrantDefinition {
        grant_id: CollaborativeProjectGrantId::new(id).expect("grant id"),
        organization_id: OrganizationId::new("org/acme").expect("organization"),
        project_id: ProjectId::new("project/alpha").expect("project"),
        principal_id: ActorId::new("user/alice").expect("principal"),
        capabilities,
        issued_by: ActorId::new("user/admin").expect("issuer"),
        revision: CollaborativeProjectGrantRevision::new(1).expect("grant revision"),
        authority_registry_revision: HostedAuthorityRegistryRevision::new(7)
            .expect("authority revision"),
        effective_at: timestamp(effective_at),
        expires_at: expires_at.map(timestamp),
    })
    .expect("grant")
}

fn revocation(
    grant: &CollaborativeProjectGrant,
    id: &str,
    revoked_at: &str,
) -> CollaborativeProjectGrantRevocation {
    CollaborativeProjectGrantRevocation::new(CollaborativeProjectGrantRevocationDefinition {
        revocation_id: CollaborativeProjectGrantRevocationId::new(id).expect("revocation id"),
        grant_id: grant.grant_id().clone(),
        grant_commitment: grant.commitment().clone(),
        organization_id: grant.organization_id().clone(),
        project_id: grant.project_id().clone(),
        principal_id: grant.principal_id().clone(),
        revoked_at: timestamp(revoked_at),
        reason: CollaborativeProjectGrantRevocationReason::AccessRemoved,
        revoked_by: ActorId::new("user/admin").expect("revoking actor"),
    })
    .expect("revocation")
}

#[test]
fn provider_registration_is_canonical_round_trips_and_rejects_tampering() {
    let provider = provider(CollaborativeIdentityProviderLifecycle::Active);
    assert!(provider.audience_commitments()[0] < provider.audience_commitments()[1]);

    let serialized = serde_json::to_value(&provider).expect("serialize provider");
    let restored: CollaborativeIdentityProviderRegistration =
        serde_json::from_value(serialized.clone()).expect("restore provider");
    assert_eq!(restored, provider);

    let mut tampered = serialized;
    tampered["revision"] = Value::from(8);
    let error = serde_json::from_value::<CollaborativeIdentityProviderRegistration>(tampered)
        .expect_err("tampered commitment must fail");
    assert_eq!(
        error.to_string(),
        "collaborative identity provider is invalid"
    );
}

#[test]
fn duplicate_provider_audiences_and_invalid_revisions_fail_closed() {
    let audience = CollaborativeIdentityAudienceCommitment::new(hash("audience"))
        .expect("audience commitment");
    let error = CollaborativeIdentityProviderRegistration::new(
        CollaborativeIdentityProviderId::new("provider/corporate").expect("provider id"),
        CollaborativeIdentityProviderRegistrationVersion::new(1).expect("version"),
        HostedAuthorityRegistryRevision::new(1).expect("authority revision"),
        OrganizationId::new("org/acme").expect("organization"),
        CollaborativeIdentityIssuerCommitment::new(hash("issuer")).expect("issuer"),
        vec![audience.clone(), audience],
        CollaborativeIdentityProviderLifecycle::Active,
    )
    .expect_err("duplicate audience must fail");
    assert_eq!(
        error.code(),
        "collaborative_identity.provider.audiences.duplicate"
    );

    let revision = CollaborativePrincipalRevision::new(0).expect_err("zero revision must fail");
    assert_eq!(
        revision.code(),
        "collaborative_identity.principal.revision.invalid"
    );
}

#[test]
fn bindings_and_principals_preserve_lifecycle_without_raw_subjects() {
    let binding = binding(CollaborativePrincipalIdentityBindingLifecycle::Superseded);
    let principal = principal(CollaborativePrincipalLifecycle::Disabled);
    let serialized = serde_json::to_string(&(binding.clone(), principal.clone()))
        .expect("serialize lifecycle records");
    assert!(!serialized.contains("subject-alice"));
    assert!(!serialized.contains("alice@example"));
    assert_eq!(
        serde_json::from_str::<(
            CollaborativePrincipalIdentityBinding,
            CollaborativePrincipalRecord,
        )>(&serialized)
        .expect("restore lifecycle records"),
        (binding, principal)
    );
}

#[test]
fn secret_like_identity_values_are_rejected_without_leakage() {
    let secret = "user/api-token-sensitive-value";
    let error = CollaborativePrincipalRecord::new(
        OrganizationId::new("org/acme").expect("organization"),
        ActorId::new(secret).expect("actor id shape"),
        HostedPrincipalKind::Human,
        CollaborativePrincipalRevision::new(1).expect("revision"),
        timestamp("2026-10-20T10:00:00Z"),
        CollaborativePrincipalLifecycle::Active,
    )
    .expect_err("secret-like principal must fail");
    assert_eq!(error.kind(), WorkflowOsErrorKind::Security);
    assert_eq!(error.code(), "collaborative_identity.secret_like_value");
    assert!(!error.to_string().contains(secret));
}

#[test]
fn project_grant_is_canonical_and_rejects_duplicate_capabilities_and_time_reversal() {
    let grant = grant(
        "grant/alpha",
        vec![
            HostedProjectCapability::ApprovalDecide,
            HostedProjectCapability::RunRead,
        ],
        "2026-10-20T10:00:00Z",
        Some("2026-10-21T10:00:00Z"),
    );
    assert_eq!(
        grant.capabilities(),
        &[
            HostedProjectCapability::RunRead,
            HostedProjectCapability::ApprovalDecide
        ]
    );
    let serialized = serde_json::to_value(&grant).expect("serialize grant");
    assert_eq!(
        serde_json::from_value::<CollaborativeProjectGrant>(serialized.clone())
            .expect("restore grant"),
        grant
    );

    let duplicate = CollaborativeProjectGrant::new(CollaborativeProjectGrantDefinition {
        grant_id: CollaborativeProjectGrantId::new("grant/duplicate").expect("grant id"),
        organization_id: OrganizationId::new("org/acme").expect("organization"),
        project_id: ProjectId::new("project/alpha").expect("project"),
        principal_id: ActorId::new("user/alice").expect("principal"),
        capabilities: vec![
            HostedProjectCapability::RunRead,
            HostedProjectCapability::RunRead,
        ],
        issued_by: ActorId::new("user/admin").expect("issuer"),
        revision: CollaborativeProjectGrantRevision::new(1).expect("revision"),
        authority_registry_revision: HostedAuthorityRegistryRevision::new(7)
            .expect("authority revision"),
        effective_at: timestamp("2026-10-20T10:00:00Z"),
        expires_at: None,
    })
    .expect_err("duplicate capabilities must fail");
    assert_eq!(
        duplicate.code(),
        "collaborative_identity.grant.capabilities.duplicate"
    );

    let reversed = CollaborativeProjectGrant::new(CollaborativeProjectGrantDefinition {
        grant_id: CollaborativeProjectGrantId::new("grant/reversed").expect("grant id"),
        organization_id: OrganizationId::new("org/acme").expect("organization"),
        project_id: ProjectId::new("project/alpha").expect("project"),
        principal_id: ActorId::new("user/alice").expect("principal"),
        capabilities: vec![HostedProjectCapability::RunRead],
        issued_by: ActorId::new("user/admin").expect("issuer"),
        revision: CollaborativeProjectGrantRevision::new(1).expect("revision"),
        authority_registry_revision: HostedAuthorityRegistryRevision::new(7)
            .expect("authority revision"),
        effective_at: timestamp("2026-10-20T10:00:00Z"),
        expires_at: Some(timestamp("2026-10-20T09:00:00Z")),
    })
    .expect_err("reversed time must fail");
    assert_eq!(
        reversed.code(),
        "collaborative_identity.grant.time_window.invalid"
    );

    let mut tampered = serialized;
    tampered["project_id"] = Value::String("project/other".to_owned());
    let error = serde_json::from_value::<CollaborativeProjectGrant>(tampered)
        .expect_err("tampered grant must fail");
    assert_eq!(error.to_string(), "collaborative project grant is invalid");
}

#[test]
fn revocation_is_additive_and_distinguishes_exact_replay_from_conflict() {
    let grant = grant(
        "grant/alpha",
        vec![HostedProjectCapability::RunRead],
        "2026-10-20T10:00:00Z",
        None,
    );
    let revocation = revocation(&grant, "revocation/grant-alpha", "2026-10-20T11:00:00Z");
    assert_eq!(
        reconcile_collaborative_project_grant_revocation(&revocation, &revocation)
            .expect("exact replay"),
        CollaborativeProjectGrantRevocationReconciliation::ExactReplay
    );

    let conflict =
        CollaborativeProjectGrantRevocation::new(CollaborativeProjectGrantRevocationDefinition {
            revocation_id: revocation.revocation_id().clone(),
            grant_id: grant.grant_id().clone(),
            grant_commitment: grant.commitment().clone(),
            organization_id: grant.organization_id().clone(),
            project_id: grant.project_id().clone(),
            principal_id: grant.principal_id().clone(),
            revoked_at: timestamp("2026-10-20T12:00:00Z"),
            reason: CollaborativeProjectGrantRevocationReason::SecurityResponse,
            revoked_by: ActorId::new("user/admin").expect("revoking actor"),
        })
        .expect("conflicting candidate shape");
    let error = reconcile_collaborative_project_grant_revocation(&revocation, &conflict)
        .expect_err("replacement must fail");
    assert_eq!(error.code(), "collaborative_identity.revocation.conflict");
}

#[test]
fn current_authority_projection_unions_only_active_exact_project_grants() {
    let provider = provider(CollaborativeIdentityProviderLifecycle::Active);
    let binding = binding(CollaborativePrincipalIdentityBindingLifecycle::Active);
    let principal = principal(CollaborativePrincipalLifecycle::Active);
    let organization = OrganizationId::new("org/acme").expect("organization");
    let project = ProjectId::new("project/alpha").expect("project");
    let active = grant(
        "grant/active",
        vec![HostedProjectCapability::RunRead],
        "2026-10-20T10:00:00Z",
        None,
    );
    let revoked = grant(
        "grant/revoked",
        vec![HostedProjectCapability::ApprovalDecide],
        "2026-10-20T10:00:00Z",
        None,
    );
    let expired = grant(
        "grant/expired",
        vec![HostedProjectCapability::RunCancel],
        "2026-10-19T10:00:00Z",
        Some("2026-10-20T11:00:00Z"),
    );
    let revocation = revocation(&revoked, "revocation/revoked", "2026-10-20T11:00:00Z");
    let grants = vec![revoked, active.clone(), expired];
    let revocations = vec![revocation];
    let input = CollaborativeCurrentAuthorityProjectionInput {
        provider: &provider,
        binding: &binding,
        principal: &principal,
        organization_id: &organization,
        project_id: &project,
        authority_registry_revision: HostedAuthorityRegistryRevision::new(8)
            .expect("current revision"),
        grants: &grants,
        revocations: &revocations,
        evaluated_at: timestamp("2026-10-20T12:00:00Z"),
    };
    let projection = project_collaborative_current_authority(&input).expect("projection");
    assert_eq!(projection.active_grants().len(), 1);
    assert_eq!(projection.active_grants()[0].grant_id(), active.grant_id());
    assert_eq!(
        projection.capabilities(),
        &[HostedProjectCapability::RunRead]
    );
    assert!(projection.allows(HostedProjectCapability::RunRead));
    assert!(!projection.allows(HostedProjectCapability::ApprovalDecide));

    let serialized = serde_json::to_string(&projection).expect("serialize trusted projection");
    let unverified: UnverifiedCollaborativeCurrentAuthorityProjection =
        serde_json::from_str(&serialized).expect("decode only as unverified claim");
    let debug = format!("{unverified:?}");
    assert!(!debug.contains("user/alice"));
    assert!(!debug.contains("project/alpha"));
}

#[test]
fn lifecycle_and_current_revision_fail_closed() {
    let disabled_provider = provider(CollaborativeIdentityProviderLifecycle::Disabled);
    let active_binding = binding(CollaborativePrincipalIdentityBindingLifecycle::Active);
    let active_principal = principal(CollaborativePrincipalLifecycle::Active);
    let organization = OrganizationId::new("org/acme").expect("organization");
    let project = ProjectId::new("project/alpha").expect("project");
    let grants = vec![grant(
        "grant/active",
        vec![HostedProjectCapability::RunRead],
        "2026-10-20T10:00:00Z",
        None,
    )];
    let error =
        project_collaborative_current_authority(&CollaborativeCurrentAuthorityProjectionInput {
            provider: &disabled_provider,
            binding: &active_binding,
            principal: &active_principal,
            organization_id: &organization,
            project_id: &project,
            authority_registry_revision: HostedAuthorityRegistryRevision::new(8)
                .expect("current revision"),
            grants: &grants,
            revocations: &[],
            evaluated_at: timestamp("2026-10-20T12:00:00Z"),
        })
        .expect_err("disabled provider must fail");
    assert_eq!(
        error.code(),
        "collaborative_identity.projection.provider_inactive"
    );

    let active_provider = provider(CollaborativeIdentityProviderLifecycle::Active);
    let stale =
        project_collaborative_current_authority(&CollaborativeCurrentAuthorityProjectionInput {
            provider: &active_provider,
            binding: &active_binding,
            principal: &active_principal,
            organization_id: &organization,
            project_id: &project,
            authority_registry_revision: HostedAuthorityRegistryRevision::new(6)
                .expect("stale revision"),
            grants: &grants,
            revocations: &[],
            evaluated_at: timestamp("2026-10-20T12:00:00Z"),
        })
        .expect_err("stale authority must fail");
    assert_eq!(
        stale.code(),
        "collaborative_identity.projection.authority_revision.stale"
    );
}

#[test]
fn missing_or_fully_revoked_grants_do_not_create_fake_authority() {
    let provider = provider(CollaborativeIdentityProviderLifecycle::Active);
    let binding = binding(CollaborativePrincipalIdentityBindingLifecycle::Active);
    let principal = principal(CollaborativePrincipalLifecycle::Active);
    let organization = OrganizationId::new("org/acme").expect("organization");
    let project = ProjectId::new("project/alpha").expect("project");
    let grant = grant(
        "grant/revoked",
        vec![HostedProjectCapability::RunRead],
        "2026-10-20T10:00:00Z",
        None,
    );
    let revocation = revocation(&grant, "revocation/revoked", "2026-10-20T11:00:00Z");
    let error =
        project_collaborative_current_authority(&CollaborativeCurrentAuthorityProjectionInput {
            provider: &provider,
            binding: &binding,
            principal: &principal,
            organization_id: &organization,
            project_id: &project,
            authority_registry_revision: HostedAuthorityRegistryRevision::new(8)
                .expect("current revision"),
            grants: &[grant],
            revocations: &[revocation],
            evaluated_at: timestamp("2026-10-20T12:00:00Z"),
        })
        .expect_err("fully revoked authority must fail");
    assert_eq!(
        error.code(),
        "collaborative_identity.projection.active_grant_missing"
    );
}

#[test]
fn projection_rejects_cross_scope_grants_instead_of_filtering_them() {
    let provider = provider(CollaborativeIdentityProviderLifecycle::Active);
    let binding = binding(CollaborativePrincipalIdentityBindingLifecycle::Active);
    let principal = principal(CollaborativePrincipalLifecycle::Active);
    let organization = OrganizationId::new("org/acme").expect("organization");
    let project = ProjectId::new("project/alpha").expect("project");
    let mismatched_grant = CollaborativeProjectGrant::new(CollaborativeProjectGrantDefinition {
        grant_id: CollaborativeProjectGrantId::new("grant/wrong-project").expect("grant id"),
        organization_id: organization.clone(),
        project_id: ProjectId::new("project/beta").expect("other project"),
        principal_id: principal.principal_id().clone(),
        capabilities: vec![HostedProjectCapability::RunRead],
        issued_by: ActorId::new("user/admin").expect("issuer"),
        revision: CollaborativeProjectGrantRevision::new(1).expect("revision"),
        authority_registry_revision: HostedAuthorityRegistryRevision::new(7)
            .expect("authority revision"),
        effective_at: timestamp("2026-10-20T10:00:00Z"),
        expires_at: None,
    })
    .expect("grant");
    let error =
        project_collaborative_current_authority(&CollaborativeCurrentAuthorityProjectionInput {
            provider: &provider,
            binding: &binding,
            principal: &principal,
            organization_id: &organization,
            project_id: &project,
            authority_registry_revision: HostedAuthorityRegistryRevision::new(8)
                .expect("current revision"),
            grants: &[mismatched_grant],
            revocations: &[],
            evaluated_at: timestamp("2026-10-20T12:00:00Z"),
        })
        .expect_err("cross-scope input must fail closed");
    assert_eq!(
        error.code(),
        "collaborative_identity.projection.grant_scope_mismatch"
    );
}

#[test]
fn active_binding_sets_reject_ambiguous_subjects_and_principals() {
    let first = binding(CollaborativePrincipalIdentityBindingLifecycle::Active);
    let duplicate_subject = CollaborativePrincipalIdentityBinding::new(
        CollaborativePrincipalIdentityBindingId::new("binding/other").expect("binding id"),
        CollaborativeIdentityProviderId::new("provider/corporate").expect("provider id"),
        HostedAuthorityRegistryRevision::new(7).expect("provider revision"),
        OrganizationId::new("org/acme").expect("organization"),
        ActorId::new("principal/other").expect("principal"),
        first.subject_commitment().clone(),
        timestamp("2026-10-20T09:00:00Z"),
        CollaborativePrincipalIdentityBindingLifecycle::Active,
    )
    .expect("second binding");
    let error = validate_collaborative_identity_binding_set(&[first.clone(), duplicate_subject])
        .expect_err("one subject cannot map to two active principals");
    assert_eq!(
        error.code(),
        "collaborative_identity.binding_set.subject_ambiguous"
    );

    let duplicate_principal = CollaborativePrincipalIdentityBinding::new(
        CollaborativePrincipalIdentityBindingId::new("binding/other").expect("binding id"),
        CollaborativeIdentityProviderId::new("provider/other").expect("provider id"),
        HostedAuthorityRegistryRevision::new(7).expect("provider revision"),
        OrganizationId::new("org/acme").expect("organization"),
        first.principal_id().clone(),
        CollaborativeExternalSubjectCommitment::new(
            SpecContentHash::from_text("different-subject").as_str(),
        )
        .expect("subject commitment"),
        timestamp("2026-10-20T09:00:00Z"),
        CollaborativePrincipalIdentityBindingLifecycle::Active,
    )
    .expect("second binding");
    let error = validate_collaborative_identity_binding_set(&[first, duplicate_principal])
        .expect_err("one principal cannot have two active external identities");
    assert_eq!(
        error.code(),
        "collaborative_identity.binding_set.principal_ambiguous"
    );
}
