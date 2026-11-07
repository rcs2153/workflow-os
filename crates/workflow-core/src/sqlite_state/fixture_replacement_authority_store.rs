use rusqlite::{params, Connection, OptionalExtension};

use crate::local_fixture_replacement_authority::LocalFixturePrincipalVerifier;
use crate::{
    ActorId, CapabilityGrant, CapabilityGrantId, CapabilityGrantLifecycle, SpecContentHash,
    WorkflowOsError,
};

use super::{decode_json, encode_json, map_sqlite_error, sqlite_state_error, SqliteStateBackend};

#[derive(Eq, PartialEq)]
pub(crate) struct StoredFixtureAuthority {
    pub(crate) grant: CapabilityGrant,
    pub(crate) intent: SpecContentHash,
    pub(crate) verifier: LocalFixturePrincipalVerifier,
}

impl SqliteStateBackend {
    /// Resolves the registered principal for the supplied grant credential. This
    /// identifies the caller only; fixture validation must recheck authority.
    ///
    /// # Errors
    ///
    /// Rejects unknown or revoked authority and non-matching credentials.
    pub fn authenticate_local_fixture_grant_principal(
        &self,
        grant_id: &CapabilityGrantId,
        credential: &[u8; 32],
    ) -> Result<ActorId, WorkflowOsError> {
        let stored = self.read_active_fixture_authority(grant_id)?;
        stored.verifier.authenticate(credential)?;
        Ok(stored.verifier.principal().clone())
    }

    pub(crate) fn read_active_fixture_authority(
        &self,
        grant_id: &CapabilityGrantId,
    ) -> Result<StoredFixtureAuthority, WorkflowOsError> {
        read_active_fixture_authority(&self.existing_connection()?, grant_id)
    }
    /// Registers an operator-provisioned local principal once. The caller must
    /// be the trusted local operator, not a model-facing tool handler.
    ///
    /// # Errors
    ///
    /// Rejects duplicate identities or an unavailable writable store.
    pub fn register_local_fixture_principal(
        &self,
        verifier: &LocalFixturePrincipalVerifier,
    ) -> Result<(), WorkflowOsError> {
        let payload = encode_json(verifier, "fixture principal verifier")?;
        let inserted = self
            .existing_connection()?
            .execute(
                "INSERT INTO local_fixture_principal_registrations
                 (principal_id, verifier_payload, revision) VALUES (?1, ?2, 1)
             ON CONFLICT(principal_id) DO NOTHING",
                params![verifier.principal().as_str(), payload],
            )
            .map_err(store_error)?;
        if inserted != 1 {
            return Err(authority_unavailable());
        }
        Ok(())
    }

    /// Permanently revokes a local fixture principal and makes its grants unusable.
    /// This is a trusted local operator operation, not a model-facing tool.
    ///
    /// # Errors
    ///
    /// Rejects unknown or already revoked identities and unavailable state.
    pub fn revoke_local_fixture_principal(
        &self,
        principal: &ActorId,
    ) -> Result<(), WorkflowOsError> {
        let changed = self
            .existing_connection()?
            .execute(
                "UPDATE local_fixture_principal_registrations SET revision = 2
             WHERE principal_id = ?1 AND revision = 1",
                [principal.as_str()],
            )
            .map_err(store_error)?;
        if changed != 1 {
            return Err(authority_unavailable());
        }
        Ok(())
    }

    /// Permanently revokes one fixture grant without altering the approved intent.
    /// This is a trusted local operator operation, not a model-facing tool.
    ///
    /// # Errors
    ///
    /// Rejects unknown or already revoked grants and unavailable state.
    pub fn revoke_local_fixture_grant(
        &self,
        grant_id: &CapabilityGrantId,
    ) -> Result<(), WorkflowOsError> {
        let changed = self
            .existing_connection()?
            .execute(
                "UPDATE local_fixture_replacement_grants SET revision = 2
             WHERE grant_id = ?1 AND revision = 1",
                [grant_id.as_str()],
            )
            .map_err(store_error)?;
        if changed != 1 {
            return Err(authority_unavailable());
        }
        Ok(())
    }
}

pub(crate) fn fixture_presentation_is_current(
    connection: &Connection,
    expected: &crate::ApprovalPresentationRecord,
) -> Result<bool, WorkflowOsError> {
    let actual: Option<crate::ApprovalPresentationRecord> = super::read_optional_payload(
        connection,
        "approval_presentations",
        "presentation_id",
        expected.presentation_id().as_str(),
        "approval presentation",
    )?;
    Ok(actual.as_ref() == Some(expected))
}

pub(crate) fn create_checked_fixture_grant(
    connection: &Connection,
    grant: &CapabilityGrant,
    intent: &SpecContentHash,
) -> Result<(), WorkflowOsError> {
    if grant.lifecycle() != CapabilityGrantLifecycle::Active {
        return Err(authority_unavailable());
    }
    let verifier_payload: String = connection
        .query_row(
            "SELECT verifier_payload FROM local_fixture_principal_registrations
         WHERE principal_id = ?1 AND revision = 1",
            [grant.subject().as_str()],
            |row| row.get(0),
        )
        .optional()
        .map_err(store_error)?
        .ok_or_else(authority_unavailable)?;
    let verifier: LocalFixturePrincipalVerifier =
        decode_json(&verifier_payload, "fixture principal verifier")?;
    if verifier.principal() != grant.subject()
        || grant.scope().harness_contract_id() != Some(verifier.harness())
    {
        return Err(authority_unavailable());
    }
    let payload = encode_json(grant, "fixture replacement grant")?;
    let inserted = connection
        .execute(
            "INSERT INTO local_fixture_replacement_grants
             (grant_id, principal_id, grant_payload, intent_commitment, revision)
         SELECT ?1, principal_id, ?3, ?4, 1 FROM local_fixture_principal_registrations
         WHERE principal_id = ?2 AND revision = 1
         ON CONFLICT(grant_id) DO NOTHING",
            params![
                grant.grant_id().as_str(),
                grant.subject().as_str(),
                payload,
                intent.as_str()
            ],
        )
        .map_err(store_error)?;
    if inserted != 1 {
        return Err(authority_unavailable());
    }
    Ok(())
}

pub(crate) fn read_active_fixture_authority(
    connection: &Connection,
    grant_id: &CapabilityGrantId,
) -> Result<StoredFixtureAuthority, WorkflowOsError> {
    let (principal, grant_payload, intent, verifier_payload): (String, String, String, String) =
        connection
            .query_row(
                "SELECT p.principal_id, g.grant_payload, g.intent_commitment, p.verifier_payload
         FROM local_fixture_replacement_grants g
         JOIN local_fixture_principal_registrations p ON p.principal_id = g.principal_id
         WHERE g.grant_id = ?1 AND g.revision = 1 AND p.revision = 1",
                [grant_id.as_str()],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .optional()
            .map_err(store_error)?
            .ok_or_else(authority_unavailable)?;
    let grant: CapabilityGrant = decode_json(&grant_payload, "fixture replacement grant")?;
    let verifier: LocalFixturePrincipalVerifier =
        decode_json(&verifier_payload, "fixture principal verifier")?;
    if grant.grant_id() != grant_id
        || grant.subject() != verifier.principal()
        || grant.scope().harness_contract_id() != Some(verifier.harness())
        || grant.subject().as_str() != principal
        || grant.lifecycle() != CapabilityGrantLifecycle::Active
    {
        return Err(authority_unavailable());
    }
    Ok(StoredFixtureAuthority {
        grant,
        intent: SpecContentHash::new(intent)?,
        verifier,
    })
}

fn authority_unavailable() -> WorkflowOsError {
    sqlite_state_error(
        "fixture_authority.unavailable",
        "local fixture replacement authority is unavailable",
    )
}

fn store_error(error: rusqlite::Error) -> WorkflowOsError {
    map_sqlite_error(
        error,
        "fixture_authority.store_failed",
        "local fixture authority storage operation failed",
    )
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use crate::{HarnessContractId, WorkflowRunId};

    #[test]
    fn fresh_snapshot_rejects_authority_revoked_after_preliminary_authentication() {
        let root = std::env::temp_dir().join(WorkflowRunId::generate().as_str());
        std::fs::create_dir(&root).expect("isolated test directory");
        let path = root.join("fixture.sqlite3");
        let writer = SqliteStateBackend::open(&path).expect("initialize store");
        let reader =
            SqliteStateBackend::open_read_only(&path).expect("open second connection source");
        let actor = ActorId::new("agent/fixture-replacement").expect("actor");
        let harness = HarnessContractId::new("harness/fixture").expect("harness");
        let verifier = LocalFixturePrincipalVerifier::new(actor, harness, &[42; 32]);
        writer
            .register_local_fixture_principal(&verifier)
            .expect("register principal");
        let grant: CapabilityGrant = serde_json::from_value(serde_json::json!({
            "grant_id": "grant/revocation-race", "subject": "agent/fixture-replacement",
            "capability": "fixture/github-pr-comment-continuation",
            "resource": {"kind": "adapter_resource", "reference": "github/example/repo/pull/1"},
            "scope": {"workflow_id": "test/fixture", "run_id": "run-fixture", "step_id": "comment", "harness_contract_id": "harness/fixture"},
            "issuer": "user/maintainer", "issued_at": "2026-11-07T00:00:00Z", "expires_at": "2026-11-07T00:01:00Z",
            "lifecycle": "active", "revocation_reference": null, "delegation": {"mode": "disabled"},
            "requirements": {"policy_ids": [], "approval_ids": [], "evidence_ids": [], "check_ids": []},
            "sensitivity_ceiling": "internal", "redaction": {"redacted_fields": [], "field_states": []}
        })).expect("valid storage test grant");
        create_checked_fixture_grant(
            &writer.existing_connection().expect("writer"),
            &grant,
            &SpecContentHash::from_text("fixture binding"),
        )
        .expect("seed exact storage record");
        let preliminary = reader
            .read_active_fixture_authority(grant.grant_id())
            .expect("preliminary authority");
        assert!(preliminary.verifier.authenticate(&[42; 32]).is_ok());
        writer
            .revoke_local_fixture_grant(grant.grant_id())
            .expect("revoke on separate connection");
        let mut connection = reader.existing_connection().expect("read-only connection");
        let final_snapshot = connection.transaction().expect("fresh final snapshot");
        assert_eq!(
            read_active_fixture_authority(&final_snapshot, grant.grant_id())
                .err()
                .expect("revoked grant not current")
                .code(),
            "state.sqlite.fixture_authority.unavailable"
        );
        drop(final_snapshot);
        drop(connection);
        std::fs::remove_dir_all(root).expect("remove disposable test directory");
    }
}
