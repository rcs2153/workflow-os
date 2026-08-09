# RustSec Advisory Database Pin Blocker Fix Report

## 1. Executive Summary

The required Dependency And Security CI gate is restored without weakening
the Rust advisory audit. CI now checks out one exact, valid RustSec advisory
database revision and runs pinned `cargo-audit` with database fetching
disabled.

## 2. Blocker Fixed

RustSec advisory-database commit
`e11d6b330dd033a9ed7476de71029cfb8f2d1095` added a placeholder advisory under
`crates/gettext-rs` while declaring package `gettext-sys`. `cargo-audit 0.22.2`
therefore rejected the database before evaluating Workflow OS `Cargo.lock`.
The same upstream-data failure reproduced in two GitHub Actions attempts.

## 3. Implementation

- Pin the database to the immediately preceding valid commit,
  `309ad29d8fe448bf986019e05d47b9e0e29a2218`.
- Verify the checked-out commit before scanning.
- Pin `cargo-audit` to version `0.22.2`.
- Run with `--no-fetch` against the exact checked-out database.
- Keep any database-load error or advisory finding blocking.

No advisory is ignored, no audit failure is converted into success, and no
Workflow OS runtime behavior changes.

## 4. Validation

Required validation includes the pinned local audit, repository format, lint,
test, documentation, and diff checks, followed by all seven required GitHub
Actions jobs. Final results are recorded in the pull request and governed
phase close.

## 5. Remaining Limitation

The evidence source is intentionally fixed rather than continuously updated.
Maintainers must advance the pin after RustSec publishes a candidate revision
that loads successfully and passes the complete audit. The pin must not become
a permanent substitute for advisory updates.

## 6. Governed Phase

- Workflow: `dg/blocker`
- Run ID: `run-1786282797757204000-2`
- Approval ID: `approval/run-1786282797757204000-2/fix-approved`
- Approval presentation ID: `presentation/5a1b4f7633a88838`
- Approval outcome: granted by delegated maintainer with persisted proof
- Approved boundary: restore the mandatory CI audit without ignores, bypasses,
  runtime changes, or merge before green CI
