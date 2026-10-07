# Trusted-Host Local Application Caller Composition Plan Review

## 1. Executive Verdict

**Plan accepted with non-blocking follow-ups; proceed to the bounded
unpublished library and visibility slice only.**

The plan defines a coherent next boundary. Core continues to own preparation,
authority, identity construction, revalidation, admission, continuation, and
workflow truth. The future `workflow-local-host` package receives only one
opaque prepared pair and may own one synchronous foreground call plus scoped
cooperative-cancellation custody.

The review corrected one Cargo feature-isolation claim. A workspace-wide
build will intentionally compile Core with the feature once the local-host
package requests it. Isolation must be proved with package-specific build and
metadata checks, not by claiming the feature is absent from every workspace
build.

## 2. Review Scope

This review assessed:

- the accepted topology and private preparation boundaries;
- proposed prepared-pair visibility;
- package ownership and API shape;
- cancellation cloning and custody;
- authority and state separation;
- failure, privacy, and process-loss posture;
- external compile fixtures and package test strategy;
- Cargo feature behavior; and
- first implementation scope and sequencing.

It did not implement the package, change Rust visibility, add a caller,
introduce discovery or scheduling, add provider behavior, or change release
posture.

## 3. Scope Verification

The plan remains within caller-composition planning. It does not authorize a
binary, command, production preparation source, operational caller, runtime
configuration, schema, provider read or mutation, OpenShell integration,
nested harness, hosted behavior, or public authority factory.

The proposed first implementation is narrow enough to review independently:
one visibility change for an unconstructible pair, one unpublished library,
one concrete wrapper, bounded cancellation control, tests, and documentation.

## 4. Package Topology Assessment

An unpublished library is the correct next composition boundary. The current
CLI uses filesystem-backed local execution and cannot lawfully assemble the
SQLite trusted-host context. Hosted execution uses PostgreSQL claims, leases,
and fencing. Putting this path in either application would blur durable state
truth and create accidental product behavior.

`workflow-local-host` must remain `publish = false`, library-only, and absent
from CLI and hosted dependency graphs. It is not a process owner until an
actual embedding process invokes it.

## 5. Prepared-Pair Visibility Assessment

Exposing only `TrustedHostLocalApplicationPreparedSession` behind the existing
non-default feature is the minimum viable cross-crate change. The pair remains
unconstructible outside Core, one-shot, non-cloneable, non-serde, redacted,
and lifetime-bound. Its consuming parts operation does not provide a way to
prepare or reconstruct authority.

The preparation function and every authority-bearing input remain private.
That separation is essential. A public preparation API would turn a packaging
feature into an authority assembly surface and is not authorized.

## 6. Cancellation Assessment

Cloning cancellation control is acceptable only as duplication of one
operation-scoped cancel capability. Every clone must address the same shared
in-memory cancellation state and must not create a session, execution window,
attempt, or new cancellation domain.

The implementation must prove idempotent repeated requests, pre-entry
zero-write behavior, bounded non-leaking failures, and harmless post-drop or
post-return requests. It must not claim active-attempt interruption.

The plan correctly defers signal installation and thread spawning. Any future
`Send`, `Sync`, or `'static` claim must be proven by the concrete types and
embedding design rather than inferred from internal `Arc` use.

## 7. Execution And Lifecycle Assessment

The one-shot foreground wrapper is appropriately small. It consumes one
prepared pair, retains cancellation state, invokes one session, and returns
the existing bounded outcome or failure. It does not interpret the outcome as
workflow completion or perform policy, approval, persistence, evidence, or
report work.

Dropping an unrun wrapper must remain zero-write. Process loss remains outside
the package's recovery authority. A later process must obtain a newly prepared
current pair from an accepted Core source.

## 8. Authority And State Assessment

The package API correctly excludes identifiers, hashes, backend handles,
skill input, executor implementations, current authority, retry posture, and
provider configuration. It receives no caller-authored facts that could
reconstruct execution authority.

Successful preparation remains weaker than successful execution. Session
consumption must continue to rehydrate and revalidate current state before
authority use. The package must return bounded failure on drift rather than
refreshing or widening context.

## 9. Privacy And Failure Assessment

Prepared pair, wrapper, session, handle, and cancellation control must remain
non-serde and Debug-redacted. Existing bounded application failures are
sufficient. The package has no reason to store raw Core errors, identities,
paths, payloads, command output, provider data, or credentials.

No package-specific error vocabulary should be added unless implementation
finds a genuinely new bounded failure that cannot use the accepted Core
application failure.

## 10. Test Strategy Assessment

The three-layer strategy is sound:

- Core tests retain direct authority-backed preparation and consumption proof;
- package tests exercise concrete ownership and call ordering through private
  test seams; and
- external compile fixtures prove feature visibility and negative
  construction properties.

The external fixture need not obtain a production pair. It can prove that a
function receiving the unconstructible type can pass it into the concrete
package API. Runtime authority proof remains in Core and must not be replaced
with a fake public factory.

Package-private test abstractions are acceptable only if production exports
remain concrete and cannot accept arbitrary forged sessions.

## 11. Cargo Feature Assessment

The original plan wording implied that workspace default tests could avoid
activating the SPI. That is not reliable after a workspace member requests the
feature because Cargo unifies features for the Core package in the workspace
build graph.

The plan now correctly distinguishes:

- package-specific default Core, CLI, and hosted builds, which must not request
  the feature;
- the local-host package build, which intentionally requests it; and
- workspace-wide validation, which intentionally compiles the feature-bearing
  consumer.

This correction prevents a false isolation claim and is sufficient for the
implementation phase.

## 12. Blockers

None for the bounded unpublished library and visibility slice.

Operational adoption remains blocked until a production preparation source
and actual embedding caller are separately planned, implemented, and reviewed.

## 13. Non-Blocking Follow-Ups

- Prefer `#[doc(hidden)]` or equally explicit unstable documentation for the
  exposed prepared type.
- Keep package production APIs concrete; use private test seams only.
- Prove repeated cancellation requests remain idempotent across clones.
- Prove post-return cancellation cannot mutate durable state or imply a new
  lifecycle transition.
- Add package-specific Cargo metadata/build checks for CLI and hosted feature
  absence.
- Preserve the direct fresh substitution non-mutation follow-up when the Core
  fixture is next touched.

## 14. Recommended Next Phase

Implement the bounded `workflow-local-host` library and prepared-pair
visibility slice exactly as planned. Include external compile fixtures and
focused cancellation/ownership/privacy tests.

Do not add a binary, production preparation source, embedding caller,
discovery, scheduling, provider behavior, OpenShell, nested harnesses, public
configuration, hosted parity, or release change.

## 15. Validation

- `npm run check:docs`
- `git diff --check`

Runtime tests are not required because this review changes documentation only.

## 16. Governed Review Record

- workflow: `dg/review`
- run: `run-1791387836962147000-2`
- approval:
  `approval/run-1791387836962147000-2/review-scope-approved`
- presentation: `presentation/03bf6c79be1c0949`
- presentation hash:
  `03bf6c79be1c09495f4e0fdc93f59ca103bb8471a271e245eb0fd89574150923`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused caller-composition plan review only
- phase status: completed
- event summary: 39 ordered events, one approval, no retry or escalation;
  approval-presentation proof was enforced
- validation summary: `npm run check:docs` and `git diff --check` passed;
  runtime tests were not run because this review changed documentation only
- out-of-kernel work: source, test, Cargo, plan, roadmap, and prior review
  inspection; review authoring; validation; and later git or pull-request work
- report posture: no runtime WorkReport artifact was generated or persisted
