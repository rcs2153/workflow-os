# Governed Workflow Authoring File Output Implementation Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds the first explicit, inactive file-output path for governed workflow authoring while preserving the intended safety boundary. The new `workflow-os author workflow --from-recommendation <id> --output workflows/drafts/<name>.workflow.yml` path writes one review-only draft file, refuses unsafe paths and overwrites, checks duplicate loaded workflow ids, and avoids workflow registration, promotion, command execution, provider calls, runtime state, schemas, examples, hosted behavior, write-capable adapters, and release posture changes.

No blocker was found.

## 2. Scope Verification

The phase stayed within approved scope.

Confirmed in scope:

- explicit `--output` support for `author workflow`;
- inactive draft output under `workflows/drafts/`;
- output dry-run preview;
- path safety checks;
- duplicate active workflow id check;
- no-overwrite behavior;
- bounded draft rendering;
- focused CLI tests;
- CLI and roadmap documentation updates;
- implementation report.

No accidental implementation was found for:

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

## 3. CLI Boundary Assessment

The CLI boundary is appropriately narrow and explicit.

The existing dry-run command remains available:

```sh
workflow-os author workflow --from-recommendation <id> --dry-run
```

The new file-output command requires an explicit recommendation id and an explicit draft output path:

```sh
workflow-os author workflow --from-recommendation <id> --output workflows/drafts/<name>.workflow.yml
```

This is the right shape for the first repository-mutating authoring feature because it does not hide file creation behind recommendation detail or first-run output.

## 4. Inactive Draft Assessment

The implementation keeps generated drafts inactive.

The chosen boundary writes under `workflows/drafts/`, which the current loader does not load as active workflow specs. The generated YAML also includes inactive posture:

- explicit inactive draft comments;
- `owner.lifecycle_status: experimental`;
- `disabled_by_default: true`;
- `triggers: []`;
- `steps: []`;
- draft tags.

This is acceptable for the first file-output slice. It avoids adding a new lifecycle schema value and avoids relying only on comments for safety.

Non-blocking follow-up: promotion planning should decide whether Workflow OS needs a typed draft lifecycle/status, a separate proposal artifact format, or a loader-aware draft directory contract before active promotion exists.

## 5. Path Safety And Conflict Assessment

Path handling is appropriately conservative.

The implementation accepts only:

```text
workflows/drafts/<name>.workflow.yml
```

It rejects:

- absolute paths;
- traversal;
- deeper nested paths;
- non-UTF-8 filenames;
- unexpected suffixes;
- unsafe filename characters;
- secret-like filename content.

It also refuses existing files and rejects proposed workflow ids that conflict with loaded active workflow ids.

Non-blocking follow-up: future promotion or catalog work should add purpose/surface conflict detection, not only id conflict detection.

## 6. Privacy And Redaction Assessment

The implementation preserves the first-run safe metadata boundary.

Generated drafts use bounded Workflow OS vocabulary, recommendation ids, obligation labels, and static non-goals. They do not copy:

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
- run ids or approval ids.

Errors use stable codes and do not echo unsafe paths or secret-like values.

## 7. Runtime And State Assessment

The implementation does not mutate runtime state.

The file-output path writes only the requested inactive draft file. It does not create `.workflow-os/state`, append workflow events, run local checks, invoke providers, create report artifacts, or alter executor behavior.

The test suite covers the no-runtime-state posture for output dry-run, output write, unsafe path rejection, overwrite rejection, and duplicate-id rejection.

## 8. Test Quality Assessment

The test coverage is strong for the first file-output slice.

Reviewed coverage includes:

- output dry-run is non-mutating;
- output write creates an inactive draft;
- draft output includes inactive posture;
- draft output is not loaded by the current project loader;
- generated draft does not contain run ids or approval ids;
- unsafe output path fails closed without leaking secret-like path material;
- existing output file is not overwritten;
- duplicate active workflow id is rejected;
- no runtime state is created;
- existing dry-run authoring tests continue to pass.

No blocker-level test gap was found.

Non-blocking follow-up: add JSON-specific tests for output dry-run/write shape before treating the JSON preview as anything more than preview-stage.

## 9. Documentation Review

Docs now state:

- the inactive file-output path is implemented;
- file output is explicit and opt-in;
- generated drafts are review-only and inactive;
- drafts are written under `workflows/drafts/`;
- workflow registration is not implemented;
- workflow promotion is not implemented;
- command execution is not implemented;
- provider calls are not implemented;
- runtime state creation is not implemented;
- schemas, examples, hosted behavior, write-capable adapters, and release posture changes are not implemented.

Reviewed docs:

- `ROADMAP.md`;
- `docs/cli/author-workflow.md`;
- `docs/implementation-plans/governed-workflow-authoring-plan.md`;
- `docs/implementation-plans/governed-workflow-authoring-file-output-plan.md`;
- `docs/concepts/GOVERNED_WORKFLOW_AUTHORING_FILE_OUTPUT_IMPLEMENTATION_REPORT.md`.

## 10. Validation Reviewed

Implementation phase validation passed:

- `cargo fmt --all --check`;
- `cargo test -p workflow-cli --test cli author_workflow`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`;
- `npm run dogfood:benchmark -- phase-close run-1783401888191424000-2 --phase implementation`.

Review phase validation:

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783403658490693000-2 --phase review`: passed.

## 11. Governed Dogfood Summary

- Workflow: `dg/review`.
- Run ID: `run-1783403658490693000-2`.
- Approval ID: `approval/run-1783403658490693000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer after the full approval handoff was emitted.
- Scope: phase-level maintainer review of the governed workflow authoring inactive file-output implementation.
- Event summary: 39 events total; 1 approval; 0 retries; 0 escalations; terminal status `Completed`.
- Out-of-kernel work disclosed: documentation review, report writing, docs validation, diff check, git commit, PR creation, and PR merge remain agent/GitHub actions outside the kernel.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Plan promotion/steward review before any active workflow registration.
- Decide whether future drafts need a typed draft lifecycle/status, separate proposal artifact, or loader-aware draft contract.
- Add purpose/surface conflict checks before catalog or promotion work.
- Add JSON output shape tests before treating authoring JSON as stable.
- Keep generated draft content preview-stage until schema and promotion semantics are reviewed.

## 14. Recommended Next Phase

Recommended next phase: governed workflow authoring promotion and steward-review planning.

File output now creates reviewable inactive drafts. The next design question is how a human, delegated maintainer, or future steward should inspect, complete, validate, approve, and promote a draft without accidentally registering active governance or weakening safety boundaries.
