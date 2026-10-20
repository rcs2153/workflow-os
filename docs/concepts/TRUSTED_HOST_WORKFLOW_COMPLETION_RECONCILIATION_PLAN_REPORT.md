# Trusted-Host Workflow Completion Reconciliation Planning Report

## 1. Executive Summary

Planning now defines the missing ownership boundary between an authoritative
trusted-host attempt becoming terminal and the workflow run becoming terminal.
The corrected design keeps lifecycle composition inside Core and makes the
existing opening, dispatch, and attempt-outcome transactions own the matching
workflow invocation events. The terminal attempt-outcome transaction owns
workflow terminalization before the prepared session reduces the result to
bounded application posture.

No runtime behavior was implemented.

## 2. Scope Completed

- Identified the existing operational-terminal versus workflow-terminal gap.
- Selected Core, not the foreground application, as completion owner.
- Defined exact authoritative inputs and forbidden substitutes.
- Defined supported success and terminal-failure mappings for the existing
  one-step docs-check profile.
- Required an atomic invocation-result plus workflow-terminal event sequence.
- Defined exact replay, conflicting replay, ambiguity, and crash posture.
- Defined privacy, validation, and focused test requirements.
- Sequenced focused plan review before implementation.

## 3. Scope Explicitly Not Completed

- No Rust implementation.
- No workflow status or event mutation.
- No scheduler, discovery, automatic approval, or broader profile.
- No provider, OpenShell, nested harness, hosted, SideEffect, or write behavior.
- No public CLI, SDK, workflow schema, example, or release change.
- No WorkReport or report-artifact generation.

## 4. Architecture Decision

The foreground host remains an executor and bounded posture presenter. It does
not become a workflow lifecycle authority.

The future Core path will carry one private lifecycle context through opening,
dispatch, and outcome persistence. Opening atomically commits invocation
request, dispatch atomically commits invocation start, and terminal outcome
atomically commits continuity result, invocation result, and workflow result
while the exact `SkillOutput` is still available.

## 5. Terminal Mapping Summary

- Authoritative `Succeeded` plus one valid in-memory output may commit the
  continuity result, `SkillInvocationSucceeded`, and `RunCompleted` atomically.
- Authoritative `TerminalFailure` with no output may append
  `SkillInvocationFailed` and `RunFailed` atomically using fixed non-leaking
  failure metadata.
- Retryable, yielded, ambiguous, stale, mismatched, or incomplete posture does
  not terminalize the workflow.

## 6. Atomicity And Recovery Summary

The corrected plan requires each lifecycle event to share the authoritative
transaction that establishes its fact: invocation request with opening,
invocation start with dispatch, and invocation/run terminal events with the
attempt outcome. Exact replay validates the complete associated event sequence.
Ambiguous commit is reconciled on a fresh connection. Partial state is corrupt
and never repaired piecemeal.

## 7. Privacy Summary

The plan stores only existing bounded output references and fixed failure
metadata. It prohibits raw output, commands, paths, environment values,
payloads, credentials, and secrets. The application continues to receive only
fixed posture strings.

## 8. Governed Planning Record

- Workflow: `dg/d`
- Run: `run-1791503675327866000-2`
- Approval: `approval/run-1791503675327866000-2/planning-approved`
- Presentation proof: `presentation/837e76a4b1b74f70`
- Approval outcome: granted under delegated maintainer authority
- Phase status: completed
- Event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, and one run completion; zero retries and zero escalations
- Approval-presentation enforcement: proof enforced with matching persisted
  presentation and event marker

Repository inspection and documentation authoring were performed by the
delegated maintainer outside the kernel. The kernel governed scope, approval,
sequencing, and durable phase evidence.

No runtime handler, report artifact, provider call, repository write, or other
kernel execution was simulated. Shell inspection, documentation edits,
validation commands, and later git/PR operations are out-of-kernel maintainer
work and are disclosed as such.

## 9. Validation

Phase validation passed:

- `npm run check:docs`;
- `git diff --check`; and
- manual diff review confirmed that the roadmap describes planned, not
  implemented, workflow completion; keeps the foreground host
  non-authoritative; and preserves scheduler, provider, OpenShell, nested
  harness, public-surface, schema, write, and release non-goals.

## 10. Remaining Limitations

- The corrected plan requires focused maintainer/security re-review.
- Existing storage sufficiency for atomic exact replay remains a review
  question.
- Required-output validation failure mapping remains a review question.
- No runtime completion owner exists until a separately approved
  implementation lands.

## 11. Recommended Next Phase

Perform a focused maintainer and security re-review of the corrected
trusted-host workflow completion reconciliation plan. Do not begin
implementation or broaden runtime surfaces before that review accepts the
boundary.
