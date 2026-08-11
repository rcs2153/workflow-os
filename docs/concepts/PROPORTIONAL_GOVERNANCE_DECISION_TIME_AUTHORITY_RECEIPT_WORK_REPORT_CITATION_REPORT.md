# Decision-Time Authority Receipt WorkReport Citation Report

## 1. Executive Summary

Workflow OS WorkReports can now cite a decision-time governance authority
receipt by its validated stable ID. The citation is dedicated vocabulary, not
an approval citation, a copy of the receipt, or reusable authority.

The implementation is model-only. It does not derive citations, populate
reports, persist receipts, write artifacts, render output, or change runtime
approval and execution semantics.

## 2. Scope Completed

- Added `WorkReportCitationKind::GovernanceDecisionAuthorityReceipt`.
- Added a typed `GovernanceDecisionAuthorityReceipt` citation target.
- Added the stable local-check contract label for the new citation kind.
- Added focused construction, serde, invalid-wire, payload-exclusion, and Debug
  tests.
- Updated the roadmap and receipt documentation honestly.

## 3. Scope Explicitly Not Completed

No citation derivation, report generation integration, persistence, events,
audit projection, artifacts, referential-integrity reads, schemas, SDK fields,
CLI/UI behavior, automatic approvals, providers, OpenShell, SideEffect
execution, writes, hosted expansion, enterprise identity, defaults, or release
changes were added.

## 4. Contract Summary

The serialized target uses kind
`governance_decision_authority_receipt` and carries one
`GovernanceDecisionAuthorityReceiptId`. The dedicated kind prevents consumers
from interpreting the receipt as an approval decision or future authority.

## 5. Validation And Privacy

The typed ID enforces the receipt prefix and hash shape during deserialization.
Malformed IDs fail closed with a static error that does not echo the value.
Citation Debug output reveals the kind while redacting the stable ID and
bounded summary.

The wire shape contains the stable receipt ID because that is the reference.
It contains no raw facts, approval presentation content, commands, provider
payloads, credentials, tokens, or reusable authority.

## 6. Compatibility Summary

Existing WorkReports remain valid. No current report helper or executor path
automatically emits the new target. Local-check command contracts can refer to
the new citation kind through a stable canonical label without changing check
execution behavior.

## 7. Test Coverage

Focused tests prove typed construction, kind mapping, target retention, serde
round trip, stable wire naming, invalid-ID rejection, payload exclusion, and
Debug redaction. Existing workspace tests remain the compatibility boundary.

## 8. Validation Commands And Results

- Rust formatting: passed.
- Focused WorkReport citation tests: passed.
- Workspace clippy: passed.
- Full workspace tests: passed.
- Documentation checks: passed.
- Diff checks: passed.

## 9. Remaining Limitations

- No helper derives a citation from a trusted receipt.
- No generated report contains the citation automatically.
- Receipts are not persisted or resolved by report artifact gates.
- Serialized receipt claims remain unverified unless produced through the
  trusted Core path.
- The citation proves only that a report references receipt evidence; it does
  not authenticate the receipt or authorize another operation.

## 10. Recommended Next Phase

Add a pure, explicit, in-memory receipt-to-citation derivation helper and
review it before report-generation composition or persistence.

## 11. Governed Phase Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1786419072258487000-2`
- Approval ID: `approval/run-1786419072258487000-2/composition-approved`
- Presentation ID: `presentation/3239ff940fe15661`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Out-of-kernel work: implementation, tests, documentation, validation, and
  git/PR work
