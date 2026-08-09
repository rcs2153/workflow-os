# Proportional-Governance Approval-Resume Runtime-Fact Source Consumer Plan Review

## 1. Executive Verdict

Plan accepted; proceed to the explicit local approval-resume source consumer
implementation.

The plan closes the next concrete authority gap without broadening defaults.
It replaces caller-assembled approval-resume facts on one opt-in path with the
same registered source and durable provenance boundary already accepted for
fresh execution and retry.

## 2. Scope Verification

The plan remains within one local Core consumer. It does not authorize default
activation, existing API replacement, reusable authority, raw fact persistence,
schemas, CLI or UI behavior, providers, OpenShell, SideEffects, writes, hosted
expansion, report citation, enterprise identity, or release changes.

## 3. Architectural Assessment

The integration boundary is appropriately narrow:

- the caller explicitly supplies registration, source, profile, decision time,
  approval request, and store;
- Core binds the source call to the exact stored immutable bundle;
- the durable V3 commitment proves initial provenance;
- a fresh same-call observation proves decision-time posture; and
- existing approval application remains the sole mutation boundary.

This preserves the product distinction between governance and execution. The
source supplies bounded facts; it does not grant approval or execute work.

## 4. Ordering Assessment

The required ordering is correct. Existing approval preparation first validates
the pending request and current resolved context without mutation. Durable
bundle, snapshot, and registration preflight then occurs before source
invocation. Same-call source assessment and durable-binding comparison occur
before the existing approval application path.

This prevents source calls for obviously mismatched registrations and prevents
`ApprovalGranted`, `RunResumed`, policy, or skill events from preceding current
fact validation.

## 5. Grant And Denial Assessment

The plan correctly distinguishes resumption authority from denial. A grant can
resume execution and therefore requires fresh decision-time facts. A denial
cannot expand authority or execute work, so requiring source availability would
make fail-closed behavior operationally weaker.

The implementation must prove that denial invokes the source zero times and
must not return a fabricated decision-time snapshot for that path.

## 6. Provenance And Freshness Assessment

The durable initial snapshot commitment remains provenance, not authority. The
decision-time snapshot remains call-local and cannot be deserialized or reused
as a grant. That is the correct separation.

Registration commitment equality before source invocation is essential. The
existing assessment helper then owns source identity, contract version, bundle,
freshness, coverage, and canonical assessment validation. The implementation
should reuse those gates rather than duplicate them in executor code.

## 7. Compatibility Assessment

Rejecting V1 and V2 bindings on this new source-backed consumer is correct.
Those versions remain valid for their existing paths but cannot prove the
source provenance required here.

Allowing a new observation to differ in snapshot identity and fact commitment
while reproducing the same assessment core preserves honest freshness. Failing
on a changed assessment is conservative and appropriate until runtime
escalation routing is separately composed.

## 8. Privacy And Error Assessment

The proposed error boundary is stable and non-leaking. It reuses source-owned
validation codes, adds only bounded executor state errors, redacts request and
result Debug output, and keeps raw facts out of events and durable records.

The implementation review must verify that registration preflight and corrupt
binding errors do not echo identifiers, commitments, paths, or timestamps.

## 9. Test Quality Assessment

The planned matrix covers the critical positive path, operation ordering,
equivalent fresh snapshots, registration preflight, durable corruption,
freshness, exact coverage, denial availability, idempotency, event stability,
privacy, and regressions. It is sufficient for implementation.

One implementation detail should remain explicit in tests: capture the event
history before a failing grant attempt and compare it for exact equality after
the error, rather than checking only event count.

## 10. Product Feedback Reconciliation

Fresh-pull evaluation confirms that Workflow OS is credible but low-risk work
still carries too much ceremony. Source-backed approval resume is not itself a
UX feature, but it is a prerequisite for reducing prompts safely: the kernel
cannot make or preserve a proportional decision at a resumption boundary while
accepting unproven caller-classified facts.

The Node 24 integration-check opacity and duplicate missing-manifest diagnostic
reported by the evaluator are already fixed on current `main`. They do not need
to interrupt this phase.

## 11. Planning Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add a proof-enforced presentation wrapper after the base source consumer is
  accepted.
- Decide later whether a report cites the durable initial commitment, a bounded
  decision-time receipt, or both.
- Keep production time authority and authenticated source identity separate
  from this local integration.
- Compose changed-assessment escalation only after a dedicated runtime-routing
  review.

## 13. Recommended Next Phase

Implement the explicit local approval-resume current-runtime-fact source
consumer in one governed runtime-composition phase, including its focused
implementation report and maintainer review.
