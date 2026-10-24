# Trusted-Host Input Observer Freshness Authority Fix Review

## 1. Executive Verdict

Accepted; proceed to the fixture-backed in-memory observer proof.

The freshness-authority fix closes the final Core-model blocker. Request and
result values now bind the same explicit ordering authority, define every
compared revision in that authority's domain, reject authority substitution,
and commit the authority into both canonical fingerprints.

## 2. Scope Verification

The fix stayed within the approved model-only scope. It added no callable
observer, host inspection, filesystem or environment access, process
execution, provider integration, runtime admission, persistence, workflow
events, schemas, CLI behavior, examples, writes, capability grants, or release
changes.

## 3. Shared Ordering-Authority Assessment

`TrustedHostInputObservationRequest` and
`TrustedHostInputObservationResult` each carry an `ordering_authority`
commitment. The model defines static-discovery completion, observation start,
and observation completion revisions in that one shared domain.

This resolves the prior semantic defect. The implementation no longer infers
temporal order by comparing unrelated producer-local counters.

## 4. Request And Result Binding Assessment

Result-to-request validation requires exact ordering-authority equality before
performing the revision comparison. It then requires observation start to be
strictly after static-discovery completion. Existing operation, static-result,
repository, request, producer, and security-requirement bindings remain in
force.

The authority is a request-bound claim, not reusable execution authority.
Later provider review must determine which ordering authorities and producer
identities are acceptable for live observation.

## 5. Canonical Commitment Assessment

Both canonical encodings include the ordering-authority commitment. Changing
only that authority changes the request and result fingerprints. The updated
pinned vectors make omission or accidental removal detectable.

The serialization boundary requires the field and continues to deny unknown
fields. Missing authority and authority substitution fail closed.

## 6. Privacy And Redaction Assessment

The authority is represented as a fixed-size content commitment. It contains
no host path, user name, command, clock payload, environment value, source
content, provider response, credential, or token. Debug output redacts it, and
validation or deserialization errors do not echo it.

## 7. Test Quality Assessment

Focused tests cover request and result commitment sensitivity, authority
substitution, missing serialized authority, and stale observation. Existing
tests continue to cover producer substitution, fact provenance, closed reason
codes, posture/method coherence, strict wire behavior, serde revalidation,
deterministic ordering, and non-leakage.

The complete workspace validation matrix passed on reviewed commit
`fd086834`. Opt-in live tests remained ignored behind their existing gates.

## 8. Blockers

None.

## 9. Non-Blocking Follow-Ups

- The fixture proof should construct equivalent semantic results twice from
  explicit synthetic facts and prove identical commitments.
- Later provider review must establish which producer and ordering-authority
  identities can support live admission.
- The model still does not create, operate, or persist an ordering authority.
- Live host observation and static-plus-host admission composition remain
  separately planned work.

## 10. Recommended Next Phase

Implement the fixture-backed in-memory observer proof already sequenced in the
provider-neutral observer plan. The proof should use an explicit synthetic
fact source and existing validated constructors only. It must not inspect the
host, execute a process, select or call a provider, persist observations,
admit runtime work, add schemas or CLI behavior, enable writes, or broaden
authority.

## 11. Validation

- `cargo fmt --all --check`: passed on reviewed commit `fd086834`.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed on reviewed
  commit `fd086834`.
- `cargo test --workspace`: passed on reviewed commit `fd086834`; opt-in live
  tests remained ignored behind their existing environment gates.
- `npm run check:docs`: passed after this review update.
- `git diff --check`: passed after this review update.

## 12. Governed Review Evidence

- Workflow ID: `dg/review`.
- Run ID: `run-1792838672049879000-2`.
- Approval ID:
  `approval/run-1792838672049879000-2/review-scope-approved`.
- Approval presentation ID: `presentation/378db3836f3b96e7`.
- Approval presentation content hash:
  `378db3836f3b96e74e66950238cbdd6c5d4707b0c26e910a354e8134db27fb6d`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact focused review scope.
- Phase status: `Completed`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, six scheduled steps, six successful skill invocations, zero
  retries, and zero escalations. Approval presentation enforcement was
  proof-enforced and its event marker was present.
- Out-of-kernel work: Codex inspected the implementation, tests, plans, and
  reports and authored this review. The kernel governed scope and approval but
  did not perform the review or edit files.
