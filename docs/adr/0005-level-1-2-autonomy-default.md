# ADR 0005: Level 1 And Level 2 Autonomy By Default

## Status

Accepted

## Context

AI workflows can range from low-risk assistance to high-impact external writes. Enterprise users need safe defaults and explicit governance before sensitive actions occur.

## Decision

Level 1 and Level 2 autonomy are the default. Level 3 and Level 4 autonomy must be explicitly policy-enabled and must not be default behavior.

## Consequences

- Early workflow behavior emphasizes drafting, validation, recommendations, summaries, and human-reviewed execution.
- Higher-autonomy actions require policy enablement, capability gating, auditability, idempotency, and human approval for sensitive or ambiguous cases.
- Unknown or unsafe actions fail closed.
