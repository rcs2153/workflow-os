# Governed Workflow Authoring Active Promotion Implementation Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The active promotion implementation delivers the first explicit local authoring
mutation boundary for governed workflow drafts. `workflow-os author workflow
promote` promotes one preflight-passing inactive draft into the active
`workflows/` surface only after fresh preflight, active-context validation, and
same-process steward/delegated-maintainer approval input.

The implementation stays narrow. It does not start runs, create runtime state,
persist approvals, execute commands, call providers, write report artifacts,
change schemas, update examples, enable external writes, or change release
posture.

## 2. Scope Verification

The phase stayed within the approved first active-promotion scope.

Implemented:

- one explicit CLI command, `workflow-os author workflow promote`;
- required `--draft`, `--reviewer`, and `--reason` inputs;
- optional `--dry-run`;
- fresh project validation and draft preflight;
- active-placement validation before writing;
- same-process steward-review authorization;
- overwrite refusal for the active output path;
- one active workflow file write under `workflows/`;
- draft preservation under `workflows/drafts/`;
- post-write project validation;
- bounded text and JSON output;
- focused CLI tests and documentation.

Not implemented:

- automatic promotion;
- multi-draft promotion;
- persisted steward approval records;
- workflow catalog persistence;
- runtime state creation;
- workflow run creation;
- command execution;
- local check execution;
- provider calls;
- report artifact creation;
- workflow-declared steward configuration;
- schemas;
- examples;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters or provider mutation;
- release posture changes.

No accidental out-of-scope behavior was found.

## 3. CLI Boundary Assessment

The CLI shape is clear and consistent with the plan:

```sh
workflow-os author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason>
```

The dry-run shape is also supported:

```sh
workflow-os author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason> \
  --dry-run
```

The command is explicit. Passing preflight or steward-review preview output does
not implicitly promote a workflow.

## 4. Promotion Semantics Assessment

Promotion is correctly implemented as repository authoring, not runtime
execution.

The implementation:

- derives the active path from the draft filename;
- writes to `workflows/<same-file-name>`;
- preserves the draft file;
- makes the promoted workflow visible to the existing loader;
- requires a subsequent runtime invocation before any workflow can run.

Promotion does not imply:

- a workflow run started;
- checks passed;
- providers were called;
- report artifacts exist;
- external writes are authorized;
- future draft or active workflow edits are approved.

## 5. Validation Assessment

Validation is appropriately fail-closed.

The command validates:

- current Workflow OS project load and validation;
- relative draft path under `workflows/drafts/`;
- draft file existence;
- draft parseability;
- draft content hash;
- promotion preflight blockers and warnings;
- active workflow id conflicts;
- active output path overwrite;
- active-placement candidate validation before writing;
- reviewer id;
- bounded non-secret approval reason;
- steward-review authorization;
- post-write project validation.

Errors use stable codes and do not echo raw YAML, approval reason text, parser
payloads, command output, provider payloads, environment values, credentials, or
token-like values.

## 6. Write Boundary Assessment

The write boundary is suitably narrow.

The command writes at most one active workflow file and only when `--dry-run` is
absent. It refuses active output overwrite before writing and repeats the
overwrite check at the write boundary. It writes via a temporary sibling file
and rename, then reloads project validation.

The command does not mutate `.workflow-os/state`, append workflow events, create
report artifacts, call providers, or execute local checks.

One non-blocking limitation remains: if post-write validation failed after the
active file was written, the command would report the error but does not perform
automatic rollback. Because active-placement validation runs before writing,
this should be exceptional. A later recovery/rollback policy can be planned if
needed.

## 7. Steward Review Assessment

The implementation correctly reuses the existing
`review_workflow_draft_for_promotion` helper in the same process after fresh
preflight.

This is appropriate for the current phase because persisted approval records do
not exist yet. The output is honest that approval is not persisted.

The implementation does not falsely claim enterprise steward configuration,
RBAC, IdP, quorum approval, notifications, or durable approval consumption.

## 8. Privacy And Redaction Assessment

The privacy boundary is clean.

Output includes bounded values:

- relative draft path;
- relative active workflow path;
- candidate workflow id;
- draft content hash;
- preflight status;
- warning and blocker codes;
- reviewer id;
- validation status;
- explicit non-runtime boundary flags.

Output does not copy:

- raw draft YAML;
- approval reason text;
- raw source contents;
- package script bodies;
- dependency values;
- lockfile contents;
- CI logs;
- provider payloads;
- parser payloads;
- environment values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

## 9. Test Quality Assessment

The focused CLI tests cover the main behavior and risk boundaries:

- dry-run validates without writing;
- successful promotion writes one active file;
- promoted project validates;
- inactive draft is preserved;
- bounded JSON output;
- active output overwrite rejection before writing;
- blocked preflight rejection before writing;
- secret-like reason rejection without leakage;
- no runtime state creation;
- no command/provider/report artifact behavior.

Existing authoring tests continue to cover dry-run proposal, inactive draft
output, preflight, steward-review preview, and redaction behavior.

Non-blocking test follow-ups:

- Add a focused test for active-context validation failure before writing if a
  compact fixture can be constructed without overfitting to validator internals.
- Add a focused test for missing `--reason`/`--reviewer` once CLI usage tests
  for authoring subcommands are expanded.
- Add a recovery-focused test if a future rollback/recovery behavior is
  designed for exceptional post-write validation failure.

## 10. Documentation Review

Documentation is accurate.

The roadmap, CLI docs, active promotion plan, authoring plan, and implementation
report state that:

- active promotion is implemented for one explicit local CLI file mutation;
- dry-run is supported;
- the inactive draft is preserved;
- the active output path refuses overwrite;
- persisted approvals are not implemented;
- automatic promotion is not implemented;
- runtime state, runs, commands, providers, artifacts, schemas, examples,
  hosted behavior, writes, and release posture changes are not implemented.

## 11. Governed Dogfood Review Summary

- Workflow: `dg/review`.
- Run ID: `run-1783477805220674000-2`.
- Approval ID:
  `approval/run-1783477805220674000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer after the full approval
  handoff was emitted.
- Approved scope: inspect active promotion implementation code, tests, CLI docs,
  roadmap status, implementation report, and validation posture; create bounded
  review documentation only.
- Strict non-goals: no implementation fixes, active promotion behavior changes,
  runtime execution changes, schemas, examples, provider calls, writes beyond
  review documentation, or release posture changes.

## 12. Validation Commands Reviewed

The implementation report records these commands as passed:

- `cargo test -p workflow-cli --test cli author_workflow_promote`;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `npm run dogfood:benchmark -- phase-close run-1783475396014231000-2 --phase implementation`.

This review reran the required validation set:

- `cargo fmt --all --check` - passed;
- `cargo clippy --workspace --all-targets -- -D warnings` - passed;
- `cargo test --workspace` - passed;
- `npm run check:docs` - passed;
- `npm run dogfood:benchmark -- phase-close run-1783477805220674000-2 --phase review` - passed.

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- Plan persisted steward approval consumption before treating approval as a
  durable governance record.
- Plan draft cleanup, archive, or supersession semantics.
- Plan workflow catalog/state integration separately if loader-visible file
  placement becomes insufficient.
- Consider exceptional post-write validation recovery/rollback policy.
- Add focused active-context validation failure coverage if practical.

## 15. Recommended Next Phase

Recommended next phase: draft cleanup/archive/supersession planning.

Reason: active promotion now preserves inactive drafts by design. Before adding
persisted steward approvals or workflow catalog state, the project should decide
how promoted drafts are marked superseded, archived, retained, or cleaned up
without losing auditability or surprising maintainers.
