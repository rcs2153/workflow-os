# Workflow OS

Workflow OS is an enterprise-grade open-source framework for defining, validating, testing, executing, governing, observing, and scaling AI-driven workflows.

The v0 focus is a correct local-first kernel: declarative workflow specs, validation, runtime state, policy, audit, observability, and CLI foundations. Workflow OS is not a UI product in v0, and it does not yet provide GitHub, Jira, CI, SaaS, or other real external integrations.

## Product Boundary

Workflow OS Core is a local-first, declarative framework for governed AI workflows through pluggable skills and adapters.

Workflow OS Core is not:

- A replacement for Temporal, Airflow, GitHub Actions, or Zapier.
- A generic business process management engine.
- A chat agent framework.
- A one-off repository, ticketing, or CI automation tool.
- An enterprise SaaS application.
- A UI product in v0.

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
target/debug/workflow-os --project-dir examples/vertical-slice-approval run ex/review
```

The run pauses for approval and prints a `run_id` plus `approval_id`.

```sh
target/debug/workflow-os --project-dir examples/vertical-slice-approval approve <run-id> <approval-id> --actor user/example-approver --reason reviewed-local-example
target/debug/workflow-os --project-dir examples/vertical-slice-approval inspect <run-id>
```

This example uses a deterministic local mock skill. It exercises the v0 kernel path without external services, secrets, real adapters, or production deployment claims.

## Current Status

Workflow OS has a local-first v0 kernel foundation: declarative specs, validation, event-sourced local execution, approvals, policy checks, durable local state, audit/observability signals, CLI commands, and TypeScript spec-generation helpers. Real external adapters and production deployment backends have not been implemented.

See [docs/release/V0_READINESS.md](docs/release/V0_READINESS.md) and [docs/release/V0_KNOWN_LIMITATIONS.md](docs/release/V0_KNOWN_LIMITATIONS.md) for the current readiness assessment.

## License

Workflow OS is licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
