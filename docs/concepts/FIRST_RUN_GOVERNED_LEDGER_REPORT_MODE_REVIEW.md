# First-Run Governed Ledger/Report Mode Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds a useful first-run onboarding surface without crossing the phase boundary. `workflow-os first-run` gives a valid Workflow OS project immediate governed-work posture, but it does not pretend that a workflow run, approval, evidence capture, audit record, or report artifact exists.

Proceed next to **sidecar existing-repo governance planning** or **first-run ledger mode follow-up hardening**, depending on whether the next priority is external-repo onboarding or strengthening the current in-repo onboarding command.

## 2. Scope Verification

The phase stayed within approved first-run ledger/report posture scope.

Implemented:

- explicit `workflow-os first-run` CLI command;
- local project load and validation before output;
- bounded report-ready context output;
- all v1 report section kinds represented through validated `WorkReportSection` constructors;
- bounded incomplete-work, limitation, risk, and handoff-note disclosures through existing WorkReport note constructors;
- explicit missing-evidence, skipped-check, and unsupported-side-effect posture;
- review-only workflow recommendations;
- focused tests and documentation updates.

No accidental scope expansion was found:

- no workflow execution from `first-run`;
- no terminal `WorkReport` fabrication;
- no runtime state creation;
- no workflow event appends;
- no approval requests or decisions;
- no EvidenceReference creation;
- no local command execution;
- no local check execution;
- no real local skill handler registration;
- no provider reads or writes;
- no raw repository content inspection;
- no report artifact writing;
- no persistence;
- no general CLI report renderer;
- no workflow generation or registration;
- no sidecar external-repo mode;
- no capability-aware blocked-vs-failed classification;
- no patch artifact modeling;
- no hosted/distributed behavior;
- no recursive agents, agent swarms, or Level 3/4 autonomy.

## 3. Command/API Assessment

The command shape is appropriate:

```sh
workflow-os first-run
```

The command is explicit, local, and opt-in. It fits the existing CLI style and follows naturally after:

```sh
workflow-os init-repo-governance
workflow-os validate
workflow-os first-run
```

The implementation does not add new public Rust core model types or new primitive families. Keeping the helper internal to the CLI is appropriate for this first onboarding slice because the output is a report-ready context, not a stable report generation API.

## 4. Report-Ready Context Assessment

The implementation correctly chooses a report-ready context instead of a terminal `WorkReport`.

That choice is important. A full `WorkReport` requires a terminal workflow run identity, run status, generated context, and report identity. First-run mode does not execute a workflow and therefore must not invent:

- run IDs;
- workflow event IDs;
- approval IDs;
- audit records;
- evidence references;
- terminal run status;
- report artifact identity.

The output remains useful because it still exposes the governed work posture:

- validation passed;
- scaffold present or not detected;
- safe spec counts;
- all section labels;
- missing evidence;
- skipped checks;
- unsupported side effects;
- disclosures and handoff posture;
- recommended workflow candidates.

## 5. Validation Boundary Assessment

Validation behavior is deterministic and appropriately conservative.

Verified:

- missing manifest fails with `cli.first_run.manifest_missing`;
- missing manifest error points to `workflow-os init-repo-governance`;
- invalid projects fail with `cli.first_run.validation_failed`;
- first-run does not print raw validation diagnostics through its error path;
- valid projects produce a bounded report-ready context;
- first-run uses existing project loading and validation rather than a parallel parser;
- WorkReport section and note constructors remain the validation gate for report text.

The command does not mutate invalid projects, create state, or write artifacts when validation fails.

## 6. Section And Disclosure Assessment

All v1 WorkReport section kinds are represented:

- work performed;
- evidence considered;
- decisions made;
- policy gates evaluated;
- approvals;
- validation and quality checks;
- side effects;
- incomplete or deferred work;
- known limitations;
- risks;
- operator handoff notes.

The section summaries are bounded and intentionally static. This is acceptable for the first slice because the command does not inspect raw repository contents or run checks. The command clearly states what is unavailable, skipped, unsupported, or deferred.

The disclosure note constructors are used for incomplete work, limitations, risks, and handoff notes. This protects the output from bypassing the WorkReport text validation boundary.

## 7. Evidence And Citation Assessment

The command does not fabricate citations.

Verified:

- no `EvidenceReference` values are created;
- no workflow event IDs are invented;
- no approval IDs are invented;
- no adapter telemetry references are invented;
- no local check result references are invented;
- missing evidence remains explicit section text.

This matches the accepted missing-citation policy: absent optional references should be represented as explicit section text, not fake citation records.

## 8. Privacy/Redaction Assessment

The implementation is redaction-safe for the approved scope.

Verified:

- no raw repository source contents are read;
- no raw command output is copied;
- no raw parser payloads are copied;
- no provider payloads are copied;
- no CI logs are copied;
- no Git diffs are copied;
- no environment variable values are copied;
- no credentials, authorization headers, private keys, or token-like values are copied;
- `.git` is represented as a boolean only;
- output uses static bounded text, booleans, counts, and section labels.

The tests include a raw payload marker file and confirm those markers do not appear in output.

## 9. Workflow Semantics Assessment

The command preserves workflow semantics.

Verified:

- no workflow is started;
- no `LocalExecutor` path is invoked;
- no local state backend is required;
- no `.workflow-os/state` directory is created;
- no events are appended;
- no approval checkpoint is requested or resolved;
- the explicit mock workflow run remains a separate user action.

This is the right boundary for first-run onboarding: it gives the user a governed ledger/report posture without silently turning setup into execution.

## 10. CLI Output Assessment

Text output is bounded, readable, and clear enough for onboarding.

Preview JSON output is implemented consistently with the existing `--json` preview posture. It remains intentionally non-stable as covered by CLI docs.

The output does not include raw paths, private repository names, command output, run IDs, approval IDs, or fabricated evidence IDs.

## 11. Test Quality Assessment

Tests cover the important first slice behaviors:

- first-run after `init-repo-governance` emits report-ready context;
- preview JSON is bounded;
- missing manifest fails actionably;
- invalid project fails without leaking secret-like content;
- raw repository payload markers are not copied;
- no state root is created;
- no report artifact directory is created;
- help output documents the command.

Existing workspace tests continue to cover project validation, local executor behavior, WorkReport constructors, EvidenceReference behavior, side effects, hooks, adapters, and state behavior.

Non-blocking test gaps:

- Direct test for first-run on a valid non-scaffold Workflow OS project where `scaffold: not_detected`.
- Direct assertion that `first-run` does not create `.workflow-os/reports` when `.workflow-os` already exists from the scaffold beyond the current report-directory check.
- Direct assertion that first-run output does not include private absolute paths.
- Direct assertion that the command does not emit runtime event/audit files when a custom `--state-dir` is supplied.

These are non-blocking because the implementation does not call runtime state or artifact-writing APIs, and existing tests cover the main mutation boundaries.

## 12. Documentation Review

Documentation is aligned with implementation.

Docs now state:

- `workflow-os first-run` is implemented;
- it emits a bounded report-ready context;
- it does not fabricate a terminal `WorkReport`;
- it does not run workflows;
- it does not create runtime state;
- it does not append events;
- it does not write report artifacts;
- it does not inspect raw source contents;
- it does not call providers;
- workflow recommendations are review-only;
- automatic workflow generation and registration remain unsupported.

Historical scaffold report/review docs preserve their original findings with fix-forward notes, which is the right treatment for phase history.

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- Add a focused test for valid non-scaffold projects showing `scaffold: not_detected`.
- Add a focused test for custom `--state-dir` confirming no runtime files are created.
- Consider adding an explicit `first_run.sections.expected_count` assertion helper if more report-ready commands are added.
- Consider a future stable JSON envelope only after CLI machine-output compatibility is separately planned.
- Consider sidecar onboarding planning next so users can govern external target repositories without adding files to the target repo.

## 15. Recommended Next Phase

Choose one:

- **sidecar existing-repo governance planning** if the next priority is external-repo onboarding and sandboxed OSS experiments;
- **first-run ledger mode follow-up hardening** if the next priority is tightening tests and UX around the current in-repo command.

Recommendation: proceed to **sidecar existing-repo governance planning**.

Reason: the in-repo scaffold plus first-run report-ready context now covers the immediate local onboarding gap. The next user scenario is testing or governing an existing external repository without copying Workflow OS dogfood workflows and without mutating the target repo. That should be planned before implementation and must preserve the same boundaries: no arbitrary command execution, no writes, no provider mutation, no artifact writing by default, no workflow auto-registration, and no higher-autonomy claims.

## 16. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
