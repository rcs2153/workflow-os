# Policy Effect Enforcement P0 Report

## 1. Executive Summary

The first P0 policy-effect enforcement slice is implemented.

The core invariant is now represented in code:

```text
Declared policy effects must be enforced or rejected.
```

Workflow OS now has a typed v0 policy-effect vocabulary, fail-closed validation for unsupported effects and unsupported actor bindings, policy-reference context validation, and runtime consumption of typed effects for approval, retry, escalation, and supported read-only adapter access.

This phase does not implement a broad policy DSL, RBAC, IdP integration, write-capable adapters, side-effect execution, schemas, hosted policy service, or Level 3/4 autonomy.

## 2. Scope Completed

Completed:

- Added typed `PolicyEffect` vocabulary.
- Added `PolicyEffectSet` for step-scoped runtime policy effects.
- Added stable parse errors for unsupported and malformed policy effects.
- Validated every policy rule effect against the supported v0 vocabulary.
- Rejected unsupported policy-rule `actor` bindings during validation.
- Rejected policy effects used in unsupported reference contexts.
- Required read-only adapter steps to cite a policy requirement with `allow_external_read`.
- Rejected external-write and secret-read adapter-backed steps before runtime execution.
- Fed typed effects into local executor step plans.
- Used typed approval effects for executor approval pauses.
- Used typed retry effects for bounded retry max attempts.
- Used typed escalation effects for local escalation availability.
- Required typed `allow_external_read` before the conservative policy engine allows supported Phase 2 read-only adapter invocation.
- Split dogfood, example, CLI scaffold, CLI test, and TypeScript SDK fixture policies so general requirements, approval, and read-only adapter access are not mixed into one decorative policy file.

## 3. Scope Explicitly Not Completed

Not implemented:

- broad policy DSL;
- OPA/Rego/Cedar or another embedded policy engine;
- RBAC, IdP, groups, teams, steward/admin policy management;
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

## 4. Model And Runtime Summary

The implemented v0 vocabulary includes:

- `allow_local`;
- `allow_external_read`;
- `require_approval` and existing approval aliases;
- `retry` and `bounded_retry`;
- `max_attempts=N`;
- `escalate` and existing escalation aliases.

The local executor resolves policy references into `PolicyEffectSet` during execution planning. Raw policy effect strings are parsed before runtime consumption. The conservative policy engine now requires `allow_external_read` in the context before allowing supported Phase 2 read-only adapter invocation.

## 5. Validation Boundary Summary

Validation now fails closed when:

- a policy rule effect is empty;
- a policy rule effect is unsupported;
- a parameterized effect such as `max_attempts=N` is malformed;
- a policy rule includes an unsupported actor binding;
- a policy referenced by `policy_requirements` contains non-requirement effects;
- a policy referenced by `approval_policy` contains non-approval effects;
- a policy referenced by `retry_policy` contains non-retry effects;
- a policy referenced by `escalation_policy` contains non-escalation effects;
- a read-only adapter-backed step lacks `allow_external_read`;
- an adapter-backed step requests external write;
- an adapter-backed step requests secret read.

Validation errors use stable codes and do not include raw policy effect values.

## 6. Redaction And Security Summary

The implementation does not add raw provider payload storage, command output storage, spec content copying, secret storage, or policy payload logging.

Unsupported policy effects and unsupported actor bindings are reported with stable diagnostic codes and bounded messages. Runtime policy-denial errors remain structured and non-leaking.

## 7. Test Coverage Summary

Focused coverage added or updated for:

- supported read-only adapter policy effect allowance;
- missing read-only adapter policy effect denial;
- unsupported policy effects;
- unsupported actor bindings;
- wrong policy reference context;
- missing `allow_external_read`;
- external-write rejection before runtime;
- existing approval pause/resume behavior;
- existing retry behavior and max attempts;
- existing escalation behavior;
- dogfood validation after policy split;
- local executor behavior after external-write validation tightening.

## 8. Commands Run And Results

Completed successfully:

- `npm run dogfood:benchmark -- validate`
- `cargo test -p workflow-core --test policy --test project_validation --test local_executor`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `npm run check:integrations`

## 9. Remaining Known Limitations

- `allow_local` remains a conservative local requirement effect; it does not create new local execution authority.
- Policy-rule actor authority is rejected rather than enforced.
- Policy effects are still a small v0 vocabulary, not a general policy language.
- Read-only adapter gating is limited to supported Phase 2 read-only adapters.
- External writes, secret reads, provider mutations, side-effect execution, hosted policy service, RBAC, IdP, schemas, and Level 3/4 autonomy remain unimplemented.
- Existing local executor runtime policy denial tests that previously used external write were tightened to validation rejection; separate future runtime-denial fixtures should use a validation-valid but runtime-denied supported scenario if needed.

## 10. Recommended Next Phase

Recommended next phase: **Policy effect enforcement review**.

This phase is security-sensitive because it changes validation posture and adapter-read policy gating. It should be reviewed before moving to broader high-assurance approval controls, write-capable adapter readiness, or any side-effect execution work.
