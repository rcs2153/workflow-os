# ADR 0003: Stateless Workers Over Durable State

## Status

Accepted

## Context

Enterprise workflow execution must tolerate worker crashes, restarts, horizontal scaling, duplicate triggers, and operational recovery.

## Decision

Runtime workers will be designed as stateless workers operating over durable workflow state. Durable state is externalized through interfaces, and meaningful state transitions are recorded as auditable events.

## Consequences

- Workflow correctness must not depend on process memory.
- Workers must be restart-safe.
- Current state must be derivable from, or reconcilable against, the event log.
- Durable state interfaces must be designed before production runtime behavior.
