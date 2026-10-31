# v0 Known Limitations

Workflow OS v0 is a public governed-execution and continuity preview built on a serious local-first kernel foundation. It is not a production deployment platform, production distributed runtime, production hosted product, adapter-complete framework, UI product, broad write-capable integration system, autonomous scheduler, or Level 3/4 autonomy system.

## Runtime Scope

- The public CLI execution path remains primarily local. Bounded hosted, trusted-host, shared-state, and containment compositions are alpha or proof surfaces, not a general production executor.
- The executor supports sequential ordered local workflow steps. Governed multi-step workflow execution is now the P0 kernel path, accepted in [ADR 0010](../adr/0010-governed-multi-step-workflow-execution.md) and scoped in [Governed Multi-Step Workflow Execution Plan](../implementation-plans/governed-multi-step-workflow-execution-plan.md). The implemented slice remains local and sequential only.
- CLI execution of declared `local/*` skills requires explicit handler registration. The `--mock-all-local-skills` flag is a deterministic mock convenience for examples and smoke tests, not a real skill implementation system.
- Conditional branches are parsed and validated in limited form but not executed.
- Parallel execution, DAG scheduling, and nested harness execution are not implemented.
- Real trigger processing is not implemented.
- No trigger ingestion service is implemented.
- Active background timers are not implemented.
- Timeout policies are parsed and represented, but there is no background timeout scheduler.
- No active timeout scheduler exists.
- Escalation and continuation have selected explicit operator and authoritative-context paths, but there is no autonomous production scheduler that resolves every wait, timeout, or escalation.
- The self-governance dogfood project governs planning/docs work only. It can exercise the docs check only through explicit `DocsCheckLocalHandler` registration; it does not execute arbitrary build commands or repository edits.
- Self-governed validation/check execution is limited to explicit local check handlers; true default registration, CLI exposure, workflow schema activation, and broad build/test command execution are not implemented.

## Integrations

- The `0.1.0-preview.1` local kernel preview does not include real GitHub, Jira, CI, SaaS, generic HTTP, OAuth, webhook, or external event ingestion adapters.
- The `0.2.0-preview.1` posture adds a public read-only integration preview for GitHub, Jira, and GitHub Actions / CI.
- Read-only adapters support fixture-backed tests by default and opt-in live reads through documented environment variables.
- Live smoke evidence is intentionally shallow: GitHub exercised `octocat/Hello-World` repository metadata, GitHub Actions exercised workflow run metadata only, and Jira exercised issue metadata only.
- The Jira sandbox API token used during smoke testing should be rotated because a sandbox token was pasted into the local evaluation thread.
- Narrow GitHub PR-comment and managed draft-PR sandbox proofs exist behind explicit authority, policy, approval, SideEffect, report, allowlist, and transport gates. They are not a general GitHub write adapter.
- Jira write operations do not exist.
- CI rerun, dispatch, cancellation, artifact upload, log deletion, and check mutation operations do not exist.
- No generic HTTP client adapter exists.
- No OAuth, webhook, or external event ingestion flows exist.
- Adapter contracts are documented and modeled; GitHub, Jira, and GitHub Actions read-only implementations are public preview integrations, not production integrations.
- Adapter telemetry starts as contract-level adapter telemetry in Phase 2. Controlled fixture-backed CLI examples map redacted adapter invocation and observability records into local runtime-visible adapter telemetry records, but this is not generic adapter execution, live adapter execution by default, production telemetry export, SIEM integration, or OpenTelemetry integration.
- No marketplace or package registry behavior exists.
- The `0.3.0-preview.1` release does not broaden the narrow GitHub sandbox proofs into branch creation, arbitrary pull-request mutation, general comments, Jira writes, CI writes, OAuth app flows, webhook ingestion, or production provider automation.
- Read-only adapter support and narrow sandbox proofs must not imply distributed-worker readiness, production database readiness, or Level 3/4 autonomy enablement.

## State And Operations

- The local filesystem backend is for local development and tests.
- An explicit opt-in shared `PostgreSQL` state preview implements the accepted
  Core transaction families, revisions, fenced leases, one shared run-event
  consumer, projection rebuild, and CI backup/restore rehearsal.
- The `PostgreSQL` adapter is not selected automatically and is not a
  production database or hosted-runtime claim.
- There is no reviewed production TLS factory, pooling, replication, high
  availability, point-in-time recovery, capacity proof, or production SLO.
- There is no automatic distributed worker service or general job queue.
- Redis, SQS, NATS, and general distributed-lock services are not implemented.
- Local locks are filesystem-local and are not safe as distributed coordination across machines.
- Filesystem backup and restore remain manual directory-level operations.
- The `PostgreSQL` adapter has a CI logical backup/restore integrity rehearsal;
  this does not establish production disaster recovery.
- `workflow-os doctor state` can inspect local state corruption without mutation, but no automated state repair command exists.
- Work report artifacts may be written explicitly through selected local artifact paths. The first-run quiet-success path can write an exact validated artifact only when explicitly requested; general CLI rendering/export and automatic artifact generation for every run remain unavailable.

## Evidence And Work Reports

- `EvidenceReference` is implemented as a core model with selected adapter telemetry, diagnostic, and schema-version diagnostic attachment paths.
- EvidenceReference persistence, CLI rendering, examples, approval attachment, and broad automatic attachment remain unimplemented.
- `WorkReportContract` and `WorkReport` core models are implemented.
- Explicit in-memory terminal local report generation, runtime result exposure, and executor-integrated report-bearing local execution APIs are implemented.
- Explicit local work report artifact storage is implemented through `WorkReportArtifactStore`.
- Automatic runtime report generation for every run is not implemented.
- Automatic report artifact writing for every executor path is not implemented; selected opt-in paths do not create a universal runtime guarantee.
- CLI report rendering/export and report schemas are not implemented.
- Approval-resume and cancellation report-bearing APIs are not implemented.

## Security And Governance

- Level 3 and Level 4 autonomy are declaration-only and denied by default.
- Level 3 and Level 4 autonomy enablement is not implemented in v0.
- The conservative policy engine is not enterprise RBAC.
- No IdP integration exists.
- No real secret provider exists.
- `secret.read` is denied by default.
- Maintainer decision through `0.3.0-preview.1`: `serde_yaml` is accepted for trusted local project files only, despite being deprecated and depending on `unsafe-libyaml`.
- v0 does not claim hardened malicious-spec parsing, remote YAML ingestion safety, parser sandboxing, or production-grade parser isolation.
- `YAML-001` tracks replacing or isolating the YAML parser before any production-readiness or adversarial-input hardening claim.

## SDK And Contracts

- Rust remains the source of truth.
- The TypeScript SDK emits specs and does not execute workflows.
- TypeScript types are manually synchronized with JSON Schema and Rust models in v0, with package-level and repository-level contract tests against Rust CLI validation.
- JSON Schemas are manually checked-in v0 compatibility artifacts. CI verifies presence, version pinning, examples, and representative SDK output, but schemas and TypeScript types are not generated from Rust yet.
- Full schema-generated TypeScript types are deferred.

## CLI

- `workflow-os --version` and `workflow-os version` report bounded CLI version
  identity without requiring a project.
- The CLI supports local validate, run, status, approve, inspect, and doctor commands.
- The CLI implements existing-repository onboarding scaffolding through
  `workflow-os init-repo-governance` and agent instruction scaffolding through
  `workflow-os init-agent-harness`.
- `workflow-os first-run` emits bounded report-ready posture and review-only
  recommendations. Its ordinary path does not start a run, create runtime state,
  execute local checks, call providers, inspect raw source contents, or generate
  workflows automatically. An explicit `--write-report-artifact` path may write
  the exact validated quiet-success artifact and fails closed on incompatible
  durable contract state.
- `workflow-os first-run --recommendation <id>` and
  `workflow-os author workflow --from-recommendation <id> --dry-run` bridge a
  recommendation to review-only authoring posture; they do not make the
  recommendation active governance.
- `workflow-os run` and approval resume fail closed when no local handler is registered, unless `--mock-all-local-skills` is explicitly provided for mock execution.
- Local mock handlers are trusted local preview tooling only; they are not a production skill plugin system.
- CLI `--json` output remains experimental through `0.3.0-preview.1` and is not yet a versioned stable machine-output contract.
- The CLI does not implement docs generation, generic live adapter execution
  commands, distributed worker commands, hosted operation, or production
  deployment commands.
- The CLI does not expose real local build/check skill handlers for self-governance dogfooding; current dogfood execution uses explicit deterministic mock handling.

## Validation And Execution

- Semantic validation is deterministic but intentionally scoped to v0 concepts.
- Full input/output schema validation is not implemented in the executor.
- Nested contract validation and deep payload-level redaction inspection are deferred. v0 audit avoids raw skill inputs, records references/summaries, and redacts sensitive-looking audit summaries deterministically.
- Mock local skill behavior proves the kernel path only and must not be presented as production AI or business decisioning.
