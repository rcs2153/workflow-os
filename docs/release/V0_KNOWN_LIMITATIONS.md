# v0 Known Limitations

Workflow OS v0 is a serious local-first kernel foundation, not a production deployment platform.

## Runtime Scope

- Only the local executor path is implemented.
- The executor supports one local workflow step.
- Conditional branches are parsed and validated in limited form but not executed.
- Real trigger processing is not implemented.
- Active background timers are not implemented.
- Timeout policies are parsed and represented, but there is no background timeout scheduler.
- Escalated runs have no operator resume flow in v0.

## Integrations

- No real GitHub adapter exists.
- No real Jira adapter exists.
- No real CI adapter exists.
- No generic HTTP client adapter exists.
- No OAuth, webhook, or external event ingestion flows exist.
- Adapter contracts are documented and modeled, but external API clients are deferred.

## State And Operations

- The local filesystem backend is for local development and tests.
- There is no production Postgres, Redis, queue, or distributed-lock backend.
- Local locks are filesystem-local and are not safe as distributed coordination across machines.
- Backup and restore are manual directory-level operations.
- No automated state repair command exists.

## Security And Governance

- Level 3 and Level 4 autonomy are declaration-only and denied by default.
- The conservative policy engine is not enterprise RBAC.
- No IdP integration exists.
- No real secret provider exists.
- `secret.read` is denied by default.
- Dependency review identifies `serde_yaml` and its transitive `unsafe-libyaml` dependency as a v0 risk to revisit before production-readiness claims.

## SDK And Contracts

- Rust remains the source of truth.
- The TypeScript SDK emits specs and does not execute workflows.
- TypeScript types are manually synchronized with JSON Schema and Rust models in v0, with contract tests against Rust CLI validation.
- Full schema-generated TypeScript types are deferred.

## CLI

- The CLI supports local validate, run, status, approve, inspect, and doctor commands.
- The CLI does not implement project initialization, docs generation, adapter commands, distributed worker commands, or production deployment commands.

## Validation And Execution

- Semantic validation is deterministic but intentionally scoped to v0 concepts.
- Full input/output schema validation is not implemented in the executor.
- Nested contract validation and field-level runtime redaction enforcement are deferred.
- Mock local skill behavior proves the kernel path only and must not be presented as production AI or business decisioning.
