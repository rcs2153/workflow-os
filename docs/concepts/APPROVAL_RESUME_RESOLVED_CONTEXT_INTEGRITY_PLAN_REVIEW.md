# Approval Resume Resolved-Context Integrity Plan Review

## 1. Executive Verdict

**Plan accepted; proceed to the P0 blocker fix.**

The plan accurately captures the current-main TOCTOU condition and defines a
bounded fail-closed fix that validates the complete executable declarative
context before any grant-side durable mutation.

## 2. Finding Verification

Current executor evidence confirms:

- all local approval APIs reach `apply_approval_decision`;
- granted decisions append `ApprovalGranted`, resume policy, and `RunResumed`
  before current project state is validated;
- `prepare_resume_execution` rebuilds from current project files;
- the rebuilt plan is located by approved `step_id` only;
- the rebuilt workflow hash is not compared to run identity before mutation;
- skill and policy content hashes are not bound to the approval;
- request-side checkpoint, hook, and SideEffect inputs are reconstructed as
  defaults.

The external report cited an older commit, but the defect remains present on
current `main` after PR 322.

## 3. Scope Assessment

The plan is appropriately narrower than a complete immutable run bundle. It
adds one payload-free commitment, pre-mutation validation, compatibility
behavior, and tests. It does not persist raw specs, attest handlers, implement
general replay, add provider writes, change CLI/schema behavior, or broaden
runtime topology.

## 4. Commitment Coverage Assessment

The proposed v1 commitment covers the behavior that can execute after resume:

- workflow identity and content hash;
- ordered workflow steps;
- every resolved step skill identity/version/content hash;
- every referenced policy identity/content hash;
- required checkpoint step IDs;
- explicit hook-input presence;
- SideEffect event/lifecycle input presence through counts;
- derived report-artifact policy posture.

Binding every step is necessary. After an approved step succeeds, later steps
can continue from the same reconstructed plan without another pause. Binding
only the approved step would leave later skill/policy changes exposed.

Unreferenced spec changes are correctly excluded to avoid unrelated approval
invalidation.

## 5. Transient Request Context Assessment

The first fix does not attempt to persist transient hook or SideEffect inputs.
Instead, their presence changes the original commitment while resume currently
reconstructs defaults, producing a mismatch before approval grant. This is a
safe and honest interim boundary: unsupported resume fails closed rather than
silently weakening governance.

Future bundle/resume work may carry or require re-supply of those validated
inputs through a separately reviewed contract.

## 6. Mutation Ordering Assessment

The required sequence is correct:

1. validate presentation/high-assurance controls;
2. build a candidate plan without event append;
3. compare run identity and commitment;
4. validate approved step and skill identity;
5. only then append grant, policy, and resume events;
6. attach the advanced builder to the validated plan and execute.

This prevents a mismatch from leaving an approval granted or a run resumed.

## 7. Denial And Legacy Assessment

Denial should remain independent of current project files because it cannot
authorize execution. The plan correctly preserves denial for changed, missing,
or legacy contexts.

Using an optional field for backward-readable state while rejecting `None` on
grant is the right preview compatibility tradeoff. Historical completed runs
remain readable; pending legacy runs cannot gain fabricated integrity.

## 8. Privacy And Error Assessment

The commitment stores only a SHA-256 digest. The planned fixed error codes and
messages exclude IDs, paths, hashes, payloads, approval reasons, and credentials.
Raw authored or runtime inputs are not copied into approval events.

## 9. Test Plan Assessment

The test plan covers workflow, skill, policy, step, transient context, legacy,
denial, no-mutation, non-leakage, determinism, duplicate policy references,
serde, all approval API families, and full regressions.

Implementation review should insist on direct event-count and approval-decision
assertions for every mismatch class, not merely returned error codes.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Consider a dedicated commitment type after v1 behavior is proven.
- Add handler/check implementation attestation only through later bundle work.
- Reuse the v1 commitment as the future bundle integrity root if the bundle
  review confirms the same canonical input set.

## 12. Recommended Next Phase

Implement the P0 approval-resume resolved-context commitment and pre-mutation
validation exactly as planned. Run full validation and focused security review
before returning to immutable run-bundle planning.

Do not add provider writes, raw bundle persistence, CLI/schema behavior, hosted
runtime, RBAC, or release changes.

## 13. Governed Review Evidence

- Workflow: `dg/review`.
- Run: `run-1783871545756308000-2`.
- Approval:
  `approval/run-1783871545756308000-2/review-scope-approved`.
- Presentation: `presentation/562e3f831f27772a`.
- Approval outcome: granted through proof-enforced presentation.
- Final status: completed.
- Event summary: 39 events, one approval request, one proof-marked approval
  grant, eight policy decisions, six scheduled steps, six successful mock skill
  invocations, no retries, and no escalations.
- Validation: `npm run check:docs` and `git diff --check` passed.
- Out-of-kernel work: Codex inspected current executor, runtime, loader, state,
  and plan behavior and authored this review. The kernel governed scope and
  approval but did not perform the review or edit files.
- Report posture: this document is the review record. No WorkReport or artifact
  was generated or persisted.
