# Runtime Proportional-Governance Assessment Binding Blocker Fix Report

## 1. Executive Summary

The assessment-binding model now proves that the accepted assessment set was
derived from the exact immutable run bundle supplied to binding construction.
Same workflow and run IDs are no longer sufficient when bundle identity or root
differs.

## 2. Blocker Fixed

Initial review showed that two valid stores could contain different bundle
roots under identical workflow/run IDs. Because the assessment set retained
only those IDs and its aggregate fingerprint, construction could combine the
first bundle's fingerprint with the second bundle's root.

`ImmutableBundleGovernanceAssessmentSet` now retains the exact
`ImmutableRunBundleBinding` used during derivation.
`GovernanceAssessmentBinding::from_assessment_set` requires that binding to
equal the supplied stored bundle's binding before it copies either root or
fingerprint.

## 3. Integrity Boundary

The fix compares bundle ID, bundle version, and root hash through exact typed
equality. It does not infer equivalence from workflow/run identity and it does
not recompute or weaken the accepted aggregate fingerprint algorithm.

The assessment set's `Debug` posture redacts the retained bundle binding. The
stable failure uses
`governance.proportional.assessment_binding.bundle_mismatch` and does not echo
IDs, roots, fingerprints, paths, definitions, or runtime facts.

## 4. Compatibility

The assessment set remains in-memory and has no deserialization contract. Its
existing aggregate fingerprint algorithm and stable known vector are unchanged.
The binding, event, snapshot, and audit serialization shapes are unchanged.

## 5. Test Coverage

Focused coverage constructs two separately valid stored bundles with the same
workflow and run IDs but different definition roots. It proves:

- the bundle bindings differ;
- cross-bundle binding construction fails closed;
- the stable mismatch code is returned;
- the error does not expose workflow/run identity or aggregate fingerprint;
- valid same-bundle construction and existing event/audit behavior still pass.

## 6. Validation Commands

- Focused assessment-binding, runtime-event, and audit tests: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; opt-in live integration tests remained
  intentionally ignored.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 7. Scope Explicitly Not Added

No executor integration, automatic event emission, persistence or store change,
retry/resume reassessment, schema, CLI, UI, provider call, provider write,
automatic approval, enterprise administration, or default behavior was added.

## 8. Remaining Limitations

- Runtime facts still have no trusted freshness proof.
- No executor establishes the binding before `RunCreated`.
- No executor emits or enforces the event.
- Retry and approval resume reassessment remain future work.

## 9. Recommended Next Phase

Perform a focused re-review of the exact bundle/set integrity fix. Only after
acceptance should one explicit opt-in executor binding path begin.

## 10. Governed Phase Record

- Dogfood workflow: `dg/blocker`
- Run ID: `run-1784498620488952000-2`
- Approval ID: `approval/run-1784498620488952000-2/fix-approved`
- Approval outcome: granted with persisted presentation proof
- Out-of-kernel work: Rust model/test edits, focused validation, documentation
  updates, diff inspection, and blocker-report drafting
