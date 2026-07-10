# Executor-Adjacent Approval Proof-Marker Projection Persistence Helper Report

## 1. Executive Summary

This phase implements the narrow executor-adjacent helper planned in [Executor-Adjacent Approval Proof-Marker Projection Persistence Plan](../implementation-plans/executor-adjacent-approval-proof-marker-projection-persistence-plan.md).

The new helper persists bounded approval proof-marker projection records from supplied `WorkflowRun` approval decision events into an explicit caller-supplied local projection store. It closes the manual hand-population gap for explicit artifact/proof-marker-gate paths without changing default executor behavior.

## 2. Scope Completed

- Added `persist_approval_proof_marker_projections_for_run(...)`.
- Added explicit input, policy, result, record-result, and disposition types for projection persistence.
- Derived projection persistence from approval decision workflow events only.
- Supported proof-marked granted approval decisions.
- Supported proof-marked denied approval decisions by default policy.
- Skipped marker-free decisions by default.
- Failed closed for marker-free selected approvals when the caller requires selected approvals to project.
- Treated matching duplicate records as already present.
- Failed closed for conflicting duplicate records.
- Wrote projection records only through `LocalApprovalProofMarkerAuditProjectionStore`.
- Preserved redaction-safe `Debug` output for new input and result types.
- Exported the helper and model types from `workflow-core`.

## 3. Scope Explicitly Not Completed

- No default executor projection persistence.
- No automatic projection persistence for all approvals.
- No automatic report artifact writing.
- No report generation changes.
- No CLI behavior.
- No workflow schema fields.
- No examples.
- No provider calls.
- No provider writes.
- No side-effect execution.
- No hosted or distributed runtime behavior.
- No reasoning lineage.
- No release posture changes.

## 4. Helper API Summary

The helper accepts:

- a supplied `WorkflowRun`;
- a supplied `LocalApprovalProofMarkerAuditProjectionStore`;
- `ApprovalProofMarkerProjectionPersistencePolicy`;
- optional selected `ApprovalReferenceId` values;
- sensitivity and redaction metadata.

It returns `ApprovalProofMarkerProjectionPersistenceResult`, including bounded counts for:

- newly persisted records;
- already-present matching records;
- marker-free decisions skipped by policy;
- denied decisions skipped by policy;
- approval decision events considered after selection filtering.

## 5. Source-Of-Truth Boundary

The helper uses the existing `derive_approval_proof_marker_audit_projection(...)` event-derived projection helper and persists only records derived from approval decision workflow events.

It does not use approval-presentation records alone, approval reasons, handoff prose, report text, CLI output, provider payloads, inferred approval IDs, or fabricated proof markers as a source of truth.

## 6. Persistence Behavior

The default policy persists proof-marked granted and denied approval decisions and skips marker-free decisions.

If selected approvals are required to project, marker-free selected approvals fail closed with:

```text
approval_proof_marker_projection_persistence.marker_missing
```

If no selected approval decision events are present, the helper fails with:

```text
approval_proof_marker_projection_persistence.no_approval_events
```

Matching duplicate records are reported as already present. Conflicting duplicate records fail closed with:

```text
approval_proof_marker_projection_persistence.duplicate_conflict
```

Store write/list failures are mapped to stable non-leaking persistence errors.

## 7. Workflow Semantics Summary

The helper does not mutate `WorkflowRun`, `WorkflowRunSnapshot`, event history, approval state, report state, artifact stores, or provider state.

Projection persistence failure does not change workflow execution status. The helper returns a structured error to the caller and leaves workflow semantics unchanged.

## 8. Redaction And Privacy Summary

The helper persists bounded projection posture only:

- source workflow event reference;
- approval reference;
- workflow/run/spec identity;
- decision kind;
- proof-marker-present posture;
- booleans indicating whether proof marker presentation ID and content hash were present;
- sensitivity and validated redaction metadata.

It does not persist approval-presentation payloads, handoff text, approval reasons, report text, command output, provider payloads, raw source/spec contents, environment values, credentials, tokens, authorization headers, private keys, or secret-like values.

New `Debug` implementations redact run, store, projection, event, and approval references.

## 9. Test Coverage Summary

Focused tests cover:

- proof-enforced granted approval persists one bounded projection record;
- proof-enforced denied approval persists under default policy;
- marker-free approval skips by default;
- marker-free selected approval fails closed when projection is required;
- matching duplicate record reports already-present posture;
- conflicting duplicate record fails closed;
- workflow event history remains unchanged;
- persisted records preserve workflow identity, run identity, source event, approval reference, decision kind, proof-marker posture, sensitivity, and redaction posture;
- debug output does not leak approval, presentation, or local test identity strings.

## 10. Commands Run And Results

- `cargo fmt --all`: passed.
- `cargo test -p workflow-core --test local_executor executor_adjacent_projection_persistence`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.

## 11. Governed Implementation Record

- Dogfood workflow: `dg/implement`.
- Run ID: `run-1783683972720623000-2`.
- Approval ID: `approval/run-1783683972720623000-2/implementation-approved`.
- Approval presentation ID: `presentation/3fb5147472be9a2e`.
- Approval presentation hash: `3fb5147472be9a2eba52deefc81f486b20720bd9e5cd79275967dade71125d30`.
- Approval outcome: granted by delegated maintainer.
- Approved scope: add an explicit local helper that persists bounded projection records from supplied `WorkflowRun` approval decision proof markers with focused tests and docs report.
- Strict non-goals: no default executor changes, no automatic projection persistence, no artifact writes, no CLI behavior, no schemas, no examples, and no provider writes.
- Phase-close status: `Completed`.
- Phase-close event summary: 39 total events, including one approval request, one approval grant, eight policy decisions, six step schedules, six skill invocation requests, six skill invocation starts, six skill invocation successes, one run resume, and one run completion.
- Approval-presentation enforcement: `proof_enforced`.

## 12. Remaining Known Limitations

- The helper is not called by default executor methods.
- Artifact-capable executor paths still need explicit composition to call this helper before proof-marker-gated artifact writes.
- Projection store configuration remains caller-supplied.
- Approval-resume artifact paths remain deferred.
- CLI rendering or operator commands for this helper remain deferred.

## 13. Recommended Next Phase

Recommended next phase: executor-adjacent projection persistence helper review.

Reason: this helper is security-sensitive because it persists the durable proof-marker posture that future report artifact gates can trust. It should be reviewed before composing it into broader artifact-capable executor flows.
