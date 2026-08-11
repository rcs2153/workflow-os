use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ApprovalReferenceId, EventId, GovernanceDecisionAuthorityReceipt,
    GovernanceDecisionAuthorityReceiptClaimVerificationPosture,
    GovernanceDecisionAuthorityReceiptEffect, GovernanceDecisionAuthorityReceiptId,
    GovernanceDecisionAuthorityReceiptSignaturePosture, GovernanceDecisionAuthorityReceiptValidity,
    SpecContentHash, UnverifiedGovernanceDecisionAuthorityReceipt, WorkflowId, WorkflowOsError,
    WorkflowRunId,
};

/// Structurally verified receipt data loaded from a persistence boundary.
///
/// This record is local, unsigned, point-in-time evidence only. Reading it does
/// not restore the trusted in-memory receipt type and cannot authorize another
/// operation.
#[derive(Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct PersistedGovernanceDecisionAuthorityReceiptRecord {
    claim: UnverifiedGovernanceDecisionAuthorityReceipt,
}

impl PersistedGovernanceDecisionAuthorityReceiptRecord {
    /// Validates deterministic receipt identity and commitment consistency.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the persisted claim is invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.claim.validate_claim()
    }

    /// Returns the explicit unverified serialized-claim posture.
    #[must_use]
    pub const fn verification_posture(
        &self,
    ) -> GovernanceDecisionAuthorityReceiptClaimVerificationPosture {
        self.claim.verification_posture()
    }

    /// Returns the deterministic receipt identity.
    #[must_use]
    pub const fn receipt_id(&self) -> &GovernanceDecisionAuthorityReceiptId {
        self.claim.receipt_id()
    }

    /// Returns the committed workflow identity.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        self.claim.workflow_id()
    }

    /// Returns the committed run identity.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        self.claim.run_id()
    }

    /// Returns the committed approval reference.
    #[must_use]
    pub const fn approval_reference_id(&self) -> &ApprovalReferenceId {
        self.claim.approval_reference_id()
    }

    /// Returns the committed approval-decision event reference.
    #[must_use]
    pub const fn approval_decision_event_id(&self) -> &EventId {
        self.claim.approval_decision_event_id()
    }

    /// Returns the complete deterministic receipt commitment.
    #[must_use]
    pub const fn receipt_commitment(&self) -> &SpecContentHash {
        self.claim.receipt_commitment()
    }

    /// Returns the explicitly non-authorizing effect.
    #[must_use]
    pub const fn effect(&self) -> GovernanceDecisionAuthorityReceiptEffect {
        self.claim.effect()
    }

    /// Returns the point-in-time validity posture.
    #[must_use]
    pub const fn validity(&self) -> GovernanceDecisionAuthorityReceiptValidity {
        self.claim.validity()
    }

    /// Returns the local unsigned signature posture.
    #[must_use]
    pub const fn signature_posture(&self) -> GovernanceDecisionAuthorityReceiptSignaturePosture {
        self.claim.signature_posture()
    }
}

impl<'de> Deserialize<'de> for PersistedGovernanceDecisionAuthorityReceiptRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let claim = UnverifiedGovernanceDecisionAuthorityReceipt::deserialize(deserializer)?;
        claim.validate_claim().map_err(|_| {
            serde::de::Error::custom(
                "invalid persisted governance decision authority receipt record",
            )
        })?;
        Ok(Self { claim })
    }
}

impl std::fmt::Debug for PersistedGovernanceDecisionAuthorityReceiptRecord {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PersistedGovernanceDecisionAuthorityReceiptRecord")
            .field("verification_posture", &self.verification_posture())
            .field("effect", &self.effect())
            .field("validity", &self.validity())
            .field("signature_posture", &self.signature_posture())
            .field("receipt_identity", &"[REDACTED]")
            .field("workflow_identity", &"[REDACTED]")
            .field("approval_reference", &"[REDACTED]")
            .field("commitment", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

/// Result of a create-only receipt-record write.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GovernanceDecisionAuthorityReceiptWriteOutcome {
    /// The receipt record was written for the first time.
    Written,
    /// The exact receipt record already exists.
    AlreadyExists,
}

/// Transport-neutral create-only persistence contract for decision-time
/// governance authority receipt records.
///
/// Implementations accept only a trusted in-memory receipt for writes. Reads
/// return structurally verified, explicitly non-authorizing persisted records.
pub trait GovernanceDecisionAuthorityReceiptRecordStore {
    /// Writes one trusted receipt using create-only, exact-idempotent semantics.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when validation or persistence fails,
    /// existing content is corrupt, or the same receipt identity has conflicting
    /// content.
    fn write_governance_decision_authority_receipt(
        &self,
        receipt: &GovernanceDecisionAuthorityReceipt,
    ) -> Result<GovernanceDecisionAuthorityReceiptWriteOutcome, WorkflowOsError>;

    /// Reads one exact persisted receipt record by stable identity.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when stored data cannot be read,
    /// fails validation, or does not match its storage address.
    fn read_governance_decision_authority_receipt(
        &self,
        receipt_id: &GovernanceDecisionAuthorityReceiptId,
    ) -> Result<Option<PersistedGovernanceDecisionAuthorityReceiptRecord>, WorkflowOsError>;
}

const RECEIPT_RECORDS_DIR: &str = "records";
static TEMP_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Local create-only persistence for governance decision authority receipts.
///
/// Stored records remain unsigned, point-in-time, non-authorizing evidence.
/// This store does not list receipts, restore trusted authority, or integrate
/// with executor state automatically.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalGovernanceDecisionAuthorityReceiptRecordStore {
    root: PathBuf,
}

impl LocalGovernanceDecisionAuthorityReceiptRecordStore {
    /// Creates a local receipt-record store rooted at the supplied directory.
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Returns the configured store root.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    fn records_dir(&self) -> PathBuf {
        self.root.join(RECEIPT_RECORDS_DIR)
    }

    fn record_path(&self, receipt_id: &GovernanceDecisionAuthorityReceiptId) -> PathBuf {
        self.records_dir()
            .join(encoded_id_file_name(receipt_id.as_str()))
    }

    fn reconcile_existing(
        path: &Path,
        receipt: &GovernanceDecisionAuthorityReceipt,
        expected_bytes: &[u8],
    ) -> Result<GovernanceDecisionAuthorityReceiptWriteOutcome, WorkflowOsError> {
        let existing_bytes = read_record_bytes(path)?;
        let existing = deserialize_record(&existing_bytes)?;
        if existing.receipt_id() != receipt.receipt_id() || existing_bytes != expected_bytes {
            return Err(store_error(
                "governance_decision_authority_receipt_store.duplicate.conflict",
                "governance decision authority receipt identity has conflicting content",
            ));
        }
        Ok(GovernanceDecisionAuthorityReceiptWriteOutcome::AlreadyExists)
    }
}

impl GovernanceDecisionAuthorityReceiptRecordStore
    for LocalGovernanceDecisionAuthorityReceiptRecordStore
{
    fn write_governance_decision_authority_receipt(
        &self,
        receipt: &GovernanceDecisionAuthorityReceipt,
    ) -> Result<GovernanceDecisionAuthorityReceiptWriteOutcome, WorkflowOsError> {
        receipt.validate().map_err(|_| invalid_record_error())?;
        let bytes = serde_json::to_vec_pretty(receipt).map_err(|_| invalid_record_error())?;
        let path = self.record_path(receipt.receipt_id());

        if path.exists() {
            return Self::reconcile_existing(&path, receipt, &bytes);
        }

        match write_record_create_only(&path, &bytes) {
            Ok(()) => Ok(GovernanceDecisionAuthorityReceiptWriteOutcome::Written),
            Err(error)
                if error.code() == "governance_decision_authority_receipt_store.record.exists" =>
            {
                Self::reconcile_existing(&path, receipt, &bytes)
            }
            Err(error) => Err(error),
        }
    }

    fn read_governance_decision_authority_receipt(
        &self,
        receipt_id: &GovernanceDecisionAuthorityReceiptId,
    ) -> Result<Option<PersistedGovernanceDecisionAuthorityReceiptRecord>, WorkflowOsError> {
        let path = self.record_path(receipt_id);
        let Some(bytes) = read_record_bytes_if_present(&path)? else {
            return Ok(None);
        };
        let record = deserialize_record(&bytes)?;
        if record.receipt_id() != receipt_id {
            return Err(store_error(
                "governance_decision_authority_receipt_store.read.identity_mismatch",
                "governance decision authority receipt storage identity does not match",
            ));
        }
        Ok(Some(record))
    }
}

impl std::fmt::Debug for LocalGovernanceDecisionAuthorityReceiptRecordStore {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LocalGovernanceDecisionAuthorityReceiptRecordStore")
            .field("root", &"[REDACTED]")
            .finish()
    }
}

fn deserialize_record(
    bytes: &[u8],
) -> Result<PersistedGovernanceDecisionAuthorityReceiptRecord, WorkflowOsError> {
    serde_json::from_slice(bytes).map_err(|_| invalid_record_error())
}

fn read_record_bytes(path: &Path) -> Result<Vec<u8>, WorkflowOsError> {
    read_record_bytes_if_present(path)?.ok_or_else(|| {
        store_error(
            "governance_decision_authority_receipt_store.read.failed",
            "failed to read governance decision authority receipt record",
        )
    })
}

fn read_record_bytes_if_present(path: &Path) -> Result<Option<Vec<u8>>, WorkflowOsError> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => {
            return Err(store_error(
                "governance_decision_authority_receipt_store.read.failed",
                "failed to read governance decision authority receipt record",
            ));
        }
    };
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(|_| {
        store_error(
            "governance_decision_authority_receipt_store.read.failed",
            "failed to read governance decision authority receipt record",
        )
    })?;
    Ok(Some(bytes))
}

fn write_record_create_only(path: &Path, bytes: &[u8]) -> Result<(), WorkflowOsError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|_| {
            store_error(
                "governance_decision_authority_receipt_store.write.failed",
                "failed to create governance decision authority receipt directory",
            )
        })?;
    }

    let temp_path = unique_temp_path(path);
    let result = write_temp_and_publish(&temp_path, path, bytes);
    let _ = fs::remove_file(temp_path);
    result
}

fn write_temp_and_publish(
    temp_path: &Path,
    path: &Path,
    bytes: &[u8],
) -> Result<(), WorkflowOsError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(temp_path)
        .map_err(|_| {
            store_error(
                "governance_decision_authority_receipt_store.write.failed",
                "failed to create governance decision authority receipt record",
            )
        })?;
    file.write_all(bytes).map_err(|_| {
        store_error(
            "governance_decision_authority_receipt_store.write.failed",
            "failed to write governance decision authority receipt record",
        )
    })?;
    file.sync_all().map_err(|_| {
        store_error(
            "governance_decision_authority_receipt_store.write.failed",
            "failed to sync governance decision authority receipt record",
        )
    })?;
    drop(file);

    fs::hard_link(temp_path, path).map_err(|error| {
        if error.kind() == std::io::ErrorKind::AlreadyExists {
            store_error(
                "governance_decision_authority_receipt_store.record.exists",
                "governance decision authority receipt record already exists",
            )
        } else {
            store_error(
                "governance_decision_authority_receipt_store.write.failed",
                "failed to publish governance decision authority receipt record",
            )
        }
    })
}

fn unique_temp_path(path: &Path) -> PathBuf {
    let mut temp_path = path.to_path_buf();
    let process_id = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    let sequence = TEMP_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    temp_path.set_extension(format!("tmp-{process_id}-{nanos}-{sequence}"));
    temp_path
}

fn encoded_id_file_name(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len() * 2 + 5);
    for byte in value.as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(&mut encoded, "{byte:02x}");
    }
    encoded.push_str(".json");
    encoded
}

fn invalid_record_error() -> WorkflowOsError {
    store_error(
        "governance_decision_authority_receipt_store.record.invalid",
        "governance decision authority receipt record is invalid",
    )
}

fn store_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::invalid_state(code, message)
}
