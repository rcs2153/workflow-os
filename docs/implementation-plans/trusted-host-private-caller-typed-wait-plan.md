# Private Trusted-Host Caller And Typed-Wait Boundary Plan

## Implementation Status

The first prerequisite slice is implemented: authoritative waits can carry an
exact private `TimeWindow` dependency binding, SQLite schema v6 persists that
binding without inventing values for legacy rows, and a crate-private verifier
obtains trusted time and transitions only the exactly bound wait. Legacy
unbound deadline waits fail closed.

The first private trusted-host caller is now implemented for one existing exact
`TimeWindow` wait. It derives mutable expectations from authoritative SQLite
state, invokes the accepted verifier, recovers exact operation replay across a
reopened backend, and returns bounded transition status plus current
continuation posture. It does not treat satisfaction as execution authority.

The supervisor typed-wait registration path, opaque wait handoff, scheduling,
polling, executor recirculation, and other wake-source families remain
unimplemented. This document continues to govern their sequencing; the
implemented slices are recorded in
[`TRUSTED_HOST_TIME_WINDOW_WAIT_BINDING_REPORT.md`](../concepts/TRUSTED_HOST_TIME_WINDOW_WAIT_BINDING_REPORT.md).
The caller integration is recorded in
[`TRUSTED_HOST_TIME_WINDOW_CALLER_IMPLEMENTATION_REPORT.md`](../concepts/TRUSTED_HOST_TIME_WINDOW_CALLER_IMPLEMENTATION_REPORT.md).
Its focused maintainer/security review accepted the private boundary with
non-blocking test follow-ups in
[`TRUSTED_HOST_TIME_WINDOW_CALLER_REVIEW.md`](../concepts/TRUSTED_HOST_TIME_WINDOW_CALLER_REVIEW.md).

Status: accepted after focused maintainer/security review with in-review
corrections and non-blocking follow-ups. The private local SQLite operational
entry boundary and first exact `TimeWindow` caller are implemented and
accepted. This plan defines the remaining genuine wait-registration,
wait-handoff, and explicit reinvocation boundaries. It does not authorize a
scheduler or public runtime behavior. The planning review is recorded in
[Private Trusted-Host Caller And Typed-Wait Plan
Review](../concepts/TRUSTED_HOST_PRIVATE_CALLER_TYPED_WAIT_PLAN_REVIEW.md).

## 1. Executive Summary

Workflow OS can now classify and enter one authorized local operation from a
fresh process or restart. It can open with current authority, resume through a
fresh directive, or return wait, blocked, and terminal posture without
reconstructing capability.

The next integration cannot safely be a scheduler or a generic loop around
that helper. The current trusted-host supervisor registers yielded attempts
with an empty wait list. The private durable wait record also binds a condition
and wake-trigger class but not the exact dependency reference expected to
satisfy it. A caller therefore cannot yet prove “this exact approval,
evidence, check, external event, capability change, deadline, authority
revision, or conflict resolution satisfied this wait.”

The safe sequence is:

1. strengthen authoritative wait dependency binding;
2. let an injected executor return bounded typed wait declarations that Core
   binds and registers atomically with yield;
3. expose an opaque, non-authoritative wait handoff;
4. satisfy a wait only through a source-specific same-call verifier;
5. explicitly reinvoke the accepted operational entry helper; and
6. add one private caller that coordinates those operations without owning
   authority, polling, or execution policy.

## 2. Current Accepted Foundation

The future boundary may compose, but must not weaken:

- immutable run-bundle and exact workflow/run/step binding;
- registered current-authority resolution;
- atomic opening and attempt-one start;
- one-use, non-serializable opening and resume capabilities;
- exact invocation and executor commitments;
- one-winner dispatch reservation;
- the accepted one-shot supervisor and bounded redispatch loop;
- authoritative continuation disposition;
- atomic yield, wait-transition, and directive-consumption operations;
- payload-free continuity receipts and event projections; and
- the accepted private operational entry helper as the only admission path.

## 3. Problem Statement

Three integration gaps remain.

First, `TrustedHostAttemptExecutionResult::Yielded` carries only a reason. The
supervisor always persists `waits: Vec::new()`. Ordinary turn boundaries can
therefore continue, but no trusted-host execution path can register a genuine
dependency wait.

Second, `AuthoritativeWaitRecord` and the SQLite wait table retain condition
identity, generation, trigger, state, source commitment after transition, and
revision. They do not retain the exact expected dependency reference before
transition. A source of the correct trigger class but the wrong resource could
otherwise satisfy the wait.

Third, `AwaitCondition` is currently only a disposition. No bounded opaque
handoff tells a private host which exact current waits to preserve, and no
accepted caller coordinates verified transition followed by explicit
reinvocation.

## 4. Goals

- Preserve the operational entry helper as the sole execution admission path.
- Register zero or more genuine typed waits atomically with one yielded
  attempt.
- Bind every wait to one exact dependency identity or commitment.
- Derive workflow/run/step/window/attempt/generation/cursor fields inside Core.
- Return a bounded non-authoritative wait handoff for host orientation.
- Verify wake facts through source-specific same-call Core boundaries.
- Persist only bounded references and commitments, never dependency payloads.
- Reinvoke explicitly after an authoritative state change.
- Remain deterministic and one-winner under replay and competing callers.
- Keep ordinary no-wait turn boundaries eligible for immediate bounded
  redispatch.

## 5. Non-Goals

This plan does not authorize:

- implementation during this planning phase;
- a scheduler, daemon, worker pool, queue, poller, timer service, or wake bus;
- automatic model-turn creation or conversational recirculation;
- automatic approval or authority inferred from an agent identity;
- provider execution, provider mutation, OpenShell, or another sandbox;
- nested harness execution, recursive agents, or agent swarms;
- public runtime configuration, workflow fields, CLI, SDK, UI, or examples;
- filesystem, PostgreSQL, multi-host, lease, heartbeat, or failover parity;
- capability persistence, serialization, cloning, or reconstruction;
- raw approval, evidence, check, provider, command, prompt, or source payloads;
- hosted, distributed, production, or release-readiness claims; or
- enterprise identity, RBAC, or administration.

## 6. Core Invariants

1. Only Core may derive current continuation posture.
2. Only the accepted operational entry helper may admit executor entry.
3. A wait declaration is a bounded dependency request, not authority.
4. Core derives all run, window, attempt, generation, and cursor bindings.
5. A wait binds one exact dependency reference and one compatible wake class.
6. Wake satisfaction requires a fresh source-specific verifier in the same
   call that creates the private wake capability.
7. A persisted source commitment proves the source used; it is not reusable
   authority.
8. Wait handoffs are orientation and cannot satisfy or resume work.
9. Reinvocation always rehydrates current state and consumes fresh authority.
10. A host may deliver a wake fact or call Core; it may not choose progress.
11. Competing callers admit at most one transition and one resumed attempt.
12. Unknown, stale, mismatched, unsupported, or ambiguous state fails closed.

## 7. Candidate Private Model

The first implementation should consider only crate-private types:

- `TrustedHostWaitDeclaration`
  - condition ID and version;
  - typed dependency kind;
  - bounded required dependency reference or commitment;
  - compatible wake-trigger class;
  - optional deadline when the kind is `TimeWindow`.
- `TrustedHostYieldRequest`
  - existing yield reason;
  - bounded ordered wait declarations.
- `TrustedHostWaitHandoff`
  - opaque handoff ID or commitment;
  - window and generation identity;
  - current window revision or cursor commitment;
  - bounded current condition identities and versions;
  - closed next operation: `await_authoritative_change`.
- `TrustedHostWakeObservationInput`
  - exact condition identity and version;
  - source-specific verifier input;
  - fresh operation and receipt identities.
- `TrustedHostOperationalCallOutcome`
  - completed entry outcome;
  - opaque wait handoff;
  - blocked posture; or
  - execution-window terminal posture.

Names are provisional. No type should be public in the first slice.

Executor declarations are untrusted liveness requests produced by the exact
authorized invocation. They cannot grant authority, satisfy a policy or
approval gate, weaken current governance, or bypass fresh reassessment. Core
must reject any declaration unsupported by the first accepted source-specific
contract. In the first slice, only `TimeWindow` is supported.

## 8. Authoritative Wait Dependency Binding

Before caller integration, the private authoritative wait state must retain
enough information to reject a wrong dependency of the same trigger class.
The minimum durable binding is:

- condition ID and version;
- owning window and yield generation;
- typed condition kind;
- exact required dependency reference commitment;
- deterministic wake-trigger class;
- state and revision; and
- optional deadline commitment for a time wait.

The stored reference must remain bounded and non-secret. If a useful external
reference could expose sensitive text or path data, persist only a
domain-separated commitment and a safe source category.

Kind-to-trigger compatibility must be deterministic:

- human decision -> approval decision recorded;
- evidence required -> evidence accepted;
- check required -> check accepted;
- external event -> external event recorded;
- capability unavailable -> capability availability changed;
- time window -> deadline reached;
- authority refresh -> authority source changed; and
- conflict resolution -> conflict resolved.

A future implementation requires a new explicit SQLite continuity schema
version covering both relational columns and canonical `record_json`; an
additive side table would make cross-representation integrity harder to prove.
It must not silently reinterpret existing rows. Existing rows without exact
dependency binding remain unsupported for automatic satisfaction and fail
closed.

## 9. Yield And Wait Registration

The executor may return `Succeeded`, retryable or terminal failure, ambiguity,
or a bounded `TrustedHostYieldRequest`. For a yield:

1. validate count, uniqueness, identifier bounds, condition versions, kind,
   trigger compatibility, reference safety, and deadline shape;
2. derive workflow/run/step/window/attempt/generation/cursor binding from the
   live private attempt capability;
3. build canonical private wait seeds;
4. include every semantic field in the register-yield request commitment;
5. atomically record attempt yield, generation, waits, window posture,
   receipt, and event projection; and
6. derive continuation posture from committed state.

The executor cannot provide actor, run, window, attempt, cursor, revision,
authority, governance, or immutable-bundle facts. An empty wait list remains
valid for an ordinary executor turn boundary and must not be converted into a
fake external wait.

## 10. Wait Handoff

When Core derives `AwaitCondition`, the caller should return an opaque bounded
handoff built from authoritative records. It may identify current condition
IDs, versions, safe dependency categories, and the owning revision or cursor
commitment. It must not expose raw dependency values, approval reasons,
evidence bodies, check output, provider payloads, prompts, commands, source
contents, credentials, or authority values.

The handoff cannot be submitted as proof, used as a wake capability, or
translated into approval. Its only allowed next operation is to await an
authoritative change and later request fresh classification.

## 11. Wake Assessment And Transition

Satisfaction requires a source-specific verifier. In one Core-owned call, the
verifier must:

1. load the current wait and exact expected dependency binding;
2. load or evaluate the current authoritative source;
3. verify condition kind, trigger, source identity, source revision, and
   accepted posture;
4. construct a private one-use `WakeAssessmentCapability`;
5. transition the exact wait with compare-and-set semantics; and
6. persist only source commitment and revision.

The first implementation supports `TimeWindow` only. Core binds the exact
deadline plus trusted-time source, provenance, and epoch commitment when the
wait is registered. The deadline must be later than the registering
observation and no later than the execution-window expiry. In one same-call
verification, Core obtains a fresh trusted-time observation, validates source,
provenance, epoch, monotonicity, and `observed_at >= deadline`, then constructs
the private wake capability and transitions the wait. A caller-supplied clock,
timestamp assertion, boolean, or wake capability is prohibited.

Approval, evidence, check, external-event, capability, authority-refresh, and
conflict sources remain unsupported until each has a source-specific plan and
verifier. Executor yield must not create or replace an approval gate.

Expiration, supersession, and cancellation remain explicit Core transitions
and do not require a satisfaction capability. They must still validate current
window, generation, condition, cursor, and revision.

## 12. Explicit Reinvocation

After a transition, the caller explicitly invokes the accepted operational
entry helper again with the exact immutable invocation and executor binding.
The entry helper rehydrates posture:

- remaining unsatisfied waits -> return a new wait handoff;
- all waits satisfied and current authority available -> consume one fresh
  directive and enter the bounded redispatch loop;
- blocked or recovery-required -> return blocked;
- closed, expired, revoked, or superseded window -> return terminal posture;
- stale or mismatched input -> fail closed.

The transition operation must not call the executor itself. This separation
keeps wake verification and execution admission independently reviewable.

## 13. Smallest Private Caller

The first caller should be one synchronous crate-private local SQLite helper.
It receives explicit injected dependencies and performs one requested action:

- `Enter`: call `enter_trusted_host_operation` and map its bounded outcome;
- `ObserveWait`: return the current opaque handoff without mutation; or
- `ApplyVerifiedWake`: call one accepted source-specific transition boundary
  and return the resulting current handoff posture.

It must not loop, sleep, poll, schedule, open network connections, create model
turns, choose a weaker governance path, or own durable truth. A later host can
call it after receiving a real event. That host remains transport and timing,
not governance.

## 14. Replay, Concurrency, And Failure Posture

- Exact operation replay returns the recorded bounded result and no new
  capability.
- Reusing an operation ID with changed content fails.
- Two yield registrations admit one exact operation.
- Two wake transitions admit one exact current revision.
- Two reinvocations compete through directive consumption and dispatch
  reservation; one executor entry wins.
- Crash after wait transition but before reinvocation leaves truthful durable
  state and requires only fresh re-entry.
- Crash after opening or resume commit never authorizes capability recovery
  from a receipt.
- Unknown legacy wait rows cannot be auto-satisfied.
- Source unavailability leaves the wait unsatisfied and returns a stable
  non-leaking error or bounded unavailable posture.

## 15. Privacy And Redaction

The boundary may persist only bounded IDs, categories, commitments, revisions,
cursors, timestamps, lifecycle posture, receipts, and event projections.

It must not persist or echo:

- raw prompts, conversation history, or model reasoning;
- approval rationale or presentation text;
- evidence, check, source, command, or provider payloads;
- file contents or unbounded paths;
- environment values, credentials, tokens, authorization headers, or keys;
- executor input or output; or
- capability internals.

Debug output, serialization, deserialization, conflict errors, and source
verification errors must remain bounded and redaction-safe.

## 16. Proposed Implementation Sequence

1. Focused maintainer/security review of this plan.
2. Authoritative wait dependency-binding model and SQLite schema update for
   `TimeWindow` only.
3. Dependency-binding and trusted-time verifier implementation with
   conformance tests.
4. Direct successful fresh-opening composition test for the accepted entry
   helper.
5. Private typed wait declaration and atomic yield-registration integration.
6. Focused review.
7. One source-specific local wake verifier and wait-transition composition.
8. Focused review.
9. Opaque wait handoff and one synchronous private caller.
10. End-to-end restart, wake, reinvocation, concurrency, and privacy tests.
11. Maintainer/security review before any scheduler, provider, sandbox,
    nested-harness, or public runtime integration.

Implementation should not combine all stages into one unreviewed change.

## 17. Test Plan

Future tests should prove:

1. successful fresh entry through the operational entry helper;
2. no-wait yield remains eligible for fresh authorization;
3. a valid typed wait is registered atomically with yield;
4. duplicate or incompatible waits fail before persistence;
5. wrong dependency of the correct trigger class cannot satisfy a wait;
6. wrong trigger, condition, generation, window, revision, or source fails;
7. source unavailability does not fabricate satisfaction;
8. satisfaction persists only bounded commitment and revision;
9. one of two competing wake transitions wins;
10. a crash after transition can restart and rehydrate truthfully;
11. remaining waits preserve `AwaitCondition`;
12. all satisfied waits permit only fresh directive consumption;
13. one of two competing reinvocations enters the executor;
14. handoff values cannot authorize transition or execution;
15. legacy unbound waits fail closed;
16. blocked and terminal posture never enters the executor;
17. errors and Debug omit secret-like dependency and invocation values;
18. serialization does not expose raw source material;
19. complete continuity, SQLite, opening, supervisor, redispatch, runtime,
    adapter, and report suites remain green; and
20. repository documentation remains honest about the private boundary.

## 18. Open Questions

- Should the durable wait retain a safe bounded reference plus commitment, or
  commitment and source category only?
- What is the smallest opaque handoff identity that supports diagnosis without
  exposing dependency references?
- Should cancellation and supersession share one private transition helper or
  remain separate exact operations?

## 19. Final Recommendation

Proceed next to exact authoritative wait dependency binding and the private
`TimeWindow` verifier only. Use a new reviewed SQLite continuity schema
version, preserve legacy rows as unsupported for automatic wake, and add no
caller integration yet. A caller over trigger-only waits would create the
appearance of governed waiting without proving that the correct dependency
satisfied the condition.

Do not add provider execution, OpenShell, nested harnesses, automatic
approval, public configuration, CLI, SDK, hosted behavior, multi-host
execution, or release claims.
