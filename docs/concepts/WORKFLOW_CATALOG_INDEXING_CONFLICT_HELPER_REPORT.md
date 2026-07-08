# Workflow Catalog Indexing Conflict Helper Report

## 1. Executive Summary

The pure in-memory workflow catalog indexing and conflict helper is
implemented in `workflow-core`.

The helper consumes explicit active workflow, inactive draft, archived draft,
catalog record, stewardship record, and archive record inputs. It returns a
deterministically ordered catalog inventory plus bounded conflict disclosures.

This phase does not wire the helper into commands, runtime workflow
registration, promotion, archive, catalog writes, schemas, examples, providers,
hosted behavior, write-capable adapters, or release posture.

## 2. Scope Completed

- Added active workflow, draft, and archived draft summary input models.
- Added `WorkflowCatalogIndexInput`.
- Added `WorkflowCatalogIndex`.
- Added `WorkflowCatalogConflict`.
- Added conflict kind, severity, and source vocabulary.
- Added `build_workflow_catalog_index`.
- Added read-only accessors for existing catalog, stewardship, and archive
  record fields needed by the index helper.
- Added focused tests for deterministic indexing and conflict disclosure.
- Updated roadmap and related implementation plans.

## 3. Scope Explicitly Not Completed

This phase did not implement:

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

## 4. Helper API Summary

The new exported helper is:

```text
build_workflow_catalog_index(input: WorkflowCatalogIndexInput) -> Result<WorkflowCatalogIndex, WorkflowOsError>
```

The helper accepts explicit inputs only. It does not discover files, read
workflow YAML, load catalog store roots, mutate records, or write derived index
files.

The index exposes read-only accessors for:

- active workflow summaries;
- inactive draft summaries;
- archived draft summaries;
- catalog records;
- stewardship records;
- archive records;
- conflict disclosures;
- conflict counts by severity.

## 5. Conflict Policy Summary

The first conflict taxonomy covers deterministic exact conflicts only.

Blocker disclosures include:

- duplicate active workflow id;
- duplicate active workflow path;
- active workflow missing catalog record when strict coverage is requested;
- active catalog record missing loader-visible workflow file;
- catalog active path mismatch;
- catalog active content hash mismatch;
- draft stewardship hash mismatch;
- archive record missing archived draft;
- archive path mismatch;
- archive hash mismatch.

Warning disclosures include:

- active workflow missing catalog record when strict coverage is not requested;
- missing owner;
- missing escalation contact;
- missing latest stewardship decision;
- missing side-effect posture.

The helper does not infer semantic similarity, purpose overlap, or business
process overlap using model judgment.

## 6. Determinism Summary

Index construction sorts:

- active workflows by workflow id and path;
- drafts by workflow id and path;
- archived drafts by workflow id and archive path;
- catalog records by catalog record id;
- stewardship records by decision id;
- archive records by archive record id;
- conflicts by severity, kind, workflow id, source, and bounded source
  reference.

No output ordering depends on caller vector ordering, filesystem traversal, or
hash-map iteration.

## 7. Privacy And Redaction Summary

The helper stores only validated ids, repository-relative paths, content hashes,
schema versions, lifecycle/status codes, record references, bounded conflict
kinds, severities, and counts.

It does not store or copy:

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

Unsafe, absolute, traversal, overlong, and secret-like helper paths/statuses are
rejected with stable non-leaking validation errors. `WorkflowCatalogConflict`
debug output redacts source references.

## 8. Test Coverage Summary

Focused tests cover:

- valid empty index construction;
- deterministic active workflow ordering;
- draft, archived draft, catalog, stewardship, and archive inputs;
- duplicate active workflow id blocker;
- duplicate active workflow path blocker;
- active workflow without catalog record warning by default;
- active workflow without catalog record blocker when strict coverage is
  requested;
- catalog active path mismatch blocker;
- catalog active content hash mismatch blocker;
- catalog active missing workflow file blocker;
- stale draft stewardship hash blocker;
- archive hash mismatch blocker;
- archive record missing archived draft blocker;
- missing owner/escalation/stewardship/side-effect warning disclosures;
- unsafe and secret-like path rejection without leaking raw values;
- conflict debug non-leakage.

Existing workflow catalog and catalog store tests were run as adjacent
non-regression coverage.

## 9. Commands Run And Results

Dogfood governance:

```text
workflow_id: dg/implement
run_id: run-1783519780126361000-2
approval_id: approval/run-1783519780126361000-2/implementation-approved
approval_outcome: granted
events_total: 39
event_summary: ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8, RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1, RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6, SkillInvocationSucceeded:6, StepScheduled:6
```

Commands run:

```text
cargo fmt --all
cargo test -p workflow-core --test workflow_catalog_index
cargo test -p workflow-core --test workflow_catalog --test workflow_catalog_store
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
```

Results: all commands passed.

## 10. Remaining Known Limitations

- No command consumes the helper yet.
- No catalog health command exists.
- No authoring command writes catalog records.
- No promotion command requires persisted stewardship decisions.
- No archive command writes archive records.
- No strict enforcement policy is wired to promotion or steward-review.
- No hosted/team workflow catalog backend exists.
- No schema or example integration exists.
- Semantic workflow overlap detection remains deferred.

## 11. Recommended Next Phase

Recommended next phase: workflow catalog indexing/conflict helper review.

The helper is a new governance primitive that will eventually influence
promotion and catalog health behavior. It should be reviewed before command
integration, persisted stewardship checks, archive metadata writes, or strict
catalog enforcement are added.
