# Fresh-Checkout Aggregate Check P1 Report

## Status

Implemented and accepted after focused owner review.

## Finding

Independent fresh-main evaluation found that the advertised top-level
validation command was not self-contained. After `npm ci` and a release-only
Rust build, `npm run check` reached the dogfood helper suite without a
`target/debug/workflow-os` binary. Six tests then failed closed with
`dogfood.helper.binary_missing`. Running a workspace test command happened to
create the binary and made the identical npm command pass, but that inherited
artifact was not part of the command contract.

This was a P1 reproducibility and evaluator-confidence defect, not a kernel
runtime failure.

## Implemented Boundary

The aggregate command now starts with:

```text
npm run build:check-kernel
```

That named prerequisite performs exactly:

```text
cargo build -p workflow-cli --bin workflow-os
```

before documentation, dogfood helper, integration-helper, release-package,
candidate-workflow, release-state, TypeScript, or contract checks begin.

Focused JavaScript coverage requires the named prerequisite to remain bound to
the exact CLI build and requires `npm run check` to invoke it before the
dogfood suite. A dedicated `Fresh Checkout Aggregate Check` CI job installs
locked JavaScript dependencies, fetches complete Git history required by the
release-state ancestry validator, proves the debug binary is absent, runs the
advertised aggregate command, and proves the binary exists afterward.

## Governance Evidence

- workflow: `dg/implement`
- run: `run-1793831056485445000-2`
- approval: `approval/run-1793831056485445000-2/implementation-approved`
- presentation: `presentation/ddb7f10ed80850fd`
- approval outcome: granted with persisted presentation proof

Focused review used:

- workflow: `dg/review`
- run: `run-1793831256513063000-2`
- approval: `approval/run-1793831256513063000-2/review-scope-approved`
- presentation: `presentation/1bff7339db09a60c`
- approval outcome: granted with persisted presentation proof

The review found no blocking defect. It confirmed that the build prerequisite
precedes the dependent suite, the clean-checkout CI proof starts without the
debug binary, and the implementation does not widen runtime or release
authority. This is an owner-conducted repository review, not independent
assurance.

Exact-head CI then exposed one review blocker: checkout's default depth of one
could not prove the release-state snapshot's historical merge commits were
ancestors of the pull-request merge ref. The correction keeps release-state
validation fail-closed and gives this clean checkout complete Git history. The
focused test now locks that prerequisite as part of the advertised aggregate
check contract.

## Validation Contract

Required validation for this phase is:

- focused dogfood helper tests;
- `npm run check` after removing only the debug CLI artifact;
- documentation checks;
- integration checks;
- CI workflow YAML parsing;
- `git diff --check`.

The clean-checkout CI proof is intentionally redundant with local validation:
it prevents future aggregate-command changes from silently depending on an
artifact created by another job or a maintainer's prior Rust command.

## Non-Goals

This phase does not change:

- Workflow OS runtime semantics;
- governed-phase kernel freshness behavior;
- approval, policy, durable state, or report behavior;
- provider or adapter behavior;
- release-candidate workflow behavior;
- dependency versions;
- schemas or public contracts;
- release, tag, registry, publication, or announcement posture.

The separately governed `v0.4.0-preview.1` candidate dispatch remains blocked
on repository-scoped GitHub workflow-start access and is not reclassified as
successful by this implementation.
