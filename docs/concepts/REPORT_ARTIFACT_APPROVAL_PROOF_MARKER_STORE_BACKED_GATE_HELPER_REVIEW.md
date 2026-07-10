# Report Artifact Approval Proof Marker Store-Backed Gate Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation is appropriately narrow: it adds an explicit local store-backed validation helper for report artifact approval proof-marker coverage, reuses the reviewed in-memory gate semantics, and does not introduce artifact writes, executor default behavior, automatic projection persistence, CLI behavior, schemas, examples, provider writes, hosted behavior, reasoning lineage, or release posture changes.

Recommended next phase: artifact-write composition planning with store-backed proof-marker gate enforcement.

## 2. Scope Verification

The phase stayed within the approved validation-only helper scope.

No accidental implementation was found for:

- automatic report artifact writing;
- executor default proof-marker gate enforcement;
- automatic proof-marker projection persistence;
- workflow-declared proof-marker artifact requirements;
- CLI rendering or commands;
- schemas;
- examples;
- public approval cards;
- dedicated proof-marker audit sink records;
- new workflow event kinds;
- mutation of approval decision events;
- EvidenceReference creation;
- approval evidence attachment;
- provider writes;
- runtime side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 3. Helper API Assessment

The helper API is explicit and bounded.

The implemented input type, `WorkReportArtifactApprovalProofMarkerStoreGateInput`, borrows:

- a `WorkReportArtifactRecord`;
- a caller-supplied `LocalApprovalProofMarkerAuditProjectionStore`;
- an explicit `WorkReportArtifactApprovalProofMarkerGatePolicy`.

The function `validate_work_report_artifact_approval_proof_marker_gate_from_store(...)` validates the artifact, reads from the supplied local store, and delegates the actual approval citation/projection policy semantics to `validate_work_report_artifact_approval_proof_marker_gate(...)`.

This is minimal and idiomatic for the current codebase. Accepting `LocalApprovalProofMarkerAuditProjectionStore` directly is acceptable for the first concrete local helper because no generic projection-store trait exists yet.

## 4. Store Read Boundary Assessment

The helper reads only from the explicit store supplied by the caller. It does not infer a store root, discover hidden state, create stores, persist projection records, append events, mutate workflow state, call providers, or approve work.

The store read currently uses `list()` and then delegates filtering and immutable run identity validation to the reviewed in-memory gate. This matches the implementation plan's fallback path for the current store API. It is acceptable for this phase, with a non-blocking follow-up to add narrower query APIs if projection stores become large or if callers need stricter read scoping at the storage boundary.

## 5. Validation Assessment

Validation behavior is deterministic and fail-closed where required.

Verified behavior:

- invalid artifacts map to `work_report_artifact.approval_proof_marker_gate.invalid_artifact`;
- store read failures map to stable artifact gate errors;
- corrupt or invalid projection records map to `work_report_artifact.approval_proof_marker_gate.record_corrupt`;
- identity mismatches map to `work_report_artifact.approval_proof_marker_gate.identity_mismatch`;
- missing projections fail under strict policy;
- missing projections are counted only under explicit permissive policy;
- marker-free projections fail under strict policy;
- marker-free projections pass only under explicit marker-free policy;
- result exposure remains count-only.

The helper correctly avoids introducing new proof-marker policy semantics.

## 6. Error-Handling Assessment

The error boundary is stable and non-leaking.

Store errors are mapped through `map_approval_proof_marker_gate_store_error(...)` into artifact-gate-specific codes. Public error messages are bounded and do not include approval IDs, run IDs, projection IDs, report IDs, local paths, payload snippets, approval reasons, or command/provider output.

No partial gate result is emitted on failure. The helper returns a structured `WorkflowOsError` and leaves artifact writes to future explicit composition paths.

## 7. Privacy And Redaction Assessment

The privacy posture is appropriate for an audit-adjacent helper.

The helper does not copy or expose:

- approval-presentation payloads;
- approval handoff text;
- approval reasons;
- report text;
- command output;
- provider payloads;
- source/spec contents;
- local store paths;
- credentials, authorization headers, private keys, token-like values, or secret-like metadata.

`Debug` for `WorkReportArtifactApprovalProofMarkerStoreGateInput` redacts the artifact and projection store while exposing only policy posture. Existing gate result `Debug` remains count-only.

## 8. Relationship To Existing Gates And Artifact Writes

The helper composes cleanly with existing artifact gate foundations without changing them.

It does not alter:

- SideEffect referential integrity gates;
- approval-side-effect linkage gates;
- high-assurance disclosure gates;
- explicit artifact write helpers;
- executor report generation or result exposure behavior.

The implementation is a correct bridge from durable local proof-marker projections into the accepted in-memory artifact approval proof-marker gate. It is ready to be considered in a future explicit artifact-write composition plan.

## 9. Test Quality Assessment

The focused tests are strong for this phase.

Tests cover:

- successful strict validation from a persisted matching projection;
- missing required projection failure without leaking approval/run/store details;
- permissive missing projection counting;
- explicit marker-free policy behavior;
- corrupt store data mapping to a non-leaking gate error;
- bounded `Debug` output for store-backed input and result;
- existing workspace regression coverage through full validation.

Non-blocking test follow-ups:

- add a wrapper-level identity mismatch fixture if store query behavior becomes narrower than `list()`;
- add a direct store read failure fixture if the local store later exposes injectable read failures beyond corrupt data and unsafe roots;
- add a no-mutation regression around store contents when this helper is composed with artifact-write paths.

## 10. Documentation Review

Documentation accurately states that the store-backed helper is implemented and remains validation-only.

The docs continue to state that the following remain unimplemented:

- automatic report artifact writing;
- executor default proof-marker gate enforcement;
- automatic projection persistence;
- workflow-declared proof-marker artifact requirements;
- CLI behavior;
- schemas;
- examples;
- provider writes;
- hosted behavior;
- reasoning lineage;
- release posture changes.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Consider a generic approval proof-marker projection store trait when a non-local store exists.
- Consider narrower store query APIs by immutable run identity if projection stores grow beyond local helper scale.
- Add no-mutation regression tests when the helper is composed into artifact-write paths.
- Keep artifact write composition explicit and opt-in; do not make this gate automatic by default.

## 13. Recommended Next Phase

Recommended next phase: artifact-write composition planning with store-backed proof-marker gate enforcement.

The accepted helper now gives Workflow OS a durable local projection read boundary for proof-marker artifact gates. The next step should decide how an explicit artifact-capable path may compose SideEffect integrity, approval-linkage, high-assurance disclosure, and store-backed proof-marker gates before artifact writes without changing executor defaults, adding automatic report generation, adding CLI behavior, adding schemas/examples, or enabling provider writes.

## 14. Validation

Validation commands run for this review:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`
- `npm run dogfood:benchmark -- phase-close run-1783644539934364000-2 --phase review`

Dogfood governance summary:

- workflow: `dg/review`
- run: `run-1783644539934364000-2`
- approval: `approval/run-1783644539934364000-2/review-scope-approved`
- approval presentation: `presentation/fa91af79b9e37caf`
- approval outcome: granted
- approval-presentation enforcement: proof-enforced approval path used
- out-of-kernel work: repository edits, shell validation commands, git/PR actions, and GitHub merge operations are performed by Codex/human executor outside the kernel and disclosed here.
