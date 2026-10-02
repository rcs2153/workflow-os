# Operational Execution Window Opening Blocker Fix Report

## 1. Executive Summary

The operational execution-window opening blockers are fixed in the private,
local SQLite boundary. The repair completes the canonical opening request and
snapshot binding, validates serialized projection events by reconstruction,
and adds the missing contention and commit-fault proofs.

This phase does not invoke work. It does not add a trusted-host supervisor,
scheduler, skill dispatch, provider mutation, automatic approval, CLI or
schema surface, hosted behavior, or release posture change.

## 2. Blockers Fixed

The phase fixes the five blockers from the opening review:

1. incomplete request commitment;
2. missing expected snapshot commitment;
3. unvalidated serialized projection commitments;
4. missing concurrent-opener and generic-event cursor-contention proof; and
5. missing before/during/after commit-fault proof.

## 3. Complete Request Binding

Request commitment V2 binds every bounded decision-relevant opening input:

- operation and receipt identities;
- workflow, run, step, window, and first-attempt identities;
- subject actor;
- immutable run-bundle identity, version, and root commitment;
- expected event identity and sequence;
- expected run-snapshot commitment;
- expiry and maximum-attempt budget;
- trusted timestamp, source, provenance, and epoch;
- current source-authority commitment;
- Core governance commitment; and
- exact operation-binding commitment.

Exact replay therefore requires exact canonical content. A valid
same-operation request with changed bound content returns a stable
idempotency conflict. A request whose fields no longer match its supplied
commitment fails before replay lookup. Payloads are not copied into the
commitment or durable opening record.

## 4. Snapshot Integrity

Core derives the expected run-snapshot commitment from the same canonical JSON
and domain separation used by SQLite snapshot persistence. The opening request
binds and persists that commitment. The transaction rehydrates the current run
and requires workflow/run identity, immutable bundle, event cursor, and
snapshot commitment to match before mutation.

Snapshot substitution therefore fails closed with no opening operation,
window, attempt, projection binding, or event append.

## 5. Projection Integrity

Operational opening projection V2 no longer trusts caller-supplied derived
fields during deserialization. A private wire type is reconstructed through
the validated constructor. The boundary enforces:

- projection contract V2;
- the closed `invoke_current_step_skill` operation;
- first attempt number and first window revision equal to one;
- committed sequence exactly one greater than the expected sequence;
- deterministic committed event identity; and
- canonical projection commitment over every disclosed identity, binding,
  commitment, and cursor identity/sequence.

Tampered commitment, cursor, attempt, revision, binding, and event identity
fail closed with stable non-leaking errors.

## 6. Concurrency Proof

Focused tests use separate SQLite connections against one database and prove:

- two opening requests at the same current cursor produce exactly one durable
  opening winner;
- the loser receives a stable stale-binding or active-window conflict and
  writes no second opening state; and
- a generic runtime event that wins the shared next cursor causes the stale
  opening to fail without partial opening state.

The durable result contains one operation, one window, one first attempt, one
projection binding, and one opening event.

## 7. Commit-Fault And Reconciliation Proof

Opening-specific test-only fault injection covers before, during, and after
commit acknowledgement:

- before-commit failure rolls back all opening, continuity, event, and
  snapshot mutations and reconciles as confirmed absent;
- during/after acknowledgement ambiguity returns an error rather than an
  attempt-use capability; and
- reconciliation returns the exact durably committed operation, event,
  snapshot commitment, request commitment, and receipt after ambiguous
  acknowledgement.

No fault path returns operational authority when durability is uncertain.

## 8. Privacy And Error Posture

The repair remains commitment-first and payload-free. It does not store raw
prompts, source/spec bodies, command output, provider payloads, environment
values, paths, credentials, approval reasons, or evidence bodies. Debug output
continues to redact identifiers and binding material. Validation,
idempotency, stale-binding, and ambiguity errors use stable codes and do not
echo supplied values.

## 9. Test Coverage

Focused coverage includes:

- exact replay with complete content;
- receipt and authority/governance substitution conflicts;
- actor, immutable-bundle, cursor, snapshot, expiry, budget, and trusted-time
  substitutions;
- expected snapshot mismatch before mutation;
- projection wire tampering across derived relationships;
- exactly-one-winner concurrent opening;
- generic-event/opening cursor contention;
- before/during/after commit faults and fresh reconciliation; and
- existing opening, SQLite migration, runtime projection, privacy, and
  unsupported-backend behavior.

## 10. Validation

Validation results:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- operational opening projection model tests: passed, 4 tests;
- operational opening SQLite store tests: passed, 13 tests;
- `cargo test -p workflow-core --test sqlite_state_backend`: passed, 17 tests;
- `npm run check:docs`: passed;
- `npm run check:integrations`: passed, including GitHub, Jira, and CI adapter
  contracts plus all three read-only CLI examples; and
- `git diff --check`: passed.

`cargo test --workspace -j 2` was attempted. This host remained in silent
workspace compilation for more than sixteen minutes and the command was
stopped without a test failure. A complete workspace pass is therefore not
claimed. The changed Core model/store paths, SQLite schema contract, workspace
clippy targets, docs, and integration gate all completed successfully.

## 11. Remaining Limitations

- The opening capability remains private, local, SQLite-only, and
  opt-in/internal.
- Filesystem and PostgreSQL opening support do not exist.
- An opened first attempt cannot invoke, yield, resume, or complete work in
  this phase.
- No trusted-host supervisor, scheduler, daemon, sandbox, OpenShell adapter,
  provider call, CLI path, runtime config, schema exposure, or automatic
  approval exists.
- The opening first-attempt record remains distinct from accepted continuity
  attempts until a separately reviewed bridge is implemented.

## 12. Governed Phase Record

- workflow: `dg/blocker`;
- run: `run-1790841565536044000-2`;
- approval: `approval/run-1790841565536044000-2/fix-approved`;
- presentation: `presentation/abb2019d373bf429`;
- presentation hash:
  `abb2019d373bf429eb818a6e288cde1d63685b80a504d564e8abffa5516b37ee`;
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented; and
- phase close: `Completed`, 39 events, 1 approval, 0 retries, and 0
  escalations, with proof-enforced presentation marker present; and
- out-of-kernel work: the external executor edited source/docs, ran checks,
  and will commit the repair. The kernel governed scope and approval but did
  not edit files, run commands, or mutate a provider.

## 13. Recommended Next Phase

Perform a focused maintainer/security review of this blocker fix. Do not begin
trusted-host supervisor implementation unless that review accepts the complete
request/snapshot binding, projection reconstruction, contention proofs, and
commit-ambiguity posture.
