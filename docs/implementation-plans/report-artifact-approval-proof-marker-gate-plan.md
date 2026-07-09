# Report Artifact Approval Proof Marker Gate Plan

Status: planning only. This plan follows the accepted durable local approval proof-marker audit projection persistence helper in [Approval Proof Marker Durable Audit Projection Persistence Helper Review](../concepts/APPROVAL_PROOF_MARKER_DURABLE_AUDIT_PROJECTION_PERSISTENCE_HELPER_REVIEW.md). It does not implement artifact gates, executor defaults, automatic artifact writing, CLI behavior, schemas, examples, writes, hosted behavior, reasoning lineage, or release posture changes.

## 1. Executive Summary

Workflow OS can now persist bounded local approval proof-marker audit projection records through an explicit helper. Those records summarize that an approval decision used the approval-presentation proof path without copying approval handoff text, presentation payloads, command output, provider payloads, source contents, or secret-like values.

The next artifact question is how future explicit report artifact write paths should require proof-marker posture before persisting a `WorkReport` artifact. This plan defines that gate only. It does not implement the gate, make report artifact writing automatic, change executor defaults, add workflow schema fields, add CLI behavior, or broaden into provider writes.

The guiding rule is:

```text
Report artifacts may require durable proof-marker projection records before write; they must not infer proof from prose or fabricate missing approval evidence.
```

## 2. Goals

- Define an explicit artifact-write gate for approval proof-marker posture.
- Use accepted proof-marker citation and durable projection boundaries.
- Preserve workflow execution, approval, report generation, and artifact-store semantics.
- Keep the base `WorkReportArtifactStore` as a storage boundary, not a governance-policy engine.
- Require proof-marker projection only when an explicit artifact path opts in.
- Validate report approval citations against durable projection records by stable references.
- Fail closed before artifact write when required projection proof is absent or mismatched.
- Avoid copying approval-presentation payloads into reports, artifacts, errors, or debug output.
- Keep errors stable, bounded, and non-leaking.
- Prepare a small implementation prompt for an opt-in helper/gate.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- automatic report generation;
- automatic report artifact writing from default executor paths;
- changing `LocalExecutor::execute(...)`;
- changing normal approval grant/denial semantics;
- changing `WorkReportArtifactStore` base write semantics;
- making proof-marker citation automatic for all reports;
- workflow-declared proof-marker artifact requirements;
- workflow schema changes;
- CLI rendering or commands;
- examples;
- public approval cards;
- dedicated proof-marker audit sink events;
- new workflow event kinds;
- mutation of approval decision events;
- EvidenceReference creation;
- approval evidence attachment;
- provider writes;
- runtime side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Current Foundation

Implemented foundation:

- approval-presentation proof records for dogfood approval gates;
- `ApprovalDecisionProofMarker` on approval decisions in the opt-in proof path;
- bounded inspect/projection for proof markers;
- pure WorkReport proof-marker citation derivation helper;
- terminal report opt-in proof-marker citation integration;
- executor report input propagation for proof-marker citation policy;
- pure proof-marker audit projection helper;
- explicit durable local proof-marker audit projection store helper;
- explicit local `WorkReport` artifact store;
- explicit governed artifact write paths with SideEffect and high-assurance disclosure gates.

Not implemented:

- report artifact proof-marker gates;
- automatic durable proof-marker projection persistence from executor paths;
- automatic report artifact writing;
- workflow-declared proof-marker artifact requirements;
- public approval-card UX;
- CLI report artifact inspection;
- schemas or examples for proof-marker artifact policy.

## 5. Source Of Truth Boundary

Approval decision workflow events remain the source of truth for whether a decision was granted or denied.

Approval proof-marker projection records are durable bounded projections derived from those approval decision events. They can support artifact gates only when they are linked back to stable source references.

Accepted gate sources:

- `WorkReport` approval citations created through reviewed proof-marker citation paths;
- persisted `ApprovalProofMarkerAuditProjectionStoreRecord` values;
- source approval workflow event IDs;
- approval reference IDs;
- run, workflow, workflow version, schema version, and spec hash identity fields;
- bounded proof-marker posture fields.

Rejected gate sources:

- report section prose;
- approval request or decision reason text;
- approval handoff block text;
- presentation record payloads;
- raw presentation IDs or content hashes as report text;
- policy string inference;
- actor name inference;
- command output;
- provider payloads;
- source/spec contents;
- chat transcripts.

## 6. Recommended First Gate Boundary

The first implementation should be an explicit helper used immediately before a governed artifact write, not a change to the base artifact store.

Recommended boundary:

1. Caller constructs a validated `WorkReportArtifactRecord`.
2. Caller supplies an explicit proof-marker gate policy.
3. Caller supplies durable projection records read from an explicit local projection store or another reviewed bounded source.
4. Helper validates that required report approval citations have matching projection records.
5. Helper returns a bounded gate result or a structured non-leaking error.
6. Caller writes the artifact only after the gate passes.

The helper should not:

- read the projection store by itself in the first implementation;
- generate or mutate reports;
- persist projection records;
- append workflow or audit events;
- write report artifacts;
- mutate workflow state;
- change approval decisions.

This mirrors the existing artifact-gate pattern: policy composition happens in governed write helpers, while storage remains a validated persistence boundary.

## 7. Candidate API Shape

The implementation may choose more idiomatic names, but the first model should stay small.

Candidate policy:

```rust
pub struct WorkReportArtifactApprovalProofMarkerGatePolicy {
    pub require_proof_marker_projection: bool,
    pub require_all_approval_citations_projected: bool,
    pub allow_marker_free_approvals: bool,
}
```

Candidate input:

```rust
pub struct WorkReportArtifactApprovalProofMarkerGateInput<'a> {
    pub artifact: &'a WorkReportArtifactRecord,
    pub projection_records: &'a [ApprovalProofMarkerAuditProjectionStoreRecord],
    pub policy: WorkReportArtifactApprovalProofMarkerGatePolicy,
}
```

Candidate result:

```rust
pub struct WorkReportArtifactApprovalProofMarkerGateResult {
    pub evaluated_approval_citation_count: usize,
    pub matched_projection_count: usize,
    pub marker_free_approval_count: usize,
    pub gate_passed: bool,
}
```

The first implementation should expose counts and booleans only. It should avoid returning raw approval IDs, presentation IDs, hashes, report text, or local paths in `Debug` output.

## 8. Matching Rules

The gate should match report approval citations to projection records using stable references.

Required matching dimensions when available:

- report run ID equals projection run ID;
- workflow ID equals projection workflow ID;
- workflow version equals projection workflow version;
- schema version equals projection schema version;
- spec hash equals projection spec hash;
- approval citation target equals projection approval reference ID;
- approval workflow event citation, if present, equals projection source workflow event ID;
- projection proof-marker posture satisfies the requested policy.

Rules:

- Do not match on report section text.
- Do not match on approval reason text.
- Do not accept a projection for a different run or workflow.
- Do not accept a projection whose identity mismatches the artifact metadata.
- Do not require projection for non-approval citations.
- Do not create missing citations or synthetic projection records.

## 9. Policy Semantics

Recommended first semantics:

- if `require_proof_marker_projection` is false, the gate is skipped and returns a passed/disabled posture;
- if it is true and the report has no approval citations, the gate returns a clear no-approval-citations posture rather than fabricating a failure reason;
- if `require_all_approval_citations_projected` is true, every approval citation must have a matching projection record;
- if marker-free approvals are allowed, the result must disclose how many approval citations were marker-free or not projected;
- if marker-free approvals are not allowed, missing projection for an approval citation fails closed;
- mismatched run/workflow/spec identity always fails closed;
- duplicate or ambiguous projection matches fail closed.

A future stricter policy may require proof markers only for high-assurance approval decisions, write-adjacent approvals, or workflow-declared proof-marker requirements. That is deferred.

## 10. Artifact Write Semantics

A successful proof-marker artifact gate should mean only:

```text
The report artifact's approval citations matched bounded durable proof-marker projection records according to the requested gate policy before artifact write.
```

It must not mean:

- every approval in the organization used proof markers;
- every approval in the run was high-assurance;
- identity provider or RBAC authority was verified;
- approval presentation payloads are stored in the artifact;
- the artifact is signed, notarized, or externally compliant;
- write-capable adapters are safe to enable.

If the gate fails:

- no artifact should be written by the governed write helper;
- the workflow run remains unchanged;
- the generated in-memory report remains unchanged;
- approval decisions remain unchanged;
- no workflow event or audit event is appended by this gate;
- the caller receives a stable non-leaking artifact gate error.

## 11. Error Handling

Recommended stable error codes:

- `work_report_artifact.approval_proof_marker_gate.missing_projection`;
- `work_report_artifact.approval_proof_marker_gate.identity_mismatch`;
- `work_report_artifact.approval_proof_marker_gate.ambiguous_projection`;
- `work_report_artifact.approval_proof_marker_gate.marker_required`;
- `work_report_artifact.approval_proof_marker_gate.reference_invalid`;
- `work_report_artifact.approval_proof_marker_gate.policy_invalid`.

Errors must not include:

- raw approval IDs;
- presentation IDs;
- presentation content hashes;
- approval reasons;
- approval handoff text;
- report section text;
- local filesystem paths;
- command output;
- provider payloads;
- source/spec contents;
- tokens, credentials, authorization headers, private keys, or secret-like values.

## 12. Privacy And Redaction

The gate must preserve the accepted proof-marker privacy boundary.

Do not copy into reports, artifact records, gate results, debug output, serialization, or errors:

- approval-presentation payloads;
- work summaries, approved scopes, strict non-goals, validation expectations, or why-now text;
- approval command strings;
- approval reasons;
- presentation IDs or content hashes unless already present only in the durable projection store's bounded internal model;
- command output;
- provider payloads;
- CI logs;
- GitHub or Jira bodies;
- source or spec contents;
- parser payloads;
- environment variable values;
- credentials, tokens, authorization headers, private keys, or secret-like values.

Gate results should expose posture and counts, not identifiers.

## 13. Relationship To Existing Gates

This proof-marker gate should compose with existing artifact gates without changing their semantics.

Ordering recommendation for explicit governed artifact write helpers:

1. validate artifact/report identity;
2. run SideEffect referential integrity gate when requested;
3. run approval-side-effect linkage gate when requested;
4. run high-assurance approval disclosure gate when requested;
5. run approval proof-marker projection gate when requested;
6. write artifact only if all requested gates pass.

The exact ordering may change if implementation reveals a safer local pattern. Any failure before write must leave no partial artifact.

## 14. Relationship To Executor Defaults

Executor defaults remain unchanged.

This plan does not make `LocalExecutor::execute(...)`, `execute_with_report(...)`, approval decisions, terminal report generation, or artifact writing automatically require proof-marker projection records.

The first implementation should be callable only from explicit artifact-capable paths or tests. A later phase may plan workflow-declared proof-marker artifact requirements after this opt-in helper is reviewed.

## 15. Relationship To Durable Projection Store

The durable projection store remains explicit and caller-owned.

The first gate should accept already-loaded projection records. It should not hide filesystem reads inside gate validation. This keeps:

- projection derivation explicit;
- projection persistence explicit;
- artifact gate validation deterministic;
- tests simple and side-effect free;
- future store-backed integration reviewable as a separate phase.

A later phase may add a store-backed convenience helper if the pure gate is accepted.

## 16. Test Plan

Future implementation tests should cover:

- disabled policy passes without projection records;
- report with proof-marker approval citation passes with matching projection record;
- missing required projection fails before artifact write;
- run ID mismatch fails closed;
- workflow ID mismatch fails closed;
- workflow version mismatch fails closed;
- schema version mismatch fails closed;
- spec hash mismatch fails closed;
- approval reference mismatch fails closed;
- workflow event reference mismatch fails closed when event citation is supplied;
- duplicate projection matches fail closed;
- marker-free approval citations are disclosed when allowed;
- marker-free approval citations fail when disallowed;
- gate failure writes no artifact;
- gate failure does not mutate workflow run or report;
- gate result `Debug` does not leak approval IDs, presentation IDs, hashes, reasons, or report text;
- serialization, if added, does not leak forbidden payload fields;
- existing artifact, approval, proof-marker projection, WorkReport, executor, and docs tests still pass.

## 17. Proposed Implementation Sequence

1. Implement a pure in-memory proof-marker artifact gate helper.
2. Add focused tests for matching, failure, non-leakage, and no-write behavior.
3. Review.
4. Plan explicit artifact-capable path integration only after the helper is accepted.
5. Defer store-backed convenience integration until pure matching semantics are stable.
6. Defer workflow-declared proof-marker artifact requirements until artifact gate behavior is reviewed.

## 18. Deferred Work

Deferred:

- artifact gate implementation in this planning phase;
- store-backed artifact gate integration;
- executor default enforcement;
- automatic proof-marker projection persistence;
- automatic report artifact writing;
- public approval card rendering;
- workflow-declared proof-marker artifact requirements;
- schemas;
- CLI commands;
- examples;
- provider writes;
- runtime side-effect execution;
- hosted behavior;
- reasoning lineage;
- signing/notarization;
- DLP/access control;
- release posture changes.

## 19. Final Recommendation

Proceed next to a pure in-memory report artifact approval proof-marker gate helper.

The helper should validate an explicit `WorkReportArtifactRecord` against caller-supplied durable proof-marker projection records and an explicit gate policy. It must not read stores, write artifacts, mutate runtime state, change executor defaults, create evidence, render CLI output, add schemas, update examples, enable writes, add hosted behavior, implement reasoning lineage, or change release posture.
