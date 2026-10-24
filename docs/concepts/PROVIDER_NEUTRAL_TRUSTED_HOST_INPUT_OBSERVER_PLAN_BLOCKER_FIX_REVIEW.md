# Provider-Neutral Trusted-Host Input Observer Plan Blocker-Fix Review

Status: focused re-review complete; blocker fixed.

Reviewed materials:

- [Provider-Neutral Trusted-Host Input Observer
  Plan](../implementation-plans/provider-neutral-trusted-host-input-observer-plan.md)
- [Original Focused Plan
  Review](PROVIDER_NEUTRAL_TRUSTED_HOST_INPUT_OBSERVER_PLAN_REVIEW.md)

## 1. Executive Verdict

**Blocker fixed; proceed to the provider-neutral observer Core model.**

The corrected plan now binds the independently observed repository input root
that was missing from the original result boundary. It reuses the accepted
`TrustedHostRootObservation` vocabulary, binds the exact static repository
commitment, requires read-only and non-invocation-scoped posture, defines
method/object/continuity coherence, and adds the required planned regressions.

No parallel root model or new runtime authority was introduced.

## 2. Scope Verification

The fix remained documentation-only. It did not add Core code, host
inspection, Git integration, process execution, provider behavior, runtime
admission, persistence, schemas, CLI behavior, examples, writes, authority, or
release changes.

## 3. Repository-Root Composition

The plan now requires exactly one `TrustedHostRootObservation` with:

- `TrustedHostRootRole::RepositoryInput`;
- the exact repository commitment accepted by static discovery;
- `TrustedHostRootAccess::ReadOnly`;
- object-binding and continuity postures;
- one closed `TrustedHostRootObservationMethod`;
- `invocation_scoped = false`; and
- one valid telemetry reference.

This is the correct reuse boundary. The nine host obligations remain a closed
set, while repository-root identity is modeled as the cross-cutting root
observation already accepted by the live-security contract.

## 4. Coherence And Failure Behavior

The corrected plan fails closed for:

- missing or duplicate repository roots;
- wrong root role;
- writable repository access;
- invocation-scoped repository creation;
- static repository commitment substitution; and
- incoherent method, object-binding, or continuity postures.

Unavailable, unsupported, degraded, and uncertain repository observations
remain representable but cannot support a future satisfied assessment. This
preserves honest evidence without converting incompleteness into authority.

## 5. TOCTOU And Provider Boundary

The plan does not pretend that the model proves live object identity. It keeps
protected handles, immutable projections, and provider-native identities as
closed methods while deferring selection of the live mechanism.

Git-specific observation is not coupled into Core. Any provider that later
constructs the observation must independently prove its method and continuity
semantics.

## 6. Test Coverage Assessment

The planned regressions now cover:

- required repository-root presence and role uniqueness;
- exact static repository commitment binding;
- writable, invocation-scoped, and wrong-role rejection;
- incoherent root method/posture rejection;
- unavailable binding remaining non-satisfying; and
- reuse of existing root-observation semantics.

This is sufficient for the model-only implementation phase. Live provider and
host tests remain correctly deferred.

## 7. Remaining Non-Blocking Follow-Ups

The original review's non-blocking follow-ups remain:

- pin the first contract's Cargo/Rust scope;
- select a semantically correct observation evidence-reference kind;
- pin environment influence classes;
- decide dynamic-library completeness requirements; and
- define cache freshness without wall-clock-only authority.

These do not block a non-authorizing Core model.

## 8. Recommended Next Phase

Implement the provider-neutral observer request/result/fact Core model only,
including canonical commitments, exact repository-root composition, strict
serde, privacy-safe Debug/errors, and focused tests.

Do not inspect a host, run a process, add a callable observer, integrate a
provider, admit execution, persist observations, add schemas or CLI behavior,
enable writes, or change release posture.

## 9. Governed Review Record

- Workflow ID: `dg/review`.
- Run ID: `run-1792833702646205000-2`.
- Approval ID:
  `approval/run-1792833702646205000-2/review-scope-approved`.
- Approval-presentation ID: `presentation/7f8f265b8e56819b`.
- Approval-presentation content hash:
  `7f8f265b8e56819b67ca5025c0d6d20af7edf1f97df0c06fc190522b89175e7d`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact re-review scope.
- Phase status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, zero retries, and zero
  escalations. Approval presentation enforcement was proof-enforced and its
  event marker is present.
- Validation summary: `npm run check:docs` and `git diff --check` passed.
- Out-of-kernel work: the delegated maintainer inspected the corrected plan,
  original finding, and existing root-observation model; authored this review
  and roadmap update; and ran documentation validation. The kernel governed
  scope and approval but did not edit files, run validation, or perform git or
  pull-request operations.
