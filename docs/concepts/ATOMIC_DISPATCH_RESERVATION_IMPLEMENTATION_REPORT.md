# Atomic Dispatch Reservation Implementation Report

## 1. Executive Summary

The private SQLite atomic dispatch-reservation slice is implemented. The
trusted-host supervisor no longer relies on a read-only dispatchability check.
Before executor entry, one immediate SQLite transaction now validates the
exact started attempt, inserts one unique reservation, appends one payload-free
`AuthorizedExecutionAttemptDispatchAdmitted` event, projects the run snapshot,
and returns a private one-use capability only after an unambiguous commit.

Concurrent, replayed, stale, conflicting, and ambiguous callers cannot cross
the executor boundary. Durable reservation evidence cannot be converted back
into execution authority.

This remains private, local, opt-in, one-shot, and SQLite-only. It does not add
a scheduler, provider execution, OpenShell, nested harnesses, automatic
approval, public configuration, CLI, SDK, schemas, hosted behavior, or another
provider mutation family.

## 2. Scope Completed

- Added private bounded reservation operation and receipt identities.
- Added private request, record, binding, replay-receipt, outcome, store, and
  non-cloneable dispatch-capability types.
- Added payload-free dispatch-admission event and snapshot projection
  vocabulary.
- Added explicit SQLite schema V5 reservation and projection tables.
- Added explicit V4-to-V5 upgrade with managed-manifest validation.
- Added one-winner reservation under an immediate SQLite transaction.
- Added exact replay and fresh-connection ambiguous-commit reconciliation.
- Replaced supervisor read-only dispatch validation with reservation before
  executor entry.
- Bound outcome, yield, and ambiguous-recovery persistence to the reservation
  receipt, commitment, and admission cursor.
- Added focused concurrency, replay, fault, migration, projection, and
  regression tests.

## 3. Scope Explicitly Not Completed

This phase did not add:

- a scheduler, daemon, queue, polling loop, or repeated supervisor loop;
- model turns, conversational auto-resume, or executor recirculation;
- provider execution, provider mutation, live adapters, or OpenShell;
- nested harness execution, recursive agents, or agent swarms;
- automatic approval or any evidence, check, policy, or authority bypass;
- public runtime configuration, workflow schema, CLI, SDK, UI, or example;
- filesystem or PostgreSQL dispatch reservation;
- lease renewal, reservation stealing, automatic timeout recovery, or host
  reassignment;
- prompts, transcripts, source contents, command output, environment values,
  credentials, authorization headers, or provider payload storage; or
- release-posture changes.

## 4. Private Model And Event Boundary

`dispatch_reservation` owns the only constructor for the private
`ReservedAttemptDispatchCapability`. The capability is owned, non-cloneable,
non-serializable, Debug-redacted, and bound to the exact accepted attempt,
window, invocation, executor, authority, governance, trusted-time, operation,
receipt, commitment, and admission cursor.

The additive `AuthorizedExecutionAttemptDispatchAdmitted` event contains only
bounded identities, revisions, commitments, and cursors. Rehydration projects
the latest admission binding without changing workflow status or claiming
attempt, step, or run completion.

## 5. Atomic Transaction Boundary

One immediate SQLite transaction:

1. validates the request commitment and trusted-time posture;
2. loads the exact executing window and started attempt;
3. validates actor, immutable run, governance, authority, invocation,
   executor, cursor, revision, epoch, and expiry bindings;
4. rejects an existing reservation for the attempt;
5. advances the bounded continuity cursor and revisions;
6. appends and projects one dispatch-admission event;
7. inserts the unique reservation and projection binding;
8. commits; and
9. returns private execution authority only after unambiguous success.

The managed V5 schema digest is
`17534b84a0ee25e73b4415842c5127eee8d5190fe28280aa7e5915a2f7bdcd6f`.
Older managed schemas require explicit upgrades; unmanaged, partial, newer,
or drifted schemas fail closed.

## 6. Replay, Ambiguity, And Recovery

Exact operation replay validates the durable reservation/projection relation
and returns a bounded redacted replay receipt without a capability or another
event. A different operation for the same attempt is rejected before executor
entry.

If commit acknowledgement is ambiguous, reconciliation opens a fresh
connection. A proven commit returns the same non-authorizing receipt posture;
confirmed absence returns a stable error. The implementation never
reconstructs a capability from a row, receipt, event, or snapshot.

Outcome, yield, and ambiguous-attempt recovery transactions validate the
reservation receipt, commitment, admission cursor, event payload, attempt,
and projection before recording downstream state. An old attempt capability
without the admission binding is insufficient.

## 7. Trusted-Host Supervisor Integration

The private one-shot supervisor now reserves the exact attempt before calling
the injected executor. Only the sole `Admitted` outcome can enter the
executor. `AlreadyAdmitted`, committed-but-capability-unavailable, stale,
expired, conflicting, and corrupt postures return stable non-leaking errors.

The existing closed result matrix remains intact: success, retryable failure,
terminal failure, turn-boundary yield, and ambiguous-may-have-started. A
successful executor callback still does not complete the workflow run.

## 8. Privacy And Redaction

- Identifiers are bounded, ASCII-only, whitespace-free, and reject common
  secret-like markers.
- Capability, reservation binding, event, projection, and replay-receipt Debug
  output is redacted.
- Serialization validates commitments and rejects malformed or unknown data.
- Stable errors never include caller values, paths, invocation data, command
  output, metadata, credentials, or provider payloads.
- SQLite stores bounded identities, commitments, revisions, cursors, trusted
  timestamps, and event/projection relations only.

## 9. Test Coverage

Focused coverage proves:

- successful reservation and executor entry for opening and resumed attempts;
- before-commit failure causes zero executor calls and no admission event;
- after-commit ambiguity withholds capability;
- exact replay never issues another capability or event;
- concurrent claimants enter the executor exactly once;
- stale, yielded, terminal, substituted invocation, and substituted executor
  postures fail before dispatch;
- the complete closed result and persistence-fault matrix remains intact;
- admission does not fabricate workflow completion;
- continuity result, yield, and recovery writes require the reservation
  binding;
- legacy yield, outcome, and recovery request commitments remain unchanged
  when no dispatch reservation binding is present;
- runtime event ordering and rehydrated projection remain deterministic;
- schema upgrades are explicit through V5; and
- existing continuity and SQLite conformance suites remain green.

## 10. Validation

Passed during implementation:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p workflow-core --lib` (`347 passed`)
- focused operational-opening and supervisor module tests (`23 passed`)
- `cargo test -p workflow-core --test authorized_execution_continuity --test authorized_execution_continuity_state_contract --test sqlite_state_backend` (`36 passed`)
- `npm run check:docs`
- `git diff --check`

`cargo test --workspace` was started with the normal test profile and again
with test-profile debug information disabled. Every completed suite passed,
including CLI unit and integration coverage, all examples, Core library,
adapters, hook contracts, approval presentation, audit projection,
capability/authority, continuity, durable state, evidence, and GitHub adapter
coverage. On this local macOS host, launching each integration-test binary
still took multiple minutes; even the debug-stripped retry required 56 minutes
to rebuild and retained per-binary launch latency. Both commands were stopped
cleanly without a reported failure. The pull-request CI workspace test remains
the authoritative complete workspace proof and must pass before merge.

## 11. Remaining Known Limitations

- Reservation is private, one-shot, local, and SQLite-only.
- An admitted capability lost before executor entry is not reassigned.
- There is no lease, scheduler, repeated dispatch, queue, or host recovery
  loop.
- PostgreSQL and filesystem parity are deferred.
- No public caller can select or configure this path.
- No provider or sandbox executes through this boundary.
- Runtime event vocabulary is additive and experimental until the surrounding
  private execution topology is reviewed.

## 12. Recommended Next Phase

Perform a **focused maintainer/security review of the atomic dispatch
reservation implementation**. Review the one-winner transaction, replay and
ambiguity posture, event/projection integrity, reservation-bound downstream
mutations, migration, privacy, and regression matrix before any repeated
supervisor loop or scheduler work.

## 13. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1790909269542533000-2`
- approval: `approval/run-1790909269542533000-2/implementation-approved`
- presentation: `presentation/53672a0b9e8c2a6f`
- approval outcome: granted by delegated maintainer with persisted
  presentation proof
- phase status: completed
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, and six successful skill
  invocations; no retries or escalations
- validation summary: formatting, workspace clippy, Core unit and focused
  integration suites, docs checks, and diff hygiene passed; the complete
  workspace test remains required in pull-request CI because local per-binary
  launch latency prevented a practical complete run
- approved boundary: private SQLite atomic dispatch reservation and exact
  trusted-host supervisor integration only
- out-of-kernel execution: source edits and validation commands are performed
  by the trusted local development host; Workflow OS governs the phase but
  does not edit repository files or execute shell commands
