use std::collections::BTreeSet;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{DurableStateBackendKind, WorkflowOsError, WorkflowOsErrorKind};

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

fn migration_error(suffix: &str, message: &str) -> WorkflowOsError {
    WorkflowOsError::new(
        WorkflowOsErrorKind::Validation,
        format!("state.migration.{suffix}"),
        message,
    )
}
