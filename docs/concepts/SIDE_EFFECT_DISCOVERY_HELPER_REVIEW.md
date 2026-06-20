# SideEffect Discovery Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The in-memory SideEffect discovery helper is appropriately bounded, deterministic, and redaction-safe for the first discovery slice. It should proceed to store-backed SideEffect discovery planning before any WorkReport auto-discovery, EvidenceReference side-effect attachment, attempted/completed/failed lifecycle behavior, runtime side-effect execution, or write-capable adapter work.

## 2. Scope Verification

The phase stayed within the approved in-memory helper scope.

Implemented scope:

- explicit `SideEffectDiscoveryInput`;
- explicit `SideEffectDiscoverySource`;
- explicit `SideEffectDiscoveryReference`;
- explicit `SideEffectDiscoveryResult`;
- `discover_side_effect_references`;
- discovery from caller-supplied SideEffect IDs;
- discovery from already-loaded proposed, denied, and skipped workflow events;
- discovery from already-loaded validated SideEffect records;
- deterministic de-duplication and result ordering;
- immutable run identity validation for supplied events and records;
- optional required-record enforcement;
- bounded unsupported-event accounting for attempted/completed/failed event vocabulary;
- redaction-safe Debug output;
- focused tests and documentation.

No accidental implementation found for:

- store-backed discovery wrapper;
- `StateBackend` reads during discovery;
- `SideEffectRecordStore` reads during discovery;
- automatic executor report discovery;
- WorkReport side-effect discovery integration;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage enforcement;
- runtime side-effect execution;
- attempted/completed/failed executor behavior;
- write-capable adapters;
- provider mutation;
- workflow schema fields;
- CLI behavior;
- example updates;
- hosted or distributed runtime behavior;
- reasoning lineage;
- rollback or compensation behavior;
- Level 3/4 autonomy enablement;
- release posture changes.

## 3. API Assessment

The helper API is narrow and appropriate:

- `discover_side_effect_references(input: &SideEffectDiscoveryInput) -> Result<SideEffectDiscoveryResult, WorkflowOsError>`

The API accepts explicit in-memory context and already-loaded sources rather than hidden global state. It does not require a state backend, live adapters, runtime configuration, report generation, filesystem output, or CLI output. The return type exposes stable references and bounded counts rather than payloads.

The public export from `workflow-core` is consistent with the surrounding SideEffect model and report helper APIs. The scope is still local and explicit; export does not make discovery automatic.

## 4. Discovery Behavior Assessment

Discovery behavior matches the plan:

- explicit caller-supplied IDs are discovered;
- proposed workflow events are discovered;
- denied workflow events are discovered;
- skipped workflow events are discovered;
- supplied records are validated and discovered;
- duplicate IDs are de-duplicated deterministically;
- output is ordered by SideEffect ID;
- duplicate source priority is preserved by first inserted source;
- attempted/completed/failed SideEffect events are counted as unsupported and not treated as implemented lifecycle behavior.

This is the right boundary for the first slice. It gives future report generation a deterministic reference discovery primitive without implying that Workflow OS can execute or complete side effects.

## 5. Identity And Validation Assessment

Validation is appropriately strict:

- workflow event identity must match workflow ID, workflow version, schema version, spec hash, and run ID;
- SideEffect record identity must match workflow ID, workflow version, schema version, spec hash, and run ID;
- supplied records must pass `SideEffectRecord::validate`;
- missing records fail closed when `require_records` is true;
- optional missing records remain represented as bounded missing-record count when records are not required.

Validation errors use stable codes:

- `side_effect_discovery.identity_mismatch`;
- `side_effect_discovery.record_missing`;
- `side_effect_discovery.record_corrupt`.

The error messages are non-leaking and do not include SideEffect IDs, run IDs, workflow IDs, spec hashes, target references, provider payloads, command output, snippets, credentials, or token-like values.

## 6. Privacy And Redaction Assessment

The helper remains reference-only.

Verified safe behavior:

- raw provider payloads are not read or copied;
- raw command output is not read or copied;
- raw CI logs are not read or copied;
- raw Jira issue bodies or comments are not read or copied;
- raw GitHub file contents are not read or copied;
- raw spec contents are not read or copied;
- parser payloads are not read or copied;
- environment variable values are not read or copied;
- credentials, authorization headers, private keys, and token-like values are not read or copied;
- helper Debug output redacts SideEffect IDs and immutable run identity values;
- result Debug output reports counts only;
- reference Debug output redacts the SideEffect ID.

No serde surface was added for discovery inputs or results, which is appropriate for this first in-memory helper.

## 7. Source-Of-Truth Boundary Assessment

The phase preserves the intended source-of-truth boundaries:

- workflow events remain authoritative workflow run history;
- SideEffect records remain the future durable source for side-effect intent and lifecycle records;
- audit projections are not treated as authoritative discovery inputs;
- adapter telemetry is not treated as authoritative discovery input;
- reports are not used as discovery input;
- debug strings and prose are not used as discovery input.

The helper does not create SideEffect records, repair missing records, append events, mutate snapshots, emit audit events, call adapters, execute side effects, write files, or persist reports.

## 8. Test Quality Assessment

Tests cover the most important first-slice behavior:

- explicit SideEffect IDs are returned deterministically;
- duplicate explicit IDs are de-duplicated;
- proposed, denied, and skipped events are discovered;
- attempted/completed/failed events are counted as unsupported and not discovered;
- matching records are discovered;
- required records satisfy the required-record policy;
- missing required records fail closed without leaking the SideEffect ID;
- workflow event identity mismatch fails without leaking identity values;
- record identity mismatch fails without leaking identity values;
- Debug output does not leak IDs or immutable run identity values;
- existing workspace tests pass.

Non-blocking test gaps:

- add an explicit cross-source duplicate test proving explicit IDs win over event and record sources;
- add an explicit non-SideEffect workflow event ignored test;
- add separate mismatch tests for workflow version, schema version, and spec hash if the store-backed wrapper makes those failure modes more likely;
- add future store-backed corrupt-record tests when the wrapper exists.

These gaps do not block the current phase because the implemented helper is not yet wired into report generation or executor automation.

## 9. Documentation Review

Documentation is accurate and scope-clean.

Verified documentation states:

- the first explicit in-memory SideEffect discovery helper is implemented;
- store-backed discovery is not implemented;
- automatic WorkReport discovery is not implemented;
- EvidenceReference side-effect attachment is not implemented;
- approval-side-effect linkage is not implemented;
- runtime side-effect execution is not implemented;
- attempted/completed/failed executor behavior is not implemented;
- write-capable adapters and provider mutation are not implemented;
- schemas, CLI behavior, examples, hosted behavior, reasoning lineage, and release posture changes are not implemented.

The roadmap and persistence/discovery planning documents should now advance from "review next" to "review accepted; store-backed discovery planning next."

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add cross-source duplicate source-priority coverage before automatic report discovery uses this helper.
- Add an explicit ignored non-SideEffect event test.
- Plan a store-backed discovery wrapper that loads records through `SideEffectRecordStore` and reuses this helper for validation and deterministic output.
- Keep attempted/completed/failed lifecycle discovery deferred until runtime execution semantics are planned and reviewed.
- Keep audit projection and adapter telemetry as non-authoritative hints unless a later plan defines stable SideEffect ID citation behavior for those surfaces.

## 12. Recommended Next Phase

Recommended next phase: store-backed SideEffect discovery planning.

The in-memory helper is accepted, but the next risky boundary is not WorkReport automation yet. The next safe phase is to plan how a wrapper should load already-persisted `SideEffectRecord` values from `SideEffectRecordStore`, verify full immutable identity, and pass explicit records into the accepted in-memory helper without mutating runtime state or treating discovery failure as workflow execution failure.

## 13. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.
