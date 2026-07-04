# High-Assurance Approval Disclosure Executor/Report Integration Report

## 1. Executive Summary

This phase implements the first explicit in-memory executor/report bridge for high-assurance approval disclosure.

`LocalExecutor` now has an additive approval decision path that validates high-assurance approval controls, derives report-safe disclosure before approval decision events are appended, applies the approval decision, and returns the resulting `WorkflowRun` plus `WorkReportHighAssuranceApprovalDisclosure`.

The implementation remains opt-in and local. It does not change existing approval methods, generate reports automatically, write report artifacts, append disclosure events, add schemas, expose CLI behavior, implement workflow-declared controls, execute side effects, add writes, or change release posture.

## 2. Scope Completed

- Added `LocalHighAssuranceApprovalDecisionWithDisclosureResult`.
- Added `LocalExecutor::decide_approval_with_high_assurance_disclosure(...)`.
- Reused `prepare_approval_decision(...)`, `validate_high_assurance_approval_decision(...)`, and `apply_approval_decision(...)`.
- Derived disclosure from bounded validation posture with `discover_high_assurance_approval_disclosure(...)`.
- Failed closed when multiple controls have conflicting report-relevant posture.
- Returned report-safe disclosure for granted and denied high-assurance approval decisions.
- Added focused executor/report tests.
- Updated roadmap and implementation-plan status language.

## 3. Scope Explicitly Not Completed

- No change to `LocalExecutor::decide_approval(...)`.
- No change to `LocalExecutor::decide_approval_with_high_assurance(...)`.
- No automatic high-assurance disclosure for all approvals.
- No automatic report generation.
- No report artifacts or artifact gates.
- No workflow events or audit events for disclosure.
- No workflow-declared high-assurance controls.
- No runtime config or CLI behavior.
- No schemas or examples.
- No RBAC, IdP, quorum approval, role-bound approval authority, or revocation enforcement.
- No side-effect execution, provider mutations, write-capable adapters, hosted behavior, reasoning lineage, or release posture changes.

## 4. API Summary

The new result type is:

```rust
LocalHighAssuranceApprovalDecisionWithDisclosureResult
```

It exposes:

- `run(&self) -> &WorkflowRun`
- `high_assurance_approval(&self) -> &WorkReportHighAssuranceApprovalDisclosure`
- `into_parts(self) -> (WorkflowRun, WorkReportHighAssuranceApprovalDisclosure)`

The new executor method is:

```rust
LocalExecutor::decide_approval_with_high_assurance_disclosure(...)
```

It accepts the existing `LocalHighAssuranceApprovalDecisionRequest` to avoid adding a parallel request shape.

## 5. Workflow Semantics Summary

The method preserves the existing approval state machine:

- approval preparation still verifies the run is waiting for approval;
- high-assurance validation runs before decision events are appended;
- disclosure derivation also runs before decision events are appended;
- granted approval resumes the run exactly like the existing high-assurance path;
- denied approval fails closed exactly like the existing high-assurance path;
- disclosure is returned in memory and does not mutate runtime state.

If validation or disclosure derivation fails, no approval granted/denied event is appended.

## 6. Disclosure Derivation Summary

Disclosure input is derived only from bounded validation context:

- validation used/passed posture;
- approval decision kind;
- control count;
- requester/approver posture;
- required and supplied reference counts;
- expiration posture;
- revocation posture;
- denial behavior posture.

The bridge does not copy approval payloads, actor IDs, control IDs, evidence IDs, policy payloads, provider payloads, command output, paths, or raw text.

Multiple controls are supported only when their report-relevant posture is identical. Conflicting posture fails closed with:

```text
high_assurance_approval.disclosure_integration.control_posture_conflict
```

## 7. Redaction And Privacy Summary

The returned disclosure contains booleans, enums, and bounded counts only.

`LocalHighAssuranceApprovalDecisionWithDisclosureResult` has redaction-safe `Debug` output. It does not print run IDs, approval IDs, actor IDs, control IDs, evidence IDs, reference values, paths, free-form text, or provider payloads.

Errors use stable codes and bounded messages.

## 8. Test Coverage Summary

Added focused tests for:

- granted high-assurance approval returns a completed run plus disclosure;
- denied high-assurance approval returns a failed run plus disclosure;
- disclosure maps decision, requester/approver, expiration, revocation, and fail-closed posture;
- disclosure contains bounded required/supplied reference counts;
- disclosure can be passed into explicit report input and appears in the approvals section;
- conflicting multi-control posture fails before decision events are appended;
- new result `Debug` output does not leak run IDs, approval IDs, or evidence IDs.

Existing high-assurance validation failure tests continue to cover missing references and same-actor rejection before event mutation.

## 9. Commands Run And Results

- `cargo fmt --all`: passed.
- `cargo test -p workflow-core --test local_executor high_assurance`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

Dogfood governance:

- workflow: `dg/implement`;
- run ID: `run-1783058367772710000-2`;
- approval ID: `approval/run-1783058367772710000-2/implementation-approved`;
- approval outcome: granted;
- phase-close status: completed;
- event summary: 39 events, including one approval request, one approval grant, eight policy decisions, six step schedules, six skill invocation requests, six starts, six successes, one run resume, and one run completion.

## 10. Remaining Known Limitations

- Disclosure is returned in memory only.
- Disclosure is not persisted as a stable validation-result record.
- Existing report generation does not automatically discover high-assurance disclosure from approval events.
- Artifact gates do not yet require high-assurance approval disclosure.
- Workflow-declared high-assurance controls are not implemented.
- RBAC/IdP, quorum approval, role-bound approval authority, expiration timers, revocation enforcement, and write-capable adapters remain future work.

## 11. Recommended Next Phase

Recommended next phase: high-assurance approval disclosure executor/report integration review.

The new API is safety-sensitive because it composes approval enforcement with report disclosure. It should be reviewed before any artifact gate requires disclosure or any broader approval/report automation is added.
