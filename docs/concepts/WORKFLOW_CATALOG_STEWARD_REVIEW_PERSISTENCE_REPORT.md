# Workflow Catalog Steward Review Persistence Report

## 1. Executive Summary

The first opt-in workflow catalog stewardship persistence slice is implemented.
`workflow-os author workflow steward-review` remains preview-only by default.
When `--persist-stewardship` is supplied, the command writes one validated local
catalog stewardship record for the explicit steward-review decision.

This gives later promotion and archive phases a durable stewardship decision id
to cite without moving workflow files, registering workflows, creating runtime
state, calling providers, or changing release posture.

## 2. Scope Completed

- Added explicit `--persist-stewardship` support to `author workflow
  steward-review`.
- Added optional `--catalog-root` support for the persistence path only.
- Wrote validated `WorkflowStewardshipRecord` values through
  `LocalWorkflowCatalogStore`.
- Preserved default steward-review preview behavior as non-mutating.
- Added bounded output disclosures for workflow file writes, catalog record
  writes, approval persistence, stewardship persistence, runtime state,
  commands, providers, workflow registration, and promotion.
- Added focused CLI regression tests.
- Updated CLI documentation, persistence planning, and roadmap status.

## 3. Scope Explicitly Not Completed

- No promotion catalog record writes.
- No archive metadata writes.
- No workflow runtime registration.
- No catalog repair.
- No automatic workflow generation.
- No draft deletion.
- No runtime state creation.
- No command execution.
- No provider calls.
- No hosted collaboration.
- No schema changes.
- No example updates.
- No write-capable adapters.
- No release posture changes.

## 4. CLI/API Summary

Default preview remains:

```sh
workflow-os author workflow steward-review \
  --draft workflows/drafts/<name>.workflow.yml \
  --decision approved-for-promotion \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason>
```

Opt-in persistence is explicit:

```sh
workflow-os author workflow steward-review \
  --draft workflows/drafts/<name>.workflow.yml \
  --decision approved-for-promotion \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason> \
  --persist-stewardship \
  [--catalog-root .workflow-os/catalog]
```

`--catalog-root` is rejected unless `--persist-stewardship` is also present.

## 5. Persistence Boundary

The persistence path writes exactly one local catalog stewardship record when
the review input is valid, preflight passes, and the decision record does not
already exist.

It does not write workflow files, promote drafts, register workflows, persist
approval records, append runtime events, create state backend entries, run
commands, or call providers.

## 6. Failure And Atomicity Summary

Invalid reviewer, unsafe reason, failed preflight, unsafe catalog root, invalid
record construction, and duplicate stewardship record writes fail closed with
stable bounded error codes.

Duplicate persistence does not overwrite the existing record. The implementation
uses the catalog store's write-if-absent boundary so a retry cannot silently
replace stewardship history.

## 7. Privacy And Redaction Summary

The persisted record stores bounded identifiers, content hashes, actor id,
decision kind, draft path, and bounded reason summary. It does not store raw
workflow YAML bodies, source contents, command output, provider payloads, parser
payloads, environment values, credentials, or token-like values.

CLI output does not echo the review reason or draft body. Errors are bounded and
do not echo unsafe catalog roots or secret-like values.

## 8. Test Coverage Summary

Focused CLI tests cover:

- default steward-review remains preview-only and does not create a catalog;
- explicit `--persist-stewardship` writes one stewardship record;
- persisted JSON output discloses bounded persistence posture;
- `--catalog-root` requires `--persist-stewardship`;
- unsafe catalog roots are rejected without leakage;
- duplicate persistence fails closed without overwriting;
- existing steward-review preview and JSON tests still pass.

## 9. Commands Run And Results

- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.

Governed dogfood run:

- workflow id: `dg/implement`;
- run id: `run-1783529455279356000-2`;
- approval id: `approval/run-1783529455279356000-2/implementation-approved`;
- approval outcome: granted;
- final status: `Completed`;
- event summary: 39 events, including 1 approval request, 1 approval grant, 8
  policy decisions, 6 skill invocation requests, 6 skill invocation starts, 6
  skill invocation successes, and 1 run completion.

Out-of-kernel work disclosed: repository edits, shell validation commands, and
documentation updates were performed by the maintainer/agent outside the kernel
execution layer. The kernel governed the phase boundary and approval checkpoint.

## 10. Remaining Known Limitations

- Promotion does not yet require or cite a persisted stewardship decision.
- Promotion catalog records are not written.
- Archive metadata records are not written by archive commands.
- Catalog status is not enforced by promotion/archive commands.
- No workflow-declared catalog policy exists.
- No hosted or collaborative catalog backend exists.

## 11. Recommended Next Phase

Recommended next phase: steward-review persistence review.

The review should verify that persistence remains opt-in, default preview is
unchanged, exactly one validated stewardship record is written, duplicate writes
fail closed, and no runtime, provider, schema, example, or release-posture scope
was introduced.
