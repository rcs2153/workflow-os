# Trusted-Host Local Application Preparation Source Plan

Status: superseded as a standalone implementation plan. The focused
[plan review](../concepts/TRUSTED_HOST_LOCAL_APPLICATION_PREPARATION_SOURCE_PLAN_REVIEW.md)
found that a source with only a test issuer would be an unused production
abstraction. The subsequent [blocker-fix
report](../concepts/TRUSTED_HOST_LOCAL_APPLICATION_PREPARATION_SOURCE_PLAN_BLOCKER_FIX_REPORT.md)
confirms that no current application owns the SQLite run-admission moment
required to issue the source lawfully.

This document now records the rejected standalone boundary and the
prerequisites for reconsidering a source. It does not authorize implementation.

## 1. Executive Summary

Core can privately prepare one opaque trusted-host local application session,
and the unpublished `workflow-local-host` package can consume that prepared
pair. The repository has no lawful production path connecting those
boundaries.

The missing piece is not another wrapper. It is application-owned admission:
one process must own the SQLite backend and foreground lifecycle while Core
selects one exact runnable operation and derives its current authority,
immutable run, actor, executor, and skill-input bindings.

No current application does this:

- `workflow-cli` owns a filesystem-backed executor;
- `workflow-hosted` owns PostgreSQL claims, leases, and fencing;
- `workflow-core` owns SQLite authority but is not a process lifecycle; and
- `workflow-local-host` owns only prepared-operation consumption.

The next phase is trusted-host local application admission planning. A future
preparation source may be introduced only as part of an accepted end-to-end
admission-to-operation path.

## 2. Original Goal

The original plan attempted to add an opaque, Core-issued, one-shot
preparation source that privately owned an already-resolved preparation input
and returned only the existing prepared pair or bounded application failure.

That shape preserved authority opacity, dependency direction, bounded failure,
one-shot ownership, and privacy. It did not identify a production issuer or
embedding caller, so its first implementation would have been unreachable.

## 3. Why Standalone Implementation Is Rejected

A standalone source would fail the repository engineering standard because:

1. its only issuer would be a test seam;
2. no production code could obtain or consume it;
3. package tests would still simulate the decisive Core-to-application path;
4. the runtime-composition gap would remain unchanged; and
5. the roadmap could incorrectly imply operational progress.

Making the source publicly constructible is not an alternative. Accepting
backend handles, locators, identifiers, authority facts, executor internals,
or current-context claims would create a second execution truth outside Core.

## 4. Repository Ownership Findings

### 4.1 Core

Core owns:

- `SqliteStateBackend` and current continuity state;
- immutable run-bundle and operational-window validation;
- fresh and existing preparation posture;
- executor and `SkillInput` commitment checks;
- read-only preparation and consumed-execution revalidation;
- identity generation, redispatch, timer continuation, and authority use.

Core does not own foreground process lifetime, cancellation-handle custody, or
operator invocation timing.

### 4.2 Local host

`workflow-local-host` owns one synchronous prepared operation and scoped
cooperative cancellation. It accepts no identifiers, backend, authority,
executor, or skill input. It does not start or resume runs, choose runnable
steps, evaluate policy or approvals, or create SQLite state.

### 4.3 CLI

The CLI runs the established filesystem-backed executor. Its SQLite surface is
limited to explicit migration and health operations. Adopting the source in an
existing command would bridge runtime state models. Adding a new command would
create a public product surface before the local SQLite application contract
is ready.

### 4.4 Hosted

The hosted worker is PostgreSQL-backed and depends on hosted claim, lease, and
fencing semantics. A SQLite source would bypass those contracts and falsely
imply hosted parity.

## 5. Required Admission Owner

The future foreground owner is the local-host application boundary, not the
current library by itself. Before implementation, a reviewed plan must define
how that application:

1. owns one SQLite backend for its foreground lifetime;
2. starts or resumes one exact governed run through Core;
3. receives one Core-selected runnable operation without scanning or arbitrary
   identifier selection;
4. obtains the exact reviewed executor and validated `SkillInput` from that
   admitted operation;
5. retains cancellation custody while admitted work executes synchronously;
   and
6. returns bounded posture without inventing workflow truth.

Naming `workflow-local-host` as owner is not enough. The plan must identify the
actual application entry and the Core transition that yields the operation.

## 6. Required Core Issuance Point

The future issuer must sit immediately after one authoritative local
run-admission transition has established:

- workflow, run, step, window, and actor identity;
- immutable run-bundle binding;
- fresh opening or existing continuation posture;
- current authority and approval/policy eligibility;
- exact executor commitment;
- validated skill input; and
- required opening persistence identities.

Core may then package those private facts into one opaque, one-shot source or
prepare the pair directly. The application must not reconstruct them from
strings, files, stale snapshots, or copied event data.

No such local run-admission transition currently connects to an application.
That is the prerequisite to plan and review next.

## 7. Dependency Direction

`workflow-local-host` depends on `workflow-core`; Core must not depend on the
local-host package. A future admitted operation therefore originates in Core
and is consumed by local host through a narrow non-default unstable SPI.

Dependency inversion must not be used to hide missing ownership. A trait,
callback, provider, resolver, token, or source is still speculative unless a
real Core transition issues it and a real foreground component consumes it.

## 8. Preserved Source Requirements

If the admission plan later justifies a preparation source, it must be:

- constructed only by Core from already-resolved current state;
- one-shot, non-cloneable, non-serde, and lifetime-bound;
- Debug-redacted;
- weaker than successful preparation and consumed execution;
- zero-write on construction, failed preparation, and drop;
- mapped to fixed payload-free application failure before crossing Core; and
- incapable of carrying reusable authority or caller-authored context.

Session consumption remains the authoritative revalidation and authority-use
edge. State drift must fail closed rather than refresh or broaden the request.

## 9. Explicit Non-Goals

This superseded plan does not authorize:

- a source, issuer, resolver, factory, binary, application, or command;
- Rust implementation or a new crate;
- run discovery, durable-state scanning, queue polling, startup replay, or
  automatic scheduling;
- public backend, locator, authority, executor, or skill-input assembly;
- CLI or hosted adoption;
- provider reads or mutations, OpenShell, nested harnesses, or broader writes;
- signal installation, active-attempt interruption, or process recovery;
- schemas, SDKs, runtime configuration, or release changes; or
- automatic approval or authority broadening.

## 10. Required Next Plan

The trusted-host local application admission plan must answer:

- What starts the local SQLite run and owns its backend lifecycle?
- Which Core transition identifies the exact runnable step?
- How are policy, approval, actor, immutable bundle, executor, and skill input
  bound before preparation?
- What opaque admitted-operation value, if any, crosses Core?
- How are fresh opening and existing continuation paths separated?
- Which foreground component retains cancellation and waits for return?
- How does one production-shaped integration test exercise the same path
  without a public authority factory?
- What operator surface remains absent from the first slice?

The plan must not simply rename the source or add another type-only bridge.

## 11. Future End-To-End Test Requirement

The first accepted implementation must prove one real path:

```text
Core local admission
  -> exact current runnable operation
  -> Core preparation
  -> local-host prepared operation
  -> synchronous consumed execution
  -> bounded returned posture
```

The test must use production constructors and ownership boundaries. It must
prove zero writes on admission/preparation failure and drop, fail-closed state
drift, exactly-once executor admission, no source reuse, bounded errors, and no
public test authority factory.

## 12. Deferred Questions

- Whether the admitted operation should contain a separate source or return a
  prepared pair directly.
- Whether the first foreground application remains unpublished indefinitely.
- What bounded metrics or audit references application ownership requires.
- How process loss is observed before any restart claim is made.
- When, if ever, a public operator surface is justified.

## 13. Validation

This blocker-fix phase requires:

- `npm run check:docs`; and
- `git diff --check`.

No Rust changes are authorized.

## 14. Final Recommendation

Do not implement the standalone preparation source. Plan and review one local
application admission vertical slice first. Introduce a source only if it is
required by that real end-to-end path and can be issued and consumed without
discovery, state translation, or caller-authored authority.
