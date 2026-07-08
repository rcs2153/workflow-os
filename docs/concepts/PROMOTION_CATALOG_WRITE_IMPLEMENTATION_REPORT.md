# Promotion Catalog Write Implementation Report

## 1. Executive Summary

The opt-in promotion catalog record write phase is implemented.

`workflow-os author workflow promote` keeps its default behavior unchanged: it
promotes one reviewed inactive draft into `workflows/`, preserves the draft, and
writes no catalog record unless explicitly requested.

The new `--persist-catalog-record` flag adds one local sidecar write after
active promotion validation. When supplied, the command writes one validated
`WorkflowCatalogRecord` through the existing local catalog store. It may also
cite a persisted stewardship decision when `--stewardship-decision-id` is
provided and verified.

This phase does not add runtime workflow registration, archive metadata writes,
catalog repair, automatic workflow generation, draft deletion, schemas,
examples, hosted behavior, provider calls, write-capable adapters, or release
posture changes.

## 2. Scope Completed

- Added explicit opt-in promotion catalog record persistence through
  `workflow-os author workflow promote --persist-catalog-record`.
- Added `--catalog-root` support for promotion catalog persistence, accepted
  only with `--persist-catalog-record`.
- Added `--stewardship-decision-id` support for promotion catalog persistence,
  accepted only with `--persist-catalog-record`.
- Verified supplied stewardship records before active workflow file mutation.
- Constructed catalog records through the existing `WorkflowCatalogRecord`
  model constructor.
- Wrote catalog records through `LocalWorkflowCatalogStore` with
  write-if-absent semantics.
- Preserved default promotion behavior when `--persist-catalog-record` is not
  supplied.
- Extended text and JSON output with bounded catalog persistence fields.
- Added focused CLI tests for success, default non-write behavior, invalid flag
  combinations, and stale stewardship rejection.
- Updated CLI and roadmap documentation.

## 3. Scope Explicitly Not Completed

- No runtime workflow registration.
- No automatic workflow generation or promotion.
- No archive metadata writes.
- No catalog repair.
- No draft deletion.
- No runtime state creation.
- No workflow run creation.
- No command execution or local check execution.
- No provider calls.
- No report artifacts.
- No workflow schema changes.
- No examples.
- No hosted or distributed catalog behavior.
- No RBAC, IdP integration, notifications, or admin UI.
- No write-capable adapters or provider mutation.
- No release posture changes.

## 4. CLI/API Summary

Promotion still supports the existing active workflow file mutation:

```sh
workflow-os author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason>
```

Catalog persistence is explicit:

```sh
workflow-os author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason> \
  --persist-catalog-record \
  --stewardship-decision-id stewardship/<id>
```

`--catalog-root` and `--stewardship-decision-id` fail closed when supplied
without `--persist-catalog-record`.

Dry-run accepts the same flags for preview but writes neither the active file
nor the catalog record.

## 5. Stewardship Citation Behavior

When `--stewardship-decision-id` is supplied, the command reads the selected
local catalog stewardship record before active workflow mutation and verifies:

- the record id matches the supplied decision id;
- the decision kind is `ApprovedForPromotion`;
- the workflow id matches the draft candidate;
- the draft path matches the current promoted draft;
- the candidate content hash matches the current draft content hash.

Missing, invalid, stale, non-approving, corrupt, or mismatched stewardship
records fail closed before the active workflow file is written.

When no stewardship decision id is supplied, the command still allows the
catalog record write and discloses that no persisted stewardship decision was
cited. Strict persisted-stewardship requirements remain deferred to a later
profile/config phase.

## 6. Catalog Record Write Behavior

The implementation constructs a deterministic catalog record id from the
workflow id and writes one workflow catalog record under
`.workflow-os/catalog/workflows/` by default.

The record includes bounded references and summaries:

- workflow id;
- active workflow path;
- draft source path;
- workflow content hash;
- schema version;
- active lifecycle status;
- owner and escalation references when present in the workflow spec;
- authority scope summary;
- evidence/check/report posture summary;
- side-effect posture summary;
- verified stewardship/promotion decision id when supplied;
- conservative sensitivity and explicit redaction metadata.

The catalog record does not store raw workflow YAML, raw source contents,
command output, provider payloads, parser payloads, package script bodies,
environment values, credentials, or token-like values.

## 7. Failure And Atomicity Behavior

Invalid catalog inputs fail before active workflow mutation.

After active file promotion and post-write project validation, catalog record
write failure returns a stable partial-integration error:

- the active workflow promotion already succeeded;
- the catalog record was not written;
- no runtime state was created;
- no automatic rollback was attempted.

Automatic rollback and catalog repair remain deferred.

## 8. Privacy And Redaction

Errors use stable codes and avoid raw paths, raw YAML, review reason text,
serialized record payloads, parser payloads, command output, provider payloads,
and secret-like values.

Output reports bounded status fields only, including:

- whether catalog persistence was requested;
- whether a catalog record was written;
- the catalog record id;
- the catalog root;
- the stewardship decision id when supplied;
- whether the stewardship decision was verified.

## 9. Test Coverage Summary

Added focused CLI coverage for:

- successful opt-in catalog record write after persisted stewardship review;
- default promotion remaining unchanged without catalog record writes;
- `--catalog-root` rejection without `--persist-catalog-record`;
- stale stewardship decision rejection before active workflow and catalog writes;
- catalog output non-leakage for bounded review reasons;
- continued active promotion behavior and validation.

Focused validation run:

```sh
cargo test -p workflow-cli --test cli author_workflow_promote -- --nocapture
```

Result: 10 passed.

## 10. Dogfood Governance Summary

Governed dogfood phase:

- workflow id: `dg/implement`
- run id: `run-1783533152480548000-2`
- approval id:
  `approval/run-1783533152480548000-2/implementation-approved`
- approval outcome: granted
- approval actor: `user/delegated-maintainer`
- approval reason: `approved-promotion-catalog-write-implementation`
- terminal status: Completed
- event summary:
  `ApprovalGranted:1,ApprovalRequested:1,PolicyDecisionRecorded:8,RunCompleted:1,RunCreated:1,RunResumed:1,RunStarted:1,RunValidated:1,SkillInvocationRequested:6,SkillInvocationStarted:6,SkillInvocationSucceeded:6,StepScheduled:6`

The phase was executed as a governed implementation phase with explicit scope,
strict non-goals, expected touched surfaces, and validation requirements.
Repository edits, shell validation commands, documentation updates, and this
report were performed by the executor outside the kernel and are disclosed here
as out-of-kernel work.

## 11. Commands Run And Results

- `cargo fmt --all`: passed.
- `cargo test -p workflow-cli --test cli author_workflow_promote -- --nocapture`:
  passed, 10 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.

## 12. Remaining Known Limitations

- Catalog record updates are not implemented; duplicate record writes fail
  closed through the store.
- Archive metadata writes are not implemented.
- Catalog repair and rollback are not implemented.
- Strict mode requiring persisted stewardship before catalog writes is deferred.
- Runtime workflow registration remains file-loader based; the catalog is not a
  runtime registry.
- No workflow schema fields expose catalog behavior.
- No hosted or team catalog backend exists.

## 13. Recommended Next Phase

Recommended next phase: promotion catalog write implementation review.

The implementation is small but crosses a new persistence boundary. It should
be reviewed before archive metadata writes, catalog repair, or stricter catalog
enforcement are considered.
