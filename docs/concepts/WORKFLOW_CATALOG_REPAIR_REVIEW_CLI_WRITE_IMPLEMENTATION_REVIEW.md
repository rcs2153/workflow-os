# Workflow Catalog Repair Review CLI Write Implementation Review

## 1. Executive Verdict

Needs blocker fixes.

The implementation stays within the approved narrow CLI write scope and
correctly avoids repair apply behavior, automatic repair, workflow rewrites,
runtime registration, provider calls, schemas, examples, writes, and release
posture changes.

One blocker remains: the accepted plan required stable CLI-specific error codes
for the explicit `--dry-run` and `--persist-review` requirements, but the
implementation currently routes those failures through the generic
`cli.usage` error path. The behavior fails closed and does not leak sensitive
values, but the public error contract is not the one this phase promised.

## 2. Scope Verification

The phase stayed within the approved CLI write scope.

Confirmed implemented scope:

- `workflow-os author workflow catalog-repair review` exists.
- `--dry-run` and `--persist-review` are required.
- proposal id, review id, decision, reviewer, and reason are explicit inputs.
- fresh repair proposals are recomputed before review selection.
- exactly one proposal is selected by id.
- review construction uses the existing repair review helper.
- persistence uses `LocalWorkflowCatalogStore::write_repair_review_record_if_absent`.
- successful execution writes one local `repair-reviews/` sidecar.
- human and JSON output are bounded.
- focused tests and documentation were added.

No accidental scope expansion found:

- no repair apply mode;
- no automatic repair;
- no catalog record mutation beyond the repair review sidecar;
- no active workflow rewrite;
- no draft/archive movement;
- no runtime workflow registration;
- no runtime state creation;
- no event or audit append;
- no report artifact generation;
- no workflow schema changes;
- no examples;
- no hosted/team catalog backend behavior;
- no provider calls;
- no local command/check execution;
- no write-capable adapter behavior;
- no release posture change.

## 3. CLI API Assessment

The CLI shape is appropriate for the phase. Requiring both `--dry-run` and
`--persist-review` keeps the operator posture explicit: the command persists a
review sidecar from a fresh dry-run proposal but does not imply repair
application.

The decision vocabulary matches the existing model and accepts both hyphenated
CLI labels and underscore serialized labels. This is pragmatic and bounded.

The help text documents the command and its required flags clearly enough for
the current experimental authoring surface.

## 4. Fresh Proposal Assessment

The command recomputes the catalog status context and repair proposals before
selecting a proposal by id. It does not read persisted reviews as proposal
truth, does not fabricate proposal ids, and fails closed when the proposal id is
not present in the fresh proposal set.

The fresh proposal identity remains the source of truth for review persistence.
This is the right boundary before any future apply planning.

## 5. Persistence Assessment

The write path delegates to
`LocalWorkflowCatalogStore::write_repair_review_record_if_absent`, preserving
the reviewed store-helper invariants:

- duplicate review ids are rejected;
- stale proposal identity is rejected;
- repair review records live under the dedicated `repair-reviews/` sidecar
  directory;
- successful review persistence does not create workflow catalog, stewardship,
  archive, runtime state, or artifact records.

The implementation maps duplicate and stale store-helper errors to bounded CLI
errors. That part is appropriate.

## 6. Error Handling Assessment

Most error handling is stable and non-leaking:

- unknown proposal selection maps to
  `cli.workflow_catalog.repair_review.proposal_not_found`;
- ambiguous proposal selection has a dedicated bounded error path;
- invalid decision labels map to
  `cli.workflow_catalog.repair_review.invalid_decision`;
- invalid review construction maps to
  `cli.workflow_catalog.repair_review.invalid_review`;
- duplicate persistence maps to
  `cli.workflow_catalog.repair_review.duplicate_review`;
- stale proposal persistence maps to
  `cli.workflow_catalog.repair_review.stale_proposal`.

Blocker:

- missing `--dry-run` currently returns generic `cli.usage` instead of the
  plan-required `cli.workflow_catalog.repair_review.requires_dry_run`;
- missing `--persist-review` currently returns generic `cli.usage` instead of
  the plan-required
  `cli.workflow_catalog.repair_review.requires_persist_review`;
- the focused test asserts the human message contains the required flag text,
  but it does not assert the stable error code contract.

This is specific and fixable. The implementation should return the promised
repair-review-specific stable codes for those two explicit boundary failures
and add focused tests that assert those codes.

## 7. Privacy And Redaction Assessment

The CLI output does not copy raw workflow YAML, catalog payloads, source
contents, command output, provider payloads, parser payloads, CI logs,
environment values, credentials, authorization headers, private keys, tokens, or
secret-like values.

The review reason is not printed in human or JSON success output. Secret-like
review reasons fail through model validation without leaking the raw value.

The persisted review record may store a bounded reviewer reason after model
validation, which is consistent with the reviewed store-helper boundary and is
not exposed through CLI output.

## 8. Test Quality Assessment

Tests cover the important happy path and most boundary cases:

- successful persisted repair review sidecar write;
- bounded human output;
- bounded JSON output;
- required explicit flags at the behavior level;
- unknown proposal rejection without writes;
- duplicate review id rejection without overwrite;
- secret-like reason rejection without leakage;
- no runtime state creation;
- no unrelated catalog sidecar directories created by the successful write path.

Missing/blocking test coverage:

- missing `--dry-run` should assert
  `cli.workflow_catalog.repair_review.requires_dry_run`;
- missing `--persist-review` should assert
  `cli.workflow_catalog.repair_review.requires_persist_review`.

Non-blocking possible additions:

- an explicit unsafe catalog root test for this command, although the command
  reuses existing catalog-root resolution and adjacent catalog commands already
  test that boundary;
- an explicit invalid serialized persisted review fixture is covered at store
  helper level rather than this CLI layer.

## 9. Documentation Review

The documentation correctly says:

- explicit CLI repair review write behavior is implemented;
- default repair dry-run remains non-mutating;
- persisted repair reviews are not apply permission;
- repair apply mode is not implemented;
- automatic repair is not implemented;
- workflow rewrites, runtime registration, schemas, examples, provider calls,
  hosted behavior, writes, and release posture changes remain deferred.

The implementation report records the governed dogfood run and validation
commands honestly.

## 10. Validation Semantics Assessment

Local validation for the implementation phase passed before this review:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test -p workflow-cli --test cli author_workflow_catalog_repair_review`;
- `cargo test --workspace`;
- `npm run check:docs`.

Review validation is rerun for this review artifact and recorded below.

## 11. Blockers

1. Replace generic `cli.usage` for missing `--dry-run` in
   `workflow-os author workflow catalog-repair review` with stable code
   `cli.workflow_catalog.repair_review.requires_dry_run`.

2. Replace generic `cli.usage` for missing `--persist-review` in
   `workflow-os author workflow catalog-repair review` with stable code
   `cli.workflow_catalog.repair_review.requires_persist_review`.

3. Add focused CLI tests asserting those exact stable error codes and confirming
   the failures remain non-mutating and non-leaking.

## 12. Non-Blocking Follow-Ups

- Consider adding optional CLI citations for approval, policy, evidence,
  validation, and work-report references in a separately planned phase.
- Consider adding a catalog-status snapshot reference before any future repair
  apply planning.
- Consider an explicit unsafe catalog-root test for this specific subcommand
  even though the shared catalog-root helper is already covered nearby.

## 13. Recommended Next Phase

Workflow catalog repair review CLI write blocker fix.

The fix should be intentionally tiny: stable error-code repair for the two
missing-flag paths, focused tests, validation, and a short blocker-fix report.
No repair apply mode, automatic repair, catalog mutation expansion, runtime
registration, schemas, examples, provider calls, writes, or release posture
changes should be introduced.

## 14. Governed Review Metadata

- dogfood workflow id: `dg/review`
- run id: `run-1783554492136684000-2`
- approval id:
  `approval/run-1783554492136684000-2/review-scope-approved`
- approval outcome: granted by delegated maintainer
- out-of-kernel work: repository review, shell validation, documentation edits,
  git, PR, and merge operations are performed by Codex/human execution layer;
  the kernel coordinated governance only.

## 15. Review Validation Commands

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
```

Result: passed.
