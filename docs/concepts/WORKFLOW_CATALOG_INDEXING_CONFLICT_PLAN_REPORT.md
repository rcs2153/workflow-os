# Workflow Catalog Indexing And Conflict Plan Report

## 1. Executive Summary

This planning phase defines the next safe workflow catalog step after the
accepted local catalog store helper: a pure in-memory indexing and conflict
helper.

The planned helper will consume explicit active workflow summaries, draft
summaries, archived draft summaries, catalog records, stewardship records, and
archive records. It will return deterministic inventory and conflict
disclosures without mutating files, writing catalog records, registering
workflows, running commands, calling providers, changing schemas, creating
examples, enabling hosted behavior, or changing release posture.

## 2. Scope Completed

- Created
  [Workflow Catalog Indexing And Conflict Helper Plan](../implementation-plans/workflow-catalog-indexing-conflict-plan.md).
- Updated
  [Workflow Catalog Persistence And Stewardship Integration Plan](../implementation-plans/workflow-catalog-persistence-plan.md)
  to link the accepted store helper review and this indexing/conflict plan.
- Updated
  [Governed Workflow Authoring Catalog And Stewardship Plan](../implementation-plans/governed-workflow-authoring-catalog-stewardship-plan.md)
  to identify the pure in-memory indexing/conflict helper as the next
  implementation boundary.
- Updated [Roadmap](../../ROADMAP.md) to state that indexing/conflict planning
  is documented and implementation remains next.

## 3. Scope Explicitly Not Completed

This planning phase did not implement:

- indexing helper code;
- conflict helper code;
- command integration;
- runtime workflow registration;
- automatic workflow generation;
- automatic promotion;
- automatic archive cleanup;
- persisted approval consumption;
- promotion command catalog writes;
- archive command catalog writes;
- draft deletion;
- workflow schema changes;
- examples;
- provider calls;
- command execution or local check execution;
- hosted or distributed behavior;
- RBAC, IdP integration, notifications, or admin UI;
- write-capable adapters or provider mutation;
- release posture changes.

## 4. Plan Summary

The plan recommends a pure helper such as:

```text
build_workflow_catalog_index(input) -> Result<WorkflowCatalogIndex, WorkflowOsError>
```

The helper should produce:

- active workflow index entries;
- inactive draft index entries;
- archived draft index entries;
- catalog record index entries;
- stewardship record index entries;
- archive record index entries;
- deterministic conflict disclosures;
- bounded counts by lifecycle and conflict severity.

The helper should accept already-derived summaries and already-loaded records
instead of reading hidden global state.

## 5. Conflict Policy Summary

The plan separates deterministic blockers from warning-only disclosures.

Hard blocker candidates include:

- duplicate active workflow id;
- duplicate active workflow path;
- catalog record marked active but missing the active workflow file;
- catalog path mismatch;
- catalog content hash mismatch;
- stale draft stewardship hash;
- archive path or hash mismatch;
- archived draft re-promotion without explicit new review input.

Warning-only candidates include:

- active workflow without catalog coverage when strict catalog coverage is not
  requested;
- catalog record without a currently loader-visible workflow;
- source recommendation already active;
- missing owner or escalation contact;
- missing stewardship decision reference;
- exact posture overlap disclosures.

Semantic similarity remains deferred. The first helper must not use
model-generated similarity judgments to block workflows.

## 6. Privacy And Redaction Summary

The plan preserves the catalog privacy boundary.

The helper must not copy raw workflow YAML, raw draft YAML, source contents,
manifest bodies, package script bodies, dependency values, lockfile contents,
CI logs, command output, provider payloads, parser payloads, absolute private
paths, environment values, credentials, authorization headers, private keys,
token-like values, unbounded reviewer reasons, or existing agent instruction
bodies.

Allowed outputs are bounded identifiers, repository-relative paths that already
pass validation, content hashes, lifecycle/status codes, decision ids, archive
ids, conflict kind/severity codes, counts, and stable references.

## 7. Governed Dogfood Summary

- workflow: `dg/d`
- phase: planning
- run id: `run-1783518795609093000-2`
- approval id: `approval/run-1783518795609093000-2/planning-approved`
- approval outcome: granted
- approval reason: `approved-workflow-catalog-indexing-conflict-helper-planning`
- terminal status: `Completed`
- events: `39`
- event summary:
  `ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8, RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6, SkillInvocationSucceeded:6, StepScheduled:6`
- out-of-kernel work disclosed: Codex wrote planning documentation, updated
  roadmap/planning files, ran docs validation, and will perform git/PR
  packaging outside the kernel.

## 8. Validation

Required validation for this planning phase:

- `npm run check:docs`
  - Passed.

## 9. Remaining Limitations

- No in-memory indexing/conflict helper exists yet.
- No catalog command consumes the store or future index.
- No promotion or archive command writes catalog metadata.
- No persisted stewardship decision is consumed by promotion.
- No strict catalog coverage enforcement exists.
- No hosted/team catalog backend exists.

## 10. Recommended Next Phase

Recommended next phase: pure in-memory workflow catalog indexing/conflict helper
implementation.

That implementation should stay helper-only, deterministic, explicit-input,
redaction-safe, and non-mutating. Command integration, runtime registration,
schemas, examples, hosted behavior, provider calls, writes, and release posture
changes must remain out of scope.
