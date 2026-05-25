# Contributing

Thank you for helping build Workflow OS. Start by reading:

- [docs/ENGINEERING_STANDARD.md](docs/ENGINEERING_STANDARD.md)
- [docs/PROJECT_CHARTER.md](docs/PROJECT_CHARTER.md)
- [docs/release/SEMVER.md](docs/release/SEMVER.md)

## Development Principles

- Keep changes minimal and scoped.
- Do not implement speculative features.
- Do not add integrations before the local-first kernel is correct.
- Do not present mock-only or placeholder behavior as production behavior.
- Preserve the Rust core as the canonical model.
- Keep TypeScript compatible with the Rust-owned contracts.
- Deny unsafe Rust code unless a future ADR explicitly justifies an exception.

## Local Checks

Run before opening a pull request:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
npm ci
npm run check
npm run check:integrations
```

`npm run check:integrations` is the Phase 2 read-only integration gate. It runs offline GitHub, Jira, and CI/GitHub Actions adapter contract tests plus the fixture-backed examples. It must not require live credentials.

Opt-in live read-only tests are documented in:

- [docs/operations/github-read-only-setup.md](docs/operations/github-read-only-setup.md)
- [docs/operations/jira-read-only-setup.md](docs/operations/jira-read-only-setup.md)
- [docs/operations/github-actions-read-only-setup.md](docs/operations/github-actions-read-only-setup.md)

Do not enable live tests in normal CI. Do not use write-capable provider credentials for read-only integration work.

## ADRs

Architecture-significant decisions require an ADR in [docs/adr](docs/adr). ADRs should explain the context, decision, consequences, and alternatives considered.

## Pull Requests

Pull requests must include the structured implementation report required by the engineering standard. The report must disclose incomplete, placeholder, or mock-only work explicitly.
