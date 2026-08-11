# Decision-Time Authority Receipt WorkReport Composition Report

## 1. Executive Summary

Workflow OS can now compose one trusted decision-time governance authority
receipt into one validated in-memory terminal WorkReport. The additive API
requires the opaque trusted receipt, proves its exact terminal run and granted
approval-event context, derives the payload-free citation internally, and
places it in the decisions and approvals sections.

Existing report generators remain unchanged and emit no receipt citation. No
executor propagation, persistence, artifact write, schema, CLI/UI behavior,
provider execution, OpenShell change, SideEffect execution, write, hosted
expansion, default change, or release change was added.

## 2. Scope Completed

- Added `TerminalLocalWorkReportAuthorityReceiptInput`.
- Added `generate_terminal_local_work_report_with_authority_receipt`.
- Reused the accepted trusted-receipt citation derivation and WorkReport
  constructors.
- Validated the receipt against the report run, workflow, approval request,
  granted decision, and exact approval-decision event.
- Kept authority-receipt citations distinct from approval-decision citations.
- Added the receipt citation to decisions and approvals only.
- Added focused real proof-path, legacy compatibility, mismatch, no-mutation,
  serialization, Debug, and unsafe-redaction tests.

## 3. Scope Explicitly Not Completed

The phase did not modify executor inputs or defaults, automatically generate
reports, accept unverified receipt claims, accept arbitrary citations as trusted
provenance, persist receipts or reports, write artifacts, add referential
integrity lookup, append events, add audit projections, expose schemas/SDK/CLI
or UI fields, change approvals, invoke providers or OpenShell, execute
SideEffects or writes, expand hosted behavior, add enterprise identity, or
change release posture.

## 4. API And Trust Boundary

The new input owns the existing `TerminalLocalWorkReportInput` and borrows one
trusted `GovernanceDecisionAuthorityReceipt`. The generator validates the
trusted receipt and its report context before citation derivation. Public
callers cannot substitute a serialized unverified claim or launder an arbitrary
typed receipt ID through a generic citation input.

The context gate requires matching run and workflow identity, a matching
approval request bound to the terminal run's schema/version/spec identity, a
granted matching decision, and the exact matching `ApprovalGranted` event. A
mismatch returns
`work_report_generation.authority_receipt.context_mismatch` without values.

## 5. Report Semantics

The citation is placed in `decisions_made` and `approvals`. It is not placed in
`evidence_considered`, because the receipt explains governance authority at the
approval-resume boundary rather than evidence considered by the work itself.
Its presence does not prove that resumed work succeeded and does not grant
reusable authority.

## 6. Privacy And Failure Posture

Only the stable receipt ID enters report serialization. Debug output redacts
the receipt and report identities. Raw facts, presentation content, approval
reasons, commands, provider payloads, credentials, and tokens remain outside
the report. Invalid context or secret-like redaction metadata returns no report,
does not mutate the borrowed run, and emits a stable non-leaking error.

## 7. Test Coverage

The real proof-enforced current-fact approval path produces the trusted receipt
and completed run used by composition tests. Tests verify exact section
placement, absence from evidence-considered, legacy generator non-regression,
stable receipt-ID serialization, Debug redaction, run mismatch rejection,
unsafe-redaction rejection, and no run mutation.

## 8. Validation Commands And Results

- `cargo fmt --all --check`: passed.
- Focused authority-receipt composition test: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Remaining Limitations

- No executor result path supplies the receipt to report generation.
- Receipts and reports remain in memory at this boundary.
- No artifact store resolves or validates receipt references.
- Serialized receipt claims remain unverified and cannot enter this API.
- Receipt evidence remains decision-scoped, not proof of terminal work success.

## 10. Recommended Next Phase

Plan explicit executor propagation of a successful trusted receipt into this
additive report generator. Keep it opt-in and in-memory before considering
persistence or artifact referential integrity.

## 11. Governed Phase Record

- Dogfood workflow: `dg/implement`
- Run ID: `run-1786421733078071000-2`
- Approval ID: `approval/run-1786421733078071000-2/implementation-approved`
- Presentation ID: `presentation/33f37be43dacf2c8`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: implementation, tests, documentation, validation, and
  git/PR work
