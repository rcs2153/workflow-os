# Canonical Local-Check Declaration Structural-Coverage Adapter Review

## 1. Executive Verdict

**Needs blocker fixes.**

The private adapter correctly establishes canonical stored provenance,
distinguishes authoritative empty sets from missing sources, and preserves the
existing unresolved path. One ambiguous-step invariant must be fixed before the
adapter can be accepted as authoritative.

## 2. Scope Verification

The phase stayed within its approved private-adapter scope. It did not add
aggregate governance posture, proportional-governance reassessment, executor
gates, check execution, handlers, inference, public APIs, schemas, CLI
behavior, events, persistence changes, evidence/report generation, providers,
SideEffect execution, writes, hosted behavior, or release changes.

## 3. Source Boundary Assessment

The adapter accepts `StoredImmutableRunBundle`, not arbitrary manifest or
record values. It derives workflow/run/bundle/step binding and requirement
facts from validated stored data. Legacy bundles fail with an authoritative
source-missing error. Canonical empty records remain distinct from source
absence.

The source posture participates in candidate identity, so caller-supplied
unresolved candidates cannot be relabeled as canonical by reusing the same
binding and obligations.

## 4. Completeness Assessment

The adapter requires declaration references and resolved records to cover the
set of step-scoped skill references and verifies exact reference-to-record
content-address matching.

One blocker remains: immutable-bundle definition validation rejects duplicate
reference tuples but permits two different skill identities to share one step
ID. The adapter currently collects expected step IDs into a set without first
checking that the set cardinality equals the number of skill references. A
crafted but otherwise valid stored manifest could therefore collapse an
ambiguous step binding and accept one declaration record for it.

## 5. Identity And Fingerprint Assessment

Canonical source posture, immutable bundle ID/version/root, workflow
ID/version, run ID, step ID, requirement fingerprints, and required/optional
levels all participate in deterministic candidate identity. Structural
coverage inherits the candidate fingerprint. This is appropriate and preserves
the earlier cross-bundle relabeling fix.

## 6. Privacy And Error Assessment

Errors use stable bounded codes and static messages. `Debug` output reveals
only source posture and counts while redacting identities and fingerprints.
The adapter does not ingest command text, raw output, source contents, paths,
provider payloads, credentials, or evidence bodies.

## 7. Test Assessment

Tests use real enriched and legacy bundles persisted through the create-only
store. They cover:

- canonical populated records;
- canonical empty records;
- required-level derivation;
- authoritative provenance propagation;
- authoritative/unresolved fingerprint separation;
- legacy rejection;
- unknown-step rejection; and
- error non-leakage.

The required blocker regression is missing: a stored manifest with two
different skill references bound to the same step must fail before candidate
construction.

Store tests already cover missing, corrupt, mismatched, and unreferenced
records. Direct duplicate-record construction is prevented by the validated
stored-bundle boundary.

## 8. Documentation Assessment

The roadmap, derivation plan, and phase report accurately state that the
adapter is private and does not yet create aggregate posture or executor
enforcement. The implementation report records the governed run and full
validation results.

## 9. Blocker

Reject ambiguous step-scoped skill bindings before authoritative adaptation:

1. count step-scoped skill references before deduplication;
2. compare that count with the unique expected-step set;
3. fail with a stable non-leaking duplicate-source code when they differ; and
4. add a focused regression proving two distinct skill references on one step
   cannot produce an authoritative candidate.

## 10. Non-Blocking Follow-Ups

- Keep defense-in-depth record/reference checks even though the store already
  validates normal construction.
- Consider shared test fixture utilities only when another phase needs the same
  immutable-bundle project setup; do not refactor solely for this review.

## 11. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 12. Governed Review Record

- workflow: `dg/review`
- run: `run-1785003987382552000-2`
- approval:
  `approval/run-1785003987382552000-2/review-scope-approved`
- presentation: `presentation/68c0999f656ca313`
- approval outcome: granted by delegated maintainer through proof enforcement
- review status: completed
- out-of-kernel work: source inspection, test review, documentation authoring,
  and validation commands

## 13. Recommended Next Phase

Execute the focused ambiguous-step blocker fix and re-review it. Do not begin
aggregate posture conversion or executor integration until the authoritative
adapter is accepted.

## 14. Fix-Forward Status

**Blocker fixed; phase accepted.**

The original verdict and finding remain preserved above. Governed re-review
confirmed that:

- step-scoped skill references are counted before set construction;
- the adapter rejects a cardinality mismatch before any step deduplication;
- the failure uses the stable
  `local_check_attestation.structural_coverage.authoritative_source_duplicate`
  code without identity or payload values;
- a focused stored-bundle fixture with two distinct skill identities bound to
  one step fails before candidate construction; and
- all existing authoritative, legacy, privacy, and unresolved behavior remains
  unchanged.

No blocker remains in this private adapter phase. Aggregate posture conversion
and executor integration remain separately governed.

## 15. Fix Re-Review Record

- workflow: `dg/review`
- run: `run-1785006304863217000-2`
- approval:
  `approval/run-1785006304863217000-2/review-scope-approved`
- presentation: `presentation/7e0a6d9ad2b7fa42`
- approval outcome: granted by delegated maintainer through proof enforcement
- event summary: 39 events, one approval, zero retries, zero escalations
- validation: focused tests, formatting, clippy, workspace tests, docs check,
  and diff check passed
- out-of-kernel work: source inspection, fixture review, test execution,
  documentation updates, and validation commands

## 16. Final Recommendation

Proceed to planning the aggregate evidence/check posture conversion from the
accepted authoritative structural-coverage result. Keep executor integration,
check execution, handler defaults, and proportional-governance reassessment
out of that planning phase unless separately approved.
