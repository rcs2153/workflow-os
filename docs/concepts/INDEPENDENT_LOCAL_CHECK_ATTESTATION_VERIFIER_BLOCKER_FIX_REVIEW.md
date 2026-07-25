# Independent Local Check Attestation Verifier Blocker Fix Review

## 1. Executive Verdict

Blocker fixed; proceed to one opt-in `DocsCheck` runtime-composition planning
phase.

## 2. Scope Verification

The fix stayed within the approved verifier-boundary scope. It changed the
immutable-bundle verification input, derived the trusted binding from the
validated stored manifest, extended focused tests, and updated status docs.

It did not add check execution, executor integration, persistence changes,
events, audit projection, evidence, reports, artifacts, schemas, SDK or CLI
behavior, providers, SideEffects, writes, hosted behavior, or release changes.

## 3. Original Blocker

The verifier accepted `ImmutableRunBundleBinding`, which contains bundle ID,
version, and root hash. Matching that reference across supplied inputs proved
agreement but did not prove that a complete stored manifest and every canonical
definition record had been resolved and validated.

Accepted independent proof must not be issued over an unresolved root-only
assertion.

## 4. Fix Assessment

The verifier now accepts `StoredImmutableRunBundle`. That type has private
fields, no public constructor, and no deserialization implementation. The local
immutable-bundle store returns it only after validating manifest identity,
canonical record resolution, content addresses, record-set completeness, and
bundle integrity.

The verifier derives `manifest().run_binding()` once and compares the
pre-execution binding, candidate, and Core-owned observation to that exact
value. The accepted record stores the derived binding. This is minimal,
idiomatic, and preserves a clear source-of-truth boundary.

## 5. Validation And Failure Assessment

Mismatched stored, execution, candidate, or observation bundle context returns
the stable `local_check_attestation.verify.bundle_mismatch` family. Errors do
not include bundle identifiers, roots, paths, records, or supplied values. The
function returns no partial accepted record.

Store validation and verifier validation remain separate. The verifier does not
reread files or trust a caller-recomputed root.

## 6. Privacy And Redaction Assessment

No raw stdout, stderr, command arguments, paths, environment values, source
contents, credentials, tokens, provider payloads, or free-form claims were
added. Debug behavior remains redacted. Accepted proof remains payload-free and
non-serializable.

## 7. Test Quality Assessment

Focused coverage constructs a real project bundle, writes it through the
create-only local store, reads a validated `StoredImmutableRunBundle`, and uses
that value for successful proof. It verifies canonical records are present and
the accepted binding came from the manifest.

A second independently valid stored bundle with changed canonical workflow
content is rejected. Existing observation substitution, assurance, result,
policy, freshness-boundary, truncation, stable-vector, and Debug non-leakage
coverage remains green.

Rust privacy enforces that unresolved or manually assembled stored bundles
cannot enter this API. No new compile-fail dependency is justified for this
focused fix.

## 8. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed with no failures and one intentional opt-in
  live sandbox test ignored.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Runtime composition must create the immutable execution binding before
  process execution and construct the observation only from Core-owned results.
- Later consumers must reevaluate freshness at time of use.
- Stronger handler implementation provenance remains a future assurance tier.
- Attestation persistence, event/audit projection, evidence/report use, and
  proportional-governance fact composition require separate phases.
- The dogfood phase-close approval-presentation list-cap defect remains open.

## 11. Governed Review

- workflow: `dg/review`
- run: `run-1784521538794406000-2`
- approval: `approval/run-1784521538794406000-2/review-scope-approved`
- presentation: `presentation/63b67b34fef99b3b`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; code inspection,
  documentation, and validation ran outside the kernel

## 12. Recommended Next Phase

Plan one explicit opt-in `DocsCheck` runtime-composition path. The plan must
bind command, handler, policy, and immutable stored run context before
execution; derive observation from Core-owned structured process results; and
invoke the accepted verifier afterward.

Do not enable automatic checks or add persistence, events, schemas, CLI,
evidence/report attachment, broader providers, SideEffects, writes, hosted
behavior, or release changes.
