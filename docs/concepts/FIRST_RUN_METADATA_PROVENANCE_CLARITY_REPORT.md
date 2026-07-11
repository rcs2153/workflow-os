# First-Run Metadata Provenance Clarity Report

## 1. Executive Summary

This phase clarifies `workflow-os first-run` safe metadata provenance for
repositories that have just been scaffolded by Workflow OS.

Previously, the generated `tests/first-run-governance.test.yml` scaffold could
cause first-run output to report `test_dirs: tests`, which could read like
detected user repository test metadata. First-run now reports scaffold-only
Workflow OS test directories under `workflow_os_scaffold_dirs` instead.

## 2. Scope Completed

- Added bounded scaffold-only directory detection for the generated
  `tests/first-run-governance.test.yml` file.
- Excluded scaffold-only `tests/` from conventional user repository
  `test_dirs`.
- Added `workflow_os_scaffold_dirs` to verbose first-run text output.
- Added `workflow_os_scaffold_dirs` to preview JSON first-run output.
- Preserved user repository `test/` and `tests/` detection when user test files
  are present.
- Added focused tests for scaffold-only provenance and existing user test
  metadata behavior.
- Updated first-run CLI docs and roadmap.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- raw source inspection;
- command execution;
- local check execution;
- provider calls;
- workflow generation;
- workflow registration;
- runtime state creation;
- report artifact writing;
- schema changes;
- examples;
- hosted or distributed behavior;
- write-capable adapters;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Behavior Added

For a freshly scaffolded repository where the only `tests/` entry is the
Workflow OS generated first-run test spec, verbose first-run output now shows:

```text
test_dirs: none
workflow_os_scaffold_dirs: tests
```

Preview JSON now includes:

```json
"conventional_test_dirs": [],
"workflow_os_scaffold_dirs": ["tests"]
```

If a repository already has user test files under `tests/`, first-run continues
to report:

```json
"conventional_test_dirs": ["tests"],
"workflow_os_scaffold_dirs": []
```

## 5. Safety And Privacy

The implementation checks bounded file names only. It does not read test file
contents, source contents, manifest bodies, command output, provider payloads,
environment values, credentials, or token-like values.

The new field is disclosure-only. It does not affect recommendations, execute
checks, create runtime state, or change workflow behavior.

## 6. Tests Added

Focused tests cover:

- scaffold-only `tests/` is reported as `workflow_os_scaffold_dirs`;
- scaffold-only `tests/` is not reported as conventional user `test_dirs`;
- preview JSON reports scaffold-only provenance;
- user-created `tests/` remains conventional test metadata;
- user-created `test/` remains conventional test metadata;
- existing metadata privacy tests still reject raw package script and payload
  leakage.

## 7. Dogfood Governance

- Workflow: `dg/implement`
- Run ID: `run-1783741071756629000-2`
- Approval ID: `approval/run-1783741071756629000-2/implementation-approved`
- Approval presentation ID: `presentation/fa4d6451220f056a`
- Approval presentation hash:
  `fa4d6451220f056afe5518f956ed43d7084048a7919de93ac29f23f78f55830e`
- Approval outcome: delegated maintainer approved.

## 8. Commands Run And Results

All commands passed:

```sh
cargo test -p workflow-cli --test cli first_run_separates_scaffold_only_test_dir_from_repo_metadata
cargo test -p workflow-cli --test cli first_run
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

## 9. Remaining Limitations

- First-run still reports bounded metadata only. It does not inspect arbitrary
  source contents.
- Scaffold provenance currently covers the first generated `tests/` directory
  ambiguity. Other future generated directories should be added only if they
  become safe metadata candidates.
- Preview JSON remains experimental through the preview CLI contract.

## 10. Recommended Next Phase

First-run metadata provenance clarity review.

The review should verify that scaffold-generated governance files are no longer
presented as user repository test metadata, that user test directories still
work, and that the implementation remains bounded, non-mutating, and
redaction-safe.
