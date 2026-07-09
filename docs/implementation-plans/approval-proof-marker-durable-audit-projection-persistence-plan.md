# Approval Proof Marker Durable Audit Projection Persistence Plan

## 1. Executive Summary

Approval proof markers now have an accepted in-memory audit projection helper. The helper can derive bounded proof-marker posture from approval decision events without copying approval-presentation payloads or changing workflow semantics.

The next question is whether Workflow OS should persist that bounded posture locally so later report artifact gates, audit review, and operator inspection can verify whether approval decisions were backed by presented context.

This plan is planning only. It does not implement durable persistence, dedicated audit sink records, executor defaults, report artifact gates, schemas, CLI rendering, examples, provider writes, hosted behavior, reasoning lineage, or release posture changes.

Recommended v1 direction: add an explicit local persistence helper for bounded approval proof-marker audit projection records, driven only by caller-supplied projection results and caller-supplied storage root. Do not make executor paths persist these records by default.

## 2. Goals

- Persist bounded approval proof-marker audit projection posture locally.
- Reuse the accepted in-memory helper as the only derivation boundary.
- Preserve approval decision workflow events as the source of truth.
- Preserve marker-free compatibility unless an explicit caller policy requires proof markers.
- Store stable references and bounded posture vocabulary, not approval-presentation payloads.
- Keep persistence explicit, local, deterministic, and opt-in.
- Prepare future report artifact proof-marker gates without implementing them in the persistence phase.
- Preserve workflow execution semantics.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- executor default persistence;
- automatic runtime report generation;
- report artifact proof-marker gates;
- dedicated audit sink emission;
- new workflow event kinds;
- mutation of approval decision events;
- workflow schema changes;
- CLI rendering;
- examples;
- provider writes;
- side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Existing Accepted Foundation

Implemented and accepted:

- approval-presentation record model;
- local dogfood approval-presentation persistence;
- approval decision proof marker model;
- opt-in approval-presentation enforcement;
- bounded inspect/projection exposure for approval proof markers;
- pure in-memory WorkReport citation derivation helper;
- terminal report opt-in proof-marker citation integration;
- executor report input propagation for explicit proof-marker citation policy;
- pure in-memory audit projection posture helper.

The persistence phase must build on the in-memory helper rather than re-deriving projection posture from arbitrary inputs.

## 5. Source Of Truth Boundary

The approval decision workflow event remains the source of truth.

Persistent projection records may cite:

- source workflow event ID;
- approval reference ID;
- workflow ID;
- workflow version;
- schema version;
- run ID;
- spec hash;
- decision vocabulary: granted or denied;
- proof-marker status: present or not required;
- presentation ID presence boolean;
- presentation content hash presence boolean;
- projection sensitivity;
- validated redaction metadata.

Persistent projection records must not treat an `ApprovalPresentationRecord` by itself as evidence that approval used the presentation. The approval decision proof marker is the decision-time proof.

## 6. Candidate Persistence Model

The first persistence slice should introduce a local record type only if it is needed to keep storage validation separate from the pure helper result.

Candidate types:

- `ApprovalProofMarkerAuditProjectionRecordId`;
- `ApprovalProofMarkerAuditProjectionStoreRecord`;
- `ApprovalProofMarkerAuditProjectionStoreInput`;
- `LocalApprovalProofMarkerAuditProjectionStore`;
- `ApprovalProofMarkerAuditProjectionStoreHealth`.

Candidate record fields:

| Field | Purpose |
| --- | --- |
| `projection_record_id` | Stable local record identity. |
| `source_workflow_event_id` | Approval decision event reference. |
| `approval_reference_id` | Approval request/decision reference. |
| `workflow_id` | Workflow identity from source run. |
| `workflow_version` | Workflow version from source run. |
| `schema_version` | Schema version from source run. |
| `run_id` | Workflow run identity. |
| `spec_hash` | Source workflow spec hash. |
| `decision` | Granted/denied vocabulary only. |
| `proof_marker_status` | Present/not-required vocabulary only. |
| `presentation_id_present` | Boolean, not the presentation ID. |
| `presentation_content_hash_present` | Boolean, not the content hash. |
| `sensitivity` | Conservative sensitivity classification. |
| `redaction` | Validated redaction metadata. |

Do not persist approval-presentation IDs or content hashes in v1. The accepted helper exposes presence booleans, and that is sufficient for the first durable posture slice.

## 7. Storage Boundary

The first persistence helper should be explicit and caller-owned.

Rules:

- require a caller-supplied storage root;
- require caller-supplied validated projection results;
- reject duplicate record IDs;
- use deterministic safe file names;
- provide deterministic list/read behavior;
- provide a bounded health summary;
- write only projection records, never runtime events;
- never mutate `WorkflowRun`;
- never append workflow events;
- never emit audit sink records;
- never write report artifacts;
- never call providers.

Recommended local layout:

```text
.workflow-os/audit-projections/approval-proof-markers/<encoded-record-id>.json
```

This layout is proposed only for a later implementation. It must not be treated as a schema contract until implemented and reviewed.

## 8. Derivation And Persistence Flow

Future implementation should separate derivation from persistence:

1. Caller obtains a `WorkflowRun`.
2. Caller calls `derive_approval_proof_marker_audit_projection(...)`.
3. Caller maps each accepted projection record to a store record with explicit workflow/run/spec identity.
4. Caller writes records through a local store helper.
5. Store helper validates and rejects duplicates.

Persistence failure must not retroactively change workflow execution status.

## 9. Privacy And Redaction

The persistence phase must not store or copy:

- approval-presentation payloads;
- approval handoff blocks;
- work summaries, approved scopes, strict non-goals, validation expectations, or why-now text;
- approval reasons;
- presentation IDs;
- presentation content hashes;
- command output;
- provider payloads;
- CI logs;
- GitHub or Jira bodies;
- source or spec contents;
- parser payloads;
- environment variable values;
- credentials, tokens, authorization headers, private keys, or secret-like values.

Debug output, serialization, deserialization errors, duplicate-write errors, identity mismatch errors, list/read errors, and health summaries must remain non-leaking.

## 10. Error Handling

Recommended stable error namespace:

- `approval_proof_marker_audit_projection_store.duplicate`;
- `approval_proof_marker_audit_projection_store.invalid_record`;
- `approval_proof_marker_audit_projection_store.identity_mismatch`;
- `approval_proof_marker_audit_projection_store.read_failed`;
- `approval_proof_marker_audit_projection_store.write_failed`;
- `approval_proof_marker_audit_projection_store.corrupt_record`;
- `approval_proof_marker_audit_projection_store.unsafe_root`.

Errors must not include raw approval IDs, event IDs, paths, presentation IDs, content hashes, handoff text, command output, provider payloads, source snippets, tokens, credentials, or secret-like values.

## 11. Relationship To Report Artifacts

The persistence phase should not gate report artifacts.

Future report artifact proof-marker gates may later:

- require a persisted projection record for each report-cited approval decision;
- verify the persisted record references the same run/spec/report context;
- fail artifact write when required proof posture is missing.

That is a separate phase. Persistence should only make bounded records available for later review.

## 12. Relationship To Executor Defaults

The persistence phase should not change executor defaults.

No default executor path should automatically persist projection records until a later explicit opt-in or workflow-declared policy is planned and reviewed.

The first implementation should be callable as a helper from tests and future explicit paths only.

## 13. Test Plan

Future implementation tests should cover:

- valid projection records persist and read back deterministically;
- duplicate record writes are rejected without overwrite;
- record file names are encoded safely;
- list order is deterministic;
- health summary reports bounded counts;
- corrupt record read fails without leaking payload contents;
- identity mismatch fails without leaking IDs;
- unsafe store root is rejected;
- marker-present records persist booleans only;
- marker-free not-required records persist compatibility posture;
- presentation IDs and content hashes are not persisted;
- approval handoff text and approval reasons are not persisted;
- Debug output does not leak IDs, paths, redaction metadata, or secret-like values;
- serialization does not include forbidden raw payload markers;
- persistence does not mutate workflow run state;
- persistence does not append workflow events;
- persistence does not write report artifacts;
- existing approval, WorkReport, local executor, state, and audit projection tests continue to pass.

## 14. Proposed Implementation Sequence

1. Review this plan.
2. Add model types for a durable local projection store record only if needed.
3. Add local store helper for explicit writes/reads/lists under caller-supplied root.
4. Add focused store and non-leakage tests.
5. Update docs and create an implementation report.
6. Review the persistence helper.
7. Plan report artifact proof-marker gates separately.

## 15. Deferred Work

Deferred:

- executor default persistence;
- workflow-declared proof-marker persistence requirements;
- report artifact proof-marker gates;
- dedicated audit sink records;
- CLI rendering;
- schemas;
- examples;
- public approval cards;
- writes and provider mutations;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 16. Final Recommendation

Proceed next to maintainer review of this plan.

If accepted, the next implementation should be a local explicit persistence helper for bounded approval proof-marker audit projection records. It must remain helper-only and opt-in, and it must not add executor defaults, report artifact gates, schemas, CLI behavior, examples, writes, hosted behavior, reasoning lineage, or release posture changes.
