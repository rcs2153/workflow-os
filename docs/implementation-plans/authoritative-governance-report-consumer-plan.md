# Authoritative Governance Report Consumer Plan

Status: Implemented; awaiting focused phase review.

Related foundations:

- [Authoritative Proportional-Governance Route Dispatcher Plan](authoritative-proportional-governance-route-dispatcher-plan.md)
- [Authoritative Proportional-Governance Route Dispatcher Review](../concepts/AUTHORITATIVE_PROPORTIONAL_GOVERNANCE_ROUTE_DISPATCHER_REVIEW.md)
- [Executor-Integrated Report Result Plan](executor-integrated-report-result-plan.md)
- [Terminal Local Report Generation Plan](terminal-local-report-generation-plan.md)
- [Terminal Report Local Check Citation Integration Plan](terminal-report-local-check-citation-integration-plan.md)
- [Command Output Evidence Policy Plan](command-output-evidence-policy-plan.md)

## 1. Executive Summary

Workflow OS now has one accepted authoritative dispatcher for the explicit
fresh-run local `DocsCheck` slice. The dispatcher prepares the immutable run
bundle, executes the canonical check, derives one complete source-bound
proportional-governance assessment, and selects quiet proceed, visible proceed,
approval required, or denial without caller route choice.

The dispatcher is not yet consumed by a product-relevant explicit runtime
surface. The next narrow phase should add one in-memory report-bearing consumer
that:

- calls the dispatcher exactly once;
- preserves the route-specific result;
- generates a `WorkReport` only when the selected route returns a terminal run;
- treats an approval-pending or otherwise non-terminal run as report-deferred,
  not as report failure;
- creates one validated, payload-free local-check result reference from the
  actual same-call `DocsCheck` result and explicit caller metadata; and
- returns report-generation failure separately without rewriting workflow
  semantics.

This is an additive opt-in helper. It is not default executor integration and
does not add CLI or UI behavior, schemas, examples, report artifacts,
persistence, providers, OpenShell, SideEffect execution, writes, hosted
behavior, or release changes.

## 2. Product Decision

The first explicit consumer should compose authoritative governance with the
existing in-memory report foundation.

The selected boundary is:

```text
explicit caller
  -> authoritative DocsCheck dispatcher
  -> route-specific WorkflowRun result
  -> terminal-status check
  -> validated local-check result reference
  -> existing terminal WorkReport helper
  -> route-preserving in-memory result
```

This is more useful than adding another method that only renames or forwards to
the dispatcher. It proves that the governance decision, independently executed
check, and governed handoff report can remain connected without introducing
automatic execution or persistence.

The caller still supplies report identity and bounded report context. The
caller does not select the governance route, assert the check outcome, or
fabricate a local-check result reference.

## 3. Goals

- Add one explicit local in-memory consumer of the authoritative dispatcher.
- Preserve quiet, visible, approval-required, and denied route truth.
- Call the dispatcher once and never rerun the canonical `DocsCheck`.
- Generate a report only for a terminal route result.
- Treat expected non-terminal posture as report deferred.
- Preserve report-generation errors separately from workflow outcomes.
- Construct a validated local-check result reference from the actual
  dispatcher result and explicit reference metadata.
- Cite that reference in the report validation and quality-check section.
- Reuse existing report constructors, hook checkpoint behavior, redaction
  gates, and terminal status rules.
- Keep all existing executor and dispatcher APIs unchanged.

## 4. Non-Goals

The phase must not add:

- default invocation from `LocalExecutor::execute(...)`;
- automatic report generation for every run;
- CLI, UI, workflow-schema, SDK, scaffold, or example exposure;
- report artifacts, filesystem output, or report persistence;
- automatic citation discovery from stores or event history;
- raw command-output evidence or copied stdout/stderr;
- a new proportional-governance mode or caller-selected route;
- automatic approval, model self-approval, or approval bypass;
- approval-resume or cancellation report-bearing methods;
- retry, recovery, or existing-run support for the dispatcher;
- providers, OpenShell, sandbox lifecycle, credentials, or network access;
- SideEffect execution, provider mutation, or new write families;
- hosted behavior, enterprise administration, reasoning lineage, or release
  changes.

## 5. Exact Opt-In Caller

Add one free function adjacent to the accepted dispatcher and explicit report
helpers:

```text
execute_with_authoritative_docs_check_governance_report(...)
    -> Result<
        LocalExecutionWithAuthoritativeGovernanceReportResult,
        WorkflowOsError
    >
```

The function should accept:

- `LocalExecutor`;
- `LocalImmutableRunBundleStore`;
- explicit `DocsCheckLocalHandler`;
- optional visible-proceed dependencies accepted by the dispatcher;
- one `LocalExecutionWithAuthoritativeGovernanceReportRequest`.

It should call `route_authoritative_docs_check_governance(...)` exactly once.

Do not add an identically behaving method to `LocalExecutor` in the first
slice. A free function matches the current explicit composition APIs, keeps
ambient executor behavior unchanged, and makes every extra dependency visible
at the call site.

## 6. Candidate Request Model

Add:

```text
LocalExecutionWithAuthoritativeGovernanceReportRequest
```

Candidate fields:

- `execution: LocalExecutionWithAuthoritativeDocsCheckGovernanceRequest`;
- `report: LocalExecutionReportInputs`;
- `local_check_reference:
  AuthoritativeDocsCheckReportReferenceInputs`.

The request must not contain:

- a route enum;
- a governance execution or disclosure disposition;
- a check result or check status;
- a prebuilt `LocalCheckResultReference`;
- a caller-supplied report terminal status;
- raw command output;
- a report artifact path;
- provider or sandbox configuration.

## 7. Local-Check Reference Input

The existing dispatcher returns the bounded `LocalCheckResult` produced by the
same-call canonical `DocsCheck`, but terminal report generation currently
accepts only supplied stable report references.

Add the smallest explicit metadata input needed to construct a validated
reference from the real result:

```text
AuthoritativeDocsCheckReportReferenceInputs
```

Candidate fields:

- `result_id: LocalCheckResultId`;
- `workflow_event_id: Option<EventId>`;
- `audit_event_id: Option<EventId>`;
- `output_reference: Option<String>`;
- `redaction: RedactionMetadata`;
- `sensitivity: WorkReportSensitivity`.

Core must derive from the selected route result:

- command ID;
- command kind;
- check status;
- workflow ID;
- run ID.

Core should call `LocalCheckResultReference::from_result(...)`. The caller may
provide stable reference identity and optional already-existing event, audit,
or output references, but cannot replace the real check result fields.

The first `DocsCheck` slice must produce exactly one local-check result. Zero or
multiple results fail closed with stable non-leaking consumer error codes.

## 8. Report Citation Construction

After constructing the validated `LocalCheckResultReference`, Core should
derive the existing `WorkReportStableReference` from its result ID and append
it to a cloned report input.

Rules:

- preserve caller-supplied local-check report references;
- reject duplicate stable references rather than silently emit duplicate
  citations;
- never copy local-check output summaries into the report;
- never create an `EvidenceReference` implicitly;
- never claim command-output evidence;
- cite the reference in `ValidationAndQualityChecks` through the existing
  terminal report helper;
- retain the complete local-check result only in the route result already
  returned by the dispatcher.

This is automatic reference construction from an independently produced result,
not automatic check discovery or automatic command-output evidence.

## 9. Candidate Result Model

Add:

```text
LocalExecutionWithAuthoritativeGovernanceReportResult
```

Required fields:

- `route:
  LocalExecutionWithAuthoritativeGovernanceRouteResult`;
- `report_posture:
  AuthoritativeGovernanceReportPosture`;
- `work_report: Option<WorkReport>`;
- `report_generation_error: Option<WorkflowOsError>`;
- `local_check_result_reference: Option<LocalCheckResultReference>`.

Candidate report posture:

- `Generated`;
- `DeferredNonTerminal`;
- `GenerationFailed`.

The result must retain the original route enum. It must not flatten:

- approval required into success;
- denial into ordinary failure without denial route identity;
- visible proceed into quiet proceed;
- report generation failure into workflow failure.

Read-only accessors may expose the route, run, report posture, optional report,
optional report error, and local-check result reference.

The optional reference is required when report posture is `Generated`. It may
be absent only when post-route reference or report construction failed. A
post-route failure must preserve and return the already-created route result
and run.

## 10. Route-Specific Report Behavior

### Quiet Proceed

- Preserve the quiet route result.
- If the run is terminal, generate the report.
- If ordinary execution returns non-terminal, return `DeferredNonTerminal`.
- Do not create visible-disclosure posture.

### Visible Proceed

- Preserve the visible route result and disclosure receipt.
- Generate a report only after the route has delivered the disclosure and the
  run is terminal.
- Do not reinterpret surface acceptance as human acknowledgement or approval.

### Approval Required

- Preserve the approval-required route and aggregate approval binding.
- Return `DeferredNonTerminal`.
- Do not generate a terminal report while the run waits for approval.
- Do not return a report-generation error merely because approval is pending.
- Approval decision and resume report integration remain deferred.

### Denied

- Preserve the denied route and `PolicyDenied` terminal run.
- Generate a failed terminal WorkReport through the existing helper.
- The report may cite the local-check result reference and existing run/event
  references supplied by the caller.
- Do not invent an approval denial, provider denial, or SideEffect record.

## 11. Report Generation Boundary

The consumer should reuse the existing report-generation behavior currently
embedded in `LocalExecutor::execute_with_report(...)`.

If needed, extract one private helper that:

1. receives an already-produced `WorkflowRun`;
2. applies the existing `BeforeReport` hook checkpoint to a cloned report
   input;
3. constructs terminal report input;
4. calls `expose_terminal_local_work_report_result(...)`; and
5. returns report or structured report-generation error without mutating the
   run.

Both `execute_with_report(...)` and the new consumer should use that helper.
The refactor must preserve existing behavior and tests.

The new consumer must not call `LocalExecutor::execute_with_report(...)`
because that would execute the workflow a second time.

## 12. Failure Semantics

Failures before the dispatcher returns a route result remain `Err`:

- invalid or duplicate derived stable reference identity that can be detected
  from caller inputs before execution;
- immutable bundle failure;
- canonical check failure;
- incomplete or invalid assessment;
- visible dependency mismatch;
- disclosure delivery failure;
- route enforcement failure;
- state append failure.

After a route result exists:

- invalid local-check reference construction returns the route result with
  `GenerationFailed`, no report, no reference, and a structured report error;
- an impossible zero-or-multiple local-check result invariant returns the route
  result with `GenerationFailed` rather than hiding a run that already exists;
- non-terminal run returns a successful result with
  `DeferredNonTerminal`;
- terminal report construction failure returns the route result with
  `GenerationFailed`, no report, and a structured report error;
- report failure must not rewrite run status or append workflow events.

Stable consumer error codes should use:

```text
executor.authoritative_local_check.report_consumer.*
```

Errors must not contain identifiers, paths, fingerprints, check output,
approval context, disclosure metadata, report text, or secret-like values.

Once the dispatcher has returned a route result, the consumer must not return
top-level `Err`. This matches `LocalExecutor::execute_with_report(...)`:
execution truth remains available even when derived report work fails.

## 13. Event And State Semantics

The consumer adds no workflow event kind.

It must preserve dispatcher ordering:

- visible delivery, when selected;
- immutable and governance binding persistence;
- ordinary run start;
- route-specific execution, approval pause, or denial.

Report generation remains derived in-memory behavior:

- no post-terminal event append;
- no report state in `WorkflowRun`;
- no report write to `StateBackend`;
- no audit or observability event claiming report persistence;
- no report artifact creation.

The local-check result reference is returned in memory and cited in the report.
Persistence of that reference remains separately scoped.

## 14. Privacy And Redaction

The consumer must not store or copy:

- raw stdout or stderr;
- command transcripts;
- raw spec or source contents;
- parser payloads;
- provider payloads;
- raw CI logs;
- environment values;
- credentials, authorization headers, private keys, or token-like values;
- disclosure surface payloads;
- approval presentation text.

It must:

- use existing report and local-check reference constructors;
- preserve WorkReport redaction metadata validation;
- reject secret-like reference metadata;
- keep `Debug` output limited to route, run status, report posture, counts,
  and optional error code;
- treat the WorkReport as sensitive even when its citations are payload-free.

## 15. Compatibility

Keep unchanged:

- `LocalExecutor::execute(...)`;
- `LocalExecutor::execute_with_report(...)`;
- all four exact route APIs;
- `route_authoritative_docs_check_governance(...)`;
- `LocalExecutionWithAuthoritativeGovernanceRouteResult`;
- terminal status semantics;
- existing report helper inputs and results;
- default local handler registration posture;
- workflow schemas and CLI behavior.

The new request, result, posture, reference-input model, and free function are
additive experimental Rust API.

## 16. Test Plan

Future implementation tests must prove:

1. quiet terminal proceed returns the quiet route and generated report;
2. visible terminal proceed preserves the disclosure receipt and generated
   report;
3. approval-required returns the exact waiting route and
   `DeferredNonTerminal`;
4. approval-required does not create a report or report error;
5. denied returns the denied route and a failed terminal report;
6. the dispatcher and canonical `DocsCheck` run exactly once;
7. all route-specific results remain matchable without semantic flattening;
8. the local-check reference is built from the actual result;
9. caller cannot replace command ID, kind, status, workflow ID, or run ID;
10. the generated report cites the local-check result ID;
11. raw check output is absent from report, result `Debug`, serialization, and
    errors;
12. duplicate supplied/derived report references detected before dispatch fail
    closed without execution;
13. zero or multiple post-route check results preserve the route and return
    `GenerationFailed`;
14. ordinary non-terminal execution returns `DeferredNonTerminal`;
15. terminal report construction failure returns `GenerationFailed` without
    changing run status or event history;
16. existing `BeforeReport` checkpoint behavior remains active;
17. report generation appends no post-terminal events;
18. no report artifact, filesystem output, or report persistence occurs;
19. secret-like reference metadata fails without leakage and never hides an
    already-created run;
20. existing dispatcher, route, report, local-check, approval, denial,
    EvidenceReference, Diagnostic, adapter, and runtime tests pass;
21. `cargo test --workspace` passes.

## 17. Implementation Sequence

1. Add request, local-check reference input, report posture, and result models.
2. Extract or reuse one private already-run report-generation helper.
3. Implement the explicit dispatcher-plus-report consumer.
4. Construct and append the validated local-check stable reference.
5. Add focused route, report, reference, failure, event, and privacy tests.
6. Run full validation.
7. Perform focused maintainer review before any default or operator-facing
   integration.

Do not split the first implementation into route-specific public consumer
functions. One point of this phase is to preserve dispatcher authority.

## 18. Open Questions

- Should report-generation failure after a denied route remain warning-style,
  matching `execute_with_report(...)`, or should the explicit consumer expose a
  stricter contract option later?
- Should a future approval-resume consumer reuse the original report identity
  or require a fresh report request after terminal completion?
- When should local-check result references become durable artifacts or event
  projections?
- Should the generated WorkReport cite the governance assessment binding once a
  dedicated stable citation target exists?
- Which operator surface should eventually render the four route variants
  without becoming a policy authority?

These questions do not block the first explicit in-memory consumer.

## 19. Relationship To OpenShell

OpenShell remains a later optional execution substrate behind Workflow OS
authorization.

The consumer planned here proves that authorized execution results can remain
connected to bounded check evidence and a WorkReport. It does not add a
sandbox-provider interface or authorize OpenShell execution.

A later provider-neutral sandbox boundary may return sandbox ID, image digest,
effective policy revision and hash, completion status, denial-event
references, durable log references, and artifact references. Those values must
enter Workflow OS as bounded evidence after authoritative routing and scoped
capability resolution; they must not select or weaken the route.

Do not fork or integrate OpenShell in this phase.

## 20. Final Recommendation

Implemented one additive
`execute_with_authoritative_docs_check_governance_report(...)` consumer,
in-memory and fresh-run-only.

The implementation should preserve the dispatcher result, derive one validated
local-check result reference from the actual same-call result, and generate a
WorkReport only for terminal route outcomes. Approval-pending and other
non-terminal results should remain explicitly deferred.

Do not add default executor behavior, CLI/UI exposure, schemas, examples,
artifacts, persistence, providers, OpenShell, SideEffect execution, writes,
hosted behavior, or release changes.

## 21. Governed Planning Record

- workflow: `dg/d`
- run: `run-1785087554537199000-2`
- approval:
  `approval/run-1785087554537199000-2/planning-approved`
- presentation: `presentation/595c54293a61e6b1`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- validation: `npm run check:docs` and `git diff --check` passed
- out-of-kernel work: architecture inspection, plan authoring, documentation
  edits, and validation
- missing coverage: the kernel coordinated governance only; it did not edit
  files, execute validation, create a WorkReport artifact, or perform git or PR
  actions

## 22. Implementation Status

The first explicit consumer is implemented in `workflow-core`.

It:

- calls `route_authoritative_docs_check_governance(...)` exactly once;
- preserves quiet, visible, approval-required, and denied route variants;
- creates a validated `LocalCheckResultReference` from the actual same-call
  result;
- cites that reference in terminal WorkReports;
- defers report generation without error for approval-pending runs;
- preserves the route and run when post-route reference or report generation
  fails; and
- leaves existing executor and dispatcher APIs unchanged.

The implementation remains explicit, local, in-memory, and fresh-run-only.
Default invocation, CLI/UI exposure, schemas, examples, artifacts,
persistence, providers, OpenShell, SideEffect execution, writes, hosted
behavior, and release changes are not implemented.

See
[Authoritative Governance Report Consumer Report](../concepts/AUTHORITATIVE_GOVERNANCE_REPORT_CONSUMER_REPORT.md)
for the implementation record.
