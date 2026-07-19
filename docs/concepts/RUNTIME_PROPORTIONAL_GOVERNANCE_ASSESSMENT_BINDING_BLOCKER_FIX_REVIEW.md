# Runtime Proportional-Governance Assessment Binding Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; proceed to explicit opt-in executor assessment-binding
integration.**

The fix closes the cross-bundle integrity gap without changing the accepted
assessment fingerprint algorithm, event shape, snapshot compatibility, or
runtime defaults.

## 2. Original Blocker

The initial binding constructor compared only workflow and run identity between
the supplied stored immutable bundle and the assessment set. Two separately
valid stores could therefore reuse those IDs while carrying different bundle
roots, allowing a binding to combine one bundle's root with another bundle's
assessment fingerprint.

That violated the model's central integrity claim: an accepted assessment set
must be attributable to the exact immutable definitions that were assessed.

## 3. Fix Assessment

`ImmutableBundleGovernanceAssessmentSet` now retains the exact
`ImmutableRunBundleBinding` used during assessment derivation.
`GovernanceAssessmentBinding::from_assessment_set` requires exact typed equality
between that retained binding and the supplied stored bundle before constructing
the durable binding.

The equality covers bundle identity, bundle version, and integrity root. The
constructor copies the retained assessment-set binding only after equality is
proven. Workflow and run identity checks remain additive defenses rather than a
substitute for bundle integrity.

The approach is narrow and idiomatic. It strengthens the existing trusted
construction boundary instead of adding a second fingerprint, recomputing
assessment output, or broadening runtime behavior.

## 4. Compatibility Assessment

The stable assessment-set algorithm identifier and known aggregate fingerprint
vector are unchanged. The assessment set remains an in-memory validated value
without a deserialization contract. The durable binding, runtime event,
snapshot, and audit serialization shapes are unchanged from the reviewed phase.

Existing streams and legacy snapshots without the optional binding remain
readable. No workflow spec schema was changed.

## 5. Privacy And Error Assessment

The retained bundle binding remains redacted in assessment-set `Debug` output.
Mismatch returns the stable code
`governance.proportional.assessment_binding.bundle_mismatch` without echoing
workflow IDs, run IDs, bundle IDs, integrity roots, assessment fingerprints,
paths, definitions, runtime facts, or secret-like values.

No raw definitions, provider payloads, command output, parser payloads, source
content, environment values, credentials, or tokens were introduced.

## 6. Regression Assessment

The focused regression builds two valid stores with the same workflow and run
IDs but different immutable definition roots. It proves that:

- the resulting immutable bundle bindings differ;
- cross-bundle binding construction fails closed;
- the stable mismatch code is returned;
- the error does not disclose identity or fingerprint material;
- same-bundle construction remains valid;
- event, snapshot, audit, and legacy compatibility behavior remains intact.

## 7. Validation Results

- Focused assessment-binding, runtime-event, and audit tests: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; opt-in live integration tests remained
  intentionally ignored.
- `npm run check:docs`: passed before this review artifact and is rerun at phase
  close.
- `git diff --check`: passed before this review artifact and is rerun at phase
  close.

## 8. Scope Verification

The fix did not add executor integration, automatic event emission,
persistence-before-run behavior, retry or approval-resume reassessment, schema
or CLI behavior, UI, provider calls, provider writes, automatic approval,
enterprise administration, or default changes.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- The future executor must derive the binding from a freshly assessed immutable
  bundle rather than treating an arbitrary deserialized binding as proof.
- The future persistence boundary must establish the accepted binding before
  `RunCreated`; `GovernanceAssessmentBound` remains a projection of that fact.
- Runtime-fact freshness and retry/approval-resume reassessment remain separate
  reviewed phases.

## 11. Recommended Next Phase

Implement one explicit opt-in executor path that derives and establishes the
accepted assessment binding before `RunCreated`, then emits the bounded event
projection. Existing executor defaults, retry/resume behavior, schemas, CLI,
UI, provider calls, writes, automatic approvals, and enterprise administration
must remain unchanged.

## 12. Governed Review Record

- Dogfood workflow: `dg/review`
- Run ID: `run-1784500772367391000-2`
- Approval ID:
  `approval/run-1784500772367391000-2/review-scope-approved`
- Approval presentation ID: `presentation/8bf56dabdae4c126`
- Approval outcome: granted with persisted presentation proof
- Phase status: completed
- Out-of-kernel work: source and regression inspection, compatibility and
  privacy review, validation review, and review-document drafting
