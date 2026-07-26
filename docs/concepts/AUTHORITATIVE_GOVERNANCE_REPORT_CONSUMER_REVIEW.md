# Authoritative Governance Report Consumer Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation is a narrow, additive runtime composition of already
accepted primitives. It preserves authoritative route truth, executes the
canonical check once, binds report evidence to the same-call result, and does
not hide an existing run behind post-route report failure.

## 2. Scope Verification

The phase stayed within approved scope.

It did not add:

- default executor behavior;
- automatic report generation;
- CLI or UI behavior;
- schemas, SDKs, scaffolds, or examples;
- report artifacts or persistence;
- provider or OpenShell integration;
- SideEffect execution or writes;
- hosted behavior or release changes.

## 3. API Assessment

The free-function API is appropriate for the first slice. It makes the
executor, immutable-bundle store, `DocsCheck` handler, optional visible
delivery dependencies, authoritative request, report inputs, and reference
metadata explicit.

It does not add an ambient executor method or hidden runtime configuration.
The request contains no route selector, caller-supplied check status, prebuilt
result reference, raw output, or report artifact path.

## 4. Dispatcher Authority

The consumer calls
`route_authoritative_docs_check_governance(...)` exactly once.

It does not independently reassess governance, select a route, rerun the
canonical check, or flatten the dispatcher result. Quiet proceed, visible
proceed, approval required, and denial remain distinct result variants.

## 5. Local-Check Reference Assessment

The implementation requires exactly one local-check result and derives:

- command ID;
- command kind;
- result status;
- workflow ID; and
- run ID

from the actual route result.

Caller input is limited to stable reference identity, optional existing event,
audit, and output references, redaction metadata, and sensitivity.
`LocalCheckResultReference::from_result(...)` remains the validation boundary.

The resulting WorkReport citation contains only the stable result reference.
Raw stdout, stderr, summaries, command transcripts, and EvidenceReference
payloads are not copied or recreated.

## 6. Route And Report Semantics

- Quiet terminal runs generate a report.
- Visible terminal runs generate a report after disclosure delivery.
- Approval-required runs return `DeferredNonTerminal`, no report, and no false
  report error.
- Denied terminal runs retain the denied route and generate a failed terminal
  report.

This is consistent with existing Workflow OS execution/report separation.

## 7. Failure Assessment

Duplicate stable references are rejected before dispatch. Focused tests show
that this path executes no check and creates no run events.

After a route exists:

- reference construction failure returns `GenerationFailed`;
- report construction failure returns `GenerationFailed`;
- the route and exact run remain available;
- report errors do not rewrite workflow status; and
- durable events remain unchanged.

This resolves the blocker identified during planning: post-route report-layer
failure cannot hide execution truth behind top-level `Err`.

## 8. Privacy And Redaction Assessment

The new request and result `Debug` implementations redact identities and
caller metadata. Result debug output is limited to route posture, run status,
result count, report posture, presence flags, and a stable error code.

Focused tests verify secret-like report input does not appear in returned
errors or debug output. Existing local-check reference and WorkReport
constructors continue to validate redaction metadata and bounded text.

## 9. Regression Assessment

The shared already-run report helper preserves existing
`LocalExecutor::execute_with_report(...)` behavior.

Evidence:

- 45 existing executor report-path tests passed;
- four focused consumer tests passed;
- the full Rust workspace passed;
- clippy passed with warnings denied; and
- documentation and diff checks passed.

## 10. Test Quality

Focused coverage verifies:

- all four route variants;
- one canonical check call;
- one visible disclosure delivery;
- same-call result-reference binding;
- report citation propagation;
- approval deferral;
- duplicate preflight without execution;
- post-route failure with run/event preservation; and
- privacy-safe errors and debug output.

Two direct consumer-specific cases remain shallow:

- `BeforeReport` behavior is proven through the shared helper's existing
  report-path tests rather than a consumer-specific test.
- The zero/multiple-result defensive branch is not directly reachable through
  the accepted dispatcher, which currently guarantees one canonical result.

These are non-blocking because the reused validation boundaries have direct
coverage and the public route invariant prevents the defensive branch today.

## 11. Documentation Review

The roadmap, plan, implementation report, and review state that the consumer is
implemented and remains explicit, local, in-memory, and fresh-run-only.

They do not overclaim default execution, CLI/UI exposure, artifacts,
persistence, providers, OpenShell, SideEffect execution, writes, hosted
behavior, or production readiness.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Add one direct consumer `BeforeReport` test when the next report-path change
  touches this helper.
- Add direct defensive result-count coverage if the dispatcher ever supports
  more than one canonical check result.
- Plan approval-resume report completion separately.
- Keep Node 24 integration-tooling behavior and duplicate missing-manifest
  output in the onboarding polish backlog.
- Do not integrate OpenShell until the provider-neutral execution-substrate
  boundary and required evidence surfaces are separately planned and reviewed.

## 14. Recommended Next Phase

Proceed to the next accepted proportional-governance runtime phase that makes
quiet success authoritative for low-risk work without making policy or report
behavior ambient.

The next phase should continue composing existing primitives rather than add a
new model family. Broader provider mutations and OpenShell integration should
remain behind immutable inputs, scoped authority, check attestation, and
report/evidence integrity.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1785092733780929000-2`
- approval:
  `approval/run-1785092733780929000-2/review-scope-approved`
- presentation: `presentation/cec5056058ef7ae5`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval presentation enforcement: proof enforced
- validation: implementation validation reviewed; docs and diff checks rerun
  after review authoring
- out-of-kernel work: code inspection, test assessment, review authoring, and
  validation
- missing coverage: the kernel coordinated governance only; it did not inspect
  code, execute validation, edit files, or perform git or PR actions
