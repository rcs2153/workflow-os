# Capability-Gated GitHub Read-Only Fixture Activation Report

## Status

Phase 3 implementation and focused maintainer/security review are complete.
Review accepted the boundary after correcting denial so it remains available
when executable fixture context is missing or changed.
The exact `github.repository.metadata` operation is publicly selectable through
ordinary `run`, `approve`, `status`, and `inspect` CLI commands in fixture mode.

## Implemented Boundary

This phase adds one fixture-only vertical slice over the accepted Phase 2
atomic provider lifecycle:

- an exact one-step workflow and skill contract select
  `symbolic/github-read-only` plus `github.repository.metadata`;
- the validated project declaration supplies the exact allowed repository;
- the fixture payload hash is folded into the immutable resolved execution
  context so payload drift before approval or execution fails closed;
- an authored approval pauses the run before provider admission;
- the CLI persists a complete approval-presentation record and Core requires
  matching, fresh durable proof before accepting the decision;
- approval records only the decision and resume state; a later `run` performs
  fresh context validation and provider admission;
- fixture mode never resolves, reads, or accepts access material and never
  opens live transport;
- the existing atomic supervisor owns window opening, dispatch admission,
  attempt execution, terminal persistence, ambiguity posture, and replay;
- bounded adapter audit and observability records are persisted in the same
  dedicated SQLite provider state; and
- `status` and `inspect` resolve the provider state and disclose operation,
  fixture mode, disabled live transport, terminal report posture, and bounded
  recovery posture.

## Public Example

`examples/github-repository-metadata-fixture` is the canonical public example.
It includes the exact project activation, workflow, skill, policies, and
repository fixture. It is explicitly documented as fixture-only and no-write.

```text
workflow-os run example/github-repository-metadata-fixture
workflow-os approve <run-id> <approval-id> --actor user/<approver> --reason <reason>
workflow-os run example/github-repository-metadata-fixture --run-id <run-id>
workflow-os status <run-id>
workflow-os inspect <run-id>
```

## Integrity And Replay

The run bundle binds the exact workflow, skill, policies, project declaration,
resource, and fixture-content commitment. Approval reconstruction and every
later dispatch reproduce that commitment. A changed fixture is rejected before
an approval event or provider entry.

An already-terminal run returns `terminal_replay` without another adapter call.
A may-have-started posture remains `recovery_required`; automatic retry is
prohibited and inspect directs the operator to explicit reassessment.

## Validation

- focused public CLI fixture lifecycle tests pass;
- fixture drift after presentation fails closed;
- proof-backed denial remains available after fixture removal and produces no
  provider invocation or adapter telemetry;
- adapter telemetry is visible through provider-backed inspect;
- formatting and warnings-denied Clippy pass for Core and CLI;
- full workspace Rust, repository, integration, documentation, and diff
  validation pass under the repository's pinned toolchain.

## Governance

- workflow: `dg/runtime-composition`;
- run: `run-1793526623089151000-2`;
- approval: `approval/run-1793526623089151000-2/composition-approved`;
- presentation proof: `presentation/8f8a3c41cf9e664e`;
- approval outcome: granted under delegated maintainer authority;
- implementation phase close: completed with 39 events, one approval, zero
  retries, and zero escalations;
- focused review workflow: `dg/review`;
- focused review run: `run-1793528082632812000-2`;
- focused review approval:
  `approval/run-1793528082632812000-2/review-scope-approved`;
- focused review presentation proof: `presentation/8babdf2a66813ee4`; and
- focused review outcome: accepted after one blocking correction; and
- focused review phase close: completed with 39 events, one approval, zero
  retries, and zero escalations.

## Non-Goals

This phase does not add live GitHub access, token discovery, provider writes,
another GitHub operation, generic connector dispatch, OpenShell, hosted runtime,
release or publication behavior, or broader autonomy claims.

## Next Step

Phase 4 live read-only activation remains separately governed. The new P0
external-connector authority projection lane must be reconciled before broader
provider mutation or continuous delegated-maintainer claims.

See the [focused review](CAPABILITY_GATED_GITHUB_READ_ONLY_FIXTURE_ACTIVATION_REVIEW.md).
