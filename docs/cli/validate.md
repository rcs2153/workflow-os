# `workflow-os validate`

Loads the current Workflow OS project and runs deterministic semantic validation.

```text
workflow-os validate
workflow-os --project-dir examples/vertical-slice-approval validate
workflow-os --json validate
```

Validation prints loader and semantic diagnostics with source information where available.

If no `workflow-os.yml` is found, human-readable output includes the next step:

```text
next_step: workflow-os init-repo-governance
```

The command exits non-zero when validation has errors. Warnings are printed but do not fail validation by themselves.

`--json` output remains experimental through `0.2.0-preview.1`. It is useful for preview automation, but it is not yet a versioned stable machine-output contract.

The command never executes workflows, invokes skills, calls adapters, or mutates runtime state.
