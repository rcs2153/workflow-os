# Governed Workflow Authoring Draft Archive Command Implementation Report

## 1. Executive Summary

This phase implemented the first explicit local draft archive command for
governed workflow authoring.

`workflow-os author workflow archive-draft` can now preview or move one eligible
inactive workflow draft from `workflows/drafts/` into
`workflows/drafts/archive/`. The command is intentionally narrow: it archives
only already-promoted or superseded drafts, refuses active candidates, refuses
archive overwrite, leaves active workflow files untouched, and creates no
runtime state.

## 2. Scope Completed

- Added `workflow-os author workflow archive-draft`.
- Added required `--draft`, `--reviewer`, and bounded `--reason` inputs.
- Added optional `--dry-run` preview mode.
- Reused existing authoring path validation and draft-status classification.
- Allowed archive for `promoted_preserved` and `superseded_by_active` drafts.
- Rejected active candidates by default.
- Rejected archive destination overwrite.
- Moved eligible drafts to `workflows/drafts/archive/<name>.workflow.yml`.
- Validated the project after archive.
- Added bounded text and JSON output.
- Added focused CLI tests.
- Updated roadmap and planning docs.

## 3. Scope Explicitly Not Completed

This phase did not implement:

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

## 4. CLI API Summary

Preview:

```sh
workflow-os author workflow archive-draft \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-reason> \
  --dry-run
```

Archive:

```sh
workflow-os author workflow archive-draft \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-reason>
```

The command emits a bounded result with prior draft status, draft path, archive
path, active workflow path, candidate workflow id, draft content hash, reviewer,
reason status, archive mutation flags, and explicit non-runtime boundary flags.
The raw reason text is not printed.

## 5. Eligibility And Status Behavior

Allowed first targets:

- `promoted_preserved`;
- `superseded_by_active`.

Rejected by default:

- `active_candidate`;
- missing draft;
- unsafe draft path;
- invalid draft YAML;
- archive destination overwrite;
- secret-like or unbounded reason text.

The command derives eligibility from the same draft-status logic used by
`workflow-os author workflow draft-status`.

## 6. Archive Surface

Eligible drafts are moved to:

```text
workflows/drafts/archive/<name>.workflow.yml
```

The active workflow file remains in `workflows/<name>.workflow.yml` when present
and is not rewritten by the archive command.

## 7. Safety And Non-Mutation Boundary

The command:

- validates the project before archive;
- validates the draft path;
- refuses ambiguous or ineligible states;
- refuses overwrite of an existing archive file;
- moves exactly one draft file when not in dry-run mode;
- validates the project after archive;
- does not write active workflow files;
- does not promote or register workflows;
- does not create runtime state;
- does not append workflow events;
- does not execute commands;
- does not call providers;
- does not write report artifacts.

## 8. Privacy And Redaction Summary

The command output is bounded. It does not copy raw draft YAML, active workflow
YAML, source contents, manifest bodies, package scripts, dependency values,
lockfiles, CI logs, command output, provider payloads, parser payloads, absolute
private paths, environment values, credentials, authorization headers, private
keys, token-like strings, reviewer reason text, or existing agent instruction
bodies.

Secret-like reason text fails closed with a stable error code and is not echoed
to stdout or stderr.

## 9. Tests Added

Added focused CLI tests covering:

- dry-run preview without mutation;
- archiving a promoted-preserved draft;
- archiving a superseded-by-active draft;
- rejecting an active candidate without mutation;
- rejecting archive destination overwrite;
- bounded JSON output;
- secret-like reason rejection without leakage.

Existing authoring tests continue to cover preflight, steward-review, promotion,
and draft-status behavior.

## 10. Commands Run And Results

- `cargo fmt --all` passed.
- `cargo test -p workflow-cli --test cli author_workflow_archive -- --nocapture`
  passed with 7 tests.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.
- `npm run dogfood:benchmark -- phase-close run-1783482533102085000-2 --phase implementation`
  passed.

## 11. Dogfood Summary

This implementation ran under the governed dogfood implementation workflow.

- workflow: `dg/implement`
- run: `run-1783482533102085000-2`
- approval: `approval/run-1783482533102085000-2/implementation-approved`
- approval actor: `user/delegated-maintainer`
- status: approved before implementation work continued
- phase close status: completed
- event summary:
  `ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8,
  RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1,
  SkillInvocationRequested:6, SkillInvocationStarted:6,
  SkillInvocationSucceeded:6, StepScheduled:6`

Out-of-kernel work consisted of repository edits, shell validation commands, and
documentation updates performed by the executor under the governed phase
boundary. No skipped required checks remain.

## 12. Remaining Known Limitations

- Archive command review has not been performed yet.
- No persisted archive metadata exists.
- No persisted steward approval is consumed or linked.
- No automatic cleanup after promotion exists.
- No deletion or abandon command exists.
- No workflow catalog state exists beyond loader-visible file placement.
- Runtime event/audit emission for authoring archive actions remains deferred.

## 13. Recommended Next Phase

Proceed to **governed workflow authoring draft archive command implementation
review**.

The review should verify archive eligibility, output bounds, non-mutating active
workflow behavior, overwrite refusal, post-archive validation, privacy posture,
and continued exclusion of deletion, automatic cleanup, runtime state, schemas,
examples, provider calls, writes, and release posture changes.
