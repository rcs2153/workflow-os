# Workflow Discovery Field Coverage Integration Report

## 1. Executive Summary

The Workflow Discovery Field Coverage Integration phase is implemented as bounded `workflow-os first-run` recommendation output.

`first-run` now turns existing local onboarding signals into structured, review-only workflow discovery recommendations. The recommendations cite safe posture labels, ownership/escalation issue codes, and spec-field coverage item codes instead of raw YAML values or source content.

This phase does not implement workflow generation, workflow registration, catalog storage, schema changes, command execution, provider calls, writes, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy.

## 2. Scope Completed

- Added internal first-run workflow discovery recommendation records.
- Added recommendation kinds for create workflow, assign ownership, add evidence/check requirements, add side-effect posture, and add report/handoff obligations.
- Derived recommendations from existing governance posture, ownership/escalation warnings, and spec-field coverage signals.
- Added bounded text output under `workflow_discovery_recommendations`.
- Added preview JSON output under `workflow_discovery_recommendations`.
- Preserved existing human-readable recommendation text for operator readability.
- Added focused CLI tests for structured text output, JSON output, deterministic issue-code ordering, and non-leakage.
- Updated first-run CLI documentation, scaffold field operationalization planning, workflow discovery integration planning, and the roadmap.

## 3. Scope Explicitly Not Completed

This phase did not add:

- workflow generation;
- workflow registration;
- workflow catalog or store persistence;
- workflow proposal state;
- conflict-resolution engine;
- runtime workflow discovery execution;
- local check execution;
- command execution;
- provider calls;
- write-capable adapters;
- schema changes;
- CLI workflow-discovery command;
- RBAC, IdP integration, paging, or enterprise notifications;
- hosted/distributed behavior;
- recursive agents, agent swarms, or Level 3/4 autonomy.

## 4. Output Summary

`workflow-os first-run` now emits recommendation rows like:

```text
workflow_discovery_recommendations: 7
workflow_discovery_recommendation: id=first_run.repo_implementation kind=create_workflow target=project#1 status=review_only summary=repo_implementation_workflow rationale=first_run.report_ready_context|governed_work_pattern.implementation_boundary coverage=spec_field.workflow.steps_enforced_supported_local_paths|spec_field.workflow.policy_requirements_enforced_supported_local_paths|spec_field.workflow.audit_observability_disclosed ownership=none
```

The JSON output includes the same data as a bounded preview object with `id`, `kind`, `target`, `status`, `summary`, `rationale_codes`, `coverage_codes`, and `ownership_issue_codes`.

## 5. Recommendation Boundary Summary

Recommendations are advisory and review-only. They are intended to help users see likely next workflow boundaries without forcing manual YAML authoring as the only adoption path.

Recommendations may cite:

- existing first-run posture labels;
- stable spec-field coverage item codes;
- stable ownership/escalation issue codes;
- static Governed Work Pattern rationale codes.

Recommendations do not include raw owner names, maintainer IDs, escalation contact values, config values, mapping values, command output, provider payloads, parser payloads, source snippets, environment values, credentials, tokens, or private keys.

## 6. Privacy And Redaction Summary

The implementation uses static recommendation vocabulary and bounded codes. It does not inspect raw repository source contents, read provider data, execute commands, or copy raw YAML field values into recommendation output.

Ownership/escalation recommendations include only issue codes such as `ownership.placeholder_owner`, not the underlying owner or escalation values.

## 7. Test Coverage Summary

Focused tests cover:

- structured first-run recommendation text output;
- preview JSON recommendation output;
- create-workflow recommendation shape;
- ownership-assignment recommendation shape;
- related spec-field coverage codes;
- related ownership/escalation issue codes;
- preservation of legacy recommendation text;
- non-leakage of raw owner, maintainer, run ID, approval ID, raw config, raw mapping, provider, command, parser, and source-content markers;
- no runtime state artifacts created by `first-run`.

## 8. Commands Run And Results

- `cargo fmt --all` using the repository bundled Rust toolchain: passed.
- `cargo fmt --all --check` using the repository bundled Rust toolchain: passed.
- `cargo clippy --workspace --all-targets -- -D warnings` using the repository bundled Rust toolchain: passed.
- `cargo test -p workflow-cli first_run -- --nocapture` using the repository bundled Rust toolchain: passed.
- `cargo test --workspace` using the repository bundled Rust toolchain: passed.
- `npm run check:docs` using the repository bundled Node.js runtime: passed.
- `git diff --check`: passed.

## 9. Remaining Known Limitations

- Recommendations are not persisted.
- Recommendations are not promoted into workflow specs.
- No workflow catalog or store exists.
- Conflict hints are not implemented.
- The CLI has no standalone workflow discovery command.
- Recommendations do not yet classify dogfood-specific versus portable user-facing workflow suggestions.
- Missing citations remain output posture rather than catalog records.

## 10. Recommended Next Phase

Recommended next phase: Workflow Discovery Field Coverage Integration Review.

This phase touched user-facing first-run output and should be reviewed before planning catalog/store support, workflow proposal state, conflict hints, or any workflow-generation path.
