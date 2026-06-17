# Hook Disclosure Discovery Plan

Status: First in-memory implementation complete. This plan follows the accepted [Executor Hook Disclosure Report Input Propagation Review](../concepts/EXECUTOR_HOOK_DISCLOSURE_REPORT_INPUT_PROPAGATION_REVIEW.md) and was accepted in [Hook Disclosure Discovery Plan Review](../concepts/HOOK_DISCLOSURE_DISCOVERY_PLAN_REVIEW.md). The first implementation derives hook disclosure IDs only from already-validated in-memory `BeforeReport` hook results in the explicit report-bearing executor path, merges them with caller-supplied IDs, and forwards stable IDs through the existing WorkReport citation path. It does not implement discovery from workflow events or audit projections, warning/skipped continuation, blocked behavior, hook optionality, workflow-declared hook configuration, runtime hook configuration, dedicated hook audit sink emission, hook persistence, report artifact writes, CLI behavior, schemas, local check execution, command execution, adapter invocation, approvals, evidence attachment, side effects, writes, reasoning lineage, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 1. Executive Summary

Workflow OS can now cite hook disclosure IDs in WorkReports when callers supply those IDs explicitly, and the explicit `BeforeReport` report-bearing executor path can discover disclosure IDs from the already-validated in-memory hook result it just executed.

The next question is whether any hook disclosure IDs can be discovered safely instead of requiring callers to pass every ID by hand. The answer is yes, but only from bounded sources that already contain validated `AgentHarnessHookDisclosure` values in memory or from a future durable disclosure source that has separately accepted source-of-truth semantics.

This plan defined the allowed sources, rejected sources, first implementation target, error posture, privacy posture, and tests for the first implementation. Broader discovery remains deferred.

## 2. Goals

- Define safe discovery boundaries for `AgentHarnessHookDisclosureId` values.
- Preserve explicit report input propagation for caller-supplied IDs.
- Avoid fabricating disclosure IDs from hook invocation IDs, event counts, audit summaries, report notes, local check results, validation diagnostics, or text.
- Preserve workflow pass/fail semantics.
- Preserve event-log source-of-truth semantics.
- Preserve audit projection boundaries.
- Keep generated WorkReports reference-first.
- Avoid copying disclosure title, summary, references, hook context, audit records, provider payloads, command output, parser output, raw spec contents, paths, tokens, or unbounded prose.
- Prepare a small implementation prompt for in-memory discovery from explicit hook results.

## 3. Non-Goals

This plan does not authorize:

- automatic executor hook invocation;
- broad runtime hook discovery;
- warning continuation;
- skipped-with-disclosure continuation;
- blocked runtime behavior;
- hook optionality;
- workflow-declared hook configuration;
- runtime hook configuration;
- automatic discovery from workflow events that do not carry disclosure IDs;
- automatic discovery from generic audit events that do not carry disclosure IDs;
- automatic discovery from report section text, diagnostics, local check output, adapter telemetry summaries, or operator notes;
- hook disclosure persistence;
- dedicated hook audit sink emission;
- workflow event append behavior changes;
- report artifact writes;
- CLI hook commands or report rendering;
- workflow schema fields;
- automatic local check execution;
- command execution;
- adapter invocation;
- `EvidenceReference` creation or attachment;
- approval request or approval decision creation;
- approval evidence attachment;
- reasoning lineage;
- side-effect boundary implementation;
- writes;
- recursive agents or agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Current Baseline

Implemented:

- bounded `AgentHarnessHookDisclosure` model with validated IDs, kind, severity, title, summary, stable references, redaction metadata, and sensitivity;
- in-memory hook invocation helpers that can return `AgentHarnessHookInvocationResult` values containing disclosures;
- model-only `AgentHarnessHookAuditRecord` values that can contain disclosure context;
- hook workflow event vocabulary for requested/evaluated hook events;
- generic hook workflow event audit projection;
- first explicit `BeforeSkillInvocation` event append path for `Passed` and explicit `FailedClosed`;
- WorkReport citation target and citation kind for `AgentHarnessHookDisclosureId`;
- terminal report helper support for explicitly supplied hook disclosure IDs;
- executor report input propagation for explicitly supplied hook disclosure IDs.

Not implemented:

- automatic discovery of disclosure IDs;
- durable hook disclosure store;
- hook audit record persistence;
- dedicated hook audit sink emission;
- workflow events that carry disclosure IDs;
- warning/skipped/blocked continuation;
- context-aware WorkReport routing based on disclosure kind or severity.

## 5. Discovery Source Inventory

| Candidate source | First implementation? | Classification | Reason |
| --- | --- | --- | --- |
| Caller-supplied `LocalExecutionReportInputs.agent_harness_hook_disclosure_ids` | Already implemented | Keep | Explicit, typed, deterministic input. |
| Caller-supplied `TerminalLocalWorkReportInput.agent_harness_hook_disclosure_ids` | Already implemented | Keep | Explicit, typed, deterministic input. |
| In-memory `AgentHarnessHookInvocationResult.disclosures()` from an explicitly executed hook | Yes | Preferred first target | Contains already-validated disclosures without reading storage or fabricating IDs. |
| In-memory `RuntimeAgentHarnessHookResult` from an explicit runtime hook helper | Yes | Preferred first target | Wraps validated invocation result and audit record in memory. |
| In-memory `AgentHarnessHookAuditRecord` returned with a runtime hook helper result | Defer to same helper slice if needed | Accept only as paired in-memory result | Safe only when produced by the same validated helper call, not as a persisted audit source. |
| `HookInvocationRequested` workflow events | Reject for disclosure discovery today | Unsafe source | Current payload has counts and hook identity, not disclosure IDs. |
| `HookInvocationEvaluated` workflow events | Reject for disclosure discovery today | Unsafe source | Current payload has status and counts, not disclosure IDs. |
| Generic `AuditEvent` projection | Reject for disclosure discovery today | Unsafe source | Projection intentionally omits disclosure payloads and IDs. |
| Report section text | Reject | Unsafe source | Text is not source-of-truth data and may be redacted or user-authored. |
| Validation diagnostics | Reject | Unsafe source | Diagnostics are not hook disclosures. |
| Local check result references | Reject | Unsafe source | Local checks may be cited separately but do not imply hook disclosures. |
| Adapter telemetry references | Reject | Unsafe source | Adapter telemetry may be cited separately but does not imply hook disclosures. |
| Future durable disclosure store | Defer | Possible later source | Needs persistence, retention, redaction, referential integrity, and access policy first. |
| Future hook audit sink/store | Defer | Possible later source | Needs accepted sink/storage semantics before discovery. |

## 6. First Implementation Target Recommendation

Recommended first implementation: **in-memory hook disclosure ID extraction from explicitly executed hook results**.

The smallest useful future code slice should:

1. Add a small helper that extracts `AgentHarnessHookDisclosureId` values from an already-validated `AgentHarnessHookInvocationResult` or `RuntimeAgentHarnessHookResult`.
2. Use that helper only where the executor already explicitly executes a hook in memory, likely the existing `BeforeReport` report-bearing path.
3. Merge discovered IDs with caller-supplied `agent_harness_hook_disclosure_ids`.
4. Deduplicate by ID while preserving stable ordering.
5. Forward the merged list into terminal report generation.

This is not broad automatic discovery. It is local extraction from a value the executor just received from an explicit hook helper call.

Do not discover from workflow events or audit events in the first implementation because current event/audit payloads do not carry disclosure IDs.

## 7. Source-Of-Truth Policy

Discovery must use source-of-truth data, not inference.

Allowed source-of-truth data for the first implementation:

- validated `AgentHarnessHookDisclosure` values already present in an in-memory hook invocation result.

Rejected inference patterns:

- infer disclosure ID from hook invocation ID;
- infer disclosure ID from `input_reference_count` or `output_reference_count`;
- infer disclosure ID from hook status;
- infer disclosure ID from generic audit decision context;
- infer disclosure ID from section text or operator notes;
- infer disclosure ID from validation/local-check/adapter references;
- fabricate a placeholder disclosure ID when a hook reports warning-like text.

If no validated disclosure value is available, no disclosure ID should be discovered.

## 8. Merge And Ordering Policy

When caller-supplied and discovered disclosure IDs both exist:

- preserve caller-supplied IDs first;
- append discovered IDs in hook result order;
- remove duplicates by `AgentHarnessHookDisclosureId`;
- keep ordering deterministic;
- do not treat a duplicate as an error unless future contract enforcement requires uniqueness diagnostics.

The generated report should cite the merged stable ID list through existing WorkReport citation construction.

## 9. Report Section Policy

First implementation should keep section placement unchanged:

- discovered disclosure IDs should be cited in `ValidationAndQualityChecks`;
- do not route to `Risks`, `IncompleteOrDeferredWork`, `PolicyGatesEvaluated`, `DecisionsMade`, or `OperatorHandoffNotes` based only on disclosure IDs;
- do not copy disclosure title or summary into section text;
- do not claim a warning, skip, policy decision, approval, failed gate, or incomplete work item unless a separately accepted model provides that context.

Future routing by disclosure kind and severity remains deferred.

## 10. Workflow Semantics Policy

Discovery must not change workflow semantics.

The future implementation must not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events because a disclosure was discovered;
- emit audit or observability events because a disclosure was discovered;
- create hook disclosures;
- create hook invocation results;
- create hook audit records;
- read or write a hook disclosure store;
- write report artifacts automatically;
- change terminal status;
- change policy decisions;
- change approval behavior;
- change retry, escalation, cancellation, or duplicate-run behavior;
- change `execute(...)`, `decide_approval(...)`, or `cancel_run(...)`.

If discovery fails after a run exists, report-generation failure should remain separate from workflow execution failure.

## 11. Error Handling

Discovery from in-memory hook results should be mostly infallible because disclosure IDs were already validated.

If a future helper encounters invalid or inconsistent data:

- return a stable non-leaking report-generation error;
- preserve the run;
- attach no partial newly discovered disclosure IDs;
- keep caller-supplied IDs only if the implementation can prove atomic report construction remains safe;
- do not convert the discovery error into a workflow diagnostic;
- do not append events or audit records;
- do not leak disclosure title, summary, references, hook context, paths, tokens, command output, provider output, parser payloads, or secret-like values.

Recommended stable error code for unexpected discovery failure:

```text
work_report_generation.hook_disclosure_discovery.failed
```

## 12. Privacy And Redaction

Discovery must remain ID-only at the report boundary.

The first implementation must not copy:

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

Debug output should remain count-only or redacted for executor/report request/result wrappers. WorkReport serialization may include stable disclosure IDs as citation targets, matching the existing WorkReport citation policy.

## 13. Relationship To Warning And Skipped Semantics

Hook disclosure discovery does not authorize warning or skipped continuation.

Discovered disclosure IDs may make a report more complete, but they do not mean:

- a warning is allowed to continue;
- a skipped hook is optional;
- a blocked hook is representable as a runtime status;
- policy allowed continuation;
- approval was requested or granted;
- a side effect was authorized.

Warning/skipped/blocked status broadening still requires separate policy, optionality, event, audit, replay, report, and test work.

## 14. Relationship To Event And Audit Semantics

Do not discover disclosure IDs from current hook workflow events or generic audit projections.

Reasons:

- current hook workflow event payloads carry hook identity, status, counts, redaction metadata, and sensitivity, not disclosure IDs;
- generic audit projection intentionally avoids copying hook payloads or disclosure content;
- deriving disclosure IDs from those sources would fabricate runtime history.

Future event/audit discovery may be reconsidered only after event payloads, audit records, or a disclosure store explicitly carry disclosure IDs with accepted persistence and replay semantics.

## 15. Test Plan

Future implementation tests should cover:

- extraction returns disclosure IDs from a validated in-memory hook result;
- extraction returns an empty list when the hook result has no disclosures;
- extraction preserves hook result disclosure order;
- merge preserves caller-supplied IDs before discovered IDs;
- duplicate caller/discovered IDs are deduplicated deterministically;
- executor `BeforeReport` path can cite disclosures discovered from the explicit in-memory hook result;
- existing explicitly supplied disclosure IDs still work;
- no disclosure IDs are discovered from workflow events;
- no disclosure IDs are discovered from generic audit events;
- generated report cites discovered IDs without copying disclosure title or summary;
- generated report keeps disclosure citations in `ValidationAndQualityChecks`;
- discovery does not mutate run state;
- discovery does not append events;
- discovery does not emit audit or observability records;
- discovery does not write report artifacts automatically;
- discovery failure, if modeled, preserves the run and returns non-leaking report-generation error;
- Debug output does not leak disclosure IDs, titles, summaries, references, paths, tokens, or raw payload markers;
- existing hook, WorkReport, executor, audit projection, EvidenceReference, Diagnostic, validation, local-check, adapter telemetry, and runtime tests still pass.

## 16. Proposed Implementation Sequence

1. Add a pure helper for extracting disclosure IDs from validated in-memory hook results.
2. Add deterministic merge/deduplication helper for caller-supplied and discovered IDs.
3. Wire only the explicit `BeforeReport` report-bearing path to merge IDs from the in-memory hook result it already executes.
4. Add focused tests for extraction, merge ordering, report citation behavior, non-mutation, and non-leakage.
5. Review.
6. Only after review, consider whether durable event/audit/disclosure-store discovery should be planned.

## 17. Deferred Work

Deferred:

- discovery from workflow events;
- discovery from audit sink records;
- durable hook disclosure store;
- hook audit record persistence;
- dedicated hook audit sink emission;
- warning continuation;
- skipped-with-disclosure continuation;
- blocked runtime behavior;
- hook optionality;
- context-aware WorkReport section routing;
- workflow-declared hook configuration;
- runtime hook configuration;
- report artifact writes;
- CLI rendering;
- schemas;
- approvals;
- evidence attachment;
- side effects;
- writes;
- reasoning lineage.

## 18. Final Recommendation

Recommended next implementation phase: **in-memory hook disclosure discovery helper for explicit `BeforeReport` hook results**.

That phase should extract only stable disclosure IDs from already-validated in-memory hook results, merge them with caller-supplied report input IDs, and forward them through the existing WorkReport citation path. It must not discover from workflow events or audit projections, persist disclosures, append events, write artifacts, broaden hook statuses, implement warning/skipped continuation, add schemas, add CLI behavior, add side effects, add writes, or change release posture.
