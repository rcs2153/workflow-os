# Workflow Catalog Persistence Integration Plan Report

## 1. Executive Summary

This planning phase refreshed the workflow catalog persistence plan after the
accepted `workflow-os author workflow catalog-status` command review.

The project now has the core catalog/stewardship/archive model, a validated
local store helper, a reviewed catalog indexing/conflict helper, and a
read-only catalog status command. The next gap is command write integration.
This report records the decision to start with opt-in steward-review
persistence before promotion catalog writes or archive metadata writes.

No runtime behavior was implemented in this phase.

## 2. Scope Completed

- Updated
  [Workflow Catalog Persistence And Stewardship Integration Plan](../implementation-plans/workflow-catalog-persistence-plan.md)
  to reflect completed store-helper, conflict-helper, and status-command
  phases.
- Chose opt-in steward-review persistence as the next smallest command-write
  implementation phase.
- Defined the recommended command-write sequence:
  1. opt-in steward-review persistence;
  2. persisted stewardship review;
  3. opt-in promotion catalog-record write;
  4. promotion catalog write review;
  5. archive metadata write;
  6. archive metadata review.
- Clarified failure/atomicity posture for stewardship, promotion catalog, and
  archive metadata writes.
- Updated [Roadmap](../../ROADMAP.md) with the refreshed next phase.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- command behavior changes;
- catalog record writes;
- stewardship record writes;
- archive record writes;
- catalog status enforcement;
- promotion catalog writes;
- archive metadata writes;
- workflow runtime registration;
- automatic catalog repair;
- automatic workflow generation;
- automatic promotion;
- automatic archive cleanup;
- draft deletion;
- workflow schema changes;
- examples;
- hosted or team catalog backend;
- provider calls;
- command execution or local check execution;
- write-capable adapters;
- release posture changes.

## 4. Integration Decision

The next command write should be persisted stewardship, not promotion.

Reason:

- steward-review already derives a bounded review decision without moving
  workflow files;
- persisted stewardship creates a durable reference that promotion and archive
  commands can cite later;
- default steward-review behavior can remain preview-only;
- the first write path can be tested without mutating active workflow files;
- this reduces the risk of partial promotion/catalog state while the catalog
  write boundary matures.

Recommended first CLI posture:

```text
workflow-os author workflow steward-review \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason> \
  --persist-stewardship \
  [--catalog-root .workflow-os/catalog]
```

## 5. Failure And Atomicity Summary

Persisted stewardship should fail closed without changing workflow files.

Promotion catalog write integration should:

- reject invalid catalog inputs before active workflow mutation;
- write catalog records only after active placement validation and post-write
  validation succeed;
- surface explicit partial-integration status if a catalog sidecar write fails
  after the active workflow file exists;
- avoid automatic rollback until recovery policy is separately designed.

Archive metadata integration should:

- write archive records only after a successful archive move;
- surface explicit partial-integration status if archive metadata cannot be
  written;
- avoid draft deletion or automatic cleanup.

## 6. Privacy And Redaction Summary

The plan keeps catalog persistence reference-oriented.

Future command writes must not persist or print:

- raw workflow YAML;
- raw draft YAML;
- source contents;
- package script bodies;
- dependency values or lockfile contents;
- CI logs;
- command output;
- provider payloads;
- parser payloads;
- absolute private paths;
- environment values;
- credentials, authorization headers, private keys, or token-like values;
- existing agent instruction bodies.

Errors must use stable codes and avoid echoing rejected field values.

## 7. Test Coverage Required For Future Implementation

The next implementation phase should add tests proving:

- preview-only steward-review remains non-mutating;
- persisted steward-review writes a stewardship record only when explicitly
  requested;
- dry-run writes no catalog files;
- duplicate stewardship decisions fail closed;
- invalid serialized stewardship records fail closed;
- corrupt catalog files do not leak contents;
- unsafe catalog roots fail closed;
- no workflow files or runtime state are modified by persisted stewardship;
- no raw YAML, command output, provider payloads, paths, or sensitive values are
  copied into errors, Debug, or serialization;
- existing authoring, catalog store, catalog index, and status command tests
  continue to pass.

## 8. Dogfood Governance

```text
workflow_id: dg/d
run_id: run-1783528476420992000-2
approval_id: approval/run-1783528476420992000-2/planning-approved
approval_outcome: granted
status: Completed
events_total: 39
event_summary: ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8, RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6, SkillInvocationSucceeded:6, StepScheduled:6
retries: 0
escalations: 0
```

The planning phase was approved before this artifact was written. The kernel
governed the planning boundary; document edits, validation commands, git
operations, and PR operations were performed outside the kernel and are
disclosed here.

## 9. Commands Run And Results

```text
npm run check:docs
```

Result: passed.

## 10. Remaining Known Limitations

- Catalog persistence still has no command write integration.
- Steward-review decisions are not yet persisted.
- Promotion does not yet write catalog records.
- Archive command does not yet write archive metadata.
- Catalog status remains read-only.
- Strict catalog coverage remains status-only.
- There is no runtime workflow registration, hosted catalog backend, schema
  exposure, examples, or provider behavior.

## 11. Recommended Next Phase

Recommended next phase: opt-in steward-review persistence.

This phase should keep existing preview-only steward-review behavior unchanged
by default and add an explicit persistence flag that writes one validated
stewardship record through `LocalWorkflowCatalogStore`.
