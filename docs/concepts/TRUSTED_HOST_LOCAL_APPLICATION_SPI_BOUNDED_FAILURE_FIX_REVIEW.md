# Trusted-Host Local Application SPI Bounded-Failure Fix Review

## 1. Executive Verdict

**Blocker fixed; proceed to trusted-host local application session-preparation
planning.**

The original blocker is resolved. `TrustedHostLocalApplicationSession::run`
no longer returns the broad `WorkflowOsError`. Every internal Core error kind
is exhaustively projected inside Core to a fixed, payload-free,
non-serializable `TrustedHostLocalApplicationFailure` before the result crosses
the feature-gated application SPI.

One pre-adoption constraint remains: the public cancellation handle still
returns `WorkflowOsError`. Production code cannot currently construct that
handle, so this does not reopen the reviewed `run` blocker or create an active
operational leak. Session-preparation planning must require the cancellation
path to use the same bounded failure contract before any production helper can
issue the handle across the crate boundary.

## 2. Scope Verification

The fix stayed within the approved blocker scope. It added one fixed failure
type, private error-kind projection, feature-gated re-exports, regression
tests, and honest documentation.

It did not add a session-preparation helper, local-host package, executable,
operational caller, discovery, state bridge, provider behavior, writes,
schemas, SDK behavior, hosted parity, or release changes.

## 3. Public Failure Contract Assessment

`TrustedHostLocalApplicationFailure` is a closed enum with seven variants:

- parse;
- validation;
- unsupported;
- policy denied;
- invalid state;
- security; and
- internal.

The type has no caller-supplied or internal string field. Its `code`, Debug,
and Display surfaces are derived solely from fixed literals owned by Core.
It implements `Error` without retaining a source error.

This is an appropriately small unstable application-boundary contract. It
preserves a useful failure category without exporting the private Core error
object.

## 4. Projection Assessment

The session runner remains private and may use `WorkflowOsError` internally.
The consuming public `run` method maps that result immediately through a
private `from_core_error` function.

The match covers every `WorkflowOsErrorKind` without a wildcard arm. Adding a
new Core error kind therefore requires an explicit projection decision before
the feature build can compile. The original error is borrowed only for its
kind and is dropped after mapping; its code, message, diagnostics, source,
path, identifiers, and payload are not retained.

## 5. Debug, Display, And Serialization Assessment

Debug contains the public type name and fixed code only. Display returns the
same fixed code. Neither surface can format the private error.

The failure type derives no serde traits. A feature-enabled compile-fail doc
test proves callers cannot serialize it through `serde_json`. The type also
provides no accessor to the original error or arbitrary text.

## 6. Privacy And Security Assessment

The regression test injects secret-like private code and message text for
every internal error kind and proves the public Debug and Display output omit
that material. The structural no-payload enum is stronger than redacting an
arbitrary error after it crosses the boundary.

The fix does not claim that internal Core errors are free of sensitive data.
It correctly constrains what the experimental application SPI may reveal.

## 7. Feature Isolation And Authority Assessment

The failure type, session, handle, and root re-exports remain behind the
non-default `trusted-host-application-spi` feature. No workspace package
enables the feature for an operational path.

Feature enablement still grants no execution authority. Production code has
no constructor for a session or cancellation handle, and the fix adds none.
The existing opaque construction and one-shot session boundaries are
unchanged.

## 8. Compatibility Assessment

The return type change is intentionally breaking only within an explicitly
unstable, non-default SPI that has no production consumer. Correcting the
failure contract before adoption is preferable to preserving the unsafe
shape.

Default CLI, hosted, filesystem, PostgreSQL, provider, schema, and SDK
behavior remain unchanged.

## 9. Test Quality Assessment

The focused tests prove:

- exhaustive mapping for every current `WorkflowOsErrorKind`;
- secret-like private error code and message non-leakage;
- fixed Debug and Display behavior;
- stable public codes;
- serialization rejection;
- unchanged one-shot session behavior;
- unchanged bounded outcomes;
- unchanged cancellation behavior; and
- unchanged private-owner wrapping.

Feature-focused clippy, unit tests, the private-owner test, and compile-fail
doctests pass. Full workspace clippy and tests also pass.

## 10. Documentation Assessment

The roadmap, original implementation report, original review, and fix report
preserve the history accurately. The original blocker finding was not erased.
The fix report does not overclaim a constructor, host package, caller, or
operational application path.

## 11. Remaining Pre-Adoption Constraint

`TrustedHostLocalApplicationCancellationHandle::request_cancellation`
continues to return `WorkflowOsError`. The current private timer path emits a
bounded state error, and no production caller can obtain the handle, so this
does not invalidate the completed `run` fix.

It is nevertheless the same broad cross-crate type shape that caused the
original blocker. Before a production preparation helper can issue a session
and handle pair, planning and implementation must project cancellation failure
to fixed application vocabulary and add equivalent non-leakage coverage.

## 12. Blockers

None for acceptance of the bounded-failure fix.

Operational cross-crate issuance remains blocked until the preparation-helper
phase addresses the cancellation failure return and all current binding,
authority, backend, and trusted-time requirements from the topology plan.

## 13. Non-Blocking Follow-Ups

- Add external-consumer feature-matrix coverage before another crate depends
  on the SPI.
- Require bounded cancellation failure before issuing a production handle.
- Move the private owner-to-public outcome mapping into production Core code
  only when the preparation helper needs it.
- Repeat during-wait and post-admission cancellation tests through the future
  prepared session pair.

## 14. Recommended Next Phase

Plan the trusted-host local application session-preparation helper.

The plan should identify the exact explicit inputs and private Core boundary
that can rehydrate current SQLite state, validate immutable run and authority
bindings, construct the existing private owner, and return one opaque session
and cancellation handle. It must include bounded cancellation failure as a
precondition and must not add the helper, application package, executable,
caller, discovery, provider behavior, writes, schemas, SDK behavior, hosted
parity, or release changes during planning.

## 15. Validation

- repository and feature-activation inspection: passed;
- public construction-path inspection: passed;
- exhaustive failure-projection inspection: passed;
- feature-focused clippy, unit, private-owner, and compile-fail doc tests:
  passed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## 16. Governed Review Record

- workflow: `dg/review`
- run: `run-1791380472606166000-2`
- approval: `approval/run-1791380472606166000-2/review-scope-approved`
- presentation: `presentation/587964801117bb36`
- presentation hash:
  `587964801117bb36800305d9bfa86cafe1c295c2f9d17551c6bfb865c2cbc3f5`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused bounded-failure fix review only
- phase status: completed
- event summary: 39 events, one approval, zero retries, and zero escalations;
  presentation proof enforced with one persisted record and matching event
  marker
- validation summary: feature-focused clippy, unit tests, private-owner tests,
  and compile-fail doctests plus workspace formatting, clippy, tests, docs, and
  diff hygiene passed
- out-of-kernel work: source, feature graph, tests, plans, reports, validation,
  and later git or pull-request work
