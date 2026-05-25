# `workflow-os doctor`

Checks the local Workflow OS environment.

```text
workflow-os doctor
workflow-os --json doctor
workflow-os doctor state
workflow-os --json doctor state
```

The command checks:

- project manifest presence
- project loader diagnostics
- local backend accessibility
- local backend event/index consistency
- schema directory availability where present

`doctor` exits non-zero when the local project is missing or cannot be loaded safely.

`--json` output for `doctor` is experimental in `0.1.0-preview.1`. It is useful for preview automation, but it is not yet a versioned stable machine-output contract.

## `workflow-os doctor state`

`doctor state` inspects the configured local state backend without mutating or repairing it.

The command reports:

- missing event ID index files for event files
- dangling event ID index files that point to missing event files
- malformed or corrupt event/index/projection JSON files
- run rehydration failures caused by invalid event streams
- approval projection records that do not have matching event-log approval requests
- pending event-backed approvals that are missing local projection files

Human-readable output summarizes the state root, backend health, and issues:

```text
workflow-os doctor state
```

JSON output is available:

```text
workflow-os --json doctor state
```

The JSON shape is experimental in `0.1.0-preview.1` and includes `healthy`, `backend`, `root`, and an `issues` array. It is not yet a versioned stable machine-output contract. `doctor state` exits non-zero when any error-severity issue is found. Warning-severity findings, such as a missing rebuildable approval projection for a pending approval, are printed for operator review.

`doctor state` does not create the state directory, write health probes, delete files, rebuild projections, or repair indexes. Preserve the full state root before manual investigation.

The command does not execute workflows or invoke skills.
