# Local Unsigned Authority Receipt Report

## 1. Executive Summary

The first local unsigned `AuthorityReceipt` core model is implemented. It
defines a trusted payload-free receipt vocabulary without adding a production
producer.

The receipt is payload-free, point-in-time only, and explicitly
non-authorizing. The trusted type is serialize-only. Serialized input becomes
an explicitly unverified claim with no conversion into trusted evidence.

## 2. Scope Completed

- Added deterministic receipt identity, version, source, freshness, validity,
  signature, effect, redaction, and claim-verification postures.
- Bound exact immutable execution identity, required-context identity,
  capability, sensitivity boundary, resource commitment, selected grant, and
  source commitments.
- Added a serialize-only trusted type and a separate deserializable
  `UnverifiedAuthorityReceipt`.
- Added safe serde, redacted `Debug`, stable errors, and fixed V1 commitment
  framing.
- Added focused determinism, unverified-claim, tamper, and privacy tests.

## 3. Scope Explicitly Not Completed

No production receipt issuance, runtime authorization, replayable authority,
operation-outcome binding, dereference, execution, OpenShell, provider
integration, access material, SideEffects, writes, persistence, events,
schemas, CLI, hosted administration, signatures, or release changes were
added.

## 4. Model And Trust Boundary

`AuthorityReceipt` is publicly inspectable and serializable. It has no public
or crate-visible production constructor.

`UnverifiedAuthorityReceipt` is the only deserializable wire type. It validates
deterministic field consistency but explicitly does not authenticate source
provenance. There is no conversion from an unverified claim into a trusted
receipt.

An independent review found that the initial source-backed draft allowed a
caller to reconstruct a self-consistent unsigned commitment and that
crate-visible issuance did not prove a concrete operation succeeded. The
source integration was removed. Future issuance must be owned by one concrete
Core read-only operation and occur only after successful outcome.

## 5. Validation And Privacy

The receipt commitment covers every stored field with versioned,
length-framed SHA-256 hashing. Receipt identity is derived from that
commitment. Unverified-claim deserialization rejects unknown fields and
validates both commitment and identity.

The model stores no raw resource reference, source record, context payload,
policy or approval body, evidence body, check output, command, path,
environment value, credential, or provider response. `Debug` redacts all
identities, timestamps, and commitments.

Validation proves internal consistency only. It does not make a serialized
claim trusted. The fixed `evidence_only_not_authorization` effect prevents the
model from claiming fresh or reusable permission.

## 6. Test Coverage

Focused tests cover:

- deterministic trusted-model identity through a private test fixture;
- a fixed V1 receipt commitment vector;
- trusted serialization into an explicitly unverified claim;
- a different self-consistent claim remaining unverified;
- substituted grant failure without value leakage; and
- payload-field and `Debug` non-leakage.

## 7. Validation

Validation results:

- six focused authority-receipt tests: passed;
- all 209 `workflow-core` unit tests: passed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed; and
- canonical GitHub CI: required before merge.

## 8. Governed Phase Record

- workflow ID: `dg/implement`;
- run ID: `run-1785417096382389000-2`;
- approval ID:
  `approval/run-1785417096382389000-2/implementation-approved`;
- presentation ID: `presentation/739c56ff4340a055`;
- approval outcome: granted under delegated-maintainer authority; and
- approval-presentation enforcement: proof persisted before execution.

Phase close completed with 39 events, one approval, zero retries, and zero
escalations.

Repository edits, shell commands, validation, and later git/PR operations are
out-of-kernel executor work and are disclosed rather than represented as
kernel-executed activity.

## 9. Remaining Limitations

The receipt has no production producer and is not signed, durable,
replay-protected, cross-service verifiable, or accepted by any execution
provider. It does not cite independently evaluated policy, approval, evidence,
or check prerequisites. Operation-outcome binding and optional-gap receipts
remain deferred.

OpenShell may later be evaluated as an optional sandboxed execution substrate.
It is not an authority source, and this receipt is not an OpenShell credential.

## 10. Recommended Next Phase

Perform focused blocker-fix review of the trusted-versus-unverified boundary,
then consider one opt-in Core-owned read-only receipt production path that
issues only after successful operation outcome.

Do not broaden provider mutations first.
