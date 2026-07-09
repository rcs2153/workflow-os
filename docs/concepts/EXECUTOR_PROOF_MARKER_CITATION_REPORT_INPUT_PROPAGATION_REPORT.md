# Executor Proof Marker Citation Report Input Propagation Report

## 1. Executive Summary

The executor report-bearing path now accepts an explicit approval proof-marker citation policy and forwards it into terminal local WorkReport generation.

This keeps proof-marker citation behavior opt-in, local, in-memory, and report-only. Existing executor execution behavior is unchanged when the policy is absent.

## 2. Scope Completed

- Added optional `approval_proof_marker_citation_policy` to `LocalExecutionReportInputs`.
- Propagated the policy into `TerminalLocalWorkReportInput`.
- Preserved default marker-free executor report behavior.
- Added focused local executor tests for successful proof-marker citation propagation.
- Added focused local executor tests for missing required proof markers returning report errors only.
- Preserved non-mutating report generation behavior.

## 3. Scope Explicitly Not Completed

- No changes to `LocalExecutor::execute(...)`.
- No executor default requiring proof-marker citations.
- No automatic runtime report generation.
- No report artifact proof-marker gates.
- No audit projection persistence.
- No CLI rendering.
- No schema changes.
- No examples.
- No provider writes.
- No hosted or distributed behavior.
- No reasoning lineage.
- No side-effect execution.
- No release posture changes.

## 4. API Summary

`LocalExecutionReportInputs` now includes:

```rust
approval_proof_marker_citation_policy:
    Option<TerminalReportApprovalProofMarkerCitationPolicy>
```

When supplied, `LocalExecutor::execute_with_report(...)` forwards the policy into terminal local report generation. When absent, behavior remains unchanged.

## 5. Behavior Summary

- `require_proof_markers: true` requires approval decisions cited in the generated report to include durable proof markers.
- `include_workflow_event_citations: true` allows terminal report generation to cite proof-marked workflow approval events.
- Missing required proof markers fail report generation through the existing report error channel.
- Workflow execution status, run events, runtime state, and report artifacts are unchanged by report-generation failure.

## 6. Error Handling And Workflow Semantics

The implementation relies on existing terminal report generation errors.

Missing proof markers use stable error code `approval_proof_marker_citation.marker_missing`. The error path remains report-only: the completed run is returned, the report is absent, and workflow status is preserved.

Errors do not include approval IDs, presentation IDs, content hashes, provider payloads, command output, raw source contents, paths, tokens, or secret-like values.

## 7. Privacy And Redaction Summary

The executor propagation path passes only the bounded policy value. It does not copy approval presentation payloads, approval handoff text, work summaries, approved scope, strict non-goals, validation expectations, command output, provider payloads, source/spec contents, credentials, tokens, authorization headers, private keys, or secret-like values.

## 8. Test Coverage Summary

Focused tests cover:

- executor report input with proof-marker policy cites proof-marked granted approvals;
- workflow event citations are included when requested;
- required marker missing returns a report error and no report;
- missing-marker errors are stable and non-leaking;
- run events are not mutated;
- no report artifacts are written.

Existing local executor, terminal report, WorkReport, approval presentation, and artifact behavior remain covered by the workspace test suite.

## 9. Commands Run And Results

- `cargo fmt --all`: passed.
- `cargo test -p workflow-core --test local_executor execute_with_report_forwards_approval_proof_marker_citation_policy -- --nocapture`: passed.
- `cargo test -p workflow-core --test local_executor execute_with_report_required_proof_marker_missing_returns_report_error_only -- --nocapture`: passed.

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783620402176188000-2 --phase implementation`: passed.

## 10. Dogfood Governance

This phase ran under the self-governed build benchmark.

- workflow_id: `dg/implement`
- run_id: `run-1783620402176188000-2`
- approval_id: `approval/run-1783620402176188000-2/implementation-approved`
- presentation_id: `presentation/a6a17c66859f6ede`
- presentation_content_hash: `a6a17c66859f6ede863502dd8dd5ed2d388e5d4ee9a9ccc8b7255d639b44e821`
- approval_outcome: granted
- event_summary: 39 events; 1 approval; 0 retries; 0 escalations; terminal Completed
- proof_enforcement: proof_enforced; approval-presentation event marker present

## 11. Remaining Known Limitations

- Proof-marker citations remain opt-in on executor report inputs.
- Executor defaults do not require proof-marker citations.
- Audit projection persistence for proof-marker citation posture remains deferred.
- Report artifact proof-marker gates remain deferred.
- Workflow-declared proof-marker citation requirements remain deferred.
- CLI rendering of proof-marker citation posture remains deferred.

## 12. Recommended Next Phase

Recommended next phase: executor proof-marker citation report input propagation review.

This phase touched executor-adjacent report behavior and should receive a maintainer review before audit projection persistence or report artifact proof-marker gates are implemented.
