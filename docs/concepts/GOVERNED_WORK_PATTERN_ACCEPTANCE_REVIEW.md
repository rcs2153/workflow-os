# Governed Work Pattern Acceptance Review

Review date: 2026-06-04

Reviewed materials:

- `docs/adr/0007-governed-work-pattern.md`
- `docs/concepts/governed-work-pattern.md`
- `docs/adr/0008-reasoning-lineage-claim-graph.md`
- `docs/concepts/reasoning-lineage.md`
- Phase 2 public read-only preview readiness and live-smoke evidence
- v0 runtime, spec, concept, and architecture documentation
- user-guide field guide, workbook, and RC1 evaluation guide

This review is architecture and scoping only. It does not implement runtime features, schemas, CLI behavior, writes, generic runtime adapter execution, domain packs, production backends, or reasoning lineage.

## 1. Executive Verdict

**Ready to Accept with scoped MVP implementation path.**

ADR 0007 is mature enough to accept as architecture direction, provided acceptance is explicit that it does not authorize implementation by itself. The next implementation work should be split into separately reviewed, small ADRs or implementation plans for:

1. core `evidence_reference`;
2. core `work_report_contract` and terminal local work report artifact;
3. later side-effect boundary modeling before write-capable adapters.

This review does not edit ADR 0007 status. A maintainer should decide whether to update ADR 0007 from `Proposed` to `Accepted` in a separate focused change after reviewing this document.

## 2. Status Change Criteria Review

| ADR 0007 criterion | Status | Review finding |
| --- | --- | --- |
| Phase 2 live-smoke/public-preview readiness is resolved or explicitly paused. | Met | Phase 2 live smoke evidence was recorded and the public read-only preview readiness rerun approved the narrow `0.2.0-preview.1` posture. Final release review says the release pack is ready for maintainer tag. |
| Maintainers agree whether `evidence_reference` belongs in core. | Ready to decide | This review recommends **yes**, as a core MVP concept. It should be reference-only, redacted, and domain-neutral. |
| Maintainers agree whether `work_report_contract` belongs in core. | Ready to decide | This review recommends **yes**, as a core MVP concept for terminal governed handoff artifacts. Domain-specific report templates should stay outside core. |
| The relationship between work reports and audit events is defined. | Met by this review | Audit events are low-level operational truth. Work reports are high-level governed handoff artifacts that cite audit events where useful. |
| The relationship to future Reasoning Lineage / Claim Graph work is clarified. | Met by this review | Reasoning Lineage should remain future provenance substrate work. Evidence references and work reports should be designed so lineage can attach later without becoming the runtime. |
| A minimum viable implementation plan is reviewed and scoped. | Met by this review | A small phased MVP is proposed below. It starts with evidence references and terminal local work reports, not writes or domain packs. |
| The accepted decision still preserves the Workflow OS product boundary. | Met | The proposed MVP remains local-first, declarative, policy-gated, auditable, and domain-neutral. |
| Acceptance does not authorize writes, domain packs, schemas, generic runtime adapter execution, or new CLI behavior by itself. | Met with guardrail | This review explicitly keeps implementation separate. Any schema or CLI work requires a scoped follow-up plan. |

Conclusion: ADR 0007 can move to Accepted as architecture direction if maintainers agree with the scoping in this review. Implementation still requires separate scoped work.

## 3. Core Vs Skills Vs Domain Packs

| Concept | Recommended layer | Rationale |
| --- | --- | --- |
| `required_context` | Core later | It is a workflow preparation boundary, but it should follow evidence/report MVP so the project learns what context must be enforceable. |
| `evidence_reference` | Core now | Evidence must be domain-neutral, redacted, reference-oriented, and reusable by audit, approvals, adapters, reports, and future lineage. |
| `decision` | Core later | Policy and approval decisions already exist. A generic decision model should wait until evidence/report usage proves the common shape. |
| `policy_gate` | Core now | Policy gates already exist conceptually and in runtime behavior. Future report contracts should cite them rather than redefining them. |
| `approval` | Core now | Approval is already core runtime behavior and should be citeable from evidence and reports. |
| `side_effect` | Core later | Needed before writes, but premature before evidence/report MVP. It must be designed before any write-capable adapter. |
| `audit_record` | Core now | Audit records already exist as local runtime/audit projections. Work reports should cite them, not replace them. |
| `work_report` | Core now | Terminal governed handoff artifacts should be core because they are cross-domain and tied to run identity, evidence, approvals, validation, and incomplete work. |
| `work_report_contract` | Core now | A minimal declarative contract should define required report sections without imposing domain-specific language. |
| `quality_gate` | Core later | Existing validation and policy checks can be cited first. Generic quality gates should be added after report needs are clear. |
| `known_limitations` | Docs/convention only, core later as report section | Known limitations are already documentation practice. They should become a report section before becoming a standalone runtime primitive. |
| `incomplete_work_disclosure` | Core now as report section | Disclosure of incomplete/deferred/skipped work is central to governed handoff and should be mandatory in the MVP report contract. |

In this table, **core now** means “belongs in the next scoped MVP if maintainers approve implementation.” It does not mean implemented by this review.

## 4. Evidence Reference Decision

### Should `evidence_reference` belong in core?

Yes. `evidence_reference` should belong in core because every governed workflow domain needs a safe, durable way to point at the evidence used for decisions, approvals, validation, and reports without copying raw sensitive payloads into workflow state.

### Minimum Useful v1

The minimum useful v1 should be a small, typed reference object that can attach to workflow runs, steps, adapter telemetry records, validation results, approval decisions, audit events, and terminal work reports.

Required fields:

- `id`: stable evidence reference ID.
- `kind`: domain-neutral kind such as `local_file`, `spec`, `validation_result`, `workflow_event`, `audit_event`, `adapter_invocation`, `approval_decision`, `operator_note`, or `external_reference`.
- `title`: short human-readable label.
- `uri_or_reference`: non-secret pointer or internal object reference.
- `source_component`: component that created the reference, such as `validator`, `runtime`, `adapter`, `cli`, `operator`, or `skill`.
- `workflow_id`, `workflow_version`, `schema_version`, `spec_hash`, and `run_id` where run-scoped.
- `step_id` and `skill_id/version` where step-scoped.
- `correlation_id` where available.
- `created_at`.
- `actor` or `system_actor` where available.
- `redaction_metadata`.
- `summary`: optional redacted summary.
- `content_hash` or provider ETag/checksum where available and safe.

### What Must Not Be Stored

`evidence_reference` must not store:

- provider tokens;
- authorization headers;
- private keys;
- raw CI logs by default;
- raw Jira descriptions or comments by default;
- raw large GitHub file contents by default;
- raw provider payloads by default;
- personal or sensitive data unless explicitly summarized and redacted;
- secret values copied from specs, environment variables, or operator shells.

### Relationships

- Adapters should produce evidence references from read summaries and provider object references.
- Audit events may cite evidence references but remain low-level operational records.
- Validation results may become evidence references for work reports.
- Approval decisions may cite evidence references used by the approver.
- Work reports should cite evidence references in report sections.
- Future Reasoning Lineage may link claims/findings to evidence references.

### Privacy And Redaction Rules

Evidence references should use references and summaries by default. Redaction metadata must state whether content is safe, redacted, reference-only, or omitted. Debug and display output must not leak sensitive payloads. Provider metadata should be treated as sensitive unless explicitly safe.

## 5. Work Report Contract Decision

### Should `work_report_contract` belong in core?

Yes, but only as a minimal domain-neutral contract. Core should define report shape, required sections, run identity, citations, and incomplete-work disclosure. Domain packs should define domain-specific report templates later.

### Minimum Useful v1

The minimum useful v1 should define a terminal local work report artifact with:

- report ID;
- report contract ID/version;
- workflow/run identity;
- status: completed, failed, canceled, escalated, or blocked;
- work performed;
- evidence references used;
- decisions made;
- policy gates evaluated;
- approvals requested/granted/denied;
- validation and quality checks run;
- side effects attempted/completed/skipped/denied, if any exist in scope;
- incomplete or deferred work;
- known limitations;
- risks and operator handoff notes;
- redaction metadata;
- generated timestamp and actor/system actor.

### Terminal Artifact Only?

For v1, yes. Reports should be terminal artifacts produced at completion, failure, cancellation, or escalation. Mid-run reports can wait until the terminal report contract proves useful.

### Should Every Workflow Require A Report?

Not immediately. v1 should allow workflows to declare report requirements. The vertical slice and Phase 2 read-only examples should opt in first. Making every workflow require a report should wait until the contract is stable.

### Runtime-Enforced Or Workflow-Declared?

Workflow-declared first. Validation should ensure workflows that declare report requirements use a known report contract and include required sections. Runtime enforcement should be narrow: produce or persist the terminal report when a report contract is declared. Broad mandatory runtime enforcement should wait.

### Citation Rules

Reports should cite:

- evidence references;
- audit events;
- workflow events;
- adapter telemetry records;
- validation results;
- approval decisions;
- policy decisions;
- incomplete/deferred work disclosures.

Reports should cite by stable reference, not by copying raw payloads.

### Explicitly Out Of Scope

v1 work reports must not include:

- domain packs;
- production report export;
- report signing or notarization;
- SIEM/OpenTelemetry export;
- reasoning lineage graph storage;
- generic live adapter execution;
- write-capable adapter behavior;
- UI rendering;
- natural-language marketing summaries as the source of truth.

## 6. Audit Vs Work Report Relationship

Definitions:

- **Audit event**: low-level operational record of who or what did what, when, under which run identity and policy context. Audit events are for reconstruction, compliance review, troubleshooting, and accountability.
- **Adapter telemetry**: local/runtime-preview record of adapter invocation and observability data, including adapter kind, action, capability, operation mode, policy precheck provenance, latency, result summary, error classification, and redaction metadata.
- **Work report**: high-level governed handoff artifact describing what was done, what evidence was used, what decisions and approvals occurred, what validation passed or failed, what remains incomplete, and what operator handoff is needed.
- **Evidence reference**: safe pointer to evidence used by a conclusion, decision, approval, validation result, audit projection, telemetry record, report section, or future claim/finding.
- **Reasoning lineage**: future provenance layer that may explain how claims, findings, corrections, and decisions emerged from evidence and prior reasoning. It is not implemented and should not replace workflow state or reports.

Source-of-truth boundaries:

- Workflow event stream is source of truth for run state.
- Audit events are source of truth for low-level operational audit projections.
- Adapter telemetry is source of truth for local preview adapter invocation summaries and observability records.
- Evidence references are source of truth for what evidence was cited, not for the full underlying provider payload.
- Work reports are source of truth for governed handoff narrative and report contract compliance.
- Reasoning lineage, if implemented later, should be source of truth for claim/finding derivation relationships, not for run execution state.

## 7. Side-Effect Boundary Before Writes

Before any write-capable adapter, Workflow OS should define domain-neutral side-effect states:

- `proposed`: a mutation or external action is suggested but not authorized.
- `approved`: policy and/or human approval authorizes the side effect.
- `attempted`: an adapter or runtime path attempted the side effect.
- `completed`: the side effect completed successfully.
- `denied`: policy, approval, capability, or safety checks blocked the side effect.
- `skipped`: the workflow intentionally did not attempt the side effect.
- `failed`: the side effect was attempted and failed.
- `rolled_back` or `compensated`: future only, and only where an adapter honestly supports rollback or compensation.

The side-effect model should require:

- capability and policy gate before attempt;
- approval when action is sensitive, ambiguous, or irreversible;
- idempotency key before external mutation;
- evidence references supporting the decision to attempt;
- audit events for proposal, approval/denial, attempt, completion/failure;
- report disclosure for skipped, denied, failed, or incomplete side effects.

No write-capable adapter should be built until this model is scoped and accepted.

## 8. Minimum Viable Implementation Path

Proposed sequence:

1. **Evidence Reference Core Model**
   - Add domain-neutral `EvidenceReference` types.
   - Support local/spec/validation/event/audit/adapter/approval/operator reference kinds.
   - Add redaction metadata and safe debug/display behavior.
   - Add tests for non-leakage and reference-only provider payload handling.

2. **Work Report Contract Core Model**
   - Add minimal `WorkReportContract` and `WorkReport` types.
   - Keep report sections domain-neutral.
   - Require incomplete/deferred work disclosure when a report is declared.
   - Add serialization and validation tests.

3. **Terminal Local Work Report Artifact**
   - Persist report artifacts in the local backend for workflows that declare report requirements.
   - Produce reports at completion, failure, cancellation, or escalation.
   - Link reports to run identity, evidence references, audit events, adapter telemetry, validation results, and approvals.

4. **Validation Rules For Report Requirements**
   - Let workflows declare a report contract.
   - Validate required sections and known report contract IDs.
   - Do not require every workflow to report until the contract stabilizes.

5. **Operator Display**
   - Extend inspection/report display only after report artifacts exist.
   - Keep CLI JSON experimental unless a separate CLI contract update is accepted.

6. **Example Updates**
   - Add terminal work reports to the vertical slice.
   - Add terminal work reports to GitHub/Jira/CI read-only examples.
   - Keep all examples fixture-first and read-only.

7. **Side-Effect Boundary ADR**
   - Before writes, create a separate ADR for side-effect lifecycle, idempotency, approval, evidence, audit, and report requirements.

This path should be implemented as small PRs. It must not include writes, domain packs, generic live adapter execution, production backends, or reasoning lineage implementation.

## 9. Product Boundary Risks

### Becoming A BPM Engine

Risk: adding too many process-modeling primitives could turn Workflow OS into a generic BPM system.

Guardrail: core should model only the primitives needed for governed AI workflow execution: evidence, reports, policy, approvals, side-effect boundaries, state, events, audit, and observability.

### Becoming A Chat Agent Framework

Risk: work reports and reasoning concepts could drift into transcript capture.

Guardrail: store governed artifacts, references, and report sections, not chat turns or general memory.

### Overfitting To Software Engineering

Risk: evidence/report concepts could be named around pull requests, tickets, CI, commits, or reviews.

Guardrail: core names must stay domain-neutral. Provider/domain terms belong in adapters, skills, examples, or future domain packs.

### Duplicating Audit Logs

Risk: work reports could become noisy copies of audit events.

Guardrail: reports cite audit records and summarize governed handoff. They do not replace low-level audit history.

### Storing Sensitive Evidence Payloads

Risk: reports and evidence references could copy raw provider payloads.

Guardrail: reference by default, summarize carefully, redact deterministically, and test debug/display paths for leakage.

### Turning Reports Into Marketing Summaries

Risk: reports become optimistic prose rather than governed handoff artifacts.

Guardrail: report contracts must require evidence, validation, incomplete work, limitations, risks, and operator handoff notes.

### Adding Writes Too Early

Risk: evidence/report work becomes a pretext for provider mutations.

Guardrail: side-effect lifecycle must be accepted before writes. This review does not authorize write-capable adapters.

## 10. Recommendation

Recommended decision:

- **Accept ADR 0007 as architecture direction**, but do not treat acceptance as implementation authorization.
- Create separate scoped implementation ADRs or plans for:
  - `EvidenceReference` core model.
  - `WorkReportContract` and terminal local `WorkReport` artifacts.
  - Side-effect boundary model before writes.
- Keep ADR 0008 proposed until evidence references and work report contracts are designed enough to decide where reasoning lineage belongs.
- Implement a scoped MVP only after maintainers approve the evidence/report implementation plan.

What must still not be built:

- write-capable GitHub, Jira, CI, or generic adapter actions;
- generic live adapter execution from arbitrary workflow specs;
- domain packs;
- production database backend;
- distributed workers;
- hosted service;
- OAuth or webhooks;
- production telemetry export;
- Level 3/4 autonomy enablement;
- reasoning lineage graph storage or query behavior;
- UI.

Next concrete step:

1. Maintainer reviews this acceptance review.
2. If accepted, update ADR 0007 status in a small docs-only change.
3. Draft a focused ADR or implementation plan for `EvidenceReference` and terminal `WorkReport` MVP.
4. Keep all implementation work local, fixture-safe, and read-only until the side-effect boundary ADR is accepted.
