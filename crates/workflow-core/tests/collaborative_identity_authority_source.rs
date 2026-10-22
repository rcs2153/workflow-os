#![allow(clippy::expect_used)]

//! Focused tests for the closed collaborative identity authority-source model.

use workflow_core::{
    ActorId, CollaborativeExternalSubjectCommitment, CollaborativeIdentityAudienceCommitment,
    CollaborativeIdentityAuthorityMatchPosture, CollaborativeIdentityAuthorityQuery,
    CollaborativeIdentityAuthorityRevisionCandidate, CollaborativeIdentityIssuerCommitment,
    CollaborativeIdentityProviderId, CollaborativeIdentityProviderLifecycle,
    CollaborativeIdentityProviderRegistration, CollaborativeIdentityProviderRegistrationVersion,
    CollaborativePrincipalIdentityBinding, CollaborativePrincipalIdentityBindingId,
    CollaborativePrincipalIdentityBindingLifecycle, CollaborativePrincipalLifecycle,
    CollaborativePrincipalRecord, CollaborativePrincipalRevision, CollaborativeProjectGrant,
    CollaborativeProjectGrantDefinition, CollaborativeProjectGrantId,
    CollaborativeProjectGrantRevision, CollaborativeProjectGrantRevocation,
    CollaborativeProjectGrantRevocationDefinition, CollaborativeProjectGrantRevocationId,
    CollaborativeProjectGrantRevocationReason, HostedAuthorityRegistryRevision,
    HostedPrincipalKind, HostedProjectCapability, OrganizationId, ProjectId, SpecContentHash,
    Timestamp,
};

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).expect("timestamp")
}

fn hash(value: &str) -> String {
    SpecContentHash::from_text(value).as_str().to_owned()
}

fn grant(expires_at: Option<&str>) -> CollaborativeProjectGrant {
    CollaborativeProjectGrant::new(CollaborativeProjectGrantDefinition {
        grant_id: CollaborativeProjectGrantId::new("grant/alpha").expect("grant id"),
        organization_id: OrganizationId::new("org/acme").expect("organization"),
        project_id: ProjectId::new("project/alpha").expect("project"),
        principal_id: ActorId::new("user/alice").expect("principal"),
        capabilities: vec![HostedProjectCapability::ApprovalDecide],
        issued_by: ActorId::new("user/admin").expect("issuer"),
        revision: CollaborativeProjectGrantRevision::new(1).expect("grant revision"),
        authority_registry_revision: HostedAuthorityRegistryRevision::new(7)
            .expect("authority revision"),
        effective_at: timestamp("2026-10-20T10:00:00Z"),
        expires_at: expires_at.map(timestamp),
    })
    .expect("grant")
}

fn candidate(
    grant: CollaborativeProjectGrant,
    revoked: bool,
) -> CollaborativeIdentityAuthorityRevisionCandidate {
    let provider = CollaborativeIdentityProviderRegistration::new(
        CollaborativeIdentityProviderId::new("provider/corporate").expect("provider"),
        CollaborativeIdentityProviderRegistrationVersion::new(1).expect("version"),
        HostedAuthorityRegistryRevision::new(7).expect("revision"),
        OrganizationId::new("org/acme").expect("organization"),
        CollaborativeIdentityIssuerCommitment::new(hash("issuer")).expect("issuer"),
        vec![CollaborativeIdentityAudienceCommitment::new(hash("audience")).expect("audience")],
        CollaborativeIdentityProviderLifecycle::Active,
    )
    .expect("provider");
    let binding = CollaborativePrincipalIdentityBinding::new(
        CollaborativePrincipalIdentityBindingId::new("binding/alice").expect("binding"),
        provider.provider_id().clone(),
        provider.revision(),
        provider.organization_id().clone(),
        ActorId::new("user/alice").expect("principal"),
        CollaborativeExternalSubjectCommitment::new(hash("external-subject")).expect("subject"),
        timestamp("2026-10-20T10:00:00Z"),
        CollaborativePrincipalIdentityBindingLifecycle::Active,
    )
    .expect("binding");
    let principal = CollaborativePrincipalRecord::new(
        provider.organization_id().clone(),
        ActorId::new("user/alice").expect("principal"),
        HostedPrincipalKind::Human,
        CollaborativePrincipalRevision::new(1).expect("principal revision"),
        timestamp("2026-10-20T10:00:00Z"),
        CollaborativePrincipalLifecycle::Active,
    )
    .expect("principal");
    let revocations = if revoked {
        vec![CollaborativeProjectGrantRevocation::new(
            CollaborativeProjectGrantRevocationDefinition {
                revocation_id: CollaborativeProjectGrantRevocationId::new("revocation/alpha")
                    .expect("revocation"),
                grant_id: grant.grant_id().clone(),
                grant_commitment: grant.commitment().clone(),
                organization_id: grant.organization_id().clone(),
                project_id: grant.project_id().clone(),
                principal_id: grant.principal_id().clone(),
                revoked_at: timestamp("2026-10-21T10:00:00Z"),
                reason: CollaborativeProjectGrantRevocationReason::AccessRemoved,
                revoked_by: ActorId::new("user/admin").expect("revoker"),
            },
        )
        .expect("revocation")]
    } else {
        Vec::new()
    };
    CollaborativeIdentityAuthorityRevisionCandidate::new(
        HostedAuthorityRegistryRevision::new(7).expect("revision"),
        vec![provider],
        vec![binding],
        vec![principal],
        vec![grant],
        revocations,
    )
    .expect("candidate")
}

fn query() -> CollaborativeIdentityAuthorityQuery {
    CollaborativeIdentityAuthorityQuery::new(
        OrganizationId::new("org/acme").expect("organization"),
        ProjectId::new("project/alpha").expect("project"),
        ActorId::new("user/alice").expect("principal"),
        HostedProjectCapability::ApprovalDecide,
    )
    .expect("query")
}

#[test]
fn complete_candidate_matches_only_active_unexpired_unrevoked_exact_grants() {
    let active = candidate(grant(Some("2026-10-22T10:00:00Z")), false);
    assert_eq!(
        active.active_matching_grant_count(&query(), timestamp("2026-10-21T12:00:00Z")),
        1
    );
    assert_eq!(
        active.active_matching_grant_count(&query(), timestamp("2026-10-23T12:00:00Z")),
        0
    );
    let revoked = candidate(grant(None), true);
    assert_eq!(
        revoked.active_matching_grant_count(&query(), timestamp("2026-10-21T12:00:00Z")),
        0
    );
}

#[test]
fn query_and_candidate_debug_are_payload_safe_and_commitments_are_deterministic() {
    let query = query();
    let first = candidate(grant(None), false);
    let second = candidate(grant(None), false);
    assert_eq!(first.commitment(), second.commitment());
    let debug = format!("{query:?} {first:?}");
    assert!(!debug.contains("user/alice"));
    assert!(!debug.contains("project/alpha"));
    assert!(!debug.contains("org/acme"));
    let serialized = serde_json::to_string(&query).expect("serialize query");
    assert!(!serialized.contains("external-subject"));
    assert!(!serialized.contains("issuer"));
}

#[test]
fn candidate_rejects_dangling_identity_facts_without_leaking_values() {
    let mut provider = candidate(grant(None), false).providers()[0].clone();
    provider = CollaborativeIdentityProviderRegistration::new(
        CollaborativeIdentityProviderId::new("provider/other").expect("provider"),
        provider.version(),
        provider.revision(),
        provider.organization_id().clone(),
        provider.issuer_commitment().clone(),
        provider.audience_commitments().to_vec(),
        provider.lifecycle(),
    )
    .expect("provider");
    let base = candidate(grant(None), false);
    let error = CollaborativeIdentityAuthorityRevisionCandidate::new(
        base.authority_revision(),
        vec![provider],
        base.bindings().to_vec(),
        base.principals().to_vec(),
        base.grants().to_vec(),
        Vec::new(),
    )
    .expect_err("dangling binding must fail");
    assert_eq!(
        error.code(),
        "collaborative_identity_authority_source.candidate.binding.provider_missing"
    );
    assert!(!error.to_string().contains("provider/corporate"));
}

#[test]
fn match_posture_vocabulary_is_closed() {
    assert_eq!(
        serde_json::to_string(&CollaborativeIdentityAuthorityMatchPosture::MatchingGrantPresent)
            .expect("serialize"),
        "\"matching_grant_present\""
    );
}
