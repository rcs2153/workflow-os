# Explicit Local Trusted-Host Production Caller Plan

Status: planning only. The private synchronous local timer and bounded repeated
scheduling driver are implemented and accepted. Focused review found five
planning blockers; this plan now resolves them with an owner-created private
cancellation pair, a direct `getrandom` 0.4 identity source, all-or-nothing
identity-set construction, a fixed two-wake budget, the existing bounded
repeated-scheduling outcome, and corrected current-authority language. Focused
re-review remains required. No production caller is implemented.

## 1. Executive Summary

Workflow OS can now wait for one durable `TimeWindow`, reassess authoritative
state after every wake, and reinvoke one exact local skill through the accepted
continuity, reservation, and supervisor boundaries. The implementation remains
crate-private and has no explicit production-shaped caller.

The next slice should add one synchronous, process-owned local caller inside
`workflow-core`. The caller receives an already-selected durable operational
locator plus exact executor and invocation inputs, consumes one owner-created
private cancellation receiver, creates fresh bounded operation identities,
uses a fixed two-wake budget, calls the accepted private timer once, and
returns its bounded outcome unchanged.

This is not a scheduler, daemon, run-discovery service, public API, or CLI
feature. Core remains authoritative for every wait, transition, dispatch, and
execution decision. This plan does not implement anything.

## 2. Goals

- Define one explicit synchronous caller for an already-selected local run.
- Keep ownership on the calling process and thread.
- Define an explicit cancellation owner and owner-loss posture.
- Generate fresh operation and receipt identities without granting authority.
- Select a fixed, validated two-wake budget.
- Reenter only through `run_trusted_host_local_timer`.
- Return a bounded operator-facing outcome without leaking invocation data.
- Reconstruct safely after process restart from durable Core state.
- Preserve all accepted continuity, trusted-time, reservation, and executor
  bindings.

## 3. Non-Goals

This phase must not add:

- implementation in the planning prompt;
- run discovery, startup scanning, or automatic run selection;
- a daemon, background thread, queue, worker pool, or detached service;
- durable host jobs, timer persistence, or host-owned workflow state;
- automatic model turns, automatic approval, or delegated self-approval;
- public API, CLI, SDK, schema, workflow field, or runtime configuration;
- provider reads or mutations;
- OpenShell or another sandbox/runtime provider;
- nested harnesses or agent teams;
- hosted, distributed, PostgreSQL, or filesystem scheduling parity;
- additional wait-source families;
- release posture changes.

## 4. Caller Boundary

The future call flow should be:

```text
explicit trusted local owner
  -> crate-private local production caller
  -> private local timer
  -> bounded repeated scheduling driver
  -> schedule-once / source-specific readiness
  -> explicit reinvocation / operational entry
  -> atomic dispatch reservation
  -> one injected executor attempt
```

The caller owns process lifetime, cancellation signaling, exact injected
executor and skill input, and fresh non-authorizing identity generation. It
does not own eligibility, authority, wait satisfaction, workflow state,
approval, or completion.

## 5. Placement And Visibility

Add one private sibling module under SQLite state, likely:

```text
crates/workflow-core/src/sqlite_state/trusted_host_local_production_caller.rs
```

The module and all of its types remain `pub(crate)`. It may compose only the
accepted private operational locator, timer, identity-provider, executor, and
skill-input contracts. It must not be exported from `workflow-core` or adopted
by the CLI or hosted crates in this phase.

## 6. Candidate Private API

Use the smallest private API needed, such as:

- `TrustedHostLocalProductionCallerInput`;
- `TrustedHostLocalProductionIdentitySource`;
- `run_trusted_host_local_production_caller`.

The input should contain only:

- `&SqliteStateBackend`;
- one `TrustedHostOperationalEntryLocator`;
- one exact `&dyn TrustedHostAttemptExecutor`;
- one exact `SkillInput`;
- one `TrustedHostLocalTimerCancellation` created by the private owner-facing
  pair factory; and
- one private identity source, production by default and deterministically
  injectable only in focused tests.

The caller must not accept caller-authored authority, a claimed current
disposition, a claimed satisfied deadline, approval state, or mutable workflow
definitions.

## 7. Ownership And Cancellation

The first caller is synchronous. The invoking thread remains the sole owner
until the function returns.

The explicit local owner creates the existing private cancellation pair before
calling the synchronous function. The owner retains
`TrustedHostLocalTimerCancellationHandle`; the caller input consumes
`TrustedHostLocalTimerCancellation`. The production caller creates no thread,
callback, trait object, or second cancellation abstraction.

Tests must prove that:

- cancellation is idempotent;
- cancellation wakes a current wait promptly;
- cancellation does not mutate Core state;
- dropping the cancellation handle does not imply approval, completion, or
  cancellation; and
- caller loss is not claimed to be detected.

Detached owner-loss semantics remain deferred. A panic or process exit loses
only process-local waiting state; durable Workflow OS state remains the source
for later explicit reconstruction.

## 8. Fresh Identity Generation

The production caller must satisfy the existing
`TrustedHostRedispatchIdentityProvider` and
`TrustedHostScheduleWakeIdentityProvider` contracts without turning identity
generation into authority.

The first production identity source must use a direct `getrandom` 0.4
dependency. Production construction is private. Focused tests may inject a
deterministic byte-filling function through a test-only constructor.

For every redispatch iteration, the source performs one all-or-nothing fill of
96 bytes and partitions it into six independent 128-bit values for consume
operation, consume receipt, generated attempt, supervisor operation,
supervisor receipt, and yield generation. For every scheduled wake, it performs
one all-or-nothing fill of 32 bytes for the wake operation and receipt. It
validates the complete typed identity set before returning any part of it.

The source must:

- prefix every identifier with a short fixed operation-family domain;
- remain within existing 128-byte identifier bounds;
- produce independent operation, receipt, attempt, supervisor, yield, and wake
  identifiers;
- never derive identities from secret material, paths, prompts, payloads, or
  timestamps alone;
- encode random bytes as fixed-width lowercase hexadecimal;
- surface entropy failure as one stable non-leaking error;
- expose no partially constructed identity set after entropy or validation
  failure; and
- permit deterministic injected generation in focused tests.

Duplicate identity remains a Core replay conflict. The caller must not catch
that conflict and retry automatically, because a retry could hide ambiguity
about whether a durable operation committed.

## 9. Wake Budget

The caller uses a private constant of exactly **two wakes**, constructed through
`TrustedHostRepeatedWakeBudget::new(2)`. It accepts no wake budget from a user,
environment variable, config file, or caller input. Two is the smallest value
that proves repeated lawful waiting rather than only schedule-once behavior.
Any later increase requires a separate reviewed change.

Budget exhaustion is a non-terminal host stop. It does not fail or complete the
workflow and must be surfaced as requiring a fresh explicit caller decision.

## 10. Restart Entry

Restart does not restore the old timer, cancellation state, identity provider,
or remaining budget. A new explicit caller invocation must receive:

- a reopened `SqliteStateBackend`;
- the same durable operational locator;
- the exact immutable executor and skill-input binding;
- a fresh cancellation owner;
- a fresh finite budget; and
- fresh identity generation state.

The accepted timer and operational-entry boundaries then rehydrate current
Core state. Wait, blocked, terminal, stale, corrupt, ambiguous, or
security-rejected posture must remain exactly classified. No durable receipt or
event may be converted back into in-memory authority.

## 11. Operator Outcome

The caller returns the existing `TrustedHostRepeatedSchedulingOutcome`
unchanged. It contains only:

- current authoritative continuation disposition;
- number of scheduled wakes;
- number of executor entries;
- a closed stop reason.

Accepted stop reasons remain:

- `Canceled`;
- `Blocked`;
- `Terminal`;
- `UnsupportedWait`;
- `WakeBudgetExhausted`.

Failures remain `WorkflowOsError`; they are not converted into a stop reason.
The caller returns no reconstruction, retry, or resume-advice boolean. A later
owner decision must reenter Core and cannot be inferred from caller output.

The outcome must not contain prompts, source content, commands, credentials,
approval reasons, evidence bodies, raw skill output, provider payloads, file
paths, or reusable authority. It is operator posture, not a WorkReport or audit
record.

## 12. Failure Semantics

- Invalid input fails before timer entry and before state mutation.
- Identity-generation failure fails closed with fixed non-leaking errors.
- Cancellation returns the accepted typed canceled stop.
- Unsupported waits, blocks, terminal state, and budget exhaustion preserve
  their accepted typed posture.
- State corruption, stale bindings, trusted-time rejection, operation replay
  conflict, ambiguity, or executor-binding mismatch return immediately.
- The caller performs no automatic retry after any structured error.
- Host failure never becomes workflow failure or completion.
- A final model response or caller return never makes a non-terminal run
  terminal.

## 13. Observability And Disclosure

The first caller should not add host-job persistence or new workflow events.
Existing continuity events remain the authoritative durable record of admitted
operations and outcomes.

The private caller may expose bounded counts and stop posture to its direct
owner. A later public or hosted adoption must separately define metrics,
operator notification, stuck-run detection, cancellation provenance, and
machine-readable reporting before claiming operational scheduling.

## 14. Test Plan

Future focused tests should prove:

1. an already-eligible exact run enters the executor at most once;
2. one and multiple `TimeWindow` waits use the accepted timer path;
3. fresh identities are distinct, bounded, and domain-separated;
4. deterministic test-only byte filling makes tests reproducible;
5. each identity family receives independent random material;
6. duplicate identities fail closed without a hidden retry;
7. entropy or typed-validation failure exposes no partial identity set;
8. cancellation is reachable through the owner-retained handle while the
   synchronous caller is blocked;
9. cancellation before and during waiting creates no unauthorized mutation;
10. the fixed two-wake budget cannot be widened by input;
11. budget exhaustion is non-terminal and explicitly disclosed;
12. blocked, terminal, unsupported, stale, corrupt, and security-rejected
    posture remain distinct;
13. restart uses a reopened backend and fresh process-local state;
14. competing callers preserve aggregate at-most-once executor entry;
15. Debug and error output omit bindings, paths, payloads, and secrets;
16. no reconstruction or resume-advice boolean exists in the outcome;
17. no public export, CLI command, schema, background thread, or durable host
    job is added; and
18. existing trusted-host and workspace tests continue to pass.

## 15. Implementation Sequence

1. Perform focused maintainer/security review of this plan.
2. Add the direct `getrandom` 0.4 dependency and implement the private
   all-or-nothing production identity source with focused tests.
3. Implement the synchronous caller as a thin owner over the accepted timer.
4. Add restart, cancellation, concurrency, budget, and privacy tests.
5. Run full repository validation.
6. Perform focused implementation/security review.
7. Only after acceptance, plan one explicit internal adoption site.

Each item remains a separate governed phase where repository practice requires
it. The caller implementation must not silently become its own adoption site.

## 16. Deferred Questions

- Which private internal adoption site, if any, should follow implementation?
- What operator surface should later receive non-terminal budget exhaustion
  without implying a public scheduler?
- What owner-loss contract is needed before any detached caller exists?

## 17. Final Recommendation

Proceed next to focused maintainer/security review of this plan. If accepted,
implement one crate-private synchronous local caller and production identity
source only.

Do not add run discovery, a daemon, detached scheduling, public configuration,
CLI, SDK, schemas, automatic approval, provider mutation, OpenShell, nested
harnesses, hosted scheduling, or release changes.
