# Archive Metadata Write Plan Report

## 1. Executive Summary

This phase planned the next local workflow catalog sidecar: opt-in archive
metadata writes after successful `workflow-os author workflow archive-draft`
moves.

The plan keeps the existing archive command boundary intact. Default
`archive-draft` remains unchanged. A future implementation may add
`--persist-archive-record` to write one validated `WorkflowArchiveRecord` only
after an eligible draft is moved and post-archive validation succeeds.

## 2. Scope Completed

- Created [Archive Metadata Write Plan](../implementation-plans/archive-metadata-write-plan.md).
- Defined the proposed CLI shape for `--persist-archive-record`.
- Defined catalog-root and stewardship-decision flag boundaries.
- Defined archive record construction rules using the existing
  `WorkflowArchiveRecord` model.
- Defined stewardship citation policy.
- Defined write timing and partial-integration behavior.
- Defined relationship to `catalog-status`.
- Defined privacy/redaction requirements.
- Defined future implementation tests.
- Updated [Workflow Catalog Persistence And Stewardship Integration Plan](../implementation-plans/workflow-catalog-persistence-plan.md).
- Updated [Roadmap](../../ROADMAP.md).

## 3. Scope Explicitly Not Completed

This planning phase did not implement:

- archive metadata write code;
- runtime workflow registration;
- automatic workflow generation;
- automatic promotion;
- automatic archiving;
- automatic cleanup;
- draft deletion;
- catalog repair;
- workflow catalog record update semantics;
- runtime state creation;
- workflow run creation;
- command execution or local check execution;
- provider calls;
- report artifacts;
- workflow schema changes;
- examples;
- hosted or distributed behavior;
- write-capable adapters or provider mutation;
- release posture changes.

## 4. Planned Implementation Boundary

The planned implementation is local and opt-in:

```sh
workflow-os author workflow archive-draft \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-reason> \
  --persist-archive-record \
  [--catalog-root .workflow-os/catalog] \
  [--stewardship-decision-id stewardship/<id>]
```

The default archive command remains unchanged.

## 5. Validation Boundary Summary

The plan requires future implementation to validate catalog persistence inputs
before draft movement. Supplied stewardship decisions must match workflow id,
draft path, and draft content hash before they can be cited.

If the archive move succeeds but the archive record write fails after
post-archive validation, the command should return a stable partial-integration
error. It should not attempt automatic rollback until recovery policy is planned
separately.

## 6. Privacy And Redaction Summary

Archive metadata remains reference-oriented. The plan forbids raw draft YAML,
active workflow YAML, source contents, package scripts, CI logs, command output,
provider payloads, parser payloads, absolute private paths, environment values,
credentials, authorization headers, private keys, token-like values, and raw
reviewer reason text in records, output, and errors.

## 7. Commands Run And Results

- `npm run check:docs`: passed.

## 8. Governed Dogfood Summary

- workflow: `dg/d`
- run: `run-1783536057396212000-2`
- approval: `approval/run-1783536057396212000-2/planning-approved`
- approval outcome: granted by delegated maintainer for archive metadata write
  planning
- phase scope: planning/docs only
- close summary: completed terminal run with 39 events, one approval, zero
  retries, and zero escalations
- event kinds: `RunCreated`, `RunValidated`, `RunStarted`, `StepScheduled`,
  `PolicyDecisionRecorded`, `ApprovalRequested`, `ApprovalGranted`,
  `RunResumed`, `SkillInvocationRequested`, `SkillInvocationStarted`,
  `SkillInvocationSucceeded`, and `RunCompleted`

Out-of-kernel work consisted of repository documentation edits and validation
commands performed by Codex under the governed planning boundary.

## 9. Remaining Known Limitations

- No archive metadata writes are implemented yet.
- No catalog record update/repair semantics are implemented.
- No strict stewardship requirement is implemented.
- No automatic archive cleanup or deletion is implemented.
- No runtime registration, schemas, examples, provider calls, hosted behavior,
  external writes, or release posture changes are implemented.

## 10. Recommended Next Phase

Recommended next phase: opt-in archive metadata write implementation.

The next implementation should add the narrow `--persist-archive-record` path
for `archive-draft`, with focused tests and no catalog repair, deletion,
runtime registration, schemas, examples, provider calls, hosted behavior,
external writes, or release posture changes.
