# Changelog

All notable changes to Workflow OS will be documented in this file.

This project follows the versioning policy in [docs/release/SEMVER.md](docs/release/SEMVER.md).

## Unreleased

### Added

- Initial repository foundation for the Rust-core and TypeScript-SDK monorepo.
- Engineering standard and project charter.
- Open-source governance files, release documentation, ADRs, issue templates, pull request template, and CI quality gates.
- v0 hardening documentation for release readiness, known limitations, threat model, security review, and local troubleshooting.
- Phase 2 read-only integration posture and GitHub/Jira read-only adapter foundations.
- GitHub read-only fixture and live-client abstractions for repository metadata, default branch, file content references, pull request metadata, pull request diff summaries, changed files, comments, and check-run summaries.
- Jira read-only fixture and live-client abstractions for issue metadata, summary, description references, comments, status, priority, labels, assignee/reporter display data, and project metadata.
- GitHub Actions read-only fixture and live-client abstractions for workflow run metadata, job summaries, check summaries, failure summaries, log references, and bounded redacted log excerpts.
- GitHub read-only setup, token-scope, and troubleshooting documentation.
- Jira read-only setup, token-scope, and troubleshooting documentation.
- GitHub Actions read-only setup and CI log redaction documentation.

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
