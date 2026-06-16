# Self-Governance Dogfood Multi-Step Conversion Report

## 1. Executive Summary

The self-governance dogfood project has been converted from a single approval-gated placeholder step into a small sequential multi-step placeholder workflow.

The conversion exercises the hardened local multi-step executor while preserving the project boundary: Workflow OS governs the run, and Codex or a human still performs repository edits and validation commands outside the kernel.

## 2. Scope Completed

- Converted `dogfood/workflow-os-self-governance` to a five-step sequential local workflow:
  - `scope-requested`
  - `planning-approved`
  - `implementation-handoff`
  - `validation-disclosure`
  - `review-and-report-posture`
- Scoped human approval to `planning-approved`.
- Kept the placeholder skill local, deterministic, low-sensitivity, and read-only.
- Updated dogfood README and roadmap documentation.
- Added CLI integration tests against the real dogfood project with isolated temporary state.

## 3. Scope Explicitly Not Completed

- No runtime code changes.
- No real build-command execution.
- No arbitrary shell execution.
- No automatic Codex control through the kernel.
- No automatic runtime report generation.
- No automatic report artifact writing.
- No report CLI rendering.
- No workflow schema changes.
- No examples outside the dogfood project.
- No branching execution.
- No parallel or DAG execution.
- No nested harness runtime behavior.
- No side-effect boundary model.
- No write behavior.
- No approval evidence attachment.
- No command-output evidence attachment.
- No reasoning lineage or claim graph implementation.
- No hosted or distributed runtime behavior.
- No release posture change.

## 4. Dogfood Workflow Summary

The converted workflow remains `dg/d` for continuity.

The first step records the non-secret task scope. The second step requires human approval before implementation proceeds. The remaining steps record that implementation, validation, and final review/report posture remain Codex- or human-executed outside the kernel.

Every non-final step uses sequential continuation. The final step preserves the existing terminal behavior shape.

## 5. Governance Boundary Summary

The workflow remains Level 2 and local.

The kernel governs:

- project validation;
- run identity;
- event history;
- step scheduling;
- policy decisions;
- approval pause/resume;
- downstream placeholder checkpoint execution;
- audit and observability events.

The kernel does not perform implementation edits, run Rust/npm checks, execute arbitrary commands, call providers, or write report artifacts automatically.

## 6. Test Coverage Summary

Added CLI tests verify:

- the real dogfood project validates;
- the dogfood run pauses at `planning-approved`;
- only `scope-requested` is invoked before approval;
- approval grant completes all five steps in order;
- approval denial stops before downstream steps;
- isolated temporary state is used for dogfood test runs.

## 7. Commands Run And Results

- `cargo test -p workflow-cli --test cli dogfood_multi_step` - pass: 3 tests.
- `cargo fmt --all --check` - pass.
- `cargo clippy --workspace --all-targets -- -D warnings` - pass.
- `cargo test --workspace` - pass.
- `npm run check:docs` - pass.

## 8. Remaining Known Limitations

- The dogfood workflow still uses deterministic placeholder local skill behavior.
- Validation/check commands still run outside the kernel unless separately scoped.
- No automatic reports or report artifacts are generated.
- No typed handoffs are produced by the dogfood workflow yet.
- Partial-run restart between dogfood steps is not separately tested beyond existing local executor replay/idempotency coverage.

## 9. Recommended Next Phase

Recommended next phase: self-governance dogfood multi-step conversion review.

The review should verify the converted dogfood workflow, approval boundary, CLI test coverage, documentation honesty, and preservation of the Codex/human execution boundary before any real local check execution or typed handoff integration is considered.
