# SideEffect Core Model Review

Review date: 2026-06-17

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The SideEffect core model phase delivers the approved model-only scope: domain-neutral side-effect model types, lifecycle and authority separation, idempotency binding, stable references, deterministic validation, serde support, redaction-safe `Debug`, focused tests, documentation updates, and an end-of-phase report.

The implementation does not introduce writes, provider mutations, write-capable adapters, runtime side-effect execution, persistence, workflow events, audit projections, schemas, CLI behavior, examples, hosted behavior, or release posture changes.

## 2. Scope Verification

The phase stayed within approved model-only scope.

No accidental implementation was found for:

- writes or provider mutations;
- write-capable adapters;
- generic runtime adapter execution;
- runtime side-effect proposal, attempt, completion, denial, skip, or failure paths;
- side-effect workflow events;
- side-effect audit projections or sinks;
- side-effect persistence or state backend changes;
- WorkReport side-effect citation support;
- EvidenceReference side-effect attachment;
- workflow schema fields;
- CLI commands or rendering;
- examples;
- domain packs;
- hosted or distributed runtime behavior;
- rollback or compensation behavior;
- Level 3/4 autonomy enablement;
- release posture changes.

## 3. Model Assessment

The model is domain-neutral and appropriately minimal for the first side-effect boundary.

Implemented concepts include:

- `SideEffectRecord`;
- `SideEffectRecordDefinition`;
- `SideEffectId`;
- `SideEffectLifecycleState`;
- `SideEffectTargetKind`;
- `SideEffectTargetReference`;
- `SideEffectCapability`;
- `SideEffectAuthority`;
- `SideEffectAuthorityDecision`;
- `SideEffectIdempotencyBinding`;
- `SideEffectIdempotencyScope`;
- `SideEffectOutcomeReference`;
- `SideEffectOutcomeReferenceKind`;
- `SideEffectReference`;
- `SideEffectReferenceKind`;
- `SideEffectSensitivity`.

The model records side-effect intent and lifecycle state. It does not execute side effects.

## 4. Lifecycle Assessment

The lifecycle vocabulary matches ADR 0011 and the implementation plan:

- proposed;
- attempted;
- completed;
- denied;
- skipped;
- failed.

The implementation correctly avoids `approved` as a lifecycle state. Approval remains authority context.

Lifecycle-specific validation is present:

- attempted, completed, and failed records require allowed authority;
- completed records require an outcome reference;
- denied records require denied or unsupported authority;
- denied and skipped records require stable reason codes;
- failed records require a failure reference or stable reason code.

This preserves the separation between proposed work, authorized work, attempted work, completed work, denied work, skipped work, and failed work.

## 5. Authority Assessment

Authority is modeled separately from lifecycle state through `SideEffectAuthority` and `SideEffectAuthorityDecision`.

The authority vocabulary covers:

- not evaluated;
- allowed by policy;
- requires approval;
- approved by human;
- denied by policy;
- denied by approval;
- denied by capability;
- denied by kill switch;
- denied by validation;
- unsupported.

Attempted, completed, and failed records reject denied, unsupported, and not-evaluated authority. Denied records reject allowed authority. Unknown capability can be recorded only in denied form, which preserves fail-closed behavior without making unsafe requests disappear.

## 6. Target And Capability Assessment

Targets are represented as bounded references through `SideEffectTargetReference`. Capability vocabulary is domain-neutral and includes external, local, adapter, GitHub, Jira, CI, workflow dispatch, CI rerun, and unknown capabilities.

The model correctly avoids provider-specific payloads, command arguments, raw URLs with secrets, issue bodies, file contents, and write routing. Unknown target kind is rejected by target validation. Unknown capability fails closed for attempted, completed, and failed states.

## 7. Idempotency Assessment

`SideEffectIdempotencyBinding` records:

- idempotency key;
- binding scope;
- optional prior side-effect ID;
- optional outcome reference.

This is the right model-only boundary. The implementation does not add idempotency-store behavior, retry behavior, duplicate mutation handling, or adapter execution.

## 8. Reference Assessment

`SideEffectReference` supports stable references to:

- workflow events;
- audit events;
- policy decisions;
- approval decisions;
- adapter telemetry;
- EvidenceReference IDs;
- WorkReport IDs;
- local check results;
- typed handoffs.

References are bounded, non-payload strings. Duplicate references are rejected. The model does not fabricate missing IDs and does not copy referenced payloads.

## 9. Validation Assessment

Validation is deterministic and redaction-safe.

It enforces:

- valid side-effect ID;
- valid target reference;
- known target kind;
- lifecycle and authority compatibility;
- lifecycle and capability compatibility;
- actor or system actor presence;
- idempotency binding validation;
- bounded reference lists;
- duplicate reference rejection;
- completed outcome requirement;
- denied/skipped reason requirements;
- failed reference or reason requirement;
- bounded summary;
- bounded and unique reason codes;
- bounded redaction metadata;
- secret-like value rejection;
- invalid serialized records failing closed.

Validation errors use stable `side_effect.*` codes and do not include rejected raw target values, summaries, reason values, redaction values, provider payloads, command output, snippets, paths, credentials, or token-like strings.

## 10. Privacy And Redaction Assessment

The model maintains the expected privacy posture.

It rejects secret-like:

- IDs;
- target references;
- side-effect references;
- outcome references;
- summaries;
- reason codes;
- redaction metadata fields;
- redaction metadata reasons.

It does not store:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira bodies/comments;
- raw GitHub file contents;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded summaries.

`Debug` output redacts IDs, target references, workflow/run identity, actors, step/skill/adapter/integration identifiers, correlation IDs, summaries, references, and redaction metadata details. Serialization of valid records remains bounded and does not introduce forbidden raw payload fields.

## 11. Serde And Compatibility Assessment

Valid records serialize and deserialize successfully.

Invalid serialized records fail closed through constructor-backed deserialization for the main model and bounded nested reference types. The serialization shape is suitable for future schema work, but no workflow schema fields were introduced.

The model is exported from `workflow-core`, which is consistent with other canonical core models. Public schema exposure remains deferred.

## 12. Runtime And State Boundary Assessment

The implementation does not touch runtime behavior.

No code path was added that:

- mutates `WorkflowRun`;
- appends workflow events;
- emits audit events;
- emits observability events;
- writes state backend records;
- persists side-effect records;
- invokes adapters;
- calls external systems;
- writes files;
- emits CLI output.

Existing policy, approval, adapter, runtime, local check, EvidenceReference, WorkReport, and typed handoff behavior remains unchanged.

## 13. Test Quality Assessment

The focused SideEffect test suite covers:

- valid proposed record;
- all required lifecycle states;
- completed outcome requirement;
- attempted denied-authority rejection;
- denied authority requirement;
- denied/skipped reason requirements;
- unknown capability fail-closed behavior;
- denied unknown-capability recording;
- invalid ID rejection;
- secret-like target rejection without leakage;
- duplicate reference rejection;
- actor/system actor requirement;
- system actor-only validity;
- secret-like summary, reason, and redaction metadata rejection;
- serde round trip;
- invalid serialized secret-like target failure without leaking value;
- redaction-safe Debug;
- serialization non-leakage of forbidden raw payload markers;
- valid redaction metadata;
- idempotency binding with prior side-effect reference.

Existing workspace tests also passed, including policy, approval, adapter, EvidenceReference, WorkReport, local check, runtime event, and executor tests.

No blocker-level test gaps were found.

## 14. Documentation Review

Documentation accurately states:

- SideEffect core model is implemented;
- writes are not implemented;
- write-capable adapters are not implemented;
- runtime side-effect execution is not implemented;
- side-effect persistence is not implemented;
- side-effect workflow events are not implemented;
- side-effect audit projections are not implemented;
- WorkReport side-effect citation support was not implemented in this phase;
- EvidenceReference side-effect attachment is not implemented;
- schemas are not updated;
- CLI behavior is not added;
- examples are not updated;
- hosted behavior and release posture are unchanged.

The ADR, roadmap, governed work concept, EvidenceReference concept, WorkReport planning document, side-effect plan, and phase report are consistent with the implemented scope.

## 15. Blockers

No blockers.

## 16. Non-Blocking Follow-Ups

- Consider making `SideEffectAuthority` fields private, or add explicit standalone mutation-bypass tests, before treating authority as a durable standalone public contract. `SideEffectRecord` construction revalidates authority today, so this is not a blocker.
- Add explicit tests for unknown target kind deserialization failure if future schema exposure makes target kind compatibility important.
- WorkReport side-effect citation vocabulary was planned and later implemented as model-only vocabulary in [WorkReport SideEffect Citation Report](WORK_REPORT_SIDE_EFFECT_CITATION_REPORT.md).
- Plan side-effect workflow events, audit projection, and persistence separately before any runtime side-effect execution.
- Keep write-adapter readiness and high-assurance approval control planning behind the reviewed SideEffect model boundary.

## 17. Recommended Next Phase

Recommended next phase at review time: WorkReport side-effect citation planning.

The SideEffect model is accepted enough to plan how WorkReports should cite side-effect records without making WorkReport prose the source of truth. That planning should remain model/report-citation only and must not introduce writes, runtime side-effect execution, persistence, adapter mutations, CLI behavior, schemas, examples, hosted behavior, or release posture changes.

High-assurance approval controls and write-adapter readiness can continue in parallel planning lanes, but write-capable adapter implementation should still wait until side-effect citation, event/audit, persistence, and approval semantics are explicitly reviewed.

Follow-up status: WorkReport side-effect citation vocabulary is now implemented as model-only vocabulary. Terminal helper propagation, executor propagation, side-effect workflow events, audit projection, persistence, runtime side-effect execution, writes, schemas, CLI behavior, examples, hosted behavior, and release posture changes remain unimplemented.

## 18. Validation

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
