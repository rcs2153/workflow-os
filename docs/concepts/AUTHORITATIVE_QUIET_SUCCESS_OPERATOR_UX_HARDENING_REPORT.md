# Authoritative Quiet-Success Operator UX Hardening Report

## 1. Executive Summary

The first quiet-success operator UX hardening slice is implemented.

An authoritative run now uses concise human output only when Core selected
`QuietProceed`, the run completed successfully, and its in-memory WorkReport
was generated. The new `run --verbose` path retains the existing bounded
route, disclosure, report, and local-check reference detail. Preview JSON is
unchanged.

The phase changes presentation only. It does not alter route selection,
execution, policy, approval, evidence, local-check, WorkReport, or durable run
semantics.

## 2. Scope Completed

- Added command-local `run --verbose` for authoritative execution.
- Added four-line quiet-success human output.
- Kept the run ID and durable `inspect` next action visible.
- Retained detailed output for every non-quiet or unsuccessful posture.
- Retained the existing bounded JSON shape.
- Rejected `run --verbose` when authoritative execution is not active.
- Added focused CLI regressions and user-facing documentation.

## 3. Quiet-Success Contract

Compact output requires all three conditions:

1. Core selected `QuietProceed`;
2. durable run status is `Completed`; and
3. report posture is `Generated`.

The default output is:

```text
status: Completed
governance: quiet_success
run_id: <run-id>
inspect: workflow-os inspect <run-id>
```

The renderer does not independently infer risk or weaken a selected route.

## 4. Explicit Detail And Failure Behavior

`run --verbose` exposes the existing bounded fields for an authoritative run:

- workflow and run identity;
- selected route;
- execution and disclosure posture;
- report posture and report ID;
- local-check result reference ID;
- approval identity and complete handoff when applicable;
- stable report error code; and
- inspect command.

Global `--json` remains unchanged. A failed run, failed report, visible
disclosure, approval-required route, or denial continues through the detailed
renderer even without `--verbose`.

## 5. Compatibility Boundary

Ordinary `workflow-os run <workflow-id>` behavior is unchanged.

`--verbose` is accepted only when authoritative execution is active through
the project declaration or the experimental compatibility flag. It does not
enable authoritative execution, choose a governance route, select a command,
or change workflow state.

## 6. Privacy And Redaction

Compact output contains only terminal status, a fixed governance label, run
identity, and an inspect command. It contains no raw command output, source
content, provider payload, path, environment value, credential, token,
approval reason, report body, or local-check payload.

Verbose and JSON output retain their previously reviewed bounded,
payload-free shapes.

## 7. Tests Added Or Changed

Focused CLI tests prove:

- default completed quiet output is concise;
- verbose quiet output retains route, disclosure, report, and check-reference
  detail;
- a project declaration receives the same concise default behavior;
- non-authoritative `run --verbose` fails before state creation;
- JSON remains bounded and machine-readable;
- denial and approval output remain explicit through existing regressions; and
- no report artifact is created.

## 8. Validation

The following commands passed:

- focused authoritative-governance CLI tests;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:docs`; and
- `git diff --check`.

`npm run check` used the repository-pinned Node 20 toolchain and covered docs,
the dogfood phase helper, the integration helper, the TypeScript SDK, and
cross-language contracts.

## 9. Scope Explicitly Not Completed

This phase does not add or change:

- governance selection or workload inference;
- execution, retry, approval, denial, or resume semantics;
- automatic or delegated approval;
- report persistence, report artifacts, or export;
- provider or OpenShell integration;
- SideEffect execution or external writes;
- schemas, scaffold defaults, hosted behavior, enterprise controls, or release
  posture.

## 10. Remaining Limitations

- WorkReports remain in memory on this path.
- Only the closed `observe_and_report` plus
  `workflow_os_project_validation` project activation is supported.
- The explicit compatibility flag remains available.
- A later operator surface may project quiet decisions live without changing
  their quiet governance obligation.

## 11. Recommended Next Phase

Perform a focused maintainer review of this operator UX hardening slice.
After acceptance, select the next runtime-composition phase from current
`main`; do not broaden provider mutations or begin OpenShell integration from
this presentation-only change.

## 12. Governed Implementation Record

- workflow: `dg/implement`
- run: `run-1785185666278844000-2`
- approval:
  `approval/run-1785185666278844000-2/implementation-approved`
- presentation: `presentation/0a0c75f98a7909f8`
- approval outcome: granted by delegated maintainer through persisted
  presentation-proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation summary: focused authoritative CLI tests, Rust formatting and
  lint checks, workspace tests, Node 20 repository checks, documentation
  checks, and diff checks passed
- report posture: this implementation report is persisted in the repository;
  no runtime WorkReport artifact was generated
- out-of-kernel work: code and documentation inspection, repository edits,
  validation commands, focused review, git, and pull-request operations
- kernel boundary: the kernel governed scope and approval; it did not inspect
  code, edit files, run validation, or perform git and pull-request actions

## 13. CI Blocker Fix-Forward

PR validation under Rust 1.97.1 found that the authoritative renderer exceeded
Clippy's 100-line threshold after the quiet-success branch was added. The
branch was extracted into a private helper without changing eligibility or
output. The focused verbose regression was also corrected to assert the
authoritative generator's stable `report/<run-id>` contract rather than an
unsupported `work-report/` prefix.

The fix was governed through `dg/blocker`, run
`run-1785186663143511000-2`, with proof-enforced approval
`approval/run-1785186663143511000-2/fix-approved`.
The completed run recorded 39 events, one approval, zero retries, and zero
escalations. Focused authoritative CLI tests, workspace formatting, workspace
Clippy with warnings denied, the full Rust workspace test suite, `npm run
check`, and `git diff --check` passed after the fix.
