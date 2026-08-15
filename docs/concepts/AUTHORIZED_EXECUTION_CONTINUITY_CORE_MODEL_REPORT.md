# Authorized Execution Continuity Core Model Report

## 1. Executive Summary

Workflow OS now has a model-only vocabulary for preserving the distinction
between an external executor turn ending and a governed workflow becoming
waiting or terminal. The model is deliberately non-authoritative: it can
describe gate presentability, bounded execution windows, genuine typed waits,
executor yield, and attempt outcomes, but it cannot authorize, schedule,
resume, retry, approve, or complete work.

## 2. Scope Completed

- Added non-authoritative gate-presentability assessment bound to an exact
  approval request, action, immutable run bundle, step, run, and event cursor.
- Added bounded execution-window vocabulary with subject, action/resource
  scope, approval references, authority source and commitment, immutable run
  binding, sensitivity ceiling, cursor, expiry, and lifecycle provenance.
- Added exact typed wait conditions with identity/version, run/window/action,
  step/attempt, cursor/event, dependency reference, deadline, wake trigger,
  and current wait posture.
- Added executor-yield vocabulary derived from one validated open window.
- Added attempt outcomes that always block automatic retry until a fresh
  current-authority decision exists.
- Added redaction-safe Debug and fail-closed, non-echoing deserialization.
- Exported the model through `workflow-core` and added focused tests.

## 3. Scope Explicitly Not Completed

This phase does not add runtime events, snapshot fields, backend writes,
compare-and-set operations, executor integration, host scheduling, automatic
resume, automatic approval, delegated approval, provider mutation, CLI
behavior, schemas, hosted workers, nested harness execution, or release
posture changes.

## 4. Model Types Added

The core model includes:

- `AuthorizedExecutionGateAssessment` and typed prerequisite blockers;
- `AuthorizedExecutionWindow` and bounded lifecycle posture;
- `AuthorizedExecutionWaitCondition` and deterministic wake vocabulary;
- `AuthorizedExecutionYield` and turn-boundary reasons;
- `AuthorizedExecutionResumeDisposition`;
- `AuthorizedExecutionAttemptOutcome`;
- validated IDs and action/resource/authority-source references.

## 5. Validation Boundary

Gate readiness means only `ReadyForDecision`; it is not an approval decision
or execution grant. A pending gate requires typed prerequisite blockers, while
a ready gate forbids them. Approval cannot satisfy missing evidence or checks.

An open execution window requires a known sensitivity, non-empty action and
resource scope, positive bounded attempt budget, future expiry, exact subject,
immutable bundle, current-authority binding, and no closure event. Every
non-open lifecycle state requires an event reference. The model intentionally
does not represent `Exhausted` until attempt-event derivation can prove it.

## 6. Yield, Wait, And Retry Posture

An ordinary turn boundary may yield with no wait conditions and remain
eligible only for a new Core-owned authorization assessment. A genuine wait
must be actively bound to the same run, window, action, step, attempt, cursor,
and event as the yield. Its deterministic wake trigger must match its typed
dependency.

Yield construction rejects closed or expired windows, stale cursors,
out-of-scope actions, future-created waits, waits or yields that predate the
window's authority evaluation, cursor/event contradictions, and mismatched
identity. Serialized yields explicitly state that owning-window reconciliation
is required. They are not bearer authority.

All attempt outcomes, including retryable failure, block automatic retry. A
retry may occur only after fresh current facts and one-time authority are
resolved by a later runtime boundary.

## 7. Privacy And Redaction

Models store bounded identifiers, hashes, enum posture, timestamps, and stable
references only. They do not store prompts, transcripts, hidden reasoning,
source contents, provider payloads, command output, environment values,
credentials, tokens, or arbitrary explanations. Debug output redacts bound
identity and commitments. Invalid unknown fields, enum variants, and
secret-like values fail with generic non-echoing deserialization errors.

## 8. Test Coverage

Focused coverage includes:

- ready and pending gate posture;
- duplicate and contradictory blockers;
- every typed wait and wake-trigger pairing;
- condition version and deadline validation;
- subject/scope/authority-bound window round trips;
- invalid window lifecycle, time, sensitivity, and scope;
- ordinary turn-boundary yield without a false wait;
- exact wait attempt/cursor/action/time binding;
- expiry, evaluation-time, and cursor/event mismatch rejection;
- fresh-authorization requirement for every retry outcome;
- serialized non-authority and reconciliation posture;
- Debug and deserialization non-leakage.

## 9. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1786798279903534000-2`
- approval: `approval/run-1786798279903534000-2/implementation-approved`
- presentation: `presentation/b2770915fd0ccab9`
- approval outcome: granted by delegated maintainer after complete handoff
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval presentation enforcement: proof-enforced with one matching durable
  presentation record and event marker
- out-of-kernel work: source edits, tests, documentation, and command
  execution were performed by the external executor under the governed scope

## 10. Validation

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- focused continuity test target: 12 passed, 0 failed;
- `cargo test --workspace`: passed;
- `npm run check`: passed;
- `npm run check:integrations`: passed;
- `git diff --check`: passed;
- independent security re-review: no blockers remain after temporal binding
  hardening.

## 11. Remaining Limitations

- Models are not persisted or event-projected.
- No backend atomically registers yield/wait or consumes resume directives.
- No host supervisor observes or resumes lawful work.
- Serialized yield orientation must be reconciled against its owning window.
- Attempt-started/outcome events and restart recovery remain absent.
- General delegated approval remains blocked on parent-grant lineage,
  attenuation, expiry, and revocation semantics.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of this core model. If accepted,
the next implementation should define the atomic durable-state contract and
backend conformance behavior before adding runtime events or a supervisor.
