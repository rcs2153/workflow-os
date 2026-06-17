# Terminal Report Hook Disclosure Citation Integration Plan

Status: Implemented for the in-memory terminal local report helper only. WorkReport hook disclosure citation target vocabulary is implemented and reviewed. This plan defines the narrow helper-level integration for explicitly supplied hook disclosure IDs. It does not implement executor propagation, runtime hook behavior, automatic disclosure discovery, warning/skipped continuation, blocked behavior, hook optionality, event append behavior, audit sink emission, persistence, report artifacts, CLI rendering, schemas, side effects, writes, reasoning lineage, recursive agents, agent swarms, hosted behavior, or release posture changes.

## 1. Executive Summary

`AgentHarnessHookDisclosureId` now identifies bounded hook disclosures.

`WorkReportCitationTarget::AgentHarnessHookDisclosure` now allows a WorkReport citation to point at a specific hook disclosure ID without embedding disclosure title, summary, references, redaction metadata, hook context, hook audit records, workflow events, evidence references, provider payloads, command output, parser payloads, or raw file contents.

The next question is how the in-memory terminal local WorkReport helper should accept explicitly supplied hook disclosure IDs and include them in generated reports.

That behavior is now implemented for the in-memory terminal local report helper only. Executor propagation and automatic discovery remain deferred.

## 2. Goals

- Let terminal local report generation cite explicitly supplied `AgentHarnessHookDisclosureId` values.
- Preserve existing terminal report helper behavior for reports without disclosure IDs.
- Preserve existing hook invocation citation behavior.
- Preserve workflow pass/fail semantics.
- Preserve report-generation error separation from workflow execution errors.
- Keep citations reference-first and bounded.
- Avoid copying hook disclosure title or summary into report sections by default.
- Avoid copying hook disclosure references, hook context, audit records, event payloads, provider payloads, command output, parser output, file contents, paths, tokens, or unbounded prose.
- Keep implementation small, explicit, in-memory, and testable.
- Prepare later executor input propagation without implementing it in the first helper slice.

## 3. Non-Goals

This plan did not authorize:

- executor report input propagation for hook disclosure IDs;
- automatic runtime report generation;
- automatic hook disclosure discovery;
- hook disclosure creation from reports;
- hook invocation result creation from reports;
- hook audit record creation from reports;
- hook audit record persistence;
- workflow event append behavior;
- audit sink emission;
- warning continuation;
- skipped-with-disclosure continuation;
- blocked runtime behavior;
- hook optionality;
- policy-controlled continuation;
- context-aware disclosure section routing by disclosure kind or severity;
- `EvidenceReference` creation or attachment;
- approval request or approval decision creation;
- approval evidence attachment;
- report artifact behavior changes;
- CLI hook commands or report rendering;
- workflow schema fields;
- automatic local check execution;
- command execution;
- adapter invocation;
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
- `WorkReportCitationKind::AgentHarnessHookDisclosure`;
- `WorkReportCitationTarget::AgentHarnessHookDisclosure`;
- terminal report helper support for explicitly supplied `AgentHarnessHookInvocationId` values;
- terminal report helper support for explicitly supplied `AgentHarnessHookDisclosureId` values;
- executor report input propagation for explicitly supplied hook invocation IDs.

Not implemented:

- executor report input propagation for hook disclosure IDs;
- section population from hook disclosure kind or severity;
- automatic hook disclosure discovery;
- warning/skipped/blocked continuation;
- hook optionality;
- hook audit persistence.

## 5. Helper Input Change

The implementation adds one field to `TerminalLocalWorkReportInput`:

```rust
pub agent_harness_hook_disclosure_ids: Vec<AgentHarnessHookDisclosureId>,
```

Rules:

- accept typed `AgentHarnessHookDisclosureId` values only;
- do not accept raw strings;
- do not accept full `AgentHarnessHookDisclosure` values;
- do not accept disclosure kind, severity, title, summary, references, redaction metadata, or sensitivity;
- do not discover disclosures from hook invocation results, audit records, workflow events, or persistence;
- do not require supplied disclosure IDs to have corresponding supplied hook invocation IDs in the first helper slice;
- do not fabricate missing disclosure IDs.

The helper already borrows an already-terminal run and does not mutate state, append events, write files, persist reports, or emit CLI output. That behavior must remain unchanged.

## 6. Citation Construction Policy

The implementation adds a bounded citation builder similar to `agent_harness_hook_citations(...)`:

```rust
fn agent_harness_hook_disclosure_citations(
    disclosure_ids: Vec<AgentHarnessHookDisclosureId>,
    sensitivity: WorkReportSensitivity,
    redaction: &RedactionMetadata,
) -> Result<Vec<WorkReportCitation>, WorkflowOsError>
```

Each citation should use:

```rust
WorkReportCitationTarget::AgentHarnessHookDisclosure { disclosure_id }
```

Summary text should be generic and bounded:

```text
Agent harness hook disclosure reference considered.
```

Rules:

- use `WorkReportCitation::new(...)`;
- cite stable disclosure IDs only;
- do not create `AgentHarnessHookDisclosure` values;
- do not recreate `EvidenceReference` values;
- do not create hook invocation results;
- do not create hook audit records;
- do not copy disclosure title or summary;
- do not copy disclosure references, redaction metadata, sensitivity, hook input/output references, supplemental references, workflow IDs, run IDs, actor IDs, provider payloads, command output, parser output, file contents, environment values, credentials, authorization headers, private keys, tokens, or token-like values;
- return structured non-leaking report-generation errors if citation construction fails.

## 7. Section Placement Policy

The first implementation places hook disclosure citations in `ValidationAndQualityChecks`.

Rationale:

- helper inputs carry disclosure IDs only;
- the helper does not know disclosure kind, severity, hook status, hook optionality, policy linkage, or skipped/warning context;
- `ValidationAndQualityChecks` already carries validation diagnostics, local check results, and hook invocation checkpoint citations;
- this avoids implying a skipped hook, policy decision, approval decision, incomplete work item, risk, or operator note without context.

Future context-aware routing remains deferred:

| Future disclosure context | Possible section | Deferred reason |
| --- | --- | --- |
| `Warning` | `Risks` or `ValidationAndQualityChecks` | Requires disclosure kind and severity context. |
| `Skipped` | `IncompleteOrDeferredWork` | Requires hook optionality and skipped continuation semantics. |
| `PolicyNote` | `PolicyGatesEvaluated` or `DecisionsMade` | Requires policy decision linkage. |
| `ValidationNote` | `ValidationAndQualityChecks` | Safe first candidate. |
| `OperatorNote` | `OperatorHandoffNotes` | Requires bounded handoff-note policy. |

## 8. Section Summary Policy

The existing validation/quality section summary should remain explicit and conservative.

Recommended behavior:

- if validation, local check, hook invocation, or hook disclosure citations exist, use existing "references were supplied" style text;
- if none exist, preserve existing not-available text;
- do not mention a warning, skipped hook, risk, policy decision, approval, failure, or operator note unless the helper has accepted context that proves it;
- do not copy disclosure title or summary into section text.

The implementation updates the internal `validation_summary(...)` helper to account for disclosure citations as another reference category without copying disclosure payloads.

## 9. Error Handling

Typed `AgentHarnessHookDisclosureId` construction should reject invalid or secret-like values before callers build helper input.

If helper-level citation construction fails:

- return the existing structured report-generation error path;
- do not convert citation failures into workflow diagnostics;
- do not mutate `WorkflowRun`;
- do not mutate `WorkflowRunSnapshot`;
- do not append workflow events;
- do not emit audit events;
- do not emit observability events;
- do not persist reports;
- do not create report artifacts;
- do not change workflow pass/fail result;
- do not fabricate missing disclosure IDs;
- do not leak rejected IDs, summaries, notes, paths, tokens, payloads, command output, provider output, parser output, or secret-like values.

## 10. Runtime And Workflow Semantics

Helper integration remains local and in-memory.

It must not:

- call `LocalExecutor`;
- change `LocalExecutor::execute(...)`;
- change `LocalExecutor::execute_with_report(...)`;
- add executor report input fields;
- append post-terminal workflow events;
- create hook events;
- create hook audit records;
- write to `StateBackend`;
- create filesystem artifacts;
- emit CLI output;
- change terminal status mapping;
- change report-generation failure semantics.

Executor propagation should be planned separately after helper integration is reviewed.

## 11. Privacy And Redaction

The helper implementation preserves WorkReport privacy rules.

Required posture:

- use typed `AgentHarnessHookDisclosureId` values;
- use `WorkReportCitation::new(...)`;
- validate redaction metadata through existing report-safe boundaries;
- keep `Debug` output redaction-safe;
- keep serde fail-closed;
- treat reports as sensitive even when citations are reference-only;
- serialize stable disclosure IDs only;
- never serialize full disclosure payloads inside WorkReport citations.

The helper must not store or copy:

- hook disclosure title or summary;
- disclosure references;
- hook input or output references;
- supplemental hook references;
- hook audit records;
- workflow event payloads;
- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira or GitHub bodies;
- raw file contents;
- raw spec contents;
- raw parser payloads;
- environment values;
- credentials, authorization headers, private keys, tokens, or token-like values.

## 12. Relationship To Executor Propagation

This helper integration should precede executor propagation.

Future executor propagation should require a separate plan and review because it changes `LocalExecutionReportInputs`, `LocalExecutionWithReportRequest`, and report-bearing execution tests.

The helper phase should not add those executor fields yet.

## 13. Relationship To Event And Audit Semantics

This phase does not require durable event or audit semantics for disclosures.

Supplied disclosure IDs may be cited as stable references without validating their existence against persistence. That matches the existing explicit-reference pattern used for other report citation inputs.

Before automatic discovery or workflow-declared report requirements, Workflow OS should separately decide:

- whether disclosure IDs are durable event fields;
- whether hook audit records are persisted;
- whether disclosure IDs are replay-stable;
- whether duplicate run replay preserves disclosure identity;
- whether disclosure citations should require corresponding hook invocation citations;
- whether audit projections should include disclosure counts, IDs, or bounded snapshots.

## 14. Test Plan

Implementation tests cover:

- terminal report helper accepts explicitly supplied `AgentHarnessHookDisclosureId` values;
- generated report cites disclosure IDs through `WorkReportCitationTarget::AgentHarnessHookDisclosure`;
- generated disclosure citations have `WorkReportCitationKind::AgentHarnessHookDisclosure`;
- disclosure citations are placed in `ValidationAndQualityChecks`;
- existing hook invocation citations remain unchanged;
- reports with no disclosure IDs preserve existing section text;
- helper does not create `AgentHarnessHookDisclosure` values;
- helper does not create hook invocation results;
- helper does not create hook audit records;
- helper does not recreate `EvidenceReference` values;
- invalid or secret-like disclosure IDs fail before or during report generation without leaking values;
- disclosure citation summaries do not copy disclosure title or summary;
- serialization does not copy disclosure title, summary, references, hook context, provider payloads, command output, parser payloads, spec contents, or secret-like markers;
- `Debug` output does not leak disclosure IDs or payload-like text;
- helper does not mutate workflow runs, snapshots, or event history;
- helper does not append workflow events;
- helper does not write state, persist reports, create artifacts, or emit CLI output;
- existing WorkReport, hook disclosure, hook invocation, EvidenceReference, Diagnostic, validation, adapter, local-check, executor, and runtime tests still pass.

## 15. Implementation Sequence

Completed small phase:

1. Add `agent_harness_hook_disclosure_ids` to `TerminalLocalWorkReportInput`.
2. Add internal disclosure citation construction using `WorkReportCitationTarget::AgentHarnessHookDisclosure`.
3. Include disclosure citations in the `ValidationAndQualityChecks` section.
4. Update validation/quality summary logic only as needed to preserve explicit not-available behavior.
5. Add focused helper tests.
6. Update docs and create an implementation report.
7. Review helper integration next.
8. Plan executor propagation separately.
9. Only after event/audit planning, consider automatic disclosure discovery.
10. Only after warning/skipped semantics are accepted, consider context-aware section routing.

## 16. Open Questions

- Should helper inputs require a parent `AgentHarnessHookInvocationId` alongside disclosure IDs in a later phase?
- Should helper inputs accept disclosure IDs before durable disclosure event/audit semantics exist? This plan recommends yes, explicitly supplied only.
- Should reports require both checkpoint and disclosure citations when both are known?
- Should WorkReport contracts ever require hook disclosure citations?
- Should future disclosure summaries be copied into report sections under a bounded policy, or should report sections stay generic?
- Should skipped disclosures eventually map to incomplete/deferred work by default?
- Should warning disclosures eventually map to risks by default?
- Should disclosure citations stay in `ValidationAndQualityChecks` until hook optionality and policy-controlled continuation are implemented?

## 17. Final Recommendation

Recommended next phase: **terminal report hook disclosure citation integration review**.

The implemented helper phase accepts explicitly supplied typed hook disclosure IDs, cites them through `WorkReportCitation::new(...)`, places them conservatively in `ValidationAndQualityChecks`, adds focused tests, updates docs, and creates an implementation report.

It does not implement executor propagation, automatic disclosure discovery, runtime hook behavior, warning continuation, skipped-with-disclosure continuation, blocked behavior, hook optionality, policy-controlled continuation, event append behavior, audit sink emission, persistence, report artifacts, CLI rendering, schemas, side effects, writes, reasoning lineage, recursive agents, agent swarms, hosted behavior, or release posture changes.
