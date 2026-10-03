# Private Trusted-Host Scheduling Boundary Planning Report

## 1. Executive Summary

The next P0 trusted-host boundary is now planned as a private, local,
schedule-once composition for one exact durable `TimeWindow` wait. Core derives
an inert scheduling ticket from authoritative state; an injected host waits
without polling; and one wake requests one fresh source-specific reinvocation.
The callback is never authority and does not prove deadline satisfaction.

The plan authorizes no implementation. It explicitly requires two accepted
reinvocation proofs before scheduling code may begin.

## 2. Scope Completed

- Defined the narrow schedule-once boundary.
- Separated authoritative wait state, inert host scheduling posture, trusted
  time verification, execution authority, and dispatch admission.
- Defined an injected deadline waiter that cannot inspect or mutate Core
  state.
- Defined early, late, duplicate, cancellation, restart, blocked, terminal,
  corrupt, and security-rejected behavior.
- Defined the test and review sequence that gates implementation.
- Updated the roadmap to identify the planned boundary and its blockers.

## 3. Scope Explicitly Not Completed

- No Rust scheduler types or helper.
- No timer, queue, daemon, worker pool, polling loop, or wake bus.
- No automatic model or conversational turn.
- No automatic approval or evidence bypass.
- No provider or sandbox execution, OpenShell, or nested harnesses.
- No public API, runtime configuration, CLI, SDK, schema, UI, or example.
- No hosted or distributed runtime, provider write, or release change.

## 4. Boundary Summary

The planned host owns waiting only. Core owns:

- the exact durable wait and deadline binding;
- coherent handoff and scheduling-ticket derivation;
- trusted-time verification after wake;
- current continuation classification;
- current authority and required-context checks;
- one-use directive consumption;
- atomic dispatch reservation; and
- durable terminal state.

A timer callback merely asks Core to verify the exact source-specific wait.

## 5. Pre-Implementation Blockers

Before scheduler implementation, the accepted explicit reinvocation helper
must prove:

1. full-composition concurrent callers admit at most one executor entry; and
2. a crash after committed transition but before operational entry recovers
   after backend reopen without duplicate executor entry or reconstructed
   authority.

Both proofs require focused maintainer/security review.

## 6. Restart And Liveness Posture

The plan avoids storing private authority in a scheduler. A restarted host
reopens authoritative SQLite state and asks Core for a fresh inert scheduling
observation. Genuine waits remain waits, blocks remain blocks, and terminal
state remains terminal. Ending an assistant turn or host callback is not
workflow completion.

This does not claim Workflow OS can force Codex, ChatGPT, or another product
to create a new model turn. The first boundary governs only a local injected
executor process.

## 7. Privacy And Security Summary

The planned ticket contains only bounded scheduling posture and payload-free
commitments. It carries no executor authority, prompts, commands, source
contents, approval reasons, evidence bodies, check output, provider payloads,
credentials, or raw runtime state. Core rechecks every authoritative fact
after wake.

## 8. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

No Rust validation is required for this documentation-only phase unless later
repository tooling identifies a broader requirement.

## 9. Remaining Limitations

- Only one exact `TimeWindow` source is planned.
- The first boundary is private, local SQLite, single-host, and schedule-once.
- Clock-domain representation remains an open design question.
- Repeated scheduling and other wake-source families remain separate future
  phases.
- No public product path consumes this boundary yet.

## 10. Recommended Next Phase

Perform focused maintainer/security review of the scheduling-boundary plan.
If accepted, implement the two prerequisite reinvocation proofs before any
scheduling code. Do not broaden provider mutations or nested harness work
first.

## 11. Governed Planning Record

- workflow: `dg/d`
- run: `run-1791069937684982000-2`
- approval: `approval/run-1791069937684982000-2/planning-approved`
- presentation: `presentation/6229ae24b0542a70`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: planning and documentation only
- phase status: `Completed`
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, zero retries, and zero escalations; approval-presentation proof
  enforced with one matching record and event marker
- validation summary: documentation and diff checks passed
- out-of-kernel work: architecture inspection, plan authoring, documentation
  validation, and later git and pull-request actions
