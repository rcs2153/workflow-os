# Trusted-Host Operational Entry Boundary Implementation Report

## 1. Executive Summary

The private local SQLite trusted-host operational entry boundary is
implemented. One Core-owned helper now rehydrates authoritative continuity
state, validates the exact workflow/run/step, immutable run bundle, invocation,
and executor binding, and chooses only among accepted initial opening,
immediate resume, typed wait, blocked, or terminal paths.

Initial opening reuses `open_with_registered_current_authority`; the entry
helper cannot construct opening authorization itself. Restarted `ResumeNow`
entry consumes one current directive and passes the resulting one-use
capability into the accepted bounded redispatch loop. Wait, blocked, and
terminal postures return without executor entry. No capability is persisted,
serialized, cloned, inferred, or reconstructed.

The slice remains crate-private, local, injected, SQLite-only, and
review-pending. It adds no scheduler, daemon, automatic caller, model turn,
provider execution, OpenShell integration, nested harness execution, public
configuration, CLI, SDK, schema, hosted behavior, or release change.

## 2. Scope Completed

- Added a crate-private operational-entry locator bound to workflow, run,
  step, window, subject actor, and immutable run bundle.
- Added a crate-private explicit input carrying SQLite state, optional
  accepted opening context, exact in-memory `SkillInput`, injected executor,
  bounded persistence identities, and the existing non-authorizing identity
  provider.
- Added deterministic fresh-state classification for no window, one exact
  window, and ambiguous window state.
- Reused the accepted registered-current-authority opening operation for a
  fresh eligible scope.
- Reused projected directive consumption for restarted `ResumeNow` posture.
- Reused the accepted bounded redispatch loop for every admitted executor
  entry.
- Returned zero-entry bounded outcomes for `AwaitCondition`, `Blocked`, and
  execution-window `Terminal`.
- Added stable non-leaking errors for missing or mismatched opening context,
  ambiguous state, corrupt durable binding, changed invocation, and invalid
  identity.
- Added focused tests for missing opening context, blocked entry, terminal
  re-entry, restart/resume, and invocation substitution.

## 3. Scope Explicitly Not Completed

This phase did not add:

- a scheduler, daemon, queue, worker, poller, wake service, or timer;
- automatic caller integration, model-turn creation, or agent recirculation;
- automatic approval or delegated-authority expansion;
- provider execution, provider mutation, OpenShell, or another sandbox;
- nested harnesses, recursive agents, or agent swarms;
- public runtime configuration, workflow fields, CLI, SDK, UI, or examples;
- filesystem, PostgreSQL, multi-host, lease, heartbeat, or failover parity;
- capability persistence, serialization, cloning, or reconstruction;
- invocation payload persistence or inference from commitments;
- new SideEffect, WorkReport, reasoning-lineage, or approval vocabulary; or
- hosted, distributed, production, or release-readiness claims.

## 4. Private API Summary

The implementation is isolated in
`sqlite_state::trusted_host_operational_entry` and is not exported from
`workflow-core`.

Its private surface consists of:

- `TrustedHostOperationalEntryLocator`;
- `TrustedHostOperationalEntryInput`; and
- `enter_trusted_host_operation`.

The input is explicit and contains no hidden global state. The host supplies
the exact in-memory invocation and executor, but neither grants authority.
Core compares their commitments against the durable opening binding before
resume or dispatch.

## 5. Initial Opening Boundary

When no execution window exists, opening context is mandatory. The helper
verifies that the opening backend, workflow/run/step identity, actor,
immutable run bundle, window identity, and invocation commitment exactly
match the entry locator and injected invocation.

It then calls the existing `open_with_registered_current_authority` boundary.
That same-call path resolves current registered authority, constructs the
private opening authorization inside Core, and commits opening atomically.
Only a newly opened result carries a one-use capability. Exact replay proves
durable history but is rejected because it cannot reissue authority.

## 6. Restart And Resume Boundary

When one exact window exists, the helper validates actor and immutable-bundle
binding, reads the payload-free persisted operation commitment, and compares
it with the exact current invocation and executor commitment.

The helper derives continuation disposition freshly:

- `ResumeNow` consumes one current directive and obtains one fresh resumed
  capability;
- `AwaitCondition` returns a zero-entry typed stop;
- `Blocked` returns a zero-entry bounded block; and
- `Terminal` returns a zero-entry execution-window terminal outcome.

The resumed capability and exact invocation are passed to the accepted
bounded redispatch loop. The entry helper never reconstructs authority from a
row, event, receipt, projection, snapshot, report, or assistant memory.

## 7. Workflow Semantics

The boundary does not mutate workflow lifecycle semantics beyond the accepted
opening, continuation, supervisor, and projection operations that it composes.
Execution-window `Terminal` is not workflow completion. Focused tests confirm
that resumed execution can close the window while the workflow run remains
`Running`.

Wait and blocked results do not fabricate approvals, polling, success, or
completion. The future host remains responsible for registering a lawful wait
condition and reinvoking this private boundary after an authoritative state
change.

## 8. Security And Privacy

- Capabilities remain owned, one-use, non-serializable, and private.
- `SkillInput` stays in memory and is never written by the entry helper.
- Input and locator Debug output redact all binding details.
- Stable errors contain static bounded messages and do not echo rejected
  identifiers, invocation values, paths, credentials, tokens, or output.
- Durable reads contain only accepted identifiers, commitments, cursors,
  revisions, trusted-time facts, and closed state vocabulary.
- Changed secret-like invocation input fails before identity generation or
  executor entry and is absent from error Debug output.

## 9. Test Coverage

Focused tests prove:

- a fresh scope without accepted opening context fails closed before executor
  entry or identity generation;
- an existing executing window returns `Blocked` with zero executor entries;
- an execution-window terminal posture returns `Terminal` without another
  executor entry;
- a process-style restart at `ResumeNow` consumes fresh authority and invokes
  exactly one resumed attempt;
- changed invocation input fails before resume, identity generation, or
  executor entry; and
- workflow status remains separate from execution-window terminal posture.

The accepted opening tests separately cover initial opening, exact replay,
ambiguous commit reconciliation, active-window conflict, and transactional
projection. Accepted directive, reservation, supervisor, and redispatch tests
retain one-winner, stale-binding, failure, ambiguity, restart, and privacy
coverage for the exact operations this helper composes.

## 10. Validation

Passed during implementation:

- `cargo fmt --all --check`
- focused Core operational-entry tests (`5 passed`)
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

Pull-request CI remains an independent repository proof rather than a
substitute for local validation.

## 11. Remaining Known Limitations

- The helper is private, local, injected, and SQLite-only.
- No automatic or public runtime caller invokes it.
- The phase does not register waits or recirculate an executor after a wake.
- A committed opening whose in-memory capability is lost cannot be recovered
  or reconstructed and fails closed.
- Direct entry-level fresh-opening fixture coverage is deferred; the entry
  composes the separately tested same-call registered-current-authority
  opening boundary and directly tests missing-context failure.
- Composition-level two-caller races and commit fault injection remain
  covered at reused primitive boundaries rather than repeated at this helper.
- There is no filesystem, PostgreSQL, multi-host, scheduler, provider,
  sandbox, or nested-harness integration.

## 12. Recommended Next Phase

Perform a focused maintainer/security implementation review. Verify exact
source-of-truth classification, reuse of the registered-current-authority
opening boundary, no capability reconstruction, exact invocation and executor
binding, one-use resume authority, closed wait/block/terminal behavior,
non-leaking errors, and absence of public/runtime/provider broadening.

If accepted, plan the smallest private trusted-host caller that can register
typed waits and reinvoke this boundary after lawful external state changes.
Do not add provider execution, OpenShell, nested harnesses, automatic
approval, public configuration, CLI, SDK, hosted behavior, or release claims.

## 13. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1791014084961873000-2`
- approval: `approval/run-1791014084961873000-2/implementation-approved`
- presentation: `presentation/def5c5b96971d75f`
- presentation hash:
  `def5c5b96971d75f5ac641987ac182fb0bbe5b7ce9881ffb7fc322cd3d38c568`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: private local SQLite trusted-host operational entry
  helper only
- phase status: completed
- event summary: 39 events, including 1 approval request, 1 approval grant,
  8 policy decisions, 6 scheduled steps, 6 successful skill invocations, 0
  retries, and 0 escalations
- out-of-kernel execution: source inspection, code edits, tests,
  documentation, and git actions are performed by the delegated trusted host;
  Workflow OS governs scope and approval but does not edit files or execute
  shell commands
