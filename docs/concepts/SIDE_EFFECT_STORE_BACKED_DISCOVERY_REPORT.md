# SideEffect Store-Backed Discovery Report

## 1. Executive Summary

Store-backed SideEffect discovery is implemented as a small explicit helper wrapper.

Workflow OS can now load already-persisted `SideEffectRecord` values through a supplied `SideEffectRecordStore`, combine them with explicit SideEffect IDs and caller-supplied workflow events, and delegate deterministic discovery to the accepted in-memory `discover_side_effect_references` helper.

This is not automatic report discovery. It is not executor integration. It does not create SideEffect records, append workflow events, execute side effects, create EvidenceReference values, persist reports, write artifacts, expose CLI output, add schemas, update examples, or enable write-capable adapters.

## 2. Scope Completed

- Added `SideEffectStoreBackedDiscoveryInput`.
- Added `discover_side_effect_references_from_store`.
- Loaded records through `SideEffectRecordStore::list_side_effect_records_for_workflow_run`.
- Reused the accepted in-memory discovery helper for validation, de-duplication, source priority, and result shaping.
- Preserved full immutable run identity validation through helper-level checks.
- Preserved explicit source priority:
  1. explicit caller-supplied SideEffect IDs;
  2. caller-supplied workflow events;
  3. loaded SideEffect records.
- Mapped store errors into stable non-leaking discovery errors.
- Exported the store-backed helper API from `workflow-core`.
- Added focused tests for happy path, source priority, missing required records, store error mapping, and Debug non-leakage.

## 3. Scope Explicitly Not Completed

- No automatic SideEffect discovery in executor paths.
- No WorkReport side-effect discovery integration.
- No report artifact integration.
- No `LocalExecutor::execute_with_report(...)` changes.
- No EvidenceReference side-effect attachment.
- No approval-side-effect linkage enforcement.
- No runtime side-effect execution.
- No attempted/completed/failed executor behavior.
- No write-capable adapters.
- No provider mutation.
- No SideEffect record creation, repair, update, or lifecycle transition.
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

New API:

- `SideEffectStoreBackedDiscoveryInput`
- `discover_side_effect_references_from_store(store, input)`

The input carries:

- workflow ID;
- workflow version;
- schema version;
- spec hash;
- run ID;
- explicit SideEffect IDs;
- caller-supplied workflow events;
- required-record policy.

The helper:

1. loads records through `SideEffectRecordStore::list_side_effect_records_for_workflow_run`;
2. builds `SideEffectDiscoveryInput`;
3. calls `discover_side_effect_references`;
4. returns `SideEffectDiscoveryResult`.

The helper depends on the `SideEffectRecordStore` trait only. It does not construct a local backend, infer hidden state, or require `LocalStateBackend`.

## 5. Store Loading Behavior

The wrapper intentionally uses run-level listing for the first slice:

- `list_side_effect_records_for_workflow_run(&workflow_id, &run_id)`

This keeps the implementation deterministic and small. The store-level listing checks workflow ID and run ID, and the delegated in-memory helper validates workflow version, schema version, spec hash, and run ID for every loaded record before returning references.

If the store returns no records:

- optional discovery can still return explicit/event references with a missing-record count;
- required-record discovery fails closed for discovered references without matching records;
- discovery returns no references when no explicit IDs, events, or records exist.

## 6. Validation Boundary Summary

The wrapper preserves two validation layers:

1. Store-level workflow/run listing through `SideEffectRecordStore`.
2. Helper-level full immutable identity validation through `discover_side_effect_references`.

Store errors are mapped to non-leaking discovery errors:

- `side_effect_record.read.identity_mismatch` -> `side_effect_discovery.identity_mismatch`;
- `side_effect_record.read.corrupt` -> `side_effect_discovery.record_corrupt`;
- other store read failures -> `side_effect_discovery.store_read_failed`.

Missing required records continue to fail as:

- `side_effect_discovery.record_missing`.

## 7. Redaction And Privacy Summary

Discovery remains reference-only.

The store-backed helper does not store, copy, serialize, debug-print, or include in errors:

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
- raw record JSON;
- unbounded summaries;
- secret-like target identifiers or metadata;
- local filesystem paths.

`SideEffectStoreBackedDiscoveryInput` Debug output redacts IDs and immutable identity values and reports counts only.

## 8. Runtime Boundary Summary

The helper does not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events;
- create or update SideEffect records;
- repair corrupt records;
- emit audit events;
- emit observability events;
- execute side effects;
- call adapters or providers;
- write report artifacts;
- create CLI output;
- change workflow pass/fail behavior.

The only side-effect-adjacent action is reading through the caller-supplied store trait.

## 9. Test Coverage Summary

Added focused tests cover:

- store-backed discovery returns persisted records for the requested run;
- explicit IDs and store records are merged deterministically;
- workflow event IDs and store records are merged deterministically;
- explicit IDs preserve source priority over record sources;
- workflow events preserve source priority over record sources;
- optional missing records return bounded missing count;
- required missing records fail closed without leaking IDs;
- store identity errors map to non-leaking discovery errors;
- store corrupt errors map to non-leaking discovery errors;
- generic store read failures map to non-leaking discovery errors;
- store-backed input Debug output does not leak IDs or identity values;
- existing in-memory discovery tests still pass.

## 10. Commands Run And Results

- `cargo fmt --all` - passed.
- `cargo test -p workflow-core --test side_effect_discovery` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 11. Remaining Known Limitations

- WorkReport generation does not automatically use store-backed SideEffect discovery.
- Executor report-bearing paths do not call this helper.
- EvidenceReference side-effect attachment is not implemented.
- Report artifact referential integrity checks do not validate SideEffect citations against the store.
- Audit projections are not used as discovery sources.
- Adapter telemetry is not used as a discovery source.
- Attempted/completed/failed lifecycle discovery remains unsupported until runtime semantics are separately accepted.
- Write-capable adapters remain deferred.

## 12. Recommended Next Phase

Recommended next phase: SideEffect store-backed discovery review.

The helper should be reviewed before adding automatic WorkReport discovery, executor integration, EvidenceReference side-effect attachment, attempted/completed/failed lifecycle behavior, runtime side-effect execution, or write-capable adapters.
