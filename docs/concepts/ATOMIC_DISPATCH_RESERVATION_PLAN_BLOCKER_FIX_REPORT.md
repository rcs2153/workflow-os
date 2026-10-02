# Atomic Dispatch Reservation Plan Blocker Fix Report

## 1. Executive Summary

The atomic dispatch-reservation planning blocker is fixed. The plan now
defines how one committed executor admission is represented in the append-only
run ledger and projected snapshot without creating a new caller-invoked
continuity operation or execution credential.

This was a documentation-only blocker fix. No runtime code or behavior
changed.

## 2. Blocker Fixed

The initial plan left the durable event/audit representation of dispatch
admission open. A reservation relation without an atomic run event would make
the event history incomplete even though admission is the durable decision
that permits executor entry.

## 3. Fix Approach

The plan now requires one payload-free
`AuthorizedExecutionAttemptDispatchAdmitted` runtime event in the same
immediate SQLite transaction as the unique reservation. The same transaction
projects the exact binding into the run snapshot and validates reservation,
event, and snapshot commitments before commit.

The event is runtime audit vocabulary only. It does not add a workflow-spec
field, caller operation, CLI command, SDK API, provider action, or durable
execution credential.

## 4. Replay And Ambiguity Posture

- Exact replay returns the original receipt and event binding without another
  event or capability.
- A losing claimant creates no admission event.
- After-commit reconciliation may return the committed receipt and event
  binding but never reconstructs authority.
- Missing, duplicate, conflicting, or partially projected admission state
  fails closed.

## 5. Downstream Mutation Binding

Attempt outcome, yield, and ambiguous-attempt persistence must carry and
verify the exact reservation receipt, reservation commitment, and admission
cursor in their existing mutation transactions. The prior attempt capability
alone is no longer sufficient after reservation.

## 6. Capability Privacy

The reservation module owns the only capability constructor. The opaque
capability remains non-cloneable and non-serializable, and receipts or events
cannot be converted back into authority.

## 7. Tests Added To The Plan

The implementation test plan now covers ordered event emission, no duplicate
event on loss or replay, ambiguous reconciliation binding, reservation/event/
snapshot integrity, rehydration, localized capability construction, and exact
downstream mutation binding.

## 8. Scope Explicitly Not Completed

No runtime implementation, scheduler, repeated loop, provider, OpenShell,
nested harness, automatic approval, public configuration, CLI, SDK, workflow
schema, hosted behavior, or new write family was added.

## 9. Validation

Required documentation validation passed:

- `npm run check:docs`
- `git diff --check`

## 10. Recommended Next Phase

Perform a focused maintainer/security review of this planning fix. If
accepted, implement the private SQLite atomic dispatch-reservation slice.

## 11. Governed Phase Record

- workflow: `dg/blocker`
- run: `run-1790909113866712000-2`
- approval: `approval/run-1790909113866712000-2/fix-approved`
- presentation: `presentation/218431725e1ea4e3`
- presentation hash:
  `218431725e1ea4e3cf3aad37dde5a5f2ac81850a02511a668747ec3e5553a3d7`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, and six successful skill
  invocations; no retries or escalations
- validation summary: documentation and diff checks passed
- approved boundary: planning blocker fix only
- out-of-kernel work: document authoring, validation, git, and future PR work
  are performed by the delegated maintainer; Workflow OS governs the phase but
  does not edit repository files or run shell commands
