# Local Unsigned Authority Receipt Plan

## 1. Executive Summary

Workflow OS resolves exact current authority from registered source facts
inside a private same-call use boundary. This phase adds the first payload-free
`AuthorityReceipt` vocabulary without adding a production producer.

The trusted model can represent that one required governed-context obligation
was satisfied by a fresh, gap-free assessment. It is local, unsigned,
point-in-time only, and explicitly non-authorizing. Serialized input is always
an unverified claim and cannot become trusted evidence through deterministic
self-consistency alone.

## 2. Goals

- Add deterministic receipt identity and versioning.
- Bind the immutable workflow/run/step/actor/harness execution identity.
- Bind one required-context requirement, capability, resource commitment, and
  selected source-backed grant.
- Bind source snapshot, fact-set, and assessment commitments.
- Keep trusted construction unavailable until one concrete Core-owned
  operation can prove both current authority and successful operation outcome.
- Deserialize wire input only into an explicitly unverified claim.
- Preserve safe serde, redacted `Debug`, stable errors, and deterministic
  hashing.

## 3. Non-Goals

This phase does not add:

- production receipt issuance;
- runtime authorization or reusable bearer authority;
- target dereference, tool loading, skill execution, or provider calls;
- OpenShell or another execution provider;
- access material, credentials, environment values, or raw payloads;
- SideEffect execution or external writes;
- persistence, events, audit projection, schemas, CLI, SDK, or UI behavior;
- cryptographic signatures, notarization, cross-service verification, or
  replay prevention; or
- hosted administration, enterprise identity, or release changes.

## 4. Model

The model includes:

- `AuthorityReceipt`;
- `UnverifiedAuthorityReceipt`;
- `AuthorityReceiptId`;
- `AuthorityReceiptVersion`;
- `AuthorityReceiptSourceKind`;
- `AuthorityReceiptFreshnessPosture`;
- `AuthorityReceiptClaimVerificationPosture`;
- `AuthorityReceiptValidity`;
- `AuthorityReceiptSignaturePosture`;
- `AuthorityReceiptEffect`; and
- `AuthorityReceiptRedactionPosture`.

The V1 fixed postures are:

- source: `registered_current_authority_resolution_v1`;
- freshness: `fresh_at_issuance`;
- validity: `point_in_time_only`;
- signature: `local_unsigned`;
- effect: `evidence_only_not_authorization`; and
- redaction: `reference_only`.

## 5. Trust And Future Issuance Boundary

No public or crate-visible production constructor exists in this phase.
`AuthorityReceipt` is serialize-only. Deserialization produces
`UnverifiedAuthorityReceipt`, which has no conversion into the trusted type.

A future Core-owned producer must issue a trusted receipt only after:

- the registered current-authority assessment is `Ready`;
- its reason vector is exactly `Ready`;
- required-context consumption is `Satisfied`;
- the selected requirement is required and satisfied;
- the retained source resolution is `Authorized` and `Available`;
- the resolution reason is exactly `ActiveGrantMatched`;
- one selected grant exists;
- actor, workflow, run, step, harness, resource, capability, and evaluation
  time match the immutable execution boundary; and
- one concrete read-only operation succeeds with a bounded outcome that can be
  committed into the receipt or an adjacent result.

The future producer must not return a trusted receipt on failed or ambiguous
operation outcomes. Optional-gap issuance remains deferred.

## 6. Receipt Contents

The receipt stores typed identities and commitments only:

- receipt identity and version;
- execution-binding hash and exact execution identities;
- required-context contract hash and requirement identity;
- target kind, access level, capability, and sensitivity boundary;
- resource kind and a domain-separated resource-scope commitment;
- selected grant identity;
- source snapshot, fact-set, and assessment commitments;
- issuance time; and
- fixed freshness, validity, signature, effect, and redaction postures.

It stores no source record, resource reference, policy or approval body,
evidence payload, check output, command, path, provider response, credential,
or arbitrary metadata.

## 7. Validation And Compatibility

V1 uses fixed-width framed SHA-256 commitments. Receipt identity is derived
from the complete receipt commitment. Unverified-claim deserialization rejects
unknown fields and fails closed when any committed field or receipt identity
is substituted.

Validation proves internal consistency only. It does not authenticate an
issuer, restore source freshness, or convert a claim into trusted evidence. A
fixed V1 commitment vector protects the current local serialization contract.

## 8. Test Plan

Tests must prove:

- the trusted model has deterministic identity and a fixed commitment vector;
- a trusted receipt serializes into an explicitly unverified claim;
- a different self-consistent unsigned claim remains unverified;
- the public API exposes no production minting or claim-to-trusted conversion;
- valid unverified-claim deserialization;
- field substitution fails closed without leaking the substituted value;
- receipt `Debug` redacts identities and commitments;
- serialized output contains no raw provider, command, spec, credential, or
  private-key payload fields; and
- receipt data cannot enter the same-call authority API.

## 9. Recommended Follow-Up

After focused blocker-fix review, add at most one opt-in, Core-owned, read-only
receipt production path. It must re-resolve current authority in the same
call, issue only after a successful operation outcome, and never treat the
resulting receipt as permission.

Do not integrate OpenShell, broader execution providers, provider mutations,
or write-capable adapters before that read-only enforcement boundary is
reviewed.
