# Proportional-Governance Selected Project-Validation Report Adapter Blocker Fix Report

## 1. Executive Summary

The selected project-validation report adapter now reassesses an existing
terminal run with truthful current-time provenance. It executes the canonical
check once with a fresh Core-owned evaluation time, compares stable semantic
commitments with the original durable governance binding, and preserves the
original run binding and event history. Current semantic drift fails closed.

This remains an additive Core-only report-adapter path. CLI adoption and the
selected approval adoption envelope are not implemented.

## 2. Blocker Fixed

The prior implementation reran the canonical check for an existing terminal
run but assigned the original runtime-fact snapshot's evaluation timestamp to
the new observation. Full binding equality was achieved by replaying time,
which made the new observation's provenance false.

The adapter no longer reads the original timestamp to drive reassessment.
Every call receives a fresh Core-owned evaluation time.

## 3. Implementation Approach

The fix makes three bounded changes:

1. `execute_selected_project_validation_governance_report` always selects
   `Timestamp::now_utc()` for its source-backed route.
2. existing-terminal reassessment validates the fresh binding with
   `GovernanceAssessmentBinding::validate_current_runtime_fact_binding`
   instead of requiring byte-for-byte binding equality;
3. the existing-terminal result exposes only the transient reassessment time
   while continuing to return the original durable governance binding.

No fresh reassessment binding is persisted or returned as reusable authority.

## 4. Validation Boundary

Semantic comparison now requires:

- V3 current-runtime-fact binding vocabulary on both bindings;
- identical workflow and run identity;
- identical immutable run-bundle binding;
- identical assessment algorithm and aggregate fingerprint;
- identical step count, execution disposition, disclosure, and completeness;
- identical trusted source-registration commitment;
- identical runtime-fact commitment and fact count; and
- identical snapshot assessment aggregate.

Observation and evaluation timestamps are deliberately excluded from semantic
equality because a current reassessment must have current temporal provenance.

## 5. Provenance And Durable-State Posture

The fresh reassessment time is call-local provenance. It does not replace the
original timestamp inside the durable binding, does not append workflow events,
and does not become an authority receipt. The original durable governance
binding remains the run's historical record.

Current check or fact drift returns the stable non-leaking error
`executor.governance_assessment_binding.reassessment_mismatch` before report
regeneration or runtime mutation.

## 6. Privacy And Error Posture

The change adds no payload fields, source contents, command output, paths,
credentials, tokens, or provider data. Debug output redacts the transient
reassessment timestamp. Errors retain stable codes and bounded messages.

## 7. Test Coverage

Focused tests prove that:

- an existing terminal call executes the canonical check a second time but
  does not rerun workflow skills;
- reassessment uses a time later than the original durable snapshot time;
- the returned and persisted governance binding remains the original binding;
- the workflow event history remains unchanged; and
- changed current check posture fails closed without event or skill replay.

The existing adapter tests continue to cover terminal report generation,
approval deferral, exact check-reference projection, duplicate-reference
preflight, and Debug redaction.

## 8. Scope Explicitly Not Completed

This fix does not implement:

- CLI cutover;
- the selected approval adoption envelope;
- workflow schema changes;
- provider mutation;
- new persistence;
- skill replay or event mutation;
- reusable reassessment authority;
- automatic report generation;
- hosted behavior; or
- release posture changes.

## 9. Governed Phase Record

- Workflow: `dg/blocker`
- Run: `run-1786471531593349000-2`
- Approval: `approval/run-1786471531593349000-2/fix-approved`
- Presentation: `presentation/82976575ad0bc7b2`
- Approval outcome: granted by delegated maintainer with persisted
  presentation proof
- Presentation content hash:
  `82976575ad0bc7b24825131c44e798debe150c12b7975e9927290e119e63f41f`
- Phase status: completed
- Event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six completed skill invocations, and one run
  completion; zero retries and zero escalations
- Approval-presentation enforcement: proof-enforced with the presentation
  marker present in the durable event trail

## 10. Validation Commands

The phase passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p workflow-core --test local_executor
  selected_project_validation_report_adapter` (5 passed)
- `cargo test -p workflow-cli --test cli` (169 passed)
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

The workspace suite passed with no failures. Its live GitHub, Jira, and local
DocsCheck probes remained explicitly ignored because their opt-in environment
flags and live credentials were not supplied; fixture and injected-provider
coverage passed.

## 11. Remaining Limitations

The reassessment time is returned only in memory and is not an audit event or
durable authority record. This is intentional for the no-mutation retry path.
Direct selected-adapter coverage for visible-disclosure and denied outcomes
remains a non-blocking follow-up.

## 12. Recommended Next Phase

Perform the focused selected project-validation report-adapter blocker-fix
review. Only an accepted review should unblock the selected approval adoption
envelope. CLI cutover remains later.

## 13. Out-Of-Kernel Disclosure

The kernel governed phase scope, approval, and durable event history. Codex
implemented the Rust changes, tests, and documentation and ran repository
validation outside the kernel. No Workflow OS runtime state was edited by
hand.
