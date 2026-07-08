# Workflow Catalog Repair Review Persistence Plan Report

## 1. Executive Summary

Workflow catalog repair review persistence is now planned.

The plan defines how a future implementation should persist
`WorkflowCatalogRepairProposalReview` records as explicit local catalog sidecars
after the accepted in-memory review helper. It keeps persisted reviews as
governance metadata, not repair application authority.

## 2. Scope Completed

- Created
  [Workflow Catalog Repair Review Persistence Plan](../implementation-plans/workflow-catalog-repair-review-persistence-plan.md).
- Defined the recommended `repair-reviews/` local catalog storage layout.
- Defined record identity and file naming posture.
- Defined write timing, duplicate, replacement, and stale-review policies.
- Defined a future explicit CLI review-recording posture.
- Defined privacy, redaction, error-handling, and test expectations.
- Updated the roadmap and related catalog planning docs to link the plan.

## 3. Scope Explicitly Not Completed

- No persistence implementation.
- No persisted review writes.
- No CLI review write behavior.
- No repair apply mode.
- No automatic catalog repair.
- No catalog record creation, update, overwrite, deletion, or cleanup.
- No active workflow rewrite.
- No draft or archive movement.
- No runtime workflow registration.
- No runtime state creation.
- No event or audit append.
- No report artifact generation.
- No schema exposure.
- No examples.
- No provider calls.
- No write-capable adapters.
- No hosted/team catalog backend behavior.
- No release posture change.

## 4. Planning Summary

The recommended persistence layout is:

```text
.workflow-os/catalog/
  repair-reviews/
    <safe-repair-review-id>.json
```

The first future implementation should be a local store helper only. It should
write and read validated review records, reject duplicate ids, preserve
deterministic listing, and never apply repairs.

## 5. Safety Summary

Persisted review records must be treated as sensitive governance metadata.
They may include bounded reviewer reasons and repository-relative source
references, but must never copy raw workflow YAML, raw catalog payloads, source
contents, command output, provider payloads, parser payloads, CI logs,
environment values, credentials, tokens, or secret-like values.

Future apply planning must require a fresh matching dry-run proposal and must
not treat a persisted review record alone as mutation permission.

## 6. Validation Summary

Validation command:

```text
npm run check:docs
```

Result: passed.

Rust validation was not run for this phase because it was planning-only and did
not change Rust code.

## 7. Governed Phase Metadata

- dogfood workflow id: `dg/d`
- run id: `run-1783548765677626000-2`
- approval id: `approval/run-1783548765677626000-2/planning-approved`
- approval outcome: granted by delegated maintainer
- event summary: 39 events; 1 approval; 0 retries; 0 escalations
- event kinds:
  `ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8,
  RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1,
  RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6,
  SkillInvocationSucceeded:6, StepScheduled:6`
- out-of-kernel work: documentation edits, validation, git, PR, and merge
  operations were executed by Codex/human execution layer; the kernel
  coordinated governance only

## 8. Recommended Next Phase

Recommended next phase: workflow catalog repair review persistence plan review.

Why: the persistence plan should be reviewed before any local store helper,
CLI review-recording surface, or future apply-mode planning is implemented.
