# Bounded Trusted-Host Redispatch Loop Planning Report

## 1. Executive Summary

Workflow OS now has accepted execution-window, resume-directive, one-shot
trusted-host supervisor, and atomic dispatch-reservation boundaries. This
phase planned the smallest private composition layer that may repeatedly
redispatch lawful local work without turning an executor turn boundary into a
workflow stop.

The plan keeps Core authoritative. Only a freshly derived `ResumeNow`
disposition may lead to a fresh directive consumption, atomic reservation, and
one injected executor call. Typed wait, blocked, terminal, ambiguous, corrupt,
or security-rejected posture stops the loop without fabricated progress.

No runtime code was implemented in this phase.

## 2. Scope Completed

- Defined the current composition gap after atomic dispatch reservation.
- Defined a private, finite, local, injected redispatch-loop boundary.
- Defined source-of-truth boundaries and required invariants.
- Defined candidate private input, provider, budget, result, and stop models.
- Defined the closed iteration algorithm.
- Defined immediate redispatch, typed wait, blocked, terminal, and budget
  behavior.
- Defined concurrency, replay, ambiguity, restart, privacy, and error posture.
- Defined the first implementation boundary and focused future test plan.
- Updated the roadmap to point to the plan and its required security review.

## 3. Scope Explicitly Not Completed

- No redispatch loop implementation.
- No scheduler, queue, daemon, worker, polling, or hosted service.
- No model-turn or conversation creation.
- No automatic approval or evidence/check synthesis.
- No provider execution or mutation.
- No OpenShell integration.
- No nested harness execution.
- No public runtime configuration, CLI, SDK, workflow schema, or example.
- No filesystem or PostgreSQL implementation.
- No multi-host lease, heartbeat, failover, or reservation stealing.
- No SideEffect, report-artifact, reasoning-lineage, or release change.

## 4. Planning Decisions

The plan makes five central decisions:

1. `AuthoritativeContinuationDisposition` remains the workflow-liveness source
   of truth; the loop adds no new workflow disposition.
2. Every resumed iteration must consume fresh directive authority and commit a
   new one-winner dispatch reservation.
3. The loop is finite, but budget exhaustion is explicit host posture rather
   than workflow completion, failure, approval wait, or typed wait.
4. The injected iteration-input provider may supply bounded input material but
   cannot provide authority or bypass Core commitment validation.
5. Any ambiguity, integrity drift, or security rejection stops redispatch; the
   loop never guesses that retry is safe.

## 5. Runtime Boundary Summary

The first implementation is planned as one crate-private Workflow Core helper
over local SQLite state. It accepts one initial opened or resumed capability,
an injected executor, an injected deterministic iteration-input provider, and
a positive executor-entry budget.

It composes the existing one-shot supervisor. It does not open a window,
create model turns, call providers, or expose a public configuration surface.

## 6. Workflow Semantics Summary

- `ResumeNow`: obtain fresh one-time authority and continue if budget remains.
- `AwaitCondition`: return the durable typed-wait posture immediately.
- `Blocked`: return immediately without hidden retry or approval translation.
- `Terminal`: return immediately; inspect run state for workflow-level status.
- budget exhausted while `ResumeNow`: return explicit resume-required host
  posture without changing workflow state.

Successful executor output does not itself complete a workflow run.

## 7. Security And Privacy Summary

The plan preserves owned non-serializable capabilities, atomic one-winner
admission, exact cursor/revision/commitment checks, fresh trusted time, exact
ambiguity reconciliation, and payload-free events. It prohibits raw source,
prompts, transcripts, command output, provider payloads, environment values,
credentials, tokens, and authorization material.

## 8. Test Plan Summary

Future tests cover multi-iteration success, each stop disposition, explicit
budget exhaustion, zero-budget rejection, competing loops, directive and
reservation replay, substitution, ambiguous commits, executor result classes,
restart behavior, event ordering, no false completion, and non-leaking Debug
and errors.

## 9. Validation

Planning validation passed:

- `npm run check:docs`
- `git diff --check`
- manual review for scope and capability overclaiming

No Rust build or runtime test was required because this phase changed only
planning documentation. The governed phase closed successfully with 39 events,
one approval, no retries, no escalations, and approval-presentation proof
enforced.

## 10. Remaining Limitations

- The plan does not yet resolve whether directive derivation and consumption
  need a new atomic private helper.
- The iteration-input provider requires security review to ensure it cannot
  substitute authority or invocation scope.
- Budget-exhaustion result semantics require focused review.
- The first implementation will remain SQLite-only and process-local.
- No external host can create or resume an agent turn through this boundary.

## 11. Recommended Next Phase

Perform a focused maintainer/security review of the bounded trusted-host
redispatch-loop plan. If accepted, implement only the private local SQLite
slice and require another focused review before provider, sandbox, nested
harness, or public runtime integration.

## 12. Governed Planning Record

- workflow: `dg/d`
- run: `run-1790952180681104000-2`
- approval: `approval/run-1790952180681104000-2/planning-approved`
- presentation: `presentation/73ee9514b91fcc99`
- presentation hash:
  `73ee9514b91fcc9971f1d1a7a7319a8b8deeb35eee6c35b87adae7053fcf5501`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval-presentation proof enforced
- validation summary: `npm run check:docs`, `git diff --check`, and manual
  scope/overclaim review passed; no code validation was required for this
  planning-only change
- out-of-kernel work: source inspection, plan authoring, validation, and git
  actions were performed by the delegated maintainer; the kernel governed
  scope and approval but did not inspect code, edit files, or run shell
  commands
