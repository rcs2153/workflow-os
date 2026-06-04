# Troubleshooting

This guide covers the local-first v0 kernel. It does not cover hosted services, distributed workers, production databases, UI behavior, production integrations, or write-capable adapters because those are not implemented in v0. GitHub, Jira, and GitHub Actions read-only adapter setup has dedicated operation docs and remains fixture-first with opt-in live tests.

## Validation Fails

Run:

```sh
workflow-os validate
```

Use the diagnostic code, file path, line, column, and document path to fix the spec. Common causes are:

- missing `schema_version`
- unsupported schema version
- missing workflow triggers or steps
- unresolved skill or policy references
- unbounded retry policy
- missing approval policy for approval-sensitive behavior
- secret-like keys or values in specs

Validation errors block execution. Warnings should still be reviewed before release.

## Run Does Not Start

Confirm:

- `workflow-os validate` succeeds
- the workflow ID passed to `workflow-os run <workflow-id>` exists
- the workflow has exactly one executable local step in v0
- the referenced skill has a local deterministic handler path
- the conservative policy engine allows the action
- the kill switch is not enabled in the caller configuration

The CLI does not expose generic live adapter execution commands. Unsupported adapter-backed or external-write behavior must fail closed.

## Run Waits For Approval

Use:

```sh
workflow-os inspect <run-id>
```

Find the approval request ID, then approve with:

```sh
workflow-os approve <run-id> <approval-id> --actor user/example --reason "reviewed"
```

Approval denial fails or cancels according to documented runtime behavior. Approval-gated steps must not execute before approval is granted.

## Run Escalates

Inspect the `EscalationTriggered` event:

```sh
workflow-os inspect <run-id>
```

Review the run ID, workflow version, spec hash, step ID, skill ID, attempt count, last error, failure class, and suggested next action. v0 has no external notification or operator resume flow for escalated runs.

## Corrupt Local State

If `status` or `inspect` returns a `state.corrupt` error, do not delete state immediately. First copy the full state root for investigation.

Run the read-only state inspection command:

```sh
workflow-os doctor state
workflow-os --json doctor state
```

Use the reported issue codes and paths to identify missing event files, dangling event ID indexes, malformed JSON, rehydration failures, or approval projection drift. `doctor state` does not mutate or repair local state.

Then verify:

- event files are valid JSON
- sequence files are contiguous for the run
- event IDs are unique
- no partial backup restore omitted event files

Snapshots and approval records are projections and may be rebuilt in future tooling, but v0 does not provide an automated repair command.

## Lock Contention

`state.lock_contended` means another local process is using the same state root. Wait for the other command to finish. If the process crashed, remove the stale lock directory only after confirming no active process is using that state root.

## Secrets In Output

Specs must not contain secrets. Runtime logs, audit events, and inspected output should contain references or summaries, not raw sensitive payloads.

If a secret appears in output:

1. Stop using the affected spec or handler.
2. Rotate the exposed secret.
3. Preserve audit/state files for review.
4. File a private security report through `SECURITY.md`.
