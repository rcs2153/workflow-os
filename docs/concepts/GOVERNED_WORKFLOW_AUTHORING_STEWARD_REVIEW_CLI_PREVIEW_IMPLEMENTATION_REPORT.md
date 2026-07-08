# Governed Workflow Authoring Steward Review CLI Preview Implementation Report

## 1. Executive Summary

This phase implements the bounded steward-review CLI preview surface:

```sh
workflow-os author workflow steward-review \
  --draft workflows/drafts/<name>.workflow.yml \
  --decision approved-for-promotion \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason>
```

The command derives fresh preflight context for the current inactive draft, calls the existing in-memory steward-review helper, and prints a bounded review card and decision result.

The implementation remains preview-only and non-mutating. It does not promote drafts, register workflows, move files, persist steward approval records, create runtime state, execute commands, call providers, write report artifacts, add schemas, add examples, enable writes, hosted behavior, or change release posture.

## 2. Scope Completed

- Added CLI parsing for `workflow-os author workflow steward-review`.
- Added required `--draft`, `--decision`, `--reviewer`, and `--reason` inputs.
- Reused the existing inactive draft path validation and preflight candidate loading.
- Derived current draft content hash and preflight blocker/warning context in-process.
- Called `review_workflow_draft_for_promotion` for preflight-passing drafts.
- Printed bounded text output and bounded JSON output through top-level `--json`.
- Reported non-mutation flags in all preview paths.
- Added focused CLI tests for success, blocked preflight, non-authorizing decisions, JSON output, secret-like reason rejection, and non-mutation.
- Updated CLI documentation, roadmap, and implementation plans.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- active workflow promotion;
- workflow registration;
- file movement from `workflows/drafts/` to `workflows/`;
- persisted steward approval records;
- runtime state creation;
- command execution;
- local check execution;
- provider calls;
- report artifacts;
- workflow-declared steward configuration;
- schemas;
- examples;
- hosted or distributed runtime behavior;
- RBAC, IdP, admin UI, paging, or notifications;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 4. CLI API Summary

Implemented command:

```sh
workflow-os author workflow steward-review \
  --draft workflows/drafts/<name>.workflow.yml \
  --decision approved-for-promotion|denied|needs-changes|deferred \
  --reviewer <actor-id> \
  --reason <bounded-review-reason>
```

JSON output uses the existing global flag:

```sh
workflow-os --json author workflow steward-review \
  --draft workflows/drafts/<name>.workflow.yml \
  --decision needs-changes \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason>
```

## 5. Behavior Added

For preflight-passing drafts, the CLI prints:

- mode `author_workflow_steward_review_preview`;
- status `approved_for_future_promotion` or `not_authorized`;
- draft path;
- candidate workflow id;
- current draft content hash;
- preflight status;
- warning codes;
- steward decision;
- reviewer;
- bounded owner, escalation, policy, evidence/report, and side-effect summaries;
- approval allows / does-not-allow text;
- non-mutation flags;
- privacy boundary;
- next action.

For blocked drafts, the CLI prints `review_blocked`, blocker/warning codes, non-mutation flags, and exits with `cli.workflow_authoring.steward_review_blocked`.

## 6. Validation Boundary Summary

The CLI fails closed when:

- the draft path is missing, unsafe, or outside `workflows/drafts/`;
- the draft is missing or cannot be parsed;
- the project cannot be loaded or validated;
- preflight finds blockers;
- the decision is unknown;
- the reviewer actor id is invalid;
- the reason is missing, too long, or secret-like;
- the underlying steward-review helper rejects the derived input.

Errors use stable codes and do not echo raw draft content, raw paths beyond bounded relative draft paths, review reason text, parser payloads, command output, provider payloads, or secret-like values.

## 7. Privacy And Redaction Summary

The preview uses bounded codes and summaries only.

It does not print or serialize:

- raw draft YAML;
- raw source contents;
- raw package script bodies;
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

## 8. Test Coverage Summary

Focused tests cover:

- approved preflight-passing draft produces a steward-review preview;
- non-authorizing `needs-changes` decision remains preview-only;
- blocked preflight prevents steward review;
- JSON output is bounded and non-mutating;
- secret-like review reason is rejected without leakage;
- no active workflow file is created;
- no runtime state directory is created;
- raw draft input payload markers are not copied to output.

## 9. Governed Dogfood Summary

- Workflow: `dg/implement`.
- Run ID: `run-1783435842299146000-2`.
- Approval ID: `approval/run-1783435842299146000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer after the full approval handoff was emitted.
- Approved scope: add a non-mutating CLI surface that derives fresh preflight context, calls the existing steward-review helper, and prints bounded text/JSON review card and decision result.
- Strict non-goals: no active promotion, registration, file movement, persisted approvals, runtime state, commands, providers, artifacts, schemas, examples, writes, hosted behavior, or release posture changes.
- Phase-close event summary: 39 total events; ApprovalGranted:1,
  ApprovalRequested:1, PolicyDecisionRecorded:8, RunCompleted:1,
  RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1,
  SkillInvocationRequested:6, SkillInvocationStarted:6,
  SkillInvocationSucceeded:6, StepScheduled:6.
- Retries: 0.
- Escalations: 0.
- Out-of-kernel work disclosed: repository edits, shell validation,
  documentation updates, git/PR actions, and report writing were performed by
  Codex/human execution outside the kernel while the kernel governed phase
  scope, approval, and closeout.

## 10. Validation Commands

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783435842299146000-2 --phase implementation`:
  passed.

Focused validation already run:

- `cargo check -p workflow-cli`: passed.
- `cargo test -p workflow-cli --test cli author_workflow_steward_review`: passed.
- `cargo test -p workflow-cli --test cli author_workflow`: passed.

## 11. Remaining Known Limitations

- Steward-review preview does not persist approval.
- Approval cannot yet be consumed by a promotion command.
- Active promotion remains unimplemented.
- Workflow registration remains unimplemented.
- No report artifact is written for steward review.
- Workflow-declared steward configuration remains deferred.

## 12. Recommended Next Phase

Recommended next phase: governed workflow authoring steward-review CLI preview review.

The preview command crosses a user-facing review boundary and should receive a maintainer review before active promotion planning or implementation proceeds.
