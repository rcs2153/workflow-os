# Collaborative Identity Core Model Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; proceed to authoritative collaborative identity source-boundary planning.**

The public caller-supplied projection surface is now explicitly non-authorizing.
No public alias, serde path, capability helper, or runtime consumer restores the
original trusted-current-authority claim.

## 2. Scope Verification

The fix stayed within the approved model-only boundary. It renamed and demoted
the projection API, added a fixed posture, changed commitment vocabulary,
updated focused tests, and corrected documentation. It did not add identity
persistence, an authoritative registry source, trusted time, runtime authority
consumption, OIDC, provider integration, hosted cutover, schemas, CLI behavior,
writes, or a new capability family.

## 3. Original Blocker

The original public function accepted caller-assembled provider, binding,
principal, grant, revocation, revision, and time values, then returned a type
named and documented as trusted current authority. Its validation established
internal coherence, but could not establish authoritative provenance, complete
current fact coverage, or trusted time.

## 4. Fix Approach Assessment

The fix takes the smallest safe approach: it demotes the result to
`CollaborativeAuthorityProjectionClaim`, renames the constructor to
`validate_collaborative_authority_projection_claim`, and fixes its posture to
`CollaborativeAuthorityProjectionClaimPosture::NonAuthorizing`.

This is idiomatic and appropriately conservative. It preserves useful
canonicalization and coherence validation without pretending that caller-owned
facts are an authorization decision.

## 5. Authority Boundary Assessment

- The verified claim is serialize-only and has private fields.
- The deserializable wire type is explicitly unverified and has no conversion
  into the verified claim or runtime authority.
- The fixed `non_authorizing` posture is serialized and included in the
  canonical commitment under a new claim-specific domain.
- `contains_capability` describes claim membership; it does not imply
  permission.
- No runtime consumer accepts the claim as authority.
- The old trusted-looking types and constructor are not exported or retained as
  compatibility aliases.

The boundary now matches what the model can prove.

## 6. Validation And Error Assessment

Existing fail-closed checks remain intact for lifecycle, identity/scope
matching, revision order, time order, duplicate grants, revocation integrity,
active grant presence, and bounded collections. Stable error codes remain
unchanged. Error messages are bounded and do not include caller values.

## 7. Privacy And Serde Assessment

No raw provider subject, email, username, token, credential, group payload, or
external assertion is stored. Debug output remains redacted. Serialization
exposes bounded opaque identities and the non-authorizing posture. Ordinary
deserialization cannot reconstruct a verified or authorizing value.

## 8. Test Quality Assessment

Focused tests cover the fixed posture, descriptive capability membership,
serialized posture, unverified decoding, active exact-project grant selection,
revocation, lifecycle, stale revision, cross-scope rejection, ambiguity,
canonicalization, and secret-like input non-leakage.

A future authoritative resolver will need separate tests for Core-owned fact
provenance, complete fact-set proof, trusted time, rollback protection,
concurrent revocation, and runtime rechecks. Those are correctly deferred.

## 9. Documentation Review

The roadmap, lifecycle plan, implementation report, original review, and
blocker-fix report now distinguish internally coherent claims from trusted
authority. The original blocker finding remains preserved with a fix-forward
note rather than being rewritten away.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Define a Core-owned authoritative source interface and complete-current-fact
  contract before producing any authorizing projection.
- Define trusted database time, monotonic authority revisions, rollback
  protection, and concurrent revocation semantics.
- Add persistence and restart tests before runtime authority consumption.
- Keep OIDC assertion validation, SCIM, groups, administration, and hosted
  authentication cutover in separately reviewed phases.

## 12. Validation

- Focused collaborative identity tests: passed, 11 tests.
- Focused collaborative identity clippy: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Broad workspace regression evidence passed all 475 Core unit tests, all 343
  local-executor integration tests, CLI tests, adapter tests, and the focused
  collaborative identity suite. Two earlier broad runs each exposed a
  different existing live-time trusted-host scheduling failure that passed in
  isolation; the final broad run was terminated by the desktop execution
  session after the local-executor suite. A clean uninterrupted
  `cargo test --workspace` result is therefore not claimed by this review.

## 13. Recommended Next Phase

Authoritative collaborative identity source-boundary planning.

Define the smallest Core-owned source boundary for complete provider, binding,
principal, grant, and revocation facts, trusted time, monotonic revision, and
rollback protection. Do not begin protocol transport, hosted authentication
cutover, administration, broad provider writes, or production identity claims.

## 14. Governed Phase Evidence

- Dogfood workflow: `dg/review`.
- Run: `run-1792646823958482000-2`.
- Approval: `approval/run-1792646823958482000-2/review-scope-approved`.
- Approval presentation: `presentation/e7f6a094a90bab9f`.
- Approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof.
- Phase status: completed.
- Event summary: 39 events, including one approval request, one approval grant,
  six governed skill steps, zero retries, and zero escalations.
- Out-of-kernel work: code and documentation inspection, searches, review
  writing, validation, and later git/PR actions are performed by the delegated
  maintainer; the kernel governs scope and records the approval/event trail.
