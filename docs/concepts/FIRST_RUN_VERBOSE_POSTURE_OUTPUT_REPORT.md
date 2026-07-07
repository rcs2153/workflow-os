# First-Run Verbose Posture Output Report

## 1. Executive Summary

The first-run verbose posture output slice is implemented.

`workflow-os first-run` now keeps default human output focused on the high-signal operator summary and optional mock approval/audit demo guidance. The full bounded posture matrix remains available through `workflow-os first-run --verbose`, while `--json first-run` continues to emit the detailed preview JSON.

This is a real-repo onboarding UX improvement only. It does not change validation, runtime execution, report generation, workflow generation, local check behavior, provider behavior, writes, schemas, examples, hosted behavior, or release posture.

## 2. Scope Completed

- Added command-local `workflow-os first-run --verbose`.
- Kept default `workflow-os first-run` concise by hiding the detailed posture matrix.
- Preserved the full detailed posture matrix in verbose human output.
- Preserved detailed preview JSON output for `--json first-run`.
- Updated CLI help text.
- Updated first-run CLI documentation.
- Updated the real-repo onboarding UX plan and roadmap status.
- Added focused CLI tests for concise default output and verbose detailed output.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- command execution;
- local check execution;
- source-content inspection;
- provider calls;
- automatic workflow generation;
- workflow registration or promotion;
- runtime state writes from `first-run`;
- report artifacts;
- schema changes;
- examples;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- write-capable adapters;
- release posture changes.

## 4. CLI Behavior Summary

Default human output now leads with:

- first-run status;
- what happened;
- what was not done;
- `what_matters_now`;
- the recommended review/setup action;
- optional approval/audit demo command;
- a pointer to `workflow-os first-run --verbose`.

Verbose human output appends the existing detailed posture matrix, including safe repo metadata, report section posture, ownership/escalation findings, spec-field coverage findings, workflow discovery recommendations, and bounded next-step text.

JSON output remains unchanged in posture: it keeps detailed bounded fields for machine readers and remains preview JSON.

## 5. Privacy And Redaction Summary

The change moves detailed bounded fields behind `--verbose` in human output, but does not add new data sources.

The command still does not print:

- raw source contents;
- raw package script command bodies;
- raw dependency values;
- command output;
- parser payloads;
- provider payloads;
- environment values;
- credentials;
- token-like strings;
- raw owner, maintainer, or escalation-contact values.

## 6. Test Coverage Summary

Focused test coverage now verifies:

- default `workflow-os first-run` includes the concise summary;
- default output points users to `workflow-os first-run --verbose`;
- default output omits the detailed posture matrix;
- verbose output includes the full posture matrix;
- safe package metadata details remain available in verbose output;
- detailed ownership/escalation posture checks remain available in verbose output;
- preview JSON remains detailed and bounded;
- `first-run` still does not create runtime state.

Existing first-run, scaffold, validation, runtime, and docs tests remain in scope for full validation.

## 7. Commands Run And Results

Governed dogfood:

- `npm run dogfood:benchmark -- phase-start --phase implementation ...`: passed.
- `workflow-os ... approve run-1783345160759394000-2 approval/run-1783345160759394000-2/implementation-approved ...`: passed.

Local validation:

- `cargo test -p workflow-cli --test cli first_run -- --nocapture`: passed.
- `cargo fmt --all --check`: passed after applying `cargo fmt --all`.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Remaining Known Limitations

- The CLI still only detects a bounded first slice of repository metadata.
- Rust, Python, Go, and richer GitHub Actions metadata detection remain future slices.
- `first-run` recommendations remain review-only and do not create workflows.
- The optional first-run workflow remains a mock approval/audit demo when run with mock local skills.
- `first-run` does not execute package scripts or local checks.

## 9. Recommended Next Phase

Recommended next phase: real-repo onboarding UX PR merge readiness or richer safe metadata detection planning.

The default output is now less dense, so the current PR is closer to the external real-repo onboarding feedback. If this PR is merged, the next implementation should either add another bounded ecosystem detector or return to the broader roadmap lane after maintainer review.
