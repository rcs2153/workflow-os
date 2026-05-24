# Runtime

Runtime documentation describes the v0 local-first runtime kernel.

The current runtime implements a deliberately narrow local execution path:

- event-sourced workflow run state
- deterministic rehydration from durable events
- local filesystem state backend
- single-step local skill execution
- approval pause/resume
- bounded retry, cancellation, and escalation semantics
- conservative policy checks before meaningful actions
- audit and observability sink interfaces

The runtime does not implement distributed workers, production database backends, real external adapters, real trigger processing, UI, hosted SaaS behavior, or Level 3/4 autonomy by default.

Start with:

- [event model](event-model.md)
- [run rehydration](run-rehydration.md)
- [state machine](state-machine.md)
- [local executor](local-executor.md)
- [state backends](state-backends.md)
- [policy engine](policy-engine.md)
