use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::{
    ImmutableRunBundleBuildResult, ImmutableRunBundleDefinitionRecord,
    ImmutableRunBundleDefinitionReference, ImmutableRunBundleId, ImmutableRunBundleManifest,
    SpecContentHash, WorkflowOsError, WorkflowRunId,
};

const DEFINITION_RECORDS_DIR: &str = "definition-records";
const MANIFESTS_DIR: &str = "manifests";

#[derive(Serialize, Deserialize)]
struct StoredManifestEnvelope {
    manifest: ImmutableRunBundleManifest,
    definition_record_hashes: Vec<SpecContentHash>,
}

/// Validated immutable run bundle loaded from a local create-only store.
#[derive(Clone, Eq, PartialEq)]
pub struct StoredImmutableRunBundle {
    manifest: ImmutableRunBundleManifest,
    definition_records: Vec<ImmutableRunBundleDefinitionRecord>,
}

impl StoredImmutableRunBundle {
    /// Returns the validated stored manifest.
    #[must_use]
    pub const fn manifest(&self) -> &ImmutableRunBundleManifest {
        &self.manifest
    }

    /// Returns the canonical records resolved for the manifest.
    #[must_use]
    pub fn definition_records(&self) -> &[ImmutableRunBundleDefinitionRecord] {
        &self.definition_records
    }

    /// Consumes the stored bundle into its validated parts.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        ImmutableRunBundleManifest,
        Vec<ImmutableRunBundleDefinitionRecord>,
    ) {
        (self.manifest, self.definition_records)
    }
}

impl std::fmt::Debug for StoredImmutableRunBundle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("StoredImmutableRunBundle")
            .field("manifest", &self.manifest)
            .field("definition_record_count", &self.definition_records.len())
            .finish_non_exhaustive()
    }
}

/// File-backed create-only store for immutable run-bundle material.
///
/// Canonical definition records are addressed by their canonical-record hash.
/// One manifest file is addressed by each run ID, making the run-to-bundle
/// binding create-only. Publishing a manifest is the bundle commit marker;
/// content-addressed records written before a failed manifest publication are
/// harmless immutable orphans and may be reused by a later valid bundle.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalImmutableRunBundleStore {
    root: PathBuf,
}

impl LocalImmutableRunBundleStore {
    /// Creates a local store rooted at a caller-provided bundle directory.
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Returns the store root.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Writes a canonical definition record if its content address is absent.
    ///
    /// Rewriting the identical validated record is idempotent. Existing corrupt
    /// or different content at the same address fails closed.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when validation, serialization, local
    /// I/O, or content-address integrity fails.
    pub fn write_definition_record_if_absent(
        &self,
        record: &ImmutableRunBundleDefinitionRecord,
    ) -> Result<(), WorkflowOsError> {
        let path = self.definition_record_path(record.canonical_record_hash());
        if path.exists() {
            return ensure_existing_definition_record(&path, record);
        }

        match write_json_create_new(&path, record) {
            Ok(()) => Ok(()),
            Err(error) if error.code() == "immutable_run_bundle_store.record_exists" => {
                ensure_existing_definition_record(&path, record)
            }
            Err(error) => Err(error),
        }
    }

    /// Reads and validates one canonical definition record by content address.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the record is missing, corrupt,
    /// invalid, or stored under a mismatched content address.
    pub fn read_definition_record(
        &self,
        canonical_record_hash: &SpecContentHash,
    ) -> Result<ImmutableRunBundleDefinitionRecord, WorkflowOsError> {
        let record: ImmutableRunBundleDefinitionRecord =
            read_json(&self.definition_record_path(canonical_record_hash))?;
        if record.canonical_record_hash() != canonical_record_hash {
            return Err(store_error(
                "immutable_run_bundle_store.identity_mismatch",
                "immutable definition record does not match its storage address",
            ));
        }
        Ok(record)
    }

    /// Writes a manifest create-only after all referenced definitions resolve.
    ///
    /// One manifest address exists per run ID. Any duplicate write is rejected,
    /// including an identical manifest, so a run cannot be rebound silently.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when referenced definitions are
    /// missing or ambiguous, the run is already bound, or local I/O fails.
    pub fn write_manifest_create_only(
        &self,
        manifest: &ImmutableRunBundleManifest,
    ) -> Result<(), WorkflowOsError> {
        let path = self.manifest_path(manifest.run_id());
        if path.exists() {
            return Err(store_error(
                "immutable_run_bundle_store.manifest_exists",
                "immutable run bundle manifest already exists for the run",
            ));
        }
        let records = self.resolve_definition_records(manifest)?;
        let envelope = StoredManifestEnvelope {
            manifest: manifest.clone(),
            definition_record_hashes: records
                .iter()
                .map(|record| record.canonical_record_hash().clone())
                .collect(),
        };
        write_json_create_new(&path, &envelope).map_err(|error| {
            if error.code() == "immutable_run_bundle_store.record_exists" {
                store_error(
                    "immutable_run_bundle_store.manifest_exists",
                    "immutable run bundle manifest already exists for the run",
                )
            } else {
                error
            }
        })
    }

    /// Persists canonical records and then publishes the manifest commit marker.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the build result is internally
    /// inconsistent, a record cannot be written, or the run is already bound.
    pub fn write_bundle(
        &self,
        bundle: &ImmutableRunBundleBuildResult,
    ) -> Result<(), WorkflowOsError> {
        let manifest_path = self.manifest_path(bundle.manifest().run_id());
        if manifest_path.exists() {
            return Err(store_error(
                "immutable_run_bundle_store.manifest_exists",
                "immutable run bundle manifest already exists for the run",
            ));
        }
        validate_supplied_records(bundle.manifest(), bundle.definition_records())?;
        for record in bundle.definition_records() {
            self.write_definition_record_if_absent(record)?;
        }
        self.write_manifest_create_only(bundle.manifest())
    }

    /// Reads one run-bound manifest and verifies its expected bundle identity.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the manifest is missing, corrupt,
    /// invalid, or does not match the supplied run and bundle identities.
    pub fn read_manifest(
        &self,
        run_id: &WorkflowRunId,
        bundle_id: &ImmutableRunBundleId,
    ) -> Result<ImmutableRunBundleManifest, WorkflowOsError> {
        let envelope = self.read_manifest_envelope(run_id, bundle_id)?;
        self.read_envelope_records(&envelope)?;
        Ok(envelope.manifest)
    }

    /// Reads a complete stored bundle and resolves every canonical record.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the manifest or any referenced
    /// definition is missing, corrupt, invalid, mismatched, or ambiguous.
    pub fn read_bundle(
        &self,
        run_id: &WorkflowRunId,
        bundle_id: &ImmutableRunBundleId,
    ) -> Result<StoredImmutableRunBundle, WorkflowOsError> {
        let envelope = self.read_manifest_envelope(run_id, bundle_id)?;
        let definition_records = self.read_envelope_records(&envelope)?;
        Ok(StoredImmutableRunBundle {
            manifest: envelope.manifest,
            definition_records,
        })
    }

    fn read_manifest_envelope(
        &self,
        run_id: &WorkflowRunId,
        bundle_id: &ImmutableRunBundleId,
    ) -> Result<StoredManifestEnvelope, WorkflowOsError> {
        let envelope: StoredManifestEnvelope = read_json(&self.manifest_path(run_id))?;
        if envelope.manifest.run_id() != run_id || envelope.manifest.bundle_id() != bundle_id {
            return Err(store_error(
                "immutable_run_bundle_store.identity_mismatch",
                "immutable run bundle manifest does not match its storage address",
            ));
        }
        Ok(envelope)
    }

    fn read_envelope_records(
        &self,
        envelope: &StoredManifestEnvelope,
    ) -> Result<Vec<ImmutableRunBundleDefinitionRecord>, WorkflowOsError> {
        let mut records = Vec::with_capacity(envelope.definition_record_hashes.len());
        for hash in &envelope.definition_record_hashes {
            records.push(self.read_definition_record(hash)?);
        }
        validate_supplied_records(&envelope.manifest, &records)?;
        records.sort_by(|left, right| {
            left.canonical_record_hash()
                .as_str()
                .cmp(right.canonical_record_hash().as_str())
        });
        Ok(records)
    }

    fn resolve_definition_records(
        &self,
        manifest: &ImmutableRunBundleManifest,
    ) -> Result<Vec<ImmutableRunBundleDefinitionRecord>, WorkflowOsError> {
        let stored_records = self.list_definition_records()?;
        resolve_manifest_records(manifest, &stored_records)
    }

    fn list_definition_records(
        &self,
    ) -> Result<Vec<ImmutableRunBundleDefinitionRecord>, WorkflowOsError> {
        let directory = self.definition_records_dir();
        if !directory.exists() {
            return Ok(Vec::new());
        }
        let mut records = Vec::new();
        for entry in fs::read_dir(directory).map_err(|_| {
            store_error(
                "immutable_run_bundle_store.read_dir_failed",
                "failed to read immutable definition record directory",
            )
        })? {
            let path = entry
                .map_err(|_| {
                    store_error(
                        "immutable_run_bundle_store.read_dir_failed",
                        "failed to read immutable definition record directory entry",
                    )
                })?
                .path();
            if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
                continue;
            }
            let record: ImmutableRunBundleDefinitionRecord = read_json(&path)?;
            let expected_name = hash_file_name(record.canonical_record_hash());
            if path.file_name().and_then(|name| name.to_str()) != Some(expected_name.as_str()) {
                return Err(store_error(
                    "immutable_run_bundle_store.identity_mismatch",
                    "immutable definition record does not match its storage address",
                ));
            }
            records.push(record);
        }
        records.sort_by(|left, right| {
            left.canonical_record_hash()
                .as_str()
                .cmp(right.canonical_record_hash().as_str())
        });
        Ok(records)
    }

    fn definition_records_dir(&self) -> PathBuf {
        self.root.join(DEFINITION_RECORDS_DIR)
    }

    fn manifests_dir(&self) -> PathBuf {
        self.root.join(MANIFESTS_DIR)
    }

    fn definition_record_path(&self, hash: &SpecContentHash) -> PathBuf {
        self.definition_records_dir().join(hash_file_name(hash))
    }

    fn manifest_path(&self, run_id: &WorkflowRunId) -> PathBuf {
        self.manifests_dir()
            .join(encoded_id_file_name(run_id.as_str()))
    }
}

impl std::fmt::Debug for LocalImmutableRunBundleStore {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LocalImmutableRunBundleStore")
            .field("root", &"[REDACTED]")
            .finish()
    }
}

fn validate_supplied_records(
    manifest: &ImmutableRunBundleManifest,
    records: &[ImmutableRunBundleDefinitionRecord],
) -> Result<(), WorkflowOsError> {
    let resolved = resolve_manifest_records(manifest, records)?;
    if resolved.len() != records.len() {
        return Err(store_error(
            "immutable_run_bundle_store.unreferenced_record",
            "immutable run bundle contains an unreferenced definition record",
        ));
    }
    Ok(())
}

fn resolve_manifest_records(
    manifest: &ImmutableRunBundleManifest,
    records: &[ImmutableRunBundleDefinitionRecord],
) -> Result<Vec<ImmutableRunBundleDefinitionRecord>, WorkflowOsError> {
    let mut resolved_hashes = BTreeSet::new();
    let mut resolved = Vec::new();
    for reference in manifest.definitions() {
        let matches = records
            .iter()
            .filter(|record| record_matches_reference(manifest, record, reference))
            .collect::<Vec<_>>();
        match matches.as_slice() {
            [] => {
                return Err(store_error(
                    "immutable_run_bundle_store.definition_missing",
                    "immutable run bundle definition record is missing",
                ));
            }
            [record] => {
                if resolved_hashes.insert(record.canonical_record_hash().clone()) {
                    resolved.push((*record).clone());
                }
            }
            _ => {
                return Err(store_error(
                    "immutable_run_bundle_store.definition_ambiguous",
                    "immutable run bundle definition reference is ambiguous",
                ));
            }
        }
    }
    resolved.sort_by(|left, right| {
        left.canonical_record_hash()
            .as_str()
            .cmp(right.canonical_record_hash().as_str())
    });
    Ok(resolved)
}

fn record_matches_reference(
    manifest: &ImmutableRunBundleManifest,
    record: &ImmutableRunBundleDefinitionRecord,
    reference: &ImmutableRunBundleDefinitionReference,
) -> bool {
    record.kind() == reference.kind()
        && record.definition_id() == reference.definition_id()
        && record.definition_version() == reference.definition_version()
        && record.schema_version() == reference.schema_version()
        && record.source_content_hash() == reference.content_hash()
        && record.record_version() == manifest.bundle_version()
        && record.sensitivity() == manifest.sensitivity()
        && record.redaction_required() == manifest.redaction_required()
}

fn ensure_existing_definition_record(
    path: &Path,
    expected: &ImmutableRunBundleDefinitionRecord,
) -> Result<(), WorkflowOsError> {
    let existing: ImmutableRunBundleDefinitionRecord = read_json(path)?;
    if &existing != expected || existing.canonical_record_hash() != expected.canonical_record_hash()
    {
        return Err(store_error(
            "immutable_run_bundle_store.record_conflict",
            "immutable definition record address contains different content",
        ));
    }
    Ok(())
}

fn read_json<T>(path: &Path) -> Result<T, WorkflowOsError>
where
    T: DeserializeOwned,
{
    let mut file = File::open(path).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            store_error(
                "immutable_run_bundle_store.not_found",
                "immutable run bundle record was not found",
            )
        } else {
            store_error(
                "immutable_run_bundle_store.read_failed",
                "failed to read immutable run bundle record",
            )
        }
    })?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(|_| {
        store_error(
            "immutable_run_bundle_store.read_failed",
            "failed to read immutable run bundle record",
        )
    })?;
    serde_json::from_slice(&bytes).map_err(|_| {
        store_error(
            "immutable_run_bundle_store.invalid_record",
            "immutable run bundle record is corrupt or invalid",
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
                "immutable_run_bundle_store.mkdir_failed",
                "failed to create immutable run bundle directory",
            )
        })?;
    }
    if path.exists() {
        return Err(store_error(
            "immutable_run_bundle_store.record_exists",
            "immutable run bundle record already exists",
        ));
    }

    let temp_path = unique_temp_path(path);
    let result = write_temp_and_publish(&temp_path, path, value);
    let _ = fs::remove_file(&temp_path);
    result
}

fn write_temp_and_publish<T>(
    temp_path: &Path,
    path: &Path,
    value: &T,
) -> Result<(), WorkflowOsError>
where
    T: Serialize,
{
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(temp_path)
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                store_error(
                    "immutable_run_bundle_store.temp_exists",
                    "temporary immutable run bundle record already exists",
                )
            } else {
                store_error(
                    "immutable_run_bundle_store.write_failed",
                    "failed to create immutable run bundle record",
                )
            }
        })?;
    let bytes = serde_json::to_vec_pretty(value).map_err(|_| {
        store_error(
            "immutable_run_bundle_store.serialize_failed",
            "failed to serialize immutable run bundle record",
        )
    })?;
    file.write_all(&bytes).map_err(|_| {
        store_error(
            "immutable_run_bundle_store.write_failed",
            "failed to write immutable run bundle record",
        )
    })?;
    file.sync_all().map_err(|_| {
        store_error(
            "immutable_run_bundle_store.sync_failed",
            "failed to sync immutable run bundle record",
        )
    })?;
    drop(file);

    fs::hard_link(temp_path, path).map_err(|error| {
        if error.kind() == std::io::ErrorKind::AlreadyExists {
            store_error(
                "immutable_run_bundle_store.record_exists",
                "immutable run bundle record already exists",
            )
        } else {
            store_error(
                "immutable_run_bundle_store.publish_failed",
                "failed to publish immutable run bundle record atomically",
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
    temp_path.set_extension(format!("tmp-{process_id}-{nanos}"));
    temp_path
}

fn hash_file_name(hash: &SpecContentHash) -> String {
    format!("{}.json", hash.as_str())
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
