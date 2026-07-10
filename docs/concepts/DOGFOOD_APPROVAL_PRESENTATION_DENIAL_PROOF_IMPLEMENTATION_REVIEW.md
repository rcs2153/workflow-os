# Dogfood Approval-Presentation Denial Proof Implementation Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation closes the repo-local dogfood approval decision-direction
asymmetry for the first bounded slice: proof-validated dogfood denials now have
focused coverage confirming that they use the same approval-presentation
enforcement boundary as proof-validated grants. The phase stayed narrow and did
not change default public approval behavior.

## 2. Scope Verification

The phase stayed within the approved repo-local dogfood scope.

Confirmed in scope:

- focused denial success coverage;
- focused stale denial proof failure coverage;
- verification that the hidden dogfood `--deny` path uses
  `LocalExecutor::decide_approval_with_presentation(...)`;
- documentation and implementation report updates.

No accidental implementation was found for:

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

## 3. Implementation Assessment

The implementation is intentionally test-focused because the underlying hidden
dogfood command already routed both grant and denial decisions through the
proof-enforced executor boundary.

The command path is appropriate:

- `--deny` is parsed as an explicit decision direction;
- denial builds `ApprovalDecisionKind::Denied`;
- the hidden dogfood command still requires `--presentation-id`;
- the command calls `LocalExecutor::decide_approval_with_presentation(...)`;
- stale proof returns before denial or terminal events are appended.

This is minimal and consistent with the current dogfood-only posture.

## 4. Denial Semantics Assessment

Successful proof-validated denial behavior is appropriate:

- the decision is recorded as denied;
- the run fails closed through existing executor denial semantics;
- the approval event carries proof-marker posture;
- inspect output exposes bounded proof-marker presence rather than raw
  approval-presentation contents.

Failed proof validation behavior is appropriate:

- stale proof returns
  `approval_presentation_enforcement.proof_stale`;
- the run remains `WaitingForApproval`;
- no `ApprovalDenied` event is appended;
- no `RunFailed` event is appended;
- the stale presentation ID is not leaked.

## 5. Public Behavior Assessment

Default public approval behavior remains unchanged.

The implementation did not alter ordinary `workflow-os approve --deny`
semantics. The proof requirement remains on the hidden repo-local dogfood
approval-presentation path and the explicit opt-in executor boundary.

## 6. Privacy And Redaction Assessment

No new payload storage or public output surface was introduced.

The implementation does not copy or expose:

- approval handoff text;
- approval-presentation record payloads;
- raw command output;
- provider payloads;
- source/spec contents;
- token-like values;
- credentials or private keys.

Focused stale-denial coverage verifies the returned error does not include the
presentation ID.

## 7. Test Quality Assessment

Added tests are meaningful and behavior-focused:

- `dogfood_approval_presentation_denial_uses_proof_marker_and_fails_closed`
  covers fresh proof, explicit denial, failed-closed terminal behavior, proof
  marker storage, and inspect projection.
- `dogfood_approval_presentation_denial_rejects_stale_proof_without_events`
  covers stale proof, non-leaking error posture, unchanged waiting status, and
  absence of denial/terminal events.

Existing nearby tests continue to cover grant proof-marker projection and stale
grant rejection, which gives useful symmetry around the shared executor path.

Non-blocking gap: the implementation plan listed missing and mismatched denial
proof as future coverage candidates. This implementation covers stale proof but
does not add dedicated denial tests for missing or mismatched proof. The shared
executor boundary already has broader proof validation coverage, so this is not
a blocker for the narrow dogfood denial slice.

## 8. Documentation Review

Documentation is honest and scoped.

Docs now state:

- dogfood denial proof enforcement is implemented for the repo-local dogfood
  path;
- default public approval behavior remains unchanged;
- automatic approvals and denials are not implemented;
- public approval-card UI is not implemented;
- schemas, examples, provider writes, side effects, hosted behavior, reasoning
  lineage, and release posture changes remain unimplemented.

## 9. Validation

Implementation validation reported:

- `cargo test -p workflow-cli --test cli dogfood_approval_presentation -- --nocapture` - passed;
- `cargo fmt --all --check` - passed;
- `cargo clippy --workspace --all-targets -- -D warnings` - passed;
- `cargo test --workspace` - passed;
- `npm run check:docs` - passed;
- `git diff --check` - passed.

Review validation:

- `npm run check:docs` - passed;
- `git diff --check` - passed.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add dedicated dogfood denial tests for missing and mismatched proof if the
  dogfood path grows more public or more operator-facing.
- Consider a separate copy-safe denial command in the dogfood phase-start output
  if denial becomes a common maintainer action.
- Consider phase-close language that distinguishes granted proof enforcement
  from denied proof enforcement when a phase is denied.

## 12. Recommended Next Phase

Recommended next phase: approval-presentation denial proof follow-up defer, then
return to the next runtime-composition lane.

The denial-proof asymmetry is closed for the narrow repo-local dogfood path.
The next roadmap work should avoid adding more approval vocabulary unless it
directly composes existing primitives into runtime-enforced paths.

## 13. Dogfood Governance

Workflow OS governed this review phase:

- workflow: `dg/review`;
- run: `run-1783705612165413000-2`;
- approval: `approval/run-1783705612165413000-2/review-scope-approved`;
- approval presentation: `presentation/95b1378cefad7651`;
- approval outcome: granted;
- approval reason: `approved-denial-proof-review-scope`;
- close status: `Completed`;
- events: 39 total;
- event summary: `ApprovalGranted:1`, `ApprovalRequested:1`,
  `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`,
  `RunResumed:1`, `RunStarted:1`, `RunValidated:1`,
  `SkillInvocationRequested:6`, `SkillInvocationStarted:6`,
  `SkillInvocationSucceeded:6`, `StepScheduled:6`;
- approval presentation enforcement: proof enforced;
- approval presentation event marker: present.

Codex performed repository review, documentation edits, and validation outside
the kernel. The kernel governed the review approval boundary.
