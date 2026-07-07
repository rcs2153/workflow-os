# Governed Workflow Authoring CLI Dry-Run Implementation Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds `workflow-os author workflow --from-recommendation <id> --dry-run` as an explicit, non-mutating authoring preview surface. It reuses the accepted inactive draft proposal helper, requires `--dry-run`, requires an existing first-run recommendation id, and keeps workflow authoring review-only.

No blocker-level issue was found.

## 2. Scope Verification

The phase stayed within approved dry-run-only scope.

Confirmed in scope:

- `Command::AuthorWorkflow`;
- parser support for `author workflow`;
- required `--dry-run`;
- required `--from-recommendation <id>`;
- project loading and validation before preview;
- reuse of bounded first-run recommendation data;
- reuse of inactive draft proposal construction;
- bounded human preview output;
- bounded preview JSON output behind global `--json`;
- CLI docs and focused CLI tests.

No accidental implementation was found for:

- workflow file generation;
- repository file writes;
- workflow registration;
- workflow promotion or activation;
- workflow catalog storage;
- runtime state creation;
- command execution;
- local check registration or execution;
- provider calls;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 3. CLI Surface Assessment

The CLI shape is appropriate for this phase:

```sh
workflow-os author workflow --from-recommendation <id> --dry-run
```

`author workflow` separates authoring intent from execution. `--from-recommendation` keeps the command tied to existing bounded first-run signals. Requiring `--dry-run` makes the non-mutating posture explicit and prevents users from assuming that authoring has become a write path.

The command also fits existing parser conventions without adding a broader authoring subsystem.

## 4. Behavior Assessment

The command:

- loads the local project;
- validates the project before preview;
- recomputes first-run recommendation context;
- finds one requested recommendation;
- builds the inactive draft proposal;
- prints required authoring decisions, validation expectations, missing fields, non-goals, privacy posture, and next action.

It does not create `.workflow-os/state`, append runtime events, create approvals, or start a workflow run. Existing `validate`, `first-run`, `first-run --recommendation`, `run`, and scaffold behavior remain unchanged.

## 5. Validation And Error Handling Assessment

Validation is deterministic and fail-closed.

The command rejects:

- missing `--dry-run` with `cli.workflow_authoring.dry_run_required`;
- missing recommendation id with `cli.workflow_authoring.recommendation_required`;
- missing project with `cli.workflow_authoring.manifest_missing`;
- invalid project with `cli.workflow_authoring.validation_failed`;
- unavailable project bundle with `cli.workflow_authoring.project_unavailable`;
- unknown recommendation id with `cli.workflow_authoring.recommendation_not_found`;
- invalid or secret-like recommendation ids with `cli.workflow_authoring.unsafe_payload_rejected`.

Errors do not echo unsafe recommendation ids. This is the right boundary for a preview command that accepts user-supplied identifiers.

## 6. Privacy And Redaction Assessment

The implementation uses bounded first-run metadata and static Workflow OS vocabulary.

Review confirmed the command does not copy:

- raw source contents;
- manifest bodies;
- package script command bodies;
- dependency values;
- lockfile contents;
- CI logs;
- provider payloads;
- issue or pull request bodies;
- parser payloads;
- absolute private paths;
- environment values;
- credentials, authorization headers, private keys, or token-like strings;
- existing agent instruction bodies.

The output uses codes, labels, counts, and proposal obligations. Secret-like recommendation ids are rejected without being echoed.

## 7. JSON Assessment

Preview JSON is bounded and clearly scoped under `author_workflow_dry_run`.

It includes:

- preview schema version;
- preview mode and status;
- source recommendation metadata;
- inactive draft proposal fields;
- non-mutation booleans;
- next action.

The docs correctly mark JSON as preview-only through `0.2.0-preview.1`. This should remain true until a stable machine-output contract is intentionally designed.

## 8. Test Quality Assessment

Tests cover the main risk boundaries:

- missing `--dry-run` fails closed;
- missing recommendation id fails closed;
- known recommendation produces inactive dry-run preview;
- preview includes required authoring decisions and missing fields;
- preview states no files, workflow registration, commands, providers, or runtime state;
- preview does not emit run or approval ids;
- preview JSON is bounded;
- unknown recommendation id fails closed without echoing the id;
- secret-like recommendation id fails closed without leakage;
- runtime state is not created.

No blocker-level test gaps were found.

Non-blocking test follow-ups:

- Parse preview JSON structurally once the JSON shape is closer to stable.
- Add one test for invalid project failure if authoring preview starts to grow more public usage.
- Add per-recommendation tests if proposal text becomes more ecosystem-specific.

## 9. Documentation Review

Docs correctly say:

- the command is dry-run only;
- inactive authoring obligations are previewed;
- no files are written;
- no workflows are registered or promoted;
- no commands are executed;
- no providers are called;
- no runtime state is created;
- raw repository payloads are not copied;
- schemas, examples, hosted behavior, writes, and release posture changes remain unimplemented.

The roadmap accurately places this as the next step after first-run recommendation detail and inactive draft proposal output.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Keep `author workflow` dry-run only until file-output behavior has a separate plan and review.
- Plan conflict detection before any future workflow file output.
- Keep generated workflow drafts inactive until explicit promotion semantics exist.
- Keep preview JSON marked unstable until compatibility rules exist.
- Consider making future preview output more repo-specific only through bounded metadata, not raw source or command payloads.

## 12. Recommended Next Phase

Recommended next phase: governed workflow authoring file-output planning.

The dry-run command gives operators a safe preview of what needs to be authored. The next question is whether Workflow OS should support an explicit file-output path for inactive draft workflow files. That must be planned before implementation because it introduces repository mutation, conflict detection, preservation behavior, and stronger validation requirements.

The next phase should remain planning-only and must not implement file writes, workflow registration, promotion, command execution, provider calls, schemas, examples, hosted behavior, write-capable adapters, or release posture changes.

## 13. Governed Dogfood Summary

- Workflow: `dg/review`.
- Run ID: `run-1783399778189350000-2`.
- Approval ID: `approval/run-1783399778189350000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer after the full approval handoff was emitted.
- Event summary: 39 events total; 1 approval; 0 retries; 0 escalations; terminal status `Completed`.
- Scope approved: create the maintainer review document for the non-mutating author workflow dry-run CLI implementation.
- Strict non-goals: no implementation, workflow writes, workflow registration, promotion, command execution, provider calls, runtime state, schemas, examples, hosted behavior, writes, or release posture changes.
- Out-of-kernel work disclosed: file editing, docs validation, diff check, git commit, PR creation, and PR merge remain agent/GitHub actions outside the kernel.

## 14. Validation Commands Run

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783399778189350000-2 --phase review`: passed.
