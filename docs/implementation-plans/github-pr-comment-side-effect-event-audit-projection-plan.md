# GitHub PR Comment Proposed SideEffect Event/Audit Projection Plan

Status: Planning complete, first pure proposed-event construction helper implemented, and explicit append planning documented. This plan follows the accepted GitHub PR comment proposed `SideEffectRecord` persistence helper review. It defines the next safe boundary for projecting a persisted proposed GitHub pull request comment side-effect record into workflow event and audit semantics later. The first helper implementation is documented in [GitHub PR Comment SideEffect Event Helper Report](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_EVENT_HELPER_REPORT.md), accepted in [GitHub PR Comment SideEffect Event Helper Review](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_EVENT_HELPER_REVIEW.md), and the explicit persisted-record-to-executor-input append plan is documented in [GitHub PR Comment Proposed SideEffect Event Append Plan](github-pr-comment-side-effect-event-append-plan.md). It does not implement workflow event appends, audit sink emission, runtime side-effect execution, provider mutation, report artifact writes, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, autonomy expansion, or release posture changes.

## 1. Executive Summary

Workflow OS can now compose and explicitly persist a validated `Proposed` `SideEffectRecord` for the GitHub pull request comment write candidate through a caller-supplied `SideEffectRecordStore`.

That durable record is governed write intent. It is not yet workflow runtime history and it is not proof of provider mutation.

The next question is how a future runtime path should represent that persisted proposed record in the workflow event stream and audit projection without overclaiming execution. This plan keeps the next implementation narrow: construct or append a `SideEffectProposed` workflow event only from an already persisted proposed record, then rely on existing bounded generic audit projection. It does not implement that behavior in this planning phase.

## 2. Goals

- Define how persisted proposed GitHub PR comment records should relate to workflow events.
- Preserve `SideEffectRecord` as the source of truth for write intent.
- Preserve `WorkflowRunEvent` as the source of truth for accepted runtime history.
- Preserve `AuditEvent` as a bounded projection of accepted workflow events.
- Require record-before-event ordering for this provider write candidate.
- Keep projected events reference-only and redaction-safe.
- Avoid implying provider mutation, provider attempt, or provider completion.
- Prepare a small implementation prompt for a future append/projection helper.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- GitHub provider calls;
- GitHub PR comment creation;
- live sandbox writes;
- runtime side-effect execution;
- `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` behavior;
- automatic executor integration;
- automatic workflow event append from default executor paths;
- dedicated side-effect audit sink storage;
- report artifact writes;
- automatic WorkReport mutation;
- CLI commands or rendering;
- workflow schema fields;
- examples;
- hosted or distributed behavior;
- production credential management;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented baseline:

- adapter-neutral write preflight helper;
- model-only GitHub pull request comment request/response boundary;
- preflighted GitHub PR comment write helper;
- fixture-only GitHub PR comment validation helper;
- in-memory proposed `SideEffectRecord` composition helper;
- explicit proposed-record persistence helper through `SideEffectRecordStore`;
- generic `SideEffectWorkflowEvent` vocabulary;
- bounded generic audit projection for accepted SideEffect workflow events;
- explicit local executor append support for caller-supplied `SideEffectProposed`, `SideEffectDenied`, and `SideEffectSkipped` events.

Implemented after this plan:

- pure event construction from an already-loaded persisted proposed record;
- store-backed event construction from an explicit `SideEffectId`;
- lifecycle, target, capability, outcome, and identity validation before event construction.

Not implemented for the GitHub PR comment write candidate:

- automatic event append after proposed-record persistence;
- audit sink emission;
- report artifact citation from the persisted proposed record;
- live GitHub write planning or mutation.

## 5. Source-Of-Truth Boundary

The future implementation must preserve these boundaries:

| Surface | Boundary |
| --- | --- |
| `SideEffectRecord` | Source of truth for proposed GitHub PR comment write intent, target, authority, idempotency, references, sensitivity, and redaction posture. |
| `SideEffectRecordStore` | Persistence boundary for validated records; not runtime history by itself. |
| `WorkflowRunEvent` | Source of truth for accepted runtime history and event ordering. |
| `AuditEvent` | Bounded projection from accepted workflow events. |
| `WorkReport` | Governed handoff artifact that cites stable IDs; not the source of truth for side-effect intent or execution. |

A persisted proposed record without a workflow event means the write intent is durable but has not been accepted into a run event history. A workflow event without a matching persisted record should fail closed for this GitHub PR comment candidate path once the integration is implemented.

## 6. Recommended First Implementation Target

Implemented first target:

```text
compose or append SideEffectProposed from an already persisted GitHub PR comment proposed SideEffectRecord
```

The implemented helper:

- accept an explicit `SideEffectRecordStore`;
- accept a `SideEffectId` or already-loaded `SideEffectRecord`;
- load and validate the record if only an ID is supplied;
- require lifecycle `Proposed`;
- require GitHub PR comment capability and target posture;
- verify workflow/run/schema/spec identity against the active run context;
- build a reference-only `SideEffectWorkflowEvent::Proposed`;
- returns only a validated in-memory `SideEffectWorkflowEvent`;
- does not append events or emit audit records.

Do not support attempted/completed/failed events in this lane.

## 7. Record-Before-Event Ordering

The future implementation should require:

1. validated GitHub PR comment write request;
2. write preflight;
3. preflighted write model;
4. optional fixture/dry-run validation;
5. proposed `SideEffectRecord` composition;
6. proposed record persistence;
7. proposed workflow event construction;
8. optional explicit event append;
9. bounded audit projection from the accepted workflow event.

This ordering avoids event history claiming proposed write intent before the durable record exists.

## 8. Event Payload Policy

The future `SideEffectProposed` event should cite the persisted record by `SideEffectId`.

It may include bounded stable references already supported by `SideEffectWorkflowEvent`, such as:

- step ID;
- skill ID/version;
- policy decision reference;
- approval reference;
- evidence count or stable evidence IDs if already supplied;
- outcome reference count only if zero for proposed records;
- sensitivity;
- redaction metadata.

It must not copy:

- raw GitHub provider payloads;
- PR bodies or diffs;
- generated comment bodies beyond bounded validated summaries already in the record;
- command output;
- CI logs;
- file contents;
- spec contents;
- tokens, credentials, authorization headers, or environment variable values.

## 9. Audit Projection Policy

The first implementation should not add a dedicated GitHub PR comment audit sink.

Instead:

- append or accept a bounded `SideEffectProposed` workflow event;
- project that event through the existing generic audit projection;
- ensure the projected audit event cites the `SideEffectId` and event identity;
- avoid copying the `SideEffectRecord` payload into audit output;
- disclose missing record/event alignment as failure, not as fake evidence.

Dedicated side-effect audit storage can be planned later if generic workflow event projection proves insufficient.

## 10. Error Handling

Future errors must use stable, non-leaking codes.

Recommended code families:

- `github_pr_comment_side_effect_event.record_missing`;
- `github_pr_comment_side_effect_event.record_invalid`;
- `github_pr_comment_side_effect_event.unsupported_lifecycle`;
- `github_pr_comment_side_effect_event.identity_mismatch`;
- `github_pr_comment_side_effect_event.append_failed`;
- `github_pr_comment_side_effect_event.audit_projection_failed`.

Errors must not leak:

- SideEffect IDs;
- run IDs;
- workflow IDs;
- spec hashes;
- repository names;
- pull request numbers;
- idempotency keys;
- summaries;
- target references;
- provider references;
- redaction metadata values;
- raw payloads;
- secret-like values.

## 11. Privacy And Redaction

The projection path should be reference-first and conservative.

It must not store or project:

- raw GitHub tokens;
- authorization headers;
- raw provider payloads;
- raw pull request bodies;
- raw diffs;
- raw CI logs;
- raw command output;
- raw file contents;
- raw spec contents;
- environment variable values;
- unbounded prompts;
- secret-like values.

Debug output, serialized workflow event output, audit projection output, and error messages must remain redaction-safe.

## 12. Test Plan

Future implementation tests should cover:

- persisted proposed GitHub PR comment record can produce a `SideEffectProposed` event;
- lifecycle other than `Proposed` is rejected;
- missing store record fails closed;
- workflow/run/schema/spec identity mismatch fails closed;
- event payload cites the `SideEffectId` without copying the full record;
- generic audit projection cites the event and side-effect reference only;
- no provider mutation occurs;
- no attempted/completed/failed event is produced;
- errors do not leak record IDs, target references, summaries, or secret-like values;
- Debug/serialization do not leak forbidden payloads;
- existing provider-write, SideEffect event, audit projection, store, and executor tests continue to pass.

## 13. Proposed Implementation Sequence

Completed:

1. Add a pure helper that loads or accepts a persisted proposed GitHub PR comment `SideEffectRecord` and constructs a validated `SideEffectWorkflowEvent::Proposed`.
2. Add focused unit tests for lifecycle, identity, redaction, and store-backed loading.

Next:

3. Implement the explicit persisted-record to `LocalExecutionSideEffectEventInput` helper described in [GitHub PR Comment Proposed SideEffect Event Append Plan](github-pr-comment-side-effect-event-append-plan.md).
4. Review the append helper before report artifact citation, automatic discovery, or live sandbox write planning.
5. Defer live sandbox writes until event/audit/report posture is reviewed.

## 14. Deferred Work

- `SideEffectAttempted`, `SideEffectCompleted`, and `SideEffectFailed` semantics.
- Live GitHub provider mutation.
- Runtime side-effect execution.
- Dedicated audit sink persistence.
- Automatic WorkReport/report artifact citation from the proposed record.
- CLI write commands.
- Workflow schema fields.
- Examples.
- Hosted runtime behavior.
- Reasoning lineage.
- Release posture changes.

## 15. Final Recommendation

Proceed next to the explicit persisted-record to `LocalExecutionSideEffectEventInput` helper for GitHub PR comment proposed events.

Do not implement live provider writes, attempted/completed/failed lifecycle transitions, automatic executor behavior, report artifacts, CLI behavior, schemas, examples, hosted behavior, or release posture changes before that helper is implemented and reviewed.
