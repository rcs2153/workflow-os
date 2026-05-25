# CI Read-Only Adapters

CI read-only adapters are Phase 2 development-branch integrations. They retrieve CI run facts, status summaries, failure context, and log references without mutating CI systems.

GitHub Actions is the first concrete CI read-only implementation.

This work is not part of the `0.1.0-preview.1` local kernel release contract, and it is not a public read-only integration preview until a follow-up maintainer review approves that posture.

See [the CI read-only failure summary example](../../examples/ci-read-only-failure-summary/README.md) for a fixture-backed workflow that reads GitHub Actions failure context without rerunning or modifying CI.

## Scope

Generic CI read-only operations include:

- read workflow or run metadata
- read job status summaries
- read check status summaries
- read failure summaries
- read log references
- read limited log excerpts only when explicitly requested and redacted
- normalize CI data into `AdapterResponse`, `AdapterInvocationRecord`, and `AdapterObservabilityRecord` values

## Explicit Non-Goals

CI read-only adapters do not support:

- rerunning workflows
- rerunning failed jobs
- canceling workflows
- dispatching workflows
- uploading artifacts
- deleting logs
- modifying checks
- webhook receivers

Write-capable CI actions fail closed in Phase 2.

## Capability

CI read-only adapter requests use `ci.read`.

These capabilities remain unsupported or denied:

- `ci.write`
- `ci.rerun`
- `adapter.write`

Credentials are not permission. Runtime policy must still allow the adapter invocation.

## Logs

CI logs are sensitive by default.

The default posture is:

- store log references rather than raw logs
- never store full logs in audit by default
- allow bounded excerpts only through explicit read actions
- redact sensitive-looking log lines
- truncate excerpts to a small preview size

## Modes

- `fixture`: static offline CI responses for tests.
- `mock`: deterministic test doubles.
- `live-read-only`: real read-only CI API calls.
- `live-write-capable`: represented by the generic adapter contract but denied in Phase 2.

CI must not require live credentials in normal tests or CI.
