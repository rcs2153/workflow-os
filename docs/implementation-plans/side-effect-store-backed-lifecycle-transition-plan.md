# SideEffect Store-Backed Lifecycle Transition Plan

Status: Planning complete and accepted in [SideEffect Store-Backed Lifecycle Transition Plan Review](../concepts/SIDE_EFFECT_STORE_BACKED_LIFECYCLE_TRANSITION_PLAN_REVIEW.md). The first store-backed helper implementation is documented in [SideEffect Store-Backed Lifecycle Transition Helper Report](../concepts/SIDE_EFFECT_STORE_BACKED_LIFECYCLE_TRANSITION_HELPER_REPORT.md).

This plan follows the accepted [SideEffect Lifecycle Transition Helper Review](../concepts/SIDE_EFFECT_LIFECYCLE_TRANSITION_HELPER_REVIEW.md). It defines how store-backed attempted/completed/failed `SideEffectRecord` transitions persist through `SideEffectRecordStore` while reusing the pure transition helpers. The first helper implementation is complete. It does not implement executor event append behavior, provider calls, runtime side-effect execution, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, autonomy expansion, or release posture changes.

## 1. Executive Summary

Workflow OS can now construct validated attempted/completed/failed SideEffect transition records and reference-only event payloads without writing state or calling providers.

The next boundary is store-backed transition persistence: when a caller has an existing proposed or attempted `SideEffectRecord`, Workflow OS needs an explicit helper that loads the prior record from a `SideEffectRecordStore`, validates transition preconditions, writes the transitioned record, and returns the same reference-only event payload for a future executor append path.

This plan is still pre-write work. It does not authorize live provider mutation. It defines local store semantics so future provider-write code cannot bypass lifecycle validation or silently overwrite side-effect state.

## 2. Goals

- Add a future explicit helper for store-backed attempted/completed/failed SideEffect lifecycle transitions.
- Reuse the existing pure transition helpers as the only construction path.
- Require explicit `SideEffectId` and transition input.
- Load the prior record from `SideEffectRecordStore`.
- Validate prior lifecycle state before writing.
- Preserve workflow/run identity, target, capability, authority, idempotency, references, sensitivity, and redaction metadata.
- Write exactly one transitioned `SideEffectRecord` on success.
- Return the transitioned record and reference-only `SideEffectWorkflowEvent` payload.
- Define idempotent replay and conflict behavior.
- Fail closed on missing, corrupt, mismatched, or conflicting prior state.
- Keep provider calls, executor event append behavior, and CLI behavior out of scope.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- provider calls;
- live GitHub pull request comment creation;
- runtime side-effect execution;
- executor attempted/completed/failed event append behavior;
- automatic side-effect discovery changes;
- automatic report artifact writing changes;
- CLI mutation commands or rendering;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- production credential management;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented:

- `SideEffectRecord` validation for lifecycle, authority, capability, outcome, reason codes, references, summaries, sensitivity, and redaction metadata.
- `SideEffectRecordStore` local persistence with validated write/read/list behavior.
- Pure helpers:
  - `transition_side_effect_to_attempted(...)`;
  - `transition_side_effect_to_completed(...)`;
  - `transition_side_effect_to_failed(...)`.
- Reference-only `SideEffectWorkflowEvent` payload construction.
- Executor append paths for `SideEffectProposed`, `SideEffectDenied`, and `SideEffectSkipped`.
- Model vocabulary for `SideEffectAttempted`, `SideEffectCompleted`, and `SideEffectFailed`.
- Audit projection for SideEffect lifecycle events.
- WorkReport/report artifact SideEffect citation and referential integrity helpers.

Not implemented:

- executor append path for attempted/completed/failed lifecycle events;
- provider calls;
- runtime side-effect execution.

## 5. Future Helper Boundary

The future helper should be explicit and store-adjacent, for example:

```rust
transition_side_effect_to_attempted_in_store(...)
transition_side_effect_to_completed_in_store(...)
transition_side_effect_to_failed_in_store(...)
```

or a single typed helper:

```rust
transition_side_effect_lifecycle_in_store(...)
```

The first implementation should prefer the smallest shape that mirrors the pure helpers and stays easy to test.

The helper should:

- accept `&impl SideEffectRecordStore`;
- accept a stable `SideEffectId`;
- accept transition-specific input fields;
- read the prior record from the store;
- validate prior state and identity;
- call the existing pure helper;
- write the transitioned record through `SideEffectRecordStore`;
- return a result containing the transitioned record and reference-only event payload.

The helper must not:

- call a provider;
- append workflow events;
- mutate `WorkflowRun`;
- emit audit records;
- write report artifacts;
- inspect credentials;
- create CLI output.

## 6. Store Read Policy

Store-backed transition helpers should fail closed when:

- the prior record is missing;
- the prior record cannot be read;
- the prior record fails validation;
- the prior record lifecycle state is incompatible;
- the prior record identity does not match optional caller-supplied workflow/run identity;
- the prior record authority cannot support attempted/completed/failed state;
- the store contains a corrupt record.

Candidate stable error codes:

- `side_effect.transition.prior_missing`;
- `side_effect.transition.store_read_failed`;
- `side_effect.transition.prior_invalid`;
- `side_effect.transition.invalid_prior_state`;
- `side_effect.transition.identity_mismatch`.

Errors must not leak side-effect IDs, target references, paths, provider payloads, request bodies, tokens, snippets, or secret-like values.

## 7. Store Write Policy

The helper should write only the validated transitioned record returned by the pure helper.

The helper should fail closed when:

- the store write fails;
- the store rejects the record;
- a conflicting record already exists for the same `SideEffectId`;
- replay cannot prove the existing record is equivalent and safe to return.

Candidate stable error codes:

- `side_effect.transition.store_write_failed`;
- `side_effect.transition.conflict`;
- `side_effect.transition.replay_mismatch`.

Store write failure must not append events, fabricate a report citation, or claim provider outcome.

## 8. Idempotency And Replay

The first store-backed helper should be conservative.

Recommended v1 behavior:

- Missing prior record fails closed.
- First valid transition writes the transitioned record.
- Repeating the same transition may return the existing transitioned record only when lifecycle state, workflow/run identity, idempotency binding, target, capability, authority, outcome reference, reason codes, and stable references match exactly.
- Repeating with different outcome, reason codes, references, timestamp posture, target, authority, or idempotency fails closed as a conflict.
- Attempted transitions should not be reclassified into completed or failed without an attempted prior record.
- Completed and failed are terminal lifecycle outcomes for the same side-effect attempt.

This keeps restart behavior safe before provider writes exist.

## 9. Attempted Transition Semantics

Store-backed `Proposed -> Attempted` should:

- read a prior `Proposed` record;
- reject denied or skipped prior records;
- reject already completed or failed records unless exact replay semantics are explicitly satisfied;
- require allowed authority through pure helper/model validation;
- write an `Attempted` record;
- return a `SideEffectAttempted` event payload for future append by an executor path.

It must not perform the provider attempt itself.

## 10. Completed Transition Semantics

Store-backed `Attempted -> Completed` should:

- read a prior `Attempted` record;
- require a valid outcome reference;
- preserve the original side-effect identity and idempotency binding;
- write a `Completed` record;
- return a `SideEffectCompleted` event payload for future append by an executor path.

It must not copy raw provider response payloads, raw GitHub bodies, raw diffs, command output, CI logs, credentials, authorization headers, private keys, token-like values, or paths.

## 11. Failed Transition Semantics

Store-backed `Attempted -> Failed` should:

- read a prior `Attempted` record;
- require a failure outcome reference or stable reason code;
- reject raw provider error bodies;
- write a `Failed` record;
- return a `SideEffectFailed` event payload for future append by an executor path.

Failure classification must be bounded and non-leaking.

## 12. Event Append Boundary

The store-backed helper should return a validated event payload, but it should not append it.

Future executor integration must separately decide:

- when attempted/completed/failed events may be appended;
- how event append is ordered relative to provider attempts and store writes;
- how to handle store write success followed by event append failure;
- whether report artifact gates require record, event, or both;
- how duplicate event append is deduplicated.

This plan intentionally keeps store persistence and workflow event mutation separate.

## 13. Relationship To Provider Writes

Provider writes remain future.

A future live provider path should eventually look like:

1. Persist proposed SideEffect record.
2. Append accepted proposed event.
3. Validate approval/policy/high-assurance posture.
4. Persist attempted transition.
5. Execute provider call.
6. Persist completed or failed transition.
7. Append accepted lifecycle event payloads.
8. Project audit/report posture by reference.

This plan only addresses steps 4 and 6 as explicit store-backed helpers. It does not implement the provider call or executor append behavior.

## 14. Privacy And Redaction

Store-backed transition helpers must preserve the redaction posture of the pure helpers.

The helper must not store or copy:

- raw provider payloads;
- raw provider errors;
- raw request bodies;
- raw GitHub pull request bodies or diffs;
- raw comment bodies beyond already validated bounded request models;
- raw command output;
- raw CI logs;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- environment variable values;
- raw spec contents.

Debug output should expose only bounded counts and lifecycle posture. Serialization should remain limited to validated model fields.

## 15. Test Plan

Future implementation tests should cover:

- proposed record in store transitions to attempted;
- attempted record in store transitions to completed;
- attempted record in store transitions to failed;
- missing prior record fails closed;
- corrupt prior record fails closed without leaking payload;
- denied/skipped prior records cannot transition to attempted;
- completed transition rejects non-attempted prior state;
- failed transition requires failure reference or stable reason code;
- store write failure maps to stable non-leaking error;
- duplicate exact replay returns existing safe state or is explicitly rejected by policy;
- duplicate conflicting replay fails closed;
- identity mismatch fails closed;
- idempotency mismatch fails closed;
- transitioned record is written only through the store;
- workflow events are not appended;
- workflow run/snapshot/event history are not mutated;
- no provider calls occur;
- no report artifacts are written;
- Debug output does not leak IDs, targets, provider payloads, or secret-like values;
- serialization does not copy raw provider payload markers;
- existing pure helper tests continue to pass;
- existing SideEffect store, discovery, report, artifact, approval-linkage, and runtime tests continue to pass.

## 16. Proposed Implementation Sequence

Recommended future phases:

1. Add store-backed transition helper input/result types.
2. Implement attempted transition against `SideEffectRecordStore`.
3. Implement completed and failed transitions.
4. Add focused store/idempotency/privacy tests.
5. Review store-backed helper phase.
6. Plan executor attempted/completed/failed event append behavior.
7. Only after review, consider provider-write sandbox execution planning.

## 17. Open Questions

- Should exact replay return the existing record, or should v1 reject duplicate transitions categorically?
- Should the helper require caller-supplied workflow/run identity to guard against wrong-store reads?
- Should completed and failed transition helpers require an accepted attempted event before writing terminal state?
- Should store-backed transition write and event append ever be atomic, or must they remain separate with reconciliation?
- What is the minimum event append integration needed before a live GitHub sandbox write?
- Should report artifact gates require both record and event references for attempted/completed/failed posture?

## 18. Governed Planning Run

- workflow: `dg/d`;
- run: `run-1783265344229307000-2`;
- approval: `approval/run-1783265344229307000-2/planning-approved`;
- approval outcome: granted by delegated maintainer;
- phase closeout: completed;
- event summary: 39 total events, 1 approval, 0 retries, 0 escalations;
- validation: `npm run check:docs` passed.

## 19. Final Recommendation

Proceed next to a maintainer review of this plan.

If accepted, the next implementation should be store-backed SideEffect lifecycle transition helpers only. It should not add provider calls, executor event append behavior, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, autonomy expansion, or release posture changes.
