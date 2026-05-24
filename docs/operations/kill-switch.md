# Kill Switch

The kill switch is a conservative local runtime control that prevents new execution and further non-terminal mutating actions.

## v0 Behavior

When enabled, the kill switch denies:

- starting workflow execution
- requesting approvals
- resuming workflow execution
- invoking skills
- invoking adapters

It still allows:

- cancellation
- inspection

This lets an operator stop further work while still allowing safe shutdown of waiting or running local runs.

## Limitations

The v0 kill switch is local runtime configuration. It is not distributed, centralized, or backed by an enterprise control plane.

Future production backends must define how kill-switch state is distributed, audited, and made durable without allowing workers to bypass it.
