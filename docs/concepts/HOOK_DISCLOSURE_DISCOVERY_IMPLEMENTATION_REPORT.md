# Hook Disclosure Discovery Implementation Report

## 1. Executive Summary

The first hook disclosure discovery slice is implemented.

`LocalExecutor::execute_with_report(...)` now discovers `AgentHarnessHookDisclosureId` values from the already-validated in-memory `BeforeReport` hook result it explicitly executed for report-bearing local execution. The executor merges those discovered IDs with caller-supplied hook disclosure IDs and forwards the stable ID list through the existing WorkReport citation path.

This is not broad automatic discovery. It does not discover from workflow events, generic audit projections, report text, diagnostics, local check output, adapter telemetry, durable stores, or operator notes.

## 2. Scope Completed

Completed:

- extracted hook disclosure IDs from validated in-memory `BeforeReport` hook results;
- preserved caller-supplied disclosure IDs;
- appended discovered disclosure IDs in hook result order;
- deduplicated caller-supplied and discovered disclosure IDs deterministically;
- cited merged disclosure IDs through existing `WorkReportCitationTarget::AgentHarnessHookDisclosure`;
- kept citations in `ValidationAndQualityChecks`;
- preserved existing hook invocation ID citation behavior;
- preserved workflow run state and event history;
- added focused executor tests for discovery, merge ordering, deduplication, non-leakage, and no artifact writes;
- updated roadmap and concept documentation honestly.

## 3. Scope Explicitly Not Completed

Not implemented:

- discovery from workflow events;
- discovery from generic audit projections;
- durable hook disclosure store;
- hook audit record persistence;
- dedicated hook audit sink emission;
- workflow events carrying disclosure IDs;
- warning continuation;
- skipped-with-disclosure continuation;
- blocked runtime behavior;
- hook optionality;
- context-aware WorkReport section routing;
- workflow-declared hook configuration;
- runtime hook configuration;
- report artifact writes;
- CLI rendering or hook commands;
- schemas;
- approvals or approval evidence attachment;
- evidence attachment;
- side effects;
- writes;
- reasoning lineage;
- recursive agents or agent swarms;
- hosted or distributed runtime behavior;
- release posture changes.

## 4. Implementation Summary

The executor now treats `execute_before_report_hook(...)` as a local report-helper operation that returns:

- the stable hook invocation ID; and
- the stable disclosure IDs contained in the validated in-memory hook result.

`execute_with_report(...)` merges the discovered disclosure IDs into the cloned report inputs before creating `TerminalLocalWorkReportInput`. The merge keeps caller-supplied IDs first, appends discovered IDs, and skips duplicates.

No public workflow schema, CLI, persistence, artifact, audit, or event API changed.

## 5. Citation Construction Summary

Discovered disclosure IDs are forwarded through the existing terminal report helper.

The report generator continues to construct citations with:

- `WorkReportCitationKind::AgentHarnessHookDisclosure`;
- `WorkReportCitationTarget::AgentHarnessHookDisclosure`;
- bounded generic citation summary text;
- the existing report sensitivity and redaction metadata.

The implementation does not recreate `EvidenceReference` values and does not copy disclosure titles, summaries, references, hook context, or audit records into the report.

## 6. Workflow Semantics Summary

Workflow semantics remain unchanged.

The implementation does not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events;
- emit hook audit events;
- emit observability events;
- create hook disclosures;
- create hook audit records beyond the existing in-memory helper result;
- read or write a disclosure store;
- write report artifacts;
- change terminal status;
- change policy decisions;
- change approval behavior;
- change retry, cancellation, or duplicate-run behavior;
- change `execute(...)`, `decide_approval(...)`, or `cancel_run(...)`.

## 7. Redaction And Privacy Summary

The implementation is ID-only at the report boundary.

It does not copy:

- hook disclosure title or summary;
- hook disclosure references;
- hook disclosure redaction metadata;
- hook input, output, or supplemental references;
- hook invocation context;
- hook audit records;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

Debug output remains count/redaction oriented. WorkReport serialization may include stable hook disclosure IDs as citation targets, matching the existing WorkReport citation policy.

## 8. Test Coverage Summary

Added or extended focused tests cover:

- discovery of hook disclosure IDs from explicit `BeforeReport` hook results;
- citation of discovered disclosure IDs in `ValidationAndQualityChecks`;
- no copying of disclosure titles or summaries into report serialization;
- Debug non-leakage for discovered disclosure values;
- merge ordering with caller-supplied IDs first;
- deterministic deduplication of caller-supplied and discovered IDs;
- preservation of hook invocation citation behavior;
- no hook workflow events appended by the `BeforeReport` report path;
- no report artifact writes.

Existing explicit caller-supplied hook disclosure ID tests continue to cover the non-discovery input path.

## 9. Commands Run And Results

- `cargo test -p workflow-core --test local_executor hook_disclosure -- --nocapture` - passed.
- `cargo fmt --all --check` - initially reported one formatting change needed.
- `cargo fmt --all` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 10. Remaining Known Limitations

- Discovery is limited to explicit in-memory `BeforeReport` hook results in `execute_with_report(...)`.
- Discovery from workflow events remains unsupported because current event payloads do not carry disclosure IDs.
- Discovery from generic audit projections remains unsupported because projections intentionally omit disclosure IDs and payloads.
- Durable disclosure stores, hook audit sinks, and event/audit replay semantics remain future work.
- Discovered disclosure IDs are not routed to risk, incomplete-work, policy, decision, approval, or handoff sections based on disclosure kind or severity.
- Warning, skipped, and blocked hook semantics remain deferred.

## 11. Recommended Next Phase

Recommended next phase: hook disclosure discovery implementation review.

In parallel, the local-check lane can proceed to review the opt-in live DocsCheck smoke plan before any live local command execution is added.
