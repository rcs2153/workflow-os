# ADR 0001: Rust Core And TypeScript SDK

## Status

Accepted

## Context

Workflow OS needs a canonical implementation for workflow specs, validation, runtime state, policy, audit, and observability. It also needs ergonomic developer-facing SDKs for common enterprise environments.

## Decision

Rust owns the canonical core. TypeScript is used for SDK ergonomics and spec generation, but it must remain compatible with the Rust-owned model and must not become a parallel incompatible implementation.

## Consequences

- Core invariants are implemented in Rust first.
- Public TypeScript contracts must be generated from, validated against, or otherwise kept compatible with Rust-owned contracts.
- TypeScript may improve developer experience but must not define conflicting runtime semantics.
