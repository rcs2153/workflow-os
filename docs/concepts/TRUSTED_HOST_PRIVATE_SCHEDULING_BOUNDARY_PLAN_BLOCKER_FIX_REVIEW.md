# Private Trusted-Host Scheduling Boundary Plan Blocker-Fix Review

## 1. Executive Verdict

**Planning blockers fixed; proceed to prerequisite reinvocation proof
implementation.**

The corrected plan now distinguishes benign timer jitter from a security
rejection, selects one coherent first clock-domain representation, and keeps
the atomic transition as the only deadline-satisfaction proof. It remains
private, schedule-once, and planning-only.

## 2. Scope Verification

The fix stayed within documentation-only planning scope. It added no Rust
behavior, scheduler, timer driver, polling, model turn, automatic approval,
provider execution, OpenShell, nested harness, public configuration, CLI, SDK,
schema, hosted runtime, provider write, or release change.

## 3. Early-Wake Fix Assessment

The planned Core-owned readiness assessment resolves the original
contradiction:

- it observes trusted time through Core;
- validates exact source, provenance, epoch, dependency, wait, generation,
  window, and inert scheduling bindings;
- returns `Eligible`, `NotYetEligible`, or structured failure;
- writes no operation, rejection, event, directive, reservation, or receipt;
- grants no authority; and
- cannot replace the transaction's fresh trusted-time check.

`NotYetEligible` is now the only benign early-wake result and causes zero
executor entry. This prevents ordinary timer jitter from creating durable
security-rejection noise.

## 4. Clock-Domain Fix Assessment

The first private ticket now uses the exact durable absolute UTC deadline as
an inert scheduling hint. That matches the accepted `CoreInjectedClockV1`
system-UTC domain. A host may derive an ephemeral relative delay for a timer
API, but it cannot persist or return that delay as satisfaction evidence.

The plan correctly requires separate review before supporting another trusted
time source or host clock domain.

## 5. Atomicity And Authority Assessment

The readiness result is not authority. After `Eligible`, explicit
reinvocation still:

1. validates the inert handoff commitment;
2. obtains trusted time again inside the transition transaction;
3. commits or exactly replays the source-specific transition;
4. reloads current state;
5. consumes fresh one-use authority;
6. reserves dispatch atomically; and
7. reassesses current authority and required context.

A state or clock change between readiness and transition therefore fails
closed at the authoritative mutation boundary.

## 6. Cancellation, Duplicate, And Restart Assessment

- Cancellation before wake is host-local and writes no Core state.
- Cancellation cannot revoke a transition already committed by Core.
- Duplicate callbacks remain bounded by readiness, exact replay, transition,
  directive, and reservation one-winner rules.
- The first helper adds no automatic retry or re-arming.
- Restart derives fresh inert scheduling posture from SQLite and never
  deserializes authority.

These semantics are sufficient for the planned schedule-once slice.

## 7. Security And Privacy Assessment

The ticket remains a private liveness hint, not a capability. The plan exposes
no prompt, command, source content, approval reason, evidence body, check
output, provider payload, credential, or private runtime authority. Security
failures remain structured and non-oracular.

## 8. Test Assessment

The corrected future tests now cover:

- zero-write `NotYetEligible` behavior;
- non-authorizing `Eligible` behavior;
- atomic recheck after clock or state movement;
- wrong source, provenance, epoch, dependency, and revision;
- early and spurious callbacks;
- cancellation and duplicate races; and
- restart without persisted capability.

The two pre-scheduler proofs remain mandatory and are the next implementation
scope:

1. full-composition concurrent callers admit at most one executor entry; and
2. transition-to-entry crash recovery after backend reopen produces no
   duplicate entry or reconstructed authority.

## 9. Blockers

None for proceeding to the prerequisite proof implementation.

This verdict does not authorize scheduling types or behavior.

## 10. Non-Blocking Follow-Ups

- Choose refreshed-ticket return ergonomics during later scheduling
  implementation review.
- Define privacy-safe wake latency and false-stall metrics later.
- Keep repeated scheduling and additional wake families separate.

## 11. Recommended Next Phase

Implement only the two prerequisite proofs on the existing explicit
reinvocation composition. Add focused concurrent-caller coverage and an
injected transition-to-entry crash seam with restart recovery. Do not add the
readiness assessment, ticket, deadline waiter, or scheduler helper yet.

## 12. Governed Review Record

- workflow: `dg/review`
- run: `run-1791071277516253000-2`
- approval: `approval/run-1791071277516253000-2/review-scope-approved`
- presentation: `presentation/f732da490bbb28f9`
- approval outcome: granted by delegated maintainer through proof enforcement
- reviewed fix commit: `fdad062`
- approved boundary: focused planning blocker-fix re-review only
- phase status: `Completed`
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, zero retries, and zero escalations; approval-presentation proof
  enforced with one matching record and event marker
- validation summary: documentation and diff checks passed
- out-of-kernel work: source and plan inspection, security analysis, review
  authoring, documentation validation, and later git and pull-request actions
