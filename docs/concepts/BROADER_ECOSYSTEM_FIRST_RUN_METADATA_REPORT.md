# Broader Ecosystem First-Run Metadata Report

## 1. Executive Summary

The broader safe ecosystem metadata first-run slice is implemented.

`workflow-os first-run` now detects bounded Rust, Python, Go, and GitHub Actions posture in addition to the existing package/TypeScript metadata. It uses those labels to emit concrete review-only workflow recommendations without reading manifest bodies, executing commands, inspecting source contents, generating workflows, or registering local checks.

## 2. Scope Completed

- Added bounded Rust metadata labels:
  - `Cargo.toml` presence;
  - `Cargo.lock` presence.
- Added bounded Python metadata labels:
  - `pyproject.toml` presence;
  - allowlisted lock/requirements labels for `uv.lock`, `poetry.lock`, `Pipfile.lock`, and `requirements.txt`.
- Added bounded Go metadata labels:
  - `go.mod` presence;
  - `go.sum` presence.
- Added explicit GitHub Actions posture:
  - workflow file count;
  - `github_actions_detected` JSON flag.
- Added review-only workflow discovery recommendations for:
  - Rust implementation workflow;
  - Rust validation obligations;
  - Python implementation workflow;
  - Python validation obligations;
  - Go implementation workflow;
  - Go validation obligations;
  - GitHub Actions CI evidence obligations.
- Added concise human recommendation text for detected Rust, Python, Go, and GitHub Actions metadata.
- Added focused text and JSON tests for bounded output and non-leakage.

## 3. Scope Explicitly Not Completed

- No source-content inspection.
- No manifest body parsing for Rust, Python, Go, or GitHub Actions workflows.
- No raw lockfile contents.
- No command execution.
- No local check execution.
- No provider calls.
- No automatic workflow generation.
- No automatic workflow registration or promotion.
- No workflow schema changes.
- No examples.
- No hosted or distributed runtime behavior.
- No writes.
- No recursive agents, agent swarms, or Level 3/4 autonomy.
- No release posture changes.

## 4. Behavior Added

Verbose `first-run` output now includes bounded metadata labels such as:

- `cargo_toml`;
- `cargo_lock`;
- `pyproject_toml`;
- `python_lock_files`;
- `go_mod`;
- `go_sum`;
- `github_workflows`.

Preview JSON now includes equivalent bounded fields:

- `cargo_toml_present`;
- `cargo_lock_present`;
- `pyproject_toml_present`;
- `python_lock_files`;
- `go_mod_present`;
- `go_sum_present`;
- `github_workflow_count`;
- `github_actions_detected`.

When detected, those fields can add review-only first-run recommendations. These recommendations help users understand what governed workflows might be worth authoring next, but they do not activate checks, execute tools, or create workflow files.

## 5. Privacy And Redaction

The implementation reports only presence, counts, and allowlisted labels. It does not copy:

- Rust manifest contents;
- Python project metadata;
- Go module names;
- GitHub Actions workflow names or bodies;
- source file contents;
- test file contents;
- lockfile contents;
- command output;
- environment values;
- credentials;
- token-like values.

Focused tests include secret-like payloads in fixture manifests, lockfiles, workflow files, source files, and test files, then assert that first-run text and JSON do not copy those values.

## 6. Tests Added

Focused tests cover:

- Rust/Python/Go/GitHub Actions metadata labels in verbose first-run output;
- review-only workflow recommendations for each detected ecosystem;
- bounded JSON metadata output;
- no source, manifest, workflow, lockfile, or module payload copying;
- no runtime state creation.

Existing first-run tests continue to cover generic, package/TypeScript, JSON, ownership/escalation, field coverage, and no-artifact behavior.

## 7. Governed Dogfood Summary

- Workflow: `dg/implement`.
- Run ID: `run-1783388307384390000-2`.
- Approval ID: `approval/run-1783388307384390000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer after the complete approval handoff block was surfaced.
- Scope: broader safe first-run metadata detection for Rust, Python, Go, and GitHub Actions.
- Close summary: completed with 39 events, 1 approval, 0 retries, and 0 escalations.

## 8. Commands Run And Results

- `npm run dogfood:benchmark -- phase-start --phase implementation ...`: passed.
- `./target/debug/workflow-os ... approve run-1783388307384390000-2 ...`: passed.
- `cargo test -p workflow-cli --test cli first_run -- --nocapture`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Remaining Known Limitations

- Recommendations remain review-only.
- Metadata inspection remains shallow by design.
- No workflow generation or registration exists.
- No local check handlers are activated by this slice.
- Default human output remains concise; detailed metadata is in `--verbose` text and preview JSON.

## 10. Recommended Next Phase

Recommended next phase: broader ecosystem first-run metadata review.

The review should verify that detection remains bounded, recommendations are useful but not overclaimed, and no raw manifest/source/lockfile/workflow payloads are copied.
