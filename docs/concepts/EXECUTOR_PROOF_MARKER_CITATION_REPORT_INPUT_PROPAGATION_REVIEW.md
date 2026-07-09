# Executor Proof Marker Citation Report Input Propagation Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation is a narrow additive executor report-input propagation slice. It does not change default executor behavior, approval semantics, event append behavior, report artifact behavior, CLI behavior, schemas, writes, hosted behavior, or release posture.

Recommended next phase: audit projection persistence planning for proof-marker citation posture.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented:

- optional `LocalExecutionReportInputs.approval_proof_marker_citation_policy`;
- forwarding into `TerminalLocalWorkReportInput`;
- focused local executor tests;
- roadmap/planning documentation updates;
- implementation report.

No accidental implementation was found for:

- executor defaults requiring proof markers;
- `LocalExecutor::execute(...)` behavior changes;
- automatic runtime report generation;
- report artifact proof-marker gates;
- audit projection persistence;
- CLI rendering;
- schema changes;
- examples;
- provider writes;
- hosted or distributed behavior;
- reasoning lineage;
- side-effect execution;
- release posture changes.

## 3. API Assessment

The new API surface is appropriate and minimal.

`LocalExecutionReportInputs` now carries an optional `TerminalReportApprovalProofMarkerCitationPolicy`. The policy remains explicit caller input and is not inferred from workflow state, runtime config, schemas, defaults, or approval events.

The custom `Debug` implementation exposes only a boolean indicating whether the policy is present. It does not expose approval IDs, presentation IDs, content hashes, reasons, handoff text, provider payloads, command output, paths, tokens, or secret-like values.

## 4. Behavior Assessment

The forwarding path is correct.

`terminal_report_input_for_run(...)` now passes the optional policy into terminal report generation instead of hardcoding `None`. This delegates enforcement and citation derivation to the already-reviewed terminal report layer.

Behavior remains:

- absent policy preserves existing report-bearing executor behavior;
- present policy can require proof markers;
- missing required proof markers fail report generation only;
- workflow status is preserved;
- run events are not mutated;
- report artifacts are not written;
- EvidenceReference values are not created implicitly.

## 5. Error Handling Assessment

The missing-marker path is conservative and non-leaking.

The focused regression test verifies stable error code `approval_proof_marker_citation.marker_missing`, no generated report, preserved completed run status, unchanged event history, and no artifact writes.

The tested debug output avoids leaking approval IDs and presentation text markers. This is consistent with the existing terminal report proof-marker citation helper boundary.

## 6. Privacy And Redaction Assessment

The executor path passes only a bounded policy value.

No approval-presentation payloads, approval handoff text, work summaries, approved scopes, strict non-goals, validation expectations, command output, provider payloads, source/spec contents, credentials, tokens, authorization headers, private keys, or secret-like values are copied into executor report inputs by this implementation.

## 7. Workflow Semantics Assessment

Workflow semantics are preserved.

The implementation does not:

- change `LocalExecutor::execute(...)`;
- change approval request or approval decision semantics;
- append post-terminal events during report generation;
- mutate completed run state;
- require a StateBackend artifact write;
- add CLI output;
- turn report generation errors into workflow execution failures.

## 8. Test Quality Assessment

Tests added are focused and meaningful:

- proof-marked approval decisions can be cited through executor report input propagation;
- workflow event proof-marker citations are included when explicitly requested;
- required proof markers missing from approval decisions return report-generation errors only;
- report errors are stable and non-leaking;
- event history is unchanged;
- no report artifacts are written.

Existing workspace tests cover default absent-policy behavior, terminal report proof-marker helper behavior, WorkReport construction/redaction, approval presentation proof markers, local executor report-bearing behavior, and artifact boundaries.

Non-blocking gap:

- add a focused executor-level denied-approval proof-marker citation test when the next proof-marker review or audit projection phase touches the same area. The terminal report layer already covers denied proof-marker citations, so this is not a blocker for the propagation slice.

## 9. Documentation Review

Documentation is honest.

The implementation report and roadmap state that executor report input propagation is implemented while keeping the following unimplemented:

- executor default proof-marker citation behavior;
- audit projection persistence;
- report artifact proof-marker gates;
- automatic runtime behavior;
- automatic artifact writing;
- CLI rendering;
- schemas;
- examples;
- approval/cancellation report-bearing methods;
- workflow-declared report contracts;
- provider writes;
- hosted behavior;
- release posture changes.

## 10. Dogfood Governance

Implementation phase:

- workflow_id: `dg/implement`
- run_id: `run-1783620402176188000-2`
- approval_id: `approval/run-1783620402176188000-2/implementation-approved`
- presentation_id: `presentation/a6a17c66859f6ede`
- approval_outcome: granted
- phase_close: Completed; 39 events; proof_enforced

Review phase:

- workflow_id: `dg/review`
- run_id: `run-1783621761170118000-2`
- approval_id: `approval/run-1783621761170118000-2/review-scope-approved`
- presentation_id: `presentation/6afb07f08c95a1c2`
- approval_outcome: granted
- phase_close: Completed; 39 events; proof_enforced

## 11. Validation Commands

Implementation validation:

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

Review validation:

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Add executor-level denied proof-marker citation coverage when the next adjacent phase touches this area.
- Plan audit projection persistence for proof-marker citation posture before report artifact proof-marker gates.

## 14. Recommended Next Phase

Recommended next phase: audit projection persistence planning for proof-marker citation posture.

Reason: proof markers can now be modeled, opt-in enforced, inspected, derived as citations, integrated into terminal reports, and propagated through executor report inputs. The next missing governance surface is durable audit/report posture projection, still without default behavior, artifact gates, schemas, writes, or CLI expansion.
