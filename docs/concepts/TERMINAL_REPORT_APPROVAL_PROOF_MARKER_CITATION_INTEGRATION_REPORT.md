# Terminal Report Approval Proof Marker Citation Integration Report

## 1. Executive Summary

Terminal local `WorkReport` generation can now opt in to approval proof-marker citations.

The implementation adds a small explicit report input policy and wires terminal report citation construction to reuse the existing pure in-memory approval proof-marker citation helper. Existing terminal reports remain unchanged unless callers supply the new policy.

This phase does not add executor default behavior, audit projection persistence, report artifact gates, CLI rendering, workflow schema changes, examples, provider writes, hosted behavior, reasoning lineage, side-effect execution, or release posture changes.

## 2. Scope Completed

- Added `TerminalReportApprovalProofMarkerCitationPolicy`.
- Added optional `approval_proof_marker_citation_policy` to `TerminalLocalWorkReportInput`.
- Wired `terminal_report_citations(...)` to call `derive_approval_proof_marker_report_citations(...)` only when the policy is present.
- Merged derived approval decision citations into existing approval citations.
- Merged derived workflow event citations into existing workflow event citations only when requested.
- Preserved existing explicit approval reference citations and deterministic ordering.
- Added focused tests for opt-in behavior, default behavior, failure behavior, ordering, and non-leakage.
- Updated roadmap and WorkReport planning documentation.

## 3. Scope Explicitly Not Completed

- No executor default proof-marker citation behavior.
- No executor report-input propagation for the new policy.
- No audit projection persistence for proof-marker posture.
- No report artifact gates requiring proof-marker citations.
- No automatic report generation for every run.
- No CLI rendering of proof-marker citation posture.
- No workflow schema changes.
- No examples.
- No EvidenceReference creation for approval proof records.
- No provider writes.
- No hosted or distributed behavior.
- No reasoning lineage.
- No side-effect execution.
- No release posture changes.

## 4. API Summary

The new policy type is:

```rust
pub struct TerminalReportApprovalProofMarkerCitationPolicy {
    pub require_proof_markers: bool,
    pub include_workflow_event_citations: bool,
}
```

`TerminalLocalWorkReportInput` now accepts:

```rust
pub approval_proof_marker_citation_policy:
    Option<TerminalReportApprovalProofMarkerCitationPolicy>
```

When the option is absent, terminal report generation behaves as before. When present, terminal report generation derives bounded citations from approval decision events that carry approval-presentation proof markers.

## 5. Citation Construction Summary

The implementation reuses `derive_approval_proof_marker_report_citations(...)`.

Approval decision citations are added to the terminal report approvals citation set. Because decisions already combine policy and approval citations, the same approval proof-marker citations also appear in the Decisions Made section.

Workflow event citations are added to the Work Performed citation set only when `include_workflow_event_citations` is true.

The integration does not create `EvidenceReference` values, fabricate approval references, fabricate workflow events, read hidden state, mutate the borrowed run, append events, write artifacts, or persist anything.

## 6. Marker-Free And Failure Behavior

Marker-free approval decisions remain compatible by default and when `require_proof_markers` is false.

When `require_proof_markers` is true and an approval decision lacks a proof marker, terminal report generation fails closed with:

```text
approval_proof_marker_citation.marker_missing
```

That report-generation failure does not mutate the run, alter run status, append events, or create partial report artifacts.

## 7. Redaction And Privacy Summary

The implementation does not copy approval-presentation payloads, approval handoff text, work summaries, approved scope, strict non-goals, validation expectations, command output, provider payloads, source/spec contents, credentials, tokens, private keys, or secret-like values.

Citation summaries are fixed bounded strings. The report uses existing `WorkReportCitation` and `WorkReport` validation/redaction behavior.

Focused tests assert that debug and serialization output do not leak presentation IDs, presentation content hashes, approval request reasons, or approval decision reasons.

## 8. Test Coverage Summary

Focused tests cover:

- default terminal report without the proof-marker option is unchanged;
- proof-marked granted approvals emit approval citations;
- proof-marked denied approvals emit approval citations;
- optional workflow event citations appear only when requested;
- marker-free approvals remain compatible when markers are not required;
- missing required markers fail closed without mutating the run;
- citation ordering is deterministic;
- approval proof-marker citations appear in Approvals and Decisions Made;
- presentation payloads and approval handoff text are not copied into debug or serialization output.

The existing `work_report` test suite passes with the new tests included.

## 9. Commands Run And Results

- Dogfood governed phase:
  - `workflow_id`: `dg/implement`
  - `run_id`: `run-1783616160715201000-2`
  - `approval_id`: `approval/run-1783616160715201000-2/implementation-approved`
  - `approval_outcome`: granted
  - `approval_presentation_enforcement`: `proof_enforced`
  - `event_summary`: 39 events, 1 approval, 0 retries, 0 escalations, terminal `Completed`
- `cargo fmt --all --check` - passed.
- `cargo test -p workflow-core --test work_report terminal_report_ -- --nocapture` - passed, 12 selected tests.
- `cargo test -p workflow-core --test work_report` - passed, 181 tests.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 10. Remaining Known Limitations

- The policy is not yet propagated through executor report input types.
- Existing executor report-bearing paths do not enable proof-marker citations by default.
- Report artifact gates do not yet require proof-marker citation presence.
- Audit projections do not yet emit dedicated proof-marker citation posture.
- Workflow specs cannot declare proof-marker citation requirements.
- CLI output does not render proof-marker citation posture.

## 11. Recommended Next Phase

Recommended next phase: terminal report proof-marker citation integration review.

The implementation is intentionally narrow and security-adjacent. Maintainer review should confirm the opt-in boundary, deterministic ordering, failure behavior, and privacy posture before executor input propagation or artifact gate work begins.
