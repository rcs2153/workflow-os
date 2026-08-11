# Decision-Time Authority Receipt WorkReport Citation Derivation Report

## 1. Executive Summary

Workflow OS can now derive a payload-free WorkReport citation from a trusted
in-memory decision-time governance authority receipt. The helper validates the
receipt, copies only its stable ID, and constructs the citation through the
existing privacy boundary.

This is a pure bridge, not report composition or authority enforcement.

## 2. Scope Completed

- Added an explicit borrowed receipt citation input.
- Added pure trusted-receipt-to-citation derivation.
- Reused receipt and `WorkReportCitation` validation.
- Added real approval-resume integration coverage and privacy regression tests.
- Updated the roadmap and citation documentation.

## 3. Scope Explicitly Not Completed

No unverified claim conversion, report population, persistence, events, audit
projection, artifacts, lookup gates, schemas, SDK fields, CLI/UI behavior,
automatic approvals, providers, OpenShell, SideEffects, writes, hosted
expansion, enterprise identity, defaults, or release changes were added.

## 4. API Summary

`GovernanceDecisionAuthorityReceiptCitationInput` contains a borrowed trusted
receipt plus explicit citation sensitivity and redaction metadata.
`derive_governance_decision_authority_receipt_report_citation` returns one
validated `WorkReportCitation` with the dedicated authority-receipt target, no
summary, and a present citation posture.

## 5. Trust And Validation Boundary

The helper accepts the trusted Core model, not its unverified serialized claim.
It invokes receipt validation before citation construction. Existing citation
validation bounds redaction metadata and sensitivity. Failure remains stable
and does not echo caller-supplied secret-like metadata.

## 6. Privacy Summary

Only the stable receipt ID is copied. Serialization excludes raw facts,
approval presentation content, commands, provider payloads, credentials, and
reusable authority. Debug redacts the target reference.

## 7. Test Coverage

The real proof-enforced approval-resume test now derives and inspects the
citation from its successful trusted receipt. It covers target identity,
summary/missing posture, serialization boundaries, Debug redaction, and
non-leaking rejection of unsafe redaction metadata.

## 8. Validation Commands And Results

- Rust formatting: passed.
- Focused trusted-receipt derivation test: passed.
- Workspace clippy: passed.
- Full workspace tests: passed.
- Documentation checks: passed.
- Diff checks: passed.

## 9. Remaining Limitations

- Report generators do not consume the derived citation.
- Receipts are not persisted or resolved by artifact gates.
- The citation does not authenticate serialized receipt claims.
- The receipt and citation do not prove resumed work completed successfully.
- No default runtime path emits the citation.

## 10. Recommended Next Phase

Plan an explicit in-memory report-composition input for the trusted citation.
Do not add persistence, artifacts, providers, SideEffects, writes, or defaults
as part of that phase.

## 11. Governed Phase Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1786420139866384000-2`
- Approval ID: `approval/run-1786420139866384000-2/composition-approved`
- Presentation ID: `presentation/fb1335430d2e215e`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: implementation, tests, documentation, validation, and
  git/PR work
