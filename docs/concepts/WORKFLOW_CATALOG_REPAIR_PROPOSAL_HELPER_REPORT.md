# Workflow Catalog Repair Proposal Helper Report

## 1. Executive Summary

The first non-mutating workflow catalog repair implementation is complete as a
core helper. It derives bounded repair/recovery proposals from the existing
`WorkflowCatalogIndex` conflict list.

The helper does not repair, clean up, delete, overwrite, move drafts, rewrite
active workflows, create runtime state, append events, register workflows,
write files, call providers, expose schemas, add examples, or change release
posture.

## 2. Scope Completed

Implemented:

- `WorkflowCatalogRepairProposalId`;
- `WorkflowCatalogRepairActionKind`;
- `WorkflowCatalogRepairProposal`;
- `propose_workflow_catalog_repairs(&WorkflowCatalogIndex)`;
- deterministic proposal ids based on index conflict order;
- conflict-to-action classification;
- future-apply safety flags;
- human-review-required flags;
- conservative sensitivity;
- explicit redaction metadata;
- redaction-safe Debug output;
- serde validation on deserialization;
- focused tests.

## 3. Scope Explicitly Not Completed

Not implemented:

- CLI `catalog-repair` command;
- apply mode;
- automatic catalog repair;
- catalog cleanup;
- record deletion;
- record overwrite/update;
- active workflow rewrites;
- draft or archive movement;
- runtime workflow registration;
- runtime state creation;
- event/audit append;
- WorkReport artifact generation;
- workflow schema changes;
- examples;
- hosted or team catalog backend behavior;
- provider calls;
- command execution or local check execution;
- write-capable adapters;
- release posture changes.

## 4. Helper API Summary

The new helper is:

```rust
propose_workflow_catalog_repairs(&WorkflowCatalogIndex)
    -> Result<Vec<WorkflowCatalogRepairProposal>, WorkflowOsError>
```

It consumes only an already-built catalog index. The helper does not read
files, inspect the catalog store, or mutate state. Callers remain responsible
for deriving the index through existing catalog-status inputs.

## 5. Proposal Semantics

The helper maps existing catalog conflict kinds to repair action kinds:

- missing active catalog record -> `CreateMissingCatalogRecord`;
- missing owner/escalation/stewardship/side-effect metadata ->
  `UpdateCatalogRecordMetadata`;
- active catalog path/hash/missing-file mismatch ->
  `ReviewCatalogRecordMismatch`;
- stale draft stewardship hash -> `ReviewStaleStewardshipDecision`;
- archive record/path/hash mismatch -> `ReviewArchiveRecordMismatch`;
- duplicate active workflow id/path -> `ReviewDuplicateActiveWorkflow`.

Only missing active catalog records are marked as safe candidates for a future
apply mode. Human review is required for all proposals in this first slice.

## 6. Validation Boundary Summary

The helper validates:

- proposal ids;
- bounded source references;
- bounded summaries;
- redaction metadata field names and reasons;
- secret-like values in proposal ids, source references, summaries, and
  redaction metadata.

Invalid serialized proposals fail closed through the manual deserializer.

## 7. Redaction And Privacy Summary

Proposal records store only bounded ids, conflict kinds, source categories,
repository-relative source references, action posture, sensitivity, and
redaction metadata.

They do not store raw workflow YAML, source contents, command output, provider
payloads, parser payloads, package script bodies, CI logs, environment values,
credentials, authorization headers, private keys, token-like values, or raw
catalog record payloads.

Debug output redacts redaction metadata internals by showing counts only.

## 8. Test Coverage Summary

Added focused coverage for:

- missing active catalog record proposal classification;
- deterministic proposal id;
- future-apply flag for missing catalog records;
- manual-review posture for duplicate active workflows;
- manual-review posture for catalog path/hash mismatches;
- stale stewardship classification;
- archive mismatch classification;
- missing metadata classification;
- serde round trip;
- invalid serialized secret-like summary fail-closed behavior;
- invalid serialized action posture fail-closed behavior;
- Debug redaction behavior;
- non-mutating behavior against index inputs.

Focused test command:

```text
cargo test -p workflow-core --test workflow_catalog_index
```

## 9. Commands Run And Results

Governed dogfood run:

```text
workflow_id: dg/implement
run_id: run-1783540698133652000-2
approval_id: approval/run-1783540698133652000-2/implementation-approved
approval_outcome: granted
```

Validation commands:

```text
cargo fmt --all
cargo test -p workflow-core --test workflow_catalog_index
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
```

Result: passed.

## 10. Remaining Known Limitations

- No CLI dry-run surface is implemented yet.
- No apply mode exists.
- The helper only classifies conflicts that the existing catalog index already
  emits.
- The existing index does not yet emit an archived-draft-missing-archive-record
  conflict.
- Corrupt store handling remains at the store/index caller boundary.
- Repair proposal review is not persisted as an audit event or WorkReport.

## 11. Recommended Next Phase

Recommended next phase: workflow catalog repair proposal helper review.

After review, the next implementation can add a read-only
`workflow-os author workflow catalog-repair --dry-run` surface if the helper is
accepted.
