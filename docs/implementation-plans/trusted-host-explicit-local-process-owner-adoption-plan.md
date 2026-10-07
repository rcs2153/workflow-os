# Trusted-Host Explicit Local Process Owner Adoption Plan

Status: planning completed with an application-topology blocker. The accepted
private process owner is not adopted by an application in this phase.
Repository inspection found no current process that both owns the complete
SQLite trusted-host operational binding and can retain the cooperative
cancellation handle without crossing a deferred public or backend boundary.
The two prerequisite private-owner test-hardening follow-ups are now
implemented; focused review remains next.

## 1. Executive Summary

The accepted `TrustedHostExplicitLocalProcessOwner` is ready for one exact
application caller only after that caller already possesses an immutable
SQLite trusted-host operational input, an injected attempt executor, the exact
skill input, an identity provider, and process-lifetime control.

No current Workflow OS application satisfies that contract:

- `workflow-cli` executes the established filesystem-backed `LocalExecutor`
  path and does not possess the SQLite continuity locator, opening capability,
  dispatch binding, or private owner types;
- `workflow-hosted` owns a PostgreSQL worker lifecycle and must not be made to
  call a SQLite-only private scheduler; and
- `workflow-core` contains the private owner but is a library, not an
  application process that can truthfully own shutdown behavior.

The correct outcome is therefore to block operational adoption rather than
invent a state bridge, expose an unreviewed command, or describe a test caller
as production ownership. The next phase should resolve this topology decision:
choose either an explicit local host application boundary or PostgreSQL
trusted-host parity, then return to this adoption plan.

## 2. Goals

- Identify the exact facts an adopting process must own.
- Evaluate every current application boundary against those facts.
- Preserve the accepted private owner's authority and cancellation semantics.
- Carry the two implementation-review test-hardening requirements forward.
- Define the smallest prerequisite decision needed before adoption can be
  implemented.
- Prevent a convenient but false integration from becoming runtime behavior.

## 3. Non-Goals

This plan does not authorize:

- application adoption or runtime code changes;
- a new CLI command, hidden command, SDK method, schema, or runtime setting;
- exporting crate-private trusted-host types as public API;
- filesystem-to-SQLite runtime translation;
- SQLite scheduling inside the PostgreSQL hosted worker;
- run discovery, startup scanning, queue polling, or automatic invocation;
- automatic approval or delegated authority broadening;
- a daemon, detached task, background thread, or worker pool;
- signal-handler implementation or active-attempt interruption;
- provider execution, OpenShell, nested harnesses, or additional mutations;
- hosted scheduling parity; or
- release posture changes.

## 4. Required Adoption Contract

An application caller must already own all of the following without
reconstructing them from user-authored or stale values:

- the selected `SqliteStateBackend` instance;
- one exact `TrustedHostOperationalEntryLocator`;
- the immutable run bundle bound to that locator;
- either the accepted opening capability or an exact existing-window entry;
- the injected `TrustedHostAttemptExecutor`;
- the exact `SkillInput` bound to the invocation;
- opening persistence identities and the production identity provider;
- the invoking thread or equivalent finite process lifetime;
- the returned cancellation handle; and
- a bounded surface for the owner outcome that does not reinterpret workflow
  status.

The caller must not rediscover or synthesize any of these bindings merely to
make the owner callable.

## 5. Current Application Boundary Assessment

### 5.1 `workflow-cli`

The CLI is the only current local process with a natural foreground lifetime,
but it is not an eligible adoption site today. Its `run`, `approve`, and
inspection commands use the established project loader, `LocalExecutor`, and
filesystem state path. SQLite is exposed only for explicit migration staging,
activation verification, and health inspection. The CLI does not construct a
trusted-host operational opening or hold the complete private binding.

Adopting the owner in `run` or `approve` would silently bridge two runtime
models. Adopting it in `next-action` would violate that command's preview-only,
zero-write contract. Adding a new command would create the public runtime
surface that preceding phases intentionally deferred.

### 5.2 `workflow-hosted`

The hosted worker has explicit process and shutdown ownership, but it uses the
PostgreSQL backend and hosted worker contracts. Calling the SQLite owner from
that process would create backend asymmetry, bypass hosted fencing and lease
semantics, and falsely imply PostgreSQL trusted-host scheduling parity.

### 5.3 `workflow-core`

The private owner belongs in Core because Core controls the accepted SQLite
continuity invariants. Core itself is not an application lifecycle. Adding
another crate-private wrapper or invoking the owner only from tests would not
solve adoption, cancellation custody, shutdown waiting, or operator outcome
delivery.

## 6. Planning Decision

No current application adoption site is approved.

The first viable site should be a deliberately selected **explicit local host
application boundary** that:

1. is local and SQLite-specific;
2. receives one already-selected immutable operational input from an accepted
   source;
3. runs one owner synchronously on a foreground-owned thread;
4. retains the cancellation handle in process control;
5. waits for admitted work to return during cooperative shutdown;
6. emits only bounded owner outcome codes; and
7. remains opt-in, finite, and non-discovering.

That boundary does not exist yet. Its crate placement and visibility must be
decided before code is written. The decision must explain how an application
can call Core without making the current private scheduling internals a broad
public compatibility contract.

## 7. Cancellation And Shutdown Requirements

The future application must create the owner and retain the returned handle
before starting `run`. A shutdown request may call `cancel`, but the process
must observe the accepted distinction:

- cancellation that wins before entry yields `CanceledBeforeEntry` and no
  Core access;
- cancellation during a timer wait wakes that wait and returns the existing
  canceled scheduling posture; and
- cancellation after entry cannot revoke admission or interrupt an active
  executor attempt.

The application must wait for an admitted synchronous call to return. It must
not claim successful shutdown while an attempt remains active, map process
termination into workflow cancellation, or append a fabricated terminal
event. Operating-system signal orchestration requires its own approved design.

## 8. Outcome Surfacing

The future application may surface only bounded classifications derived from:

- `CanceledBeforeEntry`;
- `EntryStopped` with its accepted stop reason;
- `ContinuationStopped` with its accepted scheduling stop posture; or
- a stable `WorkflowOsError` code.

It must not print locators, immutable bindings, skill input, deadlines,
provider payloads, paths, tokens, or raw errors. It must not translate a host
stop into workflow completion, failure, approval, cancellation, retry, or a
claim that work was performed.

## 9. Test-Hardening Prerequisites

Before any application adoption implementation begins, the private owner
suite must add:

1. a barrier-controlled or repeated simultaneous race between `cancel` and
   `begin_entry`, asserting exactly one linearized result and no duplicate
   admission; and
2. complete classification of both competing-owner results, including the
   losing bounded outcome or stable error and direct non-leakage assertions.

The adoption implementation must then add process-boundary tests for:

- cancellation-handle custody;
- pre-entry shutdown;
- shutdown during timer wait;
- shutdown during an active non-interruptible attempt;
- waiting for admitted work to return;
- one exact invocation and no discovery;
- bounded outcome rendering; and
- no public/default activation.

## 10. Rejected Shortcuts

- **Call from `approve`:** approval persistence is not scheduling ownership and
  does not supply the trusted-host binding.
- **Call from `next-action`:** that surface is advisory and zero-write.
- **Add a hidden CLI command:** hidden syntax is still a product and
  compatibility surface.
- **Use filesystem run state to rebuild the locator:** this creates an
  unreviewed state translation and weakens immutable authority binding.
- **Use hosted worker-once:** the backend and fencing semantics do not match.
- **Count an integration test as adoption:** tests prove behavior but do not
  own a process lifecycle.

## 11. Proposed Resolution Sequence

1. Implement and review the two private-owner test-hardening follow-ups.
   Implementation is complete; focused review remains pending.
2. Plan the local host application topology and Core-to-application visibility
   boundary.
3. Review that topology for state, authority, shutdown, privacy, and release
   implications.
4. Implement one finite explicit local host application caller only after the
   review accepts the boundary.
5. Review the implementation before any discovery, automatic invocation,
   hosted parity, or broader runtime exposure.

## 12. Validation Plan

This planning phase requires:

- `npm run check:docs`; and
- `git diff --check`.

No Rust source changes are authorized, so Rust validation is not required for
this phase.

## 13. Open Questions

- Should the explicit local host be a separate unpublished workspace package,
  or should an existing application receive a deliberately narrow internal
  facade?
- How can cross-crate access remain intentionally internal without pretending
  Rust `pub` visibility is a stable product API?
- What accepted source constructs the initial operational opening and exact
  skill binding for the application?
- Which process-shutdown mechanism can request cooperative cancellation and
  still wait for an active attempt without claiming interruption?
- What operator surface owns bounded stop reporting without creating a new
  automatic scheduler?

## 14. Final Recommendation

Do not implement owner adoption against a current application. First harden
the private race tests, then plan the explicit local host application topology.
The owner remains accepted and useful, but making it reachable through the
wrong process would weaken the very authority, backend, and lifecycle
boundaries it was built to preserve.
