# Timeouts

Timeout behavior must be explicit in runtime-facing workflow definitions.

## Local Runtime Scope

Workflow and step timeout declarations are parsed and represented in the Rust model. The local executor can classify a workflow timeout policy into:

- maximum duration
- timeout behavior: escalate, fail, or cancel
- timeout failure class

## Deferred Active Timers

The v0 local executor does not run background timers and does not interrupt running local handlers. No distributed scheduler, timer wheel, async worker, or external queue is implemented.

Future active timeout handling must emit explicit events and must not silently abandon a run. Timeout behavior must lead to escalation, terminal failure, or cancellation according to the workflow policy.

## Safety Requirement

When active timeout scheduling is implemented, timeout handling must be durable, auditable, restart-safe, and idempotent.
