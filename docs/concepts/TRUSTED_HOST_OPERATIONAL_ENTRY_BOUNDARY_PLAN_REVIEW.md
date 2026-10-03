# Trusted-Host Operational Entry Boundary Plan Review

## 1. Executive Verdict

**Plan accepted with non-blocking follow-ups.**

The corrected plan defines a narrow private local SQLite entry boundary over
accepted authority, opening, continuation, reservation, supervisor, and
redispatch primitives. It does not reconstruct capabilities, infer invocation
payloads, or let a host choose continuation posture.

The review found one planning defect: the initial text left the source of
fresh opening authorization as an open question even though Core already owns
the accepted `open_with_registered_current_authority` same-call boundary. The
plan now requires reuse of that exact path and prohibits the entry helper from
constructing opening authorization itself. No blocker remains after that
correction.

## 2. Scope Verification

The plan stays within a private planning boundary. It authorizes one future
crate-private SQLite helper and focused tests only. It does not authorize:

- runtime implementation in this review;
- a scheduler, daemon, queue, poller, or wake service;
- model-turn or conversation creation;
- automatic approval or delegated-authority broadening;
- provider execution or mutation;
- OpenShell or another sandbox;
- nested harness execution;
- public configuration, CLI, SDK, schemas, or examples;
- filesystem or PostgreSQL parity;
- hosted or distributed execution; or
- release-posture changes.

## 3. Authority Acquisition Assessment

The corrected plan uses the accepted
`open_with_registered_current_authority` boundary for initial opening. That
function rehydrates the run, enters the private registered source's same-call
use, derives current authority and governance commitments, constructs the
private opening authorization inside the closure, and returns a one-use
capability only after unambiguous SQLite commit.

The operational entry helper may pass the existing registered source,
required-context bindings, exact invocation commitment, trusted time, and
fresh non-authorizing identities. It may not accept caller-authored
authorization, readiness, governance commitment, or authority capability.

Resume authority remains separate. A current `ResumeNow` must consume one
current directive through the accepted projected compare-and-set operation.
Neither path derives authority from durable receipts or events.

## 4. Restart And Rehydration Assessment

The plan correctly treats restart as fresh classification against SQLite:

- eligible state with no active window may enter the accepted opening path;
- current `ResumeNow` may consume one fresh directive;
- `AwaitCondition` returns typed wait posture;
- `Blocked` returns without retry or approval translation;
- `Terminal` returns without executor entry; and
- ambiguous, stale, executing, recovery-required, or inconsistent posture
  fails closed or uses an already accepted recovery operation.

No in-memory capability is serialized or reconstructed. The plan does not
claim that a process restart itself wakes work or creates a model turn.

## 5. Immutable Invocation And Executor Assessment

The invocation remains an injected in-memory `SkillInput`. The future helper
must validate its exact commitment against the immutable run and opening
binding before authority use and again through the accepted supervisor before
executor entry. The same injected executor and binding must be passed into the
bounded redispatch loop unchanged.

The host cannot use a current mutable workflow file, assistant memory, or
persisted payload as invocation truth. The proposed invocation source provides
content, not authority, and cannot select a different workflow/run/step.

## 6. Replay, Ambiguity, And Capability-Loss Assessment

Exact opening replay and reconciliation return durable proof but no capability.
The plan correctly forbids translating that proof into authority. An
ambiguous opening commit may be reconciled for audit truth, but the executor
is not entered.

The committed-but-unused capability-loss case remains intentionally fail
closed in the first implementation. A future recovery mechanism, if needed,
must be a separately planned closed Core operation. This is a non-blocking
limitation because the plan neither promises recovery nor weakens authority.

## 7. Wait And Blocked Handoff Assessment

The proposed outcome carries bounded identity, stop-reason, revision or
cursor commitment, typed condition identities, and a closed next-operation
category. It excludes raw evidence, approval reasons, prompts, command output,
provider payloads, credentials, and source contents.

This is sufficient for an internal return boundary. A later host may register
the condition and reinvoke the same entry point after a lawful state change.
The current plan correctly defers wake registration and scheduling.

## 8. Concurrency Assessment

The plan relies on accepted immediate SQLite transactions rather than
process-local locks:

- active-window conflict or replay protects initial opening;
- current-directive consumption admits one resume caller;
- dispatch reservation admits one executor caller; and
- stale callers fail on cursor, revision, generation, or binding checks.

Independent-connection concurrency tests are required. The plan does not add
a second source of liveness or admission truth.

## 9. Privacy And Error Assessment

The planned boundary keeps invocation input and capabilities in memory, uses
bounded closed outcomes, and requires redaction-safe Debug and static errors.
It prohibits persistence or error echo of source/spec contents, prompts,
transcripts, command output, provider payloads, environment values,
credentials, tokens, authorization headers, approval reasons, evidence
bodies, and raw outputs.

The privacy posture matches the accepted opening, supervisor, and redispatch
boundaries.

## 10. Test Quality Assessment

The future test plan covers fresh opening, restart resume, every closed stop
posture, replay, ambiguous commit, capability loss, concurrent callers, stale
state, invocation/executor substitution, restart behavior, redaction, and the
separation between execution-window terminal posture and workflow completion.

Two additional assertions should be included during implementation:

- prove the entry helper cannot compile with a caller-constructed opening
  authorization path; and
- prove the registered current-authority source is evaluated in the same call
  that performs opening, rather than cached by the entry helper.

These are non-blocking additions to an otherwise sufficient test plan.

## 11. Blockers

None after the in-review correction requiring the existing
`open_with_registered_current_authority` path.

## 12. Non-Blocking Follow-Ups

- Decide whether committed-but-unused capability loss needs a future explicit
  recovery operation after observing the private implementation.
- Choose the smallest opaque typed-wait handoff shape.
- Add direct composition-level fault injection around process loss between
  opening commit, capability return, reservation, and executor entry.
- Keep the first caller private and injected; do not expose operational entry
  through CLI, SDK, schema, or hosted runtime.

## 13. Recommended Next Phase

Implement the private local SQLite trusted-host operational entry boundary.
Reuse the accepted registered-current-authority opening path, current
directive consumption, one-shot supervisor, atomic dispatch reservation, and
bounded redispatch loop. Add no public caller or scheduler. Require focused
maintainer/security review before any operational host integration.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1791013634569619000-2`
- approval: `approval/run-1791013634569619000-2/review-scope-approved`
- presentation: `presentation/5e95041c5cf751e6`
- presentation hash:
  `5e95041c5cf751e60cbad3df58513ca09700dce7ebdfdf8d5a507c1b59f6aa8b`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval-presentation proof enforced
- validation summary: `npm run check:docs`, `git diff --check`, and focused
  source/security/privacy/scope review passed; Rust checks were not required
  because this phase changed documentation only
- out-of-kernel work: source and plan inspection, review authoring, validation,
  and git actions were performed by the delegated maintainer; the kernel
  governed phase scope and approval but did not perform those actions
