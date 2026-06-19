# SideEffect Workflow Event And Audit Projection Plan

Status: Planning complete. This plan follows accepted SideEffect core model work, WorkReport SideEffect citation vocabulary, terminal report SideEffect citation propagation, and executor SideEffect report input propagation. It defines a conservative event/audit source-of-truth boundary for future SideEffect history. It does not implement event vocabulary, audit projection, executor append behavior, SideEffect persistence, automatic discovery, EvidenceReference side-effect attachment, runtime side-effect execution, write-capable adapters, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 1. Executive Summary

Workflow OS now has:

- a SideEffect core model;
- WorkReport SideEffect citation vocabulary;
- terminal report helper propagation for explicitly supplied `SideEffectId` values;
- executor report input propagation for explicitly supplied `SideEffectId` values.

The remaining gap is source of truth. Explicit report inputs can cite SideEffect IDs, but Workflow OS does not yet have side-effect workflow events, audit projections, persistence, automatic discovery, or runtime side-effect execution.

The next question is how future SideEffect history should become durable and auditable without enabling writes prematurely.

This plan recommends a small first implementation after planning: model-only SideEffect workflow event vocabulary with generic audit projection rules. Executor append behavior, SideEffect persistence, automatic discovery, EvidenceReference side-effect attachment, report artifact referential integrity, and runtime side-effect execution remain later phases.

## 2. Goals

- Define how SideEffect lifecycle events should relate to the workflow event stream.
- Preserve `WorkflowRunEvent` as the source of truth for runtime state transitions.
- Preserve `SideEffectRecord` as the source of truth for side-effect intent, authority, lifecycle, idempotency, outcome references, and related references.
- Preserve `AuditEvent` as a bounded projection of accepted workflow events.
- Define event vocabulary without enabling mutation.
- Define audit projection posture without adding dedicated side-effect audit sink storage.
- Keep future WorkReport automatic discovery grounded in durable event or store semantics rather than report prose.
- Keep event and audit payloads reference-first, bounded, deterministic, and redaction-safe.
- Prepare for future runtime side-effect execution while preserving current no-write posture.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- runtime side-effect execution;
- write-capable adapters;
- provider mutations;
- local filesystem writes through runtime;
- SideEffect record creation by the executor;
- SideEffect record persistence;
- SideEffect workflow event append behavior in the executor;
- SideEffect audit sink storage or dedicated side-effect audit records;
- automatic SideEffect discovery from events, audit records, stores, reports, local checks, hooks, adapter telemetry, or typed handoffs;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage implementation;
- report artifact behavior changes;
- workflow schema fields;
- CLI rendering, commands, or export;
- example updates;
- hosted or distributed runtime behavior;
- reasoning lineage;
- rollback or compensation behavior;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. Current Baseline

Implemented:

- `SideEffectId`;
- `SideEffectRecord`;
- `SideEffectLifecycleState`;
- `SideEffectTargetReference`;
- `SideEffectCapability`;
- `SideEffectAuthority`;
- `SideEffectIdempotencyBinding`;
- `SideEffectReference`;
- `WorkReportCitationTarget::SideEffect`;
- terminal report helper support for supplied SideEffect IDs;
- `LocalExecutionReportInputs.side_effect_ids`;
- executor forwarding of supplied SideEffect IDs into terminal reports.

Existing runtime/audit baseline:

- workflow events are append-only and rebuild `WorkflowRunSnapshot`;
- terminal states reject further workflow events;
- hook workflow event vocabulary exists as a precedent for model-only, state-preserving event vocabulary;
- generic hook workflow event audit projection exists through `AuditEvent::from_workflow_event(...)`;
- the first explicit `BeforeSkillInvocation` hook append path exists, but only after separate planning and review.

Not implemented:

- SideEffect workflow event kinds;
- SideEffect audit projection rules;
- SideEffect persistence;
- SideEffect automatic discovery;
- runtime side-effect execution;
- write-capable adapter behavior.

## 5. Source-Of-Truth Boundaries

SideEffect event/audit work must preserve these boundaries:

- `WorkflowRunEvent`: source of truth for accepted workflow history and run-state projection.
- `WorkflowRunSnapshot`: rebuildable projection from workflow events.
- `SideEffectRecord`: source of truth for side-effect intent, target, requested capability, authority, lifecycle, idempotency, outcome references, sensitivity, and redaction posture.
- `AuditEvent`: bounded projection from accepted workflow events.
- `PolicyAuditRecord`: policy decision audit record, including pre-run decisions.
- `AdapterRuntimeAuditRecord`: adapter invocation audit telemetry.
- `EvidenceReference`: citation pointer, not payload storage.
- `WorkReport`: governed handoff artifact that cites SideEffect IDs but does not become the SideEffect source of truth.

Reports may cite SideEffect IDs. They must not create, resolve, mutate, or infer SideEffect records.

## 6. Event Vocabulary Options

### Option A: SideEffectRecord-Only, No Workflow Events

SideEffect lifecycle is stored only in SideEffect records or a future SideEffect store.

Benefits:

- avoids event-stream expansion;
- keeps SideEffect state in one model;
- avoids accidental run-state changes.

Risks:

- workflow event history would not show when side-effect-relevant decisions affected execution;
- audit projection would require separate SideEffect store semantics;
- automatic report discovery from a run would need another durable source.

### Option B: SideEffect Workflow Event Vocabulary

Add workflow event vocabulary that references already-created SideEffect IDs and lifecycle moments.

Benefits:

- preserves run-local history;
- enables generic audit projection;
- supports later automatic report discovery by event ID;
- aligns with hook event precedent.

Risks:

- event ordering and state transitions must be precise;
- premature executor append behavior could imply write support;
- payloads must stay reference-only to avoid copying SideEffect records.

### Option C: Dedicated SideEffect Audit Sink Only

Add a side-effect audit sink or record type without workflow events.

Benefits:

- clear audit ledger for side-effect activity;
- avoids changing runtime event vocabulary.

Risks:

- duplicates source-of-truth questions;
- requires persistence and retention design;
- can drift from workflow event history.

### Option D: Report-Only Disclosure

Continue explicit SideEffect IDs in WorkReports and defer event/audit work.

Benefits:

- smallest behavior;
- avoids runtime changes.

Risks:

- insufficient before writes;
- no durable event/audit source for automatic discovery;
- absence of citations may be misread as absence of side effects.

## 7. Recommended Posture

Recommended first target after this plan: **SideEffect workflow event vocabulary, model-only, with generic audit projection planning carried into the same implementation prompt only if it remains bounded**.

The implementation should prefer the hook precedent:

1. Add model-only event vocabulary and transition rules.
2. Keep events state-preserving from `Running` unless separately reviewed.
3. Add generic audit projection only for modeled SideEffect event kinds if the projection can remain reference-only and bounded.
4. Do not add executor append behavior yet.
5. Review before any executor integration, persistence, automatic discovery, or runtime side-effect execution.

If the implementation becomes broad, split it:

1. SideEffect workflow event vocabulary model-only.
2. SideEffect generic audit projection.
3. Executor append planning.

## 8. Candidate Workflow Event Vocabulary

Candidate minimal event names:

- `SideEffectProposed`;
- `SideEffectDenied`;
- `SideEffectSkipped`;
- `SideEffectAttempted`;
- `SideEffectCompleted`;
- `SideEffectFailed`.

Do not add `SideEffectApproved` as a lifecycle event. Approval is authority context, not execution.

Do not add `SideEffectRolledBack` or `SideEffectCompensated` until rollback or compensation behavior is separately scoped and adapter-specific proof exists.

## 9. Candidate Event Payload

A future SideEffect workflow event payload should be reference-first.

Recommended fields:

- `side_effect_id`;
- `lifecycle_state`;
- optional `step_id`;
- optional `skill_id`;
- optional `skill_version`;
- optional `policy_decision_reference`;
- optional `approval_reference`;
- optional `adapter_telemetry_reference`;
- optional `evidence_reference_count`;
- optional `outcome_reference_count`;
- `sensitivity`;
- `redaction`;

Potentially deferred fields:

- full SideEffect target reference;
- full authority packet;
- idempotency binding details;
- outcome details;
- reason code list;
- adapter ID and integration ID.

The first implementation should not copy `SideEffectRecord` into workflow events. The event should cite the SideEffect ID and bounded counts or stable references only.

## 10. State Transition Policy

Initial SideEffect workflow events should be state-preserving from `Running`.

Candidate transition posture:

- `SideEffectProposed`: state-preserving from `Running`;
- `SideEffectDenied`: state-preserving from `Running`;
- `SideEffectSkipped`: state-preserving from `Running`;
- `SideEffectAttempted`: state-preserving from `Running`;
- `SideEffectCompleted`: state-preserving from `Running`;
- `SideEffectFailed`: state-preserving from `Running`.

Do not allow SideEffect workflow events after terminal states in the first event vocabulary phase.

Do not allow SideEffect workflow events before `RunStarted`.

Do not introduce new run statuses.

If a future attempted/failed side effect should fail the run, pair it with existing reviewed failure transitions rather than making the SideEffect event itself terminal.

## 11. Event Ordering Policy

Future executor integration must define exact event placement before implementation.

Candidate ordering:

| Future source | Candidate event placement | Initial posture |
| --- | --- | --- |
| proposed side effect before policy | after proposal is validated, before authority allows attempt | Plan later. |
| policy denial | after policy decision is recorded, before run failure/continuation handling | Plan later. |
| approval denial | after approval decision event, before terminal/continuation handling | Plan later. |
| skipped unsupported side effect | before continuing past unsupported operation | Plan later. |
| attempted side effect | after policy/approval authority, before adapter mutation attempt | Defer until writes are scoped. |
| completed side effect | after adapter reports success, before next step scheduling | Defer until writes are scoped. |
| failed side effect | after adapter reports failure, before retry/escalation/failure handling | Defer until writes are scoped. |

The first event vocabulary phase should not choose executor insertion points.

## 12. Idempotency And Replay Policy

Future SideEffect workflow events must require an idempotency key when appended.

Rules:

- duplicate run replay must not create duplicate SideEffect events;
- duplicate side-effect idempotency keys must not reattempt mutation;
- event replay should reconstruct the same run state;
- side-effect record resolution must not require external providers during replay;
- SideEffect events should cite stable IDs, not raw outcomes.

The first event vocabulary implementation should test idempotency-key requirements and replay behavior if event kinds are added.

## 13. Audit Projection Posture

Recommended first audit posture: generic `AuditEvent::from_workflow_event(...)` projection for SideEffect workflow events.

Generic projection should preserve:

- source event ID;
- source timestamp;
- event type;
- workflow ID;
- schema version;
- workflow version;
- run ID;
- spec hash;
- optional step ID;
- optional skill ID/version;
- actor;
- correlation ID;
- idempotency key;
- bounded decision context, such as `side effect proposed` or `side effect denied`;
- bounded input or output reference using the SideEffect ID or count-only reference;
- redaction metadata indicating reference-only side-effect context.

Do not add dedicated SideEffect audit sink methods in the first projection phase.

Do not add SideEffect audit persistence in the first projection phase.

## 14. Audit Action Policy

Do not add new `Action` variants in the first projection implementation unless separately reviewed.

Recommended v1 projection:

- `AuditEvent.action` remains `None` for SideEffect workflow events;
- `event_type` carries the stable SideEffect event kind;
- `decision_context` carries bounded lifecycle vocabulary;
- future policy-gated write phases may introduce action vocabulary when policy semantics require it.

Rationale:

- adding action vocabulary affects policy semantics;
- SideEffect event vocabulary should not imply write authorization;
- policy decisions already carry action/capability semantics separately.

## 15. Relationship To SideEffectRecord

The event should cite a SideEffect ID; it should not embed the full record.

SideEffectRecord remains the source of truth for:

- target reference;
- requested capability;
- authority decision;
- policy and approval references;
- idempotency binding;
- lifecycle-specific validation;
- outcome references;
- summary and reason codes;
- sensitivity and redaction metadata.

Workflow events can make SideEffect moments durable in run history. They should not replace SideEffectRecord validation.

## 16. Relationship To Policy And Approval

SideEffect workflow events must not create policy or approval decisions.

Future events may cite existing policy and approval references when stable IDs already exist. They must not:

- evaluate policy;
- request approvals;
- grant or deny approvals;
- infer approval from credentials;
- infer approval from model review;
- bypass approval gates;
- change autonomy level.

Sensitive side effects should remain blocked until policy and approval semantics are explicitly modeled and reviewed.

## 17. Relationship To EvidenceReference And WorkReport

This plan does not implement EvidenceReference side-effect attachment.

Future WorkReport automatic SideEffect discovery should use reviewed sources such as:

- SideEffect workflow events;
- SideEffect store queries;
- audit projections that cite SideEffect IDs.

Reports must not infer SideEffect history from prose, debug strings, local check output, command output, or adapter payloads.

EvidenceReference side-effect attachment should be planned separately after event/audit and persistence semantics are understood.

## 18. Relationship To Persistence And Artifacts

This plan does not implement SideEffect persistence or report artifact referential integrity.

Future persistence planning must decide:

- whether SideEffect records live in `StateBackend`, a separate store, or artifact-like records;
- how SideEffect records are indexed by run ID;
- whether workflow events are enough for discovery;
- how report artifacts validate SideEffect citation references;
- how corrupt SideEffect records fail closed without leaking payloads.

Until persistence exists, executor report inputs remain explicit caller-supplied references only.

## 19. Privacy And Redaction

SideEffect workflow events and audit projections must not store or copy:

- side-effect target references unless separately reviewed as safe;
- side-effect summaries;
- side-effect reason codes if they could contain sensitive text;
- side-effect authority packets;
- idempotency details beyond stable idempotency key;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded metadata.

Debug output, serialization, deserialization errors, validation errors, audit projection, and docs examples must remain redaction-safe.

## 20. Error Handling

Future implementation errors must use stable codes and avoid rejected values.

Candidate error code families:

- `runtime.side_effect_event.id.required`;
- `runtime.side_effect_event.id.invalid`;
- `runtime.side_effect_event.lifecycle.mismatch`;
- `runtime.side_effect_event.redaction.field.secret_like`;
- `runtime.side_effect_event.redaction.reason.secret_like`;
- `runtime.idempotency_key.missing`;
- `runtime.transition.invalid`.

Invalid serialized event payloads should fail closed and not leak raw SideEffect IDs, target references, paths, snippets, provider payloads, command output, parser output, credentials, tokens, or secret-like strings.

## 21. Future Test Plan

For a future model-only event vocabulary implementation, add tests for:

- SideEffect event kind names are stable and serializable;
- valid SideEffect event payload is bounded and accessible;
- invalid/secret-like SideEffect IDs are rejected without leaking;
- invalid/secret-like redaction metadata is rejected without leaking;
- SideEffect events require idempotency keys;
- SideEffect events rehydrate as state-preserving from `Running`;
- SideEffect events are rejected before `RunStarted`;
- SideEffect events are rejected after terminal states;
- serialization does not copy target references, summaries, reason codes, authority context, idempotency details, raw provider payloads, command output, parser payloads, or secrets.

For a future audit projection implementation, add tests for:

- each modeled SideEffect event projects to a bounded `AuditEvent`;
- projection preserves workflow/run identity;
- projection preserves correlation ID and idempotency key;
- projection does not add action vocabulary unless explicitly scoped;
- projection redaction marks side-effect context as reference-only;
- projection does not emit dedicated SideEffect audit records or observability events unless separately scoped;
- deserialization errors do not leak secret-like values.

For future executor append behavior, add tests only after separate planning:

- exact event ordering;
- replay/idempotency behavior;
- no duplicated SideEffect events on duplicate run ID;
- no provider mutation unless write phase is accepted;
- report discovery cites existing SideEffect IDs without fabricating records.

## 22. Proposed Implementation Sequence

Recommended small phases:

1. SideEffect workflow event vocabulary model-only.
2. SideEffect generic audit projection.
3. SideEffect event/audit projection review.
4. SideEffect persistence planning.
5. SideEffect automatic report discovery planning.
6. SideEffect EvidenceReference attachment planning.
7. Runtime side-effect execution planning.
8. Write-capable adapter planning only after the above are accepted.

If phase 1 and phase 2 are implemented together, keep the implementation strictly bounded to event vocabulary and generic audit projection. Do not add executor append behavior in the same phase.

## 23. Open Questions

- Should SideEffect workflow events cite only `SideEffectId`, or also bounded policy/approval references?
- Should `SideEffectProposed` exist before a `SideEffectRecord` is persisted?
- Should denied/skipped SideEffect records be allowed without an attempted operation?
- Should audit projection expose SideEffect ID as `input_reference`, `output_reference`, or decision context?
- Should SideEffect workflow events require a future SideEffect store for referential integrity?
- Should WorkReport automatic discovery use workflow events first or a SideEffect store first?
- Should SideEffect events ever be terminal, or always pair with existing run failure/cancellation events?
- When should report artifacts validate SideEffect citation references?
- When should `Action` vocabulary grow to include side-effect proposal or attempt?
- How should approval-side-effect linkage be represented without treating approval as execution?

## 24. Final Recommendation

Recommended next implementation phase: **SideEffect workflow event vocabulary model-only, with generic audit projection included only if it remains a small bounded slice**.

The implementation must still not add executor append behavior, SideEffect persistence, automatic SideEffect discovery, EvidenceReference side-effect attachment, runtime side-effect execution, writes, write-capable adapters, provider mutations, schemas, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 25. Validation

For this planning phase, run:

- `npm run check:docs`;
- `git diff --check`.

For any future implementation phase, run:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.
