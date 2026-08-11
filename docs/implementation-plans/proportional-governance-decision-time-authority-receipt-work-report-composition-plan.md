# Decision-Time Authority Receipt WorkReport Composition Plan

Status: Planned and accepted; implementation is not included in this phase.
See the
[planning report](../concepts/PROPORTIONAL_GOVERNANCE_DECISION_TIME_AUTHORITY_RECEIPT_WORK_REPORT_COMPOSITION_PLAN_REPORT.md)
and [maintainer review](../concepts/PROPORTIONAL_GOVERNANCE_DECISION_TIME_AUTHORITY_RECEIPT_WORK_REPORT_COMPOSITION_PLAN_REVIEW.md).

## 1. Executive Summary

Workflow OS can produce a trusted, payload-free decision-time authority
receipt and derive a dedicated WorkReport citation from it. The next runtime
composition slice should place that citation into one terminal in-memory
WorkReport without weakening the trust boundary or changing existing report
generation defaults.

A generic `WorkReportCitation` is public vocabulary and can be constructed from
a typed ID without proving how that ID was obtained. The implementation must
therefore not accept an arbitrary prebuilt citation as trusted provenance. The
smallest safe boundary is one additive generator input that borrows the trusted
`GovernanceDecisionAuthorityReceipt` and owns the existing
`TerminalLocalWorkReportInput`. The generator derives and inserts the citation
inside the same call.

## 2. Goals

- Compose one trusted receipt citation into one in-memory terminal report.
- Preserve the trusted receipt as the source of citation provenance.
- Reuse the accepted derivation helper and existing report constructors.
- Use the report's sensitivity and redaction metadata for the citation.
- Place the citation in the decisions and approvals sections.
- Keep every existing report generator and executor result path unchanged.
- Preserve stable, non-leaking failure behavior.

## 3. Non-Goals

No generic citation trust promotion, unverified receipt-claim verification,
automatic report generation, executor default changes, persistence, receipt
stores, artifact writes, referential-integrity lookup, workflow events, audit
projection, schemas, SDK fields, CLI/UI behavior, approval semantic changes,
providers, OpenShell changes, SideEffect execution, writes, hosted behavior,
enterprise identity, cryptographic receipts, or release changes are included.

## 4. Proposed API

Add one explicit input, provisionally named
`TerminalLocalWorkReportAuthorityReceiptInput<'a>`:

- `report: TerminalLocalWorkReportInput<'a>`; and
- `authority_receipt: &'a GovernanceDecisionAuthorityReceipt`.

Add one free function, provisionally named
`generate_terminal_local_work_report_with_authority_receipt`.

The function should:

1. validate the trusted receipt;
2. derive the dedicated citation using the report input's sensitivity and
   redaction metadata;
3. call an internal report-generation path with that citation; and
4. return the same validated `WorkReport` model used by existing generators.

The public `generate_terminal_local_work_report` function remains unchanged
and supplies no authority-receipt citation. No executor input field or runtime
default is added in this phase.

## 5. Trust Boundary

The additive input must require the trusted receipt type produced by the Core
approval-resume path. It must not accept
`UnverifiedGovernanceDecisionAuthorityReceiptClaim` or treat an arbitrary
`WorkReportCitation` as proof of provenance.

Derivation and report composition occur in one call. The receipt and report run
identity must match before report construction. At minimum, the receipt's
workflow, workflow version, schema, spec hash, run, correlation, and approval
context must be coherent with the supplied terminal run where those identities
are available. Mismatch fails before a report is returned.

The receipt remains evidence-only and does not authorize resume, imply that
execution succeeded, or create reusable authority.

## 6. Section Placement

Add the citation to:

- `decisions_made`, because it identifies the evidence binding the fresh-fact
  authority decision; and
- `approvals`, because it explains the authority basis associated with the
  approval-resume boundary.

Do not add it to `evidence_considered` in the first slice. That section describes
evidence considered by the work, while this receipt is evidence about the
governance decision itself. Keep a separate internal citation collection so
approval-decision citations and authority-receipt citations remain distinct.

## 7. Validation And Failure Behavior

- Reject non-terminal report inputs through existing report validation.
- Reject invalid trusted receipts through receipt validation.
- Reject receipt/run identity mismatch with a stable static error code.
- Reject unsafe redaction metadata through existing citation/report gates.
- Return no partial report when derivation or composition fails.
- Do not mutate the run, append events, persist data, or write files.
- Do not include IDs, paths, facts, presentation content, commands, provider
  payloads, credentials, or token-like values in errors or Debug output.

## 8. Test Plan

- A proof-enforced granted approval produces a trusted receipt and terminal run.
- The additive generator returns a valid report with the receipt citation in
  decisions and approvals.
- Existing terminal report generation emits no receipt citation.
- The exact stable receipt ID is retained in serialization and redacted in
  Debug.
- Approval-decision and authority-receipt citations remain distinct.
- Receipt/run identity mismatch fails before returning a report and does not
  leak values.
- An unverified serialized claim cannot be passed to the typed input.
- Secret-like redaction metadata fails without leakage.
- Failure appends no events, mutates no run, writes no state, and creates no
  artifact.
- Existing WorkReport, executor, approval, and workspace tests remain green.

## 9. Implementation Sequence

1. Add the explicit input and additive generator wrapper.
2. Refactor only the private citation/section assembly needed to accept one
   optional trusted authority-receipt citation.
3. Add real proof-path composition and mismatch tests.
4. Run formatting, focused tests, workspace clippy/tests, docs, and diff checks.
5. Perform a focused maintainer review before any executor propagation.

## 10. Deferred Follow-Ups

After implementation and review, separately plan whether an explicit executor
result path should pass the successful receipt into this generator. Receipt
persistence, artifact referential integrity, CLI rendering, schemas, and
provider/write expansion remain later independent phases.

## 11. Governed Planning Record

- Dogfood workflow: `dg/d`
- Run ID: `run-1786421135259266000-2`
- Approval ID: `approval/run-1786421135259266000-2/planning-approved`
- Presentation ID: `presentation/8cbe37349f26fd59`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
