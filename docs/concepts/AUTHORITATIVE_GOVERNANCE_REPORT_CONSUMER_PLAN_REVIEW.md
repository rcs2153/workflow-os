# Authoritative Governance Report Consumer Plan Review

Status: Plan accepted after one planning blocker correction.

## 1. Executive Verdict

Plan accepted; proceed to the explicit in-memory authoritative governance
report consumer implementation.

The plan selects a product-relevant but bounded first dispatcher consumer. It
connects one authoritative governance route result to the existing terminal
WorkReport boundary without changing default execution, flattening route truth,
or authorizing persistence and providers.

The initial draft contained one planning blocker: it permitted local-check
reference construction to return top-level `Err` after the dispatcher may have
already created a durable run. The plan is corrected so every post-route
reference or report failure preserves and returns the route result and run with
`GenerationFailed`.

## 2. Scope Assessment

The plan remains within the accepted narrow-consumer scope:

- explicit and opt-in;
- local and in-memory;
- fresh-run-only;
- `DocsCheck`-only;
- one dispatcher invocation;
- existing WorkReport generation boundary;
- no default executor behavior.

It does not authorize:

- CLI or UI behavior;
- schemas, SDK changes, scaffolds, or examples;
- report artifacts or persistence;
- providers or OpenShell;
- SideEffect execution or writes;
- hosted behavior;
- automatic approval;
- retry or resume support;
- release changes.

## 3. Consumer Choice Assessment

The explicit dispatcher-plus-report helper is a better first consumer than a
method that merely forwards to the dispatcher.

It proves a useful runtime chain:

```text
authoritative route
  -> real run posture
  -> payload-free local-check reference
  -> terminal governed handoff report
```

The free-function shape keeps every execution dependency explicit and avoids
making the path ambient on `LocalExecutor`.

## 4. Authority Assessment

The plan preserves the accepted authority hierarchy:

- caller supplies workflow, typed facts, report identity, and bounded
  dependencies;
- the dispatcher derives and selects the route;
- the actual same-call `DocsCheck` result supplies command and status facts;
- report generation derives from the returned run;
- no caller field can replace route or check truth.

The consumer calls the dispatcher exactly once and does not call
`execute_with_report(...)`, so it cannot rerun the workflow or check.

## 5. Route Semantics Assessment

The route enum remains the primary result:

- quiet remains quiet;
- visible retains its delivery receipt;
- approval required remains waiting and report-deferred;
- denial remains a terminal `PolicyDenied` route.

The proposed report posture is correctly separate:

- `Generated`;
- `DeferredNonTerminal`;
- `GenerationFailed`.

Approval pending is not mislabeled as report failure. Denial is not mislabeled
as an approval outcome or ordinary success.

## 6. Report Semantics Assessment

Terminal report generation reuses the existing helper and warning-style failure
policy. The plan correctly keeps reports derived and in memory:

- no post-terminal events;
- no mutation of `WorkflowRun`;
- no report state in the backend;
- no artifact write;
- no false claim of report persistence.

Extracting one private already-run report helper is an acceptable implementation
refactor only if existing `execute_with_report(...)` behavior remains
byte-for-byte equivalent at its public boundary.

## 7. Local-Check Evidence Assessment

The plan closes a meaningful truth gap. A dispatcher consumer that generated a
report without citing the check it just executed could report validation
references as unavailable.

The proposed reference input appropriately leaves these facts to Core:

- command ID and kind;
- status;
- workflow ID;
- run ID.

The caller supplies only stable reference identity and optional already-existing
event, audit, output-reference, redaction, and sensitivity metadata.

The consumer must use `LocalCheckResultReference::from_result(...)` and the
existing WorkReport stable-reference path. It must not create command-output
evidence or copy output summaries.

## 8. Failure Assessment

### Corrected Planning Blocker

The initial plan said invalid reference construction after the route returned
would produce top-level `Err`. Because the dispatcher may already have appended
events and completed or paused a durable run, that behavior could hide execution
truth from the caller.

The corrected rule is:

- caller-input failures detectable before dispatch may return `Err`;
- dispatcher failures before a route exists may return `Err`;
- after a route exists, reference and report failures return the route result,
  no report, optional no reference, `GenerationFailed`, and a structured error.

This matches the established `execute_with_report(...)` policy and prevents an
ambiguous completed-run/error boundary.

No planning blockers remain after this correction.

## 9. Privacy Assessment

The plan uses existing constructors and forbids:

- raw command output;
- command transcripts;
- raw source or spec contents;
- parser or provider payloads;
- environment values;
- credentials and secret-like values;
- approval presentation text;
- disclosure payloads.

`Debug` is limited to bounded route, status, posture, count, and error-code
fields. The report and reference remain sensitivity- and redaction-aware.

## 10. Test Plan Assessment

The planned tests cover:

- all four routes;
- one dispatcher/check invocation;
- route preservation;
- approval deferral;
- denial reporting;
- actual-result reference binding;
- citation propagation;
- duplicate handling;
- post-route invariant failure;
- report failure without run mutation;
- hook preservation;
- event immutability;
- privacy and non-leakage;
- full regression validation.

The implementation review should specifically verify that a post-route
reference failure returns the exact run and event history instead of
top-level `Err`.

## 11. Blockers

None after the plan correction.

## 12. Non-Blocking Follow-Ups

- Decide when approval-resume can produce a terminal report without weakening
  reassessment and presentation-proof requirements.
- Plan durable local-check result references separately.
- Add a dedicated governance-assessment WorkReport citation only after a stable
  citation target is designed.
- Keep operator rendering and OpenShell behind later accepted boundaries.

## 13. Recommended Next Phase

Implement the explicit in-memory authoritative governance report consumer.

The implementation should include the request/result/posture/reference-input
models, one already-run report helper refactor if required, one dispatcher
invocation, actual-result reference construction, focused tests, full
validation, an implementation report, and focused maintainer review.

Do not add default executor behavior, CLI/UI exposure, schemas, examples,
artifacts, persistence, providers, OpenShell, SideEffect execution, writes,
hosted behavior, or release changes.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1785087748218452000-2`
- approval:
  `approval/run-1785087748218452000-2/review-scope-approved`
- presentation: `presentation/e8c6ac0aed485a2c`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- validation: `npm run check:docs` and `git diff --check` passed after the
  planning blocker correction
- out-of-kernel work: plan inspection, maintainer review, plan correction,
  documentation edits, and validation
- missing coverage: the kernel coordinated governance only; it did not edit
  files, execute validation, create a WorkReport artifact, or perform git or PR
  actions
