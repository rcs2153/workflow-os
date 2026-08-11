# Decision-Time Authority Receipt Executor Report Propagation Planning Report

## 1. Executive Summary

The next trusted receipt boundary is planned as one additive, executor-adjacent
in-memory composition helper. It consumes the accepted proof-enforced approval
result, uses its exact run and trusted receipt to generate a WorkReport, and
returns owned run, receipt, and report posture.

No runtime implementation was added in this planning phase.

## 2. Scope Completed

- Inspected the trusted approval result and terminal report composition APIs.
- Defined a provenance-preserving executor-result propagation boundary.
- Defined ownership and non-self-referential result handling.
- Defined grant, denial, and report-failure semantics.
- Defined focused privacy, trust, no-mutation, and regression tests.
- Updated the roadmap.

## 3. Scope Explicitly Not Completed

No Rust implementation, executor default change, automatic report generation,
generic receipt injection, persistence, artifacts, events, audit projection,
schemas, CLI/UI behavior, providers, OpenShell changes, SideEffects, writes,
hosted behavior, defaults, or release changes were added.

## 4. Key Architecture Decision

The future helper should consume
`LocalCurrentRuntimeFactsGovernanceAuthorityReceiptDecisionResult` rather than
adding a receipt field to `LocalExecutionReportInputs`. The former carries
Core-owned provenance; the latter is public caller input and cannot prove that
a receipt came from the accepted approval path.

## 5. Proposed Composition Boundary

The helper consumes the trusted result, temporarily borrows its owned run and
receipt during report construction, then returns owned run, receipt, report,
and report-error posture. This avoids self-referential Rust types and retains
the original trusted values without reconstruction.

## 6. Semantics And Failure Posture

Report construction occurs only after an approval result exists. A report
failure cannot rewrite approval/resume semantics. Denial fabricates no receipt
or evidence. The helper performs no second assessment, source read, mutation,
event append, persistence, artifact write, provider call, or CLI output.

## 7. Privacy And Trust Posture

Only the accepted trusted receipt type enters the helper. Serialized claims and
generic citations cannot be promoted to provenance. Existing receipt-context,
citation, WorkReport, bounded-text, sensitivity, and redaction checks remain
the enforcement points, with stable non-leaking errors.

## 8. Validation Commands And Results

- Documentation checks: passed.
- Diff checks: passed.

No Rust checks are required because this phase changes documentation only.

## 9. Remaining Limitations

- Executor-result propagation is not implemented.
- No report is generated automatically.
- Receipts are not persisted or resolved by artifact gates.
- Denial has no receipt because it grants no resume authority.
- The phase runner does not yet rebuild when its binary is stale.

## 10. Recommended Next Phase

Implement the accepted additive in-memory composition helper and focused tests,
then perform a maintainer review before persistence or artifact integration.

## 11. Governed Planning Record

- Dogfood workflow: `dg/d`
- Run ID: `run-1786423286110143000-2`
- Approval ID: `approval/run-1786423286110143000-2/planning-approved`
- Presentation ID: `presentation/653349c3cc29c431`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: code-boundary inspection, documentation edits,
  validation, and git/PR work
