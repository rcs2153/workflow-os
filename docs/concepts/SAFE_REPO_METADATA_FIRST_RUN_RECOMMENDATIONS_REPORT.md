# Safe Repo Metadata First-Run Recommendations Report

## 1. Executive Summary

The safe repo metadata first-run recommendation slice is implemented.

`workflow-os first-run` now inspects bounded repository metadata and uses it to make more concrete review-only recommendations. The first implementation targets package/TypeScript posture because real-repository onboarding feedback showed that the generic first-run output was useful but not specific enough for normal repositories.

This remains a local report-ready context. It does not run commands, inspect source contents, generate workflows, persist reports, call providers, create runtime state, or enable writes.

## 2. Scope Completed

- Added bounded safe repository metadata detection to `workflow-os first-run`.
- Detects `package.json` presence.
- Detects package-manager posture from lockfile presence.
- Detects allowlisted package script keys: `build`, `test`, `lint`, `typecheck`, `format`, `prepare`, and `release`.
- Detects TypeScript posture from `typescript`, `ts-node`, `tsx`, and `tsconfig.json` markers.
- Detects `.github/workflows/*.yml` and `.github/workflows/*.yaml` count.
- Detects conventional source and test directory presence by directory name only.
- Detects README, license, contributing, and security policy presence.
- Emits bounded text and JSON metadata output.
- Adds TypeScript/package-specific review-only workflow recommendations.
- Adds focused tests for text output, JSON output, and no script/dependency payload copying.

## 3. Scope Explicitly Not Completed

- No source-content inspection.
- No README/body/code parsing.
- No command execution.
- No package script execution.
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

`workflow-os first-run` now prints a `safe_repo_metadata` section in human output. The section uses bounded labels, counts, and allowlisted keys only.

For package repositories, first-run can now add concrete review-only recommendations:

- `first_run.typescript_implementation`
- `first_run.package_validation_obligations`

These recommendations make the first-run posture more useful for a normal TypeScript package without pretending that Workflow OS executed package scripts or generated ready-to-run workflows.

## 5. Privacy And Redaction

The implementation does not copy package script command bodies, dependency values, source contents, command output, parser payloads, provider payloads, environment values, credentials, or token-like values.

Script handling is allowlist-based. The output reports script keys such as `test` or `build`, not the command bodies behind those keys.

Dependency handling is marker-based for TypeScript posture only. It reports bounded marker labels such as `dependency_typescript`; it does not print raw dependency names beyond the built-in marker vocabulary and does not print versions.

## 6. Tests Added

Focused tests cover:

- package metadata appears in human `first-run` output;
- package-manager posture is inferred from lockfile presence;
- common script keys are reported without copying command bodies;
- TypeScript markers are reported without copying dependency values;
- GitHub workflow count and conventional source/test directory presence are reported;
- TypeScript/package review-only recommendations are added;
- JSON output contains the bounded metadata model;
- secret-like script payloads and dependency markers are not copied;
- no runtime state is created.

Existing first-run tests continue to cover the generic no-metadata path.

## 7. Governed Dogfood Summary

- Workflow: `dg/implement`.
- Run ID: `run-1783317186098185000-2`.
- Approval ID: `approval/run-1783317186098185000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer after the complete approval handoff block was surfaced.
- Close summary: completed with 39 events, 1 approval, 0 retries, and 0 escalations.

## 8. Commands Run And Results

- `npm run dogfood:benchmark -- phase-start --phase implementation ...`: passed.
- `npm run dogfood:benchmark -- approve run-1783317186098185000-2 approval/run-1783317186098185000-2/implementation-approved --reason delegated-maintainer-approved-safe-metadata-first-run-slice`: passed.
- `cargo test -p workflow-cli --test cli first_run -- --nocapture`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783317186098185000-2 --phase implementation`: passed.

## 9. Remaining Known Limitations

- Rust, Python, Go, and GitHub Actions metadata are intentionally shallow in this slice.
- First-run output remains dense; summary polishing remains a follow-up.
- Recommendations remain review-only and are not active workflow definitions.
- Missing-citation records, report artifacts, and runtime execution remain out of scope.

## 10. Recommended Next Phase

Recommended next phase: safe repo metadata first-run recommendations review.

Review should verify privacy boundaries, recommendation usefulness, bounded JSON/text output, and preservation of the product boundary before broader ecosystem metadata or first-run summary polishing.
