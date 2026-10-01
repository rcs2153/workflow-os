# Operational Execution Window Opening Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed with non-blocking follow-ups; proceed to the one-shot local
trusted-host supervisor implementation.**

The repaired opening boundary now binds the complete decision-relevant request,
checks the exact current run snapshot before mutation, validates durable event
projection reconstruction, proves one-winner cursor behavior, and never returns
an attempt-use capability from replay, reconciliation, rollback, or ambiguous
commit acknowledgement. Trusted-host work may consume this private capability
in the separately scoped next phase. This verdict does not authorize skill or
provider invocation by any other path.

## 2. Scope Verification

The fix stayed within the approved private, local, SQLite-only opening scope.
It did not add a supervisor, scheduler, model turn, skill invocation, provider
call, OpenShell integration, automatic approval, CLI or SDK surface, workflow
schema, example, hosted behavior, Reasoning Lineage, or release change. It did
not broaden the accepted five-operation continuity contract.

## 3. Original Blockers

The original review identified five blockers:

1. incomplete opening-request commitment;
2. no expected run-snapshot commitment;
3. projection deserialization that trusted caller-supplied derived values;
4. missing concurrent-opener and generic-event cursor-contention proof; and
5. missing opening-specific commit-fault and reconciliation proof.

All five are addressed sufficiently for the next private supervisor slice.

## 4. Request Commitment Assessment

Request commitment V2 binds operation and receipt IDs, workflow/run/step,
window and first-attempt IDs, subject actor, immutable bundle identity/version/
root, expected event cursor, expected snapshot commitment, expiry, attempt
budget, trusted-time observation/source/provenance/epoch, source authority,
Core governance, and exact operation binding.

The store recomputes this commitment before replay lookup. A changed field with
an unchanged commitment fails as a commitment mismatch; a recomputed request
that reuses an operation identity with changed content fails as an idempotency
conflict. Exact replay therefore requires exact canonical content.

## 5. Snapshot Binding Assessment

Core computes the expected snapshot commitment from the same serialized
`WorkflowRunSnapshot` representation and domain separator used by SQLite
snapshot persistence. The transaction rehydrates event history and compares
workflow/run identity, immutable bundle, event cursor, and the complete
snapshot commitment before any opening mutation.

Snapshot or cursor substitution returns the stable
`runtime_binding_stale` error without an operation, window, attempt,
projection, or event write.

## 6. Projection Integrity Assessment

Operational-opening event V2 deserializes through a private wire type and the
validated constructor. It requires the closed operation vocabulary, attempt
and revision one, a result sequence exactly one after the expected sequence,
the deterministic event ID, and the recomputed canonical projection
commitment. The commitment covers every disclosed event field and both cursor
identities and sequences. Unknown fields and internal tampering fail closed
without echoing supplied identifiers.

The event, not a caller-authored snapshot cache, remains the authoritative
projection input. The public projection snapshot still derives ordinary
Serde; independent snapshot-wire validation should be considered before that
cache is exposed as a standalone interchange or schema surface.

## 7. Concurrency Assessment

`BEGIN IMMEDIATE`, current event-history rehydration, exact cursor validation,
and uniqueness constraints compose correctly. Tests using separate SQLite
connections prove that two same-cursor openers produce exactly one winner and
one stable stale-binding loser. A generic runtime event that wins the next
cursor also makes the opening stale without partial opening state.

The durable winner consists of one opening operation, one execution window,
one first attempt, one projection binding, and one opening event.

## 8. Commit Ambiguity And Reconciliation Assessment

Pre-commit injected failure exits before `commit`; transaction drop rolls back
all opening, continuity, event, and snapshot writes. Commit errors map to a
stable ambiguity result before the capability-return branch. Simulated
acknowledgement loss after durable commit also returns ambiguity, and a fresh
connection reconciles the exact operation, receipt, request commitment,
projection event, and snapshot commitment.

No rollback, ambiguity, replay, or reconciliation path returns the private
attempt-use capability. Only a newly committed opening in the same call can
return it.

The test label `During` currently exercises the projected-operation
acknowledgement-ambiguity posture after SQLite commit, as does `After`; it does
not inject a distinct failure inside SQLite's commit implementation. This is a
non-blocking test-harness limitation because the real `commit()` error branch
already exits before capability construction and reconciliation is mandatory
for uncertain durability. The implementation report's wording should continue
to describe this as acknowledgement ambiguity, not proof of SQLite internals.

## 9. Privacy And Error Assessment

The boundary stores bounded identities, timestamps, revisions, and
commitments. It does not add prompts, transcripts, source or spec bodies,
command output, provider payloads, environment values, paths, credentials,
approval reasons, or evidence bodies. Debug implementations redact binding
material. Validation, stale-binding, idempotency, corruption, and ambiguity
errors remain stable and non-leaking.

## 10. Schema And Compatibility Assessment

The repaired V4 schema persists the expected snapshot commitment and constrains
the new operation record to V2. The schema checksum and manifest digest were
updated before this unreleased V4 boundary was accepted. Existing V1-V3
migration behavior is unchanged, and no migration invents opening authority
for historical state.

## 11. Test Quality Assessment

Focused tests cover complete request-field substitution families, expected
snapshot mismatch, projection-wire tampering, exact replay, fresh-connection
reconciliation, two-opener contention, generic-event contention, rollback,
ambiguous acknowledgement, privacy, and unsupported backend posture.

The coverage is sufficient for private supervisor consumption. Non-blocking
follow-ups are to distinguish the fault-injection naming more precisely and to
add independent projection-snapshot wire validation before any future public
interchange use.

## 12. Validation Reviewed

The fix report records:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- projection model tests: 4 passed;
- operational opening SQLite tests: 13 passed;
- SQLite backend integration tests: 17 passed;
- `npm run check:docs`: passed;
- `npm run check:integrations`: passed; and
- `git diff --check`: passed.

This review independently reran formatting, the 4 projection model tests, the
13 operational-opening SQLite tests, and
`cargo clippy -p workflow-core --all-targets -- -D warnings`; all passed.
The earlier workspace test attempt was stopped during prolonged silent
compilation without a failure, so this review does not claim a fresh complete
workspace test pass.

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- Rename or refine the opening fault harness if future tests need to
  distinguish pre-commit rollback from post-commit acknowledgement loss more
  precisely.
- Require validated deserialization for the projection snapshot before it is
  ever promoted from a derived cache to a standalone external interchange
  type.
- Keep filesystem and PostgreSQL opening support unavailable until each earns
  equivalent transactional and reconciliation proof.

## 15. Recommended Next Phase

Implement the **one-shot injected local trusted-host supervisor** from the
accepted plan. It may consume only the newly returned private opening
capability, invoke only the current step's registered local skill, and record
one bounded yield or outcome through the accepted continuity operations.

Do not add repeated scheduling, provider mutation, OpenShell, nested harnesses,
automatic approval, public runtime configuration, CLI exposure, or broad
dispatch in that phase.

## 16. Governed Review Record

- workflow: `dg/review`;
- run: `run-1790848614008784000-2`;
- approval: `approval/run-1790848614008784000-2/review-scope-approved`;
- presentation: `presentation/141fd37d3d0841a1`;
- presentation hash:
  `141fd37d3d0841a1a76f8833053ecc1fa5525146e29cede169dd1110fb08268f`;
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented;
- phase close: `Completed`, 39 events, 1 approval, 0 retries, and 0
  escalations, with the approval-presentation proof marker present; and
- out-of-kernel work: the external executor inspected source, tests, schema,
  reports, and roadmap, ran focused checks, and authored this review. The
  kernel governed the scope and approval but did not inspect code, edit files,
  run checks, or invoke work.
