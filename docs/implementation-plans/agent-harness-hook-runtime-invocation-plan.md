# Agent Harness Hook Runtime Invocation Plan

Status: Planning only. Runtime agent harness hook invocation is not implemented.

## 1. Executive Summary

The agent harness hook contract model is implemented and reviewed as a model-only boundary. Workflow OS can now represent deterministic named checkpoint contracts, but it does not invoke hooks at runtime.

The next question is where and how hooks should eventually be invoked without making the agent scaffold brittle, over-automated, or authority-expanding. This plan defines a conservative runtime invocation direction for future work.

This plan does not implement runtime hook execution, CLI hook commands, workflow schema fields, automatic local checks, default handler registration, persistence changes, report artifact auto-writing, side-effect modeling, writes, hosted behavior, recursive agents, agent swarms, or release posture changes.

## 2. Goals

- Define a future runtime invocation boundary for deterministic named hooks.
- Preserve the product model: `Agent executes. Workflow OS governs.`
- Reduce reliance on prose-only agent instruction following.
- Keep hook invocation local, explicit, deterministic, and auditable.
- Reuse existing primitives: workflow/run identity, step identity, policy gates, approvals, local check result references, EvidenceReference, typed handoffs, WorkReports, audit records, and runtime status.
- Preserve workflow semantics until hook failure behavior is separately implemented and reviewed.
- Avoid raw payload copying.
- Avoid creating fake evidence, fake approvals, fake local check results, fake typed handoffs, or fake reports.
- Prepare a small future implementation phase that introduces invocation types only, before any executor integration.

## 3. Non-Goals

Do not implement in this plan:

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

## 4. Invocation Boundary Recommendation

The first runtime-facing implementation should not wire hooks directly into `LocalExecutor`.

Recommended first implementation after this plan:

1. Add model-only or helper-only invocation request/result types.
2. Accept explicit hook contract and explicit invocation context.
3. Validate context against the hook contract.
4. Return an in-memory structured hook invocation result.
5. Do not execute external commands, local checks, adapters, or agent code.
6. Do not mutate workflow state or append runtime events.

This gives Workflow OS a deterministic invocation envelope before deciding whether hooks should be executor-integrated.

## 5. Invocation Level

Future hook invocation should initially be **phase-level within a workflow run**.

Rationale:

- Phase-level checkpoints match the current agent harness mental model: planning, implementation, validation, review, and report.
- They avoid premature workflow schema changes.
- They can be supplied explicit context by the agent or harness helper.
- They are easier to test without mutating runtime state.

Deferred levels:

- workflow-level hooks;
- harness-level hooks;
- step-level hooks;
- nested harness hooks.

These may be added later after the phase-level invocation envelope is reviewed.

## 6. Contract Source

The first invocation helper should accept an explicit `AgentHarnessHookContract`.

Do not add:

- workflow schema fields;
- workflow-declared hook contracts;
- runtime config;
- default hook registry;
- CLI hook configuration.

Explicit contract input keeps the boundary testable and avoids ambient hook behavior.

## 7. Invocation Input Context

A future invocation input model should include:

- hook contract;
- workflow ID;
- workflow version;
- run ID;
- schema version;
- spec hash;
- hook kind;
- actor or system actor;
- generated or invoked timestamp;
- optional correlation ID;
- optional step ID or phase ID if available;
- supplied input references by stable name;
- supplied output expectations by stable name;
- local check result references when already available;
- EvidenceReference IDs when already available;
- typed handoff IDs when already available;
- policy decision IDs when already available;
- approval decision IDs when already available;
- bounded notes, risks, limitations, or disclosures if needed.

Inputs must be explicit. The helper must not read hidden global state.

## 8. Forbidden Inputs

Hook invocation inputs must not copy or store:

- raw prompts;
- raw spec contents;
- raw command output;
- raw command transcripts;
- raw provider payloads;
- raw CI logs;
- raw Jira bodies or comments;
- raw GitHub file contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded natural-language summaries.

File paths should be treated as sensitive references and should not be expanded into raw contents.

## 9. Invocation Result Shape

A future invocation result should be in-memory and structured.

Candidate fields:

- hook contract ID;
- hook contract version;
- hook kind;
- workflow ID;
- run ID;
- actor or system actor;
- invoked timestamp;
- status;
- validated input names;
- validated output names;
- cited local check result references;
- cited EvidenceReference IDs;
- cited typed handoff IDs;
- cited policy or approval references when supplied;
- missing required input names;
- missing required output names;
- bounded disclosures;
- redaction metadata;
- sensitivity.

Candidate statuses:

- `passed`;
- `failed_closed`;
- `warning`;
- `skipped_with_disclosure`;
- `blocked`;

These statuses should remain model vocabulary until runtime semantics are separately accepted.

## 10. Failure Semantics

The first invocation helper should fail closed when:

- the hook contract is invalid;
- the hook kind does not match the invocation context;
- required inputs are missing;
- required outputs are missing if the invocation point expects outputs;
- supplied references are invalid or secret-like;
- redaction metadata is invalid;
- side effects are requested or implied.

Hook invocation failure must not automatically fail or mutate a workflow run in the first implementation. It should return a structured non-leaking hook result or error to the caller.

Executor-level behavior is deferred. A later executor integration plan must decide whether a hook failure blocks, warns, fails the run, pauses for approval, or records an audit event.

## 11. Audit And Event Posture

The first invocation helper should not append workflow events or audit records.

Reasons:

- audit/event semantics for hooks affect runtime history;
- hook invocation may be used in pre-runtime or agent-facing contexts;
- event emission should be designed with executor integration;
- runtime mutation would exceed a narrow invocation-helper phase.

Future executor integration should decide:

- whether hook invocation emits audit events;
- whether hook failures emit audit events;
- whether hook result IDs become stable references;
- whether hook events are part of workflow event history or a separate audit stream.

## 12. Relationship To Local Checks

Hooks must not automatically execute local checks.

A hook invocation may cite local check result references already produced by explicit local check APIs. It must not:

- register local check handlers;
- run `DocsCheck`;
- run arbitrary shell commands;
- create command-output evidence;
- treat absent local check results as successful checks.

Missing local check references should be represented as missing context, not fabricated success.

## 13. Relationship To EvidenceReference

Hooks may cite `EvidenceReference` IDs supplied by the caller.

Hooks must not:

- create EvidenceReference values implicitly;
- attach evidence to diagnostics, adapters, approvals, or reports;
- copy evidence payloads;
- copy raw source, command, provider, parser, CI, Jira, or GitHub payloads.

Evidence completeness rules should be planned separately if hook contracts begin requiring evidence.

## 14. Relationship To WorkReports

Hooks may later feed bounded structured checkpoint results into WorkReport sections.

The first invocation helper should not:

- generate WorkReports;
- write report artifacts;
- append report audit events;
- change terminal report generation behavior;
- expose CLI report rendering.

Any WorkReport integration should remain a future planning phase.

## 15. Relationship To Approvals And Policy Gates

Hooks may cite supplied policy decisions or approval decisions by stable reference.

Hooks must not:

- request approvals;
- decide approvals;
- bypass approval gates;
- create approval evidence;
- create policy decisions;
- treat model self-review as policy.

If a future hook contract requires approval or policy context, missing references should fail closed at the hook invocation boundary.

## 16. Relationship To Side Effects And Writes

Hooks must not authorize side effects.

The current `AgentHarnessHookContract` rejects side-effect authorization. Runtime invocation must preserve that boundary.

No hook invocation phase should add writes until side-effect boundary modeling, high-assurance approval controls, write-capable adapter policy, and audit semantics are accepted.

## 17. Privacy And Redaction

Future invocation models must:

- use existing validated ID and redaction primitives;
- bound all caller-supplied text;
- reject secret-like values;
- redact Debug output;
- fail closed on invalid serialized payloads;
- avoid leaking raw values in errors;
- serialize only validated bounded references and metadata.

Reports, hook results, audit records, and errors may be sensitive even when they contain only references.

## 18. Test Plan For Future Implementation

Future invocation-helper tests should cover:

- valid phase-level hook invocation context;
- hook kind mismatch rejection;
- required input missing failure;
- required output missing failure when output validation is required;
- invalid hook contract rejection;
- secret-like reference rejection without leakage;
- local check result references accepted by stable ID only;
- EvidenceReference IDs accepted by stable ID only;
- typed handoff IDs accepted by stable ID only;
- policy and approval references accepted only when supplied;
- absent optional references do not fabricate citations;
- side-effect requests rejected;
- no workflow state mutation;
- no workflow events appended;
- no audit events emitted in helper-only phase;
- no local checks executed;
- no adapters invoked;
- no filesystem artifacts created;
- no CLI output emitted;
- serde round trip for valid invocation result if serialized;
- invalid serialized invocation result fails closed;
- Debug output redaction;
- existing hook contract, local check, EvidenceReference, WorkReport, runtime, and validation tests still pass.

## 19. Proposed Implementation Sequence

Recommended small phases:

1. Hook runtime invocation helper model, in-memory only.
2. Maintainer review.
3. Hook audit/event semantics planning.
4. Executor integration planning.
5. Optional executor-integrated hook invocation implementation.
6. Workflow schema planning only after runtime/API behavior is reviewed.
7. CLI hook commands only after schema/API behavior is stable.

The next implementation should start with invocation helper model only, not executor integration.

## 20. Open Questions

- Should hook invocation produce a durable hook result ID?
- Should hook invocation results eventually be cited by WorkReports?
- Should missing required outputs be validated at `before_*` hooks or only `after_*` hooks?
- Should hook failures ever become workflow failures automatically?
- Should warning-only hooks be allowed before fail-closed hooks?
- Should hooks be invoked by an agent-facing helper, executor integration, or both?
- Should hook audit records live in the workflow event stream or a separate audit stream?
- Should hook contracts later include first-class policy, approval, evidence, local check, report, or handoff requirements?
- What is the smallest useful first checkpoint for dogfooding: before validation, after validation, before report, or after report?

## 21. Final Recommendation

Proceed next to **agent harness hook runtime invocation helper model, in-memory only**.

That phase should add explicit request/result model types and validation for phase-level hook invocation context. It must not execute hooks, integrate with `LocalExecutor`, append events, emit audit records, run local checks, invoke adapters, write files, persist results, expose CLI behavior, add schema fields, authorize side effects, add writes, implement reasoning lineage, enable recursive agents, enable agent swarms, or change release posture.
