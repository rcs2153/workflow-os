# v0 Release Readiness

This document records the current v0 readiness assessment for Workflow OS as a
public local kernel preview with a narrow read-only integration preview.

## Release Position

Workflow OS v0 is ready to be described as a **public local kernel preview**.

Current preview version: `0.2.0-preview.1`.

That means the project is suitable for public evaluation of the local-first
kernel: specs, validation, sequential local execution, event-sourced state,
policy, approvals, audit/observability foundations, CLI ergonomics,
TypeScript spec-generation helpers, read-only adapter contracts, selected
evidence-reference attachment paths, and early work-report model/helper APIs.

The `0.1.0-preview.1` layer established the local kernel preview. The
`0.2.0-preview.1` posture adds a narrow public read-only integration preview
for GitHub, Jira, and GitHub Actions / CI.

It must not be described as a production deployment platform, hosted service,
production distributed runtime, adapter-complete framework, enterprise
deployment system, UI product, write-capable integration layer, or Level 3/4
autonomy system.

## v0 Feature List

- Rust-owned canonical core primitives.
- YAML project/spec parsing with schema versioning.
- Checked-in JSON Schemas under `schemas/v0/`.
- Deterministic project loader and semantic validator.
- Event-sourced workflow run model.
- Local filesystem durable state backend and test-only in-memory backend.
- Sequential local executor with explicit local handler registration.
- Governed multi-step local workflow execution for ordered local steps.
- Approval pause/resume for local runs.
- Bounded retry, escalation, cancellation, and timeout representation.
- Conservative policy and capability model.
- Audit and observability sink interfaces with local sinks.
- CLI commands: `validate`, `run`, `status`, `approve`, `inspect`,
  `doctor`, and local scaffolding helpers.
- TypeScript SDK for spec generation only.
- Vertical-slice approval example with explicitly enabled deterministic local
  mock skill.
- GitHub, Jira, and GitHub Actions / CI read-only adapter preview surfaces.
- Fixture-backed read-only integration examples and contract checks.
- Opt-in maintainer live smoke test paths for narrow read-only provider checks.
- Adapter telemetry mapping for controlled fixture-backed examples.
- `EvidenceReference` core model with selected adapter telemetry, diagnostic,
  and schema-version diagnostic attachment paths.
- `WorkReportContract` and `WorkReport` core models.
- Explicit in-memory terminal local report generation/result helper APIs.
- Executor-integrated report-bearing local execution API.
- Explicit local work-report artifact storage API.
- Self-governance dogfood project for kernel-governed planning/docs work.
- Agent harness documentation scaffold for local coding-agent adoption.

## Preview Scope

The v0 preview is public so contributors and early evaluators can inspect the
architecture, run local examples, exercise the validator and CLI, review runtime
invariants, evaluate fixture-backed read-only integrations, and understand the
governed-work roadmap.

The preview supports local kernel and read-only integration evaluation only. It
does not imply:

- production deployment readiness
- production distributed worker readiness
- hosted service readiness
- write-capable GitHub, Jira, CI, SaaS, or HTTP adapter readiness
- generic live adapter execution from arbitrary workflow specs
- enterprise RBAC, IdP, or secret-provider readiness
- UI, marketplace, or package registry behavior
- Level 3/4 autonomy enablement
- support for sensitive real-world workflow automation without additional review

## v0 Non-Goals

- Write-capable GitHub, Jira, CI, generic HTTP, or SaaS adapters.
- CI rerun, workflow dispatch, cancellation, artifact mutation, or check
  mutation.
- Generic live adapter execution.
- OAuth, webhook, or external trigger ingestion services.
- Distributed workers.
- Production database backend.
- Enterprise RBAC or IdP integration.
- Real secret provider integration.
- UI.
- Marketplace or package registry behavior.
- Automatic runtime report generation for every run.
- Automatic report artifact writing from executor paths.
- CLI report rendering or report export.
- EvidenceReference persistence or broad automatic attachment.
- Reasoning Lineage / Claim Graph implementation.
- Composable Harness Contract runtime execution.
- Side-effect boundary execution or write behavior.
- Level 3/4 autonomy by default.

## Quality Gates

The expected v0 quality gates are:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- `cargo metadata --locked --format-version 1`
- `npm ci`
- `npm run check`
- `npm run check:contracts`
- `npm run check:integrations`
- `npm audit --audit-level=moderate`
- Rust dependency audit in CI

CI covers Rust formatting, clippy, tests, docs, Cargo lock validation,
TypeScript install, typecheck, lint, tests, npm lock validation, docs checks,
schema/SDK/example synchronization, read-only integration contract checks, and
dependency audits.

The schema/SDK/example gate verifies:

- required `schemas/v0/*.schema.json` files are present and pinned to
  `workflowos.dev/v0`
- checked-in example projects validate through the Rust CLI
- SDK-generated minimal and approval-gated projects validate through the Rust
  CLI
- SDK-generated invalid and schema-version-mismatched projects fail Rust
  validation

`npm run check:integrations` verifies read-only GitHub, Jira, and CI/GitHub
Actions adapter contracts and fixture-backed examples without live credentials.

## OSS Readiness

Present:

- Apache 2.0 license.
- Code of conduct.
- Contributing guide.
- Security policy and vulnerability reporting process.
- Changelog.
- Maintainers file with current maintainer contact.
- `CODEOWNERS`.
- Roadmap.
- ADR process.
- Semantic versioning policy.
- Release process.
- Issue templates.
- Pull request template.
- GitHub Actions CI.

Repository branch/ruleset protection must be verified in GitHub before public
promotion. Local repository files can document ownership and checks, but GitHub
settings are what prevent direct mutation of `main`.

## Security Readiness

Current posture:

- Rust core denies unsafe code.
- Specs reject secret-like keys and values.
- Redaction wrappers exist for sensitive values.
- Conservative policy denies unknown actions and capabilities.
- External write and adapter write behavior are denied in v0.
- Level 3/4 execution is denied by default.
- Audit/log paths use summaries and references rather than raw sensitive
  payloads.
- Dependency risk is documented.
- Read-only provider credentials are opt-in maintainer-only and must not appear
  in specs, fixtures, examples, docs, screenshots, issues, or pull requests.

Accepted preview limitations:

- `serde_yaml` remains accepted for trusted local project files only.
- v0 does not claim hardened malicious-spec parsing, remote YAML ingestion
  safety, parser sandboxing, or production-grade parser isolation.
- Deterministic preview redaction is not enterprise DLP.
- The Jira sandbox API token referenced in live-smoke process history should be
  rotated before public promotion.

See [SECURITY_REVIEW.md](../security/SECURITY_REVIEW.md),
[THREAT_MODEL.md](../security/THREAT_MODEL.md), and
[V0_KNOWN_LIMITATIONS.md](V0_KNOWN_LIMITATIONS.md).

## Release Checklist

Before public promotion of the current v0 preview:

- CI is green on the release commit.
- `CHANGELOG.md` includes the release entry.
- Crate and package versions match the intended release version.
- Known limitations are linked from the release notes.
- Security review is current.
- Dependency audits pass or exceptions are documented.
- Vertical-slice and read-only fixture-backed examples validate and run locally.
- No documentation implies write-capable adapters, generic live adapter
  execution, hosted production operation, distributed workers, production
  backends, CLI report rendering, or Level 3/4 autonomy exist.
- CLI examples that use mock skills show `--mock-all-local-skills` explicitly.
- Release notes call the release a public local kernel/read-only integration
  preview, not a production runtime or broad release candidate.
- Maintainer and vulnerability-reporting contact paths are valid.
- GitHub branch/ruleset protection for `main` is configured and verified.
- Jira sandbox API token rotation is confirmed or tracked as an explicit
  operational follow-up before announcement.

## Upgrade And Deprecation Stance

Before `1.0.0`, public contracts may evolve. Breaking changes must still be
documented in release notes, tied to an ADR where architecture-significant, and
accompanied by migration guidance where practical.

Deprecated spec fields or CLI behavior must remain explicit. Experimental
features must be clearly marked.

## Readiness Assessment

Ready for:

- public local kernel preview
- public read-only integration preview
- local kernel development
- contributor onboarding
- core primitive modeling
- validator/runtime hardening
- future adapter design against documented contracts
- early evaluation of evidence-reference and work-report foundations
- kernel-governed agent-harness adoption experiments

Not ready for:

- production deployment
- production distributed runtime
- hosted service operation
- external write integrations
- generic live adapter execution
- distributed operation
- sensitive real-world workflow automation without additional review
- enterprise security certification
- Level 3/4 autonomy enablement
