# Governed Workflow Authoring Active Promotion Plan Review

## 1. Executive Verdict

Plan accepted with non-blocking follow-ups.

The active promotion plan is appropriately scoped for the next implementation
phase. It defines the first explicit repository mutation boundary where one
preflight-passing, steward-approved inactive draft can become an active workflow
file under `workflows/`.

The plan correctly keeps promotion local, explicit, deterministic, and
reviewable. It does not authorize runtime execution, runtime state, provider
calls, report artifacts, schemas, examples, hosted behavior, writes to external
systems, or release posture changes.

During review, the plan was tightened to require pre-write active-context
validation before writing the active workflow file. Post-write validation should
remain a final sanity check, not the first proof that the promoted workflow is
valid.

## 2. Scope Verification

The plan stayed within planning-only scope.

It did not implement:

- active promotion;
- file movement or active workflow file writing;
- active workflow registration through loader-visible placement;
- persisted steward approval records;
- workflow catalog persistence;
- runtime state creation;
- workflow run creation;
- command execution;
- local check execution;
- provider calls;
- report artifacts;
- schemas;
- examples;
- hosted or distributed runtime behavior;
- RBAC, IdP, admin UI, paging, or notifications;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters or provider mutation;
- release posture changes.

## 3. Promotion Boundary Assessment

The plan defines promotion as a repository authoring boundary, not runtime
execution.

That distinction is important. Promotion means the draft becomes loader-visible
as an active workflow spec. It does not mean:

- a run has started;
- checks passed;
- provider calls occurred;
- report artifacts exist;
- external writes are authorized;
- future edits are approved.

The plan correctly requires explicit CLI intent and rejects automatic promotion.

## 4. CLI Shape Assessment

The proposed command is clear:

```sh
workflow-os author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason>
```

The dry-run variant is appropriate and should be implemented first in the same
command surface:

```sh
workflow-os author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason> \
  --dry-run
```

Using explicit reviewer/reason inputs is acceptable for the first implementation
because persisted steward approval records do not exist yet. The plan is honest
that this does not create a durable approval store.

## 5. Preflight And Steward-Review Assessment

The plan correctly requires the implementation to reuse existing machinery:

- project load and validation;
- draft path validation;
- isolated draft loading;
- content hashing;
- promotion preflight assessment;
- `review_workflow_draft_for_promotion`.

It also correctly requires same-process recomputation rather than trusting stale
CLI output, pasted JSON, or model summaries.

## 6. File Write Policy Assessment

The plan's first implementation policy is acceptable:

- derive active path from the draft filename;
- write to `workflows/<same-file-name>`;
- reject active path overwrite;
- reject active workflow id conflicts;
- preserve the draft file after promotion.

Preserving the draft avoids destructive deletion in the first mutation slice.
Draft cleanup, archive, supersession, or deletion should remain a later phase.

The reviewed plan now requires active-placement validation before writing. That
is necessary because a post-write validation failure should be treated as an
unexpected consistency failure, not a normal path that leaves the project broken.

## 7. Error Handling Assessment

The plan defines fail-closed behavior before writing for:

- invalid project state;
- unsafe draft paths;
- missing or unparsable drafts;
- invalid candidate workflow id;
- active workflow id conflict;
- active output path overwrite;
- preflight blockers;
- invalid reviewer;
- missing, long, or secret-like reason;
- steward-review rejection;
- unsafe active output derivation.

Errors are required to use stable codes and avoid raw draft content, unsafe path
values, review reason text, parser payloads, command output, provider payloads,
or secret-like values.

Post-write validation failure remains explicitly documented as a recovery
boundary, but the plan now requires pre-write active-context validation to make
that path exceptional.

## 8. Privacy And Redaction Assessment

The privacy boundary is appropriate:

- bounded ids, relative paths, hashes, and codes only;
- no arbitrary source-content reads;
- no manifest body reads beyond existing loaders;
- no raw package scripts, dependency values, lockfile contents, CI logs,
  provider payloads, parser payloads, credentials, authorization headers,
  private keys, token-like values, existing agent instruction bodies, or review
  reason text in output.

The plan correctly requires validating the review reason while not printing it.

## 9. Runtime Boundary Assessment

The plan preserves runtime boundaries:

- no `WorkflowRun`;
- no workflow events;
- no audit events;
- no `.workflow-os/state`;
- no local skill execution;
- no local check execution;
- no provider calls;
- no report artifact writes.

The only permitted mutation in the future implementation is the explicit active
workflow file write.

## 10. Test Plan Assessment

The proposed test plan covers the important first implementation risks:

- dry-run non-mutation;
- successful promotion creates exactly one active workflow file;
- draft preservation;
- active-context validation before write;
- post-promotion project validation;
- conflict and overwrite rejection;
- blocked preflight rejection;
- invalid reviewer and secret-like reason rejection;
- unsafe path non-leakage;
- raw payload non-leakage;
- no runtime state, runs, commands, providers, or checks;
- JSON boundedness;
- regression coverage for authoring dry-run, file-output, preflight, and
  steward-review paths.

This is sufficient to drive a small implementation prompt.

## 11. Documentation Review

Documentation correctly states:

- active promotion is planned, not implemented;
- the steward-review CLI preview is implemented and reviewed;
- active promotion remains separate;
- persisted steward approvals remain unimplemented;
- runtime state, commands, providers, artifacts, schemas, examples, writes,
  hosted behavior, and release posture changes remain out of scope.

The plan report and roadmap were updated to point to the active promotion plan.

## 12. Governed Dogfood Review Summary

- Workflow: `dg/review`.
- Run ID: `run-1783474717726952000-2`.
- Approval ID: `approval/run-1783474717726952000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer after the full approval
  handoff was emitted.
- Approved scope: inspect the active promotion plan, plan report, roadmap/status
  docs, current authoring CLI docs, and prior preflight/steward-review reviews;
  create bounded maintainer review.
- Strict non-goals: no implementation, file movement, registration, approval
  store, runtime state, commands, providers, artifacts, schemas, examples,
  writes, hosted behavior, or release posture change.

## 13. Validation

Validation commands run for this review:

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783474717726952000-2 --phase review`:
  passed.

## 14. Blockers

No blockers.

## 15. Non-Blocking Follow-Ups

- Decide during implementation whether active-placement validation should use a
  temporary project overlay, an in-memory bundle construction path, or another
  structured validation path.
- Keep persisted approval consumption out of the first promotion implementation.
- Plan draft cleanup/archive/supersession separately after active promotion is
  implemented and reviewed.
- Keep JSON output preview-only unless the authoring promotion API is
  intentionally stabilized.

## 16. Recommended Next Phase

Recommended next phase: active promotion implementation.

The implementation should remain narrow:

- add `workflow-os author workflow promote`;
- support `--dry-run`;
- recompute preflight and steward-review in the same process;
- validate active placement before writing;
- write one active workflow file with overwrite refusal;
- preserve the draft;
- validate after writing;
- add focused tests and docs;
- do not add persisted approvals, runtime state, provider calls, report
  artifacts, schemas, examples, hosted behavior, write-capable adapters, or
  release posture changes.
