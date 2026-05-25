# Vertical Slice

The first Workflow OS vertical slice is a local approval-gated workflow in `examples/vertical-slice-approval`.

It exists to prove the v0 kernel end to end without introducing real external integrations.

## What It Demonstrates

- Declarative project manifest.
- Workflow definition.
- Skill definition.
- Policy definition.
- Explicitly enabled local deterministic mock skill execution.
- Approval-gated execution.
- Policy checks before meaningful runtime actions.
- Event-sourced run creation.
- Durable local state.
- Audit event emission.
- Observability event emission.
- CLI validation, run, status, approval, and inspection.

## Why The Example Is Generic

The workflow reviews a structured internal business request and produces an approval-gated recommendation. It deliberately avoids GitHub, Jira, CI, pull requests, tickets, or other software-engineering-only concepts.

This keeps the product boundary clear: Workflow OS Core is a generic governed workflow kernel. Domain-specific behavior belongs in future skills, adapters, templates, and examples.

## Mock Boundary

The local skill is deterministic and mock-only. CLI runs must opt into it with `--mock-all-local-skills`, and tests that avoid the CLI register the handler explicitly in `LocalSkillRegistry`. It proves controlled skill invocation through the runtime path. It does not call an AI model or external service.

The runtime behavior around loading, validation, policy, approvals, state, audit, observability, and CLI commands is real v0 behavior.
