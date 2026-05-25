# CI Read-Only Failure Summary Example

This example demonstrates a Workflow OS Phase 2 read-only integration path. It reads GitHub Actions workflow run, job, failure, and log-reference context through the CI read-only adapter contract, then uses a deterministic local mock skill to produce a failure diagnosis summary.

It is a development-branch reference example, not part of the `0.1.0-preview.1` local kernel release contract and not a public read-only integration preview.

It does not rerun CI, dispatch workflows, cancel workflows, modify checks, write comments, or attempt auto-repair.

## What Is Real

- The project manifest, workflow spec, skill spec, policy spec, and test spec are real Workflow OS declarative files.
- `workflow-os validate` runs the real project loader and semantic validator.
- `workflow-os run` uses the real local executor, policy checks, approval pause/resume, durable local state, event log, audit events, and observability events.
- The GitHub Actions reads go through the real read-only adapter contract.
- The GitHub Actions adapter produces contract-level adapter telemetry records.
- Log excerpts are bounded and redacted before they appear in summaries.

## What Is Mocked

- The GitHub Actions data is local fixture data under `fixtures/github-actions/`.
- The CLI fixture path is intentionally explicit. It uses `--mock-all-local-skills` to register the deterministic example handler, and that handler uses the real GitHub Actions read-only adapter contract against local fixture files.
- No live GitHub credentials are required for normal use or tests.

## Validate

Build the CLI from the repository root, then validate the example:

```sh
cargo build -p workflow-cli --bin workflow-os
target/debug/workflow-os --project-dir examples/ci-read-only-failure-summary validate
```

Expected result:

```text
Project is valid.
```

## Run With Fixture Data

```sh
target/debug/workflow-os \
  --project-dir examples/ci-read-only-failure-summary \
  --mock-all-local-skills \
  run ex/ci
```

The run pauses for approval before producing the final diagnosis summary. Copy the printed `run_id` and `approval_id`.

```sh
target/debug/workflow-os \
  --project-dir examples/ci-read-only-failure-summary \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/example-reviewer \
  --reason reviewed-fixture-ci-context
```

Inspect the completed run:

```sh
target/debug/workflow-os \
  --project-dir examples/ci-read-only-failure-summary \
  inspect <run-id>
```

The event history should show approval, policy, and skill invocation runtime events. The output summary should include a CI failure diagnosis and a manual escalation recommendation for ambiguous failures. It should not contain the token-like or password-like values present in the fixture log.

## Optional Live Read-Only Exploration

Normal tests and the example commands above use fixture mode. Live GitHub Actions read-only experiments are opt-in and are documented in [GitHub Actions read-only setup](../../docs/operations/github-actions-read-only-setup.md).

Live mode still does not support reruns, dispatch, cancellation, check mutation, comments, or auto-repair.

## Safety Boundary

This example is a governed read-only context-gathering workflow. It is not production CI automation, not a repair bot, and not a write-capable GitHub Actions integration. Credentials must never be stored in specs.

## Telemetry Posture

This example emits normal runtime audit/observability signals for workflow, policy, approval, and skill events. CI adapter telemetry is contract-level adapter telemetry in Phase 2: the adapter produces `AdapterInvocationRecord` and `AdapterObservabilityRecord` values, but the fixture-backed CLI path does not yet persist those records as first-class runtime audit/observability records.
