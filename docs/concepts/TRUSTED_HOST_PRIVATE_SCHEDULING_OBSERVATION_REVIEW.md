# Trusted-Host Private Scheduling Observation Review

## 1. Executive Verdict

**Phase accepted; proceed to one private injected deadline-wait interface and
one schedule-once helper.**

The merged slice preserves the accepted security boundary. It derives one
coherent read-only observation, one inert absolute-UTC ticket, and one
non-mutating readiness assessment. It does not satisfy a wait, manufacture
authority, reserve dispatch, or enter an executor.

## 2. Scope Verification

The implementation stayed within the approved first-slice boundary:

- one crate-private scheduling observation;
- one crate-private inert ticket;
- one crate-private readiness assessment;
- focused tests and honest documentation; and
- no waiter, timer, polling, callback, schedule-once helper, executor entry,
  wait mutation, provider, OpenShell, public API, CLI, SDK, schema, hosted
  behavior, write behavior, or release change.

## 3. Coherent Observation Assessment

The observation opens the existing database read-only, loads one continuity
snapshot inside one deferred transaction, obtains one Core-owned trusted-time
observation, and derives the disposition, handoff, and ticket from that same
snapshot. It does not compose separately observed state.

Only one exact unsatisfied `TimeWindow` wait can produce a ticket. Other
postures either return no scheduling material or fail closed through the
existing bounded disposition and error vocabulary.

## 4. Ticket Assessment

The ticket binds:

- the current opaque handoff commitment;
- execution-window identity and revision;
- yield generation;
- wait condition identity, version, and revision;
- dependency commitment and exact deadline;
- trusted-time source, provenance, and epoch; and
- the closed next operation,
  `WaitOnceThenRequestFreshVerification`.

The model is private, not serializable, and cannot be constructed through a
public caller surface. Copying or delaying it cannot satisfy the wait. Its
Debug implementation exposes only the closed next-operation vocabulary and a
redacted binding marker.

## 5. Readiness Assessment

Readiness reloads authoritative state through a read-only connection and
freshly derives the current handoff and ticket. The supplied commitments must
match those current derivations. Trusted-time source, provenance, and epoch
must also match the durable trust root.

An early observation returns `NotYetEligible` with refreshed inert posture.
An elapsed observation returns `Eligible` without a capability, directive,
reservation, or executor input. The authoritative transition still obtains
trusted time again inside its mutation transaction, so a later clock or state
change cannot be bypassed by this assessment.

## 6. Zero-Write And Atomicity Assessment

Both production entry points use `existing_read_only_connection`. Focused
tests also compare the conformance snapshot before and after early, elapsed,
and rejected assessments. No operation, event, security rejection, directive,
reservation, receipt, wait transition, or workflow state is written.

The accepted source-specific transition and operational-entry composition
remain the sole authority-bearing boundary. The new code does not weaken the
one-winner, one-use, crash-recovery, or fresh-context reassessment proofs.

## 7. Stale And Substitution Assessment

The implementation rejects a substituted ticket before any mutation with the
stable `trusted_host_time_window_scheduling.ticket_stale` code. It rejects a
trusted-time provenance, epoch, or source mismatch with the stable
`trusted_host_time_window_scheduling.trusted_time_binding_mismatch` code.

Because the ticket is private and has no serde or public constructor, callers
cannot independently edit its bound fields while preserving its commitment.
Future exposure would require a new review; this acceptance does not authorize
serialization or a public ticket API.

## 8. Privacy And Error Assessment

- Debug output omits the deadline, identifiers, paths, and commitments.
- Errors use static bounded messages and do not echo supplied values.
- No command, prompt, provider payload, credential, evidence body, or source
  content is stored or rendered.
- The ticket contains payload-free commitments and private typed values only.

The focused secret-marker test confirms that an invalid trusted-time binding
does not leak through Debug output.

## 9. Test Quality Assessment

Focused coverage proves deterministic derivation, restart stability, exact
private deadline representation, early and elapsed readiness, zero writes,
stale-ticket rejection, trusted-time binding rejection, and Debug redaction.

Formatting, strict workspace clippy, the full workspace suite, documentation
checks, diff checks, and all seven required GitHub checks passed for PR #527.

Cancellation after scheduling, duplicate callbacks, waiter failure, and
non-polling behavior are correctly absent because this slice has no waiter or
callback. Those are required proofs for the next implementation, not missing
claims in this one.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Keep the ticket crate-private and non-serializable in the next slice.
- Make the waiter consume only the exact deadline and opaque ticket, never
  workflow state or authority.
- Prove early, late, cancellation, duplicate-callback, restart, host-failure,
  and non-polling behavior around the future schedule-once helper.
- Keep fresh trusted-time verification inside the existing atomic transition.

## 12. Recommended Next Phase

Implement one private injected deadline-wait interface and one private
schedule-once host helper. The helper may wait once and request fresh Core
verification once. It must not loop, poll, re-arm itself, infer approval,
create a model turn, persist authority, or broaden to another wake family.

Repeated scheduling, public host integration, provider execution, OpenShell,
nested harnesses, CLI, SDK, schema, hosted behavior, and writes remain
deferred.

## 13. Governed Review Record

- workflow: `dg/review`
- run: `run-1791353450856913000-2`
- approval:
  `approval/run-1791353450856913000-2/review-scope-approved`
- presentation: `presentation/f4dc21b117986803`
- presentation hash:
  `f4dc21b11798680340980c91cf74109bb1d6a6bce60e5e9529b2db3f0128706e`
- approval outcome: granted by delegated maintainer through proof enforcement
- reviewed merge commit: `ed77f7d`
- approved boundary: focused maintainer/security review only
- phase status: `Completed`
- event summary: 39 events; one approval; zero retries; zero escalations;
  approval presentation proof enforced with one persisted presentation record
- validation summary: focused tests, strict clippy, docs and diff checks, the
  already-green full workspace suite, and seven required GitHub checks
- out-of-kernel work: source inspection, security analysis, review authoring,
  validation, and later git and pull-request actions
