# Required Context Contract Consumption Review

## 1. Executive Verdict

**Needs blocker fixes.**

The model is narrow, payload-free, deterministic, and well tested, but the
consumption input does not independently bind the projections to the execution
context for which consumption is being requested.

Fix-forward status: the blocker is addressed in
[Required Context Contract Consumption Blocker Fix Report](REQUIRED_CONTEXT_CONTRACT_CONSUMPTION_BLOCKER_FIX_REPORT.md).
This note does not erase or revise the original finding. The correction must
receive a separate focused review before the phase is accepted.

## 2. Scope Verification

The phase stayed within the approved model/helper boundary. It did not add
target dereference, repository inspection, executor integration, persistence,
events, schemas, CLI behavior, providers, OpenShell, SideEffect execution,
writes, hosted behavior, or release changes.

## 3. Model Assessment

The model is domain-neutral and appropriately reuses existing typed reference,
access-level, sensitivity, harness-contract, and content-hash vocabulary.

The contract binding canonicalizes exact requirements and retains enough source
data to recompute its content hash. Existing name-only
`HarnessContextRequirement` values are not silently upgraded into enforceable
targets.

## 4. Exact-Matching Assessment

The helper correctly enforces:

- exact target-set equality;
- exact access-level equality;
- at most one projection per access level;
- exact shared actor, workflow, run, step, harness, and evaluation time across
  all supplied projections;
- exact harness-contract identity;
- required-gap blocking;
- explicit optional gaps;
- sensitivity ceilings; and
- rejection of ambient extra projected context.

## 5. Blocking Finding

### Independent execution-context binding is missing

`RequiredContextConsumptionInput` carries only the contract and projections.
`validate_projection_set` proves that the projections agree with the first
projection, but there is no independently supplied expected actor, workflow,
run, step, harness, or evaluation time.

Consequently, a coherent authorized projection from a different execution
context can be consumed successfully if it uses the same harness contract. The
result retains that wrong projection, but the helper has already returned
`Satisfied`; requiring every downstream caller to rediscover the mismatch would
make the model boundary misleading and brittle.

This violates the accepted plan's requirement that consumption bind to one
exact execution context. The fix should add one validated explicit consumption
context, retain it in the result, compare every projection against it, and
recompute that equality during deserialization.

## 6. Authority And Least-Privilege Assessment

Subject to the blocker above, the authority boundary is correct:

- declarations do not issue grants;
- availability does not imply authority;
- approval cannot manufacture required context;
- broader metadata access does not satisfy reference-only requirements;
- extra projected context fails closed; and
- satisfaction is not a dereference lease.

## 7. Serde And Privacy Assessment

Contract and aggregate-result deserialization recompute content and derived
posture and reject tampering. Errors are stable and non-leaking. Debug output
redacts sensitive identities, and the wire model contains no raw payload fields.

The standalone `RequiredContextSatisfaction` and `RequiredContextGap` types are
informational records rather than authority. Their direct serde shape is not a
current aggregate-integrity blocker because `RequiredContextConsumptionResult`
recomputes them from retained sources, but their non-authoritative status should
remain explicit and later encapsulation may be preferable before schema
exposure.

## 8. Test Assessment

The focused suite covers exact matching, required and optional gaps, missing
authority, overbroad access, extra context, sensitivity, serde tampering, and
privacy. It does not cover a valid projection from a different expected
actor/run/step because no expected-context input exists.

The blocker fix must add mismatch tests for every execution-context dimension
and result-wire substitution.

## 9. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed before this review file.
- `git diff --check`: passed before this review file.

Green checks do not resolve the semantic-binding blocker.

## 10. Blockers

1. Add an independently supplied, validated exact execution context to the
   consumption input.
2. Retain that context in the result and enforce exact projection equality
   during construction, validation, and deserialization.
3. Add cross-actor, workflow, run, step, harness, timestamp, and serialized
   context-substitution regression tests.

## 11. Non-Blocking Follow-Ups

- Consider reducing standalone serde exposure of derived satisfaction and gap
  records before any schema or SDK surface.
- Bind the accepted contract and consumption context to the immutable run bundle
  in a separately reviewed phase.
- Require fresh time-of-use authority resolution before any target dereference.

## 12. Recommended Next Phase

Perform a focused **required-context execution-binding blocker fix**.

Do not add target dereference, runtime consumption, persistence, events,
schemas, CLI behavior, providers, OpenShell, SideEffect execution, writes,
hosted administration, or release changes.
