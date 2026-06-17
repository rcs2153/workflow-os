# Executor Hook Disclosure Report Input Propagation Plan Review

## 1. Executive Verdict

Plan accepted; proceed to executor hook disclosure report input propagation implementation.

The plan defines a narrow, additive executor report input propagation slice for explicitly supplied `AgentHarnessHookDisclosureId` values. It preserves the current runtime boundary and avoids automatic discovery, hook execution, event append behavior, audit sink emission, persistence, CLI, schemas, side effects, writes, reasoning lineage, hosted behavior, recursive agents, agent swarms, and release posture changes.

## 2. Scope Verification

The plan stayed within planning-only scope.

It does not authorize:

- runtime hook execution;
- executor-integrated hook invocation changes;
- automatic hook disclosure discovery;
- hook disclosure creation from reports;
- hook invocation result creation from reports;
- hook audit record creation or persistence;
- workflow event append behavior;
- audit sink emission;
- warning continuation;
- skipped-with-disclosure continuation;
- blocked runtime behavior;
- hook optionality;
- policy-controlled continuation;
- context-aware section routing by disclosure kind or severity;
- `EvidenceReference` creation or attachment;
- approval request or approval decision creation;
- report artifact behavior changes;
- CLI hook commands or report rendering;
- workflow schema fields;
- automatic local check execution;
- command execution;
- adapter invocation;
- reasoning lineage;
- side-effect boundary implementation;
- writes;
- recursive agents or agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

The plan correctly states that executor propagation is not yet implemented.

## 3. Baseline Assessment

The plan accurately describes the current baseline:

- hook disclosure IDs and bounded hook disclosure model exist;
- WorkReport citation vocabulary for hook disclosures exists;
- terminal local WorkReport helper can cite explicitly supplied hook disclosure IDs;
- executor-integrated report-bearing execution exists;
- executor report inputs can already forward hook invocation IDs;
- executor report inputs cannot yet forward hook disclosure IDs.

The identified gap is correctly scoped as input propagation only.

## 4. Proposed API Assessment

The proposed API change is minimal and appropriate:

```rust
pub agent_harness_hook_disclosure_ids: Vec<AgentHarnessHookDisclosureId>
```

Adding this field to `LocalExecutionReportInputs` mirrors the existing hook invocation ID propagation pattern and keeps the API explicit. The plan correctly requires typed IDs instead of raw strings and rejects full disclosure payloads, disclosure summaries, references, redaction metadata, hook context, audit records, and event payloads.

This is the right first executor slice because the helper already supports the target field.

## 5. Propagation Boundary Assessment

The plan's intended implementation boundary is suitably narrow:

```rust
agent_harness_hook_disclosure_ids: report.agent_harness_hook_disclosure_ids.clone()
```

This should be the only runtime-facing behavior change. The plan preserves the executor flow:

- execute first;
- return execution errors unchanged;
- preserve the run when report generation fails after execution;
- do not mutate runtime state for report generation;
- do not append events;
- do not invoke hooks;
- do not create disclosures, invocation results, or audit records;
- do not write report artifacts automatically;
- do not expose CLI output.

## 6. Debug And Redaction Assessment

The count-only `Debug` requirement is appropriate:

```rust
agent_harness_hook_disclosure_count
```

The plan correctly forbids leaking hook disclosure IDs, report IDs, paths, tokens, notes, limitations, risks, handoff text, hook invocation IDs, disclosure title, disclosure summary, references, hook output summaries, and payload-like strings in executor request debug output.

The plan also correctly notes that serialization is not currently the primary surface for executor report inputs and should be separately reviewed if introduced.

## 7. Report Behavior Assessment

The planned report behavior matches the accepted helper integration:

- supplied disclosure IDs become `WorkReportCitationTarget::AgentHarnessHookDisclosure`;
- citation kind is `WorkReportCitationKind::AgentHarnessHookDisclosure`;
- citations appear in `ValidationAndQualityChecks`;
- summaries remain generic and bounded;
- no full `AgentHarnessHookDisclosure`, `AgentHarnessHookInvocationResult`, or `AgentHarnessHookAuditRecord` values are created or copied;
- absent disclosure IDs preserve current behavior and do not fabricate missing citations.

This is consistent with the terminal report helper review.

## 8. Workflow Semantics Assessment

The plan preserves workflow semantics.

It explicitly forbids:

- mutating `WorkflowRun`;
- mutating `WorkflowRunSnapshot`;
- appending workflow events;
- emitting audit or observability events;
- invoking hooks;
- calling `execute_runtime_agent_harness_hook(...)`;
- creating hook disclosures, invocation results, or audit records;
- touching `StateBackend` beyond existing executor behavior;
- writing report artifacts;
- reading or persisting hook disclosures;
- exposing CLI output;
- changing workflow pass/fail behavior;
- changing `execute(...)`, `decide_approval(...)`, or `cancel_run(...)`.

Report-generation failure remains separate from workflow execution failure in `LocalExecutionWithReportResult`.

## 9. Privacy And Payload Assessment

The plan keeps executor report inputs reference-only.

It correctly forbids copying:

- hook disclosure title or summary;
- hook disclosure references;
- hook disclosure redaction metadata;
- hook input/output references;
- supplemental hook references;
- hook audit records;
- hook invocation results;
- hook contracts;
- workflow/run/actor context from hook records;
- hook output summaries;
- raw prompts;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

The plan's posture that hook disclosure IDs are stable references but still potentially sensitive is appropriate.

## 10. Relationship Assessments

Runtime hook execution:

The plan correctly separates report input propagation from runtime hook execution. Supplying a disclosure ID must not cause hook execution, contract evaluation, local check registration, command execution, file reads, external calls, or default hook profiles.

Audit and workflow events:

The plan correctly avoids modeling hook disclosure IDs as audit or workflow event citations in this phase. Doing so would fabricate runtime history because disclosure event append behavior and dedicated hook audit sink emission are not implemented.

Warning and skipped semantics:

The plan correctly avoids interpreting disclosure IDs as warning, skipped, blocked, optional, or policy-controlled continuation behavior. Section placement remains `ValidationAndQualityChecks` until later semantics are accepted.

## 11. Test Plan Assessment

The test plan is appropriately focused and production-shaped.

It covers:

- `LocalExecutionReportInputs` accepting disclosure IDs;
- `execute_with_report(...)` forwarding disclosure IDs;
- generated citation target and kind;
- section placement in `ValidationAndQualityChecks`;
- coexistence with hook invocation IDs;
- absence behavior;
- redaction-safe `Debug`;
- report-generation failure preserving run and event history;
- no runtime event append;
- no extra `StateBackend` write beyond existing execution behavior;
- no automatic artifact writes;
- no full disclosure value creation or copying;
- no disclosure payload markers in debug or serialized reports;
- no automatic discovery from events, audit records, or persistence;
- existing WorkReport, hook disclosure, hook invocation, executor, artifact, EvidenceReference, Diagnostic, validation, adapter telemetry, local-check, and runtime tests.

Non-blocking test follow-up:

- Include a test that `LocalExecutionReportInputs` debug counts both hook invocation IDs and hook disclosure IDs without showing either stable ID. This is implied by the plan but worth making explicit in implementation.

## 12. Documentation Assessment

The planning document is explicit that this is planning only and that executor propagation is not implemented yet.

Related docs now point to the plan without overclaiming implementation:

- `ROADMAP.md`;
- `docs/implementation-plans/work-report-contract-plan.md`;
- `docs/concepts/governed-work-pattern.md`;
- `docs/concepts/evidence-reference.md`.

The docs preserve the product boundary and do not claim automatic nested harness behavior, production hook execution, hosted behavior, write support, or Level 3/4 autonomy.

## 13. Planning Blockers

No planning blockers.

## 14. Non-Blocking Follow-Ups

- Add the explicit combined debug-count test noted above during implementation.
- Keep implementation report wording clear that executor propagation does not validate disclosure ID existence.
- Consider a later cleanup for the validation summary wording around combined hook plus disclosure references.
- Revisit context-aware section placement only after warning/skipped/blocked semantics are accepted.

## 15. Recommended Next Phase

Recommended next phase: executor hook disclosure report input propagation implementation.

The implementation should add only the explicit input field, redaction-safe debug count, and forwarding behavior described in the plan. It must not implement runtime hook execution, automatic disclosure discovery, event append behavior, audit sink emission, persistence, report artifacts, CLI rendering, workflow schema changes, `EvidenceReference` creation, reasoning lineage, side-effect modeling, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.

## Validation

- `npm run check:docs` - passed.
