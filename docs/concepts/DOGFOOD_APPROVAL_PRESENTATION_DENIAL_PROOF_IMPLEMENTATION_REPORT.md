# Dogfood Approval-Presentation Denial Proof Implementation Report

## 1. Executive Summary

This phase closes the repo-local dogfood approval decision-direction asymmetry.
Dogfood grants already required persisted, fresh approval-presentation proof.
This phase verifies and tests that dogfood denials use the same hidden
proof-enforced approval-presentation path.

The implementation is intentionally narrow: it adds focused denial-path tests
and documentation/reporting. The existing hidden dogfood command already routed
`--deny` through `LocalExecutor::decide_approval_with_presentation(...)`, so no
runtime code change was required.

## 2. Scope Completed

- Added focused CLI coverage for proof-validated dogfood denial success.
- Added focused CLI coverage for stale dogfood denial proof rejection.
- Verified the hidden dogfood command routes `--deny` through the existing
  approval-presentation enforcement boundary.
- Verified successful proof-validated denial records an approval proof marker.
- Verified stale denial proof fails before denial or terminal events are
  appended.
- Updated the denial-proof plan to implemented status.
- Updated the roadmap to point to this implementation report.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- public approval behavior changes;
- automatic approvals or automatic denials;
- public approval-card UI;
- workflow schema fields;
- examples;
- provider writes;
- side effects;
- report artifact writes;
- hosted or distributed runtime behavior;
- high-assurance approval changes beyond existing explicit opt-in paths;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. Behavior Summary

The hidden dogfood approval-presentation command supports `--deny` and builds
an `ApprovalDecisionKind::Denied` request. It then calls
`LocalExecutor::decide_approval_with_presentation(...)` with:

- run ID;
- approval ID;
- presentation ID;
- optional max presentation age;
- actor;
- denial reason.

The core opt-in enforcement path validates the persisted approval presentation
before appending approval decision events. On success, the existing executor
denial semantics fail the run closed.

## 5. Fail-Closed Summary

The stale denial proof test verifies:

- `approval_presentation_enforcement.proof_stale` is returned;
- the stale presentation ID is not leaked;
- the run remains `WaitingForApproval`;
- no `ApprovalDenied` event is appended;
- no `RunFailed` event is appended.

This keeps stale or invalid denial proof from silently stopping governed work.

## 6. Privacy And Redaction Summary

The phase adds no new payload storage and no new public output surface.

The denial path continues to avoid copying:

- approval handoff text;
- presentation record payloads;
- raw command output;
- provider payloads;
- source/spec contents;
- token-like values;
- credentials or private keys.

Focused tests assert stale denial errors do not leak the presentation ID.

## 7. Test Coverage Summary

Added tests:

- `dogfood_approval_presentation_denial_uses_proof_marker_and_fails_closed`;
- `dogfood_approval_presentation_denial_rejects_stale_proof_without_events`.

Existing focused tests still cover granted proof-marker inspect projection and
stale granted proof rejection.

## 8. Commands Run And Results

- `cargo test -p workflow-cli --test cli dogfood_approval_presentation -- --nocapture` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 9. Remaining Known Limitations

- Denial proof enforcement is repo-local dogfood behavior only.
- Default public `workflow-os approve --deny` behavior remains unchanged.
- The helper does not print a separate copy-safe denial command; the explicit
  `--deny` flag remains the first implementation path.
- Public approval-card UX remains unimplemented.

## 10. Recommended Next Phase

Recommended next phase: dogfood approval-presentation denial proof
implementation review.

The review should confirm that the implementation is test-only/runtime-code
neutral, that the existing hidden command path truly enforces proof for denials,
and that public approval behavior remains unchanged.

## 11. Dogfood Governance

Workflow OS governed this implementation phase:

- workflow: `dg/implement`;
- run: `run-1783704472604622000-2`;
- approval: `approval/run-1783704472604622000-2/implementation-approved`;
- approval presentation: `presentation/b569026e96f171ad`;
- approval outcome: granted;
- approval reason: `approved-denial-proof-implementation-scope`;
- close status: `Completed`;
- events: 39 total;
- event summary: `ApprovalGranted:1`, `ApprovalRequested:1`,
  `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`,
  `RunResumed:1`, `RunStarted:1`, `RunValidated:1`,
  `SkillInvocationRequested:6`, `SkillInvocationStarted:6`,
  `SkillInvocationSucceeded:6`, `StepScheduled:6`;
- approval presentation enforcement: proof enforced;
- approval presentation event marker: present.

Codex performed repository edits and validation outside the kernel. The kernel
governed the phase approval boundary.
