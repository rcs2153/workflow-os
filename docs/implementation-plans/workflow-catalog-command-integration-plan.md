# Workflow Catalog Command Integration Plan

Status: Planned. This plan follows the accepted
[Workflow Catalog Indexing Conflict Helper Blocker Fix Review](../concepts/WORKFLOW_CATALOG_INDEXING_CONFLICT_HELPER_BLOCKER_FIX_REVIEW.md).
It does not implement command behavior.

## 1. Executive Summary

Workflow catalog records, the local catalog store helper, and the pure
in-memory catalog indexing/conflict helper are now implemented and reviewed.
The next question is how the CLI should expose that helper in a small,
non-mutating command path.

The first command integration should be a local catalog status/health surface.
It should consume loader-visible workflow files, inactive drafts, archived
drafts, and optional local catalog store records, then print a bounded inventory
and conflict summary.

This plan does not implement the command. It does not add runtime workflow
registration, catalog writes, promotion enforcement, archive metadata writes,
schemas, examples, hosted behavior, side effects, writes, or release posture
changes.

## 2. Goals

- Add a future CLI command surface that consumes `build_workflow_catalog_index`.
- Keep active workflow files as the execution source of truth.
- Report catalog coverage and conflicts without mutating workflow files.
- Disclose missing catalog records, stale catalog records, and warning-level
  stewardship gaps.
- Prepare future promotion, steward-review, archive, and catalog health
  enforcement.
- Keep output bounded, deterministic, and redaction-safe.
- Preserve existing authoring commands and validation semantics.

## 3. Non-Goals

Do not implement in this command-integration planning phase:

- command code;
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

The future first implementation should also keep these out of scope unless a
separate plan explicitly approves them.

## 4. Current Foundation

Implemented and reviewed:

- `WorkflowCatalogRecord`, `WorkflowStewardshipRecord`, and
  `WorkflowArchiveRecord`;
- `LocalWorkflowCatalogStore`;
- deterministic record listing and health summaries;
- safe encoded local catalog file names;
- duplicate-write rejection;
- `WorkflowCatalogActiveWorkflowSummary`;
- `WorkflowCatalogDraftSummary`;
- `WorkflowCatalogArchivedDraftSummary`;
- `WorkflowCatalogIndexInput`;
- `WorkflowCatalogIndex`;
- `WorkflowCatalogConflict`;
- `build_workflow_catalog_index`;
- serde validation boundary hardening for exported helper summary and conflict
  types.

Existing authoring CLI surfaces:

- `workflow-os author workflow --from-recommendation <id> --dry-run`;
- `workflow-os author workflow --from-recommendation <id> --output ...`;
- `workflow-os author workflow preflight --draft ...`;
- `workflow-os author workflow steward-review --draft ...`;
- `workflow-os author workflow promote --draft ...`;
- `workflow-os author workflow draft-status --draft ...`;
- `workflow-os author workflow archive-draft --draft ...`.

Current gaps:

- no command consumes the catalog index helper;
- no catalog status command exists;
- no command reads local catalog records for conflict disclosure;
- no promotion or archive command writes catalog records;
- no strict catalog enforcement is wired into authoring flows.

## 5. Recommended First Command Surface

Recommended first command:

```text
workflow-os author workflow catalog-status [--catalog-root <path>] [--strict-catalog-coverage] [--json]
```

Rationale:

- It stays close to the existing `author workflow` command family.
- It avoids introducing a broader top-level catalog product surface too early.
- It can remain non-mutating and review-only.
- It gives users immediate visibility into active workflows, drafts, archived
  drafts, persisted catalog metadata, and deterministic conflicts.

Alternative:

```text
workflow-os catalog status [--catalog-root <path>] [--strict-catalog-coverage] [--json]
```

This may become the better public shape once catalog commands include record
write, health, export, or stewardship subcommands. For the first slice, keeping
the surface under `author workflow` reduces product overclaim.

## 6. Input Discovery Boundary

The first command should derive explicit helper inputs from existing local
sources:

- loaded active workflows from the project bundle;
- inactive draft files under `workflows/drafts/`;
- archived draft files under `workflows/drafts/archive/`;
- local catalog records from `LocalWorkflowCatalogStore`, when a catalog root
  exists or is supplied;
- local stewardship records from the same store;
- local archive records from the same store;
- optional strict catalog coverage flag.

Allowed reads:

- workflow and draft YAML needed to parse ids and content hashes through
  existing loader/authoring parsing paths;
- catalog JSON records through `LocalWorkflowCatalogStore`;
- directory entries for known workflow/draft/catalog directories.

Forbidden reads and copies:

- arbitrary source file contents;
- manifest bodies beyond existing validated metadata;
- command output;
- provider payloads;
- CI logs;
- environment values;
- credentials, tokens, authorization headers, or private keys.

## 7. Output Policy

Human output should include:

- mode: `workflow_catalog_status`;
- active workflow count;
- draft count;
- archived draft count;
- catalog record count;
- stewardship record count;
- archive record count;
- blocker conflict count;
- warning conflict count;
- bounded conflict codes;
- next action.

JSON output should include the same bounded fields, plus deterministic arrays of
conflict summaries.

The command must not print:

- raw workflow YAML;
- draft YAML;
- raw catalog JSON;
- raw reason summaries;
- raw redaction metadata values;
- absolute private paths;
- command output;
- provider payloads;
- secrets or secret-like values.

## 8. Conflict Enforcement Posture

The first command should be status-only.

Recommended exit behavior:

- exit success when no blocker conflicts exist;
- exit non-zero when blocker conflicts exist;
- keep warning conflicts non-fatal by default;
- use `--strict-catalog-coverage` to turn missing active catalog coverage into
  blocker conflicts through the existing helper input flag.

This does not mean promotion or archive commands enforce catalog status yet.
Future phases must explicitly wire catalog conflict checks into those command
paths.

## 9. Store Boundary

The first command may read a local catalog store but must not write one.

Recommended catalog root policy:

- default to `.workflow-os/catalog` under the project root if it exists;
- allow `--catalog-root <path>` for explicit local testing;
- if no catalog root exists, proceed with empty catalog records and disclose
  `catalog_store: not_available`;
- reject unsafe, absolute, or traversal-shaped supplied roots unless existing
  repository path helpers already safely normalize them.

The command must not create `.workflow-os/catalog` merely by inspecting status.

## 10. Relationship To Existing Authoring Commands

The first catalog status command should not change existing authoring behavior.

Existing commands should remain unchanged:

- `preflight` continues to validate one draft without catalog store
  enforcement;
- `steward-review` continues to print a bounded preview without persisted
  approval;
- `promote` continues to explicitly write one active workflow file and reload
  validation;
- `draft-status` continues to inspect one draft;
- `archive-draft` continues to move one eligible draft without writing archive
  metadata.

Future phases may wire catalog conflict checks into promotion and archive, but
that requires separate implementation and review.

## 11. Error Handling

Errors should be stable and non-leaking.

Recommended stable error families:

- `cli.workflow_catalog.status_project_missing`;
- `cli.workflow_catalog.catalog_root_rejected`;
- `cli.workflow_catalog.catalog_read_failed`;
- `cli.workflow_catalog.draft_read_failed`;
- `cli.workflow_catalog.draft_parse_failed`;
- `cli.workflow_catalog.status_blocked`.

Errors must not echo unsafe paths, raw YAML, raw JSON, record ids containing
secret-like markers, or private root paths.

When catalog records cannot be read because the root is absent, the command
should disclose `not_available` rather than fail. When records exist but are
corrupt or invalid, the command should fail closed.

## 12. Privacy And Redaction

The command must use existing validated constructors:

- `WorkflowCatalogActiveWorkflowSummary::new`;
- `WorkflowCatalogDraftSummary::new`;
- `WorkflowCatalogArchivedDraftSummary::new`;
- `LocalWorkflowCatalogStore` read/list methods;
- `build_workflow_catalog_index`.

The command should only print bounded ids, counts, safe relative paths where
already accepted by the helper, and conflict codes.

It must not copy raw payloads into errors, Debug output, JSON output, or human
output.

## 13. Test Plan

Future implementation tests should cover:

- valid project with no catalog root reports active workflow inventory;
- `--json` output is bounded and deterministic;
- absent catalog root is disclosed as not available and does not create files;
- existing catalog root records are read through `LocalWorkflowCatalogStore`;
- strict catalog coverage returns non-zero when active workflows lack catalog
  records;
- warning-only conflicts remain non-fatal by default;
- corrupt catalog records fail closed without leaking file contents;
- unsafe catalog root is rejected without leaking the supplied path;
- inactive drafts are counted without registering workflows;
- archived drafts are counted without moving or deleting files;
- blocker conflict codes are printed without raw YAML or raw catalog JSON;
- command does not write catalog files, runtime state, report artifacts, or
  workflow files;
- existing authoring command tests continue to pass;
- existing workflow catalog index and store tests continue to pass;
- docs check passes.

## 14. Proposed Implementation Sequence

Recommended small phases:

1. Add bounded internal CLI helper functions that derive active, draft, archived
   draft, and optional store inputs for `build_workflow_catalog_index`.
2. Add `workflow-os author workflow catalog-status --json` and human output as a
   non-mutating status command.
3. Add focused CLI tests for no-store, strict-coverage, corrupt-store, and
   non-mutating behavior.
4. Review the status command before wiring catalog status into promotion or
   archive.
5. Plan promotion/archive catalog enforcement separately.

## 15. Deferred Work

Explicitly deferred:

- top-level `workflow-os catalog ...` command family;
- catalog record writes from promotion;
- stewardship record writes from steward review;
- archive record writes from archive command;
- strict promotion enforcement from catalog status;
- automatic catalog repair;
- catalog export/import;
- workflow schema changes;
- examples;
- hosted/team catalog backend;
- RBAC, IdP, steward groups, notifications, and admin UI;
- provider calls;
- write-capable adapters;
- release posture changes.

## 16. Open Questions

- Should the first public command live under `author workflow` or should a
  top-level `catalog` namespace be introduced now?
- Should warning conflicts ever produce a non-zero exit in v0, or only under a
  future `--strict` flag?
- Should `.workflow-os/catalog` be considered present only when the directory
  exists, or should an empty root be virtualized as empty state?
- Should draft parsing reuse the current authoring preflight loader exactly, or
  add a lighter inventory-only loader?
- When catalog writes are eventually implemented, should they be committed to
  Git by default or remain local state unless explicitly opted in?

## 17. Final Recommendation

Next implementation phase: `workflow-os author workflow catalog-status`,
non-mutating status command only.

The command should consume explicit local project and optional catalog-store
inputs, call the already reviewed index helper, report bounded inventory and
conflicts, and write nothing.

It must still not implement catalog writes, promotion enforcement, archive
metadata writes, runtime workflow registration, schemas, examples, hosted
behavior, side effects, writes, or release posture changes.

## 18. Dogfood Governance

```text
workflow_id: dg/d
run_id: run-1783524910525301000-2
approval_id: approval/run-1783524910525301000-2/planning-approved
approval_outcome: granted
events_total: 39
event_summary: ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8, RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6, SkillInvocationSucceeded:6, StepScheduled:6
```

The planning phase was approved before this artifact was written. The kernel
governed the phase boundary; document authoring, validation commands, git
operations, and PR operations were performed outside the kernel and must be
disclosed in phase handoffs.

## 19. Validation

```text
npm run check:docs
```

Result: passed.
