# Proportional-Governance Selected Project-Validation Report Adapter Report

Fix-forward note: the existing-terminal temporal-provenance blocker identified
by the focused review is fixed in
[Proportional-Governance Selected Project-Validation Report Adapter Blocker Fix Report](PROPORTIONAL_GOVERNANCE_SELECTED_PROJECT_VALIDATION_REPORT_ADAPTER_BLOCKER_FIX_REPORT.md).
The original phase report remains the record of the implementation as first
delivered; focused blocker-fix review is still required.

## 1. Executive Summary

The selected project-validation consumer now has an additive Core report
adapter. It accepts the existing selected execution request, explicit report
inputs, and bounded reference metadata, then returns the existing authoritative
route result with an exact same-call `LocalCheckResultReference` and terminal
WorkReport posture.

The implementation remains Core-only, local, explicit, and opt-in. It does not
change CLI behavior, executor defaults, provider execution, SideEffect
behavior, schemas, examples, hosted behavior, or release posture.

## 2. Scope Completed

- Added `LocalSelectedProjectValidationGovernanceReportRequest`.
- Added `execute_selected_project_validation_governance_report`.
- Reused the selected Core-owned runtime-fact source and existing report
  compositor.
- Derived the report reference from the exact canonical check result produced
  by the selected route.
- Generated terminal WorkReports for terminal outcomes.
- Returned deferred report posture for non-terminal approval outcomes.
- Supported existing-terminal reassessment without workflow skill
  re-execution.
- Added a bounded accessor for the evaluation time already committed by a
  durable runtime-fact snapshot binding.

## 3. Scope Explicitly Not Completed

- No CLI run or approval adoption.
- No selected approval adoption envelope or approval-gate-kind projection.
- No report-artifact persistence changes.
- No automatic activation or executor-default changes.
- No provider execution, OpenShell integration, SideEffect execution, or new
  mutation family.
- No schemas, SDKs, examples, hosted behavior, enterprise administration,
  nested harness execution, reasoning lineage, or release changes.

## 4. API And Composition Summary

The request contains only the accepted selected project-validation execution
request, existing `LocalExecutionReportInputs`, and existing
`AuthoritativeDocsCheckReportReferenceInputs`.

The adapter converts the closed selected request to the existing internal
report request and invokes the canonical report compositor with the selected
Core-owned runtime-fact source option. The compositor performs reference
preflight before process use, executes the canonical check once per call,
constructs the exact local-check reference, and preserves route truth if later
report construction fails.

## 5. Terminal And Non-Terminal Behavior

Quiet terminal execution returns a completed run, generated WorkReport, and
the exact local-check reference. Approval-required execution returns the
waiting run, deferred non-terminal report posture, no fabricated WorkReport,
and the exact check reference that supported the approval route.

An existing terminal retry rehydrates the run, revalidates immutable inputs,
reruns only the canonical check, and regenerates the in-memory report without
executing workflow skills again.

## 6. Source-Binding Integrity

The initial implementation probe exposed an important retry invariant: a new
wall-clock evaluation time changes the selected runtime-fact snapshot binding.
The adapter therefore reads the evaluation time already committed by the
durable runtime-fact snapshot for existing-terminal reassessment and routes the
recheck through the same source bridge used by the initial selected run.

This does not weaken comparison or reuse authority. It reproduces the exact
durable assessment context while obtaining a fresh same-call check result.

## 7. Privacy And Error Posture

Request Debug output redacts execution identity and relies on the existing
redaction-safe report and reference Debug implementations. Duplicate stable
references fail before check execution or event creation. Errors retain the
existing stable, non-leaking authoritative report-consumer codes. No raw check
output, paths, source contents, environment values, tokens, provider payloads,
or credentials are copied into the report reference.

## 8. Test Coverage

Focused tests cover:

- terminal quiet report generation;
- exact result and output-reference projection;
- approval-required report deferral;
- existing-terminal reassessment without workflow re-execution;
- duplicate-reference preflight before process or event use; and
- request Debug non-leakage.

The workspace regression suite remains the compatibility boundary for
WorkReport, approval, immutable bundle, evidence, SideEffect, provider,
runtime, and CLI behavior.

## 9. Commands Run And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- Focused selected report-adapter tests: 4 passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Governed Phase Record

- Dogfood workflow: `dg/implement`.
- Run ID: `run-1786446058514276000-2`.
- Approval ID:
  `approval/run-1786446058514276000-2/implementation-approved`.
- Presentation ID: `presentation/f924d31a1d6b015c`.
- Approval outcome: granted with persisted presentation proof.
- Phase status: `Completed`.
- Approval-presentation enforcement: `proof_enforced`; one matching proof
  record and an approval-event proof marker were present.
- Event summary: 39 events; one approval request; one approval grant; six
  scheduled steps; six skill invocations requested, started, and succeeded;
  zero retries; zero escalations.

Repository edits, test commands, documentation, and git actions are executed
outside the kernel under this bounded governed phase. The kernel coordinates
scope and approval; it does not edit files, run checks, mutate git state, or
publish changes.

## 11. Remaining Limitations

- The adapter is not wired into CLI behavior.
- The selected approval result does not yet expose the exact decision-time
  reference or bounded approval-gate kind required for CLI compatibility.
- Report artifacts remain on the existing separate persistence paths.
- The selected consumer remains limited to the closed one-step local
  project-validation profile.

## 12. Recommended Next Phase

Perform a focused maintainer review of this selected report adapter. If
accepted, implement and review the selected approval adoption envelope before
any CLI cutover.
