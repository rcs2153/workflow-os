# CLI

The Workflow OS CLI is the primary v0 user interface for the local-first kernel.

Supported v0 commands:

- [validate](validate.md)
- [run](run.md)
- [status](status.md)
- [approve](approve.md)
- [inspect](inspect.md)
- [doctor](doctor.md)
- [init-agent-harness](init-agent-harness.md)

See [overview](overview.md) and [exit codes](exit-codes.md) for shared behavior.

The CLI only uses the local executor and explicitly registered local skill handlers. The optional `--mock-all-local-skills` flag registers deterministic mock handlers for local examples and smoke tests; it is not a real skill implementation system. `init-agent-harness` is documentation/scaffold-only and does not run workflows, approve checkpoints, execute checks, register handlers, or write runtime state. The CLI does not expose generic live adapter execution commands, CI integration, hosted operation, distributed worker operation, production backend operation, or write-capable external adapter workflows.
