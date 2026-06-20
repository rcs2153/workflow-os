# SideEffect Store-Backed Discovery Plan

Status: Implemented and accepted. The in-memory SideEffect discovery helper is implemented and accepted in [SideEffect Discovery Helper Review](../concepts/SIDE_EFFECT_DISCOVERY_HELPER_REVIEW.md). This plan defines the next implementation slice: an explicit store-backed discovery wrapper that loads already-persisted `SideEffectRecord` values and delegates deterministic reference discovery to the accepted in-memory helper. Store-backed discovery is implemented in [SideEffect Store-Backed Discovery Report](../concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REPORT.md) and accepted in [SideEffect Store-Backed Discovery Review](../concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REVIEW.md). Automatic report discovery, executor integration, runtime side-effect execution, writes, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, and release posture changes remain unimplemented.

## 1. Executive Summary

Workflow OS now has:

- a validated `SideEffectRecord` model;
- an explicit local `SideEffectRecordStore`;
- an accepted in-memory `discover_side_effect_references` helper;
- proposed/denied/skipped SideEffect workflow event vocabulary and executor append support;
- WorkReport SideEffect citation vocabulary and explicit SideEffect ID propagation.

The next question is how a future caller can discover SideEffect references from already-persisted local records without making report generation automatic or enabling writes.

This plan recommends a small store-backed wrapper that accepts explicit immutable run identity plus optional already-loaded workflow events and explicit IDs, loads records through `SideEffectRecordStore`, and then calls the accepted in-memory helper. The wrapper must not mutate runtime state, append events, create records, execute side effects, write provider state, create report artifacts, expose CLI output, or change workflow semantics.

## 2. Goals

- Add a bounded plan for store-backed SideEffect discovery.
- Reuse the accepted in-memory discovery helper as the validation and deterministic de-duplication boundary.
- Load records only through the explicit `SideEffectRecordStore` contract.
- Preserve full immutable run identity: workflow ID, workflow version, schema version, spec hash, and run ID.
- Keep discovery reference-only.
- Keep workflow events as run-history source of truth.
- Keep SideEffect records as durable side-effect intent/lifecycle references.
- Fail closed on corrupt, missing, or identity-mismatched records when records are required.
- Return structured non-leaking discovery errors.
- Prepare for later WorkReport integration without implementing it.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- automatic WorkReport side-effect discovery;
- executor integration;
- report artifact integration;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage enforcement;
- runtime side-effect execution;
- attempted/completed/failed executor behavior;
- write-capable GitHub, Jira, CI, HTTP, local filesystem, or generic adapters;
- provider mutation;
- record creation, repair, update, or lifecycle transition;
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

## 4. Current Baseline

Implemented persistence and discovery surfaces:

- `SideEffectRecordStore::write_side_effect_record`;
- `SideEffectRecordStore::read_side_effect_record`;
- `SideEffectRecordStore::list_side_effect_records`;
- `SideEffectRecordStore::list_side_effect_records_for_workflow_run`;
- local filesystem storage for validated `SideEffectRecord` values;
- in-memory test backend support for the store trait;
- deterministic record ordering by `SideEffectId`;
- duplicate ID rejection;
- full immutable run identity conflict rejection on writes and workflow/run listing;
- `discover_side_effect_references` for explicit IDs, already-loaded workflow events, and already-loaded records.

The store API currently lists by workflow ID and run ID, then relies on each record to carry workflow version, schema version, and spec hash. The store-backed discovery wrapper must therefore pass loaded records through the in-memory helper so full immutable identity is checked again before references are returned.

## 5. Recommended Wrapper Shape

Add the smallest explicit API, likely:

- `SideEffectStoreBackedDiscoveryInput`
- `discover_side_effect_references_from_store`

Candidate shape:

```rust
pub struct SideEffectStoreBackedDiscoveryInput {
    pub workflow_id: WorkflowId,
    pub workflow_version: WorkflowVersion,
    pub schema_version: SchemaVersion,
    pub spec_hash: SpecContentHash,
    pub run_id: WorkflowRunId,
    pub explicit_side_effect_ids: Vec<SideEffectId>,
    pub workflow_events: Vec<WorkflowRunEvent>,
    pub require_records: bool,
}

pub fn discover_side_effect_references_from_store(
    store: &impl SideEffectRecordStore,
    input: &SideEffectStoreBackedDiscoveryInput,
) -> Result<SideEffectDiscoveryResult, WorkflowOsError>
```

The wrapper should:

1. Load records through `list_side_effect_records_for_workflow_run(&workflow_id, &run_id)`.
2. Build `SideEffectDiscoveryInput` with the explicit IDs, workflow events, loaded records, immutable identity, and record requirement.
3. Call `discover_side_effect_references`.
4. Return the helper result without creating records or citations.

The wrapper should not read hidden global state, construct a state backend, infer workflow identity from filesystem paths, or require `LocalStateBackend` specifically. It should depend on the `SideEffectRecordStore` trait only.

## 6. Source Priority And De-Duplication

Source priority should remain:

1. explicit caller-supplied SideEffect IDs;
2. accepted workflow events supplied by the caller;
3. records loaded from `SideEffectRecordStore`.

This preserves the accepted in-memory helper behavior and keeps caller intent explicit. Duplicate IDs should continue to de-duplicate deterministically, with output ordered by SideEffect ID and source attribution retaining first-source priority.

Changing source priority should require a separate review because it affects report citation semantics.

## 7. Store Loading Policy

The first store-backed wrapper should use:

- `list_side_effect_records_for_workflow_run(&workflow_id, &run_id)`

The wrapper should not use `read_side_effect_record` for every explicit/event-discovered ID in the first slice. Listing the run keeps the implementation small, deterministic, and consistent with existing local backend behavior.

If the store returns no records:

- discovery may still return explicit/event references when `require_records` is false;
- discovery must fail closed when `require_records` is true and discovered IDs have no matching record;
- discovery may return no references when no explicit IDs, events, or records exist.

If the store returns corrupt or identity-mismatched records, discovery must fail closed and must not silently omit those records.

## 8. Identity And Validation Policy

The wrapper must preserve two identity checks:

1. Store-level workflow/run identity check through `list_side_effect_records_for_workflow_run`.
2. Helper-level full immutable identity check through `discover_side_effect_references`.

The second check is required because the store listing API currently accepts workflow ID and run ID, while workflow version, schema version, and spec hash live inside each record.

The wrapper must not trust:

- filesystem paths;
- index files alone;
- report text;
- event prose;
- audit projections;
- adapter telemetry payloads;
- natural-language summaries.

## 9. Error Handling

Errors must remain stable and non-leaking.

Recommended wrapper behavior:

- propagate accepted in-memory helper errors when they are already non-leaking;
- map store corrupt-read errors to a stable discovery-layer corrupt/read error if needed;
- preserve non-leaking store identity mismatch errors or map them to `side_effect_discovery.identity_mismatch`;
- keep missing required records as `side_effect_discovery.record_missing`;
- do not include SideEffect IDs, workflow IDs, run IDs, workflow versions, schema versions, spec hashes, target references, provider payloads, command output, snippets, paths, credentials, tokens, or raw record JSON in error messages.

Candidate stable codes:

- `side_effect_discovery.identity_mismatch`;
- `side_effect_discovery.record_missing`;
- `side_effect_discovery.record_corrupt`;
- `side_effect_discovery.store_read_failed`.

Discovery failure must remain separate from workflow execution failure. A later report integration may surface a report-generation error, not a misleading user project diagnostic.

## 10. Runtime And State Boundary

The store-backed wrapper must not:

- append workflow events;
- mutate `WorkflowRun`;
- mutate `RunSnapshot`;
- create or update `SideEffectRecord` values;
- repair corrupt records;
- emit audit events;
- emit observability events;
- execute side effects;
- call adapters or providers;
- write report artifacts;
- create files beyond store reads already performed by the caller-provided store implementation;
- expose CLI output;
- change executor pass/fail behavior.

The wrapper may read records through the explicit store trait only.

## 11. WorkReport Integration Boundary

WorkReport integration remains deferred.

Future report integration should:

- call the store-backed wrapper only from an explicitly scoped report-generation path;
- cite discovered SideEffect IDs through existing `WorkReportCitation` constructors;
- not recreate `SideEffectRecord` values;
- not copy SideEffect summaries, reasons, target references, provider payloads, command output, record JSON, or raw store data into report sections;
- preserve report-generation errors separately from workflow execution errors;
- keep absent optional discovery as explicit section text.

The first store-backed discovery implementation should not modify `LocalExecutor::execute_with_report(...)`.

## 12. EvidenceReference Boundary

EvidenceReference side-effect attachment remains deferred.

Store-backed discovery should not:

- create EvidenceReference values;
- attach EvidenceReference values to SideEffect records;
- treat SideEffect records as evidence payload storage;
- infer evidence from store presence;
- cite raw provider payloads or command outputs.

EvidenceReference side-effect attachment needs separate planning after store-backed discovery is reviewed.

## 13. Privacy And Redaction

Store-backed discovery must remain reference-only.

The wrapper must not store, copy, serialize, debug-print, or include in errors:

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

If the wrapper adds Debug output for a new input type, it should expose counts and high-level booleans only, redacting IDs and immutable identity values.

## 14. Test Plan

Future implementation tests should cover:

- store-backed discovery returns records already persisted for the requested workflow/run;
- explicit IDs and store records are merged deterministically;
- workflow event IDs and store records are merged deterministically;
- explicit IDs win source priority over event and record sources for duplicate IDs;
- workflow event source wins over record source for duplicate IDs when no explicit ID exists;
- empty store plus optional records returns explicit/event references and missing count;
- empty store plus required records fails closed when references lack records;
- corrupt store record fails closed without leaking payloads;
- store workflow/run identity mismatch fails closed without leaking values;
- helper-level workflow version mismatch fails closed without leaking values;
- helper-level schema version mismatch fails closed without leaking values;
- helper-level spec hash mismatch fails closed without leaking values;
- attempted/completed/failed events remain unsupported and counted;
- non-SideEffect events are ignored after identity validation;
- wrapper does not create records;
- wrapper does not append workflow events;
- wrapper does not mutate snapshots;
- wrapper does not emit audit or observability events;
- wrapper does not call providers or adapters;
- wrapper does not create report artifacts;
- Debug output does not leak IDs, identity values, store paths, target references, or secret-like values;
- existing SideEffect, SideEffect discovery, SideEffect store, WorkReport, EvidenceReference, executor, runtime, adapter telemetry, and local check tests continue to pass.

## 15. Proposed Implementation Sequence

Implemented sequence:

1. Add a small store-backed discovery input type and wrapper function. Completed.
2. Load records through `SideEffectRecordStore::list_side_effect_records_for_workflow_run`. Completed.
3. Delegate to `discover_side_effect_references`. Completed.
4. Add focused in-memory backend tests and local backend tests. Completed.
5. Document the implementation and create an end-of-phase report. Completed in [SideEffect Store-Backed Discovery Report](../concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REPORT.md).
6. Review store-backed discovery before any WorkReport integration. Completed in [SideEffect Store-Backed Discovery Review](../concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REVIEW.md).

Do not implement WorkReport integration in the same phase.

## 16. Open Questions

- Should store read errors be propagated as store codes or normalized into discovery codes?
- Should the wrapper expose loaded record count separately from discovery reference count?
- Should `require_records` mean records are required only for discovered explicit/event IDs, or also that at least one record must exist for the run?
- Should future report artifact integrity checks require store-backed SideEffect citation validation?
- Should store-backed discovery later support targeted read-by-ID for explicit IDs to avoid listing all run records?
- Should attempted/completed/failed records require a separate lifecycle reconciliation plan before discovery cites them?

## 17. Final Recommendation

Proceed next to **WorkReport SideEffect discovery integration planning**.

The implementation must still not add automatic WorkReport discovery, executor integration, report artifacts, EvidenceReference side-effect attachment, approval-side-effect linkage, runtime side-effect execution, attempted/completed/failed executor behavior, write-capable adapters, provider mutation, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, rollback/compensation behavior, Level 3/4 autonomy, or release posture changes.
