# Trusted-Host Local Application SPI Bounded-Failure Fix Report

## 1. Executive Summary

The trusted-host local application SPI failure-boundary blocker is fixed for
review. `TrustedHostLocalApplicationSession::run` no longer returns the broad
`WorkflowOsError` type. It now projects every private Core error kind into a
fixed, payload-free `TrustedHostLocalApplicationFailure` before returning to
the application boundary.

The phase remains model-only. It does not add a production constructor,
preparation helper, host package, executable, caller, or operational behavior.

## 2. Blocker Fixed

The maintainer/security review found that `WorkflowOsError` publicly exposes
message text, diagnostics, Debug, Display, and serde. Returning that complete
type meant the cross-crate SPI could not guarantee bounded failure disclosure
as deeper private Core paths evolve.

The public session result now contains only a fixed application failure enum.
The original Core error is consumed inside `run` and is not retained.

## 3. Implementation Approach

- Added `TrustedHostLocalApplicationFailure` behind the existing non-default
  feature.
- Kept the private session runner's internal `WorkflowOsError` result.
- Mapped the private error kind exhaustively inside the consuming `run` call.
- Returned only the fixed projected failure to the caller.
- Added fixed stable codes for each public failure variant.
- Added bounded Debug and Display implementations.
- Deliberately omitted serde from the public failure type.
- Re-exported the failure alongside the existing feature-gated SPI types.

## 4. Failure Vocabulary

The public fixed variants are:

- parse;
- validation;
- unsupported;
- policy denied;
- invalid state;
- security; and
- internal.

The projection does not preserve the original error code, message,
diagnostics, source, path, identifier, payload, or serialized error.

## 5. Security And Privacy

The application boundary now structurally excludes caller-visible private
error text. Debug and Display contain only the fixed application code. The
failure type is non-serializable and carries no string field.

This phase does not claim that internal Core errors lack sensitive content. It
ensures that such content cannot cross this SPI through the public session
result.

## 6. Compatibility And Scope

The SPI remains explicitly unstable and feature-gated. No existing default
consumer enabled the feature, and no operational caller exists. The return
type correction therefore changes only the reviewed experimental surface.

No session construction, application composition, invocation source,
discovery, bridge, provider behavior, write behavior, schema, SDK, hosted, or
release surface was added.

## 7. Test Coverage

Focused tests cover:

- exhaustive projection of every `WorkflowOsErrorKind`;
- removal of secret-like internal code and message text;
- fixed Debug and Display output;
- stable application failure codes;
- compile-time serialization rejection;
- existing one-shot session behavior;
- existing bounded success outcomes;
- existing cancellation behavior; and
- existing private-owner wrapping behavior.

## 8. Validation

- feature-focused clippy passed with warnings denied;
- feature-focused unit and private-owner tests passed;
- feature-focused compile-fail doc tests passed;
- `cargo fmt --all --check` passed;
- `cargo clippy --workspace --all-targets -- -D warnings` passed;
- `cargo test --workspace` passed;
- `npm run check:docs` passed; and
- `git diff --check` passed.

## 9. Remaining Limitations

- No production code can construct a session or cancellation handle.
- No preparation helper validates and binds current authoritative inputs.
- No application package or process owns invocation or shutdown.
- External-consumer feature-matrix coverage remains a non-blocking follow-up.
- Prepared-session cancellation semantics still require later direct coverage.

## 10. Recommended Next Phase

Perform a focused maintainer/security review of the bounded-failure fix.
Review exhaustive mapping, payload elimination, Debug and Display safety,
non-serialization, feature isolation, and the continued absence of a
production constructor before allowing session-preparation work.

## 11. Governed Fix Record

- workflow: `dg/blocker`
- run: `run-1791379649379657000-2`
- approval: `approval/run-1791379649379657000-2/fix-approved`
- presentation: `presentation/382c714b48fcd10b`
- presentation hash:
  `382c714b48fcd10b313b6ad85853ad3ab2b4ca63456509dfacec655ac47c002a`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: fixed payload-free failure projection only
- phase status: completed
- event summary: 39 events, one approval, zero retries, zero escalations;
  presentation proof enforced with one presentation record
- validation summary: feature-focused clippy, unit tests, private-owner test,
  compile-fail doctests, workspace clippy, workspace tests, formatting, docs,
  and diff hygiene passed
- out-of-kernel work: code and documentation edits, validation commands, and
  later git or pull-request work
