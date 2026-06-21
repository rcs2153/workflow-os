# Roadmap

Workflow OS grows from the local-first kernel outward.

Current sprint sequencing is captured in [Next Roadmap Sprint Plan](docs/implementation-plans/next-roadmap-sprint-plan.md). It identifies the immediate code exit from planning, the parallel subagent lanes, and the remaining prerequisite gates before write-capable adapter work.

Future demo workflow concepts are captured in [Workflow OS Demo Workflow Portfolio](docs/concepts/WORKFLOW_OS_DEMO_WORKFLOW_PORTFOLIO.md). They are candidate examples and benchmark narratives only; they do not implement schemas, runtime behavior, writes, hosted behavior, recursive agents, agent swarms, or release posture changes.

## Foundation

- Establish governance, contribution, security, release, and quality-gate standards.
- Set up the Rust workspace and TypeScript SDK workspace.
- Prepare documentation structure for concepts, specs, runtime, CLI, SDK, operations, security, and release.

## v0 Kernel

- Model canonical workflow specs in Rust.
- Define schema versioning and content hashing.
- Build validation for workflow definitions.
- Define durable state interfaces.
- Define append-only meaningful runtime events.
- Define policy, audit, and observability primitives.
- Build local-first CLI commands only after their contracts are documented.

## v0 Local Kernel Preview Release Hygiene

- Keep the public posture clear: v0 is a local kernel preview, not a production distributed runtime.
- Keep README, changelog, release readiness, known limitations, and example docs aligned.
- Keep CI green across Rust, TypeScript, docs, dependency audits, examples, and schema/SDK contracts.
- Apply release versions consistently across crates, packages, changelog, and release notes.
- Track schema/TypeScript synchronization explicitly until generated contracts exist.
- `YAML-001`: replace `serde_yaml` or isolate YAML parsing behind a maintained, bounded parser strategy before any production-readiness or malicious-spec hardening claim.
- Keep CLI JSON output marked as preview until a stable machine-output contract is designed.

## Adapter Readiness Criteria

Write-capable and production adapters should not be built until release posture and local kernel contracts are settled. Phase 2 read-only adapters are the narrow exception: they exist to prove the adapter contract against real systems without writes.

Before any real adapter implementation:

- Adapter capability, policy, idempotency, audit, and redaction contracts must remain enforced.
- External writes must remain denied unless explicitly designed, policy-gated, audited, and idempotent.
- Adapter health, error classification, dry-run/plan behavior, and redacted response summaries must be tested.
- Docs must continue to state that adapters cannot mutate core workflow state directly.

## Phase 2 Read-Only Integration Posture

Phase 2 is the read-only integration capability phase. It is documented in [docs/integrations/PHASE_2_READ_ONLY_INTEGRATIONS.md](docs/integrations/PHASE_2_READ_ONLY_INTEGRATIONS.md).

The `0.2.0-preview.1` public read-only integration preview includes initial Phase 2 read-only adapters:

- GitHub read-only adapter foundation.
- Jira read-only adapter foundation.
- GitHub Actions CI read-only adapter foundation.

GitHub Actions is the first CI target for read-only adapter proving. Other CI providers remain future work.

The `0.2.0-preview.1` posture approves a narrow public read-only integration preview after live smoke evidence was recorded and reviewed. That approval is limited to read-only provider access, fixture-first normal CI, and opt-in live tests.

Read-only adapter work must not imply write support, OAuth completeness, webhook ingestion, hosted operation, distributed workers, production database readiness, production integration readiness, broad live provider compatibility, or Level 3/4 autonomy enablement.

The following remain out of scope for Phase 2:

- Creating branches.
- Opening pull requests.
- Posting pull request comments.
- Updating Jira issues or comments.
- Changing Jira status.
- Rerunning CI.
- Workflow dispatch.
- Webhooks or an event ingestion service.
- OAuth app implementation.
- External writes of any kind.

## Governed Work Pattern Architecture

[Governed Work Pattern](docs/concepts/governed-work-pattern.md) is accepted as architecture direction by [ADR 0007](docs/adr/0007-governed-work-pattern.md). Acceptance does not implement runtime behavior or authorize schemas, CLI changes, writes, generic runtime adapter execution, or domain packs.

## P0 Blocker: Governed Multi-Step Workflows

Kernel dogfooding surfaced the next product blocker: one-governance-check workflows are not enough to govern realistic work at scale. Workflow OS becomes more valuable when a run can move through multiple deterministic governed steps, each with explicit policy checks, approval semantics, validation/check references, event history, failure behavior, and final work-report citations.

Governed multi-step workflow execution is now the P0 roadmap priority. [ADR 0010: Governed Multi-Step Workflow Execution](docs/adr/0010-governed-multi-step-workflow-execution.md) is accepted, and the bounded implementation plan is [Governed Multi-Step Workflow Execution Plan](docs/implementation-plans/governed-multi-step-workflow-execution-plan.md). The first sequential local executor slice is implemented: the local executor can run one or more ordered local steps, preserve per-step policy and approval behavior, retry/fail/escalate at the current step, and return report-bearing results for completed multi-step runs. It does not introduce parallel execution, branching execution, nested harness execution, writes, hosted/distributed runtime, schemas, examples, CLI behavior, automatic report generation, or reasoning lineage.

This pivot is distinct from Composable Harness Contracts. Multi-step governed execution is the kernel prerequisite; harness contracts and nested harness execution remain later capabilities that depend on the kernel proving durable step-by-step governance first.

The first sequential local multi-step executor slice has been reviewed and hardened with focused later-step approval, retry, policy-denial, cancellation, and report-generation-failure coverage. The self-governance dogfood workflow has been converted to a sequential multi-step workflow and reviewed. A tiny follow-up docs cleanup is implemented in [Self-Governance Dogfood Docs Cleanup Plan](docs/implementation-plans/self-governance-dogfood-docs-cleanup-plan.md), aligning the implemented conversion plan's historical current-state wording with the converted workflow.

With the dogfood docs cleanup complete, [Self-Governance Dogfood Hardening Test Plan](docs/implementation-plans/self-governance-dogfood-hardening-test-plan.md) is implemented as a test-only phase covering dogfood cancellation at the planning approval checkpoint, duplicate run-id replay/rehydration behavior, and report-bearing dogfood execution through existing explicit APIs. Real command execution, default handler registration, command-output evidence, side-effect boundary implementation, writes, and nested harness runtime behavior remain deferred.

## P0 Adoption: Agent Harness By Default

User feedback showed that evaluators often begin by hand-writing YAML and manually testing the kernel. The stronger adoption path is to connect Codex, Claude Code, or another coding agent to the local kernel and instruct the agent to use Workflow OS as the governing layer.

The onboarding phase is implemented in [Agent Harness Onboarding Plan](docs/implementation-plans/agent-harness-onboarding-plan.md), [Agent Harness Quickstart](docs/user-guide/agent-harness-quickstart.md), and [AGENTS.md](AGENTS.md). The explicit scaffold command `workflow-os init-agent-harness` is implemented as documented in [Agent Harness CLI Scaffold Plan](docs/implementation-plans/agent-harness-cli-scaffold-plan.md). The scaffold has been dogfooded in [Agent Harness Scaffold Dogfood And Adoption Plan](docs/implementation-plans/agent-harness-scaffold-dogfood-adoption-plan.md). The next adoption maturity layer is planned in [Agent Harness Hook Integration Plan](docs/implementation-plans/agent-harness-hook-integration-plan.md), and the first model-only agent harness hook contract is implemented.

The intended mental model is:

```text
Agent executes. Workflow OS governs.
```

This is a P0 adoption/docs layer, not nested harness runtime behavior. The scaffold command creates or updates `AGENTS.md` and `.workflow-os/agent-harness-prompt.md` only. The scaffold is an orientation layer for humans and agents: useful for declaring conventions, expectations, and structure, but not itself an enforcement layer.

The future hook layer should provide deterministic, named checkpoints that a harness or agent invokes before or after important work phases. The hook contract model is implemented as vocabulary and validation only, and the in-memory invocation helper model is implemented as documented in [Agent Harness Hook Runtime Invocation Plan](docs/implementation-plans/agent-harness-hook-runtime-invocation-plan.md). Hook audit/event semantics planning is documented in [Agent Harness Hook Audit/Event Semantics Plan](docs/implementation-plans/agent-harness-hook-audit-event-semantics-plan.md), and the hook audit record core model is implemented as model-only vocabulary and validation.

WorkReport hook citation target planning is documented in [WorkReport Agent Harness Hook Citation Target Plan](docs/implementation-plans/work-report-hook-citation-target-plan.md), and WorkReport citation vocabulary for agent harness hook invocation IDs is implemented as model-only vocabulary. Terminal report helper hook citation integration is implemented in [Terminal Report Agent Harness Hook Citation Integration Plan](docs/implementation-plans/terminal-report-hook-citation-integration-plan.md) for explicit supplied IDs only. Executor report input propagation for hook IDs is implemented in [Executor Hook Report Input Propagation Plan](docs/implementation-plans/executor-hook-report-input-plan.md). Runtime hook execution planning is documented in [Agent Harness Hook Runtime Execution Plan](docs/implementation-plans/agent-harness-hook-runtime-execution-plan.md), and the explicit in-memory runtime hook execution helper is implemented.

Executor hook checkpoint planning is documented in [Executor Hook Checkpoint Plan](docs/implementation-plans/executor-hook-checkpoint-plan.md), and the explicit `BeforeReport` report-path checkpoint is implemented for `execute_with_report(...)` only. Executor hook event and audit semantics planning is documented in [Executor Hook Event And Audit Semantics Plan](docs/implementation-plans/executor-hook-event-audit-semantics-plan.md); the model-only hook workflow event vocabulary is implemented for bounded, state-preserving future hook events. Generic hook workflow event audit projection is implemented as projection-only in [Hook Event Audit Projection Plan](docs/implementation-plans/hook-event-audit-projection-plan.md), and the first explicit `BeforeSkillInvocation` executor hook event append path is implemented in [Executor Hook Event Append Plan](docs/implementation-plans/executor-hook-event-append-plan.md).

BeforeSkillInvocation status and failure semantics planning is documented in [BeforeSkillInvocation Hook Status And Failure Semantics Plan](docs/implementation-plans/before-skill-hook-status-failure-semantics-plan.md), boundary hardening tests cover later-step targeting, missing handlers, policy denial, and redaction behavior, the first explicit failed-closed result path is implemented as documented in [BeforeSkillInvocation Failed-Closed Result Path Plan](docs/implementation-plans/before-skill-hook-failed-closed-result-plan.md), warning/skipped disclosure semantics planning is documented in [BeforeSkillInvocation Warning And Skipped Disclosure Semantics Plan](docs/implementation-plans/before-skill-hook-warning-skipped-disclosure-plan.md), and unsupported-status hardening tests are implemented in [BeforeSkillInvocation Unsupported Status Hardening Report](docs/concepts/BEFORE_SKILL_HOOK_UNSUPPORTED_STATUS_HARDENING_REPORT.md).

Bounded hook disclosure core model implementation is documented in [Hook Disclosure Model Plan](docs/implementation-plans/hook-disclosure-model-plan.md), WorkReport hook disclosure citation vocabulary is implemented as model-only vocabulary as documented in [WorkReport Hook Disclosure Citation Plan](docs/implementation-plans/work-report-hook-disclosure-citation-plan.md), terminal report helper hook disclosure citation integration is implemented in [Terminal Report Hook Disclosure Citation Integration Plan](docs/implementation-plans/terminal-report-hook-disclosure-citation-integration-plan.md) for explicit supplied IDs only, and executor hook disclosure report input propagation is implemented in [Executor Hook Disclosure Report Input Propagation Plan](docs/implementation-plans/executor-hook-disclosure-report-input-plan.md). Hook disclosure discovery planning and the first in-memory implementation are documented in [Hook Disclosure Discovery Plan](docs/implementation-plans/hook-disclosure-discovery-plan.md); discovery is implemented only for already-validated in-memory `BeforeReport` hook results in the explicit report-bearing executor path. `Passed` remains the only continuing hook status today, while explicit `FailedClosed` fails the run before `SkillInvocationRequested`. Warning/skipped/blocked status broadening, discovery from workflow events or audit projections, dedicated hook audit sink emission, hook persistence, workflow-declared hook configuration, runtime hook configuration, and broader automatic executor checkpoints are not implemented. This does not implement runtime harness auto-generation, workflow schema fields, automatic local check execution, recursive agents, agent swarms, hosted execution, writes, side-effect modeling, or Level 3/4 autonomy, and it must not silently enable command execution, writes, schemas, hosted behavior, or release posture changes.

The first scoped MVP concept is [EvidenceReference](docs/concepts/evidence-reference.md), proposed in [ADR 0009](docs/adr/0009-evidence-reference-core-model.md) with a phased implementation plan in [docs/implementation-plans/evidence-reference-mvp.md](docs/implementation-plans/evidence-reference-mvp.md). EvidenceReference Phase 1 core type model is implemented and reviewed. Adapter telemetry evidence attachment, `Diagnostic` evidence attachment, and selected schema-version validation diagnostic call-site evidence are implemented and reviewed. Broader validation attachment, approval attachment, persistence, CLI, and example attachments remain future scoped work.

The current scoped report foundation has advanced through the `WorkReportContract` core model, `WorkReport` core model, in-memory terminal local report generation helper, in-memory runtime result exposure helper, explicit executor-integrated report-bearing execution for local runs, and an explicit local report artifact store. These phases are documented in [docs/implementation-plans/work-report-contract-plan.md](docs/implementation-plans/work-report-contract-plan.md), [docs/implementation-plans/terminal-local-report-generation-plan.md](docs/implementation-plans/terminal-local-report-generation-plan.md), [docs/implementation-plans/runtime-result-report-exposure-plan.md](docs/implementation-plans/runtime-result-report-exposure-plan.md), [docs/implementation-plans/executor-integrated-report-result-plan.md](docs/implementation-plans/executor-integrated-report-result-plan.md), and [docs/implementation-plans/report-artifact-plan.md](docs/implementation-plans/report-artifact-plan.md). Report/audit/missing-citation semantics are hardened in [docs/implementation-plans/report-audit-missing-citation-semantics-plan.md](docs/implementation-plans/report-audit-missing-citation-semantics-plan.md): reports remain derived governed handoff artifacts rather than audit events, report-generation failures remain separate from workflow results, and absent optional references remain explicit section text instead of fabricated missing citations. Automatic runtime report generation for every run, approval/cancellation report-bearing methods, automatic report artifact writing from executor paths, CLI rendering, schema changes, and examples remain later phases and require separate accepted implementation work.

Workflow OS has begun self-governance dogfooding. The current dogfood slice is [dogfood/workflow-os-self-governance](dogfood/workflow-os-self-governance/README.md): a local, approval-gated, sequential multi-step workflow that uses the kernel as the governing wrapper for Workflow OS planning/docs work. The conversion is documented in [Self-Governance Dogfood Multi-Step Conversion Plan](docs/implementation-plans/self-governance-dogfood-multi-step-conversion-plan.md). This is kernel-governed and Codex-executed. It does not add real build-command skills, automatic code execution, recursive agents, agent swarms, production self-hosting, or Level 3/4 autonomy.

Self-governance should now become a maintained benchmark protocol for building Workflow OS with Workflow OS. Planning is documented in [Self-Governed Build Benchmark Plan](docs/implementation-plans/self-governed-build-benchmark-plan.md) and accepted in [Self-Governed Build Benchmark Plan Review](docs/concepts/SELF_GOVERNED_BUILD_BENCHMARK_PLAN_REVIEW.md). The benchmark framing is: Workflow OS governs its own development loop while agents and maintainers execute the work. The benchmark runbook is implemented in [Self-Governed Build Benchmark](docs/user-guide/self-governed-build-benchmark.md), linked from the dogfood project, and accepted in [Self-Governed Build Benchmark Runbook Review](docs/concepts/SELF_GOVERNED_BUILD_BENCHMARK_RUNBOOK_REVIEW.md). Focused behavior coverage through existing explicit APIs is implemented in [Self-Governed Build Benchmark Behavior Test Report](docs/concepts/SELF_GOVERNED_BUILD_BENCHMARK_BEHAVIOR_TEST_REPORT.md) and accepted in [Self-Governed Build Benchmark Behavior Test Review](docs/concepts/SELF_GOVERNED_BUILD_BENCHMARK_BEHAVIOR_TEST_REVIEW.md). The repo-local `npm run dogfood:benchmark` helper is implemented as development tooling, documented in [Self-Governed Build Benchmark CLI/Dev-Helper Plan](docs/implementation-plans/self-governed-build-benchmark-cli-dev-helper-plan.md), accepted in [Self-Governed Build Benchmark CLI/Dev-Helper Review](docs/concepts/SELF_GOVERNED_BUILD_BENCHMARK_CLI_DEV_HELPER_REVIEW.md), hardened in [Self-Governed Build Benchmark CLI/Dev-Helper Hardening Report](docs/concepts/SELF_GOVERNED_BUILD_BENCHMARK_CLI_DEV_HELPER_HARDENING_REPORT.md), and accepted in [Self-Governed Build Benchmark CLI/Dev-Helper Hardening Review](docs/concepts/SELF_GOVERNED_BUILD_BENCHMARK_CLI_DEV_HELPER_HARDENING_REVIEW.md). It recommends using governed run identity, approvals, validation/check references, hooks, typed handoffs, WorkReports, and report artifacts as those primitives are implemented and reviewed. It does not authorize automatic kernel control of agents, automatic local check execution, arbitrary shell execution, workflow schema changes, repository writes from inside the kernel, recursive agents, agent swarms, hosted execution, production self-hosting, write-capable adapters, or Level 3/4 autonomy claims.

Self-governed validation/check planning is documented in [Self-Governed Validation/Check Plan](docs/implementation-plans/self-governed-validation-check-plan.md). A local validation/check command contract model is implemented with canonical command-template binding, and the first explicit test-only handler for `WorkflowOsValidateDogfood` is implemented and documented in [Test-Only Local Check Handler Plan](docs/implementation-plans/test-only-local-check-handler-plan.md). Broader local check handler planning is documented in [Broader Local Check Handler Plan](docs/implementation-plans/broader-local-check-handler-plan.md), and the first infrastructure slice adds a structured local check result model plus injectable process-runner boundary. The first non-dogfood explicit handler, `DocsCheck`, has advanced to a production-shaped explicit `DocsCheckLocalHandler` while remaining non-default/non-CLI; it is documented in [DocsCheck Local Handler Plan](docs/implementation-plans/docs-check-local-handler-plan.md), [DocsCheck Local Handler Production-Posture Plan](docs/implementation-plans/docs-check-production-posture-plan.md), and [DocsCheck Default-Registration Plan](docs/implementation-plans/docs-check-default-registration-plan.md). An explicit non-default registry helper is implemented for callers that supply a prebuilt `DocsCheckLocalHandler`. [Local Check Handler Default-Registration Plan](docs/implementation-plans/local-check-handler-default-registration-plan.md) implements an explicit non-default registration profile/helper before any ambient default registration. The local-check dogfood lane in [Dogfood Real DocsCheck Plan](docs/implementation-plans/dogfood-real-docs-check-plan.md) is implemented: the self-governance workflow now has an explicit docs-check checkpoint that can run only when a caller supplies `DocsCheckLocalHandler` through explicit profile registration, with injected-runner tests proving the boundary. Local check side-effect/cache/write boundary planning and the model-only boundary are documented in [Local Check Side-Effect Boundary Plan](docs/implementation-plans/local-check-side-effect-boundary-plan.md), and the ignored opt-in live DocsCheck smoke is implemented as documented in [Opt-In Live DocsCheck Smoke Plan](docs/implementation-plans/opt-in-live-docscheck-smoke-plan.md). Local check result citation planning is documented in [Local Check Result Citation Plan](docs/implementation-plans/local-check-result-citation-plan.md), and the first local check result reference model is implemented. WorkReport local check citation target planning is documented in [WorkReport Local Check Result Citation Target Plan](docs/implementation-plans/work-report-local-check-citation-target-plan.md), and WorkReport citation vocabulary for local check results is implemented. Terminal report helper integration for supplied local check result references is implemented and documented in [Terminal Report Local Check Citation Integration Plan](docs/implementation-plans/terminal-report-local-check-citation-integration-plan.md). Command-output evidence policy planning is documented in [Command Output Evidence Policy Plan](docs/implementation-plans/command-output-evidence-policy-plan.md), with command-output evidence attachment explicitly deferred. Evidence attachment, command-output evidence implementation, true default registration, arbitrary shell execution, CLI exposure, automatic check execution, non-ignored live local check execution, live side-effect enforcement, and writes remain future scoped work.

Side-effect boundary modeling must be accepted before policy-gated writes, generic runtime adapter execution, or domain packs. [ADR 0011: Side-Effect Boundary Core Model](docs/adr/0011-side-effect-boundary.md) is accepted as the domain-neutral architecture boundary for side-effect intent, authority, lifecycle state, idempotency, audit, evidence, and report citation. The SideEffect core model is implemented as model-only Rust types and accepted in [SideEffect Core Model Review](docs/concepts/SIDE_EFFECT_CORE_MODEL_REVIEW.md). WorkReport side-effect citation vocabulary is implemented as model-only vocabulary and accepted in [WorkReport SideEffect Citation Review](docs/concepts/WORK_REPORT_SIDE_EFFECT_CITATION_REVIEW.md). Terminal report SideEffect citation propagation is implemented for explicit helper inputs and accepted in [Terminal Report SideEffect Citation Integration Review](docs/concepts/TERMINAL_REPORT_SIDE_EFFECT_CITATION_INTEGRATION_REVIEW.md). Executor SideEffect report input propagation is implemented in [Executor SideEffect Report Input Propagation Report](docs/concepts/EXECUTOR_SIDE_EFFECT_REPORT_INPUT_PROPAGATION_REPORT.md) and accepted in [Executor SideEffect Report Input Propagation Review](docs/concepts/EXECUTOR_SIDE_EFFECT_REPORT_INPUT_PROPAGATION_REVIEW.md). SideEffect workflow event and audit projection planning is documented in [SideEffect Workflow Event And Audit Projection Plan](docs/implementation-plans/side-effect-workflow-event-audit-projection-plan.md), and model-only SideEffect workflow event vocabulary plus bounded generic audit projection are implemented in [SideEffect Workflow Event Model Report](docs/concepts/SIDE_EFFECT_WORKFLOW_EVENT_MODEL_REPORT.md) and accepted in [SideEffect Workflow Event Model Review](docs/concepts/SIDE_EFFECT_WORKFLOW_EVENT_MODEL_REVIEW.md). Executor SideEffect event append planning is documented in [Executor SideEffect Event Append Plan](docs/implementation-plans/executor-side-effect-event-append-plan.md), and the first explicit local proposed/denied/skipped append path is implemented in [Executor SideEffect Event Append Report](docs/concepts/EXECUTOR_SIDE_EFFECT_EVENT_APPEND_REPORT.md). SideEffect persistence and discovery planning is documented in [SideEffect Persistence And Discovery Plan](docs/implementation-plans/side-effect-persistence-discovery-plan.md), the first explicit local `SideEffectRecordStore` persistence slice is implemented in [SideEffect Record Store Report](docs/concepts/SIDE_EFFECT_RECORD_STORE_REPORT.md), and the immutable run identity blocker found in [SideEffect Record Store Review](docs/concepts/SIDE_EFFECT_RECORD_STORE_REVIEW.md) is fixed in [SideEffect Record Store Blocker Fix Report](docs/concepts/SIDE_EFFECT_RECORD_STORE_BLOCKER_FIX_REPORT.md) and accepted in [SideEffect Record Store Blocker Fix Review](docs/concepts/SIDE_EFFECT_RECORD_STORE_BLOCKER_FIX_REVIEW.md). Concrete SideEffect discovery planning is documented in [SideEffect Discovery Plan](docs/implementation-plans/side-effect-discovery-plan.md), the first explicit in-memory discovery helper is implemented in [SideEffect Discovery Helper Report](docs/concepts/SIDE_EFFECT_DISCOVERY_HELPER_REPORT.md) and accepted in [SideEffect Discovery Helper Review](docs/concepts/SIDE_EFFECT_DISCOVERY_HELPER_REVIEW.md), and store-backed discovery is implemented in [SideEffect Store-Backed Discovery Report](docs/concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REPORT.md) following [SideEffect Store-Backed Discovery Plan](docs/implementation-plans/side-effect-store-backed-discovery-plan.md) and accepted in [SideEffect Store-Backed Discovery Review](docs/concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REVIEW.md). WorkReport SideEffect discovery integration planning is documented in [WorkReport SideEffect Discovery Integration Plan](docs/implementation-plans/work-report-side-effect-discovery-integration-plan.md), and the explicit WorkReport-side discovery helper is implemented in [WorkReport SideEffect Discovery Integration Report](docs/concepts/WORK_REPORT_SIDE_EFFECT_DISCOVERY_INTEGRATION_REPORT.md). Executor SideEffect discovery opt-in is implemented in [Executor SideEffect Discovery Opt-In Report](docs/concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REPORT.md), following [Executor SideEffect Discovery Opt-In Plan](docs/implementation-plans/executor-side-effect-discovery-opt-in-plan.md), and accepted with non-blocking follow-ups in [Executor SideEffect Discovery Opt-In Review](docs/concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REVIEW.md). Report artifact SideEffect referential integrity validation is implemented as an explicit helper in [Report Artifact SideEffect Referential Integrity Report](docs/concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REPORT.md), following [Report Artifact SideEffect Referential Integrity Plan](docs/implementation-plans/report-artifact-side-effect-referential-integrity-plan.md), and accepted in [Report Artifact SideEffect Referential Integrity Review](docs/concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REVIEW.md). Approval-side-effect linkage planning is documented in [Approval SideEffect Linkage Plan](docs/implementation-plans/approval-side-effect-linkage-plan.md), and the validation-only helper is implemented in [SideEffect Approval Linkage Report](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_REPORT.md) and accepted with non-blocking follow-ups in [SideEffect Approval Linkage Review](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_REVIEW.md). Approval-side-effect linkage composition planning is documented in [Approval SideEffect Linkage Composition Plan](docs/implementation-plans/approval-side-effect-linkage-composition-plan.md). The explicit store-backed approval linkage helper is accepted in [SideEffect Approval Linkage Store-Backed Review](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_STORE_BACKED_REVIEW.md), and explicit executor report artifact writing with SideEffect integrity and approval-linkage gates is implemented in [Executor Report Artifact SideEffect Gates Report](docs/concepts/EXECUTOR_REPORT_ARTIFACT_SIDE_EFFECT_GATES_REPORT.md) and accepted in [Executor Report Artifact SideEffect Gates Review](docs/concepts/EXECUTOR_REPORT_ARTIFACT_SIDE_EFFECT_GATES_REVIEW.md). This does not implement writes, provider mutations, schemas, CLI behavior, examples, hosted behavior, runtime side-effect execution, EvidenceReference side-effect attachment, automatic executor report discovery, automatic artifact writes from existing executor paths, automatic approval-side-effect validation in existing report/artifact paths, or release posture changes.

## High-Assurance Approval Controls

High-assurance approval controls are a future governance capability, not a current production claim. User feedback has highlighted "nuclear key" style approval workflows as an important mental model: sensitive actions should be impossible unless the required authority, evidence, policy gates, approvals, audit trail, and report disclosures are all present.

Workflow OS already has several prerequisites in place or underway:

- event-sourced approval requests and decisions;
- policy gates before meaningful runtime actions;
- approval expiration metadata;
- denial reasons and fail-closed denial behavior;
- audit and observability records;
- EvidenceReference foundations;
- report and report-artifact foundations;
- sequential governed multi-step execution.

The future roadmap capability should be framed as **high-assurance multi-party approval controls**, not as safety-critical certification. Candidate features include:

- multi-party approval or quorum rules;
- separation of requester and approver;
- role-bound approval authority;
- prevention of self-approval for sensitive actions;
- approval expiry, revocation, and escalation semantics;
- evidence-required approval contexts;
- policy-tested approval chains;
- immutable approval audit trails;
- final work-report disclosure of approvals requested, granted, denied, expired, skipped, or deferred.

This belongs before any serious write-capable adapter work. Write-capable operations should not be introduced until high-risk approvals can be modeled with scoped authority, evidence requirements, durable audit, and deterministic fail-closed behavior.

Non-goals:

- No claim that Workflow OS supports nuclear-grade, medical, aviation, defense, or other safety-critical certification.
- No claim that v0 approvals implement multi-party approval, quorum approval, role-based authority, external identity provider integration, or approval revocation.
- No replacement of deterministic policy and audit with model self-review.
- No write-capable adapter authorization as part of this roadmap note.
- No Level 3/4 autonomy claim.

## Composable Harness Contracts

Composable Harness Contracts are a future governed-work capability, not a v1 requirement. Planning is documented in [Composable Harness Contract Plan](docs/implementation-plans/composable-harness-contract-plan.md), and the core model is implemented. Typed handoff planning is documented in [Typed Handoff Plan](docs/implementation-plans/typed-handoff-plan.md), and the typed handoff core model is implemented. No harness contract or typed handoff runtime behavior is implemented.

Workflow OS should not become agents managing agents. The strategic direction is for Workflow OS to become the governed substrate that makes nested harness work safe, durable, auditable, composable, and useful.

A harness is a bounded, governed execution envelope inside a workflow. It is not synonymous with an agent: a harness may contain an agent, deterministic code, tools, policy checks, validation, or human approval. A future harness contract should define the harness name or ID, purpose, allowed inputs, required context, allowed tools, allowed side effects, output schema, evidence requirements, approval policy, timeout/budget/retry policy, failure semantics, and handoff requirements.

This belongs after the local deterministic kernel and basic governed workflow execution are stable. Nested harness execution depends on earlier primitives:

- workflow and run identity;
- durable state or event log;
- EvidenceReference and evidence-ledger behavior;
- policy gates;
- approval model;
- typed handoffs;
- scoped authority;
- validation;
- terminal work reports.

Roadmap placement:

- Local deterministic kernel: foundational.
- Governed single-run workflows: foundational.
- Core governance primitives: evidence, approval, policy gates, audit records, and work reports.
- Composable Harness Contracts: future contract model for bounded harnesses.
- Nested harness execution patterns: future execution topology after contracts are reviewed.
- Reasoning Lineage / Claim Graph: later provenance layer after evidence, reports, and harness boundaries are understood.

Initial illustrative future pattern: an AI-assisted software engineering workflow could be decomposed into a spec harness, planning harness, implementation harness, test/verification harness, review harness, security/risk harness, and final work report harness. This is illustrative only; it is not an immediate implementation promise and should not imply production nested execution support.

Non-goals:

- No arbitrary recursive agent spawning.
- No agent swarm positioning.
- No claim that Workflow OS currently supports production nested execution.
- No live write integrations as part of this roadmap direction.
- No hosted or distributed runtime claim.
- No Level 3/4 autonomy claim.
- No replacement of deterministic governance with model self-review.

Current planning decisions:

- governed multi-step workflow execution ADR and implementation planning
- remaining EvidenceReference attachment boundaries, including approval evidence and broader validation evidence
- explicit executor/helper artifact-writing planning
- report/audit/missing-citation semantics review
- explicit DocsCheck registry helper before any default production check handler registration
- whether generated report exposure should return report-generation errors separately from workflow results
- how much report structure the runtime should enforce
- how side-effect boundaries should be represented before write-capable adapters
- how future Reasoning Lineage or Claim Graph concepts should relate to governed work

Parallel planning sprint outputs are documented in [Parallel Planning Sprint Report](docs/concepts/PARALLEL_PLANNING_SPRINT_REPORT.md). Typed handoff planning is documented in [Typed Handoff Plan](docs/implementation-plans/typed-handoff-plan.md), and the core model is implemented and reviewed. WorkReport typed handoff citation planning is documented in [WorkReport Typed Handoff Citation Plan](docs/implementation-plans/work-report-typed-handoff-citation-plan.md), and WorkReport typed handoff citation target vocabulary is implemented and reviewed. Terminal report helper typed handoff citation integration is implemented and documented in [Terminal Report Typed Handoff Citation Integration Plan](docs/implementation-plans/terminal-report-typed-handoff-citation-integration-plan.md). Executor-integrated typed handoff report input propagation is implemented in [Executor Typed Handoff Report Input Propagation Plan](docs/implementation-plans/executor-typed-handoff-report-input-plan.md). Report/audit/missing-citation semantics hardening is implemented in [Report, Audit, And Missing-Citation Semantics Plan](docs/implementation-plans/report-audit-missing-citation-semantics-plan.md). Side-effect boundary ADR planning is documented in [Side-Effect Boundary ADR Plan](docs/implementation-plans/side-effect-boundary-adr-plan.md), [ADR 0011: Side-Effect Boundary Core Model](docs/adr/0011-side-effect-boundary.md) is accepted, the SideEffect core model is accepted in [SideEffect Core Model Review](docs/concepts/SIDE_EFFECT_CORE_MODEL_REVIEW.md), WorkReport side-effect citation vocabulary is accepted in [WorkReport SideEffect Citation Review](docs/concepts/WORK_REPORT_SIDE_EFFECT_CITATION_REVIEW.md), terminal report helper SideEffect citation propagation is accepted in [Terminal Report SideEffect Citation Integration Review](docs/concepts/TERMINAL_REPORT_SIDE_EFFECT_CITATION_INTEGRATION_REVIEW.md), executor SideEffect report input propagation is accepted in [Executor SideEffect Report Input Propagation Review](docs/concepts/EXECUTOR_SIDE_EFFECT_REPORT_INPUT_PROPAGATION_REVIEW.md), side-effect workflow event/audit projection planning is documented in [SideEffect Workflow Event And Audit Projection Plan](docs/implementation-plans/side-effect-workflow-event-audit-projection-plan.md), the model-only SideEffect workflow event vocabulary plus bounded generic audit projection are implemented in [SideEffect Workflow Event Model Report](docs/concepts/SIDE_EFFECT_WORKFLOW_EVENT_MODEL_REPORT.md), SideEffect workflow event/audit projection review is accepted in [SideEffect Workflow Event Model Review](docs/concepts/SIDE_EFFECT_WORKFLOW_EVENT_MODEL_REVIEW.md), the first explicit local proposed/denied/skipped SideEffect event append path is implemented in [Executor SideEffect Event Append Report](docs/concepts/EXECUTOR_SIDE_EFFECT_EVENT_APPEND_REPORT.md), SideEffect persistence/discovery planning is documented in [SideEffect Persistence And Discovery Plan](docs/implementation-plans/side-effect-persistence-discovery-plan.md), the first explicit local SideEffect record persistence slice is implemented in [SideEffect Record Store Report](docs/concepts/SIDE_EFFECT_RECORD_STORE_REPORT.md), the immutable run identity blocker is fixed in [SideEffect Record Store Blocker Fix Report](docs/concepts/SIDE_EFFECT_RECORD_STORE_BLOCKER_FIX_REPORT.md) and accepted in [SideEffect Record Store Blocker Fix Review](docs/concepts/SIDE_EFFECT_RECORD_STORE_BLOCKER_FIX_REVIEW.md), concrete discovery planning is documented in [SideEffect Discovery Plan](docs/implementation-plans/side-effect-discovery-plan.md), the first explicit in-memory discovery helper is implemented in [SideEffect Discovery Helper Report](docs/concepts/SIDE_EFFECT_DISCOVERY_HELPER_REPORT.md) and accepted in [SideEffect Discovery Helper Review](docs/concepts/SIDE_EFFECT_DISCOVERY_HELPER_REVIEW.md), store-backed discovery is implemented in [SideEffect Store-Backed Discovery Report](docs/concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REPORT.md) and accepted in [SideEffect Store-Backed Discovery Review](docs/concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REVIEW.md), WorkReport SideEffect discovery integration planning is documented in [WorkReport SideEffect Discovery Integration Plan](docs/implementation-plans/work-report-side-effect-discovery-integration-plan.md), executor SideEffect discovery opt-in is implemented in [Executor SideEffect Discovery Opt-In Report](docs/concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REPORT.md), executor SideEffect discovery opt-in helper review is accepted in [Executor SideEffect Discovery Opt-In Review](docs/concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REVIEW.md), report artifact SideEffect referential integrity validation is implemented as an explicit helper in [Report Artifact SideEffect Referential Integrity Report](docs/concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REPORT.md), following [Report Artifact SideEffect Referential Integrity Plan](docs/implementation-plans/report-artifact-side-effect-referential-integrity-plan.md), and accepted in [Report Artifact SideEffect Referential Integrity Review](docs/concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REVIEW.md), approval-side-effect linkage planning is documented in [Approval SideEffect Linkage Plan](docs/implementation-plans/approval-side-effect-linkage-plan.md), and the validation-only helper is implemented in [SideEffect Approval Linkage Report](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_REPORT.md) and accepted with non-blocking follow-ups in [SideEffect Approval Linkage Review](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_REVIEW.md). Approval-side-effect linkage composition planning is documented in [Approval SideEffect Linkage Composition Plan](docs/implementation-plans/approval-side-effect-linkage-composition-plan.md), and the explicit store-backed helper is implemented in [SideEffect Approval Linkage Store-Backed Report](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_STORE_BACKED_REPORT.md) and accepted in [SideEffect Approval Linkage Store-Backed Review](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_STORE_BACKED_REVIEW.md). Explicit executor report artifact writing with SideEffect integrity and approval-linkage gates is implemented in [Executor Report Artifact SideEffect Gates Report](docs/concepts/EXECUTOR_REPORT_ARTIFACT_SIDE_EFFECT_GATES_REPORT.md). The recommended next phase is write-capable adapter readiness planning, still before runtime side-effect execution or provider mutation.

This milestone must not introduce domain packs, write-capable adapters, or new runtime primitives until a scoped ADR or implementation plan is accepted.

## Reasoning Lineage / Claim Graph Architecture

The [Governed Work Pattern](docs/concepts/governed-work-pattern.md) is accepted as architecture direction, and [Reasoning Lineage / Claim Graph](docs/concepts/reasoning-lineage.md) remains captured as proposed architecture direction in [ADR 0008](docs/adr/0008-reasoning-lineage-claim-graph.md). Reasoning Lineage is a follow-on provenance direction after Governed Work Pattern, and neither direction is implemented as runtime behavior.

Revisit Reasoning Lineage after the EvidenceReference and WorkReportContract foundations are scoped. Revisit these directions together before policy-gated writes, generic runtime adapter execution, or broader domain packs. Implementation of either direction requires a separate accepted ADR or scoped implementation plan.

This milestone should treat reasoning lineage as supporting structure for governed work, not as the primary workflow runtime. Workflow OS must remain a declarative workflow kernel with durable state, policy gates, approvals, auditability, observability, and adapter boundaries.

Candidate decisions:

- how to represent claim or finding nodes
- how to represent derivation edges between claims, evidence, validations, decisions, and reports
- how additive corrections should work without rewriting history
- whether confidence metadata belongs in core, skills, domain packs, or reports
- how actor attribution should attach to generated, reviewed, corrected, or approved claims
- how reference resolution and context binding should connect claims to evidence
- how reasoning lineage should link to evidence references, work reports, audit events, adapter invocation records, validation results, and approval decisions
- what belongs in core versus skills versus domain packs

This milestone must not interrupt Phase 2 live-smoke/public-preview readiness. Implementation of either concept requires a separate accepted ADR or scoped implementation plan.

## Later Production Backend Phase

Production backends are deferred until after local kernel preview release hygiene and adapter readiness criteria are settled.

Future backend work should include:

- Production database contract tests.
- Migration and compatibility strategy for persisted state.
- Backup and restore guidance.
- Corruption detection and repair procedures.
- Locking/fencing semantics.
- Audit persistence and export posture.
- Threat model updates.

## Deferred Until Kernel Correctness And Release Posture

- GitHub write adapters.
- Jira write adapters.
- CI write adapters and additional CI providers.
- Production database backend.
- Distributed workers.
- SaaS control plane.
- UI product.
- Marketplace or package registry.
- High-autonomy external write behavior.
