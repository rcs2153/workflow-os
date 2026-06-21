# SideEffect Store-Backed Discovery Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The store-backed discovery helper is appropriately bounded, deterministic, and redaction-safe for the first store-backed SideEffect discovery slice. It should proceed to WorkReport SideEffect discovery integration planning before any automatic report discovery, runtime side-effect execution, attempted/completed/failed lifecycle broadening, EvidenceReference side-effect attachment, or write-capable adapter work.

## 2. Scope Verification

The phase stayed within the approved explicit helper scope.

Implemented scope:

- explicit `SideEffectStoreBackedDiscoveryInput`;
- explicit `discover_side_effect_references_from_store`;
- store reads through `SideEffectRecordStore`;
- delegation to the accepted in-memory `discover_side_effect_references` helper;
- deterministic reference ordering and source priority;
- full immutable run identity validation after records are loaded;
- stable non-leaking discovery errors;
- focused tests;
- documentation and end-of-phase report.

No accidental out-of-scope behavior was found:

- no automatic WorkReport side-effect discovery;
- no executor integration;
- no report artifact integration;
- no EvidenceReference side-effect attachment;
- no approval-side-effect linkage enforcement;
- no runtime side-effect execution;
- no attempted/completed/failed executor behavior;
- no write-capable adapter behavior;
- no provider mutation;
- no SideEffect record creation, repair, update, or lifecycle transition;
- no workflow-declared SideEffect configuration;
- no runtime SideEffect configuration;
- no workflow schema fields;
- no CLI commands or rendering;
- no example updates;
- no hosted or distributed runtime behavior;
- no reasoning lineage;
- no rollback or compensation behavior;
- no Level 3/4 autonomy enablement;
- no release posture changes.

## 3. API Assessment

The API is narrow and consistent with the accepted discovery boundary:

- `SideEffectStoreBackedDiscoveryInput` carries explicit immutable run identity, explicit SideEffect IDs, caller-supplied workflow events, and the required-record policy.
- `discover_side_effect_references_from_store` accepts an explicit `SideEffectRecordStore` reference and returns `SideEffectDiscoveryResult`.
- The helper does not construct a backend, read hidden global state, infer identity from local paths, or require `LocalStateBackend`.
- The API is exported from `workflow-core` with the existing SideEffect discovery vocabulary.

This is a good fit for the next integration layer because it gives future callers a single validated reference-discovery boundary without making discovery automatic.

## 4. Store Loading Assessment

The implementation uses:

- `SideEffectRecordStore::list_side_effect_records_for_workflow_run(&workflow_id, &run_id)`

That is the right first-slice choice. It keeps discovery deterministic and small, avoids per-ID targeted store semantics before they are needed, and works with the existing local store contract.

The implementation preserves the intended two-layer identity boundary:

1. store-level workflow/run listing check;
2. helper-level full immutable identity check for workflow ID, workflow version, schema version, spec hash, and run ID.

This is important because the store listing API accepts workflow ID and run ID, while the full immutable identity is carried by each loaded `SideEffectRecord`.

## 5. Discovery Semantics Assessment

The implementation preserves accepted in-memory discovery behavior:

- explicit caller-supplied SideEffect IDs have first source priority;
- caller-supplied supported workflow events have second source priority;
- loaded SideEffect records have third source priority;
- duplicate IDs de-duplicate deterministically;
- references are ordered by SideEffect ID;
- `Proposed`, `Denied`, and `Skipped` events are supported;
- `Attempted`, `Completed`, and `Failed` events remain unsupported and counted rather than cited;
- missing records are counted when optional;
- missing records fail closed when `require_records` is true.

The helper does not create WorkReport citations or EvidenceReference values. It returns stable SideEffect IDs and bounded counts only.

## 6. Validation And Error Assessment

Validation behavior is appropriate:

- loaded records are revalidated by the in-memory helper;
- loaded records must match full immutable run identity;
- store identity mismatch maps to `side_effect_discovery.identity_mismatch`;
- corrupt store records map to `side_effect_discovery.record_corrupt`;
- generic store read failures map to `side_effect_discovery.store_read_failed`;
- missing required records still fail as `side_effect_discovery.record_missing`.

The errors are stable and non-leaking. They do not include SideEffect IDs, workflow IDs, run IDs, workflow versions, schema versions, spec hashes, target references, provider payloads, local paths, command output, snippets, credentials, tokens, or raw record JSON.

## 7. Privacy And Redaction Assessment

The store-backed helper remains reference-only.

It does not store, copy, serialize, debug-print, or include in errors:

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

`SideEffectStoreBackedDiscoveryInput` Debug output redacts immutable identity values and SideEffect IDs, exposing only bounded counts and non-sensitive flags.

## 8. Runtime And State Boundary Assessment

The implementation does not alter runtime semantics.

The helper:

- does not mutate `WorkflowRun`;
- does not mutate `WorkflowRunSnapshot`;
- does not append workflow events;
- does not create or update SideEffect records;
- does not repair corrupt records;
- does not emit audit events;
- does not emit observability events;
- does not execute side effects;
- does not call adapters or providers;
- does not write report artifacts;
- does not create CLI output;
- does not change workflow pass/fail behavior.

The only store interaction is a read through the explicit `SideEffectRecordStore` trait.

## 9. Test Quality Assessment

The tests are strong for this phase. They cover:

- store-backed discovery returning persisted records for a requested run;
- deterministic ordering of records loaded from the local store;
- explicit ID priority over record discovery;
- workflow event priority over record discovery;
- optional missing records producing bounded missing counts;
- required missing records failing closed without leaking IDs;
- store identity errors mapping to non-leaking discovery errors;
- corrupt store errors mapping to non-leaking discovery errors;
- generic store read failures mapping to non-leaking discovery errors;
- store-backed input Debug non-leakage;
- existing in-memory discovery behavior.

Existing store tests and discovery helper tests provide supporting coverage for validated records, duplicate rejection, corrupt record behavior, and identity mismatch behavior.

Shallow or missing tests:

- There is not a dedicated store-backed test proving the store contents are unchanged before and after discovery. This is non-blocking because the helper only calls the read-only store listing method, but a future integration test would make the boundary more visible.
- The store-backed test suite does not separately exercise every immutable identity field mismatch through the wrapper. The delegated in-memory helper and local store tests cover the mechanics, but future tests could add one wrapper-level schema/spec mismatch case if the API broadens.

## 10. Documentation Review

Documentation is accurate for the implemented scope.

Docs state that:

- store-backed SideEffect discovery is implemented;
- the implementation is an explicit helper wrapper;
- automatic WorkReport discovery is not implemented;
- executor integration is not implemented;
- report artifact integration is not implemented;
- EvidenceReference side-effect attachment is not implemented;
- runtime side-effect execution is not implemented;
- attempted/completed/failed executor behavior is not implemented;
- writes and provider mutations are not implemented;
- schemas, CLI behavior, examples, hosted behavior, reasoning lineage, and release posture changes are not implemented.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Add an explicit no-mutation regression test around store-backed discovery if the helper is later called from report generation paths.
- Consider targeted read-by-ID discovery only if report integration needs to avoid listing all run records.
- Add wrapper-level immutable identity mismatch tests if future store implementations make identity filtering more complex.
- Keep attempted/completed/failed lifecycle discovery deferred until runtime execution semantics are separately accepted.

## 13. Recommended Next Phase

Recommended next phase: WorkReport SideEffect discovery integration planning.

The store-backed helper is now accepted as a bounded reference-discovery primitive. The next step should be planning how explicit report-generation paths may call it to populate WorkReport SideEffect citations without making discovery automatic, changing executor semantics, creating report artifacts, executing side effects, or enabling write-capable adapters.

## 14. Validation

Commands run:

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.
