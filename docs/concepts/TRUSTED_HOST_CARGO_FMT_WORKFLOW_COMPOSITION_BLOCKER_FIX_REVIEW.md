# Trusted-Host Cargo Format Workflow Composition Blocker Fix Review

## 1. Executive Verdict

**Needs additional blocker fixes.**

The fix correctly closes the original dispatch-binding, failed-terminal,
pre-result invocation-linkage, schedule-citation, successor-order, result
substitution, artifact-conflict, and privacy gaps for the covered paths. Full
workspace validation is green.

The phase cannot yet be accepted because the refactored history validator
rejects an otherwise valid pre-result failure on the first or second attempted
step, and the mandatory duplicate-dispatch and composed-timeout regressions
remain absent.

## 2. Scope Verification

The fix stayed within the approved private three-step composition boundary. It
did not add a profile, generic command registry, new topology, provider,
external write, public CLI behavior, schema, example, hosted behavior, nested
harness execution, or release-posture change.

## 3. Original Blocker Assessment

The original review required:

1. exact attempted-step dispatch validation;
2. exact failed-terminal validation;
3. pre-result invocation identity linkage; and
4. direct corruption, substitution, timeout, artifact, and privacy proofs.

The implementation now reconstructs immutable step input, resolves the closed
profile, validates deterministic invocation identity, requires one matching
dispatch, binds accepted result truth, and rejects unexpected dispatch counts.
Report composition binds pre-result failure to the exact invocation request
and cites schedule events directly.

## 4. Dispatch And Invocation Integrity

For covered terminal and resumed paths, each attempted step must have exactly
one dispatch whose invocation commitment matches the immutable `SkillInput`.
Resumed admission additionally requires the current closed profile to
reproduce the durable executor commitment. Terminal report composition keeps
the admitted dispatch commitment authoritative, which preserves honest
reporting after a pre-result executable-integrity failure.

Unexpected dispatches are rejected by comparing total dispatch count with the
number of matched attempted-step dispatches. Invocation requests are bound by
deterministic idempotency key, skill identity, step identity, and event order.

## 5. Failed Terminal Assessment

Accepted local-check results are now bound to exact step, profile, command,
immutable bundle, attempt, invocation request, invocation terminal, and
terminal order. A failed run must end in either a non-passing final result or a
pre-result failure chain.

### Blocker: early pre-result failure is rejected

`validate_trusted_host_three_step_attempt` accepts a pre-result failure only
when `index + 1 == history.workflow.steps.len()`. That condition permits only
the third step. The report composer correctly permits the no-result failure on
the last **scheduled** step, including a first- or second-step attempt.

An integrity failure may occur before result acceptance on project validation
or docs check as well as Cargo format. Such a run should fail honestly without
fabricating a result. The history validator instead rejects that valid shape,
so history validation and report composition disagree.

The fix must compare against the exact scheduled prefix, not the full workflow
length, and add first- and second-step pre-result regressions.

## 6. Report And Artifact Assessment

The v4 report now includes schedule event IDs in commitment material and
workflow-event citations. Result paths and the covered Cargo pre-result path
bind schedule, invocation request, dispatch, result when present, invocation
terminal, and run terminal in causal order.

The direct three-step conflicting-artifact regression proves create-only
behavior: a different valid v4 artifact is not overwritten and run history is
not mutated. This closes the original artifact-specific test gap.

## 7. Privacy Assessment

No privacy blocker was found. New validation errors remain stable and
payload-free. Corruption tests assert that repository paths and substituted
identities are not emitted. Reports continue to use stable references rather
than raw process output, source contents, environment values, credentials, or
implicit `EvidenceReference` creation.

## 8. Test Quality Assessment

The focused suite now directly covers:

- dispatch executor-commitment substitution;
- pre-result failed-invocation substitution;
- causal corruption at both successor boundaries;
- result/profile substitution;
- schedule-event report citations;
- conflicting v4 artifact refusal; and
- the existing success, failure, restart, replay, recovery, concurrency, and
  one-step/two-step compatibility paths.

The suite passes 40 tests. Two mandatory proof gaps remain:

1. no test injects a duplicate or unrelated dispatch into a three-step run;
2. no composed three-step test proves that a third-step timeout records
   `TimedOut`, fails terminally, schedules no successor, and produces an honest
   v4 artifact.

Existing lower-level timeout and generic dispatch tests do not prove this new
composition boundary.

## 9. Validation Assessment

The following passed on the blocker-fix revision:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- focused foreground admission tests: 40 passed
- `cargo test --workspace`
- `npm run check`
- `npm run check:integrations`
- `npm run check:docs`
- `git diff --check`

Opt-in live provider and sandbox tests remained behind their existing explicit
gates.

## 10. Governed Review Record

- workflow: `dg/review`
- run: `run-1792684874655161000-2`
- approval: `approval/run-1792684874655161000-2/review-scope-approved`
- presentation: `presentation/fe3b272700f6c3b4`
- presentation hash:
  `fe3b272700f6c3b4e5f8f1ec08e449fb58b77de8a83736b25fb8f5bd4dafb300`
- approval outcome: granted under delegated maintainer authority
- phase status: completed
- event summary: 39 events, one approval, zero retries, zero escalations

## 11. Blockers

1. Validate pre-result failure against the last scheduled step rather than the
   final workflow step, and test first- and second-step pre-result failures.
2. Add a direct duplicate or unrelated dispatch regression for the three-step
   history validator.
3. Add a composed Cargo timeout regression proving terminal truth, no
   successor, bounded report content, and v4 artifact integrity.

## 12. Non-Blocking Follow-Ups

- Keep the exact sibling composition until it is accepted; do not generalize
  this validator into arbitrary N-step topology yet.
- Preserve the durable-dispatch/current-profile distinction between report
  reconstruction and admission.
- Keep opt-in real-toolchain smoke separate from deterministic fixture tests.

## 13. Recommended Next Phase

Perform one additional narrow blocker-fix phase for the early pre-result
terminal condition and the two missing direct regressions. Then repeat focused
maintainer/security review. Do not broaden profiles, topology, providers,
writes, public surfaces, hosted behavior, or release posture.
