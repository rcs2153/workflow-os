# Agent Harness Hook Audit Record Core Model Review

Review date: 2026-06-16

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The Agent Harness Hook audit record core model is appropriately model-only, validated, redaction-safe, and aligned with the accepted hook audit/event semantics plan. It adds stable hook invocation identity and a bounded audit-record model without implementing runtime hook execution, executor integration, workflow events, audit sink emission, local check execution, adapter invocation, persistence, CLI behavior, schema fields, side effects, writes, reasoning lineage, recursive agents, agent swarms, or release posture changes.

## 2. Scope Verification

The phase stayed within the approved model-only scope.

Implemented:

- `AgentHarnessHookInvocationId`;
- `AgentHarnessHookAuditRecord`;
- `AgentHarnessHookAuditRecordDefinition`;
- `AgentHarnessHookAuditRecord::from_invocation_result(...)`;
- exports through `workflow-core`;
- focused model tests;
- roadmap, concept, quickstart, and planning documentation updates;
- end-of-phase report.

No accidental implementation was found for:

- runtime hook execution;
- executor-integrated hook invocation;
- workflow event kinds;
- workflow event append behavior;
- audit sink emission;
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

## 3. Model Assessment

The model is minimal and fits the existing hook vocabulary.

`AgentHarnessHookInvocationId` is a stable, validated ID suitable for future citation or audit identity. It is bounded, character-constrained, secret-aware, serde-compatible, and redaction-safe in `Debug`.

`AgentHarnessHookAuditRecord` uses private storage fields and read-only accessors. It captures:

- hook invocation ID;
- contract ID and version;
- hook kind;
- workflow ID and version;
- run ID;
- schema version;
- spec hash;
- actor;
- invocation timestamp;
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

The implementation did not add separate `AgentHarnessHookAuditScope`, `AgentHarnessHookAuditOutcome`, or `AgentHarnessHookAuditReference` types. That is acceptable for this phase because the existing invocation status and hook reference vocabulary are sufficient for a minimal model-only record. Those types can be revisited if persistence, report citation, or executor integration creates a concrete need.

## 4. Validation Assessment

Validation is deterministic and fail-closed.

The implementation validates:

- hook invocation ID;
- hook contract ID;
- hook contract version;
- workflow ID;
- workflow version;
- run ID;
- schema version;
- spec hash;
- actor;
- optional correlation ID;
- optional step ID;
- optional phase ID;
- duplicate input reference names;
- duplicate output reference names;
- supplemental references;
- disclosures;
- redaction metadata.

Validation errors use stable `agent_harness_hook_invocation.*` and `agent_harness_hook_audit.*` codes and do not include raw caller-supplied values.

## 5. Audit/Event Semantics Assessment

The model preserves the planned audit/event boundary.

`AgentHarnessHookAuditRecord` is not a `WorkflowRunEvent`, not an `AuditEvent` projection, and not a persisted audit ledger entry. It provides a bounded record shape only.

The implementation does not:

- append workflow events;
- add workflow event variants;
- emit audit sink records;
- touch `StateBackend`;
- define a hook audit store;
- change run snapshots;
- change terminal-state behavior;
- add post-terminal metadata events.

This is the right order: record vocabulary first, then WorkReport citation and executor/audit semantics in separate phases.

## 6. Relationship To Existing Hook Models

The audit record aligns with the existing hook contract and invocation helper models.

`AgentHarnessHookAuditRecord::from_invocation_result(...)` builds from a validated `AgentHarnessHookInvocationResult` without re-running the hook helper and without inventing new evidence, approvals, local check results, typed handoffs, policy decisions, workflow events, audit events, or reports.

The model reuses:

- `AgentHarnessHookContractId`;
- `AgentHarnessHookContractVersion`;
- `AgentHarnessHookKind`;
- `AgentHarnessHookInvocationStatus`;
- `AgentHarnessHookNamedReference`;
- `AgentHarnessHookReference`;
- `AgentHarnessHookDisclosure`.

This keeps the record small and avoids a second reference taxonomy.

## 7. Privacy And Redaction Assessment

The privacy posture is appropriate for the phase.

The model does not store raw prompts, raw spec contents, raw command output, raw command transcripts, raw provider payloads, raw CI logs, raw Jira or GitHub bodies, parser payloads, environment variable values, credentials, authorization headers, private keys, token-like values, or unbounded natural-language text.

`Debug` output redacts workflow/run/actor/reference/disclosure context and reports counts for reference collections. `AgentHarnessHookInvocationId` has redaction-safe `Debug`.

Serialization stores stable references and bounded metadata. Tests verify forbidden raw payload markers are not serialized.

## 8. Serde Assessment

Serde behavior is acceptable for the reviewed boundary.

`AgentHarnessHookAuditRecord` serializes and deserializes successfully for valid records. Deserialization re-enters `AgentHarnessHookAuditRecord::new(...)` and fails closed for invalid payloads without leaking raw invalid values.

The serialized shape is explicit and suitable for future schema planning, but no workflow schema fields were introduced.

Non-blocking follow-up: nested hook context values such as `AgentHarnessHookNamedReference`, `AgentHarnessHookDisclosure`, and `AgentHarnessHookReference` still derive standalone `Deserialize`. They are validated when stored in `AgentHarnessHookAuditRecord` or `AgentHarnessHookInvocationResult`, but future schema-facing use should either route standalone deserialization through constructors or document those types as context values validated by containing records.

## 9. Test Quality Assessment

The tests cover the important model-only behaviors.

Tests cover:

- hook invocation ID invalid and secret-like rejection;
- valid hook audit record construction;
- required identity accessors;
- audit record construction from a validated invocation result;
- duplicate output reference rejection;
- secret-like reference rejection without leakage;
- redaction-safe `Debug`;
- serialization non-leakage;
- serde round trip;
- invalid serialized payload fail-closed behavior;
- absence of encoded runtime event, audit sink, persistence, executor, schema, and CLI behavior.

Existing hook invocation helper tests continue to cover invocation validation, required input/output behavior, side-effect rejection, stable reference handling, and non-leakage.

No blocking test gaps were found.

Non-blocking future tests should cover:

- duplicate input reference rejection on `AgentHarnessHookAuditRecord` directly;
- secret-like redaction metadata rejection through serialized audit record payloads;
- all invocation status variants represented on audit records;
- future WorkReport citation behavior once scoped.

## 10. Documentation Review

Documentation is honest and aligned.

Docs state:

- hook audit/event semantics planning is complete;
- hook audit record core model is implemented as model-only vocabulary and validation;
- runtime hook execution is not implemented;
- workflow event emission is not implemented;
- audit sink emission is not implemented;
- persistence is not implemented;
- executor integration is not implemented;
- automatic local check execution is not implemented;
- CLI hook commands are not implemented;
- workflow schema fields are not implemented;
- side-effect modeling and writes remain unsupported;
- recursive agents and agent swarms remain non-goals.

The roadmap and user guide continue to position Workflow OS as a governed work runtime, not a generic multi-agent framework.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Decide whether hook invocation ID and hook audit record ID should remain the same identity or split before persistence.
- Decide whether a future hook audit store is needed or whether records should be projected from future hook workflow events.
- Decide WorkReport hook citation vocabulary before reports cite hook records.
- Consider constructor-routed standalone deserialization for nested hook context values before schema exposure.
- Add direct audit-record tests for duplicate input references, redaction metadata serialized failure, and all invocation statuses.
- Keep executor integration, workflow events, audit sink emission, persistence, CLI behavior, schemas, side effects, writes, recursive agents, and agent swarms in separate scoped phases.

## 13. Recommended Next Phase

Recommended next phase: **WorkReport hook citation target planning**.

The hook audit record model now provides stable hook invocation identity, but WorkReports do not yet cite hook records or hook invocation IDs. A planning-only phase should decide whether WorkReports cite hook invocation IDs, hook audit records, future workflow event IDs, or another stable reference. It must not implement runtime hook execution, executor integration, event emission, audit sink emission, persistence, CLI behavior, schema changes, side effects, writes, recursive agents, agent swarms, or release posture changes.

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
