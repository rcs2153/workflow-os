# SideEffect Discovery Helper Report

## 1. Executive Summary

The first SideEffect discovery slice is implemented as an explicit in-memory helper.

Workflow OS can now discover stable SideEffect IDs from caller-supplied IDs, accepted in-memory workflow event inputs, and already-loaded validated SideEffect records. The helper validates immutable run identity, deduplicates references deterministically, reports unsupported attempted/completed/failed SideEffect event vocabulary as bounded status, and fails closed when required records are missing.

This is not automatic discovery. It is not wired into `LocalExecutor`, WorkReport generation, EvidenceReference attachment, `SideEffectRecordStore`, CLI, schemas, examples, provider mutation, or write-capable adapters.

## 2. Scope Completed

- Added `SideEffectDiscoveryInput`.
- Added `SideEffectDiscoverySource`.
- Added `SideEffectDiscoveryReference`.
- Added `SideEffectDiscoveryResult`.
- Added `discover_side_effect_references`.
- Exported the helper API from `workflow-core`.
- Discovered explicit caller-supplied SideEffect IDs.
- Discovered SideEffect IDs from proposed, denied, and skipped workflow events.
- Discovered SideEffect IDs from supplied validated `SideEffectRecord` values.
- Deduplicated discovered IDs deterministically.
- Preserved source priority for duplicate IDs.
- Validated full immutable run identity for supplied workflow events and records.
- Supported optional versus required record presence.
- Kept attempted/completed/failed events unsupported for this first discovery slice.
- Added focused tests for discovery, identity mismatch, missing records, unsupported event vocabulary, and Debug non-leakage.

## 3. Scope Explicitly Not Completed

- No automatic SideEffect discovery in executor paths.
- No WorkReport side-effect discovery integration.
- No store-backed discovery wrapper.
- No `StateBackend` dependency.
- No `SideEffectRecordStore` reads during discovery.
- No EvidenceReference side-effect attachment.
- No approval-side-effect linkage enforcement.
- No runtime side-effect execution.
- No `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` executor behavior.
- No write-capable adapters.
- No provider mutation.
- No local filesystem writes as runtime side effects.
- No workflow-declared SideEffect configuration.
- No runtime SideEffect configuration.
- No workflow schema fields.
- No CLI commands or rendering.
- No example updates.
- No hosted or distributed runtime behavior.
- No reasoning lineage.
- No rollback or compensation behavior.
- No Level 3/4 autonomy enablement.
- No release posture changes.

## 4. Helper API Summary

The new helper is:

- `discover_side_effect_references(input: &SideEffectDiscoveryInput) -> Result<SideEffectDiscoveryResult, WorkflowOsError>`

The input carries:

- workflow ID;
- workflow version;
- schema version;
- spec hash;
- run ID;
- explicit SideEffect IDs;
- already-loaded workflow events;
- already-loaded SideEffect records;
- whether records are required for discovered IDs.

The result exposes:

- discovered references;
- source for each reference;
- missing record count;
- unsupported SideEffect event count;
- whether records were required.

The helper does not read hidden global state, use a state backend, append events, mutate snapshots, call adapters, create SideEffect records, create WorkReport citations, or emit CLI output.

## 5. Discovery Behavior

Discovery uses bounded in-memory sources:

1. explicit caller-supplied SideEffect IDs;
2. supported SideEffect workflow events: proposed, denied, skipped;
3. supplied validated SideEffect records.

Duplicate IDs are deduplicated deterministically. The output is ordered by SideEffect ID, and the recorded source preserves first source priority.

Attempted, completed, and failed SideEffect workflow events remain unsupported in this first discovery slice. The helper counts them as unsupported rather than treating them as implemented runtime lifecycle behavior.

## 6. Validation Boundary Summary

The helper validates:

- workflow event identity matches workflow ID, workflow version, schema version, spec hash, and run ID;
- SideEffect record identity matches workflow ID, workflow version, schema version, spec hash, and run ID;
- supplied records still pass `SideEffectRecord::validate`;
- required records are present for all discovered IDs when `require_records` is true.

Errors are stable and non-leaking:

- `side_effect_discovery.identity_mismatch`;
- `side_effect_discovery.record_missing`;
- `side_effect_discovery.record_corrupt`.

The helper does not convert discovery errors into project diagnostics or workflow execution failures.

## 7. Redaction And Privacy Summary

Discovery remains reference-only.

The helper does not store, copy, serialize, or debug-print:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira issue bodies or comments;
- raw GitHub file contents;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded summaries;
- secret-like target identifiers or metadata.

Debug output exposes counts and source/status only. SideEffect IDs, workflow IDs, run IDs, workflow versions, spec hashes, and target references are not printed by the helper's Debug implementations.

## 8. Test Coverage Summary

Added focused tests cover:

- explicit SideEffect IDs returned deterministically;
- duplicate IDs deduplicated deterministically;
- proposed SideEffect events discovered;
- denied SideEffect events discovered;
- skipped SideEffect events discovered;
- attempted/completed/failed events counted as unsupported and not discovered;
- matching records discovered;
- required records satisfying the requirement;
- missing required records fail closed without leaking the SideEffect ID;
- workflow event identity mismatch fails without leaking identity values;
- record identity mismatch fails without leaking identity values;
- Debug output does not leak IDs or immutable run identity values.

## 9. Commands Run And Results

- `cargo fmt --all` - passed.
- `cargo test -p workflow-core --test side_effect_discovery` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 10. Remaining Known Limitations

- Store-backed discovery is not implemented.
- WorkReport generation does not automatically discover SideEffect IDs.
- Executor report-bearing paths are not wired to this helper.
- EvidenceReference side-effect attachment is not implemented.
- Audit projections are not used as discovery sources.
- Adapter telemetry is not used as a discovery source.
- Attempted/completed/failed lifecycle discovery remains unsupported until runtime semantics are separately accepted.
- Write-capable adapters remain deferred.

## 11. Recommended Next Phase

Recommended next phase: SideEffect discovery helper review.

The helper should be reviewed before adding a store-backed discovery wrapper, WorkReport integration, EvidenceReference side-effect attachment, attempted/completed/failed lifecycle behavior, runtime side-effect execution, or write-capable adapters.
