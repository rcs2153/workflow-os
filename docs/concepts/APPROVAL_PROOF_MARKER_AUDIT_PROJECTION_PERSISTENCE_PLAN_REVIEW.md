# Approval Proof Marker Audit Projection Persistence Plan Review

## 1. Executive Verdict

Plan accepted; proceed to approval proof-marker audit projection helper implementation, in-memory only.

The plan correctly keeps this phase as planning and does not authorize audit persistence, executor defaults, report artifact gates, schemas, CLI behavior, writes, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope Verification

The plan stayed within planning-only scope.

It did not accidentally authorize:

- implementation in the planning phase;
- durable audit projection persistence;
- dedicated proof-marker audit sink records;
- executor default proof-marker citation behavior;
- automatic approval proof-marker enforcement;
- automatic report generation for every run;
- report artifact proof-marker gates;
- report artifact writes;
- workflow schema changes;
- CLI rendering;
- examples;
- provider writes;
- side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage implementation;
- release posture changes.

## 3. Current Surface Assessment

The plan accurately describes the implemented approval proof-marker surfaces:

- approval-presentation records exist;
- opt-in approval-presentation enforcement exists;
- approval decision proof markers exist;
- proof-marker runtime wiring exists for opt-in approval grants and denials;
- bounded inspect/projection exposure exists;
- pure in-memory WorkReport citation derivation exists;
- terminal report opt-in integration exists;
- executor report input propagation exists.

It also accurately states that audit projection persistence, dedicated proof-marker audit records, executor default behavior, artifact gates, workflow-declared proof-marker requirements, public approval cards, and writes are not implemented.

## 4. Source-Of-Truth Assessment

The source-of-truth boundary is correct.

The approval decision workflow event is the right source of truth for whether a decision used approval-presentation proof. The plan correctly avoids treating an `ApprovalPresentationRecord` alone as proof that an approval decision used the presentation.

This matters because presentation persistence and approval decision use are related but distinct facts:

- presentation persistence proves the approval context was durably recorded;
- the decision proof marker proves that the approval decision used that recorded context.

## 5. Candidate Persistence Model Assessment

The candidate persistence posture model is appropriately bounded.

The proposed fields focus on identity, source event references, proof-marker posture, validation/enforcement vocabulary, redaction, and sensitivity. The plan does not propose storing approval handoff text, presentation payloads, command output, provider payloads, or source contents.

The plan also correctly says a future implementation should prefer existing generic `AuditEvent` projection behavior if the projection can remain bounded and compatible. A dedicated record type should require separate implementation review.

## 6. Projection Rules Assessment

The projection rules are conservative and aligned with existing audit projection posture.

The plan requires future projection to:

- derive from accepted workflow event history or reviewed report-generation inputs;
- preserve source workflow/run/spec identity;
- preserve approval decision event identity;
- store posture as vocabulary;
- store stable references only;
- avoid implicit `EvidenceReference` creation;
- avoid fabricated IDs;
- preserve marker-free compatibility when proof markers are not required;
- fail safely or disclose incompleteness when required proof markers are missing;
- avoid changing workflow execution status.

This is the right implementation boundary for the next helper phase.

## 7. Missing Proof And Failure Semantics Assessment

The missing-proof policy is clear enough for a first implementation.

The plan distinguishes between:

- marker-free approvals when no proof-marker policy was requested;
- missing markers when proof markers were explicitly required;
- missing source approval events;
- persistence failure after successful projection.

The recommended stable error codes are reasonable and non-leaking:

- `approval_proof_marker_audit_projection.marker_missing`;
- `approval_proof_marker_audit_projection.approval_event_missing`.

Future implementation may refine exact code names, but it should preserve stable, bounded, non-leaking errors.

## 8. Privacy And Redaction Assessment

Privacy posture is strong.

The plan explicitly forbids storing or copying:

- raw approval-presentation content;
- approval handoff blocks;
- work summaries, approved scopes, strict non-goals, validation expectations, or why-now text;
- chat transcripts;
- command output;
- provider payloads;
- CI logs;
- GitHub or Jira bodies;
- source or spec contents;
- environment variable values;
- credentials, tokens, authorization headers, or private keys;
- secret-like citation summaries or redaction metadata.

The requirement to mark proof-marker context as reference-only is consistent with existing audit projection patterns for hook and side-effect events.

## 9. Workflow Semantics Assessment

The plan preserves workflow semantics.

It does not authorize changes to:

- `LocalExecutor::execute(...)`;
- approval request behavior;
- approval decision behavior;
- post-terminal event append behavior;
- completed run state;
- report artifact writes;
- provider behavior;
- default CLI output;
- workflow pass/fail semantics.

The plan correctly reserves any future artifact failure behavior for a separately reviewed explicit artifact gate.

## 10. Relationship To WorkReports And Report Artifacts

The relationship is well separated.

The plan keeps WorkReports as governed handoff artifacts and audit projections as operational accountability summaries. It allows future WorkReports to cite persisted audit projection references only after such stable references exist.

The proposed sequence is correct:

1. derive bounded proof-marker posture in memory;
2. optionally persist bounded audit projection posture;
3. validate report artifacts against persisted posture only in explicit artifact-capable paths;
4. keep default executor paths unchanged.

## 11. Test Plan Assessment

The planned tests are adequate for the next implementation prompt.

They cover:

- proof-enforced approval projection;
- marker-free compatibility;
- missing-required behavior;
- identity preservation;
- non-copying of presentation payloads and handoff text;
- no implicit `EvidenceReference` creation;
- no fabricated references;
- Debug and serialization non-leakage;
- run-state preservation;
- persistence failure semantics if a later phase implements persistence;
- existing approval, WorkReport, audit projection, executor, dogfood, and report artifact regressions.

One useful non-blocking addition for the implementation prompt: include a denied approval decision case, because denied proof-marker decisions are supported by the runtime wiring and should remain report/audit-compatible.

## 12. Documentation Review

The roadmap and WorkReport planning index now state that audit projection persistence planning is documented while audit projection persistence remains unimplemented.

The plan is honest that it does not implement:

- durable audit projection persistence;
- dedicated proof-marker audit sink records;
- report artifact proof-marker gates;
- automatic executor proof-marker citation defaults;
- workflow-declared proof-marker requirements;
- public approval cards;
- CLI rendering;
- schemas;
- examples;
- writes and provider mutations;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 13. Dogfood Governance

Planning phase:

- workflow_id: `dg/d`
- run_id: `run-1783622801289008000-2`
- approval_id: `approval/run-1783622801289008000-2/planning-approved`
- presentation_id: `presentation/2d4ea4a11c2ed0dc`
- approval_outcome: granted
- phase_close: Completed; 39 events; proof_enforced

Review phase:

- workflow_id: `dg/review`
- run_id: `run-1783623549807496000-2`
- approval_id: `approval/run-1783623549807496000-2/review-scope-approved`
- presentation_id: `presentation/fb1a1ee7d58cc790`
- approval_outcome: granted

## 14. Validation Commands

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 15. Blockers

None.

## 16. Non-Blocking Follow-Ups

- Include denied approval decision coverage in the next implementation prompt.
- Decide during implementation whether the first helper should return a generic audit projection summary type or a proof-marker-specific posture type that remains in-memory only.
- Keep report artifact proof-marker gates deferred until after the in-memory helper is implemented and reviewed.

## 17. Recommended Next Phase

Recommended next phase: approval proof-marker audit projection helper implementation, in-memory only.

Reason: proof markers can now be modeled, opt-in enforced, inspected, cited in WorkReports, integrated into terminal reports, and propagated through executor report inputs. The next practical step is a pure helper that derives bounded audit projection posture from explicit run/caller inputs without adding persistence, executor defaults, artifact gates, schemas, CLI behavior, writes, hosted behavior, reasoning lineage, or release posture changes.
