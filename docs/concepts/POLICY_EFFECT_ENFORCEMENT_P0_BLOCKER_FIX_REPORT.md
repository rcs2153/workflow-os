# Policy Effect Enforcement P0 Blocker Fix Report

## 1. Executive Summary

The Policy Effect Enforcement P0 review blocker is fixed.

The reviewed blocker was a validation/runtime mismatch: standalone `max_attempts=N` validated as retry policy vocabulary, but runtime retry detection did not treat `MaxAttempts` as enabling bounded retry. That created a narrow false-governance path where a supported declared policy effect could pass validation without being enforced.

The fix makes `MaxAttempts` a bounded retry effect at the `PolicyEffectSet` boundary and adds focused regression tests proving both model behavior and executor behavior.

## 2. Blocker Fixed

Fixed blocker:

- Standalone `max_attempts=N` now enables bounded retry behavior when used through `retry_policy`.

Before the fix:

- validation accepted `max_attempts=N` as a retry effect;
- context validation allowed it in `retry_policy`;
- runtime stored the max-attempt value;
- runtime still returned one attempt because `has_bounded_retry()` ignored `MaxAttempts`.

After the fix:

- `PolicyEffectSet::has_bounded_retry()` returns true when `max_attempts` is present;
- retry max attempts are enforced from the typed effect set;
- a policy containing only `max_attempts=3` retries the current step and emits retry events with the expected cap.

## 3. Implementation Approach

The implementation chose the review's recommended enforcement path rather than rejecting standalone `max_attempts=N`.

Changes:

- Updated `PolicyEffectSet::has_bounded_retry()` to treat `MaxAttempts` as bounded retry.
- Updated retry validation's bounded check to use typed `PolicyEffectSet` behavior rather than stale raw-string matching.
- Added direct model coverage for `PolicyEffect::MaxAttempts`.
- Added local executor coverage proving a retry policy with only `max_attempts=3` retries the current step without rerunning prior steps.

No broad policy DSL, RBAC, IdP, actor authority, write-capable adapter, side-effect execution, schema change, hosted service, or release posture change was introduced.

## 4. Validation Boundary Summary

Validation and runtime now agree on the supported retry vocabulary:

- `retry` enables bounded retry;
- `bounded_retry` enables bounded retry;
- `max_attempts=N` enables bounded retry and supplies the attempt cap;
- malformed `max_attempts` still fails validation;
- unsupported retry effects still fail validation.

Validation errors remain stable and non-leaking.

## 5. Runtime Boundary Summary

The local executor continues to consume typed policy effects through `PolicyEffectSet`.

The retry runtime now treats standalone `max_attempts=N` as an actionable retry policy effect. Retry remains local, bounded, deterministic, and evented. The fix does not add external notifications, hosted behavior, provider writes, or side-effect execution.

## 6. Redaction And Security Summary

The fix does not add new payload storage, logging, provider access, command output capture, spec content copying, secret reads, or external writes.

No new errors include raw policy effect values, secret-like values, command output, provider payloads, or raw spec snippets.

## 7. Test Coverage Summary

Added or updated focused coverage for:

- `PolicyEffect::MaxAttempts(3)` enabling bounded retry in `PolicyEffectSet`;
- standalone `max_attempts=3` retry policy executing a retry of the current step;
- retry event emission preserving `max_attempts == 3`;
- existing policy, validation, and local executor tests.

## 8. Commands Run And Results

Completed successfully:

- `npm run dogfood:benchmark -- validate`
- `cargo test -p workflow-core --test policy --test local_executor --test project_validation`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `npm run check:integrations`

## 9. Remaining Known Limitations

- `PolicyEffect` remains a small v0 vocabulary, not a general policy language.
- Policy-rule actor authority is rejected rather than enforced.
- `allow_local` records local posture but does not create new execution authority.
- Read-only adapter gating remains limited to supported Phase 2 read-only adapters.
- External writes, secret reads, provider mutations, side-effect execution, hosted policy service, RBAC, IdP, schemas, and Level 3/4 autonomy remain unimplemented.

## 10. Recommended Next Phase

Recommended next phase: **Policy effect enforcement blocker fix review**.

The fix is narrow and directly addresses the reviewed blocker, but it touches policy validation/runtime semantics and should be reviewed before moving on to high-assurance approval controls or write-capable adapter readiness.
