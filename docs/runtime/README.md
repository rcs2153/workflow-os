# Runtime

Runtime documentation describes the v0 local-first kernel and explicit
evaluation-only shared-state and hosted proofs.

The current runtime implements a deliberately narrow local execution path:

- event-sourced workflow run state
- deterministic rehydration from durable events
- local filesystem state backend
- sequential local skill execution
- approval pause/resume
- bounded retry, cancellation, and escalation semantics
- conservative policy checks before meaningful actions
- audit and observability sink interfaces
- explicit in-memory report-bearing local execution APIs
- explicit local work report artifact store support
- explicit opt-in shared `PostgreSQL` state semantics
- a single-tenant hosted alpha foundation with an authenticated API,
  stateless fenced worker, and no-write provider

The runtime does not implement automatic work-report generation for every run,
CLI report rendering, automatic report artifact writing, a production database
deployment, real write-capable external adapters, real trigger processing,
multi-tenant hosted SaaS behavior, UI, or Level 3/4 autonomy by default.

Start with:

- [event model](event-model.md)
- [run rehydration](run-rehydration.md)
- [state machine](state-machine.md)
- [local executor](local-executor.md)
- [state backends](state-backends.md)
- [single-tenant hosted alpha](single-tenant-hosted-alpha.md)
- [policy engine](policy-engine.md)
