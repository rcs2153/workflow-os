# Workflow Catalog Repair And Recovery Plan Review

## 1. Executive Verdict

Plan accepted; proceed to non-mutating workflow catalog repair proposal helper
implementation.

The plan defines the right next boundary after explicit steward-review,
promotion catalog, and archive metadata writes: turn catalog-status conflicts
into deterministic repair/recovery proposals before any apply mode exists. It
does not authorize automatic cleanup, record deletion, record overwrite,
runtime registration, schema exposure, examples, hosted behavior, provider
calls, external writes, or release posture changes.

## 2. Scope Verification

The plan stayed within planning-only scope.

It does not authorize:

- catalog repair command implementation in the planning phase;
- automatic catalog repair;
- automatic cleanup;
- record deletion;
- record overwrite;
- active workflow rewrites;
- draft or archive movement;
- runtime workflow registration;
- runtime state creation;
- workflow schema changes;
- examples;
- hosted or team catalog backend behavior;
- provider calls;
- command execution or local check execution;
- write-capable adapters or provider mutation;
- release posture changes.

## 3. Boundary Assessment

The plan clearly separates status, recovery, and repair:

- `catalog-status` reports the current inventory and conflicts.
- recovery explains what happened and what should be reviewed.
- repair proposes explicit sidecar changes that could restore consistency.

The first implementation recommendation is non-mutating proposal or dry-run
behavior. That is the right boundary because promotion and archive sidecar
writes can now fail after file mutation, but automatic rollback and cleanup
semantics have not been designed or reviewed.

## 4. Source-Of-Truth Assessment

The plan preserves the correct source-of-truth hierarchy:

- active workflow files remain the loader-visible execution source of truth;
- drafts and archived drafts remain file artifacts;
- catalog records remain lifecycle sidecars;
- stewardship and archive records remain decision/action sidecars;
- runtime state remains separate from catalog state;
- Git remains optional review context, not the correctness database.

The plan correctly rejects any sidecar-over-loader posture. A mismatch between
an active file and a catalog record stays a conflict until a maintainer chooses
an explicit recovery action.

## 5. Conflict Inventory Assessment

The plan covers the conflict families currently produced by the reviewed
catalog index helper:

- duplicate active workflow ids and paths;
- missing active catalog records;
- active path and hash mismatches;
- missing active workflow files referenced by catalog records;
- missing owner, escalation, latest stewardship, and side-effect posture;
- draft stewardship hash mismatch;
- archive record missing archived draft;
- archive path and hash mismatches;
- corrupt or unsafe catalog/store inputs.

It also correctly requires the next implementation to consume the existing
index helper rather than invent a second conflict taxonomy.

## 6. Proposal Model Assessment

The proposed vocabulary is appropriately conservative. It distinguishes:

- missing-record creation candidates;
- metadata update candidates;
- manual workflow-file review;
- manual sidecar review;
- catalog-store cleanup;
- unsupported first-slice cases.

The proposed action kinds are useful enough for a first helper while leaving
room to shrink the type surface during implementation. The plan is especially
right to mark whether each proposal is safe for a future apply mode and whether
human review is required.

## 7. Safety And Recovery Assessment

The recovery posture is acceptable:

- no automatic rollback;
- no deletion;
- no overwrite;
- no active workflow mutation;
- no archived draft movement;
- no runtime state;
- no event-history append;
- no workflow registration.

The plan identifies the key safe future apply candidates: missing catalog
records for still-valid active workflows and missing archive records for
archived drafts whose hashes still match. It also correctly refuses to
auto-repair hash/path mismatches or duplicate active workflow conflicts.

## 8. Privacy And Redaction Assessment

The plan keeps repair output bounded and redaction-safe.

Allowed outputs are limited to workflow ids, repository-relative paths, stable
record ids, content hashes, conflict kinds, source categories, posture labels,
and bounded proposal summaries.

Forbidden outputs include raw workflow YAML, source file contents, command
output, provider payloads, parser payloads, package script bodies, CI logs,
environment values, credentials, authorization headers, private keys,
token-like values, and unbounded maintainer reasons.

That matches the existing catalog-status privacy posture.

## 9. Error-Handling Assessment

The proposed error family is stable and appropriately scoped:

- `cli.workflow_catalog.repair_catalog_root_rejected`;
- `cli.workflow_catalog.repair_read_failed`;
- `cli.workflow_catalog.repair_index_failed`;
- `cli.workflow_catalog.repair_proposal_failed`;
- `cli.workflow_catalog.repair_blocked`.

The plan requires corrupt store records to fail closed before proposal
generation unless a separately reviewed corruption-summary mode is introduced.
That is the right fail-closed default.

## 10. Test Plan Assessment

The future test plan covers the important first implementation risks:

- missing-record proposals;
- path/hash mismatch proposals;
- duplicate active workflow manual-review posture;
- corrupt store failure;
- unsafe catalog root rejection;
- dry-run non-mutation;
- no runtime state or events;
- deterministic ordering;
- bounded JSON;
- redaction-safe Debug;
- existing catalog-status, promotion, archive, and stewardship tests.

Non-blocking follow-up: the implementation phase should include at least one
test proving a proposal cannot silently become an apply operation.

## 11. Documentation Assessment

The roadmap and catalog persistence plan now state:

- workflow catalog repair and recovery planning is documented;
- the first recommended implementation is non-mutating proposal/dry-run
  behavior;
- automatic repair remains deferred;
- deletion and overwrite remain deferred;
- runtime registration remains deferred;
- schemas, examples, hosted behavior, providers, external writes, and release
  posture remain deferred.

## 12. Blockers

No blockers.

## 13. Non-Blocking Follow-Ups

- In the implementation phase, keep the first helper smaller than the full
  candidate model if the narrower type surface is enough.
- Add an explicit regression that dry-run/proposal generation writes no files,
  creates no runtime state, and appends no events.
- Consider whether corrupt-store summary should remain fail-closed or become a
  separate bounded summary mode after the first helper is reviewed.
- Defer any apply mode until proposal semantics are implemented and reviewed.

## 14. Recommended Next Phase

Recommended next phase: non-mutating workflow catalog repair proposal helper
implementation.

Why: the plan is specific enough to drive code, and the next useful runtime
move is not automatic cleanup. It is a deterministic helper that maps existing
catalog-status conflicts into bounded repair/recovery proposals while proving
that no files, records, runtime state, events, schemas, examples, provider
calls, or release posture changes are introduced.

## 15. Validation

Review-phase validation:

```text
npm run check:docs
```

Code checks are intentionally skipped for this docs-only review phase. The next
implementation phase must run the relevant Rust checks.

## 16. Governed Phase Metadata

- dogfood workflow id: `dg/review`
- run id: `run-1783540119731637000-2`
- approval id: `approval/run-1783540119731637000-2/review-scope-approved`
- approval outcome: granted by delegated maintainer
- event summary: phase close required after validation
