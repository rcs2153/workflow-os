# Selected Local Project Consumer CLI Adoption CI Blocker Fix Report

## 1. Executive Summary

Hosted Rust 1.97 rejected the selected CLI adoption because
`authoritative_governance_run_command` contained 101 lines. The bounded fix
extracts request construction into a private helper. It does not suppress the
lint or change CLI behavior, governance routes, approval semantics, stores,
artifacts, schemas, provider execution, or writes.

Focused validation then exposed two adjacent compatibility defects. Selected
project report composition still trusted a CLI-supplied local-check result ID
instead of the canonical Core-derived ID, and terminal retries regenerated a
weaker report before comparing it with the persisted authoritative artifact.
The final blocker fix derives the selected report reference from immutable
check material and reconciles an exact artifact already validated by the
artifact store as idempotent success.

## 2. Blocker Fixed

The run adapter now contains 95 lines, below the hosted
`clippy::too_many_lines` threshold. The extracted helper preserves ownership
and construction of the same
`LocalSelectedProjectValidationGovernanceReportRequest` value.

The selected-project report adapter now replaces the compatibility input ID
with the canonical `authoritative-check/<digest>` identity derived from the
bound immutable run bundle and selected check profile. Generic authoritative
report composition continues to preserve its caller-supplied ID; this change
does not broaden that API.

For a terminal rerun, the CLI reads the deterministic
`report/<run-id>` artifact through `WorkReportArtifactStore`. When that exact
artifact exists and passes store validation, Core returns
`AlreadyPersisted` without regeneration, overwrite, event append, or weaker
content comparison. Missing or invalid artifacts continue through the
existing fail-closed persistence path.

## 3. Validation

- `cargo fmt --all --check`: passed under Rust 1.95.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed under Rust
  1.95.
- `cargo test --workspace`: passed. Opt-in live provider checks remained
  ignored by design; all executed workspace tests passed.
- `npm run check:docs`: passed under the repository-pinned Node 20 toolchain.
- `git diff --check`: passed.
- Hosted Rust 1.97 required CI: pending at report creation.
- `cargo test -p workflow-core --test local_executor
  selected_project_validation_report_adapter_reassesses_terminal_run_without_reexecution
  -- --exact`: passed.
- `cargo test -p workflow-cli --test cli authoritative_governance`: passed;
  ten focused authoritative CLI regressions.

The focused authoritative CLI suite exposed a pre-existing stale assertion:
the accepted selected-envelope path emits the Core-derived
`authoritative-check/<digest>` reference while the older compatibility test
still expected the retired CLI-authored `local-check-result/<run-id>` shape.
A separately governed test-only blocker phase corrected the human assertion
and added explicit aggregate and authored JSON assertions for the non-empty
Core-derived reference shape. The wider focused suite found and corrected the
same stale expectation in the verbose quiet-success assertion.

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

The test-only follow-up used `dg/blocker` run
`run-1786513110297325000-2`, approval
`approval/run-1786513110297325000-2/fix-approved`, and persisted presentation
proof `presentation/a84ddd4725f8301a`.

The selected-reference and terminal-retry runtime correction used
`dg/blocker` run `run-1786513402436489000-2`, approval
`approval/run-1786513402436489000-2/fix-approved`, and persisted presentation
proof `presentation/daafc047b3471659`. The governed run completed with 39
events, one approval, zero retries, and zero escalations.

## 5. Recommended Next Phase

Require green hosted CI before merge, then proceed with the accepted roadmap
sequence.
