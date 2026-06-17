# Hook Disclosure Discovery Implementation Review

Review date: 2026-06-17

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation satisfies the accepted scope: `LocalExecutor::execute_with_report(...)` discovers hook disclosure IDs only from the already-validated in-memory `BeforeReport` hook result that the explicit report-bearing path just executed. It preserves caller-supplied IDs, merges discovered IDs deterministically, cites stable IDs through the existing WorkReport citation path, and keeps workflow state, event history, audit emission, artifact writing, CLI behavior, schemas, side effects, writes, and release posture unchanged.

The next implementation lane should move to the opt-in live DocsCheck smoke after its plan review. Hook disclosure discovery should not broaden further until there is a separate accepted plan for event/audit discovery, section routing, or warning/skipped semantics.

## 2. Scope Verification

The phase stayed within the approved in-memory discovery scope.

Implemented:

- extraction of `AgentHarnessHookDisclosureId` values from validated in-memory `BeforeReport` hook results;
- merge of caller-supplied and discovered disclosure IDs;
- deterministic deduplication preserving caller-supplied IDs first;
- forwarding of merged IDs into existing terminal WorkReport generation;
- focused tests and implementation report;
- honest roadmap/concept documentation updates.

No accidental implementation was found for:

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

## 3. Implementation Assessment

The implementation is appropriately small and idiomatic for the current codebase.

`execute_before_report_hook(...)` now returns a private `BeforeReportHookExecutionResult` with:

- `hook_invocation_id`;
- `disclosure_ids`.

`execute_with_report(...)` continues to clone explicit report inputs, runs the existing explicit `BeforeReport` hook only after the run is terminal, appends the hook invocation ID as before, and then merges discovered disclosure IDs into `report.agent_harness_hook_disclosure_ids`.

The discovery helper is private to the executor path. That is appropriate for this slice because the phase did not approve a public discovery API.

## 4. Source-Of-Truth Assessment

The implementation uses source-of-truth data.

It extracts disclosure IDs from `result.invocation_result().disclosures()`, which are already validated by the runtime hook helper. It does not infer IDs from:

- hook invocation IDs;
- hook status;
- input/output reference counts;
- workflow events;
- audit projections;
- report section text;
- diagnostics;
- local check results;
- adapter telemetry;
- operator notes.

No fabricated disclosure IDs were found.

## 5. Merge And Ordering Assessment

The merge policy matches the accepted plan:

- caller-supplied IDs remain first;
- discovered IDs append in hook result order;
- duplicates are skipped;
- duplicate IDs are not treated as errors in this phase.

The focused merge test verifies caller-first ordering and deduplication for a duplicate caller/discovered ID.

## 6. Report Behavior Assessment

The report behavior is correct.

Discovered disclosure IDs are cited through the existing WorkReport path as:

- `WorkReportCitationKind::AgentHarnessHookDisclosure`;
- `WorkReportCitationTarget::AgentHarnessHookDisclosure`;
- citations in `ValidationAndQualityChecks`.

The implementation does not copy disclosure title, disclosure summary, disclosure references, hook context, or audit records into report section text. The tests assert the discovered ID appears as a serialized citation target while the disclosure title and summary do not appear.

The implementation correctly avoids routing discovered disclosure IDs to risks, incomplete work, decisions, policy gates, approvals, or handoff notes.

## 7. Workflow Semantics Assessment

Workflow semantics are preserved.

The implementation does not:

- mutate the terminal `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append hook workflow events for `BeforeReport`;
- emit dedicated hook audit records;
- emit observability events;
- write report artifacts;
- read or write a disclosure store;
- change terminal status;
- change policy or approval behavior;
- change retry, cancellation, or duplicate-run behavior;
- change `execute(...)`, `decide_approval(...)`, or `cancel_run(...)`.

The tests verify event history remains equal to the run events and that no report artifacts are written.

## 8. Privacy And Redaction Assessment

The implementation remains ID-only at the report boundary.

No copying was found for:

- hook disclosure title or summary;
- hook disclosure references;
- hook disclosure redaction metadata;
- hook input/output/supplemental references;
- hook invocation context;
- hook audit records;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira/GitHub bodies;
- raw spec contents;
- parser payloads;
- environment values;
- credentials, authorization headers, private keys, or token-like values.

Debug output remains redaction/count oriented. WorkReport serialization may include stable hook disclosure IDs as citation targets, which matches the existing WorkReport citation policy.

## 9. Error Handling Assessment

The implementation keeps failure behavior bounded.

Unexpected failures from `execute_runtime_agent_harness_hook(...)` return a structured report-generation error in `LocalExecutionWithReportResult` while preserving the run, matching the pre-existing `BeforeReport` hook failure posture.

Discovery itself is currently infallible because disclosure IDs are extracted from validated hook disclosure values. No new user-facing workflow diagnostic path was introduced.

## 10. Test Quality Assessment

The tests cover the most important behaviors:

- discovered `BeforeReport` hook disclosure IDs are cited;
- citation kind and target are correct;
- citations remain in `ValidationAndQualityChecks`;
- disclosure title and summary are not copied into report serialization;
- result debug output does not leak discovered disclosure details;
- caller-supplied IDs remain first;
- duplicate caller/discovered IDs are deduplicated;
- hook invocation ID citation behavior is preserved;
- `BeforeReport` discovery appends no hook workflow events;
- no report artifacts are written;
- full workspace tests pass.

Non-blocking test follow-ups:

- Add a tiny explicit assertion that a `BeforeReport` hook result with no disclosures produces no hook disclosure citation.
- Add a focused regression name that states current workflow events and audit projections are not discovery sources. Existing architecture and tests imply this, but a named test would make the boundary easier to review later.

These are not blockers because the implementation has no event/audit discovery path and the current tests cover non-mutation and no hook event append behavior.

## 11. Documentation Review

Documentation is honest.

Updated docs say:

- first in-memory hook disclosure discovery is implemented;
- discovery is limited to already-validated in-memory `BeforeReport` hook results in the explicit report-bearing executor path;
- discovery from workflow events or audit projections is not implemented;
- warning/skipped/blocked semantics are not implemented;
- dedicated hook audit sink emission is not implemented;
- persistence, CLI behavior, schemas, side effects, writes, reasoning lineage, recursive agents, hosted behavior, and release posture changes are not implemented.

Historical reports that say automatic hook disclosure discovery was not implemented remain accurate for their original phase and should not be rewritten.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Add an explicit empty-disclosure `BeforeReport` test.
- Add a named no-event/no-audit-discovery regression test if future refactors make event/audit payload access easier to misuse.
- Keep section routing by disclosure kind/severity deferred until separately planned.
- Keep warning/skipped/blocked semantics separate from discovery.
- Keep durable disclosure store or hook audit sink discovery separate from this report-helper path.

## 14. Recommended Next Phase

Recommended next phase: opt-in live DocsCheck smoke implementation, after accepting the DocsCheck smoke plan review.

The hook disclosure discovery lane should pause at this accepted slice until a concrete need justifies broader discovery, section routing, or hook status semantics.

## 15. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
