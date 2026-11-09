//! Private observation history and bounded recovery after acknowledgement loss.
#![allow(dead_code)] // No operator observer is admitted yet.

use std::{num::NonZeroU64, time::Instant};

use rusqlite::{params, TransactionBehavior};
use serde::{Deserialize, Serialize};

use crate::authorized_execution_continuity_state::internal::TrustedTimeObservation;
use crate::dispatch_reservation::DispatchReservationOperationId;
use crate::provider_comment_observation::CurrentCommentPosture;
use crate::provider_comment_transport::{
    installation_exchange::reader::{
        ReaderSignerRequest, ReviewedReadInstallation, VerifiedReadInstallationScope,
    },
    installation_notification::{
        CreationCandidate, CreationNotification, ReviewedCreationReceiver,
    },
    IndependentCommentInventory, InstallationObserverProfile,
};
use crate::provider_write::GitHubCommentContinuationAction;
use crate::{
    EventId, GitHubPullRequestCommentTarget, SideEffectOutcomeReference,
    SideEffectOutcomeReferenceKind, SideEffectRecord, SpecContentHash, Timestamp, WorkflowOsError,
    WorkflowRunEvent,
};

use super::{
    corrupt, find_live_comment_proof, installation_disposition, installation_inventory_context,
    installation_send_preparation_payload, live_comment_proof_key, not_dispatchable,
    persist_bound_live_comment_outcome, read_bound_installation_preparation,
    read_installation_send_preparation_record, read_live_recovery_admission, storage_error,
    verify_comment_disposition, CollectedInstallationInventory, InstallationCommentRecoveryPosture,
    InstallationInventoryContext, LiveCommentProofKind, LiveRecoveryAdmission,
};
use crate::sqlite_state::{
    continuity_store, decode_json, encode_json, validate_operational_schema, SqliteStateBackend,
    INSTALLATION_OBSERVATION_JOURNAL_SCHEMA_VERSION,
};

const MAX_OBSERVATIONS: u16 = 16;
const MAX_RECOVERY_OBSERVATION_AGE: std::time::Duration = std::time::Duration::from_secs(30);

pub(in crate::sqlite_state) struct RecoveryReaderAuthorization<'a> {
    commitment: SpecContentHash,
    target: &'a GitHubPullRequestCommentTarget,
    repository: NonZeroU64,
    profile: &'a InstallationObserverProfile,
    started: Instant,
}

pub(in crate::sqlite_state) struct AuthorizedRecoveryReader {
    commitment: SpecContentHash,
    scope: VerifiedReadInstallationScope,
}

impl RecoveryReaderAuthorization<'_> {
    pub(in crate::sqlite_state) fn exchange(
        self,
        reviewed: ReviewedReadInstallation<'_>,
        issue: impl FnOnce(&ReaderSignerRequest) -> Option<VerifiedReadInstallationScope>,
    ) -> Option<AuthorizedRecoveryReader> {
        let (not_before, request) = self.signer_request(reviewed)?;
        let scope = issue(&request)?;
        if !scope.matches_fresh_review(reviewed, not_before, Timestamp::now_utc())
            || self.started.elapsed() >= MAX_RECOVERY_OBSERVATION_AGE
        {
            return None;
        }
        Some(AuthorizedRecoveryReader {
            commitment: self.commitment,
            scope,
        })
    }

    fn signer_request(
        &self,
        reviewed: ReviewedReadInstallation<'_>,
    ) -> Option<(Timestamp, ReaderSignerRequest)> {
        let not_before = Timestamp::now_utc();
        if reviewed.target != self.target
            || reviewed.repository != self.repository
            || reviewed.reader != self.profile.reader
            || reviewed.access_commitment().as_ref() != Some(&self.profile.reader_access_review)
            || not_before >= reviewed.deadline
            || self.started.elapsed() >= MAX_RECOVERY_OBSERVATION_AGE
        {
            return None;
        }
        let remaining = MAX_RECOVERY_OBSERVATION_AGE.checked_sub(self.started.elapsed())?;
        let deadline = Timestamp::from_offset_date_time(
            not_before
                .as_offset_date_time()
                .checked_add(time::Duration::try_from(remaining).ok()?)?,
        )
        .min(reviewed.deadline);
        let request = ReaderSignerRequest::new(
            self.commitment.clone(),
            self.profile.reader_access_review.clone(),
            deadline,
        );
        Some((not_before, request))
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(in crate::sqlite_state) fn exchange_with_signer(
        self,
        reviewed: ReviewedReadInstallation<'_>,
        signer: crate::provider_comment_transport::installation_signer_channel::reader_signer::RecoveryReaderSigner,
    ) -> Option<AuthorizedRecoveryReader> {
        self.exchange(reviewed, |request| {
            let assertion = signer.request(request).ok()?;
            crate::provider_comment_transport::installation_exchange::reader::exchange_scoped_reader(reviewed, request, assertion)
        })
    }
}

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    observer_profile: Option<SpecContentHash>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    recovery_started_at: Option<Timestamp>,
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
    let observer_profile = observer_profile_commitment(context)?;
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
            || record.identity.observer_profile != observer_profile
            || context
                .observer_profile
                .as_ref()
                .is_some_and(|profile| record.identity.reader != profile.reader)
            || !verify_clock(&record.clock, &context.record.clock)
            || record.identity.received_latest < context.record.clock.observed_at()
            || record.identity.received_latest >= context.record.deadline
            || record.clock.observed_at() < record.identity.received_latest
            || record.identity.recovery_started_at.is_some_and(|started| {
                !within_recovery_observation_interval(
                    started,
                    record.clock.observed_at(),
                    record.identity.received_latest,
                ) || record.identity.posture != CurrentCommentPosture::ExactCurrentMatch
            })
        {
            return Err(corrupt());
        }
        records.push((key, record));
    }
    Ok(records)
}

fn observation_identity(
    candidate: &CreationCandidate,
    (context, traversal, comment): &CollectedInstallationInventory,
) -> Result<ObservationIdentity, WorkflowOsError> {
    let (notification, receipt) = candidate.commitments();
    let (_, preparation) = installation_send_preparation_payload(&context.record)?;
    Ok(ObservationIdentity {
        operation: context.record.operation.clone(),
        preparation,
        notification: notification.clone(),
        receipt: receipt.clone(),
        received_latest: candidate.received_latest(),
        inventory: traversal.commitment.clone(),
        reader: traversal.reader,
        candidate: *comment,
        posture: traversal.posture,
        observer_profile: observer_profile_commitment(context)?,
        recovery_started_at: None,
    })
}

fn observer_profile_commitment(
    context: &InstallationInventoryContext,
) -> Result<Option<SpecContentHash>, WorkflowOsError> {
    context
        .observer_profile
        .as_ref()
        .map(|profile| profile.commitment().ok_or_else(corrupt))
        .transpose()
}

fn within_recovery_observation_interval(
    started: Timestamp,
    observed: Timestamp,
    received_latest: Timestamp,
) -> bool {
    started >= received_latest
        && observed >= started
        && observed.as_offset_date_time() - started.as_offset_date_time()
            <= MAX_RECOVERY_OBSERVATION_AGE
}

fn persist_observation(
    tx: &rusqlite::Transaction<'_>,
    existing: &[(SpecContentHash, ObservationRecord)],
    context: &InstallationInventoryContext,
    identity: ObservationIdentity,
) -> Result<(SpecContentHash, SpecContentHash), WorkflowOsError> {
    let id = identity_commitment(&identity)?;
    if let Some((_, prior)) = existing.iter().find(|(prior_id, _)| prior_id == &id) {
        if prior.identity != identity {
            return Err(corrupt());
        }
        return Ok((id, record_commitment(prior)?));
    }
    if existing.len() >= usize::from(MAX_OBSERVATIONS) {
        return Err(not_dispatchable());
    }
    let clock = continuity_store::observe_continuity_trusted_time()?;
    if !verify_clock(&clock, &context.record.clock)
        || clock.observed_at() < identity.received_latest
    {
        return Err(not_dispatchable());
    }
    let record = ObservationRecord { identity, clock };
    let hash = record_commitment(&record)?;
    tx.execute(
        "INSERT INTO installation_observation_journal (operation_id, evidence_id, payload, payload_hash) VALUES (?1,?2,?3,?4)",
        params![context.record.operation, id.as_str(), encode_json(&record, "installation observation")?, hash.as_str()],
    ).map_err(|_| storage_error("write_failed"))?;
    Ok((id, hash))
}

fn compatible_observation(record: &ObservationRecord, comment: NonZeroU64) -> bool {
    record.identity.posture == CurrentCommentPosture::ExactCurrentMatch
        && record.identity.candidate == comment
}

fn comment_outcome(comment: NonZeroU64) -> Result<SideEffectOutcomeReference, WorkflowOsError> {
    SideEffectOutcomeReference::new(
        SideEffectOutcomeReferenceKind::Outcome,
        format!("github/comment/{comment}"),
    )
}

pub(super) fn verify_observed_completion(
    tx: &rusqlite::Transaction<'_>,
    admission: &LiveRecoveryAdmission,
    effect: &SideEffectRecord,
    event: &WorkflowRunEvent,
) -> Result<bool, WorkflowOsError> {
    require_journal_schema(tx)?;
    let operation = &admission.operation;
    let witness = admission.witness.as_ref().ok_or_else(corrupt)?;
    let context = InstallationInventoryContext {
        record: read_bound_installation_preparation(tx, operation, admission)?
            .ok_or_else(corrupt)?,
        provider: witness.provider.clone(),
        observer_profile: witness.installation_observer_profile.clone(),
    };
    let records = read_journal(tx, operation, &context)?;
    let Some((_, first)) = records.first() else {
        return Ok(false);
    };
    let comment = first.identity.candidate;
    if effect.outcome_reference() != Some(&comment_outcome(comment)?) {
        return Ok(false);
    }
    let mut matched = false;
    for (_, record) in &records {
        if !compatible_observation(record, comment) || record.clock.observed_at() > event.timestamp
        {
            return Ok(false);
        }
        let key = live_comment_proof_key(
            admission,
            effect,
            LiveCommentProofKind::InstallationObservation,
            Some(&record_commitment(record)?),
        )?;
        if event.idempotency_key.as_ref() == Some(&key)
            && record.identity.recovery_started_at.is_some_and(|started| {
                within_recovery_observation_interval(
                    started,
                    event.timestamp,
                    record.identity.received_latest,
                )
            })
        {
            if matched {
                return Err(corrupt());
            }
            matched = true;
        }
    }
    Ok(matched)
}

impl SqliteStateBackend {
    #[cfg(all(test, any(target_os = "linux", target_os = "macos")))]
    pub(in crate::sqlite_state) fn recover_installation_comment_unix_rehearsal<'a>(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        receive: impl FnOnce(
            &InstallationObserverProfile,
        )
            -> Option<(ReviewedCreationReceiver<'a>, CreationNotification<'a>)>,
        reviewed: ReviewedReadInstallation<'_>,
        read_inventory: impl FnOnce(&ReaderSignerRequest) -> Option<Vec<u8>>,
    ) -> Result<InstallationCommentRecoveryPosture, WorkflowOsError> {
        self.recover_installation_comment_using(
            operation,
            action,
            receive,
            |profile, writer_scope, candidate| {
                let Some(authorization) = self.authorize_recovery_reader(
                    operation,
                    action,
                    profile,
                    writer_scope,
                    candidate,
                )?
                else {
                    return Ok(());
                };
                let Some((_, request)) = authorization.signer_request(reviewed) else {
                    return Ok(());
                };
                self.reconcile_prepared_installation_comment_using(
                    operation,
                    action,
                    candidate,
                    |expected| {
                        let bytes = read_inventory(&request)?;
                        if authorization.started.elapsed() >= MAX_RECOVERY_OBSERVATION_AGE
                            || Timestamp::now_utc() >= request.deadline
                        {
                            return None;
                        }
                        crate::provider_comment_transport::observe_unix_rehearsal_inventory(
                            expected,
                            reviewed.reader,
                            &bytes,
                        )
                    },
                )?;
                Ok(())
            },
        )
    }

    /// Resolves historical state before asking the trusted host for evidence.
    pub(in crate::sqlite_state) fn recover_installation_comment<'a>(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        receive: impl FnOnce(
            &InstallationObserverProfile,
        )
            -> Option<(ReviewedCreationReceiver<'a>, CreationNotification<'a>)>,
        exchange_reader: impl FnOnce(
            RecoveryReaderAuthorization<'_>,
        ) -> Option<AuthorizedRecoveryReader>,
    ) -> Result<InstallationCommentRecoveryPosture, WorkflowOsError> {
        self.recover_installation_comment_using(
            operation,
            action,
            receive,
            |profile, writer_scope, candidate| {
                let Some(authorization) = self.authorize_recovery_reader(
                    operation,
                    action,
                    profile,
                    writer_scope,
                    candidate,
                )?
                else {
                    return Ok(());
                };
                let commitment = authorization.commitment.clone();
                let started = authorization.started;
                if let Some(reader) = exchange_reader(authorization) {
                    if reader.commitment != commitment {
                        return Err(not_dispatchable());
                    }
                    if started.elapsed() >= MAX_RECOVERY_OBSERVATION_AGE {
                        return Ok(());
                    }
                    self.reconcile_prepared_installation_comment(
                        operation,
                        action,
                        candidate,
                        &reader.scope.inventory(writer_scope),
                    )?;
                }
                Ok(())
            },
        )
    }

    fn authorize_recovery_reader<'a>(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'a>,
        profile: &'a InstallationObserverProfile,
        writer_scope: &SpecContentHash,
        candidate: &CreationCandidate,
    ) -> Result<Option<RecoveryReaderAuthorization<'a>>, WorkflowOsError> {
        let started = Instant::now();
        let mut connection = self.existing_read_only_connection()?;
        let tx = connection
            .transaction()
            .map_err(|_| storage_error("begin_failed"))?;
        require_journal_schema(&tx)?;
        let admission = read_live_recovery_admission(&tx, operation, action.run)?;
        if installation_disposition(&tx, &admission, action.run, action.side_effect)?
            != InstallationCommentRecoveryPosture::ReconciliationRequired
        {
            return Ok(None);
        }
        let Some(record) =
            read_installation_send_preparation_record(&tx, operation, action, &admission)?
        else {
            return Ok(None);
        };
        let witness = admission.witness.as_ref().ok_or_else(corrupt)?;
        if witness.installation_observer_profile.as_ref() != Some(profile)
            || &record.scope != writer_scope
        {
            return Err(not_dispatchable());
        }
        let Some(comment) = candidate.prepared_comment_id(
            &record.reservation,
            &record.body_commitment,
            &record.marker_commitment,
            Some(profile),
        ) else {
            return Err(not_dispatchable());
        };
        let (_, preparation) = installation_send_preparation_payload(&record)?;
        let binding = encode_json(
            &(
                "workflow-os/installation-recovery-reader/v1",
                operation.as_str(),
                &record.invocation,
                preparation,
                &profile.reader_access_review,
                comment,
                candidate.commitments(),
            ),
            "installation recovery reader",
        )?;
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(Some(RecoveryReaderAuthorization {
            commitment: SpecContentHash::from_text(&binding),
            target: action.target,
            repository: witness.provider.repository_id,
            profile,
            started,
        }))
    }

    #[cfg(test)]
    pub(in crate::sqlite_state) fn recover_installation_comment_fixture<'a, 'b>(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        receive: impl FnOnce(
            &InstallationObserverProfile,
        )
            -> Option<(ReviewedCreationReceiver<'a>, CreationNotification<'a>)>,
        open_reader: impl FnOnce(
            &InstallationObserverProfile,
        ) -> Option<IndependentCommentInventory<'b>>,
    ) -> Result<InstallationCommentRecoveryPosture, WorkflowOsError> {
        self.recover_installation_comment_using(
            operation,
            action,
            receive,
            |profile, _, candidate| {
                if let Some(reader) = open_reader(profile) {
                    self.reconcile_prepared_installation_comment(
                        operation, action, candidate, &reader,
                    )?;
                }
                Ok(())
            },
        )
    }

    fn recover_installation_comment_using<'a>(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        receive: impl FnOnce(
            &InstallationObserverProfile,
        )
            -> Option<(ReviewedCreationReceiver<'a>, CreationNotification<'a>)>,
        reconcile: impl FnOnce(
            &InstallationObserverProfile,
            &SpecContentHash,
            &CreationCandidate,
        ) -> Result<(), WorkflowOsError>,
    ) -> Result<InstallationCommentRecoveryPosture, WorkflowOsError> {
        let (profile, writer_scope) = {
            let mut connection = self.existing_read_only_connection()?;
            let tx = connection
                .transaction()
                .map_err(|_| storage_error("begin_failed"))?;
            let admission = read_live_recovery_admission(&tx, operation, action.run)?;
            if admission.event.invocation_commitment() != &action.commitment()? {
                return Err(not_dispatchable());
            }
            let posture =
                installation_disposition(&tx, &admission, action.run, action.side_effect)?;
            if posture != InstallationCommentRecoveryPosture::ReconciliationRequired
                || validate_operational_schema(&tx)?
                    != INSTALLATION_OBSERVATION_JOURNAL_SCHEMA_VERSION
            {
                return Ok(posture);
            }
            let Some(preparation) =
                read_bound_installation_preparation(&tx, operation, &admission)?
            else {
                return Ok(posture);
            };
            let profile = admission
                .witness
                .and_then(|witness| witness.installation_observer_profile)
                .filter(|profile| profile.webhook_material_tag.is_some());
            (profile, preparation.scope)
        };
        let unresolved = InstallationCommentRecoveryPosture::ReconciliationRequired;
        let Some(profile) = profile else {
            return Ok(unresolved);
        };
        let Some((receiver, notification)) = receive(&profile) else {
            return Ok(unresolved);
        };
        let Some(candidate) = self.verify_prepared_installation_notification(
            operation,
            action,
            receiver,
            notification,
        )?
        else {
            return Ok(unresolved);
        };
        reconcile(&profile, &writer_scope, &candidate)?;
        self.inspect_installation_comment_recovery(action.run, operation, action.side_effect)
    }

    pub(in crate::sqlite_state) fn record_prepared_installation_inventory(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        candidate: &CreationCandidate,
        reader: &IndependentCommentInventory<'_>,
    ) -> Result<Option<SpecContentHash>, WorkflowOsError> {
        require_journal_schema(&self.existing_read_only_connection()?)?;
        let Some(collected) = self.collect_prepared_installation_inventory(
            operation,
            action,
            candidate,
            |expected| reader.observe(expected),
        )?
        else {
            return Ok(None);
        };
        let before = &collected.0;
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| storage_error("begin_failed"))?;
        require_journal_schema(&tx)?;
        if installation_inventory_context(&tx, operation, action, false)?.as_ref() != Some(before) {
            return Err(not_dispatchable());
        }
        let existing = read_journal(&tx, operation, before)?;
        let (id, _) = persist_observation(
            &tx,
            &existing,
            before,
            observation_identity(candidate, &collected)?,
        )?;
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        Ok(Some(id))
    }

    fn reconcile_prepared_installation_comment(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        candidate: &CreationCandidate,
        reader: &IndependentCommentInventory<'_>,
    ) -> Result<Option<EventId>, WorkflowOsError> {
        self.reconcile_prepared_installation_comment_using(
            operation,
            action,
            candidate,
            |expected| reader.observe(expected),
        )
    }

    fn reconcile_prepared_installation_comment_using(
        &self,
        operation: &DispatchReservationOperationId,
        action: GitHubCommentContinuationAction<'_>,
        candidate: &CreationCandidate,
        observe: impl FnOnce(
            &crate::provider_comment_transport::PreparedCommentInventory<'_>,
        )
            -> Option<crate::provider_comment_transport::VerifiedInventoryTraversal>,
    ) -> Result<Option<EventId>, WorkflowOsError> {
        {
            let mut connection = self.existing_read_only_connection()?;
            let tx = connection
                .transaction()
                .map_err(|_| storage_error("begin_failed"))?;
            require_journal_schema(&tx)?;
            if installation_inventory_context(&tx, operation, action, true)?.is_none() {
                return Ok(None);
            }
            let admission = read_live_recovery_admission(&tx, operation, action.run)?;
            let history = Self::read_events_with_connection(&tx, action.run)?;
            if let Some((event, kind)) = find_live_comment_proof(&admission.event, &history)? {
                return Ok((kind == LiveCommentProofKind::InstallationObservation)
                    .then(|| event.event_id.clone()));
            }
        }

        let started = std::time::Instant::now();
        let started_clock = continuity_store::observe_continuity_trusted_time()?;
        let Some(collected) =
            self.collect_prepared_installation_inventory(operation, action, candidate, observe)?
        else {
            return Ok(None);
        };
        let (before, traversal, comment) = &collected;
        let mut connection = self.existing_connection()?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| storage_error("begin_failed"))?;
        require_journal_schema(&tx)?;
        if installation_inventory_context(&tx, operation, action, false)?.as_ref() != Some(before) {
            return Err(not_dispatchable());
        }
        let existing = read_journal(&tx, operation, before)?;
        if existing.len() >= usize::from(MAX_OBSERVATIONS) {
            return Ok(None);
        }
        let mut identity = observation_identity(candidate, &collected)?;
        let clock = continuity_store::observe_continuity_trusted_time()?;
        let eligible = traversal.posture == CurrentCommentPosture::ExactCurrentMatch
            && existing.iter().all(|(_, record)| {
                compatible_observation(record, *comment)
                    && record.clock.observed_at() <= clock.observed_at()
            })
            && verify_clock(&clock, &started_clock)
            && verify_clock(&started_clock, &before.record.clock)
            && within_recovery_observation_interval(
                started_clock.observed_at(),
                clock.observed_at(),
                candidate.received_latest(),
            )
            && started.elapsed() <= MAX_RECOVERY_OBSERVATION_AGE;
        if eligible {
            identity.recovery_started_at = Some(started_clock.observed_at());
        }
        let (_, receipt) = persist_observation(&tx, &existing, before, identity)?;
        if !eligible {
            tx.commit().map_err(|_| storage_error("commit_failed"))?;
            return Ok(None);
        }
        let admission = read_live_recovery_admission(&tx, operation, action.run)?;
        let event = persist_bound_live_comment_outcome(
            &tx,
            action,
            &admission,
            comment_outcome(*comment)?,
            LiveCommentProofKind::InstallationObservation,
            Some(&receipt),
        )?;
        if verify_comment_disposition(&tx, &admission, action.run, action.side_effect)?
            != Some(LiveCommentProofKind::InstallationObservation)
            || started.elapsed() > MAX_RECOVERY_OBSERVATION_AGE
        {
            return Err(not_dispatchable());
        }
        #[cfg(test)]
        let fault = super::INJECTED_COMMIT_FAULT.with(std::cell::Cell::take);
        #[cfg(test)]
        if fault == Some(super::InjectedDispatchCommitFault::Before) {
            return Err(storage_error("commit_failed"));
        }
        tx.commit().map_err(|_| storage_error("commit_failed"))?;
        #[cfg(test)]
        if fault == Some(super::InjectedDispatchCommitFault::After) {
            return Err(storage_error("commit_failed"));
        }
        Ok(Some(event))
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
