# Agent Harness Hook Audit Record Core Model Report

Report date: 2026-06-16

## 1. Executive Summary

Implemented the Agent Harness Hook audit record core model as a model-only phase. Workflow OS can now represent a stable hook invocation identity and a validated, redaction-safe hook audit record derived from explicit hook invocation context.

This phase does not execute hooks. It does not integrate with `LocalExecutor`, append workflow events, emit audit records to sinks, run local checks, invoke adapters, write files, persist records, expose CLI output, add schema fields, authorize side effects, add writes, implement reasoning lineage, enable recursive agents, enable agent swarms, or change release posture.

## 2. Scope Completed

- Added `AgentHarnessHookInvocationId`.
- Added `AgentHarnessHookAuditRecord`.
- Added `AgentHarnessHookAuditRecordDefinition`.
- Added `AgentHarnessHookAuditRecord::from_invocation_result(...)`.
- Exported the new model types through `workflow-core`.
- Added focused audit record model tests.
- Updated roadmap, concept, quickstart, and planning documentation.

## 3. Scope Explicitly Not Completed

- No runtime hook execution.
- No executor-integrated hook invocation.
- No workflow event kinds.
- No workflow event append behavior.
- No audit sink emission.
- No automatic workflow execution.
- No automatic local check execution.
- No default local check handler registration.
- No command-output evidence.
- No CLI hook commands.
- No workflow schema fields.
- No workflow-declared hook configuration.
- No runtime harness generation.
- No nested harness execution.
- No recursive agents.
- No agent swarms.
- No hosted or distributed execution.
- No side-effect modeling.
- No writes.
- No approval evidence attachment.
- No reasoning lineage.
- No persistence changes.
- No report artifact auto-writing.
- No examples.
- No release posture change.

## 4. Model Types Added

`AgentHarnessHookInvocationId` is a stable, validated identifier for future hook invocation result or audit citations. It is bounded, character-constrained, secret-aware, serde-compatible, and redaction-safe in `Debug`.

`AgentHarnessHookAuditRecord` is a private-field, validated model-only record containing:

- hook invocation ID;
- hook contract ID;
- hook contract version;
- hook kind;
- workflow ID;
- workflow version;
- run ID;
- schema version;
- spec hash;
- actor;
- invoked timestamp;
- optional correlation ID;
- optional step ID;
- optional phase ID;
- invocation status;
- input references;
- output references;
- supplemental references;
- disclosures;
- redaction metadata;
- sensitivity.

`AgentHarnessHookAuditRecordDefinition` is the explicit construction input for the validated record.

## 5. Validation Boundary Summary

Validation ensures:

- hook invocation ID is valid;
- contract identity is valid;
- workflow/run/schema/spec/actor identity is valid;
- optional correlation, step, and phase identifiers are valid;
- duplicate input reference names are rejected;
- duplicate output reference names are rejected;
- stable references validate and reject secret-like values;
- disclosures are bounded and secret-aware;
- redaction metadata is bounded and secret-aware;
- deserialization re-enters the validated constructor and fails closed.

Validation errors use stable codes and do not include raw caller-supplied values.

## 6. Audit/Event Semantics Summary

The audit record model is not a workflow event and does not emit audit records.

It means only that a hook invocation context can be represented as a bounded, validated, redaction-safe record. It does not mean:

- a hook executed;
- a workflow event was appended;
- an audit sink received a record;
- a local check ran;
- an adapter was invoked;
- an approval was requested or decided;
- evidence was created;
- a side effect was authorized;
- a write occurred.

## 7. Redaction And Privacy Summary

The model does not store raw prompts, raw spec contents, raw command output, raw provider payloads, parser payloads, environment variable values, credentials, authorization headers, private keys, token-like values, or unbounded natural-language text.

`Debug` output redacts workflow/run/actor/reference/disclosure context and reports counts for reference collections. Serialization tests verify forbidden raw payload markers are not emitted.

## 8. Test Coverage Summary

Added focused tests covering:

- hook invocation ID invalid and secret-like rejection;
- valid hook audit record construction;
- required identity accessors;
- building an audit record from a validated invocation result;
- duplicate output reference rejection;
- secret-like reference rejection without leakage;
- redaction-safe `Debug`;
- serialization non-leakage;
- serde round trip;
- invalid serialized payload fail-closed behavior;
- absence of encoded runtime event, audit sink, persistence, executor, schema, or CLI behavior.

Existing hook invocation helper tests remain in the same test file and continue to pass.

## 9. Commands Run And Results

- `cargo fmt --all`
  - Passed.
- `cargo test -p workflow-core --test agent_harness_hook_invocation`
  - Passed.
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

## 10. Remaining Known Limitations

- The hook audit record core model is not reviewed yet.
- Runtime hook execution is not implemented.
- Executor integration is not implemented.
- Hook workflow events are not implemented.
- Audit sink emission for hook records is not implemented.
- Hook audit records are not persisted.
- WorkReport hook citation vocabulary is not implemented.
- Workflow schema support for hooks is not implemented.
- CLI hook commands are not implemented.
- Side effects and writes remain unsupported.

## 11. Recommended Next Phase

Recommended next phase: **agent harness hook audit record core model review**.

That review should verify model shape, validation behavior, serde behavior, redaction/privacy posture, test quality, documentation honesty, and scope boundaries before any WorkReport hook citation, executor integration, workflow event, audit sink, persistence, CLI, schema, side-effect, write, recursive-agent, or agent-swarm work begins.
