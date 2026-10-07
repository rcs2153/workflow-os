# Trusted-Host Local Application Session Preparation Blocker Fix Review

## 1. Executive Verdict

Needs additional blocker fix.

The production correction is sound: existing-window preparation now validates
the persisted actor, immutable run bundle, and operation binding against the
current locator and invocation before issuing the opaque session pair. The
consumed run still recomputes the preparation commitment and retains the
atomic authority-consuming boundary.

The phase cannot yet authorize local-host composition because the accepted
fresh-path proof remains incomplete. The new authority-backed fixture directly
proves read-only fresh preparation and one consumed run, but fresh drop,
pre-run cancellation, and fresh-context substitutions are not directly tested.

## 2. Review Scope

This review inspected:

- exact existing-window validation before pair issuance;
- preparation commitment construction and consumed-run revalidation;
- actor, immutable-bundle, and invocation/executor substitution failures;
- the test-only current-authority fixture;
- fresh preparation and one-run behavior;
- drop, cancellation, and stale-state coverage;
- feature isolation and application-boundary privacy; and
- the implementation report, original review, plan, and roadmap claims.

The review did not implement runtime fixes, a host package, executable,
production caller, discovery, scheduling, provider behavior, writes, schemas,
SDK behavior, hosted parity, nested harnesses, OpenShell, a public authority
factory, or release changes.

## 3. Production Fix Assessment

The existing-window branch of `current_preparation_context` now calls
`existing_window_binding` before pair issuance. That helper verifies:

- the persisted window actor equals the locator actor;
- the persisted immutable run bundle equals the locator bundle; and
- the persisted opening operation binding equals the binding derived from the
  current workflow, run, step, skill input, and executor commitment.

The validated operation-binding commitment is included in the domain-separated
preparation commitment. This closes the original pre-issuance gap without
duplicating validation logic.

## 4. Zero-Write Substitution Assessment

Direct tests now cover existing-window substitution of:

- actor identity;
- immutable run bundle; and
- invocation or executor binding.

Each test asserts a stable bounded error, zero executor entry, unchanged
continuity state, and no workflow-event append. No prepared pair is returned.
These tests satisfy the existing-window portion of the original blocker.

The immutable-bundle substitution currently fails first through run
eligibility rather than the later window-binding check. That is acceptable:
the mismatch still fails before issuance, with no write or leaked value.

## 5. Preparation Commitment And Revalidation

Preparation commits to the current run snapshot, invocation, locator,
authoritative window record, continuation disposition, and validated persisted
operation binding. Consumed `run` recomputes the same context and compares it
with the captured commitment before opening or resuming.

The existing stale-state test proves a post-preparation state change is
rejected through the bounded application security failure. Atomic opening and
resume operations remain responsible for races after the recomputation. The
review found no bypass of consumed-run revalidation.

## 6. Fresh Authority Fixture Assessment

The fixture exposure is crate-private and compiled only under `cfg(test)` with
the trusted-host application feature. It does not add a production authority
factory or broaden the feature-gated SPI.

The fresh test correctly proves that preparation:

- leaves continuity state unchanged;
- appends no workflow event;
- creates no opening operation, attempt, projection binding, or window; and
- does not invoke the executor.

Consuming the session then creates one opening operation, one attempt, one
projection binding, and one continuity window, and enters the executor once.

## 7. Remaining Fresh-Path Proof Blocker

The original review required the private fresh fixture to prove fresh
preparation, drop, cancellation, and one consumed run. It also required direct
fresh-context substitution coverage at the intended validation boundary.

The implementation adds only the fresh preparation-plus-run case. The drop and
pre-run cancellation tests still construct an existing-window posture. Those
tests establish generic wrapper behavior, but they do not prove that dropping
or canceling a prepared fresh closure leaves its current-authority source,
opening records, trusted-time state, continuity state, workflow events, and
executor untouched.

No direct fresh test substitutes the backend, actor, bundle, invocation or
executor binding, trusted-time context, or opening identities. Existing lower-
level opening tests are valuable but do not replace proof at the newly issued
prepared-pair boundary.

## 8. Security And Privacy Assessment

- Pre-issuance failures use stable codes and fixed messages.
- Tests do not expose actor, bundle, invocation, executor, path, payload, or
  state values.
- Prepared pair, session, handle, locator, and capabilities remain private and
  non-serializable.
- Test-only authority helpers do not compile into production code.
- No provider payload, command output, credential, source text, or raw state is
  copied into application-facing values.
- Successful preparation remains explicitly distinct from successful later
  execution.

## 9. Scope Verification

The blocker-fix implementation stayed within the approved private Core scope.
It did not add:

- a local-host package or executable;
- a production caller, discovery, or scheduling path;
- provider reads or mutations;
- schemas, CLI, SDK, or hosted behavior;
- nested harnesses or OpenShell;
- a public authority factory; or
- release posture changes.

## 10. Test Quality Assessment

Strong direct coverage now exists for existing-window pre-issuance validation,
read-only preparation, consumed-run revalidation, one-shot execution, bounded
failure projection, and workspace non-regression.

Blocking missing coverage:

1. fresh prepared-pair drop with zero authority use and zero durable change;
2. fresh pre-run cancellation with zero authority use, zero opening, and zero
   executor entry; and
3. direct fresh-context substitution cases sufficient to prove backend,
   execution binding, invocation/executor binding, trusted-time, and opening
   identity failures occur at the documented boundary without leakage.

## 11. Documentation Assessment

The implementation report accurately describes the production change and the
fresh prepare-plus-run test, but its opening statement says both blockers are
fixed. That is too broad while the original fresh proof matrix remains
incomplete. A review-forward note now preserves the historical report and
records the narrower accepted result.

The roadmap and preparation plan now keep local-host composition blocked until
the missing direct proof is implemented and reviewed.

## 12. Blockers

1. Add an authority-backed fresh prepared-pair drop test proving zero opening,
   authority use, continuity change, event append, and executor entry.
2. Add an authority-backed fresh pre-run cancellation test proving the same
   zero-entry and zero-write properties.
3. Add direct fresh-context substitution tests at the preparation boundary,
   with stable non-leaking errors and no partial pair issuance.

These are test-first blockers. Production behavior should change only if the
new direct tests expose a real validation-boundary defect.

## 13. Non-Blocking Follow-Ups

- Add an external-consumer feature fixture before a separate package depends
  on the SPI.
- Repeat during-wait and post-admission cancellation behavior through a later
  real application boundary.
- Keep the prepared wrapper private until a concrete embedding package proves
  a public need.
- Preserve the statement that successful preparation does not guarantee later
  successful execution.

## 14. Recommended Next Phase

Implement one focused test-only blocker fix:

1. reuse the current private authority fixture;
2. add fresh drop and pre-run cancellation proofs;
3. add the missing direct fresh substitution matrix;
4. change production code only if a test demonstrates a real defect; and
5. rerun feature-focused and workspace validation.

Do not begin local-host package or production caller composition first.

## 15. Validation

- feature-focused application-preparation tests passed;
- `cargo fmt --all --check` passed;
- `cargo clippy --workspace --all-targets -- -D warnings` passed;
- `cargo test --workspace` passed;
- `npm run check:docs` passed; and
- `git diff --check` passed.

## 16. Governed Review Record

- workflow: `dg/review`
- run: `run-1791385150474420000-2`
- approval: `approval/run-1791385150474420000-2/review-scope-approved`
- presentation: `presentation/5a42276488c7d5a9`
- presentation hash:
  `5a42276488c7d5a958783100ec5e4745a11d3326c0610fc48239aae383b1b7af`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused blocker-fix maintainer/security review only
- phase status: completed
- event summary: 39 events, one approval, zero retries, and zero escalations;
  presentation proof enforced with one persisted record and matching event
  marker
- validation summary: focused preparation tests, workspace formatting,
  clippy and tests, documentation, and diff hygiene
- out-of-kernel work: implementation and test inspection, review authoring,
  validation commands, and later git or pull-request work
