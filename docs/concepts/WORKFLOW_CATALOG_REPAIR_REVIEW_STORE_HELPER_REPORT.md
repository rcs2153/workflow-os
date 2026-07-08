# Workflow Catalog Repair Review Store Helper Report

## 1. Executive Summary

The local workflow catalog store now supports persisted repair proposal review
sidecars as a core helper-only boundary.

`LocalWorkflowCatalogStore` can write, read, and list validated
`WorkflowCatalogRepairProposalReview` records under `repair-reviews/`. A review
can be written only when it still matches a fresh repair proposal identity, so
persisted review records cannot silently authorize stale repair planning.

This phase does not implement CLI review write behavior, repair apply mode,
automatic repair, catalog mutation, workflow rewrites, schemas, examples,
provider calls, hosted behavior, writes, or release posture changes.

## 2. Scope Completed

- Added `repair-reviews/` as a sibling local catalog sidecar directory.
- Added `LocalWorkflowCatalogStore::write_repair_review_record_if_absent`.
- Added `LocalWorkflowCatalogStore::read_repair_review_record`.
- Added `LocalWorkflowCatalogStore::list_repair_review_records`.
- Added repair review counts to `WorkflowCatalogStoreHealth`.
- Reused existing encoded id file-name posture for review sidecar paths.
- Required fresh proposal identity matching before persistence.
- Rejected duplicate review ids without overwrite.
- Added focused store-helper tests.
- Updated roadmap and planning docs.

## 3. Scope Explicitly Not Completed

- No CLI repair review write command.
- No repair apply mode.
- No automatic catalog repair.
- No catalog record creation, update, deletion, overwrite, or cleanup from this
  helper.
- No active workflow rewrite.
- No draft or archive movement.
- No runtime workflow registration.
- No runtime state creation.
- No event or audit append.
- No report artifact generation.
- No workflow schema changes.
- No examples.
- No provider calls.
- No local check or command execution.
- No hosted or team catalog backend behavior.
- No write-capable adapter behavior.
- No release posture changes.

## 4. Helper API Summary

The implemented API is local, explicit, and model-backed:

```text
LocalWorkflowCatalogStore::write_repair_review_record_if_absent(
    review,
    fresh_proposal,
)

LocalWorkflowCatalogStore::read_repair_review_record(review_id)

LocalWorkflowCatalogStore::list_repair_review_records()
```

The write helper validates stale proposal posture before persistence and writes
exactly one JSON sidecar file when the id is unused.

## 5. Validation Boundary Summary

The store helper relies on existing review constructors and serde validation for
bounded record contents. Before writing, it calls the existing fresh-proposal
identity check and maps stale review attempts to a stable non-leaking store
error.

Duplicate review ids fail closed before overwrite. Read and list paths validate
that the record id matches the encoded storage address.

## 6. Redaction And Privacy Summary

The helper does not copy raw workflow YAML, raw catalog payloads, source
contents, command output, provider payloads, parser payloads, CI logs,
environment values, credentials, authorization headers, private keys, tokens, or
secret-like values.

Repair review sidecars can contain bounded reviewer reasons and source
references that passed model validation. Store errors remain stable and do not
echo raw review ids, stale proposal paths, or invalid serialized payload values.

## 7. Test Coverage Summary

Focused tests cover:

- repair review write/read/list round trip;
- deterministic review id ordering;
- `repair-reviews/` sidecar path isolation;
- duplicate review rejection without overwrite;
- stale proposal rejection before persistence;
- invalid serialized repair review failure without leaking payload values;
- existing catalog, stewardship, archive, health, identity mismatch, corrupt
  JSON, and debug non-leakage behavior.

## 8. Commands Run And Results

```text
cargo test -p workflow-core --test workflow_catalog_store
```

Result: passed.

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
```

Result: passed.

Governed phase close:

- dogfood workflow id: `dg/implement`
- run id: `run-1783550026888584000-2`
- approval id:
  `approval/run-1783550026888584000-2/implementation-approved`
- approval outcome: granted by delegated maintainer
- event summary: 39 events; 1 approval; 0 retries; 0 escalations
- event kinds:
  `ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8,
  RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1,
  RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6,
  SkillInvocationSucceeded:6, StepScheduled:6`
- out-of-kernel work: repository edits, Rust/doc validation, git, PR, and merge
  operations are performed by Codex/human execution layer; the kernel
  coordinated governance only.

## 9. Remaining Known Limitations

- Repair review persistence is available only through the core store helper.
- No CLI review write command exists.
- Persisted repair reviews are not repair-apply permission.
- Supersession, replacement, deletion, and cleanup semantics are not designed.
- Persisted reviews do not cite catalog-status snapshot ids yet.
- Hosted/team catalog persistence remains future work.

## 10. Recommended Next Phase

Recommended next phase: workflow catalog repair review store helper review.

The review should verify scope containment, stale proposal rejection,
non-leaking error behavior, deterministic sidecar persistence, and the continued
absence of CLI write behavior or repair apply mode.
