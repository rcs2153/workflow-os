# ADR 0004: Event-Sourced Run Model

## Status

Accepted

## Context

Workflow runs need auditable state transitions, replay and reconciliation support, duplicate event handling, and clear failure diagnosis.

## Decision

Workflow OS will model meaningful run transitions as append-only events. Current state must be derivable from, or reconcilable against, the event log.

## Consequences

- State transitions must be explicit.
- Events must include enough context for audit and diagnosis.
- Runtime updates must preserve idempotency and safe retry behavior.
- No workflow may silently terminate in an unsafe or ambiguous state.
