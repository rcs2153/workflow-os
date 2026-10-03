# Private Trusted-Host Scheduling Boundary Plan Blocker-Fix Report

## 1. Executive Summary

The two planning blockers found by focused maintainer/security review are now
addressed without implementing scheduler behavior.

The corrected plan selects the durable absolute UTC deadline as the first
private host scheduling hint and adds a Core-owned non-mutating readiness
assessment. An early or spurious callback now has a planned benign
`NotYetEligible` path that writes no security rejection and enters no executor.
The accepted transition still obtains trusted time again atomically and
remains the only deadline-satisfaction proof.

## 2. Blockers Fixed

### Early wake semantics

The original plan sent every host wake directly to explicit reinvocation even
though an unelapsed transition is a durable security rejection. The corrected
plan requires a non-mutating source-specific readiness assessment first.

### Clock-domain representation

The original plan deferred the choice among absolute instant and relative
delay. The corrected first slice uses the exact private absolute UTC deadline,
matching the current durable binding and `CoreInjectedClockV1` domain. A host
may derive an ephemeral relative delay for its timer API, but that value is not
persisted or returned as proof.

## 3. Corrected Readiness Boundary

The planned readiness assessment:

- obtains fresh Core-owned trusted time;
- validates exact source, provenance, epoch, dependency, wait, window,
  generation, and inert scheduling commitments;
- returns `Eligible`, `NotYetEligible`, or structured failure;
- writes no operation, rejection, event, directive, reservation, or receipt;
- grants no authority or wake capability; and
- cannot replace the atomic transition recheck.

Only `Eligible` permits one call to explicit reinvocation.
`NotYetEligible` returns refreshed inert scheduling posture with zero mutation
and zero executor entry.

## 4. Cancellation And Duplicate Races

The corrected plan states:

- cancellation before wake is host-level posture with no Core mutation;
- cancellation after Core commits a transition cannot revoke that transition;
- duplicate callbacks remain subject to fresh readiness, exact replay, and
  existing one-winner mutation boundaries; and
- the first helper adds no retry or automatic re-arming behavior.

## 5. Scope Preserved

No Rust code, timer driver, scheduler, queue, daemon, polling loop, automatic
model turn, automatic approval, provider execution, OpenShell, nested harness,
public configuration, CLI, SDK, schema, hosted runtime, provider write, or
release change was added.

The full-composition concurrency and transition-to-entry crash proofs remain
mandatory before any scheduling implementation.

## 6. Test Plan Changes

The future test plan now requires proof that:

- `NotYetEligible` writes no operation or security rejection;
- `Eligible` grants no authority;
- a state or clock change before transition is caught atomically;
- wrong source, provenance, epoch, dependency, or revision fails closed;
- early and spurious callbacks enter no executor; and
- cancellation and duplicate races preserve authoritative posture.

## 7. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Remaining Limitations

- The corrected boundary is still planning only.
- Only current Core system UTC and one exact `TimeWindow` are covered.
- Repeated scheduling, another time source, and other wake families remain
  deferred.
- The two pre-scheduler runtime proofs are not implemented yet.

## 9. Recommended Next Phase

Perform focused maintainer/security re-review of the corrected plan. If the
fix is accepted, implement the two prerequisite explicit-reinvocation proofs
before any scheduling model or interface.

## 10. Governed Blocker-Fix Record

- workflow: `dg/blocker`
- run: `run-1791070857136910000-2`
- approval: `approval/run-1791070857136910000-2/fix-approved`
- presentation: `presentation/b53bf9d0c6645716`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: documentation-only planning blocker fix
- phase status: `Completed`
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, zero retries, and zero escalations; approval-presentation proof
  enforced with one matching record and event marker
- validation summary: documentation and diff checks passed
- out-of-kernel work: plan correction, report authoring, documentation
  validation, and later git and pull-request actions
