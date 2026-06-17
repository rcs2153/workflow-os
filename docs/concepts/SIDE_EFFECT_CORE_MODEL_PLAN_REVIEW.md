# SideEffect Core Model Plan Review

Review date: 2026-06-17

## 1. Executive Verdict

Plan accepted with non-blocking follow-ups.

The SideEffect Core Model Plan is sufficiently scoped and implementation-ready. It preserves the accepted ADR 0011 boundary by limiting the next phase to model types, validation, serde, redaction-safe Debug behavior, and focused tests. It does not authorize writes, write-capable adapters, runtime side-effect execution, persistence, schemas, CLI behavior, examples, hosted behavior, or release posture changes.

## 2. Scope Verification

The plan stays within planning-only scope and correctly positions the next implementation as model-only.

No accidental authorization was found for:

- writes or provider mutations;
- write-capable adapters;
- generic runtime adapter execution;
- runtime side-effect proposal, attempt, or completion paths;
- workflow events or audit projections for side effects;
- side-effect persistence or state backend changes;
- WorkReport generation changes;
- automatic report artifact behavior;
- EvidenceReference side-effect attachment;
- workflow schema fields;
- CLI commands or rendering;
- examples;
- domain packs;
- hosted or distributed runtime behavior;
- Level 3/4 autonomy claims;
- release posture changes.

## 3. Model Boundary Assessment

The proposed model boundary is domain-neutral and appropriately minimal. It describes a `SideEffectRecord` as a record of governed mutation intent and outcome, not as a write executor.

The planned concepts cover the necessary v1 surface:

- side-effect identity;
- lifecycle state;
- target reference;
- requested capability;
- authority context and decision;
- idempotency binding;
- stable references to related governance records;
- sensitivity;
- redaction metadata.

The plan correctly avoids modeling provider-specific payloads, command transcripts, file contents, write routing, adapter execution, or rollback behavior.

## 4. Required Field Assessment

The required field list is complete for a model-only first slice.

The plan requires:

- side-effect ID;
- lifecycle state;
- target reference;
- requested capability;
- authority;
- actor or system actor;
- workflow/run identity when applicable;
- schema version;
- spec hash when run-scoped;
- idempotency binding;
- stable references;
- summary or bounded reason code;
- sensitivity;
- redaction metadata.

This is enough to make a later runtime execution phase citeable and auditable without letting the model imply that writes are available.

## 5. Lifecycle State Assessment

The lifecycle state vocabulary is appropriate: proposed, denied, skipped, attempted, completed, and failed are distinct and sufficient for the first model.

The plan correctly states that:

- approval is not equivalent to execution;
- proposed does not imply attempted;
- completed must require outcome references;
- failed must require failure references or stable failure codes;
- skipped and denied are first-class terminal outcomes;
- invalid lifecycle transitions should fail closed.

This separation is important because Workflow OS needs to prove what was authorized, what was attempted, and what actually happened without collapsing those into a single vague "action" state.

## 6. Authority Assessment

The authority model is well scoped. It separates capability, actor/system actor, policy/approval references, and authority decision from lifecycle state.

The plan correctly requires attempted and completed records to reject denied or unsupported authority. It also keeps high-assurance approval controls as future work rather than smuggling them into the core model phase.

## 7. Target And Capability Assessment

The plan treats targets and capabilities as bounded references rather than raw provider payloads. That is the right boundary for this phase.

The plan should continue to avoid provider-specific target schemas in the first implementation. Stable, bounded target references are enough to support later adapter-specific expansion without leaking raw URLs, issue bodies, command arguments, file contents, or secrets.

## 8. Idempotency Assessment

Idempotency is correctly modeled as a binding, not as retry behavior.

The next implementation should keep this distinction sharp: the model may record the key/scope/strategy needed for future safe retry decisions, but must not add mutation retry execution or adapter-level idempotency enforcement.

## 9. Reference And Existing Model Relationship Assessment

The plan correctly preserves existing model boundaries:

- workflow events remain run-state truth;
- audit events remain governance projections;
- adapter telemetry remains adapter invocation summary;
- EvidenceReference remains the citation substrate;
- WorkReport remains the final governed handoff artifact;
- policy and approval records remain their own decision sources.

SideEffect should cite these models by stable references only. The plan correctly defers WorkReport side-effect citation vocabulary unless separately scoped.

## 10. Validation Assessment

The planned validation boundary is deterministic and redaction-safe.

It covers:

- valid identity fields;
- valid lifecycle state;
- bounded target references;
- valid capability/lifecycle compatibility;
- authority compatibility;
- workflow/run identity;
- schema version and spec hash;
- idempotency binding;
- bounded reference lists;
- duplicate reference rejection;
- bounded summaries and reason codes;
- sensitivity;
- redaction metadata;
- invalid serde failure.

The plan also requires stable non-leaking validation error codes. That is necessary before side-effect records can become an input to reports, audit projections, or future policy-gated writes.

## 11. Privacy And Redaction Assessment

The privacy posture is strong and consistent with prior EvidenceReference and WorkReport phases.

The plan explicitly forbids:

- raw provider payloads;
- command output;
- CI logs;
- Jira issue/comment bodies;
- GitHub file contents;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded summaries;
- secret-like target identifiers;
- secret-like redaction metadata.

It also requires redaction-safe Debug output, safe serialization behavior, and non-leaking deserialization errors.

## 12. Runtime And State Boundary Assessment

The plan cleanly preserves runtime boundaries.

The next implementation must not:

- mutate workflow state;
- append workflow events;
- emit audit events;
- emit observability events;
- persist side-effect records;
- write files;
- call adapters;
- call external systems;
- expose CLI output;
- change executor behavior;
- change policy behavior;
- change approval behavior.

This is exactly the right posture before Workflow OS has reviewed side-effect eventing, persistence, approval controls, and write-adapter readiness.

## 13. Test Plan Assessment

The planned tests are broad enough for a model-only implementation.

They cover:

- valid records across lifecycle states;
- invalid identity and target rejection;
- capability/lifecycle compatibility;
- authority compatibility;
- actor/system actor requirements;
- workflow/run identity requirements;
- spec hash and idempotency binding;
- duplicate reference rejection;
- bounded reason/summary validation;
- redaction metadata validation;
- sensitivity validation;
- serde round trip;
- invalid serde failure;
- Debug and serialization non-leakage;
- proof that runtime behavior, persistence, schemas, CLI, examples, and writes are not introduced;
- existing policy, approval, adapter, EvidenceReference, WorkReport, local check, and runtime regression tests.

No test blockers were found.

## 14. Documentation Review

The plan requires future implementation docs to state:

- SideEffect core model is implemented;
- writes are not implemented;
- write-capable adapters are not implemented;
- runtime side-effect execution is not implemented;
- side-effect persistence is not implemented;
- side-effect workflow events and audit projections are not implemented unless separately scoped;
- WorkReport side-effect citation support is not implemented unless separately scoped;
- EvidenceReference side-effect attachment is not implemented;
- schemas are not updated;
- CLI behavior is not added;
- examples are not updated;
- release posture is unchanged.

That documentation boundary is sufficient for the next implementation phase.

## 15. Planning Blockers

No planning blockers.

## 16. Non-Blocking Follow-Ups

- During implementation, keep `SideEffectTargetReference` bounded and provider-neutral; avoid provider-specific target schemas until adapter write readiness is separately planned.
- Treat WorkReport side-effect citation vocabulary as a separate phase unless model compilation truly requires shared vocabulary.
- Keep idempotency as recorded intent/identity only; do not introduce retry or mutation behavior.
- Prefer stable reason codes over prose for denied, skipped, and failed records where possible.

## 17. Recommended Next Phase

Recommended next phase: SideEffect core model implementation.

The plan is ready for a small Rust model-only phase that adds the SideEffect types, validation, serde support, redaction-safe Debug behavior, focused tests, documentation updates, and `SIDE_EFFECT_CORE_MODEL_REPORT.md`. The next phase must still exclude writes, runtime execution, adapter mutation, persistence, schemas, CLI behavior, examples, hosted behavior, and release posture changes.

## 18. Validation

- `npm run check:docs`
- `git diff --check`

