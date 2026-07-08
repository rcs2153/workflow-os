# Workflow Catalog Repair Proposal Helper Review

## 1. Executive Verdict

Phase accepted; proceed to read-only catalog repair dry-run CLI planning or
implementation.

The implementation adds the intended non-mutating core helper for deriving
bounded repair proposals from the existing workflow catalog index. It does not
apply repairs, write files, mutate catalog records, clean up store state,
register workflows, create runtime state, append events, expose schemas, add
examples, call providers, enable writes, or change release posture.

## 2. Scope Verification

The phase stayed within the approved helper-only scope.

No accidental implementation was found for:

- CLI `catalog-repair` behavior;
- apply mode;
- automatic catalog repair;
- catalog cleanup;
- record deletion;
- record overwrite/update;
- active workflow rewrites;
- draft or archive movement;
- runtime workflow registration;
- runtime state creation;
- event or audit append;
- WorkReport artifact generation;
- workflow schema changes;
- examples;
- hosted or team catalog backend behavior;
- provider calls;
- local check or command execution;
- write-capable adapters;
- release posture changes.

## 3. Helper API Assessment

The helper API is appropriately narrow:

```rust
propose_workflow_catalog_repairs(&WorkflowCatalogIndex)
    -> Result<Vec<WorkflowCatalogRepairProposal>, WorkflowOsError>
```

It consumes only an existing `WorkflowCatalogIndex`, which preserves the
reviewed catalog-status source of truth. It does not read files, inspect the
store independently, reach into runtime state, or require CLI/global context.

The proposal types are minimal enough for the first slice:

- `WorkflowCatalogRepairProposalId`;
- `WorkflowCatalogRepairActionKind`;
- `WorkflowCatalogRepairProposal`.

## 4. Proposal Semantics Assessment

The conflict-to-action mapping is acceptable:

- missing active catalog record -> `CreateMissingCatalogRecord`;
- missing owner/escalation/stewardship/side-effect metadata ->
  `UpdateCatalogRecordMetadata`;
- active catalog missing file/path/hash mismatch ->
  `ReviewCatalogRecordMismatch`;
- stale stewardship decision hash -> `ReviewStaleStewardshipDecision`;
- archive missing/path/hash mismatch -> `ReviewArchiveRecordMismatch`;
- duplicate active workflow id/path -> `ReviewDuplicateActiveWorkflow`.

Only missing active catalog records are marked safe for a future apply mode.
All proposals require human review. This matches the plan and avoids silently
elevating a proposal into an automatic repair.

## 5. Validation Assessment

Validation covers the important helper boundary:

- proposal ids are bounded and validated;
- source references are bounded and repository-relative;
- summaries are bounded and checked for secret-like values;
- redaction metadata fields and reasons are validated;
- deserialized proposals fail closed for secret-like values;
- deserialized proposals fail closed when action posture is inconsistent.

Validation errors use stable codes and do not include raw caller-supplied
secret-like values.

## 6. Privacy And Redaction Assessment

The helper stores bounded metadata only: proposal ids, conflict kinds, conflict
source categories, optional workflow ids, repository-relative source
references, action posture, summaries, sensitivity, and redaction metadata.

It does not store raw workflow YAML, source contents, command output, provider
payloads, parser payloads, package script bodies, CI logs, environment values,
credentials, authorization headers, private keys, token-like values, or raw
catalog record payloads.

Debug output redacts redaction metadata internals by count. Repository-relative
source references remain visible, which is consistent with existing
catalog-status behavior and the repair plan.

## 7. Serialization Assessment

Serde behavior is acceptable for the helper slice:

- valid proposals round-trip;
- invalid serialized secret-like summaries fail closed;
- invalid serialized action posture fails closed;
- proposal ids deserialize through the validated constructor;
- redaction metadata is validated before storage.

The serialization shape is suitable for a future read-only CLI JSON surface.

## 8. Test Quality Assessment

Focused tests cover:

- missing active catalog record proposal classification;
- deterministic proposal id;
- future-apply flag for missing catalog records;
- manual-review posture for duplicates and mismatches;
- stale stewardship proposal classification;
- archive mismatch proposal classification;
- missing metadata proposal classification;
- serde round trip;
- invalid serialized secret-like proposal failure;
- invalid serialized action posture failure;
- Debug redaction behavior;
- non-mutating behavior against the index inputs.

The tests prove behavior rather than construction only. They also run inside
the existing workflow catalog index test surface, which keeps the helper tied
to the reviewed conflict taxonomy.

## 9. Documentation Review

Documentation now states:

- the non-mutating repair proposal helper is implemented;
- the helper maps existing catalog-status conflicts into bounded proposal
  records;
- automatic repair remains deferred;
- CLI repair behavior remains deferred;
- apply mode, deletion, overwrite, cleanup, runtime registration, schemas,
  examples, hosted behavior, providers, writes, and release posture changes
  remain unimplemented.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Add a read-only `catalog-repair --dry-run` CLI surface over the helper.
- Decide whether the first CLI surface should live under
  `workflow-os author workflow catalog-repair --dry-run` or a narrower
  status-adjacent command.
- Keep any future apply mode behind separate planning, review, and explicit
  approval.
- Consider a bounded corrupt-store summary mode after the dry-run surface is
  reviewed.

## 12. Recommended Next Phase

Recommended next phase: read-only workflow catalog repair dry-run CLI surface.

Why: the core helper is accepted, and the next runtime-useful move is to expose
its proposal output to maintainers without mutation. The next phase should
remain dry-run/read-only and must not add apply mode, automatic repair,
cleanup, deletion, overwrite, workflow registration, schemas, examples,
providers, writes, or release posture changes.

## 13. Validation

Review-phase validation:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
```

Result: passed.

## 14. Governed Phase Metadata

- dogfood workflow id: `dg/review`
- run id: `run-1783542144465469000-2`
- approval id: `approval/run-1783542144465469000-2/review-scope-approved`
- approval outcome: granted by delegated maintainer
- event summary: 39 events; 1 approval; 0 retries; 0 escalations
