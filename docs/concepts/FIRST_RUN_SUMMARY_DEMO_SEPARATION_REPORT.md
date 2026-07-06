# First-Run Summary And Demo Separation Report

## 1. Executive Summary

The first-run summary and optional demo separation slice is implemented.

`workflow-os first-run` now leads with a concise `what_matters_now` summary before the detailed posture matrix. The command also separates the recommended next action from the optional mock approval/audit demo so users understand that first-run is the real bounded repository posture analysis, while the generated mock workflow is a local demonstration of approval checkpoints and event history.

This remains a local onboarding UX improvement. It does not execute commands, inspect source contents, generate workflows, call providers, write artifacts, mutate external systems, add schemas, or change release posture.

## 2. Scope Completed

- Added a concise `what_matters_now` block to human `first-run` output.
- Changed the recommended next action to reviewing findings and assigning ownership/check obligations.
- Added `optional_approval_audit_demo` for the mock workflow command.
- Added an `optional_demo_note` clarifying that the mock run demonstrates approval and event history, not additional repository analysis.
- Added package-aware summary wording when safe TypeScript/package metadata is detected.
- Updated focused CLI tests for the new output.
- Updated CLI docs and roadmap posture.

## 3. Scope Explicitly Not Completed

- No source-content inspection.
- No package script execution.
- No local check execution.
- No provider calls.
- No automatic workflow generation.
- No workflow registration.
- No runtime state creation from `first-run`.
- No report artifact creation.
- No persistence.
- No external writes.
- No schema changes.
- No hosted/distributed behavior.
- No recursive agents, agent swarms, or Level 3/4 autonomy.
- No release posture changes.

## 4. Behavior Added

Human `first-run` output now starts with:

- what happened;
- what was not done;
- what matters now;
- recommended next action;
- optional approval/audit demo command;
- optional demo note.

The detailed posture output remains available immediately below the summary.

## 5. UX Boundary Summary

The output now makes the product boundary clearer:

- `workflow-os first-run` is the real bounded governance posture analysis.
- `workflow-os --mock-all-local-skills run local/first-run-governance` is an optional approval/audit demo.

This reduces the chance that users confuse a mock local skill run with additional repository analysis.

## 6. Privacy And Redaction

The summary uses bounded static text and existing metadata posture only. It does not print raw package script bodies, dependency values, source contents, owner values, escalation contacts, command output, provider payloads, environment values, credentials, or token-like strings.

## 7. Tests Added Or Updated

Focused CLI tests now assert:

- `what_matters_now` is present;
- the recommended next action is review/setup rather than the mock workflow;
- the mock workflow command is labeled as `optional_approval_audit_demo`;
- the mock demo note says it is not additional repository analysis;
- TypeScript/package metadata adds package-aware summary text.

Existing first-run tests continue to cover report-ready context, bounded metadata, non-leakage, no runtime state creation, and review-only recommendations.

## 8. Governed Dogfood Summary

- Workflow: `dg/implement`.
- Run ID: `run-1783343706531347000-2`.
- Approval ID: `approval/run-1783343706531347000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer after the complete approval handoff block was surfaced.

## 9. Commands Run And Results

- `npm run dogfood:benchmark -- phase-start --phase implementation ...`: passed.
- `workflow-os approve run-1783343706531347000-2 approval/run-1783343706531347000-2/implementation-approved ...`: passed.
- `cargo test -p workflow-cli --test cli first_run -- --nocapture`: passed.
- `cargo fmt --all --check`: passed after applying `cargo fmt --all`.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Remaining Known Limitations

- `first-run` human output is improved but still detailed.
- JSON output remains preview.
- The optional mock workflow still uses mock local skill behavior.
- More polished grouping or `--verbose` output remains future work.
- Broader ecosystem-specific recommendations remain future work.

## 11. Recommended Next Phase

Recommended next phase: first-run summary and demo separation review.

The review should verify that the output is clearer without weakening boundedness, privacy posture, or the distinction between real posture analysis and optional mock runtime demonstration.
