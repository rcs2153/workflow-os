# Trusted-Host Explicit Local Process Owner Plan Review

## 1. Executive Verdict

**Needs planning blocker fixes.**

The one-shot private owner shape is appropriately narrow, preserves exact
operational bindings, and avoids discovery or background scheduling. However,
the plan describes the existing timer-cancellation handle as a process-shutdown
control without defining its pre-entry and active-executor semantics.

Today that handle is observed only by the deadline waiter. If it is canceled
before `run`, the owner would still call the operational entry first and could
invoke an immediately eligible executor. If cancellation arrives after
operational admission but before a wait, it cannot interrupt the executing
attempt. That boundary must be explicit before implementation.

## 2. Scope Verification

The plan remains within the approved documentation-only scope. It proposes one
crate-private, one-shot, synchronous owner and no runtime implementation.

It does not authorize discovery, scanning, polling, a daemon, detached work,
background threads, public API, CLI, SDK, schemas, runtime configuration,
automatic approval, provider behavior, OpenShell, nested harnesses, hosted
scheduling, PostgreSQL parity, or release changes.

## 3. Boundary Assessment

The selected sibling module is the smallest coherent location. It composes
only accepted private input, cancellation, operation, and outcome types. It
requires an exact preselected operational input and does not search durable
state for work.

This preserves the distinction between explicit ownership and automatic
scheduling.

## 4. Candidate API Assessment

The proposed constructor plus consuming `run(self)` is idiomatic and makes the
owner one-shot at the type level. Retaining the operational input without
cloning capabilities or mutable bindings is correct. Returning the existing
operation outcome avoids a parallel result taxonomy.

The owner should remain non-`Clone`, non-serializable, and crate-private.

The API is not yet implementation-ready because the returned cancellation
handle is described too broadly. The current type is a local-timer
cancellation handle, not a complete owner or executor cancellation token.

## 5. Lifecycle Assessment

Constructed, running, and closed are sufficient for a synchronous one-shot
owner. Dropping an unrun owner without Core mutation is correct. Consuming the
owner prevents accidental rerun.

The lifecycle currently omits an exact boundary for cancellation requested:

- before `run` begins;
- while initial operational admission is executing;
- while the attempt executor is running;
- while the owner is waiting on a deadline; and
- after the owner has returned.

Only the deadline-wait case is implemented by the reused primitive. The plan
must not imply broader cancellation.

## 6. Cancellation And Shutdown Blocker

This is the blocking finding.

`TrustedHostLocalTimerCancellationHandle::cancel` sets the shared timer flag.
`run_explicit_trusted_host_local_operation` first calls
`enter_trusted_host_operation`; it does not consult that flag. The flag is
observed only if initial entry later returns `AwaitCondition` and the
production caller reaches `TrustedHostLocalDeadlineWaiter`.

Therefore:

- pre-canceling the handle does not prevent immediate initial executor entry;
- a cancellation racing initial admission does not establish a no-entry
  guarantee;
- an executing attempt is not interruptible through this handle; and
- “process shutdown” is too broad a name for the current effect.

The blocker fix must choose and document an exact v1 contract. The recommended
contract is:

1. add a private, zero-write pre-entry cancellation observation owned by the
   same cancellation pair;
2. if cancellation is already observed before operational entry, return a
   bounded host-canceled outcome without Core mutation or executor entry;
3. once operational entry begins, cancellation does not revoke consumed
   authority or interrupt an active attempt;
4. cancellation remains effective at the next local deadline wait; and
5. process shutdown remains an application responsibility, not a kernel claim.

An alternative may intentionally retain wait-only cancellation, but then the
plan must rename the concept, remove process-shutdown claims, and explicitly
state that callers must avoid invoking a pre-canceled owner. That alternative
provides a weaker owner boundary and requires justification.

## 7. Authority And Binding Assessment

The plan correctly adds no capability, approval, wait-satisfaction claim, or
caller-authored continuation posture. Existing Core boundaries retain current
authority, trusted-time, commitment, directive, reservation, and executor
admission decisions.

A pre-entry cancellation observation must remain non-authorizing and
zero-write. It must not mutate durable workflow state or be represented as
workflow cancellation.

## 8. Result And Output Assessment

Returning `TrustedHostExplicitLocalOperationOutcome` directly is correct for
ordinary entry and continuation stops. A pre-entry canceled owner needs an
unambiguous bounded representation. The blocker fix must decide whether that
requires a small owner-level outcome wrapper or an accepted existing outcome
shape; it must not fabricate a redispatch result that never occurred.

Immediate `EntryStopped` skill output must remain available and redacted in
Debug. Durable output retrieval remains correctly deferred.

## 9. Restart Assessment

The non-durable owner posture is appropriate. A later explicit owner may
rehydrate exact current Core state with fresh process-local cancellation and
identity state. The fixed wake budget remains a host-loop bound rather than a
durable governance quota.

The plan correctly does not persist cancellation, capabilities, executor
objects, input payloads, output, or wake-budget remainder.

## 10. Concurrency Assessment

Requiring a direct competing-owner test is the right hardening step. The test
should construct each non-`Send` borrowed input inside its executing scoped
thread or otherwise use repository-valid lifetimes; the plan must not weaken
existing traits merely to make the test convenient.

The test must measure aggregate executor admission and must not add a
process-local mutex that masks the Core transaction boundary.

The cancellation blocker adds one more required race test: pre-entry
cancellation racing `run` must have an explicitly documented result and must
never be portrayed as stronger than the implementation can prove.

## 11. Privacy And Failure Assessment

The proposed custom Debug, absence of serialization, and direct propagation of
existing structured errors are appropriate. Construction should remain
zero-write and payload-free.

Any new pre-entry cancellation observation or owner-level outcome must use a
stable bounded code or typed posture and must not expose bindings, paths,
deadlines, identities, input, output, credentials, or provider values.

## 12. Test Plan Assessment

The planned lifecycle, timed continuation, restart, input-substitution,
competing-owner, unsupported-wait, wake-budget, and non-leakage coverage is
strong.

Before implementation, the plan must add explicit tests for:

- cancellation before `run` and whether it prevents all Core mutation;
- cancellation racing initial entry;
- cancellation while an attempt executor is active, proving the documented
  non-interruptible boundary; and
- cancellation after owner return remaining harmless and idempotent.

## 13. Blockers

1. Define exact pre-entry cancellation behavior.
2. Remove or narrow the unsupported general process-shutdown claim.
3. Define the bounded return posture for an owner canceled before operational
   entry without fabricating an underlying operation outcome.
4. Add the corresponding cancellation-race and active-attempt test matrix.

## 14. Non-Blocking Follow-Ups

- Keep application selection and operator notification deferred.
- Preserve the direct competing-owner requirement.
- Decide durable terminal-output retrieval before any public owner surface.
- Define owner-loss and stuck-work observability before detached ownership.
- Preserve the fixed two-wake budget and non-retrying error posture.

## 15. Recommended Next Phase

Perform one documentation-only planning blocker fix, then re-review the plan.

Do not implement the owner until the cancellation and shutdown contract is
precise. Do not broaden into discovery, daemon behavior, public configuration,
automatic approval, provider mutation, OpenShell, nested harnesses, hosted
scheduling, or release changes.

## 16. Governed Review Record

- workflow: `dg/review`
- run: `run-1791369737389001000-2`
- approval: `approval/run-1791369737389001000-2/review-scope-approved`
- presentation: `presentation/2bf2591ff0e6282e`
- presentation hash:
  `2bf2591ff0e6282e31ca3b6c230f65f665436d9c542c372346e3c9a9dc976de2`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- reviewed merge commit: `840f11de044219682a48358b0235287b2f25b722`
- approved boundary: focused plan review only
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval-presentation proof enforced with one persisted presentation record
- out-of-kernel work: source and plan inspection, review authoring, shell
  validation, and later git and pull-request actions
