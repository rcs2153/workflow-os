# Trusted-Host Local Application Topology Plan

Status: Core SPI model, bounded-failure correction, and private session
preparation are implemented and reviewed. The fresh-path proof blocker is
fixed. Local caller composition is now documented in the
[Trusted-Host Local Application Caller Composition
Plan](trusted-host-local-application-caller-composition-plan.md); the package,
production caller, and operational adoption remain unimplemented.

## 1. Executive Summary

The private SQLite trusted-host process owner is accepted and its concurrency
test blocker is resolved. The repository still has no application that can
truthfully own it. The CLI runs the filesystem-backed executor, the hosted
worker owns PostgreSQL lifecycle and fencing, and Core is a library rather
than a process.

The planned boundary is an unpublished `workflow-local-host` application
package paired with one narrow, feature-gated, explicitly unstable Core
application SPI. Core must issue an opaque, one-shot application session from
current validated SQLite state and exact invocation authority. The local host
may own foreground lifecycle and cooperative cancellation, but it must not
construct locators, capabilities, immutable bindings, or workflow status.

The first implementation establishes the topology and visibility contracts
only behind the non-default `trusted-host-application-spi` feature. It adds an
opaque one-shot session, cooperative cancellation-handle vocabulary, bounded
outcomes, redacted Debug, and direct privacy/construction tests. Production
code has no session or handle constructor yet. No executable command,
automatic discovery, preparation helper, application package, or operational
adoption exists.

## 2. Goals

- Give one local process a truthful place to own the accepted process owner.
- Preserve Core ownership of state, authority, admission, replay, and terminal
  truth.
- Keep SQLite local scheduling separate from filesystem CLI and PostgreSQL
  hosted execution.
- Define a narrow cross-crate visibility boundary without presenting it as a
  stable public product API.
- Make cancellation-handle custody and foreground shutdown waiting explicit.
- Surface only bounded non-authorizing owner outcomes.
- Sequence implementation so no placeholder becomes operational behavior.

## 3. Non-Goals

This plan does not authorize:

- runtime implementation in this phase;
- a CLI command, SDK method, workflow field, schema, or user configuration;
- run discovery, startup scanning, queue polling, or automatic invocation;
- reconstruction of Core locators from strings, files, or stale snapshots;
- filesystem-to-SQLite or PostgreSQL-to-SQLite state translation;
- PostgreSQL trusted-host parity or hosted worker integration;
- a daemon, detached task, background scheduler, worker pool, or leader
  election;
- operating-system signal integration or active-attempt interruption;
- provider calls, provider mutations, OpenShell, nested harnesses, or broader
  write behavior;
- automatic approval or authority broadening;
- a stable public compatibility promise; or
- release posture changes.

## 4. Repository Findings

### 4.1 CLI

`workflow-cli` owns a natural foreground process, but its ordinary run paths
use `LocalExecutor` with filesystem-backed state. Its SQLite surface is
limited to explicit migration and health operations. It has no trusted-host
operational locator, opening capability, injected attempt executor, or
process-owned cancellation handle.

Adding the owner to `run`, `approve`, or `next-action` would bridge state
models or violate an existing zero-write preview boundary.

### 4.2 Hosted

`workflow-hosted` owns application lifecycle and shutdown, but its worker is
PostgreSQL-backed and governed by hosted claim, lease, and fencing contracts.
Calling the SQLite owner there would create backend asymmetry and bypass those
contracts.

### 4.3 Core

Core contains the accepted private owner and all authoritative SQLite
continuity behavior. It is not an application lifecycle. Tests can construct
the complete private input, but no production application currently does.

## 5. Topology Decision

Add an unpublished workspace package named `workflow-local-host` only after
this plan is reviewed. It should initially be a library with process-lifecycle
composition tests, not a user-invokable binary.

The package must set `publish = false`. It may depend on `workflow-core` only
through a non-default Cargo feature such as `trusted-host-application-spi`.
That feature must not be enabled by `workflow-cli`, `workflow-hosted`, default
workspace consumers, or release artifacts.

The package boundary is intentional:

- Core owns authoritative state and issues a session;
- local host owns the foreground call and cancellation handle;
- an eventual embedding application owns invocation timing; and
- no current CLI or hosted path changes implicitly.

## 6. Core Application SPI

The feature-gated SPI should expose the smallest possible contract:

- `TrustedHostLocalApplicationSession`;
- `TrustedHostLocalApplicationCancellationHandle`;
- `TrustedHostLocalApplicationOutcome`; and
- one Core preparation function or factory that returns the opaque session and
  handle together.

The session must be:

- opaque outside Core;
- one-shot and non-cloneable;
- non-serializable and non-deserializable;
- Debug-redacted;
- bound to one `SqliteStateBackend` instance;
- bound to one immutable run, step, actor, window, invocation, and authority
  context; and
- consumable only by the local-host application runner.

The SPI should be documented as unstable application integration, hidden from
ordinary generated API guidance where practical, and guarded by a non-default
feature. Rust visibility is not a security boundary, so authority must remain
enforced by opaque constructors and Core validation rather than naming or
documentation alone.

## 7. Session Issuance Boundary

Core may issue a session only from current authoritative inputs in the same
process. The preparation boundary must:

1. use one caller-selected `SqliteStateBackend` instance;
2. rehydrate current run and operational-window state;
3. verify immutable run bundle, workflow, step, actor, invocation, cursor,
   authority, and trusted-time bindings;
4. accept only an injected reviewed attempt executor and exact `SkillInput`;
5. create or reconcile the operational opening through the existing atomic
   store boundary when a fresh opening is authorized;
6. return the private owner and cancellation handle only after all bindings
   are coherent; and
7. fail closed without constructing a session when any fact is stale,
   missing, ambiguous, or unsupported.

The application must never receive a constructor for
`TrustedHostOperationalEntryLocator` or any attempt capability. It must not
rebuild a session after process restart from copied identifiers.

No production source currently satisfies all issuance inputs. The first
implementation must therefore stop at the SPI/session preparation boundary
and direct tests unless a separately reviewed source is explicitly approved.

## 8. Local Host Lifecycle

`workflow-local-host` should expose one synchronous internal runner that:

1. accepts the opaque session and associated cancellation handle custody;
2. starts exactly one foreground owner invocation;
3. returns only after admitted work has returned;
4. permits process control to request cooperative cancellation;
5. never claims active-attempt interruption;
6. does not detach, poll, discover, or restart work; and
7. returns one bounded outcome without reinterpreting workflow status.

The runner must not own project loading, workflow selection, policy
evaluation, approval, or provider configuration. Those facts must already be
bound by Core before session issuance.

## 9. Cancellation And Shutdown

The process must retain the handle before starting the owner. The accepted
semantics remain:

- cancellation before entry returns a zero-write canceled-before-entry
  outcome;
- cancellation during a supported timer wait wakes the wait and returns the
  existing bounded canceled scheduling posture;
- cancellation after admission does not revoke capability or interrupt the
  active executor attempt; and
- shutdown waits for admitted synchronous work to return.

Signal handling is a separate phase. The initial application runner should be
driven by an injected cancellation request in tests and embedding code rather
than installing global handlers.

## 10. Outcome And Visibility Boundary

The cross-crate outcome may expose only fixed classifications corresponding
to:

- canceled before entry;
- entry stopped with a bounded stop reason;
- continuation stopped with a bounded scheduling posture; or
- a stable `WorkflowOsError` code.

It must not expose or Debug-format workflow IDs, run IDs, step IDs, window IDs,
actor IDs, immutable-bundle hashes, locators, capabilities, skill input,
deadlines, paths, provider values, credentials, or raw errors.

The local host must not translate an owner stop into workflow completion,
failure, cancellation, approval, retry, or evidence that work occurred.

## 11. Failure And Restart Posture

- Preparation failure creates no application session.
- Pre-entry cancellation performs no Core read or write through the owner.
- An admitted owner remains synchronous and non-detached.
- Process loss during an active attempt remains an unresolved operational
  condition handled by existing durable continuity and later recovery design;
  the local host must not claim recovery.
- A new process may not recreate a session from copied values. A future
  authoritative redispatch source must ask Core for a fresh current session.
- Ambiguous preparation or outcome persistence must fail closed and remain
  diagnosable by stable codes.

## 12. Security And Privacy Requirements

- No caller-authored capability or continuation disposition.
- No ambient authority from package visibility or feature enablement.
- No raw state, payload, command output, provider data, or credentials in
  errors or Debug.
- No serialized session, locator, or capability.
- No session reuse, cloning, or cross-backend substitution.
- No default feature activation.
- No unsafe code or new dependency without separate justification.

## 13. Test Plan

Future implementation tests must prove:

- the SPI is absent without the non-default feature;
- only Core can issue a usable opaque session;
- session construction rejects stale run, cursor, bundle, actor, invocation,
  authority, backend, and trusted-time bindings;
- a session is one-shot and cannot be serialized or cloned;
- the local host retains cancellation-handle custody;
- pre-entry cancellation remains zero-write;
- shutdown during timer wait returns the accepted canceled posture;
- shutdown during an active attempt waits and does not claim interruption;
- competing sessions cannot admit more than one executor entry;
- exactly one explicit invocation occurs and no discovery occurs;
- outcomes and errors remain bounded and non-leaking;
- CLI, hosted, filesystem state, and PostgreSQL behavior remain unchanged; and
- no binary, command, file artifact, or default activation is introduced by
  the first slice.

## 14. Proposed Implementation Sequence

1. **SPI core model and visibility slice.** Add the non-default Core feature,
   opaque session/outcome/cancellation vocabulary, and compile-time/privacy
   tests. Do not add an application package yet.
2. **SPI preparation helper.** Compose existing Core opening and owner
   boundaries from explicit current inputs; direct tests only.
3. **Unpublished local-host library.** Add `workflow-local-host` with one
   synchronous injected-session runner and process-lifecycle tests. No binary
   or command.
4. **Maintainer/security review.** Reassess visibility, authority, cancellation,
   privacy, and backend isolation.
5. **One explicit source plan.** Identify the real in-process component that
   can supply all preparation inputs without discovery or state translation.
6. **Operational adoption implementation.** Add exactly one reviewed caller.
7. **Later planning only.** Signal handling, operator surfaces, discovery,
   restart ownership, hosted parity, and broader scheduling remain separate.

Each implementation step requires its own report and review. A later step may
not be folded into an earlier one merely because the types compile together.

## 15. Alternatives Rejected

- **Embed in `workflow-cli`:** wrong state backend and accidental product
  surface.
- **Embed in `workflow-hosted`:** wrong backend, leases, and fencing model.
- **Keep lifecycle inside Core:** Core cannot own process shutdown.
- **Export all private trusted-host types:** creates a broad authority-bearing
  API and allows caller reconstruction.
- **Use an example or integration test as the host:** proves composition but
  does not establish application ownership.
- **Add a hidden command:** hidden syntax is still an operational contract.
- **Create a filesystem/SQLite bridge:** duplicates state truth and weakens
  immutable binding.

## 16. Validation

This planning phase requires:

- `npm run check:docs`; and
- `git diff --check`.

No Rust changes are authorized.

## 17. Open Questions For Review

- Is a feature-gated unstable SPI sufficiently clear, or should the first
  model remain entirely inside Core until the local-host package is added?
- Should the application package remain library-only permanently, with a
  separate future product surface owning invocation?
- What exact production component can supply the current opening authorization,
  trusted time, executor, and skill input without reconstruction?
- Which owner outcomes merit metrics or audit references without becoming
  workflow lifecycle claims?
- What process-loss observation is required before an operational caller is
  accepted?

## 18. Final Recommendation

Proceed first with the Core SPI model and visibility slice only. Do not create
an executable, connect the CLI, or claim operational adoption. The topology is
sound only if Core issues opaque authority-bound sessions and the local host
owns lifecycle without becoming another source of workflow truth.
