# Trusted-Host Local Application Session Preparation Report

## 1. Executive Summary

The private trusted-host local application session-preparation boundary is
implemented behind the existing non-default
`trusted-host-application-spi` feature. Core can now validate one explicit
fresh or existing operational posture without consuming authority, issue one
opaque session-and-cancellation pair, create redispatch identity internally,
and revalidate the exact prepared state before the consumed session enters the
existing authoritative operational path.

The phase also closes the cancellation failure boundary. Application-facing
cancellation now returns the same fixed, payload-free failure vocabulary as
session execution.

This is not an application lifecycle. No package, executable, production
caller, discovery mechanism, scheduler, signal integration, provider action,
or write path is added.

## 2. Scope Completed

- Added an explicit private preparation input with separate fresh and existing
  posture variants.
- Added a private opaque prepared pair with consuming `into_parts` ownership.
- Added read-only preparation validation against the exact SQLite backend,
  run, immutable bundle, workflow, step, actor, window, invocation, executor,
  and skill input.
- Added a deterministic preparation commitment over current run and
  continuity posture.
- Added authoritative revalidation of that commitment at session consumption.
- Moved production redispatch identity construction inside Core.
- Added cancellation injection to the explicit local process owner.
- Projected cancellation errors into bounded application failures.
- Preserved one-shot session ownership, Debug redaction, and feature isolation.

## 3. Scope Explicitly Not Completed

This phase does not add:

- a local-host package, executable, command, or production caller;
- discovery, queue polling, scheduling, signal handling, or detached work;
- application-selected locators, authority, identity providers, or retry
  directives;
- filesystem or PostgreSQL translation;
- provider reads or mutations, OpenShell, or nested harness execution;
- schemas, SDK behavior, hosted parity, or release changes; or
- default feature activation or a stable compatibility promise.

## 4. Preparation Boundary

`prepare_trusted_host_local_application_session` is crate-private. It accepts
one borrowed SQLite backend, one exact operational locator, one explicit
fresh-or-existing posture, one attempt executor, and one validated skill
input. It returns no application value if current state is missing,
ambiguous, substituted, or ineligible.

The returned wrapper is private, non-cloneable, non-serializable, and
Debug-redacted. Consuming it yields one one-shot session and its paired
cooperative cancellation handle.

## 5. Read-Only Preparation And Revalidation

Preparation rehydrates the run and reads continuity state, but does not open a
window, consume authority, consume a resume directive, allocate an attempt,
append an event, or invoke the executor. It commits the exact current posture
into a domain-separated hash.

When `run` is consumed, Core rehydrates the state again and compares the newly
computed commitment with the prepared commitment before any authority use.
Changed run or continuity posture therefore fails closed. Existing atomic
opening and resume operations remain the authoritative authority-consuming
edges; preparation is not an authorization grant.

## 6. Fresh And Existing Posture

The private input uses separate variants:

- `Fresh` requires exact opening and persistence context and is valid only
  when no matching continuity window exists.
- `Existing` requires exactly one matching continuity window and rejects
  absent or ambiguous state.

A fresh posture cannot silently resume an existing window, and an existing
posture cannot silently create one. The fresh variant is structurally
implemented, but no accepted production current-authority source yet constructs
it for an application caller.

## 7. Cancellation Failure Boundary

`request_cancellation` now returns
`TrustedHostLocalApplicationFailure`. Internal lock or state errors are
consumed and projected to fixed codes. The application boundary receives no
private message, diagnostic, source, identifier, path, or payload.

Cancellation remains cooperative and idempotent. Cancellation before session
entry produces a bounded canceled-before-entry outcome with no executor call
or continuity mutation. This phase does not add active-attempt interruption or
revocation.

## 8. Security And Privacy

- The same SQLite backend object is required for fresh opening context.
- Exact workflow, run, step, actor, bundle, window, invocation, executor, and
  skill bindings are committed and revalidated.
- Application code cannot inject redispatch identity providers.
- Session, handle, prepared pair, locator, capabilities, and commitments are
  not serialized across this boundary.
- Debug output for the prepared pair, session, and handle is redacted.
- Application failures expose fixed classifications only.
- No provider payload, command output, credential, source text, or raw state
  is copied into the application values.

## 9. Test Coverage

Focused feature tests cover:

- existing-window preparation is read-only;
- dropping an unconsumed pair creates no state or event change;
- consuming one prepared session invokes the executor once;
- cancellation before entry invokes no executor and changes no continuity
  state;
- state changed after preparation fails through bounded security failure;
- cancellation lock failure is payload-free;
- one-shot session, cancellation, Debug, and outcome behavior remains intact;
  and
- feature-gated clippy passes with warnings denied.

Existing operational-entry coverage continues to exercise locator, window,
bundle, actor, invocation, executor, and skill-binding mismatch behavior.

## 10. Validation

- feature-focused clippy passed with warnings denied;
- feature-focused SPI and preparation tests passed;
- `cargo fmt --all --check` passed;
- `cargo clippy --workspace --all-targets -- -D warnings` passed;
- `cargo test --workspace` passed;
- `npm run check:docs` passed; and
- `git diff --check` passed.

## 11. Remaining Limitations

- No application or production caller can obtain the private prepared pair.
- No accepted production current-authority source constructs fresh-opening
  preparation context for an application.
- Direct fresh-opening preparation coverage is therefore deferred; this phase
  does not create synthetic production authority merely to expose it.
- Cooperative cancellation does not interrupt an already admitted attempt.
- The prepared pair cannot survive process loss and is intentionally not a
  durable lease.
- SQLite remains the only backend in this private composition.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of session preparation. Review
read-only behavior, posture separation, preparation commitment completeness,
run-time revalidation order, Core-owned identity, cancellation failure
projection, zero-write drop/cancel behavior, and the continued absence of a
production caller before planning a local-host package.

## 13. Governed Implementation Record

- workflow: `dg/implement`
- run: `run-1791382068770367000-2`
- approval: `approval/run-1791382068770367000-2/implementation-approved`
- presentation: `presentation/0eb086efbf54b95d`
- presentation hash:
  `0eb086efbf54b95dc596b9d3fdada2784c4efc914d93bbabbdf261e93f80b427`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: private Core session preparation and bounded
  cancellation failure only
- phase status: completed
- event summary: 39 events, one approval, zero retries, zero escalations;
  presentation proof enforced with one presentation record
- validation summary: feature-focused clippy/tests, workspace formatting,
  clippy, tests, documentation, and diff hygiene passed
- out-of-kernel work: code and documentation edits, validation commands, and
  later git or pull-request work
