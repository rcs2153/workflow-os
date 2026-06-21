# Metric Protocol Learnings For Workflow OS

This note records transferable architecture lessons from reviewing the private `rcs2153/metric-protocol` repository. It does not import code, change Workflow OS runtime behavior, or authorize a new implementation phase.

## Executive Summary

`metric-protocol` is relevant to Workflow OS because it solves a similar class of problem in a different domain: turning ambiguous, copied, manually interpreted definitions into stable, verifiable, machine-readable governed objects.

The strongest transferable lessons are:

- stable identity should combine human-readable names with immutable content identity;
- governance objects should be referenceable, verifiable, and safe for agents to consume;
- integration surfaces need conformance suites, not only documentation;
- agent-facing tooling should expose machine-readable safety constraints;
- local-first systems should leave room for future federation without requiring a central hosted service;
- signed provenance is useful, but should be staged after core local contracts are stable.

These lessons reinforce the current Workflow OS direction. They do not require Workflow OS to become a metric registry, an MCP-first product, a hosted registry, a distributed runtime, or a write-capable adapter platform in v0.

## Relevant Metric Protocol Patterns

### Stable URN Plus Content Hash

`metric-protocol` separates a readable metric URN from a content-addressed immutable identity. The useful pattern for Workflow OS is not the metric URN format itself, but the pairing:

```text
human-readable contract identity + canonical content hash
```

Workflow OS already has `WorkflowId`, `WorkflowVersion`, `SchemaVersion`, and `SpecContentHash`. The lesson is to preserve this direction as workflow catalog, report artifact, evidence, side-effect, and approval records become more durable.

Future catalog objects should be addressable by stable IDs while retaining immutable content hashes for exact reviewed content. A run should never depend on an implicit latest file after it starts.

### Protocol Before Registry

`metric-protocol` treats file formats as insufficient without identity, verification, registry semantics, and trust posture. Workflow OS has the same boundary:

```text
YAML is an authored contract format.
Workflow OS is the governed runtime and state boundary around that contract.
```

This supports the current roadmap distinction between git-authored workflow definitions and a future Workflow OS store for execution state, recommendations, catalog lifecycle, evidence, reports, approvals, hooks, local checks, and side effects.

### Conformance Suites For Integrations

The vendor integration SDK in `metric-protocol` includes a compliance suite for store behavior. That maps directly to Workflow OS.

Workflow OS should eventually provide conformance suites for:

- `StateBackend` and durable state implementations;
- report artifact stores;
- side-effect record stores;
- adapter capability contracts;
- hook handlers;
- local check handlers;
- future workflow catalog backends.

This is more useful than prose-only adapter documentation because it gives external implementers executable proof that their implementation preserves idempotency, isolation, lifecycle behavior, and failure semantics.

### Machine-Readable Agent Safety Disclosures

`metric-protocol` exposes additivity safety warnings to agents through MCP tools as non-optional machine-readable guidance. The Workflow OS equivalent is not additivity. The equivalent is governance posture:

- authority scope;
- required approvals;
- required evidence;
- side-effect posture;
- unsupported writes;
- local check status;
- hook checkpoint status;
- report completeness;
- workflow/catalog conflict warnings.

Workflow OS should keep moving from prose-only agent instructions toward deterministic, machine-readable safety disclosures that agents can consume before and during work. The current agent harness scaffold is useful orientation, but future hooks, local checks, reports, and catalog recommendations should carry structured safety state.

### Federation As Future Topology

`metric-protocol` uses a federated registry model rather than assuming one central service. Workflow OS should preserve the same strategic option for workflow catalogs.

The v0 kernel should remain local-first. A future team or company catalog should not require immediate SaaS centralization. The durable store and catalog model should be able to grow from:

1. local files and local state;
2. local embedded store;
3. team backend;
4. federated or synchronized catalog boundaries;
5. hosted collaboration only if separately designed.

### Signed Provenance Later

`metric-protocol` uses cryptographic signing and trust levels for metric provenance. Workflow OS will likely need signed provenance eventually for high-assurance workflows, report artifacts, approval records, workflow catalog promotion, and external evidence.

This should remain future work. Workflow OS should not add signing before identity, event semantics, approval controls, report artifacts, and store boundaries are stable.

## Workflow OS Adjustments

The review suggests the following roadmap emphasis:

- Preserve and strengthen content-hash identity for workflow definitions, report artifacts, and future catalog objects.
- Add conformance test suites before inviting third-party stores, adapters, hooks, local checks, or catalog backends.
- Treat machine-readable safety disclosures as a core adoption path for agents.
- Keep future workflow catalog design local-first but federation-ready.
- Defer cryptographic signing until high-assurance approval controls and artifact integrity semantics are more mature.

## What Does Not Transfer

The following `metric-protocol` concepts are domain-specific or premature for Workflow OS:

- metric URN syntax;
- additivity vocabulary;
- Metric Name Service naming;
- RocksDB as a required storage choice;
- semantic-layer compilers;
- BI vendor synchronization;
- metric merge request semantics;
- immediate MCP server positioning;
- immediate cryptographic signing implementation.

Workflow OS should learn from the architecture shape without copying the metric domain.

## Non-Goals

This review does not implement:

- workflow catalog backend;
- database-backed runtime;
- hosted registry;
- federation;
- cryptographic signing;
- MCP server;
- adapter write support;
- workflow schema changes;
- report artifact signing;
- automatic workflow generation or promotion;
- recursive agents, agent swarms, or Level 3/4 autonomy.

## Recommended Follow-Up

No immediate implementation pivot is required. The current near-term focus should remain runtime composition and governance enforcement.

The next time Workflow OS plans durable stores, workflow catalog governance, adapter expansion, or agent-facing tool surfaces, those plans should explicitly include:

- content-addressed identity;
- conformance test suites;
- machine-readable safety disclosure;
- federation-ready but local-first posture;
- signed provenance as a deferred high-assurance capability.
