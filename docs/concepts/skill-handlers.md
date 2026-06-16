# Skill Handlers

`SkillHandler` is the local execution interface used by the first minimal executor. It is for deterministic local handlers in tests and development only.

## Purpose

A skill handler connects a parsed `SkillDefinition` to local Rust behavior without introducing a real adapter. It allows the kernel to prove the vertical path:

```text
load -> validate -> run -> persist events -> rehydrate -> complete
```

Handlers are registered in a `LocalSkillRegistry` by skill ID and skill version.

Declaring a `local/*` skill in a spec is not enough to execute it. A handler must be explicitly registered for the exact skill ID and version. If no handler exists, the runtime fails closed instead of pretending the skill has an implementation.

## Inputs And Outputs

`SkillInput` includes:

- run ID
- workflow ID
- workflow version
- step ID
- skill ID
- skill version
- correlation ID
- non-secret input values

`SkillOutput` includes:

- non-secret output values
- an optional non-secret output reference or summary

Sensitive payloads must not be stored directly in handler output references. Sensitive values should be represented by references according to the redaction and audit rules.

## Boundary

Skill handlers are not adapters. They must not be used to hide real GitHub, Jira, CI, SaaS, shell, network, or external write behavior inside the local executor.

Any future external side effect must go through an adapter boundary with capability checks, policy checks, audit events, idempotency, and durable state transitions.

## Current Limitations

The v0 handler path supports sequential ordered local steps. It participates in approval, bounded retry, escalation, cancellation, policy, audit, and observability semantics that already exist in the local executor, but it does not implement branching, parallel scheduling, real triggers, adapter calls, production plugin loading, or full contract validation.

Tests using handlers prove local kernel behavior only. They must not be presented as proof that production integrations exist.

The CLI option `--mock-all-local-skills` registers deterministic mock handlers for eligible `local/*` skills. Use it only for examples and local smoke tests where the mocked boundary is explicit.

The GitHub, Jira, and CI read-only reference examples also use this flag to register fixture-only handlers for `symbolic/github-read-only`, `symbolic/jira-read-only`, `symbolic/ci-read-only`, and `symbolic/github-actions-read-only`. Those handlers are intentionally narrow: they read local fixture files through read-only adapter contracts and produce non-secret summaries. They do not make arbitrary adapter-backed skills executable, do not call live providers, and do not write to external systems.
