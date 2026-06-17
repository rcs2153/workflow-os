# Agent Harness Hook Runtime Execution Service Report

Report date: 2026-06-17

## 1. Executive Summary

Implemented the first explicit in-memory runtime hook execution helper for agent harness hooks.

The new helper consumes caller-supplied hook invocation identity and explicit hook invocation context, validates the context through the existing `invoke_agent_harness_hook(...)` boundary, builds a model-only `AgentHarnessHookAuditRecord`, and returns both values in a `RuntimeAgentHarnessHookResult`.

This phase does not add automatic executor hook checkpoints, workflow events, audit sink emission, persistence, CLI behavior, workflow schema fields, local check execution, adapter execution, command execution, side-effect modeling, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 2. Scope Completed

- Added `RuntimeAgentHarnessHookInput`.
- Added `RuntimeAgentHarnessHookResult`.
- Added `execute_runtime_agent_harness_hook(...)`.
- Exported the helper API from `workflow-core`.
- Preserved caller-supplied `AgentHarnessHookInvocationId` values.
- Reused `invoke_agent_harness_hook(...)` for validation.
- Reused `AgentHarnessHookAuditRecord::from_invocation_result(...)` for audit-shaped output.
- Added a `report_citation_target()` helper for `WorkReportCitationTarget::AgentHarnessHook`.
- Added focused regression tests for successful execution, fail-closed behavior, stable references, non-leaking errors, debug redaction, and explicit in-memory boundaries.
- Updated roadmap and concept/planning docs.

## 3. Scope Explicitly Not Completed

- No automatic executor hook invocation.
- No executor checkpoint placement.
- No workflow event kinds.
- No workflow event append behavior.
- No audit sink emission.
- No audit event IDs.
- No hook persistence.
- No workflow schema fields.
- No CLI hook commands.
- No automatic local check execution.
- No command execution.
- No adapter invocation.
- No `EvidenceReference` creation or attachment.
- No approval request or decision creation.
- No reasoning lineage implementation.
- No side-effect boundary implementation.
- No writes.
- No recursive agents.
- No agent swarms.
- No release posture change.

## 4. Helper API Summary

The helper API is:

- `RuntimeAgentHarnessHookInput`
- `RuntimeAgentHarnessHookResult`
- `execute_runtime_agent_harness_hook(input)`

`RuntimeAgentHarnessHookInput` contains:

- a caller-supplied `AgentHarnessHookInvocationId`;
- an explicit `AgentHarnessHookInvocationInput`.

`RuntimeAgentHarnessHookResult` exposes read-only accessors for:

- hook invocation ID;
- invocation result;
- model-only hook audit record.

It also provides `into_parts()` for owned extraction and `report_citation_target()` for future WorkReport citation use.

## 5. Execution Boundary Summary

Execution remains in-memory and explicit. The helper does not discover runtime state, read hidden global state, mutate a `WorkflowRun`, mutate a `WorkflowRunSnapshot`, touch `StateBackend`, append events, emit audit records, run local checks, run commands, call adapters, call external systems, write files, persist hook records, or expose CLI output.

## 6. Validation Boundary Summary

Validation is delegated to existing constructors and validators:

- `AgentHarnessHookInvocationId::new(...)`;
- `invoke_agent_harness_hook(...)`;
- `AgentHarnessHookInvocationResult::validate()`;
- `AgentHarnessHookAuditRecord::from_invocation_result(...)`;
- `RuntimeAgentHarnessHookResult::new(...)`.

The helper fails closed for invalid hook context, hook kind mismatch, missing required references, duplicate references, side-effect requests, secret-like values, invalid redaction metadata, and mismatched runtime result records.

## 7. Audit And Event Posture

The helper creates only an in-memory `AgentHarnessHookAuditRecord`.

It does not:

- append workflow events;
- emit audit sink records;
- persist the audit record;
- create audit event IDs;
- project hook records into `AuditEvent`.

Future executor checkpoint, event, audit sink, and persistence behavior remain separate phases.

## 8. Redaction And Privacy Summary

The implementation reuses existing redaction-safe hook invocation and hook audit models. Debug output redacts workflow/run context, references, disclosures, and audit record contents.

The helper does not store or copy raw prompts, raw spec contents, raw command output, raw command transcripts, raw provider payloads, raw CI logs, raw Jira/GitHub bodies, parser payloads, environment values, credentials, authorization headers, private keys, token-like values, unbounded summaries, or hook execution transcripts.

## 9. Test Coverage Summary

Tests cover:

- valid runtime hook execution returns an in-memory result and audit record;
- caller-supplied hook invocation ID is preserved;
- WorkReport hook citation target derivation;
- owned `into_parts()` extraction;
- mismatched runtime result records fail closed;
- hook kind mismatch fails closed;
- missing required inputs fail closed;
- missing required outputs fail closed when outputs are required;
- side-effect requests fail closed;
- stable EvidenceReference and local check references are cited by ID without recreating evidence;
- absent optional references do not fabricate citations;
- secret-like values fail without leakage;
- Debug output does not leak hook context;
- runtime result debug output does not encode executor, event, state backend, file, CLI, adapter, or command behavior.

## 10. Commands Run And Results

- `cargo test -p workflow-core --test agent_harness_hook_invocation`
  - Passed.
- `cargo fmt --all --check`
  - Passed after applying `cargo fmt --all`.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.

## 11. Remaining Known Limitations

- Automatic executor hook checkpoints are not implemented.
- Hook workflow events are not implemented.
- Hook audit sink emission is not implemented.
- Hook records are not persisted.
- Workflow schema support for hooks is not implemented.
- CLI hook commands are not implemented.
- Side effects and writes remain unsupported.
- Hook execution is not yet dogfooded as part of executor-driven workflow runs.

## 12. Recommended Next Phase

Recommended next phase: **agent harness hook runtime execution service review**.

The review should verify the explicit in-memory API, validation boundary, audit-record construction, non-leaking error behavior, test coverage, and strict non-goals before any executor checkpoint planning or runtime event work begins.
