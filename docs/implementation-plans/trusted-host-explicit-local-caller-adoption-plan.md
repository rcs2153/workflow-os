# Trusted-Host Explicit Local Caller Adoption Plan

Status: implementation complete, pending focused implementation/security
review. The accepted plan is implemented as one additive crate-private
operational-entry composition. It does not create automatic scheduling, a
public owner surface, or a new authority path. See the
[implementation report](../concepts/TRUSTED_HOST_EXPLICIT_LOCAL_CALLER_ADOPTION_REPORT.md).

## 1. Executive Summary

Workflow OS now has an accepted crate-private synchronous production caller
that can wait for bounded `TimeWindow` continuations and reenter the existing
trusted-host execution path. No production path adopts it.

The first adoption should be one explicit local-owner composition beside the
existing trusted-host operational entry. An already-authorized owner calls the
new function with the exact opening or resumed operational-entry input plus an
owner-created cancellation receiver. The function executes the existing
operational entry once. If Core returns `AwaitCondition`, it invokes the
accepted production caller with the same backend, locator, executor, and skill
input. Every other disposition returns immediately.

This is an explicit synchronous opt-in, not run discovery, automatic
scheduling, a daemon, or a public runtime feature.

## 2. Goals

- Adopt the accepted production caller at one real crate-private composition
  boundary.
- Preserve the existing operational-entry function unchanged.
- Require an already-selected exact operational locator and opening context.
- Preserve the same executor binding and `SkillInput` across initial entry and
  continuation.
- Let Core, not a prior host result, decide whether work is waiting.
- Keep cancellation owned by the invoking local process.
- Return a closed result that distinguishes immediate entry from bounded
  scheduled continuation.
- Surface canceled, blocked, terminal, unsupported, and budget-exhausted
  posture without changing workflow state semantics.
- Prove that stale, substituted, competing, and restarted calls fail closed or
  reconcile through existing Core boundaries.

## 3. Non-Goals

This phase must not add:

- implementation in the planning prompt;
- changes to existing `enter_trusted_host_operation` behavior;
- run discovery, startup scanning, automatic run selection, or queue polling;
- a daemon, detached task, background thread, worker pool, or durable host job;
- a public API, CLI command, SDK method, schema, workflow field, or runtime
  configuration;
- automatic approval, delegated self-approval, or broader authority;
- LocalExecutor integration or filesystem-state bridging;
- hosted or PostgreSQL scheduling parity;
- provider reads or mutations;
- OpenShell integration;
- nested harness execution or agent teams;
- additional wait-source families;
- a configurable or larger wake budget; or
- release posture changes.

## 4. Selected Adoption Site

Add one crate-private sibling composition under SQLite trusted-host state,
likely named:

```text
run_explicit_trusted_host_local_operation
```

It should live beside `trusted_host_operational_entry` and
`trusted_host_local_production_caller`, not in the public executor or CLI.

The selected site is the boundary where an explicit local owner already has:

- an authorized operational opening or exact existing-window locator;
- the immutable run and invocation binding;
- the injected executor;
- the exact skill input; and
- process lifetime and cancellation ownership.

Those are the facts needed to enter once and lawfully continue a durable
`TimeWindow` wait. No other current product surface carries this complete
binding without inventing a bridge.

## 5. Rejected Adoption Sites

### LocalExecutor

The current LocalExecutor uses the established workflow runtime and state
interfaces, not the SQLite continuity locator and operational-opening
capability used by the trusted-host path. Wiring the caller there would create
a second implicit state bridge and overclaim automatic continuation.

### CLI

A CLI command would make the private caller public before ownership,
operator-stop reporting, and compatibility contracts are reviewed. It would
also tempt caller-authored locators or reconstructed authority.

### Hosted Worker

The first caller is synchronous and SQLite-specific. Hosted adoption would
require PostgreSQL parity, durable host-job ownership, shutdown semantics,
lease behavior, and operational monitoring.

### Startup Scan Or Background Loop

Scanning durable state or automatically spawning waiters is run discovery and
general scheduling. Both remain explicitly deferred.

## 6. Candidate Private API

Use one additive input and one closed outcome, such as:

- `TrustedHostExplicitLocalOperationInput`;
- `TrustedHostExplicitLocalOperationOutcome`;
- `run_explicit_trusted_host_local_operation`.

The input should contain:

- the exact existing `TrustedHostOperationalEntryInput` needed for the first
  entry;
- one owner-created `TrustedHostLocalTimerCancellation`; and
- no claimed disposition, eligibility, satisfied wait, approval state, wake
  budget, identity set, or reconstruction advice.

Because operational entry consumes locator and skill input, the composition
may destructure the input and retain bounded clones needed for a lawful
follow-up. It must not weaken any identity or commitment check.

The outcome should be an enum that distinguishes:

- `EntryStopped(TrustedHostRedispatchLoopOutcome)` when initial entry is not
  awaiting a condition; and
- `ContinuationStopped(TrustedHostRepeatedSchedulingOutcome)` after the
  production caller owns the bounded `TimeWindow` continuation.

It must not collapse blocked, terminal, canceled, unsupported, or
budget-exhausted posture into success or failure.

## 7. Composition Algorithm

1. Destructure the exact input and preserve the backend, locator, executor,
   skill input, and cancellation receiver.
2. Call existing `enter_trusted_host_operation` exactly once.
3. If its authoritative disposition is not `AwaitCondition`, return
   `EntryStopped` unchanged.
4. If it is `AwaitCondition`, call
   `run_trusted_host_local_production_caller` exactly once with the same bound
   backend, locator, executor, and skill input.
5. Return `ContinuationStopped` unchanged.

The composition must not inspect a caller-authored wait kind or infer that a
deadline is satisfied. The production caller reobserves durable Core state;
unsupported wait families stop with existing bounded posture.

## 8. Authority And Binding Invariants

- The first operational entry remains the only initial executor-admission
  path.
- The production caller remains the only scheduling path after
  `AwaitCondition`.
- The same locator, immutable bundle, actor, executor binding, and skill input
  must flow through both calls.
- Core reloads current durable state before every continuation decision.
- A prior `AwaitCondition` result is routing information only and grants no
  resume authority.
- No durable receipt, event, ticket, or result is converted into an in-memory
  capability.
- Replay conflict, stale binding, corruption, ambiguity, and executor mismatch
  return without retry.

## 9. Ownership, Cancellation, And Shutdown

The local owner creates the existing cancellation pair before invoking the
composition and retains the handle. The call remains synchronous until it
returns.

Cancellation:

- is checked by the accepted local timer;
- wakes a blocked wait;
- does not mutate continuity state;
- does not imply workflow failure, completion, or approval; and
- returns bounded canceled posture through `ContinuationStopped`.

Dropping the handle does not cancel. Process exit loses process-local waiting
only. Detached ownership, owner-loss detection, signal wiring, and durable
shutdown records remain deferred.

## 10. Stop And Error Handling

The owner must receive every closed stop posture unchanged.

- `Terminal`: the authoritative window is terminal.
- `Blocked`: current durable state forbids progress.
- `UnsupportedWait`: another subsystem must handle the wait family.
- `Canceled`: the local owner stopped waiting.
- `WakeBudgetExhausted`: the workflow remains non-terminal and a new explicit
  owner decision is required.

`WorkflowOsError` remains an error. The composition must not convert state
corruption, security rejection, replay conflict, stale binding, timer failure,
or executor mismatch into a normal stop reason. It performs no hidden retry.

## 11. Restart And Concurrency

Restart requires a new explicit call with a reopened backend, the exact
durable locator and immutable input binding, a fresh cancellation pair, and
fresh production identities. Remaining wake budget is not restored.

Competing explicit owners may race. Existing reservation and replay boundaries
must preserve aggregate at-most-once executor entry. The losing owner may
observe terminal, blocked, stale, or replay posture; the composition must not
normalize those outcomes into a retry.

## 12. Privacy And Observability

The new input and outcome must use bounded custom `Debug`. They must not expose
locators, random identities, actor IDs, prompts, skill input, commands,
credentials, paths, provider payloads, evidence bodies, approval reasons, or
authority material.

No new workflow event, audit event, report artifact, metric, or persistent host
record is required for this private composition. Existing continuity and
supervisor events remain authoritative. A later public owner surface must plan
operator notification and stuck-work metrics separately.

## 13. Test Plan

Focused implementation tests should prove:

1. immediate terminal entry returns `EntryStopped` without timer use;
2. one durable `TimeWindow` yield routes to the production caller;
3. two lawful wakes remain bounded by the accepted fixed budget;
4. unsupported waits return `ContinuationStopped::UnsupportedWait` without
   executor reentry;
5. cancellation during waiting is owner-reachable and zero-write;
6. budget exhaustion remains non-terminal and explicit;
7. substituted locator, actor, immutable bundle, skill input, or executor
   binding fails before unauthorized entry;
8. restart with a reopened backend reconstructs current Core posture;
9. competing explicit compositions preserve aggregate at-most-once executor
   entry;
10. replay conflict is not retried;
11. input and outcome Debug are bounded and redacted;
12. existing operational entry and production caller tests remain unchanged;
13. no public export, CLI, schema, background thread, state scan, or durable
   host job appears; and
14. workspace validation passes.

## 14. Implementation Sequence

1. Perform focused maintainer/security review of this plan.
2. Add the additive private composition types and function.
3. Keep existing operational-entry and production-caller functions unchanged.
4. Add focused routing, cancellation, restart, concurrency, failure, and
   privacy tests.
5. Run formatting, clippy, workspace tests, docs checks, and diff hygiene.
6. Create an implementation report.
7. Perform focused implementation/security review before any wider owner or
   operator surface.

## 15. Deferred Questions

- Which later operator surface should receive budget-exhausted posture?
- What durable ownership model is required before detached waiting?
- How should a future host discover eligible work without turning discovery
  into authority?
- What PostgreSQL parity is required before hosted adoption?

## 16. Final Recommendation

Perform focused maintainer/security review of the implemented additive
crate-private explicit operational-entry composition before any wider owner,
operator, discovery, or automatic scheduling surface is considered.

Do not add run discovery, a daemon, background ownership, public
configuration, CLI, SDK, schemas, automatic approval, provider mutation,
OpenShell, nested harnesses, hosted scheduling, or release changes.
