# Governed Workflow Authoring Draft Archive Command Implementation Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds a narrow, explicit, local `archive-draft` command that
matches the approved archive-command plan. It preserves the authoring boundary:
eligible promoted/superseded drafts may be moved to
`workflows/drafts/archive/`, while active candidates, destructive deletion,
automatic cleanup, runtime state, provider calls, schemas, examples, writes, and
release posture changes remain out of scope.

## 2. Scope Verification

The phase stayed within the approved local archive-command scope.

No accidental implementation was found for:

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

## 3. CLI Surface Assessment

The new command shape is appropriate:

```sh
workflow-os author workflow archive-draft \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-reason> \
  [--dry-run]
```

The command requires explicit draft path, reviewer, and bounded reason input.
`--dry-run` previews the archive result without moving files. The non-dry-run
path moves exactly one eligible draft file.

The parser rejects missing required values. Reviewer parsing reuses `ActorId`
validation. Reason validation rejects empty, overlong, and secret-like values
without echoing the rejected value.

## 4. Eligibility Assessment

The implementation correctly reuses existing draft-status classification before
archiving. It permits only:

- `promoted_preserved`;
- `superseded_by_active`.

It rejects `active_candidate` drafts by default and emits bounded
`archive_blocked` output before returning
`cli.workflow_authoring.archive_draft_not_eligible`.

This is the right safety posture. It prevents a draft that is still a plausible
promotion candidate from being quietly moved out of the active authoring queue.

## 5. Archive Surface Assessment

The archive surface is:

```text
workflows/drafts/archive/<name>.workflow.yml
```

The path is derived from the validated draft path and filename, not from an
untrusted archive destination argument. The command refuses archive destination
overwrite via `cli.workflow_authoring.archive_destination_exists`.

The active workflow file is not rewritten. Tests verify that active workflow
content remains unchanged after archiving a promoted-preserved draft.

## 6. Validation And Error Handling Assessment

Validation behavior is deterministic and fail-closed:

- project loading/validation occurs before archive assessment;
- draft path validation is reused;
- draft parse failures remain bounded through existing loader paths;
- ineligible draft state fails with a stable code;
- archive destination overwrite fails with a stable code;
- secret-like or unbounded reason input fails with a stable code;
- post-archive project validation runs after the move.

Error messages avoid raw draft YAML, raw reason text, parser payloads, provider
payloads, command output, and secret-like values.

## 7. Output And Privacy Assessment

Text and JSON output are bounded. The command prints:

- status;
- prior draft status;
- relative draft and archive paths;
- active workflow path;
- candidate workflow id;
- draft content hash;
- matching active workflow path status;
- reviewer actor id;
- reason status as `provided`;
- boundary booleans.

The command does not print raw reason text. It does not copy raw draft contents,
active workflow YAML, source contents, manifest bodies, package scripts, CI logs,
command output, provider payloads, parser payloads, environment values,
credentials, authorization headers, private keys, or token-like strings.

Printing reviewer actor id is acceptable because it is validated identity
metadata and was explicitly provided for the archive action.

## 8. Non-Runtime Boundary Assessment

The command remains local authoring hygiene. It does not:

- create runtime state;
- append workflow events;
- emit audit events;
- emit observability events;
- execute commands;
- run local checks;
- call providers;
- write report artifacts;
- register workflows in a catalog;
- create or consume persisted steward approvals.

This accurately preserves the line between authoring file management and runtime
workflow execution.

## 9. Test Quality Assessment

The added tests cover the important first slice:

- dry-run preview without mutation;
- promoted-preserved archive success;
- superseded-by-active archive success;
- active candidate rejection;
- archive overwrite rejection;
- bounded JSON output;
- secret-like reason rejection without leakage;
- active workflow file preservation;
- no runtime state creation;
- no raw reason or draft payload in output.

Existing tests continue to cover neighboring authoring behavior: dry-run,
inactive draft output, preflight, steward-review preview, active promotion, and
draft-status inspection.

Non-blocking coverage follow-up: add direct archive-specific tests for missing
draft and unsafe draft path. The underlying path and draft-loading validation is
already reused and covered in neighboring commands, so this is not a blocker.

## 10. Documentation Review

Documentation now states that:

- `archive-draft` is implemented;
- the implemented scope is limited to explicit local archive of eligible
  promoted/superseded drafts;
- active workflow files remain untouched;
- deletion is not implemented;
- automatic cleanup is not implemented;
- persisted steward approvals are not implemented;
- workflow catalog state is not implemented;
- runtime state is not created;
- schemas and examples are not added;
- provider calls and writes remain unsupported;
- release posture is unchanged.

The implementation report captures dogfood run id, approval id, phase close
event summary, validation results, and remaining limitations.

## 11. Validation

Implementation validation ran and passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`

Review phase validation reran and passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `npm run dogfood:benchmark -- phase-close run-1783484083752432000-2 --phase review`

Dogfood review summary:

- workflow: `dg/review`
- run: `run-1783484083752432000-2`
- approval: `approval/run-1783484083752432000-2/review-scope-approved`
- approval actor: `user/delegated-maintainer`
- phase close status: completed
- event summary:
  `ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8,
  RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1,
  SkillInvocationRequested:6, SkillInvocationStarted:6,
  SkillInvocationSucceeded:6, StepScheduled:6`

## 12. Blockers

No blockers.

## 13. Non-Blocking Follow-Ups

- Add direct archive-specific missing-draft and unsafe-path tests.
- Consider persisted archive metadata only after workflow catalog or steward
  approval persistence is planned.
- Keep deletion/abandon behavior separate and explicitly planned before any
  destructive cleanup command exists.
- Consider whether archive actions should later emit authoring audit records,
  but only after the authoring audit model is planned.

## 14. Recommended Next Phase

Recommended next phase: workflow authoring catalog and persisted stewardship
planning.

The archive command resolves the immediate local hygiene gap for promoted and
superseded drafts. The next load-bearing gap is not deletion; it is durable
authoring stewardship. Before Workflow OS claims richer stale-draft detection,
review provenance, or team-scale workflow catalog behavior, it needs a planned
model for catalog identity, persisted steward decisions, archive metadata, and
how those records relate to active workflow files.
