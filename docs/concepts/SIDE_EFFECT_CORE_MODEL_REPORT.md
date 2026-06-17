# SideEffect Core Model Report

Report date: 2026-06-17

## 1. Executive Summary

The SideEffect core model is implemented in `workflow-core` as a model-only governance boundary for future write-capable behavior.

The implementation adds validated, domain-neutral Rust types for side-effect identity, lifecycle state, target reference, requested capability, authority context, idempotency binding, stable references, outcome references, sensitivity, and redaction metadata. The model is reference-first and does not execute writes.

## 2. Scope Completed

- Added `SideEffectRecord` and `SideEffectRecordDefinition`.
- Added `SideEffectId`.
- Added lifecycle vocabulary for proposed, attempted, completed, denied, skipped, and failed side-effect records.
- Added target reference and capability vocabulary.
- Added authority decision and authority context types.
- Added idempotency binding model.
- Added outcome and related-record reference models.
- Added sensitivity classification.
- Added deterministic validation.
- Added serde support with invalid serialized records failing closed.
- Added redaction-safe `Debug` behavior.
- Added focused tests.
- Exported the model from `workflow-core`.

## 3. Scope Explicitly Not Completed

This phase does not implement:

- writes;
- provider mutations;
- write-capable adapters;
- generic runtime adapter execution;
- runtime side-effect proposal, attempt, completion, denial, skip, or failure paths;
- side-effect workflow event kinds;
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

## 4. Model Types Added

The implementation adds:

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

## 5. Lifecycle And Authority Summary

Lifecycle state and authority remain separate.

`Attempted`, `completed`, and `failed` records require authority that allows attempt. `Denied` records require denied or unsupported authority. `Completed` records require an outcome reference. `Denied` and `skipped` records require stable reason codes. `Failed` records require a failure outcome reference or stable reason code.

Unknown capability vocabulary is representable, but it fails closed for attempted, completed, and failed records. Denied records may capture an unknown capability when the authority decision blocks it before any attempt.

## 6. Validation Boundary Summary

Validation enforces:

- valid side-effect ID;
- valid target reference;
- valid lifecycle/authority compatibility;
- valid lifecycle/capability compatibility;
- required actor or system actor;
- required workflow/run identity fields;
- required idempotency binding;
- bounded references;
- duplicate reference rejection;
- bounded summaries and reason codes;
- lifecycle-specific outcome or reason requirements;
- bounded and non-secret redaction metadata;
- invalid serde failure.

Validation errors use stable `side_effect.*` codes and avoid including rejected target values, summaries, reason values, redaction metadata values, paths, snippets, provider payloads, command output, credentials, or token-like values.

## 7. Reference And Idempotency Summary

SideEffect references are stable pointers only. They may refer to workflow events, audit events, policy decisions, approval decisions, adapter telemetry, EvidenceReference IDs, WorkReport IDs, local check results, or typed handoffs.

The idempotency binding records the idempotency key, binding scope, optional prior side-effect ID, and optional outcome reference. It does not implement idempotency-store behavior, retry behavior, mutation deduplication, or adapter execution.

## 8. Privacy And Redaction Summary

The model rejects secret-like target references, summaries, reason codes, references, and redaction metadata. It does not store raw provider payloads, command output, CI logs, Jira bodies/comments, GitHub file contents, raw spec contents, parser payloads, environment variable values, credentials, authorization headers, private keys, token-like values, or unbounded text.

`Debug` output redacts IDs, target references, workflow/run identity, summaries, references, correlation IDs, and redaction metadata details. Serialization of valid records remains bounded and does not include forbidden raw payload fields.

## 9. Test Coverage Summary

Focused tests cover:

- valid proposed side-effect record;
- all required lifecycle states;
- completed outcome requirement;
- attempted denied-authority rejection;
- denied authority requirements;
- denied/skipped reason requirements;
- unknown capability fail-closed behavior for attempted/completed/failed;
- denied unknown-capability recording before attempt;
- invalid side-effect ID rejection;
- secret-like target rejection without leakage;
- duplicate reference rejection;
- actor/system actor requirement;
- system actor-only validity;
- secret-like summary, reason code, and redaction metadata rejection;
- serde round trip;
- invalid serialized secret-like target failure without leaking the value;
- redaction-safe Debug behavior;
- forbidden raw payload marker non-leakage in serialization;
- valid redaction metadata;
- idempotency binding with prior side-effect reference.

## 10. Commands Run And Results

- `cargo fmt --all` - passed.
- `cargo test -p workflow-core --test side_effect` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 11. Remaining Known Limitations

- No runtime side-effect execution exists.
- No write-capable adapters exist.
- No side-effect workflow events or audit projections exist.
- No side-effect persistence exists.
- WorkReport side-effect citation vocabulary is implemented as model-only vocabulary in [WorkReport SideEffect Citation Report](WORK_REPORT_SIDE_EFFECT_CITATION_REPORT.md).
- No EvidenceReference side-effect attachment is implemented.
- No schemas, CLI behavior, or examples are updated for side effects.
- Rollback and compensation semantics remain deferred.

## 12. Recommended Next Phase

Recommended next phase: terminal report SideEffect citation propagation planning.

The model is accepted in [SideEffect Core Model Review](SIDE_EFFECT_CORE_MODEL_REVIEW.md), and WorkReport side-effect citation vocabulary is accepted in [WorkReport SideEffect Citation Review](WORK_REPORT_SIDE_EFFECT_CITATION_REVIEW.md). Plan terminal helper propagation for explicitly supplied SideEffect IDs before executor propagation, high-assurance approval control implementation, write-adapter readiness implementation, or runtime side-effect execution planning.
