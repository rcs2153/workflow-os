# Trusted-Host Explicit Local Caller Adoption Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups. Proceed to planning the
smallest explicit process-owned local owner boundary.**

The merged implementation faithfully composes the accepted private
operational entry and bounded local production caller. It does not create a
parallel authority path, reinterpret host timing as eligibility, or widen the
surface beyond local SQLite internals.

## 2. Scope Verification

The phase stayed within the approved implementation-only scope. It added one
crate-private composition, private input and outcome vocabulary, focused
tests, status documentation, and an implementation report.

It did not add run discovery, startup scanning, a daemon, background thread,
detached job, public API, CLI, SDK, schema, workflow configuration,
LocalExecutor bridge, automatic approval, provider read or mutation,
OpenShell, nested harness execution, hosted scheduling, PostgreSQL parity,
new wait families, a configurable wake budget, or release posture changes.

## 3. Composition Assessment

`run_explicit_trusted_host_local_operation` implements the accepted algorithm:

1. destructure one exact operational-entry input;
2. retain bounded clones of only the private locator and `SkillInput` needed
   by a possible continuation;
3. invoke `enter_trusted_host_operation` exactly once;
4. return every non-`AwaitCondition` result unchanged; and
5. route only `AwaitCondition` into
   `run_trusted_host_local_production_caller`.

The composition is small, auditable, and appropriately remains in the private
SQLite module tree. Both underlying operations remain unchanged.

## 4. Authority And Binding Assessment

No capability is cloned, serialized, persisted, or reconstructed. The cloned
locator contains inert identity and immutable-binding data, not one-use
authority. The production caller receives the same backend, exact locator,
executor, and in-memory skill input used by initial entry.

The prior `AwaitCondition` result is routing information only. The production
caller reobserves durable Core state before scheduling, and the existing
reinvocation path rechecks trusted time, wait binding, current state,
reservation, invocation commitment, and executor commitment before entry.

Changed skill input fails at the existing operational-entry commitment check
before identity generation or executor invocation. The phase adds no
caller-authored eligibility, wait-satisfaction, approval, wake-budget, or
resume claim.

## 5. Stop And Output Assessment

The closed outcome preserves ownership and stop semantics:

- `EntryStopped` contains the complete initial redispatch-loop outcome; and
- `ContinuationStopped` contains the complete accepted repeated-scheduling
  outcome.

Immediate terminal and blocked posture bypass scheduling. A no-wait
turn-boundary yield derives `ResumeNow` and remains inside operational entry;
it is not misclassified as a scheduled wait. The production caller preserves
canceled, blocked, terminal, unsupported-wait, and wake-budget-exhausted
posture.

The initial `AwaitCondition` path cannot discard successful output because it
originates from a yielded attempt, and yielded attempts carry no
`SkillOutput`. Immediate terminal output remains available, redacted in Debug,
through `EntryStopped`.

## 6. Cancellation And Restart Assessment

Cancellation ownership remains with the invoking process. The owner creates
the existing cancellation pair, retains its handle, and supplies only the
receiver. Cancellation wakes the private timer and returns bounded canceled
posture without authorizing execution or claiming workflow termination.

The reopened-backend test proves that a new process-local call reconstructs
current Core posture from durable state while preserving exact invocation
binding. Fresh production identities and a fresh fixed budget are selected by
the accepted caller; process-local waiting or identity material is not treated
as durable authority.

## 7. Concurrency And Replay Assessment

The wrapper adds no retry or race normalization. Existing atomic reservation,
directive-consumption, replay, and supervisor boundaries remain responsible
for one-winner executor admission. A losing, stale, ambiguous, or replayed
caller returns the existing bounded posture or error.

Direct full-path competing-owner and commit-fault cases are not duplicated at
this thin wrapper. The exact primitives it composes retain accepted concurrent
winner, replay, crash, and stale-binding tests. This is adequate for the
private composition, but a direct two-wrapper race should be added before an
owner becomes detached or longer-lived.

## 8. Privacy And Debug Assessment

The input implements bounded custom Debug and exposes only a redacted binding
marker. The outcome delegates to already-reviewed bounded Debug
implementations: skill output is represented only as a redacted presence
marker, and repeated scheduling exposes only closed disposition, counts, and
stop reason.

Focused tests show that secret-like substituted skill input, run identity, and
output markers do not appear in input, outcome, or error Debug. The phase adds
no serde, logs, audit records, events, metrics, report artifacts, host records,
or payload persistence.

## 9. Test Quality Assessment

The seven direct composition tests cover:

- terminal bypass without timer or identity use;
- one lawful `TimeWindow` continuation;
- two lawful wakes within the fixed production budget;
- no-wait `ResumeNow` handling;
- owner cancellation after initial yield;
- substituted invocation rejection and error non-leakage; and
- reopened-backend binding plus outcome Debug safety.

The tests exercise the existing constructors and real private composition.
They do not fabricate capabilities or bypass Core.

Unsupported-wait, wake-budget-exhaustion, early-wake, deadline-failure,
duplicate-identity, competing-reservation, replay, and zero-write cancellation
coverage remains in the unchanged lower-layer suites. The implementation
report accurately labels that coverage as inherited rather than direct.

## 10. Documentation Assessment

The plan, implementation report, and roadmap accurately describe the slice as
private, synchronous, explicit, local, finite, and SQLite-only. They do not
claim automatic continuation, a process owner, public configuration, hosted
parity, provider mutation, or production readiness.

## 11. Validation Assessment

Implementation validation passed locally and independently in pull-request
CI:

- 7 focused composition tests;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`; and
- all 7 required GitHub checks, including Rust, docs, security, hosted
  recovery, shared PostgreSQL, integrations, and schema/example contracts.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Add a direct competing-wrapper race before ownership becomes detached,
  persistent, or multi-process.
- Add direct composition-level wake-budget-exhaustion and unsupported-wait
  assertions if the wrapper grows beyond this thin routing role.
- Decide durable terminal-output ownership before any public owner needs to
  retrieve output after repeated scheduling.
- Keep the fixed two-wake budget and non-retrying replay posture until a
  separately reviewed owner contract changes them.
- Define operator notification, owner loss, and stuck-work observability
  before any background or durable ownership model.

## 14. Recommended Next Phase

Plan the smallest explicit process-owned local owner boundary that invokes the
accepted private composition for an already-selected exact operation.

The plan must not add run discovery, startup scanning, queue polling, a daemon,
detached work, public CLI or configuration, automatic approval, provider
mutation, OpenShell, nested harnesses, hosted scheduling, or release changes.
It must define owner lifecycle, cancellation handle retention, bounded result
handling, process shutdown, and what remains manual before implementation.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1791368444154816000-2`
- approval: `approval/run-1791368444154816000-2/review-scope-approved`
- presentation: `presentation/1637b97edb80d118`
- presentation hash:
  `1637b97edb80d118873149a2d1e62142bd4b90592cdd414bb05e8795a05c1a9d`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- reviewed merge commit: `014ad13a83c8a66bee9a8d12999d223ab49237a1`
- approved boundary: focused implementation and security review only
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval-presentation proof enforced with one persisted presentation record
- out-of-kernel work: source and test inspection, review authoring, shell
  validation, and later git and pull-request actions
