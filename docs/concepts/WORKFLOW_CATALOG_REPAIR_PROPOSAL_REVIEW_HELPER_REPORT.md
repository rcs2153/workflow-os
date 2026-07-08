# Workflow Catalog Repair Proposal Review Helper Report

## 1. Executive Summary

The in-memory workflow catalog repair proposal review model/helper is
implemented.

This phase adds a bounded maintainer/steward decision record for typed repair
proposals produced by the existing non-mutating catalog repair dry-run helper.
The review record can approve a proposal for future apply planning, reject it,
defer it, require manual review, or require a new dry-run. It does not apply
repairs.

## 2. Scope Completed

- Added `WorkflowCatalogRepairProposalReviewId`.
- Added `WorkflowCatalogRepairProposalDecisionKind`.
- Added `WorkflowCatalogRepairProposalReviewInput`.
- Added `WorkflowCatalogRepairProposalReview`.
- Added `review_workflow_catalog_repair_proposal`.
- Added `validate_workflow_catalog_repair_proposal_review_matches`.
- Preserved proposal identity fields needed for stale-proposal detection.
- Supported optional stable references for approvals, policy decision events,
  evidence, validation, and WorkReport records.
- Added serde validation and redaction-safe Debug behavior.
- Added focused Rust tests for valid reviews, decision kinds, invalid inputs,
  optional references, serde, Debug redaction, and stale proposal detection.

## 3. Scope Explicitly Not Completed

- No repair apply mode.
- No automatic repair.
- No persisted review sidecars.
- No CLI review command.
- No catalog record creation, update, overwrite, deletion, or cleanup.
- No active workflow rewrite.
- No draft or archive movement.
- No runtime workflow registration.
- No runtime state creation or event append.
- No report artifact generation.
- No schema exposure.
- No examples.
- No provider calls.
- No write-capable adapters.
- No hosted/team catalog backend.
- No release posture change.

## 4. Model And Helper Summary

`WorkflowCatalogRepairProposalReview` stores a bounded decision over one typed
`WorkflowCatalogRepairProposal`.

The record captures:

- review id;
- proposal id;
- proposal action kind;
- proposal conflict kind;
- proposal conflict source;
- workflow id, if available;
- bounded source reference;
- reviewer actor;
- bounded reviewer reason;
- decision kind;
- reviewed timestamp;
- optional approval, policy decision, evidence, validation, and WorkReport
  references;
- sensitivity;
- redaction metadata.

`review_workflow_catalog_repair_proposal` constructs the record from explicit
inputs and never reads hidden state, scans files, writes catalog records, or
applies repairs.

## 5. Decision Boundary Summary

The implemented decision kinds are:

- `ApprovedForFutureApplyPlanning`;
- `Rejected`;
- `Deferred`;
- `RequiresManualCatalogReview`;
- `RequiresManualWorkflowReview`;
- `RequiresNewDryRun`.

`ApprovedForFutureApplyPlanning` is intentionally not an apply permission. It
means a future apply-mode planner may consider the proposal after separate
freshness, policy, and mutation checks.

## 6. Staleness Boundary Summary

Review records preserve proposal identity:

- proposal id;
- action kind;
- conflict kind;
- conflict source;
- workflow id;
- source reference.

`validate_workflow_catalog_repair_proposal_review_matches` fails closed when a
review no longer matches a fresh proposal identity. This gives future apply
planning a typed stale-review boundary without implementing apply mode.

## 7. Redaction And Privacy Summary

The helper uses existing bounded constructors and validates:

- review ids;
- reviewer reasons;
- source references;
- redaction metadata;
- optional reference counts.

Secret-like reviewer reasons and review ids are rejected without echoing raw
values in errors. Debug output redacts reviewer reason and source reference and
shows reference counts instead of expanding redaction metadata.

Review records do not copy raw workflow YAML, raw catalog records, source file
contents, command output, provider payloads, parser payloads, CI logs,
environment values, credentials, tokens, or secret-like values.

## 8. Test Coverage Summary

Focused tests cover:

- valid approve-for-future-apply-planning review;
- rejected, deferred, manual-review, and new-dry-run decisions;
- invalid review id handling;
- invalid reviewer handling through existing `ActorId` validation;
- secret-like reason rejection without leakage;
- optional stable references for approval, policy decision event, evidence,
  validation, and WorkReport ids;
- serde round trip;
- invalid serialized review fail-closed behavior;
- Debug non-leakage for reviewer reason and source reference;
- stale proposal mismatch detection.

Existing catalog repair proposal, catalog status, promotion, archive, and
stewardship tests remain in the same focused test module.

## 9. Commands Run And Results

Governed dogfood phase:

```text
npm run dogfood:benchmark -- phase-start --phase implementation ...
npm run dogfood:benchmark -- phase-close run-1783546663353590000-2 --phase implementation
```

Result: approved and completed under `dg/implement`.

Governed phase metadata:

- dogfood workflow id: `dg/implement`
- run id: `run-1783546663353590000-2`
- approval id:
  `approval/run-1783546663353590000-2/implementation-approved`
- approval outcome: granted by delegated maintainer
- event summary: 39 events; 1 approval; 0 retries; 0 escalations
- event kinds:
  `ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8,
  RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1,
  SkillInvocationRequested:6, SkillInvocationStarted:6,
  SkillInvocationSucceeded:6, StepScheduled:6`
- out-of-kernel work: Codex edited Rust/docs, ran validation commands, and will
  handle git/PR operations; Workflow OS coordinated governance only

Focused validation:

```text
cargo fmt --all
cargo test -p workflow-core --test workflow_catalog_index
```

Result: passed, including 24 workflow catalog tests.

Full required validation:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
```

Result: passed.

Notes:

- The first clippy run identified two `doc_markdown` fixes for `WorkReport`
  references in Rust docs. Those were corrected and clippy passed on rerun.

## 10. Remaining Known Limitations

- Review records are in-memory only.
- No review record persistence exists.
- No CLI review command exists.
- No repair apply mode exists.
- Future apply planning must still validate a fresh matching dry-run proposal.
- Optional approval/policy/evidence/report references are citations only; they
  do not create or validate external approval or policy authority by
  themselves.

## 11. Recommended Next Phase

Recommended next phase: workflow catalog repair proposal review helper review.

Why: the in-memory model/helper is the safety boundary that should be reviewed
before any optional persistence, CLI review recording, or apply-mode planning.
