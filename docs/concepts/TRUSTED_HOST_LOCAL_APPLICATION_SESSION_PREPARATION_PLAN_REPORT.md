# Trusted-Host Local Application Session Preparation Planning Report

## 1. Executive Summary

The session-preparation helper is now planned, not implemented. The plan
defines a Core-owned, read-only preparation boundary that validates current
SQLite state and exact invocation bindings before issuing one opaque session
and cooperative cancellation handle.

Authoritative state is revalidated and authority is consumed only when the
session runs. This avoids both stale prepared authority and an opened attempt
that never enters because cancellation won first.

## 2. Scope Completed

- Mapped the existing private owner and operational-entry composition chain.
- Defined explicit preparation inputs and fresh/existing window posture.
- Chose read-only preparation plus authoritative run-time revalidation.
- Defined Core ownership of redispatch and wake identity generation.
- Required bounded cancellation failure before handle issuance.
- Defined opaque prepared-pair, failure, privacy, and test contracts.
- Proposed one direct-tested Core implementation slice.

## 3. Scope Explicitly Not Completed

- no preparation helper or prepared-pair code;
- no local-host package or executable;
- no production caller or invocation source;
- no discovery, scheduling, signal handling, or process recovery;
- no provider behavior, OpenShell, nested harnesses, or writes;
- no schemas, SDK behavior, hosted parity, or release changes.

## 4. Key Architecture Decision

Preparation performs read-only coherence checks and captures exact expected
bindings. It does not open a window, consume current authority, consume a
resume directive, allocate an attempt capability, append an event, or invoke
the executor.

The consumed session rehydrates and revalidates current state immediately
before the existing atomic opening or resume path. Changed state fails closed
through fixed application failure vocabulary.

## 5. Cancellation Decision

The current cancellation handle cannot be issued by a production preparation
helper while it returns broad `WorkflowOsError`. The implementation must
project cancellation failure into fixed payload-free application vocabulary
and add non-leakage tests before issuing the pair.

## 6. Implementation Boundary

The recommended implementation is one feature-gated Core slice containing:

- bounded cancellation failure;
- one private preparation input;
- one opaque prepared pair;
- read-only current-state and binding validation;
- a one-shot Core-owned runner with authoritative revalidation; and
- direct security, concurrency, privacy, and non-regression tests.

No application package or caller belongs in that slice.

## 7. Validation

- `npm run check:docs`: passed;
- `git diff --check`: passed.

## 8. Remaining Limitations

- No production source currently supplies all preparation inputs.
- No application process owns invocation or shutdown.
- Process-loss recovery and signal handling remain unresolved.
- External-consumer feature-matrix coverage is planned but not implemented.
- The exact fresh/existing private input shape requires implementation review.

## 9. Recommended Next Phase

Perform a focused maintainer/security review of the session-preparation plan.
Review read-only preparation, run-time revalidation, current-authority
handling, cancellation failure projection, identity-provider ownership,
construction privacy, test completeness, and continued backend isolation.

## 10. Governed Planning Record

- workflow: `dg/d`
- run: `run-1791381110043426000-2`
- approval: `approval/run-1791381110043426000-2/planning-approved`
- presentation: `presentation/c2a57165906d9776`
- presentation hash:
  `c2a57165906d9776fa2223a36e155f75549fa4b479f3409770f9dba723a1e745`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: planning and documentation only
- phase status: completed
- event summary: 39 events, one approval, zero retries, and zero escalations;
  presentation proof enforced with one persisted record and matching event
  marker
- validation summary: docs and diff hygiene passed
- out-of-kernel work: repository inspection, planning, documentation edits,
  validation, and later git or pull-request work
