# Workflow Catalog Repair Review CLI Write Implementation Report

## 1. Executive Summary

Workflow OS now has an explicit CLI path for persisting one workflow catalog
repair proposal review sidecar.

`workflow-os author workflow catalog-repair review --dry-run --persist-review`
recomputes fresh repair proposals, selects exactly one proposal by id, builds a
bounded `WorkflowCatalogRepairProposalReview`, validates it against the fresh
proposal identity through the accepted store helper, and writes exactly one
local `repair-reviews/` sidecar.

The command does not apply repairs, mutate workflow files, write catalog records,
delete or overwrite records, register workflows, create runtime state, append
events, call providers, execute local checks, change schemas, update examples,
enable writes, or change release posture.

## 2. Scope Completed

- Added `workflow-os author workflow catalog-repair review`.
- Required `--dry-run`.
- Required `--persist-review`.
- Required explicit proposal id, review id, decision, reviewer, and reason.
- Recomputed fresh repair proposals from existing catalog-status inputs.
- Selected exactly one proposal by id.
- Constructed reviews through the existing
  `WorkflowCatalogRepairProposalReview` helper.
- Persisted through
  `LocalWorkflowCatalogStore::write_repair_review_record_if_absent`.
- Mapped duplicate and stale helper failures to CLI-level stable error codes.
- Added bounded human and JSON output.
- Added CLI help text.
- Added focused CLI tests.
- Updated roadmap and planning docs.

## 3. Scope Explicitly Not Completed

- No repair apply mode.
- No automatic catalog repair.
- No catalog record creation, update, deletion, overwrite, or cleanup.
- No active workflow rewrites.
- No draft or archive movement.
- No runtime workflow registration.
- No runtime state creation.
- No event or audit append.
- No report artifact generation.
- No workflow schema changes.
- No examples.
- No hosted/team catalog backend behavior.
- No provider calls.
- No local check or command execution.
- No write-capable adapter behavior.
- No release posture changes.

## 4. CLI API Summary

Implemented command:

```text
workflow-os author workflow catalog-repair review \
  --dry-run \
  --proposal-id <proposal-id> \
  --review-id <review-id> \
  --decision <decision-kind> \
  --reviewer <actor> \
  --reason <bounded-reason> \
  --persist-review \
  [--catalog-root .workflow-os/catalog] \
  [--strict-catalog-coverage]
```

Supported decision labels:

- `approved-for-future-apply-planning`;
- `rejected`;
- `deferred`;
- `requires-manual-catalog-review`;
- `requires-manual-workflow-review`;
- `requires-new-dry-run`.

The command accepts underscore variants for the decision vocabulary for parity
with serialized model labels.

## 5. Fresh Proposal Behavior

The command uses the same status context and proposal builder as
`catalog-repair --dry-run`. It does not read persisted review records to derive
proposal identity and does not fabricate proposal ids.

If the selected proposal id is not present in the fresh proposal set, the command
fails closed with
`cli.workflow_catalog.repair_review.proposal_not_found` before writing any
sidecar.

## 6. Persistence Behavior

Persistence is explicit and opt-in:

- omitting `--dry-run` fails with usage;
- omitting `--persist-review` fails with usage;
- successful execution writes one repair review sidecar under
  `.workflow-os/catalog/repair-reviews/` or the caller-supplied safe catalog
  root;
- duplicate review ids fail closed with
  `cli.workflow_catalog.repair_review.duplicate_review`;
- stale proposal identity failures map to
  `cli.workflow_catalog.repair_review.stale_proposal`.

No other catalog sidecar directory is created by the successful repair review
write path.

## 7. Output Summary

Human output includes bounded fields only:

- mode;
- status;
- review id;
- proposal id;
- decision label;
- storage posture;
- catalog root display;
- explicit non-mutation booleans;
- privacy boundary;
- next action.

JSON output mirrors those bounded fields. Neither output copies the reviewer
reason.

## 8. Privacy And Redaction Summary

The command does not copy raw workflow YAML, raw catalog payloads, source
contents, command output, provider payloads, parser payloads, CI logs,
environment values, credentials, authorization headers, private keys, tokens, or
secret-like values into CLI output.

Secret-like review ids, proposal ids, reviewers, reasons, and unsafe catalog
roots fail closed without echoing raw values. Persisted review records may store
bounded reviewer reasons that passed model validation, matching the reviewed
store-helper boundary.

## 9. Test Coverage Summary

Focused tests cover:

- successful persisted repair review sidecar write;
- bounded human output;
- bounded JSON output;
- required `--dry-run`;
- required `--persist-review`;
- unknown proposal rejection without writes;
- duplicate review id rejection without overwrite;
- secret-like reason rejection without leakage;
- default dry-run command remains non-mutating through existing tests;
- no runtime state creation.

Existing catalog repair dry-run, steward-review persistence, catalog store, and
full workspace tests continue to pass.

## 10. Commands Run And Results

```text
cargo test -p workflow-cli --test cli author_workflow_catalog_repair_review
```

Result: passed.

Full validation commands:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
```

Result: passed.

Governed phase metadata:

- dogfood workflow id: `dg/implement`
- run id: `run-1783552996108101000-2`
- approval id:
  `approval/run-1783552996108101000-2/implementation-approved`
- approval outcome: granted by delegated maintainer
- out-of-kernel work: repository edits, Rust/doc validation, git, PR, and merge
  operations are performed by Codex/human execution layer; the kernel
  coordinated governance only.

## 11. Remaining Known Limitations

- Persisted repair reviews are not repair apply permission.
- Optional approval, policy, evidence, validation, and work-report citations are
  not exposed on the CLI yet.
- Review supersession, replacement, deletion, and cleanup semantics remain
  unimplemented.
- Persisted reviews do not cite catalog-status snapshot ids yet.
- Hosted/team catalog persistence remains future work.

## 12. Recommended Next Phase

Recommended next phase: workflow catalog repair review CLI write implementation
review.

The review should verify the command stays non-apply, persists exactly one
validated sidecar, rejects stale/duplicate/invalid input safely, preserves
bounded output, and keeps all repair mutation behavior deferred.
