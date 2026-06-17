# Agent Harness Hook Runtime Execution Service Review

Review date: 2026-06-17

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds the intended explicit in-memory runtime hook execution helper and stays inside the accepted boundary. It validates through existing hook invocation constructors, builds a model-only hook audit record, exposes a redaction-safe result wrapper, and does not wire hooks into `LocalExecutor`, workflow events, audit sinks, persistence, CLI behavior, schemas, side effects, writes, recursive agents, agent swarms, or release posture changes.

Proceed to **executor hook checkpoint planning** after addressing only normal follow-up polish as needed.

## 2. Scope Verification

The phase stayed within the approved in-memory helper scope.

Implemented:

- `RuntimeAgentHarnessHookInput`;
- `RuntimeAgentHarnessHookResult`;
- `execute_runtime_agent_harness_hook(...)`;
- export of the new helper API from `workflow-core`;
- result accessors and `into_parts()`;
- `report_citation_target()` for `WorkReportCitationTarget::AgentHarnessHook`;
- focused tests and documentation updates.

No accidental implementation found for:

- automatic executor hook invocation;
- executor checkpoint placement;
- workflow event kinds;
- workflow event append behavior;
- audit sink emission;
- hook persistence;
- workflow schema fields;
- CLI hook commands;
- automatic local check execution;
- command execution;
- adapter invocation;
- `EvidenceReference` creation or attachment;
- approval request or decision creation;
- reasoning lineage;
- side-effect boundary;
- writes;
- recursive agents;
- agent swarms;
- release posture changes.

## 3. Helper API Assessment

The helper API is narrow and explicit. `RuntimeAgentHarnessHookInput` carries a caller-supplied `AgentHarnessHookInvocationId` and an `AgentHarnessHookInvocationInput`. This avoids a duplicate context model and keeps validation centralized in the existing hook invocation boundary.

`execute_runtime_agent_harness_hook(...)` performs only three steps:

1. validates the invocation through `invoke_agent_harness_hook(...)`;
2. builds `AgentHarnessHookAuditRecord::from_invocation_result(...)`;
3. returns `RuntimeAgentHarnessHookResult`.

The API does not read hidden global state, infer context from process state, require `StateBackend`, require `LocalExecutor`, require live adapters, or call external systems.

## 4. Runtime Boundary Assessment

The helper is runtime-shaped but in-memory only. It does not mutate `WorkflowRun`, `WorkflowRunSnapshot`, event history, state backend records, telemetry stores, report artifacts, or files.

The implementation does not append post-terminal events, emit audit events, emit observability events, run local checks, execute shell commands, invoke adapters, or expose CLI output.

The result wrapper intentionally owns only:

- hook invocation ID;
- validated hook invocation result;
- model-only hook audit record.

## 5. Validation Boundary Assessment

Validation remains fail-closed and constructor-driven.

The implementation relies on:

- `AgentHarnessHookInvocationId::new(...)`;
- `invoke_agent_harness_hook(...)`;
- `AgentHarnessHookInvocationResult::validate()`;
- `AgentHarnessHookAuditRecord::from_invocation_result(...)`;
- `RuntimeAgentHarnessHookResult::new(...)`.

The helper fails closed for:

- hook kind mismatch;
- missing required inputs;
- missing required outputs when required;
- duplicate references;
- side-effect requests;
- secret-like references and disclosures;
- invalid redaction metadata through existing invocation validation;
- mismatched runtime result and audit record identity.

Errors use stable codes and do not include raw hook IDs, workflow IDs, run IDs, references, disclosures, or secret-like values.

## 6. Audit And Event Assessment

The implementation creates an in-memory `AgentHarnessHookAuditRecord` only.

It correctly does not:

- append workflow events;
- emit audit sink records;
- create audit event IDs;
- persist hook audit records;
- project hook records into `AuditEvent`;
- imply durable event history.

This preserves the current event-log source-of-truth boundary. Hook workflow events and audit sink emission remain future design work.

## 7. Citation Assessment

`RuntimeAgentHarnessHookResult::report_citation_target()` correctly derives `WorkReportCitationTarget::AgentHarnessHook` from the caller-supplied hook invocation ID.

The helper does not create `EvidenceReference` values, does not fabricate IDs, and does not discover hook records automatically. The citation helper is safe because it points to the stable invocation ID already validated by the hook boundary.

## 8. Privacy And Redaction Assessment

Debug output for `RuntimeAgentHarnessHookResult` is redaction-safe. It includes kind/status/counts and redacts workflow ID, workflow version, run ID, references, disclosures, and audit record contents.

The helper does not store or copy:

- raw prompts;
- raw spec contents;
- raw command output or transcripts;
- raw provider payloads;
- raw CI logs;
- Jira or GitHub raw bodies;
- parser payloads;
- environment values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- hook execution transcripts.

Serialization is not implemented for `RuntimeAgentHarnessHookResult`, which is consistent with the in-memory-only scope. Existing serialized hook invocation and audit record models retain their validated/redaction-safe behavior.

## 9. Test Quality Assessment

The test coverage is focused and meaningful.

Covered:

- valid runtime hook execution returns result and audit record;
- caller-supplied invocation ID is preserved;
- WorkReport citation target derivation;
- `into_parts()` owned extraction;
- mismatched runtime result records fail closed;
- hook kind mismatch;
- missing required inputs;
- missing required outputs;
- side-effect request rejection;
- stable EvidenceReference and local check references by ID;
- no EvidenceReference recreation;
- absent optional references do not fabricate citations;
- secret-like values fail without leakage;
- Debug non-leakage;
- result debug does not encode executor, event, state backend, file, CLI, adapter, or command behavior;
- existing workspace tests passed.

Minor test limitation: no `StateBackend` mutation test is necessary for this helper because no backend is accepted by the API, but a future executor checkpoint implementation must add direct state/event mutation tests.

## 10. Documentation Review

Documentation now states that:

- explicit in-memory runtime hook execution helper is implemented;
- automatic executor hook invocation is not implemented;
- workflow events are not implemented;
- audit sink emission is not implemented;
- persistence is not implemented;
- CLI behavior is not implemented;
- workflow schema fields are not implemented;
- automatic local checks are not implemented;
- side-effect modeling is not implemented;
- writes remain unsupported;
- recursive agents and agent swarms are not introduced;
- release posture is unchanged.

Tiny documentation corrections were made during review to update living implementation-plan status text that still described runtime hook execution as entirely unimplemented. Historical reports and reviews were left intact as phase-time records.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- In the next planning phase, define exact executor hook checkpoint names, event ordering, failure behavior, idempotency keys, and report-citation behavior before any executor wiring.
- Consider adding a short runtime docs note once executor checkpoint planning is accepted, but do not expose CLI/schema behavior yet.
- Future executor integration must add direct tests proving no unintended event append, audit sink emission, state mutation, report artifact write, local check execution, adapter invocation, command execution, or workflow semantic change.

## 13. Recommended Next Phase

Recommended next phase: **executor hook checkpoint planning**.

Reason: the explicit helper is accepted, but automatic invocation must not be wired into `LocalExecutor` until checkpoint placement, event ordering, failure semantics, idempotency, policy/approval interactions, audit posture, and report linkage are planned and reviewed.

That next phase should remain planning-only. It must not implement executor hooks, workflow events, audit sink emission, persistence, CLI behavior, schemas, side effects, writes, recursive agents, agent swarms, or release posture changes.

## 14. Validation

- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.
