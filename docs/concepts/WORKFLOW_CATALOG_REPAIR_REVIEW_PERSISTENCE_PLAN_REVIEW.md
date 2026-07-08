# Workflow Catalog Repair Review Persistence Plan Review

## 1. Executive Verdict

Plan accepted; proceed to local repair review store helper implementation.

The plan defines a conservative persistence boundary for
`WorkflowCatalogRepairProposalReview` records. It correctly treats persisted
repair reviews as local governance sidecars, not repair application authority,
and keeps CLI write behavior and apply mode deferred.

## 2. Scope Verification

The plan stayed within planning-only scope.

No accidental authorization was found for:

- persistence implementation in the planning phase;
- CLI review write behavior;
- repair apply mode;
- automatic catalog repair;
- catalog record creation, update, overwrite, deletion, or cleanup;
- active workflow rewrites;
- draft or archive movement;
- runtime workflow registration;
- runtime state creation;
- event or audit append;
- report artifact generation;
- workflow schema changes;
- examples;
- hosted or team catalog backend behavior;
- provider calls;
- local check or command execution;
- write-capable adapters;
- release posture changes.

## 3. Storage Boundary Assessment

The proposed local storage layout is appropriate:

```text
.workflow-os/catalog/
  repair-reviews/
    <safe-repair-review-id>.json
```

Separating repair reviews from workflow catalog records, stewardship records,
and archive records avoids mixing review decisions with lifecycle sidecars that
describe active workflow state. The plan also preserves active workflow files
as the execution source of truth and keeps repair reviews out of runtime state.

## 4. Identity And File Naming Assessment

The plan correctly requires file names to be derived from
`WorkflowCatalogRepairProposalReviewId` through the existing safe file-name
encoding posture.

Verified planning requirements:

- canonical review id remains in the record;
- file name is a storage address only;
- path traversal and unsafe prefixes fail closed;
- duplicate records are rejected;
- raw rejected ids are not leaked in errors.

This matches the existing catalog-store direction.

## 5. Write Timing Assessment

The planned write timing is conservative and useful.

Future persistence must require:

1. a fresh dry-run proposal;
2. explicit review id, reviewer, reason, and decision;
3. successful in-memory review construction;
4. review/proposal identity match;
5. safe target store path;
6. no duplicate review record;
7. explicit caller opt-in.

Dry-run and preview paths remain non-mutating. This avoids surprising users
who currently rely on `catalog-repair --dry-run` as a read-only inspection
surface.

## 6. Duplicate And Replacement Assessment

Rejecting duplicate repair review ids in the first persistence slice is the
right choice.

The plan correctly defers update, overwrite, replace, delete, and supersession
semantics. That keeps the initial store helper append-oriented and reviewable.

## 7. Stale Review Assessment

The plan preserves the most important safety rule: a persisted review record
must not be reused without a fresh matching proposal.

Future apply planning must still validate:

- fresh dry-run proposal exists;
- persisted review exists;
- proposal identity matches the review;
- decision kind permits future apply planning;
- separate policy and mutation checks pass.

The plan does not let persisted review become mutation permission by itself.

## 8. CLI Surface Assessment

The candidate CLI shape is appropriately explicit and opt-in:

```text
workflow-os author workflow catalog-repair review ... --persist-review
```

The plan correctly recommends implementing a store helper before CLI behavior.
That sequence reduces risk because file persistence can be reviewed and tested
before user-facing command semantics are introduced.

## 9. Privacy And Redaction Assessment

The plan treats repair review records as sensitive governance metadata.

It correctly forbids storing or emitting:

- raw workflow YAML;
- raw catalog record payloads;
- source contents;
- command output;
- provider payloads;
- parser payloads;
- CI logs;
- environment values;
- credentials;
- authorization headers;
- private keys;
- tokens;
- secret-like values.

Bounded reviewer reasons and repository-relative source references are allowed
only through existing validation. Future persistence implementation should keep
Debug, errors, serialization, and deserialization fail-closed and non-leaking.

## 10. Error Handling Assessment

The planned stable error code families are suitable for a future store helper
and CLI surface:

- invalid root;
- invalid review id;
- duplicate review;
- write/read failure;
- invalid record;
- stale proposal;
- proposal not found;
- persist failure.

The plan explicitly forbids leaking raw paths, reviewer reasons, source
references, record contents, snippets, command output, provider payloads, and
secret-like values in errors.

## 11. Test Plan Assessment

The planned tests are sufficient for the next store-helper implementation.

They cover:

- valid write/read round trip;
- duplicate rejection;
- unsafe root rejection;
- unsafe or secret-like review id rejection;
- secret-like reviewer reason rejection;
- stale proposal rejection before persistence;
- no workflow catalog record creation;
- no active workflow mutation;
- dry-run writes nothing;
- no runtime state or events;
- Debug/serialization/deserialization safety;
- existing catalog and repair tests.

No blocking test-plan gaps were found.

## 12. Documentation Review

Documentation now states:

- repair review persistence is planned;
- persisted reviews are local sidecars, not apply permission;
- the first implementation should be a store helper only;
- CLI review write behavior is deferred;
- repair apply mode is deferred;
- automatic repair is deferred;
- catalog mutation, deletion, overwrite, runtime registration, schemas,
  examples, providers, hosted behavior, writes, and release posture changes
  remain deferred.

## 13. Blockers

No blockers.

## 14. Non-Blocking Follow-Ups

- During implementation, reuse the existing catalog-store file-name encoding
  helpers rather than inventing a second path policy.
- Consider whether the store helper should expose list-by-review-id only in the
  first slice, with proposal-id filtering deferred.
- Keep the future CLI surface separate from the store helper until the helper
  is reviewed.

## 15. Recommended Next Phase

Recommended next phase: local repair review store helper implementation.

Why: the persistence plan is accepted and the next useful slice is a
model-backed/file-backed helper that can persist validated repair review
records under `repair-reviews/` without adding CLI behavior or apply mode.

## 16. Validation

Review-phase validation:

```text
npm run check:docs
```

Result: passed.

Rust validation was not run because this was a documentation-only planning
review.

## 17. Governed Phase Metadata

- dogfood workflow id: `dg/review`
- run id: `run-1783549463771811000-2`
- approval id:
  `approval/run-1783549463771811000-2/review-scope-approved`
- approval outcome: granted by delegated maintainer
- event summary: 39 events; 1 approval; 0 retries; 0 escalations
- event kinds:
  `ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8,
  RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1,
  RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6,
  SkillInvocationSucceeded:6, StepScheduled:6`
- out-of-kernel work: documentation review, documentation edit, validation,
  git, PR, and merge operations were executed by Codex/human execution layer;
  the kernel coordinated governance only
