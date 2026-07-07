# Governed Workflow Authoring CLI Dry-Run Implementation Report

## 1. Executive Summary

This phase implements the first explicit governed workflow authoring CLI surface:

```sh
workflow-os author workflow --from-recommendation <id> --dry-run
```

The command previews inactive workflow authoring obligations for one existing first-run recommendation. It reuses the accepted inactive draft proposal helper and remains non-mutating.

It does not write workflow files, register workflows, promote workflows, execute commands, call providers, create runtime state, change schemas, add examples, enable hosted behavior, or enable writes.

## 2. Scope Completed

- Added `Command::AuthorWorkflow`.
- Added parser support for `workflow-os author workflow --from-recommendation <id> --dry-run`.
- Required explicit `--dry-run`.
- Required explicit `--from-recommendation <id>`.
- Reused existing first-run safe metadata and recommendation derivation.
- Reused existing inactive draft proposal helper.
- Added bounded human dry-run output.
- Added bounded preview JSON output behind global `--json`.
- Added help text for the new command.
- Added CLI docs.
- Added focused CLI tests for success, JSON, fail-closed inputs, non-leakage, and non-mutation posture.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- workflow file generation;
- repository file writes;
- workflow registration;
- workflow promotion or activation;
- workflow catalog storage;
- runtime state creation;
- local command execution;
- local check registration or execution;
- provider calls;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 4. CLI Behavior Summary

The command requires:

- an existing valid Workflow OS project;
- `--dry-run`;
- `--from-recommendation <id>`.

Human output includes:

- preview-only mode;
- source recommendation id, kind, target, and summary;
- inactive draft proposal status;
- proposed lifecycle status;
- proposed purpose code;
- required authoring decisions;
- validation expectations;
- missing required fields;
- explicit non-mutation booleans;
- draft non-goals;
- privacy boundary;
- next action.

Preview JSON includes the same bounded posture under `author_workflow_dry_run`.

## 5. Validation And Error Handling Summary

The command fails closed when:

- `--dry-run` is missing;
- `--from-recommendation <id>` is missing;
- the recommendation id is invalid or secret-like;
- the recommendation id is unknown;
- the project is missing or invalid;
- proposal construction fails.

Errors use stable Workflow OS error codes and do not echo unsafe recommendation ids.

## 6. Privacy And Redaction Summary

The implementation uses bounded safe metadata and static Workflow OS vocabulary.

It does not copy:

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
- existing agent instruction bodies.

## 7. Test Coverage Summary

Added focused tests for:

- missing `--dry-run` fails closed;
- missing recommendation id fails closed;
- known recommendation produces inactive dry-run preview;
- preview includes required authoring decisions and missing fields;
- preview states no files, workflow registration, commands, providers, or runtime state;
- JSON output is bounded;
- unknown recommendation id fails closed without echoing the id;
- secret-like recommendation id fails closed without leakage;
- no runtime state is created.

Focused authoring tests passed.

## 8. Governed Dogfood Summary

- Workflow: `dg/implement`.
- Run ID: `run-1783398589553952000-2`.
- Approval ID: `approval/run-1783398589553952000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer after the full approval handoff was emitted.
- Event summary: 39 events total; 1 approval; 0 retries; 0 escalations; terminal status `Completed`.
- Scope: add explicit non-mutating author workflow dry-run CLI path, tests, docs, and implementation report.
- Out-of-kernel work disclosed: file editing, formatting, tests, docs validation, diff check, git commit, PR creation, and PR merge remain agent/GitHub actions outside the kernel.

## 9. Validation Commands Run

- `cargo fmt --all --check`: passed.
- `cargo test -p workflow-cli --test cli author_workflow`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783398589553952000-2 --phase implementation`: passed.

## 10. Remaining Known Limitations

- The command is dry-run only.
- No file-writing path exists.
- No catalog conflict detection exists.
- No workflow promotion path exists.
- No owner/escalation input handling exists for authoring.
- Preview JSON remains experimental.

## 11. Recommended Next Phase

Recommended next phase: governed workflow authoring CLI dry-run implementation review.

The new command is user-facing and should be reviewed before any file-writing, conflict-checking, catalog, or promotion work.
