# Dependency Policy

Workflow OS keeps dependencies intentionally small until the local-first kernel requires more.

## Current Dependency Justification

- `typescript`: development-only dependency used to typecheck the TypeScript SDK workspace. The SDK exists for ergonomics and must remain compatible with the canonical Rust core model.
- `serde`: runtime dependency used for stable serialization and deserialization of public Rust core primitives.
- `serde_json`: runtime dependency used for deterministic canonical JSON serialization when hashing parsed YAML specs.
- `serde_yaml`: runtime dependency used to parse the primary human-authored YAML project/spec format. This crate is deprecated and pulls in `unsafe-libyaml`; it is accepted for `0.1.0-preview.1` only because v0 specs are trusted local project files, the parser is wired through source-aware diagnostics and canonical hashing, and replacing it now would have broad compatibility blast radius. `YAML-001` tracks replacement or parser isolation before any production-readiness or malicious-spec hardening claim.
- `sha2`: runtime dependency used for deterministic SHA-256 spec content hashing.
- `time`: runtime dependency used for consistent RFC 3339 UTC timestamps.
- `ureq`: runtime dependency used only by optional live read-only adapter clients. The standard library does not provide HTTPS client support, and Phase 2 needs a small blocking HTTP client to prove read-only adapter contracts against real systems. Fixture tests remain the default CI path; live GitHub, Jira, and GitHub Actions calls are opt-in through environment configuration.
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
