# Dogfood Runner Approval-Presentation Persistence Implementation Report

## 1. Executive Summary

This phase implemented repo-local dogfood runner approval-presentation proof persistence. Material `phase-start` runs now persist a bounded `ApprovalPresentationRecord` before printing the approval command, then surface the `presentation_id` and content hash in the approval handoff.

The phase keeps approval explicit. It does not add automatic approval, default approval enforcement, public approval cards, schemas, examples, provider writes, side effects, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope Completed

- Added a hidden dogfood-only CLI helper: `workflow-os dogfood approval-presentation persist`.
- Wired `scripts/self-governed-benchmark.mjs` live `phase-start` to persist approval-presentation proof after a waiting approval is created.
- Added `approval_presentation_proof`, `presentation_id`, and `presentation_content_hash` to the emitted approval handoff and copy-safe approval request.
- Kept dry-run mode non-mutating and marked proof as `not_persisted`.
- Added focused helper tests for persistence command shape and dry-run proof behavior.
- Updated roadmap, runbook, and planning documentation.

## 3. Scope Explicitly Not Completed

- No automatic approval was added.
- No default approval behavior changed.
- No dogfood approval command was switched to opt-in enforcement by default.
- No public CLI approval-card UX was added.
- No workflow schemas, examples, provider writes, side effects, hosted behavior, reasoning lineage, or release posture changes were added.

## 4. Helper/API Summary

The hidden dogfood command accepts explicit run, approval, phase, work-summary, approved-scope, non-goal, touched-surface, validation, why-now, and presented-by inputs. It rehydrates the local run, verifies the run is waiting for approval, finds the pending approval request, computes the deterministic presentation content hash, writes an `ApprovalPresentationRecord`, and prints bounded proof metadata.

The repo-local benchmark helper calls this command only for live material `phase-start` runs. It does not run during dry-run.

## 5. Approval Presentation Behavior

Persisted proof binds the approval presentation to:

- run ID;
- approval ID;
- workflow ID/version;
- schema version;
- step ID;
- requested action;
- work summary;
- approved scope;
- strict non-goals;
- expected touched surfaces;
- validation expectations;
- why-now context;
- next action;
- presentation channel;
- presented actor;
- sensitivity and redaction metadata.

The runner prints the persisted proof identifiers before asking the maintainer or delegated maintainer to approve the phase.

## 6. Workflow Semantics

The runner remains governance coordination only. It does not mutate repository files, perform git operations, run provider calls, write report artifacts, execute side effects, or approve on behalf of the maintainer. Approval remains an explicit command.

If proof persistence fails during a live material phase-start, the helper exits before presenting the phase as ready for approval.

## 7. Redaction/Privacy Summary

The implementation uses existing `ApprovalPresentationRecord` constructors and validation. Secret-like work context remains rejected by the benchmark helper. Errors use stable codes and do not include raw approval handoff content, command output, provider payloads, tokens, private keys, or local file contents.

## 8. Test Coverage Summary

Focused coverage includes:

- dry-run approval handoff marks proof as `not_persisted`;
- dry-run does not claim persisted proof;
- persistence command shape includes bounded explicit phase context;
- existing dogfood helper approval-handoff, redaction, phase-start, phase-close, and command-shape tests continue to pass.

## 9. Governed Phase Summary

- dogfood workflow ID: `dg/implement`
- run ID: `run-1783596280791974000-2`
- approval ID: `approval/run-1783596280791974000-2/implementation-approved`
- approval outcome: granted by delegated maintainer
- event summary: 39 total events; 1 approval; 0 retries; 0 escalations; terminal status `Completed`
- event kinds: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`

The dogfood runner coordinated governance only. Repo edits, validation commands, git operations, PR actions, and this report were performed by the executor outside the kernel and are disclosed here.

## 10. Commands Run

- `npm run test:dogfood-helper` - passed
- `cargo check -p workflow-cli` - passed
- `cargo fmt --all --check` - passed
- `cargo clippy --workspace --all-targets -- -D warnings` - passed
- `cargo test --workspace` - passed
- `npm run check:docs` - passed
- Live dogfood runner smoke with temporary state - passed; `phase-start` printed `approval_presentation_persisted: true`, a `presentation_id`, and a content hash before the approval command.

Additional final validation commands are recorded in the phase closeout.

## 11. Remaining Known Limitations

- Dogfood approvals still use the default explicit approval command.
- The dogfood runner does not yet call `decide_approval_with_presentation(...)` by default.
- Public approval-card UX remains unimplemented.
- Default approval behavior remains unchanged.

## 12. Recommended Next Phase

Recommended next phase: dogfood runner opt-in approval-presentation enforcement planning.

That phase should decide whether and how the repo-local benchmark should pass persisted presentation proof into the existing opt-in enforcement path by default, while keeping approval explicit and preserving current kernel semantics.
