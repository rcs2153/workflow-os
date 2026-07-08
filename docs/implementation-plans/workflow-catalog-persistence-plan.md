# Workflow Catalog Persistence And Stewardship Integration Plan

## 1. Executive Summary

Workflow catalog and stewardship core model types now exist. The next question
is how Workflow OS should persist those records locally and connect them to
authoring, steward review, promotion, archive, and conflict checks without
turning loader-visible workflow files into an opaque database.

This plan originally scoped catalog persistence before implementation. The
first local store-helper slice is now implemented as a file-backed,
model-backed helper under `workflow-core` and accepted in
[Workflow Catalog Store Helper Review](../concepts/WORKFLOW_CATALOG_STORE_HELPER_REVIEW.md).
Workflow catalog indexing and conflict helper planning is documented in
[Workflow Catalog Indexing And Conflict Helper Plan](workflow-catalog-indexing-conflict-plan.md).
The first pure in-memory indexing/conflict helper is implemented in
[Workflow Catalog Indexing Conflict Helper Report](../concepts/WORKFLOW_CATALOG_INDEXING_CONFLICT_HELPER_REPORT.md).
The first command integration boundary is implemented as the read-only
`workflow-os author workflow catalog-status` command and accepted in
[Workflow Catalog Status Command Review](../concepts/WORKFLOW_CATALOG_STATUS_COMMAND_REVIEW.md).
That review recommended returning to this plan for command write integration.
The planning refresh is documented in
[Workflow Catalog Persistence Integration Plan Report](../concepts/WORKFLOW_CATALOG_PERSISTENCE_INTEGRATION_PLAN_REPORT.md).

The next implementation lane should persist catalog records from explicit
authoring commands, but only after the write semantics are made narrow and
auditable. Runtime workflow registration, automatic catalog repair, workflow
schema changes, examples, provider calls, write-capable adapters, hosted
collaboration, RBAC, IdP integration, deletion behavior, and release posture
changes remain unimplemented.

## 2. Goals

- Define a local persistence boundary for workflow catalog records.
- Preserve active workflow files as the execution source of truth.
- Persist reference-oriented stewardship and archive metadata.
- Make future promotion/archive integration deterministic and auditable.
- Prepare conflict detection without blocking current authoring behavior.
- Keep catalog records bounded, redaction-safe, and repository-relative.
- Support single-user local automation and future enterprise stewardship.
- Avoid raw YAML, source contents, command output, provider payloads, or secrets.

## 3. Non-Goals

This plan is for integration planning only. Do not implement in this phase:

- command behavior changes;
- catalog record writes;
- stewardship record writes;
- archive record writes;
- catalog status enforcement;
- runtime workflow registration;
- automatic workflow generation;
- automatic promotion;
- automatic archive cleanup;
- draft deletion;
- workflow schema changes;
- examples;
- provider calls;
- command execution or local check execution;
- hosted/distributed behavior;
- RBAC, IdP integration, notifications, or admin UI;
- write-capable adapters or provider mutation;
- release posture changes.

## 4. Source-Of-Truth Boundaries

Workflow OS should keep source-of-truth boundaries explicit:

- Active workflow spec file: loader-visible execution source of truth.
- Draft workflow file: inactive proposal artifact under `workflows/drafts/`.
- Archived draft file: inactive historical proposal under
  `workflows/drafts/archive/`.
- Catalog record: durable local index of workflow lifecycle, owner, paths,
  content hashes, posture summaries, and latest references.
- Stewardship record: durable local record of a review, approval, rejection,
  promotion, supersession, or archive decision.
- Archive record: durable local metadata for an archive action.
- Runtime state: execution event log and run state, not workflow catalog state.
- WorkReport: terminal work handoff artifact, not catalog state.

The catalog may cite workflow files and content hashes. It must not replace
loader validation or become the only place where active workflows exist.

## 5. Recommended Local Storage Layout

Recommended first storage layout:

```text
.workflow-os/catalog/
  workflows/
    <safe-catalog-record-id>.json
  stewardship/
    <safe-stewardship-decision-id>.json
  archives/
    <safe-archive-record-id>.json
  index.json
```

Rationale:

- `.workflow-os/` is already the Workflow OS local control plane.
- `catalog/` separates authoring lifecycle metadata from runtime state.
- JSON can use the existing serde model while schema exposure remains deferred.
- Splitting records by kind avoids one large mutable registry file.
- `index.json` can remain derived or lightweight in the first implementation.

Open posture:

- Whether these files are committed to Git should be a user/team policy choice.
- The first implementation should not silently add generated catalog files to
  Git.
- The storage API should be replaceable by future team or hosted backends.

## 6. Record Identity And File Naming

The first store should use validated model ids to derive safe file names.

Rules:

- do not use raw workflow ids directly as paths without encoding;
- do not allow `..`, absolute paths, path separators outside the encoding policy,
  or platform-specific prefixes;
- keep file names bounded;
- reject collisions deterministically;
- do not leak rejected raw ids in errors.

Recommended first approach:

- derive a deterministic slug from the validated id using the same safe
  character policy or a reversible percent/base encoding;
- keep the canonical id inside the record;
- treat the file name as storage address, not the durable identity.

## 7. Write Timing Policy

Recommended first write boundaries:

1. Steward-review command may write a stewardship record only when explicitly
   requested in a future phase.
2. Promotion command may require a persisted approved stewardship decision only
   in an explicit opt-in phase.
3. Promotion command may write or update a catalog record only after active
   workflow validation succeeds.
4. Archive command may write an archive record only after the draft is eligible
   and the move succeeds.
5. Failed validation must not emit partial catalog records.
6. Dry-run must never write catalog files.

The first command write integration should not start with promotion. It should
start with persisted stewardship because that command already produces a
bounded review decision without moving workflow files. Persisting the decision
first creates a durable reference that later promotion and archive commands can
cite without making active workflow mutation depend on an unproven sidecar write
path.

Recommended command-write order:

1. Add opt-in steward-review persistence.
2. Review persisted stewardship.
3. Add opt-in promotion catalog-record write that can cite a persisted
   stewardship decision.
4. Review promotion catalog write semantics.
5. Add archive metadata write after successful archive moves.
6. Review archive metadata semantics before any deletion or catalog repair.

## 8. Promotion Integration Policy

Future promotion integration should:

- derive fresh preflight context before promotion;
- require candidate content hash match;
- require explicit reviewer/reason input;
- optionally require or cite a persisted approved stewardship decision;
- validate the active-placement candidate before writing;
- write exactly one active workflow file;
- reload validation after the write;
- write or update the catalog record only after active validation succeeds;
- cite the stewardship decision and promotion content hash;
- fail closed on stale draft hash, duplicate active workflow id, or catalog
  identity conflict.

Promotion must not:

- register workflow runtime behavior outside loader-visible files;
- silently create approval records;
- copy raw workflow YAML into the catalog;
- mutate runtime state;
- execute commands or providers.

The first promotion-catalog integration should be opt-in and should fail closed
before active workflow mutation when required catalog inputs are invalid. If
catalog record writing itself fails after the active workflow file has already
been written and validated, the command must surface a stable, non-leaking
partial-integration error and state that the active workflow exists while the
catalog record was not persisted. Automatic rollback remains deferred until a
separate recovery policy exists.

## 9. Stewardship Integration Policy

Future persisted stewardship should:

- record the reviewer/delegated maintainer actor;
- record decision kind and timestamp;
- cite preflight and steward-review references where available;
- cite evidence, approval, policy, validation, and work-report references by
  stable ids only;
- store bounded reason summaries, known limitations, and strict non-goals;
- reject secret-like or unbounded text;
- avoid treating a preview-only review card as durable approval unless the
  persisted decision is explicitly written.

The first command integration should be opt-in. It should not make existing
preview-only steward-review behavior suddenly mutate disk unless the user asks
for persistence.

Recommended first CLI shape:

```text
workflow-os author workflow steward-review \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason> \
  --persist-stewardship \
  [--catalog-root .workflow-os/catalog]
```

Default behavior should remain preview-only. With `--persist-stewardship`, the
command may create the catalog root if it does not exist because the user has
explicitly requested a persistence operation. The output must disclose the
written stewardship decision id, catalog root posture, dry-run posture, and
non-runtime boundary.

## 10. Archive Integration Policy

Future archive integration should:

- inspect current draft status before moving;
- reject active candidates;
- reject archive destination overwrite;
- move exactly one eligible promoted or superseded draft;
- write an archive record only after the move succeeds;
- cite original draft path, archive path, workflow id, content hash, archive
  actor, reason summary, validation reference, and stewardship decision where
  available;
- keep archive metadata reference-only and bounded.

Archive metadata must not claim approval unless it cites an actual persisted
stewardship decision.

Archive metadata integration should follow promotion catalog integration. The
first archive metadata write should remain coupled to the existing explicit
`archive-draft` command, not automatic cleanup.

## 11. Conflict Detection Policy

The first persisted catalog integration should disclose conflicts before it
blocks promotion broadly.

Conflict categories:

- duplicate workflow id;
- duplicate active workflow path;
- stale draft content hash;
- stale active workflow content hash;
- source recommendation already active;
- overlapping authority scope;
- overlapping side-effect posture;
- conflicting owner/escalation assignment;
- conflicting approval or evidence posture;
- archived draft re-promotion attempt;
- catalog record references missing workflow file;
- workflow file has no catalog record.

Recommended v1 behavior:

- hard-block exact identity/path/hash conflicts;
- warn/disclose semantic overlap conflicts until taxonomy matures;
- keep conflict output bounded and non-leaking;
- do not read raw workflow bodies into conflict messages.

## 12. Store API Shape

Recommended first helper types:

- `WorkflowCatalogStore`
- `WorkflowCatalogStoreError`
- `LocalWorkflowCatalogStore`
- `WorkflowCatalogStoreHealth`
- `WorkflowCatalogIndex`
- `WorkflowCatalogConflict`
- `WorkflowCatalogConflictKind`
- `WorkflowCatalogWriteMode`

Recommended operations:

- write catalog record if absent;
- update catalog record by id with expected prior hash/version;
- read catalog record by id;
- list catalog records;
- write stewardship record if absent;
- read stewardship record by id;
- list stewardship records for workflow id;
- write archive record if absent;
- read archive record by id;
- health check;
- derive conflicts from active files, drafts, and catalog records.

All operations should be explicit and return structured non-leaking errors.

## 13. Privacy And Redaction

The store must not persist or print:

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
- environment variables;
- credentials, authorization headers, private keys, token-like values;
- existing agent instruction bodies.

The store should use existing model constructors before writing records. Invalid
serialized records should fail closed during reads. Corrupt files should be
reported by stable code without echoing raw file content.

## 14. Failure And Atomicity Policy

Future writes should be atomic at the file level:

- validate before write;
- write to a temporary file under the catalog directory;
- fsync or equivalent where reasonable for the local backend;
- rename into place;
- never leave partial JSON as a valid record;
- reject duplicate writes unless an explicit update mode is used;
- fail closed if record identity does not match path-derived identity;
- avoid deleting or overwriting records during the first implementation.

If a command succeeds in moving or promoting a workflow but catalog write fails,
that failure must be surfaced clearly. The first integration phase should choose
whether catalog write is precondition, best-effort warning, or rollback blocker
before implementation.

Decision for the first command-write lane:

- persisted stewardship: catalog write is the primary operation and must fail
  closed without changing workflow files;
- promotion catalog write: invalid catalog inputs fail before promotion;
  post-validation catalog write failures surface as explicit partial-integration
  errors without automatic rollback;
- archive metadata write: archive record write should happen after a successful
  move and must surface explicit partial-integration status if it fails;
- none of these failures should create runtime state or start workflow runs.

## 15. Relationship To Git

Git is useful for review, but it should not be the long-term database.

Recommended first posture:

- catalog files are ordinary local files and may be committed by user choice;
- Workflow OS does not automatically run `git add`, commit, or push;
- catalog identity uses content hashes to remain useful across Git history;
- future team backends may replace local file storage;
- Git history can supplement auditability but must not be required for local
  correctness.

## 16. Test Plan

Future implementation tests should cover:

- persisted steward-review writes stewardship record only when explicitly
  requested;
- preview-only steward-review remains non-mutating;
- persisted stewardship writes no workflow files or runtime state;
- valid local catalog store writes and reads each record kind;
- duplicate record write rejection;
- invalid serialized record fails closed;
- corrupt JSON read fails without leaking file contents;
- path traversal and absolute path rejection;
- atomic write leaves no accepted partial record;
- listing order is deterministic;
- identity mismatch between file and record fails closed;
- stale hash update fails closed;
- promotion integration writes catalog only after validation succeeds;
- failed promotion writes no catalog record;
- archive integration writes archive record only after successful move;
- dry-run writes no catalog files;
- conflicts are detected for duplicate workflow ids and active paths;
- semantic overlap conflicts are disclosed but not hard-blocking where planned;
- no raw workflow YAML, command output, provider payloads, paths, or secrets are
  copied into errors/debug/serialization;
- existing authoring command tests still pass;
- docs check passes.

## 17. Proposed Implementation Sequence

1. Implement local catalog store helper and store tests only. Completed in the
   local store-helper implementation phase.
2. Review store helper. Completed in
   [Workflow Catalog Store Helper Review](../concepts/WORKFLOW_CATALOG_STORE_HELPER_REVIEW.md).
3. Plan in-memory catalog indexing/conflict helper from active workflows,
   drafts, and catalog records. Documented in
   [Workflow Catalog Indexing And Conflict Helper Plan](workflow-catalog-indexing-conflict-plan.md).
4. Add in-memory catalog indexing/conflict helper from active workflows, drafts,
   and catalog records. Completed in the
   [Workflow Catalog Indexing Conflict Helper Report](../concepts/WORKFLOW_CATALOG_INDEXING_CONFLICT_HELPER_REPORT.md).
5. Review conflict helper. Completed after the serde validation blocker fix.
6. Plan and implement the read-only catalog status command. Completed and
   accepted in
   [Workflow Catalog Status Command Review](../concepts/WORKFLOW_CATALOG_STATUS_COMMAND_REVIEW.md).
7. Refresh catalog persistence integration planning. This document now captures
   that boundary.
8. Add opt-in steward-review persistence.
9. Review persisted stewardship.
10. Add opt-in promotion integration with catalog-record write.
11. Review promotion catalog write semantics.
12. Add archive integration with archive metadata write.
13. Review before any schema, examples, hosted, runtime registration, provider,
    or catalog repair behavior.

## 18. Open Questions

- Should local catalog records be committed by default, ignored by default, or
  user-configured?
- Should catalog write failure block promotion or report a warning after active
  file write?
- Should promotion require persisted stewardship immediately or start with
  optional record creation?
- How should catalog index rebuild handle manually edited active workflow files?
- Should semantic conflicts ever block locally, or only in enterprise steward
  profiles?
- What is the smallest useful catalog health command?
- How should future hosted/team stores preserve local-first behavior?
- Should persisted stewardship create `.workflow-os/catalog/` by default only
  when `--persist-stewardship` is present, or require an explicit
  `--catalog-root` on the first write?
- Should promotion require a persisted stewardship decision in strict mode, or
  allow same-process reviewer input plus optional catalog citation for the first
  integration slice?
- What exact partial-integration exit code should promotion/archive use if the
  workflow file operation succeeds but the catalog sidecar write fails?

## 19. Final Recommendation

Next recommended phase: opt-in steward-review persistence.

This should be the first command write integration because it records a bounded
review decision without moving workflow files. It gives promotion and archive
commands a durable stewardship reference to cite later, while keeping existing
preview-only steward-review behavior unchanged by default.

The next implementation must still not add runtime workflow registration,
promotion catalog writes, archive metadata writes, schemas, examples, providers,
hosted collaboration, catalog repair, automatic workflow generation, draft
deletion, write-capable adapters, or release posture changes.
