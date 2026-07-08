# Governed Workflow Authoring Active Promotion Plan Report

## 1. Executive Summary

This phase plans the next governed workflow authoring boundary: active
promotion of one inactive draft workflow into the active `workflows/` surface.

The plan follows the accepted steward-review CLI preview. It keeps the next
implementation narrow: an explicit CLI promotion command, dry-run support,
fresh preflight, same-process steward-review validation, overwrite refusal,
bounded output, and one repository file write into `workflows/`.

This phase does not implement active promotion.

## 2. Scope Completed

- Created [Governed Workflow Authoring Active Promotion Plan](../implementation-plans/governed-workflow-authoring-active-promotion-plan.md).
- Defined the promotion semantics and non-goals.
- Recommended the first CLI shape:
  `workflow-os author workflow promote --draft ... --reviewer ... --reason ...`.
- Defined dry-run behavior.
- Defined active output path derivation from `workflows/drafts/` to
  `workflows/`.
- Recommended preserving the draft file in the first implementation.
- Defined preflight and steward-review reuse.
- Defined file write, overwrite refusal, post-write validation, privacy, error,
  and test policies.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- active promotion;
- file movement or active workflow file writing;
- workflow registration beyond loader-visible file placement;
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
- write-capable adapters;
- release posture changes.

## 4. Key Planning Decisions

The plan recommends:

- explicit local CLI promotion rather than automatic activation;
- same-process steward-review validation instead of a fake persisted approval
  store;
- preserving the draft after promotion to avoid destructive deletion in the
  first mutation slice;
- refusing active output overwrites;
- validating the candidate in active-placement context before writing and then
  validating the project again after writing as a final sanity check;
- documenting post-write validation failure as a recovery boundary rather than
  silently rolling back without a rollback design.

## 5. Privacy And Redaction Posture

The planned command should use relative paths, ids, hashes, and bounded codes.

It must not copy raw draft YAML, source contents, package scripts, dependency
values, lockfile contents, CI logs, command output, provider payloads, parser
payloads, private absolute paths, environment values, credentials,
authorization headers, private keys, token-like strings, existing agent
instruction bodies, or review reason text.

## 6. Governed Dogfood Summary

- Workflow: `dg/d`.
- Run ID: `run-1783473883363077000-2`.
- Approval ID: `approval/run-1783473883363077000-2/planning-approved`.
- Approval outcome: granted by delegated maintainer after the full approval
  handoff was emitted.
- Approved scope: create a precise future implementation plan for explicit
  active promotion of one unchanged steward-reviewed draft and update roadmap
  status docs plus planning report.
- Strict non-goals: no implementation, file movement, registration, approval
  store, runtime state, commands, providers, artifacts, schemas, examples,
  writes, hosted behavior, or release posture change.

## 7. Validation

Validation for this planning phase:

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783473883363077000-2 --phase planning`:
  passed.

Governed phase-close event summary:

- Total events: 39.
- Approvals: 1.
- Retries: 0.
- Escalations: 0.
- Event kinds: ApprovalGranted:1, ApprovalRequested:1,
  PolicyDecisionRecorded:8, RunCompleted:1, RunCreated:1, RunResumed:1,
  RunStarted:1, RunValidated:1, SkillInvocationRequested:6,
  SkillInvocationStarted:6, SkillInvocationSucceeded:6, StepScheduled:6.

## 8. Remaining Known Limitations

- Active promotion remains unimplemented.
- Persisted steward approval remains unimplemented.
- Approval presentation proof enforcement remains a separate open hardening
  gap.
- Draft cleanup/archive/supersession remains unplanned beyond deferral.
- No workflow catalog/store integration exists.
- Runtime event/audit emission for authoring mutations remains deferred.

## 9. Recommended Next Phase

Recommended next phase: active promotion plan review.

The plan introduces the first active workflow authoring mutation boundary. It
should receive maintainer review before implementation because the next phase
will write active workflow files into `workflows/`.
