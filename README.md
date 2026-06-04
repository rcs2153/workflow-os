# Workflow OS

Workflow OS v0 is a **public local kernel preview** of a local-first framework for defining, validating, testing, executing, governing, observing, and eventually scaling AI-driven workflows.

Current preview version: `0.2.0-preview.1`.

This preview exposes the core kernel shape: declarative workflow and skill specs, deterministic validation, event-sourced local execution, policy gates, approval pause/resume, bounded retry and escalation semantics, local durable state, audit/observability signals, CLI commands, TypeScript spec-generation helpers, and adapter contracts.

It is **not** a production distributed runtime. It is not a hosted product, adapter-complete framework, enterprise deployment platform, or Level 3/4 autonomy system. The `0.1.0-preview.1` release established the local kernel preview. The `0.2.0-preview.1` posture adds a narrow public read-only integration preview for GitHub, Jira, and GitHub Actions / CI. No GitHub/Jira write behavior, CI rerun/dispatch/cancel behavior, generic live adapter execution, distributed workers, production database backend, UI, hosted service, or marketplace/package registry exists. Level 1 and Level 2 autonomy are the only default posture. Level 3 and Level 4 are declaration-only and denied by default.

## Product Boundary

Workflow OS Core is a local-first, declarative kernel for governed AI workflows through explicit local skills and future adapters.

In v0, Core does:

- Load `workflow-os.yml` projects and versioned YAML specs.
- Validate workflows, skills, policies, and tests before execution.
- Execute a narrow single-step local workflow path.
- Persist and rehydrate event-sourced local runs.
- Enforce conservative policy checks before meaningful runtime actions.
- Pause and resume approval-gated local runs.
- Record local audit and observability signals.
- Provide CLI commands for local validation, execution, approval, status, inspection, and doctor checks.
- Provide a TypeScript SDK for spec generation only.
- Define adapter contracts. `0.2.0-preview.1` adds GitHub, Jira, and GitHub Actions read-only adapters for fixture-first and opt-in live evaluation.

Workflow OS Core is not:

- A replacement for Temporal, Airflow, GitHub Actions, or Zapier.
- A generic business process management engine.
- A chat agent framework.
- A one-off repository, ticketing, or CI automation tool.
- An enterprise SaaS application.
- A UI product in v0.
- A production distributed runtime.
- A GitHub/Jira write or CI write/rerun/dispatch integration layer in v0.
- A Level 3/4 autonomy runtime by default.

See [docs/PROJECT_CHARTER.md](docs/PROJECT_CHARTER.md) and [docs/ENGINEERING_STANDARD.md](docs/ENGINEERING_STANDARD.md) before contributing.

## User Guide

The [Workflow OS User Guide](docs/user-guide/README.md) provides RC1 internal evaluation documentation: a rewritten field guide, a fillable workbook, and safe evaluation paths for the local kernel, vertical slice, read-only fixture adapters, and adapter telemetry inspection.

The user guide preserves the operating-model ideas from the earlier field guide while keeping the current implementation boundary explicit: local kernel preview, public read-only integration preview, no write-capable adapters, no production backend, no distributed workers, no hosted service, no UI, and no Level 3/4 autonomy enablement.

## Repository Layout

```text
crates/
  workflow-core/        Canonical Rust core home.
  workflow-cli/         Rust CLI home.
packages/
  sdk-typescript/       TypeScript SDK ergonomics layer.
examples/
  vertical-slice-approval/
  github-read-only-review-context/
  jira-read-only-intake-quality/
  ci-read-only-failure-summary/
docs/
  user-guide/           Field guide, workbook, and RC1 internal evaluation guide.
  adr/                  Architecture decision records.
  architecture/         Architecture documentation.
  concepts/             Public concepts.
  specs/                Spec documentation.
  runtime/              Runtime invariants and behavior.
  cli/                  CLI documentation.
  sdk/                  SDK documentation.
  operations/           Operational runbooks.
  security/             Security documentation.
  release/              Versioning and release process.
```

## Local Checks

Required local tools:

- Rust toolchain with `cargo`, `rustfmt`, and `clippy`.
- Node.js 20 or newer.
- npm 10 or newer.

Run the baseline checks:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
npm ci
npm run check
npm run check:integrations
```

`npm run check:integrations` runs the Phase 2 read-only adapter contract gate for GitHub, Jira, and CI/GitHub Actions using offline fixtures. It does not require live credentials.

The Phase 2 gate reflects `0.2.0-preview.1` read-only adapter preview work. It is fixture-first, credential-free in normal CI, and does not imply write support, production integration readiness, or broad live provider compatibility.

## Try The Vertical Slice

Build the CLI and run the local approval example:

```sh
cargo build -p workflow-cli --bin workflow-os
target/debug/workflow-os --project-dir examples/vertical-slice-approval validate
target/debug/workflow-os --project-dir examples/vertical-slice-approval --mock-all-local-skills run ex/review
```

The run pauses for approval and prints a `run_id` plus `approval_id`.

```sh
target/debug/workflow-os --project-dir examples/vertical-slice-approval --mock-all-local-skills approve <run-id> <approval-id> --actor user/example-approver --reason reviewed-local-example
target/debug/workflow-os --project-dir examples/vertical-slice-approval inspect <run-id>
```

This example uses an explicitly enabled deterministic local mock skill. The CLI does not execute arbitrary declared `local/*` skills by default. The example exercises the v0 kernel path without external services, secrets, real adapters, distributed workers, production backends, or production deployment claims.

## Try The Phase 2 GitHub Read-Only Example

This example exists to evaluate `0.2.0-preview.1` read-only adapter behavior. It is a fixture-backed preview example, not production GitHub automation.

The GitHub read-only reference example uses fixture data and does not require GitHub credentials:

```sh
target/debug/workflow-os --project-dir examples/github-read-only-review-context validate
target/debug/workflow-os --project-dir examples/github-read-only-review-context --mock-all-local-skills run ex/gh
```

The run pauses for approval before producing the final recommendation summary.

```sh
target/debug/workflow-os --project-dir examples/github-read-only-review-context --mock-all-local-skills approve <run-id> <approval-id> --actor user/example-reviewer --reason reviewed-fixture-context
target/debug/workflow-os --project-dir examples/github-read-only-review-context inspect <run-id>
```

This example reads fixture-backed pull request metadata and changed files through the GitHub read-only adapter contract. It does not post comments, create branches, open pull requests, update GitHub, or require live credentials.

## Try The Phase 2 Jira Read-Only Example

This example exists to evaluate `0.2.0-preview.1` read-only adapter behavior. It is a fixture-backed preview example, not production Jira automation.

The Jira read-only reference example uses fixture data and does not require Jira credentials:

```sh
target/debug/workflow-os --project-dir examples/jira-read-only-intake-quality validate
target/debug/workflow-os --project-dir examples/jira-read-only-intake-quality --mock-all-local-skills run ex/jira
```

The run pauses for approval before producing the final intake-quality recommendation.

```sh
target/debug/workflow-os --project-dir examples/jira-read-only-intake-quality --mock-all-local-skills approve <run-id> <approval-id> --actor user/example-reviewer --reason reviewed-fixture-intake
target/debug/workflow-os --project-dir examples/jira-read-only-intake-quality inspect <run-id>
```

This example reads fixture-backed issue metadata, description presence, and comments through the Jira read-only adapter contract. It does not update Jira, add comments, change status, assign users, change labels, or require live credentials.

## Try The Phase 2 CI Read-Only Example

This example exists to evaluate `0.2.0-preview.1` read-only adapter behavior. It is a fixture-backed preview example, not production CI automation.

The CI read-only reference example uses fixture-backed GitHub Actions data and does not require GitHub credentials:

```sh
target/debug/workflow-os --project-dir examples/ci-read-only-failure-summary validate
target/debug/workflow-os --project-dir examples/ci-read-only-failure-summary --mock-all-local-skills run ex/ci
```

The run pauses for approval before producing the final failure diagnosis summary.

```sh
target/debug/workflow-os --project-dir examples/ci-read-only-failure-summary --mock-all-local-skills approve <run-id> <approval-id> --actor user/example-reviewer --reason reviewed-fixture-ci-context
target/debug/workflow-os --project-dir examples/ci-read-only-failure-summary inspect <run-id>
```

This example reads fixture-backed workflow run metadata, job status, failure context, and bounded redacted log excerpts through the GitHub Actions read-only adapter contract. It does not rerun CI, dispatch workflows, cancel workflows, modify checks, write comments, attempt auto-repair, or require live credentials.

## Current Status

Workflow OS currently has a local-first v0 kernel foundation: declarative specs, validation, event-sourced local execution, approvals, policy checks, durable local state, audit/observability signals, CLI commands, and TypeScript spec-generation helpers. `0.2.0-preview.1` adds GitHub/Jira/GitHub Actions read-only adapter preview work. GitHub/Jira writes, CI rerun/dispatch/cancel behavior, generic live adapter execution, production integration readiness, distributed workers, production deployment backends, hosted services, and Level 3/4 execution by default have not been implemented.

See [docs/release/V0_READINESS.md](docs/release/V0_READINESS.md) and [docs/release/V0_KNOWN_LIMITATIONS.md](docs/release/V0_KNOWN_LIMITATIONS.md) for the current readiness assessment.

## License

Workflow OS is licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
