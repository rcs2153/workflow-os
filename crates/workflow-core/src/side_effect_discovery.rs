use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::{
    SchemaVersion, SideEffectId, SideEffectRecord, SideEffectRecordStore, SpecContentHash,
    WorkflowId, WorkflowOsError, WorkflowOsErrorKind, WorkflowRunEvent, WorkflowRunEventKind,
    WorkflowRunId, WorkflowVersion,
};

/// Explicit in-memory input for bounded `SideEffect` reference discovery.
#[derive(Clone, Eq, PartialEq)]
pub struct SideEffectDiscoveryInput {
    /// Workflow ID for the immutable run identity.
    pub workflow_id: WorkflowId,
    /// Workflow version for the immutable run identity.
    pub workflow_version: WorkflowVersion,
    /// Schema version for the immutable run identity.
    pub schema_version: SchemaVersion,
    /// Workflow spec content hash for the immutable run identity.
    pub spec_hash: SpecContentHash,
    /// Workflow run ID for discovery.
    pub run_id: WorkflowRunId,
    /// Explicit stable `SideEffect` IDs supplied by the caller.
    pub explicit_side_effect_ids: Vec<SideEffectId>,
    /// Already-loaded workflow events supplied by the caller.
    pub workflow_events: Vec<WorkflowRunEvent>,
    /// Already-loaded `SideEffect` records supplied by the caller.
    pub side_effect_records: Vec<SideEffectRecord>,
    /// Whether every discovered `SideEffect` ID must have a matching supplied record.
    pub require_records: bool,
}

impl fmt::Debug for SideEffectDiscoveryInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SideEffectDiscoveryInput")
            .field("workflow_id", &"[REDACTED]")
            .field("workflow_version", &"[REDACTED]")
            .field("schema_version", &self.schema_version)
            .field("spec_hash", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field(
                "explicit_side_effect_id_count",
                &self.explicit_side_effect_ids.len(),
            )
            .field("workflow_event_count", &self.workflow_events.len())
            .field("side_effect_record_count", &self.side_effect_records.len())
            .field("require_records", &self.require_records)
            .finish()
    }
}

/// Explicit input for store-backed `SideEffect` reference discovery.
#[derive(Clone, Eq, PartialEq)]
pub struct SideEffectStoreBackedDiscoveryInput {
    /// Workflow ID for the immutable run identity.
    pub workflow_id: WorkflowId,
    /// Workflow version for the immutable run identity.
    pub workflow_version: WorkflowVersion,
    /// Schema version for the immutable run identity.
    pub schema_version: SchemaVersion,
    /// Workflow spec content hash for the immutable run identity.
    pub spec_hash: SpecContentHash,
    /// Workflow run ID for discovery.
    pub run_id: WorkflowRunId,
    /// Explicit stable `SideEffect` IDs supplied by the caller.
    pub explicit_side_effect_ids: Vec<SideEffectId>,
    /// Already-loaded workflow events supplied by the caller.
    pub workflow_events: Vec<WorkflowRunEvent>,
    /// Whether every discovered `SideEffect` ID must have a matching store record.
    pub require_records: bool,
}

impl fmt::Debug for SideEffectStoreBackedDiscoveryInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SideEffectStoreBackedDiscoveryInput")
            .field("workflow_id", &"[REDACTED]")
            .field("workflow_version", &"[REDACTED]")
            .field("schema_version", &self.schema_version)
            .field("spec_hash", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field(
                "explicit_side_effect_id_count",
                &self.explicit_side_effect_ids.len(),
            )
            .field("workflow_event_count", &self.workflow_events.len())
            .field("require_records", &self.require_records)
            .finish()
    }
}

/// Source that produced a discovered `SideEffect` reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SideEffectDiscoverySource {
    /// Caller supplied the `SideEffect` ID explicitly.
    ExplicitInput,
    /// `SideEffect` ID came from accepted workflow event history.
    WorkflowEvent,
    /// `SideEffect` ID came from a supplied validated `SideEffect` record.
    SideEffectRecord,
}

/// One discovered `SideEffect` reference.
#[derive(Clone, Eq, PartialEq)]
pub struct SideEffectDiscoveryReference {
    side_effect_id: SideEffectId,
    source: SideEffectDiscoverySource,
}

impl SideEffectDiscoveryReference {
    /// Returns the discovered `SideEffect` ID.
    #[must_use]
    pub const fn side_effect_id(&self) -> &SideEffectId {
        &self.side_effect_id
    }

    /// Returns the source that produced this reference.
    #[must_use]
    pub const fn source(&self) -> SideEffectDiscoverySource {
        self.source
    }
}

impl fmt::Debug for SideEffectDiscoveryReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SideEffectDiscoveryReference")
            .field("side_effect_id", &"[REDACTED]")
            .field("source", &self.source)
            .finish()
    }
}

/// Result of bounded in-memory `SideEffect` discovery.
#[derive(Clone, Eq, PartialEq)]
pub struct SideEffectDiscoveryResult {
    references: Vec<SideEffectDiscoveryReference>,
    missing_record_count: usize,
    unsupported_event_count: usize,
    records_required: bool,
}

impl SideEffectDiscoveryResult {
    /// Returns discovered references in deterministic `SideEffect` ID order.
    #[must_use]
    pub fn references(&self) -> &[SideEffectDiscoveryReference] {
        &self.references
    }

    /// Returns the number of discovered IDs without a matching supplied record.
    #[must_use]
    pub const fn missing_record_count(&self) -> usize {
        self.missing_record_count
    }

    /// Returns the count of ignored attempted/completed/failed `SideEffect` events.
    #[must_use]
    pub const fn unsupported_event_count(&self) -> usize {
        self.unsupported_event_count
    }

    /// Returns whether matching records were required.
    #[must_use]
    pub const fn records_required(&self) -> bool {
        self.records_required
    }

    /// Returns true when references were discovered.
    #[must_use]
    pub fn has_references(&self) -> bool {
        !self.references.is_empty()
    }
}

impl fmt::Debug for SideEffectDiscoveryResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SideEffectDiscoveryResult")
            .field("reference_count", &self.references.len())
            .field("missing_record_count", &self.missing_record_count)
            .field("unsupported_event_count", &self.unsupported_event_count)
            .field("records_required", &self.records_required)
            .finish()
    }
}

/// Discovers stable `SideEffect` IDs from explicit in-memory inputs.
///
/// This helper does not read state backends, create records, append events,
/// call providers, execute `SideEffect`s, persist reports, or build `WorkReport`
/// citations. It only returns stable `SideEffect` IDs and bounded status.
///
/// # Errors
///
/// Returns a stable non-leaking error when supplied events or records do not
/// match the immutable run identity, when supplied records fail validation, or
/// when records are required but missing for discovered IDs.
pub fn discover_side_effect_references(
    input: &SideEffectDiscoveryInput,
) -> Result<SideEffectDiscoveryResult, WorkflowOsError> {
    let mut discovered = BTreeMap::<SideEffectId, SideEffectDiscoverySource>::new();
    let mut supplied_record_ids = BTreeSet::<SideEffectId>::new();
    let mut unsupported_event_count = 0usize;

    for side_effect_id in &input.explicit_side_effect_ids {
        discovered
            .entry(side_effect_id.clone())
            .or_insert(SideEffectDiscoverySource::ExplicitInput);
    }

    for event in &input.workflow_events {
        validate_event_identity(input, event)?;
        match side_effect_id_from_supported_event(event) {
            SupportedSideEffectEvent::Supported(side_effect_id) => {
                discovered
                    .entry(side_effect_id.clone())
                    .or_insert(SideEffectDiscoverySource::WorkflowEvent);
            }
            SupportedSideEffectEvent::Unsupported => {
                unsupported_event_count += 1;
            }
            SupportedSideEffectEvent::NotSideEffect => {}
        }
    }

    for record in &input.side_effect_records {
        record
            .validate()
            .map_err(|_| discovery_error("side_effect_discovery.record_corrupt"))?;
        validate_record_identity(input, record)?;
        supplied_record_ids.insert(record.side_effect_id().clone());
        discovered
            .entry(record.side_effect_id().clone())
            .or_insert(SideEffectDiscoverySource::SideEffectRecord);
    }

    let missing_record_count = discovered
        .keys()
        .filter(|side_effect_id| !supplied_record_ids.contains(*side_effect_id))
        .count();

    if input.require_records && missing_record_count > 0 {
        return Err(discovery_error("side_effect_discovery.record_missing"));
    }

    let references = discovered
        .into_iter()
        .map(|(side_effect_id, source)| SideEffectDiscoveryReference {
            side_effect_id,
            source,
        })
        .collect();

    Ok(SideEffectDiscoveryResult {
        references,
        missing_record_count,
        unsupported_event_count,
        records_required: input.require_records,
    })
}

/// Discovers stable `SideEffect` IDs using an explicit `SideEffectRecordStore`.
///
/// This wrapper reads already-persisted records through the supplied store and
/// then delegates validation and deterministic de-duplication to
/// [`discover_side_effect_references`]. It does not create records, append
/// events, execute side effects, create report citations, write artifacts, call
/// providers, or expose CLI output.
///
/// # Errors
///
/// Returns a stable non-leaking error when store reads fail, supplied events or
/// records do not match the immutable run identity, supplied records fail
/// validation, or records are required but missing for discovered IDs.
pub fn discover_side_effect_references_from_store(
    store: &impl SideEffectRecordStore,
    input: &SideEffectStoreBackedDiscoveryInput,
) -> Result<SideEffectDiscoveryResult, WorkflowOsError> {
    let side_effect_records = store
        .list_side_effect_records_for_workflow_run(&input.workflow_id, &input.run_id)
        .map_err(|error| map_store_discovery_error(&error))?;
    let helper_input = SideEffectDiscoveryInput {
        workflow_id: input.workflow_id.clone(),
        workflow_version: input.workflow_version.clone(),
        schema_version: input.schema_version.clone(),
        spec_hash: input.spec_hash.clone(),
        run_id: input.run_id.clone(),
        explicit_side_effect_ids: input.explicit_side_effect_ids.clone(),
        workflow_events: input.workflow_events.clone(),
        side_effect_records,
        require_records: input.require_records,
    };
    discover_side_effect_references(&helper_input)
}

enum SupportedSideEffectEvent<'a> {
    Supported(&'a SideEffectId),
    Unsupported,
    NotSideEffect,
}

fn side_effect_id_from_supported_event(event: &WorkflowRunEvent) -> SupportedSideEffectEvent<'_> {
    match &event.kind {
        WorkflowRunEventKind::SideEffectProposed(payload)
        | WorkflowRunEventKind::SideEffectDenied(payload)
        | WorkflowRunEventKind::SideEffectSkipped(payload) => {
            SupportedSideEffectEvent::Supported(payload.side_effect_id())
        }
        WorkflowRunEventKind::SideEffectAttempted(_)
        | WorkflowRunEventKind::SideEffectCompleted(_)
        | WorkflowRunEventKind::SideEffectFailed(_) => SupportedSideEffectEvent::Unsupported,
        _ => SupportedSideEffectEvent::NotSideEffect,
    }
}

fn validate_event_identity(
    input: &SideEffectDiscoveryInput,
    event: &WorkflowRunEvent,
) -> Result<(), WorkflowOsError> {
    if event.workflow_id != input.workflow_id
        || event.workflow_version != input.workflow_version
        || event.schema_version != input.schema_version
        || event.spec_content_hash != input.spec_hash
        || event.run_id != input.run_id
    {
        return Err(discovery_error("side_effect_discovery.identity_mismatch"));
    }
    Ok(())
}

fn validate_record_identity(
    input: &SideEffectDiscoveryInput,
    record: &SideEffectRecord,
) -> Result<(), WorkflowOsError> {
    if record.workflow_id() != &input.workflow_id
        || record.workflow_version() != &input.workflow_version
        || record.schema_version() != &input.schema_version
        || record.spec_hash() != &input.spec_hash
        || record.run_id() != &input.run_id
    {
        return Err(discovery_error("side_effect_discovery.identity_mismatch"));
    }
    Ok(())
}

fn map_store_discovery_error(error: &WorkflowOsError) -> WorkflowOsError {
    match error.code() {
        "side_effect_record.read.identity_mismatch" => {
            discovery_error("side_effect_discovery.identity_mismatch")
        }
        "side_effect_record.read.corrupt" => {
            discovery_error("side_effect_discovery.record_corrupt")
        }
        _ => discovery_error("side_effect_discovery.store_read_failed"),
    }
}

fn discovery_error(code: &'static str) -> WorkflowOsError {
    let message = match code {
        "side_effect_discovery.identity_mismatch" => {
            "side-effect discovery source does not match requested immutable run identity"
        }
        "side_effect_discovery.record_missing" => {
            "required side-effect record was not supplied for a discovered reference"
        }
        "side_effect_discovery.record_corrupt" => {
            "side-effect discovery record could not be validated"
        }
        "side_effect_discovery.reference_invalid" => {
            "side-effect discovery reference could not be validated"
        }
        "side_effect_discovery.source_unsupported" => "side-effect discovery source is unsupported",
        "side_effect_discovery.store_read_failed" => "side-effect discovery store read failed",
        _ => "side-effect discovery failed",
    };
    WorkflowOsError::new(WorkflowOsErrorKind::InvalidState, code, message)
}
