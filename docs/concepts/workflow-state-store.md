# Workflow State Store

Workflow OS distinguishes authored workflow contracts from governed execution state.

The boundary is:

```text
Git stores authored contracts.
Workflow OS stores governed execution state.
```

## Why Git Is Not The Database

Git is useful for reviewable, versioned workflow definitions:

- workflow specs;
- skill specs;
- policy specs;
- examples;
- dogfood workflows;
- documentation and implementation plans.

Git is not the right long-term store for runtime and collaboration state because that state changes during execution, may be concurrent across people or agents, and needs to be queried without rewriting source-controlled specs.

## What Belongs In A Workflow OS Store

A future durable Workflow OS store should eventually hold:

- current run state;
- append-only event history;
- approval records;
- policy decisions;
- evidence references;
- work reports and report artifact metadata;
- side-effect records;
- hook records;
- local check records;
- workflow recommendation records;
- workflow catalog lifecycle metadata;
- collaboration and stewardship decisions.

The store should keep raw sensitive payloads out by default. It should prefer stable references, bounded summaries, sensitivity markers, and redaction metadata.

## What Still Belongs In Authored Contracts

Workflow definitions should remain reviewable authored contracts. Source control is a good fit for:

- immutable workflow definitions;
- spec version changes;
- schema changes;
- examples;
- reference dogfood workflows;
- policy and skill declarations.

Once a run starts, it should reference the exact workflow identity, version, and spec hash it was created from. The run should not depend on an implicit latest file in git.

## Dogfood Versus User Workflows

The `dogfood/` directory contains Workflow OS's own self-governance workflows. These are not community defaults.

Portable learning material belongs under `examples/`. User, team, or company workflows should live in their own projects today and should later be managed through a governed workflow catalog/store with explicit ownership, lifecycle, authority, evidence, approval, and report boundaries.

## Current Boundary

In v0, Workflow OS remains a local-first kernel preview. Local file-backed run state exists for the current local execution path, but a database-backed runtime, team collaboration backend, workflow catalog backend, hosted service, and access-control system are not implemented.

The durable store roadmap should start local and conservative:

1. Clarify authored-contract versus runtime-state boundaries.
2. Define local durable store contracts for run/event/report/evidence/approval state.
3. Keep workflow definitions in files/git while storing execution state outside authored specs.
4. Add a local embedded store option for serious single-user dogfooding.
5. Add catalog metadata and conflict models after workflow discovery semantics stabilize.
6. Add team/collaboration backends only after local contracts and privacy boundaries are reviewed.

## Protocol Lessons

The `metric-protocol` repository reinforces a useful distinction for Workflow OS: a file format is not enough to create a governed protocol. A future Workflow OS store should preserve stable authored IDs and immutable content hashes, expose conformance tests for backend implementations, and remain local-first while leaving room for future federation.

This does not mean Workflow OS should adopt a specific database, registry service, signing layer, or hosted topology now. It means future store work should be designed so a backend can prove core behavior such as idempotency, isolation, append-only event handling, report artifact integrity, side-effect record lookup, and catalog lifecycle handling through executable conformance suites.

Signed provenance and federated catalog resolution may become useful later for high-assurance workflow promotion and cross-team collaboration. They remain deferred until local state contracts, report artifacts, approval controls, and catalog governance are reviewed.
