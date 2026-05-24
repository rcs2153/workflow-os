# v0 Release Readiness

This document records the current v0 hardening assessment for Workflow OS.

## Release Position

Workflow OS is ready to be described as a local-first v0 kernel foundation. It is not ready to be described as a production deployment platform, hosted service, distributed runtime, real integration framework, or UI product.

## v0 Feature List

- Rust-owned canonical core primitives.
- YAML project/spec parsing with schema versioning.
- Checked-in JSON Schemas under `schemas/v0/`.
- Deterministic project loader and semantic validator.
- Event-sourced workflow run model.
- Local filesystem durable state backend and test-only in-memory backend.
- Single-step local executor.
- Approval pause/resume for local runs.
- Bounded retry, escalation, cancellation, and timeout representation.
- Conservative policy and capability model.
- Audit and observability sink interfaces with local sinks.
- CLI commands: `validate`, `run`, `status`, `approve`, `inspect`, `doctor`.
- TypeScript SDK for spec generation only.
- Vertical-slice approval example with deterministic local mock skill.
- Generic adapter contracts without real external adapters.

## v0 Non-Goals

- Real GitHub, Jira, CI, generic HTTP, or SaaS adapters.
- Distributed workers.
- Production database backend.
- Enterprise RBAC or IdP integration.
- Real secret provider integration.
- UI.
- Marketplace or package registry behavior.
- Level 3/4 autonomy by default.
- Multi-step branch execution.

## Quality Gates

The expected v0 quality gates are:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- `cargo metadata --locked --format-version 1`
- `npm ci`
- `npm run check`
- `npm audit --audit-level=moderate`
- Rust dependency audit in CI

CI covers Rust formatting, clippy, tests, docs, Cargo lock validation, TypeScript install, typecheck, lint, tests, npm lock validation, docs checks, and dependency audits.

## OSS Readiness

Present:

- Apache 2.0 license.
- Code of conduct.
- Contributing guide.
- Security policy and vulnerability reporting process.
- Changelog.
- Maintainers file.
- Roadmap.
- ADR process.
- Semantic versioning policy.
- Release process.
- Issue templates.
- Pull request template.

## Security Readiness

Current posture:

- Rust core denies unsafe code.
- Specs reject secret-like keys and values.
- Redaction wrappers exist for sensitive values.
- Conservative policy denies unknown actions and capabilities.
- External write and adapter invocation are denied in v0.
- Level 3/4 execution is denied by default.
- Audit/log paths use summaries and references rather than raw sensitive payloads.
- Dependency risk is documented.

See [SECURITY_REVIEW.md](../security/SECURITY_REVIEW.md) and [THREAT_MODEL.md](../security/THREAT_MODEL.md).

## Release Checklist

Before tagging a v0 release:

- CI is green on the release commit.
- `CHANGELOG.md` includes the release entry.
- Crate and package versions match the intended release version.
- Known limitations are linked from the release notes.
- Security review is current.
- Dependency audits pass or exceptions are documented.
- Vertical-slice example validates and runs locally.
- No documentation implies real external adapters or hosted production operation exist.

## Upgrade And Deprecation Stance

Before `1.0.0`, public contracts may evolve. Breaking changes must still be documented in release notes, tied to an ADR where architecture-significant, and accompanied by migration guidance where practical.

Deprecated spec fields or CLI behavior must remain explicit. Experimental features must be clearly marked.

## Readiness Assessment

Ready for:

- local kernel development
- contributor onboarding
- core primitive modeling
- validator/runtime hardening
- future adapter design against documented contracts

Not ready for:

- production deployment
- external write integrations
- distributed operation
- sensitive real-world workflow automation without additional review
- enterprise security certification
