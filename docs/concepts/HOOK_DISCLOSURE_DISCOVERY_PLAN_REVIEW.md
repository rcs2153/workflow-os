# Hook Disclosure Discovery Plan Review

Review date: 2026-06-17

## 1. Executive Verdict

Plan accepted; proceed to in-memory hook disclosure discovery implementation.

The plan defines a narrow, source-of-truth-safe path for discovering `AgentHarnessHookDisclosureId` values from already-validated in-memory hook results. It correctly rejects inference from workflow events, generic audit projections, report text, diagnostics, local check output, adapter telemetry, and operator notes.

The next implementation should remain small: extract disclosure IDs from explicit in-memory hook results, merge them with caller-supplied IDs, deduplicate deterministically, and forward the merged list through the existing WorkReport citation path.

## 2. Scope Verification

The plan stayed within planning-only scope.

It does not authorize:

- runtime implementation in the plan phase;
- broad automatic executor hook invocation;
- warning continuation;
- skipped-with-disclosure continuation;
- blocked runtime behavior;
- hook optionality;
- workflow-declared hook configuration;
- runtime hook configuration;
- discovery from current workflow events;
- discovery from generic audit projections;
- discovery from report text, diagnostics, local check results, adapter telemetry, or operator notes;
- hook disclosure persistence;
- dedicated hook audit sink emission;
- workflow event append behavior changes;
- report artifact writes;
- CLI behavior;
- schemas;
- local check execution;
- command execution;
- adapter invocation;
- approval creation or approval evidence attachment;
- side effects;
- writes;
- reasoning lineage;
- recursive agents or agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

No accidental authorization of these items was found.

## 3. Baseline Assessment

The plan accurately describes the current baseline:

- the bounded `AgentHarnessHookDisclosure` model exists;
- in-memory hook invocation helpers can return disclosures;
- model-only hook audit records exist;
- hook workflow event vocabulary exists;
- generic hook workflow event audit projection exists;
- the first explicit `BeforeSkillInvocation` event append path exists for `Passed` and explicit `FailedClosed`;
- WorkReport citation target vocabulary exists for hook disclosure IDs;
- terminal report helper support exists for explicitly supplied hook disclosure IDs;
- executor report input propagation exists for explicitly supplied hook disclosure IDs.

The plan also accurately states what is not implemented: automatic disclosure discovery, durable disclosure storage, hook audit persistence, dedicated hook audit sink emission, workflow events carrying disclosure IDs, warning/skipped/blocked continuation, and context-aware report routing.

## 4. Discovery Source Assessment

The discovery source inventory is sound.

Accepted sources:

- caller-supplied report input IDs remain the explicit source and are already implemented;
- already-validated in-memory `AgentHarnessHookInvocationResult` disclosures are appropriate for the first implementation target;
- already-validated in-memory `RuntimeAgentHarnessHookResult` disclosures are appropriate when produced by the same explicit helper call.

Rejected sources:

- current `HookInvocationRequested` and `HookInvocationEvaluated` workflow events do not carry disclosure IDs, so discovery from those events would fabricate history;
- generic `AuditEvent` projection intentionally omits disclosure IDs and payloads;
- report section text, diagnostics, local check output, adapter telemetry summaries, and operator notes are not source-of-truth disclosure records.

Deferred sources:

- durable disclosure stores and future hook audit sinks require separate persistence, retention, redaction, referential integrity, and access-policy decisions before they can become discovery sources.

## 5. First Implementation Target Assessment

The recommended first target is appropriately low risk: in-memory hook disclosure ID extraction from explicitly executed hook results.

This target is useful because it removes manual report input duplication after a hook result already contains validated disclosure values. It is safe because it does not read storage, infer from text, append events, or create new disclosure records.

The first implementation should wire only the explicit `BeforeReport` report-bearing path if that path already has an in-memory hook result available. Broader executor checkpoints, event/audit discovery, durable disclosure stores, and section routing remain deferred.

## 6. Source-Of-Truth And Merge Policy Assessment

The plan correctly requires source-of-truth data rather than inference.

The merge policy is deterministic and conservative:

- caller-supplied IDs first;
- discovered IDs appended in hook result order;
- duplicates removed by `AgentHarnessHookDisclosureId`;
- stable ordering preserved;
- duplicate IDs are not errors in this phase.

This is compatible with the existing explicit report input behavior and avoids surprising report citation churn.

## 7. Report Behavior Assessment

The report behavior boundary is appropriate.

Discovered disclosure IDs should continue to be cited in `ValidationAndQualityChecks` through the existing WorkReport citation path. The implementation should not copy disclosure titles, summaries, references, hook context, or audit records into report sections.

The plan correctly defers context-aware routing to `Risks`, `IncompleteOrDeferredWork`, `PolicyGatesEvaluated`, `DecisionsMade`, or `OperatorHandoffNotes`. Disclosure IDs alone do not prove a risk, policy decision, approval, skipped item, failed gate, or incomplete work item.

## 8. Workflow Semantics Assessment

The plan preserves workflow semantics.

The future implementation must not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events;
- emit audit or observability events;
- create hook disclosures;
- create hook invocation results;
- create hook audit records;
- read or write a disclosure store;
- write report artifacts;
- change terminal status;
- change policy decisions;
- change approval behavior;
- change retry, escalation, cancellation, or duplicate-run behavior;
- change `execute(...)`, `decide_approval(...)`, or `cancel_run(...)`.

This is the correct boundary for a report-helper discovery slice.

## 9. Error Handling Assessment

The error handling posture is conservative.

Discovery should be mostly infallible when operating on already-validated in-memory hook results. If unexpected inconsistency is encountered, the plan requires a stable, non-leaking report-generation error with the recommended code:

```text
work_report_generation.hook_disclosure_discovery.failed
```

The plan also correctly requires that discovery failure preserve the run, avoid misleading workflow diagnostics, avoid event/audit writes, and avoid leaking disclosure titles, summaries, references, hook context, paths, tokens, command output, provider payloads, parser payloads, or secret-like values.

## 10. Privacy And Redaction Assessment

The privacy posture is strong.

The future implementation is ID-only at the report boundary. It must not copy:

- hook disclosure title or summary;
- hook disclosure references;
- hook disclosure redaction metadata;
- hook input references;
- hook output references;
- supplemental hook references;
- hook audit records;
- hook invocation context;
- workflow/run/actor context from hook records beyond existing report identity;
- raw prompts;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

Stable disclosure IDs may appear as WorkReport citation targets under the existing citation policy.

## 11. Relationship To Warning, Skipped, Event, And Audit Semantics

The plan correctly separates discovery from warning/skipped/blocked semantics.

Discovered disclosure IDs do not mean:

- a warning is allowed to continue;
- a skipped hook is optional;
- blocked is a runtime terminal status;
- policy allowed continuation;
- approval was requested or granted;
- side effects were authorized.

The plan also correctly rejects discovery from current workflow events and generic audit projections because those payloads do not carry disclosure IDs. Event/audit discovery should be reconsidered only after accepted event payload, audit sink, or disclosure-store semantics exist.

## 12. Test Plan Assessment

The planned tests are adequate for the next implementation.

They cover:

- extraction from validated in-memory hook results;
- empty disclosure results;
- disclosure order preservation;
- caller-supplied plus discovered merge ordering;
- deterministic deduplication;
- explicit `BeforeReport` citation behavior;
- existing caller-supplied ID behavior;
- no discovery from workflow events;
- no discovery from generic audit events;
- no title or summary copying;
- `ValidationAndQualityChecks` section placement;
- non-mutation;
- no event, audit, observability, or artifact writes;
- non-leaking discovery failure behavior;
- Debug non-leakage;
- existing hook, WorkReport, executor, audit projection, EvidenceReference, Diagnostic, validation, local-check, adapter telemetry, and runtime tests.

No planning-level blocker test gap was found. The implementation review should confirm the tests assert non-discovery from event/audit inputs rather than merely omitting those paths.

## 13. Documentation Review

The plan is honest about current capabilities.

It states discovery is planned, not implemented. It preserves the existing explicit-ID behavior and keeps warning/skipped continuation, blocked runtime behavior, workflow-declared hook configuration, runtime hook configuration, persistence, CLI behavior, schemas, side effects, writes, and reasoning lineage out of scope.

No documentation correction is required before implementation.

## 14. Planning Blockers

None.

## 15. Non-Blocking Follow-Ups

- After the first implementation review, decide whether section routing by disclosure kind/severity needs a separate plan.
- If event/audit payloads later carry disclosure IDs, plan event/audit discovery with replay, retention, and redaction semantics before implementation.
- Keep warning/skipped/blocked semantics separate from discovery.
- Keep durable hook disclosure store planning separate from report-helper discovery.
- Decide later whether discovered disclosure IDs should be exposed in runtime result wrappers for inspection beyond WorkReport citations.

## 16. Recommended Next Phase

Recommended next phase: in-memory hook disclosure discovery implementation.

The implementation should add a pure extraction helper for validated in-memory hook results, deterministic merge/deduplication of caller-supplied and discovered IDs, and wiring only for the explicit `BeforeReport` report-bearing path. It must not discover from workflow events or audit projections, persist disclosures, append events, write artifacts, broaden hook statuses, implement warning/skipped continuation, add schemas, add CLI behavior, add side effects, add writes, or change release posture.

## 17. Validation

- `npm run check:docs` - passed.
