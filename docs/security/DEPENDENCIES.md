# Dependency Policy

Workflow OS keeps dependencies intentionally small until the local-first kernel requires more.

## Current Dependency Justification

- `typescript`: development-only dependency used to typecheck the TypeScript SDK workspace. The SDK exists for ergonomics and must remain compatible with the canonical Rust core model.
- `actions/checkout`: GitHub Actions helper used by CI to check out repository contents.
- `actions/setup-node`: GitHub Actions helper used by CI to install Node.js and enable npm cache support.
- `dtolnay/rust-toolchain`: GitHub Actions helper used by CI to install stable Rust with `clippy` and `rustfmt`.
- `rustsec/audit-check`: GitHub Actions helper used by CI for Rust dependency advisory checks.

## Review Requirements

Future dependency additions must explain:

- Why the dependency is necessary.
- Why the standard library or existing tooling is insufficient.
- Whether the dependency affects runtime, build time, development, or CI only.
- License compatibility.
- Supply-chain and maintenance risk.
- Security implications.
