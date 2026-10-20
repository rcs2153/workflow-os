# Trusted-Host Workflow Completion Reconciliation Planning Blocker Fix Report

## 1. Executive Summary

The completion plan's two blockers are corrected in documentation. The design
no longer commits a terminal continuity outcome before a later workflow
reconciliation that depends on an in-memory output. It also defines a complete,
Core-owned invocation lifecycle rather than appending an orphan terminal
invocation event.

No runtime behavior was implemented.

## 2. Blockers Fixed

### Lost successful result across crash

The prior plan placed workflow reconciliation after continuity outcome
persistence. A crash could therefore lose the only `SkillOutput` after the
attempt was durably marked successful.

The corrected plan makes the existing attempt-outcome transaction atomically
commit:

1. the continuity outcome projection;
2. the invocation terminal event; and
3. the workflow terminal event.

The exact output is validated while still in memory. The transaction either
commits all terminal truth or none of it.

### Unbound invocation lifecycle

The prior plan did not define invocation request/start ordering or bind the
invocation and attempt identities.

The corrected plan requires:

- opening plus `SkillInvocationRequested` in one transaction;
- dispatch admission plus `SkillInvocationStarted` in one transaction;
- a Core-generated invocation ID;
- a domain-separated skill-attempt ID derived from the exact continuity
  attempt; and
- exact replay validation of each complete associated event sequence.

## 3. Corrected Transaction Ownership

| Existing operation | Added closed-profile responsibility |
| --- | --- |
| operational opening | validate and append invocation request |
| dispatch reservation | derive attempt identity and append invocation start |
| attempt outcome | validate output and append invocation plus run terminal events |

The generic continuity-only behavior remains unchanged for other callers. No
generic callback or application-owned event surface is planned.

## 4. Restart And Ambiguity Posture

There is no longer a lawful committed-success state whose workflow result can
only be reconstructed from lost process memory. Ambiguous commit handling
reopens SQLite and accepts only the complete operation and event sequence.
Partial or conflicting state is corrupt and is never repaired event by event.

A committed dispatch with no terminal outcome remains admitted or ambiguous;
it is not silently rerun and does not fabricate completion.

## 5. Storage Posture

The correction does not promise that existing tables are sufficient. The
future implementation must first prove whether operation request JSON,
workflow event rows, and idempotency keys can bind exact replay. Any necessary
internal SQLite migration requires explicit implementation scope and review.
No public schema is authorized.

## 6. Privacy Posture

The corrected design preserves bounded output references, fixed failure
metadata, private redacted lifecycle context, and payload-free application
results. It does not persist raw output, commands, paths, environment values,
payloads, credentials, or secrets.

## 7. Test Coverage Added To The Plan

The revised test plan now requires:

- atomic request/opening, start/dispatch, and terminal/outcome sequences;
- crash injection at every transaction phase;
- restart after committed success with no in-memory output;
- complete invocation identity and ordering;
- rejection of orphan or partial lifecycle events;
- output-validation consistency with continuity state; and
- exact replay and conflicting replay for all three operations.

## 8. Scope Explicitly Not Added

No Rust implementation, runtime mutation, storage migration, scheduler,
automatic approval, broader profile, provider, OpenShell, nested harness,
public API, write, example, or release behavior was added.

## 9. Governed Blocker-Fix Record

- Workflow: `dg/blocker`
- Run: `run-1791505894357857000-2`
- Approval: `approval/run-1791505894357857000-2/fix-approved`
- Presentation proof: `presentation/b0eeb61d22c77369`
- Approval outcome: granted under delegated maintainer authority
- Phase status: completed
- Event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, and one run completion; zero retries and zero escalations
- Approval-presentation enforcement: proof enforced with matching persisted
  presentation and event marker

Repository inspection and documentation changes were performed by the
delegated maintainer outside the kernel. The kernel governed scope, approval,
sequencing, and durable phase evidence.

No runtime handler, report artifact, provider call, repository write, or other
kernel execution was simulated. Shell inspection, documentation edits,
validation, and later git/PR operations are out-of-kernel maintainer work.

## 10. Validation

Validation passed:

- `npm run check:docs`;
- `git diff --check`; and
- manual verification confirmed that both review blockers are corrected
  without runtime capability claims.

## 11. Recommended Next Phase

Perform a focused maintainer/security re-review of the corrected plan. Do not
begin Rust implementation until the re-review accepts transaction ownership,
invocation identity, replay, restart, and privacy posture.
