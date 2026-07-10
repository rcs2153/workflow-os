# Provider Write Approval Presentation Edge Hardening Review

## 1. Executive Verdict

Phase accepted; proceed to provider-write runtime composition planning.

The hardening phase adds the exact edge-case coverage requested by the previous provider-write approval-presentation gate review. It preserves the explicit opt-in provider-write wrapper, keeps default executor behavior unchanged, and strengthens confidence that the proof gate fails closed before provider invocation.

## 2. Scope Verification

The phase stayed within the approved edge-hardening scope.

Confirmed not introduced:

- provider-write behavior expansion;
- automatic runtime provider-write execution;
- default executor changes;
- hidden provider/auth loading;
- CLI behavior;
- workflow schema changes;
- examples;
- persistence or artifact behavior changes;
- hosted runtime;
- reasoning lineage;
- side-effect execution model changes;
- release posture changes.

The only code changes were focused tests in `crates/workflow-core/tests/local_executor.rs`; the only documentation addition was the phase report.

## 3. Edge-Case Coverage Assessment

The implementation adds focused tests for the three non-blocking follow-ups identified in the prior review:

- `NotRequired` compatibility preserves the provider-write call path and reports `approval_presentation` as `NotRequired`.
- Missing approval decision reference blocks the provider call and returns `executor_github_pr_comment_write.approval_presentation.approval_reference_missing`.
- Stale approval-presentation proof blocks the provider call and returns `approval_presentation_enforcement.proof_stale`.

These are the right edge cases for this phase because they exercise the exact explicit provider-write wrapper rather than only the shared approval-presentation enforcement helpers.

## 4. Validation Behavior Assessment

The tests verify:

- completed workflow runs remain completed;
- compatible `NotRequired` posture does not require presentation proof;
- required proof posture requires a stable approval decision reference;
- stale proof fails closed before provider invocation;
- blocked paths set provider-call gate clarity to `Blocked`;
- successful compatible paths set provider-call gate clarity to `Satisfied`;
- provider call count remains zero on blocked proof/reference paths.

The implementation does not relax the proof validation boundary. It continues to delegate proof validation to the existing `approval_decision_with_presentation_policy(...)` path.

## 5. Privacy And Non-Leakage Assessment

The blocked tests assert that error strings do not leak:

- approval identifiers;
- presentation identifiers;
- non-approval reference payloads used to trigger the missing-reference case.

Provider payloads, auth tokens, command output, source contents, and raw approval-presentation payloads are not copied or introduced by this phase.

## 6. Compatibility Assessment

The `NotRequired` test is important and passes: explicit callers that opt into the provider-write wrapper but do not require approval-presentation proof still preserve existing provider-write behavior, provided they do not supply contradictory proof/freshness/posture fields.

Default executor paths remain unchanged.

## 7. Test Quality Assessment

The focused tests are strong enough for this hardening phase:

- they use the existing local executor and provider-write wrapper;
- they use existing `WorkFlowRun`/approval state rather than synthetic result-only fixtures;
- they assert provider invocation counts directly;
- they assert gate clarity states;
- they assert stable error codes and non-leakage.

No shallow or missing blocker tests found.

## 8. Documentation Review

The phase report accurately states:

- scope completed;
- scope explicitly not completed;
- tests added;
- validation commands run;
- remaining limitations;
- recommended next phase.

It does not overclaim provider-write safety, automatic runtime behavior, CLI behavior, schemas, hosted behavior, or write-capable adapter readiness.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Add API-level usage documentation for the explicit provider-write approval-presentation gate before expecting downstream users to call it directly.
- Consider consolidating provider-write gate-clarity terminology once more provider-write composition paths share the same UI/report surface.
- Keep default executor proof enforcement separate until workflow-declared/default enforcement has its own accepted plan.

## 11. Validation Commands Reviewed

Implementation validation recorded in the phase report:

- `cargo test -p workflow-core --test local_executor provider_write_presentation_gate` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

Review validation:

- `npm run check:docs` - passed.

## 12. Governed Dogfood Run

- workflow: `dg/review`
- run ID: `run-1783722069186362000-2`
- approval ID: `approval/run-1783722069186362000-2/review-scope-approved`
- approval-presentation ID: `presentation/e8b0b8459d83adc4`
- approval-presentation hash: `e8b0b8459d83adc40b4476fc35301b7063fadfab73539cb9c6730e0203dda114`
- approval outcome: granted by delegated maintainer
- approval reason: `approved-provider-write-edge-hardening-review`

## 13. Recommended Next Phase

Provider-write runtime composition planning.

Reason: the explicit provider-write approval-presentation gate is implemented, reviewed, and edge-hardened. The next useful step is to decide how the already-built provider-write pieces should compose in an explicit runtime path without broadening default execution: approval-presentation proof, approval linkage, attempted/completed/failed side-effect lifecycle, event proof, report disclosure, artifact gates, and operator recovery.

The next phase must still not add automatic provider writes, default executor behavior changes, hidden auth loading, CLI mutation commands, schemas, examples, hosted behavior, reasoning lineage, or release posture changes.
