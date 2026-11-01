# Capability-Gated GitHub Read-Only Core-Private Authority Lifecycle Report

## Status

Phase 2 implementation and blocker correction complete. Focused
maintainer/security re-review accepts the Core-private boundary. Public
fixture-backed activation remains a separate governed phase.

## Implemented Boundary

This phase adds one Core-private composition for the exact
`github.repository.metadata` operation:

- Core derives one run-, step-, harness-, actor-, resource-, and expiry-bound
  `github.read` grant from the validated project declaration and immutable
  execution binding.
- Callers cannot inject authority inventories, capability availability,
  generation, watermark, grant identity, or access values.
- Access presence affects current availability without entering the authority
  source as access material.
- Only `env/WORKFLOW_OS_GITHUB_TOKEN` is accepted; no generic host variable or
  fallback is consulted.
- The access value is non-cloneable, non-serializable, Debug-redacted, and
  borrowed only inside the provider-call closure.
- The exact operation maps success, terminal failure, and
  may-have-started ambiguity into the accepted trusted-host attempt result.
- Preparation delegates authority resolution, source continuity, operational
  window opening, dispatch admission, terminal persistence, replay, and
  ambiguity recovery to the existing atomic trusted-host lifecycle.
- The wrapper reloads the exact immutable bundle and derives the operation,
  literal target, project allowlist, skill input, and operation commitment
  inside Core. Callers cannot supply substitutes for those facts.

The module is compiled only with `trusted-host-application-spi` and remains
crate-private. No public runtime path invokes it.

## Authority Commitment

The Core-derived source commitment binds:

- the complete validated project declaration;
- fixed authored operation and internal capability;
- actor, workflow, run, step, and harness identity;
- exact repository resource;
- sensitivity;
- observation and expiry time; and
- access-presence posture.

It does not bind or serialize the access value. Grant expiry and source
valid-through both use the supplied operational expiry.

## Focused Tests

The focused all-features suite proves:

- exact run/step/harness/resource-scoped authority;
- undeclared-resource rejection;
- available versus declared-not-connected posture;
- caller-independent source generation/watermark derivation;
- exact access-reference parsing and no fallback;
- empty/missing access fails before provider entry;
- closure-scoped value consumption and Debug redaction;
- success, terminal failure, and ambiguity remain distinct; and
- the existing exact repository-read authority and continuity tests remain
  green.

The exact wrapper now has SQLite-backed lifecycle tests proving one-call
success, terminal replay without provider re-entry, may-have-started ambiguity
without blind redispatch, and absence of the access marker from durable bytes.
It composes the accepted supervisor rather than creating a parallel state
machine.

## Review Attention

Focused review found two blockers and the correction closes both. The top-level
helper no longer accepts project configuration, target, or `SkillInput` from
its caller. It reloads the exact stored bundle, validates the complete authored
operation contract including `github.repository.metadata`, resolves the literal
target, verifies it against the immutable project declaration, constructs the
skill input, and supplies a Core-derived operation commitment to the supervisor.
The exact wrapper-specific database fixture proves the composed lifecycle.

## Validation

- `cargo test -p workflow-core --all-features github_repository --lib`: 43
  passed;
- `cargo clippy -p workflow-core --all-features --all-targets -- -D warnings`:
  passed;
- `cargo fmt --all -- --check`: passed;
- default CLI build under the fresh-kernel phase runner: passed after
  feature-gating the private authority re-export;
- documentation and diff checks are required before phase close.

## Governance

- historical preemption run: `run-1793496766383560000-2`, completed and closed;
- resumed workflow: `dg/implement`;
- resumed run: `run-1793523827842712000-2`;
- approval: `approval/run-1793523827842712000-2/implementation-approved`;
- presentation proof: `presentation/8f00ffc35384e90a`;
- blocker correction run: `run-1793524717964440000-2`;
- focused re-review run: `run-1793524741210861000-2`;
- approval outcome: granted under delegated maintainer authority;
- kernel freshness: rebuilt from the exact synchronized checkout before run
  creation.

## Non-Goals

This phase does not add provider network access, public CLI/runtime selection,
schema or SDK exposure, a new mutation family, hidden access discovery, hosted
behavior, release changes, or another provider operation.

## Next Step

Proceed only through a separately governed Phase 3 public fixture activation.
Phase 2 acceptance does not authorize default provider activation, live network
access, mutation, hidden credentials, or broader provider operations.
