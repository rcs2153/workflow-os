# Authoritative Approval-Resume Report Completion Plan Review

## 1. Executive Verdict

**Plan accepted; proceed to authoritative approval-resume report completion
implementation.**

The plan closes the accepted report consumer's only deferred authoritative
route without weakening approval authority. It correctly treats the
decision-time canonical check as freshness evidence, preserves the existing
proof-enforced approval and resolved-context path, and keeps report failure
separate from workflow truth.

## 2. Scope Verification

The plan stayed within the approved planning-only scope.

It did not authorize:

- Rust implementation during review;
- CLI, UI, schema, SDK, scaffold, or example behavior;
- default or automatic report generation;
- automatic approval or model self-approval;
- report artifacts or persistence;
- providers, OpenShell, sandbox execution, or credentials;
- SideEffect execution or writes;
- hosted behavior, reasoning lineage, or release changes.

## 3. Current Gap Assessment

The accepted fresh-run report consumer:

- generates terminal reports for quiet, visible, and denied routes;
- returns a truthful deferred posture for approval-required runs; and
- exposes a validated request-time local-check reference.

The accepted authoritative approval path:

- reruns the canonical check at decision time;
- compares the fresh assessment to durable authority;
- validates aggregate approval subject and presentation proof;
- checks resolved execution context before grant mutation; and
- returns only the resulting `WorkflowRun`.

The plan correctly identifies the missing composition: the exact bounded
decision-time result is discarded after it authorizes the decision and cannot
currently be cited by the terminal report.

## 4. Evidence Freshness Assessment

The plan's corrected temporal roles are sound:

- request-time result: explains why approval was requested;
- decision-time result: establishes whether current facts still authorize the
  decision;
- terminal report citation: must use the decision-time result.

The earlier CLI planning shorthand about avoiding a rerun was inaccurate
because the accepted approval path intentionally performs fresh reassessment.
The fix-forward correction is truthful: execute one decision-time check, use
it for both reassessment and report citation, and do not execute a second
report-only check.

This preserves the product invariant that stale evidence cannot silently
become current authorization evidence.

## 5. Authority And Mutation Ordering Assessment

The proposed helper preserves the accepted ordering:

1. prepare pending approval;
2. verify immutable bundle and durable binding;
3. execute the canonical check once;
4. derive and compare the fresh assessment;
5. validate aggregate approval subject;
6. validate durable presentation proof;
7. apply resolved-context and approval-decision semantics;
8. only then construct report references and a terminal report.

The private reassessment outcome is the right internal refactor. It keeps the
fresh result and assessment binding together without exposing a public
caller-constructible authority object.

The existing public approval helper can continue returning `WorkflowRun`,
which preserves compatibility.

## 6. Grant, Denial, And Deferred Assessment

### Grant

A valid aggregate grant may generate a report only after the existing resume
path returns a terminal run. It does not satisfy later workflow step approvals
or authorize SideEffects, providers, or writes.

### Denial

The explicit authoritative aggregate path currently reassesses denial as well
as grant. Preserving that stricter behavior is correct for this helper. A
valid denial invokes no workflow skill and may produce a failed terminal
report citing the decision-time check.

### Later approval

If aggregate resume reaches a workflow-declared step approval, the helper
returns `DeferredNonTerminal`. It retains the fresh reference but does not
create a partial report or bypass the later approval.

These route semantics are appropriate and deterministic.

## 7. Report And Reference Assessment

The plan reuses:

- `LocalCheckResultReference::from_result(...)`;
- `AuthoritativeDocsCheckReportReferenceInputs`;
- `AuthoritativeGovernanceReportPosture`;
- `LocalExecutionReportInputs`; and
- the existing terminal report helper.

That is appropriately minimal. Core derives command identity, check posture,
workflow identity, and run identity from the actual decision-time result and
returned run. The caller supplies only bounded stable reference metadata.

The plan correctly forbids:

- caller-supplied result status;
- prebuilt result references;
- raw stdout or stderr;
- implicit `EvidenceReference` creation; and
- automatic request-time reference reuse.

## 8. Failure Assessment

The plan preserves the important split:

- pre-decision failures return top-level `Err` before a valid decision result
  exists;
- post-decision reference or report failures return `GenerationFailed` with
  the exact resulting run still available.

That prevents a report-layer failure from hiding an approval denial, successful
resume, later approval pause, or terminal workflow outcome.

The duplicate-reference preflight before process execution and decision
mutation is especially important because it avoids changing durable state for
an already-invalid report request.

## 9. Privacy And Redaction Assessment

The planned request and result contain only existing validated report,
approval, execution, and reference inputs.

The plan excludes raw:

- command output and transcripts;
- source and spec contents;
- runtime fact payloads;
- approval presentation prose;
- provider payloads;
- environment values;
- credentials, authorization headers, private keys, and tokens.

Stable error codes and bounded static messages remain required. The proposed
`Debug` posture exposes no report text, identity values, hashes, paths, or
caller reasons.

## 10. Compatibility Assessment

The phase is additive and preserves:

- ordinary approval methods;
- the existing proof-enforced aggregate approval helper;
- fresh authoritative route behavior;
- current CLI behavior;
- current schemas and SDKs;
- report artifact and persistence posture; and
- all provider and SideEffect boundaries.

No public existing return type needs to change.

## 11. Test Plan Assessment

The planned tests cover:

- grant, denial, and later-approval routes;
- exactly one decision-time check;
- fresh result-reference construction;
- request-time versus decision-time evidence separation;
- mutation-free preflight and integrity failures;
- resolved-context and presentation proof;
- denial without skill invocation;
- report failure preserving run and event truth;
- direct `BeforeReport` behavior;
- privacy-safe errors and `Debug`; and
- full regressions.

The direct `BeforeReport` regression closes one non-blocking gap from the
fresh-run report consumer review.

No blocking test gap was found.

## 12. Documentation Assessment

The roadmap and directly related plans now state:

- the fresh authoritative report consumer is implemented;
- approval-required report completion is planned and accepted;
- the decision-time check is the terminal freshness source;
- CLI exposure still requires this implementation plus one generic explicit
  check-profile source; and
- CLI/UI behavior, defaults, artifacts, persistence, schemas, providers,
  OpenShell, SideEffect execution, and writes remain unimplemented.

The documentation does not overclaim current capability.

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- When chained approval report completion is planned, decide whether bounded
  report inputs and temporal check references require persistence.
- Before public CLI exposure, select and review one generic explicit
  local-check profile source.
- Preserve both temporal roles if future reports cite request-time and
  decision-time checks; do not flatten them into one undifferentiated
  citation list.
- Keep OpenShell behind a provider-neutral execution-substrate contract and
  do not use it to bypass the check-profile prerequisite.

## 15. Recommended Next Phase

Implement the authoritative approval-resume report completion path.

The implementation should refactor the private reassessment outcome, add one
explicit report-bearing decision request/result, reuse the accepted
proof-enforced approval path and report constructors, add focused state and
privacy tests, and remain local and in memory.

It must not add CLI/UI behavior, default execution, artifacts, persistence,
schemas, providers, OpenShell, SideEffect execution, writes, hosted behavior,
or release changes.

## 16. Governed Review Record

- workflow: `dg/review`
- run: `run-1785094039494606000-2`
- approval:
  `approval/run-1785094039494606000-2/review-scope-approved`
- presentation: `presentation/6dfa80897ebd4c9f`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- approval-presentation enforcement: proof enforced
- validation: `npm run check:docs` and `git diff --check` passed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- out-of-kernel work: plan inspection, runtime-contract comparison, review
  authoring, and validation
- missing coverage: the kernel coordinates governance only; it did not inspect
  code, edit files, execute validation, or perform git or PR actions
