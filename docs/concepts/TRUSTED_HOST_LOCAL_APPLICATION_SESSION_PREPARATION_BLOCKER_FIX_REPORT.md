# Trusted-Host Local Application Session Preparation Blocker Fix Report

## 1. Executive Summary

The two blockers from the trusted-host local application session-preparation
review are fixed within the private, feature-gated Core boundary.

Existing-window preparation now performs the same exact actor, immutable-run-
bundle, and operation-binding validation used by consumed execution before it
issues the opaque session-and-cancellation pair. A private test-only current-
authority fixture now proves the fresh branch is read-only during preparation
and opens and enters exactly once when the session is consumed.

This fix does not add a host package, executable, caller, discovery,
scheduling, provider behavior, writes, schemas, SDK behavior, hosted parity,
nested harnesses, OpenShell, or release changes.

## 2. Blockers Fixed

1. Existing-window preparation no longer records an unchecked persisted
   operation binding. It calls the existing exact binding validator before
   pair issuance.
2. Fresh preparation now has direct authority-backed proof through a
   `#[cfg(test)]`-only fixture. No production authority factory was added.

## 3. Implementation Approach

The production correction is deliberately small. The existing preparation
branch now reuses `existing_window_binding`, which already validates:

- persisted subject actor equals the locator actor;
- persisted immutable run bundle equals the locator bundle; and
- persisted opening operation binding equals the operation binding derived
  from the current workflow, run, step, skill input, and executor commitment.

The validated operation-binding commitment is then included in the existing
preparation commitment. Consumed `run` still recomputes the complete current
preparation context, so the pre-issuance check does not replace run-time
revalidation or the atomic authority-consuming store operations.

## 4. Pre-Issuance Validation Boundary

Direct tests now prove that:

- actor substitution fails with the stable window-binding mismatch code;
- immutable-bundle substitution fails during run-eligibility validation;
- invocation or executor-binding substitution fails with the stable
  invocation-binding mismatch code; and
- each failure occurs without continuity-state mutation, workflow-event
  append, executor entry, or returned prepared pair.

The error messages remain fixed and do not include actor, bundle, invocation,
executor, path, payload, or state values.

## 5. Fresh Preparation Proof

The current-authority source test module now exposes only two crate-private,
test-only helpers: one validated required-context fixture and one ready source.
Production visibility and construction remain unchanged.

The fresh application-preparation test proves:

- preparation leaves continuity state and workflow events unchanged;
- preparation creates no opening operation, attempt, projection binding, or
  continuity window;
- preparation does not invoke the executor;
- consuming the one-shot session creates exactly one opening operation,
  attempt, projection binding, and continuity window; and
- the executor is entered exactly once.

Existing tests continue to prove zero-write drop and cancellation behavior at
the same private prepared-pair boundary.

## 6. Security And Privacy

- Exact validation happens before an opaque pair can cross the preparation
  boundary.
- The production fix introduces no new capability, authority, identity, or
  caller construction path.
- Test fixture exports are compiled only under `cfg(test)`.
- Preparation remains read-only and consumed execution remains authoritative.
- Debug and failure surfaces remain bounded and payload-free.
- No provider payload, command output, credential, source text, or raw state
  is copied into application-facing values.

## 7. Test Coverage

Focused coverage includes:

- existing-window read-only preparation and one-run behavior;
- actor substitution before issuance;
- immutable-bundle substitution before issuance;
- invocation/executor substitution before issuance;
- fresh read-only preparation and exactly one consumed run;
- zero-write drop;
- zero-entry cancellation; and
- state drift between preparation and consumed run.

The existing Workflow OS workspace suites continue to cover the surrounding
authority, continuity, executor, application SPI, privacy, and serialization
boundaries.

## 8. Validation

- feature-focused application-preparation tests passed: eight passed;
- `cargo fmt --all --check` passed;
- feature-focused clippy passed with warnings denied;
- `cargo clippy --workspace --all-targets -- -D warnings` passed;
- `cargo test --workspace` passed;
- `npm run check:docs` passed; and
- `git diff --check` passed.

## 9. Remaining Limitations

- No application or production caller can obtain the private prepared pair.
- The fresh authority fixture is test-only and is not a production authority
  source.
- Cooperative cancellation does not interrupt an admitted attempt.
- The pair is not durable across process loss.
- SQLite remains the only backend in this private composition.
- Successful preparation does not guarantee later successful execution.

## 10. Recommended Next Phase

Perform a focused maintainer/security review of this blocker fix. The review
must verify exact pre-issuance validation, no-write substitution failures,
fresh one-use proof, test-only fixture isolation, run-time revalidation, and
the absence of any application caller or broader runtime surface.

Do not begin local-host package or caller composition before that review.

## 11. Governed Fix Record

- workflow: `dg/blocker`
- run: `run-1791384026119086000-2`
- approval: `approval/run-1791384026119086000-2/fix-approved`
- presentation: `presentation/18ad4c846bd31303`
- presentation hash:
  `18ad4c846bd3130399fba17f37c50a9b88a5578649eabb6019ed94fe1fa620be`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: session-preparation blocker fix only
- phase status: completed
- event summary: 39 events, one approval, zero retries, and zero escalations;
  presentation proof enforced with one persisted record and matching event
  marker
- validation summary: focused preparation tests, workspace formatting,
  clippy and tests, documentation, and diff hygiene passed
- out-of-kernel work: code and documentation edits, validation commands, and
  later git or pull-request work

## 12. Review-Forward Note

The focused blocker-fix review accepts the production pre-issuance binding
correction but finds that the direct fresh-path proof required by the original
review is incomplete. The authority-backed fixture proves read-only fresh
preparation and one consumed run; it does not directly exercise fresh drop,
pre-run cancellation, or fresh-context substitution behavior.

See [Trusted-Host Local Application Session Preparation Blocker Fix
Review](TRUSTED_HOST_LOCAL_APPLICATION_SESSION_PREPARATION_BLOCKER_FIX_REVIEW.md).
This note preserves the original implementation record while preventing its
opening claim from being read as final phase acceptance.

## 13. Second Fix-Forward Note

The missing direct fresh-path proof is now implemented in the focused
[Fresh Preparation Proof Blocker Fix
Report](TRUSTED_HOST_LOCAL_APPLICATION_SESSION_PREPARATION_FRESH_PROOF_BLOCKER_FIX_REPORT.md).
That phase adds authority-backed fresh drop, pre-run cancellation, backend,
actor, bundle, invocation/executor, opening-shape, and trusted-time tests.

The direct tests exposed two semantic inputs that were not validated before
pair issuance: an unusable opening request shape and trusted-time context that
did not match the current continuity instance. The preparation helper now
rejects both with stable payload-free errors while leaving atomic authority
consumption at the consumed-session boundary. This note does not erase the
original review finding; focused re-review remains required.
