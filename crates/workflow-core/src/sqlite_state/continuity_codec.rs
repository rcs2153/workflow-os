use std::collections::BTreeMap;

use rusqlite::{params, Connection};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::authorized_execution_continuity_state::internal::{
    AuthoritativeAttemptRecord, AuthoritativeAttemptState, AuthoritativeDirectiveRecord,
    AuthoritativeDirectiveState, AuthoritativeOperationRecord, AuthoritativeWaitIdentity,
    AuthoritativeWaitRecord, AuthoritativeWaitState, AuthoritativeWindowRecord,
    AuthoritativeWindowState, AuthoritativeYieldRecord, CommittedOperationDisposition,
    CommittedSecurityRejectionKind, ContinuityInstanceEligibility, ContinuityRevision,
    ContinuityTrustedTimeEpochId, ContinuityYieldGenerationId, ExpectedWindowBinding,
    RecordedOperationResult, ReferenceContinuityState, SecurityRejectionCommitmentInput,
    TrustedTimePosture, TrustedTimeSecurityRecord, TrustedTimeSourceKind,
};
use crate::{SpecContentHash, Timestamp, WorkflowOsError, WorkflowOsErrorKind};

const ENVELOPE_MAX_BYTES: usize = 16_384;
const ENVELOPE_FIELD_MAX_BYTES: usize = 256;
const ENVELOPE_FIELD_MAX_COUNT: usize = 64;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RequestEnvelope {
    pub(super) version: u8,
    pub(super) domain: String,
    pub(super) fields: Vec<String>,
    pub(super) window_id: String,
    pub(super) yield_generation_id: Option<String>,
    pub(super) wait_condition_id: Option<String>,
    pub(super) wait_condition_version: Option<u32>,
    pub(super) attempt_id: Option<String>,
}

impl RequestEnvelope {
    pub(super) fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.version != 1
            || self.domain.is_empty()
            || self.domain.len() > ENVELOPE_FIELD_MAX_BYTES
            || self.fields.len() > ENVELOPE_FIELD_MAX_COUNT
            || self
                .fields
                .iter()
                .any(|field| field.len() > ENVELOPE_FIELD_MAX_BYTES)
            || self.window_id.is_empty()
            || self.window_id.len() > 128
        {
            return Err(corrupt());
        }
        Ok(())
    }
}

pub(super) fn encode<T: Serialize>(value: &T) -> Result<String, WorkflowOsError> {
    let payload = serde_json::to_string(value).map_err(|_| corrupt())?;
    if !(2..=ENVELOPE_MAX_BYTES).contains(&payload.len()) {
        return Err(corrupt());
    }
    Ok(payload)
}

pub(super) fn decode<T: DeserializeOwned + Serialize>(payload: &str) -> Result<T, WorkflowOsError> {
    if !(2..=ENVELOPE_MAX_BYTES).contains(&payload.len()) {
        return Err(corrupt());
    }
    let value = serde_json::from_str(payload).map_err(|_| corrupt())?;
    if encode(&value)? != payload {
        return Err(corrupt());
    }
    Ok(value)
}

pub(super) fn timestamp_parts(value: Timestamp) -> (i64, i64) {
    let value = value.as_offset_date_time();
    (value.unix_timestamp(), i64::from(value.nanosecond()))
}

pub(super) fn timestamp_from_parts(seconds: i64, nanos: i64) -> Result<Timestamp, WorkflowOsError> {
    if !(0..=999_999_999).contains(&nanos) {
        return Err(corrupt());
    }
    let total = i128::from(seconds)
        .checked_mul(1_000_000_000)
        .and_then(|value| value.checked_add(i128::from(nanos)))
        .ok_or_else(corrupt)?;
    time::OffsetDateTime::from_unix_timestamp_nanos(total)
        .map(Timestamp::from_offset_date_time)
        .map_err(|_| corrupt())
}

#[allow(clippy::too_many_lines)]
pub(super) fn load_snapshot(
    connection: &Connection,
) -> Result<ReferenceContinuityState, WorkflowOsError> {
    let trusted_time = load_trusted_time(connection)?;
    let mut windows = BTreeMap::new();
    read_payload_rows(
        connection,
        "SELECT window_id, record_json FROM continuity_windows",
        |id, payload| {
            let record: AuthoritativeWindowRecord = decode(payload)?;
            if record.window_id.as_str() != id || !window_projection_matches(connection, &record)? {
                return Err(corrupt());
            }
            windows.insert(record.window_id.clone(), record);
            Ok(())
        },
    )?;
    let mut attempts = BTreeMap::new();
    read_payload_rows(
        connection,
        "SELECT attempt_id, record_json FROM continuity_attempts",
        |id, payload| {
            let record: AuthoritativeAttemptRecord = decode(payload)?;
            if record.attempt_id.as_str() != id || !attempt_projection_matches(connection, &record)?
            {
                return Err(corrupt());
            }
            attempts.insert(record.attempt_id.clone(), record);
            Ok(())
        },
    )?;
    let mut yields = BTreeMap::new();
    read_payload_rows(
        connection,
        "SELECT generation_id, record_json FROM continuity_yields",
        |id, payload| {
            let record: AuthoritativeYieldRecord = decode(payload)?;
            if record.generation_id.as_str() != id
                || !yield_projection_matches(connection, &record, &attempts)?
            {
                return Err(corrupt());
            }
            yields.insert(record.generation_id.clone(), record);
            Ok(())
        },
    )?;
    let mut waits = BTreeMap::new();
    read_payload_rows(
        connection,
        "SELECT condition_id, record_json FROM continuity_waits",
        |id, payload| {
            let record: AuthoritativeWaitRecord = decode(payload)?;
            if record.condition_id.as_str() != id || !wait_projection_matches(connection, &record)?
            {
                return Err(corrupt());
            }
            waits.insert(
                AuthoritativeWaitIdentity::new(
                    record.condition_id.clone(),
                    record.condition_version,
                ),
                record,
            );
            Ok(())
        },
    )?;
    let mut directives = BTreeMap::new();
    read_payload_rows(
        connection,
        "SELECT directive_id, record_json FROM continuity_directives",
        |id, payload| {
            let record: AuthoritativeDirectiveRecord = decode(payload)?;
            if record.directive_id.as_str() != id
                || !directive_projection_matches(connection, &record)?
            {
                return Err(corrupt());
            }
            directives.insert(record.directive_id.clone(), record);
            Ok(())
        },
    )?;
    let mut operations = BTreeMap::new();
    let mut statement = connection
        .prepare("SELECT operation_id, request_json, result_json, rejection_json FROM continuity_operations")
        .map_err(|_| corrupt())?;
    let mut rows = statement.query([]).map_err(|_| corrupt())?;
    while let Some(row) = rows.next().map_err(|_| corrupt())? {
        let operation_id: String = row.get(0).map_err(|_| corrupt())?;
        let request_payload: String = row.get(1).map_err(|_| corrupt())?;
        let result_payload: Option<String> = row.get(2).map_err(|_| corrupt())?;
        let rejection_payload: Option<String> = row.get(3).map_err(|_| corrupt())?;
        let request: RequestEnvelope = decode(&request_payload)?;
        request.validate()?;
        let ((Some(payload), None) | (None, Some(payload))) = (result_payload, rejection_payload)
        else {
            return Err(corrupt());
        };
        let record: AuthoritativeOperationRecord = decode(&payload)?;
        if record.operation_id.as_str() != operation_id
            || request.window_id != operation_window_id(&record.disposition)
            || !operation_projection_matches(connection, &record, &request)?
        {
            return Err(corrupt());
        }
        operations.insert(record.operation_id.clone(), record);
    }
    let state = ReferenceContinuityState {
        trusted_time,
        windows,
        yields,
        waits,
        directives,
        attempts,
        operations,
    };
    validate_relationships(&state)?;
    Ok(state)
}

fn read_payload_rows<F>(
    connection: &Connection,
    sql: &str,
    mut read: F,
) -> Result<(), WorkflowOsError>
where
    F: FnMut(&str, &str) -> Result<(), WorkflowOsError>,
{
    let mut statement = connection.prepare(sql).map_err(|_| corrupt())?;
    let mut rows = statement.query([]).map_err(|_| corrupt())?;
    while let Some(row) = rows.next().map_err(|_| corrupt())? {
        let id: String = row.get(0).map_err(|_| corrupt())?;
        let payload: String = row.get(1).map_err(|_| corrupt())?;
        read(&id, &payload)?;
    }
    Ok(())
}

fn load_trusted_time(
    connection: &Connection,
) -> Result<TrustedTimeSecurityRecord, WorkflowOsError> {
    let row = connection
        .query_row(
            "SELECT source_kind, provenance_commitment, epoch_id, observed_seconds,
                    observed_nanos, posture, eligibility, revision
             FROM continuity_trusted_time WHERE singleton_id = 1",
            [],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<i64>>(3)?,
                    row.get::<_, Option<i64>>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, String>(6)?,
                    row.get::<_, i64>(7)?,
                ))
            },
        )
        .map_err(|_| corrupt())?;
    let last_observed_at = match (row.3, row.4) {
        (None, None) => None,
        (Some(seconds), Some(nanos)) => Some(timestamp_from_parts(seconds, nanos)?),
        _ => return Err(corrupt()),
    };
    Ok(TrustedTimeSecurityRecord {
        source: match row.0.as_str() {
            "core_injected_clock_v1" => TrustedTimeSourceKind::CoreInjectedClockV1,
            _ => return Err(corrupt()),
        },
        provenance_commitment: decode_string(&row.1)?,
        epoch_id: ContinuityTrustedTimeEpochId::new(row.2).map_err(|_| corrupt())?,
        last_observed_at,
        posture: match row.5.as_str() {
            "unobserved" => TrustedTimePosture::Unobserved,
            "healthy" => TrustedTimePosture::Healthy,
            "quarantined" => TrustedTimePosture::Quarantined,
            _ => return Err(corrupt()),
        },
        eligibility: match row.6.as_str() {
            "live_state_eligible" => ContinuityInstanceEligibility::LiveStateEligible,
            "restore_unverified" => ContinuityInstanceEligibility::RestoreUnverified,
            "quarantined" => ContinuityInstanceEligibility::Quarantined,
            _ => return Err(corrupt()),
        },
        revision: revision(row.7)?,
    })
}

fn decode_string<T: DeserializeOwned>(value: &str) -> Result<T, WorkflowOsError> {
    serde_json::from_value(serde_json::Value::String(value.to_owned())).map_err(|_| corrupt())
}

fn revision(value: i64) -> Result<ContinuityRevision, WorkflowOsError> {
    u64::try_from(value)
        .ok()
        .and_then(|value| ContinuityRevision::new(value).ok())
        .ok_or_else(corrupt)
}

fn window_projection_matches(
    connection: &Connection,
    record: &AuthoritativeWindowRecord,
) -> Result<bool, WorkflowOsError> {
    let (expires_seconds, expires_nanos) = timestamp_parts(record.expires_at);
    let (watermark_seconds, watermark_nanos) = timestamp_parts(record.trusted_time_watermark);
    let binding =
        super::super::authorized_execution_continuity_state::internal::window_binding_commitment(
            &ExpectedWindowBinding {
                workflow_id: record.workflow_id.clone(),
                run_id: record.run_id.clone(),
                step_id: record.step_id.clone(),
                subject_actor_id: record.subject_actor_id.clone(),
                immutable_run_bundle: record.immutable_run_bundle.clone(),
                governance_commitment: record.governance_commitment.clone(),
                authority_commitment: record.authority_commitment.clone(),
                cursor: record.cursor.clone(),
            },
        );
    count(connection, "SELECT COUNT(*) FROM continuity_windows WHERE window_id=?1 AND workflow_id=?2 AND run_id=?3 AND step_id=?4 AND window_binding_commitment=?5 AND subject_actor_id=?6 AND immutable_bundle_commitment=?7 AND governance_commitment=?8 AND authority_commitment=?9 AND cursor_sequence=?10 AND cursor_event_id=?11 AND state=?12 AND maximum_attempts=?13 AND next_attempt_number=?14 AND expires_seconds=?15 AND expires_nanos=?16 AND watermark_seconds=?17 AND watermark_nanos=?18 AND trusted_time_epoch_id=?19 AND revision=?20 AND active_yield_generation_id IS ?21", params![record.window_id.as_str(), record.workflow_id.as_str(), record.run_id.as_str(), record.step_id.as_str(), binding.as_str(), record.subject_actor_id.as_str(), record.immutable_run_bundle.root_hash().as_str(), record.governance_commitment.as_str(), record.authority_commitment.as_str(), i64::try_from(record.cursor.sequence_number.get()).map_err(|_| corrupt())?, record.cursor.event_id.as_str(), window_state(record.state), i64::from(record.maximum_attempts), i64::from(record.next_attempt_number), expires_seconds, expires_nanos, watermark_seconds, watermark_nanos, record.trusted_time_epoch_id.as_str(), i64::try_from(record.revision.get()).map_err(|_| corrupt())?, record.active_yield.as_ref().map(ContinuityYieldGenerationId::as_str)])
}

fn attempt_projection_matches(
    connection: &Connection,
    record: &AuthoritativeAttemptRecord,
) -> Result<bool, WorkflowOsError> {
    count(connection, "SELECT COUNT(*) FROM continuity_attempts WHERE attempt_id=?1 AND window_id=?2 AND attempt_number=?3 AND subject_actor_id=?4 AND cursor_sequence=?5 AND cursor_event_id=?6 AND authority_commitment=?7 AND consume_operation_id=?8 AND state=?9 AND revision=?10", params![record.attempt_id.as_str(), record.window_id.as_str(), i64::from(record.attempt_number), record.subject_actor_id.as_str(), i64::try_from(record.cursor.sequence_number.get()).map_err(|_| corrupt())?, record.cursor.event_id.as_str(), record.authority_commitment.as_str(), record.consume_operation_id.as_str(), attempt_state(record.state), i64::try_from(record.revision.get()).map_err(|_| corrupt())?])
}

fn yield_projection_matches(
    connection: &Connection,
    record: &AuthoritativeYieldRecord,
    attempts: &BTreeMap<crate::AuthorizedExecutionAttemptId, AuthoritativeAttemptRecord>,
) -> Result<bool, WorkflowOsError> {
    let window_id = attempts
        .get(&record.attempt_id)
        .ok_or_else(corrupt)?
        .window_id
        .as_str();
    let (seconds, nanos) = timestamp_parts(record.registered_at);
    count(connection, "SELECT COUNT(*) FROM continuity_yields WHERE generation_id=?1 AND window_id=?2 AND attempt_id=?3 AND cursor_sequence=?4 AND cursor_event_id=?5 AND reason=?6 AND registered_seconds=?7 AND registered_nanos=?8", params![record.generation_id.as_str(), window_id, record.attempt_id.as_str(), i64::try_from(record.cursor.sequence_number.get()).map_err(|_| corrupt())?, record.cursor.event_id.as_str(), yield_reason(record.reason), seconds, nanos])
}

fn wait_projection_matches(
    connection: &Connection,
    record: &AuthoritativeWaitRecord,
) -> Result<bool, WorkflowOsError> {
    count(connection, "SELECT COUNT(*) FROM continuity_waits WHERE condition_id=?1 AND condition_version=?2 AND window_id=?3 AND generation_id=?4 AND wake_trigger=?5 AND state=?6 AND source_commitment IS ?7 AND source_revision IS ?8 AND revision=?9", params![record.condition_id.as_str(), i64::from(record.condition_version), record.window_id.as_str(), record.generation_id.as_str(), wake_trigger(record.wake_trigger), wait_state(record.state), record.source_commitment.as_ref().map(SpecContentHash::as_str), record.source_revision.and_then(|value| i64::try_from(value).ok()), i64::try_from(record.revision.get()).map_err(|_| corrupt())?])
}

fn directive_projection_matches(
    connection: &Connection,
    record: &AuthoritativeDirectiveRecord,
) -> Result<bool, WorkflowOsError> {
    count(connection, "SELECT COUNT(*) FROM continuity_directives WHERE directive_id=?1 AND window_id=?2 AND generation_id=?3 AND cursor_sequence=?4 AND cursor_event_id=?5 AND authority_commitment=?6 AND state=?7 AND revision=?8", params![record.directive_id.as_str(), record.window_id.as_str(), record.generation_id.as_str(), i64::try_from(record.cursor.sequence_number.get()).map_err(|_| corrupt())?, record.cursor.event_id.as_str(), record.authority_commitment.as_str(), directive_state(record.state), i64::try_from(record.revision.get()).map_err(|_| corrupt())?])
}

#[allow(clippy::too_many_lines)]
fn operation_projection_matches(
    connection: &Connection,
    record: &AuthoritativeOperationRecord,
    request: &RequestEnvelope,
) -> Result<bool, WorkflowOsError> {
    let trusted =
        super::super::authorized_execution_continuity_state::internal::trusted_time_commitment(
            &record.trusted_time,
        );
    let expected_operation =
        super::super::authorized_execution_continuity_state::internal::operation_commitment(
            &record.request_commitment,
            &record.receipt.receipt_id,
            &record.trusted_time,
            &trusted,
            &record.disposition,
        );
    if expected_operation != record.operation_commitment
        || record.receipt.operation_commitment != expected_operation
        || record.receipt.trusted_time_commitment != trusted
        || record.receipt.committed_at != record.trusted_time.observed_at()
    {
        return Ok(false);
    }
    let (
        success_yield_generation_id,
        success_wait_condition_id,
        success_wait_condition_version,
        success_attempt_id,
        success_consume_operation_id,
        result_commitment_value,
        rejection_commitment_value,
        rejection_kind_value,
    ) = match &record.disposition {
        CommittedOperationDisposition::CommittedSuccess(result) => {
            let (yield_id, wait_id, wait_version, attempt_id, consume_operation_id) = match result {
                RecordedOperationResult::YieldRegistered { generation_id, .. } => {
                    (Some(generation_id.as_str()), None, None, None, None)
                }
                RecordedOperationResult::WaitTransitioned {
                    condition_id,
                    condition_version,
                    ..
                } => (
                    None,
                    Some(condition_id.as_str()),
                    Some(i64::from(*condition_version)),
                    None,
                    None,
                ),
                RecordedOperationResult::DirectiveConsumed { attempt_id, .. } => (
                    None,
                    None,
                    None,
                    Some(attempt_id.as_str()),
                    Some(record.operation_id.as_str()),
                ),
                RecordedOperationResult::AttemptOutcomeRecorded { attempt_id, .. } => {
                    (None, None, None, Some(attempt_id.as_str()), None)
                }
            };
            (
                yield_id,
                wait_id,
                wait_version,
                attempt_id,
                consume_operation_id,
                Some(
                    super::super::authorized_execution_continuity_state::internal::result_commitment(
                        result,
                    ),
                ),
                None,
                None,
            )
        }
        CommittedOperationDisposition::CommittedSecurityRejection(rejection) => {
            let expected =
                super::super::authorized_execution_continuity_state::internal::rejection_commitment(
                    &SecurityRejectionCommitmentInput {
                        kind: rejection.kind,
                        observation: &rejection.trusted_time,
                        expected_time_source: rejection.expected_time_source,
                        expected_provenance_commitment: &rejection.expected_provenance_commitment,
                        expected_epoch_id: &rejection.expected_epoch_id,
                        window_id: &rejection.window_id,
                        window_expires_at: rejection.window_expires_at,
                        prior_trusted_time: &rejection.prior_trusted_time,
                        resulting_trusted_time: &rejection.resulting_trusted_time,
                        prior_window: &rejection.prior_window,
                        resulting_window: &rejection.resulting_window,
                    },
                );
            if rejection.rejection_commitment != expected
                || rejection.trusted_time != record.trusted_time
            {
                return Ok(false);
            }
            (
                None,
                None,
                None,
                None,
                None,
                None,
                Some(expected),
                Some(rejection_kind(rejection.kind)),
            )
        }
    };
    let (seconds, nanos) = timestamp_parts(record.trusted_time.observed_at());
    count(connection, "SELECT COUNT(*) FROM continuity_operations WHERE operation_id=?1 AND receipt_id=?2 AND operation_kind=?3 AND request_commitment=?4 AND operation_commitment=?5 AND disposition=?6 AND request_window_id=?7 AND request_yield_generation_id IS ?8 AND request_wait_condition_id IS ?9 AND request_wait_condition_version IS ?10 AND request_attempt_id IS ?11 AND success_yield_generation_id IS ?12 AND success_wait_condition_id IS ?13 AND success_wait_condition_version IS ?14 AND success_attempt_id IS ?15 AND success_consume_operation_id IS ?16 AND result_commitment IS ?17 AND rejection_commitment IS ?18 AND rejection_kind IS ?19 AND trusted_time_source_kind='core_injected_clock_v1' AND trusted_time_provenance_commitment=?20 AND trusted_time_epoch_id=?21 AND observed_seconds=?22 AND observed_nanos=?23 AND trusted_time_commitment=?24 AND committed_seconds=?22 AND committed_nanos=?23", params![record.operation_id.as_str(), record.receipt.receipt_id.as_str(), operation_kind(record.operation_kind), record.request_commitment.as_str(), record.operation_commitment.as_str(), disposition_code(&record.disposition), request.window_id, request.yield_generation_id, request.wait_condition_id, request.wait_condition_version.map(i64::from), request.attempt_id, success_yield_generation_id, success_wait_condition_id, success_wait_condition_version, success_attempt_id, success_consume_operation_id, result_commitment_value.as_ref().map(SpecContentHash::as_str), rejection_commitment_value.as_ref().map(SpecContentHash::as_str), rejection_kind_value, record.trusted_time.provenance_commitment().as_str(), record.trusted_time.epoch_id().as_str(), seconds, nanos, trusted.as_str()])
}

fn validate_relationships(state: &ReferenceContinuityState) -> Result<(), WorkflowOsError> {
    for window in state.windows.values() {
        if window
            .active_yield
            .as_ref()
            .is_some_and(|id| !state.yields.contains_key(id))
        {
            return Err(corrupt());
        }
    }
    for yield_record in state.yields.values() {
        let attempt = state
            .attempts
            .get(&yield_record.attempt_id)
            .ok_or_else(corrupt)?;
        if yield_record.wait_ids.iter().any(|id| {
            state.waits.get(id).map_or(true, |wait| {
                wait.window_id != attempt.window_id
                    || wait.generation_id != yield_record.generation_id
            })
        }) {
            return Err(corrupt());
        }
    }
    for directive in state.directives.values() {
        if !state.yields.contains_key(&directive.generation_id)
            || !state.windows.contains_key(&directive.window_id)
        {
            return Err(corrupt());
        }
    }
    for attempt in state.attempts.values() {
        if !state.windows.contains_key(&attempt.window_id)
            || state
                .operations
                .get(&attempt.consume_operation_id)
                .map_or(true, |operation| {
                    operation.operation_kind
                        != crate::AuthorizedExecutionContinuityOperationKind::ConsumeDirective
                })
        {
            return Err(corrupt());
        }
    }
    Ok(())
}

fn operation_window_id(disposition: &CommittedOperationDisposition) -> String {
    match disposition {
        CommittedOperationDisposition::CommittedSuccess(result) => match result {
            RecordedOperationResult::YieldRegistered { window_id, .. }
            | RecordedOperationResult::WaitTransitioned { window_id, .. }
            | RecordedOperationResult::DirectiveConsumed { window_id, .. }
            | RecordedOperationResult::AttemptOutcomeRecorded { window_id, .. } => {
                window_id.as_str().to_owned()
            }
        },
        CommittedOperationDisposition::CommittedSecurityRejection(rejection) => {
            rejection.window_id.as_str().to_owned()
        }
    }
}

fn count(
    connection: &Connection,
    sql: &str,
    params: impl rusqlite::Params,
) -> Result<bool, WorkflowOsError> {
    connection
        .query_row(sql, params, |row| row.get::<_, i64>(0))
        .map(|count| count == 1)
        .map_err(|_| corrupt())
}

pub(super) fn window_state(value: AuthoritativeWindowState) -> &'static str {
    match value {
        AuthoritativeWindowState::AssessmentRequired => "assessment_required",
        AuthoritativeWindowState::Executing => "executing",
        AuthoritativeWindowState::Yielded => "yielded",
        AuthoritativeWindowState::Closed => "closed",
        AuthoritativeWindowState::RecoveryRequired => "recovery_required",
        AuthoritativeWindowState::Expired => "expired",
        AuthoritativeWindowState::Revoked => "revoked",
        AuthoritativeWindowState::Superseded => "superseded",
    }
}
pub(super) fn attempt_state(value: AuthoritativeAttemptState) -> &'static str {
    match value {
        AuthoritativeAttemptState::Started => "started",
        AuthoritativeAttemptState::Yielded => "yielded",
        AuthoritativeAttemptState::Succeeded => "succeeded",
        AuthoritativeAttemptState::RetryableFailure => "retryable_failure",
        AuthoritativeAttemptState::TerminalFailure => "terminal_failure",
        AuthoritativeAttemptState::AmbiguousMayHaveStarted => "ambiguous_may_have_started",
    }
}
pub(super) fn wait_state(value: AuthoritativeWaitState) -> &'static str {
    match value {
        AuthoritativeWaitState::Unsatisfied => "unsatisfied",
        AuthoritativeWaitState::Satisfied => "satisfied",
        AuthoritativeWaitState::Expired => "expired",
        AuthoritativeWaitState::Superseded => "superseded",
        AuthoritativeWaitState::Canceled => "canceled",
    }
}
pub(super) fn directive_state(value: AuthoritativeDirectiveState) -> &'static str {
    match value {
        AuthoritativeDirectiveState::Available => "available",
        AuthoritativeDirectiveState::Consumed => "consumed",
        AuthoritativeDirectiveState::Invalidated => "invalidated",
        AuthoritativeDirectiveState::Expired => "expired",
    }
}
pub(super) fn yield_reason(value: crate::AuthorizedExecutionYieldReason) -> &'static str {
    match value {
        crate::AuthorizedExecutionYieldReason::TurnBoundary => "turn_boundary",
        crate::AuthorizedExecutionYieldReason::ContextBudget => "context_budget",
        crate::AuthorizedExecutionYieldReason::HostPreemption => "host_preemption",
        crate::AuthorizedExecutionYieldReason::VoluntaryCheckpoint => "voluntary_checkpoint",
        crate::AuthorizedExecutionYieldReason::TransientExecutorFailure => {
            "transient_executor_failure"
        }
    }
}
pub(super) fn wake_trigger(value: crate::AuthorizedExecutionWakeTriggerKind) -> &'static str {
    match value {
        crate::AuthorizedExecutionWakeTriggerKind::ApprovalDecisionRecorded => {
            "approval_decision_recorded"
        }
        crate::AuthorizedExecutionWakeTriggerKind::EvidenceAccepted => "evidence_accepted",
        crate::AuthorizedExecutionWakeTriggerKind::CheckAccepted => "check_accepted",
        crate::AuthorizedExecutionWakeTriggerKind::ExternalEventRecorded => {
            "external_event_recorded"
        }
        crate::AuthorizedExecutionWakeTriggerKind::CapabilityAvailabilityChanged => {
            "capability_availability_changed"
        }
        crate::AuthorizedExecutionWakeTriggerKind::DeadlineReached => "deadline_reached",
        crate::AuthorizedExecutionWakeTriggerKind::AuthoritySourceChanged => {
            "authority_source_changed"
        }
        crate::AuthorizedExecutionWakeTriggerKind::ConflictResolved => "conflict_resolved",
    }
}
pub(super) fn operation_kind(
    value: crate::AuthorizedExecutionContinuityOperationKind,
) -> &'static str {
    match value {
        crate::AuthorizedExecutionContinuityOperationKind::RegisterYield => "register_yield",
        crate::AuthorizedExecutionContinuityOperationKind::TransitionWait => "transition_wait",
        crate::AuthorizedExecutionContinuityOperationKind::ConsumeDirective => "consume_directive",
        crate::AuthorizedExecutionContinuityOperationKind::RecordAttemptOutcome => {
            "record_attempt_outcome"
        }
        crate::AuthorizedExecutionContinuityOperationKind::RecoverAmbiguousAttempt => {
            "recover_ambiguous_attempt"
        }
    }
}
pub(super) fn disposition_code(value: &CommittedOperationDisposition) -> &'static str {
    match value {
        CommittedOperationDisposition::CommittedSuccess(_) => "committed_success",
        CommittedOperationDisposition::CommittedSecurityRejection(_) => {
            "committed_security_rejection"
        }
    }
}

pub(super) fn rejection_kind(value: CommittedSecurityRejectionKind) -> &'static str {
    match value {
        CommittedSecurityRejectionKind::Regressed => "time_regressed",
        CommittedSecurityRejectionKind::Untrusted => "time_untrusted",
        CommittedSecurityRejectionKind::EpochMismatch => "time_epoch_mismatch",
        CommittedSecurityRejectionKind::Expired => "time_expired",
    }
}

pub(super) fn corrupt() -> WorkflowOsError {
    super::super::authorized_execution_continuity_state::internal::continuity_state_error(
        WorkflowOsErrorKind::InvalidState,
        "state.corrupt",
        "authorized execution continuity state is corrupt",
    )
}
