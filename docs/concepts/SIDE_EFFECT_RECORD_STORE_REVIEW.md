# SideEffect Record Store Review

Fix-forward note: the blocker identified in this review is addressed in [SideEffect Record Store Blocker Fix Report](SIDE_EFFECT_RECORD_STORE_BLOCKER_FIX_REPORT.md). This review keeps the original blocker finding intact for phase history.

## 1. Executive Verdict

Needs blocker fixes.

The phase stays within the approved persistence-only boundary and adds a useful `SideEffectRecordStore` plus local filesystem persistence for explicit validated `SideEffectRecord` values. It does not introduce runtime side-effect execution, writes, automatic discovery, CLI behavior, schema changes, examples, hosted behavior, reasoning lineage, or release posture changes.

However, the local store's identity mismatch protection is incomplete. It rejects same-run records with different `workflow_id`, but it does not reject same-run records with the same workflow ID and different `workflow_version`, `schema_version`, or `spec_hash`. Workflow OS run identity is the tuple of run ID, workflow ID, workflow version, schema version, and spec content hash. A SideEffect record store must not allow records for one run bucket to mix immutable run identity values.

## 2. Scope Verification

The phase stayed within the approved explicit record-store scope.

Implemented:

- `SideEffectRecordStore` trait;
- `LocalStateBackend` implementation for explicit SideEffect record persistence;
- local filesystem layout under `side_effects/records` and `side_effects/ids`;
- write, read, list-by-run, and list-by-workflow/run behavior;
- duplicate `SideEffectId` rejection;
- partial workflow/run identity mismatch rejection;
- corrupt local record read failure with stable non-leaking errors;
- state-backend contract tests for in-memory and local store implementations;
- documentation and end-of-phase report.

No accidental implementation found for:

- automatic SideEffect discovery;
- report discovery from workflow events, audit projections, or the SideEffect store;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage;
- runtime side-effect execution;
- `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` executor behavior;
- write-capable adapters;
- provider mutation;
- local filesystem writes as runtime side effects;
- workflow-declared SideEffect configuration;
- runtime SideEffect configuration;
- workflow schema fields;
- CLI commands or rendering;
- example updates;
- hosted/distributed runtime behavior;
- reasoning lineage;
- rollback or compensation behavior;
- Level 3/4 autonomy enablement;
- release posture changes.

## 3. Store API Assessment

The `SideEffectRecordStore` API is appropriately small for the first persistence slice:

- `write_side_effect_record`;
- `read_side_effect_record`;
- `list_side_effect_records`;
- `list_side_effect_records_for_workflow_run`.

Keeping the trait separate from aggregate `StateBackend` is the right first-phase boundary. Normal workflow execution, run rehydration, report generation, audit projection, and artifact storage do not implicitly gain SideEffect discovery or write semantics.

The API is also local-development friendly: it does not require provider clients, live adapters, runtime config, workflow schema fields, report generation, or CLI behavior.

One API limitation should be addressed with the blocker fix or soon after: `list_side_effect_records_for_workflow_run` accepts only `WorkflowId` plus `WorkflowRunId`. If this method is meant to validate immutable run identity rather than just filter by workflow ID, it should eventually accept or compare `workflow_version`, `schema_version`, and `spec_hash` as well.

## 4. Local Persistence Layout Assessment

The local filesystem layout is reasonable and bounded:

- `side_effects/records/<run-id-hash>/<side-effect-id-hash>.json`
- `side_effects/ids/<side-effect-id-hash>.json`

The ID index avoids scanning all run directories for reads by `SideEffectId`, while run-scoped record directories support deterministic listing.

The implementation writes the ID index before the record and removes the index if record writing fails. That is acceptable for this local-first slice. A crash between index write and record write would leave a dangling index that `read_side_effect_record` reports as corrupt. That behavior is fail-closed and acceptable, but health-check coverage for SideEffect index/record consistency should be added before discovery depends on this store.

## 5. Source-Of-Truth And Runtime Semantics Assessment

The source-of-truth boundaries are preserved.

Verified:

- workflow events remain the source of truth for workflow run history;
- run snapshots remain projections;
- audit records remain projections;
- work reports remain governed handoff artifacts;
- `SideEffectRecord` persistence does not append workflow events;
- `SideEffectRecord` persistence does not mutate snapshots;
- the executor does not read SideEffect records during normal execution;
- report generation does not discover SideEffect records automatically;
- provider mutation and runtime side-effect execution remain unimplemented.

This is the right posture. The store creates a durable place for explicit SideEffect records without using persistence as a back door into runtime behavior.

## 6. Validation And Identity Assessment

Validation is partly correct, but the immutable run identity boundary has a blocker.

Verified:

- writes call `SideEffectRecord::validate`;
- duplicate `SideEffectId` values are rejected;
- corrupt stored record JSON fails closed on read/list;
- read-by-ID verifies the stored record's `side_effect_id` and `run_id` match the ID index;
- list-by-run verifies each record's `run_id`;
- list-by-workflow/run rejects records whose `workflow_id` differs from the requested workflow ID;
- errors use stable codes and do not include raw SideEffect IDs, run IDs, target references, raw payloads, or secret-like test strings.

Blocker:

- same-run records are only compared by `workflow_id` during write, and only by `workflow_id` during `list_side_effect_records_for_workflow_run`.
- The store does not reject records for the same `run_id` that have mismatched `workflow_version`, `schema_version`, or `spec_hash`.
- This weakens the Workflow OS invariant that a run is bound to immutable workflow identity and spec content hash.

The blocker fix should compare full immutable run identity for records sharing a `run_id`: `workflow_id`, `workflow_version`, `schema_version`, and `spec_hash`.

## 7. Privacy And Redaction Assessment

The privacy posture is sound for this phase.

Verified:

- the store persists validated `SideEffectRecord` values, not raw provider payloads;
- raw command output is not introduced;
- raw CI logs are not introduced;
- raw Jira issue bodies/comments are not introduced;
- raw GitHub file contents are not introduced;
- raw spec contents are not introduced;
- parser payloads are not introduced;
- environment variable values are not introduced;
- credentials, authorization headers, private keys, and token-like values are not introduced;
- corrupt record read failures do not leak the injected secret-like payload marker;
- duplicate and identity mismatch errors avoid raw IDs and run IDs.

Existing `SideEffectRecord` tests cover redaction-safe Debug behavior, serialization posture, secret-like target reference rejection, secret-like summary/reason/redaction rejection, serde round trips, and invalid serialized record failure without leaking secret-like values.

## 8. Idempotency And Duplicate Handling Assessment

Duplicate SideEffect ID handling is appropriate for this slice. The store rejects duplicate IDs before writing another record and maps filesystem create-new conflicts to the same stable duplicate error code.

The store does not yet integrate with the runtime idempotency store, but that is acceptable because no runtime side-effect execution exists. Future attempted/completed/failed lifecycle phases must define how SideEffect idempotency keys bind to persisted records, adapter outcomes, provider operation references, and retry/replay behavior.

## 9. Test Quality Assessment

Strong coverage added:

- write/read/list through the shared store contract;
- deterministic list order by `SideEffectId`;
- duplicate SideEffect ID rejection;
- workflow ID mismatch rejection;
- in-memory store contract behavior;
- local filesystem store contract behavior;
- corrupt local record read failure without leaking secret-like payload content;
- no mutation of persisted workflow events or snapshots when writing SideEffect records;
- full workspace regression coverage.

Missing blocker coverage:

- no test rejects same-run records with mismatched `workflow_version`;
- no test rejects same-run records with mismatched `schema_version`;
- no test rejects same-run records with mismatched `spec_hash`;
- no test verifies list-by-workflow/run fails on full immutable identity mismatch.

Non-blocking test gaps:

- no direct test for missing ID index returning `None`;
- no direct health-check test for dangling SideEffect ID index or orphan record files;
- no list/read test for an ID index pointing to a record whose SideEffect ID differs from the requested ID beyond the corruption path;
- no direct test that normal executor/report paths create no `side_effects` records, though runtime non-mutation tests cover event/snapshot behavior.

## 10. Documentation Review

Docs are mostly accurate and appropriately conservative.

Verified docs state:

- `SideEffectRecordStore` and explicit local SideEffect record persistence are implemented;
- automatic SideEffect discovery is not implemented;
- report discovery from workflow events, audit projections, or the SideEffect store is not implemented;
- EvidenceReference side-effect attachment is not implemented;
- approval-side-effect linkage is not implemented;
- runtime side-effect execution is not implemented;
- attempted/completed/failed executor behavior is not implemented;
- write-capable adapters are not implemented;
- provider mutation is not implemented;
- schema changes are not implemented;
- CLI commands/rendering are not implemented;
- examples are not updated;
- hosted/distributed runtime behavior is not implemented;
- reasoning lineage remains unimplemented;
- rollback/compensation is not implemented;
- release posture is unchanged.

One documentation nuance: earlier executor append phase reports now include fix-forward wording that SideEffect persistence did not exist in that phase, but was later added by this store phase. That preserves historical phase reporting without leaving dangerous current-state false claims.

## 11. Blockers

1. Full immutable run identity must be enforced for records sharing a `run_id`.

   Current behavior rejects same-run records with different `workflow_id`, but does not reject different `workflow_version`, `schema_version`, or `spec_hash`.

   Required fix:

   - compare `workflow_id`, `workflow_version`, `schema_version`, and `spec_hash` when writing into an existing run bucket;
   - add tests for mismatched workflow version, schema version, and spec hash;
   - ensure errors remain stable and non-leaking;
   - consider whether `list_side_effect_records_for_workflow_run` should stay workflow-ID-only or gain a full-identity variant before discovery uses it.

## 12. Non-Blocking Follow-Ups

- Add SideEffect record/index consistency checks to local backend health checks before store-backed discovery is implemented.
- Add tests for dangling SideEffect ID index and orphan SideEffect record files.
- Add a direct test that missing SideEffect IDs return `None` without side effects.
- Document whether `list_side_effect_records_for_workflow_run` is a workflow-ID filter or an immutable identity guard.
- Before attempted/completed/failed lifecycle phases, define SideEffect idempotency replay semantics.

## 13. Recommended Next Phase

Recommended next phase: **SideEffect record store blocker fix**.

The store should enforce full immutable run identity before Workflow OS proceeds to SideEffect discovery, WorkReport discovery from SideEffect records, EvidenceReference side-effect attachment, runtime side-effect execution, or write-capable adapters.

After the blocker fix and review, the next roadmap phase can return to conservative SideEffect discovery planning/implementation.

## 14. Validation

Validation commands run for this review:

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
