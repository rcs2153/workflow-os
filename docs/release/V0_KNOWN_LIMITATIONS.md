# v0 Known Limitations

Workflow OS v0 is a public local kernel preview. It is a serious local-first kernel foundation, not a production deployment platform, production distributed runtime, hosted product, adapter-complete framework, UI product, or Level 3/4 autonomy system.

## Runtime Scope

- Only the local executor path is implemented.
- The executor supports sequential ordered local workflow steps. Governed multi-step workflow execution is now the P0 kernel path, accepted in [ADR 0010](../adr/0010-governed-multi-step-workflow-execution.md) and scoped in [Governed Multi-Step Workflow Execution Plan](../implementation-plans/governed-multi-step-workflow-execution-plan.md). The implemented slice remains local and sequential only.
- CLI execution of declared `local/*` skills requires explicit handler registration. The `--mock-all-local-skills` flag is a deterministic mock convenience for examples and smoke tests, not a real skill implementation system.
- Conditional branches are parsed and validated in limited form but not executed.
- Parallel execution, DAG scheduling, and nested harness execution are not implemented.
- Real trigger processing is not implemented.
- No trigger ingestion service is implemented.
- Active background timers are not implemented.
- Timeout policies are parsed and represented, but there is no background timeout scheduler.
- No active timeout scheduler exists.
- Escalated runs have no operator resume flow in v0.
- The self-governance dogfood project governs planning/docs work only; it does not execute build commands or repository edits.
- Self-governed validation/check execution is not implemented; only the local validation/check command contract model exists.

## Integrations

- The `0.1.0-preview.1` local kernel preview does not include real GitHub, Jira, CI, SaaS, generic HTTP, OAuth, webhook, or external event ingestion adapters.
- The `0.2.0-preview.1` posture adds a public read-only integration preview for GitHub, Jira, and GitHub Actions / CI.
- Read-only adapters support fixture-backed tests by default and opt-in live reads through documented environment variables.
- Live smoke evidence is intentionally shallow: GitHub exercised `octocat/Hello-World` repository metadata, GitHub Actions exercised workflow run metadata only, and Jira exercised issue metadata only.
- The Jira sandbox API token used during smoke testing should be rotated because a sandbox token was pasted into the local evaluation thread.
- GitHub write operations do not exist.
- Jira write operations do not exist.
- CI rerun, dispatch, cancellation, artifact upload, log deletion, and check mutation operations do not exist.
- No generic HTTP client adapter exists.
- No OAuth, webhook, or external event ingestion flows exist.
- Adapter contracts are documented and modeled; GitHub, Jira, and GitHub Actions read-only implementations are public preview integrations, not production integrations.
- Adapter telemetry starts as contract-level adapter telemetry in Phase 2. Controlled fixture-backed CLI examples map redacted adapter invocation and observability records into local runtime-visible adapter telemetry records, but this is not generic adapter execution, live adapter execution by default, production telemetry export, SIEM integration, or OpenTelemetry integration.
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
- Work report artifacts may be written explicitly through the local `WorkReportArtifactStore`; executor paths do not write them automatically and CLI commands do not render or export them.

## Evidence And Work Reports

- `EvidenceReference` is implemented as a core model with selected adapter telemetry, diagnostic, and schema-version diagnostic attachment paths.
- EvidenceReference persistence, CLI rendering, examples, approval attachment, and broad automatic attachment remain unimplemented.
- `WorkReportContract` and `WorkReport` core models are implemented.
- Explicit in-memory terminal local report generation, runtime result exposure, and executor-integrated report-bearing local execution APIs are implemented.
- Explicit local work report artifact storage is implemented through `WorkReportArtifactStore`.
- Automatic runtime report generation for every run is not implemented.
- Automatic report artifact writing from executor paths is not implemented.
- CLI report rendering/export and report schemas are not implemented.
- Approval-resume and cancellation report-bearing APIs are not implemented.

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
- CLI `--json` output remains experimental through `0.2.0-preview.1` and is not yet a versioned stable machine-output contract.
- The CLI does not implement project initialization, docs generation, adapter commands, distributed worker commands, or production deployment commands.
- The CLI does not expose real local build/check skill handlers for self-governance dogfooding; current dogfood execution uses explicit deterministic mock handling.

## Validation And Execution

- Semantic validation is deterministic but intentionally scoped to v0 concepts.
- Full input/output schema validation is not implemented in the executor.
- Nested contract validation and deep payload-level redaction inspection are deferred. v0 audit avoids raw skill inputs, records references/summaries, and redacts sensitive-looking audit summaries deterministically.
- Mock local skill behavior proves the kernel path only and must not be presented as production AI or business decisioning.
