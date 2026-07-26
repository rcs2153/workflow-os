# Authoritative Approval-Resume Report Completion Review

## 1. Executive Verdict

**Phase accepted; proceed to generic explicit local-check profile planning.**

The implementation closes the authoritative report consumer's aggregate
approval gap without weakening freshness, presentation proof, immutable input,
resolved context, or later step approval boundaries.

## 2. Scope Verification

The phase stayed within the approved local, in-memory runtime-composition
scope.

It did not add:

- automatic or model approval;
- CLI or UI behavior;
- default proportional governance;
- generic handler discovery or arbitrary commands;
- report artifacts or persistence;
- schemas, SDKs, scaffolds, or examples;
- providers, OpenShell, sandbox execution, or credentials;
- SideEffect execution or writes;
- hosted behavior, reasoning lineage, or release changes.

## 3. API Assessment

The additive
`LocalAuthoritativeGovernanceApprovalReportDecisionRequest` composes the
existing proof-enforced approval request, explicit report inputs, and bounded
reference metadata.

`decide_approval_with_authoritative_docs_check_governance_report(...)`
returns
`LocalAuthoritativeGovernanceApprovalReportDecisionResult`, preserving the
exact run and exposing report posture, optional report, optional stable report
error, and optional bounded check reference through read-only accessors.

Existing approval APIs retain their signatures and behavior.

The API is narrow and testable. It does not accept caller-selected check
status, route, assessment, approval binding, or terminal report status.

## 4. Freshness And Authority Assessment

The implementation correctly distinguishes:

- request-time result as historical context for why approval was requested;
- decision-time result as the fresh check consumed by approval authority; and
- terminal report citation as a reference to that decision-time result.

The private reassessment outcome retains the exact fresh assessment and
bounded result together. The public helper executes no additional report-only
check.

Approval preparation, immutable-bundle verification, fresh reassessment,
durable assessment equality, aggregate approval-subject equality,
presentation-proof validation, resolved-context validation, and approval
mutation retain their accepted ordering.

No approval, resume, step, or skill event is appended before the pre-decision
gates succeed.

## 5. Grant, Denial, And Deferred Assessment

A valid grant that completes the workflow returns:

- the completed run;
- `Generated` report posture;
- a validated report; and
- a bounded reference derived from the exact fresh check.

A valid denial returns:

- the failed run;
- `Generated` report posture;
- a validated report; and
- no skill invocation.

When aggregate resume reaches a later workflow-declared step approval, the
result remains `WaitingForApproval`, report posture is
`DeferredNonTerminal`, no partial report is created, and the later approval
remains undecided. The aggregate grant does not satisfy or erase it.

## 6. Report And Hook Assessment

The helper reuses the existing terminal report construction boundary and
`LocalCheckResultReference::from_result(...)`.

The report cites the decision-time stable local-check reference without
copying command output. A direct regression confirms that required
`BeforeReport` behavior remains enforced and its bounded hook reference is
cited on this approval-completion path.

No workflow event, report artifact, or persistence side effect is added by
report construction.

## 7. Failure Assessment

Duplicate stable references fail before canonical check execution or approval
mutation.

Freshness, immutable-input, assessment, presentation-proof, and
resolved-context failures continue to return top-level stable errors before a
decision result exists.

After a decision returns a run, reference or report-construction failure is
represented as `GenerationFailed`. The exact run, status, and durable event
history remain available and unchanged.

No fake report or evidence reference is created.

## 8. Privacy And Redaction Assessment

The request and result use existing validated Core types.

The implementation does not accept or copy raw:

- command output or transcripts;
- source, spec, or parser contents;
- provider payloads;
- environment values;
- credentials, authorization headers, private keys, or tokens; or
- approval presentation prose.

`Debug` exposes bounded posture only. Stable errors do not include caller
values. Focused tests verify non-leakage from secret-like report input.

## 9. Test Quality Assessment

Focused coverage verifies:

- terminal grant;
- terminal denial without skill invocation;
- later step approval deferral;
- exact decision-time result citation;
- no additional report-only check;
- direct required `BeforeReport` behavior;
- duplicate-reference preflight before decision mutation;
- report failure preserving completed-run truth;
- durable event equality; and
- stable non-leaking errors and `Debug`.

The full workspace suite covers existing approval, immutable-run,
EvidenceReference, WorkReport, SideEffect, adapter, CLI, and runtime behavior.

No blocking test gap remains.

## 10. Documentation Assessment

The plan, report, and roadmap accurately state that:

- the local in-memory approval-resume report path is implemented;
- the decision-time result is terminal freshness evidence;
- request-time evidence is not silently treated as current authorization;
- CLI exposure still requires a generic explicit check-profile source; and
- defaults, artifacts, persistence, schemas, providers, OpenShell,
  SideEffect execution, and writes remain unimplemented.

The latest external user review aligns with this sequencing. It validates the
current first-run honesty and recommendation boundary, and identifies
low-friction proportional governance as the next product pressure. It does not
justify bypassing approval freshness or introducing ambient execution
authority.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Preserve explicit request-time and decision-time temporal roles if both are
  later represented in one report.
- Decide how bounded report inputs survive multiple chained approvals before
  adding persisted continuation.
- Keep default human output concise when the CLI preview becomes eligible.
- Retain Node 20 as the supported integration-check baseline until broader
  Node-version behavior is intentionally reviewed.

## 13. Recommended Next Phase

Plan one generic explicit local-check profile source.

The first source should bind an already implemented handler to a canonical
validated command contract without inferring arbitrary repository commands,
accepting shell strings, or treating safe metadata discovery as execution
authority.

Only after that source is implemented and reviewed should the explicit
authoritative quiet-success CLI preview proceed.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1785097044990235000-2`
- approval:
  `approval/run-1785097044990235000-2/review-scope-approved`
- presentation: `presentation/8cd1154610f53168`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- approval-presentation enforcement: proof enforced
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation:
  - `cargo fmt --all --check` passed
  - `cargo clippy --workspace --all-targets -- -D warnings` passed
  - `cargo test --workspace` passed
  - `npm run check:docs` passed
  - `git diff --check` passed
- out-of-kernel work: source inspection, focused review, one coverage
  correction, documentation, and validation
- missing coverage: the kernel coordinated governance but did not inspect
  code, edit files, run tests, or perform git and PR actions
