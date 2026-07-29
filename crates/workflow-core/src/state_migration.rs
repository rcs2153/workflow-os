use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{DurableStateBackendKind, WorkflowOsError, WorkflowOsErrorKind};

const MIGRATION_IDENTIFIER_MAX_BYTES: usize = 128;

macro_rules! migration_identifier {
    ($name:ident, $label:literal, $error_suffix:literal) => {
        #[doc = concat!("Validated ", $label, ".")]
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Creates a validated ", $label, ".")]
            ///
            /// # Errors
            ///
            /// Returns a stable non-leaking error when the value is empty,
            /// unbounded, malformed, or secret-like.
            pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
                let value = value.into();
                validate_migration_identifier($label, $error_suffix, &value)?;
                Ok(Self(value))
            }

            #[doc = concat!("Returns the ", $label, ".")]
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&"<redacted>")
                    .finish()
            }
        }

        impl TryFrom<String> for $name {
            type Error = WorkflowOsError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

migration_identifier!(
    StateMigrationId,
    "state migration identifier",
    "plan.id.invalid"
);
migration_identifier!(
    StateMigrationDestinationId,
    "state migration destination identifier",
    "destination.id.invalid"
);

/// Version of the immutable migration-plan contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateMigrationPlanVersion {
    /// Initial filesystem-to-SQLite staging plan.
    V1,
}

/// Version of the read-only state-migration inventory contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateMigrationInventoryVersion {
    /// Initial filesystem inventory and compatibility contract.
    V1,
}

/// One filesystem state family considered by migration inventory.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateMigrationRecordFamily {
    /// Authoritative ordered workflow events.
    WorkflowEvents,
    /// Event-ID lookup projection.
    EventIdIndexes,
    /// Rebuildable workflow-run snapshots.
    RunSnapshots,
    /// Authoritative idempotency outcomes.
    IdempotencyResults,
    /// Process-local ephemeral locks.
    LocalLocks,
    /// Rebuildable pending-approval projections.
    PendingApprovalProjections,
    /// Authoritative approval-presentation proof records.
    ApprovalPresentationRecords,
    /// Approval-presentation ID lookup projection.
    ApprovalPresentationIdIndexes,
    /// Local project metadata.
    ProjectStateRecords,
    /// Authoritative policy audit records.
    PolicyAuditRecords,
    /// Authoritative adapter audit records.
    AdapterAuditRecords,
    /// Authoritative adapter observability records.
    AdapterObservabilityRecords,
    /// Authoritative `WorkReport` artifact records.
    WorkReportArtifacts,
    /// Authoritative `SideEffect` records.
    SideEffectRecords,
    /// `SideEffect` ID lookup projection.
    SideEffectIdIndexes,
    /// Companion immutable run-bundle files retained outside `SQLite` schema v1.
    ImmutableRunBundles,
}

impl StateMigrationRecordFamily {
    const ALL: [Self; 16] = [
        Self::WorkflowEvents,
        Self::EventIdIndexes,
        Self::RunSnapshots,
        Self::IdempotencyResults,
        Self::LocalLocks,
        Self::PendingApprovalProjections,
        Self::ApprovalPresentationRecords,
        Self::ApprovalPresentationIdIndexes,
        Self::ProjectStateRecords,
        Self::PolicyAuditRecords,
        Self::AdapterAuditRecords,
        Self::AdapterObservabilityRecords,
        Self::WorkReportArtifacts,
        Self::SideEffectRecords,
        Self::SideEffectIdIndexes,
        Self::ImmutableRunBundles,
    ];

    /// Returns every inventory family in stable order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &Self::ALL
    }

    /// Returns the required migration disposition for this family.
    #[must_use]
    pub const fn disposition(self) -> StateMigrationDisposition {
        match self {
            Self::WorkflowEvents
            | Self::IdempotencyResults
            | Self::ApprovalPresentationRecords
            | Self::ProjectStateRecords
            | Self::PolicyAuditRecords
            | Self::AdapterAuditRecords
            | Self::AdapterObservabilityRecords
            | Self::WorkReportArtifacts
            | Self::SideEffectRecords => StateMigrationDisposition::CanonicalImport,
            Self::EventIdIndexes
            | Self::RunSnapshots
            | Self::PendingApprovalProjections
            | Self::ApprovalPresentationIdIndexes
            | Self::SideEffectIdIndexes => StateMigrationDisposition::ProjectionRebuild,
            Self::LocalLocks => StateMigrationDisposition::EphemeralExclude,
            Self::ImmutableRunBundles => StateMigrationDisposition::CompanionPreserve,
        }
    }

    pub(crate) const fn stable_label(self) -> &'static str {
        match self {
            Self::WorkflowEvents => "workflow_events",
            Self::EventIdIndexes => "event_id_indexes",
            Self::RunSnapshots => "run_snapshots",
            Self::IdempotencyResults => "idempotency_results",
            Self::LocalLocks => "local_locks",
            Self::PendingApprovalProjections => "pending_approval_projections",
            Self::ApprovalPresentationRecords => "approval_presentation_records",
            Self::ApprovalPresentationIdIndexes => "approval_presentation_id_indexes",
            Self::ProjectStateRecords => "project_state_records",
            Self::PolicyAuditRecords => "policy_audit_records",
            Self::AdapterAuditRecords => "adapter_audit_records",
            Self::AdapterObservabilityRecords => "adapter_observability_records",
            Self::WorkReportArtifacts => "work_report_artifacts",
            Self::SideEffectRecords => "side_effect_records",
            Self::SideEffectIdIndexes => "side_effect_id_indexes",
            Self::ImmutableRunBundles => "immutable_run_bundles",
        }
    }
}

/// Planned treatment of one state family during migration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateMigrationDisposition {
    /// Import validated canonical records through destination model boundaries.
    CanonicalImport,
    /// Validate but rebuild this projection from authoritative records.
    ProjectionRebuild,
    /// Exclude ephemeral state from migration.
    EphemeralExclude,
    /// Preserve the recognized companion store outside the destination.
    CompanionPreserve,
}

/// Bounded SHA-256 digest used by migration inventory.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub struct StateMigrationDigest(String);

impl StateMigrationDigest {
    /// Creates a validated lowercase SHA-256 digest.
    ///
    /// # Errors
    ///
    /// Returns a stable error when the digest is not exactly 64 lowercase
    /// hexadecimal characters.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        if value.len() != 64
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(migration_error(
                "digest.invalid",
                "state migration digest is invalid",
            ));
        }
        Ok(Self(value))
    }

    /// Returns the lowercase digest.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn from_hasher(hasher: Sha256) -> Self {
        Self(format!("{:x}", hasher.finalize()))
    }
}

impl<'de> Deserialize<'de> for StateMigrationDigest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(|_| serde::de::Error::custom("state migration digest is invalid"))
    }
}

/// Count and digest posture for one state family.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StateMigrationRecordCount {
    family: StateMigrationRecordFamily,
    disposition: StateMigrationDisposition,
    count: u64,
    digest: Option<StateMigrationDigest>,
}

impl StateMigrationRecordCount {
    /// Creates one family inventory entry.
    ///
    /// # Errors
    ///
    /// Returns a stable error when the disposition does not match the family.
    pub fn new(
        family: StateMigrationRecordFamily,
        disposition: StateMigrationDisposition,
        count: u64,
        digest: Option<StateMigrationDigest>,
    ) -> Result<Self, WorkflowOsError> {
        if disposition != family.disposition() {
            return Err(migration_error(
                "record_count.disposition_mismatch",
                "state migration record disposition does not match its family",
            ));
        }
        Ok(Self {
            family,
            disposition,
            count,
            digest,
        })
    }

    /// Returns the record family.
    #[must_use]
    pub const fn family(&self) -> StateMigrationRecordFamily {
        self.family
    }

    /// Returns the planned disposition.
    #[must_use]
    pub const fn disposition(&self) -> StateMigrationDisposition {
        self.disposition
    }

    /// Returns the number of discovered candidate records.
    #[must_use]
    pub const fn count(&self) -> u64 {
        self.count
    }

    /// Returns the deterministic family digest when every candidate validated.
    #[must_use]
    pub const fn digest(&self) -> Option<&StateMigrationDigest> {
        self.digest.as_ref()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StateMigrationRecordCountWire {
    family: StateMigrationRecordFamily,
    disposition: StateMigrationDisposition,
    count: u64,
    digest: Option<StateMigrationDigest>,
}

impl<'de> Deserialize<'de> for StateMigrationRecordCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = StateMigrationRecordCountWire::deserialize(deserializer)?;
        Self::new(wire.family, wire.disposition, wire.count, wire.digest)
            .map_err(|_| serde::de::Error::custom("state migration record count is invalid"))
    }
}

/// Severity of one bounded migration compatibility finding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateMigrationFindingSeverity {
    /// Reviewable finding that does not make the known state incompatible.
    Warning,
    /// Finding that blocks migration compatibility.
    Blocker,
}

/// Stable migration inventory finding code.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateMigrationFindingCode {
    /// Existing local state inspection reported an error.
    SourceUnhealthy,
    /// Existing local state inspection reported a warning.
    SourceWarning,
    /// A state directory could not be enumerated.
    SourceUnreadable,
    /// A symbolic link was found inside the state boundary.
    SymlinkRejected,
    /// A known location contained an unexpected file type.
    UnexpectedFileType,
    /// A storage address or directory shape was malformed.
    MalformedStorageAddress,
    /// A candidate record could not be validated.
    RecordInvalid,
    /// A projection index did not match canonical state.
    IndexInconsistent,
    /// An empty unknown directory was found and ignored.
    UnknownEmptyDirectory,
    /// Unknown non-empty state was found.
    UnknownRecordFamily,
    /// One record identity appeared more than once in a family.
    DuplicateIdentity,
    /// One or more process-local lock records exist.
    LockPresent,
}

impl StateMigrationFindingCode {
    const fn prevents_fingerprint(self) -> bool {
        matches!(
            self,
            Self::SourceUnhealthy
                | Self::SourceUnreadable
                | Self::SymlinkRejected
                | Self::UnexpectedFileType
                | Self::MalformedStorageAddress
                | Self::RecordInvalid
                | Self::IndexInconsistent
                | Self::UnknownRecordFamily
                | Self::DuplicateIdentity
        )
    }
}

/// One payload-free compatibility finding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct StateMigrationCompatibilityFinding {
    severity: StateMigrationFindingSeverity,
    code: StateMigrationFindingCode,
    family: Option<StateMigrationRecordFamily>,
}

impl StateMigrationCompatibilityFinding {
    /// Creates one typed finding.
    #[must_use]
    pub const fn new(
        severity: StateMigrationFindingSeverity,
        code: StateMigrationFindingCode,
        family: Option<StateMigrationRecordFamily>,
    ) -> Self {
        Self {
            severity,
            code,
            family,
        }
    }

    /// Returns the finding severity.
    #[must_use]
    pub const fn severity(self) -> StateMigrationFindingSeverity {
        self.severity
    }

    /// Returns the stable finding code.
    #[must_use]
    pub const fn code(self) -> StateMigrationFindingCode {
        self.code
    }

    /// Returns the related known family, when applicable.
    #[must_use]
    pub const fn family(self) -> Option<StateMigrationRecordFamily> {
        self.family
    }
}

/// Read-only, payload-free inventory of one local filesystem state root.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct StateMigrationInventory {
    version: StateMigrationInventoryVersion,
    source_backend: DurableStateBackendKind,
    record_counts: Vec<StateMigrationRecordCount>,
    findings: Vec<StateMigrationCompatibilityFinding>,
    empty: bool,
    healthy: bool,
    migration_compatible: bool,
    quiescence_required: bool,
    source_fingerprint: Option<StateMigrationDigest>,
}

impl StateMigrationInventory {
    /// Creates and validates a complete inventory.
    ///
    /// # Errors
    ///
    /// Returns a stable error for missing or duplicate families.
    pub fn new(
        record_counts: Vec<StateMigrationRecordCount>,
        findings: Vec<StateMigrationCompatibilityFinding>,
        quiescence_required: bool,
    ) -> Result<Self, WorkflowOsError> {
        let families = record_counts
            .iter()
            .map(StateMigrationRecordCount::family)
            .collect::<BTreeSet<_>>();
        let expected = StateMigrationRecordFamily::all()
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        if record_counts.len() != expected.len() || families != expected {
            return Err(migration_error(
                "inventory.incomplete",
                "state migration inventory family coverage is incomplete",
            ));
        }

        let mut record_counts = record_counts;
        record_counts.sort_by_key(StateMigrationRecordCount::family);
        let mut findings = findings;
        findings.sort_unstable();
        findings.dedup();

        let empty = record_counts.iter().all(|record| record.count() == 0);
        let healthy = findings
            .iter()
            .all(|finding| finding.severity() != StateMigrationFindingSeverity::Blocker);
        let source_fingerprint =
            derive_source_fingerprint(&record_counts, &findings, quiescence_required);
        let migration_compatible = healthy && source_fingerprint.is_some();

        Ok(Self {
            version: StateMigrationInventoryVersion::V1,
            source_backend: DurableStateBackendKind::LocalFilesystemPreview,
            record_counts,
            findings,
            empty,
            healthy,
            migration_compatible,
            quiescence_required,
            source_fingerprint,
        })
    }

    /// Returns the inventory contract version.
    #[must_use]
    pub const fn version(&self) -> StateMigrationInventoryVersion {
        self.version
    }

    /// Returns the source backend kind.
    #[must_use]
    pub const fn source_backend(&self) -> DurableStateBackendKind {
        self.source_backend
    }

    /// Returns family entries in stable order.
    #[must_use]
    pub fn record_counts(&self) -> &[StateMigrationRecordCount] {
        &self.record_counts
    }

    /// Returns bounded findings in stable order.
    #[must_use]
    pub fn findings(&self) -> &[StateMigrationCompatibilityFinding] {
        &self.findings
    }

    /// Returns whether no state records were discovered.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.empty
    }

    /// Returns whether no blocking compatibility finding exists.
    #[must_use]
    pub const fn is_healthy(&self) -> bool {
        self.healthy
    }

    /// Returns whether the known source shape is migration-compatible.
    #[must_use]
    pub const fn is_migration_compatible(&self) -> bool {
        self.migration_compatible
    }

    /// Returns whether a future migration must establish writer quiescence.
    #[must_use]
    pub const fn quiescence_required(&self) -> bool {
        self.quiescence_required
    }

    /// Returns the source semantic fingerprint when all state is accounted for.
    #[must_use]
    pub const fn source_fingerprint(&self) -> Option<&StateMigrationDigest> {
        self.source_fingerprint.as_ref()
    }

    /// Returns one family entry.
    #[must_use]
    pub fn record_count(
        &self,
        family: StateMigrationRecordFamily,
    ) -> Option<&StateMigrationRecordCount> {
        self.record_counts
            .iter()
            .find(|record| record.family() == family)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(clippy::struct_excessive_bools)]
struct StateMigrationInventoryWire {
    version: StateMigrationInventoryVersion,
    source_backend: DurableStateBackendKind,
    record_counts: Vec<StateMigrationRecordCount>,
    findings: Vec<StateMigrationCompatibilityFinding>,
    empty: bool,
    healthy: bool,
    migration_compatible: bool,
    quiescence_required: bool,
    source_fingerprint: Option<StateMigrationDigest>,
}

impl<'de> Deserialize<'de> for StateMigrationInventory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = StateMigrationInventoryWire::deserialize(deserializer)?;
        let inventory = Self::new(wire.record_counts, wire.findings, wire.quiescence_required)
            .map_err(|_| serde::de::Error::custom("state migration inventory is invalid"))?;
        if wire.version != inventory.version
            || wire.source_backend != inventory.source_backend
            || wire.empty != inventory.empty
            || wire.healthy != inventory.healthy
            || wire.migration_compatible != inventory.migration_compatible
            || wire.source_fingerprint != inventory.source_fingerprint
        {
            return Err(serde::de::Error::custom(
                "state migration inventory derived posture is invalid",
            ));
        }
        Ok(inventory)
    }
}

/// Immutable source binding derived from a compatible read-only inventory.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct StateMigrationSource {
    backend_kind: DurableStateBackendKind,
    inventory_version: StateMigrationInventoryVersion,
    source_fingerprint: StateMigrationDigest,
    quiescence_required: bool,
}

impl StateMigrationSource {
    /// Binds a migration source to one compatible inventory fingerprint.
    ///
    /// # Errors
    ///
    /// Returns a stable error when the inventory is not migration-compatible
    /// or has no complete source fingerprint.
    pub fn from_inventory(inventory: &StateMigrationInventory) -> Result<Self, WorkflowOsError> {
        if !inventory.is_migration_compatible() {
            return Err(migration_error(
                "source.incompatible",
                "state migration source inventory is not compatible",
            ));
        }
        let source_fingerprint = inventory.source_fingerprint().cloned().ok_or_else(|| {
            migration_error(
                "source.fingerprint_missing",
                "state migration source fingerprint is unavailable",
            )
        })?;
        Ok(Self {
            backend_kind: inventory.source_backend(),
            inventory_version: inventory.version(),
            source_fingerprint,
            quiescence_required: inventory.quiescence_required(),
        })
    }

    /// Returns the source backend kind.
    #[must_use]
    pub const fn backend_kind(&self) -> DurableStateBackendKind {
        self.backend_kind
    }

    /// Returns the inventory contract version.
    #[must_use]
    pub const fn inventory_version(&self) -> StateMigrationInventoryVersion {
        self.inventory_version
    }

    /// Returns the source semantic fingerprint.
    #[must_use]
    pub const fn source_fingerprint(&self) -> &StateMigrationDigest {
        &self.source_fingerprint
    }

    /// Returns whether a future importer must establish source quiescence.
    #[must_use]
    pub const fn quiescence_required(&self) -> bool {
        self.quiescence_required
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.backend_kind != DurableStateBackendKind::LocalFilesystemPreview {
            return Err(migration_error(
                "source.backend.invalid",
                "state migration source backend is invalid",
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for StateMigrationSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StateMigrationSource")
            .field("backend_kind", &self.backend_kind)
            .field("inventory_version", &self.inventory_version)
            .field("source_fingerprint", &"<redacted>")
            .field("quiescence_required", &self.quiescence_required)
            .finish()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StateMigrationSourceWire {
    backend_kind: DurableStateBackendKind,
    inventory_version: StateMigrationInventoryVersion,
    source_fingerprint: StateMigrationDigest,
    quiescence_required: bool,
}

impl<'de> Deserialize<'de> for StateMigrationSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = StateMigrationSourceWire::deserialize(deserializer)?;
        let source = Self {
            backend_kind: wire.backend_kind,
            inventory_version: wire.inventory_version,
            source_fingerprint: wire.source_fingerprint,
            quiescence_required: wire.quiescence_required,
        };
        source
            .validate()
            .map_err(|_| serde::de::Error::custom("state migration source is invalid"))?;
        Ok(source)
    }
}

/// Lifecycle posture of a migration destination.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateMigrationDestinationPosture {
    /// Destination is unreachable by ordinary runtime backend selection.
    Staging,
}

/// Logical, path-free identity and safety posture for a future `SQLite` target.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StateMigrationDestination {
    destination_id: StateMigrationDestinationId,
    backend_kind: DurableStateBackendKind,
    adapter_schema_version: u32,
    posture: StateMigrationDestinationPosture,
    empty_required: bool,
    runtime_selectable: bool,
}

impl StateMigrationDestination {
    /// Creates an unreachable, empty-required `SQLite` staging destination.
    ///
    /// # Errors
    ///
    /// Returns a stable error when the adapter schema version is zero.
    pub fn staging_sqlite(
        destination_id: StateMigrationDestinationId,
        adapter_schema_version: u32,
    ) -> Result<Self, WorkflowOsError> {
        if adapter_schema_version == 0 {
            return Err(migration_error(
                "destination.schema_version.invalid",
                "state migration destination schema version is invalid",
            ));
        }
        Ok(Self {
            destination_id,
            backend_kind: DurableStateBackendKind::EmbeddedSqlite,
            adapter_schema_version,
            posture: StateMigrationDestinationPosture::Staging,
            empty_required: true,
            runtime_selectable: false,
        })
    }

    /// Returns the logical destination identity.
    #[must_use]
    pub const fn destination_id(&self) -> &StateMigrationDestinationId {
        &self.destination_id
    }

    /// Returns the destination backend kind.
    #[must_use]
    pub const fn backend_kind(&self) -> DurableStateBackendKind {
        self.backend_kind
    }

    /// Returns the required destination adapter schema version.
    #[must_use]
    pub const fn adapter_schema_version(&self) -> u32 {
        self.adapter_schema_version
    }

    /// Returns the destination lifecycle posture.
    #[must_use]
    pub const fn posture(&self) -> StateMigrationDestinationPosture {
        self.posture
    }

    /// Returns whether a future importer must prove the destination is empty.
    #[must_use]
    pub const fn empty_required(&self) -> bool {
        self.empty_required
    }

    /// Returns whether ordinary runtime backend selection may open this target.
    #[must_use]
    pub const fn runtime_selectable(&self) -> bool {
        self.runtime_selectable
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StateMigrationDestinationWire {
    destination_id: StateMigrationDestinationId,
    backend_kind: DurableStateBackendKind,
    adapter_schema_version: u32,
    posture: StateMigrationDestinationPosture,
    empty_required: bool,
    runtime_selectable: bool,
}

impl<'de> Deserialize<'de> for StateMigrationDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = StateMigrationDestinationWire::deserialize(deserializer)?;
        let destination = Self::staging_sqlite(wire.destination_id, wire.adapter_schema_version)
            .map_err(|_| serde::de::Error::custom("state migration destination is invalid"))?;
        if wire.backend_kind != destination.backend_kind
            || wire.posture != destination.posture
            || wire.empty_required != destination.empty_required
            || wire.runtime_selectable != destination.runtime_selectable
        {
            return Err(serde::de::Error::custom(
                "state migration destination posture is invalid",
            ));
        }
        Ok(destination)
    }
}

/// One deterministic family operation in a migration plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct StateMigrationPlanStep {
    sequence: u16,
    family: StateMigrationRecordFamily,
    disposition: StateMigrationDisposition,
}

impl StateMigrationPlanStep {
    const fn new(sequence: u16, family: StateMigrationRecordFamily) -> Self {
        Self {
            sequence,
            family,
            disposition: family.disposition(),
        }
    }

    /// Returns the one-based execution sequence.
    #[must_use]
    pub const fn sequence(self) -> u16 {
        self.sequence
    }

    /// Returns the record family.
    #[must_use]
    pub const fn family(self) -> StateMigrationRecordFamily {
        self.family
    }

    /// Returns the required family disposition.
    #[must_use]
    pub const fn disposition(self) -> StateMigrationDisposition {
        self.disposition
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StateMigrationPlanStepWire {
    sequence: u16,
    family: StateMigrationRecordFamily,
    disposition: StateMigrationDisposition,
}

impl<'de> Deserialize<'de> for StateMigrationPlanStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = StateMigrationPlanStepWire::deserialize(deserializer)?;
        if wire.sequence == 0 || wire.disposition != wire.family.disposition() {
            return Err(serde::de::Error::custom(
                "state migration plan step is invalid",
            ));
        }
        Ok(Self {
            sequence: wire.sequence,
            family: wire.family,
            disposition: wire.disposition,
        })
    }
}

/// Interruption posture for a future importer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateMigrationResumePolicy {
    /// Resume is valid only for the exact immutable plan fingerprint.
    ExactPlanOnly,
}

/// Typed verification obligation required before future activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateMigrationVerificationRequirement {
    /// Recheck that the source fingerprint is unchanged.
    SourceFingerprintUnchanged,
    /// Prove that the destination was empty before import.
    DestinationInitiallyEmpty,
    /// Compare canonical per-family counts and digests.
    CanonicalCountsAndDigests,
    /// Verify event identity, sequence continuity, ordering, and digest.
    EventOrderingAndIdentity,
    /// Rehydrate every workflow run successfully.
    RunRehydration,
    /// Rebuild and compare derivable projections.
    ProjectionRebuildConsistency,
    /// Verify approval-presentation run and approval references.
    ApprovalPresentationReferentialIntegrity,
    /// Verify `SideEffect` workflow, run, identity, and lifecycle state.
    SideEffectReferentialIntegrity,
    /// Verify `WorkReport` run and `SideEffect` references.
    WorkReportReferentialIntegrity,
    /// Verify adapter telemetry run identity.
    AdapterTelemetryRunIdentity,
    /// Verify project and audit record identities.
    ProjectAndAuditIdentity,
    /// Prove that no process-local locks were imported.
    NoLocksImported,
    /// Prove that recognized companion state remains available.
    CompanionStateRetained,
    /// Reject unknown destination record families.
    NoUnknownDestinationRecords,
    /// Require healthy schema metadata and `SQLite` `quick_check`.
    SqliteSchemaAndQuickCheckHealthy,
}

impl StateMigrationVerificationRequirement {
    const ALL: [Self; 15] = [
        Self::SourceFingerprintUnchanged,
        Self::DestinationInitiallyEmpty,
        Self::CanonicalCountsAndDigests,
        Self::EventOrderingAndIdentity,
        Self::RunRehydration,
        Self::ProjectionRebuildConsistency,
        Self::ApprovalPresentationReferentialIntegrity,
        Self::SideEffectReferentialIntegrity,
        Self::WorkReportReferentialIntegrity,
        Self::AdapterTelemetryRunIdentity,
        Self::ProjectAndAuditIdentity,
        Self::NoLocksImported,
        Self::CompanionStateRetained,
        Self::NoUnknownDestinationRecords,
        Self::SqliteSchemaAndQuickCheckHealthy,
    ];

    /// Returns every v1 verification obligation in stable order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &Self::ALL
    }
}

/// Immutable, payload-free filesystem-to-SQLite staging migration plan.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StateMigrationPlan {
    migration_id: StateMigrationId,
    version: StateMigrationPlanVersion,
    source: StateMigrationSource,
    destination: StateMigrationDestination,
    plan_fingerprint: StateMigrationDigest,
    steps: Vec<StateMigrationPlanStep>,
    resume_policy: StateMigrationResumePolicy,
    verification_requirements: Vec<StateMigrationVerificationRequirement>,
    source_recheck_required: bool,
    activation_separate: bool,
}

impl StateMigrationPlan {
    /// Creates a deterministic staging plan from one accepted inventory.
    ///
    /// # Errors
    ///
    /// Returns a stable error for an incompatible source or invalid
    /// destination schema version.
    pub fn new(
        migration_id: StateMigrationId,
        inventory: &StateMigrationInventory,
        destination_id: StateMigrationDestinationId,
        adapter_schema_version: u32,
    ) -> Result<Self, WorkflowOsError> {
        let source = StateMigrationSource::from_inventory(inventory)?;
        let destination =
            StateMigrationDestination::staging_sqlite(destination_id, adapter_schema_version)?;
        Ok(Self::from_validated_parts(
            migration_id,
            source,
            destination,
        ))
    }

    fn from_validated_parts(
        migration_id: StateMigrationId,
        source: StateMigrationSource,
        destination: StateMigrationDestination,
    ) -> Self {
        let steps = canonical_plan_steps();
        let verification_requirements = StateMigrationVerificationRequirement::all().to_vec();
        let plan_fingerprint =
            derive_plan_fingerprint(&migration_id, &source, &destination, &steps);
        Self {
            migration_id,
            version: StateMigrationPlanVersion::V1,
            source,
            destination,
            plan_fingerprint,
            steps,
            resume_policy: StateMigrationResumePolicy::ExactPlanOnly,
            verification_requirements,
            source_recheck_required: true,
            activation_separate: true,
        }
    }

    /// Returns the caller-supplied migration identity.
    #[must_use]
    pub const fn migration_id(&self) -> &StateMigrationId {
        &self.migration_id
    }

    /// Returns the migration-plan contract version.
    #[must_use]
    pub const fn version(&self) -> StateMigrationPlanVersion {
        self.version
    }

    /// Returns the immutable source binding.
    #[must_use]
    pub const fn source(&self) -> &StateMigrationSource {
        &self.source
    }

    /// Returns the unreachable staging destination identity.
    #[must_use]
    pub const fn destination(&self) -> &StateMigrationDestination {
        &self.destination
    }

    /// Returns the digest binding identity, source, destination, and plan shape.
    #[must_use]
    pub const fn plan_fingerprint(&self) -> &StateMigrationDigest {
        &self.plan_fingerprint
    }

    /// Returns family operations in canonical dependency order.
    #[must_use]
    pub fn steps(&self) -> &[StateMigrationPlanStep] {
        &self.steps
    }

    /// Returns the interruption/resume posture.
    #[must_use]
    pub const fn resume_policy(&self) -> StateMigrationResumePolicy {
        self.resume_policy
    }

    /// Returns the required pre-activation verification obligations.
    #[must_use]
    pub fn verification_requirements(&self) -> &[StateMigrationVerificationRequirement] {
        &self.verification_requirements
    }

    /// Returns whether the source must be inventoried again before import.
    #[must_use]
    pub const fn source_recheck_required(&self) -> bool {
        self.source_recheck_required
    }

    /// Returns whether activation remains a separate future decision.
    #[must_use]
    pub const fn activation_separate(&self) -> bool {
        self.activation_separate
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StateMigrationPlanWire {
    migration_id: StateMigrationId,
    version: StateMigrationPlanVersion,
    source: StateMigrationSource,
    destination: StateMigrationDestination,
    plan_fingerprint: StateMigrationDigest,
    steps: Vec<StateMigrationPlanStep>,
    resume_policy: StateMigrationResumePolicy,
    verification_requirements: Vec<StateMigrationVerificationRequirement>,
    source_recheck_required: bool,
    activation_separate: bool,
}

impl<'de> Deserialize<'de> for StateMigrationPlan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = StateMigrationPlanWire::deserialize(deserializer)?;
        wire.source
            .validate()
            .map_err(|_| serde::de::Error::custom("state migration source is invalid"))?;
        let plan = Self::from_validated_parts(wire.migration_id, wire.source, wire.destination);
        if wire.version != plan.version
            || wire.plan_fingerprint != plan.plan_fingerprint
            || wire.steps != plan.steps
            || wire.resume_policy != plan.resume_policy
            || wire.verification_requirements != plan.verification_requirements
            || wire.source_recheck_required != plan.source_recheck_required
            || wire.activation_separate != plan.activation_separate
        {
            return Err(serde::de::Error::custom(
                "state migration plan derived posture is invalid",
            ));
        }
        Ok(plan)
    }
}

const PLAN_FAMILY_ORDER: [StateMigrationRecordFamily; 16] = [
    StateMigrationRecordFamily::WorkflowEvents,
    StateMigrationRecordFamily::EventIdIndexes,
    StateMigrationRecordFamily::RunSnapshots,
    StateMigrationRecordFamily::PendingApprovalProjections,
    StateMigrationRecordFamily::ApprovalPresentationRecords,
    StateMigrationRecordFamily::ApprovalPresentationIdIndexes,
    StateMigrationRecordFamily::IdempotencyResults,
    StateMigrationRecordFamily::ProjectStateRecords,
    StateMigrationRecordFamily::PolicyAuditRecords,
    StateMigrationRecordFamily::AdapterAuditRecords,
    StateMigrationRecordFamily::AdapterObservabilityRecords,
    StateMigrationRecordFamily::SideEffectRecords,
    StateMigrationRecordFamily::SideEffectIdIndexes,
    StateMigrationRecordFamily::WorkReportArtifacts,
    StateMigrationRecordFamily::LocalLocks,
    StateMigrationRecordFamily::ImmutableRunBundles,
];

fn canonical_plan_steps() -> Vec<StateMigrationPlanStep> {
    PLAN_FAMILY_ORDER
        .iter()
        .zip(1_u16..)
        .map(|(family, sequence)| StateMigrationPlanStep::new(sequence, *family))
        .collect()
}

fn derive_plan_fingerprint(
    migration_id: &StateMigrationId,
    source: &StateMigrationSource,
    destination: &StateMigrationDestination,
    steps: &[StateMigrationPlanStep],
) -> StateMigrationDigest {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "plan_version", "v1");
    hash_field(&mut hasher, "migration_id", migration_id.as_str());
    hash_field(
        &mut hasher,
        "source_fingerprint",
        source.source_fingerprint().as_str(),
    );
    hash_field(
        &mut hasher,
        "destination_id",
        destination.destination_id().as_str(),
    );
    hash_field(
        &mut hasher,
        "adapter_schema_version",
        &destination.adapter_schema_version().to_string(),
    );
    for step in steps {
        hash_field(&mut hasher, "family", step.family().stable_label());
        hash_field(
            &mut hasher,
            "disposition",
            match step.disposition() {
                StateMigrationDisposition::CanonicalImport => "canonical_import",
                StateMigrationDisposition::ProjectionRebuild => "projection_rebuild",
                StateMigrationDisposition::EphemeralExclude => "ephemeral_exclude",
                StateMigrationDisposition::CompanionPreserve => "companion_preserve",
            },
        );
    }
    StateMigrationDigest::from_hasher(hasher)
}

fn derive_source_fingerprint(
    records: &[StateMigrationRecordCount],
    findings: &[StateMigrationCompatibilityFinding],
    quiescence_required: bool,
) -> Option<StateMigrationDigest> {
    if records.iter().any(|record| record.digest().is_none())
        || findings
            .iter()
            .any(|finding| finding.code().prevents_fingerprint())
    {
        return None;
    }

    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "version", "v1");
    hash_field(
        &mut hasher,
        "quiescence_required",
        if quiescence_required { "true" } else { "false" },
    );
    for record in records {
        let digest = record.digest()?;
        hash_field(&mut hasher, "family", record.family().stable_label());
        hash_field(
            &mut hasher,
            "disposition",
            match record.disposition() {
                StateMigrationDisposition::CanonicalImport => "canonical_import",
                StateMigrationDisposition::ProjectionRebuild => "projection_rebuild",
                StateMigrationDisposition::EphemeralExclude => "ephemeral_exclude",
                StateMigrationDisposition::CompanionPreserve => "companion_preserve",
            },
        );
        hash_field(&mut hasher, "count", &record.count().to_string());
        hash_field(&mut hasher, "digest", digest.as_str());
    }
    Some(StateMigrationDigest::from_hasher(hasher))
}

pub(crate) fn hash_field(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update(label.len().to_le_bytes());
    hasher.update(label.as_bytes());
    hasher.update(value.len().to_le_bytes());
    hasher.update(value.as_bytes());
}

fn validate_migration_identifier(
    type_name: &str,
    error_suffix: &str,
    value: &str,
) -> Result<(), WorkflowOsError> {
    if value.is_empty()
        || value.len() > MIGRATION_IDENTIFIER_MAX_BYTES
        || !value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'/' | b'.' | b':')
        })
    {
        return Err(migration_error(
            error_suffix,
            &format!("{type_name} is invalid"),
        ));
    }
    let lowercase = value.to_ascii_lowercase();
    if [
        "authorization",
        "bearer",
        "api_key",
        "api-key",
        "apikey",
        "private_key",
        "private-key",
        "token",
        "secret",
        "credential",
        "password",
    ]
    .iter()
    .any(|marker| lowercase.contains(marker))
    {
        return Err(migration_error(
            error_suffix,
            &format!("{type_name} is invalid"),
        ));
    }
    Ok(())
}

fn migration_error(suffix: &str, message: &str) -> WorkflowOsError {
    WorkflowOsError::new(
        WorkflowOsErrorKind::Validation,
        format!("state.migration.{suffix}"),
        message,
    )
}
