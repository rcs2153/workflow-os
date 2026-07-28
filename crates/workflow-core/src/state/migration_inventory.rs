use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::{
    encode_key, read_json, ApprovalPresentationIdRecord, EventIdRecord, IdempotencyResult,
    LocalStateBackend, LocalStateIssueSeverity, LockLease, ProjectStateRecord, SideEffectIdRecord,
};
use crate::{
    AdapterRuntimeAuditRecord, AdapterRuntimeObservabilityRecord, ApprovalPresentationRecord,
    ApprovalRequest, PolicyAuditRecord, SideEffectRecord, StateMigrationCompatibilityFinding,
    StateMigrationDigest, StateMigrationFindingCode, StateMigrationFindingSeverity,
    StateMigrationInventory, StateMigrationRecordCount, StateMigrationRecordFamily,
    WorkReportArtifactRecord, WorkflowOsError, WorkflowRunEvent, WorkflowRunSnapshot,
};

const IMMUTABLE_BUNDLE_DIRS: [&str; 4] = [
    "definition-records",
    "local-check-declaration-set-records",
    "governance-assessment-bindings",
    "manifests",
];

#[derive(Clone)]
struct FamilyAccumulator {
    family: StateMigrationRecordFamily,
    count: u64,
    hasher: Sha256,
    valid: bool,
}

impl FamilyAccumulator {
    fn new(family: StateMigrationRecordFamily) -> Self {
        let mut hasher = Sha256::new();
        hash_bytes(&mut hasher, b"family", family.stable_label().as_bytes());
        Self {
            family,
            count: 0,
            hasher,
            valid: true,
        }
    }

    fn discovered(&mut self) {
        self.count = self.count.saturating_add(1);
    }

    fn invalidate(&mut self) {
        self.valid = false;
    }

    fn update<T: Serialize>(&mut self, value: &T) -> bool {
        let Ok(value) = serde_json::to_value(value) else {
            self.invalidate();
            return false;
        };
        hash_json_value(&mut self.hasher, &value);
        true
    }

    fn update_address(&mut self, address: &str) {
        hash_bytes(&mut self.hasher, b"address", address.as_bytes());
    }

    fn update_opaque(&mut self, address: &str, bytes: &[u8]) {
        hash_bytes(&mut self.hasher, b"address", address.as_bytes());
        hash_bytes(&mut self.hasher, b"content", bytes);
    }

    fn finish(self) -> Result<StateMigrationRecordCount, WorkflowOsError> {
        let digest = self
            .valid
            .then(|| StateMigrationDigest::from_hasher(self.hasher));
        StateMigrationRecordCount::new(self.family, self.family.disposition(), self.count, digest)
    }
}

impl LocalStateBackend {
    /// Inventories local filesystem state without creating or modifying files.
    ///
    /// The returned inventory contains only bounded family counts, digests,
    /// dispositions, and typed compatibility findings. It never contains the
    /// source path or record payloads.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error only when the bounded inventory
    /// result itself cannot be constructed.
    #[allow(clippy::too_many_lines)]
    pub fn inspect_migration_inventory(&self) -> Result<StateMigrationInventory, WorkflowOsError> {
        let mut accumulators = StateMigrationRecordFamily::all()
            .iter()
            .copied()
            .map(|family| (family, FamilyAccumulator::new(family)))
            .collect::<BTreeMap<_, _>>();
        let mut findings = Vec::new();

        for issue in self.inspect_state().issues {
            let (severity, code) = match issue.severity {
                LocalStateIssueSeverity::Error => (
                    StateMigrationFindingSeverity::Blocker,
                    StateMigrationFindingCode::SourceUnhealthy,
                ),
                LocalStateIssueSeverity::Warning => (
                    StateMigrationFindingSeverity::Warning,
                    StateMigrationFindingCode::SourceWarning,
                ),
            };
            push_finding(&mut findings, severity, code, None);
        }

        if !self.root().exists() {
            return finish_inventory(accumulators, findings);
        }
        let Ok(metadata) = fs::symlink_metadata(self.root()) else {
            push_finding(
                &mut findings,
                StateMigrationFindingSeverity::Blocker,
                StateMigrationFindingCode::SourceUnreadable,
                None,
            );
            invalidate_all(&mut accumulators);
            return finish_inventory(accumulators, findings);
        };
        if metadata.file_type().is_symlink() {
            push_finding(
                &mut findings,
                StateMigrationFindingSeverity::Blocker,
                StateMigrationFindingCode::SymlinkRejected,
                None,
            );
            invalidate_all(&mut accumulators);
            return finish_inventory(accumulators, findings);
        }
        if !metadata.is_dir() {
            push_finding(
                &mut findings,
                StateMigrationFindingSeverity::Blocker,
                StateMigrationFindingCode::UnexpectedFileType,
                None,
            );
            invalidate_all(&mut accumulators);
            return finish_inventory(accumulators, findings);
        }

        self.inspect_root_entries(&mut accumulators, &mut findings);
        self.scan_events(&mut accumulators, &mut findings);
        self.scan_event_indexes(&mut accumulators, &mut findings);
        Self::scan_flat_hashed_json::<WorkflowRunSnapshot>(
            &self.root().join("snapshots"),
            StateMigrationRecordFamily::RunSnapshots,
            &mut accumulators,
            &mut findings,
            |record| Some(record.identity.run_id.as_str().to_owned()),
        );
        Self::scan_flat_hashed_json::<IdempotencyResult>(
            &self.root().join("idempotency"),
            StateMigrationRecordFamily::IdempotencyResults,
            &mut accumulators,
            &mut findings,
            |_| None,
        );
        self.scan_locks(&mut accumulators, &mut findings);
        Self::scan_flat_hashed_json::<ApprovalRequest>(
            &self.root().join("approvals"),
            StateMigrationRecordFamily::PendingApprovalProjections,
            &mut accumulators,
            &mut findings,
            |record| Some(record.approval_id.as_str().to_owned()),
        );
        Self::scan_partitioned_hashed_json::<ApprovalPresentationRecord>(
            &self.root().join("approval_presentations").join("records"),
            StateMigrationRecordFamily::ApprovalPresentationRecords,
            &mut accumulators,
            &mut findings,
            |record| record.run_id().as_str().to_owned(),
            |record| record.presentation_id().as_str().to_owned(),
        );
        Self::scan_flat_hashed_json::<ApprovalPresentationIdRecord>(
            &self.root().join("approval_presentations").join("ids"),
            StateMigrationRecordFamily::ApprovalPresentationIdIndexes,
            &mut accumulators,
            &mut findings,
            |_| None,
        );
        Self::scan_flat_hashed_json::<ProjectStateRecord>(
            &self.root().join("projects"),
            StateMigrationRecordFamily::ProjectStateRecords,
            &mut accumulators,
            &mut findings,
            |record| Some(record.project_id.as_str().to_owned()),
        );
        Self::scan_flat_hashed_json::<PolicyAuditRecord>(
            &self.root().join("policy_audit"),
            StateMigrationRecordFamily::PolicyAuditRecords,
            &mut accumulators,
            &mut findings,
            |record| Some(record.audit_id.as_str().to_owned()),
        );
        Self::scan_partitioned_hashed_json::<AdapterRuntimeAuditRecord>(
            &self.root().join("adapter_audit"),
            StateMigrationRecordFamily::AdapterAuditRecords,
            &mut accumulators,
            &mut findings,
            |record| {
                record
                    .workflow_run_id
                    .as_ref()
                    .map_or_else(String::new, |run_id| run_id.as_str().to_owned())
            },
            |record| record.telemetry_id.as_str().to_owned(),
        );
        Self::scan_partitioned_hashed_json::<AdapterRuntimeObservabilityRecord>(
            &self.root().join("adapter_observability"),
            StateMigrationRecordFamily::AdapterObservabilityRecords,
            &mut accumulators,
            &mut findings,
            |record| {
                record
                    .workflow_run_id
                    .as_ref()
                    .map_or_else(String::new, |run_id| run_id.as_str().to_owned())
            },
            |record| record.telemetry_id.as_str().to_owned(),
        );
        Self::scan_partitioned_hashed_json::<WorkReportArtifactRecord>(
            &self.root().join("work_reports"),
            StateMigrationRecordFamily::WorkReportArtifacts,
            &mut accumulators,
            &mut findings,
            |record| record.run_id().as_str().to_owned(),
            |record| record.report_id().as_str().to_owned(),
        );
        Self::scan_partitioned_hashed_json::<SideEffectRecord>(
            &self.root().join("side_effects").join("records"),
            StateMigrationRecordFamily::SideEffectRecords,
            &mut accumulators,
            &mut findings,
            |record| record.run_id().as_str().to_owned(),
            |record| record.side_effect_id().as_str().to_owned(),
        );
        Self::scan_flat_hashed_json::<SideEffectIdRecord>(
            &self.root().join("side_effects").join("ids"),
            StateMigrationRecordFamily::SideEffectIdIndexes,
            &mut accumulators,
            &mut findings,
            |_| None,
        );
        self.scan_immutable_bundles(&mut accumulators, &mut findings);
        self.validate_side_effect_indexes(&mut accumulators, &mut findings);
        self.validate_approval_presentation_indexes(&mut accumulators, &mut findings);

        finish_inventory(accumulators, findings)
    }

    fn inspect_root_entries(
        &self,
        accumulators: &mut BTreeMap<StateMigrationRecordFamily, FamilyAccumulator>,
        findings: &mut Vec<StateMigrationCompatibilityFinding>,
    ) {
        const KNOWN_ROOTS: [&str; 14] = [
            "events",
            "event_ids",
            "snapshots",
            "idempotency",
            "locks",
            "approvals",
            "projects",
            "policy_audit",
            "adapter_audit",
            "adapter_observability",
            "work_reports",
            "side_effects",
            "approval_presentations",
            "immutable-run-bundles",
        ];
        let Some(entries) = directory_entries(self.root(), findings, None) else {
            invalidate_all(accumulators);
            return;
        };
        for entry in entries {
            let Some(name) = file_name(&entry) else {
                push_finding(
                    findings,
                    StateMigrationFindingSeverity::Blocker,
                    StateMigrationFindingCode::MalformedStorageAddress,
                    None,
                );
                continue;
            };
            if !KNOWN_ROOTS.contains(&name.as_str()) {
                classify_unknown_entry(&entry, findings);
                continue;
            }
            validate_expected_directory(&entry, findings, None);
        }
        Self::inspect_group_root(
            &self.root().join("side_effects"),
            &["records", "ids"],
            findings,
        );
        Self::inspect_group_root(
            &self.root().join("approval_presentations"),
            &["records", "ids"],
            findings,
        );
    }

    fn inspect_group_root(
        root: &Path,
        known_children: &[&str],
        findings: &mut Vec<StateMigrationCompatibilityFinding>,
    ) {
        if !root.exists() {
            return;
        }
        let Some(entries) = directory_entries(root, findings, None) else {
            return;
        };
        for entry in entries {
            let Some(name) = file_name(&entry) else {
                push_finding(
                    findings,
                    StateMigrationFindingSeverity::Blocker,
                    StateMigrationFindingCode::MalformedStorageAddress,
                    None,
                );
                continue;
            };
            if known_children.contains(&name.as_str()) {
                validate_expected_directory(&entry, findings, None);
            } else {
                classify_unknown_entry(&entry, findings);
            }
        }
    }

    fn scan_events(
        &self,
        accumulators: &mut BTreeMap<StateMigrationRecordFamily, FamilyAccumulator>,
        findings: &mut Vec<StateMigrationCompatibilityFinding>,
    ) {
        let family = StateMigrationRecordFamily::WorkflowEvents;
        let directory = self.root().join("events");
        let paths = partitioned_event_paths(&directory, family, findings, accumulators);
        let mut identities = BTreeSet::new();
        for (partition, path) in paths {
            let accumulator = family_accumulator(accumulators, family);
            accumulator.discovered();
            let Ok(event) = read_json::<WorkflowRunEvent>(&path) else {
                invalid_record(accumulator, findings, family);
                continue;
            };
            let sequence_name = format!("{:020}.json", event.sequence_number.get());
            let address_matches = partition == encode_key(event.run_id.as_str())
                && file_name(&path).as_deref() == Some(sequence_name.as_str());
            if !address_matches {
                malformed_address(accumulator, findings, family);
                continue;
            }
            if !identities.insert(event.event_id.as_str().to_owned()) {
                duplicate_identity(accumulator, findings, family);
                continue;
            }
            if !accumulator.update(&event) {
                invalid_record(accumulator, findings, family);
            }
        }
    }

    fn scan_event_indexes(
        &self,
        accumulators: &mut BTreeMap<StateMigrationRecordFamily, FamilyAccumulator>,
        findings: &mut Vec<StateMigrationCompatibilityFinding>,
    ) {
        let family = StateMigrationRecordFamily::EventIdIndexes;
        let paths = flat_json_paths(
            &self.root().join("event_ids"),
            family,
            findings,
            accumulators,
        );
        for path in paths {
            let accumulator = family_accumulator(accumulators, family);
            accumulator.discovered();
            let Some(address) = file_name(&path) else {
                malformed_address(accumulator, findings, family);
                continue;
            };
            match read_json::<EventIdRecord>(&path) {
                Ok(record) if accumulator.update(&record) => {
                    accumulator.update_address(&address);
                }
                _ => invalid_record(accumulator, findings, family),
            }
        }
    }

    fn scan_locks(
        &self,
        accumulators: &mut BTreeMap<StateMigrationRecordFamily, FamilyAccumulator>,
        findings: &mut Vec<StateMigrationCompatibilityFinding>,
    ) {
        let family = StateMigrationRecordFamily::LocalLocks;
        let directory = self.root().join("locks");
        if !directory.exists() {
            return;
        }
        let Some(entries) = directory_entries(&directory, findings, Some(family)) else {
            family_accumulator(accumulators, family).invalidate();
            return;
        };
        for entry in entries {
            let accumulator = family_accumulator(accumulators, family);
            accumulator.discovered();
            let Some(name) = file_name(&entry) else {
                malformed_address(accumulator, findings, family);
                continue;
            };
            if !is_hash(&name) || !validate_expected_directory(&entry, findings, Some(family)) {
                accumulator.invalidate();
                continue;
            }
            let Some(children) = directory_entries(&entry, findings, Some(family)) else {
                accumulator.invalidate();
                continue;
            };
            if children.len() != 1 || file_name(&children[0]).as_deref() != Some("owner.json") {
                malformed_address(accumulator, findings, family);
                continue;
            }
            let owner_path = &children[0];
            if !is_regular_file(owner_path, findings, Some(family)) {
                accumulator.invalidate();
                continue;
            }
            let Ok(lease) = read_json::<LockLease>(owner_path) else {
                invalid_record(accumulator, findings, family);
                continue;
            };
            if encode_key(&lease.key) != name {
                malformed_address(accumulator, findings, family);
                continue;
            }
            if !accumulator.update(&lease) {
                invalid_record(accumulator, findings, family);
            }
        }
        if family_accumulator(accumulators, family).count > 0 {
            push_finding(
                findings,
                StateMigrationFindingSeverity::Blocker,
                StateMigrationFindingCode::LockPresent,
                Some(family),
            );
        }
    }

    fn scan_flat_hashed_json<T>(
        directory: &Path,
        family: StateMigrationRecordFamily,
        accumulators: &mut BTreeMap<StateMigrationRecordFamily, FamilyAccumulator>,
        findings: &mut Vec<StateMigrationCompatibilityFinding>,
        identity: impl Fn(&T) -> Option<String>,
    ) where
        T: DeserializeOwned + Serialize,
    {
        let paths = flat_json_paths(directory, family, findings, accumulators);
        let mut identities = BTreeSet::new();
        for path in paths {
            let accumulator = family_accumulator(accumulators, family);
            accumulator.discovered();
            let Some(address) = file_name(&path) else {
                malformed_address(accumulator, findings, family);
                continue;
            };
            let Ok(record) = read_json::<T>(&path) else {
                invalid_record(accumulator, findings, family);
                continue;
            };
            if let Some(identity) = identity(&record) {
                let expected = format!("{}.json", encode_key(&identity));
                if file_name(&path).as_deref() != Some(expected.as_str()) {
                    malformed_address(accumulator, findings, family);
                    continue;
                }
                if !identities.insert(identity) {
                    duplicate_identity(accumulator, findings, family);
                    continue;
                }
            }
            if accumulator.update(&record) {
                accumulator.update_address(&address);
            } else {
                invalid_record(accumulator, findings, family);
            }
        }
    }

    fn scan_partitioned_hashed_json<T>(
        directory: &Path,
        family: StateMigrationRecordFamily,
        accumulators: &mut BTreeMap<StateMigrationRecordFamily, FamilyAccumulator>,
        findings: &mut Vec<StateMigrationCompatibilityFinding>,
        partition_identity: impl Fn(&T) -> String,
        record_identity: impl Fn(&T) -> String,
    ) where
        T: DeserializeOwned + Serialize,
    {
        let paths = partitioned_json_paths(directory, family, findings, accumulators);
        let mut identities = BTreeSet::new();
        for (partition, path) in paths {
            let accumulator = family_accumulator(accumulators, family);
            accumulator.discovered();
            let Ok(record) = read_json::<T>(&path) else {
                invalid_record(accumulator, findings, family);
                continue;
            };
            let record_identity = record_identity(&record);
            let expected = format!("{}.json", encode_key(&record_identity));
            if partition != encode_key(&partition_identity(&record))
                || file_name(&path).as_deref() != Some(expected.as_str())
            {
                malformed_address(accumulator, findings, family);
                continue;
            }
            if !identities.insert(record_identity) {
                duplicate_identity(accumulator, findings, family);
                continue;
            }
            if !accumulator.update(&record) {
                invalid_record(accumulator, findings, family);
            }
        }
    }

    fn scan_immutable_bundles(
        &self,
        accumulators: &mut BTreeMap<StateMigrationRecordFamily, FamilyAccumulator>,
        findings: &mut Vec<StateMigrationCompatibilityFinding>,
    ) {
        let family = StateMigrationRecordFamily::ImmutableRunBundles;
        let root = self.root().join("immutable-run-bundles");
        if !root.exists() {
            return;
        }
        let Some(entries) = directory_entries(&root, findings, Some(family)) else {
            family_accumulator(accumulators, family).invalidate();
            return;
        };
        for entry in entries {
            let Some(name) = file_name(&entry) else {
                family_accumulator(accumulators, family).invalidate();
                push_finding(
                    findings,
                    StateMigrationFindingSeverity::Blocker,
                    StateMigrationFindingCode::MalformedStorageAddress,
                    Some(family),
                );
                continue;
            };
            if !IMMUTABLE_BUNDLE_DIRS.contains(&name.as_str()) {
                classify_unknown_entry(&entry, findings);
                continue;
            }
            if !validate_expected_directory(&entry, findings, Some(family)) {
                family_accumulator(accumulators, family).invalidate();
                continue;
            }
            let paths = flat_json_paths(&entry, family, findings, accumulators);
            for path in paths {
                let accumulator = family_accumulator(accumulators, family);
                accumulator.discovered();
                let Some(file_name) = file_name(&path) else {
                    malformed_address(accumulator, findings, family);
                    continue;
                };
                let Some(stem) = file_name.strip_suffix(".json") else {
                    malformed_address(accumulator, findings, family);
                    continue;
                };
                if !is_hash(stem) {
                    malformed_address(accumulator, findings, family);
                    continue;
                }
                match fs::read(&path) {
                    Ok(bytes) => {
                        accumulator.update_opaque(&format!("{name}/{file_name}"), bytes.as_slice());
                    }
                    Err(_) => invalid_record(accumulator, findings, family),
                }
            }
        }
    }

    fn validate_side_effect_indexes(
        &self,
        accumulators: &mut BTreeMap<StateMigrationRecordFamily, FamilyAccumulator>,
        findings: &mut Vec<StateMigrationCompatibilityFinding>,
    ) {
        let family = StateMigrationRecordFamily::SideEffectIdIndexes;
        let directory = self.root().join("side_effects").join("ids");
        if !directory.exists() {
            return;
        }
        let Some(entries) = directory_entries(&directory, findings, Some(family)) else {
            family_accumulator(accumulators, family).invalidate();
            return;
        };
        for path in entries {
            let Ok(index) = read_json::<SideEffectIdRecord>(&path) else {
                family_accumulator(accumulators, family).invalidate();
                continue;
            };
            let Some(stem) =
                file_name(&path).and_then(|name| name.strip_suffix(".json").map(str::to_owned))
            else {
                family_accumulator(accumulators, family).invalidate();
                continue;
            };
            let record_path = self
                .root()
                .join("side_effects")
                .join("records")
                .join(encode_key(index.run_id.as_str()))
                .join(format!("{stem}.json"));
            let Ok(record) = read_json::<SideEffectRecord>(&record_path) else {
                family_accumulator(accumulators, family).invalidate();
                push_finding(
                    findings,
                    StateMigrationFindingSeverity::Blocker,
                    StateMigrationFindingCode::IndexInconsistent,
                    Some(family),
                );
                continue;
            };
            if encode_key(record.side_effect_id().as_str()) != stem
                || record.run_id() != &index.run_id
            {
                family_accumulator(accumulators, family).invalidate();
                push_finding(
                    findings,
                    StateMigrationFindingSeverity::Blocker,
                    StateMigrationFindingCode::IndexInconsistent,
                    Some(family),
                );
            }
        }
    }

    fn validate_approval_presentation_indexes(
        &self,
        accumulators: &mut BTreeMap<StateMigrationRecordFamily, FamilyAccumulator>,
        findings: &mut Vec<StateMigrationCompatibilityFinding>,
    ) {
        let family = StateMigrationRecordFamily::ApprovalPresentationIdIndexes;
        let directory = self.root().join("approval_presentations").join("ids");
        if !directory.exists() {
            return;
        }
        let Some(entries) = directory_entries(&directory, findings, Some(family)) else {
            family_accumulator(accumulators, family).invalidate();
            return;
        };
        for path in entries {
            let Ok(index) = read_json::<ApprovalPresentationIdRecord>(&path) else {
                family_accumulator(accumulators, family).invalidate();
                continue;
            };
            let Some(stem) =
                file_name(&path).and_then(|name| name.strip_suffix(".json").map(str::to_owned))
            else {
                family_accumulator(accumulators, family).invalidate();
                continue;
            };
            let record_path = self
                .root()
                .join("approval_presentations")
                .join("records")
                .join(encode_key(index.run_id.as_str()))
                .join(format!("{stem}.json"));
            let Ok(record) = read_json::<ApprovalPresentationRecord>(&record_path) else {
                family_accumulator(accumulators, family).invalidate();
                push_finding(
                    findings,
                    StateMigrationFindingSeverity::Blocker,
                    StateMigrationFindingCode::IndexInconsistent,
                    Some(family),
                );
                continue;
            };
            if encode_key(record.presentation_id().as_str()) != stem
                || record.run_id() != &index.run_id
            {
                family_accumulator(accumulators, family).invalidate();
                push_finding(
                    findings,
                    StateMigrationFindingSeverity::Blocker,
                    StateMigrationFindingCode::IndexInconsistent,
                    Some(family),
                );
            }
        }
    }
}

fn finish_inventory(
    accumulators: BTreeMap<StateMigrationRecordFamily, FamilyAccumulator>,
    findings: Vec<StateMigrationCompatibilityFinding>,
) -> Result<StateMigrationInventory, WorkflowOsError> {
    let records = accumulators
        .into_values()
        .map(FamilyAccumulator::finish)
        .collect::<Result<Vec<_>, _>>()?;
    StateMigrationInventory::new(records, findings, true)
}

fn invalidate_all(accumulators: &mut BTreeMap<StateMigrationRecordFamily, FamilyAccumulator>) {
    for accumulator in accumulators.values_mut() {
        accumulator.invalidate();
    }
}

fn family_accumulator(
    accumulators: &mut BTreeMap<StateMigrationRecordFamily, FamilyAccumulator>,
    family: StateMigrationRecordFamily,
) -> &mut FamilyAccumulator {
    accumulators
        .entry(family)
        .or_insert_with(|| FamilyAccumulator::new(family))
}

fn flat_json_paths(
    directory: &Path,
    family: StateMigrationRecordFamily,
    findings: &mut Vec<StateMigrationCompatibilityFinding>,
    accumulators: &mut BTreeMap<StateMigrationRecordFamily, FamilyAccumulator>,
) -> Vec<PathBuf> {
    if !directory.exists() {
        return Vec::new();
    }
    let Some(entries) = directory_entries(directory, findings, Some(family)) else {
        family_accumulator(accumulators, family).invalidate();
        return Vec::new();
    };
    let mut paths = Vec::new();
    for entry in entries {
        if !is_regular_file(&entry, findings, Some(family)) {
            family_accumulator(accumulators, family).invalidate();
            continue;
        }
        let Some(name) = file_name(&entry) else {
            malformed_address(family_accumulator(accumulators, family), findings, family);
            continue;
        };
        let Some(stem) = name.strip_suffix(".json") else {
            malformed_address(family_accumulator(accumulators, family), findings, family);
            continue;
        };
        if !is_hash(stem) {
            malformed_address(family_accumulator(accumulators, family), findings, family);
            continue;
        }
        paths.push(entry);
    }
    paths.sort();
    paths
}

fn partitioned_json_paths(
    directory: &Path,
    family: StateMigrationRecordFamily,
    findings: &mut Vec<StateMigrationCompatibilityFinding>,
    accumulators: &mut BTreeMap<StateMigrationRecordFamily, FamilyAccumulator>,
) -> Vec<(String, PathBuf)> {
    if !directory.exists() {
        return Vec::new();
    }
    let Some(entries) = directory_entries(directory, findings, Some(family)) else {
        family_accumulator(accumulators, family).invalidate();
        return Vec::new();
    };
    let mut paths = Vec::new();
    for entry in entries {
        let Some(partition) = file_name(&entry) else {
            malformed_address(family_accumulator(accumulators, family), findings, family);
            continue;
        };
        if !is_hash(&partition) || !validate_expected_directory(&entry, findings, Some(family)) {
            family_accumulator(accumulators, family).invalidate();
            continue;
        }
        for path in flat_json_paths(&entry, family, findings, accumulators) {
            paths.push((partition.clone(), path));
        }
    }
    paths.sort();
    paths
}

fn partitioned_event_paths(
    directory: &Path,
    family: StateMigrationRecordFamily,
    findings: &mut Vec<StateMigrationCompatibilityFinding>,
    accumulators: &mut BTreeMap<StateMigrationRecordFamily, FamilyAccumulator>,
) -> Vec<(String, PathBuf)> {
    if !directory.exists() {
        return Vec::new();
    }
    let Some(entries) = directory_entries(directory, findings, Some(family)) else {
        family_accumulator(accumulators, family).invalidate();
        return Vec::new();
    };
    let mut paths = Vec::new();
    for entry in entries {
        let Some(partition) = file_name(&entry) else {
            malformed_address(family_accumulator(accumulators, family), findings, family);
            continue;
        };
        if !is_hash(&partition) || !validate_expected_directory(&entry, findings, Some(family)) {
            family_accumulator(accumulators, family).invalidate();
            continue;
        }
        let Some(event_paths) = directory_entries(&entry, findings, Some(family)) else {
            family_accumulator(accumulators, family).invalidate();
            continue;
        };
        for path in event_paths {
            if !is_regular_file(&path, findings, Some(family)) {
                family_accumulator(accumulators, family).invalidate();
                continue;
            }
            paths.push((partition.clone(), path));
        }
    }
    paths.sort();
    paths
}

fn directory_entries(
    directory: &Path,
    findings: &mut Vec<StateMigrationCompatibilityFinding>,
    family: Option<StateMigrationRecordFamily>,
) -> Option<Vec<PathBuf>> {
    let Ok(metadata) = fs::symlink_metadata(directory) else {
        push_finding(
            findings,
            StateMigrationFindingSeverity::Blocker,
            StateMigrationFindingCode::SourceUnreadable,
            family,
        );
        return None;
    };
    if metadata.file_type().is_symlink() {
        push_finding(
            findings,
            StateMigrationFindingSeverity::Blocker,
            StateMigrationFindingCode::SymlinkRejected,
            family,
        );
        return None;
    }
    if !metadata.is_dir() {
        push_finding(
            findings,
            StateMigrationFindingSeverity::Blocker,
            StateMigrationFindingCode::UnexpectedFileType,
            family,
        );
        return None;
    }
    let Ok(entries) = fs::read_dir(directory) else {
        push_finding(
            findings,
            StateMigrationFindingSeverity::Blocker,
            StateMigrationFindingCode::SourceUnreadable,
            family,
        );
        return None;
    };
    let mut paths = Vec::new();
    for entry in entries {
        let Ok(entry) = entry else {
            push_finding(
                findings,
                StateMigrationFindingSeverity::Blocker,
                StateMigrationFindingCode::SourceUnreadable,
                family,
            );
            return None;
        };
        paths.push(entry.path());
    }
    paths.sort();
    Some(paths)
}

fn validate_expected_directory(
    path: &Path,
    findings: &mut Vec<StateMigrationCompatibilityFinding>,
    family: Option<StateMigrationRecordFamily>,
) -> bool {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        push_finding(
            findings,
            StateMigrationFindingSeverity::Blocker,
            StateMigrationFindingCode::SourceUnreadable,
            family,
        );
        return false;
    };
    if metadata.file_type().is_symlink() {
        push_finding(
            findings,
            StateMigrationFindingSeverity::Blocker,
            StateMigrationFindingCode::SymlinkRejected,
            family,
        );
        return false;
    }
    if !metadata.is_dir() {
        push_finding(
            findings,
            StateMigrationFindingSeverity::Blocker,
            StateMigrationFindingCode::UnexpectedFileType,
            family,
        );
        return false;
    }
    true
}

fn is_regular_file(
    path: &Path,
    findings: &mut Vec<StateMigrationCompatibilityFinding>,
    family: Option<StateMigrationRecordFamily>,
) -> bool {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        push_finding(
            findings,
            StateMigrationFindingSeverity::Blocker,
            StateMigrationFindingCode::SourceUnreadable,
            family,
        );
        return false;
    };
    if metadata.file_type().is_symlink() {
        push_finding(
            findings,
            StateMigrationFindingSeverity::Blocker,
            StateMigrationFindingCode::SymlinkRejected,
            family,
        );
        return false;
    }
    if !metadata.is_file() {
        push_finding(
            findings,
            StateMigrationFindingSeverity::Blocker,
            StateMigrationFindingCode::UnexpectedFileType,
            family,
        );
        return false;
    }
    true
}

fn classify_unknown_entry(path: &Path, findings: &mut Vec<StateMigrationCompatibilityFinding>) {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        push_finding(
            findings,
            StateMigrationFindingSeverity::Blocker,
            StateMigrationFindingCode::SourceUnreadable,
            None,
        );
        return;
    };
    if metadata.file_type().is_symlink() {
        push_finding(
            findings,
            StateMigrationFindingSeverity::Blocker,
            StateMigrationFindingCode::SymlinkRejected,
            None,
        );
        return;
    }
    let empty_directory = metadata.is_dir()
        && fs::read_dir(path)
            .ok()
            .is_some_and(|mut entries| entries.next().is_none());
    push_finding(
        findings,
        if empty_directory {
            StateMigrationFindingSeverity::Warning
        } else {
            StateMigrationFindingSeverity::Blocker
        },
        if empty_directory {
            StateMigrationFindingCode::UnknownEmptyDirectory
        } else {
            StateMigrationFindingCode::UnknownRecordFamily
        },
        None,
    );
}

fn invalid_record(
    accumulator: &mut FamilyAccumulator,
    findings: &mut Vec<StateMigrationCompatibilityFinding>,
    family: StateMigrationRecordFamily,
) {
    accumulator.invalidate();
    push_finding(
        findings,
        StateMigrationFindingSeverity::Blocker,
        StateMigrationFindingCode::RecordInvalid,
        Some(family),
    );
}

fn malformed_address(
    accumulator: &mut FamilyAccumulator,
    findings: &mut Vec<StateMigrationCompatibilityFinding>,
    family: StateMigrationRecordFamily,
) {
    accumulator.invalidate();
    push_finding(
        findings,
        StateMigrationFindingSeverity::Blocker,
        StateMigrationFindingCode::MalformedStorageAddress,
        Some(family),
    );
}

fn duplicate_identity(
    accumulator: &mut FamilyAccumulator,
    findings: &mut Vec<StateMigrationCompatibilityFinding>,
    family: StateMigrationRecordFamily,
) {
    accumulator.invalidate();
    push_finding(
        findings,
        StateMigrationFindingSeverity::Blocker,
        StateMigrationFindingCode::DuplicateIdentity,
        Some(family),
    );
}

fn push_finding(
    findings: &mut Vec<StateMigrationCompatibilityFinding>,
    severity: StateMigrationFindingSeverity,
    code: StateMigrationFindingCode,
    family: Option<StateMigrationRecordFamily>,
) {
    findings.push(StateMigrationCompatibilityFinding::new(
        severity, code, family,
    ));
}

fn file_name(path: &Path) -> Option<String> {
    path.file_name()?.to_str().map(str::to_owned)
}

fn is_hash(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn hash_json_value(hasher: &mut Sha256, value: &serde_json::Value) {
    match value {
        serde_json::Value::Null => hasher.update([0]),
        serde_json::Value::Bool(value) => {
            hasher.update([1]);
            hasher.update([u8::from(*value)]);
        }
        serde_json::Value::Number(value) => {
            hasher.update([2]);
            hash_bytes(hasher, b"number", value.to_string().as_bytes());
        }
        serde_json::Value::String(value) => {
            hasher.update([3]);
            hash_bytes(hasher, b"string", value.as_bytes());
        }
        serde_json::Value::Array(values) => {
            hasher.update([4]);
            hasher.update(values.len().to_le_bytes());
            for value in values {
                hash_json_value(hasher, value);
            }
        }
        serde_json::Value::Object(values) => {
            hasher.update([5]);
            hasher.update(values.len().to_le_bytes());
            let mut keys = values.keys().collect::<Vec<_>>();
            keys.sort_unstable();
            for key in keys {
                hash_bytes(hasher, b"key", key.as_bytes());
                hash_json_value(hasher, &values[key]);
            }
        }
    }
}

fn hash_bytes(hasher: &mut Sha256, label: &[u8], value: &[u8]) {
    hasher.update(label.len().to_le_bytes());
    hasher.update(label);
    hasher.update(value.len().to_le_bytes());
    hasher.update(value);
}
