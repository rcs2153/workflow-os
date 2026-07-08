# Governed Workflow Authoring Draft Archive Command Plan

Status: Implemented for one explicit local `archive-draft` command. See
[Governed Workflow Authoring Draft Archive Command Implementation Report](../concepts/GOVERNED_WORKFLOW_AUTHORING_DRAFT_ARCHIVE_COMMAND_IMPLEMENTATION_REPORT.md).

This plan follows the accepted non-mutating draft-status implementation review in
[Governed Workflow Authoring Draft Status Implementation Review](../concepts/GOVERNED_WORKFLOW_AUTHORING_DRAFT_STATUS_IMPLEMENTATION_REVIEW.md)
and the broader cleanup/supersession plan in
[Governed Workflow Authoring Draft Cleanup And Supersession Plan](governed-workflow-authoring-draft-cleanup-plan.md).

## 1. Executive Summary

`workflow-os author workflow draft-status --draft ...` now gives maintainers a
safe, non-mutating way to inspect preserved inactive drafts after promotion.

The next implementation question is whether Workflow OS should support moving a
draft out of the active draft queue without deleting it. This plan defines the
first archive command boundary.

The intended next implementation is a narrow explicit local CLI command that
moves one already-promoted or superseded draft from `workflows/drafts/` into an
archive surface. It must preserve the active workflow file, refuse ambiguous
cleanup, and remain non-destructive.

The implemented slice adds explicit archive behavior for eligible
`promoted_preserved` and `superseded_by_active` drafts only. It does not add
deletion, automatic cleanup, runtime state, workflow registration, persisted
approval records, schemas, examples, provider calls, write-capable adapters, or
release posture changes.

## 2. Goals

- Provide a clear local hygiene path for preserved inactive drafts.
- Avoid stale drafts being mistaken for active promotion candidates.
- Preserve review context instead of deleting drafts.
- Keep archive behavior explicit and maintainer initiated.
- Refuse destructive cleanup by default.
- Preserve active workflow files and loader-visible active workflow state.
- Reuse existing draft path validation and draft-status semantics.
- Keep output bounded and redaction-safe.
- Preserve compatibility with future persisted approvals and workflow catalog
  state.

## 3. Non-Goals

Do not implement or authorize:

- draft deletion;
- automatic cleanup after promotion;
- automatic archiving;
- active workflow mutation;
- workflow registration changes;
- persisted steward approval records;
- workflow catalog state;
- runtime state creation;
- workflow run creation;
- command execution;
- local check execution;
- provider calls;
- report artifacts;
- schema changes;
- examples;
- hosted or distributed behavior;
- write-capable adapters or external writes;
- release posture changes.

## 4. Proposed CLI Shape

Recommended command:

```sh
workflow-os author workflow archive-draft \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-reason>
```

Recommended dry-run mode:

```sh
workflow-os author workflow archive-draft \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-reason> \
  --dry-run
```

The command should be explicit that it archives a draft file only. It should not
promote, delete, register, run, or validate workflow semantics beyond the
archive boundary.

## 5. Archive Surface

Recommended archive surface:

```text
workflows/drafts/archive/<name>.workflow.yml
```

Rationale:

- It keeps archived drafts near the authoring surface.
- It remains outside the active workflow loader surface.
- It avoids introducing `.workflow-os/authoring/` persistence semantics before a
  catalog/store model exists.
- It keeps archive behavior understandable to a local maintainer using Git.

Archive paths must remain relative to the project root and must be validated
with the same path safety rules as draft output/status paths.

## 6. Archive Eligibility

First implementation should archive only when the draft is already safely out of
the active authoring path.

Allowed first targets:

- `promoted_preserved`;
- `superseded_by_active`.

Rejected by default:

- `active_candidate`;
- missing draft;
- unsafe draft path;
- invalid draft YAML;
- invalid project;
- active workflow parse failure;
- ambiguous active workflow status.

`active_candidate` archiving should remain deferred unless a future explicit
`--allow-unpromoted` or `--abandon` design is planned and reviewed. Even then,
it should not delete the draft.

## 7. Required Behavior

The implemented archive command:

1. Validate the project before archive.
2. Validate the draft path.
3. Load and parse the draft without copying raw YAML into output.
4. Derive draft status using the same logic as `draft-status`.
5. Refuse archive unless status is eligible.
6. Derive an archive destination under `workflows/drafts/archive/`.
7. Refuse overwrite if the archive destination exists.
8. Supports dry-run preview without moving files.
9. Moves exactly one draft file when eligible and not in dry-run mode.
10. Leaves active workflow files untouched.
11. Prints bounded text output.
12. Supports bounded JSON output consistent with neighboring authoring
    commands.
13. Prints explicit boundary booleans.
14. Returns stable, non-leaking errors.

Preferred file operation: move one draft file to the archive destination. If
implementation uses copy-then-remove for portability, it must ensure the final
state is atomic enough for local CLI use and must fail closed on partial
failure.

## 8. Required Output

Bounded text output should include:

- command status code;
- original draft path;
- archive path;
- candidate workflow id;
- prior draft status;
- active workflow path if present;
- archive action result;
- reviewer actor id only if already validated and safe;
- reason status such as `provided`, not raw reason text;
- next action code;
- non-goal boundary booleans.

It must not print raw draft contents, raw reason text, raw parser payloads,
absolute private paths, command output, provider payloads, or secrets.

Suggested boundary booleans:

- `files_written`;
- `draft_archived`;
- `draft_deleted`;
- `active_workflow_file_written`;
- `workflow_registered`;
- `workflow_promoted`;
- `runtime_state_created`;
- `report_artifact_written`;
- `provider_called`;
- `commands_executed`.

For archive, `files_written` and `draft_archived` may be `true`; all other
mutation/side-effect booleans should remain `false`.

## 9. Reviewer And Reason Policy

Archive should require explicit `--reviewer` and `--reason` inputs, consistent
with active promotion posture.

The reason should be validated as bounded and non-secret-like. Output should
record only that a reason was supplied, not the raw reason text.

This does not create a persisted approval record. Until persisted approval or
catalog state exists, archive reviewer/reason data is local CLI accountability,
not durable enterprise authorization.

## 10. Privacy And Redaction

The command must not copy into output or errors:

- raw draft YAML;
- active workflow YAML;
- source contents;
- manifest bodies;
- package script bodies;
- dependency values;
- lockfile contents;
- CI logs;
- command output;
- provider payloads;
- parser payloads;
- absolute private paths;
- environment values;
- credentials;
- authorization headers;
- private keys;
- token-like strings;
- reviewer reason text;
- existing agent instruction bodies.

Allowed output remains bounded to relative paths, workflow ids, content hashes,
status codes, action codes, and boundary flags.

## 11. Error Handling

Future errors should use stable codes and fail closed.

Candidate error codes:

- `cli.workflow_authoring.archive_draft_missing`;
- `cli.workflow_authoring.archive_draft_unsafe_path`;
- `cli.workflow_authoring.archive_draft_not_eligible`;
- `cli.workflow_authoring.archive_destination_exists`;
- `cli.workflow_authoring.archive_reason_invalid`;
- `cli.workflow_authoring.archive_failed`;

Errors must not leak raw YAML, raw reason text, secret-like values, private
paths, parser payloads, command output, provider payloads, or credentials.

## 12. Test Plan

Future implementation tests should cover:

- dry-run reports intended archive without writing files;
- promoted-preserved draft archives to `workflows/drafts/archive/`;
- superseded draft archives to `workflows/drafts/archive/`;
- active workflow file remains untouched;
- active project validates after archive;
- unpromoted active candidate is rejected by default;
- archive destination overwrite is rejected;
- missing draft fails closed;
- unsafe path fails closed without leakage;
- secret-like reason fails closed without leakage;
- raw draft YAML is not copied to output;
- raw reason text is not copied to output;
- no runtime state is created;
- no report artifacts are written;
- no commands are executed;
- no providers are called;
- JSON output is bounded if added;
- docs check passes;
- existing authoring tests still pass.

## 13. Proposed Implementation Sequence

1. Add `archive-draft --dry-run` preview using existing draft-status
   classification.
2. Add focused dry-run tests and docs.
3. Review.
4. Add explicit archive mutation for eligible drafts only.
5. Add file operation tests and non-leakage tests.
6. Review.
7. Defer deletion, abandon/force behavior, persisted approvals, and catalog
   state.

The smallest implementation may combine steps 1 and 4 only if the mutation is
simple, isolated, and covered by tests. If uncertainty appears, keep dry-run
preview as its own phase.

## 14. Deferred Work

- Draft deletion.
- Archive metadata files.
- Supersession marker files.
- Persisted steward approval consumption.
- Workflow catalog state.
- Automatic archive after promotion.
- Archive of unpromoted drafts.
- Active workflow cleanup.
- Runtime event/audit emission for authoring archive actions.
- Schema exposure.
- Example updates.
- Hosted or distributed behavior.

## 15. Final Recommendation

The `archive-draft` implementation with dry-run plus one explicit eligible
archive mutation is complete.

The next recommended phase is a maintainer review of the implemented archive
command. Deletion, automatic cleanup, catalog persistence, runtime state,
schemas, examples, provider calls, write-capable adapters, and release posture
changes remain out of scope.
