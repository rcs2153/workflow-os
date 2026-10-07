# Explicit Local Trusted-Host Production Caller Plan

Status: planning only. The private synchronous local timer and bounded repeated
scheduling driver are implemented and accepted. No production caller is
implemented.

## 1. Executive Summary

Workflow OS can now wait for one durable `TimeWindow`, reassess authoritative
state after every wake, and reinvoke one exact local skill through the accepted
continuity, reservation, and supervisor boundaries. The implementation remains
crate-private and has no explicit production-shaped caller.

The next slice should add one synchronous, process-owned local caller inside
`workflow-core`. The caller receives an already-selected durable operational
locator plus exact executor and invocation inputs, creates process-local
cancellation and fresh bounded operation identities, chooses one fixed finite
wake budget, calls the accepted private timer once, and projects the result
into a bounded operator disposition.

This is not a scheduler, daemon, run-discovery service, public API, or CLI
feature. Core remains authoritative for every wait, transition, dispatch, and
execution decision. This plan does not implement anything.

## 2. Goals

- Define one explicit synchronous caller for an already-authorized local run.
- Keep ownership on the calling process and thread.
- Define an explicit cancellation owner and owner-loss posture.
- Generate fresh operation and receipt identities without granting authority.
- Select a fixed, validated finite wake budget.
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
- `TrustedHostLocalProductionCallerCancellationHandle` if the existing timer
  handle cannot be returned without widening ownership;
- `TrustedHostLocalProductionCallerOutcome`;
- `TrustedHostLocalProductionCallerStopReason`; and
- `run_trusted_host_local_production_caller`.

The input should contain only:

- `&SqliteStateBackend`;
- one `TrustedHostOperationalEntryLocator`;
- one exact `&dyn TrustedHostAttemptExecutor`;
- one exact `SkillInput`;
- one validated finite caller policy selected by Core-owned code or a private
  constant; and
- one injected entropy/nonce source only if deterministic test substitution is
  required.

The caller must not accept caller-authored authority, a claimed current
disposition, a claimed satisfied deadline, approval state, or mutable workflow
definitions.

## 7. Ownership And Cancellation

The first caller is synchronous. The invoking thread remains the sole owner
until the function returns.

The caller should create the timer cancellation pair internally and expose the
handle only through an explicit scoped callback or owner object whose lifetime
cannot outlive the synchronous call accidentally. If a separate cancellation
handle is necessary, tests must prove that:

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

The first production identity source should:

- use a process-local cryptographically strong random source already available
  in the workspace, or a narrowly justified standard/dependency boundary;
- prefix every identifier by domain and operation family;
- remain within existing 128-byte identifier bounds;
- produce independent operation, receipt, attempt, supervisor, yield, and wake
  identifiers;
- never derive identities from secret material, paths, prompts, payloads, or
  timestamps alone;
- surface entropy failure as a stable non-leaking error; and
- permit deterministic injected generation in focused tests.

Duplicate identity remains a Core replay conflict. The caller must not catch
that conflict and retry automatically, because a retry could hide ambiguity
about whether a durable operation committed.

## 9. Wake Budget

The caller must use one private, reviewed finite wake budget. It must not accept
an arbitrary unbounded integer from a user or environment variable.

The first implementation should use the smallest budget that proves lawful
multi-wait continuation while preserving a short synchronous ownership window.
The existing maximum of eight is an upper bound, not a required default. The
implementation review should require a concrete justification for the selected
value.

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

The caller should return one bounded, redaction-safe outcome containing only:

- current authoritative continuation disposition;
- number of scheduled wakes;
- number of executor entries;
- a closed stop reason; and
- whether explicit reconstruction may be considered.

Candidate stop reasons are:

- `Canceled`;
- `Blocked`;
- `Terminal`;
- `UnsupportedWait`;
- `WakeBudgetExhausted`; and
- `Failed` only when represented by the existing structured error rather than
  a misleading workflow disposition.

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
4. deterministic injected identities make tests reproducible;
5. duplicate identities fail closed without a hidden retry;
6. entropy failure is stable and non-leaking;
7. cancellation before and during waiting creates no unauthorized mutation;
8. the fixed wake budget cannot be widened by input;
9. budget exhaustion is non-terminal and explicitly disclosed;
10. blocked, terminal, unsupported, stale, corrupt, and security-rejected
    posture remain distinct;
11. restart uses a reopened backend and fresh process-local state;
12. competing callers preserve aggregate at-most-once executor entry;
13. Debug and error output omit bindings, paths, payloads, and secrets;
14. no public export, CLI command, schema, background thread, or durable host
    job is added; and
15. existing trusted-host and workspace tests continue to pass.

## 15. Implementation Sequence

1. Perform focused maintainer/security review of this plan.
2. Implement the private production identity source and focused tests.
3. Implement the synchronous caller as a thin owner over the accepted timer.
4. Add restart, cancellation, concurrency, budget, and privacy tests.
5. Run full repository validation.
6. Perform focused implementation/security review.
7. Only after acceptance, plan one explicit internal adoption site.

Each item remains a separate governed phase where repository practice requires
it. The caller implementation must not silently become its own adoption site.

## 16. Open Questions

- Which existing workspace entropy source can produce bounded random identities
  without adding an unjustified dependency?
- Should the first caller hard-code a budget of two, four, or another reviewed
  value below the maximum of eight?
- Can the existing timer cancellation handle remain entirely internal while
  still giving the owning process a useful cancellation mechanism?
- Which private internal adoption site, if any, should follow implementation?
- What operator surface should later receive non-terminal budget exhaustion
  without implying a public scheduler?

## 17. Final Recommendation

Proceed next to focused maintainer/security review of this plan. If accepted,
implement one crate-private synchronous local caller and production identity
source only.

Do not add run discovery, a daemon, detached scheduling, public configuration,
CLI, SDK, schemas, automatic approval, provider mutation, OpenShell, nested
harnesses, hosted scheduling, or release changes.
