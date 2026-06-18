# WorkReport SideEffect Citation Plan

Status: Implemented and accepted as model-only WorkReport citation vocabulary; see [WorkReport SideEffect Citation Report](../concepts/WORK_REPORT_SIDE_EFFECT_CITATION_REPORT.md) and [WorkReport SideEffect Citation Review](../concepts/WORK_REPORT_SIDE_EFFECT_CITATION_REVIEW.md). The SideEffect core model is implemented and accepted, WorkReport can cite `SideEffectId` values, terminal report helper SideEffect citation propagation is implemented and accepted in [Terminal Report SideEffect Citation Integration Review](../concepts/TERMINAL_REPORT_SIDE_EFFECT_CITATION_INTEGRATION_REVIEW.md), and executor propagation is implemented in [Executor SideEffect Report Input Propagation Report](../concepts/EXECUTOR_SIDE_EFFECT_REPORT_INPUT_PROPAGATION_REPORT.md). This does not implement automatic SideEffect discovery, report artifact behavior changes, persistence, schemas, CLI behavior, examples, runtime side-effect execution, write-capable adapters, or release posture changes.

## 1. Executive Summary

The SideEffect core model now provides a domain-neutral record for future governed mutation intent and lifecycle state. WorkReport already has a required side-effects section, but it cannot yet cite `SideEffectRecord` values directly.

The implemented model-only WorkReport citation vocabulary lets reports point at side-effect records by stable ID without embedding side-effect payloads, inventing records, generating side effects, persisting side effects, executing writes, or changing report generation behavior.

This vocabulary phase added citation vocabulary only. A later accepted terminal helper phase now propagates explicitly supplied SideEffect IDs through `TerminalLocalWorkReportInput`; executor, artifact, persistence, and runtime side-effect paths still do not propagate or discover SideEffect IDs.

## 2. Goals

- Let future WorkReports cite `SideEffectRecord` values by stable reference.
- Keep SideEffect as the source of truth for side-effect intent and lifecycle state.
- Keep WorkReport as a governed handoff artifact.
- Avoid copying side-effect target references, summaries, reasons, outcomes, provider payloads, or raw write context into report sections.
- Preserve existing WorkReport validation, serde, redaction, and debug behavior.
- Preserve existing terminal report helper behavior.
- Preserve existing executor behavior.
- Prepare for later terminal helper and executor propagation of supplied SideEffect IDs.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- automatic WorkReport citation of side effects;
- terminal report helper integration;
- executor report input propagation;
- runtime side-effect execution;
- side-effect workflow events;
- side-effect audit projections;
- side-effect persistence;
- side-effect report artifact changes;
- EvidenceReference side-effect attachment;
- approval attachment;
- write-capable adapters;
- provider mutations;
- rollback or compensation behavior;
- workflow schema fields;
- CLI commands or rendering;
- example updates;
- hosted or distributed runtime behavior;
- domain packs;
- reasoning lineage implementation;
- release posture changes.

## 4. Current Baseline

Implemented:

- `SideEffectRecord`;
- `SideEffectId`;
- `SideEffectLifecycleState`;
- `SideEffectAuthority`;
- `SideEffectIdempotencyBinding`;
- `SideEffectReference`;
- `WorkReport`;
- `WorkReportCitation`;
- `WorkReportCitationKind`;
- `WorkReportCitationTarget`;
- required WorkReport side-effects section;
- in-memory terminal report generation helper;
- runtime result report exposure helper;
- executor-integrated report-bearing execution;
- explicit local report artifact store.

Not implemented in the model-only vocabulary phase:

- terminal report helper consumption of SideEffect IDs, which was implemented later as documented in [Terminal Report SideEffect Citation Integration Report](../concepts/TERMINAL_REPORT_SIDE_EFFECT_CITATION_INTEGRATION_REPORT.md);
- executor propagation of SideEffect IDs into reports;
- side-effect persistence;
- side-effect workflow events or audit projections;
- runtime side-effect execution.

## 5. Citation Target Decision

The implementation adds dedicated SideEffect citation vocabulary.

Recommended kind:

```rust
WorkReportCitationKind::SideEffect
```

Recommended target:

```rust
WorkReportCitationTarget::SideEffect {
    side_effect_id: SideEffectId,
}
```

Rationale:

- `SideEffectId` is the canonical stable identifier for side-effect records.
- A direct ID avoids a vague string reference while still avoiding payload copying.
- WorkReport already uses typed IDs for evidence references, validation references, typed handoffs, hook invocations, and approvals.
- The citation target can be serialized without embedding a `SideEffectRecord`.

If implementation discovers an import cycle or compatibility issue, it may use `WorkReportStableReference` only with a clear review note. The preferred path is a typed `SideEffectId`.

## 6. Source-Of-Truth Rules

- `SideEffectRecord` is the source of truth for side-effect intent, authority, lifecycle state, idempotency, outcome references, and related references.
- WorkReport cites side effects; it does not recreate, validate, summarize, or resolve them.
- Workflow events remain the source of truth for run state.
- Audit events remain governance and operational projections.
- EvidenceReference remains the evidence citation substrate.
- Adapter telemetry remains adapter invocation summary.

Absence of a SideEffect citation in a report must not be interpreted as proof that no side effect existed. The report side-effects section should continue to state none, skipped, denied, unsupported, or not available explicitly when no citation is supplied.

## 7. Model Shape

The first implementation should update only WorkReport model vocabulary:

- add `SideEffect` to `WorkReportCitationKind`;
- add `SideEffect { side_effect_id: SideEffectId }` to `WorkReportCitationTarget`;
- map the new target to the new citation kind;
- preserve existing `WorkReportCitation::new(...)` validation;
- preserve redaction-safe target `Debug`;
- preserve serde tag style;
- add focused WorkReport tests.

Do not add side-effect fields to `WorkReportSection`, `WorkReport`, `WorkReportGenerationContext`, report artifact metadata, or executor result types in this implementation.

## 8. Section Placement Policy

SideEffect citations belong in `WorkReportSectionKind::SideEffects`.

Future terminal helper integration may place supplied SideEffect IDs in the side-effects section. Until then, the existing side-effects section should continue to say that write side effects are none, skipped, or unsupported when no side-effect citation input exists.

Do not place SideEffect citations in:

- evidence considered, unless a future evidence phase explicitly bridges SideEffect to EvidenceReference;
- policy gates evaluated, unless a future report helper cites both policy decisions and SideEffect records separately;
- approvals, unless a future approval phase links approval decisions and SideEffect records explicitly;
- validation and quality checks.

## 9. Citation Construction Rules

Future call sites should construct citations by:

1. receiving an existing validated `SideEffectId`;
2. constructing `WorkReportCitationTarget::SideEffect { side_effect_id }`;
3. constructing `WorkReportCitation` through `WorkReportCitation::new(...)`;
4. using a bounded generic summary only when useful, such as "Side-effect record cited.";
5. never copying fields from the side-effect record by default.

Rules:

- Do not create `SideEffectRecord` values inside WorkReport citation construction.
- Do not fabricate SideEffect IDs.
- Do not resolve SideEffect records from stores.
- Do not copy target references.
- Do not copy outcome references.
- Do not copy reason codes or summaries by default.
- Do not copy provider payloads, command output, logs, specs, parser payloads, environment values, credentials, or token-like values.

## 10. Terminal Helper And Executor Boundary

The first implementation should be citation vocabulary only.

It should not change:

- `generate_terminal_local_work_report(...)`;
- `expose_terminal_local_work_report_result(...)`;
- `LocalExecutionReportInputs`;
- `LocalExecutor::execute_with_report(...)`;
- report artifact stores;
- CLI output.

After citation vocabulary is implemented and reviewed, a separate phase may plan terminal helper input propagation for explicitly supplied SideEffect IDs. That later phase should mirror the existing typed handoff, local check, hook invocation, and hook disclosure propagation pattern.

## 11. Privacy And Redaction

SideEffect citations must not copy:

- side-effect target references;
- side-effect summaries;
- side-effect reason codes;
- side-effect outcome references;
- side-effect redaction metadata;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira issue/comment bodies;
- raw GitHub file contents;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

`WorkReportCitationTarget` debug output should continue to redact the underlying reference value. Serialization may include the stable `SideEffectId`, just as other citation targets include stable IDs, but must not include any `SideEffectRecord` payload fields.

## 12. Validation And Serde Rules

Implementation should ensure:

- SideEffect citation target validates through typed `SideEffectId`;
- `citation_kind()` maps deterministically to `WorkReportCitationKind::SideEffect`;
- valid citations serialize and deserialize;
- invalid serialized SideEffect IDs fail closed;
- secret-like SideEffect IDs fail without leaking rejected values;
- `WorkReportCitation` redaction metadata validation still applies;
- `WorkReportCitation` summaries remain bounded and redaction-safe;
- existing WorkReport serde compatibility remains intact.

## 13. Test Plan

Future implementation tests should cover:

- SideEffect citation kind is representable;
- SideEffect citation target validates with `SideEffectId`;
- SideEffect citation target maps to `WorkReportCitationKind::SideEffect`;
- SideEffect citation target serializes and deserializes;
- invalid SideEffect ID fails closed;
- secret-like SideEffect ID fails without leaking value;
- `WorkReportCitation` debug output does not leak the SideEffect ID;
- serialization does not include SideEffect record payload fields;
- WorkReport side-effects section can hold a SideEffect citation when manually constructed;
- existing WorkReport tests still pass;
- existing SideEffect tests still pass;
- no terminal helper behavior changes;
- no executor behavior changes;
- no report artifact behavior changes;
- no persistence, CLI, schemas, examples, runtime side-effect execution, adapter writes, or release posture changes.

## 14. Documentation Updates For Future Implementation

Update:

- [WorkReportContract Planning Document](work-report-contract-plan.md);
- [Governed Work Pattern](../concepts/governed-work-pattern.md);
- [Evidence Reference](../concepts/evidence-reference.md);
- [SideEffect Core Model Report](../concepts/SIDE_EFFECT_CORE_MODEL_REPORT.md);
- [Roadmap](../../ROADMAP.md).

Docs must say:

- WorkReport side-effect citation vocabulary is implemented;
- terminal helper SideEffect ID propagation is implemented for explicit supplied IDs;
- executor SideEffect ID propagation is implemented for explicit supplied IDs;
- runtime side-effect execution is not implemented;
- side-effect persistence is not implemented;
- side-effect workflow events and audit projections are not implemented;
- EvidenceReference side-effect attachment is not implemented;
- writes and write-capable adapters remain unsupported;
- schemas, CLI behavior, examples, hosted behavior, and release posture are unchanged.

## 15. Proposed Implementation Sequence

1. Add `SideEffect` citation kind and target vocabulary to `WorkReport`.
2. Add focused citation target tests.
3. Update docs and create an implementation report.
4. Run validation.
5. Review the citation vocabulary.
6. Plan terminal helper SideEffect ID propagation separately.

## 16. Deferred Work

Deferred:

- terminal report helper SideEffect ID inputs;
- executor report input propagation;
- report artifact behavior changes;
- automatic discovery from workflow events, audit events, stores, adapter telemetry, or side-effect persistence;
- side-effect workflow events;
- side-effect audit projections;
- side-effect persistence;
- EvidenceReference side-effect attachment;
- approval-side-effect linkage;
- runtime side-effect execution;
- write-capable adapters;
- schemas;
- CLI;
- examples;
- reasoning lineage;
- hosted behavior.

## 17. Acceptance Criteria

- WorkReport can cite a SideEffect record by stable `SideEffectId`.
- WorkReport does not copy SideEffect payloads.
- Citation validation is deterministic and redaction-safe.
- Existing WorkReport and SideEffect tests pass.
- Terminal helper, executor, persistence, CLI, schema, examples, runtime side-effect execution, and writes remain unchanged.
- End-of-phase report exists.

## 18. Validation

Run:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`.

Run broader checks if repository tooling requires it.

## 19. Final Recommendation

Recommended next phase: executor SideEffect report input propagation review.

Terminal report helper propagation is implemented and accepted in [Terminal Report SideEffect Citation Integration Review](../concepts/TERMINAL_REPORT_SIDE_EFFECT_CITATION_INTEGRATION_REVIEW.md), and executor propagation is implemented in [Executor SideEffect Report Input Propagation Report](../concepts/EXECUTOR_SIDE_EFFECT_REPORT_INPUT_PROPAGATION_REPORT.md). Do not implement report artifact behavior changes, side-effect persistence, runtime side-effect execution, writes, schemas, CLI behavior, examples, hosted behavior, or release posture changes until separately scoped and reviewed.
