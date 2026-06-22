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

Schema directory availability is optional in the local preview. Human-readable output reports missing optional schemas as `schemas: unavailable_optional` rather than a hard schema failure. Missing or invalid project manifests still cause `doctor` to exit non-zero.

`doctor` exits non-zero when the local project is missing or cannot be loaded safely.

`--json` output for `doctor` remains experimental through `0.2.0-preview.1`. It is useful for preview automation, but it is not yet a versioned stable machine-output contract.

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

The JSON shape remains experimental through `0.2.0-preview.1` and includes `healthy`, `backend`, `root`, and an `issues` array. It is not yet a versioned stable machine-output contract. `doctor state` exits non-zero when any error-severity issue is found. Warning-severity findings, such as a missing rebuildable approval projection for a pending approval, are printed for operator review.

`doctor state` does not create the state directory, write health probes, delete files, rebuild projections, or repair indexes. Preserve the full state root before manual investigation.

The command does not execute workflows or invoke skills.
