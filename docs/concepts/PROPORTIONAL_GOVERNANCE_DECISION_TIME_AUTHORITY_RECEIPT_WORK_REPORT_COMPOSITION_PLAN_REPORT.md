# Decision-Time Authority Receipt WorkReport Composition Planning Report

## 1. Executive Summary

The next authority-receipt reporting boundary is planned as one additive,
in-memory terminal report generator. It accepts the trusted receipt and derives
the citation inside the composition call rather than treating a generic public
citation as proof of provenance.

No runtime implementation was added in this planning phase.

## 2. Scope Completed

- Inspected the existing terminal report input, citation assembly, section
  assembly, and executor report input boundaries.
- Defined the smallest additive generator API.
- Defined trusted receipt/run identity validation.
- Defined decisions/approvals section placement.
- Defined focused privacy, failure, and non-regression tests.
- Updated the roadmap and prior derivation documents.

## 3. Scope Explicitly Not Completed

No Rust implementation, report behavior change, executor propagation,
persistence, artifacts, schemas, CLI/UI behavior, approvals, providers,
OpenShell changes, SideEffects, writes, hosted behavior, defaults, or release
changes were added.

## 4. Key Architecture Decision

The implementation should not accept a generic `WorkReportCitation` as trusted
input. That model proves shape, not provenance. The additive generator should
accept the trusted receipt and derive its citation inside the same call using
the report's sensitivity and redaction metadata.

This keeps existing report generators source-compatible and default-free while
preventing a caller from laundering an arbitrary typed receipt ID into a report
as trusted evidence.

## 5. Proposed Composition Boundary

One new input owns `TerminalLocalWorkReportInput` and borrows one trusted
receipt. One new free function validates identity, derives the citation, and
delegates to the existing report constructor path. Private citation assembly
gains a separate authority-receipt collection used by the decisions and
approvals sections.

## 6. Privacy And Failure Posture

Only the stable receipt ID may enter the report. Raw facts, presentation
content, commands, provider payloads, credentials, and reusable authority stay
outside the model. Identity mismatch, invalid receipt, and unsafe metadata fail
before a report is returned with stable non-leaking errors and no mutation.

## 7. Validation Commands And Results

- Documentation checks: passed.
- Diff checks: passed.

No Rust checks were required because this phase changed documentation only.

## 8. Remaining Limitations

- The additive generator is not implemented.
- No executor path supplies a receipt to report generation.
- Receipts are not persisted or resolved by artifact gates.
- No default report contains the receipt citation.
- The receipt remains evidence of a prior decision, not execution success.

## 9. Recommended Next Phase

Implement the accepted additive in-memory generator boundary only, followed by
a focused maintainer review. Keep executor propagation and persistence later.

## 10. Governed Planning Record

- Dogfood workflow: `dg/d`
- Run ID: `run-1786421135259266000-2`
- Approval ID: `approval/run-1786421135259266000-2/planning-approved`
- Presentation ID: `presentation/8cbe37349f26fd59`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: code-boundary inspection, documentation edits,
  validation, and git/PR work
