# SideEffect Approval Linkage Store-Backed Report

## 1. Executive Summary

The store-backed SideEffect approval linkage helper is implemented as an explicit, validation-only composition boundary.

The helper loads already-persisted `SideEffectRecord` values through a supplied `SideEffectRecordStore`, validates the loaded records, and delegates approval linkage checks to the accepted in-memory `validate_side_effect_approval_linkage(...)` helper.

It does not run automatically during executor execution, report generation, report artifact writes, SideEffect discovery, CLI behavior, schema validation, provider mutation, runtime side-effect execution, write-capable adapters, approval evidence attachment, reasoning lineage, or release posture changes.

## 2. Scope Completed

Implemented:

- `SideEffectApprovalLinkageFromStoreInput`;
- `SideEffectApprovalLinkageFromStoreResult`;
- `SideEffectApprovalLinkageStoreLoadMode`;
- `SideEffectMissingRecordPolicy`;
- `validate_side_effect_approval_linkage_from_store(...)`;
- `workflow-core` exports for the helper and types;
- explicit SideEffect ID loading through `SideEffectRecordStore::read_side_effect_record(...)`;
- all-records-for-run loading through `SideEffectRecordStore::list_side_effect_records_for_workflow_run(...)`;
- deterministic duplicate explicit ID counting;
- optional or required missing-record behavior;
- stored-record validation before approval linkage;
- delegation to the accepted in-memory approval linkage helper;
- bounded count result;
- redaction-safe `Debug` for input and result;
- focused tests for store-backed loading, missing records, duplicate IDs, store failures, inherited decision mismatch, and non-leakage;
- documentation status updates.

## 3. Scope Explicitly Not Completed

Not implemented:

- automatic approval-side-effect validation during report generation;
- automatic approval-side-effect validation during report artifact writes;
- automatic approval-side-effect validation during executor execution;
- automatic approval-side-effect validation during SideEffect discovery;
- report/artifact composition helper;
- report artifact writes;
- runtime side-effect execution;
- `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` executor behavior;
- write-capable adapters;
- provider mutation;
- approval evidence attachment;
- approval packet model;
- high-assurance approval role/quorum controls;
- requester/approver separation enforcement;
- approval revocation or expiration enforcement changes;
- workflow schema fields;
- CLI behavior;
- examples;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Helper API Summary

The helper accepts:

- a caller-supplied `SideEffectRecordStore`;
- a borrowed `WorkflowRun`;
- optional explicit `SideEffectId` values;
- a load mode for explicit IDs, all records for the run, or both;
- a missing explicit record policy that either requires all records or counts missing records;
- existing approval-linkage policy flags for `RequiresApproval`, `ApprovedByHuman`, and `DeniedByApproval`.

The helper returns bounded counts:

- explicit SideEffect ID count;
- loaded SideEffect record count;
- missing SideEffect record count;
- duplicate explicit SideEffect ID count;
- in-memory approval linkage record/reference counts.

The helper does not construct a backend, infer filesystem paths, mutate a store, append workflow events, call adapters, generate reports, write artifacts, or execute side effects.

## 5. Store Read Policy

The helper supports two explicit source modes:

- explicit ID mode: reads only caller-supplied `SideEffectId` values;
- all-records-for-run mode: lists records for the supplied run's workflow/run identity.

At least one source must be selected. Duplicate explicit IDs are de-duplicated deterministically and counted. Missing explicit records fail closed when `require_all_referenced_records` is true and are counted when it is false.

Store read and list failures are mapped to stable non-leaking linkage errors.

## 6. Validation Boundary Summary

Validation checks:

- at least one store source is selected;
- the supplied run can be rehydrated from event history;
- the supplied run snapshot matches the rehydrated snapshot;
- loaded records validate before linkage;
- loaded records match the supplied run identity through the delegated helper;
- approval references resolve to approval events from the same run;
- approval decision kinds match SideEffect authority posture.

The helper intentionally does not validate role authority, approver identity separation, evidence sufficiency, approval expiration, external identity, provider state, or side-effect execution outcome.

## 7. Error Handling Summary

New store-backed error codes:

- `side_effect_approval_linkage.invalid_input`
- `side_effect_approval_linkage.store_read_failed`
- `side_effect_approval_linkage.record_missing`
- `side_effect_approval_linkage.record_corrupt`

Delegated linkage errors remain unchanged:

- `side_effect_approval_linkage.identity_mismatch`
- `side_effect_approval_linkage.approval_missing`
- `side_effect_approval_linkage.decision_missing`
- `side_effect_approval_linkage.decision_kind_mismatch`
- `side_effect_approval_linkage.step_mismatch`
- `side_effect_approval_linkage.skill_mismatch`
- `side_effect_approval_linkage.record_invalid`
- `side_effect_approval_linkage.run_invalid`

Errors do not include SideEffect IDs, approval IDs, workflow IDs, run IDs, step IDs, skill IDs, spec hashes, target references, summaries, reasons, redaction metadata, store paths, record JSON, provider payloads, command output, parser payloads, snippets, credentials, tokens, authorization headers, or private keys.

## 8. Redaction And Privacy Summary

The helper is reference-only.

It may inspect:

- SideEffect IDs;
- SideEffect record identity fields;
- SideEffect authority references;
- authority decisions;
- approval decision kind;
- immutable run identity;
- optional step and skill scope;
- bounded counts.

It does not copy approval reasons, SideEffect summaries, SideEffect target references, raw provider payloads, raw CI logs, Jira/GitHub bodies, command output, spec contents, parser payloads, environment values, credentials, authorization headers, private keys, token-like values, local paths, or stored record JSON into errors, results, reports, artifacts, or Debug output.

## 9. Test Coverage Summary

Added focused tests for:

- explicit SideEffect ID mode with matching granted approval;
- all-records-for-run mode;
- duplicate explicit ID de-duplication and counting;
- required missing record failure without leaking the missing ID;
- optional missing record counting;
- store read failure mapping without leaking lower-level error text;
- invalid input failure when no source is selected;
- inherited decision mismatch validation;
- bounded non-leaking input/result `Debug` output.

Existing SideEffect model and in-memory approval linkage tests continue to pass.

## 10. Commands Run And Results

Commands run:

- `cargo fmt --all` - passed
- `cargo fmt --all --check` - passed
- `cargo clippy --workspace --all-targets -- -D warnings` - passed
- `cargo test -p workflow-core --test side_effect` - passed
- `cargo test --workspace` - passed
- `npm run check:docs` - passed
- `git diff --check` - passed

## 11. Remaining Known Limitations

- The helper is not automatically invoked by report generation, report artifact writes, executor execution, or SideEffect discovery.
- The helper does not compose referential integrity validation and approval linkage into one artifact/report path.
- The helper does not emit audit or observability events.
- Successful linkage does not imply side-effect execution, provider mutation, evidence completeness, artifact persistence, or write safety.
- High-assurance approval controls remain future work.

## 12. Recommended Next Phase

Recommended next phase: **SideEffect store-backed approval linkage helper review**.

After that review, the project should move to a larger **write-capable adapter readiness slice** rather than continuing to split every small composition helper into separate planning phases. That slice should still preserve the current safety line: no runtime side-effect execution or provider mutation until policy, approval, evidence, state, audit, idempotency, and report/report-artifact boundaries are reviewed together.
