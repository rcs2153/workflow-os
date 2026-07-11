# CLI

The Workflow OS CLI is the primary v0 user interface for the local-first kernel.

Supported v0 commands:

- [version](version.md)
- [validate](validate.md)
- [run](run.md)
- [status](status.md)
- [approve](approve.md)
- [inspect](inspect.md)
- [doctor](doctor.md)
- [init-agent-harness](init-agent-harness.md)
- [init-repo-governance](init-repo-governance.md)
- [first-run](first-run.md)
- [author workflow](author-workflow.md)
- [provider GitHub PR comment recovery summary](provider-github-pr-comment-recovery-summary.md)

See [overview](overview.md) and [exit codes](exit-codes.md) for shared behavior.

The CLI only uses the local executor and explicitly registered local skill handlers. The optional `--mock-all-local-skills` flag registers deterministic mock handlers for local examples and smoke tests; it is not a real skill implementation system. `init-agent-harness` is documentation/scaffold-only and does not run workflows, approve checkpoints, execute checks, register handlers, or write runtime state. `init-repo-governance` creates a minimal local Workflow OS project envelope for an existing repository, but it does not run workflows, approve checkpoints, execute commands, register real handlers, write runtime state, create report artifacts, or call providers. `first-run` emits a bounded report-ready context after validation; it does not run workflows, create runtime state, write report artifacts, inspect raw source contents, call providers, or auto-generate workflows. `author workflow --dry-run` previews inactive authoring obligations from a first-run recommendation; it does not write files, register workflows, execute commands, call providers, or create runtime state. `author workflow steward-review` previews steward review for a preflight-passing inactive draft; it does not promote drafts, persist approval records, create runtime state, execute commands, call providers, or write artifacts. `author workflow promote` is an explicit active-file mutation boundary for one reviewed draft; it writes at most one active workflow file, preserves the draft, refuses overwrites, and still does not persist approvals, start runs, create runtime state, execute commands, call providers, write report artifacts, add schemas, add examples, or enable writes. `provider github-pr-comment recovery-summary` renders an explicit local recovery summary input; it does not call GitHub, load hidden auth, retry writes, repair state, append events, mutate side-effect records, or write report artifacts. The CLI does not expose generic live adapter execution commands, CI integration, hosted operation, distributed worker operation, production backend operation, or write-capable external adapter workflows.
