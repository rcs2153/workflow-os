# Collaborative Identity Core Model Blocker Fix Report

## 1. Executive Summary

The collaborative identity authority-boundary blocker is fixed. Public callers
can validate that an assembled identity, grant, revocation, scope, revision,
and time claim is internally coherent, but the resulting value is explicitly
non-authorizing and is not represented as trusted current authority.

## 2. Blocker Fixed

The original public projection function accepted caller-constructed facts,
caller-supplied authority revision, and caller-supplied evaluation time, then
returned a type named and documented as trusted current authority. Validation
could establish coherence but not authoritative provenance, fact completeness,
or trusted time.

## 3. Implementation Approach

- Replaced `project_collaborative_current_authority` with
  `validate_collaborative_authority_projection_claim`.
- Replaced trusted-looking projection types with
  `CollaborativeAuthorityProjectionClaim` and
  `CollaborativeAuthorityProjectionClaimInput`.
- Added `CollaborativeAuthorityProjectionClaimPosture::NonAuthorizing` as a
  fixed serialized field.
- Replaced the permission-sounding `allows` method with descriptive
  `contains_capability` membership.
- Renamed the wire shape and commitment vocabulary around projection claims.
- Added no compatibility aliases that would preserve the misleading trusted
  authority API.

## 4. Validation Boundary Summary

Existing fail-closed coherence checks remain intact: lifecycle, exact scope,
revision ordering, grant uniqueness, revocation integrity, effective/expiry
time, and at least one active exact-project grant. These checks validate an
asserted fact set only. They do not establish that the facts are authoritative,
complete, current, or evaluated using trusted time.

## 5. Authority And Privacy Summary

The claim cannot authorize runtime work. It has no trusted conversion and no
runtime consumer. Serialization identifies it as `non_authorizing`; ordinary
deserialization produces only the unverified wire shape. Redacted Debug and
stable non-leaking errors remain unchanged.

No raw provider subject, token, credential, email, username, group payload, or
external assertion was added.

## 6. Test Coverage Summary

Focused coverage confirms the fixed non-authorizing posture, descriptive
capability membership, serialized posture, unverified wire decoding, and all
existing lifecycle, canonicalization, scope, revocation, ambiguity, and
non-leakage behavior.

## 7. Commands Run And Results

- Focused collaborative identity tests: passed, 11 tests.
- Focused collaborative identity clippy: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: broad regression evidence passed through all 475
  Core unit tests, all 343 local-executor integration tests, CLI tests, adapter
  tests, and the collaborative-identity suite. Two earlier runs each exposed a
  different existing live-time trusted-host scheduling failure; every failing
  test passed in isolation and the complete Core unit suite passed serially and
  on the final parallel run. The final broad run was terminated by the desktop
  execution-session limit after the local-executor suite, with no assertion
  failure. This phase does not claim a clean uninterrupted workspace command.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Remaining Known Limitations

- No Core-owned authoritative identity/grant source exists.
- No proof of complete current fact coverage exists.
- No trusted-time boundary exists for authority projection.
- No identity persistence, protocol transport, hosted authentication cutover,
  administration, or runtime authority consumption exists.

## 9. Recommended Next Phase

Collaborative Identity Core Model blocker-fix review.

Review that every public projection surface is explicitly non-authorizing and
that no old alias, serde path, or capability-membership method can confer
trusted authority. Do not begin persistence, external identity transport,
hosted cutover, provider writes, schemas, or administration before acceptance.

## 10. Governed Phase Evidence

- Dogfood workflow: `dg/blocker`.
- Run: `run-1792646791827320000-2`.
- Approval: `approval/run-1792646791827320000-2/fix-approved`.
- Approval presentation: `presentation/c6d928a5fe29ad21`.
- Approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof.
- Phase status: completed.
- Event summary: 39 events, including one approval request, one approval grant,
  six governed skill steps, zero retries, and zero escalations.
- Governance note: the original completed run's temporary event stream was
  incomplete at phase close. State was not repaired or fabricated; the same
  bounded close-out scope was started, proof-presented, approved, and completed
  in the replacement run recorded above.
- Out-of-kernel work: repository inspection, Rust and documentation edits,
  validation, and later git/PR actions are performed by the delegated
  maintainer; the kernel governs phase scope and records the approval/event
  trail.
