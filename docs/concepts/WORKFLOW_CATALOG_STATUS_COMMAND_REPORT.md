# Workflow Catalog Status Command Report

## 1. Executive Summary

This phase implements the first workflow catalog command integration slice:
`workflow-os author workflow catalog-status`.

The command is local, read-only, and non-mutating. It derives loader-visible
active workflows, inactive drafts, archived drafts, and optional local catalog
store records, then calls the reviewed `build_workflow_catalog_index` helper to
produce bounded inventory and conflict output.

It does not write catalog records, promote drafts, archive drafts, register
workflows, create runtime state, execute commands, call providers, add schemas,
add examples, enable writes, or change release posture.

## 2. Scope Completed

- Added `workflow-os author workflow catalog-status`.
- Added `workflow-os author workflow catalog-status --json`.
- Added `--strict-catalog-coverage` for status-only blocker posture.
- Added `--catalog-root <path>` for explicit safe repository-relative catalog
  roots.
- Derived active workflow summaries from the loaded project bundle.
- Derived inactive draft summaries from `workflows/drafts/*.workflow.yml`.
- Derived archived draft summaries from
  `workflows/drafts/archive/*.workflow.yml`.
- Read local catalog, stewardship, and archive records through
  `LocalWorkflowCatalogStore` when a catalog root exists.
- Disclosed absent catalog roots as `catalog_store: not_available` without
  creating files.
- Reported bounded conflict counts and conflict codes.
- Added focused CLI tests.
- Updated roadmap, CLI docs, and the command integration plan.

## 3. Scope Explicitly Not Completed

- No catalog record writes.
- No stewardship record writes from steward review.
- No archive metadata writes from archive commands.
- No promotion enforcement from catalog status.
- No automatic catalog repair.
- No top-level `workflow-os catalog ...` command family.
- No workflow runtime registration.
- No workflow schema changes.
- No examples.
- No hosted/team catalog backend.
- No provider calls.
- No side effects or writes.
- No release posture changes.

## 4. Command Summary

```text
workflow-os author workflow catalog-status [--catalog-root <path>] [--strict-catalog-coverage] [--json]
```

Default behavior:

- loads and validates the local Workflow OS project;
- reads `.workflow-os/catalog` only if that root already exists;
- proceeds with empty catalog records when no catalog root exists;
- prints active workflow, draft, archived draft, catalog, stewardship, and
  archive counts;
- prints blocker and warning conflict counts;
- prints bounded conflict summaries;
- exits non-zero only when blocker conflicts exist.

## 5. Conflict Behavior

Warning conflicts remain non-fatal by default. Missing catalog records for active
workflows are warnings unless `--strict-catalog-coverage` is supplied.

With `--strict-catalog-coverage`, missing catalog records become blocker
conflicts for the status command. This does not change promotion, archive, or
runtime behavior.

## 6. Store Boundary

The command does not create `.workflow-os/catalog` or any other catalog root.
When a catalog root exists or is explicitly supplied, records are read through
`LocalWorkflowCatalogStore` list methods.

Unsafe, absolute, traversal-shaped, non-UTF-8, or secret-like catalog roots are
rejected with stable non-leaking validation errors.

## 7. Redaction And Privacy Summary

The command prints bounded ids, counts, relative paths, hashes, and conflict
codes. It does not print raw workflow YAML, draft YAML, catalog JSON payloads,
provider payloads, command output, environment values, credentials, tokens, or
secret-like catalog root values.

Errors are stable and non-leaking.

## 8. Test Coverage Summary

Added focused CLI tests for:

- no-store inventory status without mutation;
- existing empty catalog root read without mutation;
- JSON status output shape and bounded conflict summaries;
- strict catalog coverage blocker behavior;
- unsafe catalog root rejection without leakage.

Existing tests for workflow authoring, catalog model/index/store behavior, and
runtime behavior remain in the workspace validation surface.

## 9. Commands Run And Results

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p workflow-cli --test cli author_workflow_catalog_status
cargo test --workspace
npm run check:docs
```

Result: passed.

## 10. Remaining Known Limitations

- The status command is not a top-level catalog command family.
- The command does not write catalog records from promotion.
- The command does not write stewardship records from steward review.
- The command does not write archive records from archive commands.
- The command does not enforce catalog status in promotion or archive paths.
- The command does not implement hosted/team catalog behavior.
- The command does not implement write-capable adapters.

## 11. Dogfood Governance

```text
workflow_id: dg/implement
run_id: run-1783525500684956000-2
approval_id: approval/run-1783525500684956000-2/implementation-approved
approval_outcome: granted
phase_close_status: Completed
events_total: 39
```

The implementation phase was approved before code changes were made. The kernel
governed the phase boundary; code edits, validation commands, git operations,
and PR operations were performed outside the kernel and are disclosed in the
phase handoff.

## 12. Recommended Next Phase

Recommended next phase: workflow catalog status command review.

Review should verify the command remains non-mutating, preserves active workflow
files as source of truth, reports bounded inventory/conflicts, fails safely on
blockers, and does not imply catalog enforcement in promotion or archive paths.
