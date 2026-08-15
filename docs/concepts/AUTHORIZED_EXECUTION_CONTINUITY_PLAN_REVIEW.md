# Authorized Execution Continuity Plan Review

## 1. Executive Verdict

**Plan accepted; proceed to the core continuity decision model.**

The plan correctly treats false stalls as a kernel-state and host-supervision
problem rather than an approval-UX or prompt problem. It preserves the accepted
source-backed one-time continuation boundary and sequences model, atomic state,
event projection, and one local one-shot supervisor proof instead of attempting
one broad scheduler feature.

## 2. Scope Verification

The plan remains planning-only. It does not authorize Rust implementation,
runtime state changes, provider mutation, nested harness execution, automatic
approval, schemas, CLI execution automation, hosted scheduling, enterprise
identity, or release changes.

It does not position Workflow OS as a model host. It states that Core can make
resume posture durable while an integrated host must schedule an executor.

## 3. Problem Assessment

The problem is real and P0:

- current run terminality is durable and event-derived, but agent turn endings
  are outside that model;
- a model can emit a final response while the run remains non-terminal;
- current continuation claims authorize one exact operation but do not model
  turn yield, supervisor delivery, or durable consumer outcome;
- manual prompting can therefore become an accidental scheduler even when no
  governed wait exists.

The plan correctly states that conversation lifecycle must not define workflow
lifecycle.

## 4. Gate Readiness Assessment

The corrected plan makes gate readiness a non-authoritative typed assessment.
This is required. Persisting or trusting `ready: true` would conflict with the
current authority boundary, where current facts become usable only inside a
private same-call consumer.

The final decision path must independently reload evidence, checks, policy,
presentation proof, immutable binding, SideEffect posture, current authority,
and separation-of-duty facts. Approval cannot satisfy missing prerequisites.

## 5. Execution Window Assessment

The plan correctly defines an execution window as durable scheduler
eligibility, not a reusable authority lease. The window may survive executor
turns, but every material action still requires:

- current run rehydration;
- exact immutable binding;
- fresh current-authority and required-context resolution;
- current event cursor;
- one-time durable claim;
- same-call entry into the integrated consumer.

Expiry, revocation, stricter policy, capability loss, changed evidence, or a
new cursor may close or narrow the window before the next action.

## 6. Yield And Wait Assessment

Executor yield is correctly separated from completion, failure, cancellation,
approval wait, and evidence satisfaction. Typed waits include exact identity,
version, cursor, step/attempt, and wake posture rather than free-form pause
text.

The plan must not reinterpret existing weak `RunPaused`,
`ExternalEventReceived`, or payload-free `RunResumed` events as the complete
continuity protocol. The planned additive model and later event projection are
appropriate.

## 7. Atomicity And Recovery Assessment

The plan now requires compare-and-set durable operations for:

- registering one yield or wait plus its event projection at an expected
  cursor; and
- consuming one resume directive plus its resume projection at an expected
  waiting cursor.

This is blocking for event-integrated runtime work. Separate writes would
create crash windows that can lose work or permit duplicate resume.

The plan also recognizes the existing crash-after-continuation-claim gap.
Attempt-started and bounded success, failure, or ambiguous outcome posture must
exist before the local proof can claim restart-safe continuity.

## 8. Delegated Authority Assessment

The plan correctly defers general delegated approval from the first slice.
Current capability delegation vocabulary does not yet prove parent-grant
lineage, attenuation, remaining depth, revocation cascade, or cycle rejection.

The first continuity slice may consume an already-supported direct authority
path. Future delegated approval must prove strict child scope, expiry,
revocation, prerequisites, separation of duty, and comparable actor identity at
use time. Human-only or independent-review gates remain human-only.

## 9. Compatibility Assessment

Keeping existing `WorkflowRunStatus` variants unchanged in the first model and
projection slice is conservative and appropriate. These values are serialized
and widely matched. Continuity should first be an additive model and derived
projection.

Any later event variants require exhaustive review across runtime replay,
audit, observability, hosted code, reports, and backend validation. Existing
`execute()` and `decide_approval()` behavior should remain unchanged while the
new path stays explicit and opt-in.

## 10. Privacy Assessment

The plan stores bounded IDs, hashes, enumerated posture, timestamps, and
references only. It explicitly excludes prompts, transcripts, hidden
reasoning, raw source, provider payloads, command output, environment values,
credentials, and unrestricted free-form wait/yield text.

Debug, serde, validation, and host-delivery errors must remain non-leaking.

## 11. Test Assessment

The planned tests cover the important positive and adversarial paths:

- non-actionable unmet prerequisites;
- evidence-by-approval rejection;
- human-only and separation-of-duty enforcement;
- exact window binding and lifecycle;
- yield without false completion or approval wait;
- typed wait restart behavior;
- duplicate resume with one winner;
- stale cursor, expiry, revocation, policy escalation, capability loss, and
  evidence/check changes;
- unsupported non-atomic backends;
- crash boundaries before and after wait registration, directive claim,
  resume event, continuation claim, consumer entry, and outcome;
- non-leaking Debug and serialization;
- regression coverage across existing runtime and governance models.

## 12. Blockers

No blocker remains in the planning document.

The following are implementation gates, not planning defects:

- runtime integration cannot proceed until the core decision model is reviewed;
- event integration cannot proceed without an accepted atomic state contract;
- delegated approval cannot proceed without parent-grant lineage and use-time
  revocation/expiry semantics;
- general scheduling claims cannot proceed from a one-shot local helper.

## 13. Non-Blocking Follow-Ups

- Reconcile stale top-level boundary text about the bounded hosted and provider
  slices in a separate documentation-honesty phase.
- Decide whether host delivery exhaustion escalates or remains a typed wait.
- Decide when yield and resume posture enters WorkReport and audit projection.
- Keep operator-facing resume projection distinct from private authority.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1786798172060843000-2`
- approval: `approval/run-1786798172060843000-2/review-scope-approved`
- presentation: `presentation/9114ef0d38dbe6cd`
- approval outcome: granted by delegated maintainer after complete handoff
- review work: code and documentation inspection occurred outside the kernel;
  the kernel governed review scope and approval

## 15. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- independent run-state, security, and host-boundary analyses: completed.

## 16. Recommended Next Phase

Implement the **Authorized Execution Continuity core decision model only**.

The model phase should define and validate non-authoritative gate assessment,
execution-window binding and lifecycle, executor yield, typed wait conditions,
resume disposition, and attempt/outcome posture. It must not add runtime event
integration, backend atomic operations, host scheduling, delegated approval,
provider writes, nested runtime, schemas, or CLI behavior.
