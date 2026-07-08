# Archive Metadata Write Implementation Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation stays inside the approved opt-in local archive metadata
boundary. `workflow-os author workflow archive-draft` remains unchanged by
default, and `--persist-archive-record` adds one bounded archive sidecar record
after a successful eligible draft archive move and post-move project
validation.

The phase is ready to proceed after this review. The next recommended phase is
workflow catalog repair and recovery planning.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented:

- explicit `author workflow archive-draft --persist-archive-record`;
- optional `--catalog-root`, accepted only with archive persistence;
- optional `--stewardship-decision-id`, accepted only with archive persistence;
- pre-move verification of supplied persisted stewardship decisions;
- one validated local `WorkflowArchiveRecord` sidecar write;
- default archive behavior unchanged;
- bounded text and JSON output fields;
- focused CLI regression tests;
- roadmap, implementation-plan, and implementation-report documentation.

No accidental implementation found for:

- runtime workflow registration;
- catalog repair or update of existing workflow catalog records;
- automatic archive cleanup;
- draft deletion;
- runtime state creation;
- workflow run creation;
- command execution or local check execution;
- provider calls;
- report artifacts;
- workflow schema changes;
- examples;
- hosted or distributed catalog behavior;
- RBAC, IdP integration, notifications, or admin UI;
- write-capable adapters or provider mutation;
- release posture changes.

## 3. CLI Boundary Assessment

The CLI shape is appropriately explicit:

```sh
workflow-os author workflow archive-draft \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-reason> \
  --persist-archive-record \
  [--catalog-root .workflow-os/catalog] \
  [--stewardship-decision-id stewardship/<id>]
```

Default `author workflow archive-draft` still moves one eligible draft into
`workflows/drafts/archive/` and does not create
`.workflow-os/catalog/archives/`.

`--catalog-root` and `--stewardship-decision-id` fail closed when supplied
without `--persist-archive-record`.

Dry-run accepts the archive metadata flags and performs validation without
moving the draft or writing an archive record.

## 4. Stewardship Citation Assessment

Supplied stewardship decisions are verified before draft movement.

The implementation verifies:

- supplied decision id matches the persisted record id;
- decision kind is `ApprovedForPromotion`;
- workflow id matches the candidate workflow;
- draft path matches the draft being archived;
- candidate content hash matches the current draft content hash.

Missing, stale, corrupt, non-approving, or mismatched stewardship records fail
closed before the draft move and before archive sidecar writes.

No persisted stewardship decision is required in the first slice. That is
consistent with the plan: single-user local archive hygiene can remain
ergonomic while stricter future profiles may require persisted stewardship
coverage.

## 5. Archive Record Construction Assessment

Archive records are constructed through `WorkflowArchiveRecord::new`, not
serialized ad hoc.

The record stores bounded references and metadata:

- deterministic archive record id;
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
- conservative sensitivity and explicit redaction metadata.

The implementation does not store raw workflow YAML, raw source contents,
command output, provider payloads, parser payloads, package script bodies,
environment values, credentials, authorization headers, private keys, or
token-like values in the archive record.

The implementation report correctly documents that active workflow content hash
population remains deferred.

## 6. Write Timing And Atomicity Assessment

The implementation validates catalog inputs and optional stewardship citation
before draft movement.

The draft archive move happens first, then the project is revalidated, then the
archive record is written through `LocalWorkflowCatalogStore` with
write-if-absent behavior.

If archive record persistence fails after archive movement succeeds, the command
returns a stable partial-integration error:

- `cli.workflow_authoring.archive_record_persist_failed`

No automatic rollback is attempted. That matches the plan and avoids inventing
unreviewed recovery semantics.

## 7. Privacy And Redaction Assessment

The implementation remains redaction-safe.

Observed safeguards:

- archive reasons are validated before loading or mutation;
- error messages use stable codes;
- stale stewardship mismatch errors do not echo draft contents or reason text;
- text output discloses ids and posture fields, not raw reasons or YAML;
- JSON output uses escaped bounded values;
- archive records store `archive_reason_summary: "provided"` rather than the
  submitted archive reason;
- tests assert bounded archive and stewardship reasons are not leaked.

No raw provider payloads, command output, parser payloads, environment values,
credentials, authorization headers, private keys, or token-like values are
stored or emitted by the new path.

## 8. Test Quality Assessment

Focused CLI tests cover:

- default archive dry-run remains non-mutating;
- default archive move remains unchanged and writes no archive records;
- opt-in archive persistence writes one archive record;
- archive records cite workflow id, original draft path, archive path, prior
  status, and verified stewardship id;
- `--catalog-root` without `--persist-archive-record` fails without writes;
- stale stewardship decision rejection before draft movement;
- superseded-by-active archive eligibility;
- active candidate rejection;
- archive destination overwrite rejection;
- JSON boundedness for archive output;
- secret-like archive reason rejection without leakage.

Existing workspace tests also cover:

- local catalog store write-if-absent behavior;
- archive model constructor and serde validation;
- catalog status inventory and conflict helper behavior;
- promotion and stewardship persistence paths;
- project validation after authoring mutations.

Non-blocking test gaps:

- add a targeted CLI test for duplicate archive record persistence when the
  archive sidecar already exists;
- add a targeted test for late archive sidecar write failure after the archive
  move succeeds, if it can be induced without test-only hooks;
- add a `catalog-status` integration assertion showing that a matching archive
  record reduces the relevant archived-draft conflict surface.

## 9. Documentation Review

Docs now state:

- opt-in archive metadata writes are implemented;
- default archive behavior remains unchanged;
- archive writes require explicit `--persist-archive-record`;
- `--catalog-root` and `--stewardship-decision-id` are scoped to archive
  persistence;
- automatic archive cleanup is not implemented;
- draft deletion is not implemented;
- runtime registration is not implemented;
- catalog repair and workflow catalog record update are not implemented;
- schemas are not changed;
- examples are not added;
- hosted behavior, provider calls, external writes, write-capable adapters, and
  release posture changes remain deferred.

The implementation report includes dogfood run id, approval id, approval
outcome, event summary, validation summary, limitations, and next phase.

## 10. Validation Semantics Assessment

Validation behavior remains deterministic:

- invalid archive reason fails before project loading and mutation;
- invalid archive status fails before mutation;
- existing archive destination fails before mutation;
- invalid catalog flags fail before mutation;
- invalid catalog roots fail through the existing safe catalog-root resolver;
- invalid stewardship ids fail through the typed id constructor;
- missing, stale, corrupt, or mismatched stewardship records fail before draft
  movement;
- post-move project validation remains required before archive sidecar
  persistence;
- duplicate archive record writes fail closed through the store.

Default validation and archive semantics remain unchanged outside the explicit
archive persistence flag.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Add a focused regression test for duplicate archive record persistence.
- Add a focused regression test for late archive sidecar write failure after
  draft movement if it can be induced without widening production code.
- Add a `catalog-status` integration assertion for matching archive records.
- Decide in a future strict-profile phase whether persisted stewardship should
  become mandatory for archive sidecar writes.
- Plan recovery semantics for partial catalog sidecar failures before adding
  any automatic cleanup or repair behavior.

## 13. Recommended Next Phase

Recommended next phase: workflow catalog repair and recovery planning.

Why:

- stewardship persistence is implemented and reviewed;
- promotion catalog writes are implemented and reviewed;
- archive metadata writes are implemented and accepted by this review;
- the next risk is not another sidecar type, but operator recovery when catalog
  metadata is missing, stale, duplicated, or partially written;
- repair and recovery must be planned before automatic cleanup, catalog update
  semantics, or stricter catalog enforcement;
- runtime registration, schemas, examples, provider calls, hosted behavior, and
  writes outside the local catalog remain deferred.

## 14. Validation Commands Run

Implementation validation before review:

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- GitHub Actions on PR #132: all required checks passed.

Review validation after this document:

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.

Governed review run:

- workflow: `dg/review`
- run: `run-1783538535203618000-2`
- approval: `approval/run-1783538535203618000-2/review-scope-approved`
- approval outcome: granted by delegated maintainer for archive metadata write
  implementation review
- close summary: completed terminal run with 39 events, one approval, zero
  retries, and zero escalations
- event kinds: `RunCreated`, `RunValidated`, `RunStarted`, `StepScheduled`,
  `PolicyDecisionRecorded`, `ApprovalRequested`, `ApprovalGranted`,
  `RunResumed`, `SkillInvocationRequested`, `SkillInvocationStarted`,
  `SkillInvocationSucceeded`, and `RunCompleted`
- kernel posture: review scope approved before review work; review produced
  this document and roadmap wording only; no implementation fixes, runtime
  registration, catalog repair, schemas, examples, provider calls, hosted
  behavior, external writes, or release posture changes were introduced.
