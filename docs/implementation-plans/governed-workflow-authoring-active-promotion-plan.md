# Governed Workflow Authoring Active Promotion Plan

Status: Planned. Active promotion is not implemented. This plan follows the
accepted steward-review CLI preview in [Governed Workflow Authoring Steward
Review CLI Preview Review](../concepts/GOVERNED_WORKFLOW_AUTHORING_STEWARD_REVIEW_CLI_PREVIEW_REVIEW.md).

## 1. Executive Summary

Workflow OS can now:

- emit first-run workflow recommendations;
- preview authoring obligations;
- write one inactive draft under `workflows/drafts/`;
- run deterministic preflight against that draft;
- preview a bounded steward-review decision for a preflight-passing draft.

The next implementation question is how an inactive draft becomes an active
workflow spec that the current project loader can validate and the local runtime
can execute.

Active promotion must be an explicit repository mutation boundary. It should
move only one reviewed draft into the active `workflows/` surface after fresh
preflight and steward/delegated-maintainer approval context are evaluated in the
same command. It must not create runtime state, execute commands, call
providers, persist approval records, write report artifacts, add schemas, add
examples, enable writes, hosted behavior, or change release posture.

This plan does not implement promotion.

## 2. Goals

- Define the first active promotion command/path.
- Make promotion explicit, deterministic, and reviewable.
- Reuse the existing draft path validation, draft loader, preflight assessment,
  and steward-review helper.
- Require a fresh preflight result against the current draft content.
- Require explicit steward/delegated-maintainer approval input in the promotion
  command until a persisted approval model exists.
- Activate exactly one draft workflow file.
- Refuse overwrites and active workflow id conflicts.
- Preserve existing active workflow behavior.
- Keep output bounded and redaction-safe.
- Provide text and JSON output.
- Add focused tests for mutation, non-mutation, failure, and redaction
  boundaries.

## 3. Non-Goals

Do not implement:

- automatic promotion;
- promotion without explicit CLI intent;
- promotion of multiple drafts at once;
- persisted steward approval records;
- workflow catalog persistence;
- runtime state creation;
- workflow run creation;
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
- write-capable adapters or provider mutation;
- release posture changes.

## 4. Candidate CLI Shape

Recommended first command:

```sh
workflow-os author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason>
```

Recommended dry-run:

```sh
workflow-os author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason> \
  --dry-run
```

JSON output should use the existing global flag:

```sh
workflow-os --json author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason>
```

The first implementation should not require a separate persisted approval id.
Instead, it should make the approving reviewer and bounded reason explicit and
call the existing steward-review helper in the same process after recomputing
preflight. Persisted approval consumption can be planned later.

## 5. Promotion Semantics

Promotion means:

- the draft is copied or moved into the active `workflows/` directory;
- the resulting active file is visible to the existing project loader;
- subsequent `workflow-os validate` validates it as an active workflow spec;
- future `workflow-os run <workflow-id>` can target it if the spec is runnable.

Promotion does not mean:

- a workflow run has started;
- local checks have run;
- provider calls have occurred;
- report artifacts have been written;
- approval was persisted for future reuse;
- external writes are authorized;
- future edits to the draft or active workflow are approved.

## 6. File Movement Policy

Recommended first implementation: create one active workflow file from the
reviewed draft content, then leave the draft file in place.

Rationale:

- creating the active file is enough for loader visibility;
- preserving the draft avoids destructive deletion in the first mutation slice;
- overwrite refusal protects existing active workflows;
- a later cleanup/retire-draft phase can decide whether archived drafts should
  be deleted, moved, or marked superseded.

Active output path:

- derive from draft filename by removing the `workflows/drafts/` prefix;
- require `.workflow.yml`;
- write to `workflows/<same-file-name>`;
- reject if the active output path already exists;
- reject if any active workflow already has the candidate workflow id.

The command must not write outside `workflows/`.

## 7. Required Inputs

Required:

- `--draft workflows/drafts/<name>.workflow.yml`;
- `--reviewer <actor-id>`;
- `--reason <bounded-review-reason>`.

Optional:

- `--dry-run`;
- future `--output workflows/<name>.workflow.yml` only if separately planned.

Derived by the command:

- candidate workflow id;
- current draft content hash;
- preflight blocker/warning codes;
- active workflow id/path conflict status;
- owner and escalation posture summary;
- policy posture summary;
- evidence/report posture summary;
- side-effect posture summary;
- active output path.

## 8. Promotion Algorithm

Recommended first implementation sequence:

1. Load and validate the current project.
2. Validate the draft path is relative and under `workflows/drafts/`.
3. Load and parse the draft as an isolated candidate.
4. Compute the current draft content hash.
5. Run existing promotion preflight logic against the current draft.
6. If preflight has blockers, fail closed before any file write.
7. Build `WorkflowDraftStewardReviewInput` with
   `ApprovedForPromotion`, reviewer, bounded reason, current content hash, and
   derived bounded summaries.
8. Call `review_workflow_draft_for_promotion`.
9. If review is not `AuthorizedForPromotion`, fail closed before any file
   write.
10. Derive the active output path under `workflows/`.
11. Refuse overwrite if the active output file exists.
12. Validate the candidate in active-placement context before writing, using a
    temporary project overlay or equivalent structured validation path that does
    not mutate the repository.
13. Write the active workflow file atomically where the platform supports it:
    write to a temporary sibling file, flush, then rename to the final path.
14. Reload and validate the project after the active file is created as a final
    sanity check.
15. If post-promotion validation fails despite pre-write validation, return a
    structured error, do not claim the workflow is safe to run, and provide
    explicit recovery instructions.
16. Print bounded success output.

## 9. Dry-Run Behavior

`--dry-run` should run every deterministic validation step except the final file
write.

Dry-run output should include:

- mode `author_workflow_active_promotion_dry_run`;
- draft path;
- candidate workflow id;
- active output path;
- preflight status;
- warning codes;
- reviewer;
- non-mutation flags;
- privacy boundary;
- next action.

Dry-run must not write files, register workflows, create runtime state, execute
commands, call providers, or persist approval records.

## 10. Output Policy

Successful promotion output should include:

- mode `author_workflow_active_promotion`;
- status `promoted`;
- draft path;
- active workflow path;
- candidate workflow id;
- draft content hash;
- reviewer;
- preflight status;
- warning codes;
- post-promotion validation status;
- non-runtime flags;
- privacy boundary;
- next action.

Output must not copy:

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
- existing agent instruction bodies;
- review reason text.

## 11. Error Handling

Promotion must fail closed before writing when:

- the project cannot be loaded or validated;
- the draft path is missing, unsafe, absolute, or outside `workflows/drafts/`;
- the draft file is missing;
- the draft cannot be parsed;
- candidate workflow id is invalid;
- candidate workflow id conflicts with an active workflow;
- active output path exists;
- current preflight has blockers;
- reviewer actor is invalid;
- reason is missing, too long, or secret-like;
- steward-review helper rejects the derived input;
- active output path cannot be derived safely.

Errors must use stable codes and must not echo raw draft content, unsafe path
values, review reason text, parser payloads, command output, provider payloads,
or secret-like values.

Post-write validation failure needs explicit handling:

- return a stable code such as
  `cli.workflow_authoring.active_promotion_post_validation_failed`;
- do not claim the workflow is safe to run;
- do not create runtime state;
- treat the condition as an unexpected consistency failure because active-context
  validation should have run before the write;
- print recovery guidance that tells the maintainer to inspect or remove the
  active workflow file.

## 12. Privacy And Redaction

Promotion output should use bounded summaries, relative paths, ids, hashes, and
codes only.

The command must not read arbitrary source contents or manifest bodies. It may
read only:

- the current Workflow OS project files needed by existing loaders;
- the selected draft workflow file;
- active workflow files needed by existing loaders/preflight.

Review reason must be validated but not printed in text or JSON output.

## 13. Relationship To Steward Review

The first implementation should call the steward-review helper in the same
process. That makes approval context explicit without pretending a durable
approval store exists.

Future persisted approval consumption can be planned separately. It must prove:

- the approval was presented with exact scope;
- the approval binds to draft path, workflow id, content hash, reviewer, and
  preflight context;
- stale approvals fail closed when draft content changes;
- approvals are not reused for future edits.

## 14. Relationship To Runtime

Promotion is repository authoring, not runtime execution.

The command must not:

- create a `WorkflowRun`;
- append workflow events;
- append audit events;
- create `.workflow-os/state`;
- execute local skills;
- execute local checks;
- call providers;
- write report artifacts.

Runtime behavior changes only indirectly after the active workflow file exists
and the user later chooses to validate or run the workflow.

## 15. Test Plan

Future implementation tests should cover:

- dry-run succeeds for a preflight-passing, steward-approved draft without
  writing files;
- promotion succeeds for one preflight-passing draft and creates exactly one
  active workflow file;
- draft file remains preserved after first implementation promotion;
- pre-write active-context validation runs before any active file write;
- resulting project validates after promotion;
- active workflow id conflict fails closed before writing;
- active output path overwrite fails closed before writing;
- blocked preflight fails closed before writing;
- invalid reviewer fails closed before writing;
- secret-like reason fails closed without leakage;
- unsafe draft path fails closed without leakage;
- raw draft YAML marker is not copied to output;
- reason text is not copied to output;
- no runtime state directory is created;
- no workflow run is created;
- no command/provider/local-check execution occurs;
- JSON output is bounded;
- existing authoring dry-run, file-output, preflight, and steward-review tests
  continue to pass;
- docs check passes.

## 16. Proposed Implementation Sequence

Recommended next implementation prompt:

1. Add CLI parsing for `workflow-os author workflow promote`.
2. Add active output path derivation from draft path.
3. Reuse existing preflight candidate loading and assessment.
4. Call `review_workflow_draft_for_promotion` with explicit
   `ApprovedForPromotion` input.
5. Add dry-run output.
6. Add atomic active file write with overwrite refusal.
7. Reload and validate the project after write.
8. Add bounded text/JSON output.
9. Add focused tests.
10. Update docs and create an implementation report.
11. Review before planning persisted approval consumption or draft cleanup.

## 17. Deferred Work

- Persisted steward approval records.
- Approval presentation proof enforcement.
- Draft cleanup, archive, supersession, or deletion.
- Workflow catalog/store integration.
- Workflow-declared steward configuration.
- Enterprise steward/admin controls.
- RBAC/IdP integration.
- Report artifacts.
- Runtime event/audit emission for authoring operations.
- Schemas and examples.
- Hosted/distributed runtime behavior.
- Write-capable adapters.

## 18. Final Recommendation

Proceed next to active promotion implementation with an explicit local CLI
command, dry-run support, same-process preflight and steward-review validation,
and one bounded repository file write into `workflows/`.

Do not implement persisted approvals, runtime state, report artifacts, provider
calls, schemas, examples, hosted behavior, write-capable adapters, or release
posture changes in that implementation phase.
