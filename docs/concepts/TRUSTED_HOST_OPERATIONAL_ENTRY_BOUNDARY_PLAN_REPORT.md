# Trusted-Host Operational Entry Boundary Planning Report

## 1. Executive Summary

This phase planned the smallest private local boundary that may enter the
accepted bounded trusted-host redispatch loop from a fresh process. The plan
keeps Workflow Core authoritative for opening, continuation, wait, block, and
terminal posture. It never reconstructs authority from durable evidence and
never infers sensitive invocation payloads from persisted commitments.

No runtime code was implemented.

## 2. Scope Completed

- Defined the fresh-process and restart composition gap.
- Defined source-of-truth boundaries for run state, authority, invocation,
  executor identity, wait posture, and completion.
- Defined a candidate private operational entry API.
- Defined initial-opening, resume, wait, blocked, and terminal algorithms.
- Defined exact replay, ambiguity, committed-capability-loss, and restart
  posture.
- Defined bounded typed wait and blocked handoff requirements.
- Defined concurrency, privacy, error, and redaction requirements.
- Defined focused future tests and an implementation sequence.
- Updated the roadmap to make focused maintainer/security review the next P0
  phase.

## 3. Scope Explicitly Not Completed

- No operational entry implementation.
- No scheduler, daemon, queue, worker, polling, or wake service.
- No model-turn or conversation creation.
- No automatic approval.
- No provider execution or mutation.
- No OpenShell or sandbox integration.
- No nested harness execution.
- No public configuration, CLI, SDK, schema, or example.
- No filesystem or PostgreSQL parity.
- No hosted, distributed, production, or release change.

## 4. Core Planning Decisions

1. Fresh process entry rehydrates current authoritative state; it does not
   trust assistant memory or caller-selected mode.
2. Initial and resumed execution use distinct accepted one-use authority
   paths.
3. Replay and reconciliation may prove durable history but never reissue a
   capability.
4. Exact invocation material remains injected in memory and must match its
   committed binding before authority or executor use.
5. Typed wait, blocked, and terminal posture return without polling,
   fabricated approval, or false workflow completion.

## 5. Restart And Failure Summary

A restart reruns one deterministic classification against SQLite state. A
lawful `ResumeNow` consumes fresh current authority. Wait, blocked, and
terminal posture is surfaced directly. Ambiguous, stale, inconsistent, or
committed-with-lost-capability posture fails closed.

The plan intentionally does not solve wake scheduling. It defines a bounded
condition handoff so a later trusted host can register a wait and reinvoke the
same entry boundary after a lawful external change.

## 6. Security And Privacy Summary

Capabilities remain private, owned, non-serializable, and non-reconstructable.
Durable receipts remain evidence only. Invocation payloads remain in memory.
Errors and Debug output use stable bounded redaction-safe posture and do not
expose paths, source contents, command output, provider payloads, credentials,
tokens, approval reasons, evidence bodies, or raw skill output.

## 7. Validation

Planning validation:

- `npm run check:docs`
- `git diff --check`
- manual security and scope review

No Rust validation is required for this documentation-only planning phase.

## 8. Remaining Limitations

- The first implementation must reuse the existing
  `open_with_registered_current_authority` same-call boundary; the operational
  entry helper may not construct opening authorization itself.
- Committed-but-unused opening recovery may need an additional closed
  operation; capability reconstruction remains prohibited.
- No accepted host yet registers typed waits or reinvokes the entry boundary.
- The planned first implementation remains private, SQLite-only, and local.
- No external executor, provider, sandbox, or model runtime is integrated.

## 9. Recommended Next Phase

Perform a focused maintainer/security review of the trusted-host operational
entry boundary plan. Resolve the authority-source and committed-capability-loss
questions before implementing the private SQLite slice.

## 10. Governed Planning Record

- workflow: `dg/d`
- run: `run-1791013356634415000-2`
- approval: `approval/run-1791013356634415000-2/planning-approved`
- presentation: `presentation/70a02f14b50acf0b`
- presentation hash:
  `70a02f14b50acf0b5b38bd80da61260d7cea06961ff54e13ddff7a018a813fa9`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval-presentation proof enforced
- validation summary: `npm run check:docs`, `git diff --check`, and manual
  security/scope review passed; Rust checks were not required because this
  phase changed documentation only
- out-of-kernel work: source inspection, plan authoring, validation, and git
  actions were performed by the delegated maintainer; the kernel governed the
  phase scope and approval but did not inspect code, edit files, or execute
  shell commands
