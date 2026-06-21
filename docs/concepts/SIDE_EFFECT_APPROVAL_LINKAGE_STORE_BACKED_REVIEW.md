# SideEffect Approval Linkage Store-Backed Review

Review date: 2026-06-21

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The store-backed SideEffect approval linkage helper stays within the approved explicit, validation-only composition scope. It loads already-persisted `SideEffectRecord` values from a caller-supplied `SideEffectRecordStore`, supports explicit ID and all-records-for-run loading, maps store failures to stable non-leaking linkage errors, and delegates authority validation to the accepted in-memory `validate_side_effect_approval_linkage(...)` helper.

It does not automatically run during executor execution, report generation, report artifact writes, SideEffect discovery, CLI behavior, schema validation, runtime side-effect execution, provider mutation, write-capable adapter behavior, approval evidence attachment, reasoning lineage, or release posture changes.

## 2. Scope Verification

The phase stayed within the approved store-backed helper-only scope.

Implemented:

- `SideEffectApprovalLinkageFromStoreInput`
- `SideEffectApprovalLinkageFromStoreResult`
- `SideEffectApprovalLinkageStoreLoadMode`
- `SideEffectMissingRecordPolicy`
- `validate_side_effect_approval_linkage_from_store(...)`
- `workflow-core` exports for the helper and types
- explicit SideEffect ID loading through `SideEffectRecordStore::read_side_effect_record(...)`
- all-records-for-run loading through `SideEffectRecordStore::list_side_effect_records_for_workflow_run(...)`
- deterministic duplicate explicit ID counting
- required or counted missing-record behavior
- loaded-record validation before delegated approval linkage validation
- delegation to `validate_side_effect_approval_linkage(...)`
- bounded count result and redaction-safe `Debug`
- focused positive, failure, duplicate-ID, inherited-linkage-failure, and non-leakage tests
- documentation status updates

No accidental implementation was found for:

- automatic approval-side-effect validation during report generation;
- automatic approval-side-effect validation during report artifact writes;
- automatic approval-side-effect validation during executor execution;
- automatic approval-side-effect validation during SideEffect discovery;
- report/artifact composition helper;
- report artifact writes;
- creating, mutating, or deleting SideEffect records;
- creating, mutating, or deleting approval requests or decisions;
- appending workflow events;
- emitting audit or observability events;
- runtime side-effect execution;
- `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` executor behavior;
- write-capable adapters;
- provider mutation;
- approval evidence attachment;
- approval packet modeling;
- high-assurance role, quorum, requester/approver separation, expiration, or revocation controls;
- workflow schema fields;
- CLI behavior;
- examples;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 3. Helper API Assessment

The helper API is appropriately explicit:

```rust
pub fn validate_side_effect_approval_linkage_from_store(
    store: &impl SideEffectRecordStore,
    input: SideEffectApprovalLinkageFromStoreInput<'_>,
) -> Result<SideEffectApprovalLinkageFromStoreResult, WorkflowOsError>
```

The input requires a caller-supplied store and borrowed `WorkflowRun`, accepts optional explicit `SideEffectId` values, exposes load mode and missing-record policy, and passes through the existing approval-linkage policy booleans.

The result exposes only bounded counts:

- explicit SideEffect ID count;
- unique loaded SideEffect record count;
- missing SideEffect record count;
- duplicate explicit SideEffect ID count;
- delegated approval-linkage record/reference counts.

The API does not construct a backend, infer filesystem paths, assume artifact and SideEffect stores are the same object, append events, call executor methods, call adapters, emit audit records, generate reports, write artifacts, or execute side effects.

One non-blocking clarity point remains: `loaded_side_effect_record_count` is a unique loaded record count after explicit and all-records-for-run sources are merged. That is the right behavior, but it should be documented or tested directly for the `ExplicitIdsAndAllRecordsForRun` overlap case before broader composition work depends on the exact count semantics.

## 4. Store Read Policy Assessment

The store read policy matches the composition plan.

Positive behavior is correct:

- `ExplicitIds` reads only caller-supplied IDs.
- `AllRecordsForRun` reads through `list_side_effect_records_for_workflow_run(...)`.
- `ExplicitIdsAndAllRecordsForRun` combines both sources and de-duplicates loaded records by SideEffect ID.
- empty explicit IDs with `ExplicitIds` fail closed as invalid input.
- duplicate explicit IDs are counted and only loaded once.
- missing explicit IDs fail closed under `RequireAll`.
- missing explicit IDs are counted and ignored under `CountMissing`.
- store read/list failures are mapped to `side_effect_approval_linkage.store_read_failed`.

Loaded records are validated before linkage and then revalidated for immutable run identity by the delegated helper. The local and in-memory stores already preserve read-by-ID integrity through their store contracts.

Non-blocking hardening: add a direct helper-level guard and test that a record returned from `read_side_effect_record(requested_id)` carries the same `SideEffectId` as `requested_id`. The current production stores enforce this, but an explicit check would make the helper less dependent on store implementation discipline.

## 5. Validation Boundary Assessment

The helper preserves the accepted validation boundary.

It validates:

- at least one store source is selected;
- the supplied run can be rehydrated from event history;
- the supplied run snapshot matches the rehydrated snapshot;
- store-loaded records pass `SideEffectRecord::validate()`;
- loaded records match the supplied run identity through delegated linkage validation;
- approval references resolve to approval events from the same immutable run;
- approval decision kind matches `ApprovedByHuman` or `DeniedByApproval` authority posture;
- `RequiresApproval` can link to an approval request without treating it as granted;
- optional step, skill ID, and skill version checks continue to be enforced by the in-memory helper.

It intentionally does not validate role authority, quorum, approver/requester separation, approval evidence sufficiency, approval expiration, approval revocation, external identity, provider state, report completeness, artifact persistence, or side-effect execution outcome.

Successful validation means only that the loaded SideEffect records that claim approval authority are backed by matching approval events in the supplied run.

## 6. Error Handling/Privacy Assessment

The helper uses stable non-leaking store-backed error codes:

- `side_effect_approval_linkage.invalid_input`
- `side_effect_approval_linkage.store_read_failed`
- `side_effect_approval_linkage.record_missing`
- `side_effect_approval_linkage.record_corrupt`

Delegated linkage failures retain the accepted in-memory linkage codes:

- `side_effect_approval_linkage.identity_mismatch`
- `side_effect_approval_linkage.approval_missing`
- `side_effect_approval_linkage.decision_missing`
- `side_effect_approval_linkage.decision_kind_mismatch`
- `side_effect_approval_linkage.step_mismatch`
- `side_effect_approval_linkage.skill_mismatch`
- `side_effect_approval_linkage.record_invalid`
- `side_effect_approval_linkage.run_invalid`

Store errors are not propagated directly, which prevents leaking store paths, lower-level storage details, record JSON, SideEffect IDs, approval IDs, workflow IDs, run IDs, step IDs, skill IDs, spec hashes, target references, summaries, reasons, redaction metadata, provider payloads, command output, parser payloads, snippets, credentials, tokens, authorization headers, or private keys.

`SideEffectApprovalLinkageFromStoreInput` and `SideEffectApprovalLinkageFromStoreResult` `Debug` output expose only counts, policy enums, booleans, and redacted run placeholders.

## 7. Relationship To Discovery/Report/Artifact/Executor Paths

The helper remains separate from discovery, report generation, report artifact integrity, and executor execution.

Discovery still answers which SideEffect IDs are relevant. Report generation still constructs governed handoff content and citations. Report artifact integrity still checks whether cited SideEffect IDs resolve to records matching artifact run identity. Executor paths still execute existing local workflow behavior without automatic approval-linkage validation.

Approval linkage now has an explicit store-backed composition point, but no existing discovery, report, artifact, or executor path calls it automatically. That separation is important because automatic linkage would add new failure modes to paths that were previously reviewed as citation, artifact, or execution boundaries rather than authority validators.

Future composition should be explicit and reviewed. A combined artifact/report helper should validate the artifact, validate SideEffect referential integrity, validate approval linkage for resolved records, and only then write an artifact. Executor and write-capable adapter paths should remain out of scope until policy, approval, evidence, state, audit, idempotency, and report/report-artifact boundaries are reviewed together.

## 8. Test Quality Assessment

Tests cover:

- explicit SideEffect ID mode with matching granted approval;
- all-records-for-run mode;
- duplicate explicit ID de-duplication and counting;
- required missing-record failure without leaking the missing ID;
- optional missing-record counting;
- store read failure mapping without leaking lower-level error text;
- invalid input when no source is selected;
- delegated decision mismatch validation;
- bounded non-leaking input/result `Debug` output;
- existing in-memory approval linkage behavior.

No blocker-level test gaps were found.

Non-blocking gaps:

- Add a direct test for `ExplicitIdsAndAllRecordsForRun` result semantics, especially explicit/all overlap.
- Add a direct test for list failure mapping, not only read failure mapping.
- Add a direct test for a store returning a record whose `side_effect_id` does not match the explicit requested ID, paired with the helper-level guard recommended above.
- Add a direct store-backed identity mismatch test to prove explicit-ID records from another run fail through the delegated helper.
- Carry forward the previous in-memory linkage follow-ups for direct skill-version mismatch, non-approval reference kind, invalid run, approval request identity mismatch, and policy-boolean semantics.

## 9. Documentation Review

Documentation accurately describes the store-backed helper as explicit and validation-only.

The composition plan and store-backed report state that the helper:

- loads persisted SideEffect records through a supplied store;
- supports explicit IDs and all-records-for-run loading;
- delegates approval linkage validation to the accepted in-memory helper;
- returns bounded counts only;
- does not run automatically in report generation, artifact writing, executor execution, or discovery;
- does not implement report/artifact composition, runtime side-effect execution, provider mutation, write-capable adapters, approval evidence attachment, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

The remaining documentation improvement is to document the exact unique-record merge semantics for `ExplicitIdsAndAllRecordsForRun` and the helper's reliance on store read-by-ID integrity, or to make that integrity check explicit in code and tests.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Add a helper-level explicit-ID returned-record ID equality guard and test.
- Add a direct `ExplicitIdsAndAllRecordsForRun` overlap/count semantics test.
- Add a direct list failure mapping test.
- Add a direct store-backed run identity mismatch test.
- Document unique loaded-record count semantics for merged load modes.
- Carry forward in-memory linkage test/documentation follow-ups before report/artifact composition depends on those edges.

## 12. Recommended Next Phase

Recommended next phase: **write-capable adapter readiness planning**.

The store-backed helper is accepted as the explicit persisted-record composition boundary. The next useful phase should stop splitting small linkage helpers and instead plan the larger readiness slice for future writes: policy gates, approval/evidence requirements, state transitions, audit and observability, idempotency, report/report-artifact composition, adapter contracts, and failure semantics. Runtime side-effect execution and provider mutation should remain unimplemented until that readiness boundary is planned and reviewed.

## 13. Validation

Commands run:

- `npm run check:docs` - passed using repo-local npm at `.tools/node-v20.19.5-darwin-arm64/bin/npm`
