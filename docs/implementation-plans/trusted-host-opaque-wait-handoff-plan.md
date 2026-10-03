# Trusted Host Opaque Wait Handoff Plan

## 1. Executive Summary

Workflow OS can now register one exact private `TimeWindow` wait atomically
when an authorized trusted-host executor yields. The accepted registration
boundary returns `AwaitCondition`, but it does not yet give a trusted host a
small, explicit, non-authoritative value describing that posture.

This plan defines the next private slice: an opaque wait handoff constructed by
Core from one coherent authoritative SQLite snapshot. The handoff exists only
to orient a trusted host after Core has classified the current continuation
posture as `AwaitCondition`. It is not durable truth, proof of satisfaction,
approval, authority, a wake capability, or permission to invoke an executor.

This plan does not implement anything. It does not authorize polling,
sleeping, scheduling, automatic reinvocation, provider execution, sandbox
execution, nested harnesses, public configuration, CLI, SDK, schema, hosted,
distributed, write, or release behavior.

## 2. Goals

- Define the smallest crate-private handoff that represents an already
  registered exact wait without exposing its dependency values.
- Derive the handoff only from authoritative Core-owned state.
- Keep current continuation disposition authoritative and independently
  reclassified on every later operation.
- Give a trusted host enough bounded identity to correlate an observation with
  the exact window and wait posture it previously received.
- Prevent the handoff from authorizing wait transition, directive consumption,
  executor entry, or authority reconstruction.
- Preserve exact replay, restart safety, privacy, and stable non-leaking errors.
- Prepare one narrow synchronous private caller without creating a scheduler.

## 3. Non-Goals

- No wait satisfaction or wake verification.
- No trusted-time observation by the handoff.
- No polling, sleeping, timers, queues, wake buses, or background workers.
- No automatic executor invocation, conversational recirculation, or agent
  turn management.
- No approval, evidence, check, external-event, capability,
  authority-refresh, or conflict wait sources.
- No authority, capability, directive, receipt, or immutable-run-bundle
  reconstruction.
- No provider, OpenShell, sandbox, or nested-harness execution.
- No workflow schema, public Rust API, CLI, SDK, UI, example, or runtime
  configuration.
- No persistence schema changes or new durable handoff record.
- No hosted, distributed, write, release, or production-readiness claims.

## 4. Existing Boundary

The current private path already provides the security-critical behavior:

1. an exact authorized executor may declare zero or one bounded `TimeWindow`
   wait;
2. Core derives the trusted-time dependency binding;
3. the SQLite continuity transaction revalidates mutable authority, trusted
   time, deadline, cursor, revision, and replay facts;
4. the transaction atomically persists the yield and wait; and
5. Core derives `AuthoritativeContinuationDisposition::AwaitCondition` from
   committed state.

The private `TimeWindow` caller can later apply the accepted source-specific
verifier for an explicitly identified wait. That caller returns bounded
transition status and current disposition, but does not schedule or invoke an
executor.

The missing piece is a bounded value for the host-facing `AwaitCondition`
return path. The handoff must summarize authoritative posture without becoming
a second source of truth.

## 5. Candidate Private Model

The first implementation should add only crate-private types, likely:

- `TrustedHostWaitHandoff`
- `TrustedHostWaitHandoffId`
- `TrustedHostWaitHandoffCondition`
- `TrustedHostWaitHandoffDependencyKind`
- `TrustedHostWaitHandoffNextOperation`

Names may change to match the implementation, but the model should remain
private and minimal.

`TrustedHostWaitHandoff` should contain:

- one opaque handoff ID or commitment derived by Core;
- the exact window ID;
- the authoritative window revision;
- the authoritative continuation cursor or a domain-separated commitment to
  it;
- the active yield generation ID;
- one or more bounded condition descriptors;
- current disposition, which must be `AwaitCondition`; and
- the closed next-operation vocabulary.

Each condition descriptor should contain only:

- condition ID;
- condition version;
- safe dependency category, initially `TimeWindow`; and
- condition state, initially `Unsatisfied`.

The handoff must not contain the exact deadline, trusted-time source,
provenance commitment, epoch, dependency commitment, approval rationale,
evidence content, check output, provider data, prompt, command, credential, or
authority value.

## 6. Handoff Identity

The handoff ID should be a domain-separated commitment over bounded
authoritative identity and posture, such as:

- window ID;
- window revision;
- cursor commitment;
- yield generation ID; and
- sorted condition identities and versions.

It should not commit raw dependency values merely to make them indirectly
guessable. It should not be random durable state requiring another write. The
same coherent snapshot should derive the same handoff ID, and any relevant
authoritative change should derive a different ID.

The handoff ID proves only correlation with one point-in-time projection. It
does not prove that the snapshot is still current and cannot be exchanged for
authority.

## 7. Construction Boundary

Core should construct the handoff from one coherent read transaction or one
already-loaded authoritative snapshot. Construction should:

1. load the exact window and its active wait identities;
2. validate the window, cursor, revision, generation, and wait ownership;
3. classify continuation posture through the existing semantic function;
4. require `AwaitCondition`;
5. reject unsupported, missing, terminal, blocked, satisfied, or corrupt rows;
6. sort condition descriptors deterministically; and
7. derive the opaque handoff ID from the validated bounded projection.

The caller must not assemble the handoff from separately read rows or provide
any authoritative field. No handoff should be returned when the coherent
snapshot no longer classifies as `AwaitCondition`.

## 8. Allowed Next Operation

The first handoff should expose exactly one non-mutating next-operation value:

`RequestFreshClassification`

This means a host may later ask Core to load current authoritative state and
classify it again. It does not mean the host may submit the handoff as proof,
transition a wait, consume a directive, reserve dispatch, invoke an executor,
or create another handoff without a fresh Core read.

The handoff itself should not be accepted by the existing `TimeWindow` wake
caller. That caller must continue to require its exact explicit locator,
condition, operation, and receipt inputs and must independently revalidate
authoritative state.

## 9. Private Caller Integration

The smallest implementation should add an observation-only path adjacent to
the existing private trusted-host caller. It should:

- accept the SQLite backend and exact private locator;
- derive current disposition from authoritative state;
- return `Some(TrustedHostWaitHandoff)` only for `AwaitCondition`;
- return a bounded non-handoff posture for `Blocked` or `Terminal` if the
  accepted caller result needs that distinction; and
- never mutate continuity state.

The first slice should not combine observation with wake transition or
reinvocation. Those operations remain independently reviewable.

## 10. Authority And Security Invariants

- The handoff is not a bearer capability.
- Possession of a handoff grants no permission.
- Handoff equality proves no current state.
- A stale handoff cannot transition a wait or enter an executor.
- Core remains the only continuation classifier.
- Core remains the only constructor of wake-assessment capabilities.
- The accepted SQLite transition remains the only `TimeWindow` satisfaction
  boundary.
- Directive consumption and dispatch reservation remain separate exact
  operations.
- Unsupported or corrupt state fails closed without emitting a plausible
  handoff.

## 11. Replay, Restart, And Concurrency

Because the handoff is a deterministic projection rather than durable state:

- repeating observation of the same snapshot returns the same bounded
  identity;
- reopening the SQLite backend over unchanged state returns the same handoff;
- a changed revision, cursor, generation, or condition set changes the
  handoff identity;
- a wake transition racing with observation yields either the coherent old
  `AwaitCondition` projection or a fresh later posture, never a mixed value;
- stale handoffs remain inert; and
- no operation replay table entry is needed for handoff observation.

## 12. Privacy And Redaction

Debug output should expose only safe enum posture and counts. It should redact
all IDs or show bounded opaque markers unless repository conventions clearly
permit a specific private identifier.

The handoff must never expose or serialize:

- exact deadlines or trusted-time observations;
- source, provenance, epoch, or dependency commitments;
- raw prompts, source contents, commands, logs, or provider payloads;
- approval rationale, evidence bodies, or check output;
- immutable invocation inputs or executor output;
- environment values, credentials, tokens, authorization headers, or keys; or
- authority, capability, directive, or receipt internals.

The first slice should not add public serde. Stable errors must not echo
locator fields, condition values, paths, timestamps, or secret-like data.

## 13. Error Posture

Use stable private error codes consistent with the continuity boundary:

- locator or immutable binding mismatch -> security error;
- stale revision, cursor, generation, or changed posture -> invalid-state
  error;
- missing or internally inconsistent authoritative rows -> corruption-shaped
  invalid-state error; and
- unsupported dependency kind -> validation or invalid-state error that does
  not fabricate a handoff.

Errors should disclose only the failure class. A stale observation should be
resolved by fresh classification, not by weakening validation.

## 14. Test Plan

Future focused tests should prove:

1. one registered unsatisfied `TimeWindow` wait returns one handoff;
2. the handoff is derived only after authoritative `AwaitCondition`
   classification;
3. repeated observation of unchanged state returns the same handoff ID;
4. restart over unchanged SQLite state returns the same handoff ID;
5. changed window revision changes or invalidates the handoff;
6. changed cursor changes or invalidates the handoff;
7. changed generation changes or invalidates the handoff;
8. condition descriptors are deterministic and sorted;
9. satisfied, canceled, expired, superseded, blocked, terminal, or corrupt
   posture cannot produce an `AwaitCondition` handoff;
10. unsupported dependency kinds fail closed;
11. handoff values cannot be passed to the wake transition as authority;
12. observation performs no write and appends no event;
13. observation racing with a transition never produces a mixed snapshot;
14. Debug and errors omit IDs and secret-like values;
15. no public serde or public API is introduced;
16. existing continuity, wait binding, wake verifier, caller, registration,
    supervisor, redispatch, runtime, adapter, and report tests remain green;
17. `cargo fmt --all --check` passes;
18. `cargo clippy --workspace --all-targets -- -D warnings` passes;
19. `cargo test --workspace` passes; and
20. documentation checks remain green.

## 15. Proposed Implementation Sequence

1. Focused maintainer/security review of this plan.
2. Add the crate-private handoff model and deterministic commitment helper.
3. Add coherent SQLite observation and Core construction.
4. Integrate the observation-only path into the smallest private caller.
5. Add restart, race, no-write, and privacy tests.
6. Create an end-of-phase report.
7. Perform focused maintainer/security review.
8. Only after acceptance, plan explicit reinvocation from freshly reclassified
   authoritative state.

Implementation should begin with the private model and coherent observation
only. It should not combine handoff implementation with reinvocation.

## 16. Documentation Updates

The implementation phase should update:

- `ROADMAP.md`;
- `docs/implementation-plans/trusted-host-private-caller-typed-wait-plan.md`;
- this plan;
- the implementation report; and
- the focused review document.

Documentation must continue to say that the handoff is private,
non-authoritative, non-scheduling, and not a public runtime feature.

## 17. Review Decisions

The focused maintainer/security
[review](../concepts/TRUSTED_HOST_OPAQUE_WAIT_HANDOFF_PLAN_REVIEW.md) resolves
the open questions:

- condition IDs and versions may remain private handoff fields but must not be
  exposed by Debug or serialization;
- the handoff carries a cursor commitment, not the cursor;
- blocked and terminal posture return ordinary dispositions without a
  handoff; and
- the handoff ID is a dedicated crate-private newtype backed by a
  domain-separated commitment.

Implementation must construct the handoff inside one explicit SQLite read
transaction and prove old-or-new coherence under a concurrent wait transition.

## 18. Final Recommendation

The focused maintainer/security review accepts the plan. Implement only the
crate-private handoff model and coherent observation path for an
already-registered exact `TimeWindow` wait.

Do not implement wait satisfaction, polling, scheduling, automatic
reinvocation, provider or sandbox execution, nested harnesses, public APIs,
schemas, CLI, SDK, hosted behavior, writes, or release changes in that phase.

## 18.1 Implementation Status

The private model and coherent observation slice is implemented. Core now:

- derives a dedicated opaque handoff ID from bounded authoritative posture;
- commits the cursor rather than exposing it;
- loads, classifies, and projects from one explicit SQLite read transaction;
- returns a handoff only for a supported unsatisfied `TimeWindow` wait whose
  coherent snapshot classifies as `AwaitCondition`;
- returns ordinary non-handoff posture for blocked or resumed state;
- exposes only `RequestFreshClassification` as the next-operation vocabulary;
  and
- performs no write, event append, wake transition, scheduling, or executor
  invocation.

The [implementation report](../concepts/TRUSTED_HOST_OPAQUE_WAIT_HANDOFF_REPORT.md)
records the completed scope and validation. Focused maintainer/security review
is required before any reinvocation planning.

## 19. Governed Planning Record

- workflow: `dg/d`
- run: `run-1791062337349366000-2`
- approval:
  `approval/run-1791062337349366000-2/planning-approved`
- presentation: `presentation/08136db03655cbd5`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- validation summary: `npm run check:docs` and `git diff --check` passed
- out-of-kernel work: architecture inspection, plan authoring, documentation
  edits, validation commands, and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, run documentation checks, create a WorkReport artifact, or perform git
  actions
