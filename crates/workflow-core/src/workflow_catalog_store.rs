use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::{
    validate_workflow_catalog_repair_proposal_review_matches, WorkflowArchiveRecord,
    WorkflowArchiveRecordId, WorkflowCatalogRecord, WorkflowCatalogRecordId,
    WorkflowCatalogRepairProposal, WorkflowCatalogRepairProposalReview,
    WorkflowCatalogRepairProposalReviewId, WorkflowId, WorkflowOsError,
    WorkflowStewardshipDecisionId, WorkflowStewardshipRecord,
};

const WORKFLOWS_DIR: &str = "workflows";
const STEWARDSHIP_DIR: &str = "stewardship";
const ARCHIVES_DIR: &str = "archives";
const REPAIR_REVIEWS_DIR: &str = "repair-reviews";

/// File-backed local store for workflow catalog metadata.
///
/// The store persists only validated catalog, stewardship, archive, and repair
/// review metadata. It does not register workflows with the runtime, integrate
/// authoring commands, execute local checks, mutate provider state, apply
/// repairs, or run Git commands.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalWorkflowCatalogStore {
    root: PathBuf,
}

impl LocalWorkflowCatalogStore {
    /// Creates a local workflow catalog store rooted at `.workflow-os/catalog`
    /// or an equivalent caller-provided catalog directory.
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Returns the catalog root path.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Writes a catalog record if it does not already exist.
    ///
    /// # Errors
    ///
    /// Returns a structured non-leaking error when validation, serialization,
    /// directory creation, or atomic file creation fails.
    pub fn write_catalog_record_if_absent(
        &self,
        record: &WorkflowCatalogRecord,
    ) -> Result<(), WorkflowOsError> {
        record.validate()?;
        write_json_create_new(&self.catalog_record_path(record.record_id()), record)
    }

    /// Reads a catalog record by id.
    ///
    /// # Errors
    ///
    /// Returns a structured non-leaking error when the record is missing,
    /// corrupt, invalid, or stored under a mismatched address.
    pub fn read_catalog_record(
        &self,
        record_id: &WorkflowCatalogRecordId,
    ) -> Result<WorkflowCatalogRecord, WorkflowOsError> {
        let path = self.catalog_record_path(record_id);
        let record: WorkflowCatalogRecord = read_json(&path)?;
        if record.record_id() != record_id {
            return Err(store_error(
                "workflow_catalog_store.identity_mismatch",
                "workflow catalog record identity does not match its storage address",
            ));
        }
        Ok(record)
    }

    /// Lists catalog records in deterministic record-id order.
    ///
    /// # Errors
    ///
    /// Returns a structured non-leaking error when any stored record is corrupt,
    /// invalid, or stored under a mismatched address.
    pub fn list_catalog_records(&self) -> Result<Vec<WorkflowCatalogRecord>, WorkflowOsError> {
        let mut records = Vec::new();
        for path in json_paths_in_dir(&self.workflows_dir())? {
            let record: WorkflowCatalogRecord = read_json(&path)?;
            ensure_path_matches_id(&path, record.record_id().as_str())?;
            records.push(record);
        }
        records.sort_by(|left, right| left.record_id().as_str().cmp(right.record_id().as_str()));
        Ok(records)
    }

    /// Writes a stewardship record if it does not already exist.
    ///
    /// # Errors
    ///
    /// Returns a structured non-leaking error when validation, serialization,
    /// directory creation, or atomic file creation fails.
    pub fn write_stewardship_record_if_absent(
        &self,
        record: &WorkflowStewardshipRecord,
    ) -> Result<(), WorkflowOsError> {
        write_json_create_new(&self.stewardship_record_path(record.decision_id()), record)
    }

    /// Reads a stewardship record by id.
    ///
    /// # Errors
    ///
    /// Returns a structured non-leaking error when the record is missing,
    /// corrupt, invalid, or stored under a mismatched address.
    pub fn read_stewardship_record(
        &self,
        decision_id: &WorkflowStewardshipDecisionId,
    ) -> Result<WorkflowStewardshipRecord, WorkflowOsError> {
        let path = self.stewardship_record_path(decision_id);
        let record: WorkflowStewardshipRecord = read_json(&path)?;
        if record.decision_id() != decision_id {
            return Err(store_error(
                "workflow_catalog_store.identity_mismatch",
                "workflow stewardship record identity does not match its storage address",
            ));
        }
        Ok(record)
    }

    /// Lists stewardship records for a workflow in deterministic decision-id
    /// order.
    ///
    /// # Errors
    ///
    /// Returns a structured non-leaking error when any stored record is corrupt,
    /// invalid, or stored under a mismatched address.
    pub fn list_stewardship_records_for_workflow(
        &self,
        workflow_id: &WorkflowId,
    ) -> Result<Vec<WorkflowStewardshipRecord>, WorkflowOsError> {
        let mut records = Vec::new();
        for path in json_paths_in_dir(&self.stewardship_dir())? {
            let record: WorkflowStewardshipRecord = read_json(&path)?;
            ensure_path_matches_id(&path, record.decision_id().as_str())?;
            if record.workflow_id() == workflow_id {
                records.push(record);
            }
        }
        records.sort_by(|left, right| {
            left.decision_id()
                .as_str()
                .cmp(right.decision_id().as_str())
        });
        Ok(records)
    }

    /// Writes an archive record if it does not already exist.
    ///
    /// # Errors
    ///
    /// Returns a structured non-leaking error when validation, serialization,
    /// directory creation, or atomic file creation fails.
    pub fn write_archive_record_if_absent(
        &self,
        record: &WorkflowArchiveRecord,
    ) -> Result<(), WorkflowOsError> {
        write_json_create_new(
            &self.archive_record_path(record.archive_record_id()),
            record,
        )
    }

    /// Reads an archive record by id.
    ///
    /// # Errors
    ///
    /// Returns a structured non-leaking error when the record is missing,
    /// corrupt, invalid, or stored under a mismatched address.
    pub fn read_archive_record(
        &self,
        archive_record_id: &WorkflowArchiveRecordId,
    ) -> Result<WorkflowArchiveRecord, WorkflowOsError> {
        let path = self.archive_record_path(archive_record_id);
        let record: WorkflowArchiveRecord = read_json(&path)?;
        if record.archive_record_id() != archive_record_id {
            return Err(store_error(
                "workflow_catalog_store.identity_mismatch",
                "workflow archive record identity does not match its storage address",
            ));
        }
        Ok(record)
    }

    /// Lists archive records in deterministic archive-record-id order.
    ///
    /// # Errors
    ///
    /// Returns a structured non-leaking error when any stored record is corrupt,
    /// invalid, or stored under a mismatched address.
    pub fn list_archive_records(&self) -> Result<Vec<WorkflowArchiveRecord>, WorkflowOsError> {
        let mut records = Vec::new();
        for path in json_paths_in_dir(&self.archives_dir())? {
            let record: WorkflowArchiveRecord = read_json(&path)?;
            ensure_path_matches_id(&path, record.archive_record_id().as_str())?;
            records.push(record);
        }
        records.sort_by(|left, right| {
            left.archive_record_id()
                .as_str()
                .cmp(right.archive_record_id().as_str())
        });
        Ok(records)
    }

    /// Writes a repair proposal review sidecar if it does not already exist
    /// and still matches a fresh proposal identity.
    ///
    /// # Errors
    ///
    /// Returns a structured non-leaking error when the review is stale,
    /// duplicated, invalid, or cannot be persisted.
    pub fn write_repair_review_record_if_absent(
        &self,
        review: &WorkflowCatalogRepairProposalReview,
        fresh_proposal: &WorkflowCatalogRepairProposal,
    ) -> Result<(), WorkflowOsError> {
        validate_workflow_catalog_repair_proposal_review_matches(review, fresh_proposal).map_err(
            |_| {
                store_error(
                    "workflow_catalog.repair_review_store.stale_proposal",
                    "workflow catalog repair review no longer matches the fresh proposal identity",
                )
            },
        )?;
        let path = self.repair_review_record_path(review.review_id());
        if path.exists() {
            return Err(store_error(
                "workflow_catalog.repair_review_store.duplicate_review",
                "workflow catalog repair review already exists",
            ));
        }
        write_json_create_new(&path, review)
    }

    /// Reads a repair proposal review sidecar by id.
    ///
    /// # Errors
    ///
    /// Returns a structured non-leaking error when the record is missing,
    /// corrupt, invalid, or stored under a mismatched address.
    pub fn read_repair_review_record(
        &self,
        review_id: &WorkflowCatalogRepairProposalReviewId,
    ) -> Result<WorkflowCatalogRepairProposalReview, WorkflowOsError> {
        let path = self.repair_review_record_path(review_id);
        let record: WorkflowCatalogRepairProposalReview = read_json(&path)?;
        if record.review_id() != review_id {
            return Err(store_error(
                "workflow_catalog_store.identity_mismatch",
                "workflow catalog repair review identity does not match its storage address",
            ));
        }
        Ok(record)
    }

    /// Lists repair proposal review sidecars in deterministic review-id order.
    ///
    /// # Errors
    ///
    /// Returns a structured non-leaking error when any stored review is
    /// corrupt, invalid, or stored under a mismatched address.
    pub fn list_repair_review_records(
        &self,
    ) -> Result<Vec<WorkflowCatalogRepairProposalReview>, WorkflowOsError> {
        let mut records = Vec::new();
        for path in json_paths_in_dir(&self.repair_reviews_dir())? {
            let record: WorkflowCatalogRepairProposalReview = read_json(&path)?;
            ensure_path_matches_id(&path, record.review_id().as_str())?;
            records.push(record);
        }
        records.sort_by(|left, right| left.review_id().as_str().cmp(right.review_id().as_str()));
        Ok(records)
    }

    /// Returns a bounded health summary for the local catalog store.
    ///
    /// # Errors
    ///
    /// Returns a structured non-leaking error when directory traversal or record
    /// validation fails.
    pub fn health_check(&self) -> Result<WorkflowCatalogStoreHealth, WorkflowOsError> {
        let catalog_records = self.list_catalog_records()?.len();
        let stewardship_records = {
            let mut count = 0;
            for path in json_paths_in_dir(&self.stewardship_dir())? {
                let record: WorkflowStewardshipRecord = read_json(&path)?;
                ensure_path_matches_id(&path, record.decision_id().as_str())?;
                count += 1;
            }
            count
        };
        let archive_records = self.list_archive_records()?.len();
        let repair_review_records = self.list_repair_review_records()?.len();

        Ok(WorkflowCatalogStoreHealth {
            root_exists: self.root.exists(),
            catalog_records,
            stewardship_records,
            archive_records,
            repair_review_records,
        })
    }

    fn workflows_dir(&self) -> PathBuf {
        self.root.join(WORKFLOWS_DIR)
    }

    fn stewardship_dir(&self) -> PathBuf {
        self.root.join(STEWARDSHIP_DIR)
    }

    fn archives_dir(&self) -> PathBuf {
        self.root.join(ARCHIVES_DIR)
    }

    fn repair_reviews_dir(&self) -> PathBuf {
        self.root.join(REPAIR_REVIEWS_DIR)
    }

    fn catalog_record_path(&self, record_id: &WorkflowCatalogRecordId) -> PathBuf {
        self.workflows_dir()
            .join(encoded_id_file_name(record_id.as_str()))
    }

    fn stewardship_record_path(&self, decision_id: &WorkflowStewardshipDecisionId) -> PathBuf {
        self.stewardship_dir()
            .join(encoded_id_file_name(decision_id.as_str()))
    }

    fn archive_record_path(&self, archive_record_id: &WorkflowArchiveRecordId) -> PathBuf {
        self.archives_dir()
            .join(encoded_id_file_name(archive_record_id.as_str()))
    }

    fn repair_review_record_path(
        &self,
        review_id: &WorkflowCatalogRepairProposalReviewId,
    ) -> PathBuf {
        self.repair_reviews_dir()
            .join(encoded_id_file_name(review_id.as_str()))
    }
}

impl std::fmt::Debug for LocalWorkflowCatalogStore {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LocalWorkflowCatalogStore")
            .field("root", &"[REDACTED]")
            .finish()
    }
}

/// Bounded health summary for the local workflow catalog store.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkflowCatalogStoreHealth {
    root_exists: bool,
    catalog_records: usize,
    stewardship_records: usize,
    archive_records: usize,
    repair_review_records: usize,
}

impl WorkflowCatalogStoreHealth {
    /// Returns whether the catalog root currently exists.
    #[must_use]
    pub const fn root_exists(&self) -> bool {
        self.root_exists
    }

    /// Returns the number of valid catalog records.
    #[must_use]
    pub const fn catalog_records(&self) -> usize {
        self.catalog_records
    }

    /// Returns the number of valid stewardship records.
    #[must_use]
    pub const fn stewardship_records(&self) -> usize {
        self.stewardship_records
    }

    /// Returns the number of valid archive records.
    #[must_use]
    pub const fn archive_records(&self) -> usize {
        self.archive_records
    }

    /// Returns the number of valid repair review records.
    #[must_use]
    pub const fn repair_review_records(&self) -> usize {
        self.repair_review_records
    }
}

fn json_paths_in_dir(directory: &Path) -> Result<Vec<PathBuf>, WorkflowOsError> {
    if !directory.exists() {
        return Ok(Vec::new());
    }

    let mut paths = Vec::new();
    for entry in fs::read_dir(directory).map_err(|_| {
        store_error(
            "workflow_catalog_store.read_dir_failed",
            "failed to read workflow catalog directory",
        )
    })? {
        let entry = entry.map_err(|_| {
            store_error(
                "workflow_catalog_store.read_dir_failed",
                "failed to read workflow catalog directory entry",
            )
        })?;
        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) == Some("json") {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths)
}

fn read_json<T>(path: &Path) -> Result<T, WorkflowOsError>
where
    T: DeserializeOwned,
{
    let mut file = File::open(path).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            store_error(
                "workflow_catalog_store.not_found",
                "workflow catalog record was not found",
            )
        } else {
            store_error(
                "workflow_catalog_store.read_failed",
                "failed to read workflow catalog record",
            )
        }
    })?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(|_| {
        store_error(
            "workflow_catalog_store.read_failed",
            "failed to read workflow catalog record",
        )
    })?;
    serde_json::from_slice(&bytes).map_err(|_| {
        store_error(
            "workflow_catalog_store.invalid_record",
            "workflow catalog record is corrupt or invalid",
        )
    })
}

fn write_json_create_new<T>(path: &Path, value: &T) -> Result<(), WorkflowOsError>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|_| {
            store_error(
                "workflow_catalog_store.mkdir_failed",
                "failed to create workflow catalog directory",
            )
        })?;
    }

    if path.exists() {
        return Err(store_error(
            "workflow_catalog_store.record_exists",
            "workflow catalog record already exists",
        ));
    }

    let temp_path = unique_temp_path(path);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp_path)
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                store_error(
                    "workflow_catalog_store.temp_exists",
                    "temporary workflow catalog record already exists",
                )
            } else {
                store_error(
                    "workflow_catalog_store.write_failed",
                    "failed to create workflow catalog record",
                )
            }
        })?;

    let bytes = serde_json::to_vec_pretty(value).map_err(|_| {
        store_error(
            "workflow_catalog_store.serialize_failed",
            "failed to serialize workflow catalog record",
        )
    })?;
    file.write_all(&bytes).map_err(|_| {
        store_error(
            "workflow_catalog_store.write_failed",
            "failed to write workflow catalog record",
        )
    })?;
    file.sync_all().map_err(|_| {
        store_error(
            "workflow_catalog_store.sync_failed",
            "failed to sync workflow catalog record",
        )
    })?;
    drop(file);

    match fs::hard_link(&temp_path, path) {
        Ok(()) => {
            let _ = fs::remove_file(&temp_path);
            Ok(())
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let _ = fs::remove_file(&temp_path);
            Err(store_error(
                "workflow_catalog_store.record_exists",
                "workflow catalog record already exists",
            ))
        }
        Err(_) => {
            let _ = fs::remove_file(&temp_path);
            Err(store_error(
                "workflow_catalog_store.publish_failed",
                "failed to publish workflow catalog record atomically",
            ))
        }
    }
}

fn unique_temp_path(path: &Path) -> PathBuf {
    let mut temp_path = path.to_path_buf();
    let process_id = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    temp_path.set_extension(format!("tmp-{process_id}-{nanos}"));
    temp_path
}

fn ensure_path_matches_id(path: &Path, id: &str) -> Result<(), WorkflowOsError> {
    let expected_file_name = encoded_id_file_name(id);
    if path.file_name().and_then(|name| name.to_str()) != Some(expected_file_name.as_str()) {
        return Err(store_error(
            "workflow_catalog_store.identity_mismatch",
            "workflow catalog record identity does not match its storage address",
        ));
    }
    Ok(())
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

fn store_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::invalid_state(code, message)
}
