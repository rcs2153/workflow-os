# Bounded Trusted-Host Redispatch Loop Plan Blocker Fix Report

## 1. Executive Summary

The bounded trusted-host redispatch-loop plan review found two blockers: a
caller-selected host budget could return normally while Core still required
immediate continuation, and the iteration provider could resupply invocation
data already fixed by the accepted operation commitment.

The corrected plan removes both ambiguities. The authoritative execution
window attempt limit is now the loop's finite bound. The helper continues
while Core derives `ResumeNow` and returns normally only for `AwaitCondition`,
`Blocked`, or `Terminal`. The provider is narrowed to fresh bounded
non-authorizing identities; one exact `SkillInput` and executor binding remain
immutable across the loop.

No runtime code was implemented.

## 2. Blockers Fixed

### False Stall At Host Budget Exhaustion

The original plan proposed `ResumeRequiredBudgetExhausted` as a normal loop
outcome. That allowed the host call or agent turn to end while durable Core
state still said `ResumeNow`.

The corrected plan has no caller-selected iteration budget and no successful
resume-required stop. Each iteration consumes one authoritative attempt
allocation, and the durable maximum-attempt posture provides the finite bound.
An inconsistent `ResumeNow` with no lawful attempt allocation is a structured
integrity/liveness error, not completion, wait, block, approval, or success.

### Invocation Substitution Surface

The original plan allowed the iteration provider to return invocation input.
The existing operation commitment already binds every `SkillInput` identity
and value together with the executor binding.

The corrected plan carries one immutable invocation input and executor binding
from the accepted opening operation. The provider may return only fresh
bounded operation, receipt, attempt, and yield-generation identities required
by existing private persistence APIs. It cannot receive, construct, replace,
or return invocation data, authority, commitments, or capabilities.

## 3. Corrected Liveness Contract

- `ResumeNow` always continues inside the helper.
- `AwaitCondition` returns the typed wait posture.
- `Blocked` returns the exact bounded block posture.
- `Terminal` returns the exact window-terminal posture.
- ambiguity, corruption, stale state, security rejection, projection drift,
  or inconsistent attempt limits return structured errors.
- a successful executor callback does not complete the workflow by itself.
- a host or assistant response cannot treat a still-resumable loop as closed.

Future process preemption or scheduling remains deferred until it has a durable
typed continuation handoff that cannot be mistaken for workflow completion.

## 4. Corrected Provider Boundary

The renamed identity provider is not an authority or invocation provider. It
supplies fresh bounded identities only. Core remains responsible for:

- current disposition derivation;
- directive consumption;
- attempt allocation;
- capability construction;
- invocation and executor commitment validation;
- dispatch reservation; and
- outcome, yield, and recovery persistence.

## 5. Test Plan Changes

The future test plan now requires proof that:

- the loop never returns normally while Core says `ResumeNow`;
- authoritative attempt limits bound executor entries;
- inconsistent liveness and attempt posture fails closed;
- the identity provider cannot supply or replace `SkillInput` or executor
  binding; and
- no completion, wait, block, or approval is fabricated at the finite bound.

Existing concurrency, replay, ambiguity, restart, event-ordering, privacy, and
regression coverage remains required.

## 6. Scope Explicitly Not Completed

- No loop implementation.
- No scheduler, queue, worker, process preemption, or model-turn creation.
- No automatic approval, evidence, check, or policy synthesis.
- No provider execution or mutation.
- No OpenShell or nested harness execution.
- No public configuration, CLI, SDK, workflow schema, hosted, or release
  behavior.
- No filesystem or PostgreSQL parity.

## 7. Validation

Validation passed:

- `npm run check:docs`
- `git diff --check`
- source comparison against current supervisor invocation commitments,
  directive-consumption compare-and-set behavior, and continuation posture

## 8. Remaining Limitations

- The implementation must determine the smallest private read needed to
  cross-check authoritative remaining-attempt posture.
- The exact non-authorizing identity-provider method shape remains an
  implementation detail subject to review.
- Cross-run fairness and trusted-host process preemption remain future work.
- External hosts still cannot create model turns or provider executions.

## 9. Recommended Next Phase

Perform a focused maintainer/security review of the corrected plan. If the
review accepts both fixes, implement only the private local SQLite loop and
review it before any provider, sandbox, nested-harness, public-config, CLI, or
hosted integration.

## 10. Governed Blocker-Fix Record

- workflow: `dg/blocker`
- run: `run-1790952706739919000-2`
- approval: `approval/run-1790952706739919000-2/fix-approved`
- presentation: `presentation/000265fa268038fd`
- presentation hash:
  `000265fa268038fd752e9ec815c95be0a41d1ec2e3f385f5844e417b80a56aa9`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval-presentation proof enforced
- validation summary: docs and diff checks passed; corrected claims were
  compared with current private Core contracts
- out-of-kernel work: plan correction, source comparison, documentation
  validation, and git actions are performed by the delegated maintainer;
  Workflow OS governs scope and approval but does not inspect code, edit files,
  or run shell commands
