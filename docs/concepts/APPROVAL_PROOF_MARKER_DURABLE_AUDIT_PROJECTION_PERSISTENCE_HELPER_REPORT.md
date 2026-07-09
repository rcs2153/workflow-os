# Approval Proof Marker Durable Audit Projection Persistence Helper Report

## 1. Executive Summary

The first durable local approval proof-marker audit projection persistence helper is implemented in `workflow-core`.

The implementation adds an explicit local store helper for caller-supplied, already-derived approval proof-marker audit projection posture. It persists bounded store records under a caller-supplied root with encoded filenames, duplicate rejection, deterministic read/list behavior, bounded health summaries, and redaction-safe errors.

This phase remains helper-only and opt-in. It does not add executor default persistence, report artifact proof-marker gates, dedicated audit sink records, workflow event changes, schemas, CLI behavior, examples, provider writes, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope Completed

Completed:

- Added `ApprovalProofMarkerAuditProjectionRecordId`.
- Added `ApprovalProofMarkerAuditProjectionStoreRecordDefinition`.
- Added `ApprovalProofMarkerAuditProjectionStoreRecord`.
- Added `ApprovalProofMarkerAuditProjectionStoreInput`.
- Added `ApprovalProofMarkerAuditProjectionStoreHealth`.
- Added `LocalApprovalProofMarkerAuditProjectionStore`.
- Exported the new types from `workflow-core`.
- Added focused persistence, duplicate, listing, non-leakage, and non-mutation tests.
- Updated roadmap/status documentation.

## 3. Scope Explicitly Not Completed

Not implemented:

- executor default persistence;
- report artifact proof-marker gates;
- dedicated audit sink records;
- new workflow event kinds;
- mutation of approval decision events;
- workflow schema changes;
- CLI rendering or commands;
- examples;
- provider writes;
- side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Helper API Summary

The helper is explicit and local:

- `LocalApprovalProofMarkerAuditProjectionStore::new(root)` validates a caller-supplied root.
- `write(...)` writes caller-supplied store records using duplicate-safe create semantics.
- `read(...)` reads one record by ID and validates identity.
- `list(...)` returns readable records in deterministic ID order.
- `health(...)` returns a bounded count-only summary.

Store records include:

- local projection record ID;
- source workflow event ID;
- approval reference ID;
- workflow ID/version;
- schema version;
- run ID;
- spec hash;
- granted/denied decision vocabulary;
- proof-marker posture;
- presentation ID/content-hash presence booleans;
- sensitivity;
- validated redaction metadata.

The helper does not derive projection posture. Callers must continue to use `derive_approval_proof_marker_audit_projection(...)` as the derivation boundary.

## 5. Validation Boundary Summary

Validation ensures:

- record IDs are bounded and non-secret-like;
- store roots are non-empty and do not contain parent-directory traversal;
- redaction metadata is validated through the existing WorkReport redaction guard;
- duplicate records are rejected without overwrite;
- encoded file names keep slash-containing record IDs confined to one file;
- record identity mismatches fail closed.

Stable error namespace:

- `approval_proof_marker_audit_projection_store.duplicate`;
- `approval_proof_marker_audit_projection_store.invalid_record`;
- `approval_proof_marker_audit_projection_store.identity_mismatch`;
- `approval_proof_marker_audit_projection_store.read_failed`;
- `approval_proof_marker_audit_projection_store.write_failed`;
- `approval_proof_marker_audit_projection_store.corrupt_record`;
- `approval_proof_marker_audit_projection_store.unsafe_root`.

## 6. Redaction And Privacy Summary

The helper does not store:

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

Serialized store records include stable event/approval/run/spec references because they are the durable projection references, but custom `Debug` redacts those identifiers. Store, read, duplicate, corrupt-record, identity-mismatch, and unsafe-root errors use stable non-leaking messages.

## 7. Workflow Semantics Summary

The helper is not wired into executor defaults.

It does not:

- mutate `WorkflowRun`;
- append workflow events;
- emit audit sink records;
- write report artifacts;
- call providers;
- touch a `StateBackend`;
- generate reports;
- change workflow pass/fail semantics.

Persistence failure remains a helper failure for explicit callers.

## 8. Test Coverage Summary

Focused tests cover:

- valid record write/read round trip;
- duplicate write rejection without overwrite;
- encoded file names for slash-containing IDs;
- deterministic list ordering;
- bounded health count;
- unsafe root rejection;
- corrupt record non-leaking error;
- identity mismatch non-leaking error;
- no persisted presentation ID/content hash or forbidden raw payload markers;
- secret-like redaction rejection without leakage;
- tampered serialized redaction rejection on read without leakage;
- no run state or event-history mutation.

The existing focused audit projection tests continue to cover marker-present, marker-free, denied, missing-required, debug, serialization, and non-mutation behavior.

## 9. Commands Run And Results

- `cargo test -p workflow-core --test work_report approval_proof_marker_audit_projection_store`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Dogfood Governance

- workflow_id: `dg/implement`
- run_id: `run-1783627870152845000-2`
- approval_id: `approval/run-1783627870152845000-2/implementation-approved`
- presentation_id: `presentation/a0f964427febc9f3`
- approval_outcome: granted
- phase_close_status: Completed
- phase_close_events: 39 total events
- approval_presentation_enforcement: proof_enforced
- approval_presentation_content_hash: `a0f964427febc9f34966ca56e90253510dcd7607d7248f5ad88426c4eb005676`

## 11. Remaining Known Limitations

- The helper is explicit and opt-in only.
- Executor default persistence remains unimplemented.
- Report artifact proof-marker gates remain unimplemented.
- Dedicated proof-marker audit sink records remain unimplemented.
- Workflow-declared proof-marker persistence requirements remain unimplemented.
- CLI rendering and schemas remain unimplemented.

## 12. Recommended Next Phase

Recommended next phase: approval proof-marker durable audit projection persistence helper review.

Reason: the helper adds a durable local persistence boundary for proof-marker posture. The next phase should verify scope, non-leakage, filesystem confinement, validation behavior, and whether report artifact gates or executor defaults should remain deferred.
