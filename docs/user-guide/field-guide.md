# Workflow OS Field Guide

This field guide rewrites the earlier-stage Workflow OS field guide for the current repository. The original guide remains useful as strategic source material, but it was created before the current implementation and is not current product documentation.

This document is for RC1 internal evaluation. It explains the operating model, current capabilities, and current limits of Workflow OS without claiming production readiness.

In this document, RC1 internal evaluation means a controlled evaluator build for the local kernel and fixture-backed read-only adapter workflows. It does not mean a public release candidate or production release candidate.

This field guide is the user-facing operating-model companion to [ADR 0007: Governed Work Pattern](../adr/0007-governed-work-pattern.md). ADR 0007 is proposed architecture direction; this guide explains how evaluators should reason about applying the operating model without treating that direction as implemented runtime behavior.

## Who This Guide Is For

This guide is for:

- platform engineering teams evaluating the local kernel and project structure
- enterprise architects evaluating product boundary, portability, and future deployment shape
- security reviewers evaluating policy, capability, audit, redaction, and unsupported write boundaries
- AI deployment leads evaluating how governed AI-assisted work could be introduced safely
- design partners and internal evaluators running the current examples and reporting gaps

This is not production operator documentation yet. Production operator documentation would require production backends, distributed worker guidance, live integration evidence, production telemetry/export guidance, and operational runbooks for deployment modes that do not exist in the current build.

## How To Use This Guide

Recommended reading path:

1. Read [Current Truth](#current-truth) first so the implementation boundary is clear.
2. Read [Executive Thesis](#executive-thesis) to understand why Workflow OS exists.
3. Use the [RC1 Evaluation Guide](rc1-evaluation-guide.md) for copy/paste evaluation commands.
4. Use the [Workbook](workbook.md) to qualify and design candidate workflows before writing specs.
5. Review [Security and Governance](#security-and-governance) before testing anything with sensitive data or external systems.
6. Review [Known Limitations](../release/V0_KNOWN_LIMITATIONS.md) before making readiness claims.
7. Report issues with enough evidence for maintainers to reproduce without exposing secrets.

## Minimum Evaluator Prerequisites

RC1 evaluation assumes:

- a local development environment that can run the repository's documented Rust and Node checks
- repository access, including the examples and docs
- ability to run CLI commands from the repository root
- comfort inspecting local command output and redacting anything sensitive before sharing it
- no sensitive or live provider credentials for normal RC1 evaluation
- live provider credentials only for maintainer-owned live smoke tests with approved non-sensitive resources

Normal RC1 evaluation should use the local kernel, the vertical slice, checked-in fixtures, and deterministic mock/example handlers. Do not load GitHub, Jira, or CI credentials unless you are explicitly running the maintainer live smoke procedure.

## Current Truth

| Capability | Current status |
| --- | --- |
| Local kernel preview | Implemented and ready for public local-kernel preview evaluation. |
| Workflow specs, skill specs, policy specs, validation, CLI, local state | Implemented for the v0 local kernel scope. |
| Approval-gated vertical slice | Implemented with explicit deterministic local mock skill execution. |
| Phase 2 GitHub, Jira, and GitHub Actions read-only adapters | Implemented for internal fixture-backed evaluation on the development branch. |
| Adapter telemetry mapping | Implemented for controlled read-only fixture-backed examples as local runtime-visible telemetry. |
| Live GitHub, Jira, and GitHub Actions provider proof | Not recorded yet. Public read-only integration preview remains blocked. |
| Governed Work Pattern | Proposed architecture and product direction only. |
| Reasoning Lineage / Claim Graph | Proposed architecture and product direction only. |
| Work reports and evidence references | Future direction only. |
| Domain packs and pattern catalog | Future direction only. Current examples are reference examples, not a production catalog. |
| GitHub/Jira writes and CI reruns/dispatch/repair loops | Unsupported. |
| Production database backend, distributed workers, hosted service, UI, marketplace | Unsupported. |
| Level 3/4 autonomy | Declaration-only and denied by default. |

## Executive Thesis

Enterprise work is still largely organized as human-driven handoffs. A request is created, interpreted, routed, executed, reviewed, revised, approved, and eventually completed. AI tools can improve individual productivity inside that model, but they do not automatically make the work governed, durable, auditable, or repeatable.

Workflow OS changes the unit of transformation from an individual prompt or isolated task into a governed execution loop:

- structured intake
- declarative workflow and skill definitions
- deterministic validation
- explicit policy
- durable state
- approval gates
- bounded retries and escalation
- audit and observability
- disciplined improvement from evidence

The goal is not simply more AI usage. The goal is a safer operating model where AI-assisted work can advance under constraints authored, approved, observed, and improved by humans.

The dbt Core analogy matters because the desired shift is similar: move important work into versioned, reviewable, testable, documented, and repeatable project artifacts. Workflow OS applies that discipline to governed AI-driven enterprise work.

| dbt-style property | Workflow OS current or intended analogue |
| --- | --- |
| Version-controlled | Workflow OS projects are local files suitable for Git review. Current specs declare schema version, workflow version, ownership, and lifecycle metadata. |
| Testable | The Rust validator, CLI checks, TypeScript contract checks, and fixture-backed integration gate test current supported behavior before execution. |
| Documented | Specs, CLI behavior, runtime invariants, security posture, limitations, examples, and release posture are documented as product contract. |
| Lineage-aware / auditable | Current runs are event-sourced, rehydratable, policy-audited, approval-aware, and inspectable. Future evidence references and reasoning lineage remain proposed direction only. |
| Repeatable | The local kernel can validate and run declared examples repeatedly against local state and fixtures. Production repeatability across distributed workers is future work. |
| Operational | Current operations include local state inspection, audit/observability signals, approval flows, and troubleshooting docs. Production operations and telemetry export are not implemented. |

## Generic Execution Loop

Workflow OS should support a domain-neutral governed execution loop:

```text
Intake -> Context -> Execute/Analyze -> Validate -> Review/Approve -> Complete Safely
                         |                     |
                         v                     v
                    Retry within bounds    Escalate with context
```

The same loop can specialize differently by domain. Software engineering might express it as intake, spec, build, review, validate, and repair. Legal might express it as intake, context gathering, clause analysis, policy validation, approval, and recommendation. Finance might express it as exception intake, evidence collection, threshold analysis, approval, and decision packet.

The core pattern is broader than any one domain: gather context, perform bounded work, validate, pause for human judgment where required, and reach a safe terminal outcome. A safe terminal outcome can be successful completion, an approved exception, cancellation, terminal failure, or escalation with enough context for a human handoff.

## Operating Model Layers

| Layer | Purpose | Current implementation boundary |
| --- | --- | --- |
| Human Operating Layer | Defines intent, ownership, approvals, escalation, and operating review. | Represented through docs, workbook artifacts, approval actors, policy declarations, and CLI flows. |
| Authoring Layer | Turns human operating intent into local project files. | Implemented through `workflow-os.yml`, workflow specs, skill specs, policy specs, tests, CLI validation, and TypeScript spec generation helpers. |
| Validation and Governance | Prevents unsafe or incoherent definitions from running. | Implemented through Rust loading, semantic validation, conservative policy checks, kill switch model, capability checks, and approval gates. |
| Durable Execution Layer | Advances work through stateless execution over durable state. | Implemented locally with event-sourced workflow runs, local filesystem state, rehydration, approval pause/resume, bounded retry, escalation, cancellation, and local skill handlers. |
| Proof and Improvement | Produces evidence for trust, debugging, and operating review. | Implemented through audit and observability sinks, local state inspection, CLI inspect/status, and scoped read-only adapter telemetry for fixture-backed examples. Production telemetry export is not implemented. |

Named principle: runtime workers do not own work. Durable state owns work. Workers advance it.

## Human Control Before, During, And After Execution

Workflow OS does not remove humans from work. It changes where human judgment is applied.

| Stage | Human role | System role | Current artifacts |
| --- | --- | --- | --- |
| Before execution | Define outcomes, risks, ownership, policies, required approvals, autonomy level, and success criteria. | Load project files, validate definitions, reject invalid or unsafe declarations, and enforce conservative policy before starting. | Workbook, project specs, policy specs, validation diagnostics, readiness docs. |
| During execution | Approve gated steps, deny unsafe work, inspect state, cancel where needed, and review escalations. | Advance valid local steps, persist events, enforce policy, pause for approvals, retry within bounds, emit audit/observability records. | CLI `run`, `approve`, `status`, `inspect`, event log, local state, audit/observability records. |
| After execution | Review outcomes, classify failures, tune policies, improve skills, and decide whether more autonomy is justified. | Provide run history, local telemetry, state health checks, adapter telemetry summaries for fixture examples, and known limitation evidence. | CLI inspect, doctor state, audit log docs, metrics docs, readiness reviews. |

This is not informal prompting. It is governed operating design.

## From Workbook To Running Workflow

The workbook is a human authoring layer. It helps teams describe how work should operate before writing specs.

| Workbook artifact | Maps to Workflow OS artifacts | Current boundary |
| --- | --- | --- |
| Workflow Definition Canvas | `WorkflowDefinition`, workflow steps, triggers, state model, terminal behavior. | Implemented in v0 specs and validator. Runtime currently supports a narrow single-step local path. |
| Skill Contract Template | `SkillDefinition`, input/output contracts, sensitive field metadata, failure modes, evaluation criteria. | Implemented as spec validation and local handler contract. Declaring a skill does not implement it. |
| Governance and Policy Matrix | Policy specs, runtime policy decisions, capability requirements, approval policies. | Implemented conservatively. Unknown, unsupported, or unsafe actions fail closed. |
| Retry and Escalation Plan | Retry policy and escalation policy declarations. | Implemented for bounded local skill retries and local escalation state. No external notification exists. |
| Integration and Adapter Map | Symbolic adapter requirements and capabilities. | Read-only GitHub/Jira/CI adapters exist for internal fixture-backed Phase 2 evaluation. Writes are unsupported. |
| Audit and Observability Plan | Audit requirements, observability events, redaction metadata, inspection paths. | Local audit/observability exists. Production SIEM, OpenTelemetry export, and enterprise retention are not implemented. |
| Autonomy Readiness Assessment | `autonomy_level`, approval requirements, policy gates, operational maturity evidence. | Level 1/2 are the default posture. Level 3/4 remain denied by default. |
| RC1 Readiness Assessment | Validation, tests, example runs, limitations, operator handoff. | Assessment artifact only. It is not proof of production readiness. |

The workbook is not a replacement for specs. It is a preparation and review artifact that should make the eventual specs safer and easier to validate.

## Core Vocabulary

Workflow OS needs stable language so operators, developers, reviewers, and future domain teams can reason about the same system.

| Concept | Meaning |
| --- | --- |
| Workflow definition | The declarative process definition: identity, version, triggers, state model, steps, policies, ownership, and declared autonomy. |
| Workflow run | One execution instance bound to schema version, workflow version, and spec content hash. |
| Workflow event | Append-only meaningful state transition or decision record for a run. |
| Skill definition | A bounded capability contract with declared inputs, outputs, constraints, failures, and ownership. |
| Skill invocation | One runtime attempt to execute a bounded local skill handler. |
| Policy decision | A deterministic runtime allow, deny, or approval-required decision with reason codes. |
| Approval decision | A human grant or denial recorded against an event-backed approval request. |
| Audit event | Low-level operational history for reconstruction and accountability. |
| Observability event | Metric-style signal for latency, success/failure, retries, escalation, approvals, policy decisions, and health. |
| Adapter contract | Boundary for external systems. Phase 2 supports read-only adapters internally; writes are not implemented. |
| Autonomy level | Declared operating authority. Higher levels require future explicit policy enablement and evidence. |

## Policy As Executable Control

Policy in Workflow OS is not a meeting after deployment. It is executable control before runtime action.

Current conservative policy behavior includes:

- unknown actions fail closed
- unknown capabilities fail closed
- missing context fails closed
- `external.write` is denied
- `secret.read` is denied unless future explicit configuration exists
- Level 3/4 execution is denied by default
- kill switch blocks new execution and non-terminal mutating actions except safe cancellation/inspection
- approval-gated work pauses before skill invocation

Policy decisions are auditable. Pre-run denied starts are recorded in the durable policy audit ledger without creating misleading workflow runs.

## Security And Governance

Security and governance are not post-processing steps in Workflow OS. They are part of the runtime path and evaluation posture.

Current evaluation rules:

- do not put secrets in specs, fixtures, logs, audit records, telemetry, or docs
- use local examples and fixture-backed adapter examples before any live provider test
- treat provider metadata as sensitive even when it is read-only
- use approvals for sensitive or ambiguous work
- verify policy decisions happen before meaningful actions
- verify unsupported writes fail closed or are unavailable
- inspect run history and local telemetry rather than trusting command success alone
- do not run live smoke tests without approved non-sensitive resources and read-only credentials

See [Security](../security/README.md), [Policy Engine](../runtime/policy-engine.md), and [Phase 2 Public Read-Only Preview Readiness](../integrations/PHASE_2_PUBLIC_READ_ONLY_PREVIEW_READINESS.md) for the current posture.

## Durable Runtime Architecture

Enterprise AI work cannot depend on an agent session, one process uptime window, or local memory. Workflow OS uses stateless workers over durable state as the architectural direction.

The current implementation proves this locally:

- workflow run events are append-only
- run identity includes workflow ID, schema version, workflow version, and spec content hash
- local state can be rehydrated from events
- invalid transitions are rejected before append and during rehydration
- terminal state mutation is rejected
- approval waits survive restart
- duplicate event IDs and duplicate sequences are rejected
- local state health checks report corruption instead of hiding it

The local filesystem backend is for local development and evaluation. It is not a production database, distributed lock service, or multi-host durability guarantee.

## Audit, Observability, And Improvement

Workflow OS should make work measurable, governable, and improvable.

| Signal | Purpose | Current examples |
| --- | --- | --- |
| Structured logs | Operator debugging and correlation. | Runtime metadata and correlation IDs. |
| Audit records | Governance proof and reconstruction. | Policy decisions, approvals, retries, escalation, state changes. |
| Observability events | Improvement and operating signals. | Workflow and skill success/failure, latency, approval decisions, retry exhaustion, escalation. |
| Adapter telemetry | Read-only provider context proof for fixture examples. | Runtime-visible adapter telemetry mapping for controlled GitHub/Jira/CI fixture-backed examples. |

Audit answers who or what acted and under what policy. Observability answers whether the system is operating well. Future work reports would explain what work was performed and what evidence supports the result, but work reports are not implemented.

## Governed Adapters

Adapters are governed boundaries between Workflow OS and systems of record.

Current Phase 2 development-branch adapters are read-only:

- GitHub read-only
- Jira read-only
- CI read-only, with GitHub Actions first

They are ready for internal fixture-backed evaluation. Public read-only integration preview remains blocked until maintainer-owned live smoke evidence is recorded for GitHub, Jira, and GitHub Actions against approved non-sensitive resources.

Adapters must not mutate core workflow state directly. Future external writes must be capability-gated, policy-gated, approval-gated where appropriate, idempotent, audited, and represented through references and summaries. That write-capable future is not implemented.

## Autonomy As Earned Maturity

Autonomy is not a feature toggle. It is an earned operating state.

| Level | Operating mode | Current posture |
| --- | --- | --- |
| Level 1 | Assistive: humans execute or use outputs as guidance. | Allowed by default. |
| Level 2 | Guided: system executes bounded work with human approval gates. | Allowed with approval gates and policy controls. |
| Level 3 | Conditional autonomy inside explicit constraints. | Declaration-only and denied by default. |
| Level 4 | Scaled automation with monitoring and escalation. | Declaration-only and denied by default. |

Teams should not move toward higher autonomy until validation, policy, audit, observability, failure handling, operator control, and measured performance support it.

## Pattern Factory Direction

The original field guide described a pattern factory deployment model: successful governed workflows become reusable patterns that can be adapted, validated, governed, observed, and improved.

That remains a useful future operating model because it prevents every deployment from becoming a bespoke service. A pattern factory should convert successful governed deployments into reusable templates, docs, tests, runbooks, policies, and operating review practices. Over time, those patterns could feed future domain packs for engineering, legal, finance, security, HR, procurement, support, operations, and data/analytics.

For the model to work, every pattern needs:

- a named owner
- validation evidence
- explicit measurement and success criteria
- governance and approval boundaries
- audit and observability expectations
- known limitations and unsupported behaviors
- evidence that the pattern can be validated and operated safely
- an operating runbook or handoff path

That remains future direction. The current repository does not include a production pattern catalog or domain packs.

A workflow pattern is not eligible for future cataloging until it has an explicit owner, validation evidence, known limitations, an operating runbook or handoff path, measurement evidence, and clear governance boundaries. Reference examples in this repository are useful inputs, not catalog entries.

Current examples are reference examples:

- `examples/vertical-slice-approval`
- `examples/github-read-only-review-context`
- `examples/jira-read-only-intake-quality`
- `examples/ci-read-only-failure-summary`

They demonstrate kernel and adapter-contract behavior. They are not production templates, customer deployment packs, or proof of public read-only integration readiness.

## What Success Looks Like In RC1 Testing

An evaluator should consider the RC1 test path successful when they can:

- validate a project and understand any warnings
- run the vertical slice approval workflow
- approve the waiting workflow with actor and reason
- inspect event history, schema version, workflow version, spec hash, approval state, and terminal status
- run the fixture-backed GitHub, Jira, and CI read-only examples if integration evaluation is in scope
- inspect adapter telemetry summaries for fixture-backed examples
- understand which behavior is implemented, internal fixture-backed, future, or unsupported
- report actionable issues with command, commit SHA, expected result, actual result, and redacted evidence

Concrete failure signals include:

- docs leave the evaluator unsure what is implemented
- validation errors are inscrutable or lack useful source context
- a local run cannot be inspected after execution
- approval request, approval grant, or approval denial behavior is confusing
- adapter examples appear to imply writes, live execution by default, or production integration readiness
- secrets, tokens, raw CI logs, raw Jira bodies, or raw private provider payloads appear in output
- known limitations are hard to find or easy to misread

## Evaluation Paths

Use the [RC1 evaluation guide](rc1-evaluation-guide.md) for exact commands.

Recommended sequence:

1. Run docs and baseline checks.
2. Validate and run the vertical slice approval example.
3. Inspect local state, event history, approvals, audit, and observability behavior.
4. Run read-only fixture adapter examples.
5. Inspect adapter telemetry summaries.
6. Confirm live smoke evidence remains missing before making any public read-only integration preview claim.

## Unsupported Or Future

The following are not implemented:

- GitHub write actions: branches, commits, PR creation, comments, reviews, labels, merges, closes.
- Jira write actions: issue updates, comments, transitions, assignments, labels, links.
- CI writes: reruns, dispatch, cancellation, artifact mutation, check mutation, repair loops.
- Webhooks or trigger ingestion service.
- OAuth app flows.
- Hosted service.
- Production database backend.
- Distributed workers.
- UI.
- Marketplace or package registry.
- Domain packs.
- Production pattern catalog.
- Work reports.
- Evidence references.
- Reasoning Lineage / Claim Graph.
- Level 3/4 autonomy enablement.

## Closing Narrative

Providing access to AI can increase individual productivity. Workflow OS is trying to make AI-assisted enterprise work governable: declared before execution, validated before runtime, advanced through durable state, paused where human judgment is required, proven through audit, measured through observability, and improved through repeatable patterns.

That ambition is broader than engineering, but the current implementation is deliberately narrower. The repo today proves a serious local-first kernel, internal fixture-backed read-only adapter work, and a disciplined path toward governed enterprise work. It does not yet prove production deployment, public read-only integrations, write-capable adapters, or high-autonomy execution.
