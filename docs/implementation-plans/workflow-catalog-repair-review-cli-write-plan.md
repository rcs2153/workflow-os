# Workflow Catalog Repair Review CLI Write Plan

## 1. Executive Summary

The workflow catalog repair review store helper is implemented and reviewed.
`LocalWorkflowCatalogStore` can persist validated repair proposal review sidecars
under `repair-reviews/` only when a review still matches a fresh proposal
identity.

The next question is how an operator should explicitly persist one repair
review from the CLI without implying repair application. This plan defines that
future CLI write surface.

This plan does not implement the command. It does not implement repair apply
mode, automatic repair, catalog mutation, workflow rewrites, runtime
registration, schemas, examples, hosted behavior, provider calls, writes, or
release posture changes.

## 2. Goals

- Add a future explicit CLI path for persisting one repair proposal review.
- Require a fresh repair dry-run proposal as the source of truth.
- Require explicit operator review metadata and decision.
- Persist exactly one validated repair review sidecar.
- Reuse `WorkflowCatalogRepairProposalReview` constructors.
- Reuse `LocalWorkflowCatalogStore::write_repair_review_record_if_absent`.
- Reject stale, duplicate, invalid, unbounded, or secret-like review input.
- Preserve dry-run behavior as the default.
- Keep persisted review records separate from repair apply permission.
- Keep all errors stable and non-leaking.

## 3. Non-Goals

Do not implement in this phase or through this future CLI write slice:

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
- hosted/team catalog backend behavior;
- provider calls;
- local check or command execution;
- write-capable adapters;
- release posture changes.

## 4. Current Implemented Boundary

Implemented catalog repair surfaces are:

- `workflow-os author workflow catalog-repair --dry-run`;
- bounded repair proposal model/helper;
- bounded in-memory repair proposal review model/helper;
- stale review identity validation;
- local repair review store helper.

The implemented store helper can write, read, and list repair review sidecars.
It does not expose a CLI write command and does not apply repairs.

## 5. Proposed CLI Shape

Recommended command:

```text
workflow-os author workflow catalog-repair review \
  --dry-run \
  --proposal-id <proposal-id> \
  --decision <decision-kind> \
  --review-id <review-id> \
  --reviewer <actor> \
  --reason <bounded-reason> \
  --persist-review
```

Required flags:

- `--dry-run`;
- `--proposal-id`;
- `--decision`;
- `--review-id`;
- `--reviewer`;
- `--reason`;
- `--persist-review`.

The command should fail closed if `--persist-review` is omitted. A later preview
mode may print the review record that would be written, but the first
implementation should stay simple and require explicit persistence.

## 6. Proposal Source And Selection

The command must generate fresh repair proposals from the same bounded inputs
used by `catalog-repair --dry-run`.

Rules:

- load current project/catalog status through existing read-only paths;
- produce the fresh proposal set in memory;
- select exactly one proposal by `--proposal-id`;
- fail closed if zero or multiple proposals match;
- do not read raw workflow file contents beyond existing validation/status
  surfaces;
- do not load proposal records from stale persisted review data;
- do not fabricate proposals.

The fresh proposal identity is the only proposal identity that may be reviewed
and persisted.

## 7. Decision Vocabulary

The CLI should accept the existing review decision vocabulary only:

- `approved_for_future_apply_planning`;
- `rejected`;
- `deferred`;
- `requires_manual_catalog_review`;
- `requires_manual_workflow_review`;
- `requires_new_dry_run`.

The command name, help text, and output must make clear that even an approved
decision means only "approved for future apply planning." It is not apply
authorization.

## 8. Persistence Flow

The future implementation should perform this sequence:

1. Parse CLI inputs.
2. Validate catalog root/project inputs through existing safe path rules.
3. Recompute fresh dry-run repair proposals.
4. Select exactly one proposal by id.
5. Construct `WorkflowCatalogRepairProposalReview` from explicit operator
   input.
6. Validate the review against the selected fresh proposal.
7. Call `LocalWorkflowCatalogStore::write_repair_review_record_if_absent`.
8. Print a bounded success summary that includes stable ids only.

No partial review record should be emitted or persisted if any step fails.

## 9. Output Policy

Human output should be concise and bounded:

```text
repair_review_record_written: true
review_id: <review-id>
proposal_id: <proposal-id>
decision: <decision-kind>
storage: local_catalog_repair_review_sidecar
next_step: rerun catalog-repair --dry-run before any future apply planning
```

JSON output should use the same bounded fields. It must not include raw proposal
payloads, raw reviewer reasons, raw workflow YAML, command output, provider
payloads, parser payloads, CI logs, environment values, credentials, tokens, or
secret-like values.

## 10. Error Handling

Required stable error codes:

- `cli.workflow_catalog.repair_review.requires_dry_run`;
- `cli.workflow_catalog.repair_review.requires_persist_review`;
- `cli.workflow_catalog.repair_review.proposal_not_found`;
- `cli.workflow_catalog.repair_review.ambiguous_proposal`;
- `cli.workflow_catalog.repair_review.invalid_decision`;
- `cli.workflow_catalog.repair_review.invalid_review`;
- `cli.workflow_catalog.repair_review.stale_proposal`;
- `cli.workflow_catalog.repair_review.duplicate_review`;
- `cli.workflow_catalog.repair_review.persist_failed`.

Errors must not include raw proposal ids when invalid, raw review ids when
invalid, reviewer values, reason text, source references, file paths, snippets,
command output, provider payloads, parser payloads, or secret-like values.

Store errors from the helper may be mapped into CLI-specific codes when that
improves operator clarity, especially duplicate review and stale proposal
cases.

## 11. Privacy And Redaction

The CLI must use existing model constructors and store helpers. It must not
copy:

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
- token-like values;
- unbounded reviewer reasons;
- secret-like redaction metadata.

Debug output, JSON output, and deserialization/validation errors must remain
redaction-safe.

## 12. Relationship To Apply Mode

Persisted repair reviews are not apply permission.

Future apply planning must separately define:

- eligible repair action kinds;
- required approval/policy posture;
- stale proposal checks;
- record write ordering;
- partial failure handling;
- supersession/replacement behavior;
- audit/event/report projection;
- whether any workflow/catalog mutation can ever be automated.

This CLI write plan does not authorize apply mode.

## 13. Test Plan

Future implementation tests should cover:

- command requires `--dry-run`;
- command requires `--persist-review`;
- command rejects missing proposal id;
- command rejects unknown proposal id;
- command rejects ambiguous proposal selection if representable;
- command writes one repair review sidecar for a fresh selected proposal;
- command rejects duplicate review id without overwrite;
- command rejects stale proposal identity before persistence;
- command rejects secret-like review id, reviewer, reason, and redaction values;
- command output is bounded and does not copy raw proposal payloads;
- JSON output is bounded and deterministic;
- invalid catalog root fails closed without leaking paths;
- default `catalog-repair --dry-run` remains non-mutating;
- no workflow files are rewritten;
- no catalog records are created, updated, deleted, or overwritten;
- no runtime state, events, reports, or provider calls are created;
- existing catalog repair dry-run, proposal review, store helper, and CLI tests
  continue to pass.

## 14. Proposed Implementation Sequence

1. Add parser/help text for `workflow-os author workflow catalog-repair review`.
2. Wire proposal recomputation through the existing dry-run code path.
3. Select exactly one proposal by id.
4. Construct the repair review through existing model helper.
5. Persist through `LocalWorkflowCatalogStore`.
6. Add focused CLI tests and non-leakage tests.
7. Run full validation.
8. Create an implementation report and review before any apply planning.

## 15. Documentation Updates For Implementation

The implementation phase should update:

- `docs/implementation-plans/workflow-catalog-repair-review-persistence-plan.md`;
- `docs/implementation-plans/workflow-catalog-repair-review-cli-write-plan.md`;
- `ROADMAP.md`;
- CLI documentation if a CLI docs page exists for author workflow commands.

Docs must say:

- explicit CLI repair review write behavior is implemented;
- default repair dry-run remains non-mutating;
- persisted repair reviews are not apply permission;
- repair apply mode is not implemented;
- automatic repair is not implemented;
- workflow rewrites, runtime registration, schemas, examples, provider calls,
  hosted behavior, writes, and release posture changes remain deferred.

## 16. Open Questions

- Should the first implementation support a preview-only review rendering mode,
  or should it require `--persist-review` from the start?
- Should duplicate store races be mapped to
  `cli.workflow_catalog.repair_review.duplicate_review` even when the lower
  store helper returns a generic record-exists code?
- Should the CLI support optional citations to approval, policy, evidence,
  validation, or report references in the first write slice?
- Should persisted review records cite a catalog-status snapshot id before any
  apply planning begins?

## 17. Final Recommendation

Next implementation phase: CLI repair review write implementation.

The implementation should be narrow: one explicit command, one selected fresh
proposal, one validated review, one sidecar write, no apply behavior, and no
other mutation.
