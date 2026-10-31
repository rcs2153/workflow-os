# Capability-Gated GitHub Read-Only Public Adoption And Recovery Plan Report

## 1. Outcome

Completed the planning phase for the next authoritative roadmap lane.

The plan defines how the existing crate-private capability-gated GitHub
repository-metadata read can become one explicit opt-in public preview path
without weakening immutable-run, policy, approval, current-authority,
continuation, continuity, telemetry, privacy, or no-write guarantees.

Focused maintainer/security review is required before implementation.

## 2. Files Added Or Updated

- Added [Capability-Gated GitHub Read-Only Public Adoption And Recovery
  Plan](../implementation-plans/capability-gated-github-read-only-public-adoption-recovery-plan.md).
- Updated [Roadmap](../../ROADMAP.md) so focused plan review is the next active
  phase.
- Added this planning report.

## 3. Decisions

The plan makes the following concrete decisions:

- Use the ordinary governed `run` path rather than a one-off GitHub utility
  command.
- Require an exact authored `github.repository.metadata` adapter operation.
- Require an explicit project-level GitHub.com activation, exact resource
  allowlist, and exact Workflow OS environment access reference.
- Treat activation as a constraint, never as authority.
- Derive one exact run-scoped grant and current availability through a
  Core-owned local source; do not accept caller-assembled fact slices.
- Migrate dispatch from the current simple consumed idempotency claim to the
  accepted atomic authorized-execution continuity machinery.
- Classify started/no-terminal work as ambiguous and prohibit automatic retry.
- Permit a later fresh read only after explicit ambiguity abandonment and fresh
  authority/governance reassessment.
- Preserve payload-free evidence and disclosure without reusing an inaccurate
  `EvidenceReference` kind.
- Keep normal CI fixture-first and live smoke ignored and separately
  authorized.
- Fix the stale `workflow-os/0.1.0-preview.1` GitHub read-only user agent before
  public live use.

## 4. Alternatives Rejected

- Exposing the private executor helper directly: rejected because its source
  and claim boundaries are not public runtime contracts.
- Treating the project declaration as a reusable capability grant: rejected
  because activation and authority are distinct.
- Reusing the generic `GITHUB_TOKEN` fallback: rejected as ambient access
  discovery for this governed path.
- Adding a provider-specific CLI utility command: rejected because Workflow OS
  governs authored workflows rather than acting as a GitHub command wrapper.
- Retaining the generic idempotency claim and documenting its gap: rejected
  because accepted atomic continuity already exists.
- Automatically retrying the read after started/no-terminal ambiguity:
  rejected because the prior result cannot be authoritatively reconstructed.
- Generalizing immediately to more adapters or operations: rejected because
  one exact vertical slice must be accepted first.

## 5. Contracts Affected By Future Implementation

Planning identifies prospective additive changes to:

- `AdapterRequirement` operation declaration;
- project governance provider-read activation;
- JSON Schema and TypeScript SDK parity;
- immutable run and approval/resume commitments;
- Core-owned current-authority source derivation;
- atomic execution-continuity dispatch and recovery;
- ordinary CLI run, approve, status, inspect, and bounded recovery behavior;
- audit, telemetry, receipt, and WorkReport disclosure; and
- GitHub read-only live-client release identity.

No public contract changed in this planning phase.

## 6. Validation Required By The Plan

The plan defines contract, compatibility, authority, governance, dispatch,
recovery, privacy, reporting, workspace, and optional live-smoke acceptance
matrices. It keeps ordinary CI network-free and requires focused
maintainer/security implementation review before live proof.

Planning-phase validation:

- `npm run check:docs`
- `git diff --check`

## 7. Deferred Scope

No Rust or TypeScript implementation, schema, SDK, CLI behavior, provider
invocation, access material, additional operation, mutation, OpenShell wiring,
hosted behavior, release change, or package publication is included.

## 8. Governed Phase Record

- workflow ID: `dg/d`
- phase: `planning`
- run ID: `run-1793444642837501000-2`
- approval ID:
  `approval/run-1793444642837501000-2/planning-approved`
- approval presentation ID: `presentation/65cac0ca3c34c930`
- approval presentation content hash:
  `65cac0ca3c34c93037b4dbb06f04db062e5f94dd471b78d0f50595e4d3b318d8`
- approval outcome: granted under standing delegated-maintainer authority after
  the complete persisted handoff was presented
- kernel boundary: governance coordination only; source inspection, plan
  authorship, validation, Git, pull-request, and merge work remain outside the
  kernel

## 9. Recommended Next Phase

Perform the focused maintainer/security plan review described in Section 18 of
the implementation plan. Implementation must not begin until that review
accepts the boundary or its required blocker fixes are completed and accepted.
