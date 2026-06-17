# Governed Work Pattern

The Governed Work Pattern is accepted product and architecture direction for Workflow OS. It is not implemented as a runtime feature, schema, domain pack, or CLI command.

Further implementation remains future scoped work. `EvidenceReference` is implemented as a core model with selected attachment paths. `WorkReportContract` and `WorkReport` are implemented as core models, an in-memory terminal local report generation helper is implemented, an in-memory runtime result exposure helper is implemented, explicit executor-integrated report-bearing execution is implemented for local runs, an explicit local report artifact store is implemented, report/audit/missing-citation semantics are hardened, the Composable Harness Contract core model is implemented, and the typed handoff core model is implemented. Automatic runtime report generation for every run is not implemented. Evidence persistence, CLI rendering, examples, approval attachment, broader automatic attachment, automatic report artifact writing from executor paths, approval/cancellation report-bearing methods, nested harness execution, runtime handoff execution, and side-effect boundary modeling require separate scoped ADRs or implementation plans before any broader runtime behavior is added.

## 1. Definition

Governed Work Pattern is a reusable structure for AI-assisted enterprise work that binds context, evidence, policy, approvals, side-effect boundaries, validation, audit, observability, and structured reporting into a repeatable workflow.

The pattern describes work that follows a disciplined loop:

1. Read required context before acting.
2. Respect explicit product, policy, and domain boundaries.
3. Make scoped changes or recommendations.
4. Run validation and quality gates.
5. Preserve evidence.
6. Require approval for sensitive, ambiguous, or irreversible actions.
7. Produce a structured work report.
8. Clearly disclose incomplete, deferred, or uncertain work.

The pattern emerged from the way Workflow OS itself has been built: every meaningful task begins with required context, respects the project charter and engineering standard, makes scoped changes, runs checks, records evidence, and reports honestly on validation, risks, and incomplete work.

Workflow OS now includes a first self-governance dogfood project. It uses the local kernel as a governance wrapper for Workflow OS planning/docs work while Codex or a human still performs repository edits and validation outside the kernel. This proves the governance loop can be applied to Workflow OS work without claiming automatic build execution, recursive agents, agent swarms, or production self-hosting.

Self-governed validation/check planning is documented in [Self-Governed Validation/Check Plan](../implementation-plans/self-governed-validation-check-plan.md). The local validation/check command contract model is implemented, but real command execution remains deferred until the model is reviewed and a handler boundary is separately scoped.

### Kernel-Governed Agent Execution

The recommended local adoption path is kernel-governed agent execution:

```text
Agent executes. Workflow OS governs.
```

In this pattern, Codex, Claude Code, or another coding agent performs repository work while Workflow OS supplies the governing layer: validation, durable run state, policy gates, approval checkpoints, auditability, and report posture. The YAML project files are the governed contract the agent operates inside, not the entire user experience.

This is an onboarding and operating-model pattern, not a new runtime capability. The current implementation does not make Workflow OS execute coding agents directly, does not run arbitrary local checks by default, does not add recursive agents or agent swarms, and does not replace deterministic governance with model self-review.

The current quickstart is documented in [Agent Harness Quickstart](../user-guide/agent-harness-quickstart.md). The scaffold command `workflow-os init-agent-harness` is implemented for local documentation setup: it creates or updates `AGENTS.md` and `.workflow-os/agent-harness-prompt.md` only. It remains explicit and must not silently enable workflow execution, approvals, local check execution, handler registration, writes, hosted behavior, schema changes, or higher autonomy.

## 2. Why It Matters

Governed AI work is not unique to software engineering. The same structure applies across enterprise domains where work must be explainable, reviewable, safe, and auditable.

- Legal: review contract language against policy, cite evidence, flag ambiguous clauses, and require approval before sending recommendations externally.
- Finance: assess exceptions, reconcile evidence, apply approval thresholds, and produce a decision packet.
- HR: draft policy responses, preserve source policy references, require approval for sensitive employee-impacting actions, and record handoff notes.
- Security: triage incidents, classify evidence, escalate ambiguous findings, and preserve audit context.
- Procurement: review vendor intake, compare requirements against policy, collect missing evidence, and produce approval-ready summaries.
- Customer support: classify customer issues, gather context, recommend resolution, and avoid irreversible customer-facing updates without approval.
- Operations: inspect operational state, run local checks, classify risks, and document remediation or escalation paths.
- Data/analytics: validate source assumptions, run quality gates, capture lineage evidence, and report failed checks.
- Software engineering: read standards and architecture docs, make scoped changes, run tests, preserve review evidence, and disclose gaps.

This direction helps Workflow OS stay centered on governed enterprise work rather than becoming a narrow coding-agent wrapper or a collection of integration scripts.

## 3. Why This Belongs After Phase 2

Phase 2 read-only adapters introduced real external context into Workflow OS without adding writes. That is the right pressure point for this concept.

Once workflows can read facts from systems such as repositories, issue trackers, and CI providers, the next architectural question is not more integration breadth. The next question is how Workflow OS should represent:

- what context had to be read before work began
- what evidence was actually used to support a conclusion
- what decisions were made from that evidence
- what validation and quality gates were run
- what approvals were requested, granted, or denied
- what side effects were proposed, approved, attempted, completed, skipped, denied, or failed
- what report should be handed to an operator, reviewer, downstream workflow, or auditor

Read-only integrations make evidence and decision traceability concrete. Write-capable integrations would make the same gaps riskier. Governed Work Pattern should therefore be clarified before Workflow OS moves toward policy-gated writes, broader domain packs, or generic runtime adapter execution.

This does not mean Phase 2 public preview readiness should be interrupted. The public read-only integration preview has narrow live-smoke evidence, and Governed Work Pattern remains an architecture capture step that helps the project avoid using integration breadth as a substitute for governance depth.

## 4. Product Boundary

Governed Work Pattern does not change the Workflow OS product boundary.

It does not make Workflow OS:

- a generic chat agent framework
- a SaaS workflow builder
- a business process management engine
- a coding-agent wrapper
- an unconstrained autonomous agent framework

Workflow OS remains a declarative, local-first, policy-gated, auditable workflow kernel. The pattern is a way to describe how that kernel should support serious enterprise work over time.

## 5. Anti-Overfitting Guidance

Governed Work Pattern must not overfit core concepts to software engineering or to the Phase 2 provider examples.

Core concepts introduced for this pattern should not be named after engineering-specific artifacts such as:

- pull requests
- tickets
- CI runs
- code review
- branches
- commits
- reviews

Those terms belong in adapters, skills, examples, or future domain packs. Core should use domain-neutral names such as evidence, context, decision, approval, side effect, quality gate, audit record, and work report.

This rule matters because the same pattern must support legal contract review, finance exception approval, HR policy response, security triage, procurement intake, customer support resolution, operations review, data quality investigation, and software engineering without fragmenting the kernel.

## 6. Relationship To Existing Kernel

The pattern maps to existing v0 and Phase 2 primitives:

- Durable workflow state: preserves run progress without relying on local process memory.
- Immutable run identity: binds a run to schema version, workflow version, and spec content hash.
- Event log: records meaningful state transitions as append-only history.
- Policy decisions: evaluate governance before meaningful actions.
- Approvals: pause sensitive work until a human decision is recorded.
- Audit events: preserve who or what acted, when, against which run identity, and with what policy context.
- Observability: exposes runtime signals, failures, retry behavior, approval waits, and operational health.
- Adapter contracts: require external reads and future writes to pass through bounded, policy-aware integration interfaces.
- Capabilities: make external, local, approval, secret, and workflow actions explicit.
- Validation: catches invalid specs and unsafe declarations before execution.
- CLI reports: return operator-facing diagnostics, run status, inspection output, and structured implementation reports from Codex tasks.

The pattern does not require new primitives immediately. It gives future modeling work a vocabulary for deciding what belongs in core, skills, adapters, domain packs, reports, and docs.

### Governed Multi-Step Workflow Execution

Governed multi-step workflow execution is now a P0 kernel capability, accepted in [ADR 0010: Governed Multi-Step Workflow Execution](../adr/0010-governed-multi-step-workflow-execution.md) and scoped in [Governed Multi-Step Workflow Execution Plan](../implementation-plans/governed-multi-step-workflow-execution-plan.md).

Kernel dogfooding showed that one-governance-check workflows do not scale. The local executor now supports the first bounded slice: sequential ordered local steps with policy checks, auditability, approval-aware execution, safe failure behavior, and compatibility with final reportability.

The implementation remains sequential, local, deterministic, and conservative. It proves step-by-step governance before Workflow OS attempts branching, parallel execution, nested harness runtime behavior, write-capable adapters, or reasoning lineage.

This is not agent orchestration. It is the kernel learning to execute authored workflow steps under governance.

## 7. Future Candidate Concepts

The following are future candidates only. They are not implemented by this document.

- `required_context`: declared context that must be read, loaded, or referenced before work begins. It is an obligation or precondition.
- `evidence_reference`: non-secret pointer to source material, provider object, local file, audit record, command output, validation result, or human-supplied evidence that was actually used to support a conclusion, decision, validation result, approval, or report.
- `decision`: structured outcome of a policy, approval, classification, recommendation, or review step.
- `policy_gate`: explicit gate that must pass before a meaningful action.
- `approval`: human decision attached to a workflow run, step, actor, reason, and timestamp.
- `high_assurance_approval_controls`: future multi-party and role-bound approval controls for highly sensitive actions, including quorum rules, separation of duties, evidence-required approval context, expiry/revocation semantics, and immutable approval audit trails.
- `side_effect`: proposed, approved, attempted, completed, denied, skipped, failed, or potentially rolled back external or local mutation.
- `audit_record`: operator-facing record suitable for later reconstruction of who did what and why.
- `work_report`: structured summary of work performed, evidence considered, decisions made, validation run, incomplete work, risks, and handoff notes.
- `work_report_contract`: schema or contract describing required fields for a work report.
- `composable_harness_contract`: future contract for a bounded execution envelope inside a workflow, with typed inputs, typed outputs, scoped authority, evidence requirements, approval rules, failure semantics, and handoff obligations.
- `quality_gate`: validation, test, review, or policy check that must pass before work advances.
- `agent_harness_hook`: future deterministic named checkpoint invoked by an agent harness before or after a governed phase of work, such as planning, validation, review, or reporting.
- `known_limitations`: explicit declaration of unsupported, local-only, fixture-only, or deferred behavior.
- `incomplete_work_disclosure`: required statement of placeholder, partial, failed, skipped, or deferred work.

### Required Context Versus Evidence Reference

`required_context` and `evidence_reference` are related but not interchangeable.

`required_context` describes what must be loaded or read before work starts. It is part of the workflow's preparation and completeness boundary. A workflow might require a policy document, an approved resource list, a project manifest, a provider object reference, a prior audit record, or a human-supplied instruction packet before it can proceed safely.

`evidence_reference` describes what was actually used to support a conclusion, decision, validation result, approval, or work report. A workflow may load required context that later turns out not to support a specific conclusion. The work report should cite the evidence that mattered, not merely list everything that was available.

This distinction prevents reports from becoming context dumps while still preserving enough traceability for review.

### Composable Harness Contracts

Composable Harness Contracts are a future Governed Work Pattern capability. The core model is implemented, but they are not implemented as runtime behavior, schemas, CLI behavior, domain packs, write support, hosted execution, distributed workers, or Level 3/4 autonomy.

Planning and implementation status are documented in [Composable Harness Contract Plan](../implementation-plans/composable-harness-contract-plan.md). The model defines a contract boundary only; it does not authorize nested harness execution.

A harness is a bounded execution envelope. It is not synonymous with an agent. A harness may contain an agent, deterministic code, tools, policy checks, validation, or human approval. A composable harness contract should eventually define:

- name or ID;
- purpose;
- allowed inputs;
- required context;
- allowed tools;
- allowed side effects;
- output schema;
- evidence requirements;
- approval policy;
- timeout, budget, and retry policy;
- failure semantics;
- handoff requirements.

The enterprise need is real: AI work will increasingly be decomposed across specialized systems, tools, and reasoning actors. The hard problem is not delegation by itself. The hard problem is governed delegation: explicit authority, bounded context, durable state, side-effect control, evidence, policy gates, approvals, auditability, and traceable handoffs.

Workflow OS should therefore treat Composable Harness Contracts as a later contract layer on top of stable primitives:

- workflow and run identity;
- durable state or event log;
- EvidenceReference and evidence-ledger behavior;
- policy gates;
- approval model;
- typed handoffs;
- scoped authority;
- validation;
- final work report.

This should not be implemented too early. Harness contracts add coordination overhead, can create false governance when review is only another model opinion, can create context drift when handoffs are natural-language summaries, can create security risk when authority is ambient instead of explicitly delegated, can cause parallel write conflicts, and depend on the basic Workflow OS primitives being stable first.

Relationship to existing concepts:

- Workflow OS is the governed work runtime.
- A workflow is the authored unit of governed work.
- A harness is a bounded execution envelope within a workflow.
- An agent is a reasoning or execution actor inside a harness.
- A tool is a capability exposed under policy.
- Evidence is durable proof attached to a claim, validation, decision, or report citation.
- A handoff is a typed transfer of artifacts, claims, risks, and next obligations.
- A work report is the final auditable summary.

The typed handoff core model is implemented as a validated, reference-first model and reviewed. WorkReport typed handoff citation planning is documented in [WorkReport Typed Handoff Citation Plan](../implementation-plans/work-report-typed-handoff-citation-plan.md), and WorkReport citation vocabulary for typed handoffs is implemented. Terminal report helper typed handoff citation integration is implemented in [Terminal Report Typed Handoff Citation Integration Plan](../implementation-plans/terminal-report-typed-handoff-citation-integration-plan.md). Executor-integrated typed handoff report input propagation is implemented in [Executor Typed Handoff Report Input Propagation Plan](../implementation-plans/executor-typed-handoff-report-input-plan.md). Typed handoffs do not add runtime handoff generation, nested harness execution, schema fields, CLI behavior, persistence, side-effect modeling, writes, or reasoning lineage.

Illustrative future pattern: an AI-assisted software engineering workflow could use a spec harness, planning harness, implementation harness, test/verification harness, review harness, security/risk harness, and final work report harness. This is an example of future execution topology, not an implementation commitment.

Non-goals:

- No arbitrary recursive agent spawning.
- No agent swarm positioning.
- No claim that Workflow OS currently supports production nested execution.
- No live write integrations as part of this roadmap direction.
- No hosted or distributed runtime claim.
- No Level 3/4 autonomy claim.
- No replacement of deterministic governance with model self-review.

### Agent Harness Hooks

Agent Harness Hook Integration is a planned adoption maturity layer, not an implemented runtime feature.

The current agent scaffold is the `dbt_project.yml` equivalent for human/agent orientation: useful for declaring conventions, expectations, and structure, but not itself an enforcement layer. Future hooks should be deterministic, named checkpoints that the harness invokes before or after important work phases.

Planned hook integration should reduce reliance on prose-only agent instruction following while preserving the product boundary:

- Agent executes.
- Workflow OS governs.
- Hooks provide explicit checkpoints.
- Runtime state, approvals, evidence, reports, and local check results remain governed model/API outputs, not invented agent claims.

Hook planning is documented in [Agent Harness Hook Integration Plan](../implementation-plans/agent-harness-hook-integration-plan.md), runtime invocation planning is documented in [Agent Harness Hook Runtime Invocation Plan](../implementation-plans/agent-harness-hook-runtime-invocation-plan.md), hook audit/event semantics planning is documented in [Agent Harness Hook Audit/Event Semantics Plan](../implementation-plans/agent-harness-hook-audit-event-semantics-plan.md), WorkReport hook citation target planning is documented in [WorkReport Agent Harness Hook Citation Target Plan](../implementation-plans/work-report-hook-citation-target-plan.md), terminal report helper hook citation integration is documented in [Terminal Report Agent Harness Hook Citation Integration Plan](../implementation-plans/terminal-report-hook-citation-integration-plan.md), executor hook report input propagation is implemented in [Executor Hook Report Input Propagation Plan](../implementation-plans/executor-hook-report-input-plan.md), runtime hook execution planning is documented in [Agent Harness Hook Runtime Execution Plan](../implementation-plans/agent-harness-hook-runtime-execution-plan.md), executor checkpoint planning is documented in [Executor Hook Checkpoint Plan](../implementation-plans/executor-hook-checkpoint-plan.md), executor hook event/audit semantics planning is documented in [Executor Hook Event And Audit Semantics Plan](../implementation-plans/executor-hook-event-audit-semantics-plan.md), hook event audit projection is implemented as projection-only in [Hook Event Audit Projection Plan](../implementation-plans/hook-event-audit-projection-plan.md), and the first explicit `BeforeSkillInvocation` executor hook event append path is implemented in [Executor Hook Event Append Plan](../implementation-plans/executor-hook-event-append-plan.md). The agent harness hook contract model is implemented as a validated, model-only contract boundary for deterministic named checkpoints, the in-memory invocation helper model is implemented for explicit phase-level context validation, the hook audit record core model is implemented as model-only vocabulary and validation, WorkReport citation vocabulary for agent harness hook invocation IDs is implemented as model-only vocabulary, terminal report helper integration can cite explicitly supplied hook invocation IDs, executor report-bearing execution can forward explicitly supplied hook invocation IDs into generated reports, and `execute_runtime_agent_harness_hook(...)` can produce an in-memory invocation result plus model-only hook audit record from explicit inputs. The hook workflow event vocabulary is implemented, and `LocalExecutor::execute(...)` can append bounded `HookInvocationRequested` and `HookInvocationEvaluated` events only for an explicitly supplied `BeforeSkillInvocation` checkpoint. The explicit `BeforeReport` executor checkpoint is implemented for `execute_with_report(...)` only and remains report-path-only, in-memory-only, and non-mutating. Broader automatic executor hook invocation, workflow-declared hook configuration, runtime hook configuration, dedicated hook audit sink emission, CLI hook commands, workflow schema fields, automatic local checks, persistence changes, broader executor workflow event emission, report artifact auto-writing, side-effect modeling, writes, hosted execution, recursive agents, agent swarms, and release posture changes remain unimplemented.

### Side-Effect Boundary States

Future side-effect modeling should make side-effect state explicit. Candidate states include:

- `proposed`: a side effect has been suggested or planned but not authorized.
- `approved`: policy or human approval has authorized the side effect.
- `attempted`: the runtime or adapter attempted the side effect.
- `completed`: the side effect completed successfully.
- `denied`: policy, approval, capability, or safety checks blocked the side effect.
- `skipped`: the workflow intentionally did not attempt the side effect.
- `failed`: the side effect was attempted and did not complete successfully.
- `rolled_back`: a future version may represent compensating action or rollback where the external system supports it.

Rollback is a future candidate only. Workflow OS must not imply rollback exists for external systems unless a specific adapter contract and operation implement it honestly.

## 8. Layering Hypothesis

Workflow OS should treat the pattern as layered.

Core should own generic governance primitives:

- durable workflow state
- immutable run identity
- event log
- policy decisions
- approvals
- auditability
- observability
- evidence references
- report contracts
- side-effect boundaries

Skills should remain bounded capabilities:

- summarize
- classify
- compare against policy
- validate evidence
- draft recommendation
- generate report
- prepare approval packet

Domain packs, if introduced later, should provide opinionated templates:

- engineering review and release workflows
- legal contract review workflows
- finance exception approval workflows
- security incident triage workflows
- HR policy-response workflows
- procurement intake workflows
- support resolution workflows
- data quality workflows

This layering keeps Workflow OS generic across enterprise domains while still allowing useful domain-specific workflows later.

## 9. Work Report Direction

Codex implementation reports point toward a broader `work_report` contract.

`WorkReportContract` planning is documented in [WorkReportContract Planning Document](../implementation-plans/work-report-contract-plan.md), and the core contract and report models are implemented. Terminal local report generation planning is documented in [Terminal Local Report Generation Plan](../implementation-plans/terminal-local-report-generation-plan.md), and the in-memory helper is implemented. Runtime result exposure planning is documented in [Runtime Result Report Exposure Plan](../implementation-plans/runtime-result-report-exposure-plan.md), and the in-memory runtime result exposure helper is implemented. Executor-integrated report result planning is documented in [Executor-Integrated Report Result Plan](../implementation-plans/executor-integrated-report-result-plan.md), and `LocalExecutor::execute_with_report(...)` is implemented as an explicit additive local execution path. Report artifact planning is documented in [Report Artifact Plan](../implementation-plans/report-artifact-plan.md), and the explicit local artifact store is implemented. Report/audit/missing-citation semantics are documented in [Report, Audit, And Missing-Citation Semantics Plan](../implementation-plans/report-audit-missing-citation-semantics-plan.md): generated reports remain derived handoff artifacts, report-generation errors remain separate from workflow execution errors, and absent optional references remain explicit section text rather than fabricated citations. The helpers and plans do not add automatic runtime report generation for every run, automatic artifact writing from executor paths, CLI rendering, examples, reasoning lineage, approval evidence attachment, approval/cancellation report-bearing methods, side-effect modeling, writes, schemas, or release posture changes.

A future governed work report should be able to capture:

- work performed
- inputs considered
- evidence references
- decisions made
- policy gates evaluated
- approvals requested, granted, or denied
- side effects attempted, completed, skipped, or denied
- validation performed
- quality gates passed or failed
- incomplete work
- deferred work
- risks
- follow-ups
- confidence and uncertainty
- operator handoff notes

The report should not be a marketing summary. It should be a governed handoff artifact that operators, reviewers, auditors, and downstream workflows can inspect.

### Audit Records Versus Work Reports

Audit records and work reports serve different purposes.

Audit records are low-level operational history. They should preserve what happened, when it happened, which actor or system actor caused it, which run identity and policy context applied, and which event or decision produced the record. Audit records are optimized for reconstruction, compliance review, troubleshooting, and accountability.

Work reports are high-level governed handoff artifacts. They should explain what work was performed, what evidence supported the conclusions, what decisions were made, what validation passed or failed, what approvals occurred, what side effects were attempted or avoided, what remains incomplete, and what an operator should do next.

A work report may cite audit records, workflow events, adapter invocation records, approval decisions, validation results, and evidence references. It should not be reduced to an audit log. An audit log says what happened; a work report explains what was done and why it is ready, blocked, risky, incomplete, or ready for handoff.

Future work should determine whether `work_report_contract` belongs in core, in schemas, or in a higher-level domain template layer. It should also decide how strict runtime enforcement should be.

## 10. Evidence Direction

`evidence_reference` likely belongs close to the core over time because governed work depends on knowing what evidence was considered without copying raw sensitive payloads into workflow specs, audit logs, or reports.

Candidate evidence references could point to:

- local files
- spec files and content hashes
- validation results
- workflow events
- audit records
- provider objects read through adapters
- approval decisions
- command outputs summarized by reference
- human-provided context packets

This task does not implement `evidence_reference`. The concept should be revisited before policy-gated writes or broader domain packs because external side effects require stronger evidence and decision traceability.

## 11. Possible First Implementation Hypothesis

This section is non-binding and not implemented.

A possible first implementation could include:

- `evidence_reference` as a generic reference object that can point to local files, provider objects, validation results, workflow events, audit records, approval decisions, command output summaries, or human-supplied context packets without storing raw sensitive payloads by default.
- `work_report_contract` as a declarative report schema that defines required sections for a class of governed work.
- a terminal work report artifact produced at completion, failure, cancellation, or escalation.
- links from report sections to audit events, adapter invocation records, validation results, approval decisions, and evidence references.
- domain-specific report templates outside core for engineering, legal, finance, HR, security, procurement, support, operations, and data quality workflows.

This hypothesis should not be treated as approved design. It exists to make the next design discussion concrete and reviewable.

## 12. Timing

Do not implement Governed Work Pattern immediately.

This concept must not interrupt:

- Phase 2 live-smoke evidence work
- Phase 2 public read-only integration preview readiness
- local-kernel correctness work

Do not build domain packs yet.

Revisit this concept before:

- policy-gated writes
- broader domain packs
- generic runtime adapter execution
- public claims about automatic governed work-report generation, CLI report rendering, production report artifacts, or report schema stability

The sequencing should be:

1. Accept Governed Work Pattern as product and architecture direction.
2. Consider Reasoning Lineage or Claim Graph as a follow-on concept for claim, assumption, evidence, and decision relationships.
3. Revisit Governed Work Pattern implementation before policy-gated writes, broader domain packs, or generic runtime adapter execution.

Reasoning Lineage or Claim Graph may complement governed work reports by making claims, assumptions, and evidence relationships more explicit. They should be considered in sequence before implementation work turns this concept into runtime contracts.

## 13. Open Questions

- What belongs in core versus skills versus domain packs?
- Should `evidence_reference` become a core concept?
- Should `work_report_contract` become a core concept?
- How much report structure should be enforced by the runtime?
- How should governed work reports relate to audit events?
- How should governed work reports relate to future reasoning lineage?
- What is the minimum viable implementation after concept approval?

## Implementation Status

Partially implemented through separately scoped phases. `EvidenceReference`, `WorkReportContract`, `WorkReport`, explicit terminal local report helpers, executor-integrated report-bearing local execution, and an explicit local report artifact store now exist. The broader Governed Work Pattern is still not implemented as a complete runtime feature, schema, domain pack, CLI command, write boundary, or production operating model.
