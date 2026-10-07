# Trusted-Host Local Application Caller Composition Plan

Status: accepted with non-blocking follow-ups by the focused
[maintainer/security review](../concepts/TRUSTED_HOST_LOCAL_APPLICATION_CALLER_COMPOSITION_PLAN_REVIEW.md).
The bounded unpublished library and prepared-pair visibility slice are now
implemented and documented in the
[implementation report](../concepts/TRUSTED_HOST_LOCAL_APPLICATION_CALLER_COMPOSITION_REPORT.md).
No production preparation source, embedding caller, discovery, scheduling, or
operational adoption is implemented.

## 1. Executive Summary

The private Core preparation boundary is implemented, its pre-issuance
binding blockers are fixed, and direct fresh-path proof is accepted. Core can
read current SQLite state, validate one exact fresh or existing operational
posture, and issue one opaque session-and-cancellation pair without consuming
authority or writing state. Consuming the session remains the authoritative
revalidation and authority-use edge.

The next implementation should add the first unpublished local-host
composition library around that already prepared pair. The library should own
one synchronous foreground invocation and scoped cooperative-cancellation
custody. It must not prepare the pair, discover work, schedule runs, construct
authority, interpret workflow state, or become a new source of lifecycle
truth.

This plan does not implement the library. It defines the minimum cross-crate
visibility change, package boundary, ownership model, test strategy, and
security conditions for a later implementation phase.

## 2. Goals

- Add a truthful cross-crate composition boundary for one Core-prepared local
  operation.
- Keep Core authoritative for preparation, identity, state rehydration,
  authority consumption, admission, continuation, and workflow truth.
- Keep the future package unpublished, library-only, local, synchronous, and
  SQLite-specific.
- Preserve the pair's one-shot, non-serializable, non-reconstructable nature.
- Give an embedding process scoped cooperative-cancellation control without
  exposing locators, capabilities, bindings, or raw errors.
- Prove default-feature isolation and the external consumer shape before any
  production caller exists.
- Make operational adoption a separate reviewed phase.

## 3. Non-Goals

This plan does not authorize:

- implementation during this planning phase;
- a binary, CLI command, SDK method, schema field, workflow field, or runtime
  configuration;
- a production preparation source or public preparation function;
- run discovery, project scanning, queue polling, scheduling, redispatch
  selection, or automatic invocation;
- background tasks, detached execution, worker pools, signal installation,
  process recovery, or active-attempt interruption;
- filesystem or PostgreSQL state translation;
- provider reads or mutations, broader write behavior, OpenShell, or nested
  harness execution;
- automatic approval, authority inference, or capability broadening;
- hosted parity, enterprise administration, or release posture changes; or
- a stable public compatibility promise.

## 4. Accepted Starting Boundary

Current Core behavior already provides, behind the non-default
`trusted-host-application-spi` feature:

- `TrustedHostLocalApplicationSession`;
- `TrustedHostLocalApplicationCancellationHandle`;
- bounded outcome and failure vocabulary;
- a crate-private `TrustedHostLocalApplicationPreparedSession`;
- crate-private fresh and existing preparation inputs; and
- crate-private read-only preparation followed by consumed-run authoritative
  revalidation.

The accepted fresh proof covers drop, pre-run cancellation, backend and
binding substitution, opening-shape rejection, trusted-time rejection, and
one consumed execution. No application or production caller can currently
obtain the prepared pair.

## 5. Package Decision

Add an unpublished workspace library package named `workflow-local-host` in
the later implementation phase.

The package must:

- set `publish = false`;
- contain no binary target;
- depend on `workflow-core` with
  `features = ["trusted-host-application-spi"]` and no default activation in
  other packages;
- remain absent from `workflow-cli` and `workflow-hosted` dependencies;
- add no dependency unless the implementation review justifies it; and
- describe itself as an unstable local composition library, not an
  operational runtime or process owner.

The library becomes process-relevant only when a separately reviewed
embedding application invokes it. Merely joining the workspace does not make
trusted-host execution automatic.

Workspace-wide validation will intentionally compile Core with the feature
because the new package requests it. That Cargo feature unification is not
evidence that `workflow-cli` or `workflow-hosted` enables the feature in its
own package build or release graph. Feature-isolation tests must therefore
use package-specific build and metadata checks rather than claiming the
feature is absent from `cargo test --workspace`.

## 6. Core Visibility Decision

Expose `TrustedHostLocalApplicationPreparedSession` through the existing
non-default feature so the unpublished package can accept it by value.
Expose only its consuming `into_parts` operation.

The prepared type must remain:

- unconstructible outside Core;
- non-cloneable;
- non-serializable and non-deserializable;
- Debug-redacted;
- lifetime-bound to its borrowed Core dependencies;
- consumable exactly once; and
- unavailable in default-feature builds.

The preparation function, preparation input, fresh-opening context,
operational locator, authority source, identity providers, and execution
bindings remain crate-private. Feature enablement permits type-level
composition only; it grants no authority and provides no way to issue a pair.

The exposed prepared type should be documented as unstable application SPI
and hidden from ordinary product guidance where practical. Rust visibility is
not treated as a security boundary; unforgeable construction and Core
validation remain the security boundary.

## 7. Local Composition Model

The package should add the smallest concrete wrapper, conceptually:

```text
LocalHostPreparedOperation::from_prepared(prepared_pair)
  -> LocalHostPreparedOperation

LocalHostPreparedOperation::cancellation_control(&self)
  -> LocalHostCancellationControl

LocalHostPreparedOperation::run(self)
  -> Result<TrustedHostLocalApplicationOutcome,
            TrustedHostLocalApplicationFailure>
```

Exact Rust names may follow repository conventions. The public package API
must remain concrete. It must not expose generic executor, backend, locator,
authority, or identity-provider parameters.

Construction consumes the Core-issued pair. `run` consumes the wrapper and
invokes the session exactly once on the caller's foreground thread. The
wrapper retains cancellation state for the complete call and returns only the
existing bounded Core application outcome or failure.

## 8. Cancellation Custody

The existing private timer handle is internally cloneable. The later
implementation may make the application cancellation handle cloneable only
to create one or more references to the same operation-scoped cancellation
state. A clone must not create new execution authority, a new session, a new
window, or a new cancellation domain.

The package may expose a redacted `LocalHostCancellationControl` backed by
that scoped handle. The control may request cooperative cancellation and
return only the existing bounded application failure.

The first implementation must not install operating-system signal handlers or
spawn a thread. An embedding process may retain cancellation control while it
runs the operation in an execution context it owns, but thread-safety and
cross-thread movement must not be claimed unless the concrete Core types and
tests prove it. Pre-entry cancellation is required. During supported timer
waits remains a Core capability; actual process-triggered use is deferred
until an embedding caller exists.

Cancellation after executor admission remains non-revoking and does not
interrupt an active attempt. The synchronous caller waits for admitted work
to return.

## 9. Execution And Outcome Semantics

The local-host library must:

1. accept exactly one Core-issued prepared pair;
2. retain the paired cancellation state;
3. consume exactly one session;
4. call no preparation, discovery, policy, approval, or persistence API;
5. return only the accepted bounded application outcome or failure; and
6. make no independent workflow lifecycle claim.

`CanceledBeforeEntry`, entry-stop classifications, and continuation-stop
classifications remain operational observations. The package must not
translate them into workflow completion, failure, cancellation, retry,
approval, evidence, or report status.

## 10. Authority And State Boundary

The package receives no authority-bearing construction input beyond the
opaque prepared pair. It must not accept:

- workflow, run, step, window, actor, or approval identifiers;
- immutable-bundle or invocation hashes;
- SQLite paths or backend handles;
- skill input or executor implementations;
- current-authority or required-context records;
- retry, wake, redispatch, or continuation decisions; or
- provider credentials or configuration.

Core preparation remains read-only. Core session consumption remains the
time-of-use revalidation and authority-consuming edge. A prepared pair may
fail when consumed if state or authority changed. The library must return the
bounded failure rather than refresh, reconstruct, retry, or broaden context.

## 11. Failure, Drop, And Process-Loss Posture

- Failure before pair issuance remains wholly inside Core.
- Dropping an unconsumed local wrapper must perform no write and make no
  workflow claim.
- Cancellation before `run` must preserve the accepted zero-entry,
  zero-write result.
- A bounded run failure must not expose the private Core error.
- The wrapper is not durable and cannot be reconstructed after process loss.
- Process loss during an admitted attempt remains an unresolved operational
  condition; the package must not claim recovery.
- A later process must obtain a newly prepared current pair from an accepted
  authoritative source rather than reuse copied identifiers.

## 12. Privacy And Debug Safety

- Prepared pair, wrapper, session, handle, and cancellation control must use
  redacted Debug.
- No type may serialize workflow identities, bindings, state, authority,
  payloads, paths, command output, provider values, or credentials.
- Display and error output remain fixed and payload-free.
- The package stores no logs, artifacts, events, or reports.
- No secret-like test value may appear in Debug, Display, or failure output.

## 13. External Consumer Proof

Before the package is treated as a valid boundary, compile-time fixtures must
prove:

- ordinary `workflow-core` consumers cannot name the SPI without enabling the
  feature;
- an explicit-feature consumer can name and pass the prepared type to the
  local-host package;
- external code cannot construct, clone, serialize, or deserialize the
  prepared pair or session;
- external code cannot invoke preparation or construct authority inputs; and
- neither `workflow-cli` nor `workflow-hosted` enables the feature.

The fixture may type-check a function that accepts the prepared pair without
requiring a production pair source. It must not add a fake public authority
factory merely to make an end-to-end test convenient.

## 14. Test Strategy

The first implementation should combine three proof layers:

1. **Core behavior tests.** Preserve the accepted direct preparation, drop,
   cancellation, substitution, stale-state, one-shot, and non-leakage tests.
2. **Package behavior tests.** Test foreground call ordering, exactly-once
   consumption, cancellation-control custody, drop behavior, bounded result
   propagation, and Debug safety through private test seams that do not make
   production APIs generic or forgeable.
3. **External compile fixtures.** Prove feature absence/presence and negative
   construction, clone, and serialization properties.

The implementation must also prove:

- package-specific default builds for Core, CLI, and hosted do not request the
  SPI, while workspace-wide validation intentionally covers the local-host
  feature consumer;
- the package creates no file, event, backend write, thread, process, or
  command on construction;
- cancellation clones, if added, address one shared scoped cancellation state;
- one wrapper cannot execute twice;
- no CLI or hosted behavior changes; and
- no production code path calls the package.

## 15. First Implementation Scope

The first implementation phase should include only:

1. feature-gated external visibility for the unconstructible prepared pair
   and its consuming parts operation;
2. narrowly scoped clone semantics for cancellation control if required by
   the concrete package design;
3. the unpublished `workflow-local-host` library;
4. one concrete prepared-operation wrapper and bounded cancellation control;
5. private package tests and external compile fixtures;
6. honest roadmap and architecture documentation; and
7. an implementation report.

It must stop before adding a production preparation source or embedding
caller.

## 16. Subsequent Phase Sequence

1. Implement the bounded library and visibility slice.
2. Perform a focused maintainer/security review of authority opacity,
   cancellation cloning, one-shot ownership, feature isolation, privacy, and
   test quality.
3. Plan one explicit in-process source that can lawfully invoke private Core
   preparation without reconstructing authority.
4. Implement and review that source while keeping invocation explicit.
5. Plan one actual embedding application and process-control boundary.
6. Only after those reviews, consider signal integration or operational
   adoption.

Discovery, scheduling, hosted parity, provider mutation broadening,
OpenShell, nested harnesses, and public configuration remain separate roadmap
decisions.

## 17. Alternatives Rejected

- **Make preparation public:** exposes authority assembly and lets callers
  attempt to reconstruct current context.
- **Export private locators or capabilities:** creates ambient authority and a
  second source of execution truth.
- **Put the caller in `workflow-cli`:** bridges the wrong filesystem-backed
  execution model into SQLite continuity.
- **Put the caller in `workflow-hosted`:** bypasses PostgreSQL claims, leases,
  and fencing.
- **Add a hidden command:** hidden syntax is still an operational product
  surface.
- **Use a public test factory:** permits forged application sessions and
  confuses mock proof with production authority.
- **Spawn a thread in the library:** silently expands lifecycle and shutdown
  semantics before a real process owner exists.
- **Delay all composition until a binary exists:** leaves the cross-crate
  authority and ownership boundary unproved until the riskiest phase.

## 18. Validation For The Planning Phase

- `npm run check:docs`
- `git diff --check`

No Rust code changes or runtime tests are authorized for this planning phase.

## 19. Open Questions For Implementation Review

- Can cancellation cloning remain a direct property of the bounded Core
  handle, or should the package expose an even smaller wrapper?
- Which private test seam best proves package call ordering without adding a
  forgeable production abstraction?
- Should the prepared pair and `into_parts` be `#[doc(hidden)]`, or is the
  explicit unstable module documentation sufficient?
- Can the package remain lifetime-borrowing without accidental `'static` or
  `Send` claims?
- What exact current-authority component can later prepare the pair without
  becoming a scheduler or discovery service?

## 20. Final Recommendation

Proceed next with the bounded unpublished `workflow-local-host` library and
prepared-pair visibility slice only. Keep preparation and authority assembly
inside Core, keep invocation absent from production, and prove the cross-crate
boundary through package tests and external compile fixtures.

Do not add a binary, command, production caller, discovery, scheduling,
provider behavior, OpenShell, nested harnesses, public configuration, hosted
parity, or release change.

## 21. Governed Planning Record

- workflow: `dg/d`
- run: `run-1791387330002191000-2`
- approval:
  `approval/run-1791387330002191000-2/planning-approved`
- presentation: `presentation/e7fffa24dd89babc`
- presentation hash:
  `e7fffa24dd89babc26bd10704ca334d2d64a6bf719d2be5b1bd5c5a7e06356c1`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: caller-composition planning only
- phase status: completed
- event summary: 39 ordered events, one approval, no retry or escalation;
  approval-presentation proof was enforced
- validation summary: `npm run check:docs` and `git diff --check` passed;
  runtime checks were not run because this phase changed documentation only
- out-of-kernel work: repository inspection, plan authoring, documentation
  validation, and later git or pull-request work
- report posture: no runtime WorkReport artifact was generated or persisted
