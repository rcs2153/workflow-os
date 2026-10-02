# Atomic Dispatch Reservation Plan Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; proceed to the private SQLite atomic dispatch-reservation
implementation.**

The corrected plan is security-coherent, implementable against the existing
operational-opening, continuity, supervisor, event, snapshot, and SQLite
migration surfaces, and remains within the bounded local-first scope.

## 2. Scope Verification

The fix changed planning artifacts only. It did not add runtime code, a
scheduler, repeated dispatch, provider execution, OpenShell, nested harnesses,
automatic approval, public configuration, CLI, SDK, workflow schemas, hosted
behavior, or another write family.

## 3. Original Blocker Restatement

The initial plan treated reservation as durable state but did not resolve its
append-only run-event representation. Because dispatch admission authorizes
executor entry, omitting it from the ledger would leave the authoritative
event history incomplete.

## 4. Event And Projection Assessment

The plan now defines one payload-free
`AuthorizedExecutionAttemptDispatchAdmitted` runtime event. The reservation,
event, and snapshot projection commit in one immediate SQLite transaction and
share exact commitments and cursors.

The event is correctly separated from caller-invoked continuity operations.
It records that Core admitted one attempt; it does not authorize a caller,
complete an attempt, change workflow status, expose a workflow-spec field, or
create a CLI/SDK surface.

The event payload is appropriately bounded to identities, revisions,
commitments, trusted-time facts, and cursors. Raw invocation or provider data
is excluded.

## 5. Replay And Ambiguity Assessment

- Exact replay returns the committed receipt and event binding without another
  event or capability.
- A losing claimant creates neither capability nor admission event.
- After-commit reconciliation may prove the receipt and event binding but
  returns no capability.
- Missing, duplicate, conflicting, or partially projected reservation state
  fails closed.

This preserves idempotent inspection without converting durable evidence into
replayable execution authority.

## 6. Downstream Binding Assessment

The corrected plan names the exact private inputs required by outcome, yield,
and ambiguous-attempt persistence:

- reservation receipt ID;
- reservation commitment; and
- dispatch-admission cursor.

Verifying these inside each existing mutation transaction prevents an old
attempt capability from writing post-dispatch state without proving which
admission authorized the executor entry.

## 7. Capability Boundary Assessment

The reservation module owns the only constructor for the opaque
`ReservedAttemptDispatchCapability`. Other modules may consume but not
construct, clone, serialize, deserialize, or reconstruct it. This is the
smallest idiomatic Rust boundary compatible with the current private
supervisor design.

## 8. Test Quality Assessment

The updated plan now covers:

- real concurrent SQLite claimants;
- one executor callback;
- one ordered admission event;
- no duplicate event on replay or loss;
- after-commit reconciliation without authority;
- reservation/event/snapshot consistency;
- rehydration without false completion;
- exact downstream mutation binding;
- localized capability construction;
- trusted-time and substitution checks;
- migration and manifest integrity; and
- privacy and existing-runtime regression.

This is sufficient to begin the bounded implementation.

## 9. Privacy And Compatibility Assessment

The fix preserves payload-free persistence and generic errors. The new runtime
event is additive audit vocabulary and must be documented as experimental. It
does not change workflow schema or authorize filesystem/PostgreSQL parity.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- The later recovery design must decide whether an admitted capability lost
  before executor entry can ever be safely reassigned.
- PostgreSQL conformance remains deferred.
- A repeated supervisor loop remains blocked until this one-winner slice is
  implemented and reviewed.

## 12. Recommended Next Phase

Implement the **private SQLite atomic dispatch-reservation slice** exactly as
planned. Do not combine it with scheduling, provider execution, OpenShell,
nested harnesses, automatic approval, public runtime configuration, or another
mutation family.

## 13. Governed Review Record

- workflow: `dg/review`
- run: `run-1790909203455219000-2`
- approval: `approval/run-1790909203455219000-2/review-scope-approved`
- presentation: `presentation/52211918c8e7c088`
- presentation hash:
  `52211918c8e7c08888e51400b8de387cdce425bf0c0e1182b159eb510e9865d4`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, and six successful skill
  invocations; no retries or escalations
- validation summary: `npm run check:docs` and `git diff --check` passed
- reviewed boundary: corrected planning artifacts and current private runtime
  integration surfaces only
- out-of-kernel work: source inspection, review authoring, validation, and git
  are performed by the delegated maintainer; Workflow OS governs the phase but
  does not edit repository files or run shell commands
