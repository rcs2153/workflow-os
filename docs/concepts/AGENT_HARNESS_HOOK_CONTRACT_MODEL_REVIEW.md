# Agent Harness Hook Contract Model Review

Review date: 2026-06-16

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The Agent Harness Hook Contract model is appropriately model-only, bounded, validated, and aligned with the accepted hook integration plan. It adds deterministic checkpoint vocabulary and contract validation without implementing runtime hook invocation, CLI hook commands, workflow schema fields, automatic local checks, persistence, report artifacts, side-effect modeling, writes, hosted behavior, recursive agents, agent swarms, or release posture changes.

## 2. Scope Verification

The phase stayed within approved model-only scope.

Implemented:

- hook contract identity and version;
- hook kind vocabulary;
- required input and output requirements;
- failure semantics vocabulary;
- sensitivity and redaction policy;
- redaction metadata validation;
- side-effect posture that rejects authorization;
- serde support through validated construction;
- redaction-safe `Debug`;
- focused tests and documentation updates.

No accidental implementation was found for:

- runtime hook execution;
- automatic workflow execution;
- automatic local check execution;
- default handler registration;
- command-output evidence;
- CLI hook commands;
- workflow schema fields;
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

## 3. Model Assessment

The implemented model is appropriately small and domain-neutral.

The core model includes:

- `AgentHarnessHookContract`;
- `AgentHarnessHookContractDefinition`;
- `AgentHarnessHookContractId`;
- `AgentHarnessHookContractVersion`;
- `AgentHarnessHookKind`;
- `AgentHarnessHookInputRequirement`;
- `AgentHarnessHookOutputRequirement`;
- `AgentHarnessHookFailureSemantics`;
- `AgentHarnessHookSideEffectAllowance`.

This is enough to represent a future deterministic checkpoint contract without adding execution behavior. The model reuses existing Workflow OS privacy and contract patterns: validated ID wrappers, private stored fields, accessor methods, redacted `Debug`, and manual deserialization through `new(...)`.

## 4. Hook Vocabulary Assessment

The hook kind vocabulary covers the planned lifecycle checkpoints:

- planning;
- implementation;
- validation;
- review;
- report generation.

Both before and after variants are represented. This matches the hook plan's `before_*` and `after_*` direction while leaving runtime placement, hook invocation shape, and schema declaration deferred.

One non-blocking design question remains: the model uses contract ID plus hook kind rather than a separate hook display name. That is acceptable for this phase because the accepted first slice required hook identity and hook kind/name, and the combination of ID plus kind satisfies the contract identity boundary. A future runtime or schema phase can decide whether an explicit name field is needed.

## 5. Validation Assessment

Validation is deterministic and fail-closed.

The implementation validates:

- hook contract ID is non-empty, bounded, character-constrained, and secret-aware;
- hook contract version is non-empty, bounded, character-constrained, and secret-aware;
- schema version is checked for secret-like text;
- purpose is non-empty, bounded, and secret-aware;
- input requirements are required;
- output requirements are required;
- duplicate input names are rejected;
- duplicate output names are rejected;
- failure semantics are required;
- duplicate failure semantics are rejected;
- side-effect authorization via `ProposedOnly` is rejected;
- redaction metadata field names and reasons are bounded and secret-aware.

Validation errors use stable `agent_harness_hook.*` codes and do not include raw caller-supplied values.

## 6. Serde Assessment

Serde behavior is appropriate for the phase.

Valid contracts serialize and deserialize successfully. Invalid serialized contracts are routed through the validated `AgentHarnessHookContract::new(...)` boundary and fail closed. Deserialization errors use stable validation codes and do not leak secret-like payloads.

The serialized shape is suitable for future schema planning because it is explicit and field-based, but no workflow schema fields were introduced.

## 7. Privacy And Redaction Assessment

The privacy posture is strong for model-only scope.

The model:

- redacts contract IDs in `Debug`;
- redacts purpose text in `Debug`;
- redacts input and output names in `Debug`;
- redacts redaction metadata in `Debug`;
- rejects authorization, bearer, private-key, API-token, secret, and token-like strings;
- rejects raw provider payload markers;
- rejects raw command output markers;
- rejects raw spec content markers;
- rejects parser payload markers;
- rejects environment variable markers;
- validates redaction metadata before storage in the contract.

No raw provider payloads, command outputs, raw spec contents, parser payloads, environment values, credentials, authorization headers, private keys, or token-like values are stored by the model.

## 8. Side-Effect Boundary Assessment

The model correctly refuses to authorize side effects.

`AgentHarnessHookSideEffectAllowance::Unsupported` and `None` are representable, while `ProposedOnly` is rejected by `AgentHarnessHookContract::validate(...)`. This gives the model enough vocabulary to document posture without enabling side effects.

This is the right boundary before side-effect modeling and write-capable adapter policy are separately accepted.

## 9. Test Quality Assessment

The focused tests are good and cover the right model behaviors.

Tests cover:

- valid minimal contract construction;
- invalid hook ID rejection;
- invalid version rejection;
- hook kind vocabulary;
- required input validation;
- required output validation;
- duplicate input rejection;
- duplicate output rejection;
- failure semantic duplicate rejection;
- side-effect authorization rejection;
- no-side-effect posture representation;
- sensitivity and redaction policy serde;
- serde round trip;
- invalid serialized contract fail-closed behavior;
- deserialization error non-leakage;
- redaction metadata validation;
- raw payload marker rejection;
- redaction-safe `Debug`;
- serialization non-leakage;
- absence of encoded runtime hook behavior.

No blocking test gaps were found.

Non-blocking future test additions should be considered when runtime hook invocation is planned:

- explicit tests for hook invocation level, such as workflow, harness, step, or phase;
- tests for missing policy/evidence/local-check requirements if those fields are added later;
- tests for hook runtime errors only after runtime invocation is scoped.

## 10. Documentation Review

Documentation is honest and aligned.

Docs now state:

- the agent harness hook contract model is implemented;
- runtime hook invocation is not implemented;
- scaffold files remain orientation, not enforcement;
- automatic workflow execution is not implemented;
- automatic local check execution is not implemented;
- CLI hook commands are not implemented;
- workflow schema fields are not implemented;
- persistence and report artifact auto-writing are not implemented;
- side-effect modeling and writes remain unsupported;
- recursive agents and agent swarms remain non-goals.

The roadmap and quickstart preserve Workflow OS's positioning as a governed work runtime, not a generic multi-agent framework.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Decide whether future hook invocation is workflow-level, harness-level, step-level, or phase-level.
- Decide whether a future schema/runtime phase needs an explicit hook name separate from contract ID and hook kind.
- Decide whether future hook contracts should include policy checks, approval requirements, evidence requirements, local check requirements, and handoff/report obligations as first-class fields.
- Keep runtime invocation, audit event emission, CLI hook commands, and schema fields in separate planning phases.
- Keep side-effecting hooks deferred until side-effect boundary modeling and approval controls are accepted.

## 13. Recommended Next Phase

Recommended next phase: **agent harness hook runtime invocation planning**.

That phase should be planning-only. It should decide where hooks sit in the governed execution lifecycle, what explicit inputs they require, whether invocation is workflow-level/harness-level/step-level/phase-level, how hook failures interact with workflow semantics, and whether hook invocation emits audit events. It must not implement runtime hook execution, CLI commands, workflow schema fields, automatic local checks, side effects, writes, hosted behavior, recursive agents, agent swarms, or release posture changes.

## 14. Validation

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
