# `workflow-os doctor`

Checks the local Workflow OS environment.

```text
workflow-os doctor
workflow-os --json doctor
```

The command checks:

- project manifest presence
- project loader diagnostics
- local backend accessibility
- schema directory availability where present

`doctor` exits non-zero when the local project is missing or cannot be loaded safely.

The command does not execute workflows or invoke skills.
