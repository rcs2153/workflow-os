# Capability Resolution Helper Report

## 1. Executive Summary

Workflow OS now has a pure, deterministic, in-memory capability resolution
helper. It composes explicit capability availability with validated scoped
grants and exact actor, resource, workflow, run, step, harness, sensitivity,
and evaluation-time inputs.

The helper preserves the core authority invariant:

```text
Availability is inventory, not permission. A grant is scoped authority, not
proof that independent policy, approval, evidence, or check obligations passed.
```

No executor, adapter, tool, provider, or persistence path consumes this result.

## 2. Scope Completed

- Added `CapabilityResolutionInput` for explicit borrowed inputs.
- Added `CapabilityResolution`, `CapabilityResolutionPosture`, and stable
  `CapabilityResolutionReason` vocabulary.
- Added `resolve_capability_authority` as a pure helper.
- Matched exact capability, resource, actor, workflow, run, step, and optional
  harness scope.
- Enforced active lifecycle, explicit evaluation-time expiry, requested
  sensitivity ceiling, current availability, and deterministic grant choice.
- Returned `RequiresIndependentEvaluation` when a matching grant declares
  policy, approval, evidence, or local-check prerequisites.
- Evaluated only the most-specific matching grant tier, preventing a broader
  grant from bypassing narrower policy, approval, evidence, or check
  prerequisites; grant ID is the deterministic tie-breaker within that tier.
- Added validated deserialization and redaction-safe `Debug` behavior.

## 3. Scope Explicitly Not Completed

This phase does not add:

- executor or workflow runtime integration;
- capability requests;
- step-scoped tool or context projection;
- authority receipts;
- policy, approval, evidence, or check evaluation;
- persistence, events, audit records, or report artifacts;
- schemas, SDKs, CLI behavior, or examples;
- connector activation, credential loading, tool invocation, or provider
  mutation;
- hosted administration, RBAC, IdP, or enterprise identity;
- runtime proportional-governance enforcement;
- release-posture changes.

## 4. Helper API Summary

`resolve_capability_authority(&CapabilityResolutionInput)` accepts:

- requested capability and bounded resource;
- actor, workflow, run, step, and optional harness identity;
- requested sensitivity;
- explicit evaluation timestamp;
- current bounded availability records;
- validated candidate grants.

It returns one posture:

- `Authorized`: one active, current, matching grant has no unresolved
  prerequisite declarations;
- `RequiresIndependentEvaluation`: a matching grant exists, but policy,
  approval, evidence, or check evaluation remains necessary;
- `NotAuthorized`: availability or authority is absent, unknown, unsupported,
  expired, revoked, mismatched, or insufficient.

The result includes stable reasons, the matched availability posture, the
selected grant reference when applicable, and the explicit evaluation time.

## 5. Authority Boundary

The resolver does not treat these facts as interchangeable:

- capability availability;
- grant existence;
- grant lifecycle and expiry;
- actor and resource scope;
- workflow, run, step, and harness scope;
- sensitivity ceiling;
- independent prerequisite decisions.

`Available` without a matching grant returns `NotAuthorized`. A matching grant
with prerequisite references returns `RequiresIndependentEvaluation`; the
helper does not infer that a referenced policy passed or that approval,
evidence, or check records are valid.

## 6. Determinism And Failure Behavior

- Multiple matching availability records fail with
  `capability_authority.resolution.availability_ambiguous`.
- Duplicate grant IDs fail with
  `capability_authority.resolution.duplicate_grant`.
- Future-dated availability observations fail closed.
- Unknown requested sensitivity fails closed.
- Revoked, expired, and sensitivity-insufficient grants do not authorize.
- Grant selection evaluates only the highest-specificity run, step, and harness
  tier, then uses stable grant identity within that tier. Broader grants cannot
  bypass narrower restrictions.
- Missing or unavailable inventory produces a bounded non-authorized result,
  not a fabricated capability or implicit connector request.

## 7. Privacy And Redaction

The input and result use validated references and bounded enums only. They do
not store credentials, environment values, raw provider payloads, source
contents, command output, prompts, transcripts, or unrestricted metadata.

`Debug` redacts capability, resource, actor, workflow, run, step, harness, and
selected grant identifiers. Validation errors use stable codes and do not echo
caller-supplied identifiers. Serialized results may expose the validated stable
grant reference for machine composition, but impossible or secret-like
serialized states fail closed.

## 8. Test Coverage

Focused tests cover exact active authorization; availability without authority;
missing and unavailable inventory; independent prerequisite posture; actor,
run, step, and harness mismatch; revoked and expired grants; sensitivity
ceilings; deterministic grant selection; prevention of broad-grant bypass;
ambiguous input; validated serde; and Debug/error non-leakage.

## 9. Validation

- `cargo test -p workflow-core --test capability_authority`: passed, 29 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Known Limitations

- Availability freshness has an observation/evaluation ordering check, but no
  caller-supplied maximum age policy yet.
- Policy, approval, evidence, and check prerequisites are disclosed, not
  evaluated.
- The helper is not bound to immutable run-bundle capability requirements yet.
- No runtime action revalidates this result before invocation.
- No capability request, tool projection, context projection, or authority
  receipt exists.
- No runtime WorkReport artifact is generated or persisted for this phase.

## 11. Recommended Next Phase

Perform a focused maintainer review of the capability resolution helper.

After acceptance, implement the capability request model and review-only
projection. Do not jump directly to tool invocation or provider mutation.

## 12. Governed Phase Evidence

- Workflow: `dg/implement`.
- Run ID: `run-1784166455417821000-2`.
- Approval ID:
  `approval/run-1784166455417821000-2/implementation-approved`.
- Approval presentation: `presentation/f961ccf7e05b24f5`.
- Approval outcome: granted with persisted presentation proof under delegated
  maintainer authority.
- Out-of-kernel work: Codex read the accepted authority model and plan, edited
  Rust and documentation, and ran validation commands. The kernel coordinated
  governance only.
- Report posture: this document is the phase report; no runtime WorkReport
  artifact was generated or persisted.
