# Decision-Time Authority Receipt WorkReport Citation Plan

Status: Implemented and accepted as model-only citation vocabulary. See the
[implementation report](../concepts/PROPORTIONAL_GOVERNANCE_DECISION_TIME_AUTHORITY_RECEIPT_WORK_REPORT_CITATION_REPORT.md)
and [maintainer review](../concepts/PROPORTIONAL_GOVERNANCE_DECISION_TIME_AUTHORITY_RECEIPT_WORK_REPORT_CITATION_REVIEW.md).

## 1. Executive Summary

The accepted decision-time governance authority receipt provides bounded,
point-in-time evidence for one exact proof-enforced approval-resume decision.
WorkReports need stable vocabulary for citing that evidence without copying the
receipt, raw facts, approval presentation, or execution payloads.

This phase added a dedicated `WorkReportCitationKind` and
`WorkReportCitationTarget` carrying only a validated
`GovernanceDecisionAuthorityReceiptId`. It does not derive citations, populate
reports, persist receipts, or change approval and execution behavior.

Pure derivation from the trusted in-memory receipt is now implemented and
accepted separately in the
[citation derivation plan](proportional-governance-decision-time-authority-receipt-work-report-citation-derivation-plan.md).
This model-only phase itself remains unchanged.

## 2. Goals

- Represent a receipt citation using the existing typed receipt ID.
- Keep the citation distinct from approval-decision and policy-decision
  citations.
- Preserve validated serde and redaction-safe Debug behavior.
- Keep local-check command-contract fingerprint vocabulary exhaustive.
- Fail closed when a serialized citation contains an invalid receipt ID.

## 3. Non-Goals

This phase does not add citation derivation, report generation changes,
receipt persistence, workflow events, audit projection, report artifacts,
referential-integrity lookup, schemas, SDK fields, CLI/UI rendering, automatic
approval, provider execution, OpenShell integration, SideEffect execution,
writes, hosted behavior, enterprise identity, default changes, or release
changes.

## 4. Model Boundary

Add:

- `WorkReportCitationKind::GovernanceDecisionAuthorityReceipt`; and
- `WorkReportCitationTarget::GovernanceDecisionAuthorityReceipt { receipt_id }`.

The target cites a stable ID only. It does not embed an authority receipt and
does not promote an unverified serialized receipt claim to trusted evidence.
The receipt remains point-in-time evidence and never reusable authority.

## 5. Validation And Privacy

The existing `GovernanceDecisionAuthorityReceiptId` deserializer validates the
prefix and deterministic hash shape. Invalid wire values fail closed with a
static non-leaking error. Existing `WorkReportCitation` validation continues to
bound summaries and redaction metadata. Debug output exposes the citation kind
but redacts the target reference and summary.

Serialization necessarily contains the stable receipt ID so a future consumer
can resolve it. It contains no raw fact, presentation, command, provider,
credential, token, or reusable-authority payload.

## 6. Compatibility

The new citation kind is additive vocabulary. The local-check command-contract
fingerprint mapping receives the matching stable snake-case label so contracts
can explicitly require this citation kind without an incomplete match.

No existing report generator or runtime path emits this citation. Existing
serialized reports remain valid.

## 7. Test Plan

- Construct a validated citation from a typed receipt ID.
- Verify the dedicated citation kind and target.
- Verify serde round trip and stable wire naming.
- Verify serialized output contains only the stable ID, not receipt payloads.
- Verify Debug redacts the receipt ID and summary.
- Verify invalid serialized IDs fail closed without echoing the value.
- Run existing WorkReport and workspace suites.

## 8. Recommended Next Phase

The separate pure in-memory receipt-to-citation derivation phase is complete
and accepted. The next phase may plan one explicit report-composition input,
but persistence, artifact lookup, and broader approval or provider behavior
remain separate boundaries.

## 9. Governed Phase Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1786419072258487000-2`
- Approval ID: `approval/run-1786419072258487000-2/composition-approved`
- Presentation ID: `presentation/3239ff940fe15661`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
