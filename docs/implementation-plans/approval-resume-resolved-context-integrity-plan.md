# Approval Resume Resolved-Context Integrity Plan

Status: P0 blocker fix implemented and accepted with non-blocking follow-ups.

Related foundations:

- [Execution Semantics](../runtime/execution-semantics.md)
- [Run Rehydration](../runtime/run-rehydration.md)
- [Approval Gate Presentation Default Enforcement Plan](approval-gate-presentation-default-enforcement-plan.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)

## 1. Executive Summary

Current local approval grant paths rehydrate the waiting run, accept the
approval decision, append `ApprovalGranted`, record resume policy, append
`RunResumed`, and then rebuild an execution plan from current project files.
The rebuilt plan locates the approved step by ID but does not first prove that
the current resolved workflow, skills, policies, and request-side execution
posture match what was awaiting approval.

This creates a time-of-check/time-of-use boundary: changed work can be executed
under an approval created for earlier work, and non-default request inputs can be
silently replaced by resume defaults.

The P0 fix should persist a versioned, payload-free resolved execution-context
commitment in each new `ApprovalRequest`. Approval grant paths must reconstruct
the current candidate context and compare it before appending any approval,
policy, resume, skill, hook, or SideEffect event. Missing or mismatched
commitments fail closed without mutating durable run state. Denial remains
available because it cannot authorize changed work.

This is an integrity commitment, not a self-contained immutable run bundle.

## 2. Goals

- Bind approval to the exact resolved declarative execution context that paused.
- Validate the binding before any grant-side durable mutation.
- Cover workflow, skill, policy, and resume-sensitive request posture.
- Reject same-ID/same-version content changes.
- Reject silently dropped hook, checkpoint, and SideEffect request posture.
- Preserve all current valid unchanged approval/resume behavior.
- Keep denial available after context drift.
- Fail closed for legacy pending approvals without a commitment.
- Use stable, non-leaking errors that do not disclose paths, IDs, or hashes.
- Prepare the later immutable run-bundle model without claiming replay-grade
  bundle persistence now.

## 3. Non-Goals

This phase must not add:

- raw workflow, skill, policy, source, or configuration payload persistence;
- a durable self-contained run bundle or general replay engine;
- handler binary digesting or execution attestation;
- provider calls, provider writes, retries, recovery, or mutation expansion;
- automatic approval, model self-approval, or approval-default changes;
- CLI commands, public schemas, examples, or migration tooling;
- hosted/distributed runtime, RBAC, IdP, quorum, or remote policy sync;
- arbitrary source inspection, command execution, or secret capture;
- reasoning lineage, recursive agents, agent swarms, or release changes.

## 4. Confirmed Current-Main Failure Boundary

The current sequence is:

1. Rehydrate the waiting run and event-backed approval.
2. Append `ApprovalGranted`.
3. Record resume policy.
4. Append `RunResumed`.
5. Reload and validate current project files.
6. Rebuild all execution steps from current workflow, skills, and policies.
7. Find the approved step by `step_id`.
8. Mark approval granted and execute.

Only event identity preserves workflow ID, workflow version, schema version, and
the original workflow hash. The resume path does not compare the rebuilt
workflow hash before mutation. Skill and policy content hashes are not part of
run identity. Request-side hook/checkpoint and SideEffect inputs are rebuilt as
defaults.

The external report referenced an older pinned commit. Direct inspection
confirms the same ordering and missing comparison on current `main` after PR
322. This plan therefore treats the issue as current P0 correctness work, not a
stale evaluator note.

## 5. Commitment Model

Add one optional field to `ApprovalRequest` for backward-readable event state:

```text
resolved_execution_context_hash: Option<SpecContentHash>
```

New approval requests must always populate it. `Option` exists only so older
stored events can deserialize. Granting a legacy request with `None` must fail
closed with a stable missing-commitment error before mutation. Denial may still
proceed.

The commitment algorithm must be explicitly domain-separated and versioned,
for example:

```text
workflow-os/resolved-execution-context/v1
```

The result is a SHA-256 `SpecContentHash`. The runtime stores no raw content in
the approval request.

## 6. Required Commitment Inputs

The v1 commitment must include deterministic labels and lengths for:

- workflow ID, workflow version, schema version, and canonical workflow content
  hash;
- each workflow step in declared order;
- each step ID;
- each resolved skill ID, skill version, and canonical skill content hash;
- each policy referenced by any executable step, deduplicated and sorted by
  policy ID, with canonical policy content hash;
- required before-skill checkpoint step IDs, sorted deterministically;
- whether an explicit before-skill hook input was supplied;
- supplied SideEffect event input count;
- supplied SideEffect lifecycle event input count;
- derived report-artifact high-assurance disclosure posture;
- derived approval proof-marker policy posture.

The last request-side fields are commitments, not persisted execution inputs.
Because resume currently cannot reconstruct non-default transient inputs, their
presence causes the default resume candidate to mismatch and fail closed rather
than silently dropping governance context.

The commitment must not include paths, raw YAML, prompts, provider payloads,
command output, environment values, credentials, timestamps, generated
invocation IDs, or actor-supplied approval reasons.

## 7. Resolution Rules

- Skill resolution must reuse the current validated `resolve_skill` semantics.
- Policy selection must use the same step references used by
  `policy_effects_for_step`.
- Missing referenced skills or policies fail through existing deterministic
  project validation or a fixed executor validation error.
- Duplicate policy references must not change the commitment.
- Unrelated, unreferenced skill or policy changes must not invalidate a pending
  approval.
- Reordering executable steps must invalidate the commitment through the
  workflow hash and ordered step inputs.
- Same ID/version with changed canonical content must invalidate the commitment.

## 8. Pre-Mutation Grant Sequence

The grant path must become:

1. Rehydrate the waiting run and event-backed approval.
2. Validate presentation/high-assurance controls where the selected API
   requires them.
3. Build a candidate resume plan from current validated project state without
   appending events.
4. Require the approval commitment to be present.
5. Compare current workflow identity to immutable run identity.
6. Compare candidate resolved execution-context commitment to the approval
   commitment.
7. Locate and validate the approved step/skill identity.
8. Only after every check passes, append `ApprovalGranted`, resume policy, and
   `RunResumed`.
9. Attach the advanced event builder to the already validated plan and execute.

Any failure in steps 3 through 7 returns before durable state mutation. The run
remains `WaitingForApproval`, the approval remains undecided, and event count and
projection state remain unchanged.

## 9. Denial Semantics

Approval denial must not require current project files or a matching resolved
context commitment. Denial does not authorize execution and should remain
available when definitions were removed, changed, or cannot be loaded.

The existing denial event and terminal fail-closed behavior should remain
unchanged.

## 10. Legacy State And Compatibility

- Existing event JSON without the new field must still deserialize.
- Granting a legacy pending approval without the commitment fails with a stable
  `executor.approval.resume_context_missing` error before mutation.
- Denying a legacy pending approval remains allowed.
- Completed historical runs continue to rehydrate.
- No automatic migration or inferred commitment is allowed because current
  files cannot prove historical resolved context.
- New approval events always serialize the commitment.

This is an intentional fail-closed compatibility tradeoff for pending preview
runs.

## 11. Error Handling And Privacy

Use stable errors such as:

- `executor.approval.resume_context_missing`;
- `executor.approval.resume_context_mismatch`;
- `executor.approval.workflow_identity_mismatch`;
- existing load/validation errors mapped to bounded executor errors where they
  would otherwise expose paths or diagnostics inappropriate for approval.

Errors must not include workflow, run, step, skill, policy, approval, actor,
path, content-hash, payload, source, command-output, provider, credential, or
token values.

## 12. Relationship To Approval Presentation

Approval-presentation proof establishes what scope was shown to the approver.
The resolved execution-context commitment establishes which declarative and
request-side execution posture may resume.

Both are required for proof-enforced approval paths. Presentation proof must not
substitute for context integrity, and a matching context hash must not substitute
for presentation proof.

All approval APIs ultimately using `apply_approval_decision` should receive the
same pre-mutation context validation for granted decisions.

## 13. Relationship To Immutable Run Bundles

This fix commits to resolved inputs but does not preserve their contents or
references independently of the project files. It prevents changed current
inputs from resuming, but it cannot reconstruct deleted historical inputs or
attest handler binary identity.

The next hardening phase should define a durable immutable run-bundle manifest
containing validated content-addressed references for workflow, resolved skills,
resolved policies, governance/request posture, and future handler/check
attestations. The P0 commitment algorithm should be reusable as that bundle's
integrity root where practical.

## 14. Test Plan

Required focused tests:

- unchanged approval grant resumes and completes;
- workflow content change with same ID/version fails before mutation;
- skill content change with same ID/version fails before mutation;
- referenced policy content change fails before mutation;
- unrelated skill/policy change does not block resume;
- missing or removed current workflow/step/skill/policy fails before mutation;
- changed step ID cannot reuse the old approval;
- required checkpoint input is not silently dropped on resume;
- explicit hook input is not silently dropped on resume;
- SideEffect event/lifecycle inputs are not silently dropped on resume;
- legacy missing commitment fails grant before mutation;
- legacy missing commitment still permits denial;
- mismatch leaves status, event count, approval decision, and approval
  projection unchanged;
- all presentation/high-assurance approval grant APIs inherit the same check;
- errors do not leak IDs, paths, hashes, payloads, reasons, or secret markers;
- commitment is deterministic across identical loads;
- duplicate policy references do not change the commitment;
- valid state serde round trip includes the new commitment;
- historical completed runs without the field rehydrate;
- full existing executor, state, approval, report, SideEffect, provider, and
  runtime suites pass.

## 15. Proposed Implementation Sequence

1. Add commitment computation and focused deterministic unit tests.
2. Add the optional field to `ApprovalRequest` with backward-readable serde.
3. Populate the commitment for every new approval request.
4. Refactor resume preparation into a pre-mutation candidate-plan validation
   step.
5. Compare workflow identity and commitment before any grant-side append.
6. Preserve denial behavior.
7. Add TOCTOU, legacy, request-context, and non-leakage regressions.
8. Update runtime docs and create a blocker-fix report.
9. Run full validation and perform focused maintainer review.
10. Plan the durable immutable run-bundle manifest after acceptance.

## 16. Documentation Requirements

Implementation docs must state:

- the current TOCTOU gap is fixed;
- exactly which resolved inputs are committed;
- validation occurs before approval grant mutation;
- legacy grants fail closed while denials remain available;
- transient non-default resume inputs currently block rather than disappear;
- no raw spec bundle, handler attestation, general replay, provider write, CLI,
  schema, hosted, RBAC, or release behavior was added;
- immutable run bundles remain the next hardening phase.

## 17. Open Questions

- Should the commitment type remain `SpecContentHash` or gain a dedicated ID
  after the P0 fix proves its boundary?
- Which report-artifact policy enum labels should be part of v1 commitment
  encoding?
- Should a later resume request supply and prove transient hook/SideEffect inputs
  rather than requiring a durable bundle to carry them?
- How should future handler implementation digests or signed attestations enter
  the run bundle without coupling core to one deployment system?

These questions must not delay the fail-closed P0 fix.

## 18. Final Recommendation

Review this plan immediately, then implement the resolved-context commitment and
pre-mutation grant validation as a P0 blocker fix. Do not proceed to ordinary
immutable run-bundle planning or additional provider mutations until the fix is
accepted.

## 19. Governed Planning Evidence

- Dogfood workflow: `dg/d`.
- Run ID: `run-1783871247582345000-2`.
- Approval ID:
  `approval/run-1783871247582345000-2/planning-approved`.
- Presentation ID: `presentation/c4361eccbe68e363`.
- Approval outcome: granted through the proof-enforced approval path.
- Phase status: completed.
- Event summary: 39 events, one approval request, one proof-marked approval
  grant, eight policy decisions, six scheduled steps, six successful mock skill
  invocations, no retries, and no escalations.
- Validation: `npm run check:docs` and `git diff --check` passed.
- Out-of-kernel work: Codex inspected the current executor grant/resume ordering,
  runtime identity, loader hashes, request inputs, and external feedback and
  authored this plan. The kernel governed scope and approval but did not edit
  files or implement the fix.
- Report posture: this plan is the planning record. No runtime WorkReport or
  artifact was generated or persisted.
