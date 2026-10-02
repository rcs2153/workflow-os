# Bounded Trusted-Host Redispatch Loop Plan

Status: planning only. Atomic dispatch reservation is implemented and accepted.
This plan defines the next private local runtime boundary. It does not implement
the loop, a scheduler, provider execution, OpenShell, nested harnesses, public
configuration, CLI, SDK, schema, hosted behavior, or release changes.

## 1. Executive Summary

Workflow OS can open one authorized execution window, start one exact attempt,
invoke one injected local executor through a trusted-host supervisor, record a
bounded result or executor yield, derive an authoritative continuation
disposition, consume one fresh resume directive, and admit exactly one caller
through an atomic dispatch reservation.

Those accepted primitives are not yet composed into a bounded loop. A caller
must currently inspect `ResumeNow`, consume the next directive, and invoke the
one-shot supervisor again. This leaves lawful redispatch as host-written glue
and makes it too easy for an executor turn boundary to become a false stall.

The next implementation should add one crate-private, local, injected
redispatch helper. It may continue only while Core freshly derives
`ResumeNow`. Every iteration must consume a fresh directive, reserve the exact
new attempt atomically, and invoke the existing one-shot supervisor once.
`AwaitCondition`, `Blocked`, and `Terminal` must stop the loop immediately and
return the exact Core-derived posture.

The loop must be finite and explicit. A configured iteration budget protects
the host process from runaway local execution, but budget exhaustion must not
be represented as workflow completion, approval wait, or a fabricated typed
wait. It returns an explicit `ResumeRequired` host posture while durable Core
state remains authoritative.

## 2. Current Accepted Foundation

The implementation may compose, but must not weaken, these accepted contracts:

- immutable run-input and exact workflow/run/step binding;
- actionable gate and current-authority resolution;
- durable execution windows and typed wait conditions;
- executor yield and authoritative continuation dispositions;
- atomic projected continuity operations at an expected event cursor;
- one-winner operational window opening and first attempt start;
- one-shot injected trusted-host supervision;
- one-winner atomic dispatch reservation before executor entry;
- reservation-bound outcome, yield, and ambiguous-recovery persistence;
- append-only runtime events and deterministic run-snapshot projection; and
- fresh-connection reconciliation after ambiguous commits.

The current `AuthoritativeContinuationDisposition` vocabulary is sufficient:

- `ResumeNow`;
- `AwaitCondition`;
- `Blocked`; and
- `Terminal`.

This phase should not add a fifth workflow disposition merely to describe a
host iteration budget.

## 3. Problem Statement

The accepted one-shot boundary deliberately requires an explicit caller. That
was necessary to prove authority, reservation, result, yield, and recovery
semantics independently. It also means the host currently owns an unsafe
composition gap:

1. inspect the current disposition;
2. decide whether to continue;
3. construct the next consume operation;
4. consume a resume directive;
5. construct the next supervisor call; and
6. repeat or stop.

If this glue trusts stale disposition, reuses authority, guesses an attempt,
fabricates identifiers, treats a host turn ending as completion, or retries an
ambiguous call, it can violate the kernel invariants even though each primitive
is correct in isolation.

The loop must move that composition into one narrow Core-owned call while
leaving execution injected and local.

## 4. Goals

- Compose the accepted opening, directive, reservation, and one-shot
  supervisor boundaries into one finite local loop.
- Treat Core as the sole source of truth for continue, wait, block, and stop.
- Require a fresh directive and one-winner reservation for every redispatch.
- Invoke at most one injected executor for each admitted reservation.
- Stop immediately on typed wait, blocked, terminal, ambiguous, corrupt, or
  security-rejected posture.
- Return a bounded explicit result that distinguishes workflow disposition
  from host loop exhaustion.
- Preserve current workflow pass/fail and terminal semantics.
- Remain restart-safe and safe under two competing loop callers.
- Keep all invocation values, outputs, and failure details outside events,
  errors, Debug output, and durable reservation records.
- Define focused tests that prove behavior rather than construction.

## 5. Non-Goals

- No implementation in this planning phase.
- No daemon, queue, background worker, polling service, or hosted scheduler.
- No creation or resumption of model conversations or agent turns.
- No automatic approval, delegated-authority broadening, evidence synthesis,
  check synthesis, or policy bypass.
- No provider execution or mutation.
- No OpenShell integration or sandbox lifecycle.
- No nested harness execution, recursive agents, or agent swarms.
- No public runtime configuration, workflow spec fields, CLI, SDK, or example.
- No filesystem or PostgreSQL parity in the first implementation.
- No multiple-host lease, reservation stealing, heartbeat, or failover.
- No automatic retry after ambiguous executor entry or commit acknowledgement.
- No new SideEffect, approval, report-artifact, or reasoning-lineage behavior.
- No raw prompt, transcript, source, command output, provider payload,
  environment value, credential, token, or authorization material.
- No hosted, distributed, production, or release-readiness claim.

## 6. Source-Of-Truth Boundaries

| Concern | Source of truth | The loop must not substitute |
| --- | --- | --- |
| Workflow lifecycle | Durable events and rehydrated run snapshot | Host return value or assistant response |
| Continue/wait/stop | Fresh Core continuation disposition | Cached host decision |
| Execution authority | Fresh consumed directive or opening capability | Durable receipt or serialized projection |
| Executor admission | Atomic dispatch reservation | Read-only dispatchability check |
| Invocation binding | Accepted immutable operation and executor commitments | Host-selected replacement input |
| Wait posture | Durable typed wait records | Generic pause text or timer guess |
| Completion | Valid durable terminal transition | Successful executor callback |
| Host budget | Explicit loop input and result | Workflow status or wait condition |

## 7. Required Invariants

1. Only `ResumeNow` may begin another redispatch iteration.
2. The disposition must be re-derived from durable state for every iteration.
3. Every resumed iteration consumes one fresh one-time directive at the
   current event cursor.
4. Every executor entry commits one exact dispatch reservation first.
5. An opening capability, directive capability, attempt capability, or
   reservation capability is consumed by value and never cloned by the loop.
6. Durable receipts and events are evidence, not execution credentials.
7. Exact replay never invokes the executor and never reissues authority.
8. Ambiguous reservation commit never invokes the executor.
9. Ambiguous executor result never causes automatic redispatch until the
   accepted recovery operation establishes a lawful fresh disposition.
10. `AwaitCondition` returns without polling, sleeping, or guessing that a
    dependency became satisfied.
11. `Blocked` returns without retrying or translating the block into an
    approval request.
12. `Terminal` returns without another directive read or executor call.
13. Host iteration-budget exhaustion is not a workflow event, wait, failure,
    approval, or completion.
14. Two concurrent loops cannot execute the same attempt twice.
15. A successful local skill invocation does not by itself complete a step or
    workflow run.
16. Any binding, cursor, revision, projection, snapshot, schema, trusted-time,
    or reconciliation mismatch fails closed before another executor call.

## 8. Candidate Private API

The first implementation should add the smallest crate-private surface,
provisionally:

- `TrustedHostRedispatchLoopInput`
- `TrustedHostRedispatchLoopBudget`
- `TrustedHostRedispatchIterationInput`
- `TrustedHostRedispatchInputProvider`
- `TrustedHostRedispatchLoopOutcome`
- `TrustedHostRedispatchStopReason`
- `run_bounded_trusted_host_redispatch_loop(...)`

Names may follow adjacent conventions, but the boundary must remain private.

The loop input should contain:

- one `SqliteStateBackend` reference;
- one initial opened or resumed supervisor capability;
- one injected `TrustedHostAttemptExecutor`;
- one injected, deterministic iteration-input provider;
- one positive maximum executor-entry count; and
- the exact initial operation/receipt/input material required by the existing
  one-shot supervisor.

The input provider may supply bounded operation IDs, receipt IDs, yield
generation IDs, and an invocation input already constrained by the accepted
window binding. It is not an authority provider. Core must recompute and
validate every commitment and reject substitution before admission.

The provider must not receive raw durable state, credentials, source contents,
or authority-bearing values. Its Debug output must be bounded and redacted.

## 9. Loop Outcome Model

The loop should return one closed result with:

- final Core-derived continuation disposition;
- number of admitted executor entries;
- bounded stop reason;
- optional final successful `SkillOutput` held only in memory; and
- a report-safe summary of whether redispatch remains required.

Candidate stop reasons:

- `AwaitCondition`
- `Blocked`
- `Terminal`
- `ResumeRequiredBudgetExhausted`

Storage, integrity, security, and reconciliation failures should remain
structured `WorkflowOsError` values rather than being flattened into a normal
stop reason.

`ResumeRequiredBudgetExhausted` means durable Core state still says
`ResumeNow`, but this explicit host call consumed its allowed local execution
budget. It must not claim that the workflow is waiting or complete. A later
host call must rehydrate from durable state and obtain fresh authority; it may
not reuse any value returned by the exhausted loop.

## 10. Iteration Algorithm

The private helper should follow this closed sequence:

1. Validate the positive loop budget and initial input shape.
2. Execute the initial opened or resumed capability through the accepted
   one-shot supervisor.
3. Count an iteration only when atomic reservation admits executor entry.
4. Inspect the disposition returned after accepted result/yield/recovery
   persistence.
5. For `AwaitCondition`, return that posture immediately.
6. For `Blocked`, return that posture immediately.
7. For `Terminal`, return that posture immediately.
8. For `ResumeNow`, compare the admitted-entry count with the explicit budget.
9. If exhausted, re-read and verify `ResumeNow`, then return
   `ResumeRequiredBudgetExhausted` without consuming another directive.
10. Otherwise, ask the injected provider for the next bounded operation input.
11. Through one Core-owned path, rehydrate the exact window, derive the current
    directive, consume it at the current cursor, and obtain a private resumed
    attempt capability.
12. Pass that owned capability to the one-shot supervisor, which atomically
    reserves and invokes the next attempt once.
13. Repeat from step 3.

The implementation must not separate directive derivation and consumption
with a host-authoritative cached `ResumeNow` decision. If the existing store
API cannot provide a safe private composition helper, that helper must be
added before the loop rather than approximated in host code.

## 11. Initial And Resumed Dispatch

The initial iteration may begin only from an accepted
`TrustedHostSupervisorAttemptCapability::Opened` or `::Resumed` value supplied
by the caller. The loop does not open a window itself in the first slice.

Every later iteration must originate from the exact current directive. The
loop must never:

- derive an attempt ID from a counter alone;
- reconstruct an attempt capability from rows or events;
- reuse the opening capability;
- reuse a consumed directive;
- reuse a dispatch reservation receipt;
- change executor binding between iterations without a new governed window;
  or
- substitute a new skill input that fails the accepted invocation binding.

## 12. Budget And Fairness Posture

The first implementation should use a small explicit positive integer budget.
No default should be exposed publicly. Tests may use values such as one, two,
or three.

The budget limits executor admissions, not internal reads or reconciliation.
It must not be replenished by retries, exact replay, rejected reservations, or
errors. The implementation should use checked arithmetic and reject zero or
overflowing values with stable non-leaking codes.

This is process-safety posture, not multi-run scheduling fairness. Cross-run
fairness, queues, priorities, rate limits, and worker allocation remain future
host-runtime concerns.

## 13. Typed Wait Behavior

`AwaitCondition` is a successful bounded loop stop. The loop should return
only report-safe condition identities or blocker codes already exposed by the
accepted read-only projection. It must not:

- poll the dependency;
- sleep until a deadline;
- manufacture evidence or check results;
- ask for an approval not currently actionable;
- convert capability unavailability into approval; or
- subscribe to external systems.

Wake-up registration and external event delivery remain separate governed
operations. A later host may re-enter the loop only after Core state changes
and fresh authority is derived.

## 14. Blocked And Terminal Behavior

`Blocked` means recovery or a new governance decision is required. The loop
returns immediately with no hidden retry. Corruption, quarantined trusted
time, ambiguous execution, exhausted attempt posture, and invalidated
authority must remain distinguishable through stable internal errors or
bounded blocker codes.

`Terminal` means the execution window is terminal. It does not necessarily
mean the entire workflow run succeeded. The caller must inspect the durable
run state for workflow-level terminal status.

## 15. Concurrency And Replay

Two loop calls may race against the same durable window. Safety depends on the
accepted one-winner directive-consumption and dispatch-reservation operations,
not on process-local mutexes.

Required behavior:

- only one caller consumes a given directive;
- only one caller reserves a given attempt;
- the losing caller invokes no executor;
- exact replay returns no authority;
- ambiguous commit reconciliation returns no authority;
- stale cursors and revisions fail before executor entry; and
- a later lawful directive may be consumed only from freshly rehydrated state.

The first implementation remains one local process over SQLite, but tests must
use independent connections and concurrent threads where the one-winner
property is being proved.

## 16. Failure And Recovery Semantics

- Invalid input: stable validation error, zero writes, zero executor calls.
- Directive unavailable or stale: stable invalid-state/security result, zero
  executor calls for that iteration.
- Reservation rejected or replayed: stop with error, zero executor calls.
- Reservation commit ambiguous: reconcile, return no capability, stop.
- Executor success/retryable failure/terminal failure/yield: use the existing
  reservation-bound persistence path.
- Executor result ambiguity: use the accepted recovery path and continue only
  if fresh Core state later derives `ResumeNow`.
- Result/yield/recovery commit ambiguity: reconcile exactly; never guess.
- Projection or snapshot mismatch: recovery-required error, no redispatch.
- Iteration input provider failure: stable host-input error, no Core mutation
  for a not-yet-consumed iteration.
- Process crash: a new call starts from durable state and fresh authority; it
  does not deserialize loop-local capabilities.

The loop must not catch a security or recovery error and translate it into a
normal retry.

## 17. Events, Audit, And Reporting

The first loop should not add a new event merely for entering or leaving the
host helper. Existing opening, directive-consumption, dispatch-admission,
outcome, yield, wait, and recovery events remain authoritative.

If implementation proves that a meaningful durable transition is missing, it
must stop and return to planning rather than add unreviewed event vocabulary.

The loop result may expose bounded counts and disposition for future reports.
It must not copy raw invocation input, output, logs, prompts, transcripts, or
provider data into audit or report structures.

## 18. Privacy And Redaction

- Use existing validated constructors and commitments.
- Keep capabilities private, owned, non-cloneable, and non-serializable.
- Redact loop input, provider, capability, invocation, and output Debug fields.
- Do not include caller values or identifiers in error messages.
- Keep durable state limited to accepted identities, commitments, cursors,
  revisions, trusted-time facts, and closed result vocabulary.
- Treat local skill output as in-memory data, not evidence by default.
- Do not persist source paths, command output, environment values, credentials,
  authorization headers, tokens, provider payloads, or model transcripts.

## 19. First Implementation Boundary

The first implementation should be one private Workflow Core slice:

1. Add the bounded loop input, budget, result, and stop-reason models.
2. Add one deterministic iteration-input provider trait for injected local
   test/runtime context.
3. Add the smallest private helper that atomically derives and consumes the
   next directive before constructing resumed supervisor input.
4. Compose the existing one-shot supervisor in a finite loop.
5. Support SQLite only.
6. Add focused unit and integration tests.
7. Update roadmap and create an implementation report.
8. Run a focused maintainer/security review before any provider or sandbox
   integration.

No public export should be added unless compilation requires a crate-visible
test seam. Any such seam must remain explicitly unstable and internal.

## 20. Future Test Plan

Tests should prove:

1. one opened attempt can yield and be redispatched to success in one call;
2. two consecutive `ResumeNow` iterations consume distinct directives and
   reservations;
3. `AwaitCondition` stops with no additional directive or executor call;
4. `Blocked` stops with no hidden retry;
5. `Terminal` stops with no additional read-to-consume path;
6. budget one returns `ResumeRequiredBudgetExhausted` when Core still says
   `ResumeNow`;
7. budget exhaustion does not append completion, failure, approval, or wait;
8. zero budget is rejected before writes or executor entry;
9. two competing loops produce one executor entry for one attempt;
10. exact directive and reservation replay never invoke the executor;
11. stale cursor, revision, invocation, executor, actor, authority, and
    governance bindings fail before executor entry;
12. ambiguous reservation commit returns no authority;
13. every existing executor result remains deterministic inside the loop;
14. ambiguous executor result does not auto-redispatch without accepted
    recovery;
15. process restart can continue only through fresh durable rehydration;
16. event ordering and run snapshot projection remain deterministic;
17. workflow status is not completed merely because the loop stops;
18. Debug and errors do not leak inputs, outputs, paths, IDs, or secret-like
    markers;
19. no provider, filesystem artifact, CLI output, or report artifact is
    created; and
20. existing continuity, opening, supervisor, reservation, adapter, and
    runtime tests remain green.

## 21. Open Questions

- Should the first helper accept the initial attempt capability, or should a
  separately reviewed composition helper also open the window?
- What is the narrowest safe provider interface for per-attempt operation and
  receipt identities without making host input authoritative?
- Should budget exhaustion be a successful result or a dedicated stable host
  error while still exposing `ResumeNow`?
- Can the existing directive store API derive and consume in one safe private
  call, or is a new atomic composition operation required?
- How should a successful attempt that leaves workflow-level work pending be
  distinguished from another attempt in the same window?
- Which bounded wait metadata is safe and useful in the loop result?
- What minimum fault-injection matrix is required before the helper may be
  used by an external host process?

These questions must be resolved by focused maintainer/security review before
implementation.

## 22. Recommended Next Phase

Perform a focused maintainer/security review of this plan. The review should
concentrate on:

- whether directive derivation and consumption are sufficiently atomic;
- whether the input provider can substitute invocation or authority;
- whether budget exhaustion preserves the no-false-stall invariant;
- whether concurrency can cause duplicate executor entry;
- whether ambiguous persistence can trigger unsafe redispatch; and
- whether the proposed result vocabulary separates host posture from workflow
  lifecycle correctly.

If accepted, implement only the private, local, SQLite, injected bounded loop.
Provider execution, OpenShell, nested harnesses, public configuration, CLI,
SDK, schema exposure, hosted behavior, and release changes must remain blocked.

