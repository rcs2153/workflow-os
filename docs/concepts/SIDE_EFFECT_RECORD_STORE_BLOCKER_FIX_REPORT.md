# SideEffect Record Store Blocker Fix Report

## 1. Executive Summary

The SideEffect record store immutable identity blocker is fixed.

The blocker review found that `SideEffectRecordStore` rejected same-run records with different `workflow_id`, but did not reject same-run records with different `workflow_version`, `schema_version`, or `spec_hash`. Workflow OS run identity is immutable across all of those fields, so the store could not safely proceed to discovery or later runtime side-effect phases with that gap.

This fix enforces full immutable run identity for SideEffect records sharing a run ID.

## 2. Blocker Fixed

Fixed blocker:

- same-run SideEffect records now require matching `workflow_id`, `workflow_version`, `schema_version`, and `spec_hash`;
- local store writes reject records whose full immutable run identity conflicts with existing records for the run;
- in-memory test store writes use the same identity rule;
- list-by-workflow/run fails closed if a run bucket contains records with inconsistent immutable identity;
- identity mismatch errors remain stable and non-leaking.

## 3. Implementation Approach

Added a small shared helper in `state.rs`:

- `same_side_effect_run_identity(left, right)`

The helper compares:

- run ID;
- workflow ID;
- workflow version;
- schema version;
- spec content hash.

Both `LocalStateBackend` and the in-memory test backend now use this helper for write-time identity checks. The list-by-workflow/run path also uses it defensively to reject corrupt or manually introduced mixed-identity run buckets.

## 4. Scope Completed

- Enforced full immutable run identity during SideEffect record writes.
- Added regression tests for workflow version mismatch.
- Added regression tests for schema version mismatch.
- Added regression tests for spec hash mismatch.
- Added a local corruption-style test proving list-by-workflow/run rejects mixed full identity without leaking values.
- Preserved existing duplicate ID behavior, deterministic listing, read-by-ID behavior, and runtime non-mutation behavior.

## 5. Scope Explicitly Not Completed

- No automatic SideEffect discovery.
- No report discovery from workflow events, audit projections, or the SideEffect store.
- No EvidenceReference side-effect attachment.
- No approval-side-effect linkage implementation.
- No runtime side-effect execution.
- No attempted/completed/failed executor behavior.
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
- No release posture changes.

## 6. Validation Boundary Summary

The store now treats immutable run identity consistently with the broader Workflow OS runtime:

- `WorkflowRunEvent` identity remains immutable;
- SideEffect records sharing a run ID must use the same workflow ID, workflow version, schema version, and spec hash;
- mismatched values fail closed with `side_effect_record.write.identity_mismatch` or `side_effect_record.read.identity_mismatch`;
- error messages do not include raw identity values, run IDs, SideEffect IDs, spec hashes, target references, provider payloads, command output, or secret-like values.

## 7. Redaction And Privacy Summary

The fix does not add new stored payload fields.

The new tests verify mismatch errors do not leak:

- mismatched workflow version text;
- mismatched schema version text;
- mismatched spec hash text;
- run IDs.

Existing SideEffect record validation continues to reject raw provider payload markers, raw command output markers, secret-like target references, secret-like summaries, secret-like reasons, and secret-like redaction metadata.

## 8. Test Coverage Summary

Added or expanded tests cover:

- write rejection for mismatched workflow version;
- write rejection for mismatched schema version;
- write rejection for mismatched spec hash;
- local list-by-workflow/run rejection for a manually corrupted mixed-identity run bucket;
- non-leakage for mismatch error strings.

Existing state backend, SideEffect model, WorkReport, EvidenceReference, Diagnostic, validation, adapter telemetry, runtime, and executor tests are expected to continue passing.

## 9. Commands Run And Results

- `cargo test -p workflow-core state::tests::` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 10. Remaining Known Limitations

- Health checks do not yet validate SideEffect record/index consistency.
- Automatic SideEffect discovery is not implemented.
- Work reports do not automatically discover SideEffect records.
- EvidenceReference side-effect attachment is not implemented.
- Runtime side-effect execution remains deferred.
- Attempted/completed/failed lifecycle execution behavior remains deferred.
- Write-capable adapters remain deferred.

## 11. Recommended Next Phase

Recommended next phase: **SideEffect record store blocker fix review**.

The blocker fix should be reviewed before Workflow OS moves on to SideEffect discovery, report discovery from SideEffect records, EvidenceReference side-effect attachment, runtime side-effect execution, or write-capable adapters.
