# Workflow Catalog Indexing And Conflict Helper Plan

Status: Implemented as a pure in-memory helper in
[Workflow Catalog Indexing Conflict Helper Report](../concepts/WORKFLOW_CATALOG_INDEXING_CONFLICT_HELPER_REPORT.md).
Reviewed in
[Workflow Catalog Indexing Conflict Helper Review](../concepts/WORKFLOW_CATALOG_INDEXING_CONFLICT_HELPER_REVIEW.md);
the review found a serde validation-bypass blocker that must be fixed before
command integration. The blocker fix is documented in
[Workflow Catalog Indexing Conflict Helper Blocker Fix Report](../concepts/WORKFLOW_CATALOG_INDEXING_CONFLICT_HELPER_BLOCKER_FIX_REPORT.md)
and accepted in
[Workflow Catalog Indexing Conflict Helper Blocker Fix Review](../concepts/WORKFLOW_CATALOG_INDEXING_CONFLICT_HELPER_BLOCKER_FIX_REVIEW.md).
Command integration, runtime workflow registration, automatic promotion,
catalog command writes, schemas, examples, providers, hosted behavior,
write-capable adapters, and release posture changes remain unimplemented.

## 1. Executive Summary

Workflow catalog model types and the first local catalog store helper are now
implemented and reviewed. The store can persist validated catalog,
stewardship, and archive metadata records under an explicit local root, but it
does not yet derive an index or detect conflicts across active workflows,
drafts, archive records, and persisted catalog metadata.

The next phase should add a pure in-memory indexing and conflict helper. The
helper should consume explicit inputs from loader-visible workflow files,
inactive drafts, archived drafts, and catalog store records, then return a
bounded index plus conflict disclosures. It should not mutate files, write
catalog records, register workflows with the runtime, promote drafts, archive
drafts, run checks, call providers, change schemas, or introduce hosted
behavior.

This plan originally scoped the helper before implementation. The first helper
slice is now implemented as core model/helper code only.

## 2. Goals

- Define a deterministic in-memory workflow catalog index shape.
- Detect exact catalog/workflow/draft/archive conflicts before command wiring.
- Preserve loader-visible active workflow files as the execution source of
  truth.
- Use persisted catalog records as lifecycle metadata, not as runtime
  registration.
- Surface stale, missing, duplicate, and mismatched records with stable
  non-leaking codes.
- Distinguish hard blockers from warning-only semantic overlap disclosures.
- Prepare future steward-review, promotion, archive, and catalog health command
  integration.
- Keep all outputs bounded, reference-oriented, and redaction-safe.

## 3. Non-Goals

Do not implement in the indexing/conflict helper phase:

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

## 4. Current Foundation

Implemented foundations:

- workflow catalog, stewardship, and archive core model types;
- `LocalWorkflowCatalogStore`;
- write/read/list operations for catalog records;
- write/read/list operations for stewardship records;
- write/read/list operations for archive records;
- safe encoded file names derived from canonical ids;
- duplicate-write rejection;
- deterministic store listing;
- bounded health summary;
- redaction-safe errors and Debug behavior.

Current authoring surfaces:

- `workflow-os first-run` recommendations;
- inactive draft proposal and file output;
- promotion preflight;
- steward-review preview;
- explicit active promotion;
- draft status inspection;
- explicit archive command for eligible promoted/superseded drafts.

Current gap:

- no in-memory catalog index;
- no cross-source conflict helper;
- no command consumes catalog records;
- no persisted stewardship decision is required by promotion;
- no archive command writes archive metadata;
- no catalog health command exists.

## 5. Source Inputs

The first helper should accept explicit source inputs rather than discovering
global state internally.

Recommended input groups:

- active workflow summaries derived from loader-visible workflow files;
- inactive draft summaries derived from `workflows/drafts/`;
- archived draft summaries derived from `workflows/drafts/archive/`;
- `WorkflowCatalogRecord` values from the local store;
- `WorkflowStewardshipRecord` values from the local store;
- `WorkflowArchiveRecord` values from the local store;
- optional project root display policy, if needed for redacted output.

The helper should not read arbitrary source contents. Callers may provide
content hashes, workflow ids, and repository-relative paths from existing
validated loader/authoring surfaces.

## 6. Candidate Helper API

Possible model and helper names:

- `WorkflowCatalogIndexInput`
- `WorkflowCatalogIndex`
- `WorkflowCatalogIndexedWorkflow`
- `WorkflowCatalogIndexedDraft`
- `WorkflowCatalogIndexedArchive`
- `WorkflowCatalogConflict`
- `WorkflowCatalogConflictKind`
- `WorkflowCatalogConflictSeverity`
- `WorkflowCatalogConflictSource`
- `build_workflow_catalog_index`
- `detect_workflow_catalog_conflicts`

The smallest implementation can combine index construction and conflict
detection into one pure helper:

```text
build_workflow_catalog_index(input) -> Result<WorkflowCatalogIndex, WorkflowOsError>
```

`WorkflowCatalogIndex` should expose read-only accessors for:

- active workflow entries;
- draft entries;
- archived draft entries;
- catalog records;
- stewardship records;
- archive records;
- conflict disclosures;
- bounded counts by lifecycle and conflict severity.

## 7. Index Entries

Active workflow index entries should include:

- workflow id;
- repository-relative workflow path;
- workflow content hash;
- schema version, if already available;
- optional matching catalog record id;
- optional latest stewardship decision id;
- optional latest archive record id;
- lifecycle posture: active, missing catalog, stale catalog, or mismatch.

Draft index entries should include:

- workflow id;
- repository-relative draft path;
- draft content hash;
- draft status if already derived by existing authoring helpers;
- optional source recommendation id if already available;
- optional matching catalog record id;
- lifecycle posture: candidate, promoted-preserved, superseded, archived, or
  unknown.

Archive index entries should include:

- workflow id;
- original draft path;
- archive path;
- draft content hash;
- optional active workflow path/hash;
- optional archive record id;
- lifecycle posture: archived, archive-record-missing, or archive-record-stale.

Catalog record index entries should include:

- catalog record id;
- workflow id;
- workflow path;
- workflow content hash;
- lifecycle status;
- optional source draft path;
- optional archived draft path;
- optional latest stewardship decision id;
- optional latest promotion decision id;
- optional latest archive record id;
- bounded posture fields by code or count only.

## 8. Conflict Taxonomy

The first helper should distinguish blockers from warnings.

Hard blocker candidates:

- duplicate active workflow id;
- duplicate active workflow path;
- active workflow file has no matching catalog record when catalog enforcement
  is explicitly requested;
- catalog record references missing active workflow file while marked active;
- catalog active path differs from active workflow path for the same workflow
  id;
- catalog active content hash differs from active workflow content hash;
- draft content hash differs from associated stewardship candidate hash;
- archive record references missing archive path;
- archive record path/hash differs from archived draft summary;
- archived draft is being considered for promotion without explicit new review
  input;
- record identity mismatch already detected by the store.

Warning-only candidates:

- workflow file has no catalog record when catalog enforcement is not required;
- catalog record exists for workflow not currently loader-visible;
- source recommendation already has an active workflow;
- draft is superseded by an active workflow;
- owner or escalation contact missing from catalog record;
- authority scope missing or overlapping by exact matching text;
- evidence/check/report posture missing or conflicting by exact matching text;
- side-effect posture missing or conflicting by exact matching text;
- latest stewardship decision id is missing;
- latest archive record id is missing for archived lifecycle.

Semantic overlap, such as "similar purpose" or "same business process", should
remain warning-only or deferred until the taxonomy is stronger. The first helper
must not rely on model-generated similarity judgments.

## 9. Severity Policy

Recommended severities:

- `blocker`: deterministic identity, path, hash, lifecycle, or record mismatch
  that would make promotion/archive/catalog use unsafe;
- `warning`: missing catalog coverage, missing stewardship metadata, or exact
  posture overlap that should be reviewed but should not block by default;
- `info`: bounded inventory facts useful to a future catalog health command.

The helper should not decide command outcome by itself. It should return
conflicts and let future command integration choose enforcement policy.

## 10. Determinism And Ordering

Index construction must be stable.

Rules:

- sort active workflow entries by workflow id then path;
- sort drafts by workflow id then draft path;
- sort archives by workflow id then archive path;
- sort catalog records by catalog record id;
- sort stewardship records by decision id;
- sort archive records by archive record id;
- sort conflicts by severity, kind, workflow id, and source reference.

No output ordering should depend on filesystem traversal order, hash-map order,
or caller input order.

## 11. Privacy And Redaction

The helper must not store or print:

- raw workflow YAML;
- raw draft YAML;
- source contents;
- manifest bodies;
- package script bodies;
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
- unbounded reviewer reasons;
- existing agent instruction bodies.

Allowed output:

- workflow ids;
- repository-relative paths that already pass path validation;
- content hashes;
- lifecycle status codes;
- decision ids;
- archive record ids;
- bounded conflict kind/severity codes;
- bounded counts;
- stable references to existing catalog/stewardship/archive records.

Debug output should redact or count optional free-text posture summaries rather
than echoing them.

## 12. Error Handling

Validation errors should use stable non-leaking codes.

Candidate codes:

- `workflow_catalog_index.input.invalid`
- `workflow_catalog_index.duplicate_active_workflow`
- `workflow_catalog_index.duplicate_active_path`
- `workflow_catalog_index.catalog_missing_active_file`
- `workflow_catalog_index.catalog_path_mismatch`
- `workflow_catalog_index.catalog_hash_mismatch`
- `workflow_catalog_index.draft_hash_mismatch`
- `workflow_catalog_index.archive_missing_path`
- `workflow_catalog_index.archive_hash_mismatch`
- `workflow_catalog_index.secret_like_input`

Errors must not include raw paths beyond already validated repository-relative
display paths, raw YAML, raw reason text, private path prefixes, command output,
provider payloads, parser payloads, or secret-like values.

When possible, prefer returning `WorkflowCatalogConflict` disclosures over
failing index construction. Reserve errors for invalid helper input or unsafe
values that cannot be represented safely.

## 13. Relationship To Store Helper

The indexing helper should consume records loaded by `LocalWorkflowCatalogStore`
but should not own storage.

Rules:

- do not write catalog records;
- do not update catalog records;
- do not write stewardship records;
- do not write archive records;
- do not mutate store files;
- map store read/list failures outside the helper or through explicit input
  construction;
- treat already-loaded records as the indexing boundary.

This keeps the helper portable across future local, team, or hosted store
backends.

## 14. Relationship To Authoring Commands

Future command integration may use the index helper for:

- `author workflow preflight`;
- `author workflow steward-review`;
- `author workflow promote`;
- `author workflow draft-status`;
- `author workflow archive-draft`;
- a future `workflow-os catalog status` command.

This phase should not wire any of those commands. Command integration needs a
separate plan because each command must decide whether conflicts are blockers,
warnings, or report-only disclosures.

## 15. Relationship To Workflow Discovery

The catalog index should eventually help Workflow OS recommend workflow changes:

- workflows to create;
- workflows to split;
- workflows to merge;
- workflows to retire;
- workflows missing owner/escalation;
- workflows missing evidence/check/report obligations;
- workflows with stale side-effect posture;
- workflows with repeated failed checks or repeated manual overrides.

The first helper should not generate or mutate workflows. It should produce
deterministic conflict and inventory facts that later recommendation layers can
cite.

## 16. Relationship To Local And Enterprise Use

For a single local user, the helper should support low-friction automation by
making catalog gaps visible without requiring a hosted database or admin layer.

For enterprise use, the same index should prepare future steward/admin
features:

- required owner assignment;
- required escalation contact;
- required catalog coverage;
- required stewardship decision before promotion;
- conflict review before activation;
- department or team workflow lifecycle review;
- eventually a shared store instead of Git-as-database.

The first helper must not claim RBAC, IdP, hosted stewardship, notifications, or
production collaboration.

## 17. Test Plan For Future Implementation

Future tests should cover:

- valid empty index construction;
- active workflow summaries indexed deterministically;
- draft summaries indexed deterministically;
- archive summaries indexed deterministically;
- catalog records indexed deterministically;
- stewardship records indexed deterministically;
- archive records indexed deterministically;
- duplicate active workflow ids produce blocker conflict;
- duplicate active workflow paths produce blocker conflict;
- active workflow without catalog record produces warning by default;
- active workflow without required catalog coverage produces blocker when input
  policy requests strict catalog coverage;
- catalog record missing active file produces blocker;
- catalog path mismatch produces blocker;
- catalog content hash mismatch produces blocker;
- stale draft stewardship hash produces blocker;
- archive path/hash mismatch produces blocker;
- missing owner/escalation produces warning;
- exact authority/evidence/side-effect posture overlap produces warning;
- semantic overlap is not model-inferred;
- conflicts are sorted deterministically;
- debug output does not leak posture text or private paths;
- secret-like posture text is rejected or represented only by existing validated
  model records;
- no raw YAML, source content, command output, provider payload, parser payload,
  or existing agent instructions are copied;
- existing catalog store tests still pass;
- existing authoring command tests still pass;
- docs check passes.

## 18. Proposed Implementation Sequence

Recommended phases:

1. Implement pure in-memory catalog index and conflict model/helper.
2. Add focused unit tests for deterministic indexing and exact blocker/warning
   conflicts.
3. Review the helper.
4. Plan command integration for read-only catalog status/preflight disclosure.
5. Integrate conflict disclosures into preflight or steward-review as
   warning-only first.
6. Separately plan strict enforcement for promotion.
7. Separately plan persisted stewardship decision writes.
8. Separately plan catalog record writes from promotion and archive commands.

The first implementation should start with helper/model code only.

## 19. Documentation Updates For Future Implementation

Future implementation should update:

- `ROADMAP.md`;
- this plan;
- `docs/implementation-plans/workflow-catalog-persistence-plan.md`;
- `docs/implementation-plans/governed-workflow-authoring-catalog-stewardship-plan.md`;
- relevant concept/report/review files.

Docs must say:

- in-memory catalog indexing/conflict helper is implemented;
- command integration is not implemented unless separately scoped;
- runtime workflow registration is not implemented;
- catalog files are not automatically written by authoring commands unless
  separately scoped;
- schemas, examples, providers, hosted behavior, writes, and release posture
  changes remain unimplemented.

## 20. Final Recommendation

Recommended next phase: workflow catalog indexing/conflict helper blocker fix
review.

The helper should produce deterministic catalog inventory and conflict
disclosures from explicit inputs. It should not mutate files, write catalog
records, register workflows, run commands, call providers, change schemas,
create examples, enable hosted behavior, or change release posture.
