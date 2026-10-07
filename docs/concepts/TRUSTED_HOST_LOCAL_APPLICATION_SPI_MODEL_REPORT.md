# Trusted-Host Local Application SPI Model Report

## 1. Executive Summary

The first trusted-host local application topology implementation slice is
complete. `workflow-core` now exposes an explicitly unstable local application
SPI only when the non-default `trusted-host-application-spi` Cargo feature is
enabled.

The SPI defines an opaque one-shot session, cooperative cancellation-handle
custody, and fixed bounded owner outcomes. Enabling the feature grants no
authority: production code has no public or crate-private session-preparation
constructor in this slice. No application package or runtime caller exists.

## 2. Scope Completed

- Added the empty, non-default `trusted-host-application-spi` feature.
- Added `TrustedHostLocalApplicationSession` with consuming one-shot `run`.
- Added `TrustedHostLocalApplicationCancellationHandle` over the accepted
  cooperative cancellation primitive.
- Added bounded entry and continuation stop vocabularies.
- Re-exported the SPI from `workflow-core` only when the feature is enabled.
- Added redacted custom Debug implementations.
- Added focused model, privacy, cancellation, and private-owner wrapping tests.
- Added compile-fail documentation tests proving the session cannot be cloned
  or serialized.

## 3. Scope Explicitly Not Completed

This phase does not add:

- a production session or cancellation-handle constructor;
- a session-preparation helper or authoritative source;
- `workflow-local-host` or any other application package;
- an executable, CLI command, SDK, schema, or configuration;
- discovery, polling, startup scanning, detached execution, or signals;
- a filesystem, SQLite, or PostgreSQL state bridge;
- hosted parity, provider behavior, writes, OpenShell, or nested harnesses;
- operational adoption or release posture changes.

## 4. Feature And Visibility Boundary

The feature is absent from default builds and has no dependencies. Ordinary
workspace consumers, the CLI, and hosted runtime do not enable it. Feature
visibility is only a packaging guard; authority remains protected by private
construction and existing Core validation.

The model's production surface can consume a session but cannot create one.
Test-only constructors prove the intended composition against the private
process owner without creating ambient runtime authority.

## 5. Session And Cancellation Model

The session owns a private one-shot runner and consumes `self` on `run`. It is
not cloneable, serializable, or deserializable. Its Debug output exposes only a
redacted binding marker.

The cancellation handle wraps the existing local timer cancellation handle.
It permits idempotent cooperative cancellation but does not claim revocation
or active-attempt interruption. Its Debug output does not expose state.

## 6. Outcome Boundary

The public outcome contains fixed classifications only:

- canceled before entry;
- entry stopped because of await-condition, blocked, or terminal posture; or
- continuation stopped because of canceled, blocked, terminal, unsupported
  wait, or exhausted wake budget.

It does not expose workflow, run, step, window, actor, bundle, capability,
input, deadline, path, provider, credential, or raw-error values. A stop
classification is not translated into workflow completion or failure.

## 7. Privacy And Security

- No public constructor or caller-authored authority exists.
- Session and cancellation Debug output is redacted.
- Outcome Debug output contains fixed enum vocabulary only.
- Compile-fail tests reject cloning and serde serialization.
- No unsafe code, dependency, payload storage, or serialization was added.
- Default builds do not expose the SPI.

## 8. Test Coverage

Focused tests prove:

- a session consumes exactly one runner;
- fixed outcomes remain bounded;
- session and handle Debug are redacted;
- cancellation is idempotent and wins before entry;
- a feature-gated session can wrap the real private process owner in Core
  tests while preserving zero-write pre-entry cancellation; and
- clone and serialization attempts fail to compile.

## 9. Validation

- feature-focused `cargo clippy` with `-D warnings`: passed;
- feature-focused unit, owner-wrapping, and compile-fail doc tests: passed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

Rust validation uses `CARGO_INCREMENTAL=0` and clean target directories under
`/private/tmp`.

## 10. Remaining Limitations

- No production code can issue a session yet.
- The future preparation helper must validate all current authoritative
  bindings before constructing either value.
- No application owns invocation or shutdown.
- Process loss, signals, discovery, and restart ownership remain unresolved.
- Feature enablement is not a security boundary.

## 11. Recommended Next Phase

Perform a focused maintainer/security review of the Core SPI model and
visibility slice. Review feature isolation, opaque construction, one-shot
semantics, cancellation custody, bounded outcomes, redaction, and the absence
of operational adoption before planning or implementing a preparation helper.

## 12. Governed Implementation Record

- workflow: `dg/implement`
- run: `run-1791378038957903000-2`
- approval: `approval/run-1791378038957903000-2/implementation-approved`
- presentation: `presentation/c0a7d8118ecd838c`
- presentation hash:
  `c0a7d8118ecd838cfbc55a6b78b404129fbf044c56b1d51d4f9b42a6fa0677b8`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: Core SPI model and visibility slice only
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval presentation enforcement: proof enforced with one persisted
  presentation record and an approval-event proof marker
- validation summary: all focused feature checks, workspace Rust checks, docs
  checks, and diff checks passed
- out-of-kernel work: source inspection, code and documentation edits,
  validation commands, and later git or pull-request work

## 13. Fix-Forward Note

The subsequent maintainer/security review found that the public session
returned the complete `WorkflowOsError`, which exceeded the accepted bounded
cross-crate failure contract. The focused bounded-failure fix now projects
private Core errors into `TrustedHostLocalApplicationFailure` before they
cross the SPI. The original implementation scope and validation record above
remain unchanged.
