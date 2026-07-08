# Workflow Catalog Repair Dry-Run CLI Report

## 1. Executive Summary

The read-only workflow catalog repair dry-run CLI surface is implemented.
`workflow-os author workflow catalog-repair --dry-run` consumes the same
bounded inputs as `catalog-status`, derives review-required repair proposals
through the existing `WorkflowCatalogRepairProposal` helper, and prints human
or preview JSON output without mutating workflow files, catalog records,
runtime state, or provider systems.

This is a proposal and review surface only. It does not implement apply mode,
automatic repair, cleanup, deletion, overwrite, workflow runtime registration,
schemas, examples, hosted behavior, provider calls, or release posture changes.

Governed dogfood run:

- workflow_id: `dg/implement`
- run_id: `run-1783543084009075000-2`
- approval_id: `approval/run-1783543084009075000-2/implementation-approved`
- approval outcome: granted by delegated maintainer

## 2. Scope Completed

- Added `workflow-os author workflow catalog-repair --dry-run`.
- Reused the existing catalog-status input boundary:
  - loader-visible active workflows;
  - inactive drafts;
  - archived drafts;
  - optional local catalog store records;
  - optional `--catalog-root`;
  - optional `--strict-catalog-coverage`.
- Required `--dry-run` in this release.
- Routed proposal construction through `propose_workflow_catalog_repairs`.
- Added bounded human output.
- Added preview JSON output under the existing global `--json` posture.
- Reported non-mutation boundaries explicitly:
  - no files written;
  - no catalog records written;
  - no catalog records deleted;
  - no catalog records overwritten;
  - no workflow registration;
  - no workflow promotion;
  - no draft archive;
  - no commands executed;
  - no providers called;
  - no runtime state created.
- Added focused CLI tests.

## 3. Scope Explicitly Not Completed

- No apply mode.
- No automatic repair.
- No cleanup.
- No deletion.
- No overwrite.
- No workflow runtime registration.
- No workflow schema changes.
- No examples.
- No provider calls.
- No hosted or team catalog backend behavior.
- No write-capable adapters.
- No release posture changes.

## 4. CLI/API Summary

The new command is:

```text
workflow-os author workflow catalog-repair --dry-run
```

Optional flags:

```text
--catalog-root .workflow-os/catalog
--strict-catalog-coverage
--json
```

The command returns a bounded proposal view. It does not create the catalog
root, write sidecar records, update workflow files, or create runtime state.
Missing `--dry-run` fails with usage before proposal construction.

## 5. Proposal Output Summary

Human output includes:

- mode: `workflow_catalog_repair_dry_run`;
- status: `catalog_repair_proposals_ready` or
  `catalog_repair_no_proposals`;
- catalog store posture;
- strict catalog coverage posture;
- conflict and proposal counts;
- proposal id;
- action kind;
- conflict kind;
- workflow id when available;
- source category;
- bounded source reference;
- safe-for-future-apply flag;
- human-review-required flag;
- bounded summary;
- explicit non-mutation boundary fields;
- bounded next action.

Preview JSON mirrors the same fields and remains marked by the CLI's existing
preview JSON posture.

## 6. Validation Boundary Summary

The command reuses catalog-status loading and validation. It rejects unsafe
catalog roots before reading catalog records, preserves non-leaking error
messages, and returns proposal-construction failures as a stable internal
catalog repair error.

Strict catalog coverage can still be passed to the shared index builder. The
repair command reports proposals for missing catalog coverage instead of
turning proposal review itself into a status failure.

## 7. Privacy/Redaction Summary

The command emits bounded identifiers and references only. It does not read,
copy, or emit:

- raw workflow YAML bodies;
- source file contents;
- command output;
- provider payloads;
- parser payloads;
- package script bodies;
- CI logs;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

Unsafe path failures do not echo secret-like path fragments.

## 8. Test Coverage Summary

Focused CLI tests cover:

- dry-run proposal output without mutation;
- preview JSON proposal output;
- required `--dry-run` usage failure;
- strict catalog coverage proposal output without status failure;
- unsafe catalog root rejection without leakage;
- no catalog root creation;
- no runtime state creation;
- no raw secret-like marker leakage;
- existing catalog-status behavior remains covered.

## 9. Commands Run And Results

- `cargo fmt --all`: passed.
- `cargo test -p workflow-cli --test cli author_workflow_catalog_repair`:
  passed, 5 focused tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.

## 10. Remaining Known Limitations

- Proposals are not durable records.
- No proposal review or approval workflow exists yet.
- No apply mode exists.
- No automatic repair exists.
- Only the first repair action kind is implemented:
  `CreateMissingCatalogRecord`.
- Catalog repair does not create missing sidecar files.
- Catalog repair does not clean stale records.
- Catalog repair does not modify workflow files.

## 11. Recommended Next Phase

Recommended next phase: maintainer review of the read-only workflow catalog
repair dry-run CLI surface.

The review should verify that the command stays non-mutating, keeps output
bounded, preserves existing catalog-status behavior, and does not accidentally
authorize apply mode or cleanup semantics.
