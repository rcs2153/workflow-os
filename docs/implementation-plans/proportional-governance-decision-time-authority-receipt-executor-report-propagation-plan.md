# Decision-Time Authority Receipt Executor Report Propagation Plan

Status: Planned and accepted. No runtime implementation is included in this
phase. See the
[planning report](../concepts/PROPORTIONAL_GOVERNANCE_DECISION_TIME_AUTHORITY_RECEIPT_EXECUTOR_REPORT_PROPAGATION_PLAN_REPORT.md)
and [maintainer plan review](../concepts/PROPORTIONAL_GOVERNANCE_DECISION_TIME_AUTHORITY_RECEIPT_EXECUTOR_REPORT_PROPAGATION_PLAN_REVIEW.md).

## 1. Executive Summary

Workflow OS can emit a trusted, payload-free decision-time authority receipt
from the exact proof-enforced fresh-fact approval-resume path and can compose
that receipt into a terminal in-memory WorkReport. No accepted API yet carries
those two results through one executor-adjacent boundary.

The next implementation should add one explicit, opt-in composition helper. It
should consume the trusted receipt-bearing approval result, generate the report
from that result's terminal run and receipt, and return owned run, receipt, and
report posture. Generic report inputs must not accept a caller-supplied receipt,
and existing approval and report executor APIs must remain unchanged.

## 2. Goals

- Propagate a Core-produced trusted receipt into one terminal in-memory report.
- Preserve the proof-enforced approval result as the only receipt source.
- Reuse `generate_terminal_local_work_report_with_authority_receipt`.
- Return the terminal run even when report construction fails.
- Keep report failure distinct from approval/resume success.
- Preserve existing executor, report, persistence, and artifact defaults.
- Use stable non-leaking result and error behavior.

## 3. Non-Goals

No Rust implementation is included in this planning phase. The future slice
does not add automatic report generation, generic receipt injection, receipt
verification from serialized claims, persistence, report artifacts,
referential-integrity stores, workflow events, audit projection, schemas, SDK
fields, CLI/UI behavior, provider execution, OpenShell integration, SideEffect
execution, writes, hosted behavior, enterprise identity, cryptographic
receipts, default changes, or release changes.

## 4. Current Accepted Boundary

The accepted approval path returns
`LocalCurrentRuntimeFactsGovernanceAuthorityReceiptDecisionResult`. It owns:

- `LocalCurrentRuntimeFactsGovernanceApprovalDecisionResult`, including the
  resulting `WorkflowRun`; and
- an optional trusted `GovernanceDecisionAuthorityReceipt`, present only for a
  successful grant.

The accepted report path takes
`TerminalLocalWorkReportAuthorityReceiptInput`, validates exact run and granted
approval-event context, derives the receipt citation internally, and returns a
validated `WorkReport`.

`LocalExecutionReportInputs` is generic caller input. It must remain unable to
carry a trusted receipt because public input shape cannot prove provenance.

## 5. Proposed API

Add one executor-adjacent explicit input, provisionally named
`LocalGovernanceAuthorityReceiptReportInput`, containing:

- `decision`: `LocalCurrentRuntimeFactsGovernanceAuthorityReceiptDecisionResult`;
  and
- `report`: `LocalExecutionReportInputs` or the smallest equivalent explicit
  owned report-input model needed to build `TerminalLocalWorkReportInput`.

Add one result, provisionally named
`LocalGovernanceAuthorityReceiptReportResult`, containing owned:

- `run: WorkflowRun`;
- `authority_receipt: Option<GovernanceDecisionAuthorityReceipt>`;
- `work_report: Option<WorkReport>`; and
- `report_generation_error: Option<WorkflowOsError>`.

Provide read-only accessors and `into_parts()`. Debug output must disclose only
run status and presence/count posture, never report text, receipt IDs, raw
metadata, paths, facts, commands, provider payloads, or credentials.

Add one free function, provisionally named
`compose_governance_authority_receipt_decision_report`. Do not add a method to
the default `LocalExecutor` surface in the first slice.

## 6. Ownership And Composition Sequence

The implementation must avoid a self-referential result. It should:

1. consume the trusted decision result into its owned decision and optional
   receipt;
2. consume the decision into owned run, assessment binding, and runtime-fact
   snapshot;
3. borrow the owned run and receipt only while constructing the terminal report
   input and calling the accepted report generator;
4. finish report construction before returning; and
5. return owned run, receipt, and report posture.

The assessment binding and runtime-fact snapshot remain approval-path products.
The implementation should retain them only if an existing result contract needs
them; otherwise their deliberate omission must be documented and tested rather
than silently creating duplicate public state.

## 7. Grant, Denial, And Failure Semantics

- Granted terminal result with a trusted receipt: attempt receipt-backed report
  construction.
- Denied result: return the denied run and no receipt-backed report; do not
  fabricate a receipt or missing-citation record.
- Missing receipt on a result that otherwise claims a granted trusted path:
  fail report composition with one stable invariant error while preserving the
  returned run.
- Report validation or redaction failure: return the run and receipt with no
  report plus a structured report-generation error.
- Approval/resume errors before a decision result exists remain errors from the
  existing approval API and are outside this composition helper.

Report failure must not alter, retry, deny, or retroactively fail the approval
decision. The helper must not re-read workflow specs, re-resolve runtime facts,
reassess governance, or invoke approval a second time.

## 8. Trust And Provenance Rules

- Accept only the opaque trusted receipt-bearing Core result.
- Never accept `UnverifiedGovernanceDecisionAuthorityReceiptClaim`.
- Never accept a generic prebuilt receipt citation as trusted provenance.
- Never recreate a receipt from IDs or public fields.
- Let the accepted report generator revalidate exact run, workflow, approval,
  decision, and event identity.
- Keep the receipt evidence-only; its presence grants no reusable authority.

## 9. Privacy And Redaction

Only the stable receipt ID already permitted by the accepted citation model may
enter serialized report output. Runtime facts, presentation contents, policy
payloads, commands, provider output, paths, credentials, tokens, and reusable
authority remain outside the report and result Debug output.

Errors must use stable codes and static messages. Invalid report metadata,
identity mismatch, and missing trusted provenance must not echo supplied
values. Existing WorkReport redaction and bounded-text constructors remain the
enforcement boundary.

## 10. Compatibility And Runtime Posture

- `decide_approval_with_current_runtime_facts_governance_reassessment_presentation_and_authority_receipt`
  remains unchanged.
- `execute_with_report` and `LocalExecutionReportInputs` remain unchanged.
- Existing terminal report generators remain unchanged.
- No report is generated automatically.
- No events, state, projections, artifacts, files, or CLI output are produced.
- No provider or sandbox is invoked.

The helper is an additive composition seam for callers that already selected
the trusted approval path and explicitly request an in-memory report.

## 11. Test Plan

- A real proof-enforced granted approval result composes into a valid report.
- The report cites the exact trusted receipt in decisions and approvals.
- The returned run and receipt are the original owned values, not reconstructions.
- Existing generic report inputs cannot inject a trusted receipt.
- A denied decision returns no receipt-backed report and fabricates no evidence.
- Receipt/run mismatch fails report construction without mutating the run.
- Unsafe report metadata returns a stable non-leaking report error while
  preserving the successful run and receipt.
- Composition appends no events, performs no source call, does not reassess
  governance, and performs no state or artifact write.
- Debug and serialization do not leak facts, receipt IDs through Debug, paths,
  commands, provider payloads, or secret-like values.
- Existing approval, receipt, WorkReport, executor, and workspace tests remain
  green.

## 12. Implementation Sequence

1. Add the narrow input/result types and free composition helper.
2. Reuse the existing trusted result `into_parts()` and receipt-backed report
   generator without changing their contracts.
3. Add real proof-path, denial, report-failure, privacy, and no-mutation tests.
4. Run formatting, focused tests, workspace clippy/tests, docs, and diff checks.
5. Perform a focused maintainer review.

## 13. Deferred Follow-Ups

Receipt persistence, report artifact referential integrity, durable audit
projection, CLI rendering, workflow schema exposure, automatic report
generation, provider/OpenShell execution, SideEffect execution, and writes
remain separately governed phases.

The repo-local phase runner's stale-binary detection also needs a separate
hardening phase. It was observed during this planning run and must not be fixed
inside this authority-receipt propagation scope.

## 14. Final Recommendation

Proceed next with the additive in-memory executor-result composition helper
only, followed by focused review. Do not broaden generic report inputs or begin
persistence, artifacts, providers, or writes.

## 15. Governed Planning Record

- Dogfood workflow: `dg/d`
- Run ID: `run-1786423286110143000-2`
- Approval ID: `approval/run-1786423286110143000-2/planning-approved`
- Presentation ID: `presentation/653349c3cc29c431`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
