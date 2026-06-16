# ADR 0010: Governed Multi-Step Workflow Execution

## Status

Accepted

## Acceptance Note

Accepted as P0 architecture and product direction after review in [docs/concepts/GOVERNED_MULTI_STEP_WORKFLOW_EXECUTION_ADR_REVIEW.md](../concepts/GOVERNED_MULTI_STEP_WORKFLOW_EXECUTION_ADR_REVIEW.md).

Acceptance did not itself implement multi-step execution, add schemas, change CLI behavior, add examples, authorize writes, introduce hosted or distributed runtime behavior, enable reasoning lineage, or implement nested harness execution.

The first implementation must follow [Governed Multi-Step Workflow Execution Plan](../implementation-plans/governed-multi-step-workflow-execution-plan.md): sequential local execution only, preserving existing single-step behavior and keeping branching, parallelism, side effects, writes, nested harness runtime behavior, and automatic report generation out of scope.

Implementation note: the first sequential local executor slice is implemented and reviewed in [Governed Multi-Step Workflow Execution Review](../concepts/GOVERNED_MULTI_STEP_WORKFLOW_EXECUTION_REVIEW.md). Branching, parallelism, nested harness runtime behavior, writes, schemas, examples, CLI changes, and automatic report generation remain out of scope.

## Context

Workflow OS has implemented and reviewed the local-first kernel foundation, Governed Work Pattern direction, EvidenceReference core model and selected attachments, WorkReportContract and WorkReport models, terminal local report helpers, executor-integrated report-bearing local execution, report artifact storage, Composable Harness Contract core model, typed handoff core model, and selected self-governance dogfooding slices.

The local executor remains intentionally narrow. It validates a project, creates a run, schedules ordered local steps, evaluates policy per step, invokes registered local skill handlers, supports approval pause/resume around the current step, handles retry/escalation/cancellation, persists events, emits audit/observability records, and can return report-bearing results. Branching, parallelism, nested harness runtime behavior, writes, and automatic report generation are not implemented in v0 local execution.

Kernel dogfooding has now exposed a product blocker: one-governance-check workflows do not scale. The kernel can govern a single checkpoint, but real Workflow OS work quickly requires multiple governed steps with explicit ordering, state transfer, policy gates, approvals, evidence, validation, failure handling, and final reportability. A framework like Workflow OS becomes more valuable as work decomposes into a sequence of governed actions, not as a wrapper around one isolated check.

This makes **Governed Multi-Step Workflow Execution** a P0 blocker for the kernel roadmap. The blocker is not that Workflow OS needs recursive agents, agent swarms, or arbitrary multi-agent orchestration. The blocker is that the governed runtime must safely execute authored workflows containing more than one step while preserving deterministic state transitions, scoped authority, evidence, auditability, and handoff semantics.

Workflow specs already model ordered `steps`. The first local executor implementation only supported a single step; that mismatch was acceptable for an early local preview, but became the primary kernel limitation for dogfooding and for proving Governed Work Pattern at useful scale.

## Decision

Workflow OS will treat Governed Multi-Step Workflow Execution as a P0 kernel capability to design and implement through follow-on scoped plans. The first bounded implementation path is documented in [Governed Multi-Step Workflow Execution Plan](../implementation-plans/governed-multi-step-workflow-execution-plan.md).

The runtime direction is:

- A workflow may contain multiple authored steps.
- The local deterministic kernel must execute those steps in a governed sequence.
- Each step boundary must be explicit, auditable, and reconstructable from durable events.
- Each step may require policy evaluation, approval handling, validation, retry/escalation behavior, evidence references, typed handoffs, and report citations.
- Step outputs and handoffs must be typed, bounded, and safe to pass to later steps.
- Failure, cancellation, retry exhaustion, approval denial, and escalation semantics must be deterministic and must not silently skip required governance.
- Final work reports must be able to cite the multi-step work performed, evidence considered, decisions made, validation/checks run, handoffs, known limitations, risks, and incomplete/deferred work.

This ADR accepts the roadmap priority and architectural direction only. It does not implement multi-step execution, add schemas, change existing executor behavior, add CLI behavior, add examples, authorize writes, introduce hosted/distributed runtime behavior, or enable automatic agent orchestration. Implementation requires separately scoped plans, tests, and reviews.

## Scope

The future Governed Multi-Step Workflow Execution work should define and implement, in small phases:

- step sequencing for authored workflow steps;
- deterministic step scheduling and completion semantics;
- step-level event model behavior for scheduled, started, completed, failed, skipped, retried, escalated, canceled, and approval-gated states where applicable;
- policy-gate evaluation before each meaningful step action;
- approval pause/resume behavior across multiple steps;
- bounded retry and escalation behavior across multiple steps;
- typed handoff of step outputs to later steps;
- source-of-truth boundaries between workflow events, snapshots, audit records, evidence references, typed handoffs, and work reports;
- report citation behavior for multi-step runs;
- compatibility with existing single-step local executor behavior during migration.

The first implementation should be local, deterministic, and conservative. It should prefer sequential execution before any branching or parallelism. It should preserve existing single-step behavior and tests.

## Non-Goals

This ADR does not authorize:

- arbitrary recursive agent spawning;
- agent swarm positioning;
- agents managing agents;
- automatic agent orchestration;
- hosted or distributed runtime execution;
- production nested harness execution;
- live write integrations;
- side-effect boundary implementation;
- external write behavior;
- schema changes;
- CLI behavior;
- example updates;
- Reasoning Lineage / Claim Graph implementation;
- Level 3/4 autonomy enablement;
- replacement of deterministic governance with model self-review;
- parallel step execution;
- conditional branch execution;
- generic adapter execution;
- report artifact auto-writing from executor paths.

## Required Primitives And Dependencies

Governed multi-step execution depends on the primitives Workflow OS has been building, and it should not bypass them:

- immutable workflow and run identity;
- schema version, workflow version, and spec content hash binding;
- durable state backend and append-only event log;
- deterministic validation;
- state projection from events;
- idempotency keys for step and skill invocation boundaries;
- policy gates before meaningful actions;
- approval model for sensitive or ambiguous step execution;
- audit records projected from workflow events;
- observability records for runtime behavior;
- EvidenceReference for evidence citations;
- typed handoff model for step-to-step transfer;
- WorkReportContract and WorkReport models for final governed handoff artifacts;
- redaction and sensitivity handling;
- conservative error handling that fails closed without leaking secrets.

Before implementation, maintainers should explicitly decide:

- whether existing `StepScheduled` and skill invocation events are sufficient or whether additional step-completion/handoff events are required;
- how step output references and typed handoffs are persisted or reconstructed;
- how approval, retry, escalation, and cancellation interact with the current step index;
- how idempotency keys are derived for each step attempt;
- how final report generation cites multi-step evidence and handoffs without copying raw payloads;
- how to preserve single-step compatibility while expanding the executor.

## Relationship To Future Composable Harness Contracts

Governed Multi-Step Workflow Execution is a prerequisite for future Composable Harness Contracts becoming useful runtime behavior.

Composable Harness Contracts describe bounded execution envelopes inside a workflow. They are not recursive agents, agent swarms, or arbitrary multi-agent orchestration. A harness may contain deterministic code, an agent, tools, policy checks, validation, or human approval, but its value comes from the contract boundary: typed inputs, typed outputs, scoped authority, evidence requirements, approval rules, failure semantics, and traceable handoffs.

The runtime should learn to execute multiple governed workflow steps before it attempts nested harness execution patterns. Multi-step execution proves the kernel can sequence governed work, preserve state, enforce policy at each boundary, pass typed handoffs safely, and produce a final report across more than one step. Only after those basics are stable should Workflow OS consider runtime execution of composable harnesses.

The intended layering remains:

1. Local deterministic kernel.
2. Governed single-run workflows.
3. Core governance primitives: evidence, approval, policy gates, audit records, typed handoffs, and work reports.
4. Governed multi-step workflow execution.
5. Composable Harness Contracts as contract boundaries.
6. Nested harness execution patterns.
7. Reasoning Lineage / Claim Graph as a later provenance layer, if separately accepted and scoped.

## Consequences

Positive consequences:

- Workflow OS addresses the main dogfooding blocker exposed by real kernel use.
- The runtime becomes useful for governed work that requires more than one policy or validation checkpoint.
- The project can scale from one-step demonstrations to real governed workflows without jumping to agent swarms or hosted orchestration.
- Typed handoffs, evidence references, audit projections, policy gates, approvals, and work reports gain a practical multi-step execution surface.
- Future Composable Harness Contracts get a safer foundation.

Tradeoffs:

- Multi-step execution increases runtime complexity around event sequencing, idempotency, retries, approvals, escalation, cancellation, and report generation.
- Step-to-step handoff semantics can create context drift if they rely on untyped natural-language summaries.
- Poorly scoped multi-step execution could accidentally become a generic orchestration engine.
- Parallelism, branching, writes, and nested harnesses must remain deferred until sequential governed execution is correct.
- Tests must expand from return-value checks to event-history, snapshot, audit, report, and redaction behavior across multiple steps.

## Implementation Timing

Treat this as the next P0 kernel roadmap blocker.

The next phase should be a scoped implementation plan for sequential local governed multi-step execution. That plan should define the smallest first slice, likely:

- execute two or more sequential local steps;
- preserve current single-step behavior;
- emit deterministic events for each step boundary;
- enforce policy before each step invocation;
- pass bounded typed outputs/handoffs between steps;
- stop safely on failure, approval wait, escalation, or cancellation;
- keep report generation in-memory and citation-based;
- add focused runtime correctness tests.

Do not implement branching, parallel execution, nested harness runtime behavior, writes, schemas, CLI behavior, examples, or reasoning lineage in the first slice.

## Explicit Implementation Statement

No runtime feature is implemented by this ADR. No schema, CLI behavior, example, write capability, hosted runtime, distributed worker, side-effect model, reasoning lineage model, or nested harness execution behavior is added. Governed Multi-Step Workflow Execution is proposed as P0 architecture direction and requires separately scoped implementation work.
