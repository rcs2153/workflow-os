# Trusted-Host Local Application Caller Composition Report

## 1. Executive Summary

The bounded trusted-host caller-composition slice is implemented as an
unpublished, library-only `workflow-local-host` workspace package. The package
can receive one opaque Core-issued prepared pair, retain its scoped
cooperative-cancellation state, and synchronously consume the one-shot session
on the caller's foreground thread.

Core still owns preparation, current-state rehydration, authority validation,
authority consumption, admission, continuation, and workflow truth. The new
package cannot construct a prepared pair or any authority-bearing input. No
production source or embedding caller invokes the package.

## 2. Scope Completed

- Added the unpublished `workflow-local-host` library package with no binary.
- Exposed the unconstructible prepared pair only behind Core's existing
  non-default `trusted-host-application-spi` feature.
- Added a concrete one-shot `LocalHostPreparedOperation` wrapper.
- Added cloneable `LocalHostCancellationControl` custody over one shared,
  operation-scoped cancellation state.
- Preserved existing bounded Core outcomes and failures without adding a
  second lifecycle interpretation.
- Added package-private sequencing tests, external-consumer compile proof,
  compile-fail ownership/serialization examples, and Core cancellation-clone
  coverage.

## 3. Scope Explicitly Not Completed

This phase does not add:

- a production preparation source, embedding application, binary, command, or
  SDK method;
- discovery, queue polling, scheduling, background work, signal handling, or
  process recovery;
- automatic approval, authority inference, retry, or redispatch selection;
- provider reads or mutations, OpenShell integration, or nested harnesses;
- runtime configuration, workflow schema fields, hosted parity, or release
  posture changes; or
- a stable public compatibility promise.

## 4. Package And API Boundary

`workflow-local-host` is `publish = false`, has no binary target, and depends
on Core with the reviewed non-default feature. Its production API contains
only:

- `LocalHostPreparedOperation::from_prepared`;
- `LocalHostPreparedOperation::cancellation_control`;
- `LocalHostPreparedOperation::run`; and
- `LocalHostCancellationControl::request_cancellation`.

Construction consumes a `TrustedHostLocalApplicationPreparedSession`. The
prepared type has no public constructor. Its preparation function, locator,
backend input, authority source, opening context, identity providers, executor
binding, and skill input remain private to Core.

## 5. Ownership And Execution Semantics

The operation wrapper is non-cloneable and non-serializable. `run` consumes
the wrapper, retains the paired cancellation state for the complete call, and
invokes the Core session exactly once. It returns only the existing bounded
application outcome or failure and makes no independent workflow lifecycle
claim.

Dropping an unrun wrapper does not invoke the session. The package performs no
preparation, state write, event append, evidence creation, report generation,
thread creation, process launch, or command execution.

## 6. Cancellation Custody

Cancellation controls are cloneable references to one shared in-memory
cancellation state. A clone creates no execution authority, session, window,
attempt, or cancellation domain. Repeated requests remain idempotent through
Core, and a retained control remains bounded after the foreground call has
returned.

Cancellation is cooperative. It can stop work before entry and during the
already supported timer-wait boundary. It does not revoke authority or
interrupt an attempt after executor admission.

## 7. Feature And Adoption Posture

The local-host package intentionally activates the SPI in its package graph.
Ordinary Core, CLI, and hosted package builds do not request the feature.
Workspace-wide builds compile the feature because Cargo unifies features for
the new workspace consumer; that does not make CLI or hosted an operational
caller.

No production code path constructs or invokes `LocalHostPreparedOperation`.
Operational adoption remains a separate reviewed phase.

## 8. Privacy And Failure Posture

- Prepared pair, operation, session, handle, and cancellation control use
  redacted Debug output.
- Operation and prepared-pair ownership cannot be cloned or serialized.
- Existing Core failures remain fixed and payload-free.
- No workflow identity, authority binding, path, payload, command output,
  provider value, credential, event, artifact, or report is stored by the
  package.
- The package adds no new error vocabulary and does not retain private Core
  errors.

## 9. Test Coverage

Focused tests cover:

- one foreground session consumption with bounded outcome propagation;
- shared scoped cancellation across cloned controls;
- drop without session invocation;
- bounded cancellation after the foreground call returns;
- external consumption of the feature-gated prepared type;
- compile-time rejection of cloning and serialization for the prepared pair;
- compile-time rejection of operation cloning;
- redacted Core session, handle, and prepared-pair Debug behavior; and
- redacted public composition types and bounded fixed failures.

Validation separately inspects package-specific feature and dependency graphs
for Core, CLI, hosted, and the local-host package.

The authoritative state, authority, substitution, zero-write, and consumed
execution proofs remain in Core rather than being replaced by a fake public
pair factory.

## 10. Validation

- focused local-host package tests passed;
- focused local-host clippy passed with warnings denied;
- focused Core SPI tests passed;
- package-specific feature-isolation checks passed;
- `cargo fmt --all --check` passed;
- `cargo clippy --workspace --all-targets -- -D warnings` passed;
- `cargo test --workspace` passed;
- workspace rustdoc passed with warnings denied;
- `npm run check:docs` passed; and
- `git diff --check` passed.

## 11. Remaining Limitations

- No production source can issue a prepared pair to this package.
- No embedding process invokes the package.
- Cancellation cannot interrupt an admitted executor attempt.
- The in-memory wrapper cannot survive process loss and is not a durable
  lease.
- SQLite remains the only backend behind this private composition path.
- No signal, thread-safety, cross-thread, or process-control claim is made.
- Workspace feature unification means workspace-wide builds include the SPI;
  package-specific graphs remain the relevant isolation proof.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of the implementation. Review
authority opacity, unconstructible preparation, one-shot ownership,
cancellation cloning and post-return behavior, feature isolation, privacy,
compile fixtures, and the continued absence of any production caller.

Do not add a preparation source, embedding application, discovery, scheduling,
provider behavior, OpenShell, nested harnesses, public configuration, hosted
parity, or release change during that review.

## 13. Governed Implementation Record

- workflow: `dg/implement`
- run: `run-1791388257226447000-2`
- approval:
  `approval/run-1791388257226447000-2/implementation-approved`
- presentation: `presentation/5d7782a94ca1b1fa`
- presentation hash:
  `5d7782a94ca1b1fac1332c6988a9dd70b271127f5d454a3bec51c552c68c1464`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: bounded unpublished local-host composition library and
  prepared-pair visibility slice only
- phase status: completed
- event summary: 39 ordered events, one approval, zero retries, and zero
  escalations; approval-presentation proof was enforced with one presentation
  record
- validation summary: focused and workspace validation listed above passed
- out-of-kernel work: source, test, Cargo, roadmap, plan, and report edits;
  validation commands; and later git or pull-request work
- report posture: no runtime WorkReport artifact was generated or persisted
