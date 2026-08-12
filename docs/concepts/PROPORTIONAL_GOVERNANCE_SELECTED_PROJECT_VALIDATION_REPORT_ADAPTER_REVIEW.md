# Proportional-Governance Selected Project-Validation Report Adapter Review

Fix-forward note: the temporal-provenance blocker identified by this review
was fixed and accepted in
[Proportional-Governance Selected Project-Validation Report Adapter Blocker Fix Review](PROPORTIONAL_GOVERNANCE_SELECTED_PROJECT_VALIDATION_REPORT_ADAPTER_BLOCKER_FIX_REVIEW.md).
This document remains the original phase-review record.

## 1. Executive Verdict

Needs blocker fixes.

The selected report adapter is narrow, deterministic, and safe on the fresh-run
path. It preserves the exact same-call check reference, defers report creation
while approval is pending, and does not rerun workflow skills or mutate durable
run history. Existing-terminal reassessment, however, executes a new canonical
check while assigning the original runtime-fact snapshot evaluation time to the
new observation. That temporal replay makes the reassessment provenance false
and must be fixed before approval-envelope adoption or CLI cutover.

## 2. Scope Verification

The phase stayed within its approved Core-only adapter scope. It added no CLI
behavior, workflow schema, persistence backend, provider mutation, automatic
report generation, new policy language, new approval semantics, or runtime
configuration. Existing public executor methods remain unchanged.

## 3. API Assessment

`LocalSelectedProjectValidationGovernanceReportRequest` and
`execute_selected_project_validation_governance_report` form a small explicit
boundary. Callers provide project, execution, and report inputs without
injecting runtime facts. Core remains responsible for source selection,
canonical check execution, proportional-governance assessment, and report
composition. Debug output is bounded and does not expose report text, paths, or
caller-supplied payloads.

The adapter correctly returns the exact `LocalCheckResultReference` created
from the canonical result used by the route. It does not fabricate an ID or
recreate evidence. The public `GovernanceRuntimeFactSnapshotBinding::evaluated_at`
accessor was added only to support temporal replay and should be removed if the
blocker fix leaves no justified public consumer.

## 4. Fresh-Run Composition Assessment

The fresh-run path is sound:

- Core selects a fresh evaluation time;
- the canonical project-validation check runs once;
- the selected source-backed governance route consumes that same result;
- the adapter projects the exact result reference into report citations;
- report generation is deferred while the run awaits approval; and
- terminal outcomes compose through existing validated WorkReport APIs.

No second check is run for report construction, no workflow skill is replayed,
and no fake report is created for a non-terminal run.

## 5. Existing-Terminal Reassessment Blocker

For an existing terminal run, the adapter reads `evaluated_at` from the durable
`GovernanceRuntimeFactSnapshotBinding`, executes the canonical check again, and
passes the old timestamp into the authoritative runtime-fact source. The new
source observation is therefore recorded as if it occurred at the original
run time.

This is not stale check-result reuse: current facts and the check are recomputed.
It is nevertheless false provenance. The durable snapshot binding records the
original run's evaluation context and is not a freshness claim for later
operations. Full binding equality is being achieved by replaying time rather
than by truthfully reassessing current facts and comparing their stable semantic
commitments.

## 6. Required Blocker Fix

The blocker fix must:

1. select a fresh Core-owned time for existing-terminal reassessment;
2. execute the canonical check exactly once;
3. construct a truthful fresh runtime-fact snapshot;
4. compare stable semantic commitments with the durable original, including
   immutable run/workflow identity, source registration, fact commitment, and
   assessment aggregate, without requiring observation timestamps to match;
5. fail closed on semantic drift;
6. preserve the original durable run binding and event history;
7. prevent fresh reassessment from becoming reusable authority; and
8. avoid rerunning workflow skills or mutating the terminal run.

## 7. Report And Route Semantics

Report generation remains subordinate to route truth. Pending approval returns
an explicit deferred report posture. Terminal report construction failure does
not fabricate evidence or rewrite the workflow result. Existing-terminal
reassessment does not append events or rerun skills. These semantics should
remain unchanged by the blocker fix.

## 8. Privacy And Error Assessment

The adapter uses validated constructors and stable references. It does not copy
raw provider payloads, source contents, command output, parser payloads,
environment values, credentials, tokens, or secret-like report text. Errors
remain stable and bounded. No privacy or redaction blocker was found.

## 9. Test Quality Assessment

The focused tests cover fresh terminal report composition, approval deferral,
exact check-reference projection, and existing-terminal retry behavior without
event or skill replay. They do not prove that reassessment receives a fresh
timestamp, that semantic drift fails closed independently of time, or that the
original durable binding remains unchanged after a fresh reassessment.

The blocker fix needs focused tests for those three properties. Direct adapter
tests for visible-disclosure and denied dispositions would strengthen coverage
but are non-blocking because the shared compositor and selected route already
cover those dispositions.

## 10. Documentation Assessment

The implementation report accurately describes the approved scope and explicit
non-goals, but its claim that durable evaluation-time reuse preserves integrity
needs this review qualification. The roadmap and adoption plan now record the
blocker and keep approval-envelope adoption and CLI cutover deferred.

## 11. Blockers

- Existing-terminal reassessment gives a newly executed check and source
  observation the original run's evaluation timestamp.
- Focused tests do not yet prove fresh reassessment time, semantic drift
  rejection, and durable original-binding preservation.

## 12. Non-Blocking Follow-Ups

- Add direct adapter coverage for visible-disclosure and denied terminal
  outcomes.
- Remove the public snapshot `evaluated_at` accessor if it has no valid use
  after the blocker fix.
- Keep the adapter internal to Core until the selected approval envelope and
  CLI compatibility phases pass review.

## 13. Recommended Next Phase

Implement the existing-terminal reassessment provenance blocker fix, then run a
focused blocker-fix review. Only after both pass should work proceed to the
selected approval adoption envelope. CLI cutover remains later.

## 14. Governed Review Record

- Workflow: `dg/review`
- Run: `run-1786459757904591000-2`
- Approval: `approval/run-1786459757904591000-2/review-scope-approved`
- Presentation: `presentation/e5c812d580aa1828`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Presentation enforcement: `proof_enforced`
- Presentation content hash:
  `e5c812d580aa182861efc2acc75a3c60c8640f6cdea609fee1e3930e0a3b05e5`
- Event summary: 39 events, one approval, zero retries, zero escalations
- Event kinds: `ApprovalGranted:1`, `ApprovalRequested:1`,
  `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`,
  `RunResumed:1`, `RunStarted:1`, `RunValidated:1`,
  `SkillInvocationRequested:6`, `SkillInvocationStarted:6`,
  `SkillInvocationSucceeded:6`, and `StepScheduled:6`

## 15. Out-Of-Kernel Disclosure

The kernel governed review scope, approval, and durable event history. Codex
read the implementation and tests, formed the maintainer finding, edited this
review artifact and related roadmap text, and ran repository validation outside
the kernel. No runtime state was edited by hand.

## 16. Validation

The following commands passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p workflow-core --test local_executor selected_project_validation_report_adapter`
  (4 passed)
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

The workspace suite preserved its explicit opt-in skips for live provider and
live local-check tests. No required check was skipped.
