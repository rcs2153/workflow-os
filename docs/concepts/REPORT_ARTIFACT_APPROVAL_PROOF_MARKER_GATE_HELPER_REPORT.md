# Report Artifact Approval Proof Marker Gate Helper Report

## 1. Executive Summary

The first report artifact approval proof-marker gate helper is implemented as a pure in-memory validation boundary.

The helper validates a `WorkReportArtifactRecord` against caller-supplied `ApprovalProofMarkerAuditProjectionStoreRecord` values and an explicit gate policy. It proves that approval decision citations in a report artifact have matching bounded proof-marker projection records before a future artifact write path chooses to persist the artifact.

This phase does not add store-backed gate integration, executor defaults, automatic artifact writing, CLI behavior, schemas, examples, provider writes, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope Completed

- Added `WorkReportArtifactApprovalProofMarkerGatePolicy`.
- Added `WorkReportArtifactApprovalProofMarkerGateInput`.
- Added `WorkReportArtifactApprovalProofMarkerGateResult`.
- Added `validate_work_report_artifact_approval_proof_marker_gate(...)`.
- Exported the helper API from `workflow-core`.
- Added private report citation collection for approval decision and workflow event citations.
- Added deterministic matching against caller-supplied approval proof-marker projection records.
- Added stable, non-leaking error codes for missing, ambiguous, mismatched, marker-free-disallowed, corrupt, and invalid-artifact paths.
- Added focused regression tests.
- Updated roadmap and implementation planning docs.

## 3. Scope Explicitly Not Completed

- No store-backed proof-marker gate integration.
- No executor default enforcement.
- No automatic proof-marker projection persistence.
- No automatic report artifact writing.
- No report artifact write from default executor paths.
- No CLI rendering or commands.
- No workflow schema fields.
- No examples.
- No public approval card rendering.
- No approval evidence attachment.
- No EvidenceReference creation.
- No provider writes.
- No runtime side-effect execution.
- No hosted or distributed runtime behavior.
- No reasoning lineage.
- No release posture changes.

## 4. Helper API Summary

The new helper is:

```rust
validate_work_report_artifact_approval_proof_marker_gate(
    WorkReportArtifactApprovalProofMarkerGateInput {
        artifact,
        projection_records,
        policy,
    },
)
```

The input borrows a validated report artifact and explicit projection records supplied by the caller. The helper does not read a projection store on its own.

The policy supports:

- requiring all approval citations to have projections;
- allowing or rejecting explicitly marker-free approval projections.

The result exposes counts only:

- unique approval citations;
- matched projection count;
- marker-present count;
- marker-free count;
- missing projection count;
- duplicate approval citation count.

## 5. Validation Boundary Summary

Validation checks:

- artifact validation succeeds before evaluation;
- approval citations are deduplicated by stable approval reference ID;
- duplicate approval citations are counted but do not create false failures;
- each required cited approval has exactly one matching projection record;
- supplied projection record redaction metadata remains valid;
- projection workflow ID, workflow version, schema version, run ID, and spec hash match the artifact generation context;
- projection source workflow event ID matches report workflow event citations when the report supplies workflow event citations;
- marker-free approval projections fail unless policy explicitly allows them.

The helper uses stable error codes under:

```text
work_report_artifact.approval_proof_marker_gate.*
```

Errors do not include approval IDs, event IDs, projection IDs, presentation IDs, hashes, report text, paths, command output, provider payloads, or secret-like values.

## 6. Matching Semantics

Matching uses stable references only:

- approval decision citation target to projection approval reference ID;
- report generation context to projection immutable run identity;
- optional workflow event citations to projection source workflow event ID.

The helper does not match on prose summaries, approval reasons, presentation payloads, actor names, command output, provider payloads, source contents, or chat transcripts.

## 7. Artifact And Write Semantics

The helper is validation-only. Passing the helper means:

```text
The report artifact approval citations matched caller-supplied proof-marker projection records according to the requested policy.
```

It does not write the artifact, persist projections, append workflow events, mutate workflow state, approve work, call providers, or execute side effects.

## 8. Privacy And Redaction Summary

The helper returns bounded counts and stable non-leaking errors.

`Debug` output for input redacts the artifact and exposes only projection count and policy. `Debug` output for result exposes counts only. Tests assert that approval IDs, projection IDs, report IDs, and run IDs are not leaked.

Projection record redaction metadata is validated through the existing report-safe redaction boundary.

## 9. Test Coverage Summary

Added focused tests for:

- matching proof-marker projection succeeds;
- duplicate approval citations are deduplicated and counted;
- missing projection is counted in permissive mode;
- missing required projection fails without leaking;
- identity mismatch fails without leaking;
- ambiguous duplicate projection records fail without leaking;
- marker-free projection policy is explicit;
- helper input/result `Debug` output is bounded and non-leaking.

Existing WorkReport artifact, proof-marker, side-effect integrity, executor, validation, and docs tests remain covered by the full validation suite.

## 10. Commands Run And Results

- `cargo fmt --all`
- `cargo test -p workflow-core --test work_report approval_proof_marker_gate` passed.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.
- `git diff --check` passed.
- `npm run dogfood:benchmark -- phase-close run-1783631855793190000-2 --phase implementation` completed with `status: Completed`, `events_total: 39`, and `approval_presentation_enforcement: proof_enforced`.

## 11. Remaining Known Limitations

- The helper does not read from `LocalApprovalProofMarkerAuditProjectionStore`.
- The helper is not integrated into executor artifact write paths.
- The helper is not workflow-declared.
- The helper does not make proof-marker citation automatic.
- Marker-free approvals are only evaluated when report artifacts actually cite an approval decision.
- No CLI surface exists for this gate.

## 12. Recommended Next Phase

Recommended next phase: report artifact approval proof-marker gate helper review.

The helper is safety-sensitive and write-adjacent. It should be reviewed before store-backed integration, executor artifact composition, workflow-declared proof-marker artifact requirements, automatic artifact writing, CLI behavior, schemas, examples, or provider writes.
