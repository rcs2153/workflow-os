# SideEffect Record Store Report

Fix-forward note: the immutable run identity blocker found in [SideEffect Record Store Review](SIDE_EFFECT_RECORD_STORE_REVIEW.md) is addressed in [SideEffect Record Store Blocker Fix Report](SIDE_EFFECT_RECORD_STORE_BLOCKER_FIX_REPORT.md).

## 1. Executive Summary

The first SideEffect record persistence slice is implemented.

Workflow OS now has an explicit `SideEffectRecordStore` contract and a local filesystem implementation on `LocalStateBackend` for validated `SideEffectRecord` values. The store can write a record, read a record by `SideEffectId`, list records by run ID, and list records by workflow/run identity.

This remains a bounded persistence model for explicit records only. It does not discover side effects automatically, execute side effects, mutate providers, enable writes, attach EvidenceReference values, add schemas, add CLI behavior, update examples, or change release posture.

## 2. Scope Completed

- Added a `SideEffectRecordStore` trait for explicit validated SideEffect record persistence.
- Implemented `SideEffectRecordStore` for `LocalStateBackend`.
- Added local filesystem storage under `side_effects/records` and `side_effects/ids`.
- Added deterministic read and list behavior for `SideEffectRecord` values.
- Added duplicate SideEffect ID rejection.
- Added workflow/run identity mismatch rejection.
- Added redaction-safe, stable error behavior for corrupt stored records.
- Added focused state-backend contract tests for in-memory and local implementations.
- Added local persistence tests for corrupt record handling and non-mutation of runtime workflow state.
- Updated planning, concept, roadmap, and runtime state-backend docs to describe the implemented boundary.

## 3. Scope Explicitly Not Completed

- No automatic SideEffect discovery.
- No report discovery from workflow events, audit projections, or the SideEffect store.
- No EvidenceReference side-effect attachment.
- No approval-side-effect linkage implementation.
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

## 4. API Summary

The new store contract is:

- `SideEffectRecordStore::write_side_effect_record(&self, record: &SideEffectRecord)`
- `SideEffectRecordStore::read_side_effect_record(&self, side_effect_id: &SideEffectId)`
- `SideEffectRecordStore::list_side_effect_records(&self, run_id: &WorkflowRunId)`
- `SideEffectRecordStore::list_side_effect_records_for_workflow_run(&self, workflow_id: &WorkflowId, run_id: &WorkflowRunId)`

The trait is exported from `workflow-core`. It is deliberately separate from the aggregate `StateBackend` trait so normal runtime execution, run snapshot persistence, audit projection, report generation, and artifact storage do not implicitly gain SideEffect discovery or write semantics.

`SideEffectRecord` also exposes bounded identity accessors needed by the store:

- workflow ID;
- workflow version;
- schema version;
- spec hash;
- run ID;
- created timestamp.

## 5. Local Persistence Layout

`LocalStateBackend` now creates:

- `side_effects/records/<run-id-hash>/<side-effect-id-hash>.json`
- `side_effects/ids/<side-effect-id-hash>.json`

The ID index maps a SideEffect ID to its run ID so a record can be read without scanning all runs. The record path remains run-scoped for deterministic list behavior.

The stored payload is the validated `SideEffectRecord` model. The implementation does not store raw provider payloads, raw command output, raw CI logs, raw Jira or GitHub bodies, raw spec contents, parser payloads, environment values, credentials, authorization headers, private keys, or token-like values.

## 6. Source-Of-Truth And Runtime Semantics Summary

The store is a persistence boundary for explicit `SideEffectRecord` values.

Workflow events remain the source of truth for workflow run history. Audit records remain projections. Work reports remain governed handoff artifacts. The SideEffect record store is the future source-of-truth candidate for durable SideEffect intent, authority, lifecycle, idempotency, target, outcome, and references once execution semantics are separately implemented.

This phase does not make the executor discover SideEffect records, read SideEffect records while executing, append SideEffect events from stored records, or generate reports from stored records.

## 7. Validation And Identity Boundary Summary

Writes validate the `SideEffectRecord` before persistence.

The local store rejects:

- duplicate SideEffect IDs;
- records whose existing run bucket is already associated with a different workflow ID;
- corrupt records during read or list;
- workflow/run list requests where a stored record belongs to a different workflow.

Errors use stable codes such as:

- `side_effect_record.write.duplicate`
- `side_effect_record.write.identity_mismatch`
- `side_effect_record.write.failed`
- `side_effect_record.read.corrupt`
- `side_effect_record.read.identity_mismatch`

Error messages do not include raw stored payloads, SideEffect target references, provider output, command output, parser output, credentials, token-like values, or secret-like strings.

## 8. Redaction And Privacy Summary

The implementation relies on existing `SideEffectRecord` validation and redaction behavior and does not add any new raw payload fields.

Debug behavior for the model remains redaction-safe. Corrupt stored records fail closed through stable errors. The store does not expose file paths, snippets, raw JSON payloads, or secret-like stored values in public validation errors.

## 9. Test Coverage Summary

Added tests cover:

- write, read, and deterministic list behavior through the shared store contract;
- duplicate SideEffect ID rejection;
- workflow/run identity mismatch rejection;
- in-memory store contract behavior;
- local filesystem store contract behavior;
- corrupt local record read failure without leaking the corrupt payload;
- no mutation of persisted workflow run snapshots when writing SideEffect records.

Existing SideEffect model, WorkReport, EvidenceReference, Diagnostic, validation, adapter telemetry, runtime, and executor tests are expected to continue passing.

## 10. Commands Run And Results

- `cargo test -p workflow-core state::tests` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 11. Remaining Known Limitations

- SideEffect discovery is not implemented.
- Work reports do not automatically discover SideEffect records.
- EvidenceReference side-effect attachment is not implemented.
- The executor does not read from the SideEffect record store.
- Attempted/completed/failed runtime SideEffect lifecycle behavior remains deferred.
- Write-capable adapters remain deferred.
- There is no schema, CLI, example, hosted runtime, or release posture change.

## 12. Recommended Next Phase

Recommended next phase: **SideEffect record store review**.

The store should be reviewed before any implementation of automatic SideEffect discovery, report discovery from SideEffect records, EvidenceReference side-effect attachment, runtime side-effect execution, or write-capable adapters.
