# Skill Handlers

`SkillHandler` is the local execution interface used by the first minimal executor. It is for deterministic local handlers in tests and development only.

## Purpose

A skill handler connects a parsed `SkillDefinition` to local Rust behavior without introducing a real adapter. It allows the kernel to prove the vertical path:

```text
load -> validate -> run -> persist events -> rehydrate -> complete
```

Handlers are registered in a `LocalSkillRegistry` by skill ID and skill version.

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

The v0 handler path supports a single local step only. It does not implement approvals, retries, escalation, branching, real triggers, adapter calls, or full contract validation.

Tests using handlers prove local kernel behavior only. They must not be presented as proof that production integrations exist.
