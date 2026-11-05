# WorkReport Body Commitment Recovery And Reconciliation Review

## Verdict

Accepted for the bounded existing recovery and duplicate-reconciliation scope.

The implementation makes validated commitment posture a precondition for exact
duplicate equality across the identified Core consumers without changing their
public result vocabulary or broadening into replay.

## Reviewed Properties

1. Candidate artifacts validate before duplicate reconciliation.
2. Stored artifacts validate before content equality.
3. Exact equality is accepted only after both validations pass.
4. Invalid, missing, or unreadable stored artifacts use existing bounded
   unreadable or ambiguous outcomes.
5. Valid unequal artifacts remain explicit conflicts.
6. Trusted-host, authoritative-governance, selected project-validation, and
   governance authority-receipt reconciliation all use the shared helper.
7. Direct existing-authoritative-artifact recovery validates before claiming
   `AlreadyPersisted`.
8. Valid legacy artifacts remain `legacy_unverified` and are not upgraded.
9. Errors and debug output remain payload-free.

## Boundary Assessment

The shared helper is accepted as the correct defense-in-depth boundary. The
`WorkReportArtifactStore` trait permits injected implementations and does not
encode the built-in backends' stronger read guarantee in its type. Every Core
consumer that makes an equality-based governance decision must therefore
validate explicitly rather than trust the implementation by convention.

Preserving caller-specific error postures is also accepted. A corrupt result
cannot prove a conflict or equality, so ambiguous/unreadable is more accurate
than relabeling it as a valid conflicting duplicate.

## Residual Risks

- Transaction-local PostgreSQL hosted replay reads still require explicit
  retained-commitment enforcement.
- Other terminal/provider replay routes require a complete call-graph review.
- Legacy artifacts remain readable but unverified; no migration policy exists.
- A storage owner can still rewrite both report body and envelope commitment.
- Independent security review remains required before release admission.

## Validation Reviewed

- focused invalid injected-store duplicate test;
- existing exact-duplicate and conflict regression tests;
- focused direct recovery commitment-mismatch test;
- focused legacy-unverified recovery test;
- formatter and warnings-denied Clippy;
- full affected core suite;
- documentation checks; and
- diff whitespace validation.

## Next Phase

Proceed only to bounded terminal and provider replay enforcement. Do not combine
that work with migration, backfill, signing, witnesses, provider mutation
broadening, publication, or broader product claims.
