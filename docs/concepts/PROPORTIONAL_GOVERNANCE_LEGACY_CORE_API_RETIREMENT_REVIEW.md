# Proportional-Governance Legacy Core API Retirement Review

## 1. Executive Verdict

Phase accepted; proceed to the next runtime-composition phase after merge.

## 2. Scope Verification

The implementation stayed within the approved compatibility boundary. It did
not change CLI behavior, selected-path semantics, schemas, providers,
OpenShell, SideEffect execution, writes, hosted behavior, release posture, or
ordinary undeclared workflows.

## 3. Authority Boundary Assessment

The old public request allowed callers to author authority and other
route-driving facts. That type is now crate-private, its fields are
crate-private, and its explicit-fact route bridge is crate-private. The public
selected consumer derives route material from validated activation, immutable
input, a Core-owned source, and the same-call local check.

This closes the alternate authority model identified by the compatibility
review.

## 4. Public Surface Assessment

Crate-root inspection confirms that no retired function or request type is
re-exported. The selected and fact-free Core-owned functions and types remain
available. Shared result types remain public because supported APIs return
them; they do not accept caller-authored authority facts.

## 5. Enforcement And Regression Assessment

The implementation retained common private route preparation and consumption.
The supported integration tests still cover quiet, visible, approval, denial,
same-call checks, drift, reports, and artifact gates. Historical tests whose
only purpose was exercising the external caller-fact contract were removed
with that contract.

The retained focused executor suite passed with 338 tests and one ignored
opt-in live test. The full workspace matrix also passed. This provides direct
evidence that supported quiet, visible, approval, denial, retry, report,
artifact, ordinary executor, CLI, hosted, and provider paths remain intact.

## 6. Privacy And Failure Assessment

No new runtime errors or output paths were added. Existing errors remain
stable and bounded. External legacy consumers encounter compile-time removal,
not a permissive compatibility fallback.

## 7. Compatibility Assessment

The repository remains on preview version `0.2.0-preview.1`, no production
repository call sites used the removed API, and the migration target is
documented. This is an appropriate preview correction.

## 8. Blockers

None.

## 9. Non-Blocking Follow-Ups

- Add a stable embedding boundary only when registered current sources or
  scoped capability grants can supply authority.
- Broaden selected local-check profiles only after source and attestation
  review.

## 10. Recommended Next Phase

Return to roadmap runtime composition after merge. Do not reopen a public
caller-fact compatibility path.

## 11. Validation

The review consumed the implementation report's completed local command
matrix. Formatting, warnings-as-errors clippy, the full Rust workspace, docs,
integrations, dogfood helper, TypeScript SDK, contracts, and diff checks all
passed.

## 12. Governed Review Record

- Workflow: `dg/review`
- Run: `run-1786563980091071000-2`
- Approval:
  `approval/run-1786563980091071000-2/review-scope-approved`
- Presentation: `presentation/38c3e09675f6e62f`
- Approval outcome: granted by the delegated maintainer through the exact
  proof-enforced approval command
- Kernel boundary: the kernel governed review scope, approval, and durable
  events; source inspection, validation, documentation, and Git/PR work
  remained outside the kernel
