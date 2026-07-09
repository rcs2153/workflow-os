# Executor Proof Marker Citation Report Input Plan

Status: implemented as explicit executor report input propagation.

## 1. Executive Summary

Terminal local `WorkReport` generation can now opt in to approval proof-marker citations through `TerminalLocalWorkReportInput.approval_proof_marker_citation_policy`.

The next boundary is executor report-bearing input propagation. The goal is to let explicit executor report APIs pass the same policy into terminal report generation without making proof-marker citations automatic and without changing existing workflow semantics.

This plan's first implementation slice adds explicit executor report input propagation for the proof-marker citation policy. It does not change `LocalExecutor::execute(...)`, create report artifacts, add audit projection persistence, add CLI rendering, change schemas, enable writes, or change release posture.

## 2. Goals

- Add an explicit executor report input field for terminal report proof-marker citation policy in a future implementation.
- Preserve existing executor behavior when the field is absent.
- Preserve marker-free approval compatibility by default.
- Allow strict callers to require proof markers for report generation.
- Allow callers to request workflow event citations only when explicitly configured.
- Reuse existing `TerminalReportApprovalProofMarkerCitationPolicy`.
- Reuse terminal report generation as the enforcement point.
- Preserve workflow pass/fail semantics if report generation fails.
- Keep failures structured, stable, and non-leaking.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- changing `LocalExecutor::execute(...)`;
- automatic proof-marker citation for all report-bearing executions;
- automatic runtime report generation;
- report artifact proof-marker gates;
- audit projection persistence;
- CLI rendering;
- workflow schema changes;
- examples;
- provider writes;
- hosted or distributed behavior;
- reasoning lineage;
- side-effect execution;
- release posture changes.

## 4. Current Surfaces

Implemented surfaces:

- `TerminalReportApprovalProofMarkerCitationPolicy`;
- `TerminalLocalWorkReportInput.approval_proof_marker_citation_policy`;
- `generate_terminal_local_work_report(...)`;
- `LocalExecutionReportInputs`;
- `LocalExecutor::execute_with_report(...)`;
- executor report-bearing result behavior that returns a run plus optional report/report error.

Implemented first slice:

- `LocalExecutionReportInputs` exposes the optional proof-marker citation policy.
- `LocalExecutor::execute_with_report(...)` passes the policy into `TerminalLocalWorkReportInput`.
- Missing proof markers remain report-generation errors only and do not change workflow execution status.

## 5. Recommended First Implementation Boundary

The implemented boundary adds one optional field to `LocalExecutionReportInputs`:

```rust
pub approval_proof_marker_citation_policy:
    Option<TerminalReportApprovalProofMarkerCitationPolicy>
```

That value is passed into `TerminalLocalWorkReportInput` inside the existing executor report input conversion path.

Do not change:

- `LocalExecutor::execute(...)`;
- approval request/decision semantics;
- default executor report behavior;
- report artifact writing;
- audit event emission;
- CLI output;
- workflow specs.

## 6. Behavior Rules

- If the policy field is absent, executor report-bearing behavior remains unchanged.
- If the policy is present and `require_proof_markers` is false, marker-free approval decisions remain compatible and produce no proof-marker citations.
- If the policy is present and `require_proof_markers` is true, report generation fails when approval decisions lack proof markers.
- Report generation failure returns through the existing report error channel and does not mutate the completed/failed/canceled run.
- Workflow event proof-marker citations are included only when `include_workflow_event_citations` is true.
- No EvidenceReference values are created implicitly.
- No approval references, workflow events, audit events, artifacts, or IDs are fabricated.

## 7. Error Handling

The future implementation should rely on terminal report generation errors.

Required behavior:

- missing proof markers use `approval_proof_marker_citation.marker_missing`;
- report-generation failures do not become misleading project diagnostics;
- errors do not include approval IDs, presentation IDs, content hashes, command output, provider payloads, raw source/spec contents, paths, tokens, or secret-like values;
- report generation failure does not change workflow run status;
- report generation failure does not append events;
- report generation failure does not write artifacts.

## 8. Privacy And Redaction

The executor propagation path must not copy:

- approval-presentation payloads;
- approval handoff text;
- work summaries;
- approved scope;
- strict non-goals;
- validation expectations;
- command output;
- provider payloads;
- source/spec contents;
- credentials, tokens, authorization headers, private keys, or secret-like values.

The executor should pass only the policy value. The report helper remains responsible for citation construction and redaction-safe summaries.

## 9. Workflow Semantics

Executor propagation must preserve the existing report-bearing execution contract:

- execution failure before a run exists returns `Err` unchanged;
- terminal run plus successful report generation returns run and report;
- terminal run plus report generation failure returns run plus report error;
- non-terminal run returns run plus report error and no report;
- no runtime state mutation occurs during report construction;
- no post-terminal events are appended;
- no StateBackend report writes are introduced.

## 10. Test Plan

Implementation tests cover:

- existing `execute_with_report(...)` behavior is unchanged when the policy is absent;
- executor report input with policy cites proof-marked granted approvals;
- executor report input with policy cites proof-marked denied approvals;
- workflow event citations are included only when requested;
- marker-free approvals remain compatible when markers are not required;
- required marker missing returns report error without changing workflow status;
- report error code is `approval_proof_marker_citation.marker_missing`;
- errors do not leak approval IDs, presentation IDs, content hashes, reasons, payloads, or secret-like values;
- run events are not mutated;
- no report artifact is written;
- no CLI output is emitted;
- existing terminal report, approval presentation, local executor, and artifact tests continue to pass.

## 11. Proposed Implementation Sequence

1. Add optional policy field to `LocalExecutionReportInputs`.
2. Pass the field into `TerminalLocalWorkReportInput`.
3. Add focused local executor tests.
4. Update docs and create an implementation report.
5. Review before audit projection or report artifact gate work.

## 12. Deferred Work

- Executor defaults that require proof markers.
- Audit projection persistence for proof-marker citation posture.
- Report artifact gates requiring proof-marker citations.
- Workflow-declared proof-marker citation requirements.
- CLI rendering of proof-marker citation posture.
- Approval-card UI.
- EvidenceReference creation for approval proof records.
- Provider writes or write-capable adapter enforcement.

## 13. Final Recommendation

Proceed next with executor proof-marker citation report input propagation implementation.

The implementation should be a narrow additive field propagation only. It must keep proof-marker citation behavior explicit, local, in-memory, report-only, and non-mutating.
