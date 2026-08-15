# Authorized Execution Continuity Plan

Status: Accepted after focused maintainer/security review in
[Authorized Execution Continuity Plan Review](../concepts/AUTHORIZED_EXECUTION_CONTINUITY_PLAN_REVIEW.md).
The core continuity decision model is implemented and accepted after focused
phase review. Runtime events, durable-state integration, supervisor scheduling,
and delegated approval remain unimplemented. This P0 plan follows the accepted crate-private
source-backed authoritative continuation proof. It does not implement runtime
continuity, host scheduling, provider mutation, nested harness execution, or
automatic approval.

Related foundations:

- [Authoritative Agent Continuation Context And Rehydration Plan](authoritative-agent-continuation-context-rehydration-plan.md)
- [Scoped Runtime Authority And Capability Projection Plan](scoped-runtime-authority-capability-projection-plan.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Approval Gate Presentation Default Enforcement Plan](approval-gate-presentation-default-enforcement-plan.md)
- [Current-Authority One-Time-Use And Replay Posture Plan](current-authority-one-time-use-replay-posture-plan.md)
- [Governed Work Pattern](../concepts/governed-work-pattern.md)

## 1. Executive Summary

Workflow OS can now rehydrate fresh registered authority, consume required
context, claim one exact cursor-bound continuation, and enter one local skill
consumer. That proof prevents stale conversational context from authorizing a
material operation. It does not yet preserve lawful work across external
executor turn boundaries.

The observed dogfood failure is a false stall: a run has approved, lawful work
remaining, but the external agent ends a turn and the host treats that turn
boundary as an ordinary stopping point. The kernel has not declared the run
terminal and may not be waiting for approval, evidence, or an external event.
Conversation lifecycle is being mistaken for workflow lifecycle.

The future invariant is:

```text
Only the kernel may classify governed work as runnable, waiting, blocked, or
terminal. An executor turn may yield; it may not silently complete a
non-terminal run.
```

Workflow OS should model actionable gate readiness, durable execution windows,
executor yield, typed wait conditions, and authoritative resume directives.
A host supervisor should use those records to schedule or resume an executor
while lawful work remains. Core cannot create a model turn by itself and must
not claim that it can.

## 2. Problem Statement

The current run projection distinguishes running, approval wait, external-event
wait, retry, escalation, and terminal statuses. It does not distinguish an
actively authorized external executor window from an executor that yielded
while work remains. It also permits product surfaces to present an approval
request without one common first-class statement that all prerequisite
evidence and checks are satisfied.

This creates several failure classes:

- an agent turn ends while a run remains lawfully runnable;
- a host does not know whether to resume, wait, or stop;
- an approval request is presented before its evidence obligations are met;
- a standing delegation is confused with permission to bypass a human-only
  gate or separation-of-duty rule;
- a natural-language final response implies completion while durable run state
  remains non-terminal;
- a genuine dependency wait has no typed condition or deterministic wake-up
  posture;
- manual user prompting becomes the accidental scheduler;
- repeated resumes can race or replay against the same old event cursor.

These are kernel and host-contract gaps, not prompt-quality problems.

## 3. Goals

- Make authorized external work a first-class durable workflow posture.
- Keep approval readiness separate from approval decision and execution
  authority.
- Prevent an approval gate from becoming actionable until required evidence,
  checks, policy, authority, and presentation obligations are satisfied.
- Preserve one scoped execution window across executor turn boundaries.
- Record executor yield without marking the run complete, failed, or waiting
  for approval.
- Produce an authoritative resume directive while lawful work remains.
- Represent genuine waits with typed conditions and explicit wake criteria.
- Let a host supervisor resume eligible work without manual restart.
- Preserve source-backed, same-call, one-time-use authorization for every
  material operation within a window.
- Support scoped delegated approval authority without bypassing human-only,
  quorum, requester/approver separation, or evidence requirements.
- Keep every transition deterministic, event-sourced, redaction-safe, and
  restart-safe.

## 4. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- automatic model self-approval;
- satisfying missing evidence with an approval decision;
- bypassing explicit human-only or separation-of-duty requirements;
- a reusable serialized bearer token for execution;
- treating conversation memory or a final response as workflow authority;
- a hosted scheduler, queue, worker fleet, or distributed lease service;
- a promise that Core alone can create or resume an agent turn;
- arbitrary shell, editor, git, browser, or provider automation outside an
  integrated consumer;
- provider mutation broadening;
- nested harness runtime, recursive agents, or agent swarms;
- workflow schema, SDK, CLI automation, UI, or release posture changes in the
  first model phase;
- enterprise identity, RBAC, IdP, quorum approval, or remote administration;
- weakening immutable-run, capability, required-context, approval,
  proportional-governance, SideEffect, or evidence boundaries.

## 5. Source-Of-Truth Boundaries

| Concept | Source of truth | Must not be confused with |
| --- | --- | --- |
| Run lifecycle | Durable workflow events and rehydrated snapshot | Agent turn or final response |
| Gate readiness | Fresh deterministic prerequisite assessment | Approval request or decision |
| Approval decision | Valid decision bound to one ready request | Execution grant or evidence |
| Execution window | Durable bounded envelope for lawful external work | Reusable permission to perform any action |
| Continuation directive | Fresh same-call exact next-action authorization | Serialized brief or old model context |
| Executor yield | Durable acknowledgement that an executor turn ended | Workflow completion or approval wait |
| Wait condition | Typed unmet dependency with wake posture | Generic pause text |
| Host supervisor | Scheduler that responds to kernel directives | Governance authority |
| Delegated capability | Scoped authority to decide eligible gates | Human identity or blanket self-approval |
| Work report | Governed terminal handoff | Mechanism for making a run terminal |

## 6. Core Invariants

1. Only durable kernel state may classify a run as runnable, waiting, blocked,
   or terminal.
2. A run is terminal only after a valid terminal event is durably appended.
3. A final agent response cannot append or imply a terminal event.
4. An executor turn ending during an open execution window records yield, not
   completion.
5. A gate is actionable only after all declared prerequisites are currently
   satisfied and bound to the exact request context.
6. Approval cannot satisfy its own missing evidence, check, policy,
   presentation, or authority prerequisite.
7. Delegated approval authority must be explicit, scoped, current, and allowed
   by the gate's actor and separation-of-duty rules.
8. An execution window is durable orientation and scheduling state, not a
   reusable bearer credential.
9. Every material operation still requires fresh current-authority resolution,
   exact immutable binding, a current event cursor, and a one-time durable
   claim in the same Core-owned call as the consumer.
10. Current facts may narrow, revoke, expire, or close a window before another
    operation.
11. A wait condition must identify a typed dependency and deterministic wake
    posture; unknown dependency state fails closed.
12. At most one active executor claim may own one exact resume directive.
13. Wait/yield registration and directive consumption must be atomic at an
    expected durable event cursor; separate best-effort writes are insufficient.
14. Host scheduling failure leaves the run non-terminal and inspectable; it
    does not fabricate completion.
15. Errors and events contain stable codes and bounded references, not raw
    prompts, source, logs, payloads, credentials, or secrets.

## 7. Candidate Core Model

The first implementation should add only the smallest domain-neutral model
needed to validate continuity decisions. Candidate vocabulary:

- `GovernedExecutionWindow`
- `GovernedExecutionWindowId`
- `GovernedExecutionWindowStatus`
- `GovernedExecutionWindowBinding`
- `ActionableGateAssessment`
- `ActionableGateStatus`
- `ActionableGateBlocker`
- `ExecutorYieldRecord`
- `ExecutorYieldReason`
- `GovernedWaitCondition`
- `GovernedWaitConditionKind`
- `GovernedWaitWakePosture`
- `AuthoritativeResumeDirective`
- `AuthoritativeResumeDisposition`
- `ExecutionContinuityDecision`

Names are provisional. Existing run, workflow, step, actor, approval,
capability, assessment, immutable-bundle, event-sequence, handoff, check,
evidence, and SideEffect types should be reused instead of duplicated.

The model should not initially add `WorkflowRunStatus` variants. First prove a
separate continuity decision model and projection against existing durable run
state. Event and snapshot integration should follow only after the model review
accepts exact transition semantics and compatibility posture.

## 8. Actionable Gate Readiness

An approval request should be presentable and decidable only when a current
`ActionableGateAssessment` projects `ReadyForDecision`. The assessment is a
non-authoritative commitment and operator projection, not permission. Candidate
statuses:

- `PendingPrerequisites`
- `ReadyForDecision`
- `Decided`
- `Expired`
- `Revoked`
- `Superseded`

Prerequisites should be typed and may include:

- policy decision passed;
- required evidence references present and current;
- required check attestations accepted;
- approval presentation proof persisted;
- current actor and capability posture resolved;
- requester/approver separation satisfiable;
- immutable run and exact action binding present;
- SideEffect proposal and sensitivity posture complete;
- no superseding event cursor or changed workload assessment.

`PendingPrerequisites` is not an actionable approval. Operator UX may disclose
the blockers, but must not offer an approval command that cannot lawfully
succeed. An approval decision received against a non-ready assessment fails
closed and appends no grant or resume event. The decision consumer must reload
the owning evidence, check, policy, presentation, authority, and immutable-run
records and re-evaluate readiness in the same call; it must never trust a
serialized `ready` value.

## 9. Execution Window Semantics

An execution window records that one external executor may continue bounded
work for an exact run and scope. It should include:

- window ID and version;
- workflow, run, immutable bundle, and optional step/handoff scope;
- subject actor or harness identity;
- opening authority and approval references;
- allowed action classes and resource scope;
- event cursor at open;
- issued-at and expiry or bounded operation budget;
- revocation and closure posture;
- required current-authority source binding;
- proportional-governance and sensitivity ceiling;
- redaction metadata.

The window remains active across executor turns until it is completed,
expired, revoked, superseded, exhausted, or blocked by stricter current facts.
It does not authorize an operation by possession. Before each material action,
Core must rehydrate current state and derive a fresh one-time continuation
directive through the accepted source-backed consumer boundary.

## 10. Executor Yield

`ExecutorYieldRecord` means an external executor stopped producing work while
the governed run remained non-terminal. Candidate reasons:

- `TurnBoundary`
- `ContextBudget`
- `HostPreemption`
- `VoluntaryCheckpoint`
- `TransientExecutorFailure`

Yield must record bounded identity, window, last observed event cursor,
timestamp, executor identity, and whether a fresh resume assessment is
required. It must not contain transcript, chain-of-thought, raw tool output, or
arbitrary free-form explanations.

Yield is not:

- run completion;
- failure;
- cancellation;
- approval wait;
- evidence satisfaction;
- a promise that the same executor process will return.

## 11. Typed Wait Conditions

When work cannot lawfully continue, the kernel should register a typed wait
condition rather than rely on a final response. Candidate kinds:

- `HumanDecision`
- `EvidenceRequired`
- `CheckRequired`
- `ExternalEvent`
- `CapabilityUnavailable`
- `TimeWindow`
- `AuthorityRefresh`
- `ConflictResolution`

Each condition should include exact run/window/action binding, stable blocker
codes, required references, condition ID and version, expected event cursor,
step/attempt identity, created-at, optional deadline, wake trigger kind, and
current status. It must not include raw evidence, provider payloads,
commands, paths, or secret-like values.

Wait conditions must be monotonic within one assessment: satisfying one
condition may expose another prerequisite, but may not silently weaken the
active profile or policy. Fresh facts may increase strictness.

## 12. Authoritative Resume Directive

After rehydration, Core should derive one of these dispositions:

- `ResumeNow`
- `Wait`
- `Blocked`
- `Terminal`

`ResumeNow` must bind the active window, exact next action, immutable bundle,
current event cursor, subject, current authority commitment, required-context
consumption, and expiry. It must be private, non-serializable as authority,
single-use, and consumed in the same call that reaches one integrated
operation.

A read-only projection may expose orientation such as disposition, blocker
codes, and next wake posture. That projection is never authority.

Registering a yield or wait and consuming a resume directive must use explicit
durable compare-and-set operations at the expected cursor. The first backend
contract should atomically:

- register one exact yield or wait and its event projection; and
- consume one exact directive and append the corresponding resume projection.

Unsupported backends fail closed. A crash between claim and consumer outcome
remains ambiguous until the continuity attempt/outcome lifecycle records it.

## 13. Delegated Approval Capability

Standing user delegation should be represented through existing scoped
capability foundations, not inferred from conversation. The future capability
must bind:

- principal and optional delegated actor;
- gate/action classes eligible for delegated decision;
- workflow, run, repository, resource, and time scope;
- maximum sensitivity and proportional-governance posture;
- evidence/check prerequisites;
- prohibited self-approval or separation-of-duty combinations;
- issuance, expiry, revocation, and provenance references.

Even when the capability is current, an approval may be granted only after the
gate is ready. Human-only, quorum, independent-review, requester/approver
separation, or steward-minimum requirements remain blocking. Delegation cannot
convert missing evidence into satisfied evidence.

General delegated approval is deferred from the first continuity slice. The
current capability model does not yet prove a complete parent-grant derivation
chain. Before delegated approval becomes operational, Core must prove parent
identity, strict scope attenuation, remaining delegation depth, expiry,
revocation cascade, prerequisite preservation, and cycle rejection at use
time. The first slice may consume only an already-supported direct authority
path whose current source is independently resolved.

## 14. Kernel And Host Supervisor Contract

Workflow OS Core should own:

- durable run, window, yield, wait, and resume-decision state;
- gate readiness and approval legality;
- fresh authority and required-context resolution;
- exact one-time operation claims;
- deterministic events, errors, and report posture.

The host supervisor should own:

- observing durable `ResumeNow` posture;
- selecting and starting an eligible executor;
- passing the bounded orientation context;
- reporting executor yield or failure back to Core;
- retrying supervisor delivery according to explicit policy;
- never translating an agent final response into workflow completion.

The host must not choose legality, fabricate approval, weaken blockers, or
reuse a stale directive. If no supervisor integration exists, the kernel can
durably expose `resume_required`; it cannot physically schedule another model
turn. Documentation and tests must state this limitation plainly.

## 15. Failure And Recovery Semantics

- Gate readiness source failure: fail closed before approval presentation.
- Approval against stale readiness: reject with a stable non-leaking code.
- Window expiry/revocation: close or block before another operation.
- Duplicate resume claim: one first writer; later attempts fail closed.
- Non-atomic backend: reject continuity registration or resume before changing
  run state.
- Cursor advance before consumer entry: burn the old claim and require fresh
  reassessment.
- Executor disappears after yield: run remains non-terminal and supervisor may
  assign another eligible executor.
- Host cannot schedule: preserve `resume_required` plus bounded delivery error;
  do not fail the workflow unless explicit policy says to do so.
- Wait wake-up races: compare exact condition version and event cursor.
- Crash after claim but before outcome: preserve conservative may-have-started
  posture; record typed attempt/outcome lifecycle before claiming restart-safe
  continuity.

## 16. Privacy And Redaction

Continuity records may store stable IDs, hashes, enumerated posture, timestamps,
bounded reason codes, and references. They must not store:

- prompts or model transcripts;
- chain-of-thought or hidden reasoning;
- raw source, spec, parser, provider, CI, command, or tool payloads;
- environment values, credentials, authorization headers, private keys, or
  token-like values;
- arbitrary free-form wait, yield, or approval descriptions;
- raw capability or evidence contents.

Debug, Display, serialization, deserialization, validation, and host-delivery
errors must be non-leaking. Sensitivity and redaction metadata should default
conservatively.

## 17. Candidate Implementation Sequence

1. Add core model types and pure validation for non-authoritative actionable
   gate assessment, execution windows, yields, waits, resume dispositions, and
   continuity attempt/outcome posture. **Implemented model-only.**
2. Perform focused maintainer/security review before runtime integration.
   **Completed; phase accepted.**
3. Add an atomic durable-state contract for register-yield-at-cursor and
   consume-directive-at-cursor, plus in-memory/backend conformance tests.
4. Add event vocabulary and derived projection for one local execution window,
   one executor yield, one typed wait condition, and bounded continuation
   attempt/outcome. Preserve existing `WorkflowRunStatus` wire variants in the
   first slice.
5. Implement one explicit one-shot local supervisor/helper vertical slice that:
   - opens a window after a ready and valid grant;
   - invokes one source-backed exact operation;
   - records turn-boundary yield when work remains;
   - rehydrates and emits a fresh `ResumeNow` directive;
   - resumes once through an injected test executor;
   - records attempt start and bounded success, failure, or ambiguous outcome;
   - rehydrates once after the executor turn and returns the next kernel
     disposition;
   - completes only after a real terminal event.
6. Add restart, duplicate, stale-cursor, expiry, revocation, and supervisor
   delivery tests.
7. Review the end-to-end local proof.
8. Only after acceptance, consider CLI orientation, additional wait kinds,
   hosted supervisor integration, or broader capability use.

Implementation must begin with model types, but this P0 lane should not stop at
models. The first accepted milestone is one end-to-end local continuity proof.

## 18. Test Plan

Future tests should prove:

- an unmet evidence obligation produces `PendingPrerequisites`, not an
  actionable approval;
- approval cannot satisfy its own evidence or check prerequisite;
- ready gate plus eligible decision can open one exact execution window;
- human-only and separation-of-duty gates reject delegated self-approval;
- execution window validation rejects invalid scope, time, cursor, actor,
  immutable binding, and authority references;
- yield preserves a non-terminal run and does not create approval wait;
- final-response metadata cannot create a terminal event;
- typed waits preserve exact wake posture across restart;
- unsupported non-atomic backends fail closed;
- concurrent directive consumers produce exactly one durable winner;
- satisfying a wait condition requires fresh reassessment;
- `ResumeNow` is derived only from current state and current authority;
- duplicate resume consumers produce one winner;
- cursor advance, expiry, revocation, policy escalation, capability loss,
  evidence staleness, or check failure blocks before consumer entry;
- supervisor scheduling failure preserves inspectable non-terminal state;
- crashes before and after yield registration, directive claim, resume event,
  continuation claim, consumer start, and consumer outcome preserve explicit
  deterministic or ambiguous posture;
- one local injected supervisor resumes yielded lawful work without manual run
  restart;
- only a real terminal event makes the run terminal;
- events rehydrate deterministically;
- Debug and serde paths do not leak forbidden values;
- existing approval, continuation, capability, SideEffect, runtime, report,
  adapter, and hosted tests remain unchanged.

## 19. Open Questions

- Should the first event-integrated slice add new `WorkflowRunStatus` variants,
  or retain `Running` plus explicit continuity projection until compatibility
  requirements are proven?
- Is an execution window bounded primarily by time, operation count, exact
  phase, exact handoff, or a required combination?
- Which existing approval paths can produce a ready-gate assessment without
  parallel policy semantics?
- Should host-delivery failures remain workflow-neutral indefinitely or become
  escalations after a policy-defined budget?
- How should a replaced executor prove acceptance of the same typed handoff?
- Which wait condition should be implemented first: human decision, evidence,
  or external event?
- When should executor yield and resume appear in WorkReport sections?
- Which delegated approval capability classes are safe for local dogfood before
  enterprise stewardship exists?
- What minimum parent-grant derivation model is required before any delegated
  approval capability can be operational?

## 20. Final Recommendation

Proceed next with the **Authorized Execution Continuity core decision model
only**, followed immediately by focused maintainer/security review and one
local injected-supervisor vertical slice.

Do not broaden provider mutations, nested harness runtime, public scheduling,
workflow schemas, CLI execution automation, or delegated approval defaults
until the local proof shows that a lawful non-terminal run survives executor
yield, resumes from fresh authority, and terminates only through kernel state.
