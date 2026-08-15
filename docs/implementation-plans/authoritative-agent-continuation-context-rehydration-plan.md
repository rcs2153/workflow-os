# Authoritative Agent Continuation Context And Rehydration Plan

Status: Accepted after focused maintainer/security review. The first opt-in
local immutable-run `BeforeSkillInvocation` continuation slice is implemented
and focused-review accepted. The read-only local Core and CLI preview is also
implemented and focused-review accepted. The registered-current-authority
consumer and its final composition test blocker fix are implemented and
focused-review accepted in [Authoritative Continuation Registered
Current-Authority Consumer Plan](authoritative-continuation-registered-current-authority-consumer-plan.md).
Typed child runtime behavior and operational source configuration remain
deferred.

Related foundations:

- [Governed Work Pattern](../concepts/governed-work-pattern.md)
- [Typed Handoff Plan](typed-handoff-plan.md)
- [Required Context Contract Consumption Plan](required-context-contract-consumption-plan.md)
- [Required Context Immutable-Run Binding And Time-Of-Use Plan](required-context-immutable-run-time-of-use-plan.md)
- [Current-Authority One-Time-Use And Replay Posture Plan](current-authority-one-time-use-replay-posture-plan.md)
- [Runtime Proportional-Governance Reassessment Plan](runtime-proportional-governance-reassessment-plan.md)
- [BeforeSkillInvocation Required Checkpoint Plan](before-skill-required-checkpoint-plan.md)
- [Self-Governed Build Benchmark](../user-guide/self-governed-build-benchmark.md)

## 1. Executive Summary

Long-running agent sessions can compact conversation history. Parallel agents
and subagents can start from isolated or summarized context. In both cases, a
model may infer its next action from stale conversational understanding instead
of current Workflow OS state.

This is not primarily a model-quality or prompt-quality defect. Conversation
context is probabilistic working memory. Workflow OS durable state, immutable
run inputs, current authority, policy, approval, evidence, checks, SideEffects,
and event history are the governance source of truth.

The future runtime invariant is:

```text
The agent may remember or propose the next step.
Only the kernel may declare and authorize the next material action.
```

The first implementation should compose existing foundations into one local
end-to-end continuation boundary. It should expose a bounded continuation brief
for agent orientation, then freshly re-resolve and consume one exact material
operation through a private Core-owned same-call boundary. It must not create a
reusable authority token or treat a serialized brief as permission.

## 2. Problem Statement

The current repository provides durable run state and selected proof-enforced
runtime paths, but an agent can still choose a subsequent repository action
from compacted local context between governed checkpoints. A delegated agent
may receive only a parent-authored summary and may not receive current dynamic
run state at all.

That creates several failure classes:

- stale run status causes already-completed or denied work to continue;
- a pending, expired, revoked, or superseded approval is treated as valid;
- a changed policy, check, evidence, authority, or SideEffect fact is missed;
- two parallel agents act from the same old event position;
- a natural-language handoff widens scope or omits strict non-goals;
- a compacted summary becomes more influential than durable kernel state;
- a parent accepts a child summary without proving which governed task produced
  it;
- an agent reuses an earlier next-action result for a different operation.

Static `AGENTS.md` or harness prompts can require kernel use, but they cannot
prove current run state or enforce one exact continuation. Better summaries are
useful UX, not a correctness boundary.

## 3. Goals

- Make current durable kernel state authoritative for every material
  continuation.
- Rehydrate current run, event, approval, assessment, authority, context,
  evidence/check, and SideEffect posture before one exact operation.
- Give agents a bounded machine-readable summary of current posture without
  making that summary an authority credential.
- Bind delegated work to typed handoff identity, scope, inputs, non-goals,
  expected outputs, validation, evidence, and failure semantics.
- Reject stale, incomplete, mismatched, replayed, or ambiguous continuation.
- Preserve quiet execution for eligible low-risk reads through proportional
  governance.
- Keep model and harness vendors replaceable.
- Record enough bounded evidence to explain which kernel state governed an
  operation.
- Make compaction and parent/child context loss harmless to correctness.

## 4. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- model chain-of-thought capture or persistence;
- treating conversation history as durable workflow state;
- reusable bearer tokens or TTL-based authority leases;
- model self-approval or authority inferred from model identity;
- arbitrary recursive agent spawning or agent swarms;
- nested harness runtime execution;
- provider mutation broadening or default writes;
- automatic command execution outside an accepted handler boundary;
- universal enforcement over arbitrary shell, editor, git, browser, or provider
  actions performed outside an integrated Core consumer;
- workflow schema changes in the first slice;
- hosted or distributed execution;
- enterprise identity, RBAC, or administration;
- raw prompt, source, tool-output, provider-payload, or credential persistence;
- detecting every vendor-specific compaction event as a correctness dependency.

## 5. Core Invariants

1. A conversational summary is advisory and never sufficient for a material
   operation.
2. A serialized continuation brief is orientation, not authority.
3. Every material operation resolves current facts in the same Core-owned call
   that reaches the concrete consumer.
4. The operation must match one exact allowed next-action code and resource
   scope.
5. Current facts may preserve or increase governance strictness; stale context
   cannot weaken it.
6. Required context gaps, source failure, stale facts, unknown state, or
   identity mismatch block before consumer invocation.
7. Continuation authority is non-cloneable, non-serializable, non-reusable, and
   operation-specific.
8. Subagent results without an exact accepted typed handoff remain untrusted
   supporting input.
9. Low-risk refresh may be quiet, but it must remain durable and inspectable.
10. Correctness must hold even when the model never reports that compaction or
    context loss occurred.

## 6. Existing Foundations And Remaining Gap

Workflow OS already has:

- immutable run bundles and exact run-definition binding;
- durable event history and run rehydration;
- approval-presentation proof and selected approval-resume reassessment;
- proportional-governance assessment fingerprints;
- scoped capability and governed-context projection models;
- required-context contract consumption;
- registered current-authority source and same-call one-time-use boundaries;
- typed handoff core vocabulary;
- selected `BeforeSkillInvocation` fail-closed checkpoints;
- a governed dogfood phase runner with bounded approval handoff output.

These foundations do not yet create a general continuation contract for an
agent or child harness. Typed handoffs do not have broad runtime behavior.
Automatic executor checkpoints are limited. The dogfood runner governs phase
start and close, but it does not mechanically require fresh kernel context
before every material continuation performed by the external executor.

## 7. Candidate Core Model

The first model review should consider the smallest set of types needed for a
future continuation path:

- `GovernedContinuationBriefId`
- `GovernedContinuationBrief`
- `GovernedContinuationBinding`
- `GovernedNextAction`
- `GovernedContinuationBlocker`
- `GovernedContinuationContextPosture`
- `GovernedDelegationBinding`
- `GovernedContinuationUseOutcome`

The names are provisional. The model must reuse existing IDs, immutable bundle
bindings, assessment bindings, required-context results, authority source
commitments, approval proof markers, check/evidence references, SideEffect
references, and typed handoff IDs rather than defining parallel substitutes.

## 8. Governed Continuation Brief

The brief is a bounded, redaction-safe projection intended for agents,
operators, hooks, and machine consumers. It should include only stable identity
and posture:

- workflow, run, phase, and current step identity;
- immutable bundle identity and root;
- current event cursor, sequence, or state revision;
- terminal, waiting, running, blocked, or failed run posture;
- pending approval identity and presentation-proof posture;
- current proportional-governance assessment identity and fingerprint;
- authority source identity, freshness posture, and bounded watermark identity;
- required-context satisfaction and explicit gaps;
- required evidence and check posture;
- SideEffect and retry posture;
- typed handoff identity and expected target when delegated;
- allowed next-action codes;
- explicit blockers and strict non-goals;
- generated-at time and brief algorithm/version identity.

The brief must not contain raw prompts, source contents, command output,
provider payloads, tokens, credentials, authorization headers, environment
values, private keys, model reasoning, or unbounded summaries.

Brief validation must detect field substitution, non-canonical ordering,
duplicate obligations, mismatched execution scope, and stale binding. A valid
brief remains non-authoritative for execution.

## 9. Same-Call Continuation Use Boundary

Immediately before one material operation, Core should:

1. load and validate the exact durable run;
2. load the exact immutable run bundle;
3. read the current event position and terminal posture;
4. verify approval decision and presentation proof when applicable;
5. recompute proportional governance from current registered facts;
6. resolve current capability and authority;
7. rebuild governed context projection;
8. consume the exact required-context contract;
9. validate current evidence, checks, sensitivity, and SideEffect posture;
10. derive allowed next-action vocabulary;
11. require the requested operation and target to match exactly; and
12. invoke one concrete Core-owned consumer through the existing non-reusable
    same-call pattern.

The first consumer should be narrow and local. It should not expose a generic
callback that turns the boundary into reusable ambient authority.

### 9.1 Atomic Continuation Consumption

A non-cloneable in-process value is not sufficient when two workers or agents
can observe the same durable run position. The first slice must therefore bind
one exact `BeforeSkillInvocation` operation to:

- the immutable run root;
- the current last event sequence and event ID;
- the current step and invocation idempotency key;
- the selected next-action code; and
- the current assessment, authority, context, approval, evidence, check, and
  SideEffect commitments.

Core must create one durable idempotency claim for that exact binding. Only the
first writer may continue. Before invoking the consumer, Core must re-read the
event position and reject the operation when it no longer matches the claimed
cursor. A cursor mismatch burns that exact claim and requires a fresh
rehydration with a new cursor-bound claim; it must not retry under the stale
binding. This gives the first local slice deterministic single-consumption
behavior without creating a reusable permission lease.

The first slice should allow exactly one material next action:
`invoke_current_step_skill`. Expanding the vocabulary requires a separate
review once another concrete consumer exists.

## 10. Required Rehydration Checkpoints

Correctness should use unconditional current-state resolution at material
boundaries rather than relying on compaction detection. Candidate checkpoints:

- session or worker resume;
- run retry;
- approval resume;
- before each governed skill invocation;
- before required check execution;
- before report or artifact production;
- before SideEffect attempt or provider call;
- before subagent or child harness launch;
- before accepting a child result;
- before workflow or phase completion.

Vendor-specific compaction, interruption, or subagent lifecycle events may be
recorded when available, but absence of those events must not weaken the
boundary.

## 11. Typed Subagent And Harness Handoffs

Delegated execution must not rely on a parent-generated prose summary alone.
The kernel should bind each child task to:

- parent workflow, run, step, event position, and immutable root;
- child actor or harness identity;
- exact approved scope and strict non-goals;
- required and optional governed context references;
- allowed capabilities, tools, and SideEffect posture;
- expected output and artifact schema or stable vocabulary;
- evidence and validation requirements;
- timeout, retry, cancellation, and failure semantics;
- acceptance authority and next obligation.

The child begins from a kernel-produced continuation brief. Material child
operations still require fresh same-call resolution. The parent accepts the
result only after exact typed handoff validation. Missing, stale, superseded,
or mismatched handoffs fail closed without converting a natural-language report
into fake evidence.

## 12. Proportional Governance And Quiet Refresh

Current-state rehydration is not synonymous with a human interruption.

- Eligible bounded reads may refresh under `Proceed + Quiet` and record the
  resulting posture without verbose output.
- Review-worthy changes may use `Proceed + Visible` disclosure.
- Approval-sensitive actions must preserve proof-enforced blocking approval.
- Denied, stale, ambiguous, missing-context, or source-failure posture blocks.

This keeps automation fast while preventing stale conversational state from
becoming an authorization shortcut.

## 13. Event, Audit, Evidence, And Report Posture

Future continuation events should remain payload-free and may record:

- brief identity and algorithm version;
- immutable bundle root reference;
- source snapshot or watermark identity;
- event cursor or state revision;
- requested and selected next-action code;
- typed handoff identity;
- assessment, approval-proof, evidence, check, and SideEffect references;
- blocked, consumed, failed, or ambiguous outcome;
- actor and correlation identity.

The WorkReport should disclose continuation gaps, stale-context rejection,
delegation posture, skipped refreshes, and work performed outside the governed
boundary. It must not copy conversation history or subagent transcripts.

## 14. Failure Behavior

- Missing current source: fail closed before material consumer access.
- Stale brief: regenerate for orientation; do not reuse it as authority.
- Event cursor mismatch: rehydrate and reassess.
- Terminal or superseded run: reject continuation.
- Pending, denied, expired, or mismatched approval: block.
- Revoked or expired authority: block.
- Missing required context, evidence, or checks: block or disclose only when the
  exact contract marks the obligation optional.
- Child result without an accepted handoff: reject as governed completion.
- Consumer may-have-started ambiguity: preserve existing conservative recovery
  and retry-blocking semantics.

Errors must use stable codes and must not echo IDs, paths, prompts, summaries,
payloads, tokens, or secret-like values.

## 15. Proposed Implementation Sequence

Complete the enforcement proof as one reviewed local vertical slice rather
than an extended model-only sequence:

1. Focused maintainer/security plan review.
2. Add the minimal continuation brief, binding, next-action, blocker, and
   outcome model.
3. Add one pure brief projection from an exact immutable run and current
   registered facts.
4. Add one durable cursor-bound idempotency claim and private Core-owned
   same-call continuation consumer for the local `BeforeSkillInvocation` path.
5. Add bounded continuation events and focused tests proving stale-cursor and
   parallel-consumption rejection before skill invocation.
6. Integrate only the selected dogfood `BeforeSkillInvocation` path so one
   material dogfood continuation proves current brief projection and
   authoritative same-call consumption. Disclose that arbitrary repository,
   shell, editor, git, browser, and provider actions outside this path remain
   governed procedurally rather than intercepted mechanically.
7. Perform focused maintainer/security review before any provider-write or
   nested-harness expansion resumes.

The read-only `workflow-os next-action <run-id>` preview is now implemented for
exact local immutable runs whose current hook and SideEffect context is fully
reconstructable. Human and JSON output expose the same bounded brief and state
explicitly that preview is non-authoritative and non-consuming. Projection
does not claim idempotency, append events, invoke handlers, write artifacts, or
change run state. Typed child handoff start/result acceptance remains deferred.

## 16. Test Plan

Future tests must cover:

- a current brief projects the exact durable run and immutable root;
- a brief cannot authorize an operation by itself;
- exact allowed next action reaches the selected consumer once;
- two concurrent consumers at the same cursor produce one durable first writer
  and one fail-closed duplicate outcome;
- stale event cursor blocks before consumer invocation;
- compacted or intentionally incomplete agent context cannot bypass rehydrate;
- approval granted after brief creation requires fresh resolution;
- approval denial, expiry, revocation, or supersession blocks;
- authority revocation after child launch blocks the child operation;
- changed policy, check, evidence, sensitivity, or SideEffect posture
  invalidates the earlier continuation;
- completed, canceled, failed, or superseded runs reject continuation;
- a cursor change after the durable claim but before consumer invocation burns
  the stale claim and requires a fresh cursor-bound claim;
- parallel agents cannot consume the same operation authorization twice;
- child results require exact typed handoff identity and parent binding;
- natural-language-only child summaries are not accepted as governed results;
- unrelated definition changes do not churn the immutable execution binding;
- relevant definition changes cannot alter an active immutable bundle;
- low-risk quiet refresh remains auditable without human interruption;
- debug, serialization, errors, events, and reports do not leak forbidden
  content;
- existing executor, approval, immutable-run, proportional-governance,
  authority, required-context, typed-handoff, hook, SideEffect, and WorkReport
  tests remain green.

## 17. Acceptance Criteria

- Current kernel state, not model memory, determines the next material action.
- Every selected material consumer has a fresh same-call rehydration boundary.
- The selected operation has a durable cursor-bound single-consumption claim.
- The continuation brief is explicitly non-authoritative.
- Missing or stale context fails closed before consumer invocation.
- Subagent launch and result acceptance use exact typed handoff identity.
- Compaction detection is optional observability, not a correctness dependency.
- Low-risk work can remain quiet under proportional governance.
- No provider mutation broadening, nested harness runtime, schema expansion, or
  hosted claim is introduced by the first slice.

## 18. Open Questions

- Which additional next-action code should follow
  `invoke_current_step_skill`, once another concrete consumer is approved?
- Should later event-position versions replace the first slice's last sequence
  plus last event ID with a versioned aggregate commitment?
- Which existing dogfood `BeforeSkillInvocation` step is the safest first
  concrete consumer?
- Should the read-only CLI preview return one strict next action or a bounded
  set of independently eligible actions?
- How should parent and child completion race without granting either ambient
  mutation authority?
- Which agent platforms expose lifecycle hooks useful for observability, while
  preserving unconditional kernel resolution as the portable invariant?

## 19. Final Recommendation

The first atomic local consumer and its read-only preview are implemented and
focused-review accepted. The registered-current-authority-backed consumer is
now implemented as one crate-private composition of fresh source resolution,
exact required-context consumption, the durable cursor-bound continuation
claim, and the current local skill invocation. It awaits focused
maintainer/security review and does not create trusted runtime source
configuration or a public authority API.

Do not treat `next-action` output as permission, add typed child-handoff runtime
behavior, or resume broader provider mutation or nested harness execution until
the separately reviewed source-backed continuation boundary exists.
