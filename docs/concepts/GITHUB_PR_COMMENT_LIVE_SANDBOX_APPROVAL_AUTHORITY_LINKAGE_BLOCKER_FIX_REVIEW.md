# GitHub PR Comment Live Sandbox Approval Authority Linkage Blocker Fix Review

## 1. Executive Verdict

Needs additional blocker fixes.

The new composition correctly removes caller-selected approval readiness as
authority and validates proof-enforced presentation plus persisted SideEffect
linkage before provider invocation. One write-adjacent authority gap remains:
the helper trusts the caller-supplied `WorkflowRun` as the approval event source
without first proving it exactly matches the executor's durable event state.

## 2. Scope Verification

The implementation stayed within the approved explicit local blocker-fix
scope. It added one opt-in composition helper, request/result types, focused
tests, exports, roadmap status, and a phase report.

It did not add default writes, hidden auth, CLI mutation, retries, repair,
report artifacts, schemas, broader adapters, hosted behavior, reasoning
lineage, higher autonomy, or release changes. The ignored live test was updated
but not executed, so no new external effect occurred.

## 3. Authority Composition Assessment

The helper correctly:

- rejects non-proof presentation policy;
- requires matching terminal run and SideEffect identity;
- resolves a stable approval-decision reference from the attempted SideEffect;
- validates approval-presentation proof through the existing executor boundary;
- validates store-backed SideEffect approval linkage;
- derives `LinkedAndApproved` only after those gates;
- delegates to the accepted sandbox/provider helper afterward.

The ordering prevents provider invocation for the covered proof and linkage
failures.

## 4. Durable Run Authority Blocker

The request accepts `run: &WorkflowRun`, and the helper uses that value directly
for approval request lookup, approval decision lookup, and SideEffect linkage.
`WorkflowRun` is a public serializable model. A valid-shaped in-memory run is
not itself proof that its events are the durable executor history.

The executor has an existing rehydration boundary backed by `StateBackend`.
Before any presentation or linkage decision, the helper must:

1. rehydrate the run by supplied run ID through the executor backend;
2. fail closed if rehydration fails;
3. require the rehydrated run to equal the supplied run exactly;
4. use the rehydrated run, not caller memory, for all authority checks.

Without this binding, the complete durable authority-to-effect claim is not
yet established.

## 5. Fail-Closed And Privacy Assessment

The implemented missing-proof, stale-proof, wrong-approval, missing-linkage,
and non-proof-policy failures block before provider invocation and use stable,
non-leaking errors. Request/result Debug output is bounded and redacted.

The durable-run mismatch fix must likewise expose only a stable error code and
must not print event bodies, approval IDs, presentation IDs, actor values,
reasons, targets, paths, or provider inputs.

## 6. Test Assessment

Current focused coverage is strong for:

- successful derivation from proof and linkage;
- caller readiness posture being ignored as authority;
- missing and stale presentation proof;
- wrong approval identity;
- missing persisted SideEffect linkage;
- non-proof policy rejection;
- provider call count remaining zero for blocked paths;
- Debug non-leakage;
- compilation of the ignored real-provider harness through the new path.

Required blocker regression:

- a caller-supplied run that differs from durable backend state must fail before
  provider invocation with a stable non-leaking error;
- the matching durable run must continue to pass;
- backend rehydration failure must block without provider invocation.

## 7. Documentation Assessment

The implementation report accurately describes the intended proof-bound path,
but its complete authority-chain claim depends on the durable-run fix above.
The roadmap now records that remaining blocker rather than claiming expansion
readiness.

## 8. Blockers

1. Bind the caller-supplied run exactly to executor-rehydrated durable state and
   use the durable run for approval presentation and SideEffect linkage checks.

## 9. Non-Blocking Follow-Ups

- Keep the lower-level unbound live-sandbox helper for compatibility but do not
  describe it as a complete authority chain.
- Preserve the ignored, explicit live-provider posture.
- Keep ambiguous provider recovery and report artifacts separately gated.

## 10. Recommended Next Phase

Recommended next phase: focused durable-run authority binding blocker fix on
draft PR #320.

Do not repeat the live provider call. After the fix and focused re-review, the
next planned implementation remains the proportional-governance core decision
model.

## 11. Validation And Governed Review Evidence

Validation inspected during review:

```sh
cargo test -p workflow-core --test local_executor live_sandbox_approval_authority
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

All local checks passed. Required GitHub CI was still running when the durable
run blocker was identified; the PR remains draft and unmerged regardless of CI
outcome.

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783801217955753000-2`.
- Approval ID:
  `approval/run-1783801217955753000-2/review-scope-approved`.
- Approval presentation ID: `presentation/78d878698bf8acbb`.
- Approval outcome: granted by the delegated maintainer through the
  proof-enforced path.
- Event summary: 39 ordered events, including one approval request, one grant,
  eight policy decisions, six scheduled steps, six successful skill
  invocations, and one completed run; no retries or escalations.
- Out-of-kernel work: Codex inspected implementation, tests, docs, local
  validation, and PR state; authored this review; and will perform git/PR
  operations. The kernel governed scope and approval but did not execute
  commands, edit files, call GitHub, or mutate the repository.
