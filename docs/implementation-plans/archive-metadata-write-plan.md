# Archive Metadata Write Plan

## 1. Executive Summary

Workflow OS already supports explicit local draft archiving through
`workflow-os author workflow archive-draft`, and the workflow catalog store can
persist validated `WorkflowArchiveRecord` values. The next question is how the
archive command should optionally write archive metadata without turning local
authoring hygiene into runtime workflow registration or catalog repair.

This plan defines the smallest future implementation slice: an opt-in archive
metadata write path for `archive-draft` after one eligible draft has been moved
to `workflows/drafts/archive/`.

This plan does not implement archive metadata writes. It does not add runtime
registration, catalog repair, draft deletion, automatic cleanup, schemas,
examples, provider calls, hosted behavior, external writes, or release posture
changes.

## 2. Goals

- Add a narrow future opt-in archive metadata write boundary.
- Preserve active workflow files as the execution source of truth.
- Preserve archived draft files as the archive source of truth.
- Write one validated `WorkflowArchiveRecord` only after a successful archive
  move and post-archive validation.
- Keep default `archive-draft` behavior unchanged unless an explicit catalog
  persistence flag is supplied.
- Let archive metadata cite a persisted stewardship decision when supplied.
- Fail closed before moving the draft when required catalog prerequisites are
  invalid.
- Surface partial-integration status clearly if the draft archive succeeds but
  a later catalog sidecar write fails.
- Keep archive records reference-oriented, bounded, and redaction-safe.
- Prepare catalog health and repair planning without implementing repair.

## 3. Non-Goals

Do not implement in the archive metadata write phase:

- runtime workflow registration;
- automatic workflow generation;
- automatic promotion;
- automatic archiving;
- automatic cleanup after promotion;
- draft deletion;
- catalog repair;
- catalog status enforcement;
- runtime state creation;
- workflow run creation;
- command execution or local check execution;
- provider calls;
- report artifacts;
- workflow schema changes;
- examples;
- hosted or distributed catalog behavior;
- RBAC, IdP integration, notifications, or admin UI;
- write-capable adapters or provider mutation;
- release posture changes.

## 4. Current Boundary

Current archive behavior is an explicit local authoring mutation:

```sh
workflow-os author workflow archive-draft \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-reason>
```

It validates the project, classifies draft status, refuses active candidates,
refuses archive destination overwrite, moves exactly one eligible
`promoted_preserved` or `superseded_by_active` draft into
`workflows/drafts/archive/`, validates the project after the move, and prints
bounded output.

It does not persist archive metadata, write workflow catalog records, update
existing catalog records, register workflows, create runtime state, execute
commands, call providers, write report artifacts, or delete drafts.

## 5. Proposed CLI Shape

Add explicit opt-in catalog persistence flags to archive:

```sh
workflow-os author workflow archive-draft \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-reason> \
  --persist-archive-record \
  [--catalog-root .workflow-os/catalog] \
  [--stewardship-decision-id stewardship/<id>]
```

Dry-run should support the same flags without writing or moving files:

```sh
workflow-os author workflow archive-draft \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-reason> \
  --persist-archive-record \
  --stewardship-decision-id stewardship/<id> \
  --dry-run
```

Flag rules:

- `--catalog-root` is accepted only when `--persist-archive-record` is present.
- `--stewardship-decision-id` is accepted only when
  `--persist-archive-record` is present.
- `--persist-archive-record` may create the catalog root because the user
  explicitly requested persistence.
- Default archive without `--persist-archive-record` remains unchanged.

## 6. Archive Record Construction Policy

The future implementation should construct a `WorkflowArchiveRecord` through
the existing model constructor. The record should include only bounded
references:

- deterministic archive record id;
- original draft path;
- archive path;
- workflow id;
- draft content hash;
- matching active workflow path if available;
- matching active workflow content hash if available;
- prior draft status code;
- archive actor;
- bounded archive reason summary or an explicit bounded reason-present marker;
- archive timestamp;
- validation reference if available;
- stewardship decision id when supplied and verified;
- conservative sensitivity;
- explicit redaction metadata.

The record must not store raw workflow YAML, raw source contents, command
output, provider payloads, parser payloads, package scripts, environment values,
credentials, authorization headers, private keys, or token-like values.

## 7. Stewardship Citation Policy

The first archive metadata write implementation should support two modes:

- optional citation mode: when `--stewardship-decision-id` is supplied, read and
  validate the persisted stewardship decision before moving the draft;
- no-citation mode: when no stewardship decision id is supplied, archive record
  writing may still proceed but must disclose that no persisted stewardship
  decision was cited.

Strict requirement mode should remain deferred until a later policy/profile
phase.

If a stewardship decision id is supplied, archive must verify:

- the record exists in the selected catalog root;
- the record is valid and deserializes through the model constructor;
- `workflow_id` matches the draft candidate workflow id;
- `draft_path` matches the draft being archived;
- `candidate_content_hash` matches the current draft content hash;
- the decision kind is compatible with archive citation.

The acceptable decision kinds for first implementation should be conservative:
`ApprovedForPromotion`, `Superseded`, or `Archived` if those values are already
represented by the model. If the model does not currently distinguish archive
authorization cleanly, the first implementation should cite only
`ApprovedForPromotion` and document archive-specific decision semantics as
deferred.

Any mismatch fails closed before draft movement.

## 8. Write Timing And Atomicity

Recommended first implementation sequence:

1. Load and validate the project.
2. Validate draft path and derive current draft content hash.
3. Derive draft status using existing `draft-status` semantics.
4. Reject ineligible draft status before any mutation.
5. Derive and validate archive destination.
6. If catalog persistence is requested, validate catalog root and optional
   stewardship decision id before moving the draft.
7. If a stewardship decision id is supplied, verify workflow id, draft path, and
   content hash before moving the draft.
8. In dry-run mode, emit bounded preview output without moving files or writing
   catalog records.
9. Move exactly one eligible draft file to `workflows/drafts/archive/`.
10. Reload and validate the project after the archive move.
11. Construct the archive record from the validated archive result.
12. Write the archive record through `LocalWorkflowCatalogStore` with
    write-if-absent behavior.
13. Print bounded output that distinguishes draft archive movement from archive
    metadata persistence.

Invalid catalog inputs must fail before moving the draft. If the draft move
succeeds but archive record persistence fails after post-archive validation, the
command should return a stable partial-integration error that clearly says:

- the draft file was archived;
- the archive metadata record was not written;
- no runtime state was created;
- no automatic rollback was attempted;
- the maintainer should run `workflow-os author workflow catalog-status` or a
  future recovery command once planned.

Automatic rollback remains deferred.

## 9. Catalog Record Relationship

The first archive metadata write should not update an existing
`WorkflowCatalogRecord` unless update semantics are separately planned.

Rationale:

- existing catalog record writes use write-if-absent semantics;
- archive record persistence can stand alone as a sidecar fact;
- catalog index helpers already accept active, archived, catalog, stewardship,
  and archive inputs and can detect missing or mismatched archive records;
- updating workflow catalog records raises merge/update/reconciliation
  semantics that should be planned after archive metadata writes are reviewed.

Future catalog record update planning may decide how to set
`archived_draft_path` and `latest_archive_record_id`, but the first archive
metadata write phase should avoid mutating existing workflow catalog records.

## 10. Failure Semantics

Archive with metadata persistence must fail closed before moving the draft when:

- `--catalog-root` is unsafe or supplied without `--persist-archive-record`;
- `--stewardship-decision-id` is invalid or supplied without
  `--persist-archive-record`;
- the supplied stewardship decision record is missing;
- the supplied stewardship decision record is corrupt or invalid;
- the supplied stewardship decision does not match workflow id, draft path, or
  content hash;
- the supplied stewardship decision is not acceptable for citation;
- constructing the archive record fails;
- an archive record already exists for the target record id and update
  semantics are not implemented.

Errors must use stable codes and must not echo raw paths beyond bounded
repository-relative paths, raw YAML, review reason text, serialized record
payloads, parser payloads, command output, provider payloads, or secret-like
values.

Candidate error codes:

- `cli.workflow_authoring.archive_catalog_root_requires_persistence`;
- `cli.workflow_authoring.archive_stewardship_requires_persistence`;
- `cli.workflow_authoring.archive_stewardship_missing`;
- `cli.workflow_authoring.archive_stewardship_mismatch`;
- `cli.workflow_authoring.archive_record_construct_failed`;
- `cli.workflow_authoring.archive_record_persist_failed`.

## 11. Output Policy

Text and JSON output should disclose:

- mode: `author_workflow_draft_archive`;
- archive record persistence requested: true or false;
- draft archived: true or dry-run false;
- archive record written: true or false;
- archive record id when written;
- catalog root as a bounded repository-relative path;
- stewardship decision id when cited;
- stewardship decision required: false for the first implementation;
- stewardship decision verified: true, false, or not_available;
- workflow catalog record updated: false;
- runtime registration: false;
- runtime state created: false;
- commands executed: false;
- providers called: false;
- report artifacts written: false;
- next action: run `workflow-os author workflow catalog-status` or
  `workflow-os validate`.

Output must not copy raw reason text, raw workflow YAML, raw catalog JSON,
source contents, command output, provider payloads, environment values,
credentials, or token-like values.

## 12. Relationship To Catalog Status

After archive metadata write implementation, `catalog-status` should be able to
read the new archive record and reduce the existing
`archive_record_missing_archived_draft`, `archive_path_mismatch`, and
`archive_hash_mismatch` conflict surface when the archive record matches the
archived draft.

The archive command should not depend on catalog-status output strings.
Instead, both commands should share validated catalog-store/index helper
behavior where practical.

Strict catalog coverage remains opt-in to `catalog-status` until a separate
enforcement phase.

## 13. Privacy And Redaction

The future implementation must not persist or print:

- raw draft YAML;
- active workflow YAML;
- source file contents;
- manifest bodies;
- package script bodies;
- dependency values or lockfile contents;
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
- reviewer reason text;
- existing agent instruction bodies.

Allowed output remains bounded to relative paths, workflow ids, content hashes,
status codes, action codes, validation references, decision ids, and explicit
boundary flags.

Sensitivity should default conservatively, and redaction metadata must be
validated through the existing catalog model boundary.

## 14. Test Plan

Future implementation tests should cover:

- default `archive-draft` still does not write catalog records;
- `--catalog-root` without `--persist-archive-record` fails closed;
- `--stewardship-decision-id` without `--persist-archive-record` fails closed;
- dry-run with persistence flags validates and writes nothing;
- promoted-preserved draft archives and writes one archive record when
  requested;
- superseded-by-active draft archives and writes one archive record when
  requested;
- active candidate remains rejected without archive record write;
- missing stewardship record fails before draft movement;
- stale stewardship workflow id, draft path, or content hash fails before draft
  movement;
- archive record duplicate write returns a stable non-leaking error;
- late archive record write failure reports partial integration without
  rollback;
- archive record contains correct kind, paths, workflow id, content hash, prior
  status, actor, timestamp, optional active workflow reference, sensitivity, and
  redaction metadata;
- raw draft YAML and raw reason text are not printed or persisted;
- secret-like reason or redaction metadata fails without leakage;
- `catalog-status` sees the archive record after write;
- existing archive, promotion, steward-review, catalog store, catalog index,
  validation, and runtime tests still pass;
- `cargo test --workspace` and `npm run check:docs` pass.

## 15. Documentation Updates For Implementation

The future implementation should update:

- `docs/implementation-plans/archive-metadata-write-plan.md`;
- `docs/implementation-plans/workflow-catalog-persistence-plan.md`;
- `docs/implementation-plans/governed-workflow-authoring-draft-archive-command-plan.md`;
- `ROADMAP.md`;
- related concept reports.

Docs must say:

- opt-in archive metadata writes are implemented only when the implementation is
  complete;
- default archive behavior remains unchanged;
- automatic cleanup is not implemented;
- draft deletion is not implemented;
- runtime registration is not implemented;
- workflow catalog record update/repair is not implemented;
- schemas and examples are not changed;
- provider calls, hosted behavior, external writes, and release posture changes
  remain deferred.

## 16. Proposed Implementation Sequence

Recommended next small implementation sequence:

1. Add CLI flag parsing for `--persist-archive-record`, `--catalog-root`, and
   `--stewardship-decision-id` on `archive-draft`.
2. Reuse existing safe catalog-root resolution and stewardship decision
   verification patterns from promotion catalog writes.
3. Construct a `WorkflowArchiveRecord` from archive command context after the
   archive move and post-archive validation.
4. Persist it through `LocalWorkflowCatalogStore::write_archive_record_if_absent`.
5. Add focused CLI tests for success, dry-run, invalid flags, stale
   stewardship, duplicate record, non-leakage, and unchanged default behavior.
6. Update docs and create an implementation report.
7. Run maintainer review before catalog update/repair work.

## 17. Final Recommendation

The next implementation phase should be opt-in archive metadata write
implementation for `workflow-os author workflow archive-draft`, local only.

It must still not build runtime registration, automatic cleanup, draft deletion,
catalog repair, workflow catalog record update semantics, schemas, examples,
provider calls, hosted behavior, external writes, or release posture changes.
