# WorkReport Hook Disclosure Citation Plan

Status: Planning complete; model-only WorkReport hook disclosure citation target vocabulary is implemented. Hook disclosure core model implementation is accepted with non-blocking follow-ups. This plan defines how WorkReports should cite bounded hook disclosures without copying hook context, broadening runtime hook statuses, or creating new persistence/schema/CLI behavior. It does not implement terminal report helper inputs, executor inputs, warning continuation, skipped-with-disclosure continuation, blocked behavior, hook optionality, policy-controlled continuation, event append behavior, audit sink emission, persistence, report artifacts, CLI rendering, schemas, side effects, writes, reasoning lineage, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 1. Executive Summary

Workflow OS can now model bounded hook disclosures with stable disclosure IDs, kind, severity, title, summary, references, redaction metadata, and sensitivity.

WorkReports already support citations to `AgentHarnessHookInvocationId` values. That is sufficient to say a governed hook checkpoint was considered. It is not sufficient to cite a specific warning, skipped-hook note, policy note, validation note, or operator note once those disclosures become meaningful report inputs.

The next question is how WorkReports should cite hook disclosures while keeping reports reference-first and avoiding raw hook context. This plan does not implement citation behavior.

## 2. Goals

- Decide whether hook disclosures should become a WorkReport citation target.
- Preserve WorkReport as a governed handoff artifact, not an audit log.
- Preserve hook invocation IDs as the current stable checkpoint citation.
- Allow future reports to cite specific disclosure IDs when supplied explicitly.
- Avoid copying hook invocation context, hook audit records, raw hook references, provider payloads, command output, parser payloads, file contents, paths, tokens, or unbounded prose.
- Preserve current workflow pass/fail semantics.
- Preserve current hook runtime semantics.
- Prepare future warning/skipped report disclosure behavior without enabling warning/skipped continuation.
- Keep the first future implementation small, explicit, in-memory, and reviewable.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- warning continuation;
- skipped-with-disclosure continuation;
- blocked runtime behavior;
- hook optionality;
- policy-controlled continuation;
- automatic executor hook invocation;
- workflow-declared hook configuration;
- runtime hook configuration;
- automatic hook disclosure discovery;
- hook disclosure creation from reports;
- hook invocation result creation from reports;
- hook audit record creation from reports;
- hook audit record persistence;
- workflow event append behavior;
- audit sink emission;
- report artifact behavior changes;
- CLI hook commands or report rendering;
- workflow schema fields;
- automatic local check execution;
- command execution;
- adapter invocation;
- `EvidenceReference` creation or attachment;
- approval request or approval decision creation;
- approval evidence attachment;
- reasoning lineage;
- side-effect boundary implementation;
- writes;
- recursive agents or agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Current Baseline

Implemented:

- `AgentHarnessHookDisclosureId`;
- `AgentHarnessHookDisclosure`;
- `AgentHarnessHookDisclosureKind`;
- `AgentHarnessHookDisclosureSeverity`;
- `AgentHarnessHookDisclosureReference`;
- hook disclosure validation, serde, and redaction-safe `Debug`;
- `AgentHarnessHookInvocationId`;
- `AgentHarnessHookInvocationResult`;
- `AgentHarnessHookAuditRecord`;
- WorkReport citation target for `AgentHarnessHookInvocationId`;
- WorkReport citation target vocabulary for `AgentHarnessHookDisclosureId`;
- terminal report helper support for explicitly supplied hook invocation IDs;
- executor report input propagation for explicitly supplied hook invocation IDs;
- explicit `BeforeSkillInvocation` hook event append behavior for supported `Passed` and `FailedClosed` paths.

Not implemented:

- terminal report helper support for supplied hook disclosure IDs;
- executor report input propagation for hook disclosure IDs;
- direct WorkReport section population from hook disclosure kind/severity;
- warning/skipped/blocked continuation;
- hook optionality;
- hook audit persistence;
- automatic hook disclosure discovery.

## 5. Source-Of-Truth Boundaries

- `WorkflowRunEvent` remains the source of truth for runtime state.
- `AgentHarnessHookInvocationId` identifies a governed hook checkpoint.
- `AgentHarnessHookDisclosureId` identifies a bounded disclosure value inside hook context.
- `AgentHarnessHookAuditRecord` remains model-only until persistence/event semantics are separately accepted.
- `AuditEvent` remains an audit projection, not an alias for hook disclosure records.
- `WorkReport` cites governed references and may include bounded summaries; it must not copy hook records.
- `EvidenceReference` remains an evidence citation substrate, not the hook disclosure itself.

## 6. Citation Target Recommendation

Recommended first future implementation: add a dedicated WorkReport citation vocabulary entry for hook disclosures.

Candidate kind:

```rust
WorkReportCitationKind::AgentHarnessHookDisclosure
```

Candidate target:

```rust
WorkReportCitationTarget::AgentHarnessHookDisclosure {
    disclosure_id: AgentHarnessHookDisclosureId,
}
```

Rationale:

- Disclosure IDs now exist and are validated.
- A disclosure citation should not require embedding the full `AgentHarnessHookDisclosure`.
- Hook disclosure IDs are more precise than hook invocation IDs when a report needs to point at a specific warning, skipped disclosure, policy note, validation note, or operator note.
- It avoids treating hook disclosures as audit events before audit sink/event semantics are settled.
- It avoids treating hook disclosures as evidence before evidence attachment is designed.

## 7. Why Not Reuse `AgentHarnessHook`

`WorkReportCitationTarget::AgentHarnessHook` should continue to cite the hook checkpoint.

It should not be overloaded to mean "specific disclosure inside that hook" because:

- checkpoint identity and disclosure identity are different;
- multiple disclosures may exist for one hook invocation;
- skipped/warning report behavior may need disclosure-specific kind/severity;
- report readers need to distinguish "hook checkpoint considered" from "specific disclosure considered."

## 8. Why Not Embed `AgentHarnessHookDisclosure`

WorkReports should not embed full hook disclosure values in the citation target.

Reasons:

- reports should cite stable references, not copy model payloads;
- disclosure title and summary may be bounded but still sensitive;
- embedded disclosure payloads would couple report schema shape to hook disclosure model shape;
- future event/audit persistence semantics are not settled;
- disclosure summaries should be section content only when explicitly bounded by report generation policy.

## 9. Why Not Use Audit Or Workflow Event Citations Yet

Hook disclosure event semantics are not implemented.

Using `AuditEvent` or `WorkflowEvent` citations for hook disclosures would overclaim runtime history unless a durable event or audit record exists. A future phase may map disclosure IDs into workflow events or audit records, but that requires separate planning for ordering, idempotency, replay, projection, persistence, and terminal-state behavior.

## 10. Terminal Report Helper Policy

Recommended first helper integration, after citation target vocabulary review:

- accept explicitly supplied `AgentHarnessHookDisclosureId` values;
- do not accept full `AgentHarnessHookDisclosure` values;
- do not accept raw string IDs;
- do not discover disclosures from hook records;
- do not create hook disclosures;
- do not create hook invocations;
- do not create hook audit records;
- do not validate existence of disclosure IDs against persistence;
- cite supplied disclosure IDs through `WorkReportCitation::new(...)`.

This mirrors the existing pattern for supplied hook invocation IDs.

## 11. Section Placement Policy

Recommended first placement: `ValidationAndQualityChecks`.

Rationale:

- the helper input would initially carry disclosure IDs only;
- it would not carry hook kind, hook status, disclosure kind, or disclosure severity;
- `ValidationAndQualityChecks` already holds hook invocation checkpoint citations;
- placing disclosure ID citations there avoids implying a skipped hook, policy decision, approval, or incomplete work state without the necessary context.

Future context-aware routing may place disclosures differently:

| Disclosure context | Possible section | Deferred reason |
| --- | --- | --- |
| `Warning` | `Risks` or `ValidationAndQualityChecks` | Requires disclosure kind/severity context. |
| `Skipped` | `IncompleteOrDeferredWork` | Requires hook optionality and skipped semantics. |
| `PolicyNote` | `PolicyGatesEvaluated` or `DecisionsMade` | Requires policy decision linkage. |
| `ValidationNote` | `ValidationAndQualityChecks` | Safe first candidate. |
| `OperatorNote` | `OperatorHandoffNotes` | Requires bounded handoff policy. |

## 12. Summary Policy

First implementation should use generic bounded citation summary text such as:

```text
Agent harness hook disclosure reference considered.
```

Rules:

- do not copy `AgentHarnessHookDisclosure.title` into citation summary by default;
- do not copy `AgentHarnessHookDisclosure.summary` into citation summary by default;
- do not copy disclosure references, redaction metadata, sensitivity, workflow IDs, run IDs, actor IDs, hook inputs, hook outputs, or supplemental references;
- do not copy raw provider payloads, raw command output, raw CI logs, raw Jira/GitHub bodies, raw file contents, raw spec contents, raw parser payloads, environment values, credentials, authorization headers, private keys, tokens, or token-like values.

If future report generation wants disclosure summaries, it should require an explicit bounded-summary policy and a separate review.

## 13. Error Handling

Invalid disclosure IDs should fail at `AgentHarnessHookDisclosureId::new(...)` before report generation when callers construct typed IDs.

If citation construction fails inside report generation:

- return structured non-leaking report-generation errors;
- do not convert citation failures into workflow diagnostics;
- do not mutate workflow runs;
- do not append workflow events;
- do not emit audit or observability events;
- do not persist reports or hook records;
- do not change workflow pass/fail results;
- do not fabricate missing disclosure IDs;
- do not leak rejected IDs, paths, notes, summaries, tokens, payloads, command output, provider output, parser output, or secret-like values.

## 14. Privacy And Redaction

Disclosure citation behavior must inherit WorkReport citation privacy rules.

Required posture:

- use `WorkReportCitation::new(...)`;
- use typed `AgentHarnessHookDisclosureId` values;
- validate redaction metadata through existing report-safe boundaries;
- keep `Debug` redaction-safe;
- keep serde fail-closed;
- treat reports as sensitive even when citations are reference-only;
- serialize stable disclosure IDs only if the future implementation explicitly adds that target;
- never serialize full disclosure payloads inside WorkReport citations.

## 15. Relationship To Warning And Skipped Statuses

Disclosure citation planning does not enable warning or skipped continuation.

Future behavior remains:

- `Warning` may not continue until valid disclosure, policy allow decision, event semantics, and report behavior are accepted.
- `SkippedWithDisclosure` may not continue until valid disclosure, hook optionality, policy allow decision, event semantics, and report behavior are accepted.
- `Blocked` remains deferred until blocked/escalation semantics are accepted.

The citation model should make those future statuses more auditable, not easier to bypass.

## 16. Relationship To Event And Audit Semantics

The first citation implementation can cite supplied disclosure IDs without event/audit persistence.

However, before automatic report population or workflow-declared report requirements, Workflow OS should decide:

- whether disclosure IDs are durable event fields;
- whether hook audit records are persisted;
- whether disclosure IDs are replay-stable;
- whether duplicate run replay preserves disclosure identity;
- whether disclosure citations should require corresponding hook invocation citations;
- whether audit projections should include disclosure counts, IDs, or bounded snapshots.

## 17. Test Plan For Future Implementation

Future implementation tests should cover:

- `WorkReportCitationKind::AgentHarnessHookDisclosure` is representable;
- `WorkReportCitationTarget::AgentHarnessHookDisclosure` validates with a safe `AgentHarnessHookDisclosureId`;
- citation kind mapping returns `AgentHarnessHookDisclosure`;
- serde round trip for hook disclosure citation target;
- invalid disclosure ID fails closed;
- secret-like disclosure ID fails without leaking values;
- WorkReport citation debug output does not leak disclosure IDs or summaries;
- WorkReport serialization does not copy hook disclosure title, summary, references, redaction metadata, or hook context;
- terminal report helper accepts explicitly supplied disclosure IDs if helper integration is in scope;
- generated report cites supplied disclosure IDs without creating hook disclosures;
- absence of disclosure IDs preserves existing section text behavior;
- existing hook invocation citations remain unchanged;
- existing WorkReport, hook, EvidenceReference, Diagnostic, validation, adapter, local-check, and runtime tests still pass;
- no runtime hook execution, event append, audit sink emission, persistence, CLI, schema, side-effect, or write behavior is introduced.

## 18. Proposed Implementation Sequence

Recommended small phases:

1. Add WorkReport hook disclosure citation target vocabulary only.
2. Add focused WorkReport citation target tests.
3. Update docs and implementation report.
4. Review the citation target.
5. Plan terminal report helper support for explicitly supplied disclosure IDs.
6. Review helper integration.
7. Only after separate planning, consider context-aware section routing.
8. Only after event/audit planning, consider automatic disclosure discovery.
9. Only after report behavior is accepted, consider policy-controlled warning continuation.
10. Only after optionality behavior is accepted, consider skipped-with-disclosure continuation.

## 19. Open Questions

- Should disclosure citation target vocabulary be named `AgentHarnessHookDisclosure` or `HookDisclosure`?
- Should a disclosure citation require the parent `AgentHarnessHookInvocationId` alongside the disclosure ID?
- Should disclosure IDs be unique globally or only within a hook invocation?
- Should terminal report helper inputs accept disclosure IDs before durable event/audit semantics exist?
- Should WorkReport contracts ever require hook disclosure citations?
- Should skipped disclosures map to incomplete/deferred work by default?
- Should warning disclosures map to risks by default?
- Should bounded disclosure summaries ever be copied into report sections, or should report sections remain generic until event/audit persistence exists?
- Should disclosure citation planning happen before or after hook optionality planning?

## 20. Final Recommendation

Recommended next phase: **WorkReport hook disclosure citation target review**.

The model-only citation vocabulary phase added only a citation kind/target for `AgentHarnessHookDisclosureId`, focused validation/serde/redaction tests, docs, and an implementation report. It did not implement terminal helper integration, executor input propagation, warning continuation, skipped-with-disclosure continuation, blocked behavior, hook optionality, policy-controlled continuation, event append behavior, audit sink emission, persistence, report artifacts, CLI rendering, schemas, side effects, writes, reasoning lineage, recursive agents, agent swarms, hosted behavior, or release posture changes.
