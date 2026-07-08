# Workflow Catalog Status Command Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation delivers the planned first command-integration slice:
`workflow-os author workflow catalog-status`. The command is read-only,
bounded, and reviewable. It consumes loader-visible active workflows, inactive
drafts, archived drafts, and optional local catalog-store records through the
reviewed workflow catalog index helper.

No blocker was found.

## 2. Scope Verification

The phase stayed within the approved non-mutating status-command scope.

No accidental implementation was found for:

- catalog record writes;
- stewardship record writes from steward review;
- archive metadata writes from archive commands;
- promotion enforcement from catalog status;
- automatic catalog repair;
- top-level `workflow-os catalog ...` command family;
- workflow runtime registration;
- workflow schema changes;
- examples;
- hosted or team catalog backend;
- provider calls;
- command execution or local check execution;
- side effects or writes;
- release posture changes.

## 3. Command Surface Assessment

The command surface is appropriately narrow:

```text
workflow-os author workflow catalog-status [--catalog-root <path>] [--strict-catalog-coverage] [--json]
```

The command fits the existing `author workflow` family because it reports
authoring/catalog status for workflows rather than introducing runtime behavior
or a broader catalog-management API.

The command supports:

- default human output;
- JSON output through the existing global `--json` flag;
- safe explicit catalog roots;
- strict catalog coverage as a status-only blocker posture.

The review found no evidence that `catalog-status` promotes, archives,
registers, or mutates workflow files.

## 4. Inventory And Conflict Assessment

The command derives:

- active workflow summaries from the loaded project bundle;
- inactive draft summaries from `workflows/drafts/*.workflow.yml`;
- archived draft summaries from `workflows/drafts/archive/*.workflow.yml`;
- catalog records from `LocalWorkflowCatalogStore` when the catalog root exists;
- stewardship records for known workflow ids;
- archive records from `LocalWorkflowCatalogStore` when the catalog root exists.

It then calls `build_workflow_catalog_index` and reports the index counts and
conflict summaries.

This preserves the reviewed helper as the conflict engine and avoids inventing
a parallel CLI-only conflict taxonomy.

## 5. Store Boundary Assessment

The default `.workflow-os/catalog` root is read only when it already exists.
When it does not exist, the command reports:

```text
catalog_store: not_available
```

It does not create the catalog root.

Explicit `--catalog-root` values are constrained to safe repository-relative
paths. Absolute paths, traversal-shaped paths, non-UTF-8 segments, unsafe
segments, and secret-like values fail closed with bounded errors.

The command reads store records through `LocalWorkflowCatalogStore` list APIs.
Store read failures are mapped to stable non-leaking CLI errors.

## 6. Conflict Behavior Assessment

Default behavior is conservative and non-blocking for missing active catalog
records. Missing catalog records are warnings unless strict coverage is
explicitly requested.

With `--strict-catalog-coverage`, missing active catalog records become blocker
conflicts for the status command only. This is appropriate because it lets a
maintainer test a stricter posture without changing promotion, archive, runtime,
or schema behavior.

Direct command checks against the dogfood project confirmed:

- default status exits successfully with warning conflicts;
- JSON output reports the same bounded counts and conflict data;
- strict coverage exits non-zero with blocker conflicts;
- boundary lines report no file writes, workflow registration, promotion,
  archive mutation, provider calls, command execution, or runtime state.

## 7. Output And Privacy Assessment

The human and JSON outputs are bounded.

They contain:

- counts;
- workflow ids;
- repository-relative workflow paths;
- conflict severity and kind codes;
- conflict source category and bounded source references;
- explicit non-mutation boundary fields.

They do not copy raw workflow YAML, draft YAML, catalog JSON payloads, source
contents, manifest bodies, package script bodies, dependency values, CI logs,
provider payloads, parser payloads, command output, environment values,
credentials, authorization headers, private keys, token-like values, or
secret-like catalog-root values.

The command does print repository-relative workflow paths as conflict source
references. That is consistent with the catalog index model and existing CLI
authoring surfaces. It should remain documented as bounded project metadata, not
raw source content.

## 8. Error-Handling Assessment

The command fails closed when:

- project loading or validation fails;
- catalog root validation fails;
- existing catalog records cannot be read safely;
- draft or archived draft files cannot be read or parsed;
- index construction fails;
- blocker conflicts exist.

Errors use stable CLI codes such as:

- `cli.workflow_catalog.catalog_root_rejected`;
- `cli.workflow_catalog.catalog_read_failed`;
- `cli.workflow_catalog.draft_read_failed`;
- `cli.workflow_catalog.draft_parse_failed`;
- `cli.workflow_catalog.status_build_failed`;
- `cli.workflow_catalog.status_blocked`.

The review did not find error messages that echo rejected catalog-root values,
raw file contents, raw record payloads, or secret-like values.

## 9. Test Quality Assessment

The focused CLI tests cover:

- no-store inventory status without mutation;
- JSON output shape and bounded conflict summaries;
- existing empty catalog root read without mutation;
- strict catalog coverage blocker behavior;
- unsafe catalog root rejection without leakage;
- absence of runtime state creation.

Existing core tests continue to cover:

- workflow catalog model validation;
- local workflow catalog store writes, reads, listing, corrupt record handling,
  identity mismatch handling, and debug redaction;
- catalog index ordering, blocker/warning taxonomy, strict missing-record
  posture, stale stewardship/archive conflicts, serde validation, and debug
  redaction.

Non-blocking test gaps:

- add a CLI-level corrupt catalog-store fixture test to prove the status command
  maps store corruption into `cli.workflow_catalog.catalog_read_failed`;
- add a CLI-level populated catalog-store fixture test once a concise helper
  exists for writing valid catalog records in CLI tests;
- add direct JSON parsing in CLI tests instead of substring checks if the JSON
  surface becomes more important.

These gaps do not block this phase because the lower-level store and index
contracts already cover the safety-critical validation paths.

## 10. Documentation Review

The documentation accurately states that:

- `workflow-os author workflow catalog-status` is implemented;
- the command is read-only and non-mutating;
- missing catalog roots are reported as not available rather than created;
- strict coverage affects the status command only;
- catalog writes remain unimplemented;
- promotion enforcement from catalog status remains unimplemented;
- archive metadata writes remain unimplemented;
- workflow runtime registration remains unimplemented;
- schemas, examples, hosted behavior, provider calls, side effects, writes, and
  release posture changes remain unimplemented.

The roadmap, CLI docs, command-integration plan, and implementation report are
consistent with the implemented boundary.

## 11. Validation Assessment

Implementation phase validation passed before merge:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p workflow-cli --test cli author_workflow_catalog_status
cargo test --workspace
npm run check:docs
```

Review-phase command checks also exercised:

```text
workflow-os --project-dir dogfood/workflow-os-self-governance author workflow catalog-status
workflow-os --project-dir dogfood/workflow-os-self-governance --json author workflow catalog-status
workflow-os --project-dir dogfood/workflow-os-self-governance author workflow catalog-status --strict-catalog-coverage
```

The default and JSON commands succeeded with warning conflicts. The strict
coverage command failed closed with blocker conflicts and no mutation boundary
changes.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Add CLI-level corrupt catalog-store coverage.
- Add CLI-level populated catalog-store coverage once the test harness has a
  concise way to create valid catalog records.
- Consider making catalog status output optionally group conflicts by severity
  if human output becomes dense for large repositories.
- Keep strict catalog coverage status-only until promotion and archive catalog
  persistence are separately implemented and reviewed.

## 14. Recommended Next Phase

Recommended next phase: workflow catalog persistence integration planning.

The status command now exposes the read-only view needed to understand catalog
coverage and conflicts. The next phase should plan how promotion, archive, and
steward-review paths will write catalog, archive, and stewardship records
without weakening the active workflow file boundary or adding implicit runtime
registration.

## 15. Dogfood Governance

```text
workflow_id: dg/review
run_id: run-1783527432328516000-2
approval_id: approval/run-1783527432328516000-2/review-scope-approved
approval_outcome: granted
phase_close_status: Completed
events_total: 39
```

The review phase was approved before review work proceeded. The kernel governed
the phase boundary; command inspection, validation commands, git operations,
and PR operations remain outside the kernel and must be disclosed in phase
handoff.
