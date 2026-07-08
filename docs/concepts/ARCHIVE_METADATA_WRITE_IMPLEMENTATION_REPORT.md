# Archive Metadata Write Implementation Report

## 1. Executive Summary

The archive metadata write phase implemented an explicit, local,
opt-in catalog sidecar for successful `workflow-os author workflow
archive-draft` moves.

The default archive command remains unchanged unless
`--persist-archive-record` is supplied. When persistence is requested, the CLI
constructs a validated `WorkflowArchiveRecord`, optionally verifies a persisted
stewardship decision, moves exactly one eligible draft, validates the project
after the archive move, and writes one archive metadata record through
`LocalWorkflowCatalogStore`.

This phase does not implement runtime workflow registration, catalog repair,
automatic archive cleanup, draft deletion, schemas, examples, hosted behavior,
provider calls, external writes, or release posture changes.

## 2. Scope Completed

- Added `archive-draft --persist-archive-record`.
- Added archive-specific `--catalog-root` support gated by
  `--persist-archive-record`.
- Added archive-specific `--stewardship-decision-id` support gated by
  `--persist-archive-record`.
- Validated catalog root and optional stewardship record before draft movement.
- Verified supplied stewardship decisions against workflow id, draft path,
  draft content hash, and `ApprovedForPromotion` decision kind.
- Constructed `WorkflowArchiveRecord` values through the core model
  constructor.
- Wrote archive records with `LocalWorkflowCatalogStore::write_archive_record_if_absent`.
- Preserved dry-run behavior: dry-run validates and previews but moves and
  writes nothing.
- Added bounded text and JSON output for archive metadata persistence posture.
- Added focused CLI regression tests.

## 3. Scope Explicitly Not Completed

- No runtime workflow registration.
- No catalog repair or update of existing workflow catalog records.
- No draft deletion or automatic archive cleanup.
- No runtime state creation.
- No workflow run creation.
- No command execution, local check execution, provider calls, or external
  writes.
- No report artifact writing.
- No workflow schema changes.
- No examples.
- No hosted or distributed catalog behavior.
- No RBAC, IdP integration, notifications, or admin UI.
- No write-capable adapters.
- No release posture changes.

## 4. CLI/API Summary

Implemented CLI shape:

```sh
workflow-os author workflow archive-draft \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-reason> \
  --persist-archive-record \
  [--catalog-root .workflow-os/catalog] \
  [--stewardship-decision-id stewardship/<id>]
```

Flag behavior:

- `--persist-archive-record` requests one local archive metadata record.
- `--catalog-root` is rejected unless `--persist-archive-record` is present.
- `--stewardship-decision-id` is rejected unless
  `--persist-archive-record` is present.
- Default `archive-draft` behavior without persistence flags remains unchanged.
- Dry-run reports intended persistence posture but does not move files or write
  records.

## 5. Validation Boundary Summary

The implementation validates:

- the existing project before archive evaluation;
- draft path safety;
- archive eligibility (`promoted_preserved` or `superseded_by_active`);
- archive destination non-existence;
- catalog root safety when supplied;
- stewardship record availability and identity when supplied;
- stewardship workflow id, draft path, content hash, and decision kind;
- the project again after the archive move;
- archive record construction through `WorkflowArchiveRecord::new`.

Catalog input failures occur before draft movement. If archive record
persistence fails after an archive move and post-archive validation, the command
returns `cli.workflow_authoring.archive_record_persist_failed` and does not
attempt unplanned rollback.

## 6. Redaction And Privacy Summary

Archive records store bounded references and metadata only:

- archive record id;
- original draft path;
- archive path;
- workflow id;
- draft content hash;
- active workflow path reference;
- prior draft status;
- archive actor;
- bounded reason-present marker;
- archive timestamp;
- optional stewardship decision id;
- conservative sensitivity;
- explicit empty redaction metadata.

The implementation does not store raw workflow YAML, raw source contents,
command output, provider payloads, parser payloads, package scripts,
environment values, credentials, authorization headers, private keys, or
token-like values. CLI output and tests assert that raw archive and stewardship
reason text is not echoed or serialized into the archive record.

## 7. Test Coverage Summary

Added/updated tests cover:

- default archive dry-run remains non-mutating and does not request persistence;
- default archive move remains unchanged and writes no archive records;
- opt-in archive persistence writes one archive record;
- archive record output includes bounded id/root/stewardship posture;
- archive records cite workflow id, original draft path, archive path, prior
  status, and verified stewardship id;
- `--catalog-root` without `--persist-archive-record` fails without writes;
- stale stewardship decisions fail before draft movement;
- JSON output discloses persistence posture and uses `null` when not requested;
- raw archive/stewardship reasons are not copied.

Existing archive, stewardship, promotion, catalog store, and validation tests
remain part of the full workspace validation.

## 8. Commands Run And Results

- Dogfood phase close:
  - workflow: `dg/implement`;
  - run: `run-1783536828192796000-2`;
  - approval: `approval/run-1783536828192796000-2/implementation-approved`;
  - approval outcome: granted;
  - event summary: 39 events, 1 approval, 0 retries, 0 escalations.
- `cargo fmt --all`
- `cargo test -p workflow-cli --test cli author_workflow_archive_draft -- --nocapture`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`

All required validation passed. The focused archive CLI test suite passed after
correcting the stale stewardship test setup to keep the draft archive-eligible
while changing its content hash.

## 9. Remaining Known Limitations

- Archive records do not update existing workflow catalog records.
- Archive-specific stewardship decision kinds remain deferred; the first slice
  accepts only an existing `ApprovedForPromotion` decision as a conservative
  citation.
- Active workflow content hash is not populated in the archive record.
- No catalog repair or recovery command exists for partial archive metadata
  write failures.
- No automatic archive cleanup or draft deletion exists.
- No runtime registration or workflow execution behavior is derived from
  archive metadata.

## 10. Recommended Next Phase

Recommended next phase: archive metadata write implementation review.

This phase touched local authoring mutations and catalog sidecar persistence,
so maintainer review should verify eligibility boundaries, stewardship
verification, partial failure posture, redaction behavior, test quality, and
scope containment before any catalog repair or cleanup work is planned.
