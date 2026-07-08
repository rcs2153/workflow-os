# Governed Workflow Authoring Draft Status Implementation Report

## 1. Executive Summary

This phase implements the first draft cleanup/supersession slice: a
non-mutating `workflow-os author workflow draft-status --draft ...` command.

The command inspects one inactive draft under `workflows/drafts/`, derives its
candidate workflow id and content hash, checks for matching active workflow
placement, and reports whether the draft is still an active candidate, has been
promoted and preserved, or is superseded by an active workflow.

This phase does not implement archive, delete, metadata mutation, persisted
approval records, workflow catalog state, runtime state, command execution,
provider calls, schemas, examples, hosted behavior, writes, or release posture
changes.

## 2. Scope Completed

- Added `workflow-os author workflow draft-status --draft
  workflows/drafts/<name>.workflow.yml`.
- Reused the existing authoring draft path validation boundary.
- Reused existing draft loading, workflow parsing, and canonical YAML content
  hashing.
- Derived active workflow path from the draft filename.
- Reported bounded draft status fields:
  - draft path;
  - active workflow path;
  - candidate workflow id;
  - draft content hash;
  - matching active workflow path when present;
  - active workflow id conflict status;
  - inferred draft state;
  - recommended next action.
- Added human and preview JSON output.
- Added help text for the new command.
- Added focused CLI regression tests.
- Updated roadmap and authoring planning documentation.

## 3. Scope Explicitly Not Completed

- No cleanup command.
- No automatic cleanup after promotion.
- No archive command.
- No draft movement.
- No draft edit or metadata mutation.
- No draft deletion.
- No persisted steward approval records.
- No workflow catalog persistence.
- No runtime state creation.
- No workflow run creation.
- No command execution.
- No local check execution.
- No provider calls.
- No report artifacts.
- No schema changes.
- No examples.
- No hosted or distributed behavior.
- No external writes.
- No release posture changes.

## 4. CLI Behavior

Command:

```sh
workflow-os author workflow draft-status \
  --draft workflows/drafts/<name>.workflow.yml
```

The command reports:

- `active_candidate` when no matching active workflow is present;
- `promoted_preserved` when the derived active workflow path exists and has the
  same workflow id as the draft;
- `superseded_by_active` when the candidate workflow id or active path is
  already represented by an active workflow in a conflicting way.

The command is inspection-only. It does not promote, register, archive, delete,
move, edit, execute commands, call providers, or create runtime state.

## 5. Validation Boundary Summary

The implementation fails closed for:

- missing Workflow OS project;
- invalid project validation;
- unsafe draft paths;
- missing draft files;
- draft parse failures;
- active workflow read failures;
- active workflow parse failures.

Errors use stable codes and do not echo raw draft YAML, active workflow YAML,
private absolute paths, source contents, command output, provider payloads,
environment values, credentials, or token-like strings.

## 6. Privacy And Redaction Summary

Allowed output is bounded to relative paths that passed path validation,
workflow ids, content hashes, status codes, boundary booleans, and next-action
codes.

The command does not copy:

- raw draft YAML;
- raw active workflow YAML;
- raw source contents;
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
- existing agent instruction bodies;
- steward review reason text.

## 7. Test Coverage Summary

Added tests cover:

- unpromoted draft reports `active_candidate`;
- promoted and preserved draft reports `promoted_preserved`;
- JSON output is bounded and non-mutating;
- missing draft fails closed;
- unsafe/secret-like draft path fails closed without leakage;
- no active workflow file is written by status inspection;
- no runtime state is created by status inspection.

Existing authoring promotion, steward-review, preflight, WorkReport,
EvidenceReference, Diagnostic, validation, adapter telemetry, and runtime tests
remain covered by the workspace test suite.

## 8. Commands Run And Results

- `cargo test -p workflow-cli --test cli author_workflow_draft_status -- --nocapture` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 9. Governed Dogfood Summary

- Workflow: `dg/implement`.
- Run ID: `run-1783479440483726000-2`.
- Approval ID:
  `approval/run-1783479440483726000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer.
- Final status: `Completed`.
- Event summary: 39 events total, including 1 approval request, 1 approval
  grant, 8 policy decisions, 6 step schedules, 6 skill invocation
  request/start pairs, 6 skill successes, and run completion.
- Retries: 0.
- Escalations: 0.
- Approved scope: add non-mutating author workflow `draft-status` command,
  focused tests, docs, and phase report.
- Strict non-goals: no archive, delete, mutation, runtime state, commands,
  providers, schemas, examples, or writes.

## 10. Remaining Known Limitations

- No archive command exists.
- No delete command exists.
- No persisted approval consumption exists.
- No workflow catalog state exists.
- `stale_candidate` remains vocabulary/planning language until there is a
  persisted review, approval, or catalog state against which staleness can be
  measured.
- Status is derived from current loader-visible active workflow placement only.

## 11. Recommended Next Phase

Recommended next phase: draft status implementation review.

Reason: this command adds a new operator-visible authoring surface. It should
be reviewed before planning archive/delete behavior, persisted approval
consumption, or workflow catalog state.
