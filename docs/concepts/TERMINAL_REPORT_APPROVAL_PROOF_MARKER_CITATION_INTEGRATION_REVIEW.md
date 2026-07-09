# Terminal Report Approval Proof Marker Citation Integration Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The terminal report approval proof-marker citation integration is appropriately narrow, opt-in, deterministic, and privacy-preserving. It reuses the reviewed pure citation helper without changing approval semantics, executor defaults, report artifact behavior, CLI behavior, schemas, provider writes, hosted behavior, reasoning lineage, side-effect execution, or release posture.

## 2. Scope Verification

The phase stayed within the approved terminal report input integration scope.

Implemented:

- `TerminalReportApprovalProofMarkerCitationPolicy`;
- optional `TerminalLocalWorkReportInput.approval_proof_marker_citation_policy`;
- opt-in terminal report citation derivation through `derive_approval_proof_marker_report_citations(...)`;
- approval proof-marker citations in terminal report approval citation sets;
- optional workflow event citations when requested;
- focused tests;
- implementation report and roadmap/planning documentation updates.

No accidental implementation was found for:

- executor default proof-marker citation behavior;
- executor report-input propagation for the new policy;
- automatic report generation for every run;
- audit projection persistence;
- report artifact gates requiring proof-marker citations;
- CLI rendering;
- workflow schema changes;
- examples;
- EvidenceReference creation for approval proof records;
- provider writes;
- hosted or distributed behavior;
- reasoning lineage;
- side-effect execution;
- release posture changes.

## 3. API Boundary Assessment

The new API boundary is minimal and explicit. `TerminalReportApprovalProofMarkerCitationPolicy` carries only:

- `require_proof_markers`;
- `include_workflow_event_citations`.

The policy deliberately reuses report-level sensitivity and redaction metadata rather than adding a second redaction surface. That is the right boundary for this phase because it keeps terminal report construction local and avoids duplicating privacy policy.

Adding the policy as an `Option` on `TerminalLocalWorkReportInput` preserves existing default behavior. Existing callers that do not supply the policy continue to generate reports exactly as before.

## 4. Citation Construction Assessment

The integration correctly reuses `derive_approval_proof_marker_report_citations(...)` rather than duplicating event scanning or citation construction logic.

Approval decision citations are appended to the existing approval citation set. Because the Decisions Made section already combines policy and approval citations, proof-marker approval citations appear in both:

- `Approvals`;
- `Decisions Made`.

Workflow event citations are appended to the Work Performed citation path only when `include_workflow_event_citations` is true. This preserves the plan's default of approval citations first, event citations only by explicit request.

Citation ordering is deterministic: existing explicit approval references remain first, then proof-marker-derived approval decision citations are appended in event order.

## 5. Marker-Free And Failure Behavior Assessment

Marker-free approval decisions remain compatible when the policy is absent and when `require_proof_markers` is false.

When `require_proof_markers` is true and an approval decision lacks a proof marker, terminal report generation fails closed with:

```text
approval_proof_marker_citation.marker_missing
```

The focused test verifies that this failure does not mutate the borrowed run status or event history. No partial report artifact behavior was introduced.

## 6. Privacy And Redaction Assessment

The implementation does not copy approval-presentation payloads into reports. It also avoids copying:

- approval handoff text;
- work summaries;
- approved scope;
- strict non-goals;
- validation expectations;
- command output;
- provider payloads;
- source/spec contents;
- credentials, tokens, private keys, or secret-like values.

Citation summaries are fixed bounded strings. Tests assert that report debug output and serialization do not leak presentation IDs, presentation content hashes, approval request reasons, or approval decision reasons.

The stable approval reference itself remains a citation target when a citation is intentionally emitted. That is consistent with the existing WorkReport citation model.

## 7. Workflow Semantics Assessment

The phase does not change:

- approval grant or denial behavior;
- `WorkflowRun` status transitions;
- executor behavior;
- event append behavior;
- state persistence behavior;
- report artifact behavior;
- CLI behavior.

Report generation failure remains isolated to the report path. It does not retroactively alter the workflow run.

## 8. Test Quality Assessment

Focused tests cover:

- default terminal reports do not derive proof-marker citations;
- proof-marked granted approvals emit approval decision citations;
- proof-marked denied approvals emit approval decision citations;
- workflow event proof-marker citations are optional;
- marker-free approvals remain compatible when markers are not required;
- missing required markers fail closed with a stable non-leaking error;
- missing-marker failure does not mutate run status or event history;
- citation ordering is deterministic;
- proof-marker approval citations appear in Approvals and Decisions Made;
- approval-presentation payloads are not copied into debug or serialized report output.

The test coverage is strong for the approved integration boundary. Broader executor propagation tests are correctly deferred because executor propagation was intentionally out of scope.

## 9. Documentation Review

Documentation accurately states that terminal local reports can now opt in to proof-marker approval citations.

Documentation also continues to state that the following remain unimplemented:

- executor default proof-marker citation behavior;
- executor report-input propagation for the policy;
- audit projection persistence;
- report artifact gates requiring proof-marker citations;
- automatic report generation for every run;
- CLI rendering;
- workflow schema changes;
- examples;
- provider writes;
- hosted behavior;
- reasoning lineage;
- side-effect execution;
- release posture changes.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add explicit executor report input propagation for the proof-marker citation policy.
- Keep executor defaults marker-free until a separate policy/review phase changes them.
- Plan audit projection of proof-marker citation posture separately before adding persistence.
- Plan report artifact gates that require proof-marker citations only after executor propagation is reviewed.
- Consider workflow-declared proof-marker citation requirements in a later schema phase.

## 12. Validation

Validation for this review:

- passed: `cargo fmt --all --check`;
- passed: `cargo clippy --workspace --all-targets -- -D warnings`;
- passed: `cargo test --workspace`;
- passed: `npm run check:docs`;
- passed: `git diff --check`.

Governed review phase:

- workflow ID: `dg/review`;
- run ID: `run-1783618624014648000-2`;
- approval ID: `approval/run-1783618624014648000-2/review-scope-approved`;
- approval-presentation ID: `presentation/d585d41e0af6daba`;
- approval-presentation content hash: `d585d41e0af6daba4879b5264b1e3ee646dc42ea2f2a28d62a60e20db52c20a9`;
- approval outcome: granted.
- approval-presentation enforcement: `proof_enforced`;
- event summary: 39 events, 1 approval, 0 retries, 0 escalations, terminal `Completed`.

## 13. Recommended Next Phase

Recommended next phase: executor report input propagation planning for the proof-marker citation policy.

Why: terminal report generation now has the safe, explicit input boundary. The next step should decide how report-bearing executor APIs may pass that policy through without changing existing executor defaults, making proof-marker citation automatic, adding artifact gates, changing schemas, or broadening runtime behavior.
