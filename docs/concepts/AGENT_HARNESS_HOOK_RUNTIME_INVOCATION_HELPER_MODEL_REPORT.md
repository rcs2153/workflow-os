# Agent Harness Hook Runtime Invocation Helper Model Report

Report date: 2026-06-16

## 1. Executive Summary

Implemented the first in-memory Agent Harness Hook runtime invocation helper model. Workflow OS can now validate explicit phase-level hook invocation context against an `AgentHarnessHookContract` and return a structured in-memory `AgentHarnessHookInvocationResult`.

This phase does not execute hooks. It does not integrate with `LocalExecutor`, append workflow events, emit audit records, run local checks, invoke adapters, write files, persist results, expose CLI output, add schema fields, authorize side effects, add writes, implement reasoning lineage, enable recursive agents, enable agent swarms, or change release posture.

## 2. Scope Completed

- Added `AgentHarnessHookReference`.
- Added `AgentHarnessHookNamedReference`.
- Added `AgentHarnessHookDisclosureKind`.
- Added `AgentHarnessHookDisclosure`.
- Added `AgentHarnessHookInvocationStatus`.
- Added `AgentHarnessHookInvocationInput`.
- Added `AgentHarnessHookInvocationResult`.
- Added `AgentHarnessHookInvocationResultDefinition`.
- Added `invoke_agent_harness_hook(...)`.
- Exported the new helper model through `workflow-core`.
- Added focused invocation model tests.
- Updated roadmap, concept, and planning documentation to reflect the implemented helper model.

## 3. Scope Explicitly Not Completed

- No runtime hook execution.
- No executor-integrated hook invocation.
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

## 4. Helper API Summary

`invoke_agent_harness_hook(input)` accepts an explicit `AgentHarnessHookInvocationInput` containing:

- an `AgentHarnessHookContract`;
- workflow, run, schema, spec hash, actor, timestamp, and optional correlation/step/phase context;
- named input references;
- named output references;
- supplemental stable references;
- bounded disclosures;
- redaction metadata;
- sensitivity.

The helper validates the context and returns an in-memory `AgentHarnessHookInvocationResult` with status `Passed` when validation succeeds.

## 5. Validation Boundary Summary

Validation ensures:

- the supplied contract validates;
- invocation hook kind matches the contract hook kind;
- workflow/run/schema/spec/actor references are bounded and not secret-like;
- phase ID is bounded and uses supported identifier characters;
- named input and output references are valid and duplicate-free;
- required input references are present;
- required output references are present when output validation is requested;
- supplemental references are stable and bounded;
- disclosures are bounded and not secret-like;
- redaction metadata is bounded and not secret-like;
- side-effect requests are rejected.

Validation errors use stable `agent_harness_hook_invocation.*` codes and do not include raw caller-supplied values.

## 6. Reference Model Summary

The helper can cite supplied stable references for:

- EvidenceReference IDs;
- local check result IDs;
- typed handoff IDs;
- validation reference IDs;
- workflow event IDs;
- audit event IDs;
- policy IDs;
- policy decision event IDs;
- approval decision reference IDs.

The helper does not create EvidenceReference values, local check results, typed handoffs, policy decisions, approval decisions, audit events, workflow events, WorkReports, or report artifacts.

## 7. Workflow Semantics Summary

The helper is pure and in-memory.

It does not:

- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- append workflow events;
- emit audit events;
- emit observability events;
- call `LocalExecutor`;
- touch `StateBackend`;
- run local checks;
- invoke adapters;
- write local state;
- create filesystem artifacts;
- return CLI-formatted output.

Hook invocation failure does not change workflow pass/fail semantics.

## 8. Redaction And Privacy Summary

The helper rejects or redacts sensitive material at the invocation boundary:

- raw prompts;
- raw spec contents;
- raw command output;
- raw command transcripts;
- raw provider payloads;
- raw CI logs;
- Jira/GitHub body markers;
- parser payloads;
- environment variable markers;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded disclosures.

`Debug` output redacts workflow/run/actor/reference/disclosure context and reports only counts for reference collections.

## 9. Test Coverage Summary

Added focused tests covering:

- valid phase-level hook invocation result;
- hook kind mismatch rejection;
- missing required input rejection;
- missing required output rejection when outputs are required;
- output omission allowed when output validation is not required;
- duplicate input reference rejection;
- stable reference kinds accepted without recreating evidence;
- absent optional references do not fabricate citations;
- side-effect request rejection;
- secret-like reference rejection without leakage;
- secret-like disclosure rejection without leakage;
- redaction-safe `Debug`;
- serialization non-leakage;
- serde round trip;
- invalid serialized result fail-closed behavior;
- absence of encoded runtime execution behavior.

## 10. Commands Run And Results

- `cargo fmt --all`
  - Passed.
- `cargo fmt --all --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Passed.
- `cargo test -p workflow-core --test agent_harness_hook_invocation`
  - Passed.
- `cargo test --workspace`
  - Passed.
- `npm run check:docs`
  - Passed.
- `git diff --check`
  - Passed.

## 11. Remaining Known Limitations

- The helper model is not reviewed yet.
- Runtime hook execution is not implemented.
- Executor integration is not implemented.
- Hook audit/event semantics are not implemented.
- Workflow schema support for hooks is not implemented.
- CLI hook commands are not implemented.
- Hook results are not persisted and do not have durable result IDs.
- WorkReport integration for hook results is not implemented.
- Side effects and writes remain unsupported.

## 12. Recommended Next Phase

Recommended next phase: **agent harness hook runtime invocation helper model review**.

That review should verify scope, model shape, validation behavior, serde behavior, redaction/privacy posture, test quality, and documentation honesty before any hook audit/event or executor-integration planning begins.
