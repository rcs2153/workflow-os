# GitHub Read-Only Review Context Example

This example demonstrates Workflow OS reading GitHub pull request context through the Phase 2 GitHub read-only adapter contract.

It is a development-branch reference example, not part of the `0.1.0-preview.1` local kernel release contract and not a public read-only integration preview. It is not production review automation.

## What This Example Shows

- Project manifest.
- Workflow definition.
- Skill definition.
- Symbolic GitHub read-only adapter requirement.
- Fixture-backed GitHub read-only data.
- A workflow step that reads pull request metadata and changed files.
- A deterministic local fixture handler that produces a review-context summary.
- Approval before the summary is treated as a final recommendation.
- Contract-level adapter telemetry records produced by the GitHub read-only adapter contract.
- CLI validate and run using fixture/mock mode.

## What This Example Does Not Do

- It does not post pull request comments.
- It does not create branches.
- It does not open pull requests.
- It does not update GitHub.
- It does not require live GitHub credentials.
- It does not implement production review automation.

## Validate

From the repository root:

```sh
target/debug/workflow-os --project-dir examples/github-read-only-review-context validate
```

Expected result:

```text
Project is valid.
```

## Run With Fixture Data

The CLI fixture path is intentionally explicit. It uses `--mock-all-local-skills` to register the deterministic example handler, and that handler uses the real GitHub read-only adapter contract against local fixture files.

```sh
target/debug/workflow-os \
  --project-dir examples/github-read-only-review-context \
  --mock-all-local-skills \
  run ex/gh
```

The run pauses for approval before producing the final recommendation summary.

Approve and resume:

```sh
target/debug/workflow-os \
  --project-dir examples/github-read-only-review-context \
  --mock-all-local-skills \
  approve <run-id> <approval-id> \
  --actor user/example-reviewer \
  --reason reviewed-fixture-context
```

Inspect the run:

```sh
target/debug/workflow-os \
  --project-dir examples/github-read-only-review-context \
  inspect <run-id>
```

## Fixture Files

Fixture data lives under:

```text
fixtures/github/
```

The normal example path reads:

- `repository.json`
- `pull-7.json`
- `pull-7-files.json`
- `pull-7-comments.json`
- `checks-main.json`
- `pull-7.diff`

The workflow itself only requires pull request metadata and changed files for the final summary. The additional fixture files demonstrate the supported read-only adapter surface.

## Live Read-Only Mode

Live GitHub mode is optional and not used by normal tests.

To experiment manually with live adapter tests, see [GitHub read-only setup](../../docs/operations/github-read-only-setup.md).

Do not put GitHub tokens in specs. Use `WORKFLOW_OS_GITHUB_TOKEN` or `GITHUB_TOKEN` only for opt-in live reads.

## Boundary

This example reinforces the adapter boundary:

- GitHub context is read through an adapter.
- The adapter does not mutate workflow state directly.
- The example does not write to GitHub.
- Private repository data should be treated as sensitive.
- Large or sensitive provider payloads should be represented by references and summaries.

## Telemetry Posture

This example emits normal runtime audit/observability signals for workflow, policy, approval, and skill events. GitHub adapter telemetry is contract-level adapter telemetry in Phase 2: the adapter produces `AdapterInvocationRecord` and `AdapterObservabilityRecord` values, but the fixture-backed CLI path does not yet persist those records as first-class runtime audit/observability records.
