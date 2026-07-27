# Production Current-Authority Source Boundary Plan Report

## 1. Executive Summary

The first production current-authority source boundary is now planned.

The plan keeps source output distinct from trusted authority. A future
Core-owned registration and source invocation boundary must establish source
identity, exact-query completeness, coherent snapshot state, freshness, and
failure posture before same-call resolution can become production behavior.

This planning phase does not implement a source or runtime consumer.

## 2. Scope Completed

- Defined the production source trust root.
- Defined candidate source identity and registration vocabulary.
- Defined exact request derivation from immutable binding and contract.
- Defined fact-family and exact-query completeness semantics.
- Defined snapshot-watermark identity, optional source-defined generation,
  consistency, concurrency, and freshness boundaries.
- Defined valid negative facts versus source failures.
- Defined stable failure and future retry posture.
- Defined aggregate reference-source coordination for the first slice.
- Defined privacy, security, tests, and implementation sequencing.
- Updated roadmap sequencing.

## 3. Scope Explicitly Not Completed

No source model, trait, registry, concrete source, storage, networking,
runtime consumer, readiness API, dereference, persistence, provider,
OpenShell integration, sandbox execution, SideEffect execution, write, schema,
SDK, CLI, UI, example, hosted behavior, lineage, or release change is
implemented.

## 4. Key Architecture Decision

A public response or caller-built fact set cannot establish source trust.

The eventual runtime boundary must instantiate or register a source through
Core-owned configuration and bind every read to:

- source registration commitment;
- immutable execution binding;
- exact query-set commitment;
- coherent snapshot/watermark;
- accepted completeness and consistency; and
- explicit freshness capped by Core-owned policy.

The first implementation remains model-only and cannot confer readiness.

## 5. Snapshot Decision

The first production-shaped model should represent one aggregate source
snapshot covering grants, availability, and governed context references.

Distributed or composite source coordination is deferred. A later coordinator
may commit a source snapshot vector, but it must not silently combine
mixed-time facts.

## 6. Failure Decision

Valid negative facts remain resolver inputs. Operational failures remain
source failures.

The model distinguishes unavailable, unsupported, incomplete, stale,
future-dated, concurrent-change, ambiguous, corrupt, registration-mismatch,
query-mismatch, transport, and internal failure posture without retaining raw
error payloads.

## 7. Product Alignment

Fresh-pull evaluation says Workflow OS should reduce ceremony for low-risk
work while preserving evidence.

This source boundary is required for that outcome. Quiet success may only
consume authority that Core obtained from an accepted source at the current
decision boundary. Unknown or stale source state can increase friction or
deny, never lower it.

## 8. OpenShell Boundary

OpenShell remains a future optional execution provider. It can enforce
filesystem, network, process, inference, and credential containment after
Workflow OS governance.

It is not the source of Workflow OS authority and is not implemented by this
plan.

## 9. Validation

- `npm run check:docs`: passed with the repository Node 20 toolchain.
- `git diff --check`: passed.
- Claims were checked against the accepted fact-set, private source,
  same-call resolver, capability, context-projection, and required-context
  boundaries.

## 10. Remaining Limitations

- No Core-owned trusted registration exists.
- No production source contract or implementation exists.
- No source freshness policy is selected.
- No trusted prerequisite fact sources exist.
- No source-backed same-call assessment exists.
- No runtime consumer or dereference boundary exists.

## 11. Recommended Next Phase

Focused maintainer review is complete in the
[Production Current-Authority Source Boundary Plan Review](PRODUCTION_CURRENT_AUTHORITY_SOURCE_BOUNDARY_PLAN_REVIEW.md).

The review accepts model-only implementation after correcting watermark and
freshness semantics. Implement the production current-authority
source-boundary core model only.

## 12. Dogfood Governance

- workflow: `dg/d`
- run ID: `run-1785157209067702000-2`
- approval ID:
  `approval/run-1785157209067702000-2/planning-approved`
- presentation ID: `presentation/0fe32774073d3d76`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted planning handoff was presented
- phase status: completed
- event summary: 39 events, one approval, zero retries, zero escalations;
  approval-presentation proof was persisted and enforced
- validation summary: documentation and diff integrity checks passed
- out-of-kernel work: architecture inspection, planning, documentation edits,
  and validation were performed by the delegated maintainer; the kernel
  governed scope and approval but did not inspect code, edit files, execute
  checks, or mutate git
