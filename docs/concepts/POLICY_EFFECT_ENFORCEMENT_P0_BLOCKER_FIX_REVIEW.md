# Policy Effect Enforcement P0 Blocker Fix Review

## 1. Executive Verdict

Blocker fixed; proceed to high-assurance approval controls planning.

The blocker fix correctly closes the validation/runtime mismatch identified in [Policy Effect Enforcement P0 Review](POLICY_EFFECT_ENFORCEMENT_P0_REVIEW.md). Standalone `max_attempts=N` is now treated as bounded retry by the typed policy effect set, retry validation uses typed policy effect semantics, and executor regression coverage proves that a policy containing only `max_attempts=3` retries the current step and emits retry events with the expected cap.

## 2. Scope Verification

The fix stayed within the approved blocker-fix scope.

No accidental implementation was found for:

- broad policy DSL;
- OPA/Rego/Cedar or another embedded policy engine;
- RBAC, IdP, groups, teams, or steward/admin policy management;
- policy-rule actor authority enforcement;
- quorum approval, revocation, or separation-of-duty controls;
- write-capable adapters;
- provider mutations;
- side-effect execution;
- hosted/distributed policy service;
- workflow schema changes;
- new examples or demo workflows;
- automatic workflow generation;
- recursive agents or agent swarms;
- Level 3/4 autonomy enablement;
- release posture changes.

## 3. Original Blocker Restatement

The original blocker was:

- `PolicyEffect::MaxAttempts(_)` parsed as supported retry policy vocabulary.
- Retry policy validation accepted `max_attempts=N`.
- Retry context validation accepted `max_attempts=N`.
- Runtime stored the max-attempt value.
- Runtime did not treat `MaxAttempts` as enabling bounded retry, so `retry_max_attempts(...)` returned `1` when `MaxAttempts` appeared alone.

That meant a supported declared policy effect could pass validation without being enforced.

## 4. Fix Approach Assessment

The implementation chose the review's preferred path: enforce standalone `max_attempts=N` rather than rejecting it.

This is the correct minimal fix because the P0 plan and implementation already describe `max_attempts=N` as part of the v0 retry vocabulary. Rejecting it unless paired with `retry` or `bounded_retry` would have been defensible, but less consistent with the documented model.

The implementation is bounded and idiomatic:

- `PolicyEffectSet::has_bounded_retry()` now returns true when `max_attempts` is present.
- Retry validation builds a typed `PolicyEffectSet` and asks it whether retry is bounded.
- The executor continues consuming `PolicyEffectSet` through the existing `retry_max_attempts(...)` path.

No new abstractions, public schema changes, or broad policy language were introduced.

## 5. Validation Boundary Assessment

Validation and runtime now agree for retry vocabulary:

- `retry` enables bounded retry.
- `bounded_retry` enables bounded retry.
- `max_attempts=N` enables bounded retry and supplies the cap.
- malformed `max_attempts` still fails validation.
- unsupported retry effects still fail validation.

The old stale raw-string bounded check was replaced with typed `PolicyEffectSet` behavior. This removes the mismatch where one validation path accepted `MaxAttempts` while another raw-string check treated it as unbounded.

Validation errors remain stable and non-leaking.

## 6. Runtime Enforcement Assessment

Runtime enforcement now honors the accepted effect:

- `PolicyEffect::MaxAttempts(3)` sets `PolicyEffectSet::max_attempts() == Some(3)`.
- `PolicyEffectSet::has_bounded_retry()` returns true for standalone `MaxAttempts`.
- `retry_max_attempts(...)` returns `3`.
- The executor retries the current step and emits `RetryScheduled` with `max_attempts == 3`.

The regression test confirms prior steps are not rerun, preserving the existing multi-step retry boundary.

## 7. Privacy And Redaction Assessment

The fix does not add:

- raw provider payload storage;
- raw command output storage;
- raw spec content copying;
- environment variable value storage;
- credential, authorization header, private key, or token-like value storage.

No new error path echoes raw policy effect values, command output, provider payloads, raw snippets, paths beyond normal source locations, or secret-like values.

## 8. Regression Assessment

The fix preserves:

- existing approval policy behavior;
- existing bounded retry behavior with `retry` and `bounded_retry`;
- existing escalation behavior;
- existing read-only adapter policy gating;
- external-write rejection before runtime;
- secret-read rejection before runtime;
- existing dogfood workflow validation;
- existing docs and integration gates.

No runtime state mutation, write-capable adapter behavior, side-effect execution, hosted behavior, or CLI behavior was introduced.

## 9. Test Quality Assessment

The added tests are focused and meaningful:

- `max_attempts_effect_enables_bounded_retry` proves the typed model behavior directly.
- `max_attempts_only_retry_policy_retries_current_step` proves executor behavior with a policy containing only `max_attempts=3`.
- The executor regression verifies run completion, call count, invoked-step ordering, and retry event cap.

The tests cover the exact false-governance gap identified by the review. I do not see a remaining blocker-level test gap for this fix.

Non-blocking test follow-ups remain:

- add a clearer direct conservative-policy reason-code test if a future missing-read-policy reason code is added;
- add generated scaffold validation coverage proving generated repo-governance policies keep local requirement and approval policies separate.

## 10. Documentation Review

Documentation now states:

- the original policy-effect enforcement phase was implemented;
- the original review found the standalone `max_attempts=N` blocker;
- the blocker is fixed in [Policy Effect Enforcement P0 Blocker Fix Report](POLICY_EFFECT_ENFORCEMENT_P0_BLOCKER_FIX_REPORT.md);
- standalone `max_attempts=N` now enables bounded retry and is covered by regression tests;
- broad policy DSLs, RBAC/IdP, hosted policy service, write-capable adapters, side-effect execution, schemas, recursive agents, agent swarms, and Level 3/4 autonomy remain unimplemented.

The docs preserve the original review finding rather than erasing it.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add a more specific conservative-policy reason code for missing `allow_external_read` in direct policy-engine evaluation.
- Clarify `allow_local` semantics in more user-facing docs: it records local policy posture but does not create new local execution authority.
- Add scaffold validation coverage for split local, approval, and read-only policies.

## 13. Recommended Next Phase

Recommended next phase: **high-assurance approval controls planning**.

Policy effects are now enforce-or-reject for the supported P0 vocabulary. The next safety-critical gap before write-capable adapters is the high-assurance approval lane: separation of requester/approver where appropriate, approval context requirements, expiration/revocation semantics, and practical "nuclear key" posture. That work should remain planning-first and should not introduce provider writes, RBAC/IdP, quorum approval, hosted enforcement, or Level 3/4 autonomy until separately scoped.

## 14. Validation

Completed successfully during the blocker fix and this review:

- `npm run dogfood:benchmark -- validate`
- `cargo test -p workflow-core --test policy --test local_executor --test project_validation`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `npm run check:integrations`
- `git diff --check`

