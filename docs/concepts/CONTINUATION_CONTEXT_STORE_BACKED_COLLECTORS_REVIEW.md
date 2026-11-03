# Continuation Context Store-Backed Collectors Review

## Status

Accepted after focused maintainer/security review and fix-forward test
hardening. Phase C bounded local observers may begin as the next authorized
roadmap increment.

## Scope Reviewed

The review covered the private Phase B collector implementation, its existing
store bindings, source-specific revision commitments, scope checks, typed
unsupported postures, sensitivity and budget behavior, non-consuming read
boundary, focused tests, implementation report, and roadmap claims.

## Findings

### No authority or source-of-truth duplication found

The implementation borrows existing store traits and does not create another
database, projection, policy engine, or permission model. Collector output is
private and non-authorizing. It cannot grant or consume approval, capability,
required context, evidence, or execution authority.

### Unsupported contracts are represented honestly

Capability, required-context, and evidence collection remain typed
`Unsupported`. Their existing interfaces do not yet prove complete,
non-consuming, revision-fenced reads. The implementation does not infer
readiness from partial records or relabel missing contracts as authoritative
empty results.

### Stable-source behavior is conservative

Immutable declarations and policies come from the exact run bundle. Durable
state is reconciled with the append-only event cursor. Approval observations
bind the run projection and presentation records. Checks bind immutable
declarations and typed result events. `SideEffect` and `WorkReport`
observations bind canonically ordered complete record sets. Any changed source
revision rejects collection against the captured request anchor.

### Review assurance gap fixed

The initial focused tests proved event-cursor drift and non-mutation but did
not directly prove durable-snapshot drift or revision stability after reopening
the local backends. The review added both proofs. This was an assurance blocker,
not a production behavior defect.

## Security Assessment

- Store failures become typed, stable, non-leaking failed posture.
- Scope mismatches fail closed.
- Source and revision identifiers remain payload-free.
- Sensitivity above the request ceiling is denied without materialization.
- Budget failure cannot return source claims.
- Empty results receive only the exact Core-owned typed accounting required by
  the accepted model.
- Independently mutable records are canonically ordered before commitment.
- Reopening unchanged local stores produces the same complete source anchor.
- Collection appends no events and creates no side-effect or report records.

## Validation

- focused store-backed collector tests: 5 passed
- strict `workflow-core` clippy with warnings denied: passed with the existing
  narrow dead-code allowance
- full workspace tests: passed
- warnings-denied rustdoc: passed
- formatting: passed
- documentation checks: passed
- diff and whitespace checks: passed

## Governed Review

- workflow: `dg/review`
- run ID: `run-1793706960185028000-2`
- approval ID: `approval/run-1793706960185028000-2/review-scope-approved`
- presentation ID: `presentation/4b3d334c4dbe8c38`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff

## Boundary Preserved

This review did not add stores, Git or operating-document observers,
accepted-intent capture, aggregate composition, runtime continuation, public
surfaces, authority consumption, provider work, nested harness behavior,
Salsa, or hosted behavior.

## Decision

Phase B is accepted. The next authorized implementation increment is Phase C:
one bounded injected Git/worktree observer, exact allowlisted operating-document
observation, and typed-handoff collection from the existing handoff store. That
phase must remain private and non-authorizing and requires its own governed
scope and focused review.
