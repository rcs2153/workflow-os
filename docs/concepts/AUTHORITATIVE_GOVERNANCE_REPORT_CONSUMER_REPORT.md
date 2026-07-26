# Authoritative Governance Report Consumer Report

## 1. Executive Summary

Workflow OS now has one explicit in-memory consumer that composes the accepted
authoritative proportional-governance dispatcher with bounded local-check
evidence and terminal WorkReport generation.

The helper calls the dispatcher exactly once, preserves its route-specific
truth, derives one payload-free `LocalCheckResultReference` from the actual
same-call `DocsCheck` result, and generates a WorkReport only when the selected
route is terminal.

This phase does not make proportional governance or report generation a
default. It does not add CLI/UI behavior, schemas, examples, artifacts,
persistence, providers, OpenShell, SideEffect execution, writes, hosted
behavior, or release changes.

## 2. Scope Completed

- Added `AuthoritativeDocsCheckReportReferenceInputs`.
- Added `LocalExecutionWithAuthoritativeGovernanceReportRequest`.
- Added `AuthoritativeGovernanceReportPosture`.
- Added `LocalExecutionWithAuthoritativeGovernanceReportResult`.
- Added
  `execute_with_authoritative_docs_check_governance_report(...)`.
- Reused the existing dispatcher exactly once per request.
- Reused existing local-check reference, WorkReport, terminal-report, and
  `BeforeReport` validation boundaries.
- Exported the explicit API from `workflow-core`.
- Added focused route, citation, deferral, failure, event, and privacy tests.

## 3. Scope Explicitly Not Completed

- no default `LocalExecutor::execute(...)` integration;
- no automatic report generation;
- no CLI or UI exposure;
- no workflow schema, SDK, scaffold, or example changes;
- no report artifacts or report persistence;
- no local-check result persistence or automatic store discovery;
- no raw command-output evidence;
- no approval resume or cancellation report path;
- no providers or OpenShell integration;
- no SideEffect execution or new provider writes;
- no hosted behavior, enterprise administration, or release changes.

## 4. API Summary

The new free function accepts:

- an explicit `LocalExecutor`;
- an explicit `LocalImmutableRunBundleStore`;
- an explicit `DocsCheckLocalHandler`;
- optional visible-disclosure dependencies;
- authoritative execution inputs;
- report inputs; and
- bounded metadata for the derived local-check result reference.

It returns the original
`LocalExecutionWithAuthoritativeGovernanceRouteResult`, a report posture,
optional WorkReport, optional report-generation error, and optional derived
local-check result reference.

## 5. Route And Report Behavior

- `QuietProceed`: terminal runs generate a report.
- `VisibleProceed`: terminal runs generate a report after the existing
  disclosure-delivery boundary.
- `ApprovalRequired`: the run and reference are returned with
  `DeferredNonTerminal`; no report error is fabricated.
- `Denied`: the terminal failed run generates a truthful terminal report.

The route enum is never flattened into a generic success/failure result.

## 6. Failure Boundary

Failures detectable before dispatch, including a duplicate report reference,
return a top-level structured error and do not execute the check or create a
run.

After a route exists, reference or report failures return:

- the exact route and workflow run;
- `GenerationFailed`;
- no WorkReport;
- a structured non-leaking error; and
- the derived check reference when reference construction already succeeded.

This prevents a report-layer failure from hiding durable execution truth.

## 7. Evidence And Privacy

The local-check result reference derives command identity, command kind,
status, workflow identity, and run identity from the actual same-call result.
The caller may supply only stable reference metadata.

The consumer does not copy stdout, stderr, command transcripts, source
contents, spec contents, provider payloads, environment values, credentials,
or token-like values. `Debug` output exposes only bounded route, status,
posture, count, and error-code fields.

## 8. Test Coverage

Focused tests verify:

- quiet terminal report generation;
- visible route preservation and one disclosure delivery;
- denied route preservation and terminal report generation;
- approval-required report deferral without a false error;
- one canonical check invocation;
- same-call result-reference identity and status;
- WorkReport citation of the derived stable reference;
- duplicate-reference rejection before execution;
- post-route report failure preserving the durable run and events;
- secret-like report input non-leakage; and
- existing report helper behavior through the shared already-run generation
  path.

## 9. Validation

Passed:

- `cargo check -p workflow-core`
- focused authoritative governance report consumer tests: 4 passed
- existing executor report-path tests: 45 passed
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

## 10. Product Feedback Alignment

Fresh-pull user testing continues to validate the kernel's honest local-first
boundary, preserved agent instructions, concise first-run summary, inactive
workflow drafts, fail-closed promotion, approval state, and durable audit
history.

This phase advances the next product problem identified by that testing:
reducing ceremony for low-risk work without weakening evidence. It connects an
authoritative quiet/visible/approval/denial decision to independently produced
check evidence and a governed report, while leaving default policy and
operator UX for later phases.

Two separate polish items remain:

- integration tooling should either support Node 24 or fail early with an
  explicit Node 20 requirement; and
- pre-scaffold validation should avoid repeating the missing-manifest
  diagnostic.

Neither item changes this runtime-composition boundary.

## 11. Remaining Limitations

- The consumer is opt-in and in memory.
- Approval resume does not yet generate the deferred report.
- Local-check references are not durable artifacts.
- No operator surface renders route and report posture.
- No default proportional-governance policy selects this consumer.
- No sandbox execution provider, including OpenShell, is integrated.

## 12. Recommended Next Phase

Perform a focused maintainer review of this explicit consumer.

After acceptance, prioritize the next accepted proportional-governance phase
that reduces low-risk ceremony through authoritative quiet success. Do not
broaden provider mutation families or integrate OpenShell before immutable
inputs, scoped runtime authority, check attestation, and report/evidence
boundaries remain stable.

## 13. Governed Implementation Record

- workflow: `dg/runtime-composition`
- run: `run-1785087929778318000-2`
- approval:
  `approval/run-1785087929778318000-2/composition-approved`
- presentation: `presentation/78311ee150be086c`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- out-of-kernel work: code and documentation edits, tests, validation, and
  later git/PR actions
- missing coverage: the kernel coordinated governance only; it did not edit
  files, execute commands, create report artifacts, or perform git or PR
  actions
