# SideEffect Core Model Plan

Status: Implemented as model-only core types in `workflow-core` and accepted in [SideEffect Core Model Review](../concepts/SIDE_EFFECT_CORE_MODEL_REVIEW.md). ADR 0011 is accepted as architecture direction, but this plan does not implement writes, write-capable adapters, generic runtime adapter execution, workflow events, audit projections, persistence changes, schemas, CLI behavior, examples, hosted runtime behavior, or release posture changes.

## 1. Executive Summary

[ADR 0011: Side-Effect Boundary Core Model](../adr/0011-side-effect-boundary.md) accepts the architecture direction for a domain-neutral side-effect boundary.

The implementation adds only the SideEffect core model: Rust model types, deterministic validation, serde support, redaction-safe Debug behavior, and focused tests.

The model should represent side-effect intent, target, requested capability, authority context, lifecycle state, idempotency binding, stable references, sensitivity, and redaction metadata without enabling mutation. It should make future writes safer by giving Workflow OS a first-class record for proposed, attempted, completed, denied, skipped, and failed side effects.

This plan does not implement writes. It does not add provider mutations, runtime write routing, schemas, CLI behavior, examples, persistence changes, workflow events, audit sinks, or automatic report artifact behavior.

## 2. Goals

- Add domain-neutral SideEffect core model types.
- Represent side-effect lifecycle state without treating approval as execution.
- Represent side-effect authority separately from lifecycle state.
- Preserve fail-closed write posture.
- Bind future side effects to workflow/run identity when run-scoped.
- Bind future side effects to policy, approval, audit, adapter telemetry, evidence, and idempotency references where available.
- Keep raw provider payloads, command output, logs, specs, and secrets out of the model.
- Add deterministic validation with stable non-leaking error codes.
- Add serde round-trip and invalid serde failure tests.
- Add redaction-safe Debug behavior.
- Prepare for later WorkReport citation target support without implementing it unless explicitly included in the model-only scope.

## 3. Non-Goals

Do not implement:

- external writes;
- local filesystem writes through the runtime;
- GitHub branch creation, pull request creation, comments, issue updates, CI reruns, workflow dispatch, or status writes;
- Jira issue/comment/status updates;
- generic runtime adapter execution;
- runtime side-effect proposal, attempt, or completion paths;
- side-effect workflow event kinds;
- side-effect audit projections or sinks;
- side-effect persistence or state backend changes;
- automatic WorkReport artifact behavior;
- WorkReport generation changes;
- EvidenceReference attachment for side effects;
- workflow schema fields;
- CLI commands or rendering;
- examples;
- domain packs;
- hosted or distributed runtime behavior;
- production SIEM, DLP, access control, OAuth, or webhook behavior;
- rollback or compensation behavior;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. Current Baseline

Implemented foundations:

- conservative policy denies unknown and unsupported writes;
- workflow event stream and run state are append-only and replayable;
- policy decisions are recorded before meaningful actions;
- approval requests and decisions exist;
- idempotency store contract exists;
- read-only adapter contracts and telemetry exist;
- EvidenceReference core model and selected attachments exist;
- WorkReport and WorkReport artifact foundations exist;
- local check result references and report citations exist;
- governed multi-step local execution exists.

Not implemented:

- domain-neutral side-effect record model;
- side-effect lifecycle model;
- side-effect authority model;
- side-effect event model;
- side-effect persistence;
- side-effect audit projection;
- side-effect WorkReport citation target;
- side-effect EvidenceReference attachment;
- write-capable adapter execution.

## 5. Required Model Concepts

Add the smallest idiomatic Rust model set needed for a future governed side-effect boundary.

Likely types:

- `SideEffectRecord`
- `SideEffectRecordDefinition`
- `SideEffectId`
- `SideEffectLifecycleState`
- `SideEffectTargetReference`
- `SideEffectCapability`
- `SideEffectAuthority`
- `SideEffectAuthorityDecision`
- `SideEffectIdempotencyBinding`
- `SideEffectOutcomeReference`
- `SideEffectReference`
- `SideEffectReferenceKind`
- `SideEffectSensitivity`

Only add types justified by ADR 0011 and this plan.

Prefer an implementation file such as `crates/workflow-core/src/side_effect.rs` and focused tests in `crates/workflow-core/tests/side_effect.rs`.

Export the model from `workflow-core` consistently with existing core models after validation and tests are in place.

## 6. Required Record Fields

A `SideEffectRecord` should capture:

- side-effect ID;
- lifecycle state;
- target reference;
- requested capability;
- authority context;
- actor or system actor;
- workflow ID;
- workflow version;
- schema version;
- spec hash;
- run ID;
- step ID if available;
- skill ID and version if available;
- adapter ID/kind if available;
- integration ID if available;
- idempotency binding;
- policy decision references;
- approval decision references;
- audit event references;
- workflow event references where available;
- adapter telemetry references where available;
- evidence references where available;
- outcome reference where available;
- created timestamp;
- updated timestamp or finalized timestamp where applicable;
- correlation ID where available;
- bounded summary or reason code list;
- sensitivity;
- redaction metadata.

If the first implementation needs to keep the model smaller, it may use optional fields for step, skill, adapter, integration, event, audit, evidence, and outcome references. It must not omit core identity, lifecycle, target, capability, authority, idempotency, sensitivity, or redaction boundaries.

## 7. Lifecycle State Rules

Support the minimal lifecycle vocabulary from ADR 0011:

- `proposed`;
- `attempted`;
- `completed`;
- `denied`;
- `skipped`;
- `failed`.

Do not add `approved` as a lifecycle state.

Do not add `rolled_back` or `compensated` in the first model. Those require adapter-specific proof and separate planning.

Validation should enforce lifecycle-specific requirements:

- `proposed` records require target, capability, authority context, actor/system actor, and idempotency binding.
- `attempted` records require authority that allowed attempt and an idempotency binding.
- `completed` records require attempted/completion outcome reference or explicit non-secret outcome metadata.
- `denied` records require denial authority context or stable denial reason codes.
- `skipped` records require stable skipped reason codes.
- `failed` records require attempted authority and stable non-leaking failure classification or outcome reference.

The implementation plan may choose whether `denied` can exist without a prior `proposed` record. Recommendation: allow `denied` as a first-class record when policy/capability/approval checks block a requested side effect before attempt, because denied side effects should be explicit rather than absent.

## 8. Authority Model Rules

`SideEffectAuthority` should represent why a side effect may or may not proceed. It must not be inferred from credentials.

Recommended authority decision vocabulary:

- `not_evaluated`;
- `allowed_by_policy`;
- `requires_approval`;
- `approved_by_human`;
- `denied_by_policy`;
- `denied_by_approval`;
- `denied_by_capability`;
- `denied_by_kill_switch`;
- `denied_by_validation`;
- `unsupported`.

Validation rules:

- attempted or completed records cannot use denied, unsupported, or not-evaluated authority.
- denied records must use denied or unsupported authority.
- sensitive or ambiguous capabilities should be representable as requiring approval without granting authority by default.
- authority must cite policy/approval references where available.
- authority errors must use stable codes and must not leak target values, actor values, paths, provider payloads, or secret-like metadata.

## 9. Target And Capability Rules

`SideEffectTargetReference` should be reference-first and domain-neutral.

Candidate target kinds:

- `external_resource`;
- `adapter_resource`;
- `workflow_resource`;
- `local_resource`;
- `provider_operation`;
- `unknown`.

Target references must be bounded and must reject secret-like values. They must not store raw provider payloads, raw file contents, raw request bodies, command output, environment values, tokens, or credentials.

`SideEffectCapability` should remain domain-neutral while allowing adapter-backed classification:

- `external_write`;
- `local_write`;
- `adapter_write`;
- `github_write`;
- `jira_write`;
- `ci_write`;
- `workflow_dispatch`;
- `ci_rerun`;
- `unknown`.

Unknown capabilities must fail closed for attempted/completed states. The model may represent unknown as vocabulary for invalid serialized payload testing, but constructors should reject unsafe use.

## 10. Idempotency Rules

`SideEffectIdempotencyBinding` should require:

- idempotency key;
- binding scope, such as run/step/adapter;
- optional prior side-effect ID for duplicate handling;
- optional non-secret outcome reference for duplicate detection.

Validation rules:

- every record must include an idempotency binding unless a future review approves a clear exception;
- attempted and completed records must include a binding before they can validate;
- duplicate handling must be representable without reattempting mutation;
- raw provider response payloads must not be stored in idempotency binding fields.

The first model phase should not implement idempotency-store behavior. It only models the binding.

## 11. Reference Rules

`SideEffectReference` should support stable references to existing model identifiers:

- workflow event IDs;
- audit event IDs;
- policy IDs or policy event IDs where available;
- approval reference IDs;
- adapter telemetry record references where stable IDs exist;
- EvidenceReference IDs;
- WorkReport IDs if already available;
- local check result references if later needed;
- typed handoff IDs if later needed.

Do not fabricate IDs for missing references.

If an identifier does not exist in current code, the model should use a bounded stable reference string only if it is validated, redaction-safe, and explicitly documented as a reference pointer rather than payload storage.

## 12. Validation Behavior

Validation must ensure:

1. side-effect ID is valid;
2. lifecycle state is valid;
3. target reference is valid and bounded;
4. requested capability is valid and not unsafe for the lifecycle state;
5. authority is present and compatible with lifecycle state;
6. actor or system actor is present;
7. workflow/run identity fields are valid;
8. schema version is valid;
9. spec hash is present and valid;
10. idempotency binding is present and valid;
11. reference lists are bounded;
12. duplicate references are rejected;
13. summaries/reason codes are bounded;
14. sensitivity is valid;
15. redaction metadata is present and safe;
16. attempted/completed states cannot be constructed with denied or unsupported authority;
17. denied/skipped/failed states carry stable non-leaking reason or failure references;
18. invalid serialized records fail closed.

Validation errors must use stable codes and must not include raw target values, paths, provider payloads, command output, snippets, token-like values, metadata values, or secret-like rejected strings.

## 13. Privacy And Redaction

The model must not store:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira issue/comment bodies;
- raw GitHub file contents;
- raw spec contents;
- raw parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded summaries;
- secret-like target identifiers;
- secret-like redaction metadata.

`Debug` output must be redaction-safe.

Serialization must not leak secret-like test values.

Deserialization errors must not leak rejected raw values.

## 14. Relationship To Existing Models

SideEffect should not replace:

- workflow events as run-state truth;
- audit events as governance projections;
- adapter telemetry as adapter invocation summary;
- EvidenceReference as evidence citation substrate;
- WorkReport as final handoff artifact;
- policy decisions as policy outcomes;
- approval decisions as human decisions.

SideEffect should cite these models by stable references.

The first implementation should avoid changing any existing model's behavior. Adding future WorkReport citation target vocabulary for side-effect records should be planned separately unless the implementation review decides it is necessary for model completeness.

## 15. Runtime And State Boundary

The first implementation must be model-only.

It must not:

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

## 16. Test Plan

Future implementation tests must cover:

1. valid minimal proposed side-effect record;
2. valid denied side-effect record;
3. valid skipped side-effect record;
4. valid attempted side-effect record with allowed authority;
5. valid completed side-effect record with outcome reference;
6. valid failed side-effect record with failure reference;
7. invalid side-effect ID rejected;
8. invalid target reference rejected without leaking value;
9. unknown/unsafe capability fails closed for attempted/completed states;
10. attempted/completed records reject denied authority;
11. denied records require denied/unsupported authority or stable denial reason;
12. missing actor/system actor rejected;
13. invalid workflow/run identity rejected;
14. missing spec hash rejected;
15. missing idempotency binding rejected;
16. duplicate references rejected;
17. bounded reason/summary validation;
18. redaction metadata validation;
19. sensitivity validation;
20. serde round trip for valid records;
21. invalid serialized record fails closed;
22. deserialization errors do not leak secret-like target or metadata values;
23. Debug output does not leak target values, summaries, redaction metadata, or secret-like values;
24. serialization does not include forbidden raw payload fields;
25. no runtime events, persistence, CLI, schemas, examples, or writes are introduced;
26. existing policy, approval, adapter, EvidenceReference, WorkReport, local check, and runtime tests still pass;
27. `cargo test --workspace` passes.

If compile-time privacy is relied on for internal fields, include behavioral tests proving accessor-only usage and redaction-safe output.

## 17. Documentation Updates For Future Implementation

Update:

- [ADR 0011](../adr/0011-side-effect-boundary.md);
- [Governed Work Pattern](../concepts/governed-work-pattern.md);
- [Evidence Reference](../concepts/evidence-reference.md);
- [WorkReportContract Planning Document](work-report-contract-plan.md);
- [Roadmap](../../ROADMAP.md).

Docs must say:

- SideEffect core model is implemented;
- writes are not implemented;
- write-capable adapters are not implemented;
- runtime side-effect execution is not implemented;
- side-effect persistence is not implemented;
- side-effect workflow events/audit projections are not implemented unless separately scoped;
- WorkReport side-effect citation vocabulary, terminal helper propagation, and executor report input propagation are implemented separately; automatic discovery and runtime side-effect execution remain unimplemented unless separately scoped;
- EvidenceReference side-effect attachment is not implemented;
- schemas are not updated;
- CLI behavior is not added;
- examples are not updated;
- release posture is unchanged.

## 18. End-Of-Phase Report

Future implementation should create:

- `docs/concepts/SIDE_EFFECT_CORE_MODEL_REPORT.md`

The report must include:

1. executive summary;
2. scope completed;
3. scope explicitly not completed;
4. model types added;
5. lifecycle and authority summary;
6. validation boundary summary;
7. reference and idempotency summary;
8. privacy/redaction summary;
9. test coverage summary;
10. commands run and results;
11. remaining known limitations;
12. recommended next phase.

Recommended next phase should be one of:

- SideEffect core model review;
- WorkReport side-effect citation planning;
- high-assurance approval controls planning;
- write-adapter readiness planning;
- blocker fix;
- defer.

## 19. Validation For Future Implementation

Run:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`.

Run broader checks if repository tooling requires it.

## 20. Acceptance Criteria

- SideEffect core model exists.
- Lifecycle state and authority are modeled separately.
- Proposed, attempted, completed, denied, skipped, and failed states are representable.
- Validation is deterministic and redaction-safe.
- Idempotency binding is modeled without implementing mutation retry behavior.
- Existing stable references can be cited without copying payloads.
- No writes, provider mutations, runtime side-effect execution, persistence, CLI, schemas, examples, hosted behavior, or release posture changes are introduced.
- End-of-phase report exists.

## 21. Final Recommendation

Recommended next phase: WorkReport side-effect citation target review.

The plan is implemented as documented in [SideEffect Core Model Report](../concepts/SIDE_EFFECT_CORE_MODEL_REPORT.md) and accepted in [SideEffect Core Model Review](../concepts/SIDE_EFFECT_CORE_MODEL_REVIEW.md). WorkReport side-effect citation vocabulary is implemented as documented in [WorkReport SideEffect Citation Report](../concepts/WORK_REPORT_SIDE_EFFECT_CITATION_REPORT.md), terminal report helper propagation for explicitly supplied SideEffect IDs is accepted in [Terminal Report SideEffect Citation Integration Review](../concepts/TERMINAL_REPORT_SIDE_EFFECT_CITATION_INTEGRATION_REVIEW.md), and executor SideEffect report input propagation is implemented in [Executor SideEffect Report Input Propagation Report](../concepts/EXECUTOR_SIDE_EFFECT_REPORT_INPUT_PROPAGATION_REPORT.md). Do not start write-capable adapter implementation, high-assurance approval control implementation, or runtime side-effect execution until the relevant side-effect event/audit, persistence, and approval semantics are separately scoped and reviewed.
