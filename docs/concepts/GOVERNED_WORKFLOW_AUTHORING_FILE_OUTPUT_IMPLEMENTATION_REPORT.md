# Governed Workflow Authoring File Output Implementation Report

## 1. Executive Summary

This phase implements the first explicit file-output path for governed workflow authoring:

```sh
workflow-os author workflow --from-recommendation <id> --output workflows/drafts/<name>.workflow.yml
```

The command writes one inactive draft workflow file for review. It keeps the existing dry-run path, supports dry-run preview for the output path, validates the output boundary, rejects unsafe paths, refuses overwrites, checks duplicate active workflow ids, and keeps generated drafts outside the active loader path.

This phase does not implement workflow registration, workflow promotion, command execution, provider calls, runtime state creation, schemas, examples, hosted behavior, write-capable adapters, or release posture changes.

## 2. Scope Completed

- Added `--output <path>` parsing for `workflow-os author workflow`.
- Preserved the existing `--dry-run` preview-only behavior.
- Added output dry-run mode for file-output previews.
- Added explicit inactive draft file writing under `workflows/drafts/`.
- Added output path validation for:
  - relative `workflows/drafts/<name>.workflow.yml` paths only;
  - no absolute paths;
  - no parent traversal;
  - no nested paths beyond the draft file;
  - `.workflow.yml` suffix;
  - safe, non-secret-like draft names.
- Added proposed workflow id derivation from the draft filename.
- Added duplicate active workflow id conflict checks before writing.
- Added no-overwrite behavior for existing draft files.
- Added bounded human output for file-output dry-run and write results.
- Added bounded JSON output for file-output dry-run and write results.
- Added focused CLI tests.
- Updated CLI, roadmap, and implementation planning docs.
- Added this implementation report.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- active workflow generation;
- workflow registration;
- workflow promotion or activation;
- workflow catalog storage;
- command execution;
- local check execution;
- provider calls;
- runtime state creation;
- approval decisions;
- report artifacts;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 4. CLI/API Summary

Existing preview behavior remains:

```sh
workflow-os author workflow --from-recommendation first_run.repo_implementation --dry-run
```

New output preview behavior:

```sh
workflow-os author workflow \
  --from-recommendation first_run.repo_implementation \
  --output workflows/drafts/repo-implementation.workflow.yml \
  --dry-run
```

New inactive file-output behavior:

```sh
workflow-os author workflow \
  --from-recommendation first_run.repo_implementation \
  --output workflows/drafts/repo-implementation.workflow.yml
```

The command still requires a valid Workflow OS project and a known bounded first-run recommendation id.

## 5. Output Path Safety

The output path must be exactly inside the draft boundary:

```text
workflows/drafts/<name>.workflow.yml
```

The implementation rejects absolute paths, traversal, deeper nested paths, non-UTF-8 paths, unexpected suffixes, unsafe filename characters, and secret-like filename content.

Errors use stable codes and do not echo unsafe output path values.

## 6. Draft Lifecycle And Inactive Behavior

Generated files are review-only drafts.

The implementation writes drafts under `workflows/drafts/`, which the current project loader does not load as active workflow specs. Draft content also includes inactive posture:

- authoring-obligation comments;
- `owner.lifecycle_status: experimental`;
- `disabled_by_default: true`;
- empty triggers;
- empty steps;
- tags marking the file as an inactive Workflow OS draft.

The implementation does not add a new `draft` lifecycle schema value. If a generated file is manually moved into the active workflow directory before promotion work exists, validation fails closed because required active workflow fields are incomplete.

## 7. Conflict And Non-Overwrite Behavior

The command fails closed when:

- the output path is unsafe;
- the output file already exists;
- the proposed workflow id conflicts with a loaded active workflow id;
- the recommendation id is missing, unknown, invalid, or secret-like;
- project validation fails;
- draft writing fails.

Existing files are never overwritten.

## 8. Privacy And Redaction Summary

Generated drafts use bounded Workflow OS vocabulary and recommendation identifiers only.

The implementation does not copy:

- raw source contents;
- manifest bodies;
- package script command bodies;
- dependency values;
- lockfile contents;
- CI logs;
- provider payloads;
- issue or pull request bodies;
- parser payloads;
- absolute private paths;
- environment values;
- credentials, authorization headers, private keys, or token-like strings;
- existing agent instruction bodies;
- current usernames, machine names, run ids, approval ids, or local temp paths.

## 9. Test Coverage Summary

Added focused tests for:

- output dry-run is non-mutating;
- output write creates an inactive draft;
- generated draft is outside active loader behavior;
- generated draft preserves non-registration/non-runtime posture;
- unsafe output paths fail closed without leakage;
- existing output files are not overwritten;
- duplicate active workflow ids are rejected;
- no runtime state is created;
- generated drafts do not contain run ids, approval ids, raw payloads, or secret-like local values.

Existing authoring dry-run tests continue to pass.

## 10. Governed Dogfood Summary

- Workflow: `dg/implement`.
- Run ID: `run-1783401888191424000-2`.
- Approval ID: `approval/run-1783401888191424000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer after the full approval handoff was emitted.
- Scope: add explicit inactive workflow authoring file output, tests, docs, and implementation report.
- Phase-close event summary: 39 events total; 1 approval; 0 retries; 0 escalations; terminal status `Completed`.
- Out-of-kernel work disclosed: file editing, formatting, tests, docs validation, diff check, git commit, PR creation, and PR merge remain agent/GitHub actions outside the kernel.

## 11. Validation Commands Run

- `cargo fmt --all --check`: passed.
- `cargo test -p workflow-cli --test cli author_workflow`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783401888191424000-2 --phase implementation`: passed.

## 12. Remaining Known Limitations

- No workflow promotion path exists.
- No workflow registration path exists.
- No catalog/store-backed workflow proposal storage exists.
- No owner/escalation input handling exists for authoring.
- No generated draft is executable without human/steward completion.
- Draft output and JSON output remain preview-stage.

## 13. Recommended Next Phase

Recommended next phase: governed workflow authoring file-output implementation review.

The implementation crosses the first repository-file mutation boundary for workflow authoring and should be reviewed before promotion, active workflow registration, catalog storage, or steward approval flows are considered.
