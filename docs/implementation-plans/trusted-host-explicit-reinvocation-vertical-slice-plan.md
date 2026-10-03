# Trusted-Host Explicit Reinvocation Vertical Slice Plan

Implementation status: the first crate-private local SQLite slice is
implemented and documented in the [implementation
report](../concepts/TRUSTED_HOST_EXPLICIT_REINVOCATION_VERTICAL_SLICE_REPORT.md).
It remains private and explicit; no scheduler or automatic reinvocation was
added.

## 1. Executive Summary

Workflow OS can now register one exact `TimeWindow` wait, project an inert
opaque wait handoff from one coherent authoritative SQLite snapshot, satisfy
the exact wait through the accepted trusted-time verifier, and enter one local
operation through the accepted trusted-host operational entry helper.

Those boundaries are intentionally separate. The missing P0 proof is one
private, explicit composition that starts from a previously returned opaque
handoff, freshly revalidates authoritative state, performs the source-specific
wait transition, and then calls the existing operational entry helper. The
handoff remains orientation only; it never becomes authority.

This plan authorizes planning only. It does not implement a scheduler,
polling, background work, model-turn creation, automatic approval, provider or
sandbox execution, nested harnesses, public configuration, CLI, SDK, schema,
hosted, distributed, write, or release behavior.

## 2. Current Foundation

The first vertical slice may reuse these accepted private boundaries:

- exact authorized executor yield with zero or one bounded `TimeWindow` wait;
- SQLite schema v6 exact wait-dependency binding;
- trusted-time observation and exact wait transition;
- replay recovery for an ambiguously committed transition;
- coherent read-only opaque handoff observation;
- bounded trusted-host redispatch for immediately runnable work; and
- `enter_trusted_host_operation` for fresh opening or exact resume entry.

The opaque handoff exposes bounded point-in-time correlation only. It cannot
satisfy a wait, construct a capability, consume a directive, reserve dispatch,
or invoke an executor.

## 3. Problem Statement

Today a trusted host can observe `AwaitCondition`, retain an inert handoff, and
later call the exact `TimeWindow` transition boundary. It cannot yet perform a
single reviewed composition that proves all of the following:

1. the supplied handoff still identifies the same authoritative wait posture;
2. the exact source-specific wake condition is lawfully satisfied;
3. the transition is committed or recovered by exact replay;
4. current authoritative state is reloaded after that transition;
5. a fresh resume directive and dispatch reservation are obtained only through
   the accepted operational entry helper; and
6. only one competing caller can enter the executor.

Without that composition, a host must manually join security-sensitive
private operations. That leaves the false-stall fix incomplete and risks
turning an inert handoff into implied authority.

## 4. Goals

- Add one crate-private local SQLite composition for an already-registered
  exact `TimeWindow` wait.
- Accept an opaque handoff only as stale-detection and correlation input.
- Reconstruct every mutable expectation from current authoritative state.
- Use the accepted trusted-time verifier as the only wait-satisfaction path.
- Re-enter only through `enter_trusted_host_operation`.
- Preserve exact replay across process restart and commit ambiguity.
- Preserve one-winner directive consumption and dispatch reservation.
- Return bounded typed posture rather than fabricate progress.
- Keep errors, Debug, and test output non-leaking.

## 5. Non-Goals

- No scheduler, polling loop, timer service, queue, daemon, or background task.
- No automatic model or agent reinvocation.
- No agent self-approval or authority inference from delegated prose.
- No approval, evidence, check, external-event, capability, conflict, or
  authority-refresh wake source.
- No new executor, handler, or workflow semantics.
- No provider execution, OpenShell integration, sandbox execution, nested
  harnesses, or provider writes.
- No public Rust API, CLI, SDK, workflow schema, runtime configuration, UI,
  example, persistence backend, hosted, distributed, or release change.
- No conversion of a handoff into a bearer capability.

## 6. Candidate Private API

The implementation should add the smallest crate-private helper adjacent to
the existing trusted-host caller, likely shaped around:

- `TrustedHostTimeWindowReinvocationInput`;
- `TrustedHostTimeWindowReinvocationOutcome`; and
- `reinvoke_after_time_window_wait`.

Names may change to match repository conventions. The input should contain
only immutable or opaque caller-held values already required by accepted
boundaries:

- the previously returned opaque wait handoff;
- the exact private wait locator and operation identity needed by the accepted
  `TimeWindow` transition caller;
- the immutable invocation and executor bindings required by operational
  entry; and
- injected accepted current-authority, required-context, trusted-time, and
  executor dependencies.

The caller must not provide current revision, cursor, generation, wait state,
trusted-time observation, resume directive, dispatch capability, or authority
result. Core derives those values freshly.

The implementation must split or wrap operational entry so the existing-window
path does not require caller-supplied opening context or opening-persistence
values. Those values apply only when no window exists and are irrelevant to a
reinvocation of an already-registered wait. The reinvocation helper must not
accept placeholders for them.

The outcome should be one of the accepted operational entry outcomes:

- executor result;
- `AwaitCondition` with a newly derived handoff when another supported wait
  remains;
- `Blocked`; or
- `Terminal`.

It should also expose only bounded transition/replay posture needed for tests
and diagnosis. It must not expose private capabilities or raw authoritative
records.

## 7. Opaque Handoff Validation

The helper must begin with a fresh coherent read. It should derive the current
opaque handoff projection and compare its full private identity to the supplied
handoff before any transition.

A missing, changed, unsupported, blocked, terminal, satisfied, corrupt, or
otherwise non-`AwaitCondition` posture must fail closed or return the freshly
classified non-actionable posture. It must not transition based only on a
matching condition ID or a caller-supplied deadline.

The handoff comparison is stale detection, not authorization. Even an exact
match does not prove the deadline has elapsed and does not authorize entry.

## 8. Source-Specific Wake Transition

After handoff validation, the helper should invoke the existing exact
`TimeWindow` caller. That caller remains responsible for:

- obtaining trusted time inside the accepted boundary;
- checking exact dependency binding;
- enforcing deadline and trusted-time epoch rules;
- performing the atomic wait transition;
- recovering exact operation replay after ambiguous commit; and
- returning bounded transition status plus current disposition.

The reinvocation helper must not require a fresh generic `AwaitCondition`
classification after the deadline has elapsed. An unsatisfied elapsed
`TimeWindow` may conservatively classify as blocked before its source-specific
transition. The source-specific verifier, not a generic classifier or the
handoff, decides whether the exact deadline transition is lawful.

## 9. Fresh Operational Entry

After a committed or exactly replayed successful transition, the helper must
call `enter_trusted_host_operation` with the same immutable invocation and
executor bindings. It must not reconstruct resume authority or directly call
the executor.

Operational entry must freshly:

1. load authoritative continuation state;
2. classify current posture;
3. obtain a one-use resume directive when lawful;
4. consume that directive and reserve dispatch atomically;
5. reassess registered current authority and required context; and
6. invoke the existing bounded redispatch path.

If another wait remains, current authority blocks, the run becomes terminal,
or exact bindings no longer match, the helper returns that bounded posture and
does not invoke the executor.

## 10. Restart, Replay, And Ambiguity

The helper must support a fresh process reopening the existing SQLite state.
An exact retry after uncertain wait-transition commit must recover the durable
transition through the accepted operation replay boundary and continue to a
fresh operational entry.

An uncertain executor attempt is not silently retryable. Existing durable
attempt and outcome semantics remain authoritative. The new helper must not
invent an outcome, reuse consumed authority, or treat a missing response as a
failed attempt.

Focused review determined that the existing durable operation record is not
sufficient to bind exact replay to the supplied handoff. The committed request
binds the exact transition facts, but its durable replay projection does not
persist the opaque handoff identity. After a successful transition, the old
handoff cannot be freshly re-derived because the wait is already satisfied.

The implementation must therefore derive a payload-free handoff commitment
inside Core and bind it into both the transition request commitment and the
durable private request envelope. An exact replay must compare the supplied
handoff commitment to that durable value before treating the operation as the
same reinvocation. This is an internal replay-envelope change, not public serde
or a new authoritative handoff record. Direct wake callers that do not claim
handoff-based reinvocation must remain distinguishable and cannot be promoted
to this composition by omission or a placeholder value. Caller memory is
insufficient.

## 11. Concurrency And Idempotency

Two callers may observe the same inert handoff. Correct behavior is:

- at most one exact wait transition wins, while the other receives exact
  replay or current-state rejection;
- at most one resume directive consumption and dispatch reservation wins;
- at most one executor attempt enters for the exact operation;
- no loser reconstructs authority or invokes the executor; and
- all outcomes are explainable from durable state after restart.

The helper must preserve existing operation identities and replay semantics.
It must not add an in-memory lock as the correctness boundary.

## 12. Failure And Error Posture

- Stale handoff, binding substitution, or ownership mismatch is a stable
  security or invalid-state failure and returns no current disposition.
- Trusted-time unavailability or quarantine cannot fabricate satisfaction.
- Commit ambiguity is reconciled through the accepted fresh-connection replay
  path.
- Blocked or terminal posture never enters the executor.
- A coherent ordinary posture change discovered before mutation may return a
  newly classified bounded outcome. A security rejection must not include a
  second disposition that could become a state oracle.
- Another unsatisfied wait returns `AwaitCondition` with a newly projected
  handoff rather than reusing the old one.
- No failure becomes workflow completion, approval, or a fake external wait.

Errors must use stable codes and must not echo handoff IDs, locators,
deadlines, paths, timestamps, prompts, commands, payloads, credentials,
authority values, or secret-like test material.

## 13. Privacy And Redaction

The new input and outcome should use custom bounded Debug behavior and no
public serialization. They must not expose:

- exact deadline or trusted-time observations;
- cursor, source, provenance, epoch, or dependency commitments;
- capability, directive, receipt, or dispatch internals;
- immutable skill input or executor output;
- approval rationale, evidence bodies, check output, logs, provider payloads,
  source contents, environment values, credentials, tokens, or keys.

The helper should persist no new raw payload. Any added replay binding must be
domain-separated and payload-free.

## 14. Test Plan

Focused future tests must prove:

1. an exact unelapsed `TimeWindow` remains waiting without executor entry;
2. an exact elapsed wait transitions and enters one operation;
3. the handoff is validated before transition;
4. stale handoff identity fails before mutation;
5. revision, cursor, generation, condition, and immutable-binding substitution
   fail closed;
6. trusted-time source failure cannot satisfy the wait;
7. exact transition replay after backend reopen proceeds safely;
8. crash after transition but before entry resumes from authoritative state;
9. crash after directive consumption follows existing ambiguous-attempt rules;
10. two concurrent callers produce at most one executor entry;
11. a remaining wait returns a newly derived handoff;
12. blocked and terminal posture never enters the executor;
13. current authority and required context are reassessed at entry;
14. no handoff converts into authority or a wake capability;
15. no new write occurs before the accepted transition boundary;
16. Debug and errors omit IDs and secret-like values;
17. no public API, serde, CLI, schema, scheduler, or provider behavior appears;
18. existing continuity, SQLite, wait, handoff, operational-entry, redispatch,
   runtime, adapter, and report tests remain green;
19. `cargo fmt --all --check` passes;
20. `cargo clippy --workspace --all-targets -- -D warnings` passes;
21. `cargo test --workspace` passes; and
22. documentation checks pass.

## 15. Proposed Implementation Sequence

1. Focused maintainer/security review of this plan.
2. Resolve the replay-to-handoff binding question from durable records.
3. Add the crate-private input, outcome, and composition helper.
4. Reuse coherent handoff observation, exact `TimeWindow` transition, and
   operational entry without duplicating their validation logic.
5. Add restart, exact replay, race, stale-input, and privacy tests.
6. Create an end-of-phase implementation report.
7. Perform focused maintainer/security review.
8. Only after acceptance, consider a host scheduling boundary.

## 16. Open Questions For Review

Focused review resolves the planning questions:

1. A payload-free handoff commitment is required in the transition request
   commitment and durable private replay envelope.
2. The existing-window operational-entry input must be split or wrapped so
   reinvocation cannot carry irrelevant opening or opening-persistence values.
3. Security rejection returns only a stable non-leaking error. A coherent
   ordinary posture change may return a separately derived bounded outcome,
   but errors do not carry a disposition oracle.

None of these decisions may be implemented by trusting caller memory or
widening the public surface.

## 17. Final Recommendation

After focused maintainer/security review, implement one crate-private local
SQLite `TimeWindow` transition-and-entry helper. It should compose accepted
boundaries, not create another supervisor abstraction.

Do not add scheduling, polling, automatic agent reinvocation, provider or
sandbox execution, OpenShell, nested harnesses, public APIs, schemas, CLI,
SDK, hosted behavior, writes, or release claims.

## 18. Governed Planning Record

- workflow: `dg/d`
- run: `run-1791066437971491000-2`
- approval: `approval/run-1791066437971491000-2/planning-approved`
- presentation: `presentation/df830794ac71a630`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: documentation-only explicit reinvocation planning
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation enforcement: proof enforced with one persisted
  presentation record and event marker
- validation summary: `npm run check:docs` and `git diff --check` passed
- out-of-kernel work: architecture inspection, plan authoring, documentation
  edits, validation commands, and later git and pull-request work
- missing coverage: the kernel coordinates governance only; it does not edit
  files, execute validation, implement reinvocation, create a WorkReport
  artifact, or perform git actions
