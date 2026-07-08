# Governed Workflow Authoring Steward Review CLI Preview Review

## 1. Executive Verdict

Phase accepted; proceed to active promotion planning.

The steward-review CLI preview is appropriately bounded and ready to serve as
the last user-facing review surface before active promotion planning. It exposes
the existing in-memory steward-review helper through a deterministic CLI path,
derives fresh preflight context, prints bounded text/JSON output, and preserves
the non-mutating authoring boundary.

## 2. Scope Verification

The phase stayed within approved CLI-preview scope.

Implemented scope:

- `workflow-os author workflow steward-review --draft ...`;
- explicit `--decision`, `--reviewer`, and `--reason` inputs;
- fresh in-process preflight derivation;
- bounded steward-review card and decision output;
- text and JSON output;
- focused tests;
- CLI docs, roadmap, implementation-plan updates, and implementation report.

No accidental implementation was found for:

- active workflow promotion;
- workflow registration;
- file movement from `workflows/drafts/` to `workflows/`;
- persisted steward approval records;
- runtime state creation;
- command execution;
- local check execution;
- provider calls;
- report artifacts;
- workflow-declared steward configuration;
- schemas;
- examples;
- hosted or distributed runtime behavior;
- RBAC, IdP, admin UI, paging, or notifications;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 3. CLI Surface Assessment

The CLI shape matches the accepted plan:

```sh
workflow-os author workflow steward-review \
  --draft workflows/drafts/<name>.workflow.yml \
  --decision approved-for-promotion \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason>
```

The command name avoids `promote`, which is important because this phase does
not promote anything. The parser also preserves the existing
`author workflow --from-recommendation ...` path by routing flag-shaped tokens
back to the legacy authoring parser, avoiding a compatibility regression.

## 4. Preflight Integration Assessment

The CLI reuses the existing draft path validation, project loading, draft
loading, content hashing, and preflight assessment path. It does not trust a
caller-supplied preflight blob or pasted model summary.

For blocked drafts, it prints `review_blocked`, blocker/warning codes, and
non-mutation flags, then fails closed with
`cli.workflow_authoring.steward_review_blocked` before calling the steward-review
helper as an approval.

For passing drafts, it derives bounded review summaries and calls
`review_workflow_draft_for_promotion`.

## 5. Decision Semantics Assessment

Decision semantics are correct:

- `approved-for-promotion` maps to `approved_for_future_promotion`;
- `denied`, `needs-changes`, and `deferred` remain non-authorizing;
- the output states that approval only allows future promotion of the exact
  unchanged draft through a separately implemented promotion step.

The command does not persist the approval or make it consumable by a promotion
command. That limitation is intentionally documented and appropriate for this
phase.

## 6. Non-Mutation Assessment

The command reports and tests non-mutation posture:

- `files_written: false`;
- `workflow_registered: false`;
- `workflow_promoted: false`;
- `approval_persisted: false`;
- `commands_executed: false`;
- `providers_called: false`;
- `runtime_state_created: false`.

Tests verify that no active workflow file is created and no local state root is
created by steward review paths. No code path was found that appends events,
creates artifacts, executes local checks, or calls providers.

## 7. Privacy And Redaction Assessment

The implementation preserves the redaction boundary.

The CLI output uses bounded posture summaries and codes. It does not print the
raw approval reason, raw draft YAML, command output, provider payloads, parser
payloads, manifest/script bodies, environment values, credentials, private keys,
authorization headers, or token-like strings.

The core helper validates summary fields and approval reason with stable,
non-leaking errors. Debug output for review card summaries remains redacted in
the core model.

## 8. Error Handling Assessment

The CLI fails closed for:

- missing, unsafe, or out-of-bound draft paths;
- missing or unparsable draft files;
- invalid project state;
- preflight blockers;
- unknown decisions;
- invalid reviewer actor ids;
- missing, long, or secret-like reasons;
- helper validation failures.

Errors use stable codes and avoid echoing raw draft content, raw unsafe paths,
review reason text, parser payloads, command output, provider payloads, or
secret-like values.

## 9. Test Quality Assessment

Focused tests cover the important behavior:

- approved preflight-passing draft produces preview output;
- non-authorizing decisions stay preview-only;
- blocked preflight prevents review authorization;
- JSON output is bounded and non-mutating;
- secret-like review reasons are rejected without leakage;
- no active workflow file is created;
- no runtime state directory is created;
- existing authoring dry-run, file-output, and preflight paths continue to pass.

The tests are not shallow construction tests; they execute the CLI against
temporary projects and verify output, failure codes, and absence of mutation.

Non-blocking future coverage could add explicit tests for `denied` and
`deferred` decisions separately from `needs-changes`, but the shared
non-authorizing path is already covered.

## 10. Documentation Review

Documentation is honest and aligned:

- CLI docs describe the new preview command and usage.
- The plan status points to the implementation report.
- The roadmap states the CLI preview is implemented.
- Docs explicitly state that active promotion, registration, file movement,
  persisted approval, runtime state, schemas, examples, hosted behavior, writes,
  and release posture changes remain unimplemented.

No dangerous false claims were found.

## 11. Governed Dogfood Review Summary

- Workflow: `dg/review`.
- Run ID: `run-1783473133545639000-2`.
- Approval ID: `approval/run-1783473133545639000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer after the full approval
  handoff was emitted.
- Approved scope: inspect merged CLI preview implementation, tests, docs,
  roadmap, and report; create bounded maintainer review.
- Strict non-goals: no implementation, promotion, registration, runtime state,
  commands, providers, writes, schemas, examples, or release posture change.

## 12. Validation

Validation commands run for this review:

- `cargo test -p workflow-cli --test cli author_workflow`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783473133545639000-2 --phase review`:
  passed.

Implementation-phase validation referenced by this review:

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.

## 13. Blockers

No blockers.

## 14. Non-Blocking Follow-Ups

- Add explicit individual tests for `denied` and `deferred` decisions if future
  decision semantics diverge.
- Consider a future persisted steward approval model only as part of active
  promotion planning, not as a standalone hidden approval store.
- Keep JSON output marked preview-only until the authoring promotion contract is
  intentionally stabilized.

## 15. Recommended Next Phase

Recommended next phase: active promotion planning.

The safe sequence is now:

1. plan the active promotion boundary;
2. define how a preview approval can or cannot be consumed;
3. keep promotion explicit, deterministic, and separately reviewed;
4. continue to exclude provider calls, runtime state, schemas, examples, writes,
   hosted behavior, and release posture changes until explicitly scoped.
