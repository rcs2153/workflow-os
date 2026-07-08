# Promotion Catalog Write Implementation Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation stays inside the approved opt-in local catalog write
boundary. `workflow-os author workflow promote` remains unchanged by default,
and `--persist-catalog-record` adds a bounded sidecar catalog write after active
promotion validation.

The phase is ready to proceed after this review. The next recommended phase is
archive metadata write planning.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented:

- explicit `author workflow promote --persist-catalog-record`;
- optional `--catalog-root`, accepted only with catalog persistence;
- optional `--stewardship-decision-id`, accepted only with catalog persistence;
- pre-mutation verification of supplied persisted stewardship decisions;
- one validated local workflow catalog record write;
- default promotion path unchanged;
- bounded text and JSON output fields;
- focused tests;
- CLI and roadmap documentation;
- implementation report.

No accidental implementation found for:

- runtime workflow registration;
- automatic workflow generation;
- automatic promotion;
- archive metadata writes;
- catalog repair;
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
workflow-os author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason> \
  --persist-catalog-record \
  [--catalog-root .workflow-os/catalog] \
  [--stewardship-decision-id stewardship/<id>]
```

Default `author workflow promote` still writes only one active workflow file and
does not create `.workflow-os/catalog/workflows/`.

`--catalog-root` and `--stewardship-decision-id` fail closed when supplied
without `--persist-catalog-record`.

Dry-run accepts the catalog flags and performs validation without writing the
active workflow file or catalog record.

## 4. Stewardship Citation Assessment

Supplied stewardship decisions are verified before active workflow mutation.

The implementation verifies:

- supplied decision id matches the persisted record id;
- decision kind is `ApprovedForPromotion`;
- workflow id matches the candidate workflow;
- draft path matches the promoted draft;
- candidate content hash matches the current draft content hash.

Missing, stale, non-approving, corrupt, or mismatched stewardship records fail
closed before active workflow and catalog writes.

No persisted stewardship decision is required in the first slice. That is
consistent with the plan: single-user local promotion can remain ergonomic
while stricter future profiles may require a persisted decision.

## 5. Catalog Record Construction Assessment

Catalog records are constructed through `WorkflowCatalogRecord::new`, not
serialized ad hoc.

The record stores bounded references and summaries:

- deterministic catalog record id;
- workflow id;
- active workflow path;
- draft source path;
- workflow content hash;
- schema version;
- active lifecycle status;
- owner and escalation references from the workflow spec;
- authority scope summary;
- evidence/check/report posture summary;
- side-effect posture summary;
- verified stewardship/promotion decision id when supplied;
- conservative sensitivity and explicit redaction metadata.

The implementation does not store raw workflow YAML, raw source contents,
command output, provider payloads, parser payloads, package script bodies,
environment values, credentials, or token-like values in the workflow catalog
record.

## 6. Write Timing And Atomicity Assessment

The implementation validates catalog inputs and optional stewardship citation
before active workflow mutation.

The active workflow file is written first, then the project is revalidated, then
the catalog record is written through `LocalWorkflowCatalogStore` with
write-if-absent behavior.

If the catalog record write fails after active promotion succeeds, the command
returns a stable partial-integration error:

- `cli.workflow_authoring.promotion_catalog_persist_failed`

No automatic rollback is attempted. That matches the plan. A focused regression
test for this late failure path should be added before archive metadata write
implementation, but the current behavior is explicit and not a phase blocker.

## 7. Privacy And Redaction Assessment

The implementation remains redaction-safe.

Observed safeguards:

- review reasons are validated through existing bounded review surfaces;
- error messages use stable codes;
- stale stewardship mismatch errors do not echo draft contents;
- text output discloses ids and status fields, not raw reasons or YAML;
- JSON output uses escaped bounded values;
- tests assert bounded review reasons and updated draft literals are not leaked.

No raw provider payloads, command output, parser payloads, environment values,
credentials, authorization headers, private keys, or token-like values are
stored or emitted by the new path.

## 8. Test Quality Assessment

Focused CLI tests cover:

- successful opt-in catalog record write when a persisted stewardship decision
  is supplied;
- default promotion without catalog writes;
- rejection of `--catalog-root` without `--persist-catalog-record`;
- stale stewardship decision rejection before active workflow and catalog writes;
- non-leakage of bounded review reasons in output;
- continued active promotion validation and existing promote behavior.

Existing workspace tests also cover:

- local catalog store duplicate rejection and validation;
- workflow catalog model validation and serde behavior;
- active promotion preflight and active-placement validation;
- CLI JSON boundedness for promotion;
- project validation after active promotion.

Non-blocking test gaps:

- add a targeted CLI test for late catalog sidecar write failure after active
  promotion succeeds, such as duplicate catalog record id with absent active
  file, to lock in partial-integration behavior;
- add a JSON success assertion for `--persist-catalog-record` fields if the JSON
  shape becomes more integration-relevant.

## 9. Documentation Review

Docs now state:

- opt-in promotion catalog writes are implemented;
- default promotion remains unchanged;
- catalog writes require explicit `--persist-catalog-record`;
- `--catalog-root` and `--stewardship-decision-id` are scoped to catalog
  persistence;
- runtime registration is not implemented;
- archive metadata writes are not implemented;
- catalog repair is not implemented;
- schemas are not changed;
- examples are not added;
- hosted behavior, provider calls, external writes, write-capable adapters, and
  release posture changes remain deferred.

The implementation report includes dogfood run id, approval id, approval
outcome, event summary, validation summary, limitations, and next phase.

## 10. Validation Semantics Assessment

Validation behavior remains deterministic:

- invalid catalog flags fail before mutation;
- invalid catalog roots fail through the existing safe catalog-root resolver;
- invalid stewardship ids fail through the typed id constructor;
- stale or mismatched stewardship records fail before active workflow mutation;
- post-write project validation remains required before catalog persistence;
- duplicate catalog writes fail closed through the store.

Default validation and promotion semantics remain unchanged outside the explicit
catalog persistence flag.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Add a focused regression test for late catalog sidecar write failure after
  active workflow promotion succeeds.
- Consider a JSON success-path assertion for catalog persistence fields if
  downstream local tooling starts consuming promotion JSON.
- Decide in a future strict-profile phase whether persisted stewardship should
  become mandatory for catalog-writing promotion.

## 13. Recommended Next Phase

Recommended next phase: archive metadata write planning.

Why:

- steward-review persistence is implemented and reviewed;
- promotion catalog record writes are implemented and accepted by this review;
- the next lifecycle sidecar is archive metadata, which should be planned before
  implementation because it touches draft lifecycle history and catalog
  referential integrity;
- runtime registration, repair, schemas, examples, provider calls, hosted
  behavior, and writes outside the local catalog remain deferred.

## 14. Validation Commands Run

Implementation validation before review:

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.

Review validation after this document:

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.

Governed review run:

- workflow: `dg/review`
- run: `run-1783535150272132000-2`
- approval: `approval/run-1783535150272132000-2/review-scope-approved`
- approval outcome: granted by delegated maintainer for promotion catalog
  write implementation review
- close summary: completed terminal run with 39 events, one approval, zero
  retries, and zero escalations
- event kinds: `RunCreated`, `RunValidated`, `RunStarted`, `StepScheduled`,
  `PolicyDecisionRecorded`, `ApprovalRequested`, `ApprovalGranted`,
  `RunResumed`, `SkillInvocationRequested`, `SkillInvocationStarted`,
  `SkillInvocationSucceeded`, and `RunCompleted`
- kernel posture: review scope approved before review work; review produced this
  document only; no implementation fixes, runtime registration, archive writes,
  schemas, examples, provider calls, hosted behavior, or release posture changes
  were introduced.
