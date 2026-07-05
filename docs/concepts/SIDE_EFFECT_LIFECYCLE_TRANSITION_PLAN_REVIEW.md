# SideEffect Lifecycle Transition Plan Review

Status: Review complete.

## 1. Executive Verdict

Plan accepted; proceed to pure SideEffect lifecycle transition helper implementation.

The plan is appropriately narrow, pre-write, and aligned with the accepted GitHub PR comment provider write readiness boundary. It closes the planning gap between model-representable lifecycle states and future provider attempt/outcome handling without authorizing live mutation.

## 2. Scope Verification

The plan stayed within planning-only scope.

It did not authorize:

- live GitHub pull request comment creation;
- provider mutation;
- runtime side-effect execution;
- executor automatic attempted/completed/failed append behavior;
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

The plan explicitly positions the next code-bearing work as pure/local transition helpers only.

## 3. Current Baseline Assessment

The plan accurately summarizes the current implemented baseline:

- `SideEffectRecord` can represent `Proposed`, `Attempted`, `Completed`, `Denied`, `Skipped`, and `Failed`.
- Model validation already enforces authority/capability/outcome/reason-code boundaries.
- `SideEffectRecordStore` exists for validated local record persistence.
- SideEffect workflow event vocabulary and generic audit projection exist as reference-first surfaces.
- The current executor append path supports proposed/denied/skipped only.
- Attempted/completed/failed executor behavior remains unsupported.

This is the right reading of the repo. The plan does not mistake model vocabulary for runtime execution.

## 4. Source-Of-Truth Assessment

The plan preserves the essential source-of-truth split:

- `SideEffectRecord` is authoritative side-effect state.
- `WorkflowRunEvent` is accepted run-local history.
- `AuditEvent` is derived projection.
- WorkReports and artifacts cite records/events and do not create lifecycle state.

This boundary is important before writes. It prevents report prose, audit projection, or provider-candidate fixtures from being treated as proof of a live side effect.

## 5. Lifecycle Transition Assessment

The proposed transition mechanics are sound:

- `Proposed -> Attempted` requires existing proposed state, allowed authority, idempotency, explicit context, and no provider call.
- `Attempted -> Completed` requires a validated provider outcome reference.
- `Attempted -> Failed` requires a classified non-leaking failure reason or failure outcome reference.
- `Denied` and `Skipped` remain pre-attempt terminal postures for that intent.
- Lifecycle regression is rejected.
- Approval remains authority context, not a lifecycle state.

The plan correctly avoids adding `Approved` as a SideEffect lifecycle state.

## 6. Event And Store Boundary Assessment

The plan recommends that the first implementation return validated records and optional reference-only event payloads, rather than append events or mutate stores directly. That is the right first step.

The store-backed follow-up is also correctly deferred. Store mutation, duplicate lifecycle handling, and replay semantics deserve a separate review after the pure helper shape is visible.

## 7. Idempotency And Replay Assessment

The plan defines conservative idempotency posture:

- bind transition identity to side-effect ID, workflow/run identity, target, capability, actor/system actor, and provider operation reference where known;
- reject duplicate conflicting transitions;
- do not reattempt mutation blindly;
- keep replay behavior explicit.

This is sufficient for the next pure helper phase. Provider-visible idempotency remains an open question for live GitHub PR comment work, which is appropriate.

## 8. Privacy And Redaction Assessment

The plan is redaction-safe.

It forbids copying:

- raw provider payloads;
- raw request bodies;
- raw target URLs;
- credentials;
- authorization headers;
- token-like values;
- pull request diffs or bodies;
- command output;
- CI logs;
- file paths or snippets in errors.

It also requires stable non-leaking error codes and bounded provider outcome/failure references. This matches the rest of the write-readiness posture.

## 9. Test Plan Assessment

The future test plan is strong enough to drive implementation.

It covers:

- valid proposed-to-attempted transition;
- denied/skipped transition rejection;
- allowed authority;
- unknown capability rejection;
- identity and idempotency preservation;
- completed outcome requirement;
- failed reason/outcome requirement;
- lifecycle regression rejection;
- duplicate conflict handling;
- reference-only event payloads;
- no default executor attempted/completed/failed behavior;
- non-leaking Debug/serialization/errors;
- existing SideEffect, report artifact, approval-linkage, provider-candidate, and adapter regressions.

Non-blocking follow-up: the implementation prompt should add at least one explicit fixture/dry-run non-escalation test showing fixture validation cannot produce attempted/completed/failed lifecycle transitions.

## 10. Documentation Assessment

Documentation updates are consistent:

- `ROADMAP.md` now points to lifecycle transition planning and frames review/pure helper implementation as next.
- `GitHub PR Comment Provider Write Readiness Plan` now links to the lifecycle plan.
- `Write-Capable Adapter Readiness Plan` records the lifecycle planning status.
- `Governed Work Pattern` records planning without claiming execution.

The docs continue to say provider writes, runtime side-effect execution, CLI mutation, schemas, examples, hosted behavior, reasoning lineage, autonomy expansion, and release posture changes are not implemented.

## 11. Planning Blockers

None.

## 12. Non-Blocking Follow-Ups

- In the implementation prompt, require a fixture/dry-run guard test proving provider-candidate validation cannot create attempted/completed/failed transitions.
- Keep store-backed transition behavior separate from the pure helper phase.
- Keep live GitHub sandbox mutation separate until pure and store-backed transition behavior are reviewed.

## 13. Recommended Next Phase

Proceed to pure SideEffect lifecycle transition helper implementation.

Reason: the plan has enough specificity to implement local, deterministic transition helpers without provider calls, executor append behavior, store mutation, CLI behavior, schemas, examples, hosted behavior, or release posture changes.

## 14. Review Validation

- `npm run check:docs`: passed.
- Rust and TypeScript checks were not run because this phase is a documentation-only review.

Governed review:

- workflow: `dg/review`;
- run: `run-1783262984277425000-2`;
- approval: `approval/run-1783262984277425000-2/review-scope-approved`;
- approval outcome: granted by delegated maintainer;
- phase closeout: completed;
- events: 39 total, 1 approval, 0 retries, 0 escalations.
