# Trusted-Host Project-Validation Profile Implementation Report

## 1. Executive Summary

The second closed trusted-host no-write profile is implemented. The
unpublished `workflow-local-host` foreground binary can now execute the
existing `WorkflowOsProjectValidation` profile against the exact governed
project root while preserving the accepted immutable admission, current
authority, operational lifecycle, workflow terminalization, and replay
boundaries.

The phase also fixes a trusted-host truth defect shared by the existing
docs-check path: only a passing `LocalCheckResult` becomes a successful skill
attempt. Failed, timed-out, or internally failed checks now terminalize the
skill invocation and workflow as failure without rerunning the process or
persisting raw output.

This remains an unpublished, synchronous, local, SQLite-backed proof. It does
not add a released CLI path, arbitrary commands, automatic approval, provider
access, writes, scheduling, hosted behavior, or a WorkReport artifact.

## 2. Scope Completed

- Added an exact closed profile selector for `docs-check` and
  `workflow-os-project-validation`.
- Preserved omitted-profile compatibility for docs-check only.
- Rejected unknown selectors, implicit project-validation selection, and
  mixed profile arguments before SQLite state creation.
- Reused the existing canonical project-validation contract and handler.
- Bound project validation to the canonical governed project root.
- Added a distinct Core-owned explicit-zero current-authority source and
  harness identity for project validation.
- Added a private trusted-host local-check attempt adapter shared by both
  accepted profiles.
- Mapped only passing checks to successful skill output.
- Added actual foreground-binary coverage for successful and failed project
  validation, docs-check failure truth, selector compatibility, replay, and
  bounded output.

## 3. Scope Explicitly Not Completed

This phase does not add:

- released `workflow-os` CLI or SDK behavior;
- arbitrary executables, arguments, environments, handlers, or commands;
- repository command discovery or additional check families;
- automatic or delegated approval;
- scheduling, polling, daemons, detached workers, or process recovery;
- provider access, network access, credentials, or provider mutation;
- source writes, SideEffect execution, WorkReport generation, or report
  artifacts;
- PostgreSQL or hosted parity;
- OpenShell, containers, nested harnesses, or agent teams;
- workflow schema, example, or release-posture changes; or
- elimination of the final validation-to-process-entry race.

## 4. Host API And Selector Summary

The unpublished host recognizes two exact profile values:

```text
docs-check
workflow-os-project-validation
```

Project validation requires the explicit selector and one explicit
`--workflow-os-executable`. Its working directory is the governance project
root already supplied to admission. It accepts no caller-authored command
arguments, environment, timeout, output bounds, or alternate contract.

Explicit `docs-check` and the existing omitted-selector docs-check invocation
remain supported. Unknown selectors, project-validation arguments without its
selector, and mixed docs/project argument families fail before the backend is
opened.

## 5. Authority And Governed-Root Boundary

Core continues to resolve the exact closed profile and its executable-content
commitment. Before operational preparation, project validation compares the
canonical profile working directory with the canonical governed project root.
A mismatch or unavailable root fails with fixed non-leaking errors.

The current-authority source registration now distinguishes the two closed
profiles. Project validation uses its own stable harness and source identities;
it does not reuse docs-check labels. Each profile remains eligible only through
an exhaustive Core-owned explicit-zero decision proving that the exact
operation requires no capability grants, availability facts, governed-context
references, provider authority, or mutation authority.

## 6. Result Truth And Lifecycle

`TrustedHostLocalCheckAttemptExecutor` invokes the already resolved profile
once. A passing result is converted through the existing bounded
`LocalCheckResult` to `SkillOutput` adapter. Failed, timed-out, and internal
handler failures return the existing terminal-failure attempt outcome.

The established supervisor remains responsible for atomic invocation and run
terminal events. Non-passing checks therefore produce
`SkillInvocationFailed` and `RunFailed`, never success or completion. Exact
terminal replay returns durable terminal state without another process or
event.

## 7. Report And Evidence Posture

The path persists bounded workflow and invocation events plus the existing
bounded output reference on success. These records are audit state; they are
not a final `WorkReport`, report artifact, or copied command transcript.

The implementation does not read or persist raw project source, stdout,
stderr, parser payloads, environment values, credentials, or command output as
evidence. Terminal WorkReport composition remains a separate future phase.

## 8. Privacy And Error Boundary

- Host output remains a fixed bounded status string.
- Selector, root, executable, authority, and lifecycle failures use stable
  fixed errors.
- Paths, executable hashes, private fingerprints, environment values,
  workflow input, policy text, approval reasons, stdout, and stderr are not
  included in errors or durable failure events.
- Debug output remains redacted at profile, request, and run boundaries.
- Secret-like failure markers used by tests do not appear in host output or
  rehydrated run Debug output.

## 9. Test Coverage

Focused coverage proves:

- completed, failed, and timed-out closed-profile attempt outcomes;
- project-validation success through the actual foreground binary;
- project-validation failure terminalizes the durable run without leaking
  stderr;
- docs-check nonzero exit now terminalizes as failure;
- failed-run exact replay appends no event and executes no process;
- explicit docs-check and explicit project-validation selectors work;
- the temporary docs-check omission alias remains compatible;
- project-validation arguments without its selector, mixed argument families,
  and unknown selectors fail before state creation;
- same-path executable replacement fails closed;
- unsupported topology, replay substitution, unavailable output, approval
  wait, and incompatible SQLite posture retain their accepted behavior; and
- all existing Core and local-host tests remain active.

## 10. Validation

Completed validation:

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core`: passed.
- `cargo test -p workflow-local-host`: passed, including 13 foreground
  admission tests.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 11. Governed Phase Record

- Workflow: `dg/implement`.
- Run: `run-1792661989260540000-2`.
- Approval:
  `approval/run-1792661989260540000-2/implementation-approved`.
- Approval presentation: `presentation/b37f44e94bef0f57`.
- Approval outcome: granted under delegated-maintainer authority.
- Phase status: completed.
- Event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, and one run completion; zero retries and zero escalations.
- Approval-presentation enforcement: proof enforced with one persisted
  presentation record and an event proof marker.
- Out-of-kernel work: code and test edits, command execution, documentation,
  git, and later PR operations are performed by the delegated maintainer while
  Workflow OS governs phase scope and approval.

No `.workflow-os` runtime state is part of the implementation change set.

## 12. Remaining Known Limitations

- The foreground host interface is unpublished and intentionally unstable.
- Exactly two closed profiles are accepted.
- Current authority remains limited to exhaustive explicit-zero obligations.
- There is no discovery, scheduler, background worker, or process-loss
  recovery.
- There is no terminal WorkReport or report artifact.
- The final validation-to-process-entry race remains outside this phase.
- No provider, mutation, hosted, nested-harness, or OpenShell behavior is
  enabled.

## 13. Recommended Next Phase

Perform a focused maintainer/security implementation review. The review should
verify selector and compatibility behavior, governed-root binding,
profile-specific explicit-zero authority, non-passing result truth, exact
replay, error privacy, and the absence of report/write scope expansion.

Do not add another profile, generic commands, scheduler behavior, default
activation, provider access, writes, or hosted expansion before that review is
accepted.
