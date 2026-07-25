# Authoritative Local-Check Same-Call Composition Report

## 1. Executive Summary

Workflow OS now has one crate-private Core-owned call that composes canonical
stored local-check declarations, accepted `DocsCheck` execution and freshness
gating, exact structural coverage, and the existing provenance-bearing
aggregate evidence/check fact.

The helper is explicit and unwired. It does not invoke proportional
governance, change executor behavior, or enable automatic checks.

## 2. Scope Completed

- Added `AuthoritativeDocsCheckCompositionInput`.
- Added `AuthoritativeDocsCheckCompositionOutcome`.
- Added
  `compose_authoritative_docs_check_evidence_check_fact(...)`.
- Added complete deterministic batch preflight before process execution.
- Added exact canonical requirement and command-contract matching.
- Added canonical execution ordering by obligation fingerprint.
- Added Core-derived required-versus-optional contribution adaptation.
- Reused the existing execution, attestation, freshness-gate, structural
  coverage, and authoritative aggregate conversion paths.
- Added focused behavior, failure, and privacy tests.

## 3. Scope Explicitly Not Completed

This phase did not add:

- executor wiring or automatic local-check execution;
- proportional-governance invocation or quiet-success enforcement;
- schemas, CLI behavior, onboarding, or examples;
- persistence, events, evidence records, reports, or artifacts;
- providers, OpenShell, SideEffects, or writes;
- hosted or distributed behavior;
- default registration or automatic approvals; or
- release posture changes.

## 4. Helper Boundary

The input contains:

- one validated `StoredImmutableRunBundle`;
- one step identity; and
- an explicit borrowed batch of private
  `DocsCheckAttestationExecutionInput` values.

The output contains:

- bounded `LocalCheckResult` values in canonical execution order; and
- one `AuthoritativeLocalCheckEvidenceCheckFact`.

No adapted leaf contributions, structural coverage candidate, or detached
posture enum is exposed as reusable authority.

## 5. Full-Batch Preflight

Before any process starts, the helper:

1. derives the authoritative candidate from canonical stored declarations;
2. resolves the matching canonical declaration record;
3. verifies stored-bundle, workflow, run, and step equality;
4. verifies exact attestation-requirement identity;
5. validates and fingerprints the handler command contract;
6. compares command identity, kind, and contract fingerprint with the
   canonical declaration;
7. rejects unexpected or duplicate obligation executions; and
8. derives canonical execution order.

A mismatch in a later supplied item prevents an earlier valid item from
starting.

## 6. Execution And Aggregate Semantics

Each preflighted input executes exactly once through
`execute_docs_check_governance_contribution(...)`. That existing path retains
ownership of process observation, attestation verification, gate-time
freshness, and bounded leaf posture.

The authoritative adapter derives requirement level from the canonical
obligation:

- omitted required obligations become `RequiredUnavailable`;
- omitted optional obligations become `OptionalUnavailable`;
- executed failures remain `Failed`, including optional failures; and
- complete passing coverage becomes `Satisfied`.

The final aggregate retains exact counts and candidate, coverage, and fact
fingerprints.

## 7. Failure And Privacy Posture

Preflight errors occur before clock or process use. Execution errors return no
composition outcome or aggregate fact and do not authorize later execution.
Earlier non-source-writing checks may already have run when a later runtime
error occurs; the helper does not claim transactional rollback.

Errors use stable, bounded codes and static messages. `Debug` redacts results
and provenance commitments. Tests verify that output text and runtime
identities do not leak.

## 8. Test Coverage

Focused tests cover:

- successful authoritative composition;
- required and optional omission;
- optional executed failure;
- duplicate obligation input;
- a later batch mismatch preventing all execution;
- unexpected requirement identity;
- execution failure returning no aggregate outcome;
- canonical requirement-level derivation;
- bounded result and `Debug` behavior; and
- regression of all existing runtime attestation and contribution tests.

## 9. Governed Implementation Record

- workflow: `dg/implement`
- run: `run-1785014050795774000-2`
- approval:
  `approval/run-1785014050795774000-2/implementation-approved`
- presentation: `presentation/c30a5efed79eb49d`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- out-of-kernel work: Rust and documentation edits, focused tests, validation
  commands, and report authoring
- missing coverage: the kernel coordinated governance only; it did not execute
  the engineering work or generate a persisted WorkReport artifact

## 10. Validation

Completed successfully:

- focused runtime tests: 24 passed
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

## 11. Remaining Limitations

- The helper is not connected to executor checkpoints.
- Only the accepted private `DocsCheck` family can execute through this path.
- The helper does not invoke proportional governance.
- There is no persistence, event, evidence, report, artifact, CLI, or schema
  exposure.
- Runtime errors after an earlier check cannot roll back the earlier local
  process.

## 12. Recommended Next Phase

Phase-level maintainer review accepts this implementation in
[Authoritative Local-Check Same-Call Composition Review](AUTHORITATIVE_LOCAL_CHECK_SAME_CALL_COMPOSITION_REVIEW.md).

Proceed to planning a private reassessment binding that consumes the aggregate
fact and its fingerprint. Do not skip directly to automatic executor checks or
broader execution providers.
