# Governed Workflow Authoring Steward Review CLI Preview Plan

Status: Implemented in [Governed Workflow Authoring Steward Review CLI Preview Implementation Report](../concepts/GOVERNED_WORKFLOW_AUTHORING_STEWARD_REVIEW_CLI_PREVIEW_IMPLEMENTATION_REPORT.md). The CLI preview command is implemented as `workflow-os author workflow steward-review --draft ...`. Active promotion, workflow registration, file movement, persisted approval, runtime state, command execution, provider calls, report artifacts, schemas, examples, writes, hosted behavior, and release posture changes remain unimplemented.

This plan follows the accepted pure in-memory steward-review helper reviewed in [Governed Workflow Authoring Steward Review Helper Review](../concepts/GOVERNED_WORKFLOW_AUTHORING_STEWARD_REVIEW_HELPER_REVIEW.md). It defines the next bounded user-facing surface before active workflow promotion exists.

## 1. Executive Summary

Workflow OS can now write inactive workflow drafts, run deterministic promotion preflight, and review a preflight-passing unchanged draft through a pure in-memory core helper.

The next product boundary is a CLI preview surface that presents the steward-review card and decision result to a maintainer without moving files, registering workflows, persisting approval state, creating runtime state, running commands, or calling providers.

The implementation adds the CLI preview command. It does not implement active promotion.

## 2. Goals

- Expose the existing steward-review helper through a bounded CLI preview path.
- Keep steward review explicit and reviewable.
- Require a draft path under `workflows/drafts/`.
- Require a fresh preflight result or enough supplied preflight context to prove freshness.
- Present what approval allows and does not allow.
- Preserve the helper's non-mutation boundary.
- Preserve redaction-safe output.
- Support text and JSON output.
- Prepare for future active promotion without implementing it.

## 3. Non-Goals

Do not implement:

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

## 4. Candidate CLI Shape

Recommended command:

```sh
workflow-os author workflow steward-review \
  --draft workflows/drafts/<name>.workflow.yml \
  --decision approved-for-promotion \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason>
```

JSON output should be available through the existing top-level `--json` posture:

```sh
workflow-os --json author workflow steward-review \
  --draft workflows/drafts/<name>.workflow.yml \
  --decision needs-changes \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason>
```

The command name should avoid `promote` because this phase does not promote anything.

## 5. Required Inputs

Required:

- `--draft workflows/drafts/<name>.workflow.yml`;
- `--decision approved-for-promotion|denied|needs-changes|deferred`;
- `--reviewer <actor-id>`;
- `--reason <bounded-review-reason>`.

Derived by the CLI:

- candidate workflow id from parsed draft;
- current draft content hash;
- preflight status by reusing the existing preflight logic;
- preflight blocker and warning codes;
- active workflow conflict status;
- owner and escalation posture summary;
- policy posture summary;
- evidence/report posture summary;
- side-effect posture summary.

The first implementation should prefer deriving preflight in the same process rather than accepting raw preflight JSON from the caller. That keeps freshness tied to current draft content and avoids trusting pasted or stale agent summaries.

## 6. Output Policy

Text output should include:

- mode: `author_workflow_steward_review_preview`;
- status: `approved_for_future_promotion`, `not_authorized`, or `review_blocked`;
- draft path;
- candidate workflow id;
- current draft content hash;
- preflight status;
- blocker and warning codes;
- decision;
- reviewer;
- approval card summary;
- what approval allows;
- what approval does not allow;
- non-mutation flags;
- privacy boundary;
- next action.

JSON output should include the same bounded fields with stable snake-case keys.

Output must not copy raw draft YAML, raw source contents, package script bodies, CI logs, command output, provider payloads, parser payloads, environment values, credentials, authorization headers, private keys, token-like strings, or private absolute paths.

## 7. Validation Behavior

The CLI should fail closed when:

- draft path is missing, unsafe, or outside `workflows/drafts/`;
- draft file is missing;
- draft cannot be parsed;
- candidate workflow id is invalid;
- current preflight has blockers;
- active workflow conflict exists;
- decision is unknown;
- reviewer actor is invalid;
- review reason is missing, too long, or secret-like;
- derived review summaries are missing, too long, or secret-like;
- helper construction fails.

Errors must use stable codes and must not echo raw draft content, unsafe path values, review reason text, parser payloads, command output, provider payloads, or secret-like values.

## 8. Preflight Integration

The CLI should reuse the existing preflight implementation path rather than duplicating draft validation.

Recommended behavior:

1. Validate and load the current project.
2. Validate the draft path.
3. Parse the draft as an isolated candidate.
4. Compute current content hash.
5. Run the existing preflight checks in memory.
6. If preflight is blocked, report `review_blocked` and do not call the helper as an approval.
7. If preflight passes, construct `WorkflowDraftStewardReviewInput` from bounded derived context and explicit reviewer decision.
8. Call `review_workflow_draft_for_promotion`.
9. Print bounded preview output.

No runtime state, event, artifact, or approval record should be created.

## 9. Decision Semantics

Only `approved-for-promotion` may return an authorization status that permits a future separately implemented promotion step.

Other decisions must remain non-authorizing:

- `denied`;
- `needs-changes`;
- `deferred`.

The CLI should make clear that even an approved preview does not itself move files, register the workflow, persist approval, or approve future draft changes.

## 10. Non-Mutation Boundary

The command must report and tests must verify:

- `files_written: false`;
- `workflow_registered: false`;
- `workflow_promoted: false`;
- `approval_persisted: false`;
- `runtime_state_created: false`;
- `commands_executed: false`;
- `providers_called: false`.

The command must not create `.workflow-os/state`, append events, write artifacts, or modify draft files.

## 11. Privacy And Redaction

Use bounded codes and summaries only.

Do not print or serialize:

- raw draft YAML;
- raw source contents;
- raw package scripts;
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
- existing agent instruction bodies.

Review reason should be accepted only through the validated helper boundary and should not appear in Debug output.

## 12. Test Plan

Future implementation tests should cover:

- preflight-passing complete draft can produce steward-review preview;
- blocked preflight returns review-blocked output and does not authorize;
- stale or changed draft is reviewed against the current content hash;
- `approved-for-promotion` maps to future-promotion authorization;
- `denied`, `needs-changes`, and `deferred` do not authorize;
- unknown decision fails closed;
- invalid reviewer fails closed;
- missing or secret-like reason fails closed without leakage;
- unsafe draft path fails closed without leakage;
- duplicate active workflow id blocks review;
- text output is bounded and non-mutating;
- JSON output is bounded and non-mutating;
- no state directory, artifact, active workflow file, command execution, or provider call is created;
- existing author workflow output and preflight tests continue to pass;
- docs check passes.

## 13. Proposed Implementation Sequence

Recommended next implementation prompt:

1. Add CLI parsing for `workflow-os author workflow steward-review`.
2. Reuse existing preflight candidate loading and blocker/warning computation.
3. Derive bounded review summaries from current draft/preflight posture.
4. Call `review_workflow_draft_for_promotion`.
5. Print bounded text and JSON preview output.
6. Add focused CLI tests for success, blocked preflight, non-authorizing decisions, redaction, and non-mutation.
7. Update CLI docs, roadmap, and implementation report.
8. Review before planning active promotion.

## 14. Deferred Work

- Active promotion.
- File movement.
- Workflow registration.
- Persisted steward approvals.
- Runtime state and event emission.
- Workflow-declared steward configuration.
- Enterprise steward/admin controls.
- Report artifacts.
- Schemas and examples.
- Hosted/distributed behavior.
- Write-capable adapters.

## 15. Final Recommendation

Implement the steward-review CLI preview next, but keep it preview-only and non-mutating.

Do not implement active promotion, file movement, workflow registration, persisted approval records, runtime state, commands, providers, artifacts, schemas, examples, writes, hosted behavior, or release posture changes in that phase.
