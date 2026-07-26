# Authoritative Approval-Resume Report Completion Plan

Status: Implemented and accepted.

Related foundations:

- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Approval Resume Resolved-Context Integrity Plan](approval-resume-resolved-context-integrity-plan.md)
- [Authoritative Proportional-Governance Route Dispatcher Plan](authoritative-proportional-governance-route-dispatcher-plan.md)
- [Authoritative Governance Report Consumer Plan](authoritative-governance-report-consumer-plan.md)
- [Authoritative Governance Report Consumer Review](../concepts/AUTHORITATIVE_GOVERNANCE_REPORT_CONSUMER_REVIEW.md)
- [Authoritative Quiet-Success CLI Preview Plan](authoritative-quiet-success-cli-preview-plan.md)
- [Authoritative Approval-Resume Report Completion Plan Review](../concepts/AUTHORITATIVE_APPROVAL_RESUME_REPORT_COMPLETION_PLAN_REVIEW.md)

## 1. Executive Summary

The accepted authoritative governance report consumer generates an in-memory
`WorkReport` for terminal quiet, visible, and denied routes. An
approval-required route correctly returns `DeferredNonTerminal`, preserves its
pending run, and exposes a validated request-time local-check result reference.

The existing proof-enforced approval decision path already protects the resume
boundary. Before any approval, resume, step, or skill event is appended, it:

1. reloads the immutable run bundle and durable governance assessment;
2. reruns the canonical `DocsCheck`;
3. derives a fresh source-bound assessment from current typed facts;
4. requires exact equality with the durable assessment and aggregate approval
   subject;
5. validates durable approval-presentation proof; and
6. reuses the accepted resolved-context integrity checks before execution.

That path currently returns only the resumed `WorkflowRun`. It discards the
bounded result of the decision-time check after using it for reassessment.
Consequently, a caller cannot complete the deferred report with the exact
fresh check result that authorized resume.

The next implementation should add one explicit, local, in-memory
approval-resume report helper. It should refactor the existing authoritative
reassessment internally so the same canonical decision-time check produces
both:

- the exact binding used to authorize or deny the approval decision; and
- the payload-free `LocalCheckResultReference` cited by a terminal report.

The helper must execute the decision-time check exactly once. It must not cite
the stale request-time result as terminal authorization evidence, rerun the
check only for reporting, fabricate a result, or weaken approval-presentation,
immutable-bundle, resolved-context, or report-construction gates.

This plan does not implement anything. It does not add CLI or UI behavior,
default execution, report artifacts, persistence, schemas, providers,
OpenShell, SideEffect execution, writes, hosted behavior, or release changes.

## 2. Corrected Evidence Decision

The fresh-run authoritative route and approval-resume route answer different
questions:

- the request-time check establishes why the run entered
  `WaitingForApproval`;
- the decision-time check establishes whether current facts still match the
  approved governance context and whether the decision may mutate the run.

The terminal report must cite the decision-time result because it is the fresh
result actually consumed by the approval decision boundary.

The original request-time result may remain available to the caller through
the existing deferred route result. This phase must not present that reference
as the terminal authorization result, automatically copy it into the report,
or create a false claim that no decision-time reassessment occurred.

The earlier CLI plan language saying approval resume should cite the original
same-call result without rerunning the original check is therefore corrected:
the accepted runtime intentionally performs one fresh reassessment at decision
time. The new helper should preserve and cite that exact fresh result without
executing an additional report-only check.

## 3. Goals

- Complete the authoritative approval-required report path after a proof-
  enforced approval decision.
- Preserve the existing fresh reassessment and resolved-context integrity
  sequence.
- Execute exactly one canonical `DocsCheck` during the decision call.
- Keep the reassessment result and assessment binding inseparable until the
  decision boundary consumes them.
- Construct one validated, payload-free local-check result reference from the
  exact decision-time result.
- Generate a `WorkReport` only when the decision returns a terminal run.
- Preserve non-terminal resumed posture as report-deferred.
- Preserve the resumed or denied run when reference or report generation
  fails.
- Keep report failure separate from workflow status.
- Keep existing executor and approval APIs unchanged.
- Add focused state, event-ordering, privacy, and regression tests.

## 4. Non-Goals

The phase must not add:

- a second approval system or automatic approver;
- default or automatic report generation;
- changes to ordinary `LocalExecutor::decide_approval(...)`;
- caller-selected check status, governance route, or approval outcome;
- request-time check persistence or automatic result discovery;
- report artifacts, filesystem report output, or report persistence;
- CLI, UI, workflow-schema, SDK, scaffold, or example behavior;
- arbitrary commands, handler discovery, or runtime configuration;
- providers, OpenShell, sandbox lifecycle, credentials, or network access;
- SideEffect execution, provider mutation, or new write families;
- hosted behavior, enterprise administration, reasoning lineage, or release
  changes.

## 5. Existing Runtime Source Of Truth

The implementation must reuse
`decide_approval_with_governance_reassessment_and_presentation(...)`
semantics.

That accepted path owns:

- pending approval resolution;
- immutable run-bundle verification;
- canonical local-check execution;
- source-bound governance reassessment;
- durable assessment equality;
- aggregate approval-subject equality;
- presentation-proof validation and proof-marker derivation;
- resolved execution-context validation;
- approval grant or denial mutation; and
- ordinary resume execution.

The new report helper must not independently reproduce a weaker version of
those rules.

## 6. Internal Reassessment Outcome

Refactor the private authoritative reassessment boundary to return one private
value, for example:

```text
AuthoritativeDocsCheckApprovalReassessment
```

It should privately own:

- the exact reassessed `GovernanceAssessmentBinding`; and
- the exact bounded `LocalCheckResult` values produced by the same canonical
  check execution.

The current route guarantees one canonical `DocsCheck` result. The private
value should retain a vector only if that matches existing composition
internals; the public report helper must still require exactly one result.

The value must not be public, serializable, or independently constructible by
callers. Its `Debug` output must expose only bounded posture and result count,
not identities, hashes, paths, command data, runtime facts, or output.

The existing public approval helper should consume the binding from this
private outcome and otherwise preserve its current return type and behavior.
This keeps existing callers compatible.

## 7. Candidate Public Request

Add one explicit request adjacent to the existing approval and report
composition APIs:

```text
LocalAuthoritativeGovernanceApprovalReportDecisionRequest
```

Candidate fields:

- `approval:
  LocalGovernanceAssessmentApprovalPresentationDecisionRequest`;
- `report: LocalExecutionReportInputs`;
- `local_check_reference:
  AuthoritativeDocsCheckReportReferenceInputs`.

The approval request already includes:

- project root;
- run and approval identity;
- explicit decision;
- actor and reason;
- correlation identity;
- durable presentation-proof resolution;
- optional presentation age;
- exact authoritative execution inputs;
- typed current governance facts; and
- immutable bundle request identity.

The new request must not include:

- a request-time `LocalCheckResult`;
- a prebuilt `LocalCheckResultReference`;
- a governance assessment or approval binding;
- a route enum;
- a report terminal status;
- raw command output;
- a report artifact path; or
- provider or sandbox configuration.

## 8. Candidate Result

Add one route-preserving result:

```text
LocalAuthoritativeGovernanceApprovalReportDecisionResult
```

Required fields:

- `run: WorkflowRun`;
- `report_posture: AuthoritativeGovernanceReportPosture`;
- `work_report: Option<WorkReport>`;
- `report_generation_error: Option<WorkflowOsError>`;
- `local_check_result_reference:
  Option<LocalCheckResultReference>`.

The existing `AuthoritativeGovernanceReportPosture` vocabulary may be reused:

- `Generated`;
- `DeferredNonTerminal`;
- `GenerationFailed`.

Read-only accessors and `into_parts()` should follow existing report-result
patterns.

`Debug` must expose only:

- terminal or non-terminal run status;
- report posture;
- work-report presence;
- local-check-reference presence; and
- stable report error code.

It must not expose report text, IDs, hashes, paths, approval reason,
presentation content, runtime facts, command output, provider data, or
secret-like values.

## 9. Decision And Report Sequence

The new helper should perform:

1. Validate stable report-reference identity and reject duplicates before
   process execution or durable mutation.
2. Prepare the pending approval through the existing executor boundary.
3. Reload and verify the immutable bundle and durable governance binding.
4. Execute the canonical `DocsCheck` exactly once.
5. Derive the fresh authoritative reassessment.
6. Require exact durable-assessment and aggregate-subject equality.
7. Validate approval-presentation proof and derive its proof marker.
8. Apply the existing approval decision and resolved-context validation.
9. Retain the returned run and the exact decision-time local-check result.
10. Construct `LocalCheckResultReference::from_result(...)` from that result
    and the returned run identity.
11. If the run is non-terminal, return `DeferredNonTerminal`.
12. If the run is terminal, append the stable check reference to cloned report
    inputs and call the existing terminal report helper.
13. Return `Generated` or `GenerationFailed` without rewriting the run.

Steps 2 through 8 must preserve current mutation ordering. Steps 9 through 13
must not append workflow events or change approval semantics.

## 10. Grant Semantics

For a valid aggregate approval grant:

- the decision-time check must match the durable assessment;
- presentation proof must be valid and current;
- the resolved execution context must still match;
- the aggregate grant resumes from the beginning of the immutable workflow;
- later step-scoped approvals remain active;
- a terminal resumed run may produce a report;
- a resumed run waiting on a later step approval returns
  `DeferredNonTerminal`; and
- the report cites the decision-time check result.

The helper must not treat the aggregate approval as authority for later step
approvals, SideEffects, providers, or writes.

## 11. Denial Semantics

The accepted authoritative aggregate decision API currently reassesses both
grants and denials before mutation. The new helper should preserve that
behavior.

For a valid denial:

- current authoritative facts must still match;
- presentation proof must authorize the explicit denial decision;
- no workflow step or skill may run;
- the run becomes terminal failed through the existing denial lifecycle; and
- the failed terminal report may cite the decision-time check result.

This does not replace the separate ordinary approval-denial behavior described
by the resolved-context plan. It preserves the stricter contract of this
explicit authoritative aggregate path.

## 12. Non-Terminal Resume Semantics

An aggregate grant may resume into a later workflow-declared approval.

In that case:

- preserve the exact resumed run;
- return `DeferredNonTerminal`;
- return the validated decision-time local-check result reference;
- do not generate a partial WorkReport;
- do not treat report deferral as failure; and
- do not satisfy or bypass the later approval.

A later phase may define chained report completion across multiple approval
pauses. This phase does not persist report inputs or check references for
cross-process continuation.

## 13. Reference Semantics

Construct the reference through
`LocalCheckResultReference::from_result(...)`.

Core must derive from the decision-time result and returned run:

- command ID;
- command kind;
- result status;
- workflow ID; and
- run ID.

The caller may supply only:

- stable result-reference ID;
- optional existing workflow-event reference;
- optional existing audit-event reference;
- optional stable output reference;
- redaction metadata; and
- sensitivity.

Rules:

- require exactly one decision-time local-check result;
- reject duplicate stable references before decision mutation;
- never copy stdout, stderr, summaries, or command transcripts;
- never recreate an `EvidenceReference`;
- never claim command-output evidence;
- do not automatically add the request-time reference to the terminal report;
- retain the request-time reference only in the earlier deferred route result;
- use the decision-time reference for terminal validation/check citation.

## 14. Failure Semantics

### Before a decision result exists

Return top-level `Err` for:

- invalid or duplicate reference input;
- missing or malformed approval state;
- immutable-bundle mismatch;
- check execution or reassessment failure;
- durable assessment mismatch;
- aggregate approval-subject mismatch;
- missing, stale, or mismatched presentation proof;
- resolved execution-context mismatch; or
- ordinary approval decision failure before a resumed or denied run exists.

These failures must preserve the existing no-mutation guarantees.

### After a decision result exists

Once the existing approval path returns a run:

- reference construction failure returns `GenerationFailed`;
- report construction failure returns `GenerationFailed`;
- the exact run remains available;
- workflow status is unchanged;
- no compensating event is appended; and
- no false report is produced.

## 15. Privacy And Redaction

The helper must not store or copy:

- raw local-check output;
- command transcripts;
- source or spec contents;
- paths beyond existing validated internal execution inputs;
- runtime fact payloads;
- approval presentation prose;
- provider payloads;
- environment values;
- credentials, authorization headers, private keys, or tokens;
- raw WorkReport section text in errors or `Debug`.

All new errors must use stable codes and static bounded messages. Error strings
must not include caller values, IDs, hashes, paths, reasons, facts, output, or
secret-like markers.

## 16. Compatibility

The implementation must leave unchanged:

- `LocalExecutor::decide_approval(...)`;
- `LocalExecutor::decide_approval_with_presentation(...)`;
- `decide_approval_with_governance_reassessment(...)`;
- `decide_approval_with_governance_reassessment_and_presentation(...)`;
- fresh authoritative dispatcher and report-consumer behavior;
- ordinary executor and CLI behavior;
- workflow schemas and SDKs;
- report artifact and persistence posture.

The new API is additive, local, explicit, and in memory.

## 17. Test Plan

Required focused tests:

1. valid aggregate grant completes and produces a valid `WorkReport`;
2. valid aggregate denial produces a failed terminal report;
3. resumed later step approval returns `DeferredNonTerminal`;
4. decision-time canonical check executes exactly once;
5. request-time result is not reused as terminal authorization evidence;
6. decision-time result reference matches the actual fresh result;
7. terminal report cites the decision-time stable reference;
8. duplicate report reference fails before check execution and mutation;
9. changed current facts fail before approval or resume events;
10. changed immutable bundle fails before mutation;
11. missing or stale presentation proof fails before mutation;
12. resolved execution-context mismatch fails before mutation;
13. aggregate approval does not satisfy a later step approval;
14. no workflow step or skill runs on denial;
15. reference construction failure preserves the returned run;
16. report construction failure preserves workflow status and event history;
17. required `BeforeReport` behavior is exercised directly;
18. zero or multiple decision-time results fail safely if reachable;
19. errors and `Debug` do not leak secret-like inputs;
20. serialization of the generated report does not include raw output fields;
21. existing authoritative route, approval, immutable-bundle, report,
    EvidenceReference, SideEffect, adapter, CLI, and runtime tests pass.

Validation:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

## 18. Implementation Sequence

1. Add the private decision-time reassessment outcome.
2. Refactor the accepted proof-enforced aggregate approval helper to consume
   that outcome without behavior change.
3. Add the explicit report-bearing approval decision request and result.
4. Reuse existing reference and terminal report constructors.
5. Add focused grant, denial, deferred, failure-ordering, and privacy tests.
6. Run full validation.
7. Create an implementation report.
8. Perform a focused maintainer review before CLI or check-profile work.

## 19. Remaining Questions

- Should a future persisted continuation retain both request-time and
  decision-time check references, with explicit temporal roles?
- Should the report contract eventually distinguish `approval_requested_by`
  evidence from `approval_resumed_by` evidence?
- Should chained approval completion persist bounded report inputs, or should
  the operator resupply them through an explicit command?
- Should a generic check-profile phase use an in-process project validator or
  an explicit child-process handler first?

These questions do not block the first local, in-memory completion slice.

## 20. Final Recommendation

Proceed next to the authoritative approval-resume report completion
implementation described here.

The implementation should compose accepted runtime primitives and close the
deferred approval route. It must not add CLI/UI behavior, default execution,
artifacts, persistence, schemas, providers, OpenShell, SideEffect execution,
writes, hosted behavior, or release changes.

## 21. Governed Planning Record

- workflow: `dg/d`
- run: `run-1785093720438070000-2`
- approval:
  `approval/run-1785093720438070000-2/planning-approved`
- presentation: `presentation/d666bd3521c74c44`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- approval-presentation enforcement: proof enforced
- validation: `npm run check:docs` and `git diff --check` passed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- out-of-kernel work: source inspection, architecture analysis, plan
  authoring, and validation
- missing coverage: the kernel coordinates governance only; it did not inspect
  code, edit files, execute validation, or perform git or PR actions
