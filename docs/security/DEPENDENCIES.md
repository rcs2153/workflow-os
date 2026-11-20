# Dependency Policy

Workflow OS keeps dependencies intentionally small until the local-first kernel requires more.

## Current Dependency Justification

- `typescript`: development-only dependency used to typecheck the TypeScript SDK workspace. The SDK exists for ergonomics and must remain compatible with the canonical Rust core model.
- `serde`: runtime dependency used for stable serialization and deserialization of public Rust core primitives.
- `serde_json`: runtime dependency used for deterministic canonical JSON serialization when hashing parsed YAML specs.
- `serde_yaml`: runtime dependency used to parse the primary human-authored YAML project/spec format. This crate is deprecated and pulls in `unsafe-libyaml`; it is accepted through `0.4.0-preview.1` only because v0 specs are trusted local project files, the parser is wired through source-aware diagnostics and canonical hashing, and replacing it now would have broad compatibility blast radius. `YAML-001` tracks replacement or parser isolation before any production-readiness or malicious-spec hardening claim.
- `sha2`: runtime dependency used for deterministic SHA-256 spec content hashing.
- `time`: runtime dependency used for consistent RFC 3339 UTC timestamps.
- `ureq`: runtime dependency used only by optional live read-only adapter clients. The standard library does not provide HTTPS client support, and Phase 2 needs a small blocking HTTP client to prove read-only adapter contracts against real systems. Fixture tests remain the default CI path; live GitHub, Jira, and GitHub Actions calls are opt-in through environment configuration.
- `actions/checkout`: GitHub Actions helper used by CI to check out repository contents.
- `actions/setup-node`: GitHub Actions helper used by CI to install Node.js and enable npm cache support.
- `dtolnay/rust-toolchain`: GitHub Actions helper used by CI to install stable Rust with `clippy` and `rustfmt`.
- `cargo-audit 0.22.2`: Rust dependency advisory scanner used by CI against an
  exact RustSec advisory-database revision. CI verifies the checked-out
  revision and disables database fetching during the scan so the audit result
  is reproducible and cannot silently consume different advisory data.

Required CI checks out official `RustSec/advisory-db` `main` separately,
reports its exact resolved SHA and age, then audits those bytes with
`cargo-audit 0.22.2 --no-fetch`. `scripts/check-rustsec-freshness.mjs` rejects
missing or malformed commit metadata, future dates and revisions more than
seven days old. A green run is a point-in-time scan, not perpetual assurance.
Current-source release admission needs a fresh audit; older CI is not a
substitute. npm dependencies are audited with `npm audit --audit-level=moderate`.

No exceptional advisory pin is active. A future upstream parsing incident
requires a reviewed owner, reason, exact official revision, remediation link
and explicit expiry within seven days. The pin must obey the same freshness
limit and fail at expiry; it must not waive advisories or audit failures.
See the [release feed policy](../release/0.4.0-preview.1_RELEASE_CHECKLIST.md#rustsec-feed-policy).
The trusted-local-file `serde_yaml`/`unsafe-libyaml` maintenance exception above
does not accept new vulnerabilities or claim safe malicious-spec parsing.

## Review Requirements

Future dependency additions must explain:

- Why the dependency is necessary.
- Why the standard library or existing tooling is insufficient.
- Whether the dependency affects runtime, build time, development, or CI only.
- License compatibility.
- Supply-chain and maintenance risk.
- Security implications.
