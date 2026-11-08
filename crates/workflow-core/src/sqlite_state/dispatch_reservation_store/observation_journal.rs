//! Private, payload-free observation history. It never records an outcome.
#![allow(dead_code)] // No operator observer is admitted yet.

use std::num::NonZeroU64;

use rusqlite::{params, TransactionBehavior};
use serde::{Deserialize, Serialize};

use crate::authorized_execution_continuity_state::internal::TrustedTimeObservation;
use crate::dispatch_reservation::DispatchReservationOperationId;
use crate::provider_comment_observation::CurrentCommentPosture;
use crate::provider_comment_transport::{
    installation_notification::CreationCandidate, IndependentCommentInventory,
};
use crate::provider_write::GitHubCommentContinuationAction;
use crate::{SpecContentHash, Timestamp, WorkflowOsError};

use super::{
    corrupt, installation_inventory_context, installation_send_preparation_payload,
    not_dispatchable, storage_error, InstallationInventoryContext,
};
use crate::sqlite_state::{
    continuity_store, decode_json, encode_json, validate_operational_schema, SqliteStateBackend,
    INSTALLATION_OBSERVATION_JOURNAL_SCHEMA_VERSION,
};

const MAX_OBSERVATIONS: u16 = 16;

#[derive(Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ObservationIdentity {
    operation: String,
    preparation: SpecContentHash,
    notification: SpecContentHash,
    receipt: SpecContentHash,
    received_latest: Timestamp,
    inventory: SpecContentHash,
    reader: NonZeroU64,
    candidate: NonZeroU64,
    posture: CurrentCommentPosture,
}

#[derive(Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ObservationRecord {
    identity: ObservationIdentity,
    clock: TrustedTimeObservation,
}

fn commitment<T: Serialize>(domain: &str, value: &T) -> Result<SpecContentHash, WorkflowOsError> {
    serde_json::to_vec(&(domain, value))
        .map(SpecContentHash::from_bytes)
        .map_err(|_| corrupt())
}

fn identity_commitment(identity: &ObservationIdentity) -> Result<SpecContentHash, WorkflowOsError> {
    commitment("workflow-os/installation-observation-id/v1", identity)
}

fn record_commitment(record: &ObservationRecord) -> Result<SpecContentHash, WorkflowOsError> {
    commitment("workflow-os/installation-observation-record/v1", record)
}

fn verify_clock(clock: &TrustedTimeObservation, prepared: &TrustedTimeObservation) -> bool {
    clock.source() == prepared.source()
        && clock.epoch_id() == prepared.epoch_id()
        && clock.provenance_commitment() == prepared.provenance_commitment()
        && clock.observed_at() >= prepared.observed_at()
}

fn read_journal(
    tx: &rusqlite::Transaction<'_>,
    operation: &DispatchReservationOperationId,
    context: &InstallationInventoryContext,
) -> Result<Vec<(SpecContentHash, ObservationRecord)>, WorkflowOsError> {
    let (_, preparation) = installation_send_preparation_payload(&context.record)?;
    let mut statement = tx
        .prepare(
            "SELECT evidence_id, payload, payload_hash FROM installation_observation_journal
         WHERE operation_id=?1 ORDER BY evidence_id LIMIT ?2",
        )
        .map_err(|_| corrupt())?;
    let mut rows = statement
        .query(params![operation.as_str(), MAX_OBSERVATIONS + 1])
        .map_err(|_| corrupt())?;
    let mut records = Vec::new();
    while let Some(row) = rows.next().map_err(|_| corrupt())? {
        let (id, payload, hash): (String, String, String) = (
            row.get(0).map_err(|_| corrupt())?,
            row.get(1).map_err(|_| corrupt())?,
            row.get(2).map_err(|_| corrupt())?,
        );
        let record: ObservationRecord = decode_json(&payload, "installation observation")?;
        let key = identity_commitment(&record.identity)?;
        if records.len() == usize::from(MAX_OBSERVATIONS)
            || id != key.as_str()
            || hash != record_commitment(&record)?.as_str()
            || payload != encode_json(&record, "installation observation")?
            || record.identity.operation != operation.as_str()
            || record.identity.preparation != preparation
            || record.identity.reader == context.provider.author_id
            || !verify_clock(&record.clock, &context.record.clock)
            || record.identity.received_latest < context.record.clock.observed_at()
            || record.identity.received_latest >= context.record.deadline
            || record.clock.observed_at() < record.identity.received_latest
        {
            return Err(corrupt());
        }
        records.push((key, record));
    }
    Ok(records)
}

impl SqliteStateBackend {
    pub(in crate::sqlite_state) fn record_prepared_installation_inventory(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        candidate: &CreationCandidate,
        reader: &IndependentCommentInventory<'_>,
    ) -> Result<Option<SpecContentHash>, WorkflowOsError> {
        require_journal_schema(&self.existing_read_only_connection()?)?;
        let Some((before, traversal, comment)) =
            self.collect_prepared_installation_inventory(operation, action, candidate, reader)?
        else {
            return Ok(None);
        };
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| storage_error("begin_failed"))?;
        require_journal_schema(&tx)?;
        if installation_inventory_context(&tx, operation, action, false)?.as_ref() != Some(&before)
        {
            return Err(not_dispatchable());
        }
        let (notification, receipt) = candidate.commitments();
        let (_, preparation) = installation_send_preparation_payload(&before.record)?;
        let identity = ObservationIdentity {
            operation: operation.as_str().to_owned(),
            preparation,
            notification: notification.clone(),
            receipt: receipt.clone(),
            received_latest: candidate.received_latest(),
            inventory: traversal.commitment,
            reader: traversal.reader,
            candidate: comment,
            posture: traversal.posture,
        };
        let id = identity_commitment(&identity)?;
        let existing = read_journal(&tx, operation, &before)?;
        if let Some((_, prior)) = existing.iter().find(|(prior_id, _)| prior_id == &id) {
            if prior.identity != identity {
                return Err(corrupt());
            }
        } else {
            if existing.len() >= usize::from(MAX_OBSERVATIONS) {
                return Err(not_dispatchable());
            }
            let clock = continuity_store::observe_continuity_trusted_time()?;
            if !verify_clock(&clock, &before.record.clock)
                || clock.observed_at() < identity.received_latest
            {
                return Err(not_dispatchable());
            }
            let record = ObservationRecord { identity, clock };
            tx.execute(
                "INSERT INTO installation_observation_journal (operation_id, evidence_id, payload, payload_hash) VALUES (?1,?2,?3,?4)",
                params![operation.as_str(), id.as_str(), encode_json(&record, "installation observation")?, record_commitment(&record)?.as_str()],
            ).map_err(|_| storage_error("write_failed"))?;
        }
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(Some(id))
    }

    pub(in crate::sqlite_state) fn inspect_installation_observation_journal(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
    ) -> Result<Vec<(SpecContentHash, CurrentCommentPosture)>, WorkflowOsError> {
        let mut connection = self.existing_read_only_connection()?;
        let tx = connection
            .transaction()
            .map_err(|_| storage_error("begin_failed"))?;
        require_journal_schema(&tx)?;
        let Some(context) = installation_inventory_context(&tx, operation, action, true)? else {
            return Ok(Vec::new());
        };
        let records = read_journal(&tx, operation, &context)?;
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(records
            .into_iter()
            .map(|(id, record)| (id, record.identity.posture))
            .collect())
    }
}

fn require_journal_schema(connection: &rusqlite::Connection) -> Result<(), WorkflowOsError> {
    if validate_operational_schema(connection)? != INSTALLATION_OBSERVATION_JOURNAL_SCHEMA_VERSION {
        return Err(not_dispatchable());
    }
    Ok(())
}
