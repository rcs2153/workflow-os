# Workflow Catalog Repair And Recovery Plan

## 1. Executive Summary

Workflow catalog persistence now has explicit local write paths for steward
review records, promotion catalog records, and archive metadata records. The
read-only `workflow-os author workflow catalog-status` command can already
surface conflicts across active workflows, drafts, archived drafts, catalog
records, stewardship records, and archive records.

The next question is how Workflow OS should help a maintainer recover from
missing, stale, or partially written catalog metadata without turning catalog
repair into silent cleanup.

This plan defines a conservative catalog repair and recovery path. The first
implementation should be non-mutating: it should inspect current catalog status
and produce bounded repair proposals. It must not rewrite workflow files,
delete records, register workflows, repair automatically, create runtime state,
or hide partial-integration failures.

## 2. Goals

- Define a safe local repair and recovery boundary for workflow catalog state.
- Preserve active workflow files as the execution source of truth.
- Preserve catalog records as durable lifecycle metadata, not runtime state.
- Turn catalog-status conflicts into explicit, reviewable repair proposals.
- Make partial promotion/archive sidecar failures easier to diagnose.
- Keep repair output deterministic, bounded, and redaction-safe.
- Avoid raw workflow YAML, source contents, command output, provider payloads,
  or secrets.
- Prepare for a future opt-in apply mode without authorizing it yet.

## 3. Non-Goals

Do not implement in this planning phase:

- catalog repair command behavior;
- catalog repair helper code;
- automatic cleanup;
- automatic record creation;
- record deletion;
- record overwrites;
- active workflow rewrites;
- draft or archive movement;
- runtime workflow registration;
- runtime state creation;
- workflow schema changes;
- examples;
- hosted or team catalog backend behavior;
- provider calls;
- command execution or local check execution;
- write-capable adapters or provider mutation;
- release posture changes.

## 4. Current Source-Of-Truth Boundaries

Workflow catalog recovery must keep source-of-truth boundaries explicit:

- Active workflow file: loader-visible execution source of truth.
- Draft workflow file: inactive proposal artifact.
- Archived draft file: inactive historical proposal artifact.
- Catalog record: local lifecycle sidecar for an active workflow.
- Stewardship record: local decision sidecar for authoring review.
- Archive record: local sidecar for an archived draft action.
- Catalog-status index: derived read-only view of files and sidecars.
- Runtime state: workflow execution state, not catalog state.
- Git history: optional review context, not a correctness requirement.

Repair must not let a sidecar record override loader validation. If an active
workflow file and catalog record disagree, the disagreement should be surfaced
as a conflict until a maintainer chooses an explicit recovery action.

## 5. Repair Vs Recovery Vs Status

`catalog-status` answers: what is inconsistent right now?

Recovery answers: what happened, what may be missing, and what should a
maintainer do next?

Repair answers: which explicit local sidecar changes could restore catalog
consistency?

The first implementation should stop at recovery and repair proposals. It
should not apply changes. This keeps the next phase useful while avoiding
unreviewed mutation semantics.

## 6. Candidate Conflict Inventory

The first repair planner should consume conflicts already produced by the
workflow catalog index helper. It should not invent a second conflict engine.

Candidate conflicts:

- duplicate active workflow id;
- duplicate active workflow path;
- active workflow missing catalog record;
- catalog record references missing active workflow file;
- catalog record active path mismatch;
- catalog record active hash mismatch;
- missing owner in catalog metadata;
- missing escalation contact in catalog metadata;
- missing latest stewardship decision in catalog metadata;
- missing side-effect posture in catalog metadata;
- draft stewardship hash mismatch;
- archive record references missing archived draft;
- archive path mismatch;
- archive hash mismatch;
- corrupt or unreadable catalog records;
- unsafe catalog root or unsafe sidecar paths.

The first proposal helper should classify each conflict as:

- `repairable_by_missing_record_creation`;
- `repairable_by_metadata_update`;
- `requires_manual_workflow_file_review`;
- `requires_manual_sidecar_review`;
- `requires_catalog_store_cleanup`;
- `not_repairable_by_first_slice`.

Only proposal classification is in scope for the first implementation.

## 7. First Implementation Target Recommendation

The smallest useful first implementation is:

```text
workflow-os author workflow catalog-repair --dry-run
```

This dry-run CLI surface is implemented in
[Workflow Catalog Repair Dry-Run CLI Report](../concepts/WORKFLOW_CATALOG_REPAIR_DRY_RUN_CLI_REPORT.md)
and accepted in
[Workflow Catalog Repair Dry-Run CLI Review](../concepts/WORKFLOW_CATALOG_REPAIR_DRY_RUN_CLI_REVIEW.md).
It remains proposal-only and does not implement apply mode, cleanup, deletion,
overwrite, workflow registration, runtime state, schemas, examples, hosted
behavior, providers, or release posture changes.

The command should:

- load the same inputs as `catalog-status`;
- build the existing `WorkflowCatalogIndex`;
- derive deterministic repair proposals from conflicts;
- print bounded human output;
- support JSON output through the existing global `--json` posture;
- exit non-zero only for unsafe/corrupt inputs or blocker conflicts where the
  selected strict mode requires it;
- create no files and mutate nothing.

If command naming is considered too early, the next implementation may instead
add a core helper and CLI tests only after review. The user-facing posture
should still be dry-run/proposal-only.

## 8. Allowed Inputs

Allowed inputs:

- loader-visible active workflow summaries;
- inactive draft summaries;
- archived draft summaries;
- local catalog records;
- local stewardship records;
- local archive records;
- explicit catalog root;
- strict catalog coverage flag;
- optional maintainer actor for proposal context;
- optional bounded reason for why repair planning was requested.

The repair planner may cite workflow ids, repository-relative paths, stable
record ids, content hashes, conflict kinds, and source categories.

## 9. Forbidden Inputs And Outputs

The repair planner must not read, store, copy, or emit:

- raw workflow YAML bodies;
- raw source file contents;
- command output;
- provider payloads;
- parser payloads;
- package script bodies;
- CI logs;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded maintainer reasons.

Repository-relative workflow and catalog paths may appear as bounded metadata,
matching existing catalog-status behavior.

## 10. Proposal Model

A future model may include:

- `WorkflowCatalogRepairProposal`;
- `WorkflowCatalogRepairProposalId`;
- `WorkflowCatalogRepairAction`;
- `WorkflowCatalogRepairActionKind`;
- `WorkflowCatalogRepairRisk`;
- `WorkflowCatalogRecoveryPosture`.

The first implementation does not need all of these types. It should add only
the smallest model needed to test deterministic proposal derivation.

Candidate action kinds:

- `CreateMissingCatalogRecord`;
- `UpdateCatalogRecordMetadata`;
- `CreateMissingArchiveRecord`;
- `ReviewCatalogRecordMismatch`;
- `ReviewArchiveRecordMismatch`;
- `ReviewStaleStewardshipDecision`;
- `ReviewDuplicateActiveWorkflow`;
- `ReviewCorruptStoreRecord`;
- `NoAutomaticRepairAvailable`.

Every proposal should include:

- stable proposal id;
- conflict kind;
- source category;
- bounded source reference;
- proposed action kind;
- whether the proposal is safe for a future apply mode;
- whether human review is required;
- redaction metadata and sensitivity;
- explicit non-goal text when no automatic repair is available.

## 11. Recovery Semantics

Recovery should be conservative:

- partial promotion/archive integration errors remain explicit;
- no automatic rollback is attempted;
- no catalog sidecar is deleted in the first slice;
- no catalog sidecar is overwritten in the first slice;
- no active workflow file is modified;
- no archived draft is moved;
- no runtime state is created;
- no event history is appended;
- no workflow is registered outside loader-visible files.

If a catalog record is missing for an active workflow, a future apply mode may
create a record only after active workflow validation succeeds and all required
bounded metadata can be derived safely.

If a catalog record conflicts with an active workflow hash or path, the first
planner should require manual review. It should not guess whether the workflow
or sidecar is authoritative.

If an archive record conflicts with an archived draft, the first planner should
require manual review. It should not move or delete archived drafts.

## 12. Error Handling

Errors must use stable, non-leaking codes.

Recommended future codes:

- `cli.workflow_catalog.repair_catalog_root_rejected`;
- `cli.workflow_catalog.repair_read_failed`;
- `cli.workflow_catalog.repair_index_failed`;
- `cli.workflow_catalog.repair_proposal_failed`;
- `cli.workflow_catalog.repair_blocked`;

Errors must not echo unsafe catalog-root values, raw record payloads, raw YAML,
source snippets, command output, provider payloads, paths outside bounded
repository-relative metadata, or secret-like values.

Invalid or corrupt store records should fail closed before proposal generation,
unless a separately reviewed corruption-summary mode can report only bounded
record-address metadata.

## 13. Privacy And Redaction

Repair proposals may be sensitive even when they are read-only because they can
reveal workflow ownership, authority, lifecycle, and governance gaps.

Rules:

- use validated model constructors for proposal records;
- keep proposal summaries bounded;
- store only ids, repository-relative paths, conflict kinds, hashes, and
  posture labels;
- require explicit redaction metadata;
- default sensitivity conservatively;
- keep Debug, Display, JSON, and errors safe;
- avoid raw workflow YAML, command output, provider payloads, parser payloads,
  and secret-like values.

## 14. Relationship To Catalog Status

The repair planner should depend on `catalog-status` inputs and the reviewed
index helper.

It should not replace `catalog-status`. Instead:

- `catalog-status` remains the health/inventory command;
- repair planning translates conflicts into proposed maintainer actions;
- future apply mode may consume repair proposals only after separate review.

`catalog-status --strict-catalog-coverage` should remain status-only until an
apply mode is reviewed.

## 15. Relationship To Promotion And Archive

Promotion and archive commands now have opt-in sidecar write paths. Their
partial-integration behavior intentionally avoids rollback.

Repair planning should make that posture easier to operate:

- after a promotion catalog write failure, repair planning may propose creating
  the missing catalog record if active workflow validation still passes;
- after an archive record write failure, repair planning may propose creating
  the missing archive record if the archived draft still exists and hash checks
  match;
- stale stewardship or hash mismatch cases should require manual review;
- duplicate active workflow cases should not be auto-repaired.

## 16. Relationship To Future Stores

This plan targets the local file-backed catalog store only.

Future team or hosted stores may need:

- optimistic concurrency;
- signed or actor-scoped repair proposals;
- organization-level stewardship rules;
- audit events for repair proposal review;
- durable WorkReports for repair outcomes;
- permission checks for apply modes.

Those are out of scope for this local planning phase.

## 17. Test Plan

Future implementation tests should cover:

- dry-run repair proposal for missing active catalog record;
- dry-run proposal for catalog active path mismatch;
- dry-run proposal for catalog active hash mismatch;
- dry-run proposal for missing archive record when archived draft exists;
- dry-run proposal for archive path mismatch;
- dry-run proposal for archive hash mismatch;
- duplicate active workflow ids require manual review;
- duplicate active workflow paths require manual review;
- corrupt catalog store fails closed without leaking payloads;
- unsafe catalog root rejection without leakage;
- no files are written in dry-run;
- no workflow files are modified;
- no runtime state is created;
- no event history is appended;
- proposal ordering is deterministic;
- JSON output is bounded and stable;
- Debug output is redaction-safe;
- existing catalog-status tests still pass;
- existing promotion/archive/stewardship tests still pass;
- docs check passes.

## 18. Proposed Implementation Sequence

1. Review this repair and recovery plan.
2. Add a non-mutating repair proposal model/helper.
3. Add focused core tests for proposal classification and non-leakage.
4. Add a read-only `author workflow catalog-repair --dry-run` command if the
   helper review accepts the surface.
5. Review the dry-run command before any apply mode.
6. Plan opt-in apply mode for only the safest missing-record cases.
7. Defer deletion, overwrite, rollback, hosted repair, schema exposure,
   examples, runtime registration, and automatic cleanup.

## 19. Deferred Work

- automatic catalog repair;
- catalog record deletion;
- catalog record overwrite/update apply mode;
- archive record deletion;
- active workflow rewrites;
- draft/archive movement;
- rollback after partial promotion/archive integration;
- runtime event/audit emission for repair operations;
- WorkReport artifact generation for repair outcomes;
- workflow schema fields;
- examples;
- hosted/team catalog stores;
- RBAC, IdP, notifications, or admin UI;
- write-capable adapters or provider mutation.

## 20. Final Recommendation

Proceed next to a maintainer review of this repair and recovery plan. If the
plan is accepted, implement the smallest non-mutating repair proposal helper or
dry-run command surface.

Do not implement automatic cleanup, record deletion, record overwrites, runtime
registration, schema changes, examples, hosted behavior, provider calls,
write-capable adapters, or release posture changes.
