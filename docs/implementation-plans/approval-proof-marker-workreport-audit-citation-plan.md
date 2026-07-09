# Approval Proof Marker WorkReport And Audit Citation Plan

Status: planning only.

## 1. Executive Summary

Approval decision proof markers are now modeled, wired into the opt-in approval-presentation decision path, and exposed through bounded inspect/projection output. The next question is how future WorkReports and audit summaries should cite proof-enforced approval decisions.

This plan defines the citation and projection boundary only. It does not implement WorkReport citation generation, audit event mutation, report artifact writes, schema changes, examples, provider writes, hosted behavior, reasoning lineage, or release posture changes.

The guiding rule is:

```text
Reports and audit summaries cite proof-enforced approval decisions; they do not copy approval-presentation payloads.
```

## 2. Goals

- Let WorkReports cite approval decisions that used approval-presentation proof markers.
- Let future audit projections summarize proof-marker posture without copying approval-presentation records.
- Reuse existing `WorkReportCitationTarget::ApprovalDecision` and workflow event references where possible.
- Preserve the current default approval path.
- Keep marker-free approvals backward compatible.
- Keep citation summaries bounded, redacted, and optional.
- Make missing proof-marker citations explicit instead of fabricating evidence.
- Prepare for future high-assurance approval and write-capable adapter gates.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- new workflow schema fields;
- public approval-card UI;
- automatic approval enforcement;
- automatic report generation for every run;
- automatic report artifact writing;
- persistence changes;
- CLI rendering beyond separately scoped citation/projection behavior;
- provider writes;
- side-effect execution;
- examples;
- hosted or distributed runtime behavior;
- reasoning lineage implementation;
- release posture changes.

## 4. Current Boundary

Implemented:

- `ApprovalPresentationRecord` model and local persistence.
- Opt-in approval-presentation enforcement.
- `ApprovalDecisionProofMarker` model.
- Proof-marker runtime wiring for opt-in approval-presentation grants and denials.
- Bounded inspect/projection exposure for proof markers.
- `WorkReportCitationTarget::ApprovalDecision`.
- `WorkReportCitationTarget::WorkflowEvent`.
- `WorkReportCitationTarget::AuditEvent`.
- `WorkReportHighAssuranceApprovalDisclosure`.
- Explicit report input propagation for approval reference IDs and high-assurance approval posture.

Not implemented:

- automatic extraction of approval proof markers into reports;
- audit projection records dedicated to proof markers;
- report artifact citations to proof-marker details;
- workflow-declared proof-marker report requirements;
- approval-card UI;
- writes or provider calls.

## 5. Citation Source Of Truth

The approval decision event is the primary source of truth for proof-marker use.

Future citations should reference:

- approval decision reference ID, where available;
- workflow event ID for the `ApprovalGranted` or `ApprovalDenied` event;
- approval ID already present in the decision payload;
- presentation ID and presentation content hash only through bounded proof-marker fields;
- audit event ID only if a future audit projection emits a separate durable audit record.

Reports must not treat the approval-presentation proof record alone as proof that the approval decision used the proof. The record proves presentation was persisted; the approval decision proof marker proves the decision used it.

## 6. WorkReport Citation Policy

Preferred first implementation:

1. Continue using `WorkReportCitationTarget::ApprovalDecision` for approval decisions.
2. Add a bounded helper that derives report citations from a terminal `WorkflowRun` event history and explicit caller policy.
3. For proof-enforced decisions, cite the approval decision and optionally the workflow event ID.
4. Put proof-marker posture in a bounded citation summary such as `approval proof marker present`, not in raw payload text.
5. Do not create `EvidenceReference` values implicitly.

Candidate section placement:

- `Approvals`: approval request and decision citations.
- `Decisions Made`: proof-enforced approval decision citations when the approval affected downstream execution.
- `Evidence Considered`: only if the report already has explicit evidence references; do not convert approval presentation records into evidence automatically.
- `Risks` or `Known Limitations`: marker missing or marker not available, when the report policy expects proof-marker posture.

## 7. Audit Projection Policy

Audit projection should remain bounded.

Future audit projection may expose:

- event ID;
- approval ID;
- decision kind;
- proof-marker status: `present`, `not_available`, or `not_required`;
- enforcement mode;
- presentation ID;
- presentation content hash;
- proof validation policy;
- proof record sensitivity.

Audit projection must not copy:

- approval handoff text;
- work summary;
- approved scope;
- strict non-goals;
- validation expectations;
- why-now text;
- command output;
- provider payloads;
- chat transcripts;
- source/spec contents;
- tokens, credentials, authorization headers, private keys, or secret-like values.

## 8. Missing And Marker-Free Behavior

Missing markers must remain compatibility-safe.

Rules:

- Existing marker-free approval decisions remain valid.
- A missing marker should not imply policy failure unless the caller requested proof-marker enforcement.
- If a report policy requires proof-marker citations and a decision lacks a marker, report generation should fail safely or add an explicit missing/incomplete disclosure according to the accepted implementation policy.
- Missing citations must not create fake approval IDs, fake evidence references, or fake audit events.

## 9. Privacy And Redaction

All report and audit citation generation must use existing validated constructors.

Forbidden in WorkReports and audit projection:

- raw approval-presentation content;
- raw approval handoff blocks;
- raw local paths;
- raw command output;
- raw provider payloads;
- raw CI logs;
- raw GitHub/Jira bodies;
- raw source/spec contents;
- environment variable values;
- credentials or tokens;
- secret-like citation summaries.

Citation summaries should default to short posture text and stable IDs only.

## 10. Error Handling

Citation construction failures should be structured and non-leaking.

Recommended stable error codes:

- `approval_proof_marker_citation.approval_missing`;
- `approval_proof_marker_citation.marker_missing`;
- `approval_proof_marker_citation.reference_invalid`;
- `approval_proof_marker_citation.summary_invalid`;
- `approval_proof_marker_citation.audit_projection_unavailable`.

Errors must not include raw approval IDs, presentation IDs, content hashes, paths, payloads, snippets, command output, provider output, or secret-like values.

## 11. Test Plan

Future implementation tests should cover:

- proof-enforced approval decision produces a bounded WorkReport approval citation;
- marker-free default approval remains valid and produces no proof-marker citation unless required;
- missing required proof-marker citation fails safely or becomes explicit incomplete disclosure according to policy;
- citation uses `ApprovalDecision` and workflow event references without creating `EvidenceReference`;
- audit projection reports marker posture without copying presentation payloads;
- citation summaries are bounded and redaction-safe;
- Debug and serialization do not leak proof-marker or presentation payload values;
- old marker-free runs remain report-compatible;
- report generation behavior remains opt-in;
- existing approval, WorkReport, audit, executor, and dogfood tests continue to pass.

## 12. Proposed Implementation Sequence

1. Add pure helper planning/review for approval proof marker citation derivation.
2. Implement a pure in-memory citation derivation helper from explicit `WorkflowRun` input.
3. Add focused tests for proof-enforced, marker-free, missing-required, and non-leakage cases.
4. Review.
5. Add explicit terminal report helper/executor report input integration only after the helper is reviewed.
6. Plan audit projection separately if the helper proves useful.

## 13. Open Questions

- Should the first helper emit only `ApprovalDecision` citations or also workflow event citations?
- Should a proof-marker citation summary include presentation ID/hash, or should those remain inspect-only?
- Should high-assurance approval disclosure require proof-marker citation when marker data is available?
- Should audit projection be derived from workflow events on demand or persisted as a separate audit record?
- Should report artifact integrity validation eventually require proof-marker citations for write-capable adapter runs?

## 14. Final Recommendation

Proceed next with a pure in-memory approval proof marker citation derivation helper.

Do not build automatic report generation, artifact writes, CLI rendering, schemas, examples, provider writes, hosted behavior, reasoning lineage, or release posture changes in that implementation phase.
