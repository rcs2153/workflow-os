# Decision-Time Authority Receipt WorkReport Citation Derivation Plan

Status: Implemented and accepted. See the
[implementation report](../concepts/PROPORTIONAL_GOVERNANCE_DECISION_TIME_AUTHORITY_RECEIPT_WORK_REPORT_CITATION_DERIVATION_REPORT.md)
and [maintainer review](../concepts/PROPORTIONAL_GOVERNANCE_DECISION_TIME_AUTHORITY_RECEIPT_WORK_REPORT_CITATION_DERIVATION_REVIEW.md).

## 1. Executive Summary

Workflow OS has trusted, payload-free decision-time authority receipts and
dedicated WorkReport citation vocabulary for their stable IDs. This phase adds
the smallest pure bridge between them.

The helper accepts only a trusted in-memory
`GovernanceDecisionAuthorityReceipt`, validates it, and constructs one
`WorkReportCitation` through the existing constructor. It does not authenticate
serialized claims, populate a report, persist anything, or grant authority.

## 2. Goals

- Derive one receipt citation from one trusted Core receipt.
- Reuse existing receipt and citation validation.
- Copy only the stable receipt ID.
- Require explicit sensitivity and redaction metadata.
- Keep Debug and error output non-leaking.
- Prove the bridge through the real proof-enforced approval-resume path.

## 3. Non-Goals

No unverified-claim derivation, report composition, persistence, workflow
events, audit projection, report artifacts, referential-integrity reads,
schemas, SDK fields, CLI/UI behavior, automatic approvals, provider execution,
OpenShell integration, SideEffect execution, writes, hosted behavior, default
changes, enterprise identity, or release changes are authorized.

## 4. API Boundary

Add `GovernanceDecisionAuthorityReceiptCitationInput` with:

- a borrowed trusted receipt;
- explicit `WorkReportSensitivity`; and
- explicit `RedactionMetadata`.

Add `derive_governance_decision_authority_receipt_report_citation`. The helper
validates the receipt and delegates citation construction to
`WorkReportCitation::new`. The citation has no summary and is not marked
missing.

## 5. Trust And Privacy

The input type cannot accept an
`UnverifiedGovernanceDecisionAuthorityReceiptClaim`. Trust is established by
the existing Core path before this helper is called. The wire-visible result
contains only the stable receipt ID and citation metadata. Debug redacts the
target, and invalid redaction metadata fails with stable non-leaking errors.

The receipt remains evidence of one prior decision. Deriving a citation does
not authorize resume, reuse authority, prove work completion, or validate a
report artifact.

## 6. Tests

- Produce a trusted receipt through the proof-enforced approval-resume path.
- Derive the dedicated citation and preserve the exact stable ID.
- Verify no summary and no missing posture.
- Verify serialization excludes receipt and execution payloads.
- Verify Debug excludes the stable ID and payload markers.
- Verify unsafe citation metadata fails closed without leakage.
- Run existing WorkReport, executor, and workspace suites.

## 7. Recommended Next Phase

The explicit in-memory WorkReport composition boundary is now planned and
accepted in the
[composition plan](proportional-governance-decision-time-authority-receipt-work-report-composition-plan.md).
It preserves trust by accepting the trusted receipt and deriving the citation
inside the same additive generator call rather than accepting an arbitrary
public `WorkReportCitation`. Persistence, artifacts, schemas, providers,
SideEffects, writes, automatic approvals, and default behavior remain out of
scope.

## 8. Governed Phase Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1786420139866384000-2`
- Approval ID: `approval/run-1786420139866384000-2/composition-approved`
- Presentation ID: `presentation/fb1335430d2e215e`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
