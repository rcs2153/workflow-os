# SideEffect Lifecycle Transition Plan

Status: Planning complete and accepted in [SideEffect Lifecycle Transition Plan Review](../concepts/SIDE_EFFECT_LIFECYCLE_TRANSITION_PLAN_REVIEW.md). This plan follows the accepted [GitHub PR Comment Provider Write Readiness Plan](github-pr-comment-provider-write-readiness-plan.md). It defines the attempted/completed/failed transition mechanics that must exist before any future live provider mutation. It does not implement transition helpers, runtime side-effect execution, live provider calls, CLI mutation commands, workflow schema changes, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 1. Executive Summary

Workflow OS can now model side-effect intent, persist validated `SideEffectRecord` values, append proposed/denied/skipped SideEffect workflow events through explicit local executor inputs, cite side effects from WorkReports and report artifacts, and validate approval linkage for proposed side-effect records.

The missing pre-write primitive is a reviewed lifecycle transition boundary for the moment a side effect moves from governed intent into provider attempt and outcome. This plan defines that boundary for future implementation. The first implementation should remain pure and local: construct validated transition records and reference-only workflow event payloads from explicit inputs. It must not call providers or mutate external systems.

## 2. Goals

- Define deterministic transition mechanics for `Proposed -> Attempted`, `Attempted -> Completed`, and `Attempted -> Failed`.
- Preserve `SideEffectRecord` as the source of truth for side-effect intent, authority, lifecycle, idempotency, target, references, and outcome.
- Preserve `WorkflowRunEvent` as the source of truth for accepted run-local history.
- Preserve `AuditEvent` as a bounded projection, not a side-effect source of truth.
- Require reviewed authority, idempotency, proposal proof, and redaction posture before attempted transitions.
- Require provider outcome references for completed transitions.
- Require classified non-leaking failure references or stable reason codes for failed transitions.
- Keep transition helpers usable before provider calls exist.
- Prepare for future GitHub PR comment sandbox writes without authorizing them.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- live GitHub pull request comment creation;
- any provider mutation;
- runtime side-effect execution;
- executor automatic attempted/completed/failed append behavior;
- automatic provider calls from `LocalExecutor::execute(...)`, report generation, artifact writing, validation, first-run, or scaffolding;
- CLI mutation commands or rendering;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- production credential management;
- RBAC, IdP, SSO, SCIM, enterprise admin controls, or quorum approval;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented foundations:

- `SideEffectRecord` validates lifecycle state, authority compatibility, capability compatibility, outcome reference requirements, reason-code requirements, bounded summaries, references, and redaction metadata.
- `SideEffectLifecycleState` includes `Proposed`, `Attempted`, `Completed`, `Denied`, `Skipped`, and `Failed`.
- `SideEffectRecordStore` persists validated records locally and can read/list records by stable IDs.
- Workflow event vocabulary can represent `SideEffectAttempted`, `SideEffectCompleted`, and `SideEffectFailed` as model-only event kinds.
- Generic audit projection can project SideEffect workflow events without copying provider payloads.
- Existing executor append paths intentionally reject attempted/completed/failed behavior.
- Store-backed discovery treats attempted/completed/failed workflow events as unsupported until runtime execution semantics are reviewed.

Still missing:

- a transition helper that derives attempted/completed/failed records from a prior record;
- transition precondition validation across prior/current records;
- idempotency replay rules for attempted/completed/failed;
- provider outcome reference classification rules;
- executor integration rules for when lifecycle events may be appended;
- runtime failure semantics for provider attempts.

## 5. Source-Of-Truth Boundary

The future implementation should keep two separate facts:

1. `SideEffectRecord` is the authoritative side-effect state record.
2. `WorkflowRunEvent` records that a lifecycle moment was accepted into a run history.

A completed workflow event without a matching completed record must not be treated as proof of provider success. A completed record without an accepted workflow event may be valid local side-effect state, but it is not run-local event proof until appended. Report artifact gates that require both must validate both explicitly.

Audit projection remains derived from accepted workflow events. WorkReports and report artifacts cite records and events; they do not create lifecycle state.

## 6. Transition Invariants

All future transition helpers must enforce:

- transition input names the prior `SideEffectId`;
- workflow/run identity is preserved;
- target, capability, authority context, idempotency binding, and core references are preserved unless a reviewed rule allows additions;
- lifecycle never regresses;
- proposed/denied/skipped pre-attempt outcomes are not reclassified after the fact;
- attempted/completed/failed require allowed authority;
- attempted/completed/failed cannot use `Unknown` capability;
- completed requires a validated outcome reference;
- failed requires a validated failure outcome reference or stable reason code;
- errors use stable codes and do not include raw targets, tokens, bodies, provider payloads, snippets, paths, or secret-like strings.

## 7. Proposed To Attempted

`Proposed -> Attempted` means Workflow OS has crossed the boundary from governed intent into a provider attempt boundary. In the first helper implementation, it should only construct the record/event that would surround a future provider call; it must not perform the call.

Required preconditions:

- prior record exists and is `Proposed`;
- prior record validates;
- prior record has allowed authority;
- any required policy, approval, high-assurance approval, and proposed-event proof references are present in the explicit input;
- idempotency binding is valid and not known to conflict;
- actor or system actor is present;
- transition timestamp is present;
- redaction metadata is valid.

Allowed output:

- an `Attempted` `SideEffectRecord` with the same `SideEffectId`;
- optionally, a reference-only `SideEffectAttempted` workflow event payload for a caller to append later.

Rejected output:

- provider response data;
- raw request body;
- raw target URL;
- credential reference contents;
- automatic workflow event append;
- automatic store mutation unless separately scoped.

## 8. Attempted To Completed

`Attempted -> Completed` means a provider operation returned a validated success reference.

Required preconditions:

- prior record exists and is `Attempted`;
- prior record validates;
- provider outcome reference is present, bounded, validated, and redaction-safe;
- successful outcome reference is stable enough to cite later;
- idempotency binding matches the attempted record;
- workflow/run identity is preserved.

Allowed output:

- a `Completed` `SideEffectRecord` with a validated `SideEffectOutcomeReference`;
- optionally, a reference-only `SideEffectCompleted` workflow event payload for a caller to append later.

The completed transition must not copy raw provider response bodies, raw GitHub issue or pull request text, raw diffs, raw command output, raw CI logs, credentials, or token-like values.

## 9. Attempted To Failed

`Attempted -> Failed` means the provider attempt did not produce a validated success reference.

Required preconditions:

- prior record exists and is `Attempted`;
- prior record validates;
- failure is classified with a stable non-leaking reason code and, where available, a bounded failure outcome reference;
- idempotency binding matches the attempted record;
- workflow/run identity is preserved.

Failure reason taxonomy should initially cover:

- `provider.auth_failed`;
- `provider.permission_denied`;
- `provider.rate_limited`;
- `provider.not_found`;
- `provider.validation_failed`;
- `provider.network_failed`;
- `provider.timeout`;
- `provider.unknown_failed`.

Allowed output:

- a `Failed` `SideEffectRecord` with stable reason code and optional non-leaking outcome reference;
- optionally, a reference-only `SideEffectFailed` workflow event payload for a caller to append later.

Failed transitions must not leak raw error bodies, response payloads, URLs, headers, tokens, snippets, file paths, command output, or secret-like values.

## 10. Denied And Skipped Relationship

`Denied` and `Skipped` remain pre-attempt lifecycle states.

- `Denied` records authority or policy preventing the side effect before attempt.
- `Skipped` records explicit non-execution posture before attempt.
- Neither should transition to `Attempted`, `Completed`, or `Failed`.
- A new side-effect intent should use a new `SideEffectId` rather than rewriting denied/skipped state.

Approval remains authority context, not a lifecycle state. Do not add `Approved` as a SideEffect lifecycle transition.

## 11. Workflow Event Boundary

The first lifecycle implementation should not append events directly. It should return validated event payloads or a transition result that a later executor path may append.

Future executor append rules:

- no SideEffect lifecycle event before `RunStarted`;
- no SideEffect lifecycle event after terminal run state;
- attempted/completed/failed events are state-preserving by themselves;
- if a failed side effect should fail a workflow, append a normal run/step failure event through existing failure semantics after the side-effect failure event;
- fixture or dry-run results must not create attempted/completed/failed events unless explicitly modeled as non-provider transition tests.

## 12. Store Boundary

The first lifecycle helper may accept a prior `SideEffectRecord` directly or read one through an explicit `SideEffectRecordStore`. It should not hide store access inside default executor paths.

Future store-backed behavior should:

- write the new validated record only through explicit caller action;
- reject duplicate lifecycle writes that conflict with existing state;
- return the existing completed/failed record for idempotent replay only when the idempotency binding and outcome match;
- never mutate workflow events or snapshots from the store layer.

## 13. Idempotency And Replay

Before any provider call, idempotency must be deterministic.

Required posture:

- attempted transition binds the side-effect ID, workflow/run identity, target, capability, actor/system actor, and provider operation reference where known;
- duplicate attempt with the same idempotency binding should not blindly reattempt provider mutation;
- completed/failed replay should return stable prior state or fail with a non-leaking idempotency conflict;
- idempotency conflict errors must not reveal provider targets, request bodies, credentials, or provider payloads.

The first implementation can remain conservative: reject duplicate transition attempts unless an exact reviewed replay case is supplied.

## 14. Provider Outcome References

Provider outcome references must be stable and bounded.

For a future GitHub PR comment success, acceptable outcome reference examples:

- provider kind: `github`;
- target class: pull request comment;
- bounded provider record ID or URL-safe opaque reference;
- timestamp;
- optional adapter telemetry reference.

Rejected outcome content:

- raw comment body;
- raw GitHub API response;
- authorization headers;
- token-like values;
- pull request diff/body;
- command output;
- CI logs.

Provider failure references must be even more conservative: stable class and bounded reason only, not raw response text.

## 15. Audit, Report, And Artifact Implications

Lifecycle transitions should become visible through existing governed surfaces only by reference:

- accepted workflow events cite `SideEffectId` and bounded counts/references;
- audit projection summarizes accepted workflow events;
- WorkReports cite SideEffect IDs and disclose attempted/completed/failed posture;
- report artifacts validate referential integrity when required.

Reports must not infer success from provider candidates, fixture validation, or proposed-only records. A future report can describe live write posture only when it has the required record/event references.

## 16. Error Handling And Privacy

Transition errors must be structured and stable.

Candidate error codes:

- `side_effect.transition.prior_missing`;
- `side_effect.transition.invalid_prior_state`;
- `side_effect.transition.identity_mismatch`;
- `side_effect.transition.idempotency_conflict`;
- `side_effect.transition.authority_required`;
- `side_effect.transition.outcome_required`;
- `side_effect.transition.failure_reason_required`;
- `side_effect.transition.provider_reference_invalid`.

Errors must not include raw provider targets, URLs, request bodies, response payloads, command output, paths, tokens, credentials, redaction metadata values, or secret-like strings.

## 17. Test Plan For Future Implementation

Future implementation tests should cover:

- proposed record can transition to attempted with allowed authority;
- denied/skipped records cannot transition to attempted;
- attempted requires allowed authority;
- attempted rejects unknown capability;
- attempted preserves workflow/run identity, target, capability, authority, and idempotency;
- attempted transition can produce reference-only event payload without appending it;
- attempted to completed requires outcome reference;
- attempted to completed preserves identity and idempotency;
- completed transition does not copy provider payloads;
- attempted to failed requires stable reason code or failure outcome reference;
- failed transition does not leak provider errors;
- lifecycle regression is rejected;
- duplicate conflicting transition is rejected;
- idempotent replay behavior is explicit;
- event payloads stay state-preserving and reference-only;
- store-backed transition helper does not mutate workflow events;
- default executor paths still cannot create attempted/completed/failed records or events;
- Debug and serialization do not leak raw targets, provider payloads, command output, paths, tokens, or secret-like values;
- existing SideEffect, report artifact, approval-linkage, provider-candidate, and adapter tests still pass.

## 18. Proposed Implementation Sequence

Recommended phases:

1. **Lifecycle transition plan review**: review this plan before code.
2. **Pure lifecycle transition helper implementation**: create local helpers that accept explicit prior records and transition inputs, returning validated records and optional event payloads. No store writes, executor append, or provider calls.
3. **Store-backed lifecycle transition planning**: define explicit store write/replay behavior after pure helper review.
4. **Store-backed lifecycle transition helper implementation**: write transition records through `SideEffectRecordStore` only when explicitly called.
5. **GitHub PR comment live sandbox helper planning**: only after lifecycle helpers are reviewed, define the provider call boundary.

Do not skip directly to live provider mutation, executor writes, CLI mutation, or schema exposure.

## 19. Open Questions

- Should attempted transition helpers require accepted `SideEffectProposed` event proof, or should that be enforced only at executor/provider-call boundaries?
- Should completed/failed transitions be written with the same `SideEffectId`, or should each transition get a distinct event ID while the record ID remains stable?
- Should provider-visible idempotency markers ever be allowed in comments, or should duplicate prevention remain local only?
- Should failed provider attempts be terminal for the workflow by default, or should future callers choose continuation/failure semantics?
- How should deleted external provider resources be represented after a completed side effect?
- Should first live sandbox tests persist attempted and completed records, or keep lifecycle records in memory until store-backed transition review?

## 20. Final Recommendation

Proceed next to pure SideEffect lifecycle transition helper implementation.

The next implementation should construct validated attempted/completed/failed records and reference-only event payloads from explicit inputs. It must not call providers, append workflow events, write stores, expose CLI behavior, change schemas, update examples, or alter release posture.

## 21. Planning Validation

- `npm run check:docs`: passed.
- Code checks were not run for this planning phase because only documentation is changed.
- Governed planning:
  - workflow: `dg/d`;
  - run: `run-1783262367062724000-2`;
  - approval: `approval/run-1783262367062724000-2/planning-approved`;
  - approval outcome: granted by delegated maintainer;
  - phase closeout: completed;
  - events: 39 total, 1 approval, 0 retries, 0 escalations.
