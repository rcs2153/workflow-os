# Selected Local Project Consumer CLI Adoption CI Blocker Fix Report

## 1. Executive Summary

Hosted Rust 1.97 rejected the selected CLI adoption because
`authoritative_governance_run_command` contained 101 lines. The bounded fix
extracts request construction into a private helper. It does not suppress the
lint or change CLI behavior, governance routes, approval semantics, stores,
artifacts, schemas, provider execution, or writes.

## 2. Blocker Fixed

The run adapter now contains 95 lines, below the hosted
`clippy::too_many_lines` threshold. The extracted helper preserves ownership
and construction of the same
`LocalSelectedProjectValidationGovernanceReportRequest` value.

## 3. Validation

- `cargo fmt --all --check`: passed under Rust 1.95.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed under Rust
  1.95.
- `npm run check:docs`: passed under the repository-pinned Node 20 toolchain.
- `git diff --check`: passed.
- Hosted Rust 1.97 required CI: pending at report creation.

The focused authoritative CLI suite exposed a pre-existing stale assertion:
the accepted selected-envelope path emits the Core-derived
`authoritative-check/<digest>` reference while the older compatibility test
still expects the retired CLI-authored `local-check-result/<run-id>` shape.
Nine other authoritative tests, including the new selected-envelope regression,
passed. The assertion was not changed under this narrowly approved clippy fix;
it requires a separate governed blocker scope before merge.

## 4. Governed Phase Record

- Workflow: `dg/blocker`.
- Run ID: `run-1786512090340253000-2`.
- Approval ID: `approval/run-1786512090340253000-2/fix-approved`.
- Approval outcome: granted with persisted presentation proof
  `presentation/3de55c578937ff61`.
- Event summary: 39 events, one approval, zero retries, zero escalations.

Repository edits, shell commands, validation, Git work, and PR updates were
performed by the delegated maintainer outside the kernel. The kernel governed
scope, approval, durable event history, and phase closure.

## 5. Recommended Next Phase

Run a focused blocker fix for the stale reference-shape assertion. Preserve the
Core-derived reference as the source of truth, update human and JSON regression
coverage to assert that bounded source-derived shape, then require green hosted
CI before merge.
