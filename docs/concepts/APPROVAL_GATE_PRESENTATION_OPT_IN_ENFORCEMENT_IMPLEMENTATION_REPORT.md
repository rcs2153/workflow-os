# Approval Gate Presentation Opt-In Enforcement Implementation Report

## 1. Executive Summary

The explicit opt-in approval-presentation enforcement path is implemented for
local executor approval decisions.

Workflow OS can now require a durable `ApprovalPresentationRecord` before an
approval decision is accepted through the new opt-in executor path. The path
validates that proof matches the pending `ApprovalRequest`, validates that the
presentation happened before the approval decision, optionally enforces a
freshness window, and fails closed before appending approval events when proof
is missing, mismatched, ambiguous, corrupt, future-dated, or stale.

Default approval behavior is unchanged.

## 2. Scope Completed

- Added an explicit approval decision request wrapper for presentation-proof
  enforcement.
- Added proof resolution by explicit presentation ID.
- Added proof resolution by run ID and approval ID when unambiguous.
- Added `LocalExecutor::decide_approval_with_presentation(...)`.
- Validated proof before approval decision events are appended.
- Added optional max-age freshness checks.
- Added stable non-leaking enforcement errors.
- Extended the aggregate local state backend contract to include approval
  presentation records.
- Added focused executor tests.
- Updated roadmap and planning docs.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- default approval behavior changes;
- automatic approvals;
- hidden approvals;
- dogfood runner persistence of approval handoff proof;
- CLI approval-card rendering;
- workflow schema fields;
- examples;
- high-assurance approval integration;
- WorkReport citation changes;
- provider writes;
- side effects;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. API Summary

New local executor API:

- `LocalApprovalPresentationProof`
- `LocalApprovalPresentationDecisionRequest`
- `LocalExecutor::decide_approval_with_presentation(...)`

The API wraps an existing `LocalApprovalDecisionRequest`, adds durable proof
resolution, and optionally supplies a max presentation age. It reuses the
existing approval preparation and application path so successful approvals keep
the same runtime semantics as `decide_approval(...)`.

## 5. Enforcement Behavior

The opt-in path:

- loads the pending approval through existing runtime state;
- loads exactly one approval-presentation record by explicit presentation ID, or
  by unambiguous run/approval lookup;
- validates the record against the pending approval request;
- rejects proof presented after the approval decision timestamp;
- rejects stale proof when a caller supplies a max age;
- only applies the approval decision after all proof checks pass.

Proof validation failure occurs before approval decision events, resume events,
skill invocation, side effects, report artifacts, or other runtime mutations.

## 6. Error Handling

Stable enforcement error codes include:

- `approval_presentation_enforcement.proof_missing`
- `approval_presentation_enforcement.proof_ambiguous`
- `approval_presentation_enforcement.proof_mismatch`
- `approval_presentation_enforcement.proof_stale`
- `approval_presentation_enforcement.proof_corrupt`
- `approval_presentation_enforcement.decision_time_invalid`

Errors are bounded and do not include raw approval IDs, presentation IDs, run
IDs, handoff text, actor values, filesystem paths, corrupt payloads, command
output, provider payloads, tokens, or secret-like metadata.

## 7. Test Coverage Summary

Focused tests cover:

- approval grant succeeds with matching presentation proof;
- approval denial succeeds with matching presentation proof and still fails the
  run closed;
- missing proof fails before approval events are appended;
- mismatched proof fails closed;
- ambiguous run/approval proof lookup fails closed;
- future-dated proof fails closed;
- stale proof fails closed when max age is supplied;
- request Debug output does not leak approval reason, approval ID, or
  presentation ID.

Existing default approval tests remain unchanged and continue to exercise
`LocalExecutor::decide_approval(...)`.

## 8. Commands Run And Results

- `cargo fmt --all --check` - passed.
- `cargo test -p workflow-core --test local_executor presentation` - passed.
- `cargo test -p workflow-core --test approval_presentation` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 9. Remaining Known Limitations

- The dogfood runner emits approval handoffs but does not yet persist them as
  `ApprovalPresentationRecord` values.
- Existing `decide_approval(...)` remains intentionally unchanged and does not
  require presentation proof.
- No CLI approval-card surface exists.
- No WorkReport citation model for approval-presentation proof exists.
- High-assurance approval controls are not yet composed with
  approval-presentation proof.

## 10. Recommended Next Phase

Recommended next phase: approval-presentation opt-in enforcement review.

The implementation is security-sensitive because it affects approval gates.
Maintainer review should verify scope, fail-closed behavior, error non-leakage,
default approval compatibility, and test coverage before dogfood runner
integration begins.
