# Workflow Catalog Repair Review Persistence Plan

## 1. Executive Summary

The in-memory workflow catalog repair proposal review helper is implemented and
accepted. It records bounded maintainer decisions against non-mutating repair
proposals and can detect stale reuse against a fresh proposal identity.

The first local store-helper slice is now implemented. `LocalWorkflowCatalogStore`
can persist validated repair review sidecars under `repair-reviews/`, read them
back, list them deterministically, reject duplicates, and refuse stale proposal
identity before persistence.

This plan defines the conservative local persistence boundary that the
implementation follows. It does not implement CLI write behavior, repair apply
mode, automatic repair, catalog mutation, workflow rewrites, schemas, examples,
provider calls, hosted behavior, writes, or release posture changes.

## 2. Goals

- Persist repair proposal review records only through an explicit opt-in future
  surface.
- Preserve active workflow files as the execution source of truth.
- Preserve repair proposals as dry-run, review-required recommendations.
- Preserve review records as local governance sidecars, not mutation authority.
- Support future apply planning by requiring fresh matching proposal identity.
- Keep persisted records deterministic, bounded, redaction-safe, and
  repository-relative.
- Refuse duplicate or stale review records deterministically.
- Avoid raw workflow YAML, raw catalog payloads, command output, provider
  payloads, source contents, parser payloads, CI logs, environment values, and
  secrets.

## 3. Non-Goals

Do not implement through this persistence boundary:

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

## 4. Current Boundary

Implemented repair-review surfaces are:

- `catalog-status`: reports catalog inventory and conflicts.
- `catalog-repair --dry-run`: exposes bounded repair proposals without
  mutation.
- `review_workflow_catalog_repair_proposal`: constructs an in-memory bounded
  maintainer review record for one typed proposal.
- `validate_workflow_catalog_repair_proposal_review_matches`: verifies that a
  review still matches a fresh proposal identity.

None of these surfaces persist review records, apply repairs, write catalog
records, mutate workflow files, register runtime workflows, append events, or
call providers.

The core store helper now adds persistence primitives only:

- `write_repair_review_record_if_absent`;
- `read_repair_review_record`;
- `list_repair_review_records`.

Those helpers are not wired to a CLI review write command and do not apply
repairs.

## 5. Recommended Storage Layout

Persisted repair review records should live under the existing local catalog
root, separate from workflow catalog, stewardship, and archive records:

```text
.workflow-os/catalog/
  workflows/
  stewardship/
  archives/
  repair-reviews/
    <safe-repair-review-id>.json
```

Rationale:

- review records are catalog lifecycle metadata, not runtime state;
- a separate directory avoids mixing repair-review decisions with authoring
  stewardship decisions;
- file-per-record storage matches the existing catalog-store posture;
- JSON can reuse the existing serde model while schema exposure remains
  deferred;
- future team/hosted backends can replace the local store boundary.

The first persistence implementation should not silently create this directory
except through an explicit review-recording command or helper call.

## 6. Record Identity And File Naming

The storage address should be derived from
`WorkflowCatalogRepairProposalReviewId` using the existing safe file-name
encoding policy.

Rules:

- the canonical review id remains inside the record;
- the file name is a storage address only;
- path traversal, absolute paths, unsafe prefixes, unbounded ids, and
  secret-like ids must fail closed;
- duplicate file names must be rejected unless a separately planned replace
  mode exists;
- raw rejected ids must not appear in errors.

## 7. Write Timing Policy

The first future implementation should write a repair review record only after:

1. a fresh `catalog-repair --dry-run` proposal exists;
2. the caller supplies an explicit review id, reviewer, reason, and decision;
3. the in-memory review helper validates the record;
4. the review matches the fresh proposal identity;
5. the target store path is safe;
6. no duplicate review record already exists;
7. the caller explicitly opts into persistence.

Dry-run and preview paths must never write repair review records.

## 8. Duplicate And Replacement Policy

The first persistence implementation should reject duplicate repair review ids.

It should not implement update, overwrite, replace, or delete behavior. If a
maintainer needs to change a decision, they should create a new review id and
cite the superseded review only after supersession semantics are separately
planned.

This keeps the first persistence slice append-oriented and easier to audit.

## 9. Stale Review Policy

Persisted review records are point-in-time decisions. They must not be reused
blindly.

Any future apply planner or CLI write surface must require:

- a fresh dry-run proposal;
- a persisted review record;
- matching proposal identity between the review and fresh proposal;
- a decision kind that allows future apply planning;
- policy and mutation checks designed in the future apply phase.

If the fresh proposal no longer matches, the persisted review must be treated
as stale and require re-review.

## 10. CLI Surface Recommendation

If a CLI surface is later implemented, it should remain explicit and opt-in.

Candidate shape:

```text
workflow-os author workflow catalog-repair review \
  --proposal-id <id> \
  --decision <approved-for-future-apply-planning|rejected|deferred|requires-manual-catalog-review|requires-manual-workflow-review|requires-new-dry-run> \
  --reviewer <actor> \
  --reason <bounded-reason> \
  --persist-review
```

Recommended first implementation posture:

- require an explicit `--persist-review` flag;
- load fresh dry-run proposals from current catalog-status inputs;
- select exactly one proposal by id;
- construct the review through the existing helper;
- validate the review against the selected fresh proposal;
- write exactly one review sidecar;
- refuse duplicate review ids;
- create no workflow files, catalog records, runtime state, or events.

The CLI must not be named or described as repair application.

## 11. Relationship To Apply Mode

Persisted review records are not apply permission by themselves.

Future apply planning must still define:

- eligible proposal action kinds;
- policy requirements;
- approval requirements;
- stale-review checks;
- record-write ordering;
- rollback or partial-failure posture;
- deletion and overwrite posture, if any;
- audit/event/report projection.

This plan does not authorize apply mode.

## 12. Privacy And Redaction

Persisted repair review records should be treated as sensitive local governance
metadata.

They may contain bounded reviewer reasons and repository-relative source
references. Those are allowed only after validation, but they can still reveal
repository posture and should not be treated as public telemetry by default.

Rules:

- use existing constructors and redaction validation;
- do not copy raw workflow YAML;
- do not copy raw catalog record payloads;
- do not copy source contents;
- do not copy command output;
- do not copy provider payloads;
- do not copy parser payloads;
- do not copy CI logs;
- do not copy environment values;
- do not copy credentials, authorization headers, private keys, tokens, or
  secret-like values;
- Debug and error output must remain non-leaking;
- serialization and deserialization must fail closed on invalid or secret-like
  values.

## 13. Error Handling

Errors must use stable, non-leaking codes.

Candidate codes:

- `workflow_catalog.repair_review_store.invalid_root`;
- `workflow_catalog.repair_review_store.invalid_review_id`;
- `workflow_catalog.repair_review_store.duplicate_review`;
- `workflow_catalog.repair_review_store.write_failed`;
- `workflow_catalog.repair_review_store.read_failed`;
- `workflow_catalog.repair_review_store.invalid_record`;
- `workflow_catalog.repair_review_store.stale_proposal`;
- `cli.workflow_catalog.repair_review.proposal_not_found`;
- `cli.workflow_catalog.repair_review.persist_failed`.

Errors must not include raw paths, raw reviewer reasons, raw source references,
raw record contents, snippets, command output, provider payloads, or secret-like
values.

## 14. Test Plan

Future implementation tests should cover:

- valid repair review record persists under `repair-reviews/`;
- persisted record round trips through validated serde;
- duplicate review id is rejected;
- unsafe catalog root is rejected without leakage;
- unsafe or secret-like review id is rejected without leakage;
- secret-like reviewer reason is rejected without leakage;
- stale proposal identity is rejected before persistence;
- persisted review records do not create workflow catalog records;
- persisted review records do not mutate active workflow files;
- dry-run review path writes nothing;
- no runtime state or events are created;
- Debug output does not leak reason or source reference;
- serialization/deserialization fail closed on invalid metadata;
- existing catalog store, catalog status, repair proposal, and repair dry-run
  tests still pass;
- `cargo test --workspace` passes.

## 15. Proposed Implementation Sequence

Recommended future sequence:

1. Add local repair review store helper under the catalog store boundary.
   Implemented.
2. Add focused store tests for write/read/list/duplicate/error behavior.
   Implemented.
3. Review the store helper.
4. Add an explicit CLI review-recording surface with `--persist-review`.
5. Review the CLI surface before any apply-mode planning.
6. Plan apply mode only after persisted review records are accepted.

The completed implementation starts with the store helper only, not the CLI.

## 16. Documentation Updates For Future Implementation

Future implementation reports must state clearly:

- repair review persistence is implemented at the core store-helper boundary;
- CLI review write behavior is planned separately;
- repair apply mode remains unimplemented;
- automatic repair remains unimplemented;
- deletion, overwrite, cleanup, runtime registration, schemas, examples,
  providers, hosted behavior, writes, and release posture changes remain
  deferred.

## 17. Open Questions

- Should a persisted review record be allowed to cite a previous superseded
  review, or should supersession wait for a later model?
- Should repair review records be committed to Git by default, ignored by
  default, or left to project policy?
- Should the store helper expose deterministic list ordering by reviewed time,
  review id, or proposal id?
- Should persisted repair review records cite a catalog-status snapshot id once
  such a snapshot model exists?
- Should review persistence require an approval reference for
  `ApprovedForFutureApplyPlanning`, or should local delegated-maintainer
  contexts remain sufficient in v0?

## 18. Final Recommendation

The local repair review store helper has been implemented and accepted. The next
implementation phase should be the explicit CLI repair review write surface
described in
[Workflow Catalog Repair Review CLI Write Plan](workflow-catalog-repair-review-cli-write-plan.md).

Do not build repair apply mode, automatic repair, catalog mutation, deletion,
overwrite, workflow rewrites, runtime registration, schemas, examples,
providers, hosted behavior, writes, or release posture changes in the CLI write
phase.
