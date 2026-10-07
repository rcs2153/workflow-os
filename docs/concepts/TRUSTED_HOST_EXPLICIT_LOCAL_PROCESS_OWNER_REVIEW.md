# Trusted-Host Explicit Local Process Owner Review

Fix-forward note: the simultaneous cancellation-versus-entry race and complete
competing-owner loser classification requested by this review are now
implemented in the [test hardening
report](TRUSTED_HOST_EXPLICIT_LOCAL_PROCESS_OWNER_TEST_HARDENING_REPORT.md).
The original findings below remain unchanged as the historical review record.

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups. Proceed to planning one exact
application adoption site.**

The implementation faithfully realizes the corrected private owner contract.
It adds one process-local cancellation-versus-entry linearization point, a
truthful pre-entry-canceled outcome, one-shot ownership, and at-most-once
delegation into the previously accepted explicit local operation. It does not
create authority, reinterpret workflow lifecycle, or expose automatic runtime
behavior.

The focused tests establish the core behavior. Two test-hardening gaps should
be carried as requirements into the adoption plan: exercise a simultaneous
cancel-versus-entry race rather than only both serialized orderings, and assert
the complete bounded losing-owner result under competing owners. These gaps do
not block acceptance while the owner remains crate-private and uncalled.

## 2. Scope Verification

The phase stayed within the approved crate-private implementation scope. It
added one owner module, a private timer entry decision, focused tests, plan and
roadmap status, and an implementation report.

It did not add an application adoption site, discovery, startup scanning,
state polling, automatic continuation, automatic approval, a daemon,
background thread, worker pool, async runtime, signal handler, durable owner,
host job, lease, leader election, public API, CLI, SDK, schema, runtime
configuration, provider behavior, OpenShell integration, nested harnesses,
hosted scheduling, PostgreSQL scheduling parity, report artifact, or release
change.

## 3. Owner API And Lifecycle Assessment

`TrustedHostExplicitLocalProcessOwner` remains crate-private and owns exactly
one `TrustedHostOperationalEntryInput` plus one cancellation receiver.
Construction creates the existing in-memory cancellation pair and returns its
handle without reading or writing Core state.

`run(self)` consumes the owner, so ordinary callers cannot invoke it twice.
After a successful entry decision it delegates exactly once to
`run_explicit_trusted_host_local_operation`. The owner is not cloneable,
serializable, durable, restartable, or detached.

The outcome is appropriately closed: `CanceledBeforeEntry` states that no
operational entry occurred, while `OperationStopped` preserves the accepted
lower-level result without translating it into workflow success, failure,
cancellation, retry, or completion.

## 4. Cancellation Linearization Assessment

`begin_entry` and `cancel` acquire the same mutex over `canceled` and
`entry_started`. This creates one clear linearization point:

- cancellation first returns `CanceledBeforeEntry` before operational entry;
- entry first sets `entry_started` and permits the accepted operation; and
- cancellation after entry remains recorded for a later timer wait but cannot
  revoke consumed authority or interrupt the active attempt.

The owner cannot reach duplicate entry because `run` consumes it. The stable
`trusted_host_local_timer.entry_already_started` error remains a defensive
invariant for accidental internal reuse.

The direct timer test verifies both serialized outcomes under the exact mutex.
It does not launch `cancel` and `begin_entry` concurrently. A simultaneous
stress or barrier-controlled race should be added before the owner becomes an
operational application boundary.

## 5. Active Attempt And Stop Semantics

The blocking-executor test demonstrates that cancellation after executor entry
does not interrupt the active attempt. After the attempt lawfully yields a
`TimeWindow`, the already-recorded cancellation is observed by the existing
timer path and returns the accepted bounded canceled stop with no second
executor entry.

This accurately preserves the documented limitation: the handle is
cooperative timer control, not a general process-shutdown or executor
interruption token. No workflow cancellation event or terminal claim is
fabricated.

## 6. Authority And Binding Assessment

The owner accepts no capability, approval decision, wait-satisfaction claim,
current-authority posture, or caller-authored continuation disposition. It
holds the exact preselected operational input and passes it unchanged into the
existing reviewed composition.

Core remains responsible for current-state rehydration, immutable binding,
trusted time, directive consumption, reservation, replay, attempt admission,
and terminal truth. The owner adds no retry, fallback, or authority synthesis.

## 7. Concurrency And Replay Assessment

The direct two-owner test uses separate owner instances against the same
durable operation and proves aggregate executor admission is exactly one.
There is no owner-level singleton lock masking Core concurrency. Existing
transactional reservation and replay behavior decides the winner.

The test currently asserts that two results return and at least one is
successful, but it does not classify the losing result or verify its Debug and
error text directly. Before an application adopts the owner, the test should
assert that the loser is one of the accepted bounded postures or stable
non-leaking errors.

## 8. Privacy And Failure Assessment

Owner Debug exposes only a redacted binding marker. Owner-outcome Debug
delegates to the already-reviewed bounded operation outcome. The owner adds no
serde, logs, event, audit, metric, report, artifact, or persistence surface.

Poisoned coordination state maps to the stable timer-state error. Duplicate
entry maps to a stable invalid-state error. Existing operational errors pass
through unchanged; the owner does not retry or copy bound values into a new
message.

## 9. Test Quality Assessment

Direct tests cover:

- zero-write cancellation before entry;
- no identity generation or executor admission when cancellation wins first;
- redacted owner Debug;
- admitted active-attempt non-interruption;
- cancellation observation at a subsequent timer wait;
- idempotent cancellation after return;
- at-most-one executor admission across competing owners; and
- both serialized cancellation-versus-entry orderings plus duplicate-entry
  defense.

The unchanged lower-level suites continue to cover exact-input substitution,
terminal and blocked results, restart and rehydration, lawful timer wakes,
unsupported waits, wake-budget exhaustion, reservation races, replay, and
timer wakeup behavior.

Missing or shallow direct coverage is non-blocking at the current private,
unadopted boundary:

- no simultaneous thread race directly stresses `cancel` against
  `begin_entry`;
- the losing competing-owner result is not classified or checked for leakage;
- zero-state construction and dropping an unrun owner are established by
  implementation inspection rather than dedicated tests; and
- terminal, blocked, restart, unsupported-wait, and budget behavior remain
  inherited rather than duplicated at the thin owner wrapper.

## 10. Documentation Assessment

The plan, implementation report, and roadmap accurately describe the owner as
private, local, one-shot, synchronous, process-local, non-durable, and
non-automatic. They clearly state that active attempts cannot be interrupted
and that application shutdown, notification, discovery, and adoption remain
deferred.

## 11. Validation Assessment

Independent review validation passed on the merged implementation:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

Rust validation used `CARGO_INCREMENTAL=0` and the clean target directory
`/private/tmp/workflow-os-owner-target`, matching the successful implementation
validation recovery from the stale default incremental cache.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Add a simultaneous cancel-versus-entry race test before operational
  application adoption.
- Classify and assert the losing competing-owner result, including bounded
  Debug or error posture, before adoption.
- Add explicit unrun-owner drop and zero-state construction tests if the owner
  lifecycle grows or becomes public.
- Define process signal orchestration and waiting for admitted attempts in a
  separate phase; do not imply active interruption.
- Define operator notification, owner loss, stuck-work observation, and
  durable output ownership before detached or durable ownership.

## 14. Recommended Next Phase

Plan one exact application adoption site for this accepted private owner. The
plan must identify who constructs the already-selected operation, who retains
and invokes the cancellation handle, how the bounded owner result is surfaced,
and how shutdown waits for admitted work without claiming interruption.

The plan must include the two direct test-hardening follow-ups above and must
not implement adoption, discovery, scanning, polling, detached execution,
automatic approval, public API, CLI, SDK, schemas, providers, OpenShell,
nested harnesses, hosted scheduling, or release changes.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1791373289320858000-2`
- approval: `approval/run-1791373289320858000-2/review-scope-approved`
- presentation: `presentation/f11396d854597aba`
- presentation hash:
  `f11396d854597aba189d858885ce224d624d05d1ed70943f2247a09df12fd136`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- reviewed merge commit: `c2f43c2ce8ec837ad4df58c65ce3201e9cda0112`
- approved boundary: focused implementation and security review only
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval-presentation proof enforced with one persisted presentation record
- out-of-kernel work: source and test inspection, review authoring, shell
  validation, and later git and pull-request actions
