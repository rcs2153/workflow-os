# Trusted-Host Local Application Session Preparation Review

## 1. Executive Verdict

**Needs blocker fixes before local-host application composition.**

The implementation preserves the intended private, feature-gated boundary,
projects cancellation failures safely, keeps preparation read-only, and
revalidates a deterministic commitment before authority use. Existing-window
preparation, however, issues the opaque pair before validating three exact
bindings required by the accepted plan: persisted subject actor, persisted
immutable run bundle, and persisted operation binding against the current
invocation/executor commitment.

Those substitutions fail later when the session is consumed, so no invalid
authority reaches the executor. That is necessary but not sufficient. The
approved preparation contract requires invalid input to return no pair.

The fresh-opening variant is explicit and private but has no direct
preparation test. Before any application caller is introduced, a private
test-only authority fixture must prove fresh preparation is read-only,
droppable without writes, and consumable exactly once without exposing a
production construction path.

## 2. Scope Verification

The implementation stayed within the approved Core-only scope.

It added private preparation types and validation, a private opaque prepared
pair, Core-owned redispatch identity construction, bounded cancellation
failure, existing-window tests, and documentation.

It did not add a local-host package, executable, command, production caller,
discovery, scheduling, signal handling, provider behavior, writes, schemas,
SDK behavior, hosted parity, nested harnesses, OpenShell, or release changes.

## 3. Preparation API Assessment

The crate-private helper is appropriately narrow. It accepts one backend,
one exact locator, one explicit posture, one executor, and one validated skill
input. The fresh and existing variants avoid ambiguous combinations of
optional fields.

The prepared wrapper remains private and exposes only a consuming
`into_parts`. It is non-cloneable, non-serde, and Debug-redacted. No public or
cross-crate constructor was added.

## 4. Read-Only Boundary Assessment

Existing-window tests prove preparation does not mutate continuity state,
append workflow events, or invoke the executor. A separate test proves that
dropping the unconsumed pair has the same zero-write posture.

Cancellation before `run` produces the bounded canceled-before-entry outcome,
does not invoke the executor, and leaves continuity state unchanged.

Preparation therefore remains a read-only eligibility check, not an authority
grant or attempt admission edge.

## 5. Posture Separation Assessment

Fresh and existing posture are represented by explicit private variants.

- Fresh posture fails when a matching window already exists.
- Existing posture fails when no matching window exists.
- Multiple matching windows fail closed.

This prevents silent fresh-to-resume or resume-to-fresh reinterpretation. The
structural design is accepted.

## 6. Existing-Window Binding Blocker

`current_preparation_context` checks that the persisted window identity equals
the locator window identity. It then reads the opening operation binding and
continuation disposition for the preparation commitment.

It does not call the existing exact binding validation before issuing the
pair. As a result, preparation does not reject:

- a locator subject actor that differs from the persisted window actor;
- a locator immutable bundle that differs from the persisted window bundle
  when the run snapshot has been made self-consistent with that locator; or
- a skill input or executor binding whose expected operation binding differs
  from the persisted opening operation binding.

Consumed `run` calls `existing_window_binding` and rejects these cases before
attempt capability issuance. That protects execution, but the pair has already
crossed the preparation boundary. The plan and its review explicitly require
every such substitution to fail before issuance.

The blocker fix must reuse or factor the existing exact binding validator at
preparation time and add direct tests that assert no pair is returned and no
state, event, directive, attempt, identity, or executor action occurs.

## 7. Fresh-Opening Proof Blocker

The implementation includes a private fresh variant and validates backend
object identity, execution binding, locator identity, window identity, and
invocation commitment structurally. No test constructs the accepted current
authority source and opening context through this preparation helper.

The implementation report discloses this limitation honestly, but the
accepted implementation test plan required direct fresh preparation proof.
Before an application caller can rely on this branch, a private test-only
fixture must prove:

- valid fresh preparation performs no durable write or authority use;
- dropping the pair changes no state or event;
- cancellation before consumed `run` remains zero-entry;
- one consumed session opens and enters exactly once; and
- substituted backend, actor, bundle, invocation, executor, trusted-time, and
  opening identities fail at the intended boundary without leakage.

The fixture must not add a production authority factory or broaden the SPI.

## 8. Preparation Commitment Assessment

The domain-separated commitment includes the run snapshot commitment,
invocation commitment, locator identities, current window record,
continuation disposition, and persisted operation binding. That is sufficient
to detect the tested state change between preparation and consumed `run`.

At `run`, Core recomputes current preparation context before opening or
resuming. A mismatch returns a bounded security failure. The existing atomic
opening/resume operations remain responsible for races after this recheck.

This defense-in-depth design is accepted, subject to completing the
pre-issuance binding checks above.

## 9. Identity Ownership Assessment

The prepared runner creates `TrustedHostLocalProductionIdentitySource` inside
Core and obtains the redispatch provider privately. Application code cannot
inject the provider, random material, or generated identities.

The visibility increase remains crate-private and does not expose identity
construction through the feature-gated application SPI. This is accepted.

## 10. Cancellation And Failure Assessment

`request_cancellation` now returns the fixed
`TrustedHostLocalApplicationFailure` vocabulary. The original Core error is
consumed, and Debug/Display expose only a stable bounded code.

The poison regression test demonstrates that private lock failure text does
not cross the handle boundary. The type remains non-serializable.

Cancellation is cooperative and does not claim to interrupt an admitted
attempt. That limitation is accurately documented and is not a blocker for
this preparation slice.

## 11. Privacy And Feature Isolation Assessment

The prepared pair, session, and handle redact Debug output and do not carry
raw provider payloads, command output, credentials, source text, or serialized
Core errors.

The preparation module remains behind the non-default
`trusted-host-application-spi` feature, and no workspace application enables
or consumes the helper. Feature visibility is correctly treated as packaging,
not authority.

## 12. Test Quality Assessment

Strong coverage exists for:

- bounded run and cancellation failures;
- read-only existing-window preparation;
- zero-write drop;
- zero-entry pre-run cancellation;
- stale state between preparation and run;
- one consumed existing-window executor entry;
- Debug redaction and one-shot behavior; and
- default workspace non-regression.

Blocking omissions are direct existing-window substitution-at-preparation
tests and direct fresh-preparation tests. Broader operational-entry tests do
not replace tests at the newly introduced issuance boundary.

## 13. Documentation Assessment

The plan, roadmap, and implementation report accurately describe the private
feature-gated implementation and avoid claims of an application package,
caller, scheduler, provider behavior, or write support.

The implementation report correctly discloses the missing fresh-source test.
This review adds the stronger conclusion that direct proof is required before
application composition.

## 14. Blockers

1. Complete existing-window actor, immutable-bundle, and operation-binding
   validation before the prepared pair is issued, with direct no-write and
   no-entry substitution tests.
2. Add a private test-only fresh-opening fixture and direct preparation/drop/
   cancellation/one-run proof without exposing a production authority source.

## 15. Non-Blocking Follow-Ups

- Add an external-consumer feature fixture before a separate package depends
  on the SPI.
- Repeat during-wait and post-admission cancellation behavior through a later
  real application boundary.
- Keep the prepared wrapper private until a concrete embedding package proves
  a public need.
- Preserve the explicit statement that successful preparation does not
  guarantee later successful execution.

## 16. Recommended Next Phase

Implement one focused blocker fix only:

1. factor exact existing-window binding validation into the read-only
   preparation path;
2. add direct pre-issuance substitution tests;
3. add a private test-only current-authority fixture for fresh preparation;
4. prove fresh prepare/drop/cancel/run behavior; and
5. rerun feature-focused and workspace validation.

Do not add a host package, executable, production caller, discovery,
scheduling, provider behavior, writes, schemas, SDK behavior, hosted parity,
nested harnesses, OpenShell, or release changes.

## 17. Validation

- feature-focused clippy passed with warnings denied;
- feature-focused SPI and preparation tests passed;
- `cargo fmt --all --check` passed;
- `cargo clippy --workspace --all-targets -- -D warnings` passed;
- `cargo test --workspace` passed;
- `npm run check:docs` passed; and
- `git diff --check` passed.

## 18. Governed Review Record

- workflow: `dg/review`
- run: `run-1791383594082362000-2`
- approval:
  `approval/run-1791383594082362000-2/review-scope-approved`
- presentation: `presentation/2dfa3cb686d39114`
- presentation hash:
  `2dfa3cb686d39114389c0a4725340b1b024a0e9869b45046ef413eefa897ca10`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused session-preparation maintainer/security review
  only
- phase status: completed
- event summary: 39 events, one approval, zero retries, and zero escalations;
  presentation proof enforced with one persisted record and matching event
  marker
- validation summary: feature-focused Rust checks, workspace formatting,
  clippy and tests, documentation, and diff hygiene passed
- out-of-kernel work: implementation and test inspection, review authoring,
  validation commands, and later git or pull-request work

## 19. Fix-Forward Note

The two blockers identified by this review are now addressed in
[Trusted-Host Local Application Session Preparation Blocker Fix Report](TRUSTED_HOST_LOCAL_APPLICATION_SESSION_PREPARATION_BLOCKER_FIX_REPORT.md).
This review remains the authoritative record of the original findings. A
separate focused blocker-fix review must verify the correction before any
local-host package or caller composition begins.
