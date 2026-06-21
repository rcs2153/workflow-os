# Report Artifact SideEffect Referential Integrity Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The validation-only report artifact SideEffect referential integrity helper is scoped, conservative, and consistent with the accepted plan. It validates cited SideEffect IDs against an explicit `SideEffectRecordStore`, checks immutable run identity, returns bounded counts, and keeps artifact writes, executor behavior, automatic discovery, runtime side-effect execution, and writes out of scope.

## 2. Scope Verification

The phase stayed within approved validation-only helper scope.

Implemented:

- `WorkReportArtifactSideEffectIntegrityInput`
- `WorkReportArtifactSideEffectIntegrityResult`
- `validate_work_report_artifact_side_effect_integrity(...)`
- `workflow-core` exports for the helper and types
- focused strict/permissive integrity tests
- roadmap, concept, and implementation-plan documentation updates
- end-of-phase report

No accidental implementation found for:

- automatic report artifact writing;
- automatic artifact integrity validation inside `WorkReportArtifactStore::write_work_report_artifact(...)`;
- combined artifact write plus integrity helper;
- automatic SideEffect discovery;
- executor integration;
- workflow event appends;
- audit event emission;
- workflow state mutation;
- CLI rendering or export;
- schema changes;
- examples;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage enforcement;
- runtime side-effect execution;
- attempted/completed/failed executor behavior;
- write-capable adapters;
- provider mutation;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 3. Helper API Assessment

The helper API is appropriately narrow and testable:

```rust
pub fn validate_work_report_artifact_side_effect_integrity(
    store: &impl SideEffectRecordStore,
    input: WorkReportArtifactSideEffectIntegrityInput<'_>,
) -> Result<WorkReportArtifactSideEffectIntegrityResult, WorkflowOsError>
```

The API accepts an explicit `SideEffectRecordStore` and a borrowed artifact. It does not construct a local backend, read hidden global state, assume artifact and SideEffect stores are the same implementation, or require `StateBackend`.

The input is redaction-safe in `Debug`, and the result exposes bounded counts only.

## 4. Referential Integrity Assessment

The implementation validates only citations whose target is `WorkReportCitationTarget::SideEffect`.

It extracts citations from:

- report sections;
- incomplete/deferred work disclosures;
- known limitations;
- risks;
- operator handoff notes.

This is slightly broader than the minimal section-only wording in the plan and is appropriate because these report containers can also hold citations. It remains reference-only and does not broaden behavior.

The helper de-duplicates `SideEffectId` values through deterministic ordering and counts duplicate citations separately. This satisfies the requirement to avoid repeated store reads while preserving useful bounded metadata.

## 5. Identity Validation Assessment

Resolved records are validated against the report artifact's immutable run identity:

- workflow ID;
- workflow version;
- schema version;
- spec hash;
- run ID.

The helper also calls `SideEffectRecord::validate()` before identity comparison, so corrupt or invalid records fail closed even if a custom store returns them.

The implementation does not validate approval authority, lifecycle execution semantics, provider state, or evidence completeness. That is correct for this phase.

## 6. Strict And Permissive Policy Assessment

Strict mode:

- missing cited records fail closed with `work_report_artifact.side_effect_integrity.record_missing`;
- identity mismatches fail closed;
- corrupt records fail closed;
- store read failures fail closed.

Permissive mode:

- missing cited records are counted in the result;
- identity mismatches still fail closed;
- corrupt records still fail closed;
- store read failures still fail closed.

This matches the plan and avoids silently accepting unsafe identity mismatches.

## 7. Error Handling Assessment

The helper uses stable, non-leaking error codes:

- `work_report_artifact.side_effect_integrity.record_missing`
- `work_report_artifact.side_effect_integrity.identity_mismatch`
- `work_report_artifact.side_effect_integrity.record_corrupt`
- `work_report_artifact.side_effect_integrity.store_read_failed`
- `work_report_artifact.side_effect_integrity.invalid_artifact`

Errors do not include SideEffect IDs, WorkReport IDs, workflow IDs, run IDs, spec hashes, store paths, targets, summaries, authority context, lifecycle payloads, provider payloads, command output, parser payloads, tokens, credentials, or private keys.

Store errors are intentionally mapped to the integrity boundary rather than being propagated with lower-level details. That is the right safety posture.

## 8. Privacy And Redaction Assessment

The helper remains reference-only.

It inspects:

- citation targets;
- typed SideEffect IDs;
- report immutable run identity;
- SideEffect record immutable run identity.

It does not copy SideEffect targets, summaries, reason codes, authority context, idempotency details, raw record JSON, provider payloads, command output, CI logs, Jira/GitHub bodies, spec contents, parser payloads, environment values, credentials, token-like values, or filesystem paths into results, errors, reports, artifacts, or Debug output.

`WorkReportArtifactSideEffectIntegrityInput` Debug redacts the artifact. `WorkReportArtifactSideEffectIntegrityResult` Debug exposes counts only.

## 9. Artifact And Runtime Semantics Assessment

The phase preserves existing semantics:

- `WorkReportArtifactStore::write_work_report_artifact(...)` is unchanged.
- Artifact writes are not made automatic.
- Executor paths are unchanged.
- Workflow runs and snapshots are not mutated.
- No workflow events are appended.
- No audit or observability events are emitted.
- No SideEffect records are created, repaired, or transitioned.
- No providers or adapters are called.
- No side effects are executed.

The helper can be composed by future callers without changing the existing artifact store contract.

## 10. Relationship To Discovery And EvidenceReference

The helper correctly separates referential integrity from discovery.

Discovery asks which SideEffect IDs a report should cite. This helper asks whether already-cited SideEffect IDs resolve to matching records. It does not add citations, discover additional SideEffects, or repair reports.

EvidenceReference side-effect attachment remains deferred. The helper does not create or attach EvidenceReference values.

## 11. Test Quality Assessment

Tests cover:

- artifact with no SideEffect citations succeeds;
- matching SideEffect record succeeds;
- duplicate SideEffect citations de-duplicate deterministically;
- strict missing-record behavior fails without leaking IDs;
- permissive missing-record behavior returns bounded counts;
- immutable identity mismatch fails without leaking IDs or identity values;
- corrupt store read maps to stable non-leaking error;
- generic store read failure maps to stable non-leaking error;
- input/result Debug output is bounded and non-leaking;
- existing WorkReport, artifact store, SideEffect discovery, executor, runtime, EvidenceReference, Diagnostic, validation, adapter, hook, local-check, and docs tests pass through the workspace suite.

Non-blocking gaps:

- Identity mismatch tests currently exercise workflow ID mismatch directly, while workflow version, schema version, spec hash, and run ID share the same comparison branch. This is acceptable for this phase but could be expanded if the helper changes.
- There is no focused test for the `invalid_artifact` mapping. Existing artifact validation tests cover artifact identity mismatch, and the helper calls `artifact.validate()`, but a direct helper-level invalid artifact test would improve coverage.
- There is no explicit mutation test for the helper. Since it borrows the artifact, accepts only a `SideEffectRecordStore`, and has no artifact store or runtime state handle, the mutation surface is structurally constrained. A future test could still assert artifact equality before/after if this helper evolves.

## 12. Documentation Review

Documentation now states:

- report artifact SideEffect referential integrity validation is implemented as an explicit helper;
- normal artifact writes do not automatically run integrity validation;
- automatic artifact writing from executor paths is not implemented;
- automatic SideEffect discovery is not implemented;
- EvidenceReference side-effect attachment is not implemented;
- approval-side-effect linkage enforcement is not implemented;
- runtime side-effect execution is not implemented;
- write-capable adapters are not implemented;
- CLI rendering, schemas, examples, hosted behavior, reasoning lineage, and release posture changes remain unimplemented.

The end-of-phase report accurately summarizes the helper API, scope, privacy posture, tests, validation commands, limitations, and recommended next review phase.

## 13. Blockers

No blockers.

## 14. Non-Blocking Follow-Ups

- Add direct helper-level `invalid_artifact` coverage if the artifact model becomes easier to construct in an invalid state for tests.
- Add separate identity mismatch tests for workflow version, schema version, spec hash, and run ID if this helper is modified.
- Consider whether a future combined artifact write helper is needed once a real caller needs atomic "integrity check then write" behavior.
- Plan approval-side-effect linkage before write-capable adapters so artifact integrity does not get mistaken for approval correctness.

## 15. Recommended Next Phase

Recommended next phase: **approval-side-effect linkage planning**.

The referential integrity helper is accepted, and a combined artifact write helper can wait until there is a concrete caller. Approval-side-effect linkage is the more important safety boundary before any runtime side-effect execution or write-capable adapter planning, because record existence and immutable identity are not enough to prove that a sensitive side effect was authorized.

## 16. Validation

Commands run:

- `cargo fmt --all --check` - passed
- `cargo clippy --workspace --all-targets -- -D warnings` - passed
- `cargo test --workspace` - passed
- `npm run check:docs` - passed
- `git diff --check` - passed
