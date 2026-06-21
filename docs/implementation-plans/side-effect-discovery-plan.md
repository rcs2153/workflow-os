# SideEffect Discovery Plan

Status: First in-memory discovery helper and store-backed wrapper implemented and reviewed. This plan defines the first discovery slice for already-existing SideEffect references and records. The helper is implemented and documented in [SideEffect Discovery Helper Report](../concepts/SIDE_EFFECT_DISCOVERY_HELPER_REPORT.md), and accepted in [SideEffect Discovery Helper Review](../concepts/SIDE_EFFECT_DISCOVERY_HELPER_REVIEW.md). Store-backed discovery planning is documented in [SideEffect Store-Backed Discovery Plan](side-effect-store-backed-discovery-plan.md), and the implementation is documented in [SideEffect Store-Backed Discovery Report](../concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REPORT.md) and accepted in [SideEffect Store-Backed Discovery Review](../concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REVIEW.md). WorkReport SideEffect discovery integration planning is documented in [WorkReport SideEffect Discovery Integration Plan](work-report-side-effect-discovery-integration-plan.md). Automatic WorkReport discovery, runtime side-effect execution, writes, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, and release posture changes remain unimplemented.

## 1. Executive Summary

Workflow OS now has the SideEffect foundations needed to plan discovery:

- the `SideEffectRecord` core model;
- WorkReport SideEffect citation vocabulary;
- terminal report and executor report input propagation for explicitly supplied SideEffect IDs;
- SideEffect workflow event vocabulary and bounded audit projection;
- explicit executor append support for `SideEffectProposed`, `SideEffectDenied`, and `SideEffectSkipped`;
- explicit local `SideEffectRecordStore` persistence for validated SideEffect records;
- a reviewed fix for the immutable run identity blocker in the record store.

The next question is how a future report-generation path can discover SideEffect references deterministically without fabricating evidence, changing workflow semantics, or enabling writes.

This plan recommends a narrow first implementation: an in-memory discovery helper that accepts explicit run context plus already-available event/store inputs and returns stable SideEffect IDs or structured not-available/error outcomes. It should not mutate runtime state, append events, create records, execute side effects, or make report generation automatic.

## 2. Goals

- Discover SideEffect IDs from bounded, deterministic sources.
- Preserve workflow events as the source of truth for run history.
- Preserve `SideEffectRecord` as the durable source of truth for SideEffect intent and lifecycle records.
- Keep audit projections as hints only, not authoritative discovery sources.
- Support future WorkReport side-effect section population without copying SideEffect payloads.
- Keep report-generation errors separate from workflow execution results.
- Fail closed on corrupt, missing, or identity-mismatched records when records are required.
- Preserve full immutable run identity: workflow ID, workflow version, schema version, spec hash, and run ID.
- Keep discovery reference-first and redaction-safe.
- Prepare for future attempted/completed/failed lifecycle support without implementing it.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- runtime side-effect execution;
- `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` executor behavior;
- write-capable GitHub, Jira, CI, HTTP, local filesystem, or generic adapter behavior;
- provider mutation;
- automatic WorkReport discovery in the executor;
- automatic EvidenceReference side-effect attachment;
- approval-side-effect linkage enforcement;
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

Implemented:

- `SideEffectRecord` and related model types validate and serialize safely.
- `WorkReportCitationTarget` can cite SideEffect IDs.
- Terminal report helper and executor report inputs can accept explicit SideEffect IDs.
- `SideEffectWorkflowEvent` can represent proposed, denied, skipped, attempted, completed, and failed vocabulary.
- Generic audit projection can project SideEffect workflow events as bounded audit events.
- `LocalExecutor::execute(...)` can append explicit proposed, denied, and skipped SideEffect workflow events before local skill invocation.
- `SideEffectRecordStore` can write, read, and list validated SideEffect records.
- Local `SideEffectRecordStore` persistence rejects duplicate IDs and full immutable run identity conflicts.

Unimplemented:

- automatic SideEffect discovery;
- report discovery from workflow events, audit projections, or the SideEffect store;
- EvidenceReference side-effect attachment;
- attempted/completed/failed executor append behavior;
- runtime side-effect execution;
- write-capable adapters.

## 5. Discovery Source Priority

Future discovery should use this priority order:

1. Explicit SideEffect IDs supplied by the caller.
2. SideEffect IDs present in accepted workflow run events.
3. SideEffect records returned by `SideEffectRecordStore` for the exact run.
4. Audit events only as non-authoritative hints when they cite stable SideEffect IDs.
5. Adapter telemetry only when it cites stable SideEffect IDs.

Rejected discovery sources:

- report prose;
- debug strings;
- raw command output;
- raw local check output;
- raw provider payloads;
- raw CI logs;
- raw Jira bodies or GitHub file contents;
- parser payloads;
- natural-language summaries;
- environment variables;
- credentials or tokens.

## 6. First Implementation Target

Recommended first implementation: an explicit in-memory SideEffect discovery helper. This slice is implemented in `workflow-core`.

Candidate API shape:

- `SideEffectDiscoveryInput`
- `SideEffectDiscoverySource`
- `SideEffectDiscoveryResult`
- `discover_side_effect_references(input: &SideEffectDiscoveryInput) -> Result<SideEffectDiscoveryResult, WorkflowOsError>`

The helper should accept explicit inputs rather than reading hidden global state:

- workflow ID;
- workflow version;
- schema version;
- spec hash;
- run ID;
- optional explicit SideEffect IDs;
- optional workflow run events already loaded by the caller;
- optional SideEffect records already loaded by the caller;
- optional policy for whether stored records are required.

The first helper should not require `StateBackend` directly. Store-backed loading can remain a later wrapper so this first phase can test discovery semantics without introducing runtime coupling.

## 7. Store-Backed Discovery Boundary

After the in-memory helper is reviewed, a future store-backed wrapper may use `SideEffectRecordStore` to load records for a run.

Rules:

- load records by run ID through the store;
- use list-by-workflow/run behavior as an identity guard;
- fail closed if the store reports corrupt records or identity mismatch;
- do not silently omit corrupt records;
- do not create or repair records during discovery;
- do not append workflow events;
- do not mutate snapshots, approvals, audit records, reports, or providers;
- keep discovery failure separate from workflow execution failure.

If the current store API remains workflow ID plus run ID only, the discovery wrapper should still verify full immutable identity on every returned record before producing references.

## 8. Workflow Event Discovery Policy

Workflow events may be used to discover SideEffect IDs when:

- the events belong to the exact immutable run identity;
- the event state transition has already been accepted by runtime validation;
- the event contains a bounded SideEffect workflow event payload;
- the payload exposes a stable SideEffect ID.

Workflow event discovery must not:

- infer SideEffect records from event prose;
- infer lifecycle completion from event ordering beyond the existing event payload;
- create missing records;
- treat attempted/completed/failed runtime semantics as implemented;
- read or copy raw payloads.

For the first implementation, proposed, denied, and skipped events are the only executor-created SideEffect event families. Attempted/completed/failed should remain vocabulary-only unless separate runtime support is accepted.

## 9. WorkReport Integration Policy

Future WorkReport integration should use discovery results as SideEffect ID citations only.

Rules:

- cite SideEffect IDs through existing `WorkReportCitation` constructors;
- do not recreate `SideEffectRecord` values;
- do not copy raw SideEffect summaries, reasons, target references, provider payloads, command output, or record JSON into report sections;
- keep absent optional discovery as explicit section text when discovery is not required;
- return a report-generation error when required discovery fails;
- preserve workflow pass/fail semantics even when report discovery fails.

The first discovery implementation should not wire itself automatically into `LocalExecutor::execute_with_report(...)`. That integration should be a separate reviewed phase.

## 10. EvidenceReference Integration Policy

EvidenceReference side-effect attachment remains deferred.

Future evidence integration should:

- cite SideEffect records by stable reference;
- avoid raw provider payloads and command output;
- preserve `EvidenceReference` as a citation pointer, not a SideEffect record store;
- avoid creating EvidenceReference values implicitly during discovery;
- require separate validation and attachment planning before implementation.

## 11. Error Handling

Discovery errors must use stable codes and must not leak:

- SideEffect IDs when secret-like or invalid;
- workflow/run IDs;
- workflow versions;
- schema versions;
- spec hashes;
- target references;
- provider payloads;
- command output;
- parser payloads;
- paths;
- credentials;
- tokens;
- raw record JSON.

Recommended code families:

- `side_effect_discovery.identity_mismatch`
- `side_effect_discovery.record_missing`
- `side_effect_discovery.record_corrupt`
- `side_effect_discovery.reference_invalid`
- `side_effect_discovery.source_unsupported`

Discovery failure should not become a user project diagnostic unless a later validation phase explicitly designs that behavior.

## 12. Privacy And Redaction

Discovery must remain reference-only.

Discovery must not store, copy, serialize, or debug-print:

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
- unbounded summaries;
- secret-like target identifiers or metadata.

Debug output should expose counts and stable public status only. Serialization, if added, must be bounded and non-leaking.

## 13. Test Plan

Future implementation tests should cover:

- explicit SideEffect IDs are returned deterministically;
- duplicate discovered SideEffect IDs are deduplicated deterministically;
- workflow event SideEffect IDs are discovered from proposed/denied/skipped events;
- attempted/completed/failed events remain vocabulary-only unless runtime support exists;
- records with matching immutable run identity are accepted;
- records with mismatched workflow ID are rejected;
- records with mismatched workflow version are rejected;
- records with mismatched schema version are rejected;
- records with mismatched spec hash are rejected;
- corrupt or missing required records fail closed;
- optional missing records become explicit not-available discovery status if configured as optional;
- audit projections are not treated as authoritative;
- adapter telemetry is not treated as authoritative unless it carries a stable SideEffect ID;
- discovery does not create records;
- discovery does not append workflow events;
- discovery does not mutate snapshots;
- discovery does not write files;
- discovery does not call providers or adapters;
- WorkReport citation construction uses existing constructors in a later integration phase;
- Debug and serialization do not leak secret-like values;
- existing SideEffect, WorkReport, EvidenceReference, executor, runtime, adapter telemetry, and state backend tests continue to pass.

## 14. Proposed Implementation Sequence

Recommended small phases:

1. Add in-memory SideEffect discovery helper model and tests. Completed.
2. Review the in-memory discovery helper. Completed in [SideEffect Discovery Helper Review](../concepts/SIDE_EFFECT_DISCOVERY_HELPER_REVIEW.md).
3. Plan optional store-backed discovery wrapper using `SideEffectRecordStore`. Completed in [SideEffect Store-Backed Discovery Plan](side-effect-store-backed-discovery-plan.md).
4. Add optional store-backed discovery wrapper using `SideEffectRecordStore`. Completed in [SideEffect Store-Backed Discovery Report](../concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REPORT.md).
5. Review store-backed discovery. Completed in [SideEffect Store-Backed Discovery Review](../concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REVIEW.md).
6. Add explicit WorkReport side-effect discovery integration for report generation. Completed in [WorkReport SideEffect Discovery Integration Report](../concepts/WORK_REPORT_SIDE_EFFECT_DISCOVERY_INTEGRATION_REPORT.md).
7. Review report integration before any attempted/completed/failed lifecycle or write-capable adapter work.

Implementation should start with in-memory discovery only.

## 15. Open Questions

- Should discovery deduplicate by first-seen source priority or by deterministic ID ordering?
- Should explicit caller-supplied IDs always win over event/store discovery?
- Should store-backed discovery require full immutable identity inputs in a new API before report integration?
- Should a SideEffect workflow event require a persisted record before report discovery cites it?
- Should absent optional SideEffect records be represented as missing citations or section text?
- Should report artifact integrity checks validate SideEffect citations against the store?
- When should EvidenceReference side-effect attachment be planned?
- When should approval-side-effect linkage become enforceable?
- How much lifecycle reconciliation is needed before attempted/completed/failed execution?

## 16. Final Recommendation

WorkReport SideEffect discovery helper integration is implemented in [WorkReport SideEffect Discovery Integration Report](../concepts/WORK_REPORT_SIDE_EFFECT_DISCOVERY_INTEGRATION_REPORT.md).

Recommended next phase: WorkReport SideEffect discovery helper integration review.

The implementation must still not build runtime side-effect execution, write-capable adapters, provider mutation, automatic executor report discovery, EvidenceReference side-effect attachment, approval-side-effect linkage, workflow schema fields, CLI behavior, examples, hosted behavior, reasoning lineage, rollback/compensation behavior, Level 3/4 autonomy, or release posture changes.
