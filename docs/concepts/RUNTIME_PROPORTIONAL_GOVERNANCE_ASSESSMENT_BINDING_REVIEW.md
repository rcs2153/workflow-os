# Runtime Proportional-Governance Assessment Binding Review

## 1. Executive Verdict

**Needs blocker fixes.**

The model, event ordering, snapshot compatibility, audit projection, privacy
posture, and test quality are otherwise appropriately narrow. One missing
cross-object integrity check permits a binding to combine an assessment set
from one immutable bundle with the identity/root of another bundle when both
bundles reuse the same workflow and run IDs.

## 2. Findings

### Blocker: Assessment Set Is Not Bound To The Supplied Bundle

`GovernanceAssessmentBinding::from_assessment_set` checks only workflow and run
identity between the supplied `StoredImmutableRunBundle` and
`ImmutableBundleGovernanceAssessmentSet`. The assessment set does not retain or
expose the immutable bundle binding from which it was derived.

Two valid stores can contain different immutable bundle roots under the same
workflow and run IDs. Passing the assessment set from the first store with the
bundle from the second store therefore succeeds. The resulting binding records
the second bundle root with the first bundle's aggregate fingerprint. That
breaks the core claim that one accepted fingerprint is bound to the immutable
definitions it assessed.

Required fix:

- bind the assessment set to the exact `ImmutableRunBundleBinding` used during
  derivation;
- require exact equality in `from_assessment_set`;
- include that binding in redaction-safe `Debug` posture without exposing it;
- prove cross-store same-run/different-root mismatch rejection;
- retain the existing aggregate fingerprint vector unless the accepted
  fingerprint algorithm intentionally changes.

## 3. Scope Verification

The phase stayed within model/event scope. It did not add executor integration,
automatic event emission, persistence-before-run behavior, retry/resume
reassessment, schema or CLI behavior, UI, provider calls, provider writes,
automatic approval, enterprise administration, or default changes.

## 4. Model Assessment

The binding is otherwise minimal and domain-neutral. It records model and
algorithm version, workflow/run identity, immutable bundle identity/root,
aggregate fingerprint, bounded step count, strictest independent execution and
disclosure axes, and completeness. Construction derives posture rather than
accepting caller summaries.

Custom deserialization rejects unknown versions/algorithms/completeness values,
zero or oversized step sets, and blocking or denied execution paired with quiet
disclosure. Errors are stable and bounded.

## 5. Event And Runtime Assessment

`GovernanceAssessmentBound` is additive, requires an idempotency key, allows
exactly one binding, requires exact workflow/run/bundle identity, and is valid
only while the run remains `Created`. Snapshot retention is optional and
serde-defaulted, preserving legacy readability.

The docs correctly state that the event records an already-established binding
and does not itself establish pre-`RunCreated` durability. No executor emits or
enforces it.

## 6. Audit And Privacy Assessment

The audit projection includes only execution, disclosure, completeness, and
step count in bounded decision context. Binding identifiers and fingerprints
are not copied into that context and are marked reference-only. `Debug` output
redacts workflow/run identity, bundle identity/root, and aggregate fingerprint.

No raw definitions, evidence, check output, provider payloads, command output,
parser payloads, source content, paths, environment values, credentials, or
tokens are introduced.

## 7. Compatibility Assessment

The event and snapshot changes are additive. Existing event streams and legacy
snapshots without the optional binding remain readable. The assessment-set
algorithm was promoted from a private string constant to typed vocabulary
without changing its stable identifier or aggregate fingerprint framing.

## 8. Test Quality Assessment

Current tests cover valid construction, serde round trip, model invariants,
Debug/error non-leakage, idempotency, exact event identity, duplicate rejection,
pre-validation retention, legacy snapshot compatibility, and bounded audit
projection.

Missing blocker coverage is the exact cross-store same-workflow/same-run but
different-bundle mismatch described above.

## 9. Validation Results

- Focused assessment-binding, runtime-event, and audit tests: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed after one
  test-fixture `panic!` was replaced with the repository-approved expectation
  pattern.
- `cargo test --workspace`: passed; opt-in live tests remained intentionally
  ignored.
- `npm run check:docs`: passed before this review artifact and must be rerun.
- `git diff --check`: passed before this review artifact and must be rerun.

## 10. Blockers

1. Bind `ImmutableBundleGovernanceAssessmentSet` to the exact immutable bundle
   identity/root and reject mismatched bundle/set construction.

## 11. Non-Blocking Follow-Ups

- The future executor must construct bindings from freshly derived sets rather
  than treating arbitrary deserialized bindings as proof.
- The future persistence boundary must establish the accepted binding before
  `RunCreated`; the additive event remains a projection of that fact.
- Retry and approval-resume freshness remain separately reviewed later phases.

## 12. Recommended Next Phase

Run a narrow assessment-binding integrity blocker fix. Do not begin executor
integration until focused re-review accepts exact bundle/set binding.

## 13. Governed Review Record

- Dogfood workflow: `dg/review`
- Run ID: `run-1784498414466158000-2`
- Approval ID: `approval/run-1784498414466158000-2/review-scope-approved`
- Approval outcome: granted with persisted presentation proof
- Out-of-kernel work: source/test/diff inspection, trust-boundary analysis,
  validation review, and review-document drafting

## 14. Fix-Forward Note

The blocker fix retains the exact immutable bundle binding in every assessment
set and requires equality with the supplied stored bundle before constructing a
binding. A cross-store same-workflow/same-run but different-root regression now
fails with the stable `assessment_binding.bundle_mismatch` code. This note does
not erase the original finding. Focused re-review accepts the fix; see
[Runtime Proportional-Governance Assessment Binding Blocker Fix Review](RUNTIME_PROPORTIONAL_GOVERNANCE_ASSESSMENT_BINDING_BLOCKER_FIX_REVIEW.md).
