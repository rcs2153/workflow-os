# Policy Effect Enforcement P0 Plan

Status: First P0 implementation slice complete in [Policy Effect Enforcement P0 Report](../concepts/POLICY_EFFECT_ENFORCEMENT_P0_REPORT.md). The implementation adds typed v0 policy effects, fail-closed validation for unsupported effects and unsupported actor bindings, context validation for policy references, and local executor/conservative-policy consumption for approval, retry, escalation, and supported read-only adapter access. Broader policy DSLs, RBAC/IdP, write-capable adapters, side-effect execution, schemas, examples beyond fixture correction, hosted policy service, and Level 3/4 autonomy remain unimplemented.

## 1. Executive Summary

User testing identified a P0 governance gap: policy files can look enforceable while the current runtime only interprets a narrow subset of policy behavior. Today, policy specs carry rule `effect` strings; semantic validation checks that referenced policies exist, checks a few approval/retry/escalation effect names, and the local executor consumes retry `max_attempts` in one bounded path. The runtime policy engine itself evaluates conservative action/capability context, not the full policy-file effect vocabulary.

That creates a false-governance risk. If a user can declare a policy effect, Workflow OS must either enforce that effect deterministically or reject it during validation. Decorative policy strings are not acceptable for P0 governance.

This plan defines the narrow fix:

- create a small v0 typed policy-effect vocabulary;
- validate policy effects and their allowed reference contexts;
- reject unsupported, ambiguous, or misplaced effects fail-closed;
- feed enforced policy effects into the local executor and conservative policy evaluation boundary;
- keep write-capable adapters, broad policy languages, RBAC, hosted policy administration, schemas, CLI behavior, examples, and release posture changes out of scope.

## 2. P0 Requirement

The new invariant is:

```text
Declared policy effects must be enforced or rejected.
```

This means:

- supported effects have deterministic runtime behavior;
- unsupported effects fail validation;
- supported effects in the wrong context fail validation;
- policy rule fields that imply authority, actor binding, data access, or external capability must not be accepted unless Workflow OS enforces that meaning;
- examples and generated scaffolds must not imply policy guarantees that the runtime does not provide.

## 3. Current Baseline

Implemented today:

- `PolicySpecDocument` parses `policies/*.policy.yml`.
- `PolicyRuleShell` stores `id`, `effect: String`, and optional `actor`.
- project validation verifies that referenced policies exist.
- project validation checks expected approval/retry/escalation effects for `approval_policy`, `retry_policy`, and `escalation_policy`.
- retry policies are bounded by validation and `retry_max_attempts(...)` reads `max_attempts=` or `max_attempts:` from policy rule effects.
- `ConservativePolicyEngine` denies unknown actions, unknown capabilities, unsupported adapters, external writes, secret reads, unsafe autonomy, and sensitive actions without approval policy context.
- Phase 2 read-only adapter policy prechecks exist and fail closed when missing or denied.

Not implemented today:

- a typed `PolicyEffect` model;
- validation of every policy rule effect against a supported vocabulary;
- validation of effect placement across `policy_requirements`, `approval_policy`, `retry_policy`, and `escalation_policy`;
- runtime enforcement of arbitrary `policy_requirements` effects;
- mapping `allow_external_read` from policy specs into adapter/read permission;
- actor-aware policy rule enforcement;
- a general policy language;
- RBAC, IdP, steward/admin policy management, or hosted policy enforcement.

## 4. Goals

- Eliminate false-governance policy declarations.
- Keep the v0 policy vocabulary intentionally small.
- Make validation fail closed for unknown, unsupported, ambiguous, or misplaced effects.
- Preserve deterministic local executor behavior.
- Preserve existing conservative default denials.
- Require explicit policy effects for supported external read behavior.
- Preserve approval, retry, and escalation semantics while making the effect boundary typed and testable.
- Keep validation diagnostics stable, specific, and non-leaking.
- Update docs to make policy posture honest.
- Prepare the runtime for later side-effect and write policy work without enabling writes now.

## 5. Non-Goals

Do not implement in this phase:

- a broad policy DSL;
- OPA/Rego/Cedar or another embedded policy engine;
- policy expression parsing beyond the accepted v0 vocabulary;
- RBAC, IdP integration, teams, groups, or enterprise steward/admin policy controls;
- quorum approval, revocation, delegated authority, or separation-of-duty enforcement;
- write-capable adapters;
- provider mutations;
- side-effect execution;
- hosted/distributed policy service;
- workflow schema changes;
- public CLI behavior changes beyond diagnostics from existing validation commands;
- examples;
- automatic workflow generation;
- recursive agents or agent swarms;
- Level 3/4 autonomy enablement;
- release posture changes.

## 6. Enforced v0 Policy Effect Vocabulary

The first implementation should define a typed vocabulary such as `PolicyEffect`.

Recommended v0 effects:

| Effect | Canonical meaning | Allowed context | Runtime behavior |
| --- | --- | --- | --- |
| `require_approval` | Step requires an approval checkpoint before invocation. | `approval_policy`; optionally `policy_requirements` only if runtime enforcement is added there in the same phase. | Local executor pauses before skill invocation unless approval already exists. |
| `bounded_retry` | Retry is explicitly bounded. | `retry_policy` | Local executor uses bounded retry behavior. |
| `max_attempts=N` | Retry cap, bounded and positive. | `retry_policy` | Local executor caps attempts at `N`. |
| `escalate` | Retry exhaustion or terminal behavior may escalate according to existing local semantics. | `escalation_policy` | Local executor treats escalation as available only when the referenced policy has this effect. |
| `allow_external_read` | Allows supported read-only external adapter access when all other conservative policy checks pass. | `policy_requirements` for adapter-backed read-only skills | Conservative policy/adaptor precheck may allow supported Phase 2 read-only adapter invocation. |

Compatibility aliases may be accepted only if they are canonicalized immediately and documented. Candidate aliases already recognized by validation include:

- approval: `approval`, `approval_policy`;
- retry: `retry`, `retry_policy`;
- escalation: `escalation`, `escalation_policy`.

The implementation should prefer canonical effects in generated docs and scaffolds.

## 7. Unsupported Effects

Unknown or unsupported effects must fail validation with a stable diagnostic code, such as:

- `validation.policy.effect_unsupported`;
- `validation.policy.effect_context_invalid`;
- `validation.policy.effect_parameter_invalid`;
- `validation.policy.actor_unsupported`.

Examples of effects that must not be silently accepted:

- `allow_external_write`;
- `allow_secret_read`;
- `allow_shell`;
- `allow_command_execution`;
- `allow_provider_mutation`;
- `allow_jira_write`;
- `allow_github_write`;
- `allow_level_3`;
- `allow_level_4`;
- arbitrary strings that look like future capabilities.

Unsupported deny-like effects should also be rejected unless they have implemented runtime semantics. A policy file should not be allowed to communicate guarantees that the kernel does not enforce.

## 8. Actor And Authority Policy

`PolicyRuleShell.actor` is currently parsed, but there is no general actor-authority enforcement model for policy rules.

P0 recommendation:

- do not treat `actor` as enforceable authority unless the implementation explicitly validates and enforces it;
- either reject `actor` on enforced policy effects for now, or support only a tightly scoped non-authority use with clear documentation;
- update existing fixtures or generated policy examples if they currently include actor fields that read as enforceable authority.

This prevents a second false-governance surface where users believe a policy is limited to a human, team, service account, or system actor when the runtime has not checked that boundary.

## 9. Validation Boundary

Validation should become the first fail-closed line.

Required validation behavior:

1. Every policy rule effect parses to a supported canonical `PolicyEffect`.
2. Parameterized effects, such as `max_attempts=N`, validate bounds and syntax.
3. Unknown effects fail validation.
4. Supported effects in unsupported reference contexts fail validation.
5. Policies referenced by `approval_policy` must contain an approval effect.
6. Policies referenced by `retry_policy` must contain bounded retry behavior and no unbounded retry behavior.
7. Policies referenced by `escalation_policy` must contain escalation behavior.
8. Policies referenced by adapter-backed read-only steps must contain `allow_external_read` when external read capability is required.
9. External write, secret read, shell execution, and provider mutation policy claims fail validation until separately implemented.
10. Validation diagnostics must be stable and must not leak raw spec payloads, paths beyond source locations, secrets, tokens, command output, or provider payloads.

## 10. Runtime Enforcement Boundary

Runtime enforcement should consume typed policy effects, not raw strings.

Recommended implementation shape:

- resolve step policy references into a step-scoped `PolicyEffectSet`;
- attach the effect set to the local `StepExecutionPlan` or policy evaluation context;
- keep raw policy rules out of runtime decision branches after parsing/validation;
- require approval before invocation when the step has a typed approval requirement;
- permit bounded retry only through typed retry effects;
- permit escalation only through typed escalation effects;
- allow supported Phase 2 read-only adapter invocation only when typed `allow_external_read` is present and the conservative policy engine otherwise permits the action;
- keep external write and secret read denied regardless of policy files in this P0 phase.

Runtime failures must use structured non-leaking errors and policy reason codes.

## 11. Adapter Policy Precheck Integration

The read-only adapter boundary must not rely on fixture-like policy posture in production-shaped paths.

Required posture:

- adapter-backed local executor paths derive runtime policy precheck from typed policy effects;
- missing `allow_external_read` denies supported read-only adapter invocation;
- unsupported adapters remain denied even with `allow_external_read`;
- external writes remain denied even if a policy file attempts to allow them;
- reason codes distinguish missing policy, unsupported adapter, unsupported capability, and denied write behavior.

The first implementation should remain Phase 2 read-only only.

## 12. Approval, Retry, And Escalation Semantics

Approval:

- a policy effect requiring approval must create a real executor approval checkpoint or fail validation if placed somewhere the executor cannot honor;
- validation must reject approval-like effects that are not wired to runtime behavior.

Retry:

- retry policies must remain bounded;
- `max_attempts` must be deterministic, positive, and capped if the implementation defines a maximum;
- malformed retry parameters fail validation.

Escalation:

- escalation effects must not imply external notification, ticket creation, or human paging;
- escalation remains within current local executor semantics until separately implemented.

## 13. Migration And Compatibility

This is a P0 correctness fix and may require tightening validation.

Expected migration work:

- update fixture policies to use canonical effects;
- remove or reject unsupported `actor` fields unless enforced;
- update generated scaffold policies if they contain non-enforced rule fields;
- update docs that imply arbitrary policy effects are enforceable;
- add migration notes for users who created custom policy strings.

Compatibility principle:

```text
It is better for a policy to fail validation than to pass while implying an unenforced guarantee.
```

## 14. Test Plan

Future implementation must add focused tests for:

1. valid canonical `require_approval` policy passes validation;
2. valid canonical `bounded_retry` and `max_attempts=N` policy passes validation;
3. valid canonical `escalate` policy passes validation;
4. valid canonical `allow_external_read` policy passes validation for supported read-only adapter steps;
5. unknown policy effect fails validation;
6. write-like allow effect fails validation;
7. secret-read allow effect fails validation;
8. command-execution allow effect fails validation;
9. approval effect in an unenforced context fails validation or is enforced by runtime;
10. retry effect outside `retry_policy` fails validation;
11. escalation effect outside `escalation_policy` fails validation;
12. actor fields are rejected or enforced according to the accepted P0 policy;
13. malformed `max_attempts` fails validation;
14. unbounded retry remains rejected;
15. adapter-backed read-only step without `allow_external_read` is denied;
16. supported read-only adapter with `allow_external_read` is allowed only when all conservative policy checks pass;
17. unsupported adapter remains denied even with `allow_external_read`;
18. external write remains denied even with a policy file attempting to allow it;
19. approval-required step still pauses and resumes correctly;
20. retry max attempts are preserved;
21. escalation semantics are preserved;
22. validation diagnostics use stable codes;
23. validation and runtime errors do not leak secret-like values;
24. existing project loader, validation, executor, adapter telemetry, WorkReport, side-effect, hook, dogfood, CLI, and TypeScript SDK tests still pass.

## 15. Documentation Updates

Future implementation should update:

- `docs/specs/workflows.md`;
- README or getting-started docs only if they mention policy guarantees;
- scaffold/onboarding docs that show policy examples;
- relevant phase reports and reviews.

Docs must say:

- policy effects are a small supported vocabulary, not arbitrary strings;
- unsupported effects fail validation;
- declared supported effects are enforced by runtime boundaries;
- `allow_external_read` is read-only and limited to supported adapters;
- external writes, provider mutations, shell execution, secret reads, hosted policy service, RBAC/IdP, schemas, examples, and Level 3/4 autonomy are not implemented by this P0 fix.

## 16. Proposed Implementation Sequence

1. Add a typed policy-effect parser/model and focused unit tests.
2. Wire policy-effect validation into project validation.
3. Tighten context-specific policy reference validation.
4. Resolve step-scoped typed policy effects for local executor plans.
5. Enforce approval/retry/escalation from typed effects.
6. Gate supported read-only adapter invocation on typed `allow_external_read`.
7. Add non-leaking error and diagnostic coverage.
8. Update docs and create an end-of-phase report.
9. Run a maintainer review before any write-capable adapter or broader policy work.

## 17. Recommended Next Phase

Recommended next implementation phase:

```text
Policy effect typed vocabulary and validation fail-closed implementation.
```

This should be code-bearing and narrow. It should not start with write-capable adapters, side-effect execution, RBAC, schemas, or a broad policy language. Runtime enforcement for the supported effects should follow in the same P0 lane or the immediately subsequent implementation slice; the lane is not complete until supported effects are both validated and enforced.
