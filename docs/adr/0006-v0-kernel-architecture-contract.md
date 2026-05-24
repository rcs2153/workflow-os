# ADR 0006: v0 Kernel Architecture Contract

## Status

Accepted

## Context

Workflow OS needs precise implementation targets before runtime code is written. Without explicit architecture and runtime invariants, the project could drift into a vague automation framework or integration-first tool.

Existing ADRs establish Rust ownership, local-first kernel priority, stateless workers, event-sourced runs, and Level 1/2 autonomy defaults. The project now needs a single accepted decision that binds v0 implementation to documented kernel contracts.

## Decision

Workflow OS v0 implementation must follow the architecture and runtime contracts documented in:

- `docs/architecture/overview.md`
- `docs/architecture/runtime-invariants.md`
- `docs/runtime/state-machine.md`
- `docs/runtime/execution-semantics.md`
- `docs/architecture/rust-typescript-boundary.md`

The v0 kernel is limited to declarative specs, deterministic validation, local project loading, local execution runtime, event-sourced run state, durable local state, policy-gated execution, approval pause/resume, retry and escalation semantics, audit and observability events, CLI, and a compatible TypeScript SDK.

Real GitHub, Jira, and CI adapters; distributed workers; production database backends; hosted SaaS; UI; marketplace or package registry behavior; and Level 3/4 autonomy by default remain deferred.

## Consequences

- Future implementation prompts have precise invariants to implement against.
- Runtime behavior must be state-machine-driven and event-backed.
- Correctness, policy, idempotency, and auditability take priority over integration breadth.
- Any implementation that violates these documents requires a superseding ADR.
- TypeScript SDK work must prove compatibility with Rust validation rather than creating a parallel model.
