# Approval Proof Marker Durable Audit Projection Persistence Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds the approved helper-only, explicit, local durable store for bounded approval proof-marker audit projection records. It preserves approval decision workflow events as the source of truth, keeps derivation outside the store, rejects duplicate records, uses safe encoded filenames, validates persisted records on read/list, and keeps errors and `Debug` output non-leaking.

Recommended next phase: report artifact proof-marker gate planning.

## 2. Scope Verification

The phase stayed within the approved helper-only scope.

Implemented:

- local projection record ID;
- local projection store record and definition;
- store input and bounded health summary;
- explicit local store helper with write/read/list/health;
- focused persistence, validation, redaction, and non-mutation tests;
- documentation and implementation report.

Not implemented:

- executor default persistence;
- automatic runtime persistence;
- report artifact proof-marker gates;
- dedicated proof-marker audit sink records;
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

No accidental scope broadening was found.

## 3. Model Assessment

The model is appropriately minimal.

`ApprovalProofMarkerAuditProjectionRecordId` provides a bounded, validated local identity. `ApprovalProofMarkerAuditProjectionStoreRecord` captures only stable references and bounded posture:

- source workflow event ID;
- approval reference ID;
- workflow identity and version;
- schema version;
- run ID;
- spec hash;
- granted/denied decision vocabulary;
- proof-marker posture;
- presentation ID/content-hash presence booleans;
- sensitivity;
- redaction metadata.

The implementation does not persist approval-presentation IDs, content hashes, handoff text, approval reasons, command output, provider payloads, source contents, or parser payloads. This matches the plan and keeps the stored record as posture evidence rather than approval-presentation payload storage.

## 4. Store Boundary Assessment

`LocalApprovalProofMarkerAuditProjectionStore` is explicit and caller-owned.

Verified behavior:

- `new(root)` validates a caller-supplied root and rejects parent-directory traversal.
- `write(...)` creates the store root and uses duplicate-safe `create_new` semantics.
- `read(...)` reads one record, validates identity, and revalidates redaction metadata.
- `list(...)` reads only `.json` records, revalidates redaction metadata, validates filename-to-record identity, and sorts deterministically by record ID.
- `health(...)` returns a count-only summary.

The helper does not derive projection posture. Callers must still use `derive_approval_proof_marker_audit_projection(...)` before persistence. This preserves the accepted derivation boundary.

## 5. Validation Assessment

Validation is deterministic and fail-closed.

Verified:

- empty, oversized, invalid-character, or secret-like projection record IDs fail through stable non-leaking errors;
- unsafe roots fail through `approval_proof_marker_audit_projection_store.unsafe_root`;
- duplicate writes fail through `approval_proof_marker_audit_projection_store.duplicate`;
- corrupt JSON fails through `approval_proof_marker_audit_projection_store.corrupt_record`;
- read/list redaction validation rejects tampered serialized metadata through `corrupt_record`;
- record/file identity mismatches fail through `identity_mismatch`;
- write serialization and filesystem failures are wrapped in stable store errors.

Errors do not include raw IDs, paths, approval text, presentation IDs, hashes, provider payloads, command output, or secret-like values.

## 6. Privacy And Redaction Assessment

The privacy posture is acceptable.

The store record serialization intentionally includes stable event/approval/workflow/run/spec references because those are the durable projection references. The implementation keeps `Debug` output stricter than serialization by redacting those identifiers and only exposing bounded posture fields and redaction field counts.

The implementation does not store:

- approval-presentation payloads;
- approval handoff blocks;
- work summaries, scopes, non-goals, validation expectations, or why-now text;
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

The added tampered-redaction test is important: it proves persisted JSON cannot silently reintroduce secret-like redaction metadata on read.

## 7. Workflow Semantics Assessment

The helper does not alter runtime semantics.

Verified:

- no executor default path writes projection records;
- no workflow event is appended by the helper;
- no `WorkflowRun` is mutated by the helper;
- no `StateBackend` integration is introduced;
- no report artifact is written;
- no provider call is made;
- no CLI output path is added.

Persistence failure remains an explicit helper failure for callers, not a retroactive workflow pass/fail decision.

## 8. Test Quality Assessment

Tests are behavior-focused and cover the important boundaries.

Covered:

- valid write/read round trip;
- duplicate write rejection without overwrite;
- encoded filenames for slash-containing IDs;
- deterministic list ordering;
- health count;
- unsafe root rejection;
- corrupt record non-leakage;
- identity mismatch non-leakage;
- no persisted presentation IDs/content hashes or forbidden payload markers;
- secret-like redaction rejection on construction;
- tampered serialized redaction rejection on read;
- no run state or event-history mutation;
- existing approval, WorkReport, executor, state, side-effect, adapter, and validation regressions through the workspace suite.

No blocking test gaps were found.

Non-blocking improvement: add a dedicated `list(...)` identity-mismatch fixture in addition to the existing `read(...)` mismatch fixture, since list now validates filename-to-record identity.

## 9. Documentation Review

Documentation is honest about implemented and deferred behavior.

Docs now state that durable local audit projection persistence is implemented as an explicit helper. They continue to state that these remain unimplemented:

- default public approval behavior changes;
- automatic approvals;
- executor default proof-marker citation behavior;
- report artifact proof-marker gates;
- public approval cards;
- schemas;
- examples;
- writes;
- hosted behavior;
- release posture changes.

The implementation report includes completed scope, non-scope, helper API summary, validation boundary, redaction posture, workflow semantics, tests, commands, dogfood governance, limitations, and recommended next phase.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add a focused `list(...)` identity-mismatch regression fixture.
- Consider a future convenience mapper from accepted in-memory projection results to store records, while keeping derivation explicit and caller-owned.
- Keep executor defaults deferred until a separate plan decides when automatic persistence is appropriate.
- Plan report artifact proof-marker gates against this accepted store shape.

## 12. Validation Commands Run

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 13. Dogfood Governance

- workflow_id: `dg/review`
- run_id: `run-1783629968451992000-2`
- approval_id: `approval/run-1783629968451992000-2/review-scope-approved`
- presentation_id: `presentation/7bdebd6167326fbc`
- approval_outcome: granted
- phase_close_status: Completed
- phase_close_events: 39 total events
- approval_presentation_enforcement: proof_enforced
- approval_presentation_content_hash: `7bdebd6167326fbcfbc71b8a5d7d347aab1869e63df0fd7ae5a75280d617da89`

## 14. Recommended Next Phase

Recommended next phase: report artifact proof-marker gate planning.

Reason: the durable helper is accepted and provides the bounded persisted posture needed for later gates. The next planning phase should decide how explicit report artifact paths may require persisted proof-marker projection records without changing executor defaults, adding CLI/schema behavior, or broadening into writes.
