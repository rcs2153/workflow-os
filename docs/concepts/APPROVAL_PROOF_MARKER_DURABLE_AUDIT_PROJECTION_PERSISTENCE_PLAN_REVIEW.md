# Approval Proof Marker Durable Audit Projection Persistence Plan Review

## 1. Executive Verdict

Plan accepted with non-blocking follow-ups.

The plan defines a narrow, explicit, local-only persistence path for bounded approval proof-marker audit projection records. It preserves approval decision workflow events as the source of truth, keeps the accepted in-memory helper as the only derivation boundary, and correctly defers executor defaults, report artifact gates, dedicated audit sink records, schemas, CLI rendering, examples, writes, hosted behavior, reasoning lineage, and release posture changes.

Recommended next phase: approval proof-marker durable audit projection persistence helper implementation, helper-only and opt-in.

## 2. Scope Verification

The plan stayed within planning-only scope.

It does not authorize:

- persistence implementation in the planning phase;
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

No accidental broadening was found.

## 3. Persistence Boundary Assessment

The proposed boundary is appropriately conservative.

The plan recommends an explicit local persistence helper that accepts caller-supplied projection results and a caller-supplied storage root. It does not make persistence automatic, does not alter executor behavior, and does not infer projection posture from arbitrary inputs.

That separation is important: the accepted in-memory projection helper remains the derivation boundary, while a future store helper only validates and persists bounded records.

## 4. Source-Of-Truth Assessment

The plan correctly states that the approval decision workflow event remains the source of truth.

The durable projection record is positioned as a review and gating aid, not as a replacement for workflow events. The plan also correctly rejects treating an `ApprovalPresentationRecord` by itself as evidence that the approval decision used the presentation. The decision-time proof marker remains the meaningful proof.

This is the right model for auditability: persistence can make posture easier to inspect, but it must not manufacture authority or evidence after the approval decision has already happened.

## 5. Candidate Model Assessment

The candidate model is minimal and domain-appropriate.

Candidate types are bounded:

- `ApprovalProofMarkerAuditProjectionRecordId`;
- `ApprovalProofMarkerAuditProjectionStoreRecord`;
- `ApprovalProofMarkerAuditProjectionStoreInput`;
- `LocalApprovalProofMarkerAuditProjectionStore`;
- `ApprovalProofMarkerAuditProjectionStoreHealth`.

The proposed record fields capture stable identity and posture without copying approval-presentation payloads. The plan's recommendation to store `presentation_id_present` and `presentation_content_hash_present` booleans, rather than presentation IDs or content hashes, is conservative and consistent with the prior helper review.

The future implementation should keep these types no broader than necessary. A separate store record type is justified only if it keeps persisted shape and validation distinct from the pure helper result.

## 6. Storage Boundary Assessment

The proposed storage posture is safe for a first slice.

The plan requires:

- caller-supplied storage root;
- caller-supplied validated projection results;
- duplicate record rejection;
- deterministic safe file names;
- deterministic list/read behavior;
- bounded health summary;
- no workflow event writes;
- no `WorkflowRun` mutation;
- no report artifact writes;
- no provider calls.

The suggested layout is local and explicit:

```text
.workflow-os/audit-projections/approval-proof-markers/<encoded-record-id>.json
```

The plan correctly states that this layout is not yet a schema contract until implemented and reviewed.

## 7. Privacy And Redaction Assessment

The privacy posture is strong.

The plan forbids storing or copying:

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

It also requires non-leaking Debug output, serialization/deserialization errors, duplicate-write errors, identity mismatch errors, list/read errors, and health summaries.

One implementation detail to preserve: stable approval references and event IDs may still be sensitive in some deployments. The plan avoids raw values in errors, but implementation should also use redaction-safe Debug for any store record and health type.

## 8. Error-Handling Assessment

The proposed error namespace is specific and stable:

- `approval_proof_marker_audit_projection_store.duplicate`;
- `approval_proof_marker_audit_projection_store.invalid_record`;
- `approval_proof_marker_audit_projection_store.identity_mismatch`;
- `approval_proof_marker_audit_projection_store.read_failed`;
- `approval_proof_marker_audit_projection_store.write_failed`;
- `approval_proof_marker_audit_projection_store.corrupt_record`;
- `approval_proof_marker_audit_projection_store.unsafe_root`.

The plan correctly requires these errors to avoid raw approval IDs, event IDs, paths, presentation IDs, content hashes, handoff text, command output, provider payloads, source snippets, tokens, credentials, or secret-like values.

Future implementation should keep filesystem and serde errors wrapped behind these stable codes rather than exposing platform-specific details.

## 9. Relationship To Report Artifacts

The plan correctly defers report artifact proof-marker gates.

This is the right sequencing. A durable projection helper should make bounded records available for later review; it should not immediately become a report artifact write gate. The future gate can then be planned against a concrete persisted record shape.

No blocker found.

## 10. Relationship To Executor Defaults

The plan correctly defers executor defaults.

No default executor path should persist projection records automatically in the first persistence slice. Automatic persistence would affect runtime semantics and operator expectations, and it deserves a separate plan after the helper is accepted.

No blocker found.

## 11. Test Plan Assessment

The planned tests are appropriate and behavior-focused.

The test plan covers:

- deterministic write/read/list behavior;
- duplicate rejection;
- safe filename encoding;
- bounded health summary;
- corrupt record handling;
- identity mismatch handling;
- unsafe root rejection;
- marker-present and marker-free posture;
- no persisted presentation IDs or content hashes;
- no persisted approval handoff text or approval reasons;
- Debug and serialization non-leakage;
- no workflow run mutation;
- no event appends;
- no report artifact writes;
- existing approval, WorkReport, local executor, state, and audit projection regressions.

Implementation should add at least one filesystem-level assertion that writes are confined under the caller-supplied root and do not create sibling files through encoded IDs.

## 12. Documentation Review

The plan is clear that durable persistence is planned, not implemented.

It also keeps these items explicitly unimplemented:

- dedicated audit sink records;
- executor defaults;
- report artifact proof-marker gates;
- schemas;
- CLI rendering;
- examples;
- writes;
- hosted behavior;
- reasoning lineage;
- release posture changes.

Documentation remains honest about the current boundary.

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- In the implementation phase, add a filesystem confinement test for encoded record IDs.
- Keep store record Debug redaction stricter than serialization if stable event or approval IDs are serialized.
- Decide during implementation whether a separate store record type is necessary or whether a wrapper around the helper result is sufficient.
- Keep report artifact gates and executor defaults out of the helper implementation even if the helper makes them easier to build.

## 15. Validation Commands Run

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 16. Dogfood Governance

- workflow_id: `dg/review`
- run_id: `run-1783627243966246000-2`
- approval_id: `approval/run-1783627243966246000-2/review-scope-approved`
- presentation_id: `presentation/fd6ca331066ca863`
- approval_outcome: granted

## 17. Recommended Next Phase

Recommended next phase: approval proof-marker durable audit projection persistence helper implementation.

Reason: the plan is narrow, source-of-truth preserving, and implementation-ready. The next implementation should add only the explicit local helper/store surface, focused tests, docs updates, and an implementation report. It must not add executor defaults, report artifact gates, dedicated audit sink records, schemas, CLI behavior, examples, writes, hosted behavior, reasoning lineage, or release posture changes.
