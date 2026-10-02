# Operational Execution Window Trusted-Host Supervisor Report

> Fix-forward status: the focused review found replay, invocation-binding,
> reconciliation, and regression-matrix blockers. The bounded repair is
> documented in [Operational Execution Window Trusted-Host Supervisor Blocker
> Fix Report](OPERATIONAL_EXECUTION_WINDOW_TRUSTED_HOST_SUPERVISOR_BLOCKER_FIX_REPORT.md).
> This note preserves the original phase record rather than rewriting its
> historical claims.

## 1. Executive Summary

The first one-shot injected local trusted-host supervisor is implemented as a
private Workflow Core boundary. It composes the accepted operational opening
capability with the accepted projected continuity operations, invokes one
injected executor exactly once, and returns Core's authoritative continuation
disposition after durable outcome reconciliation.

The focused vertical slice opens a first attempt, records a turn-boundary
yield, observes `ResumeNow`, consumes one current directive to start a second
attempt, invokes an existing local `SkillHandler`, records success, and proves
that successful host work does not fabricate terminal workflow state.

This remains local, opt-in, private, and SQLite-only. It is not a scheduler,
agent runtime, provider adapter, sandbox, or public execution configuration.

## 2. Scope Completed

- Added a crate-private `TrustedHostAttemptExecutor` injection boundary.
- Added a crate-private adapter around the existing `SkillHandler` contract.
- Added one-shot supervision for opening and resumed attempt capabilities.
- Validated the exact workflow, run, step, and
  `invoke_current_step_skill` operation binding before executor entry.
- Mapped one injected result to success, retryable failure, terminal failure,
  turn-boundary yield, or ambiguous-may-have-started continuity operations.
- Persisted results through the accepted projected continuity APIs.
- Returned the current Core-derived continuation disposition after mutation.
- Added a focused SQLite proof of yield, resume, success, and no false workflow
  completion.

## 3. Scope Explicitly Not Completed

This phase did not add:

- a scheduler, daemon, polling loop, or repeated dispatch;
- model-turn creation or automatic conversational resume;
- approval automation, self-approval, or evidence/check bypass;
- provider execution, provider mutation, live adapters, or OpenShell;
- nested harness execution, recursive agents, or agent swarms;
- public runtime configuration, workflow schemas, CLI, SDK, UI, or examples;
- filesystem or PostgreSQL support for operational opening;
- prompt, transcript, source, command-output, or provider-payload storage;
- Reasoning Lineage, hosted execution, or release-posture changes.

## 4. Private Supervisor API

The private supervisor receives:

- an explicit SQLite backend;
- either the private opening capability or one consumed-directive attempt
  capability plus its expected binding;
- one injected executor;
- one already-bounded `SkillInput`; and
- explicit operation, receipt, and optional yield-generation identities.

It returns the current authoritative continuation disposition and, only after
a successful local skill invocation, the existing bounded `SkillOutput`.
Debug output redacts invocation binding and skill output.

The API does not accept caller-authored readiness, policy, approval, or
authority assertions. Those remain prerequisites of the capability supplied
by Core.

## 5. Opening-Attempt Origin Bridge

Operational opening creates attempt number one before a continuity
`consume_directive` operation exists. The accepted five-operation continuity
contract remains closed, so the implementation does not invent a sixth
continuity operation or a fake resume directive.

SQLite V4 now records a closed origin discriminator for each continuity
attempt: `consume_directive` or `operational_opening`. Resume-created attempts
must still resolve to a committed successful consume operation. Opening-created
attempts must resolve to the exact opening operation, window, and attempt.
The opening and continuity records share the same operation identity. Codec
validation fails closed if either durable relationship is missing or crossed.

The final post-migration V4 manifest contains 35 objects and is bound to digest
`1016e62bd4d5b3e27f29822212cfe771de7287206799a9135d8f054abb5c5aeb`.

## 6. Dispatch, Yield, Resume, And Outcome Behavior

One supervisor call owns one executor call and one durable result:

1. Normalize the opening or resumed one-use attempt capability.
2. Validate the exact immutable invocation identity and operation commitment.
3. Invoke the injected executor once.
4. Persist one projected yield, outcome, or recovery operation.
5. Read and return the current continuity disposition.

A turn-boundary yield remains runnable and produces `ResumeNow`; it is not
misclassified as an approval wait or workflow completion. Resume still
requires one current directive and a fresh attempt capability. The supervisor
does not loop or redispatch automatically.

## 7. Workflow Semantics

The supervisor mutates continuity state and its bounded runtime projection
only through accepted Core operations. It does not append `RunCompleted`,
change workflow pass/fail semantics, mutate project specs, or infer terminal
state from a successful executor callback. The focused proof ends with a
terminal execution-window disposition while the workflow run remains
`Running`.

## 8. Privacy And Redaction

- Invocation Debug output exposes no skill values or bindings.
- Result Debug output exposes disposition and only a redacted output marker.
- Stable errors describe the failed boundary without echoing IDs, paths,
  payloads, metadata, or secret-like values.
- Persisted origin metadata contains bounded IDs and a closed enum only.
- No prompt, transcript, source, command output, environment value,
  credential, authorization header, or provider payload is introduced.

## 9. Test Coverage

Focused tests cover:

- atomic opening and first-attempt capability consumption;
- first-attempt turn-boundary yield;
- `ResumeNow` derivation;
- exact directive consumption and second-attempt start;
- invocation through the existing local `SkillHandler` adapter;
- durable success outcome;
- all required bounded runtime projection events;
- execution-window terminal disposition without terminal workflow state; and
- rejection of a substituted invocation binding before executor entry;
- redacted supervisor result Debug output; and
- schema/origin integrity for opening-created attempts.

Existing opening, continuity, executor, adapter, report, and runtime coverage
remains part of broader validation.

## 10. Validation

Passed:

- `cargo fmt --all --check`
- `cargo clippy -p workflow-core --all-targets -- -D warnings`
- `cargo test -p workflow-core --lib` (`338 passed`)
- focused operational-opening supervisor tests (`2 passed`)
- operational-opening module tests (`15 passed`)
- continuity-store unit and conformance tests (`35 passed`)
- `npm run check:docs`
- `git diff --check`

`cargo test --workspace` was also started and produced no failures across the
completed CLI, read-only example, vertical-slice, Workflow Core library, and
adapter suites. The local host took approximately four minutes to start each
integration-test binary even though each suite completed in seconds. With 71
workspace integration-test files, the command was stopped cleanly after the
completed representative suites rather than being misreported as a pass.
Repository CI remains responsible for the remaining workspace binaries.

## 11. Remaining Known Limitations

- The supervisor is crate-private and one-shot.
- It cannot create, schedule, or guarantee a model turn.
- It does not repeatedly consume `ResumeNow` directives.
- The existing local `SkillHandler` adapter maps handler errors
  conservatively to terminal failure; richer retry classification requires a
  separately reviewed executor contract.
- Operational opening remains SQLite-only and explicitly selected.
- No public configuration selects this path.
- No provider or sandbox executes through this boundary.
- Full scheduler behavior, typed external waits, host crash recovery loops,
  and broader operations remain future work.

## 12. Recommended Next Phase

Perform a **focused maintainer/security review of the one-shot trusted-host
supervisor vertical slice**. Review the opening-attempt origin bridge,
capability binding, one-call execution rule, yield/resume path, result mapping,
no-false-completion guarantee, reconciliation posture, privacy, and regression
coverage before any repeated scheduler or additional executor integration.

## 13. Governed Phase Record

- workflow: `dg/runtime-composition`
- run: `run-1790848843763774000-2`
- approval: `approval/run-1790848843763774000-2/composition-approved`
- presentation: `presentation/ae69c9ac18e9df34`
- presentation hash:
  `ae69c9ac18e9df347b2e5923735580df5e71e6988c557a47a199911110bc0524`
- approval outcome: granted
- phase status: completed
- event summary: 39 events, including 1 approval grant, 0 retries, and 0
  escalations
- approved boundary: one injected local attempt and one typed Core outcome
- out-of-kernel execution: source edits and validation commands were executed
  by the trusted local development host; Workflow OS governed the phase but did
  not execute shell commands or edit repository files.
