# CLI

The Workflow OS CLI is the primary v0 user interface for the local-first kernel.

Supported v0 commands:

- [validate](validate.md)
- [run](run.md)
- [status](status.md)
- [approve](approve.md)
- [inspect](inspect.md)
- [doctor](doctor.md)

See [overview](overview.md) and [exit codes](exit-codes.md) for shared behavior.

The CLI only uses the local executor and explicitly registered local skill handlers. The optional `--mock-all-local-skills` flag registers deterministic mock handlers for local examples and smoke tests; it is not a real skill implementation system. The CLI does not implement real GitHub, Jira, CI, hosted, distributed worker, production backend, or external adapter integrations.
