# Trusted-Host Local Application Caller Composition Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups; the bounded unpublished library
is ready to remain in the workspace.**

The implementation preserves the reviewed authority boundary. The new
`workflow-local-host` package can consume one Core-issued prepared pair, but
cannot prepare one, construct authority, select work, or claim workflow
lifecycle truth. One-shot ownership, bounded outcomes, scoped cancellation,
feature isolation, and privacy posture are coherent with the plan.

This verdict does not authorize an operational source or caller. The next
phase should plan the one explicit in-process source that can lawfully invoke
private Core preparation without becoming a scheduler or second authority
system.

## 2. Scope Verification

The phase stayed within the accepted bounded slice:

- one unpublished library-only workspace package;
- feature-gated visibility for the unconstructible prepared pair;
- one concrete synchronous foreground wrapper;
- one scoped cooperative-cancellation control;
- focused Core, package, external-consumer, and compile-fail tests; and
- roadmap, plan, and implementation-report updates.

It did not add a binary, production preparation source, embedding caller,
discovery, scheduling, background execution, signal handling, provider
behavior, OpenShell, nested harnesses, CLI, schema, hosted adoption, or release
change.

## 3. Package Boundary Assessment

`workflow-local-host` is `publish = false`, exposes no binary, and adds no new
dependency beyond Core with the reviewed SPI feature. Its production exports
are concrete rather than generic. The private generic seam exists only to
test ownership and call ordering and cannot be used by an external caller to
forge a session.

The package remains composition code, not a runtime or process owner. No
production package depends on it and no production code path invokes it.

## 4. Authority Opacity Assessment

The only new cross-crate Core value is
`TrustedHostLocalApplicationPreparedSession`, hidden from ordinary docs and
available only behind the non-default feature. It has no public constructor.
Preparation, locator selection, backend access, opening context, identity
providers, authority source, executor binding, and skill input remain private.

Public `into_parts` consumes the opaque pair. It does not provide a way to
copy, reconstruct, serialize, refresh, or broaden authority. Core remains the
only issuer and the consumed session remains the authoritative revalidation
and authority-use edge.

## 5. Ownership And Execution Assessment

`LocalHostPreparedOperation` owns exactly one prepared session and consumes
it on `run`. The wrapper is non-cloneable and non-serde. Construction and drop
perform no execution. Package tests prove one private test session invocation,
bounded outcome propagation, and zero invocation when the wrapper is dropped
before `run`.

The package does not reinterpret Core outcomes as workflow completion,
failure, cancellation, retry, approval, evidence, or report state. It appends
no events and writes no state.

## 6. Cancellation Assessment

Cloning cancellation control duplicates access to one shared operation-scoped
in-memory state. Core tests prove the original and clone observe one canceled
entry decision and repeated requests remain idempotent. Package tests prove a
retained control remains bounded after the foreground call returns.

No clone creates a session, window, attempt, authority grant, or cancellation
domain. Cancellation remains cooperative and cannot interrupt an admitted
attempt. No signal handling, thread spawning, or process-control behavior was
added.

## 7. Feature Isolation Assessment

Core's default feature set remains empty and the SPI module and re-exports are
feature-gated. Package-specific dependency graphs prove:

- default Core does not request the SPI;
- CLI does not request the SPI;
- hosted does not request the SPI; and
- only `workflow-local-host` requests it intentionally.

Workspace-wide checks compile the SPI because Cargo unifies features for the
workspace consumer. This is expected and is not an operational adoption path.

## 8. Privacy And Failure Assessment

Prepared pair, session, Core cancellation handle, local operation, and local
cancellation control use fixed redacted Debug implementations. Existing Core
application failures expose only stable payload-free classifications. The
local package adds no raw error, identifier, path, payload, command output,
provider value, credential, event, artifact, or report storage.

Compile-fail documentation proves prepared pairs cannot be cloned or
serialized and local operations cannot be cloned. The external fixture proves
the accepted consumer shape without adding a public pair factory.

## 9. Test Quality Assessment

The three intended proof layers are present:

- Core retains direct authority-backed preparation, revalidation,
  cancellation, substitution, zero-write, and consumed-execution tests.
- Package-private tests prove one-shot call ordering, drop behavior,
  cancellation custody, post-return behavior, and bounded propagation.
- External and compile-fail tests prove the narrow consumer and ownership
  surfaces.

The tests correctly avoid a fake production authority source. Two areas remain
non-blocking follow-ups: package-specific feature absence is checked by
validation commands rather than a checked-in negative fixture, and package
Debug tests cannot instantiate a real operation outside Core. Core directly
tests the redacted bound values, while the package Debug implementations are
fixed and contain no caller data.

## 10. Validation Assessment

The following passed:

- focused local-host package tests;
- focused local-host clippy with warnings denied;
- focused Core SPI tests;
- package-specific feature graph inspection;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- workspace rustdoc with warnings denied;
- `npm run check:docs`; and
- `git diff --check`.

GitHub required checks remain part of pull-request acceptance and must be green
before merge.

## 11. Blockers

None for retaining the bounded unpublished composition library.

Operational adoption remains intentionally blocked pending separate planning,
implementation, and review of a lawful production preparation source and an
actual embedding caller.

## 12. Non-Blocking Follow-Ups

- Consider a checked-in package-graph or compile fixture that proves Core,
  CLI, and hosted default builds cannot name the SPI.
- Add direct post-return no-durable-mutation proof if a future embedding
  caller introduces a durable observation point.
- Prove any future `Send`, `Sync`, cross-thread, signal, or process-control
  claim explicitly before documenting it.
- Preserve the direct fresh substitution non-mutation follow-up when the Core
  preparation fixture is next touched.

## 13. Recommended Next Phase

Plan one explicit in-process source that can lawfully invoke private Core
preparation and hand the resulting pair to `workflow-local-host`. The source
must use current authoritative state and must not become a discovery service,
scheduler, public authority factory, or second lifecycle system.

Do not implement that source during the review. Do not add an embedding
application, binary, CLI, provider behavior, OpenShell, nested harnesses,
hosted parity, or release change.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1791388800770592000-2`
- approval:
  `approval/run-1791388800770592000-2/review-scope-approved`
- presentation: `presentation/377cf3c7877e67b9`
- presentation hash:
  `377cf3c7877e67b9bb42dd579d2282ce0c05ad02224a5472c41493fa645d9d1f`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused implementation review only
- phase status: completed
- event summary: 39 ordered events, one approval, zero retries, and zero
  escalations; approval-presentation proof was enforced with one presentation
  record
- validation summary: focused and workspace validation listed above passed;
  GitHub checks remain required before merge
- out-of-kernel work: source, test, Cargo, documentation, and validation
  inspection; review authoring; and later git or pull-request work
- report posture: no runtime WorkReport artifact was generated or persisted
