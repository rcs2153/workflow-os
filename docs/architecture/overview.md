# Workflow OS v0 Architecture Overview

Workflow OS v0 is the local-first kernel for governed AI workflows. It is deliberately narrow: correctness, determinism, auditability, and policy enforcement come before integration breadth.

This document defines the architectural boundary future implementation must obey. It is a design contract, not an implementation status report.

## Kernel Scope

The v0 kernel includes:

- Declarative workflow specs.
- Declarative skill specs.
- Deterministic validation.
- Local project loading.
- Local execution runtime.
- Event-sourced run model.
- Durable local state backend.
- Policy-gated execution.
- Approval pause and resume.
- Retry and escalation semantics.
- Audit and observability events.
- Evidence reference and work report foundations.
- CLI.
- TypeScript SDK that emits specs compatible with Rust validation.

These capabilities must work together as one local-first system. The kernel must not become a pile of unrelated automation helpers.

## Explicitly Deferred

The following are out of scope for v0 kernel implementation:

- Write-capable GitHub adapter.
- Write-capable Jira adapter.
- Write-capable CI adapter.
- Automatic work-report generation, CLI report rendering, and report schemas.
- Distributed workers.
- Production database backend.
- Hosted SaaS.
- UI.
- Marketplace or package registry.
- Level 3 or Level 4 autonomy by default.

Deferred does not mean unimportant. It means these capabilities must wait until the kernel has durable contracts for specs, validation, state, policy, audit, and execution semantics.

## Architectural Components

### Project Loader

The project loader reads a local Workflow OS project from disk. It discovers workflow specs, skill specs, policy files, and project metadata.

The loader must:

- Read files deterministically.
- Preserve source paths for diagnostics.
- Avoid network access.
- Avoid executing user code while loading.
- Produce canonical inputs for validation.

### Validator

The validator determines whether a project is safe and well-formed enough to execute.

Validation must be deterministic. Given the same files and schema versions, validation must produce the same result.

The validator must check:

- Schema version declarations.
- Required fields.
- Spec identifiers and versions.
- Referenced skills.
- Policy references.
- Approval requirements.
- Unsupported or experimental feature usage.
- Compatibility with the current core schema.

Validation must fail closed for unknown or unsafe constructs.

### Runtime Kernel

The runtime kernel owns run creation, state transitions, policy checks, event emission, retry decisions, approval pauses, escalation decisions, and terminal outcomes.

The runtime kernel must not:

- Depend on process memory for correctness.
- Let adapters mutate runtime state directly.
- Perform external side effects before policy approval.
- Silently drop unsafe, unknown, or ambiguous states.

### Durable Local State

v0 uses a durable local state backend. The backend is local-first but must still behave like a real durable state boundary.

Durable state must store:

- Run records.
- Workflow spec identity for each run.
- Spec content hashes.
- Append-only runtime events.
- Current state snapshots or projections.
- Idempotency records.
- Approval records.
- Retry and escalation metadata.
- Explicit work report artifacts when callers use the artifact store.

The current state must be derived from, or reconciled against, event history.

### Skills

Skills are declared capabilities invoked by workflows. Skill specs describe the contract a workflow expects.

Skill invocation must be:

- Policy-checked before side effects.
- Idempotency-keyed.
- Auditable.
- Retry-aware.
- Bound to explicit inputs and output references.

Mock-only skill behavior must not be presented as production behavior.

### Adapters

Adapters are the only boundary through which external systems may be reached. v0 defines adapter interfaces and includes narrow read-only preview adapters, but write-capable external adapters are deferred.

The `0.2.0-preview.1` posture includes narrow read-only GitHub, Jira, and GitHub Actions adapter previews. Write-capable adapters and generic live adapter execution remain deferred.

Adapters must not:

- Mutate core runtime state directly.
- Bypass policy checks.
- Invent hidden state transitions.
- Hide external writes from audit.

Adapters must report outcomes back through core interfaces that enforce state, policy, audit, and idempotency rules.

### Policy

Policy determines whether execution may proceed, pause, escalate, or fail.

Policy decisions must occur before side effects. Unknown or unsafe actions must fail closed.

### Audit And Observability

Audit and observability are core runtime behavior.

Runtime events and sink interfaces must provide a foundation for:

- Structured logs.
- Audit records.
- Metrics.
- Tracing in future integrations.
- Latency tracking where computable.
- Retry counts.
- Failure counts.
- Escalation counts.
- Approval wait time.
- Stuck workflow detection hooks.

## Execution Shape

A v0 run follows this shape:

1. Load local project files.
2. Validate specs deterministically.
3. Create a run bound to workflow ID, workflow version, schema version, and spec content hash.
4. Append `RunCreated`.
5. Transition through the documented state machine.
6. Check policy before side effects.
7. Invoke skills with idempotency keys.
8. Pause for approvals or external events when required.
9. Retry, escalate, fail, complete, or cancel through explicit transitions.
10. Preserve enough event history for replay, reconciliation, audit, and diagnosis.

## Correctness Over Breadth

Workflow OS v0 must prefer a small correct kernel over broad partial automation.

A feature is not complete unless it has:

- Documented contracts.
- Deterministic validation where applicable.
- Explicit state transitions where applicable.
- Policy behavior where applicable.
- Audit and observability behavior where applicable.
- Meaningful tests where implementation exists.
- Clear disclosure of unsupported or experimental behavior.

## Related Documents

- [Runtime Invariants](runtime-invariants.md)
- [State Machine](../runtime/state-machine.md)
- [Execution Semantics](../runtime/execution-semantics.md)
- [Rust/TypeScript Boundary](rust-typescript-boundary.md)
- [Project Charter](../PROJECT_CHARTER.md)
- [Engineering Standard](../ENGINEERING_STANDARD.md)
