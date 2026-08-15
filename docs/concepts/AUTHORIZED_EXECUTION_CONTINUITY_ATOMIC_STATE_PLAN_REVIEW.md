# Authorized Execution Continuity Atomic State Plan Review

## 1. Executive Verdict

**Plan accepted; proceed to atomic state contract and reference conformance
implementation.**

The plan is phase-ready after fix-forward hardening. It defines the exact
authoritative records, state machines, operation commitments, replay order,
atomic write sets, crash posture, attempt-use boundary, wake-source boundary,
trusted-time watermark, backend support posture, and test-only initialization
needed for the first implementation.

## 2. Scope Verification

The plan remains planning-only. It does not authorize:

- runtime scheduling or automatic model turns;
- executor or handler invocation;
- runtime events or new `WorkflowRunStatus` variants;
- delegated approval or model self-approval;
- provider mutation or external writes;
- SQLite or PostgreSQL schema changes;
- filesystem atomicity claims;
- CLI, workflow schema, SDK, example, or release changes;
- hosted workers or nested harness execution.

## 3. Authoritative State Assessment

The plan correctly separates public non-authoritative continuity models from
Core-owned coordination state. It defines authoritative window, yield, wait,
directive, attempt, operation-history, trusted-time, event-tail, and revision
records. Serialized orientation, host identity, final responses, and delivery
acknowledgements cannot grant authority.

Snapshots remain derived caches. Exact event sequence plus event ID and
immutable run binding control cursor validity.

## 4. Attempt Lifecycle Assessment

An attempt is one bounded executor entry or turn. The accepted lifecycle is:

```text
prepared -> started -> yielded
                    -> succeeded
                    -> retryable_failure
                    -> terminal_failure
                    -> ambiguous_may_have_started
```

The directive-consumption transaction records `started` before consumer entry
and returns a private, non-cloneable, non-serializable attempt-use capability.
Only that live consumer can record yield or an ordinary terminal outcome.
Recovery without it can record only ambiguity. A yielded attempt cannot also
receive another outcome.

This closes the unsafe crash-after-claim and fabricated-outcome gaps without
claiming exactly-once external execution.

## 5. Atomic Operation Assessment

The plan defines four narrow operation families:

1. register yield and optional waits at one exact cursor;
2. transition one versioned wait from a verified wake source;
3. consume one current directive and atomically start one attempt; and
4. reconcile one attempt outcome, with a separate ambiguity-only recovery
   path.

Each operation binds an exact expected cursor and revisions, validates current
authoritative state, updates all participating continuity records, updates the
trusted-time watermark, and records one payload-free receipt/event projection
in one transaction.

## 6. Replay And Conflict Assessment

Every operation has a domain-separated operation ID and canonical commitment
covering all semantic inputs, expected revisions, generated identities,
trusted-time observation, and requested outcome. Exact committed replay returns
the original result even after later state advances. Same ID with a different
commitment conflicts. An absent ID proceeds to locked/current-state validation.

Storage transaction retries reuse all generated identities and observations.
Semantic stale-state failures are not retried against newly read values.

## 7. Wait And Wake Assessment

Wait satisfaction is no longer caller-asserted. It requires either a private
Core wake-assessment capability or an exact durable wake record that the
transaction reloads and verifies. The store validates exact binding and
persists source commitment/revision; it does not claim to evaluate policy.

Satisfying a wait permits only fresh reassessment. It does not create
authority, consume a directive, or resume execution.

## 8. Trusted-Time Assessment

Every authoritative window carries a trusted-time watermark. Every mutation
requires a non-regressing authoritative observation and updates the watermark.
Equality at expiry fails closed. PostgreSQL later uses database time; the
reference store and SQLite use a Core-owned injected clock. Unavailable or
regressed time cannot extend eligibility.

## 9. Backend And Compatibility Assessment

Continuity uses a separate versioned capability contract rather than silently
expanding `DurableStateContractVersion::V1`. Every existing backend must first
declare all continuity operations unsupported.

- in-memory reference: test-only support after conformance;
- local filesystem: unsupported;
- SQLite: unsupported until a later schema/transaction phase;
- PostgreSQL: unsupported until explicit serializable transactions and
  dedicated conformance.

This avoids PostgreSQL's current blanket generic transaction declaration from
accidentally claiming a newly added capability.

## 10. Conformance Assessment

The test-only fixture bootstrap is unavailable through the production trait.
Consume-path tests begin from an exact fixture-seeded yielded window, prior
yielded attempt, active yield generation, wait records, cursor, revisions,
operation history, and trusted-time watermark. It cannot prove that a legal
runtime window opened.

The planned suite covers concurrency, exact/conflicting replay, cursor and
revision races, wait/wake races, attempt-budget contention, one-winner
consumption, capability misuse, ambiguity recovery, crash injection,
unsupported backend zero-write behavior, trusted-time rollback, rehydration,
and redaction.

## 11. Privacy And Redaction Assessment

Storage is limited to bounded IDs, hashes, revisions, enums, timestamps, and
stable references. Raw prompts, transcripts, source contents, command output,
provider payloads, environment values, credentials, and serialized bearer
authority are forbidden. Debug, serde, database, conflict, and delivery errors
must remain non-leaking.

## 12. Blockers

No planning blockers remain.

The following are implementation gates:

- no backend may advertise support without the dedicated executable suite;
- runtime event integration waits for an accepted durable backend;
- executor/supervisor integration waits for atomic attempt start and outcome;
- delegated approval waits for its separate authority-hardening lane.

## 13. Non-Blocking Follow-Ups

- Resolve SQLite monotonic trusted-time behavior before SQLite implementation.
- Choose the private receipt representation used before public event
  vocabulary exists.
- Select the first trusted current-authority source for the later supervisor
  proof.
- Preserve explicit unsupported behavior for the local filesystem backend.

## 14. Validation

- independent state-backend/code-path inspection: completed;
- independent concurrency/crash/privacy threat model: completed;
- `npm run check:docs`: passed;
- `git diff --check`: passed.

## 15. Recommended Next Phase

Implement the **Authorized Execution Continuity atomic state contract and
reference conformance slice only**.

Add the separately versioned capability contract, private capability trait,
authoritative request/result and lifecycle models, explicit unsupported
declarations for existing backends, test-only bootstrap, reference store, and
executable conformance suite. Do not add backend schemas, runtime events,
executor invocation, scheduling, automatic approval, provider writes, CLI
behavior, or nested harness runtime.
