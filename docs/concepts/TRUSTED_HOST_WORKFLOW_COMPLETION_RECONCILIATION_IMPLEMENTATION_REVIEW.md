# Trusted-Host Workflow Completion Reconciliation Implementation Review

## 1. Executive Verdict

**Phase accepted after in-review blocker fixes.**

The bounded one-step trusted-host docs-check path now composes operational
opening, dispatch, handler execution, continuity outcome, invocation outcome,
and workflow terminal truth without giving the foreground host authority over
workflow events or identities. The review found and fixed two terminal
transaction blockers before acceptance:

1. a current-authority security rejection could still project handler-derived
   invocation and workflow terminal events; and
2. the terminal continuity operation did not bind the exact workflow lifecycle
   context and terminal outcome into its replay commitment.

After correction, workflow terminal events are written only for a committed
successful continuity operation, and exact replay is bound to the invocation,
output contract, bounded output reference or fixed failure, and generic
attempt outcome.

## 2. Scope Verification

The implementation stays within the accepted private one-step docs-check
profile. It adds no public runtime API, additional profile, scheduler,
provider, OpenShell, nested harness, hosted, CLI, SDK, schema, automatic
approval, or write-capable adapter behavior.

The foreground application remains a synchronous, unpublished caller. It does
not construct workflow lifecycle events, select invocation identities, or
decide terminal workflow state.

## 3. Lifecycle Ownership And Ordering

Core owns one lifecycle context containing the invocation identity,
idempotency identity, step and skill identity, skill version, and immutable
required-output set.

The durable order is correct:

1. `SkillInvocationRequested` precedes
   `OperationalExecutionWindowOpened` in one opening transaction.
2. `AuthorizedExecutionAttemptDispatchAdmitted` precedes
   `SkillInvocationStarted` in one dispatch transaction.
3. `AuthorizedExecutionContinuityProjected` precedes the invocation terminal
   event and matching run terminal event in one outcome transaction.

Continuity cursors advance through the complete atomic event sequence rather
than stopping at an intermediate workflow projection.

## 4. Atomicity And Current-Authority Assessment

Opening, dispatch, and terminal lifecycle facts commit with the authoritative
operation that establishes each fact. A failure before commit leaves none of
that transaction's facts durable. A successful terminal transaction does not
depend on the caller retaining `SkillOutput` after return.

The initial implementation incorrectly supplied lifecycle terminal events to
the projection writer even when continuity classified the outcome operation as
a committed security rejection. That could have made workflow truth claim
success or failure based on the handler result after current authority rejected
the operation. The fix now:

- uses the extended lifecycle cursor only for committed-success disposition;
- uses the continuity projection cursor for security rejection;
- omits invocation and run terminal events for security rejection; and
- leaves the workflow non-terminal while the supervisor returns a bounded
  authority-rejection error.

This preserves the rule that handler output proposes an outcome; current
authority decides whether it becomes workflow truth.

## 5. Replay And Commitment Assessment

Opening and dispatch request commitments already included the lifecycle
commitment. Review found that terminal outcome persistence initially included
only the generic continuity outcome in its request commitment. That was
insufficient for exact replay because two success results with different
bounded output references, or different lifecycle/output-contract contexts,
could share the same operation identity.

The corrected terminal request commitment now composes:

- the generic attempt-outcome commitment;
- the complete lifecycle-context commitment; and
- a domain-separated terminal-outcome commitment covering success output
  reference or fixed failure metadata.

Conflicting replay therefore fails closed before it can be accepted as the
same durable operation. Exact terminal application replay continues to append
no events.

## 6. Output Validation And Failure Assessment

Required-output validation occurs before the terminal transaction chooses its
continuity outcome. Complete output commits success. Missing required output
commits the fixed `executor.output.required_missing` failure to both invocation
and run. A handler terminal failure maps to fixed
`trusted_host.execution.failed` metadata.

The implementation follows the existing executor's required-field semantics:
presence is required; this phase does not add a new output type system.

## 7. Identity And TOCTOU Assessment

Fresh admission derives lifecycle identity from the immutable selected
workflow, step, and skill. Existing non-terminal admission recovers exactly
one matching invocation request and validates its derived idempotency key.
Terminal replay recomputes the invocation and executor commitments and requires
exactly one matching dispatch admission.

Actor, correlation, bundle, invocation/executor, and same-path executable
substitution continue to fail closed. Executable content identity is
revalidated before request construction. No new application-controlled
lifecycle identity was introduced.

## 8. Privacy And Error Assessment

Lifecycle context and terminal outcome Debug output are redacted. Request
commitments hash bounded lifecycle and terminal values without exposing them.
Required-output and handler-failure events use fixed messages. The application
continues to emit fixed posture strings and does not print paths, commands,
payloads, credentials, environment values, output maps, or secret-like test
markers.

The success event may carry the existing `SkillOutput.output_ref` contract,
which is documented as a non-secret reference. This phase does not broaden the
handler set beyond the closed local docs-check profile.

## 9. Test Quality Assessment

Coverage now proves:

- the exact seven-event successful lifecycle suffix;
- required-output failure produces a failed run;
- terminal replay appends no events;
- approval wait and unsupported topology remain non-executing;
- actor, correlation, and executable substitution fail closed;
- authority rejection projects no invocation or workflow terminal event;
- changed terminal outcome produces a replay conflict; and
- bounded application and error output remain non-leaking.

The complete executable workspace suite passes, including all 475
`workflow-core` unit tests, 343 `local_executor` tests with one opt-in live
test ignored, and the 7-test foreground integration target. The large targets
remain expensive to compile and link in this local environment. Standalone
doctest execution still reproduces an idle `rustdoc` hang, so no doctest pass
is claimed.

## 10. Documentation Assessment

The plan, planning reviews, implementation report, and roadmap accurately
describe a private one-step composition. They do not claim broader scheduling,
provider execution, OpenShell, nested harnesses, automatic approval, public
configuration, or production readiness.

## 11. Blockers

None after the two in-review corrections.

## 12. Non-Blocking Follow-Ups

- Reduce the `workflow-core` unit-test link and doctest discovery cost so local
  security regressions can produce faster evidence.
- Preserve the distinction between durable workflow truth and downstream audit
  or observability sink delivery.
- Keep retryable, yielded, ambiguous, and scheduler-driven lifecycle
  completion in separately reviewed phases.
- Address dogfood phase-state retention outside OS-temporary storage so delayed
  phase close can always inspect its event trail.

## 13. Recommended Next Phase

Return to the P0 roadmap preemption: perform the focused maintainer/security
review of the Authoritative Agent Continuation Context And Rehydration Plan,
then implement one local end-to-end continuation vertical slice if accepted.
Do not broaden provider mutations or nested harness execution first.

## 14. Governed Review Record

- Workflow: `dg/review`
- Run: `run-1792500349184071000-2`
- Approval: `approval/run-1792500349184071000-2/review-scope-approved`
- Presentation proof: `presentation/14fb7c1060b094f3`
- Approval outcome: granted under delegated maintainer authority
- Final status: `Completed`
- Event summary: 39 events, one approval, zero retries, zero escalations
- Presentation enforcement: proof-enforced, with one persisted presentation
  record and approval-event marker

Repository inspection, blocker correction, tests, documentation, and review
reasoning were performed by the delegated maintainer outside the kernel. The
kernel governed review scope, approval, sequencing, and presentation proof.
