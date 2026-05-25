# Workflow OS

Workflow OS v0 is a **public local kernel preview** of a local-first framework for defining, validating, testing, executing, governing, observing, and eventually scaling AI-driven workflows.

Current preview version: `0.1.0-preview.1`.

This preview exposes the core kernel shape: declarative workflow and skill specs, deterministic validation, event-sourced local execution, policy gates, approval pause/resume, bounded retry and escalation semantics, local durable state, audit/observability signals, CLI commands, TypeScript spec-generation helpers, and adapter contracts.

It is **not** a production distributed runtime. It is not a hosted product, adapter-complete framework, enterprise deployment platform, or Level 3/4 autonomy system. v0 has no real GitHub, Jira, CI, SaaS, or generic HTTP adapters; no distributed workers; no production database backend; no UI; and no marketplace/package registry. Level 1 and Level 2 autonomy are the only default posture. Level 3 and Level 4 are declaration-only and denied by default.

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

Workflow OS Core is not:

- A replacement for Temporal, Airflow, GitHub Actions, or Zapier.
- A generic business process management engine.
- A chat agent framework.
- A one-off repository, ticketing, or CI automation tool.
- An enterprise SaaS application.
- A UI product in v0.
- A production distributed runtime.
- A real GitHub, Jira, or CI integration layer in v0.
- A Level 3/4 autonomy runtime by default.

See [docs/PROJECT_CHARTER.md](docs/PROJECT_CHARTER.md) and [docs/ENGINEERING_STANDARD.md](docs/ENGINEERING_STANDARD.md) before contributing.

## Repository Layout

```text
crates/
  workflow-core/        Canonical Rust core home.
  workflow-cli/         Rust CLI home.
packages/
  sdk-typescript/       TypeScript SDK ergonomics layer.
examples/
  vertical-slice-approval/
docs/
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
```

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

## Current Status

Workflow OS currently has a local-first v0 kernel foundation: declarative specs, validation, event-sourced local execution, approvals, policy checks, durable local state, audit/observability signals, CLI commands, and TypeScript spec-generation helpers. Real external adapters, distributed workers, production deployment backends, hosted services, and Level 3/4 execution by default have not been implemented.

See [docs/release/V0_READINESS.md](docs/release/V0_READINESS.md) and [docs/release/V0_KNOWN_LIMITATIONS.md](docs/release/V0_KNOWN_LIMITATIONS.md) for the current readiness assessment.

## License

Workflow OS is licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
