# Terminal Report Approval Proof Marker Citation Integration Plan

Status: planning only.

## 1. Executive Summary

Approval decision proof markers are implemented for the opt-in approval-presentation path, bounded inspect/projection exposure is implemented, and the pure in-memory citation helper is implemented and reviewed.

The next question is how terminal local WorkReport generation should opt in to those citations.

This plan defines a narrow integration path only. It does not implement terminal report integration, audit projection persistence, report artifact writing, CLI rendering, schema changes, provider writes, hosted behavior, reasoning lineage, side-effect execution, or release posture changes.

## 2. Goals

- Let terminal local WorkReports cite proof-marked approval decisions when explicitly requested.
- Reuse `derive_approval_proof_marker_report_citations(...)`.
- Preserve existing approval reference citation behavior.
- Preserve marker-free approval compatibility by default.
- Keep citation construction local, deterministic, and in-memory.
- Keep approval-presentation payloads out of report sections, summaries, serialization, and debug output.
- Preserve existing workflow pass/fail semantics.
- Prepare for future high-assurance approval and write-readiness report gates.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- automatic report generation for every run;
- automatic proof-marker report citation for all callers;
- audit projection persistence;
- report artifact writing;
- CLI rendering;
- workflow schema changes;
- examples;
- provider writes;
- hosted/distributed behavior;
- reasoning lineage;
- side-effect execution;
- release posture changes.

## 4. Current Surfaces

Relevant implemented surfaces:

- `ApprovalDecisionProofMarker` is attached to approval decisions only through the explicit approval-presentation enforcement path.
- `WorkflowRunEventKind::ApprovalGranted` and `WorkflowRunEventKind::ApprovalDenied` can carry proof-marked approval decisions.
- `derive_approval_proof_marker_report_citations(...)` derives bounded `ApprovalDecision` and optional `WorkflowEvent` citations from a borrowed `WorkflowRun`.
- `TerminalLocalWorkReportInput` already accepts explicit approval reference IDs and other stable citation IDs.
- `terminal_report_citations(...)` is the central internal helper that builds report citations for terminal local report generation.
- `terminal_report_sections(...)` places approval citations in the Approvals section and combines policy/approval citations in Decisions Made.

Current non-surfaces:

- terminal report generation does not call the proof-marker citation helper;
- executor report inputs do not expose proof-marker citation options;
- audit projection does not emit dedicated proof-marker audit records;
- report artifact gates do not require proof-marker citations.

## 5. Recommended First Integration Boundary

Add an explicit opt-in field to terminal report generation input rather than automatically scanning approval events for every generated report.

Recommended small implementation:

1. Add a report-safe options type, such as `TerminalReportApprovalProofMarkerCitationPolicy`.
2. Add an optional field to `TerminalLocalWorkReportInput`.
3. In `terminal_report_citations(...)`, when the option is present, call `derive_approval_proof_marker_report_citations(...)`.
4. Merge returned approval decision citations into existing approval citations.
5. Merge returned workflow event citations into existing workflow event citations only when requested.
6. Preserve existing approval reference citations and ordering.

This keeps the behavior explicit, testable, and local without changing default reports.

## 6. Candidate Input Shape

Candidate policy fields:

- `enabled: bool`;
- `require_proof_markers: bool`;
- `include_workflow_event_citations: bool`;
- `include_marker_free_count_in_summary: bool` or deferred;
- `sensitivity: WorkReportSensitivity` or reuse report sensitivity;
- `redaction: RedactionMetadata` or reuse report redaction metadata.

Preferred first implementation:

- no separate sensitivity or redaction field;
- reuse report sensitivity and redaction metadata;
- use one optional policy struct with:
  - `require_proof_markers`;
  - `include_workflow_event_citations`.

This avoids duplicating report-level redaction policy and keeps the terminal report input bounded.

## 7. Citation Placement

Approval decision proof-marker citations should be placed in:

- `Approvals`: primary location;
- `Decisions Made`: inherited through the existing combined policy/approval citation behavior.

Optional workflow event citations should be placed in:

- `Work Performed`: inherited through existing workflow event citations, if caller requests workflow event citation emission.

Do not place proof-marker citations in:

- `Evidence Considered`, unless a future phase creates explicit EvidenceReference values;
- `Validation And Quality Checks`;
- `Side Effects`;
- `Known Limitations`, except as bounded text when required markers are missing and caller policy chooses disclosure rather than failure.

## 8. Marker-Free And Missing Behavior

Default behavior must remain compatibility-safe.

Rules:

- Existing reports remain unchanged when the option is absent.
- Marker-free approval decisions remain valid when `require_proof_markers` is false.
- Marker-free approval decisions produce no proof-marker citations when `require_proof_markers` is false.
- If `require_proof_markers` is true and any approval decision lacks a marker, terminal report generation should return a report-generation error with `approval_proof_marker_citation.marker_missing`.
- Missing marker failures must not alter workflow run status.
- Missing marker failures must not append runtime events.
- Missing marker failures must not create report artifacts.

Do not fabricate:

- approval references;
- workflow event references;
- evidence references;
- audit event references;
- proof-marker records.

## 9. Privacy And Redaction

The integration must not copy:

- approval-presentation payloads;
- approval handoff text;
- work summaries;
- approved scope;
- strict non-goals;
- validation expectations;
- why-now text;
- command output;
- provider payloads;
- source/spec contents;
- credentials, tokens, authorization headers, private keys, or secret-like values.

The integration should use the existing report sensitivity and redaction metadata already validated by terminal report generation.

Debug and serialization must remain safe through existing `WorkReportCitation` and `WorkReport` behavior.

## 10. Workflow Semantics

Terminal report proof-marker citation integration must not:

- change approval grant or denial behavior;
- change `WorkflowRun` status;
- mutate `WorkflowRun`;
- append events;
- write state;
- create report artifacts;
- trigger provider calls;
- expose CLI output;
- change executor semantics by default.

If report generation fails because proof markers are required but missing, the report path fails while the original workflow run remains unchanged.

## 11. Executor Boundary

Do not add executor-level automatic behavior in the first implementation unless a caller already uses the explicit report-bearing execution path and supplies the new report input policy.

Recommended posture:

- first integrate at `TerminalLocalWorkReportInput` and `generate_terminal_local_work_report(...)`;
- then add explicit executor report input propagation only after review;
- do not enable proof-marker citation in `execute(...)`;
- do not change `execute_with_report(...)` defaults.

## 12. Test Plan

Future implementation tests should cover:

- default terminal report without proof-marker option is unchanged;
- proof-marked granted approval emits approval citation;
- proof-marked denied approval emits approval citation;
- optional workflow event citation is emitted only when requested;
- marker-free approval remains compatible when markers are not required;
- required marker missing fails closed with `approval_proof_marker_citation.marker_missing`;
- report generation failure does not mutate run/events/status;
- citation ordering is deterministic;
- approval section contains proof-marker approval citations;
- decisions section inherits proof-marker approval citations;
- no EvidenceReference values are created implicitly;
- no raw approval-presentation payload or handoff text is copied;
- Debug and serialization do not leak approval IDs, presentation IDs, hashes, or secret-like values beyond existing validated citation serialization posture;
- existing WorkReport, approval presentation, local executor, and report artifact tests still pass.

## 13. Proposed Implementation Sequence

1. Add a small terminal report approval proof-marker citation policy type.
2. Add an optional field to `TerminalLocalWorkReportInput`.
3. Wire `terminal_report_citations(...)` to call `derive_approval_proof_marker_report_citations(...)` only when policy is supplied.
4. Add focused WorkReport tests.
5. Update docs and implementation report.
6. Review before any executor report input propagation or audit projection work.

## 14. Deferred Work

- Executor report input propagation for proof-marker citation policy.
- Audit projection for proof-marker posture.
- Report artifact gates that require proof-marker citations.
- Workflow-declared proof-marker citation requirements.
- CLI rendering of proof-marker citation posture.
- Approval-card UI.
- EvidenceReference creation for approval proof records.
- Provider writes or write-capable adapter enforcement.

## 15. Open Questions

- Should terminal reports include workflow event citations by default when proof-marker citation policy is enabled, or should approval decision citations be the default?
- Should marker-free counts appear in report section summary text, or stay available only to callers through helper result counts?
- Should required marker failure always fail report generation, or should some future report contracts allow explicit incomplete disclosure?
- Should high-assurance approval disclosure require proof-marker citation once the integration exists?
- Should report artifacts eventually reject writes when required proof-marker citations are missing?

## 16. Final Recommendation

Proceed next with terminal report proof-marker citation integration implementation, limited to explicit `TerminalLocalWorkReportInput` opt-in.

Do not build executor default behavior, audit persistence, artifact gates, CLI rendering, schemas, provider writes, hosted behavior, reasoning lineage, side-effect execution, or release posture changes in that implementation phase.
