# ADR 0002: Local-First Kernel Before Integrations

## Status

Accepted

## Context

External integrations such as GitHub, Jira, and CI systems introduce real writes, permissions, secrets, retries, rate limits, and audit obligations. Adding them before the core kernel is correct would bias the project toward one-off automation.

## Decision

Workflow OS v0 focuses on the local-first kernel before real integrations. The kernel must establish specs, validation, state, policy, audit, observability, and CLI foundations before adapters are implemented.

## Consequences

- Early examples must not imply production integrations exist.
- Adapter work is deferred until core invariants are enforced.
- The project remains generic across enterprise domains.
