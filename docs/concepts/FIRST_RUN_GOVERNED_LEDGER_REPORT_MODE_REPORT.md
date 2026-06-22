# First-Run Governed Ledger/Report Mode Report

## 1. Executive Summary

The first-run governed ledger/report posture slice is implemented as `workflow-os first-run`.

The command gives a newly scaffolded or otherwise valid local Workflow OS project immediate governed-work posture without pretending a workflow run occurred. It loads and validates the project, builds all v1 report section shapes through existing `WorkReportSection` constructors, validates bounded incomplete-work, known-limitation, risk, and handoff-note disclosures through existing WorkReport note constructors, and prints a bounded report-ready context.

This phase intentionally emits a report-ready context rather than a terminal `WorkReport`, because first-run mode does not execute a workflow and must not fabricate run identity, event IDs, approval decisions, evidence references, or audit records.

## 2. Scope Completed

- Added `workflow-os first-run`.
- Added a bounded first-run report-ready context helper inside the CLI.
- Project loading and validation are required before output.
- Missing manifest errors point users to `workflow-os init-repo-governance`.
- Validation failures return a stable, bounded `cli.first_run.validation_failed` error.
- The command detects the first-run governance scaffold when present.
- Safe project counts are reported for workflows, skills, policies, and tests.
- All v1 WorkReport section kinds are represented.
- Missing evidence, skipped checks, unsupported side effects, incomplete work, limitations, risks, and handoff posture are explicit.
- Review-only workflow recommendations are emitted.
- Preview JSON output is supported through the existing `--json` posture.
- Focused CLI tests cover success, JSON, missing manifest, invalid project, no raw payload copying, and no runtime state/artifact creation.

## 3. Scope Explicitly Not Completed

This phase does not implement:

- workflow execution from `first-run`;
- terminal `WorkReport` generation without a workflow run;
- runtime state creation;
- workflow event appends;
- approval requests or decisions;
- EvidenceReference creation;
- local command execution;
- local check handler execution;
- real local skill handler registration;
- provider reads or writes;
- raw repository content inspection;
- report artifact writing;
- persistence;
- general CLI report rendering;
- workflow generation or registration;
- sidecar external-repo governance;
- capability-aware blocked-vs-failed classification;
- patch artifact modeling;
- hosted or distributed behavior;
- recursive agents, agent swarms, or Level 3/4 autonomy.

## 4. Command/API Summary

The command is:

```sh
workflow-os first-run
```

Recommended onboarding sequence:

```sh
workflow-os init-repo-governance
workflow-os validate
workflow-os first-run
```

The command returns success only after project validation passes. It prints bounded text by default and preview JSON with `--json`.

## 5. Report-Ready Context Summary

The report-ready context includes:

- mode: `report_ready_context`;
- validation posture;
- scaffold presence;
- git repository presence as a boolean only;
- workflow, skill, policy, and test counts;
- all v1 report section kinds;
- disclosure counts;
- evidence as `not_available`;
- checks as `skipped`;
- side effects as `none_skipped_unsupported`;
- review-only workflow recommendations;
- explicit next step for the approval-gated mock workflow.

It does not include raw file contents, raw paths, command output, provider payloads, parser payloads, secrets, tokens, run IDs, approval IDs, or fabricated citations.

## 6. Validation Boundary Summary

The command uses existing project loading and validation:

- missing manifest returns `cli.first_run.manifest_missing`;
- invalid projects return `cli.first_run.validation_failed`;
- diagnostic details are not echoed through the first-run error path;
- report sections and notes are validated through existing WorkReport constructors.

## 7. Privacy And Redaction Summary

The command avoids raw repository inspection. It reports only bounded booleans, counts, section labels, static disclosures, and static recommendations.

It does not copy:

- raw source files;
- raw command output;
- raw parser payloads;
- raw provider payloads;
- raw CI logs;
- raw Git diffs;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

## 8. Test Coverage Summary

Added focused CLI tests for:

- `first-run` after `init-repo-governance` emits a report-ready context;
- preview JSON output remains bounded and report-ready;
- missing manifest fails actionably without state writes;
- invalid project fails without leaking secret-like manifest content;
- raw repository payload markers are not copied;
- no runtime state or report artifact directories are created by first-run mode;
- help output includes the new command.

Existing CLI tests continue to cover `init-repo-governance`, validation, local run, approval, status, inspect, doctor, adapter fixture paths, and state behavior.

## 9. Commands Run And Results

- `cargo test -p workflow-cli --test cli first_run` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 10. Remaining Known Limitations

- `first-run` emits a report-ready context, not a terminal `WorkReport`.
- It does not execute the generated approval-gated mock workflow.
- It does not create runtime state, event history, evidence references, or audit records.
- It does not write report artifacts.
- It does not inspect raw repository files or run checks.
- It recommends workflow candidates only as review-only text.
- Sidecar external-repo governance remains deferred.
- Automatic workflow discovery, generation, registration, promotion, and catalog storage remain deferred.

## 11. Recommended Next Phase

Proceed to **first-run governed ledger/report mode review**.

The review should verify the command is useful enough for onboarding while preserving the honesty boundary: report-ready context now, no fabricated terminal run, no raw content copying, no runtime mutation, no artifacts, no provider calls, and no automatic workflow generation.
