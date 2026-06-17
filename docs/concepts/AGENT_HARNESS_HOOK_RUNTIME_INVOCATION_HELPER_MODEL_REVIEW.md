# Agent Harness Hook Runtime Invocation Helper Model Review

Review date: 2026-06-16

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The in-memory Agent Harness Hook runtime invocation helper model is appropriately scoped, explicit, validation-first, and aligned with the accepted runtime invocation plan. It adds a governed checkpoint invocation envelope without executing hooks, integrating with `LocalExecutor`, appending workflow events, emitting audit records, running local checks, invoking adapters, writing files, persisting results, exposing CLI output, adding schema fields, authorizing side effects, enabling writes, implementing reasoning lineage, enabling recursive agents, enabling agent swarms, or changing release posture.

## 2. Scope Verification

The phase stayed within the approved helper-model scope.

Implemented:

- stable hook invocation reference vocabulary;
- named input and output references;
- bounded invocation disclosures;
- hook invocation status vocabulary;
- explicit invocation input;
- validated in-memory invocation result;
- `invoke_agent_harness_hook(...)`;
- redaction-safe `Debug`;
- validated result deserialization;
- focused tests and documentation updates.

No accidental implementation was found for:

- runtime hook execution;
- executor-integrated hook invocation;
- automatic workflow execution;
- automatic local check execution;
- default local check handler registration;
- command-output evidence;
- CLI hook commands;
- workflow schema fields;
- workflow-declared hook configuration;
- runtime harness generation;
- nested harness execution;
- recursive agents;
- agent swarms;
- hosted or distributed execution;
- side-effect modeling;
- writes;
- approval evidence attachment;
- reasoning lineage;
- persistence changes;
- report artifact auto-writing;
- examples;
- release posture changes.

## 3. Helper And Model API Assessment

The helper API is narrow and testable.

`invoke_agent_harness_hook(...)` accepts explicit invocation context through `AgentHarnessHookInvocationInput`, validates it against a supplied `AgentHarnessHookContract`, and returns an in-memory `AgentHarnessHookInvocationResult`.

The implementation does not read hidden global state, runtime configuration, state backends, filesystem state, adapter state, or CLI inputs. The result type stores evidence through private fields and exposes read-only accessors for reference and disclosure collections.

The public input type uses public fields, which is acceptable for a request/context object because validation occurs when the helper constructs the result. The result storage boundary remains private and validated.

## 4. Invocation Boundary Assessment

The invocation boundary is correctly phase-level and explicit.

The helper validates:

- supplied hook contract;
- invocation hook kind matches the contract hook kind;
- workflow ID;
- workflow version;
- run ID;
- schema version;
- spec hash;
- actor;
- optional correlation ID;
- optional step ID;
- optional phase ID;
- named input references;
- named output references;
- supplemental references;
- bounded disclosures;
- redaction metadata;
- side-effect request posture.

The helper returns `Passed` only after validation succeeds. Other status values remain vocabulary for future runtime semantics and are not used to imply implemented execution behavior.

## 5. Validation Assessment

Validation is deterministic and fail-closed at the helper/result boundary.

The implementation rejects:

- hook kind mismatches;
- side-effect requests;
- invalid or secret-like workflow/run/actor/reference values;
- invalid phase IDs;
- duplicate named input references;
- duplicate named output references;
- missing required input references;
- missing required output references when output validation is requested;
- invalid supplemental references;
- invalid or secret-like disclosures;
- invalid or secret-like redaction metadata.

Validation errors use stable `agent_harness_hook_invocation.*` codes and do not include raw caller-supplied values.

The implementation correctly distinguishes context validation from runtime semantics: invocation failure returns a structured error and does not mutate workflow state.

## 6. Reference Model Assessment

The stable reference vocabulary is appropriate for the current governed-work foundation.

Supported references include:

- `EvidenceReference` ID;
- local check result ID;
- typed handoff ID;
- validation reference ID;
- workflow event ID;
- audit event ID;
- policy ID;
- policy decision event ID;
- approval decision reference ID.

The helper cites supplied stable references only. It does not create EvidenceReference values, local check results, typed handoffs, validation results, policy decisions, approval decisions, workflow events, audit events, WorkReports, or report artifacts.

No fake evidence or fake citations are created when optional context is absent.

## 7. Workflow Semantics Assessment

Workflow semantics are preserved.

The helper does not:

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
- persist hook results;
- return CLI-formatted output.

Because the helper accepts only explicit model context and returns an in-memory result, hook invocation failure cannot change workflow pass/fail behavior in this phase.

## 8. Privacy And Redaction Assessment

The privacy posture is appropriate for the helper-model phase.

The implementation rejects or avoids storing:

- raw prompts;
- raw spec contents;
- raw command output;
- raw command transcripts;
- raw provider payloads;
- raw CI logs;
- raw Jira or GitHub body markers;
- parser payload markers;
- environment variable markers;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded disclosures;
- secret-like redaction metadata.

`Debug` output redacts workflow/run/actor/reference/disclosure context and reports counts for reference collections. Serialization tests verify forbidden raw payload markers are not emitted.

## 9. Serde Compatibility Assessment

Serde behavior is acceptable for the reviewed boundary.

Valid `AgentHarnessHookInvocationResult` values serialize and deserialize successfully. Invalid serialized invocation results re-enter `AgentHarnessHookInvocationResult::new(...)` and fail closed with stable non-leaking validation errors.

The serialized shape is explicit and suitable for future schema planning, but no workflow schema fields were introduced.

Non-blocking follow-up: `AgentHarnessHookNamedReference`, `AgentHarnessHookDisclosure`, and `AgentHarnessHookReference` derive standalone `Deserialize`. They are validated when used through `invoke_agent_harness_hook(...)` or stored inside `AgentHarnessHookInvocationResult`, but if they are treated later as independent public serde surfaces, they should receive constructor-routed deserialization or explicit docs stating they are context values validated by the containing invocation boundary.

## 10. Test Quality Assessment

The focused tests cover the important helper-model behaviors.

Tests cover:

- valid phase-level invocation result;
- hook kind mismatch rejection;
- missing required input rejection;
- missing required output rejection when outputs are required;
- missing required output allowed when output validation is not required;
- duplicate input reference rejection;
- stable reference vocabulary accepted without recreating evidence;
- absent optional references do not fabricate citations;
- side-effect request rejection;
- secret-like reference rejection without leakage;
- secret-like disclosure rejection without leakage;
- redaction-safe `Debug`;
- serialization non-leakage;
- serde round trip;
- invalid serialized invocation result fails closed;
- absence of encoded runtime execution behavior.

No blocking test gaps were found.

Non-blocking future tests should cover:

- direct standalone serde behavior for nested reference and disclosure context types if they become independent public payloads;
- duplicate output reference rejection as a symmetric assertion;
- redaction metadata secret-like serialized failure for invocation results;
- eventual runtime mutation/event-history tests when executor integration is scoped.

## 11. Documentation Review

Documentation is honest and aligned.

Docs state:

- the in-memory hook invocation helper model is implemented;
- runtime hook execution is not implemented;
- executor integration is not implemented;
- audit/event semantics are not implemented;
- automatic local check execution is not implemented;
- default local check handler registration is not implemented;
- CLI hook commands are not implemented;
- workflow schema fields are not implemented;
- workflow-declared hook configuration is not implemented;
- persistence changes are not implemented;
- report artifact auto-writing is not implemented;
- side-effect modeling and writes remain unsupported;
- recursive agents and agent swarms remain non-goals.

The docs continue to position Workflow OS as a governed work runtime rather than a generic multi-agent framework.

## 12. Blockers

No blockers.

## 13. Non-Blocking Follow-Ups

- Decide whether nested invocation context types should get constructor-routed standalone deserialization before they become schema-facing payloads.
- Add symmetric duplicate output reference and redaction-metadata serde regression tests in a later hardening pass.
- Decide whether future hook invocation should produce durable hook result IDs for audit and WorkReport citation.
- Decide whether missing context should eventually produce structured failed hook results instead of errors for executor-integrated paths.
- Decide hook audit/event semantics before any executor integration mutates runtime history.
- Keep runtime execution, schema exposure, CLI commands, local check execution, side effects, writes, recursive agents, and agent swarms in separate scoped phases.

## 14. Recommended Next Phase

Recommended next phase: **agent harness hook audit/event semantics planning**.

The helper model is accepted, but it should not be wired into executor behavior until Workflow OS decides how hook invocation results relate to audit records, workflow events, durable IDs, WorkReport citations, failure semantics, and runtime history. That planning phase should remain planning-only and must not implement runtime hook execution, executor integration, CLI commands, workflow schema fields, automatic local checks, persistence changes, side effects, writes, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 15. Validation

Validation commands for this review:

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
