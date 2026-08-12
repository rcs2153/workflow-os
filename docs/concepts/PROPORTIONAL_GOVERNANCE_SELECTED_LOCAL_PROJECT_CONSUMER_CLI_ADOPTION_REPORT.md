# Proportional-Governance Selected Local Project Consumer CLI Adoption Report

## 1. Executive Summary

The existing manifest-controlled authoritative CLI `run` and `approve` paths
now consume the selected local project-validation Core compositions. Public
syntax, activation, human output, JSON shape, workflow semantics, approval
semantics, event ordering, retry posture, and report-artifact obligation remain
unchanged.

## 2. Scope Completed

- Routed declared authoritative `run` through
  `execute_selected_project_validation_governance_report`.
- Routed aggregate-governance and authored workflow-step approvals through one
  `decide_selected_project_validation_approval_envelope` call.
- Removed CLI-side approval-kind branching, decision-time check construction,
  local-check reference construction, and terminal report composition.
- Added deterministic local receipt and proof-marker projection stores at the
  already planned state-root paths.
- Preserved existing aggregate and authored JSON route labels by projecting the
  Core-produced bounded approval gate kind.
- Preserved non-terminal artifact deferral and terminal artifact closure.

## 3. Scope Explicitly Not Completed

This phase did not add commands, flags, declarations, workflow defaults,
schemas, provider execution, provider writes, SideEffect execution, hosted or
distributed behavior, runtime configuration, new approval policy, CLI report
rendering, arbitrary artifact persistence, or another mutation family.
Undeclared ordinary workflows remain on their existing execution path.

## 4. Run Integration

The selected run adapter owns canonical project-validation check execution,
fixed current-authority source observation, Core-owned evaluation time,
source-backed proportional-governance assessment, stable check-reference
derivation, and terminal WorkReport composition. The CLI continues to own only
validated project loading, explicit local dependency construction,
presentation persistence, artifact persistence for the fresh-run path, and the
existing output contract.

## 5. Approval Integration

The approval command no longer inspects an approval binding to select separate
aggregate and authored implementations. The Core adoption envelope resolves
the gate kind from durable run state and applies one proof-enforced decision
path. Grants rerun the canonical check, reassess current authority, and cite
the exact result. Aggregate grants remain non-terminal and do not persist a
transient receipt. Terminal authored grants persist and validate the trusted
authority receipt before writing the receipt-citing report artifact. Denials
retain truthful terminal report behavior without decision-time check execution.

## 6. Compatibility Boundary

The implementation preserves:

- declaration-controlled activation;
- `run` and `approve` syntax and exit behavior;
- quiet-success output;
- visible disclosure output;
- complete approval handoffs;
- `approval_decision` and `authored_approval_decision` JSON route labels;
- top-level `run_id`, `approval_id`, and status fields;
- output-before-artifact-obligation ordering;
- existing-terminal reconciliation; and
- ordinary workflow isolation.

## 7. Privacy And Security

The CLI does not author current facts, source identity, evaluation time,
governance disposition, authority receipts, or approval gate kind. Errors and
output remain bounded and do not expose source facts, report text, paths,
command output, environment values, provider payloads, or credentials.
Presentation proof remains mandatory before approval mutation.

## 8. Test Coverage

Existing authoritative CLI coverage passed unchanged for quiet, visible,
approval-required, denied, existing-terminal, drift, retry, artifact, JSON,
and ordinary-run behavior. A focused regression test now drives aggregate and
authored approvals through JSON, verifies the frozen route labels and
non-terminal artifact deferral, and confirms terminal closure produces exactly
one local authority-receipt record.

## 9. Commands Run And Results

- `cargo fmt --all`: passed.
- `cargo check -p workflow-cli`: passed.
- `cargo test -p workflow-cli --test cli authoritative_governance`: passed.
- Focused selected-envelope CLI regression test: passed through the compiled
  integration-test harness.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed under the
  repository-local toolchain during implementation. Hosted Rust 1.97 then
  identified `clippy::too_many_lines` in the 101-line run adapter; the adapter
  was reduced by extracting request construction without changing behavior.
- Hosted required CI after the structural fix: pending at initial report
  update.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed under the repository-pinned Node 20 toolchain.
- `git diff --check`: passed.

## 10. Governed Phase Record

- Dogfood workflow: `dg/implement`.
- Run ID: `run-1786510854166490000-2`.
- Approval ID:
  `approval/run-1786510854166490000-2/implementation-approved`.
- Approval outcome: granted with persisted presentation proof
  `presentation/9729233c9dd482d8`.
- Phase status: completed.
- Event summary: 39 events, one approval, zero retries, zero escalations.

Repository edits, formatting, compilation, tests, documentation checks, diff
inspection, and Git work were performed by the delegated maintainer outside
the kernel. The kernel governed phase scope, approval, durable event history,
and close reporting; it did not execute shell commands, edit files, run tests,
or perform Git or GitHub actions.

## 11. Remaining Limitations

- The selected path remains limited to the existing declared local
  project-validation profile.
- Fresh-run report artifact persistence remains CLI-composed; approval closure
  is Core-composed.
- The old public Core compatibility APIs remain available pending a separate
  cleanup review.
- No provider execution, writes, hosted behavior, or generic runtime default is
  enabled.

## 12. Recommended Next Phase

Perform a focused maintainer review of the selected CLI adoption. The review
should verify exact output compatibility, event and artifact ordering,
decision-time check behavior, receipt integrity, denial behavior, retry
reconciliation, ordinary workflow isolation, and privacy before any cleanup or
consumer broadening.
