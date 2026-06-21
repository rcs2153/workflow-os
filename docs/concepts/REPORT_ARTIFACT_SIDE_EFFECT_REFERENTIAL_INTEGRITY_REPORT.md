# Report Artifact SideEffect Referential Integrity Report

## 1. Executive Summary

The report artifact SideEffect referential integrity helper is implemented as an explicit, validation-only boundary.

The helper validates `SideEffect` citations already present in a `WorkReportArtifactRecord` against a caller-supplied `SideEffectRecordStore`. It is in-memory, reference-only, non-mutating, and separate from normal report artifact writes.

This phase does not implement automatic artifact writing, automatic artifact integrity validation during writes, automatic SideEffect discovery, executor integration, runtime side-effect execution, provider mutation, write-capable adapters, CLI rendering, schemas, examples, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope Completed

- Added `WorkReportArtifactSideEffectIntegrityInput`.
- Added `WorkReportArtifactSideEffectIntegrityResult`.
- Added `validate_work_report_artifact_side_effect_integrity(...)`.
- Exported the helper and types from `workflow-core`.
- Validates the artifact before checking citations.
- Extracts `SideEffect` citations from report sections and disclosure/limitation/risk/handoff note containers.
- De-duplicates cited `SideEffectId` values deterministically.
- Reads cited records only through an explicit `SideEffectRecordStore`.
- Supports strict mode where every cited `SideEffectId` must resolve.
- Supports permissive mode where missing records are counted but identity mismatches and corrupt records still fail closed.
- Validates immutable run identity:
  - workflow ID;
  - workflow version;
  - schema version;
  - spec hash;
  - run ID.
- Returns bounded counts only.
- Maps integrity failures to stable, non-leaking error codes.
- Adds focused regression tests.
- Updates roadmap and concept documentation.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- automatic report artifact writing;
- automatic integrity validation inside `WorkReportArtifactStore::write_work_report_artifact(...)`;
- combined artifact write plus integrity helper;
- automatic SideEffect discovery;
- executor integration;
- workflow event appends;
- audit event emission;
- workflow state mutation;
- report artifact signing or notarization;
- CLI rendering or export;
- workflow spec schema fields;
- examples;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage enforcement;
- runtime side-effect execution;
- attempted/completed/failed executor behavior;
- write-capable adapters;
- provider mutation;
- hosted or distributed runtime behavior;
- reasoning lineage;
- DLP or access-control systems;
- release posture changes.

## 4. Helper API Summary

The implemented helper is:

```rust
pub fn validate_work_report_artifact_side_effect_integrity(
    store: &impl SideEffectRecordStore,
    input: WorkReportArtifactSideEffectIntegrityInput<'_>,
) -> Result<WorkReportArtifactSideEffectIntegrityResult, WorkflowOsError>
```

`WorkReportArtifactSideEffectIntegrityInput` contains:

- borrowed `WorkReportArtifactRecord`;
- `require_all_side_effect_citations` strict/permissive policy flag.

`WorkReportArtifactSideEffectIntegrityResult` exposes bounded counts:

- cited unique SideEffect IDs;
- resolved records;
- missing records;
- duplicate citations.

The result does not expose report IDs, run IDs, SideEffect IDs, paths, targets, summaries, payloads, or raw store details.

## 5. Integrity Boundary Summary

The helper checks only SideEffect citations already present in the artifact's contained report. It does not add citations, repair reports, create missing records, transition SideEffect lifecycle state, or discover additional SideEffects.

Successful validation means only:

```text
The artifact's cited SideEffect IDs resolve to validated records with matching immutable run identity, according to the selected strict/permissive policy.
```

It does not mean:

- the SideEffect was executed;
- the SideEffect was approved;
- provider state changed;
- the record is complete evidence;
- all possible SideEffects were discovered;
- runtime writes are supported.

## 6. Error Handling Summary

Implemented stable error codes:

- `work_report_artifact.side_effect_integrity.record_missing`
- `work_report_artifact.side_effect_integrity.identity_mismatch`
- `work_report_artifact.side_effect_integrity.record_corrupt`
- `work_report_artifact.side_effect_integrity.store_read_failed`
- `work_report_artifact.side_effect_integrity.invalid_artifact`

Errors intentionally do not include SideEffect IDs, WorkReport IDs, workflow IDs, run IDs, versions, hashes, store paths, targets, summaries, authority context, lifecycle payloads, idempotency details, raw record JSON, report text, provider payloads, command output, parser payloads, tokens, credentials, private keys, or secret-like values.

## 7. Redaction And Privacy Summary

The helper is reference-only.

It may inspect:

- report citation targets;
- typed `SideEffectId` values;
- report immutable run identity;
- SideEffect record immutable run identity.

It does not copy into results, errors, Debug output, artifacts, or reports:

- SideEffect target references;
- SideEffect summaries;
- reason codes;
- authority context;
- idempotency details;
- raw record JSON;
- provider payloads;
- command output;
- CI logs;
- Jira/GitHub bodies or file contents;
- spec contents;
- parser payloads;
- environment variable values;
- credentials;
- token-like values;
- local filesystem paths.

Debug output for the new input redacts the artifact. Debug output for the new result exposes bounded counts only.

## 8. Test Coverage Summary

Added tests for:

- artifact with no SideEffect citations succeeds with zero counts;
- artifact with one matching SideEffect record succeeds;
- duplicate SideEffect citations de-duplicate deterministically and count duplicates;
- strict missing-record mode fails without leaking the missing ID;
- permissive missing-record mode returns bounded missing count;
- immutable identity mismatch fails without leaking IDs or identity values;
- corrupt store read maps to stable non-leaking integrity error;
- generic store read failure maps to stable non-leaking integrity error;
- Debug output for input/result does not leak SideEffect IDs, report IDs, or run IDs.

Existing WorkReport artifact, SideEffect discovery, executor, runtime, evidence, diagnostic, validation, adapter, hook, and local-check tests remain in the workspace suite.

## 9. Commands Run And Results

- `cargo fmt --all` - passed
- `cargo test -p workflow-core --test work_report side_effect_integrity` - passed
- `cargo fmt --all --check` - passed
- `cargo clippy --workspace --all-targets -- -D warnings` - passed
- `cargo test --workspace` - passed
- `npm run check:docs` - passed
- `git diff --check` - passed

## 10. Remaining Known Limitations

- No combined artifact write plus SideEffect integrity helper exists.
- `WorkReportArtifactStore::write_work_report_artifact(...)` does not automatically validate cited SideEffect records.
- Executor paths do not automatically write artifacts or run artifact integrity validation.
- SideEffect discovery remains explicit and separate.
- Approval-side-effect linkage remains deferred.
- EvidenceReference side-effect attachment remains deferred.
- Runtime side-effect execution and write-capable adapters remain unsupported.

## 11. Recommended Next Phase

Recommended next phase: **report artifact SideEffect referential integrity helper review**.

The review should verify the helper stays explicit, validation-only, reference-only, non-mutating, non-leaking, and separate from artifact writes, automatic discovery, executor behavior, CLI rendering, schemas, examples, runtime side-effect execution, provider mutation, and write-capable adapter planning.
