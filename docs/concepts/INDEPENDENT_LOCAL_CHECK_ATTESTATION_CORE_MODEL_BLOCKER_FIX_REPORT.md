# Independent Local Check Attestation Core Model Blocker Fix Report

## 1. Executive Summary

The independent local check attestation model's deterministic-binding blockers
are fixed. Canonical proof identity no longer depends on a caller-selected
attestation record ID, and every candidate now commits a deterministic complete
requirement fingerprint.

The model remains explicitly unverified. No verifier or runtime behavior was
added.

## 2. Blockers Fixed

### Record Identity Separation

`attestation_id` remains a bounded record identity but is excluded from the v1
binding fingerprint. Two otherwise identical candidates with different record
IDs now produce the same proof fingerprint.

### Complete Requirement Commitment

`LocalCheckAttestationRequirement` now computes and serializes a deterministic
requirement fingerprint over:

- command identity;
- minimum assurance;
- canonical accepted statuses;
- freshness mode and maximum age;
- exact immutable-run binding requirement; and
- truncation policy.

The candidate binding stores and fingerprints that complete requirement
fingerprint. Requirement serde recomputes and verifies it, failing closed on
tampering.

## 3. Determinism Boundary

Accepted status order is canonicalized before fingerprinting, so equivalent
sets have the same identity. Every valid requirement-field change alters the
requirement fingerprint. Invalid assurance weakening or removal of exact bundle
binding remains rejected rather than receiving an alternate fingerprint.

Stable known vectors cover both requirement and candidate binding algorithms.

## 4. Privacy And Authority

The fix adds hashes only. It does not add raw command data, output, paths,
environment values, source, credentials, or provider payloads. Requirement
fingerprints are redacted in `Debug`. Candidate posture remains `unverified`,
including when the caller claims kernel-observed source vocabulary.

## 5. Scope Explicitly Not Added

No verifier, process observation, check execution, registration, persistence,
events, executor enforcement, evidence/report integration, schema, CLI, UI,
provider call, write, hosted behavior, or release change was added.

## 6. Test Coverage

Tests now prove:

- caller-selected record IDs do not change canonical proof identity;
- command, statuses, freshness, and truncation alter requirement identity;
- equivalent status ordering canonicalizes to one fingerprint;
- caller, mock, and external assurance cannot weaken the v0 requirement;
- exact immutable-run binding cannot be disabled;
- stable requirement and binding vectors are fixed; and
- existing tamper, serde, privacy, freshness, and unverified-posture tests pass.

## 7. Validation Commands

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

All listed commands passed.

## 8. Remaining Limitations

The model still cannot prove an observation. The command-contract fingerprint,
handler fingerprint, source, assurance, timestamps, result, and requirement
binding remain candidate inputs until a pure verifier compares them with
kernel-owned facts and immutable definitions.

## 9. Recommended Next Phase

Perform a focused blocker-fix review. If accepted, proceed to planning and then
implementation of the pure verifier only.

## 10. Governed Phase Record

- Workflow: `dg/blocker`
- Run ID: `run-1784509718462033000-2`
- Approval ID: `approval/run-1784509718462033000-2/fix-approved`
- Presentation ID: `presentation/896f8155982b486a`
- Approval outcome: granted with persisted presentation proof
- Out-of-kernel work: Rust model and test edits, documentation edits, and local
  validation commands
