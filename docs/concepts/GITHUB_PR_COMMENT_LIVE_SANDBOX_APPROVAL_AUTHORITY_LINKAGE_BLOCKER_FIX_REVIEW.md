# GitHub PR Comment Live Sandbox Approval Authority Linkage Blocker Fix Review

## 1. Executive Verdict

Blockers fixed; proceed to proportional-governance core decision model after
PR #320 merges.

The initial review found two write-adjacent authority-boundary blockers. Both
findings are preserved below and are now fixed. The final composition removes
caller-selected approval readiness as authority, binds caller run context to
read-only durable backend state, validates proof-enforced presentation plus
persisted SideEffect linkage, and preserves workflow snapshot/event state
before provider invocation.

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

## 12. Fix-Forward Note

The durable-run blocker identified by this review is now fixed on draft PR
#320. The proof-bound helper rehydrates through the executor backend, requires
exact equality with caller-supplied run context, and uses the durable run for
presentation and SideEffect linkage checks. Focused regressions prove a
tampered caller snapshot and unavailable durable run both block before provider
invocation with bounded errors. This note preserves the original finding; a
focused re-review remains required before merge.

The focused re-review found one final boundary issue: the first durable binding
used `rehydrate_and_project(...)`, which saves a snapshot after rehydration.
That establishes durable truth but performs a local state write inside a helper
that must remain non-mutating apart from the separately authorized provider and
SideEffect outcome path. The binding must instead call the backend's read-only
`rehydrate_run(...)` path and prove pre-provider authority validation does not
create or rewrite snapshot state. PR #320 remains draft until this is fixed and
re-reviewed.

That read-only correction is now implemented. The helper calls the backend's
`rehydrate_run(...)` method directly, and the positive authority test asserts
the persisted workflow snapshot and event sequence are identical before and
after composition. The separately authorized provider path still updates only
the SideEffect outcome record. Final acceptance remains subject to focused
re-review and required CI.

## 13. Final Re-Review Verdict

Accepted.

The final implementation:

- reads durable events through `StateBackend::rehydrate_run(...)` without
  snapshot projection;
- requires exact equality between durable and supplied run context;
- uses the durable run for presentation and SideEffect linkage checks;
- derives `LinkedAndApproved` only after proof and linkage succeed;
- blocks non-proof policy, missing/stale presentation, wrong approval,
  missing linkage, unavailable durable state, and caller/durable mismatch
  before provider invocation;
- leaves persisted workflow snapshot and events unchanged during successful
  composition;
- keeps the ignored live-provider test explicit and unexecuted in this phase.

No blockers remain in the approved scope. Required local validation passed.
Merge remains conditional on all five required GitHub checks passing.

Final governed re-review:

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783804328092677000-2`.
- Approval ID:
  `approval/run-1783804328092677000-2/review-scope-approved`.
- Approval presentation ID: `presentation/9256cd5e0ff359b1`.
- Approval outcome: granted by the delegated maintainer through the
  proof-enforced path.
- Event summary: 39 ordered events, including one approval request, one grant,
  eight policy decisions, six scheduled steps, six successful skill
  invocations, and one completed run; no retries or escalations.
- Validation summary: eight focused tests, formatting, clippy with warnings
  denied, the complete workspace suite, docs validation, and diff hygiene
  passed; the ignored live-provider test was not executed.
- Out-of-kernel work: Codex performed code and documentation inspection,
  validation commands, git operations, and GitHub PR/CI inspection. The kernel
  governed scope and approval but did not execute commands, edit files, call
  GitHub, or mutate the repository.
