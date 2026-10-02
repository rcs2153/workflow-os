# Atomic Dispatch Reservation Implementation Review

## 1. Executive Verdict

**Phase accepted after blocker fix; proceed to bounded trusted-host
redispatch planning.**

The private SQLite implementation now provides one atomic winner before
executor entry, issues execution authority only after an unambiguous commit,
withholds authority on replay and ambiguous reconciliation, and binds every
downstream outcome, yield, or recovery write to the exact reservation.

The review found one blocker: replay and downstream validation checked the
reservation and admission event but did not cross-check the projected run
snapshot required by the accepted plan. The bounded fix now validates the
relational rows, serialized records, durable event, event-history prefix,
point-in-time snapshot commitment, and current rehydrated snapshot through one
shared integrity path. A corruption test proves snapshot drift blocks both
downstream persistence and replay.

## 2. Scope Verification

The phase stayed within the approved private, local, SQLite-only boundary. It
did not add:

- a scheduler, daemon, queue, polling loop, or repeated supervisor loop;
- provider execution, provider mutation, OpenShell, or live adapters;
- nested harness execution, recursive agents, or agent swarms;
- automatic approval or evidence, check, policy, or authority bypass;
- public runtime configuration, workflow schemas, CLI, SDK, UI, or examples;
- filesystem or PostgreSQL reservation support;
- lease renewal, reservation stealing, timeout reassignment, or host failover;
- prompts, transcripts, source bodies, command output, credentials, or
  provider payload storage; or
- hosted execution, Reasoning Lineage, or release-posture changes.

## 3. Model And Authority Assessment

Reservation operation and receipt identities are bounded and private. The
only constructor for `ReservedAttemptDispatchCapability` remains inside the
reservation module. The capability is owned, non-cloneable, non-serializable,
and Debug-redacted.

Durable replay returns only a redacted receipt posture. Neither replay nor
ambiguous-commit reconciliation can reconstruct executor authority from a
reservation row, event, projection, snapshot, or receipt.

## 4. Atomic Transaction Assessment

One `BEGIN IMMEDIATE` transaction validates the exact trusted-time posture,
window, attempt, actor, immutable bundle, governance and authority
commitments, cursor, revision, invocation, executor, and expiry. It then:

1. advances the continuity cursor and revisions;
2. appends the payload-free admission event;
3. projects the run snapshot;
4. inserts the unique reservation and projection relation; and
5. commits before returning the private capability.

The attempt primary key and additional unique operation, receipt, event,
sequence, and commitment constraints provide deterministic one-winner
storage semantics. Concurrent tests prove exactly one executor entry.

## 5. Replay And Ambiguity Assessment

Exact replay requires the same operation, request commitment, and receipt. It
returns no authority and appends no event. A different operation for the same
attempt fails before executor entry.

Before-commit failure rolls back all reservation state. After-commit ambiguity
opens a fresh connection, proves the exact durable relation, and returns a
non-authorizing committed-but-unavailable posture. Confirmed absence and
corrupt or partial state fail closed with stable errors.

## 6. Event And Snapshot Integrity Assessment

The additive `AuthorizedExecutionAttemptDispatchAdmitted` event is
payload-free and does not change workflow status. Its projection binds the
operation, receipt, attempt, revisions, request, invocation, executor,
authority, governance, trusted time, reservation, and before/after cursors.

The initial implementation did not validate the stored run snapshot on replay
or downstream use. That violated the plan's exact reservation/event/snapshot
relation and was blocking. The fix centralizes integrity validation and now
requires:

- relational columns to match the serialized reservation and projection;
- the durable event payload to equal the projection binding;
- the expected and result cursors to occupy the exact history positions;
- the event-prefix rehydration to produce the recorded point-in-time snapshot
  commitment and admission projection; and
- the current snapshot row and commitment to equal full-history rehydration.

Snapshot commitment tampering now prevents downstream outcome persistence and
also prevents replay before a second executor call.

## 7. Downstream Mutation Assessment

Outcome, yield, and ambiguous-recovery commitments include the exact dispatch
receipt, reservation commitment, and admission cursor when reservation is
present. Their SQLite transactions validate that binding before applying a
continuity mutation.

An old capability without a reservation binding cannot satisfy the trusted
supervisor path. For compatibility, legacy request commitments remain exactly
unchanged when no reservation exists; focused coverage protects yield,
outcome, and recovery wire shapes across the explicit V4-to-V5 upgrade.

## 8. Migration And Compatibility Assessment

SQLite schema V5 is additive and explicit. Existing managed V4 databases must
use the dedicated V4-to-V5 upgrade. Unmanaged, partial, drifted, or newer
schemas fail closed.

The schema manifest digest and migration tests cover both new tables. No
automatic migration or non-SQLite parity was introduced.

## 9. Privacy And Error Safety

Custom Debug output redacts capability and reservation bindings. Identifiers
are bounded, ASCII-only, whitespace-free, and reject common secret markers.
Stored material is limited to identities, commitments, revisions, cursors,
trusted timestamps, and event/projection relations.

Errors do not include caller values, paths, invocation inputs, command output,
credentials, authorization material, or provider payloads. Snapshot and
projection corruption maps to bounded recovery posture.

## 10. Test Quality Assessment

Coverage proves:

- successful reservation before opened and resumed executor entry;
- one winner under concurrent dispatch;
- no executor call or event after pre-commit failure;
- no authority after ambiguous commit;
- no authority or duplicate event on exact replay;
- stale and substituted invocation/executor posture fails before dispatch;
- every closed executor result remains deterministic;
- result, yield, and recovery require reservation binding;
- legacy absent-reservation commitments remain stable;
- event ordering and snapshot rehydration remain deterministic;
- snapshot projection corruption blocks downstream persistence and replay; and
- explicit schema upgrade remains valid through V5.

The complete Core unit suite passes with 348 tests after the blocker fix.
Focused integration coverage and strict Core clippy also pass. Full workspace
CI remains the authoritative merge gate because this macOS host has a known
multi-minute process-launch delay for each Rust integration binary.

## 11. Documentation Assessment

The plan, implementation report, and roadmap retain the local/private
boundary and do not overclaim scheduler, provider, sandbox, hosted, or public
configuration support. The implementation report records the locally
incomplete workspace test honestly.

## 12. Remaining Blockers

There are no remaining blockers for the private one-shot SQLite dispatch
reservation slice.

Repeated scheduling, multiple hosts, public runtime configuration, provider
execution, OpenShell, and nested harness execution remain blocked behind
separate plans and reviews.

## 13. Non-Blocking Follow-Ups

1. Add relational tamper cases for every reservation column, beyond the
   snapshot corruption regression added during review.
2. Add filesystem or PostgreSQL support only through independent conformance
   and durability work.
3. Keep `.workflow-os/` local runtime state outside source commits.
4. Require the complete workspace matrix in pull-request CI before merge.

## 14. Recommended Next Phase

Plan a **bounded trusted-host redispatch loop** over the accepted execution
window, directive, supervisor, and atomic reservation boundaries. The plan
must distinguish immediate lawful redispatch from typed waits and terminal or
blocked posture, and it must never manufacture model turns, approvals,
evidence, checks, or execution authority.

The next phase must remain private, local, injected, and free of provider
mutation, OpenShell, nested harnesses, public configuration, CLI, schemas,
hosted behavior, and release changes.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1790926668372243000-2`
- approval: `approval/run-1790926668372243000-2/review-scope-approved`
- presentation: `presentation/3920883b701e1e27`
- presentation hash:
  `3920883b701e1e279c776f3cbf73c2b36815e35d98acb59190a35cf151d3eeda`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- reviewed implementation commit: `d048b7e`
- review blocker: missing snapshot cross-check on replay and downstream use
- blocker disposition: fixed with centralized integrity validation and
  corruption regression coverage
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
  approval-presentation proof enforced
- validation summary: formatting, strict Workflow Core clippy, 348 Core unit
  tests, focused integration tests, docs, and diff checks passed; complete
  workspace CI remains required before merge
- out-of-kernel work: source inspection, security analysis, blocker repair,
  tests, review authoring, and git actions were performed by the delegated
  maintainer; the kernel governed scope and approval but did not inspect code,
  edit files, or run shell commands
