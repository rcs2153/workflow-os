# Independent Local Check Attestation Stored Manifest Identity Blocker Fix Review

## 1. Executive Verdict

Blocker fixed; resume the explicit in-memory `DocsCheck` attestation
runtime-composition implementation.

## 2. Scope Verification

The fix stayed within the verifier identity boundary. It added one stored
manifest comparison, one focused regression, and status documentation.

It did not add check composition, execution, defaults, persistence, events,
evidence, reports, artifacts, schemas, CLI, providers, SideEffects, writes,
hosted behavior, or release changes.

## 3. Source-Of-Truth Assessment

`StoredImmutableRunBundle::manifest()` is now the source of truth for:

- bundle ID, version, and root through `run_binding()`;
- workflow identity; and
- run identity.

The execution binding must match all of those values. Candidate and observation
must continue matching the execution binding. Agreement among caller-supplied
inputs is no longer sufficient to relabel a stored bundle.

## 4. Failure And Privacy Assessment

Mismatch returns the stable
`local_check_attestation.verify.bundle_mismatch` error and no partial accepted
proof. The error does not disclose bundle, workflow, run, root, path, command,
or payload values.

No Debug or serialization surface changed.

## 5. Test Quality Assessment

The new regression changes workflow and run identity consistently across the
execution binding, candidate, and observation while retaining the exact valid
stored root. Verification fails against the stored manifest identity.

This directly protects the previously missing invariant rather than relying on
single-input mismatch tests. Existing verifier and workspace tests remain
green.

## 6. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 7. Blockers

None.

## 8. Non-Blocking Follow-Ups

- Stronger handler implementation provenance remains future work.
- Accepted-proof persistence, event/audit projection, evidence/reports, and
  freshness consumption remain separate phases.
- The dogfood presentation-record close cap remains open.

## 9. Governed Review

- workflow: `dg/review`
- run: `run-1784617003942111000-2`
- approval: `approval/run-1784617003942111000-2/review-scope-approved`
- presentation: `presentation/edcee4ef5a355283`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; review, documentation, and
  validation ran outside the kernel

## 10. Recommended Next Phase

Resume the accepted `DocsCheck` runtime-composition implementation. Keep the
helper explicit, in-memory, and isolated from executor defaults, persistence,
events, evidence, reports, schemas, CLI, providers, SideEffects, and writes.
