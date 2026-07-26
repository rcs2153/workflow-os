# Authoritative Proportional-Governance Executor Routing Plan Review

## 1. Executive Verdict

**Plan accepted with non-blocking implementation constraints. Proceed to the
visible-disclosure delivery prerequisite.**

The plan defines the correct next runtime-composition boundary after the
accepted quiet-success consumer. It preserves execution and disclosure as
independent axes, refuses to treat audit recording as proof of presentation,
reuses existing approval authority rather than inventing auto-approval, and
keeps optional execution providers out of scope.

The first implementation must remain narrower than the full routing matrix.

## 2. Scope Verification

The plan stayed within planning-only scope.

It does not authorize:

- Rust or runtime behavior;
- CLI, UI, notification, or hosted presentation;
- workflow or policy schema changes;
- providers or OpenShell;
- SideEffect execution or writes;
- automatic or model-selected approval;
- additional check families;
- enterprise administration;
- reasoning lineage; or
- release changes.

The roadmap and quiet-success updates correct stale status without claiming
that visible, approval-required, or denial routing already exists.

## 3. Routing Model Assessment

The four-cell routing matrix is correct:

```text
Proceed + Quiet
Proceed + Visible
RequireApproval + Visible
Denied + Visible
```

The plan correctly excludes incomplete assessments and invalid quiet
approval/denial combinations from permissive routing.

It also retains the exact persisted `GovernanceAssessmentBinding` as the
posture source of truth. A caller enum, display preference, or detached
projection cannot select a route.

## 4. Quiet Route Assessment

The accepted fresh-run quiet consumer remains the compatibility baseline. The
plan preserves:

- immutable run-bundle validation;
- canonical `DocsCheck` execution;
- same-call authoritative fact binding;
- aggregate completeness and monotonicity;
- create-only run ownership;
- source-bound governance persistence;
- event and audit ordering; and
- existing errors and privacy behavior.

No regression or widening is authorized.

## 5. Visible Disclosure Assessment

The plan correctly treats `Visible` as an obligation independent from
execution. It does not convert visible disclosure into approval.

The proposed explicit injected surface is appropriate for a first local proof,
but its receipt must use precise semantics:

- it may prove that a configured delivery surface accepted a bounded
  disclosure;
- it must not claim that a human observed, understood, or acknowledged it;
- an in-memory callback is not independently trustworthy merely because it
  returned success;
- the caller must choose the surface explicitly;
- the exact surface kind and receipt status must remain bounded and
  inspectable; and
- no receipt may be reconstructed from a serialized callback result alone and
  treated as execution authority.

This is an implementation constraint, not a planning blocker, because the plan
already distinguishes surface acceptance from later acknowledgement and
requires a model review before executor integration.

## 6. Approval Route Assessment

The plan correctly requires reuse of existing:

- `ApprovalRequest`;
- `ApprovalDecision`;
- approval-presentation records and proof markers;
- resolved execution-context commitments;
- durable run state; and
- executor pause/resume transitions.

Current `ApprovalRequest` values are step-and-skill scoped. An aggregate
pre-execution proportional-governance gate therefore cannot be attached to an
arbitrary first step or synthetic skill without misrepresenting what was
approved.

The plan explicitly requires a narrow model prerequisite if the existing
contract cannot represent that gate truthfully. That decision must be resolved
before approval-route implementation. It does not block the visible
`Proceed` prerequisite.

## 7. Denial Route Assessment

The plan correctly requires denial before skill execution and distinguishes
governance denial from incomplete assessment, failed checks, approval denial,
delivery failure, and ordinary execution failure.

The preference to reuse `GovernanceAssessmentBound` plus existing terminal
events is sound. Implementation must prove that `RunCreated`,
`GovernanceAssessmentBound`, and `RunFailed` can represent denial without ever
emitting `RunStarted` or skill events. If that lifecycle would be misleading,
the event-model prerequisite must be reviewed separately.

## 8. Ordering And Recovery Assessment

The common ordering preserves the accepted create-only immutable claim and
same-call check authority before routing.

The implementation plan appropriately calls out bounded residue. The first
visible slice must choose and test exact commit markers for:

- immutable run ownership;
- governance binding;
- run creation;
- disclosure surface acceptance; and
- run start.

No best-effort cleanup may erase evidence of a failed or competing claim.

## 9. Privacy And Error Assessment

The plan preserves the repository's privacy posture:

- no raw source/spec contents;
- no raw check or command output;
- no paths or environment values;
- no raw policy or approval prose;
- no provider payloads;
- no credentials or tokens;
- bounded identifiers and reason codes only; and
- safe unknown-wire-value handling.

The proposed stable error family is suitable for implementation refinement.

## 10. Test Plan Assessment

The planned tests cover:

- all valid routing cells;
- invalid and incomplete states;
- quiet-path compatibility;
- disclosure ordering and missing-surface failure;
- receipt identity mismatch;
- approval pause, proof, grant, denial, and stale context;
- denial before skill execution;
- fresh-run concurrency;
- event/snapshot determinism;
- audit distinctions; and
- non-leakage.

Implementation should add one explicit regression proving that a sink which
accepts a disclosure does not project human acknowledgement or approval.

## 11. Planning Blockers

None.

The aggregate approval model question is a blocker for the later approval
implementation, not for the first visible-disclosure prerequisite.

## 12. Non-Blocking Follow-Ups

- Name the first receipt status as surface acceptance, not human presentation
  or acknowledgement.
- Decide whether a trusted injected surface needs a source commitment analogous
  to authoritative local-check source binding.
- Prove denial lifecycle truthfulness before reusing existing terminal events.
- Keep OpenShell as a later optional execution-provider candidate that consumes
  accepted routing and authority.

## 13. Governed Review Record

- workflow: `dg/review`
- run: `run-1785032517831777000-2`
- approval:
  `approval/run-1785032517831777000-2/review-scope-approved`
- presentation: `presentation/dba1c45869acb920`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- validation summary: `npm run check:docs` and `git diff --check` passed
- out-of-kernel work: source inspection, contract analysis, review authoring,
  documentation validation, and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, execute checks, create a WorkReport artifact, or perform git actions

## 14. Recommended Next Phase

Implement the smallest payload-free visible-disclosure delivery contract and
receipt model only.

Do not integrate the executor in that prerequisite phase. Do not add approval
routing, denial routing, CLI/UI behavior, providers, OpenShell, writes, schemas,
hosted behavior, enterprise administration, or release changes.
