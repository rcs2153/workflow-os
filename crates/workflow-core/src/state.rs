use std::fmt::Write as _;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    ActorId, ApprovalRequest, EventId, EventSequenceNumber, IdempotencyKey, ProjectId,
    WorkflowOsError, WorkflowOsErrorKind, WorkflowRun, WorkflowRunEvent, WorkflowRunId,
    WorkflowRunSnapshot,
};

/// Durable event log contract.
pub trait EventLogStore {
    /// Appends one event to the durable event log.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the event ID or run sequence number is
    /// duplicated, storage is corrupt, or the backend cannot durably write.
    fn append_event(&self, event: &WorkflowRunEvent) -> Result<(), WorkflowOsError>;

    /// Reads all events for a run ordered by sequence number.
    ///
    /// # Errors
    ///
    /// Returns a structured error when stored event data cannot be read or decoded.
    fn read_events(&self, run_id: &WorkflowRunId)
        -> Result<Vec<WorkflowRunEvent>, WorkflowOsError>;
}

/// Durable run snapshot projection contract.
pub trait RunSnapshotStore {
    /// Saves a snapshot projection.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the backend cannot durably write.
    fn save_snapshot(&self, snapshot: &WorkflowRunSnapshot) -> Result<(), WorkflowOsError>;

    /// Loads a snapshot projection, if present.
    ///
    /// # Errors
    ///
    /// Returns a structured error when stored snapshot data is corrupt.
    fn load_snapshot(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Option<WorkflowRunSnapshot>, WorkflowOsError>;
}

/// Durable idempotency contract.
pub trait IdempotencyStore {
    /// Records an idempotency result if the key has not been seen before.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the backend cannot durably write or read.
    fn record_idempotency_result(
        &self,
        key: &IdempotencyKey,
        result: IdempotencyResult,
    ) -> Result<IdempotencyWrite, WorkflowOsError>;
}

/// Durable local lock contract.
pub trait LockStore {
    /// Acquires a lock for a caller.
    ///
    /// # Errors
    ///
    /// Returns `state.lock_contended` when the lock is already held.
    fn acquire_lock(&self, key: &str, owner: &ActorId) -> Result<LockLease, WorkflowOsError>;

    /// Releases a lock.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the lock cannot be released.
    fn release_lock(&self, lease: &LockLease) -> Result<(), WorkflowOsError>;
}

/// Approval projection contract. Approval truth remains in the event log.
pub trait ApprovalStore {
    /// Saves an approval request projection.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the backend cannot durably write.
    fn save_approval_request(&self, request: &ApprovalRequest) -> Result<(), WorkflowOsError>;

    /// Loads an approval request projection.
    ///
    /// # Errors
    ///
    /// Returns a structured error when stored approval data is corrupt.
    fn load_approval_request(
        &self,
        approval_id: &str,
    ) -> Result<Option<ApprovalRequest>, WorkflowOsError>;
}

/// Local project-state metadata contract.
pub trait ProjectStateStore {
    /// Saves non-secret project metadata.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the backend cannot durably write.
    fn save_project_state(&self, state: &ProjectStateRecord) -> Result<(), WorkflowOsError>;

    /// Loads non-secret project metadata.
    ///
    /// # Errors
    ///
    /// Returns a structured error when stored metadata is corrupt.
    fn load_project_state(
        &self,
        project_id: &ProjectId,
    ) -> Result<Option<ProjectStateRecord>, WorkflowOsError>;
}

/// Aggregate state backend contract.
pub trait StateBackend:
    EventLogStore + RunSnapshotStore + IdempotencyStore + LockStore + ApprovalStore + ProjectStateStore
{
    /// Runs a backend health check.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the backend cannot check health.
    fn health_check(&self) -> Result<BackendHealthCheck, WorkflowOsError>;

    /// Rehydrates a workflow run from durable events.
    ///
    /// # Errors
    ///
    /// Returns a structured error when events cannot be read or replayed.
    fn rehydrate_run(&self, run_id: &WorkflowRunId) -> Result<WorkflowRun, WorkflowOsError> {
        let events = self.read_events(run_id)?;
        WorkflowRun::rehydrate(&events)
    }
}

/// Backend health check result.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BackendHealthCheck {
    /// Whether the backend is available.
    pub healthy: bool,
    /// Human-readable backend kind.
    pub backend: String,
    /// Non-secret status message.
    pub message: String,
}

/// Result stored for an idempotency key.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IdempotencyResult {
    /// Non-secret result reference or summary.
    pub result_ref: String,
}

/// Result of an idempotency write.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum IdempotencyWrite {
    /// The key was written for the first time.
    FirstWrite(IdempotencyResult),
    /// The key already existed and returned the stored result.
    Duplicate(IdempotencyResult),
}

/// Local lock lease.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LockLease {
    /// Lock key.
    pub key: String,
    /// Lock owner.
    pub owner: ActorId,
}

/// Non-secret local project state metadata.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProjectStateRecord {
    /// Project ID.
    pub project_id: ProjectId,
    /// Non-secret metadata summary.
    pub metadata: String,
}

/// Local filesystem development backend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalStateBackend {
    root: PathBuf,
}

impl LocalStateBackend {
    /// Creates a local filesystem backend rooted at `root`.
    ///
    /// # Errors
    ///
    /// Returns an error when required directories cannot be created.
    pub fn new(root: impl Into<PathBuf>) -> Result<Self, WorkflowOsError> {
        let backend = Self { root: root.into() };
        backend.ensure_layout()?;
        Ok(backend)
    }

    /// Returns the backend root.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    fn ensure_layout(&self) -> Result<(), WorkflowOsError> {
        for directory in [
            self.events_dir(),
            self.event_ids_dir(),
            self.snapshots_dir(),
            self.idempotency_dir(),
            self.locks_dir(),
            self.approvals_dir(),
            self.projects_dir(),
        ] {
            fs::create_dir_all(&directory).map_err(|error| {
                state_error(
                    "state.local.mkdir",
                    format!(
                        "failed to create state directory {}: {error}",
                        directory.display()
                    ),
                )
            })?;
        }
        Ok(())
    }

    fn events_dir(&self) -> PathBuf {
        self.root.join("events")
    }

    fn event_ids_dir(&self) -> PathBuf {
        self.root.join("event_ids")
    }

    fn snapshots_dir(&self) -> PathBuf {
        self.root.join("snapshots")
    }

    fn idempotency_dir(&self) -> PathBuf {
        self.root.join("idempotency")
    }

    fn locks_dir(&self) -> PathBuf {
        self.root.join("locks")
    }

    fn approvals_dir(&self) -> PathBuf {
        self.root.join("approvals")
    }

    fn projects_dir(&self) -> PathBuf {
        self.root.join("projects")
    }

    fn run_events_dir(&self, run_id: &WorkflowRunId) -> PathBuf {
        self.events_dir().join(encode_key(run_id.as_str()))
    }

    fn event_path(&self, event: &WorkflowRunEvent) -> PathBuf {
        self.run_events_dir(&event.run_id)
            .join(format!("{:020}.json", event.sequence_number.get()))
    }

    fn event_id_path(&self, event_id: &EventId) -> PathBuf {
        self.event_ids_dir()
            .join(format!("{}.json", encode_key(event_id.as_str())))
    }

    fn with_local_lock<T>(
        &self,
        lock_key: &str,
        owner: &ActorId,
        operation: impl FnOnce() -> Result<T, WorkflowOsError>,
    ) -> Result<T, WorkflowOsError> {
        let lease = self.acquire_lock(lock_key, owner)?;
        let result = operation();
        let release = self.release_lock(&lease);
        match (result, release) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(error), _) | (Ok(_), Err(error)) => Err(error),
        }
    }
}

impl EventLogStore for LocalStateBackend {
    fn append_event(&self, event: &WorkflowRunEvent) -> Result<(), WorkflowOsError> {
        let owner = match &event.actor {
            Some(actor) => actor.clone(),
            None => ActorId::new("system").map_err(|error| {
                state_error(
                    "state.actor.invalid",
                    format!("failed to build system lock actor: {error}"),
                )
            })?,
        };
        self.with_local_lock("event-log", &owner, || {
            self.ensure_layout()?;
            let run_dir = self.run_events_dir(&event.run_id);
            fs::create_dir_all(&run_dir).map_err(|error| {
                state_error(
                    "state.local.mkdir",
                    format!("failed to create run event directory: {error}"),
                )
            })?;

            let event_path = self.event_path(event);
            let event_id_path = self.event_id_path(&event.event_id);
            if event_id_path.exists() {
                return Err(state_error(
                    "state.event.duplicate_id",
                    format!("duplicate event id {}", event.event_id),
                ));
            }
            if event_path.exists() {
                return Err(state_error(
                    "state.event.duplicate_sequence",
                    format!(
                        "duplicate sequence {} for run {}",
                        event.sequence_number, event.run_id
                    ),
                ));
            }

            write_json_create_new(&event_path, event)?;
            write_json_create_new(
                &event_id_path,
                &EventIdRecord {
                    run_id: event.run_id.clone(),
                    sequence_number: event.sequence_number,
                },
            )?;
            Ok(())
        })
    }

    fn read_events(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Vec<WorkflowRunEvent>, WorkflowOsError> {
        let run_dir = self.run_events_dir(run_id);
        if !run_dir.exists() {
            return Ok(Vec::new());
        }
        let mut paths = Vec::new();
        for entry in fs::read_dir(&run_dir).map_err(|error| {
            state_error(
                "state.local.read_dir",
                format!(
                    "failed to read event directory {}: {error}",
                    run_dir.display()
                ),
            )
        })? {
            let entry = entry.map_err(|error| {
                state_error(
                    "state.local.read_dir_entry",
                    format!("failed to inspect event directory entry: {error}"),
                )
            })?;
            let path = entry.path();
            if path.is_file() {
                paths.push(path);
            }
        }
        paths.sort();
        paths
            .iter()
            .map(|path| read_json(path))
            .collect::<Result<Vec<_>, _>>()
    }
}

impl RunSnapshotStore for LocalStateBackend {
    fn save_snapshot(&self, snapshot: &WorkflowRunSnapshot) -> Result<(), WorkflowOsError> {
        self.ensure_layout()?;
        write_json_replace(
            &self.snapshots_dir().join(format!(
                "{}.json",
                encode_key(snapshot.identity.run_id.as_str())
            )),
            snapshot,
        )
    }

    fn load_snapshot(
        &self,
        run_id: &WorkflowRunId,
    ) -> Result<Option<WorkflowRunSnapshot>, WorkflowOsError> {
        read_optional_json(
            &self
                .snapshots_dir()
                .join(format!("{}.json", encode_key(run_id.as_str()))),
        )
    }
}

impl IdempotencyStore for LocalStateBackend {
    fn record_idempotency_result(
        &self,
        key: &IdempotencyKey,
        result: IdempotencyResult,
    ) -> Result<IdempotencyWrite, WorkflowOsError> {
        self.ensure_layout()?;
        let path = self
            .idempotency_dir()
            .join(format!("{}.json", encode_key(key.as_str())));
        if path.exists() {
            return Ok(IdempotencyWrite::Duplicate(read_json(&path)?));
        }
        match write_json_create_new(&path, &result) {
            Ok(()) => Ok(IdempotencyWrite::FirstWrite(result)),
            Err(error) if error.code() == "state.local.exists" => {
                Ok(IdempotencyWrite::Duplicate(read_json(&path)?))
            }
            Err(error) => Err(error),
        }
    }
}

impl LockStore for LocalStateBackend {
    fn acquire_lock(&self, key: &str, owner: &ActorId) -> Result<LockLease, WorkflowOsError> {
        self.ensure_layout()?;
        let path = self.locks_dir().join(encode_key(key));
        match fs::create_dir(&path) {
            Ok(()) => {
                write_json_replace(
                    &path.join("owner.json"),
                    &LockLease {
                        key: key.to_owned(),
                        owner: owner.clone(),
                    },
                )?;
                Ok(LockLease {
                    key: key.to_owned(),
                    owner: owner.clone(),
                })
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => Err(state_error(
                "state.lock_contended",
                format!("lock {key} is already held"),
            )),
            Err(error) => Err(state_error(
                "state.lock.acquire",
                format!("failed to acquire lock {key}: {error}"),
            )),
        }
    }

    fn release_lock(&self, lease: &LockLease) -> Result<(), WorkflowOsError> {
        let path = self.locks_dir().join(encode_key(&lease.key));
        if !path.exists() {
            return Ok(());
        }
        fs::remove_dir_all(&path).map_err(|error| {
            state_error(
                "state.lock.release",
                format!("failed to release lock {}: {error}", lease.key),
            )
        })
    }
}

impl ApprovalStore for LocalStateBackend {
    fn save_approval_request(&self, request: &ApprovalRequest) -> Result<(), WorkflowOsError> {
        self.ensure_layout()?;
        write_json_replace(
            &self
                .approvals_dir()
                .join(format!("{}.json", encode_key(&request.approval_id))),
            request,
        )
    }

    fn load_approval_request(
        &self,
        approval_id: &str,
    ) -> Result<Option<ApprovalRequest>, WorkflowOsError> {
        read_optional_json(
            &self
                .approvals_dir()
                .join(format!("{}.json", encode_key(approval_id))),
        )
    }
}

impl ProjectStateStore for LocalStateBackend {
    fn save_project_state(&self, state: &ProjectStateRecord) -> Result<(), WorkflowOsError> {
        self.ensure_layout()?;
        write_json_replace(
            &self
                .projects_dir()
                .join(format!("{}.json", encode_key(state.project_id.as_str()))),
            state,
        )
    }

    fn load_project_state(
        &self,
        project_id: &ProjectId,
    ) -> Result<Option<ProjectStateRecord>, WorkflowOsError> {
        read_optional_json(
            &self
                .projects_dir()
                .join(format!("{}.json", encode_key(project_id.as_str()))),
        )
    }
}

impl StateBackend for LocalStateBackend {
    fn health_check(&self) -> Result<BackendHealthCheck, WorkflowOsError> {
        self.ensure_layout()?;
        let probe = self.root.join(".healthcheck");
        write_json_replace(
            &probe,
            &BackendHealthCheck {
                healthy: true,
                backend: "local_filesystem".to_owned(),
                message: "local state backend is writable".to_owned(),
            },
        )?;
        let _ = fs::remove_file(probe);
        Ok(BackendHealthCheck {
            healthy: true,
            backend: "local_filesystem".to_owned(),
            message: "local state backend is writable".to_owned(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct EventIdRecord {
    run_id: WorkflowRunId,
    sequence_number: EventSequenceNumber,
}

fn write_json_create_new<T>(path: &Path, value: &T) -> Result<(), WorkflowOsError>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            state_error(
                "state.local.mkdir",
                format!(
                    "failed to create parent directory {}: {error}",
                    parent.display()
                ),
            )
        })?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                state_error(
                    "state.local.exists",
                    format!("state file already exists: {}", path.display()),
                )
            } else {
                state_error(
                    "state.local.write",
                    format!("failed to create state file {}: {error}", path.display()),
                )
            }
        })?;
    let bytes = serde_json::to_vec_pretty(value).map_err(|error| {
        state_error(
            "state.json.serialize",
            format!("failed to serialize state JSON: {error}"),
        )
    })?;
    file.write_all(&bytes).map_err(|error| {
        state_error(
            "state.local.write",
            format!("failed to write state file {}: {error}", path.display()),
        )
    })?;
    file.sync_all().map_err(|error| {
        state_error(
            "state.local.sync",
            format!("failed to sync state file {}: {error}", path.display()),
        )
    })
}

fn write_json_replace<T>(path: &Path, value: &T) -> Result<(), WorkflowOsError>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            state_error(
                "state.local.mkdir",
                format!(
                    "failed to create parent directory {}: {error}",
                    parent.display()
                ),
            )
        })?;
    }
    let temp_path = path.with_extension("tmp");
    {
        let mut file = File::create(&temp_path).map_err(|error| {
            state_error(
                "state.local.write",
                format!(
                    "failed to create temp state file {}: {error}",
                    temp_path.display()
                ),
            )
        })?;
        let bytes = serde_json::to_vec_pretty(value).map_err(|error| {
            state_error(
                "state.json.serialize",
                format!("failed to serialize state JSON: {error}"),
            )
        })?;
        file.write_all(&bytes).map_err(|error| {
            state_error(
                "state.local.write",
                format!(
                    "failed to write temp state file {}: {error}",
                    temp_path.display()
                ),
            )
        })?;
        file.sync_all().map_err(|error| {
            state_error(
                "state.local.sync",
                format!(
                    "failed to sync temp state file {}: {error}",
                    temp_path.display()
                ),
            )
        })?;
    }
    fs::rename(&temp_path, path).map_err(|error| {
        state_error(
            "state.local.rename",
            format!(
                "failed to replace state file {} with {}: {error}",
                path.display(),
                temp_path.display()
            ),
        )
    })
}

fn read_optional_json<T>(path: &Path) -> Result<Option<T>, WorkflowOsError>
where
    T: for<'de> Deserialize<'de>,
{
    if !path.exists() {
        return Ok(None);
    }
    read_json(path).map(Some)
}

fn read_json<T>(path: &Path) -> Result<T, WorkflowOsError>
where
    T: for<'de> Deserialize<'de>,
{
    let mut text = String::new();
    File::open(path)
        .map_err(|error| {
            state_error(
                "state.local.read",
                format!("failed to open state file {}: {error}", path.display()),
            )
        })?
        .read_to_string(&mut text)
        .map_err(|error| {
            state_error(
                "state.local.read",
                format!("failed to read state file {}: {error}", path.display()),
            )
        })?;
    serde_json::from_str(&text).map_err(|error| {
        state_error(
            "state.corrupt",
            format!("failed to decode state file {}: {error}", path.display()),
        )
    })
}

fn encode_key(value: &str) -> String {
    let mut output = String::with_capacity(value.len() * 2);
    for byte in value.as_bytes() {
        let _ = write!(output, "{byte:02x}");
    }
    output
}

fn state_error(code: impl Into<String>, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::new(WorkflowOsErrorKind::InvalidState, code, message)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use std::cell::RefCell;
    use std::collections::{BTreeMap, BTreeSet};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;
    use crate::{
        CorrelationId, EventSequenceNumber, RunRehydration, SpecContentHash, WorkflowId,
        WorkflowRunEventKind, WorkflowRunStatus, WorkflowVersion,
    };

    static NEXT_TEST_BACKEND: AtomicU64 = AtomicU64::new(1);

    #[derive(Default)]
    struct InMemoryStateBackend {
        events: RefCell<BTreeMap<WorkflowRunId, Vec<WorkflowRunEvent>>>,
        event_ids: RefCell<BTreeSet<EventId>>,
        snapshots: RefCell<BTreeMap<WorkflowRunId, WorkflowRunSnapshot>>,
        idempotency: RefCell<BTreeMap<IdempotencyKey, IdempotencyResult>>,
        locks: RefCell<BTreeMap<String, ActorId>>,
        approvals: RefCell<BTreeMap<String, ApprovalRequest>>,
        projects: RefCell<BTreeMap<ProjectId, ProjectStateRecord>>,
    }

    impl EventLogStore for InMemoryStateBackend {
        fn append_event(&self, event: &WorkflowRunEvent) -> Result<(), WorkflowOsError> {
            if self.event_ids.borrow().contains(&event.event_id) {
                return Err(state_error(
                    "state.event.duplicate_id",
                    format!("duplicate event id {}", event.event_id),
                ));
            }
            let mut events = self.events.borrow_mut();
            let run_events = events.entry(event.run_id.clone()).or_default();
            if run_events
                .iter()
                .any(|stored| stored.sequence_number == event.sequence_number)
            {
                return Err(state_error(
                    "state.event.duplicate_sequence",
                    format!(
                        "duplicate sequence {} for run {}",
                        event.sequence_number, event.run_id
                    ),
                ));
            }
            run_events.push(event.clone());
            run_events.sort_by_key(|event| event.sequence_number);
            self.event_ids.borrow_mut().insert(event.event_id.clone());
            Ok(())
        }

        fn read_events(
            &self,
            run_id: &WorkflowRunId,
        ) -> Result<Vec<WorkflowRunEvent>, WorkflowOsError> {
            Ok(self
                .events
                .borrow()
                .get(run_id)
                .cloned()
                .unwrap_or_default())
        }
    }

    impl RunSnapshotStore for InMemoryStateBackend {
        fn save_snapshot(&self, snapshot: &WorkflowRunSnapshot) -> Result<(), WorkflowOsError> {
            self.snapshots
                .borrow_mut()
                .insert(snapshot.identity.run_id.clone(), snapshot.clone());
            Ok(())
        }

        fn load_snapshot(
            &self,
            run_id: &WorkflowRunId,
        ) -> Result<Option<WorkflowRunSnapshot>, WorkflowOsError> {
            Ok(self.snapshots.borrow().get(run_id).cloned())
        }
    }

    impl IdempotencyStore for InMemoryStateBackend {
        fn record_idempotency_result(
            &self,
            key: &IdempotencyKey,
            result: IdempotencyResult,
        ) -> Result<IdempotencyWrite, WorkflowOsError> {
            let mut idempotency = self.idempotency.borrow_mut();
            if let Some(existing) = idempotency.get(key) {
                return Ok(IdempotencyWrite::Duplicate(existing.clone()));
            }
            idempotency.insert(key.clone(), result.clone());
            Ok(IdempotencyWrite::FirstWrite(result))
        }
    }

    impl LockStore for InMemoryStateBackend {
        fn acquire_lock(&self, key: &str, owner: &ActorId) -> Result<LockLease, WorkflowOsError> {
            let mut locks = self.locks.borrow_mut();
            if locks.contains_key(key) {
                return Err(state_error(
                    "state.lock_contended",
                    format!("lock {key} is already held"),
                ));
            }
            locks.insert(key.to_owned(), owner.clone());
            Ok(LockLease {
                key: key.to_owned(),
                owner: owner.clone(),
            })
        }

        fn release_lock(&self, lease: &LockLease) -> Result<(), WorkflowOsError> {
            self.locks.borrow_mut().remove(&lease.key);
            Ok(())
        }
    }

    impl ApprovalStore for InMemoryStateBackend {
        fn save_approval_request(&self, request: &ApprovalRequest) -> Result<(), WorkflowOsError> {
            self.approvals
                .borrow_mut()
                .insert(request.approval_id.clone(), request.clone());
            Ok(())
        }

        fn load_approval_request(
            &self,
            approval_id: &str,
        ) -> Result<Option<ApprovalRequest>, WorkflowOsError> {
            Ok(self.approvals.borrow().get(approval_id).cloned())
        }
    }

    impl ProjectStateStore for InMemoryStateBackend {
        fn save_project_state(&self, state: &ProjectStateRecord) -> Result<(), WorkflowOsError> {
            self.projects
                .borrow_mut()
                .insert(state.project_id.clone(), state.clone());
            Ok(())
        }

        fn load_project_state(
            &self,
            project_id: &ProjectId,
        ) -> Result<Option<ProjectStateRecord>, WorkflowOsError> {
            Ok(self.projects.borrow().get(project_id).cloned())
        }
    }

    impl StateBackend for InMemoryStateBackend {
        fn health_check(&self) -> Result<BackendHealthCheck, WorkflowOsError> {
            Ok(BackendHealthCheck {
                healthy: true,
                backend: "in_memory_test".to_owned(),
                message: "in-memory test backend is available".to_owned(),
            })
        }
    }

    #[derive(Clone)]
    struct Fixture {
        id: u64,
        run_id: WorkflowRunId,
        workflow_id: WorkflowId,
        workflow_version: WorkflowVersion,
        spec_hash: SpecContentHash,
    }

    impl Fixture {
        fn new() -> Self {
            let id = NEXT_TEST_BACKEND.fetch_add(1, Ordering::Relaxed);
            Self {
                id,
                run_id: WorkflowRunId::new(format!("run-state-{id}")).expect("run id"),
                workflow_id: WorkflowId::new(format!("workflow/state-{id}")).expect("workflow id"),
                workflow_version: WorkflowVersion::new("v0").expect("workflow version"),
                spec_hash: SpecContentHash::from_text(&format!("state backend spec {id}")),
            }
        }

        fn event(&self, sequence: u64, kind: WorkflowRunEventKind) -> WorkflowRunEvent {
            WorkflowRunEvent {
                sequence_number: EventSequenceNumber::new(sequence).expect("sequence"),
                event_id: EventId::new(format!("event-{}-{sequence}", self.id)).expect("event id"),
                timestamp: crate::Timestamp::parse_rfc3339("2026-01-01T00:00:00Z")
                    .expect("timestamp"),
                run_id: self.run_id.clone(),
                workflow_id: self.workflow_id.clone(),
                workflow_version: self.workflow_version.clone(),
                spec_content_hash: self.spec_hash.clone(),
                correlation_id: Some(CorrelationId::new("correlation-state").expect("correlation")),
                actor: Some(ActorId::new("system").expect("actor")),
                idempotency_key: None,
                kind,
            }
        }

        fn created(&self) -> WorkflowRunEvent {
            self.event(1, WorkflowRunEventKind::RunCreated { summary: None })
        }

        fn validated(&self) -> WorkflowRunEvent {
            self.event(2, WorkflowRunEventKind::RunValidated)
        }
    }

    fn contract_append_and_read_events(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        backend
            .append_event(&fixture.created())
            .expect("append created");
        backend
            .append_event(&fixture.validated())
            .expect("append validated");

        let events = backend.read_events(&fixture.run_id).expect("read events");

        assert_eq!(events.len(), 2);
        assert_eq!(events[0].sequence_number.get(), 1);
        assert_eq!(events[1].sequence_number.get(), 2);
    }

    fn contract_reject_duplicate_event_id(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        let first = fixture.created();
        let mut duplicate = fixture.validated();
        duplicate.event_id = first.event_id.clone();

        backend.append_event(&first).expect("append first");
        let error = backend
            .append_event(&duplicate)
            .expect_err("duplicate event id is rejected");

        assert_eq!(error.code(), "state.event.duplicate_id");
    }

    fn contract_reject_duplicate_sequence(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        let first = fixture.created();
        let mut duplicate_sequence = fixture.event(1, WorkflowRunEventKind::RunValidated);
        duplicate_sequence.event_id =
            EventId::new(format!("event-{}-duplicate", fixture.id)).expect("event id");

        backend.append_event(&first).expect("append first");
        let error = backend
            .append_event(&duplicate_sequence)
            .expect_err("duplicate sequence is rejected");

        assert_eq!(error.code(), "state.event.duplicate_sequence");
    }

    fn contract_rehydrate_from_backend(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        backend
            .append_event(&fixture.created())
            .expect("append created");
        backend
            .append_event(&fixture.validated())
            .expect("append validated");

        let run = backend
            .rehydrate_run(&fixture.run_id)
            .expect("rehydrates from backend");

        assert_eq!(run.snapshot.status, WorkflowRunStatus::Validated);
    }

    fn contract_idempotency_first_write_wins(backend: &impl StateBackend) {
        let key = IdempotencyKey::new("idem/state").expect("idempotency key");
        let first = backend
            .record_idempotency_result(
                &key,
                IdempotencyResult {
                    result_ref: "first".to_owned(),
                },
            )
            .expect("first idempotency write");
        let second = backend
            .record_idempotency_result(
                &key,
                IdempotencyResult {
                    result_ref: "second".to_owned(),
                },
            )
            .expect("duplicate idempotency write");

        assert_eq!(
            first,
            IdempotencyWrite::FirstWrite(IdempotencyResult {
                result_ref: "first".to_owned()
            })
        );
        assert_eq!(
            second,
            IdempotencyWrite::Duplicate(IdempotencyResult {
                result_ref: "first".to_owned()
            })
        );
    }

    fn contract_lock_acquire_release_and_contention(backend: &impl StateBackend) {
        let owner = ActorId::new("worker").expect("actor");
        let lease = backend
            .acquire_lock("run/run-state", &owner)
            .expect("lock acquired");
        let contention = backend
            .acquire_lock("run/run-state", &owner)
            .expect_err("contention rejected");
        assert_eq!(contention.code(), "state.lock_contended");

        backend.release_lock(&lease).expect("lock released");
        let reacquired = backend
            .acquire_lock("run/run-state", &owner)
            .expect("lock reacquired");
        backend.release_lock(&reacquired).expect("lock released");
    }

    fn contract_health_check(backend: &impl StateBackend) {
        let health = backend.health_check().expect("health check");

        assert!(health.healthy);
    }

    fn contract_snapshot_projection(backend: &impl StateBackend) {
        let fixture = Fixture::new();
        let snapshot = RunRehydration::rehydrate(&[fixture.created()]).expect("snapshot");
        backend.save_snapshot(&snapshot).expect("save snapshot");

        let loaded = backend
            .load_snapshot(&fixture.run_id)
            .expect("load snapshot")
            .expect("snapshot exists");

        assert_eq!(loaded.identity.run_id, fixture.run_id);
    }

    fn run_backend_contract(backend: &impl StateBackend) {
        contract_append_and_read_events(backend);
        contract_reject_duplicate_event_id(backend);
        contract_reject_duplicate_sequence(backend);
        contract_rehydrate_from_backend(backend);
        contract_idempotency_first_write_wins(backend);
        contract_lock_acquire_release_and_contention(backend);
        contract_health_check(backend);
        contract_snapshot_projection(backend);
    }

    fn local_backend() -> LocalStateBackend {
        let id = NEXT_TEST_BACKEND.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "workflow-os-state-backend-{}-{id}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("stale backend cleanup");
        }
        LocalStateBackend::new(root).expect("local backend")
    }

    #[test]
    fn backend_contract_passes_for_in_memory_backend() {
        run_backend_contract(&InMemoryStateBackend::default());
    }

    #[test]
    fn backend_contract_passes_for_local_backend() {
        let backend = local_backend();
        let root = backend.root().to_path_buf();
        run_backend_contract(&backend);
        fs::remove_dir_all(root).expect("cleanup local backend");
    }

    #[test]
    fn corrupted_local_state_returns_clear_error() {
        let backend = local_backend();
        let fixture = Fixture::new();
        let run_dir = backend.run_events_dir(&fixture.run_id);
        fs::create_dir_all(&run_dir).expect("run dir");
        fs::write(run_dir.join("00000000000000000001.json"), "{not json")
            .expect("corrupt event file");

        let error = backend
            .read_events(&fixture.run_id)
            .expect_err("corruption is reported");

        assert_eq!(error.code(), "state.corrupt");
        fs::remove_dir_all(backend.root()).expect("cleanup local backend");
    }
}
