# Skills

A Workflow OS skill is a versioned declarative capability that workflows can reference.

In v0, a skill definition describes contracts, required capabilities, adapter requirements, failure modes, evaluation criteria, retry compatibility, approval sensitivity, audit requirements, and observability requirements. It does not execute code or perform side effects.

## Contracts

Skills define input and output contracts with schema-like fields. Contracts are intentionally structured so future validation, documentation generation, and SDK generation can work from the same Rust-owned model.

Contract fields may be marked sensitive. Sensitive data should be redacted, summarized, or referenced rather than stored as raw payloads.

## Capability Boundary

`allowed_capabilities` describes what the skill may request. Future policy and runtime layers must enforce whether those capabilities are allowed before any side effect occurs.

`adapter_requirements` describe adapter needs without implementing adapters. A skill may need an adapter boundary, but adapters must not mutate core state directly.

## Generic Domain Model

Skills must remain generic across enterprise domains. A skill may be used for legal review, support triage, finance approval, software delivery, or other enterprise work without the core model changing.

Concrete integration details belong in future adapters and project-specific specs.
