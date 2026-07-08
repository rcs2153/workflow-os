# Workflow Catalog Repair Dry-Run CLI Review

## 1. Executive Verdict

Phase accepted; proceed to repair proposal review/approval planning.

The phase adds the intended read-only CLI surface:

```text
workflow-os author workflow catalog-repair --dry-run
```

The command exposes the reviewed non-mutating repair proposal helper through a
bounded human and preview JSON interface. It does not implement apply mode,
automatic repair, cleanup, deletion, overwrite, workflow registration,
workflow promotion, runtime state, provider calls, schemas, examples, hosted
behavior, write-capable adapters, or release posture changes.

## 2. Scope Verification

The phase stayed within the approved dry-run CLI scope.

No accidental implementation was found for:

- apply mode;
- automatic catalog repair;
- catalog cleanup;
- record deletion;
- record overwrite;
- active workflow rewrites;
- draft/archive movement;
- workflow runtime registration;
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

## 3. CLI Surface Assessment

The command shape is appropriate for this phase:

```text
workflow-os author workflow catalog-repair --dry-run
```

It requires `--dry-run` and fails usage when the flag is omitted. This keeps
repair semantics explicitly proposal-only and prevents users from inferring
that an apply path exists.

The command reuses the same inputs as `catalog-status`:

- loader-visible active workflows;
- inactive drafts;
- archived drafts;
- optional local catalog store records;
- optional `--catalog-root`;
- optional `--strict-catalog-coverage`;
- global `--json`.

This is the right boundary because repair proposal output stays tied to the
reviewed catalog index instead of inventing a second catalog conflict engine.

## 4. Non-Mutation Assessment

The CLI reports explicit non-mutation posture in both human and JSON output:

- `files_written: false`;
- `catalog_records_written: false`;
- `catalog_records_deleted: false`;
- `catalog_records_overwritten: false`;
- `workflow_registered: false`;
- `workflow_promoted: false`;
- `draft_archived: false`;
- `commands_executed: false`;
- `providers_called: false`;
- `runtime_state_created: false`.

Tests verify that the command does not create `.workflow-os/catalog` and does
not create runtime state. The implementation only loads catalog-status
context, calls `propose_workflow_catalog_repairs`, and renders output.

## 5. Proposal Output Assessment

Human output includes bounded proposal fields:

- proposal id;
- action kind;
- conflict kind;
- workflow id when available;
- source category;
- source reference;
- safe-for-future-apply flag;
- human-review-required flag;
- bounded summary.

Preview JSON mirrors the same bounded shape. The command uses proposal
accessors rather than exposing the raw conflict object, which preserves the
model boundary added in the helper phase.

The first supported action kind,
`create_missing_catalog_record`, is surfaced correctly for missing active
catalog records. The renderer also handles the broader action-kind vocabulary
already present in the core model, which keeps future proposal classifications
displayable without adding apply behavior.

## 6. Validation And Error Handling Assessment

The command:

- rejects missing `--dry-run` before proposal construction;
- reuses catalog-root validation from `catalog-status`;
- rejects unsafe catalog roots without echoing secret-like path fragments;
- maps proposal construction failure to the stable code
  `cli.workflow_catalog.repair_proposal_failed`;
- does not turn strict catalog coverage conflicts into command failure when
  the purpose is to review repair proposals.

This behavior is conservative and useful. `catalog-status
--strict-catalog-coverage` can still fail when a blocker exists; `catalog-repair
--dry-run --strict-catalog-coverage` can report the repair proposal for that
same blocker without mutation.

## 7. Privacy And Redaction Assessment

The output is bounded to identifiers, conflict/action codes, source categories,
repository-relative references, booleans, and bounded summaries.

No evidence was found that the command reads, copies, or emits:

- raw workflow YAML bodies;
- source file contents;
- command output;
- provider payloads;
- parser payloads;
- package script bodies;
- CI logs;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

Unsafe catalog-root failures are tested for non-leakage.

## 8. Test Quality Assessment

The focused CLI tests cover:

- dry-run proposal output without mutation;
- preview JSON proposal output;
- required `--dry-run` usage failure;
- strict catalog coverage proposal output without status failure;
- unsafe catalog-root rejection without leakage;
- no catalog-root creation;
- no runtime-state creation;
- stable id/action/conflict/workflow fields in output;
- existing catalog-status tests remain in place.

The tests are behavior-oriented and protect the safety boundary. Full
workspace validation also passed.

## 9. Documentation Review

Documentation now states:

- the repair proposal helper is implemented and reviewed;
- the dry-run CLI surface is implemented;
- the dry-run CLI is proposal-only;
- automatic repair is not implemented;
- apply mode is not implemented;
- deletion, overwrite, cleanup, workflow registration, schemas, examples,
  hosted behavior, providers, writes, and release posture changes remain
  unimplemented.

One non-blocking wording issue remains in the repair/recovery plan: after the
implemented-command note, the historical sentence "or an equivalent internal
helper plus a read-only CLI surface" is still present. It is not a dangerous
false claim, but it can be cleaned up in a later documentation polish pass.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Clean up the historical wording in the repair/recovery plan now that the CLI
  command exists.
- Add a focused maintainer-facing example snippet in the CLI/user-guide layer
  after the review is accepted.
- Consider a durable repair proposal review/approval record before any apply
  mode is designed.
- Keep future apply semantics behind separate planning, implementation, and
  review.

## 12. Recommended Next Phase

Recommended next phase: repair proposal review/approval planning.

Why: the dry-run proposal surface is useful, but the next safety boundary is
not applying repairs. It is deciding how a maintainer reviews, approves,
rejects, records, and cites repair proposals before any future local apply
mode exists.

## 13. Validation

Implementation-phase validation already completed before merge:

```text
cargo fmt --all
cargo test -p workflow-cli --test cli author_workflow_catalog_repair
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
```

Result: passed.

Review-phase validation:

```text
npm run check:docs
```

Result: passed.

## 14. Governed Phase Metadata

- dogfood workflow id: `dg/review`
- run id: `run-1783544229127104000-2`
- approval id:
  `approval/run-1783544229127104000-2/review-scope-approved`
- approval outcome: granted by delegated maintainer
- event summary: 39 events; 1 approval; 0 retries; 0 escalations
