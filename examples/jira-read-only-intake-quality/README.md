# Jira Read-Only Intake Quality Example

This example demonstrates Workflow OS reading Jira issue context through the Phase 2 Jira read-only adapter contract.

It is a `0.2.0-preview.1` reference example for the public read-only integration preview. It is not production Jira automation.

## What This Example Shows

- Project manifest.
- Workflow definition.
- Skill definition.
- Symbolic Jira read-only adapter requirement.
- Fixture-backed Jira issue data.
- A workflow step that reads issue metadata, description presence, and comments.
- A deterministic local fixture handler that produces an intake-quality assessment summary.
- Approval before the summary is treated as a final recommendation.
- contract-level adapter telemetry is mapped into runtime-visible adapter telemetry records from the Jira read-only adapter contract in fixture mode.
- CLI validate and run using fixture/mock mode.

## What This Example Does Not Do

- It does not update Jira.
- It does not add Jira comments.
- It does not change issue status.
- It does not assign users.
- It does not change labels.
- It does not require live Jira credentials.
- It does not implement production Jira automation.

## Validate

From the repository root:

```sh
target/debug/workflow-os --project-dir examples/jira-read-only-intake-quality validate
```

Expected result:

```text
Project is valid.
```

## Run With Fixture Data

The CLI fixture path is intentionally explicit. It uses `--mock-all-local-skills` to register the deterministic example handler, and that handler uses the real Jira read-only adapter contract against local fixture files.

```sh
target/debug/workflow-os \
  --project-dir examples/jira-read-only-intake-quality \
  --mock-all-local-skills \
  run ex/jira
```

The run pauses for approval before producing the final recommendation summary.

Approve and resume:

```sh
target/debug/workflow-os \
  --project-dir examples/jira-read-only-intake-quality \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/example-reviewer \
  --reason reviewed-fixture-intake
```

Inspect the run:

```sh
target/debug/workflow-os \
  --project-dir examples/jira-read-only-intake-quality \
  inspect <run-id>
```

## Fixture Files

Fixture data lives under:

```text
fixtures/jira/
```

The normal example path reads:

- `issue-OPS-42.json`
- `issue-OPS-42-comments.json`

The workflow reads issue metadata, description presence, and comments. Descriptions and comment bodies are represented as reference-only summaries.

## Live Read-Only Mode

Live Jira mode is optional and not used by normal tests.

To experiment manually with live adapter tests, see [Jira read-only setup](../../docs/operations/jira-read-only-setup.md).

Do not put Jira credentials in specs. For Atlassian Cloud live read-only experiments, use `WORKFLOW_OS_JIRA_BASE_URL` plus `WORKFLOW_OS_JIRA_EMAIL` and `WORKFLOW_OS_JIRA_API_TOKEN`. `JIRA_EMAIL` and `JIRA_API_TOKEN` are fallback names. Bearer tokens are supported only for Jira deployments that explicitly accept bearer auth.

## Boundary

This example reinforces the adapter boundary:

- Jira context is read through an adapter.
- The adapter does not mutate workflow state directly.
- The example does not write to Jira.
- Private issue data should be treated as sensitive.
- Issue descriptions and comments should be represented by references and summaries.

## Telemetry Posture

This example emits normal runtime audit/observability signals for workflow, policy, approval, and skill events. The fixture-backed Jira handler also maps the adapter's `AdapterInvocationRecord` and `AdapterObservabilityRecord` values into local runtime-visible adapter telemetry records for the run. `workflow-os inspect` prints a concise redacted adapter telemetry summary.

This is not a generic adapter execution framework, not live Jira execution by default, not production telemetry export, and not SIEM or OpenTelemetry integration.
