# SideEffect Record Store Blocker Fix Review

Review date: 2026-06-19

## 1. Executive Verdict

Blocker fixed; proceed to SideEffect discovery planning.

The SideEffect record store immutable run identity blocker is fixed. The implementation now compares full immutable run identity for SideEffect records sharing a run ID: workflow ID, workflow version, schema version, and spec content hash. Both the local filesystem store and the in-memory test backend reject same-run records whose immutable identity conflicts with existing records, and the local list-by-workflow/run path fails closed if a mixed-identity run bucket is detected.

The fix remains inside the approved blocker-fix boundary. It does not add automatic SideEffect discovery, report discovery, EvidenceReference side-effect attachment, approval-side-effect linkage, runtime side-effect execution, attempted/completed/failed executor behavior, write-capable adapters, provider mutation, local filesystem writes as runtime side effects, workflow schema fields, CLI behavior, examples, hosted behavior, reasoning lineage, rollback/compensation behavior, Level 3/4 autonomy, or release posture changes.

## 2. Scope Verification

The fix stayed within the approved blocker-fix scope.

Implemented:

- shared full immutable run identity comparison for SideEffect records;
- local store write-time rejection for same-run identity conflicts;
- in-memory test backend write-time rejection for same-run identity conflicts;
- defensive list-by-workflow/run rejection for mixed-identity run buckets;
- stable, non-leaking identity mismatch errors;
- focused regression tests for workflow version, schema version, and spec hash mismatches;
- a local corruption-style regression test for mixed full identity during listing;
- fix-forward documentation and a blocker fix report.

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
- hosted or distributed runtime behavior;
- reasoning lineage;
- rollback or compensation behavior;
- Level 3/4 autonomy enablement;
- release posture changes.

## 3. Original Blocker Restatement

The original blocker was concrete:

- `SideEffectRecordStore` rejected same-run records whose workflow ID differed from existing records.
- It did not reject same-run records whose workflow ID matched but whose workflow version, schema version, or spec content hash differed.
- Workflow OS run identity is immutable across run ID, workflow ID, workflow version, schema version, and spec content hash.
- Allowing mixed immutable identity values in a SideEffect run bucket would weaken future discovery, report citation, replay, audit, and side-effect governance semantics.

## 4. Fix Approach Assessment

The selected approach is minimal and idiomatic.

The implementation adds `same_side_effect_run_identity(left, right)` in the state backend module and applies it at the two relevant boundaries:

- write-time conflict detection for records sharing a run ID;
- defensive list-by-workflow/run validation for records already present in a run bucket.

This keeps the fix local to the record store boundary, avoids widening the public API during a blocker fix, and preserves the existing persistence shape. It is compatible with future discovery because discovery can now assume that valid store writes cannot mix immutable run identity within a run bucket.

## 5. Validation Boundary Assessment

Verified validation behavior:

- writes still call `SideEffectRecord::validate`;
- duplicate SideEffect IDs are still rejected;
- same-run workflow ID mismatch is rejected;
- same-run workflow version mismatch is rejected;
- same-run schema version mismatch is rejected;
- same-run spec hash mismatch is rejected;
- local list-by-workflow/run rejects mixed full immutable identity if a corrupt bucket is introduced outside the store write API;
- validation errors use stable codes;
- validation errors do not leak raw workflow versions, schema versions, spec hashes, run IDs, SideEffect IDs, target references, provider payloads, command output, parser payloads, or secret-like strings.

The relevant stable codes remain:

- `side_effect_record.write.identity_mismatch`;
- `side_effect_record.read.identity_mismatch`.

## 6. Store Behavior Assessment

The local filesystem store now checks existing records in the run bucket before writing a new record. If any existing record shares the run ID but fails full immutable identity comparison, the write fails before the new ID index or record file is created.

The in-memory test backend uses the same identity helper, which keeps the shared state-backend contract tests meaningful across implementations.

The list-by-workflow/run method still accepts workflow ID plus run ID rather than the full immutable identity tuple. That API shape is acceptable for this blocker fix because the method now defensively rejects mixed identity among returned records. A fuller query API can be considered before automatic discovery depends on this store.

## 7. Privacy And Redaction Assessment

The fix preserves the privacy posture of the SideEffect store.

Verified no storage or introduction of:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira issue bodies/comments;
- raw GitHub file contents;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

Mismatch errors remain intentionally generic. Tests verify that mismatch details such as workflow version text, schema version text, spec hash text, and run IDs are not emitted in public error strings.

## 8. Regression Assessment

Existing behavior remains intact for:

- valid SideEffect record writes;
- read-by-SideEffect-ID behavior;
- deterministic list-by-run behavior;
- duplicate SideEffect ID rejection;
- corrupt local record failure behavior;
- no mutation of workflow events, snapshots, audit records, reports, providers, adapters, or runtime state;
- SideEffect model behavior;
- WorkReport behavior;
- EvidenceReference behavior;
- Diagnostic and validation behavior;
- adapter telemetry behavior;
- runtime and executor tests.

The full workspace validation suite passes after the fix.

## 9. Test Quality Assessment

Tests cover the original blocker directly:

- workflow identity mismatch rejection still passes;
- workflow version mismatch rejection is covered;
- schema version mismatch rejection is covered;
- spec hash mismatch rejection is covered;
- both the in-memory backend contract and local backend contract run the mismatch tests;
- local list-by-workflow/run rejects a manually introduced mixed-identity run bucket;
- mismatch errors do not leak tested identity values.

Non-blocking test hardening opportunities:

- add a health-check test for SideEffect ID index and record-file consistency;
- add explicit tests for dangling SideEffect ID index and orphan SideEffect record files;
- add a future full-identity query test if the list API later accepts workflow version, schema version, and spec hash.

These are not blockers because the original immutable identity gap is closed at write time and defensively checked during list-by-workflow/run.

## 10. Documentation Review

Documentation is aligned with the fix.

Verified docs state:

- the SideEffect record store immutable identity blocker is fixed;
- the store persists explicit validated `SideEffectRecord` values only;
- automatic SideEffect discovery is not implemented;
- report discovery from workflow events, audit projections, or the SideEffect store is not implemented;
- EvidenceReference side-effect attachment is not implemented;
- approval-side-effect linkage is not implemented;
- runtime side-effect execution is not implemented;
- attempted/completed/failed executor behavior is not implemented;
- write-capable adapters remain deferred;
- workflow schema fields are not implemented;
- CLI commands and rendering are not implemented;
- examples are not updated;
- hosted/distributed runtime behavior is not implemented;
- reasoning lineage remains separate;
- release posture is unchanged.

Historical docs preserve the original blocker finding and link forward to the blocker fix report rather than erasing phase history.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add SideEffect record/index consistency checks to `LocalStateBackend::health_check` before automatic discovery depends on the store.
- Consider a future full-identity list/query API that accepts workflow version, schema version, and spec hash in addition to workflow ID and run ID.
- Add tests for dangling SideEffect ID indexes and orphan SideEffect record files.
- Keep automatic SideEffect discovery behind a separately scoped plan and review.

## 13. Recommended Next Phase

Recommended next phase: SideEffect discovery planning.

The blocker is fixed, and the store now preserves immutable run identity well enough to plan discovery. The next phase should remain planning-first and should define how future WorkReports, evidence references, or governance views may discover SideEffect records without adding runtime side-effect execution, write-capable adapters, provider mutation, schema changes, CLI behavior, examples, hosted behavior, reasoning lineage, rollback/compensation behavior, Level 3/4 autonomy, or release posture changes.

## Validation

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed |
| `npm run check:docs` | Passed |
| `git diff --check` | Passed |
