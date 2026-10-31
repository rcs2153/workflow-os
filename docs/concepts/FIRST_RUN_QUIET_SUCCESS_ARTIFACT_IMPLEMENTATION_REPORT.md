# First-Run Quiet-Success Artifact Implementation Report

Status: Implemented and accepted after focused maintainer/security review and
one narrow compatibility fix.

## Scope

This phase implements the accepted
[First-Run Quiet-Success Artifact Composition Plan](../implementation-plans/first-run-quiet-success-artifact-composition-plan.md).
It composes the existing bounded first-run analysis, authoritative project
validation, proportional-governance router, durable local executor, and
create-only WorkReport artifact store without weakening the separate
approval/audit demonstration.

Governed implementation identity:

- workflow: `dg/implement`;
- run: `run-1793438862908533000-2`;
- approval: `approval/run-1793438862908533000-2/implementation-approved`;
- presentation: `presentation/04c835717ec31086`; and
- outcome: granted with persisted presentation proof.

## Behavior Added

New authoritative scaffolds contain two distinct contracts:

- `local/first-run-governance` remains the medium-sensitivity,
  approval-gated mock approval/audit demo; and
- `local/first-run-report-artifact` is an exact level-1, low-sensitivity,
  local-only artifact workflow governed by `default/observe-and-report` and
  `local/allow`.

The artifact contract requires the existing no-reuse authoritative
`workflow_os_project_validation` check and declares no human approval minimum,
provider, network, arbitrary command, or external mutation capability.

`workflow-os first-run --write-report-artifact` now:

1. admits only the exact generated artifact workflow, skill, policies, and
   test fingerprints;
2. binds a CLI-private one-use request commitment covering the fixed CLI
   actor, canonical project root, workflow, run, bounded analysis commitment,
   artifact-store root, and operation version;
3. sends current facts through the existing Core-owned authoritative route;
4. persists one create-only WorkReport artifact on terminal proceed;
5. creates no approval events, presentation records, or proof markers on a
   quiet proceed route; and
6. preserves proof-backed approval handling for an escalated route.

An exact low-risk generated project currently selects `quiet_proceed`. The CLI
still prints a concise completion because the operator explicitly selected the
command; this does not relabel the Core disclosure decision.

## Scaffold Compatibility

`init-repo-governance` adds the four new canonical files without requiring
`--force` when the existing generated files remain exact. Existing unmanaged
or changed plain scaffold files still fail closed. Existing unmanaged agent
instructions retain the managed-block preservation behavior.

A new artifact start with an incomplete scaffold fails before runtime state
creation with `cli.first_run.artifact.contract_upgrade_required` and an
additive `workflow-os init-repo-governance` next action. There is no hidden
built-in fallback.

Durable run identity selects the legacy or new contract for resume/recovery.
The old workflow, skill, policy, mock demo, and ordinary non-mutating
`first-run` path remain available and unchanged.

The focused review found that the shared project reload helper still required
the new artifact contract before it inspected durable legacy identity. The
blocker fix makes reload validation identity-aware: an in-flight legacy run
validates only the allowlisted legacy contract, a new artifact run validates
only the complete new contract, and unknown identities fail closed. New starts
still have no legacy fallback.

## Reporting And Audit Truth

The generated WorkReport distinguishes approval-required execution from
proof-free proceed. A quiet run reports that no approval grant was required
and that no approval record was created. Local durable run state and the local
artifact are disclosed separately from external SideEffects, which remain
absent.

Terminal replay returns the existing artifact without rerunning analysis,
checks, or the skill. Missing-artifact recovery requires the immutable request
commitment, recomputes bounded analysis, reruns only canonical project
validation, and cites that fresh result. Metadata drift fails closed.

## Tests

CLI integration coverage proves:

- quiet completion and artifact persistence;
- zero approval requests, approval events, and presentation records;
- exact terminal replay;
- missing-artifact recovery with a fresh local-check citation;
- recovery drift rejection;
- mock-mode and customized-contract rejection;
- incomplete-scaffold failure before state creation;
- additive canonical scaffold upgrade;
- contract-aware pre-upgrade legacy reload while new-contract identity remains
  fail-closed;
- unchanged mock approval/audit demo behavior; and
- updated first-run, catalog, ownership, and metadata projections for both
  generated contracts.

## Boundaries

This phase does not add arbitrary command execution, public schemas or SDK
contracts, providers, network access, credentials, external SideEffects,
OpenShell, hosted behavior, enterprise identity, nested harnesses, report
publication, broader profiles, or release changes.

## Validation

Validation completed or required before review:

- `cargo test -p workflow-cli --test cli`;
- `cargo test --workspace --all-targets`;
- `cargo fmt --all -- --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- pinned Node 20 documentation checks;
- `git diff --check`; and
- a disposable external-repository scaffold, quiet completion, replay,
  recovery, and state-health proof.

