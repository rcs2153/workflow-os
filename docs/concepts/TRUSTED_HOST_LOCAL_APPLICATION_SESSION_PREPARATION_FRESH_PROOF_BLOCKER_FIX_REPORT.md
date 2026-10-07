# Trusted-Host Local Application Session Preparation Fresh Proof Blocker Fix Report

## 1. Executive Summary

The remaining fresh-session preparation proof blocker is fixed. Direct
authority-backed tests now cover dropping and canceling an unconsumed fresh
prepared session, fresh-context substitutions, malformed opening shape, and
incompatible trusted-time context at the application preparation boundary.

The direct tests exposed two pre-issuance validation gaps. Fresh preparation
could previously issue an opaque session-and-cancellation pair for an opening
with zero allowed attempts or an invalid expiry, and for trusted-time context
that did not match the current continuity instance. A small read-only
preflight now rejects both before issuance with stable payload-free errors.

Atomic authority consumption, opening identity collision handling, and final
time-of-use validation remain at consumed execution. No host application,
caller, discovery, scheduling, provider behavior, writes, schemas, SDK,
hosted parity, nested harness, OpenShell, or release change is introduced.

## 2. Scope Completed

- Reused the private authority-backed fresh preparation fixture.
- Added direct fresh prepared-pair drop proof.
- Added direct fresh pre-run cancellation proof.
- Added direct backend, actor, immutable-bundle, and invocation/executor
  substitution proof.
- Added direct invalid opening-shape and trusted-time substitution proof.
- Added a read-only fresh-opening semantic preflight before pair issuance.
- Preserved the consumed-session atomic authority-use boundary.

## 3. Scope Explicitly Not Completed

This phase does not add:

- a host package, executable, production caller, discovery, or scheduling;
- signal handling, detached execution, recovery, or active-attempt revocation;
- provider reads or mutations, broader write behavior, or OpenShell;
- nested harness execution;
- schemas, CLI, SDK, hosted, filesystem, or PostgreSQL adoption;
- a public current-authority factory; or
- release posture changes.

## 4. Implementation Approach

Fresh preparation still begins with the existing exact binding validation for
the SQLite backend instance, required-context execution binding, actor,
workflow/run/step identity, immutable bundle, window, skill input, and
invocation/executor commitment.

The new read-only semantic preflight then rejects:

- an opening with zero allowed attempts;
- an opening whose expiry is not after its observed trusted time;
- a trusted-time source, provenance commitment, or epoch that differs from
  the current continuity instance;
- a non-live or quarantined continuity time posture; and
- trusted-time regression relative to the current continuity state.

The preflight does not consume authority, create an opening, append an event,
or invoke the executor. The existing atomic opening path repeats authoritative
validation when the one-shot session is consumed.

## 5. Fresh Drop And Cancellation Proof

The direct fresh drop test prepares a pair and drops it without consuming the
session. It proves zero executor calls, zero opening operations, zero attempts,
zero projection bindings, zero continuity windows, unchanged continuity state,
and no workflow-event append.

The direct pre-run cancellation test requests cancellation through the paired
handle before consuming the session. The session returns the bounded
`CanceledBeforeEntry` outcome and proves the same zero-entry and zero-write
properties.

## 6. Substitution And Opening-Context Proof

Direct fresh preparation tests prove that these substitutions fail before a
pair is issued:

- backend instance;
- subject actor;
- immutable run bundle;
- invocation or executor binding;
- opening request shape; and
- trusted-time provenance context.

Each failure uses a stable fixed error code, enters no executor, and consumes
no opening authority. Immutable-bundle mismatch may fail first through run
eligibility; that remains a valid fail-closed pre-issuance outcome.

Opening operation, receipt, attempt, and window identifiers are validated by
their typed constructors and supplied as current authority inputs. Their
durable uniqueness and one-winner behavior remain the responsibility of the
atomic consumed-run store operation; preparation does not perform a racy
read-only uniqueness promise.

## 7. Security And Privacy

- Preparation remains read-only.
- No application-facing value contains authority internals or raw state.
- Stable errors contain no actor, identifier, bundle, invocation, trusted-
  time, path, payload, credential, command output, or source content.
- The prepared pair remains private, one-shot, non-cloneable, and
  non-serializable.
- The test authority source remains crate-private and `cfg(test)` only.
- Successful preparation remains explicitly weaker than successful consumed
  execution.

## 8. Test Coverage

The focused application-preparation suite now includes 16 passing tests. New
coverage proves:

- fresh drop is zero-write;
- fresh pre-run cancellation is zero-entry and zero-write;
- fresh backend substitution fails closed;
- fresh actor substitution fails closed;
- fresh immutable-bundle substitution fails closed;
- fresh invocation/executor substitution fails closed;
- invalid opening shape fails before issuance; and
- incompatible trusted-time context fails before issuance.

Existing coverage continues to prove fresh prepare-and-run, existing-window
preparation, stale preparation rejection, bounded application failures, and
surrounding authority and continuity behavior.

## 9. Validation

- feature-focused application-preparation tests passed: 16 passed;
- feature-focused clippy passed with warnings denied;
- `cargo fmt --all --check` passed;
- `cargo clippy --workspace --all-targets -- -D warnings` passed;
- `cargo test --workspace` passed;
- `npm run check:docs` passed; and
- `git diff --check` passed.

## 10. Remaining Limitations

- No production application or caller can obtain the private prepared pair.
- Cooperative cancellation cannot interrupt an admitted attempt.
- The pair is not durable across process loss.
- SQLite remains the only backend in this private composition.
- Preparation does not reserve opening identifiers or authority.
- Successful preparation does not guarantee successful later execution.

## 11. Recommended Next Phase

Perform a focused maintainer/security re-review of this proof blocker fix.
Verify the direct authority-backed test matrix, the new read-only semantic
preflight, stable non-leaking failures, and preservation of atomic consumed-
run authority use. Do not begin local-host caller composition before review
acceptance.

Fix-forward: the focused [maintainer/security
review](TRUSTED_HOST_LOCAL_APPLICATION_SESSION_PREPARATION_FRESH_PROOF_BLOCKER_FIX_REVIEW.md)
accepts the blocker fix. The direct proof and read-only preflight are
sufficient to proceed to local-host caller composition planning while atomic
authority consumption and durable identity uniqueness remain at consumed
execution.

## 12. Governed Fix Record

- workflow: `dg/blocker`
- run: `run-1791385824190119000-2`
- approval: `approval/run-1791385824190119000-2/fix-approved`
- presentation: `presentation/4125b17ad4802d78`
- presentation hash:
  `4125b17ad4802d783d55215cb974e57297e71b5089d94ca78e20ec5a50ffe40c`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: direct fresh preparation proof blocker fix only
- phase status: completed
- validation summary: focused feature tests and clippy, workspace formatting,
  clippy and tests, documentation, and diff hygiene passed
- event summary: recorded by the governed phase close with one proof-enforced
  approval and no retry or escalation
- out-of-kernel work: code and documentation edits, validation commands, and
  later git or pull-request work
