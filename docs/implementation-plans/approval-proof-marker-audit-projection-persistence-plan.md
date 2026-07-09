# Approval Proof Marker Audit Projection Persistence Plan

Status: planning only.

## 1. Executive Summary

Approval decision proof markers are now modeled, wired into the opt-in approval-presentation decision path, exposed through bounded inspect/projection output, derivable as WorkReport citations, integrated into terminal report generation by explicit policy, and propagated through the local executor report input surface.

The next question is how future audit projection persistence should record proof-marker citation posture so operators can later verify whether reportable approval decisions were backed by presented approval context.

This plan defines that future persistence boundary only. It does not implement audit persistence, new audit event types, executor defaults, report artifact proof-marker gates, schemas, CLI rendering, examples, writes, hosted behavior, reasoning lineage, or release posture changes.

The guiding rule is:

```text
Persist bounded proof-marker posture; never persist approval-presentation payloads as audit projection content.
```

## 2. Goals

- Define how future audit projection persistence should represent approval proof-marker citation posture.
- Preserve approval decision events as the source of truth for whether a decision used a proof marker.
- Let future report artifact and audit review paths verify proof-marker posture by stable references.
- Keep marker-free approvals backward compatible unless an explicit caller policy requires proof markers.
- Avoid copying approval handoff text, approval-presentation records, command output, provider payloads, or source contents.
- Preserve current workflow pass/fail semantics.
- Prepare a small implementation prompt for a future in-memory projection helper before any durable persistence work.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- new audit persistence behavior;
- new dedicated audit sink records;
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

## 4. Current Implemented Surfaces

Implemented surfaces:

- `ApprovalPresentationRecord` model and local dogfood persistence.
- Opt-in approval-presentation enforcement.
- `ApprovalDecisionProofMarker` model.
- Proof-marker runtime wiring for opt-in approval-presentation grants and denials.
- Bounded inspect/projection exposure for proof markers.
- Pure in-memory approval proof-marker WorkReport citation derivation.
- Terminal report opt-in integration for proof-marker citation policy.
- Executor report input propagation for explicit proof-marker citation policy.

Not implemented:

- persisted audit projection records for proof-marker posture;
- dedicated proof-marker audit sink records;
- automatic proof-marker citation behavior in default executor paths;
- report artifact proof-marker gates;
- workflow-declared proof-marker report requirements;
- public approval cards;
- writes or provider calls.

## 5. Source Of Truth Boundary

The source of truth for proof-marker use is the approval decision workflow event.

A future persisted audit projection may cite:

- source workflow event ID for `ApprovalGranted` or `ApprovalDenied`;
- workflow ID, workflow version, schema version, run ID, and spec hash from the source event;
- approval ID from the approval decision payload;
- proof-marker posture from the decision payload;
- presentation ID and content hash only as bounded proof-marker identifiers where accepted by the proof-marker model;
- citation policy used by the report path, when the audit projection is derived for a report-bearing context.

A future persisted audit projection must not treat an `ApprovalPresentationRecord` alone as proof that a decision used the presentation. The presentation record proves that context was persisted. The approval decision proof marker proves that the approval decision used that context.

## 6. Candidate Persistence Posture Model

The first future persistence model should be posture-oriented, not payload-oriented.

Candidate fields:

| Field | Purpose |
| --- | --- |
| `projection_id` | Stable audit projection record ID if a dedicated persisted record is introduced. |
| `source_workflow_event_id` | Source approval decision event. |
| `workflow_id` | Workflow identity copied from the source event. |
| `workflow_run_id` | Run identity copied from the source event. |
| `approval_id` | Stable approval request/decision identity. |
| `decision` | Granted or denied vocabulary only. |
| `proof_marker_status` | `present`, `missing_required`, `not_required`, or `not_available`. |
| `presentation_id_present` | Boolean or bounded identifier posture, depending on the accepted implementation. |
| `presentation_content_hash_present` | Boolean or bounded hash posture, depending on the accepted implementation. |
| `validation_policy` | Bounded proof validation policy vocabulary. |
| `enforcement_mode` | Bounded proof enforcement mode vocabulary. |
| `citation_policy` | Report/audit citation policy used, when available. |
| `redaction` | Reference-only redaction metadata. |
| `sensitivity` | Conservative sensitivity classification. |

The implementation should prefer existing `AuditEvent` projection behavior if it can remain bounded and compatible. A dedicated record type should require a separate implementation review.

## 7. Projection Rules

Future projection should follow these rules:

1. Project only from accepted workflow event history or reviewed report-generation inputs.
2. Preserve source workflow/run/spec identity.
3. Preserve the approval decision event ID.
4. Store proof-marker posture as vocabulary, not copied approval context.
5. Store stable references only when they already exist.
6. Do not create `EvidenceReference` values implicitly.
7. Do not fabricate approval IDs, workflow event IDs, audit event IDs, report IDs, or proof marker IDs.
8. If proof markers are not required, marker-free approvals remain valid and should project as `not_required` or produce no proof-marker projection according to the accepted helper policy.
9. If proof markers are required and missing, projection should return a stable non-leaking error or explicit incomplete posture according to the accepted helper policy.
10. Projection failure must not retrospectively change workflow execution status.

## 8. Missing Proof And Failure Semantics

Missing proof markers have different meanings depending on caller policy.

- If no proof-marker policy was requested, missing markers are backward-compatible and not a workflow error.
- If proof markers were required for report/audit posture, missing markers should fail the projection or report path safely with a stable code such as `approval_proof_marker_audit_projection.marker_missing`.
- If a source approval event cannot be found, future helpers should fail with a stable code such as `approval_proof_marker_audit_projection.approval_event_missing`.
- If persistence fails after projection succeeds, the workflow run status must remain unchanged unless a later, explicit report artifact gate says otherwise.

Errors must not include raw approval IDs, presentation IDs, content hashes, approval handoff text, local paths, command output, provider payloads, source snippets, tokens, credentials, authorization headers, private keys, or secret-like values.

## 9. Privacy And Redaction

Future audit projection persistence must not store or copy:

- raw approval-presentation content;
- approval handoff blocks;
- work summaries, approved scopes, strict non-goals, validation expectations, or why-now text;
- chat transcripts;
- raw command output;
- raw provider payloads;
- raw CI logs;
- raw GitHub or Jira bodies;
- raw source or spec contents;
- environment variable values;
- credentials, tokens, authorization headers, or private keys;
- secret-like citation summaries or redaction metadata.

Projection records should mark proof-marker context as reference-only. Debug, serialization, deserialization, validation errors, audit output, and docs examples must remain redaction-safe.

## 10. Workflow Semantics

Audit projection persistence must not change workflow execution semantics.

It must not:

- change `LocalExecutor::execute(...)`;
- change approval request or approval decision behavior;
- append post-terminal workflow events unless separately scoped;
- mutate completed run state;
- create report artifacts;
- write provider data;
- emit CLI output by default;
- turn projection failures into workflow execution failures.

Future artifact or high-assurance phases may choose to fail an artifact write when required proof-marker posture is missing. That must be explicitly scoped and reviewed separately.

## 11. Relationship To WorkReports And Report Artifacts

WorkReports remain governed handoff artifacts. Audit projections remain operational accountability summaries.

Future WorkReport paths may cite:

- approval decision references;
- workflow event references;
- future persisted audit projection references, if a stable audit projection record exists.

Report artifacts should not depend on audit projection persistence until a separate proof-marker artifact gate is accepted. The likely later sequence is:

1. derive bounded proof-marker citation posture in memory;
2. optionally persist bounded audit projection posture;
3. validate report artifacts against persisted posture only in explicit artifact-capable paths;
4. keep default executor paths unchanged.

## 12. Test Plan

Future implementation tests should cover:

- proof-enforced approval decision projects bounded proof-marker posture;
- marker-free approvals remain valid when proof markers are not required;
- missing required proof marker fails safely or emits explicit incomplete posture according to accepted policy;
- projection preserves workflow ID, run ID, schema version, workflow version, spec hash, and approval decision event ID;
- projection does not copy approval-presentation payloads or handoff text;
- projection does not create `EvidenceReference` values;
- projection does not fabricate workflow event, audit event, report, or approval references;
- Debug and serialization do not leak presentation IDs, raw hashes where policy redacts them, command output, provider payloads, paths, or secret-like values;
- projection failure does not mutate workflow run state;
- persistence failure, if implemented later, does not change workflow pass/fail semantics;
- existing approval, WorkReport, audit projection, executor, dogfood, and report artifact tests continue to pass.

## 13. Proposed Implementation Sequence

1. Review this planning document.
2. Add a pure in-memory audit projection posture helper from explicit `WorkflowRun` and proof-marker citation policy input.
3. Add focused tests for proof-enforced, marker-free, missing-required, denied-decision, and non-leakage cases.
4. Review the helper.
5. Plan durable local audit projection persistence only after the pure helper is accepted.
6. Implement explicit persistence if still justified.
7. Plan report artifact proof-marker gates only after persistence posture is accepted.

The first implementation should be helper-only and in-memory. It should not write audit records, report artifacts, state files, schemas, examples, or CLI output.

## 14. Deferred Work

Deferred:

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

## 15. Final Recommendation

Proceed next to a maintainer review of this plan.

After review, the likely implementation should be a pure in-memory audit projection posture helper for approval proof-marker citation posture. Do not build persistence, artifact gates, default executor behavior, schemas, CLI rendering, examples, writes, hosted behavior, reasoning lineage, or release posture changes in that implementation phase.
