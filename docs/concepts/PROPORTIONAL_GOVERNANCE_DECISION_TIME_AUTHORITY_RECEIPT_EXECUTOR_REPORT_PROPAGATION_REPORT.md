# Decision-Time Authority Receipt Executor Report Propagation Report

## 1. Executive Summary

Workflow OS now provides one explicit, local, in-memory helper that carries a
Core-produced decision-time authority receipt from the proof-enforced approval
path into a validated terminal WorkReport. The helper retains the complete
approval decision result, including its run, assessment binding, and
decision-time runtime-fact snapshot.

The phase does not change default executor behavior or add automatic reports,
persistence, artifacts, schemas, CLI/UI behavior, providers, OpenShell,
SideEffect execution, writes, hosted behavior, or release changes.

## 2. Scope Completed

- Added `LocalGovernanceAuthorityReceiptReportInput`.
- Added `LocalGovernanceAuthorityReceiptReportResult` with read-only accessors
  and `into_parts()`.
- Added `compose_governance_authority_receipt_decision_report`.
- Reused the accepted trusted receipt-bearing decision result and receipt-backed
  terminal report generator.
- Preserved the complete approval decision context.
- Added grant, denial, report-failure, citation, privacy, and ownership tests.
- Exported the additive API from `workflow-core`.

## 3. Scope Explicitly Not Completed

No automatic report generation, default executor integration, generic receipt
input, serialized-claim verification, persistence, report artifact, event or
audit projection, schema, SDK field, CLI/UI behavior, provider execution,
OpenShell integration, SideEffect execution, write behavior, hosted expansion,
enterprise identity, cryptographic receipt, or release change was added.

## 4. API Summary

`LocalGovernanceAuthorityReceiptReportInput` owns the opaque trusted
receipt-bearing approval result and explicit `LocalExecutionReportInputs`.

`compose_governance_authority_receipt_decision_report` consumes that input and
returns `LocalGovernanceAuthorityReceiptReportResult`. The result retains:

- the complete `LocalCurrentRuntimeFactsGovernanceApprovalDecisionResult`;
- the optional trusted `GovernanceDecisionAuthorityReceipt`;
- the optional validated `WorkReport`; and
- an optional structured report-generation error.

The result exposes the run through its retained decision. This preserves more
accepted context than the plan's provisional run-only result without adding
duplicate public state.

## 5. Trust And Provenance Boundary

The helper accepts only the opaque Core result emitted by the existing
proof-enforced, fresh-runtime-fact approval path. Generic report inputs still
cannot inject a receipt, serialized unverified claims are not accepted, and the
helper never rebuilds a receipt from IDs or citations.

The accepted report generator validates the exact workflow, run, approval,
decision-event, and terminal context before deriving citations. Receipt
presence remains evidence of one bounded decision, not reusable authority.

## 6. Grant, Denial, And Failure Semantics

- A granted decision with its trusted receipt attempts terminal report
  construction and cites that receipt in decisions and approvals.
- A denied decision returns its failed run and no receipt-backed report. It
  fabricates no receipt, citation, or missing-evidence record.
- Report validation, hook, or redaction failure returns the original complete
  decision and trusted receipt, no report, and a structured error.
- Report failure never rewrites, retries, or retroactively denies the accepted
  approval result.

The helper performs no source call, governance reassessment, event append,
state write, artifact write, provider call, or CLI output.

## 7. Privacy And Redaction Summary

Input and result Debug implementations expose only status and presence posture.
They do not expose report text, receipt IDs, runtime facts, presentation
contents, paths, commands, provider payloads, credentials, tokens, or supplied
redaction metadata. Existing WorkReport validation supplies stable,
non-leaking errors for unsafe report inputs.

## 8. Test Coverage Summary

Focused tests prove:

- a real proof-enforced grant composes a valid receipt-backed WorkReport;
- the exact receipt appears in decisions and approvals;
- the complete decision, assessment binding, runtime-fact snapshot, run, and
  receipt are retained rather than reconstructed;
- denial emits no receipt-backed report or fabricated evidence;
- unsafe report metadata preserves the successful decision and receipt while
  returning a stable non-leaking report error;
- Debug output does not expose receipt IDs or secret-like inputs; and
- existing local executor behavior remains compatible.

## 9. Validation Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy -p workflow-core --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --test local_executor`: 330 passed, 0 failed,
  1 ignored.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

The isolated full executor test target required the current `workflow-os` CLI
binary beside the test executable because existing handler fixtures resolve
that prerequisite from the target directory. This was test-environment setup,
not a product behavior change.

## 10. Remaining Limitations

- Report composition remains explicit and in memory only.
- Receipts are not persisted or resolved by artifact integrity gates.
- No report is generated automatically.
- No CLI or schema surface exposes the composition helper.
- The phase runner's stale-binary detection remains a separate P0 follow-up.

## 11. Recommended Next Phase

Perform the focused implementation review, then plan the smallest receipt
persistence or report-artifact referential-integrity boundary. Do not broaden
provider mutations, automatic reports, schemas, CLI/UI behavior, SideEffects,
or writes in that phase.

## 12. Governed Implementation Record

- Dogfood workflow: `dg/implement`
- Run ID: `run-1786423995992205000-2`
- Approval ID: `approval/run-1786423995992205000-2/implementation-approved`
- Presentation ID: `presentation/f30a3e957dda7d8c`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Out-of-kernel work: Rust implementation, tests, documentation, validation,
  and git/PR operations
