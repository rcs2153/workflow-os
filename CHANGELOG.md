# Changelog

All notable changes to Workflow OS will be documented in this file.

This project follows the versioning policy in [docs/release/SEMVER.md](docs/release/SEMVER.md).

## Unreleased

## 0.2.0-preview.1 - Public Read-Only Integration Preview

This release posture adds a narrow public preview for read-only GitHub, Jira, and GitHub Actions / CI integration capability.

It is not production-ready, not hosted, not distributed-runtime-ready, not write-capable, and not Level 3/4 autonomy-ready. Live provider use is opt-in. Normal CI remains fixture-first and credential-free.

### Added

- GitHub read-only adapter preview with fixture and opt-in live-client paths for repository metadata, default branch, file content references, pull request metadata, pull request diff summaries, changed files, comments, and check-run summaries.
- Jira read-only adapter preview with fixture and opt-in live-client paths for issue metadata, summary, description references, comments, status, priority, labels, assignee/reporter display data, and project metadata.
- GitHub Actions / CI read-only adapter preview with fixture and opt-in live-client paths for workflow run metadata, job summaries, check summaries, failure summaries, log references, and bounded redacted log excerpts.
- Adapter telemetry mapping for controlled fixture-backed examples into local runtime-visible adapter audit and observability telemetry records.
- GitHub, Jira, and GitHub Actions maintainer live smoke evidence for one narrow read path per provider family.
- Read-only integration setup, token-scope, live-smoke, troubleshooting, and preview-readiness documentation.

### Limitations And Non-Goals

- GitHub live smoke exercised `octocat/Hello-World` repository metadata, not the approved `rcs2153/workflow-os` repository.
- GitHub Actions live smoke exercised workflow run metadata only.
- Jira live smoke exercised issue metadata only.
- Jira sandbox API token rotation is recommended because a sandbox token was pasted into the local evaluation thread.
- Adapter telemetry is local/runtime-preview telemetry, not production telemetry export.
- Redaction is deterministic preview redaction, not enterprise DLP.
- No generic live adapter execution from arbitrary workflow specs exists.
- No write-capable GitHub, Jira, CI, generic HTTP, webhook, OAuth, hosted, distributed-worker, production-backend, production-telemetry-export, domain-pack, or Level 3/4 autonomy behavior is included.

## 0.1.0-preview.1 - v0 Local Kernel Preview

This is the first public preview posture for Workflow OS: a local-first kernel preview, not a production enterprise runtime or adapter-complete framework.

### Implemented Kernel Capabilities

- Rust-owned canonical core primitives, identifiers, diagnostics, errors, timestamps, content hashing, and redaction helpers.
- Versioned YAML project/spec loading for `workflow-os.yml`, workflows, skills, policies, and tests.
- Checked-in JSON Schemas under `schemas/v0/`.
- Deterministic semantic validation with structured diagnostics.
- Event-sourced workflow run model with schema version, workflow version, spec hash, sequence numbers, transition validation, and deterministic rehydration.
- Local filesystem development backend with safe append validation, duplicate detection, idempotency storage, local locking, corruption detection, and health checks.
- Single-step local executor for explicitly registered local skills.
- Approval pause/resume and CLI approval grant/denial.
- Bounded retry, escalation, cancellation, and timeout representation for the local runtime path.
- Conservative policy and capability model that denies unknown capabilities, Level 3/4 execution, adapter invocation, and external writes by default.
- Runtime audit and observability foundations with local sinks and durable local policy audit records.
- CLI commands: `validate`, `run`, `status`, `approve`, `inspect`, and `doctor`.
- TypeScript SDK helpers for generating Workflow OS specs, with Rust validation contract checks.
- Vertical-slice approval example using an explicitly enabled deterministic local mock skill.
- Generic adapter contracts for future integrations without real external API clients.

### Non-Goals And Unsupported Features

- No production distributed runtime.
- No production database backend.
- No distributed workers.
- No real GitHub adapter.
- No Jira write adapter.
- No CI write, rerun, dispatch, or cancellation adapter.
- No generic HTTP or SaaS adapter.
- No active timeout scheduler.
- No trigger ingestion service.
- No UI.
- No marketplace or package registry.
- No Level 3/4 autonomy enablement.
- No enterprise RBAC, IdP, or secret-provider integration.
- No production skill plugin system; local mock handlers are trusted preview tooling only.
- No generated Rust-to-TypeScript schema pipeline; schemas and TypeScript types are manually synchronized with contract tests.
- No production-readiness claim for sensitive real-world workflow automation.
