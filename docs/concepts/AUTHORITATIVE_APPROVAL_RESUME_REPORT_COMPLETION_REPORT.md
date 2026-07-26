# Authoritative Approval-Resume Report Completion Report

## 1. Executive Summary

Workflow OS now has one additive, local, in-memory path that applies a
proof-enforced authoritative aggregate approval decision and, when the
resulting run is terminal, generates a validated `WorkReport` citing the exact
decision-time `DocsCheck` result used for fresh governance reassessment.

The implementation closes the report consumer's deferred aggregate-approval
gap without adding automatic approval, CLI behavior, persistence, report
artifacts, providers, OpenShell, SideEffect execution, or writes.

## 2. Scope Completed

The phase added:

- a report-bearing authoritative approval decision request;
- a route-preserving approval decision result;
- one private reassessment outcome retaining the fresh assessment and bounded
  check result together;
- reuse of the accepted presentation-proof, immutable-bundle, aggregate
  subject, and resolved-context approval path;
- bounded local-check reference construction from the exact decision-time
  result;
- terminal report generation for approval grant and denial;
- deferred report posture for a non-terminal resumed run;
- report-only failure posture after a valid decision result exists; and
- focused ordering, privacy, reference, report, and hook regressions.

## 3. Scope Explicitly Not Completed

The phase did not add:

- automatic or model approval;
- changes to ordinary approval APIs;
- CLI or UI behavior;
- default proportional governance;
- generic local-check profile registration;
- request-time result persistence;
- report artifacts or report persistence;
- schemas, SDKs, scaffolds, or examples;
- providers, OpenShell, sandbox lifecycle, or credentials;
- SideEffect execution or provider mutation;
- hosted behavior, enterprise administration, reasoning lineage, or release
  changes.

## 4. API Summary

The public request is:

```text
LocalAuthoritativeGovernanceApprovalReportDecisionRequest
```

It contains the existing proof-enforced authoritative approval request,
explicit report inputs, and bounded reference metadata.

The public helper is:

```text
decide_approval_with_authoritative_docs_check_governance_report(...)
```

The returned
`LocalAuthoritativeGovernanceApprovalReportDecisionResult` exposes read-only
access to:

- the exact resumed or denied `WorkflowRun`;
- `Generated`, `DeferredNonTerminal`, or `GenerationFailed` report posture;
- an optional validated `WorkReport`;
- an optional stable report-generation error; and
- the bounded decision-time local-check result reference.

Existing public approval helpers retain their signatures and behavior.

## 5. Freshness And Decision Boundary

The request-time result explains why the original run entered
`WaitingForApproval`. It is not reused as current authorization evidence.

At decision time, the existing authoritative path:

1. resolves the pending aggregate approval;
2. verifies immutable run inputs and the durable governance binding;
3. executes the canonical `DocsCheck`;
4. derives a fresh source-bound governance assessment;
5. requires equality with durable assessment and approval subject;
6. validates durable approval-presentation proof;
7. validates the resolved execution context; and
8. applies the approval decision.

The implementation retains the bounded result from step 3 and uses it for
report citation. It does not execute a second report-only check.

## 6. Grant, Denial, And Deferred Behavior

For a valid grant that completes the run, the helper returns the completed run
and a generated report.

For a valid denial, the helper returns the failed run and a generated report.
No workflow skill is invoked by the denied path.

If a resumed workflow reaches another approval or otherwise remains
non-terminal, the helper returns `DeferredNonTerminal`, no partial report, and
the bounded decision-time check reference.

An aggregate approval does not satisfy later step-scoped approvals.

## 7. Failure And Workflow Truth

Invalid stable-reference metadata and duplicate report references fail before
check execution or approval mutation.

Freshness, immutable-input, presentation-proof, aggregate-subject, and
resolved-context failures preserve the existing fail-closed approval boundary.

After a decision returns a run, reference or report construction failures are
represented as `GenerationFailed`. They do not rewrite workflow status,
remove durable events, or create a false report.

## 8. Privacy And Redaction

The helper uses existing validated constructors and does not accept or store
raw command output, source contents, spec contents, parser payloads, provider
payloads, environment values, credentials, authorization headers, private
keys, or tokens.

Request and result `Debug` implementations expose bounded posture only.
Errors use stable codes and static messages. Focused tests verify that
secret-like report inputs do not leak through error or `Debug` output.

## 9. Test Coverage

Focused tests cover:

- terminal grant report generation;
- terminal denial report generation without skill invocation;
- exact decision-time result reference and report citation;
- no extra report-only canonical check;
- required `BeforeReport` hook behavior on this composition path;
- duplicate-reference preflight before decision mutation;
- report-construction failure preserving completed-run truth;
- durable event equality;
- stable non-leaking errors and `Debug`; and
- existing proof-enforced approval behavior.

The full workspace validation also covers existing immutable-run, approval,
WorkReport, EvidenceReference, SideEffect, adapter, CLI, and runtime behavior.

## 10. Validation Commands

The following commands are required for phase close:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

Results are recorded after validation completes.

All required commands passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

Focused approval-report tests also passed for grant, denial, preflight, report
failure, decision-time citation, and direct `BeforeReport` behavior.

## 11. Remaining Limitations

- The helper is local and in memory.
- Report inputs are supplied explicitly and are not persisted across chained
  approval waits.
- Request-time and decision-time check references do not yet have separate
  typed temporal roles in the report model.
- The ordinary CLI does not expose this path.
- A generic validated local-check profile source does not yet exist.
- OpenShell is not integrated; any future sandbox integration must remain a
  provider-neutral execution substrate rather than governance authority.

## 12. Recommended Next Phase

Perform a focused maintainer review of this implementation.

After acceptance, plan and implement one generic explicit local-check profile
source before exposing authoritative quiet success through the CLI. The
profile must not infer arbitrary commands or turn repository metadata into
execution authority.

## 13. Governed Phase Record

- workflow: `dg/runtime-composition`
- run: `run-1785094161756576000-2`
- approval:
  `approval/run-1785094161756576000-2/composition-approved`
- presentation: `presentation/054a8678981f9803`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- approval-presentation enforcement: proof enforced
- validation: focused approval-report tests, formatting, clippy with warnings
  denied, the full workspace test suite, docs checks, and diff checks passed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- out-of-kernel work: source inspection, Rust implementation, tests,
  documentation, and validation
- missing coverage: the kernel coordinated governance but did not edit files,
  run tests, or perform git and PR actions
