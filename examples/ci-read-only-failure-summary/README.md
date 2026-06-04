# CI Read-Only Failure Summary Example

This example demonstrates a Workflow OS Phase 2 read-only integration path. It reads GitHub Actions workflow run, job, failure, and log-reference context through the CI read-only adapter contract, then uses a deterministic local mock skill to produce a failure diagnosis summary.

It is a `0.2.0-preview.1` reference example for the public read-only integration preview.

It does not rerun CI, dispatch workflows, cancel workflows, modify checks, write comments, or attempt auto-repair.

## What Is Real

- The project manifest, workflow spec, skill spec, policy spec, and test spec are real Workflow OS declarative files.
- `workflow-os validate` runs the real project loader and semantic validator.
- `workflow-os run` uses the real local executor, policy checks, approval pause/resume, durable local state, event log, audit events, and observability events.
- The GitHub Actions reads go through the real read-only adapter contract.
- contract-level adapter telemetry from the GitHub Actions adapter is mapped into runtime-visible adapter telemetry records in fixture mode.
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

This example emits normal runtime audit/observability signals for workflow, policy, approval, and skill events. The fixture-backed CI handler also maps the adapter's `AdapterInvocationRecord` and `AdapterObservabilityRecord` values into local runtime-visible adapter telemetry records for the run. `workflow-os inspect` prints a concise redacted adapter telemetry summary.

This is not a generic adapter execution framework, not live GitHub Actions execution by default, not production telemetry export, and not SIEM or OpenTelemetry integration.
