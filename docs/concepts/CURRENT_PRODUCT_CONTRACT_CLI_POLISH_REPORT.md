# Current Product Contract And CLI Polish Report

## 1. Executive Summary

This phase fixed a small but high-trust product gap surfaced by real-repository
testing: Workflow OS had a credible local kernel, but the CLI and current-state
docs still had avoidable first-use trust debt.

The phase adds version reporting, corrects stale known-limitations language,
updates generated-file documentation for `init-repo-governance`, and adds a
concise Current Product Contract for evaluators.

## 2. Scope Completed

- Added `workflow-os --version`.
- Added `workflow-os version`.
- Added bounded JSON version output for `workflow-os --json version`.
- Updated CLI help to list `version`.
- Added CLI regression tests for version output.
- Updated `docs/release/V0_KNOWN_LIMITATIONS.md` so it no longer claims project
  initialization is missing.
- Updated `docs/cli/init-repo-governance.md` to list
  `policies/local.policy.yml`.
- Added `docs/user-guide/current-product-contract.md`.
- Linked the current product contract from the CLI overview and user-guide
  index.

## 3. Scope Explicitly Not Completed

- No new runtime primitives.
- No provider-write behavior.
- No write-capable adapters.
- No schema changes.
- No examples.
- No hosted or distributed runtime.
- No report artifact automation.
- No automatic local check execution.
- No reasoning lineage implementation.
- No release posture change.

## 4. CLI Behavior Summary

`workflow-os --version` and `workflow-os version` now print:

```text
workflow-os <crate-version>
```

`workflow-os --json version` prints a bounded local-preview JSON object with:

- `name`;
- `version`;
- `schema_version`;
- `release_posture`.

The version command does not require a Workflow OS project.

## 5. Documentation Summary

The new Current Product Contract states:

- what is real today;
- what is mock or demonstration-only;
- what is not implemented;
- the safe first evaluation loop;
- the trust boundary: `Agent executes. Workflow OS governs.`

The release limitations now distinguish implemented onboarding scaffolding from
unsupported docs generation, generic live adapter execution commands,
distributed worker commands, hosted operation, and production deployment
commands.

## 6. Dogfood Governance

This implementation phase was governed by `dg/implement`.

- run ID: `run-1783735062078181000-2`
- approval ID: `approval/run-1783735062078181000-2/implementation-approved`
- presentation ID: `presentation/0d956fa872e4048d`
- presentation hash:
  `0d956fa872e4048d6bbeedc09364f0af87a8da7cae4ea3ca0d866354ac7f92fe`
- approval outcome: granted by delegated maintainer authority

The approval covered only CLI version support and current-state documentation
polish. It did not approve new runtime behavior, writes, schemas, examples,
hosted behavior, reasoning lineage, or release posture changes.

## 7. Validation Commands

Commands run:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

Result: passed.

## 8. Remaining Known Limitations

- The current product contract is documentation; it is not a versioned external
  compatibility guarantee.
- CLI JSON remains experimental through `0.2.0-preview.1`.
- The next product-trust lane is still bridging first-run recommendations into
  clearer productive workflow authoring and promotion.

## 9. Recommended Next Phase

Recommended next phase: current product contract and CLI polish review.

Reason: the implementation is intentionally small but user-facing. A focused
review should verify that version output is stable and bounded, docs no longer
contradict current CLI behavior, and the product contract does not overclaim
runtime capabilities.
