# Collaborative Approval Authority Receipt Review

## 1. Executive Verdict

**Needs blocker fixes before Phase 3.**

The receipt model and persistence boundary are appropriately narrow,
payload-free, and non-authorizing. The blocker is verification, not a model
redesign: required CI proves the PostgreSQL v4 migration but does not execute
the receipt write, exact replay, recovery, conflict, or corruption scenario.
Restart and backup/restore preservation of receipt rows are also not proved.

## 2. Scope Verification

The implementation stayed within Phase 2. It added the receipt model, an
unverified persisted-claim boundary, additive PostgreSQL persistence, tests,
and documentation.

It did not add an approval consumer, executor integration, run resumption,
authority locking, hosted authentication, identity administration, provider
writes, workflow schemas, CLI behavior, SDK behavior, examples, or release
posture changes.

## 3. Trust Boundary Assessment

`CollaborativeApprovalAuthorityReceipt` is serialize-only and can be created
only from crate-private proof material. Serialized data can construct only
`UnverifiedCollaborativeApprovalAuthorityReceipt`. The persisted wrapper keeps
that unverified posture and cannot restore producer trust or authorize a later
action.

The fixed capability is `ApprovalDecide`; effect is
`evidence_only_not_authorization`; validity is `point_in_time_only`. No public
constructor accepts caller-assembled serialized receipt claims as trusted
evidence.

## 4. Identity And Commitment Assessment

The receipt commitment binds version, approval reference, decision-event
reference, fixed capability, source and source-model version, exact query,
authority revision and commitment, source snapshot, active grant, observation
time, decision kind, effect, and validity. Receipt identity is derived from the
complete commitment.

Deserialization revalidates both the commitment and deterministic identity.
Unknown enum variants and unknown record fields fail closed. The receipt ID
format is bounded and validated without echoing rejected values.

## 5. Privacy And Redaction Assessment

The model stores references and commitments rather than provider assertions,
external subjects, issuer or audience data, raw grants, reasons, presentation
content, workflow content, credentials, or tokens. `Debug` redacts receipt,
approval, event, source, revision, commitment, and observation values.

Serde and corruption errors are generic. Focused model tests verify forbidden
serialized field names and secret-like tamper non-leakage.

## 6. Persistence Assessment

PostgreSQL v4 adds a dedicated create-only table with a receipt primary key and
unique approval and decision-event bindings. First write, exact byte replay,
exact-ID read, and exact decision recovery are represented. Relational columns,
payload hash, canonical receipt commitment, and deterministic identity are
checked on read.

Conflict and corruption paths fail closed. One non-blocking diagnostic issue
remains: a replay against a row whose canonical payload was corrupted is
classified as conflict before the read decoder can classify corruption. This
does not accept or authorize the row, but the blocker-fix phase should add an
explicit regression test and preserve the most accurate stable failure class.

## 7. Migration And Recovery Assessment

The v4 migration is additive and the required Shared PostgreSQL State CI job
proved schema migration and receipt-table creation. The v1-to-v4 fixture also
proves preservation of an earlier generic record.

The dedicated v3-to-v4 fixture contains only schema metadata, so it does not
prove preservation of representative v3 authority data. More importantly, no
required CI path writes and recovers a receipt after backend reopen or includes
a receipt in the PostgreSQL backup/restore rehearsal.

## 8. Test Quality Assessment

Model tests are substantive and cover trusted-to-unverified round trip,
commitment validation, tampering, Debug redaction, and forbidden payload
fields.

The receipt persistence scenario is currently an environment-gated unit test
inside `postgres_state.rs`. The required Shared PostgreSQL State job runs only
the external exact test
`postgresql_backend_proves_shared_state_milestone`. That external scenario
checks v4 migration and table inventory but never calls the receipt store. The
ordinary Rust job has no PostgreSQL URL, so the internal receipt test returns
through its skip branch. Therefore PR #591's green PostgreSQL check is not live
proof of receipt write/replay/recovery/corruption behavior.

## 9. Documentation Assessment

The implementation report accurately describes the model boundary and local
environment limitation, but initially called Shared PostgreSQL CI the live
receipt persistence proof. That claim is corrected on this review branch. The
roadmap and consumer plan must keep Phase 3 blocked until executable receipt
conformance and recovery evidence passes.

## 10. Blockers

1. Make the receipt write, exact replay, decision recovery, conflict, and
   corruption scenario execute in the required Shared PostgreSQL State CI job.
2. Prove restart recovery through a reopened backend and preserve at least one
   receipt through the PostgreSQL backup/restore rehearsal.
3. Ensure the required test fails rather than skips when CI database
   configuration is absent.

These are evidence blockers. They do not authorize a broader receipt model,
consumer, executor, authentication, or provider-write change.

## 11. Non-Blocking Follow-Ups

- Distinguish corrupted durable replay from an ordinary binding conflict and
  add an explicit non-leaking regression test.
- Strengthen the direct v3-to-v4 fixture with representative v3 authority data
  if future migration work makes that fixture the primary preservation proof.
- Keep trusted receipt construction private until the reviewed same-call
  consumer can provide proof inside the decision transaction.

## 12. Recommended Next Phase

Implement a **collaborative approval authority receipt PostgreSQL verification
blocker fix** only. After its required live CI and recovery rehearsal pass,
perform a focused blocker-fix review. Do not begin the Phase 3 same-call
`ApprovalDecide` consumer before acceptance.

## 13. Validation

- PR #591 required checks: all seven passed after a transient npm registry 503
  was retried without code changes.
- Shared PostgreSQL State: passed, proving v4 migration/table creation but not
  receipt operations.
- `cargo fmt --all --check`: passed in the review phase.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed in the
  review phase and CI.
- `cargo test --workspace`: passed in the review phase and CI; the
  environment-gated receipt PostgreSQL test skipped outside a configured
  database environment.
- `npm run check:docs`: passed in the review phase.
- `git diff --check`: passed in the review phase.

## 14. Governed Phase Evidence

- Dogfood workflow: `dg/review`.
- Run: `run-1792655417360723000-2`.
- Approval: `approval/run-1792655417360723000-2/review-scope-approved`.
- Approval presentation: `presentation/9a8200eb4854df99`.
- Approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof.
- Phase status: `Completed`.
- Event summary: 39 events, one approval, zero retries, and zero escalations.
- Out-of-kernel work: repository inspection, CI evidence review, review
  writing, validation, git, and PR actions are performed by the delegated
  maintainer; Workflow OS governs phase scope and records the phase trail.

## 15. Fix-Forward Status

The narrow PostgreSQL verification blocker fix is implemented on a later
branch without erasing this review finding. It adds the exact receipt behavior
test to the required Shared PostgreSQL State job, makes required database
configuration absence fail closed, proves recovery through a reopened backend,
preserves and recovers a receipt through the existing logical backup/restore
rehearsal, and classifies corrupted exact replay as corruption. Phase 3 remains
blocked until that fix receives focused review and required CI passes.
