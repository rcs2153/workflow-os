# v0 Release Readiness

This document records the current v0 hardening assessment for Workflow OS as a public local kernel preview.

## Release Position

Workflow OS v0 is ready to be described as a **public local kernel preview**.

Current preview version: `0.1.0-preview.1`.

That means the project is suitable for public evaluation of the local-first kernel: specs, validation, local execution, event-sourced state, policy, approvals, audit/observability foundations, CLI ergonomics, TypeScript spec-generation helpers, and adapter contracts.

It must not be described as a production deployment platform, hosted service, production distributed runtime, adapter-complete framework, enterprise deployment system, UI product, or Level 3/4 autonomy system.

## v0 Feature List

- Rust-owned canonical core primitives.
- YAML project/spec parsing with schema versioning.
- Checked-in JSON Schemas under `schemas/v0/`.
- Deterministic project loader and semantic validator.
- Event-sourced workflow run model.
- Local filesystem durable state backend and test-only in-memory backend.
- Single-step local executor with explicit local handler registration.
- Approval pause/resume for local runs.
- Bounded retry, escalation, cancellation, and timeout representation.
- Conservative policy and capability model.
- Audit and observability sink interfaces with local sinks.
- CLI commands: `validate`, `run`, `status`, `approve`, `inspect`, `doctor`.
- TypeScript SDK for spec generation only.
- Vertical-slice approval example with explicitly enabled deterministic local mock skill.
- Generic adapter contracts without real external adapters.

## Preview Scope

The v0 local kernel preview is public so contributors and early evaluators can inspect the architecture, run the vertical slice, exercise the validator and CLI, and review the runtime invariants.

The preview supports local kernel evaluation only. It does not imply:

- production deployment readiness
- production distributed worker readiness
- write-capable GitHub, Jira, CI, SaaS, or HTTP adapter readiness
- enterprise RBAC, IdP, or secret-provider readiness
- Level 3/4 autonomy enablement
- support for sensitive real-world workflow automation without additional review

## v0 Non-Goals

- Write-capable GitHub, Jira, CI, generic HTTP, or SaaS adapters.
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

CI also runs the schema/SDK/example synchronization gate:

- required `schemas/v0/*.schema.json` files are present and pinned to `workflowos.dev/v0`
- checked-in example projects validate through the Rust CLI
- SDK-generated minimal and approval-gated projects validate through the Rust CLI
- SDK-generated invalid and schema-version-mismatched projects fail Rust validation

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

Before tagging a v0 local kernel preview release:

- CI is green on the release commit.
- `CHANGELOG.md` includes the release entry.
- Crate and package versions match the intended release version.
- Rust crates and TypeScript packages use `0.1.0-preview.1` for the first public local kernel preview unless a later release deliberately supersedes it.
- Known limitations are linked from the release notes.
- Security review is current.
- Dependency audits pass or exceptions are documented.
- Vertical-slice example validates and runs locally.
- No documentation implies real external adapters or hosted production operation exist.
- CLI examples that use mock skills show `--mock-all-local-skills` explicitly.
- Release notes call the release a local kernel preview, not a production runtime or broad release candidate.

## Upgrade And Deprecation Stance

Before `1.0.0`, public contracts may evolve. Breaking changes must still be documented in release notes, tied to an ADR where architecture-significant, and accompanied by migration guidance where practical.

Deprecated spec fields or CLI behavior must remain explicit. Experimental features must be clearly marked.

## Readiness Assessment

Ready for:

- public local kernel preview
- local kernel development
- contributor onboarding
- core primitive modeling
- validator/runtime hardening
- future adapter design against documented contracts

Not ready for:

- production deployment
- production distributed runtime
- external write integrations
- distributed operation
- sensitive real-world workflow automation without additional review
- enterprise security certification
