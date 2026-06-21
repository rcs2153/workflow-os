# SideEffect Persistence And Discovery Plan

Status: First persistence slice implemented and blocker fix reviewed. `SideEffectRecordStore` and local filesystem persistence for validated `SideEffectRecord` values are implemented and documented in [SideEffect Record Store Report](../concepts/SIDE_EFFECT_RECORD_STORE_REPORT.md). The immutable run identity blocker found in [SideEffect Record Store Review](../concepts/SIDE_EFFECT_RECORD_STORE_REVIEW.md) is fixed in [SideEffect Record Store Blocker Fix Report](../concepts/SIDE_EFFECT_RECORD_STORE_BLOCKER_FIX_REPORT.md) and accepted in [SideEffect Record Store Blocker Fix Review](../concepts/SIDE_EFFECT_RECORD_STORE_BLOCKER_FIX_REVIEW.md). Concrete discovery planning is documented in [SideEffect Discovery Plan](side-effect-discovery-plan.md), and the first explicit in-memory discovery helper is implemented in [SideEffect Discovery Helper Report](../concepts/SIDE_EFFECT_DISCOVERY_HELPER_REPORT.md) and accepted in [SideEffect Discovery Helper Review](../concepts/SIDE_EFFECT_DISCOVERY_HELPER_REVIEW.md). Store-backed discovery planning is documented in [SideEffect Store-Backed Discovery Plan](side-effect-store-backed-discovery-plan.md), and the implementation is documented in [SideEffect Store-Backed Discovery Report](../concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REPORT.md) and accepted in [SideEffect Store-Backed Discovery Review](../concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REVIEW.md). WorkReport SideEffect discovery integration planning is documented in [WorkReport SideEffect Discovery Integration Plan](work-report-side-effect-discovery-integration-plan.md). This plan does not implement runtime side-effect execution, writes, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 1. Executive Summary

Workflow OS now has the first SideEffect foundations:

- a domain-neutral `SideEffectRecord` model;
- WorkReport citation vocabulary for SideEffect IDs;
- terminal report helper and executor report input propagation for explicitly supplied SideEffect IDs;
- model-only SideEffect workflow event vocabulary;
- bounded generic audit projection for SideEffect workflow events;
- explicit local executor append support for `SideEffectProposed`, `SideEffectDenied`, and `SideEffectSkipped`.

The next question is not how to perform writes. The first persistence slice answered where validated SideEffect records can live locally. The remaining question is how those records should be discovered by reports or future evidence attachments, and how the source-of-truth boundaries stay clear before attempted/completed/failed execution semantics exist.

This plan defines the persistence and discovery boundary. It intentionally keeps automatic discovery, runtime side-effect execution, write-capable adapters, provider mutation, schemas, CLI rendering, examples, and release posture changes out of scope.

## 2. Goals

- Define a future persistence boundary for validated `SideEffectRecord` values.
- Preserve workflow events as the source of truth for workflow run history.
- Preserve `SideEffectRecord` as the future source of truth for side-effect intent, authority, lifecycle, idempotency, target, outcome, and references.
- Define deterministic local discovery sources for future WorkReport and EvidenceReference integration.
- Avoid treating audit projections, report prose, debug output, local check output, command output, adapter payloads, or natural-language summaries as authoritative side-effect history.
- Keep discovery reference-first and redaction-safe.
- Prepare for future attempted/completed/failed lifecycle records without enabling them.
- Preserve idempotency and replay semantics before any external mutation exists.

## 3. Non-Goals

This plan does not authorize:

- runtime side-effect execution;
- `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` executor behavior;
- provider mutation;
- write-capable GitHub, Jira, CI, HTTP, local filesystem, or generic adapter behavior;
- additional SideEffect store behavior beyond the first explicit local `SideEffectRecordStore` slice;
- automatic report discovery implementation;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage implementation;
- workflow-declared side-effect configuration;
- runtime side-effect configuration;
- workflow schema fields;
- CLI commands or rendering;
- example updates;
- hosted or distributed runtime behavior;
- reasoning lineage;
- rollback or compensation behavior;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. Current Baseline Inventory

Implemented SideEffect baseline:

- `SideEffectRecord` and related validated model types are implemented.
- WorkReport citation target vocabulary can cite SideEffect IDs.
- Terminal report helper inputs can accept explicit SideEffect IDs.
- Executor report-bearing inputs can propagate explicit SideEffect IDs.
- `SideEffectWorkflowEvent` supports proposed, denied, skipped, attempted, completed, and failed vocabulary as model-only workflow event payloads.
- Generic audit projection can project SideEffect workflow events as bounded reference-only audit events.
- `LocalExecutionRequest` can accept explicit SideEffect event inputs.
- `LocalExecutor::execute(...)` can append `SideEffectProposed`, `SideEffectDenied`, and `SideEffectSkipped` before local skill invocation.
- `SideEffectRecordStore` is implemented as an explicit record persistence contract.
- `LocalStateBackend` implements local filesystem persistence for validated `SideEffectRecord` values.
- SideEffect records can be read by `SideEffectId`, listed by run ID, and listed by workflow/run identity.
- Duplicate SideEffect IDs and workflow/run identity conflicts are rejected.
- SideEffect record persistence does not mutate workflow events, snapshots, audit records, reports, providers, or runtime state.

Unimplemented SideEffect baseline:

- no automatic SideEffect discovery;
- no report discovery from workflow events, audit projections, or the SideEffect store;
- no EvidenceReference side-effect attachment;
- no attempted/completed/failed executor append behavior;
- no runtime side-effect execution;
- no write-capable adapters;
- no schemas, CLI behavior, examples, hosted runtime behavior, or release posture change.

## 5. Source-Of-Truth Boundaries

Workflow OS should keep the following boundaries explicit:

| Surface | Source-of-truth role |
| --- | --- |
| `WorkflowRunEvent` | Authoritative workflow run history and replay input. |
| `RunSnapshot` | Projection that can be rebuilt from events. |
| `AuditEvent` | Governance and operational projection, not side-effect source of truth. |
| `SideEffectRecord` | Future source of truth for side-effect intent, authority, lifecycle, target, idempotency, references, and outcome. |
| `WorkReport` | Governed handoff artifact that cites side-effect records and discloses side-effect posture. |
| `EvidenceReference` | Citation pointer, not payload storage and not side-effect record storage. |
| Adapter telemetry | Adapter invocation/outcome summary, not side-effect authority source. |

Absence of a SideEffect record must not be used to imply safety, non-occurrence, completion, or approval.

## 6. Candidate Persistence Model

The first implementation added a SideEffect store boundary without enabling runtime mutation.

Implemented Rust concepts:

- `SideEffectRecordStore`

Implemented operations:

- create a validated `SideEffectRecord`;
- read a record by `SideEffectId`;
- list records by `RunId`;
- list records by `WorkflowId` and `RunId`;
- reject duplicate `SideEffectId`;
- reject workflow/run identity mismatch;
- reject invalid serialized records during reads;
- return records in deterministic order.

Explicitly deferred:

- record mutation/update APIs;
- attempted/completed/failed lifecycle transitions;
- idempotency-store integration for external mutation;
- adapter write routing;
- provider outcome ingestion;
- schema exposure;
- CLI inspection;
- production database backends.

## 7. Relationship To StateBackend

The first persistence implementation should keep SideEffect storage explicit and local-first.

Recommended direction:

- Add a dedicated `SideEffectRecordStore` interface.
- Keep it separate from workflow event append.
- Consider adding it to the aggregate `StateBackend` only after the store contract is reviewed.
- For the local filesystem backend, use a separate directory such as `side_effects/`.
- Do not read SideEffect records during normal workflow run rehydration.
- Do not let SideEffect records mutate run snapshots or terminal status.

This mirrors the current report artifact posture: explicit records are useful, but they are not workflow events and are not part of normal run replay.

## 8. Discovery Model

Future SideEffect discovery should be deterministic and bounded.

Candidate discovery sources:

1. Explicit SideEffect IDs supplied to report generation.
2. SideEffect workflow events in accepted workflow run history.
3. Persisted `SideEffectRecord` values in a SideEffect store.
4. Audit events that cite SideEffect IDs, as projection-only hints.
5. Adapter telemetry references only when they cite stable SideEffect IDs.

Rejected discovery sources:

- report prose;
- debug strings;
- raw local check output;
- raw command output;
- raw provider payloads;
- raw CI logs;
- raw Jira bodies or GitHub file contents;
- parser payloads;
- natural-language summaries;
- environment variables;
- credentials or tokens.

Recommended first discovery policy:

- Use explicit SideEffect IDs first.
- Add store-backed lookup only after a SideEffect store exists.
- Treat workflow events as a bounded discovery source for SideEffect IDs, not full records.
- Do not treat audit projections as authoritative.
- If a discovered ID has no matching record where a record is required, fail the report-discovery step with a stable non-leaking error.

## 9. Identity And Indexing Requirements

A persisted SideEffect record should be indexable by:

- `SideEffectId`;
- `WorkflowId`;
- `RunId`;
- `WorkflowVersion`;
- `SchemaVersion`;
- `SpecContentHash`;
- optional `StepId`;
- optional `SkillId`;
- optional `SkillVersion`;
- lifecycle state;
- target kind;
- capability;
- authority decision;
- idempotency key;
- correlation ID where available.

Indexes must store stable references only. They must not store raw target payloads, provider payloads, command output, snippets, credentials, or token-like strings.

## 10. Lifecycle And Reconciliation Policy

Proposed, denied, and skipped records can exist before runtime mutation exists because they do not imply an attempted write.

Attempted, completed, and failed records should remain deferred until runtime side-effect execution, adapter write contracts, provider outcome references, and idempotency replay semantics are planned.

Future reconciliation rules should answer:

- whether a `SideEffectProposed` workflow event must refer to an existing persisted record;
- whether a persisted proposed record may be created before its workflow event is appended;
- how to detect an event that cites a missing or corrupt record;
- how to prevent lifecycle regressions;
- how to reconcile duplicate SideEffect IDs across retry or replay;
- whether denied/skipped records are terminal for that side-effect intent.

Recommended conservative posture:

- Allow proposed/denied/skipped records in the first persistence implementation.
- Keep attempted/completed/failed persisted records model-representable but do not have the executor create them yet.
- Do not infer lifecycle transitions from event order unless separately reviewed.

## 11. Idempotency Policy

SideEffect workflow events already require idempotency keys. For disclosure-only events, those keys prevent ambiguous workflow event histories.

Future persisted SideEffect records should also carry an idempotency binding. Before any write-capable phase, the binding must be hardened around:

- workflow/run identity;
- step and skill context;
- side-effect target;
- requested capability;
- lifecycle transition;
- actor or system actor;
- provider operation reference where available.

Duplicate idempotency keys must not reattempt mutation. Duplicate reads should return stable non-secret references to prior records or outcomes.

## 12. Report Discovery Policy

Future WorkReport generation should cite SideEffect IDs without recreating SideEffect records.

Report discovery should:

- cite explicit SideEffect IDs supplied by the caller;
- optionally cite SideEffect IDs from workflow event history after a discovery phase is implemented;
- optionally resolve persisted records from a SideEffect store after persistence exists;
- use explicit not-available section text when discovery is unavailable;
- fail report-discovery validation when a required SideEffect record is missing or corrupt;
- keep report generation errors separate from workflow execution errors.

Report discovery must not:

- create SideEffect records;
- create EvidenceReference values;
- create approval or policy decisions;
- fabricate missing IDs;
- copy raw payloads into report sections;
- use audit projections as the primary source of truth.

## 13. Evidence, Approval, And Policy Relationships

SideEffect persistence should remain compatible with future EvidenceReference, approval, and policy work.

Future SideEffect records may reference:

- policy decisions;
- approval decisions;
- audit events;
- workflow events;
- adapter telemetry;
- local check results;
- EvidenceReference IDs;
- WorkReport IDs;
- typed handoffs.

Deferred work:

- EvidenceReference side-effect attachment;
- approval evidence attachment;
- approval-side-effect linkage enforcement;
- policy-declared side-effect requirements;
- report artifact referential integrity checks for SideEffect records.

Approval remains authority context. Approval is not a side-effect lifecycle state.

## 14. Privacy And Redaction

SideEffect persistence and discovery must not store or copy:

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

Persistence records, indexes, discovery errors, Debug output, serialization, deserialization errors, audit projections, and report citations must remain redaction-safe and bounded.

## 15. Runtime Semantics And Failure Behavior

The persistence/discovery layer must not change workflow pass/fail semantics by itself.

Recommended behavior:

- If SideEffect record creation is explicitly requested and validation fails, fail that scoped API call with a stable non-leaking error.
- If SideEffect discovery fails during report generation, return a report-generation error separate from workflow execution.
- If a persisted SideEffect record is corrupt, fail closed on read and do not silently omit it.
- Do not convert SideEffect persistence failures into misleading user project diagnostics.
- Do not append workflow events as a side effect of report discovery.
- Do not mutate snapshots, approvals, audit sinks, artifact stores, or provider state during discovery.

## 16. Test Plan

Future implementation tests should cover:

- valid proposed record persistence;
- valid denied record persistence;
- valid skipped record persistence;
- duplicate `SideEffectId` rejection;
- workflow/run identity mismatch rejection;
- deterministic list-by-run ordering;
- invalid serialized record fails closed without leaking values;
- secret-like target/reference/summary/redaction metadata rejection;
- no raw provider/spec/command/parser payload copied;
- no credentials or token-like values serialized;
- read by `SideEffectId`;
- list by `RunId`;
- list by workflow/run identity;
- SideEffect store does not mutate workflow events;
- SideEffect store does not mutate snapshots;
- SideEffect store does not append audit events;
- report discovery from explicit SideEffect IDs remains unchanged;
- future store-backed report discovery cites existing IDs without recreating records;
- audit projection is not treated as authoritative source of truth;
- existing WorkReport, EvidenceReference, executor, runtime, adapter telemetry, and local check tests continue to pass.

## 17. Proposed Implementation Sequence

Recommended small phases:

1. Add `SideEffectRecordStore` core trait and in-memory test contract, model/store only. Completed.
2. Add local filesystem SideEffect record persistence for validated records. Completed.
3. Add focused persistence tests and redaction tests. Completed.
4. Review SideEffect persistence before discovery. Completed.
5. Add SideEffect discovery planning for WorkReport generation. Completed in [SideEffect Discovery Plan](side-effect-discovery-plan.md).
6. Add explicit in-memory discovery helper for already-loaded inputs. Completed in [SideEffect Discovery Helper Report](../concepts/SIDE_EFFECT_DISCOVERY_HELPER_REPORT.md) and accepted in [SideEffect Discovery Helper Review](../concepts/SIDE_EFFECT_DISCOVERY_HELPER_REVIEW.md).
7. Review discovery helper before store-backed discovery. Completed in [SideEffect Discovery Helper Review](../concepts/SIDE_EFFECT_DISCOVERY_HELPER_REVIEW.md).
8. Plan store-backed SideEffect discovery. Completed in [SideEffect Store-Backed Discovery Plan](side-effect-store-backed-discovery-plan.md).
9. Add explicit store-backed SideEffect discovery implementation. Completed in [SideEffect Store-Backed Discovery Report](../concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REPORT.md).
10. Review store-backed discovery before automatic WorkReport discovery, attempted/completed/failed lifecycle, or write-capable adapter planning. Completed in [SideEffect Store-Backed Discovery Review](../concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REVIEW.md).

Implementation started with persistence only, not runtime execution.

## 18. Open Questions

- Should the aggregate `StateBackend` include `SideEffectRecordStore` immediately, or should it remain separate for one phase?
- Should proposed/denied/skipped workflow events require a persisted SideEffect record first?
- Should a proposed SideEffect record be allowed without a matching workflow event?
- Should report discovery prefer workflow events or persisted records when both exist?
- When should report artifacts validate SideEffect citation referential integrity?
- How should duplicate SideEffect IDs be reconciled across replay and idempotency duplicates?
- What is the smallest safe local filesystem layout for SideEffect records?
- Should attempted/completed/failed records require a different persistence API from proposed/denied/skipped?
- When should EvidenceReference side-effect attachment be planned?
- When should approval-side-effect linkage be enforced?

## 19. Final Recommendation

The first implementation phase, **SideEffectRecordStore core trait and local persistence model**, is implemented, and its immutable run identity blocker fix is reviewed.

WorkReport SideEffect discovery helper integration is implemented in [WorkReport SideEffect Discovery Integration Report](../concepts/WORK_REPORT_SIDE_EFFECT_DISCOVERY_INTEGRATION_REPORT.md), following [WorkReport SideEffect Discovery Integration Plan](work-report-side-effect-discovery-integration-plan.md).

Recommended next phase: **WorkReport SideEffect discovery helper integration review**.

Future implementation must still not add runtime side-effect execution, attempted/completed/failed executor behavior, write-capable adapters, provider mutations, automatic report discovery, EvidenceReference side-effect attachment, schemas, CLI behavior, examples, hosted runtime behavior, reasoning lineage, or release posture changes unless separately scoped and approved.
