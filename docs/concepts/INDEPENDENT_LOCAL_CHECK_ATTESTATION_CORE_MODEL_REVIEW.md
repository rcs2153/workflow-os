# Independent Local Check Attestation Core Model Review

## 1. Executive Verdict

Needs blocker fixes.

The model preserves the most important authority boundary: every publicly
constructed or deserialized binding remains explicitly `unverified`, and mock
or caller assurance cannot define an independent requirement. Two deterministic
binding defects must be fixed before a verifier may consume the model.

## 2. Scope Verification

The implementation stayed within model-only scope. It did not add a verifier,
check execution, handler registration, persistence, events, executor
enforcement, schema, CLI, UI, provider behavior, writes, cryptographic claims,
hosted behavior, or release changes.

## 3. Model And Authority Assessment

The model is appropriately isolated and domain-neutral. Requirement, assurance,
source, freshness, candidate binding, and verification posture are distinct.
`LocalCheckAttestationVerificationPosture` has only `Unverified`, so neither a
constructor nor serde can create accepted proof.

`CallerAsserted`, `MockObserved`, and future `ExternalVerifier` assurance are
rejected as minimum independent requirements. A caller may construct a
kernel-observed-shaped candidate, but the candidate remains unverified. The
`eligible_for_v0_verification` API is documented as eligibility only and grants
no authority.

## 4. Validation Assessment

Validation correctly enforces:

- bounded candidate identifiers;
- kernel-observed minimum assurance for independent requirements;
- non-empty unique accepted statuses;
- exact immutable-run binding requirements;
- bounded freshness;
- aligned source and assurance vocabulary;
- aligned result and exit-code posture;
- ordered observation timestamps;
- recomputed fingerprint equality during deserialization;
- fixed non-leaking validation and serde errors.

## 5. Privacy And Serde Assessment

The binding stores typed references, hashes, timestamps, and bounded posture,
not raw output, summaries, arguments, paths, environment values, source
contents, credentials, or provider payloads. Manual `Debug` redacts identities,
hashes, and timestamps. Invalid verification posture and fingerprint tampering
fail closed without echoing supplied values.

## 6. Test Quality Assessment

Focused tests cover valid requirement serde, caller/mock insufficiency,
explicitly unverified construction, candidate eligibility vocabulary,
fingerprint tampering, a stable fingerprint vector, decision-field sensitivity,
payload exclusion, non-leaking serde errors, source/assurance mismatch,
observation ordering, and freshness bounds. Workspace lint and tests pass.

The current fingerprint-sensitivity test changes one representative field. The
blocker fix should add table-driven coverage for every canonical field after
the corrected fingerprint contract is frozen.

## 7. Blockers

### 7.1 Caller-Chosen ID Changes Proof Identity

`compute_binding_fingerprint` includes `attestation_id`. That ID is supplied by
the caller and is not an observation or requirement fact. Two candidates with
identical command, immutable bundle, invocation, handler, result, timestamps,
and freshness can therefore produce different fingerprints solely because the
caller selected a different record ID.

This violates the plan's deterministic invariant and would weaken exact
idempotency and conflict detection. The canonical proof fingerprint must omit
the record ID, or the ID must be deterministically derived from the corrected
fingerprint rather than independently supplied.

### 7.2 Binding Does Not Commit Complete Requirement Identity

The candidate binding includes freshness but does not include a stable
requirement ID or fingerprint. It therefore does not commit the required
assurance, accepted result statuses, exact immutable-binding requirement, and
truncation policy as one requirement identity.

A later verifier could compare a requirement separately, but an accepted
record or restart would not durably prove which complete requirement was
verified. Add a deterministic payload-free requirement fingerprint and bind it
into the candidate fingerprint before verifier work.

## 8. Non-Blocking Follow-Ups

- Keep result duration outside canonical identity until a concrete gate needs
  it.
- Add a canonical command-contract fingerprint helper in the verifier phase or
  a separately reviewed prerequisite; do not copy command text into records.
- Keep handler identity and observation source caller-supplied and untrusted
  until the verifier receives kernel-owned facts.

## 9. Recommended Next Phase

Implement a narrow core-model blocker fix:

1. add a deterministic requirement fingerprint covering every requirement
   field;
2. bind that fingerprint into the candidate;
3. remove caller-chosen attestation ID from canonical proof identity or derive
   it deterministically;
4. add known-vector and all-field sensitivity tests;
5. retain the explicitly unverified posture.

Do not implement the verifier, runtime execution, persistence, events,
executor enforcement, schemas, CLI, providers, or writes in that fix.

## 10. Governed Review Record

- Workflow: `dg/review`
- Run ID: `run-1784509573734485000-2`
- Approval ID: `approval/run-1784509573734485000-2/review-scope-approved`
- Presentation ID: `presentation/8ed64963d8e82c4f`
- Approval outcome: granted with persisted presentation proof
- Out-of-kernel work: source and test inspection, maintainer reasoning,
  documentation edits, and validation commands

## 11. Fix-Forward Note

The two blockers were addressed in the subsequent focused blocker-fix phase:
the complete requirement now has a deterministic fingerprint committed by the
candidate binding, and caller-chosen attestation record identity no longer
participates in canonical proof identity. The original findings remain the
review record; the fix still requires focused re-review before verifier work.
