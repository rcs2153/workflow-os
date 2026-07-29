# Roadmap Vertical-Slice Acceleration Plan

## 1. Executive Summary

Workflow OS has accumulated substantial, reviewed governance foundations. The
project must now increase delivery size so those foundations become complete
runtime capabilities faster.

Effective after the currently active governed phase closes, the default unit of
roadmap delivery is a runnable end-to-end vertical slice, not an isolated model,
helper, propagation step, planning review, implementation review, or
documentation-only phase.

This change does not weaken governance. Material work still starts and closes
through the dogfood kernel, preserves explicit scope and non-goals, runs
deterministic checks, records approval and event evidence, and produces a final
phase report. The change reduces repeated planning and review ceremonies inside
one already-approved capability boundary.

## 2. Problem

The repository's careful phase discipline built strong semantic foundations,
but the same discipline now creates avoidable latency:

- closely related model, helper, propagation, integration, report, and review
  work is frequently split across separate phases;
- planning documents can become delivery units rather than tools for resolving
  uncertainty;
- accepted primitives remain opt-in and disconnected longer than necessary;
- repeated phase setup and closure can cost more than the bounded
  implementation;
- user-visible and runtime-enforced capability arrives later than the underlying
  architecture permits.

The project should retain its safety posture while changing its work packaging.

## 3. Goals

- Make runnable, testable capability the default output of a roadmap phase.
- Compose already-reviewed primitives into runtime paths sooner.
- Reduce planning-only and helper-only phase count.
- Consolidate implementation, integration, tests, documentation, and review
  within one governed milestone.
- Parallelize independent workstreams without creating conflicting ownership.
- Preserve fail-closed approval, policy, authority, evidence, SideEffect,
  immutable-input, and reporting boundaries.
- Shorten the path to a single-tenant hosted alpha and later collaborative
  operation.

## 4. Non-Goals

- No relaxation of security-sensitive approval or authority checks.
- No removal of deterministic validation or phase-close evidence.
- No silent scope expansion inside an approved phase.
- No automatic provider mutations or hosted claims.
- No bypass of blocker review when a defect can weaken a load-bearing
  invariant.
- No requirement that unrelated roadmap lanes ship together.
- No replacement of the local-first kernel with a hosted-only architecture.

## 5. Default Delivery Unit

One governed phase should normally include:

1. the minimum necessary design decision;
2. model or contract changes;
3. persistence or adapter changes;
4. runtime composition;
5. one real consumer path;
6. failure and privacy behavior;
7. focused and regression tests;
8. documentation truth updates;
9. one consolidated implementation report;
10. one milestone-level maintainer review.

Internal checkpoints may still exist, but they are checkpoints within the same
governed run and branch. They should not automatically become separate roadmap
phases, branches, pull requests, or rounds of planning.

The preferred acceptance test is behavioral:

> Can a caller exercise the new capability through a supported path and inspect
> deterministic evidence of its result?

A model-only or planning-only phase is exceptional. It is justified when:

- the decision changes a public or durable compatibility boundary;
- the work handles authority, credentials, external effects, tenant isolation,
  migration, concurrency, or recovery and uncertainty is still material;
- an ADR is required before implementation;
- independent implementation would create irreversible compatibility cost; or
- a blocker must be corrected before broader work can safely continue.

## 6. Review Policy

Use one consolidated phase review for the complete vertical slice.

Focused interim review remains mandatory for:

- approval or authority bypass risk;
- immutable run-input or time-of-check/time-of-use risk;
- idempotency and ambiguous external-effect recovery;
- durable-state migration, transaction, lease, or corruption behavior;
- credentials, tenant isolation, or secret exposure;
- destructive or externally visible SideEffects;
- schema compatibility and irreversible data migration.

Non-blocking findings should be logged in the milestone report or issue queue.
They should not automatically create a new planning phase.

Documentation-only review should be used only when documentation itself is the
product or safety boundary. Routine truth updates close with their implementation
slice.

## 7. Parallel Execution

Parallel work is encouraged when ownership is non-overlapping.

Typical parallel lanes are:

- core contract and adapter implementation;
- conformance and failure-injection tests;
- API or CLI consumer integration;
- security and privacy review;
- migration and recovery fixtures;
- documentation and operator guidance.

One lane owns each file or module family. Shared interfaces are agreed before
parallel implementation begins. Integration, full validation, and phase closure
remain single authoritative steps.

Sub-agents may investigate or implement independent lanes, but the governed
phase owner remains responsible for scope, conflict resolution, validation, and
the final report.

## 8. Accelerated Build Sequence

### Build A: Operational Embedded Durable State

Deliver one complete local transition from filesystem state to SQLite:

- cooperating root-wide writer guard across mutation paths;
- one atomic staging import transaction;
- canonical import and projection rebuild;
- destination verification;
- explicit activation and rollback posture;
- interruption, corruption, and concurrent-writer tests;
- bounded CLI/operator entry point;
- migration report and maintainer review.

Do not split each helper, model, importer stage, and CLI propagation step into
separate roadmap phases unless a security or correctness blocker requires it.

Status: implemented in the
[Operational Embedded Durable State Report](../concepts/OPERATIONAL_EMBEDDED_DURABLE_STATE_REPORT.md).
The implementation remains opt-in, preserves the filesystem source, keeps
verified staging inactive until exact-receipt activation, and does not
automatically select SQLite for runtime use. Milestone-level maintainer review
is recorded in
[Operational Embedded Durable State Review](../concepts/OPERATIONAL_EMBEDDED_DURABLE_STATE_REVIEW.md).

### Build B: Shared PostgreSQL State

Deliver the first shared durable-state path as one milestone:

- PostgreSQL adapter implementing the Core semantic contract;
- schema migration and compatibility checks;
- transactional mutation families;
- compare-and-set revisions and fenced worker leases;
- concurrent stateless-worker conformance;
- projection rebuild, health, backup, restore, and recovery posture;
- one shared run/catalog consumer path;
- failure-injection and non-leakage tests.

This build establishes shared state. It does not itself claim hosted SaaS,
multi-tenancy, enterprise identity, or production readiness.

### Build C: Single-Tenant Hosted Alpha

Deliver one narrow remote governance service:

- authenticated API for workflow validation, run creation, approval, inspect,
  cancellation, and report retrieval;
- one organization and one administrative trust domain;
- PostgreSQL durable state;
- stateless worker claim, lease, retry, and cancellation behavior;
- explicit execution-provider boundary;
- hosted credential isolation for one reviewed provider path;
- audit, metrics, health, and bounded operational diagnostics;
- deployment and recovery runbook.

The alpha must not claim multi-tenant isolation or enterprise administration.

### Build D: Collaborative Team Beta

Deliver:

- organizations, projects, users, and service actors;
- OIDC-backed identity and scoped roles;
- shared workflow/catalog versioning and promotion;
- approval routing, ownership, escalation, and notification;
- tenant-aware evidence, reports, audit, and credential access;
- conflict-safe collaborative workflow maintenance;
- administrator and steward policy boundaries.

### Build E: Enterprise Hosted Readiness

Deliver:

- verified tenant isolation;
- high availability and disaster recovery;
- audit retention and export posture;
- credential rotation and revocation;
- quotas, rate limits, abuse controls, and operational SLOs;
- enterprise stewardship and policy administration;
- security review and deployment hardening.

## 9. Phase Size And Exit Criteria

A normal implementation phase should target a complete capability spanning
multiple related modules. It should not be made artificially small merely
because the repository previously reviewed those modules separately.

Each accelerated phase must still:

- state approved scope and strict non-goals;
- identify touched ownership surfaces before implementation;
- preserve the complete approval handoff;
- include focused tests at every changed invariant;
- run the repository-required validation suite;
- disclose skipped checks and out-of-kernel work;
- inspect the governed event trail;
- update roadmap and product-boundary documentation;
- close with a structured implementation report and review verdict.

If a milestone cannot produce a supported consumer path, the report must explain
why the remaining split is necessary and name the exact next integration
boundary.

## 10. Immediate Application

The currently active phase keeps its approved scope and must not be widened
mid-run. At its safe close boundary, subsequent filesystem-to-SQLite work should
be regrouped under **Build A: Operational Embedded Durable State**.

The next new roadmap phase should approve the remaining Build A scope as one
integrated implementation milestone. PostgreSQL begins only after Build A's
state transition, verification, activation, and recovery behavior are accepted.

## 11. Expected Effect

This operating change should reduce repeated phase overhead and move the project
from reviewed primitives to exercised runtime composition.

It should not be measured by fewer documents alone. The meaningful measures are:

- elapsed time from accepted design to supported runtime consumer;
- number of roadmap phases per completed capability;
- percentage of phases producing runnable behavior;
- regression and blocker rate;
- evidence completeness at phase close;
- time required to reach shared PostgreSQL state and a single-tenant hosted
  alpha.

## 12. Final Recommendation

Adopt accelerated vertical-slice delivery immediately after the active governed
phase closes.

Keep narrow focused reviews where failure could weaken authority, persistence,
external-effect, migration, concurrency, credential, or tenant boundaries.
Everywhere else, plan once, implement the complete slice, validate it end to
end, and review the finished capability.
