# v0 Known Limitations

Workflow OS v0 is a public local kernel preview. It is a serious local-first kernel foundation, not a production deployment platform, production distributed runtime, hosted product, adapter-complete framework, UI product, or Level 3/4 autonomy system.

## Runtime Scope

- Only the local executor path is implemented.
- The executor supports one local workflow step.
- CLI execution of declared `local/*` skills requires explicit handler registration. The `--mock-all-local-skills` flag is a deterministic mock convenience for examples and smoke tests, not a real skill implementation system.
- Conditional branches are parsed and validated in limited form but not executed.
- Real trigger processing is not implemented.
- No trigger ingestion service is implemented.
- Active background timers are not implemented.
- Timeout policies are parsed and represented, but there is no background timeout scheduler.
- No active timeout scheduler exists.
- Escalated runs have no operator resume flow in v0.

## Integrations

- The `0.1.0-preview.1` local kernel preview does not include real GitHub, Jira, CI, SaaS, generic HTTP, OAuth, webhook, or external event ingestion adapters.
- The development branch contains Phase 2 GitHub, Jira, and GitHub Actions read-only adapter work for internal review.
- Phase 2 read-only adapters support fixture-backed tests by default and opt-in live reads through documented environment variables.
- Public read-only integration preview readiness has not been approved yet.
- GitHub write operations do not exist.
- Jira write operations do not exist.
- CI rerun, dispatch, cancellation, artifact upload, log deletion, and check mutation operations do not exist.
- No generic HTTP client adapter exists.
- No OAuth, webhook, or external event ingestion flows exist.
- Adapter contracts are documented and modeled; development-branch GitHub, Jira, and GitHub Actions read-only implementations are not production integrations.
- Adapter telemetry is contract-level adapter telemetry in Phase 2. Adapters produce redacted `AdapterInvocationRecord` and `AdapterObservabilityRecord` values, but fixture-backed CLI examples do not yet persist those records as first-class runtime audit/observability records.
- No marketplace or package registry behavior exists.
- Phase 2 continues to focus on read-only GitHub, Jira, and CI adapter capability only. Write operations remain out of scope until explicitly designed later.
- Future Phase 2 read-only adapter work must not imply support for creating branches, opening pull requests, posting comments, updating Jira issues, rerunning CI, workflow dispatch, OAuth app flows, webhook ingestion, distributed workers, production database backends, or Level 3/4 autonomy enablement.

## State And Operations

- The local filesystem backend is for local development and tests.
- There is no production Postgres, Redis, queue, or distributed-lock backend.
- There is no production database backend.
- There are no distributed workers.
- Local locks are filesystem-local and are not safe as distributed coordination across machines.
- Backup and restore are manual directory-level operations.
- `workflow-os doctor state` can inspect local state corruption without mutation, but no automated state repair command exists.

## Security And Governance

- Level 3 and Level 4 autonomy are declaration-only and denied by default.
- Level 3 and Level 4 autonomy enablement is not implemented in v0.
- The conservative policy engine is not enterprise RBAC.
- No IdP integration exists.
- No real secret provider exists.
- `secret.read` is denied by default.
- Maintainer decision for `0.1.0-preview.1`: `serde_yaml` is accepted for trusted local project files only, despite being deprecated and depending on `unsafe-libyaml`.
- v0 does not claim hardened malicious-spec parsing, remote YAML ingestion safety, parser sandboxing, or production-grade parser isolation.
- `YAML-001` tracks replacing or isolating the YAML parser before any production-readiness or adversarial-input hardening claim.

## SDK And Contracts

- Rust remains the source of truth.
- The TypeScript SDK emits specs and does not execute workflows.
- TypeScript types are manually synchronized with JSON Schema and Rust models in v0, with package-level and repository-level contract tests against Rust CLI validation.
- JSON Schemas are manually checked-in v0 compatibility artifacts. CI verifies presence, version pinning, examples, and representative SDK output, but schemas and TypeScript types are not generated from Rust yet.
- Full schema-generated TypeScript types are deferred.

## CLI

- The CLI supports local validate, run, status, approve, inspect, and doctor commands.
- `workflow-os run` and approval resume fail closed when no local handler is registered, unless `--mock-all-local-skills` is explicitly provided for mock execution.
- Local mock handlers are trusted local preview tooling only; they are not a production skill plugin system.
- CLI `--json` output is experimental in `0.1.0-preview.1` and is not yet a versioned stable machine-output contract.
- The CLI does not implement project initialization, docs generation, adapter commands, distributed worker commands, or production deployment commands.

## Validation And Execution

- Semantic validation is deterministic but intentionally scoped to v0 concepts.
- Full input/output schema validation is not implemented in the executor.
- Nested contract validation and deep payload-level redaction inspection are deferred. v0 audit avoids raw skill inputs, records references/summaries, and redacts sensitive-looking audit summaries deterministically.
- Mock local skill behavior proves the kernel path only and must not be presented as production AI or business decisioning.
