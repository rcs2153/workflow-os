# Governed Workflow Authoring Active Promotion Implementation Report

## 1. Executive Summary

The first governed workflow authoring active-promotion slice is implemented.

`workflow-os author workflow promote` can now promote one preflight-passing
inactive draft from `workflows/drafts/<name>.workflow.yml` into the active
`workflows/<name>.workflow.yml` surface. The command derives fresh preflight
context, validates the candidate as an active workflow before writing, requires
explicit same-process steward/delegated-maintainer input, refuses active-path
overwrites, writes exactly one active workflow file, preserves the draft, and
reloads project validation after the write.

This is a narrow local CLI mutation boundary. It does not implement automatic
promotion, persisted approval records, workflow catalog storage, runtime state,
workflow runs, command execution, provider calls, report artifacts, schemas,
examples, hosted behavior, writes, or release posture changes.

## 2. Scope Completed

- Added `workflow-os author workflow promote`.
- Added `--dry-run` support for active promotion.
- Required explicit `--draft`, `--reviewer`, and `--reason` inputs.
- Reused existing draft path validation and isolated draft loading.
- Reused existing promotion preflight assessment.
- Reused `review_workflow_draft_for_promotion` for same-process approval
  authorization.
- Added active output path derivation from the draft filename.
- Refused promotion when the active output path already exists.
- Validated the candidate in active-placement context before writing.
- Wrote one active workflow file with a temporary sibling file and rename.
- Preserved the inactive draft file.
- Reloaded and validated the project after write.
- Added bounded text and JSON output.
- Added focused CLI regression tests.
- Updated CLI and roadmap documentation.

## 3. Scope Explicitly Not Completed

- No automatic promotion.
- No promotion of multiple drafts.
- No persisted steward approval records.
- No workflow catalog persistence.
- No runtime state creation.
- No workflow run creation.
- No command execution.
- No local check execution.
- No provider calls.
- No report artifact creation.
- No workflow-declared steward configuration.
- No schema changes.
- No example updates.
- No hosted or distributed runtime behavior.
- No recursive agents or agent swarms.
- No Level 3/4 autonomy expansion.
- No write-capable adapters or provider mutation.
- No release posture changes.

## 4. CLI API Summary

Dry-run:

```sh
workflow-os author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason> \
  --dry-run
```

Promotion:

```sh
workflow-os author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason>
```

JSON output uses the existing global flag:

```sh
workflow-os --json author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason>
```

## 5. Promotion Boundary Summary

Promotion is explicit. The command does not infer promotion from preflight or
steward-review preview.

The command:

- validates the current Workflow OS project;
- validates the draft path;
- parses the inactive draft;
- recomputes the draft content hash;
- recomputes preflight blockers and warnings;
- validates the candidate as if it were active before writing;
- calls the existing steward-review helper with `ApprovedForPromotion`;
- refuses active output overwrite;
- writes only `workflows/<same-file-name>`;
- preserves `workflows/drafts/<same-file-name>`;
- reloads project validation after write.

## 6. Validation Boundary Summary

Promotion fails closed when:

- the project is invalid;
- the draft path is unsafe or missing;
- the draft cannot parse;
- preflight blockers are present;
- active-placement validation fails;
- reviewer input is invalid;
- reason input is missing, too long, or secret-like;
- the active output path already exists;
- the active file write fails;
- post-write project validation fails.

Errors use stable codes and do not echo raw draft content, reason text, unsafe
paths, provider payloads, command output, parser payloads, environment values,
credentials, or token-like values.

## 7. Redaction And Privacy Summary

Output includes bounded identifiers and status codes:

- draft path;
- active workflow path;
- candidate workflow id;
- draft content hash;
- preflight status;
- warning codes;
- reviewer id;
- validation status;
- non-runtime boundary flags.

Output does not copy draft YAML bodies, literal step input payloads, approval
reason text, raw source contents, command output, provider payloads, parser
payloads, environment values, credentials, or token-like values.

## 8. Test Coverage Summary

Focused CLI tests cover:

- dry-run promotion validates without writing;
- successful promotion writes one active workflow file;
- inactive draft preservation;
- resulting project validation;
- bounded JSON output;
- active output overwrite refusal before writing;
- preflight blockers fail before writing;
- secret-like reason rejection without leakage;
- no runtime state creation;
- no command/provider/report-artifact behavior.

Existing authoring preflight and steward-review tests continue to cover bounded
review and non-mutation behavior before promotion.

## 9. Commands Run And Results

- `cargo test -p workflow-cli --test cli author_workflow_promote` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783475396014231000-2 --phase implementation` - passed.

Governed dogfood trace:

- dogfood workflow id: `dg/implement`;
- run id: `run-1783475396014231000-2`;
- approval id: `approval/run-1783475396014231000-2/implementation-approved`;
- approval outcome: granted by delegated maintainer;
- final status: `Completed`;
- event summary: 39 events total, including 1 approval request, 1 approval
  grant, 8 policy decisions, 6 step schedules, 6 skill invocation request/start
  pairs, 6 skill successes, and run completion;
- out-of-kernel work: Codex performed repo edits, tests, docs updates, and this
  report after governed approval; no runtime state, provider calls, external
  writes, report artifacts, schemas, examples, or release posture changes were
  introduced by the implementation.

## 10. Remaining Known Limitations

- Promotion uses same-process reviewer/reason input; approval is not persisted.
- Promotion does not archive, delete, or supersede the inactive draft.
- Promotion does not create workflow catalog metadata beyond loader-visible file
  placement.
- Promotion does not run checks or execute the promoted workflow.
- Promotion does not attach report artifacts.
- Promotion does not enforce enterprise steward configuration, RBAC, IdP,
  quorum, or notifications.

## 11. Recommended Next Phase

Recommended next phase: active promotion implementation review.

Reason: active promotion is now the first authoring path that writes an active
workflow file. It should receive a maintainer review before expanding into
draft retirement, persisted steward approvals, workflow catalog state, or
runtime execution integration.
