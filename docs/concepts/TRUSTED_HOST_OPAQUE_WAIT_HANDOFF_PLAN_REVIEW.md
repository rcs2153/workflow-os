# Trusted Host Opaque Wait Handoff Plan Review

## 1. Executive Verdict

**Plan accepted with implementation constraints; proceed to the private model
and coherent observation slice.**

The plan correctly treats the wait handoff as an inert, deterministic
projection of authoritative continuity state rather than a durable record or
bearer capability. It preserves the kernel as the only continuation
classifier and keeps wake verification, directive consumption, dispatch
reservation, and executor entry outside the handoff boundary.

No planning blocker remains. Implementation must preserve the constraints in
this review, especially coherent SQLite snapshot construction, dedicated type
separation, cursor commitment rather than cursor exposure, and the absence of
public serialization.

## 2. Scope Verification

The plan remains within the approved planning-only boundary. It does not
authorize:

- wait satisfaction or wake verification;
- polling, sleeping, timers, queues, wake buses, or scheduling;
- automatic reinvocation or agent-turn management;
- additional wait-source families;
- provider, OpenShell, sandbox, or nested-harness execution;
- authority or capability reconstruction;
- public Rust API, workflow schema, CLI, SDK, UI, example, or runtime
  configuration;
- persistence schema changes or a durable handoff table;
- hosted, distributed, write, release, or production-readiness changes.

## 3. Architectural Fit

The handoff fills one real gap between authoritative continuation state and a
trusted host. `AwaitCondition` currently communicates only a disposition. The
planned value adds bounded correlation and safe condition posture without
granting permission or duplicating durable truth.

That division matches the repository's operating principle:

- SQLite continuity state remains authoritative;
- Core derives current posture;
- the handoff describes one point-in-time projection;
- source-specific verifiers own satisfaction;
- fresh operational entry owns executor admission; and
- the host remains transport and timing, not governance.

## 4. Model Assessment

The candidate model is appropriately private and small. Implementation should
use:

- a dedicated crate-private `TrustedHostWaitHandoffId` newtype;
- one crate-private handoff value;
- one bounded condition descriptor;
- one safe dependency-kind enum; and
- one closed next-operation enum.

Using a dedicated ID type is required. Reusing `SpecContentHash` directly at
call sites would make accidental substitution between content integrity and
handoff correlation easier.

The first handoff may carry the exact private window ID, generation ID,
condition ID, and version internally. Those are selectors, not authority. They
must not be public accessors, serde output, or Debug content in this slice.

## 5. Identity Assessment

A domain-separated deterministic commitment is the right handoff identity.
It should cover:

- window identity and revision;
- cursor commitment, not the raw cursor;
- active yield generation identity; and
- sorted condition identity, version, kind, and state.

It should not cover raw dependency values, deadlines, trusted-time material,
or authority internals. Relevant authoritative state transitions already
change revisions, cursors, generations, or condition posture; the handoff does
not need to hash sensitive dependency bindings to become stale when state
changes.

The ID remains correlation only. Equality cannot establish freshness or
authority.

## 6. Coherent Construction Assessment

The strongest requirement in the plan is that Core construct the handoff from
one coherent snapshot. The current general snapshot loader performs multiple
queries on a connection; calling it without an explicit SQLite read
transaction is not sufficient for this boundary under concurrent writers.

Implementation must therefore either:

1. open an explicit SQLite read transaction and load/classify/derive within
   it; or
2. consume an already-loaded snapshot that the caller can prove came from one
   transactionally coherent read.

It must not call `continuation_disposition` and then independently reload rows
to build the handoff. That would create a time-of-check/time-of-projection
split, even though the resulting handoff is inert.

A trusted-time observation may be obtained for classification, but its source,
provenance, and epoch must match the same snapshot. The observation must not be
stored in or exposed through the handoff.

## 7. Next-Operation Assessment

`RequestFreshClassification` is the correct only next-operation vocabulary for
the first slice. It communicates what a host may ask Core to do without
suggesting that the handoff can:

- satisfy a wait;
- authorize a transition;
- consume a directive;
- reserve dispatch;
- invoke an executor; or
- create a fresh handoff from stale values.

The existing `TimeWindow` wake caller must not accept a handoff ID as proof.

## 8. Non-Authoritative Security Assessment

The planned invariants are sufficient if enforced structurally:

- handoff types remain crate-private;
- no conversion to an attempt, wake, directive, or dispatch capability exists;
- no mutating store method accepts a handoff;
- later operations reload and revalidate authoritative state;
- unsupported or corrupt state returns no plausible handoff; and
- stale handoffs remain inert.

The implementation should include a compile-time/API-shape assertion where
practical: the observation result can be inspected by the private host path,
but cannot be passed into transition or entry helpers.

## 9. Replay, Restart, And Concurrency Assessment

Treating the handoff as a deterministic projection avoids a new replay table,
receipt, or write. The plan correctly requires:

- stable identity for repeated observation of unchanged state;
- stable identity after restart over unchanged state;
- changed identity after relevant state movement; and
- old-or-new coherent results under an observation/transition race, never a
  mixed projection.

The race test is required for acceptance of implementation because coherent
read behavior is the main security claim of this slice.

## 10. Privacy And Redaction Assessment

The plan's privacy posture is conservative. Implementation must provide custom
Debug output that includes only safe posture and counts. It must not expose:

- window, generation, condition, or cursor values;
- deadlines or trusted-time observations;
- dependency, source, provenance, or epoch commitments;
- prompts, commands, logs, source content, or provider payloads;
- approval, evidence, or check bodies;
- executor input or output; or
- authority, capability, directive, receipt, credential, or token material.

No serde implementation is needed in the first slice. Stable errors must
remain value-free.

## 11. Resolved Open Questions

- **Condition identity:** private condition IDs and versions may be retained
  inside the handoff; Debug and serialization must not expose them.
- **Cursor:** retain only a domain-separated cursor commitment in the handoff.
- **Blocked and terminal:** return ordinary private disposition outcomes with
  no handoff; do not create sibling blocked or terminal handoff models.
- **Handoff ID:** use a dedicated crate-private newtype backed by a
  domain-separated commitment.

## 12. Test Plan Assessment

The proposed test plan is strong. Implementation acceptance requires focused
proof of:

- coherent construction inside an explicit read transaction;
- one exact `TimeWindow` handoff;
- deterministic identity across repeated reads and restart;
- identity change after revision, cursor, generation, or condition movement;
- no handoff for blocked, terminal, satisfied, canceled, expired,
  superseded, unsupported, or corrupt posture;
- no writes or events during observation;
- old-or-new coherence under an observation/transition race;
- structural inability to use a handoff as transition authority;
- redaction-safe Debug and errors; and
- no public API or serde expansion.

Workspace formatting, clippy, tests, docs, and existing continuity suites must
remain green.

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- Consider whether a future trusted host needs a separate bounded diagnostic
  projection for operators; do not add it to the first handoff.
- Keep condition count bounded even though the first caller supports one wait.
- Revisit public serialization only when a separately reviewed transport or
  scheduler boundary exists.

## 15. Recommended Next Phase

Implement the crate-private `TrustedHostWaitHandoff` model and coherent
SQLite observation path for an already-registered exact `TimeWindow` wait.
Integrate only the observation path into the smallest private caller.

Do not combine this phase with wake transition changes, explicit reinvocation,
polling, scheduling, provider or sandbox execution, nested harnesses, public
configuration, CLI, SDK, hosted behavior, writes, or release changes.

## 16. Validation

- `npm run check:docs`
- `git diff --check`

## 17. Governed Review Record

- workflow: `dg/review`
- run: `run-1791062777875840000-2`
- approval:
  `approval/run-1791062777875840000-2/review-scope-approved`
- presentation: `presentation/ba302b787c340f72`
- approval outcome: granted by delegated maintainer through proof enforcement
- approved boundary: focused maintainer/security review only
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations;
  approval-presentation proof enforced
- validation summary: documentation and diff checks passed
- out-of-kernel work: architecture inspection, security review, review
  authoring, validation, and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not inspect
  source, edit files, execute checks, create a WorkReport artifact, or perform
  git actions
