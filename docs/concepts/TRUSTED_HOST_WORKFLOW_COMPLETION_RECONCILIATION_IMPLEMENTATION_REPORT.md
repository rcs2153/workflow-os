# Trusted-Host Workflow Completion Reconciliation Implementation Report

## 1. Executive Summary

The accepted private trusted-host completion design is implemented for the
existing one-step, no-write docs-check profile. Core now composes operational
execution and workflow lifecycle inside the same authoritative SQLite
transactions. A terminal foreground result therefore corresponds to a
terminal durable workflow run instead of leaving the run indefinitely
`Running`.

The foreground host remains non-authoritative. It cannot append workflow
events, choose invocation identity, validate required outputs, or infer run
completion from a bounded application result.

## 2. Scope Completed

- Added a private Core-owned workflow lifecycle context bound to invocation,
  step, skill, version, idempotency, and immutable required-output names.
- Opening commits `SkillInvocationRequested` before
  `OperationalExecutionWindowOpened` in one transaction.
- Dispatch commits `AuthorizedExecutionAttemptDispatchAdmitted` before
  `SkillInvocationStarted` in one transaction and before executor entry.
- Terminal persistence commits `AuthorizedExecutionContinuityProjected`, the
  invocation terminal event, and the run terminal event in one transaction.
- Required-output validation occurs before terminal outcome selection.
- Exact terminal replay validates durable invocation and executor commitments
  and appends no events.
- Existing bounded foreground posture output remains unchanged.

## 3. Scope Explicitly Not Completed

- No public CLI, SDK, workflow schema, or runtime configuration was added.
- No generic completion API or additional trusted-host profile was added.
- No scheduler, daemon, polling loop, detached worker, or background process
  was added.
- No provider mutation, OpenShell integration, nested harness execution,
  automatic approval, or hosted runtime behavior was added.
- No raw command output, environment value, provider payload, or secret is
  persisted as workflow evidence.

## 4. Transaction And Ordering Summary

The implementation uses the existing SQLite transaction boundaries instead of
adding a post-return reconciler:

1. The opening transaction validates the lifecycle commitment and writes the
   invocation request followed by the operational opening projection.
2. The dispatch transaction validates the same lifecycle commitment and writes
   dispatch admission followed by invocation start.
3. The terminal outcome transaction normalizes the exact in-memory result,
   writes the continuity projection, writes invocation success or failure, and
   writes run completion or failure.

The durable event cursor advances through each complete sequence. Operation
projection bindings remain tied to their own projection event while the
authoritative run snapshot advances through the matching lifecycle events.

## 5. Output Validation And Failure Behavior

Success is selected only when every immutable required output is present.
Missing required output maps to stable code
`executor.output.required_missing` and atomically fails the invocation and
run. A true handler error maps to fixed code
`trusted_host.execution.failed`. Neither path copies handler payloads into
errors or Debug output.

Retryable, yielded, and ambiguous attempt outcomes continue through their
existing continuity behavior and do not receive workflow terminal events.

## 6. Replay, Identity, And Crash Posture

Core owns invocation identity and deterministically derives the skill attempt
identity from the authorized execution attempt. Terminal replay rereads the
durable event stream and verifies exactly one dispatch whose invocation and
executor commitments match the current request. Mismatched actor,
correlation, invocation, or executable identity fails closed.

Opening, dispatch, and terminal lifecycle events are committed with their
matching authoritative operations. The implementation does not create a state
where continuity success is durable while workflow success depends on lost
in-memory output.

## 7. Privacy And Redaction

The lifecycle context Debug representation exposes only a redacted binding.
Terminal outcome Debug redacts output references and failure details. Stable
errors use bounded messages. Required-output failure tests include a
secret-like marker and verify that it is absent from Debug output and durable
failure construction.

## 8. Test Coverage

Focused coverage proves:

- terminal success produces the exact seven-event lifecycle suffix;
- missing required output atomically fails invocation and run;
- terminal handler failure maps to bounded failure events;
- exact terminal replay appends no events;
- actor and correlation substitution fail closed;
- same-path executable replacement fails closed;
- approval wait and unsupported topology do not execute;
- incompatible SQLite input fails without path leakage; and
- existing supervisor result-matrix, yield, ambiguity, replay, stale-cursor,
  substitution, and persistence-fault tests remain part of the workspace
  suite.

Focused maintainer/security review found two terminal-transaction blockers.
The corrected implementation now projects lifecycle terminal events only when
the continuity operation commits successfully; current-authority rejection
projects only its rejection event and leaves the workflow non-terminal. The
terminal request commitment now also binds the lifecycle context and exact
bounded terminal outcome, so changed output or lifecycle context cannot be
accepted as an exact replay.

## 9. Validation Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed with
  `CARGO_BUILD_JOBS=2` in 24 minutes 33 seconds after correcting a test-only
  assertion that had required `Debug` on a private mutation result.
- `cargo test -p workflow-local-host --test foreground_admission -- --nocapture`:
  passed, 7 tests.
- `cargo test --workspace --lib --bins --tests --examples`: passed. This
  executable-only workspace run excludes doctests intentionally and includes
  475 `workflow-core` unit tests, 343 `local_executor` tests with one opt-in
  live test ignored, and all 7 `workflow-local-host` foreground-admission
  tests.
- An earlier `cargo test --workspace` run passed every executable unit,
  integration, and example target but did not reach a successful process exit
  because `rustdoc` became idle during `workflow_core` doctest discovery.
- `cargo test --workspace --doc`: reproduced the same `workflow_core`
  `rustdoc` stall in isolation. After more than five minutes at zero CPU with
  no child process or output, the validation process was terminated. No
  doctest result is claimed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

One initial foreground-suite run returned bounded transient
`invalid_state` results in four tests after a fresh compile. Immediate isolated
and full parallel reruns passed. The result is disclosed for focused review;
no reproducible shared-state collision was established.

The first workspace run compiled successfully and executed the CLI and Core
suites. It found one unrelated timing-sensitive assertion in an existing
repeated-scheduling test after 473 other Core tests passed. That assertion
required a generated 200 ms deadline to remain later than wall-clock `now`
under a heavily loaded suite. The test now treats the generated deadline as an
opaque schedule value while retaining exact assertions for caller-supplied
deadlines. Its focused rerun passed, and the complete executable workspace
test set passed on the final rerun.

The local `rustdoc` hang is reproducible independently of the workspace test
runner. It is recorded as an environmental/toolchain validation limitation,
not as a passing doctest result and not as evidence of a product-test failure.

## 10. Governed Phase Evidence

- Implementation dogfood workflow: `dg/runtime-composition`.
- Run: `run-1791507314920490000-2`.
- Approval: `approval/run-1791507314920490000-2/composition-approved`.
- Approval presentation: `presentation/6e19722d2941b268`.
- Approval outcome: granted through the persisted presentation-proof path.
- Out-of-kernel work: source edits, tests, formatting, documentation checks,
  and this implementation report were performed by the delegated maintainer;
  the dogfood runner coordinated governance and did not perform those actions.

The focused maintainer/security review used `dg/review`, run
`run-1792500349184071000-2`, approval
`approval/run-1792500349184071000-2/review-scope-approved`, and presentation
proof `presentation/14fb7c1060b094f3`. The proof-enforced approval was granted
under delegated maintainer authority. Phase close reported `Completed` with 39
events, one approval, zero retries, and zero escalations.

The final `phase-close` command could not reconstruct the event summary. The
runner had stored this run under the operating system temporary directory,
`workflow-os-self-governance-state`, and that directory was empty when closure
was attempted after the interrupted phase interval. Both `status` and
`inspect` failed with `runtime.events.empty`. No event count, retry count, or
escalation count is invented. This is a dogfood-state retention limitation to
address separately; it does not weaken the implementation test evidence.

## 11. Remaining Known Limitations

- Only the private one-step docs-check profile receives lifecycle composition.
- Retryable, yielded, and ambiguous attempts remain governed by the existing
  continuity paths and are not a broader scheduling claim.
- Application output remains intentionally unable to distinguish workflow
  success from fixed terminal failure beyond bounded terminal posture.
- Audit and observability sink delivery remains downstream of durable workflow
  truth.
- The unusually large `workflow-core` unit-test target has high local link
  cost and requires focused review of validation duration.
- Dogfood phase state stored under the operating system temporary directory
  can be cleaned before a delayed `phase-close`, preventing final event-trail
  summarization.

## 12. Recommended Next Phase

Perform a focused maintainer and security review of this implementation before
adding any profile, scheduler, public surface, provider, OpenShell, or nested
harness behavior.
