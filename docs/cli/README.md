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

The CLI only uses the local executor and local skill handlers. It does not implement real GitHub, Jira, CI, hosted, or external adapter integrations.
