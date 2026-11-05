# Project Resource Authorization Continuity Independent Rerun

## Verdict

Gate 3 passes.

An independent evaluator reproduced the project-resource authorization defect
on the exact parent commit and proved fail-closed behavior on the exact merged
fix. A semantic no-drift control also proved that canonical resource ordering
does not create a false rejection.

This closes Gate 3 for the existing GitHub repository-metadata fixture and
opt-in live-read composition boundary. It does not claim provider writes, live
credential-backed acceptance evidence, or equivalent manifest enforcement for
other provider families.

## Revision And Isolation Boundary

- parent: `a84cbaaba663f09075a810fb71c7963562779663`;
- fix and exact merged `main`:
  `442b4a4435d65d66b974c11d17047e15327af8f5`;
- merged pull request: #744;
- exact-head Actions run: `42045120157`, nine of nine checks successful;
- disposable evaluation root:
  `/private/tmp/workflow-os-gate3-rerun.aovY09`; and
- separate parent/fix checkouts, copied projects, and isolated state stores.

The primary repository remained untouched during the independent experiment.

## Parent Negative Control

The evaluator started the GitHub repository-metadata fixture on the parent,
changed the current project declaration from `acme/widgets` to
`acme/replacement`, then approved and resumed the original run.

Run: `run-1793889832093550000-1`.

Observed result:

- approval exited successfully;
- the decision was `granted` and the run became `Running`;
- resume exited successfully and the run became `Completed`;
- `ApprovalGranted` and `SkillInvocationRequested` were persisted;
- provider dispatch, invocation start, invocation success, and `RunCompleted`
  were persisted;
- one adapter audit record was emitted; and
- one adapter observability/telemetry record was emitted.

The parent therefore executed the original immutable `acme/widgets` operation
after the current project no longer authorized that repository.

## Fix Proof

The evaluator repeated the same start, declaration replacement, and approval
attempt against exact merged `main`.

Run: `run-1793889857594726000-1`.

Approval failed with exit code `3` and:

```text
error[executor.approval.resume_context_mismatch]:
approval resume context does not match the context originally approved
```

Post-failure inspection proved:

- status remained `WaitingForApproval`;
- only the six pre-approval events existed;
- `ApprovalGranted` was absent;
- `SkillInvocationRequested` was absent;
- provider dispatch and admission were absent;
- adapter audit records were `0`;
- adapter observability records were `0`; and
- telemetry was absent.

The error did not disclose the original target, replacement target, or provider
payload.

## Canonical-Order Control

The fix control began with two allowed resources ordered `second, widgets`.
Before approval, the declaration was reordered to `widgets, second` without a
semantic change.

Approval and resume both exited successfully, the run completed, and exactly
one adapter audit plus one adapter observability record were emitted. The
canonical commitment therefore rejects changed authority without treating
declaration order as authority drift.

## Independent Governed Record

- workflow: `dg/review`;
- run: `run-1793889771262903000-2`;
- approval: `approval/run-1793889771262903000-2/review-scope-approved`;
- presentation: `presentation/92b0b4f8553d433d`;
- approval outcome: granted;
- phase status: `Completed`;
- events: 39;
- retries: 0; and
- escalations: 0.

The evaluator performed commands and fixture mutations outside the kernel in
isolated copies. The kernel coordinated the independent review phase and did
not execute or fabricate the evidence.

## Closure Reconciliation Record

- workflow: `dg/implement`;
- run: `run-1793889975673969000-2`;
- approval: `approval/run-1793889975673969000-2/implementation-approved`;
- presentation: `presentation/c80eeb24713a994f`;
- presentation content hash:
  `c80eeb24713a994f663a370f926481f8bd8752025a789a7cd65f75d9f7b11cee`;
- outcome: granted under delegated-maintainer authority;
- phase status: `Completed`;
- events: 39;
- retries: 0; and
- escalations: 0.

The maintainer reconciled documentation and ran documentation, release-state,
and diff-integrity validation outside the kernel. No runtime code changed.

## Limitations

- The independent experiment used fixture transport; no live GitHub request
  occurred.
- It did not rerun the full workspace suite because exact-head CI had already
  completed all nine required jobs successfully.
- It does not provide independent assurance for undeclared provider families,
  writes, hosted enforcement, or storage-owner tamper resistance.
