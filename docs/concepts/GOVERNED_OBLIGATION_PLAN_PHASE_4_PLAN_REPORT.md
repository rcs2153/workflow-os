# Governed Obligation Plan Phase 4 Planning Report

## Status

The smallest private Phase 4 runtime-composition slice is planned. Runtime
implementation remains unauthorized until focused maintainer/security review
accepts the plan.

## Architecture Decision

The plan uses the existing immutable run bundle as the sole canonical runtime
source boundary. `ImmutableRunBundleBuildResult` already contains typed,
validated workflow, skill, and policy records plus canonical per-step
local-check declaration sets. Its manifest binds those records to the exact
run, resolved execution context, handler and execution posture, and integrity
root.

The future extractor therefore operates on the already-built bundle result. It
does not reload YAML, inspect a mutable checkout, accept a separately assembled
project bundle, or trust caller-authored obligations or decisions. After
persistence, the consumer must read the exact stored bundle, reproduce its
existing binding, and deterministically rederive the same private plan before
delegating to the unchanged executor.

This choice closes the Phase 3 provenance concern without adding a parallel
resolver or truth store.

## Bounded Composition

The proposed slice is crate-private, opt-in, non-serializing, and
non-persistent. It adds:

- one canonical extractor from immutable bundle records;
- one private binding over run identity, immutable bundle root, algorithm,
  source closure, inventory, and count; and
- one private wrapper around the existing immutable-bundle local executor.

Extraction and binding must succeed before the first runtime event. Exact
continuation must rederive from stored immutable records rather than the
current checkout. The initial source and obligation inventories are each
bounded to 256 entries; overflow or truncation fails closed.

## Authority Boundary

The derived plan remains an inspectable requirement index only. It cannot:

- select a policy outcome;
- grant or consume approval;
- establish current authority;
- attest required context or local checks;
- authorize or reconcile a `SideEffect`;
- persist a WorkReport;
- append a runtime event; or
- complete a run.

All existing policy, authority, approval, evidence/check, handler, SideEffect,
event, report, retry, wait, replay, and completion paths remain authoritative.
The accepted Phase 3 decisions remain differential review evidence and are not
promoted into runtime proof.

## Required Review

Focused review must verify:

- the extractor uses the exact immutable bundle closure and no mutable or
  caller-authored substitute;
- the complete supported source closure is represented without manufactured
  requirements;
- cardinality and redaction rules fail closed;
- the private binding cannot be transplanted across runs or bundles;
- no differential fixture or explanation becomes an authorization input;
- no existing runtime gate is bypassed or reimplemented; and
- the proposed two-step implementation sequence is narrow enough to review.

## Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## Governed Phase

- workflow: `dg/d`
- run: `run-1793966509598356000-2`
- approval: `approval/run-1793966509598356000-2/planning-approved`
- presentation: `presentation/64d124e35450aff9`
- approval outcome: granted with persisted presentation proof
- phase status: `Completed`
- event summary: 39 events, one approval, zero retries, zero escalations
- presentation enforcement: `proof_enforced`

Workflow OS governed the planning scope and approval. Repository inspection,
documentation edits, validation, Git, PR, and merge work occur outside the
kernel and remain disclosed. No runtime implementation, provider action,
release, publication, or independent third-party assurance is simulated.

## Next Authorized Phase

Perform one focused maintainer/security review of the Phase 4 plan. Runtime
implementation remains unauthorized until that review accepts the canonical
source extraction, immutable binding, cardinality, provenance, consumption,
fail-closed tests, and sequencing boundaries.

