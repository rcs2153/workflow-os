# Trusted-Host Operational Entry Boundary Plan

Status: implemented; focused implementation review pending. The bounded
trusted-host redispatch loop is implemented and accepted as a private local
SQLite slice. The private operational entry helper described here now
classifies fresh and restarted entry from authoritative SQLite state, reuses
the accepted registered-current-authority opening boundary, consumes fresh
resume authority, and invokes the accepted loop. It is not connected to a
public or automatic runtime caller.

## 1. Executive Summary

Workflow OS can durably open an authorized execution window, supervise one
injected local attempt, consume fresh resume directives, reserve dispatch
atomically, and continue through a bounded private redispatch loop. The loop
still requires a caller-provided one-use capability and exact immutable
invocation input. No accepted boundary yet decides how a fresh trusted-host
process lawfully obtains those values.

The next implementation should add one private local SQLite operational entry
helper. The helper must rehydrate authoritative state, classify the current
entry posture, and either:

- open a new execution window and receive one fresh opening capability;
- consume one current resume directive and receive one fresh resumed
  capability;
- return a bounded typed wait, blocked, or terminal posture; or
- fail closed on stale, replayed, ambiguous, incomplete, or inconsistent
  state.

The helper may then invoke the accepted redispatch loop with the one-use
capability, exact injected `SkillInput`, and exact injected executor. It must
never reconstruct authority from events, receipts, snapshots, reports, or
serialized state. It must never infer invocation payloads from persisted
commitments.

## 2. Current Accepted Foundation

The implementation may compose, but must not weaken:

- immutable run-bundle and exact workflow/run/step binding;
- current actionable-gate and authority resolution;
- atomic operational-window opening and attempt-one start;
- one-use, non-serializable opening and resumed capabilities;
- exact replay and ambiguous-commit reconciliation without capability
  reissuance;
- authoritative continuation dispositions and typed waits;
- one-winner resume-directive consumption;
- one-winner atomic dispatch reservation before executor entry;
- the accepted one-shot trusted-host supervisor;
- the accepted bounded trusted-host redispatch loop;
- append-only events and deterministic snapshot projection; and
- payload-free, redaction-safe durable state.

## 3. Problem Statement

The accepted redispatch loop deliberately starts after authority acquisition.
That keeps the loop small, but leaves a security-sensitive host composition
gap:

1. choose a run, step, and invocation;
2. decide whether the process is opening or resuming;
3. acquire a one-use capability;
4. supply an executor and exact invocation input; and
5. invoke the loop or surface wait/block/terminal posture.

Host glue that guesses any of those facts can execute changed work under old
authority, recreate a capability from a receipt, lose a committed opening and
silently retry it, translate a real wait into conversational termination, or
invoke an executor against stale run state.

The boundary must move this classification and acquisition into one narrow
Core-owned path while keeping execution and sensitive invocation material
injected.

## 4. Goals

- Define one deterministic private entry path for fresh local processes.
- Rehydrate workflow, run, window, attempt, and continuation posture from
  authoritative SQLite state.
- Distinguish lawful initial opening, immediate resume, typed wait, blocked,
  and terminal posture before executor entry.
- Obtain a fresh one-use capability only from the accepted opening or
  directive-consumption operation.
- Validate exact immutable invocation and executor commitments before opening,
  resuming, or dispatching.
- Invoke the accepted bounded redispatch loop without broadening its authority.
- Make restart behavior deterministic without capability serialization or
  reconstruction.
- Return bounded wait/block/terminal information that a future host can
  register and revisit without a manual conversational restart.
- Preserve workflow lifecycle semantics and fail closed on ambiguity.
- Define focused race, crash-window, privacy, and restart tests.

## 5. Non-Goals

- No implementation in this planning phase.
- No scheduler, daemon, queue, polling worker, wake service, or background
  runtime.
- No model-turn or conversation creation.
- No automatic approval or delegated-authority expansion.
- No provider execution or mutation.
- No OpenShell or other sandbox integration.
- No nested harness execution, recursive agents, or agent swarms.
- No public runtime configuration, workflow schema, CLI, SDK, or example.
- No filesystem or PostgreSQL parity in the first implementation.
- No multi-host lease, heartbeat, failover, or reservation stealing.
- No capability persistence, serialization, cloning, or reconstruction.
- No invocation payload persistence or inference from a commitment.
- No approval, SideEffect, WorkReport, reasoning-lineage, or release change.
- No hosted, distributed, production, or release-readiness claim.

## 6. Source-Of-Truth Boundaries

| Concern | Source of truth | Entry must not substitute |
| --- | --- | --- |
| Run identity and lifecycle | Rehydrated durable run events/snapshot | Caller status or assistant memory |
| Immutable run context | Committed immutable run bundle | Current mutable workflow files |
| Entry posture | Current authoritative window and continuation state | Host-selected mode |
| Initial authority | Fresh successful opening transaction | Opening receipt, event, or replay |
| Resume authority | Fresh successful current-directive consumption | Directive projection or cached disposition |
| Executor admission | Atomic dispatch reservation inside supervisor | Entry success or read-only eligibility |
| Invocation content | Injected trusted source, verified against commitment | Persisted payload or guessed reconstruction |
| Executor identity | Injected executor, verified against committed binding | Host label or mutable configuration |
| Wait posture | Durable typed wait records | Generic pause prose or timer guess |
| Completion | Valid durable terminal transition | Successful executor callback or host return |

## 7. Candidate Private API

The first implementation should add the smallest crate-private surface,
provisionally:

- `TrustedHostOperationalEntryInput`
- `TrustedHostOperationalInvocationSource`
- `TrustedHostOperationalIdentityProvider`
- `TrustedHostOperationalEntryOutcome`
- `TrustedHostOperationalEntryStopReason`
- `enter_trusted_host_operation(...)`

Names may follow adjacent module conventions. The surface must remain private
and local to Workflow Core.

The entry input should contain:

- a `SqliteStateBackend` reference;
- an exact workflow/run/step or accepted window locator;
- the existing private `RegisteredInMemoryCurrentAuthoritySource` required by
  `open_with_registered_current_authority` for a fresh opening;
- the existing required-context execution and contract bindings consumed by
  that same-call authority path;
- an injected trusted invocation source;
- an injected `TrustedHostAttemptExecutor`;
- an injected provider for fresh non-authorizing operation identities;
- a trusted-time observation source already accepted by continuity state; and
- the bounded opening parameters already required by the accepted operation.

The invocation source may return only the exact in-memory `SkillInput` and the
minimum binding facts needed to validate it. It does not grant authority,
select a different step, alter the immutable run bundle, or read secrets from
Workflow OS durable state.

The identity provider may generate only fresh bounded identifiers. It may not
inspect or return capabilities, invocation payloads, authority commitments,
or durable records.

## 8. Entry Classification Algorithm

The future helper should perform this closed sequence:

1. Rehydrate the run and verify the requested workflow/run/step identity.
2. Load the immutable run bundle and current authoritative continuity state.
3. Obtain the exact in-memory invocation and executor commitments from the
   injected sources and validate them against the immutable binding.
4. If no execution window exists and the run is eligible, call the existing
   `open_with_registered_current_authority` boundary with the registered
   in-memory source, required-context bindings, exact invocation commitment,
   current trusted time, and fresh non-authorizing identities. The entry helper
   must not construct `OperationalExecutionWindowOpeningAuthorization`.
5. Let that existing same-call authority use construct the private opening
   authorization and commit the accepted opening transaction once. Continue
   only when it returns a fresh owned opening capability.
6. If a window exists, derive the current continuation disposition freshly.
7. For `ResumeNow`, consume exactly one current directive and obtain a fresh
   resumed capability through the accepted projected operation.
8. For `AwaitCondition`, return a bounded typed-wait outcome without polling or
   executor entry.
9. For `Blocked`, return a bounded blocked outcome without fabricating an
   approval request.
10. For `Terminal`, return a bounded terminal outcome without executor entry.
11. Pass the fresh owned capability, exact invocation, executor, and fresh
    persistence identities to the accepted bounded redispatch loop.
12. Return the loop's non-resumable outcome or structured error unchanged.

The classification and authority operation must remain in one Core-owned call.
The host may choose which accepted run to service, but it may not choose an
entry posture that conflicts with durable state.

## 9. Initial Opening And Replay

A new window may be opened only from current eligible run state and current
authority validated by the existing registered-current-authority same-call
use. The entry helper must reuse `open_with_registered_current_authority`; it
must not accept caller-authored opening authorization, governance commitment,
or gate-readiness posture. The operation returns an attempt-use capability
only after an unambiguous commit.

Exact replay or fresh-connection reconciliation may prove that an opening was
committed, but neither may mint another capability. If a process loses the
fresh capability after commit and before supervisor admission, the entry path
must not rerun the executor or reconstruct authority. It must return a stable
recovery-required error or a future explicitly authorized recovery posture.
The first implementation should fail closed unless an already accepted
continuity operation supplies a lawful fresh capability.

This limitation is intentional. Durable proof that authority was once issued
is not current authority to use it again.

## 10. Restart And Rehydration

A process restart reruns the entry algorithm against fresh SQLite state:

- no window and eligible run: attempt one fresh opening;
- active yielded window with `ResumeNow`: consume one current directive;
- active wait: surface the typed condition;
- blocked window: surface the bounded block;
- terminal window: return terminal posture;
- executing, recovery-required, ambiguous, or inconsistent state: fail closed
  or surface the exact accepted recovery posture without executor entry.

The process must not persist or recover an in-memory capability. Restart
safety comes from re-deriving current posture and obtaining fresh authority
through an accepted atomic operation, not from serializing authority.

## 11. Wait And Blocked Handoff

The first implementation should return a bounded internal result containing
only the minimum non-sensitive facts required by a future host:

- run/window identity;
- closed stop reason;
- current durable cursor or revision commitment;
- typed wait-condition identities and revisions when applicable; and
- a stable non-leaking next-operation category.

It must not return raw evidence, approval reasons, prompts, command output,
provider payloads, credentials, or source content.

This phase does not implement wake registration. It establishes that a future
host can persist or register the returned condition and invoke the same entry
boundary after a lawful external state change. Ending an agent turn is never
recorded as workflow completion.

## 12. Failure And Crash Windows

- Before opening or directive-consumption commit: no capability exists; a
  future entry may retry with fresh state and safe idempotency semantics.
- Ambiguous opening commit: reconcile for durable truth, but withhold authority
  even if committed.
- After committed opening but before capability use: do not recreate the
  capability; fail closed pending accepted recovery semantics.
- Before dispatch-reservation commit: executor is not invoked.
- Ambiguous dispatch-reservation commit: executor is not invoked and existing
  reconciliation posture applies.
- After executor entry: accepted outcome/yield/ambiguous-attempt persistence
  remains authoritative; entry must not guess or retry.
- Invocation or executor substitution: reject before authority acquisition or
  executor entry.
- Stale cursor, revision, wait generation, or immutable binding: reject before
  executor entry.

Errors must use stable codes and static bounded messages. They must not echo
identifiers supplied by an invalid caller, paths, payloads, tokens, secrets,
or provider output.

## 13. Concurrency

Two entry callers may inspect the same run, but only the accepted atomic
operations may decide the winner:

- opening rejects active-window conflict or exact replay without reissuing
  authority;
- resume-directive consumption admits one current operation;
- dispatch reservation admits one executor caller;
- stale callers fail before executor entry.

The helper must not add process-local locks as the authority boundary. Tests
should use independent SQLite connections to prove one-winner behavior.

## 14. Privacy And Redaction

- `SkillInput` remains in memory and is never persisted by the entry helper.
- Capabilities remain private, owned, non-serializable, and non-cloneable.
- Debug output redacts invocation, executor, authority, commitment, identity,
  and wait binding details.
- Wait/block outcomes contain bounded identities and closed vocabulary only.
- Serde and validation errors never echo rejected values.
- No prompt, transcript, source/spec content, command output, environment
  value, credential, token, authorization header, provider payload, approval
  reason, evidence body, or raw output is stored.

## 15. Test Plan

The implementation and reused primitive coverage should prove:

1. a fresh eligible run opens once and enters the existing loop;
2. a restart at `ResumeNow` consumes one fresh directive and resumes;
3. `AwaitCondition` returns without polling, identity generation, or executor
   entry;
4. `Blocked` returns without retry or fabricated approval;
5. `Terminal` returns without executor entry;
6. exact opening replay returns no capability and invokes no executor;
7. ambiguous opening commit reconciles but does not invoke the executor;
8. committed-opening capability loss fails closed;
9. two concurrent fresh entry callers admit at most one executor;
10. two concurrent resume callers admit at most one executor;
11. stale cursor/revision/wait posture fails before executor entry;
12. changed `SkillInput` or executor binding fails before authority use;
13. process-local restart uses fresh state and never serializes authority;
14. typed wait results are bounded and redaction-safe;
15. Debug and errors do not leak secret-like values;
16. execution-window terminal posture does not complete the workflow; and
17. existing opening, supervisor, reservation, redispatch, runtime, adapter,
    report, and persistence tests remain green.

## 16. Implementation Sequence

1. The focused maintainer/security plan review is complete.
2. The private entry classification and bounded outcome composition is
   implemented.
3. Existing opening, directive-consumption, and redispatch operations are
   reused without adding public APIs.
4. Focused restart, blocked, terminal, missing-opening-context, substitution,
   and privacy tests are implemented; reused primitive suites retain replay,
   ambiguity, and concurrency proofs.
5. Complete workspace validation is required for phase close.
6. A focused implementation review remains next.
7. Only after acceptance may a separate phase plan a caller that registers
   typed waits and reinvokes this boundary after lawful external changes.

## 17. Open Questions

- Is a committed-but-unused opening necessarily `recovery_required`, or does
  the existing continuity state need one additional closed recovery operation?
- What is the smallest safe locator for a host to choose a run without making
  the host authoritative for step selection?
- Should typed wait handoff expose revision commitments directly or a single
  opaque bounded handoff identity?
- How should trusted time and fresh operation identities be injected without
  allowing either provider to influence governance decisions?
- Can the exact immutable invocation source be bound to the run bundle without
  persisting sensitive input material?

These questions require focused security review before implementation. None
authorizes capability reconstruction, payload persistence, scheduling, or
provider execution.

## 18. Final Recommendation

Proceed next to a focused maintainer/security implementation review of the
private local SQLite operational entry helper. Verify that it obtains fresh
authority only from the accepted opening or directive-consumption operations,
never reconstructs capability, binds the exact invocation and executor, and
returns bounded wait, blocked, or terminal posture without executor entry.

Do not add a scheduler, daemon, public runtime API, CLI, SDK, schema, provider
execution, provider mutation, OpenShell, nested harnesses, automatic approval,
hosted behavior, or release posture during that review.
