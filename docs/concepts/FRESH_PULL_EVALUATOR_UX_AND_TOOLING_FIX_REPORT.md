# Fresh-Pull Evaluator UX And Tooling Fix Report

## 1. Executive Summary

A fresh-pull evaluation of current `main` found that Workflow OS now presents a
coherent and honest local governance-kernel experience. Existing-repository
scaffolding preserves agent guidance, first-run output is concise by default,
workflow recommendations remain review-only, inactive draft authoring fails
closed before promotion, and mock runtime demonstrations remain visibly
separate from real repository posture analysis.

The evaluation identified two bounded product-quality defects:

- `npm run check:integrations` could surface an opaque child-process result
  when a spawned command exceeded Node's implicit output boundary; and
- `workflow-os validate` printed the missing-manifest diagnostic once during
  command rendering and again through the returned error.

Both defects are fixed without changing kernel governance semantics.

## 2. Feedback Assessment

The evaluation's product verdict is credible and aligned with the current
roadmap:

- Workflow OS is a local-first governance kernel, not a general execution
  platform.
- The strongest current path is safe onboarding, bounded posture analysis,
  review-first workflow authoring, approval, and durable event inspection.
- The primary remaining product challenge is reducing ceremony for eligible
  low-risk work while preserving evidence and auditability.

That final recommendation does not require a new architecture lane.
Proportional governance already separates execution disposition from operator
disclosure, derives review-only workload recommendations from bounded facts,
binds assessments to immutable inputs, and reassesses on retry and approval
resume in an explicit opt-in path. Runtime quiet-success integration remains
intentionally incomplete.

The canonical local-check resolver remains the next deeper implementation
because proportional governance cannot safely treat check posture as complete
until workflow-authored obligations resolve deterministically against exact
allowlisted command contracts.

## 3. Integration Tooling Fix

`scripts/check-integrations.mjs` now:

- uses a portable file-URL conversion for the repository root;
- sets an explicit 16 MiB output bound for child processes;
- distinguishes spawn/output-buffer errors from nonzero command exits;
- reports output exhaustion as a bounded actionable error rather than a null
  process status; and
- exposes the bounded runner for focused tests without executing the complete
  integration suite on import.

The default remains bounded. The fix does not stream or retain unbounded
provider, command, or test output.

## 4. CLI Diagnostic Fix

`workflow-os validate` continues to render all validation diagnostics and the
actionable onboarding next step. When validation fails, it now returns the
stable `cli.validate.failed` summary without reattaching the already-rendered
diagnostic collection.

This preserves:

- complete human and JSON diagnostic output;
- nonzero validation exit behavior;
- the `workflow-os init-repo-governance` next action; and
- stable error classification.

It removes only duplicate terminal rendering.

## 5. Tests

Focused integration-helper tests prove:

- successful bounded child-process execution;
- actionable nonzero-exit output;
- explicit output-exhaustion classification; and
- absence of the previous `status null` failure shape.

The CLI regression counts `loader.manifest_missing` across stdout and stderr
and requires exactly one occurrence.

The real integration contract gate passes under:

- Node.js `24.18.0`; and
- Node.js `20.19.5`, the repository's maintained local toolchain.

## 6. Scope Explicitly Not Changed

This fix does not add or change:

- workflow execution semantics;
- policy or approval behavior;
- proportional-governance selection or enforcement;
- local-check declaration or resolver semantics;
- evidence, report, artifact, or SideEffect behavior;
- provider calls or mutations;
- schema or SDK contracts;
- hosted behavior;
- writes; or
- release posture.

## 7. Governed Execution

- workflow: `dg/implement`
- run: `run-1784991314909851000-2`
- approval: `approval/run-1784991314909851000-2/implementation-approved`
- presentation: `presentation/02cbfafce85a458f`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- phase-close presentation telemetry:
  `approval_presentation_enforcement: proof_record_read_error`; the helper
  disclosed that proof records could not be read without expanding disclosure,
  so this report does not claim a successful close-time proof read
- kernel boundary: governance coordination only; edits and validation ran
  outside the kernel

## 8. Validation

The complete repository validation gate passed:

- Node 24 integration-helper tests;
- Node 20 integration-helper tests;
- the missing-manifest CLI regression;
- `npm run check:integrations` under Node 24; and
- `npm run check:integrations` under Node 20;
- `npm run check`;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`; and
- `cargo test --workspace`.

## 9. Remaining Limitations

- Node versions newer than the CI-pinned Node 20 line remain supported by the
  package contract but are not part of the required CI matrix.
- Child-process output remains intentionally bounded; a command that exceeds
  16 MiB fails actionably rather than streaming arbitrary output.
- Workflow OS remains a preview local governance kernel, not a turnkey agent
  runtime or hosted automation system.
- Quiet-success runtime behavior remains limited to separately reviewed,
  explicit paths.

## 10. Recommended Next Phase

Complete maintainer review of this focused fix, then return directly to the
accepted **canonical local-check declaration-set record and pure resolver**
phase.

Do not use these evaluator fixes as a reason to add a new planning cycle or
delay the runtime foundations required for trustworthy quiet success.
