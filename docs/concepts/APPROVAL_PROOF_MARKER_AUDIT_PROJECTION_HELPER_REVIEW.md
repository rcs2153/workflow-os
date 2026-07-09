# Approval Proof Marker Audit Projection Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The helper implements the accepted first slice: a pure in-memory approval proof-marker audit projection posture helper. It is bounded, redaction-aware, covered by focused tests, and does not introduce durable audit persistence, dedicated sink records, executor defaults, report artifact gates, CLI behavior, schemas, writes, hosted behavior, reasoning lineage, or release posture changes.

Recommended next phase: durable audit projection persistence planning, if still needed after reviewing how the in-memory helper is consumed. Do not implement persistence without a separate accepted plan.

## 2. Scope Verification

The phase stayed within the approved helper-only scope.

Implemented:

- `derive_approval_proof_marker_audit_projection(...)`;
- `ApprovalProofMarkerAuditProjectionInput`;
- `ApprovalProofMarkerAuditProjectionResult`;
- `ApprovalProofMarkerAuditProjectionRecord`;
- `ApprovalProofMarkerAuditDecision`;
- `ApprovalProofMarkerAuditStatus`;
- exports from `workflow-core`;
- focused tests and documentation.

No accidental implementation found for:

- durable audit projection persistence;
- dedicated proof-marker audit sink records;
- executor default proof-marker behavior;
- automatic approval proof-marker enforcement;
- automatic report generation for every run;
- report artifact proof-marker gates;
- workflow schema changes;
- CLI rendering;
- examples;
- provider writes;
- side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 3. Helper API Assessment

The helper API is appropriately narrow.

It accepts explicit input:

- borrowed `WorkflowRun`;
- `require_proof_markers`;
- sensitivity;
- redaction metadata.

It returns an in-memory projection result with bounded records. It does not read hidden global state, require a `StateBackend`, write artifacts, emit audit events, create evidence references, or mutate runtime state.

The API is compatible with future persistence because it separates pure projection from storage. That keeps persistence reviewable as a later phase.

## 4. Projection Model Assessment

The projection model is bounded and domain-appropriate.

Each record captures:

- source workflow event ID;
- approval reference ID;
- granted/denied decision vocabulary;
- proof-marker status;
- whether a presentation ID was present;
- whether a presentation content hash was present;
- sensitivity;
- redaction metadata.

The helper intentionally does not copy approval-presentation payloads, approval handoff text, approval reasons, command output, provider payloads, source contents, or raw parser payloads.

The status vocabulary supports:

- `present`;
- `not_required`.

There is no `missing_required` record because required missing markers fail closed and no partial projection is emitted. That is acceptable for this helper phase.

## 5. Validation Boundary Assessment

Validation is deterministic and non-leaking.

Verified behavior:

- redaction metadata is validated through the existing WorkReport redaction boundary;
- invalid approval references map to `approval_proof_marker_audit_projection.reference_invalid`;
- missing required proof markers map to `approval_proof_marker_audit_projection.marker_missing`;
- marker-free approvals remain compatible when proof markers are not required;
- projection failure does not mutate run state or event history;
- errors avoid raw approval IDs, presentation IDs, content hashes, and payload text.

The helper does not retroactively change approval or workflow execution semantics.

## 6. Debug And Serialization Assessment

`Debug` is redaction-safe for the helper result and records:

- source workflow event IDs are redacted in debug output;
- approval reference IDs are redacted in debug output;
- presentation IDs and presentation content hashes are not present in debug output;
- only safe counts, booleans, decision vocabulary, status vocabulary, sensitivity, and redaction field count are shown.

Serialization includes stable source event and approval references because this is a projection model. It does not serialize presentation IDs, presentation content hashes, approval handoff text, approval reasons, or raw payload content.

This is acceptable for the in-memory helper. If the next phase persists records, persistence policy should re-review whether stable approval/event references need additional redaction or access-control posture.

## 7. Privacy And Redaction Assessment

No leakage path was found for:

- approval-presentation payloads;
- approval handoff blocks;
- command output;
- provider payloads;
- CI logs;
- GitHub or Jira bodies;
- source or spec contents;
- parser payloads;
- environment variable values;
- credentials, tokens, authorization headers, private keys, or secret-like values.

The helper uses existing validated redaction metadata behavior and does not bypass the WorkReport redaction guard.

## 8. Workflow Semantics Assessment

The helper preserves workflow semantics.

It does not:

- mutate `WorkflowRun`;
- append events;
- emit audit events;
- write audit records;
- touch a `StateBackend`;
- create report artifacts;
- call providers;
- emit CLI output;
- alter workflow pass/fail status.

Projection failure remains a helper error, not a workflow execution transition.

## 9. Test Quality Assessment

Tests are focused and meaningful.

Covered:

- granted approval with proof marker;
- denied approval with proof marker;
- marker-free compatibility when markers are not required;
- fail-closed missing marker when markers are required;
- source workflow event and approval reference identity preservation;
- decision/status vocabulary;
- presentation ID/hash presence booleans;
- non-leaking errors;
- debug non-leakage;
- serialization non-leakage for presentation payloads and forbidden raw markers;
- run state and event-history preservation on failure.

Existing WorkReport, approval-presentation, runtime, executor, adapter, validation, local check, side-effect, and provider-write tests passed in the full workspace run.

No blocker-level test gaps found.

## 10. Documentation Review

Documentation is honest about current state.

Verified docs state:

- the pure in-memory helper is implemented;
- durable audit projection persistence is not implemented;
- dedicated proof-marker audit sink records are not implemented;
- executor defaults are not implemented;
- report artifact proof-marker gates are not implemented;
- schemas are not implemented;
- CLI rendering is not implemented;
- examples are not updated;
- writes, hosted behavior, reasoning lineage, and release posture changes remain unsupported.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Before durable persistence, decide whether serialized stable approval/event references need additional sensitivity defaults, access controls, or redacted persistence shape.
- Consider whether a future persistence model should include explicit `missing_required` posture for review/audit views that need incomplete records instead of fail-closed helper errors.
- Keep executor default behavior deferred until a separate plan defines when proof-marker projection should be required automatically.

## 13. Validation Commands Run

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 14. Dogfood Governance

- workflow_id: `dg/review`
- run_id: `run-1783625536404440000-2`
- approval_id: `approval/run-1783625536404440000-2/review-scope-approved`
- presentation_id: `presentation/afdce45bc09c2544`
- approval_outcome: granted

## 15. Recommended Next Phase

Recommended next phase: durable audit projection persistence planning.

Reason: the pure helper is accepted and gives the project a bounded projection boundary. The next decision should be whether to persist that posture, how to shape persistence safely, and how to keep report artifact gates and executor defaults out of scope until separately accepted.
