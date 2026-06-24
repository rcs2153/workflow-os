# Policy Effect Enforcement P0 Review

## 1. Executive Verdict

Needs blocker fixes.

Fix-forward note: the blocker identified in this review is addressed in [Policy Effect Enforcement P0 Blocker Fix Report](POLICY_EFFECT_ENFORCEMENT_P0_BLOCKER_FIX_REPORT.md). This review preserves the original maintainer finding.

The phase correctly addresses the central false-governance risk for unsupported policy effects: declared policy effects are now parsed into a small typed v0 vocabulary, unsupported effects fail validation, unsupported actor bindings fail validation, effect context is checked, and supported read-only adapter access requires `allow_external_read`.

However, one supported retry shape validates but is not honored by runtime behavior: a retry policy containing only `max_attempts=N` is accepted as a retry effect, but executor retry detection does not treat `MaxAttempts` as enabling bounded retry. That leaves a narrow but real mismatch between validation and runtime enforcement.

## 2. Scope Verification

The phase stayed within the approved P0 policy-effect enforcement scope.

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
- new examples or demo workflows beyond fixture/scaffold policy correction;
- automatic workflow generation;
- recursive agents or agent swarms;
- Level 3/4 autonomy enablement;
- release posture changes.

## 3. Model Assessment

The implemented `PolicyEffect` model is intentionally small and appropriate for P0.

The supported vocabulary covers:

- `allow_local`;
- `allow_external_read`;
- `require_approval` with compatibility aliases;
- `retry` and `bounded_retry`;
- `max_attempts=N`;
- `escalate` with compatibility aliases.

The model is preferable to raw string matching and moves the runtime toward a clearer enforced-or-rejected contract. `PolicyEffectSet` is also a reasonable bounded runtime carrier for resolved step policy posture.

The main model concern is semantic: `MaxAttempts` is treated as a retry effect by validation, but `PolicyEffectSet::has_bounded_retry()` only considers `Retry` and `BoundedRetry`. This makes `MaxAttempts` partially modeled but not fully actionable when it appears alone.

## 4. Validation Assessment

Validation now correctly fails closed for:

- empty policy effects;
- unsupported policy effects;
- malformed parameterized effects;
- unsupported policy-rule actor bindings;
- effects in unsupported policy-reference contexts;
- read-only adapter-backed steps missing `allow_external_read`;
- external-write adapter-backed steps;
- secret-read adapter-backed steps.

Validation errors use stable codes and bounded messages. I did not find raw policy values, provider payloads, command output, secrets, or snippets being copied into diagnostic messages for the new checks.

Validation blocker: `policy_effect_matches(...)` accepts `PolicyEffect::MaxAttempts(_)` as satisfying a retry policy, and `PolicyEffectContext::Retry` allows `MaxAttempts`. That means a policy with only `max_attempts=3` is validation-valid as a retry policy.

## 5. Runtime Enforcement Assessment

The executor now resolves step policy references into typed `PolicyEffectSet` values and consumes them for:

- approval checkpoint behavior;
- retry max-attempt behavior;
- escalation availability;
- read-only adapter policy context.

That is the right runtime direction and avoids relying on raw policy strings inside executor decision branches.

Runtime blocker: `retry_max_attempts(...)` returns `1` unless `PolicyEffectSet::has_bounded_retry()` is true. Because `has_bounded_retry()` ignores `MaxAttempts`, a validation-valid retry policy containing only `max_attempts=N` would not retry. This violates the phase invariant that supported declared policy effects must be enforced or rejected.

Actionable fix: choose one of these paths:

- Treat `MaxAttempts(_)` as bounded retry in `PolicyEffectSet::has_bounded_retry()` and add a regression test proving a retry policy with only `max_attempts=3` retries up to that cap; or
- Reject `max_attempts=N` unless paired with `retry` or `bounded_retry`, and add a validation test proving standalone `max_attempts=N` fails closed.

The first option is probably the smallest user-friendly fix because the plan describes `max_attempts=N` as a v0 retry effect.

## 6. Adapter Policy Boundary Assessment

The read-only adapter boundary is materially stronger after this phase.

The implementation:

- requires `allow_external_read` on policy requirements for external-read adapter-backed steps;
- rejects external write before runtime execution;
- rejects secret read before runtime execution;
- requires typed `AllowExternalRead` before the conservative policy engine allows supported Phase 2 read-only adapter invocation.

No provider mutation, write-capable adapter behavior, command output copying, or secret-read behavior was introduced.

Non-blocking follow-up: the direct conservative policy engine denial for a supported read-only adapter without `AllowExternalRead` still uses the broader `policy.deny.adapter_invoke_v0` reason code. Validation catches missing `allow_external_read` on project paths, so this is not blocking, but a future reason code such as `policy.deny.external_read_policy_missing` would make audits clearer.

## 7. Approval, Retry, And Escalation Assessment

Approval behavior is appropriately tied to typed `RequireApproval` and existing approval pause/resume tests continue to pass.

Escalation behavior is appropriately tied to typed `Escalate` and does not imply external paging, notification, or ticket creation.

Retry behavior is mostly preserved for existing fixtures that include `retry`/`bounded_retry` plus `max_attempts=N`, but the standalone `max_attempts=N` case is the blocker described above.

## 8. Privacy And Redaction Assessment

The implementation does not add:

- raw provider payload storage;
- raw command output storage;
- raw spec content copying;
- environment variable value storage;
- credential, authorization header, private key, or token-like value storage.

New validation and runtime errors are stable and bounded. Unsupported policy effects and unsupported actors are reported without echoing raw caller-supplied policy values.

## 9. Test Quality Assessment

Test coverage is strong for the intended P0 surface:

- supported read-only adapter policy effect allowance;
- missing read-only adapter policy effect denial;
- unsupported policy effects;
- unsupported actor bindings;
- wrong policy reference context;
- missing `allow_external_read`;
- external-write rejection before runtime;
- approval pause/resume behavior;
- retry behavior with existing bounded retry fixtures;
- escalation behavior;
- dogfood validation after policy split;
- no runtime side effects for invalid external write projects.

Missing/blocking test:

- No test covers a retry policy where `max_attempts=N` is the only retry effect. That test would expose the validation/runtime mismatch.

Non-blocking test follow-ups:

- Add a focused direct `PolicyEffectSet` unit test covering `MaxAttempts` semantics.
- Add a direct conservative-policy reason-code test if a more specific missing-read-policy reason code is added later.
- Add a scaffold validation test proving generated repo-governance policies keep local requirement and approval policies separate.

## 10. Documentation Review

The docs are honest about the implemented scope:

- P0 policy effect enforcement is implemented.
- Policy files are not an arbitrary policy language.
- Unsupported effects and actor bindings fail validation.
- Supported read-only adapter access requires `allow_external_read`.
- Broad policy DSLs, RBAC/IdP, write-capable adapters, side-effect execution, hosted policy service, schemas, recursive agents, agent swarms, and Level 3/4 autonomy remain unimplemented.

The docs should be updated after the blocker fix to clarify whether standalone `max_attempts=N` is valid retry behavior or must accompany `retry`/`bounded_retry`.

## 11. Blockers

1. Standalone `max_attempts=N` validates as a retry policy but does not enable runtime retry.

   Evidence:

   - `PolicyEffectSet::insert` stores `PolicyEffect::MaxAttempts(value)` only in `max_attempts`.
   - `PolicyEffectSet::has_bounded_retry()` returns true only for `BoundedRetry` or `Retry`.
   - `policy_effect_matches(...)` and `PolicyEffectContext::Retry` both treat `MaxAttempts(_)` as valid retry policy vocabulary.
   - `retry_max_attempts(...)` returns `1` when `has_bounded_retry()` is false, so a standalone `max_attempts=N` policy is accepted but not enforced as retry behavior.

## 12. Non-Blocking Follow-Ups

- Add a clearer conservative-policy reason code for missing `allow_external_read` in direct policy-engine evaluation.
- Clarify `allow_local` semantics in docs: it records local permission posture but does not create new local execution authority because local execution remains the conservative v0 default.
- Add direct unit tests for `PolicyEffectSet` behavior.
- Add generated scaffold validation coverage for split local, approval, and read-only policies.

## 13. Recommended Next Phase

Recommended next phase: **Policy effect enforcement blocker fix**.

Fix the retry `max_attempts=N` mismatch before moving to high-assurance approval controls, write-capable adapter readiness, or broader runtime enforcement work. This is a narrow fix, but it sits exactly on the phase invariant: if the effect is accepted, the kernel must enforce it; otherwise it must reject it.

## 14. Validation

Completed successfully during review:

- `npm run dogfood:benchmark -- validate`
- `cargo test -p workflow-core --test policy --test project_validation --test local_executor`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `npm run check:integrations`
- `git diff --check`
