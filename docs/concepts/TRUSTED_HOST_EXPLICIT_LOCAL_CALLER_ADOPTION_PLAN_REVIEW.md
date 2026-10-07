# Trusted-Host Explicit Local Caller Adoption Plan Review

## 1. Executive Verdict

**Plan accepted with non-blocking follow-ups. Proceed to one additive
crate-private explicit local operational-entry composition.**

The plan selects the smallest current boundary that has the exact durable
locator, immutable run binding, executor, skill input, synchronous process
ownership, and cancellation ownership needed to use the accepted production
caller without inventing discovery or authority.

## 2. Scope Verification

The plan remains planning-only and does not authorize runtime behavior in this
phase. Its proposed implementation is one additive crate-private SQLite
composition beside the existing operational entry and production caller.

It does not authorize changes to the existing operational-entry function, run
discovery, startup scanning, a daemon, detached work, background threads,
public API, CLI, SDK, schemas, runtime configuration, automatic approval,
LocalExecutor integration, hosted scheduling, provider mutation, OpenShell,
nested harness execution, new wait families, or release posture changes.

## 3. Adoption-Site Assessment

The selected site is appropriate. The operational-entry boundary is the only
current local surface that already carries the exact opening or existing
window locator, immutable run bundle, actor binding, executor binding, skill
input, and one-use Core admission path.

LocalExecutor, CLI, hosted worker, startup scan, and background-loop adoption
are correctly rejected. Each would require a new state bridge, public ownership
contract, discovery mechanism, or hosted scheduling semantics that the current
private caller does not provide.

## 4. Composition Assessment

The proposed algorithm is narrow and phase-ready:

1. preserve bounded clones of the exact locator and skill input before the
   operational-entry input is consumed;
2. call `enter_trusted_host_operation` exactly once;
3. return its complete result unchanged for every disposition other than
   `AwaitCondition`;
4. route only authoritative `AwaitCondition` into
   `run_trusted_host_local_production_caller`; and
5. return the accepted repeated-scheduling outcome unchanged.

This does not reinterpret a wait result as authority. The production caller
reobserves durable Core state before scheduling and before every executor
entry.

The initial `AwaitCondition` result cannot contain a successful `SkillOutput`:
the supervisor produces that disposition only for a yielded attempt, and a
yielded attempt carries no `SkillOutput`. The proposed routing therefore does
not discard a real initial output. Immediate successful or terminal results
remain available through `EntryStopped` unchanged.

## 5. Authority And Binding Assessment

The plan preserves the accepted authority boundary:

- operational entry remains the only initial executor-admission path;
- the production caller remains the only post-yield scheduling path;
- locator, actor, immutable run bundle, executor commitment, and skill input
  remain identical across both calls;
- no durable event, receipt, ticket, handoff, or prior disposition is converted
  into an in-memory capability; and
- stale, substituted, ambiguous, replayed, or corrupt state remains an error.

The phrase `already-authorized owner` describes the calling context, not a new
authority claim accepted from input. The implementation must continue to rely
on Core checks for current authority at every admission boundary.

## 6. Outcome And Output Assessment

The proposed closed enum preserves an important ownership distinction:

- `EntryStopped` means the initial operational entry reached its bounded stop;
  and
- `ContinuationStopped` means the accepted production caller owned at least
  the continuation path after a durable yield.

Blocked, terminal, canceled, unsupported-wait, and budget-exhausted posture
must remain distinct. `WorkflowOsError` must remain an error. No outcome may
claim workflow completion, failure, approval, retry authorization, or a resume
recommendation beyond the authoritative disposition already returned by Core.

The accepted repeated-scheduling outcome intentionally does not carry a
`SkillOutput`. This composition must not invent a second output channel. Any
future public owner that needs terminal output retrieval requires a separate
review of durable result ownership and must not broaden this private adoption
slice implicitly.

## 7. Cancellation And Shutdown Assessment

The caller-owned cancellation design is correct. The invoking local owner
creates the existing cancellation pair, retains the handle, and passes the
receiver into the synchronous composition. Cancellation wakes local waiting
without changing continuity state or workflow semantics.

Process exit loses only process-local waiting. Owner-loss detection, signals,
detached jobs, durable shutdown records, and automatic restart remain
correctly deferred.

## 8. Restart And Concurrency Assessment

Restart requires a new explicit call, reopened backend, exact durable locator,
immutable execution input, fresh cancellation pair, and fresh production
identities. The fixed wake budget is not restored.

Competing callers continue through existing reservation, directive-consumption,
and replay boundaries. The implementation must return a losing caller's
actual error or bounded posture and must not normalize it into retry.

## 9. Privacy And Debug Assessment

The planned input and outcome require bounded custom `Debug` and no serde
surface. They must not expose locators, random identities, actor IDs, prompts,
skill inputs or outputs, commands, paths, credentials, provider payloads,
evidence bodies, approval reasons, or reusable authority.

No new workflow event, audit record, report artifact, metric, or durable host
record is required for this private composition. Existing Core continuity and
supervisor records remain authoritative.

## 10. Test Plan Assessment

The planned tests cover the material risks: immediate stops, one and two
lawful waits, unsupported waits, cancellation, budget exhaustion, substituted
bindings, restart, competing owners, replay conflict, privacy, and absence of
public or background surfaces.

Implementation review should additionally verify directly that:

- every non-`AwaitCondition` initial result bypasses the timer;
- the initial yielded result contains no `SkillOutput` before routing;
- the composition input and both outcome variants have bounded redacted Debug;
- same-binding propagation is tested for the locator, skill input, and executor
  commitment rather than inferred only from lower-layer tests; and
- no clone weakens a one-use authority object or creates a caller-authored
  capability.

These are focused coverage requirements, not planning blockers.

## 11. Documentation Assessment

The plan and planning report accurately describe a private, synchronous,
explicit, local, finite adoption. They do not overclaim automatic continuation,
general scheduling, hosted ownership, provider mutation, or public product
availability.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Decide durable terminal-output ownership before any public owner surface
  needs output retrieval after repeated scheduling.
- Add direct composition-level Debug and same-binding propagation assertions.
- Preserve caller-level cancellation-during-wait coverage before ownership
  becomes longer-lived or detached.
- Keep the fixed two-wake budget and non-retrying replay posture.
- Complete the separate locked dependency/MSRV audit recorded by the caller
  implementation review.

## 14. Recommended Next Phase

Implement only the additive crate-private explicit local operational-entry
composition described by the accepted plan. Keep both existing underlying
functions unchanged and add focused routing, binding, cancellation, restart,
concurrency, error, and privacy tests.

Do not add run discovery, a daemon, background ownership, public configuration,
CLI, SDK, schemas, automatic approval, provider mutation, OpenShell, nested
harnesses, hosted scheduling, or release changes.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1791366610785515000-2`
- approval: `approval/run-1791366610785515000-2/review-scope-approved`
- presentation: `presentation/6bd28d55a1f73289`
- presentation hash:
  `6bd28d55a1f73289ec706fc16dc76a75a0028b1be1f888eb2eb9b1fc162972ac`
- approval outcome: granted by delegated maintainer through proof enforcement
- reviewed planning merge commit: `190a9ef09024cf3dfe5c447fea3992d67ffd0739`
- approved boundary: focused maintainer/security planning review only
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced with one persisted record
- validation summary: source, test, plan, roadmap, docs, and diff-hygiene review
- out-of-kernel work: source and test inspection, review authoring, shell
  validation, and later git and pull-request actions
