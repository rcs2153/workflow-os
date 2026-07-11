# version

`workflow-os --version` and `workflow-os version` print bounded CLI identity
without requiring a Workflow OS project.

```sh
workflow-os --version
workflow-os version
workflow-os --json version
```

Text output is intentionally small:

```text
workflow-os 0.2.0-preview.1
```

Preview JSON output includes bounded product posture:

```json
{
  "name": "workflow-os",
  "version": "0.2.0-preview.1",
  "schema_version": "workflowos.dev/v0",
  "release_posture": "local_kernel_preview"
}
```

## Boundary

The version command does not:

- require `workflow-os.yml`;
- load project specs;
- create runtime state;
- read provider credentials;
- call external services;
- inspect repository source contents;
- execute local skill handlers;
- emit paths, environment values, tokens, or project payloads.

CLI JSON remains experimental through `0.2.0-preview.1`; the version JSON
shape is useful for preview troubleshooting but is not yet a stable machine
contract.
