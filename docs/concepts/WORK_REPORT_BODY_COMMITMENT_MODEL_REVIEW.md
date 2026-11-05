# WorkReport Body Commitment Model Review

## Decision

Accepted for the model-only boundary.

The implementation supplies the missing complete-body commitment vocabulary
without presenting a hash as independent integrity evidence. It is suitable as
the input to a separately reviewed persistence-anchor phase.

This is an owner-conducted focused maintainer/security review. It is not the
independent security review still required before release admission resumes.

## Reviewed Properties

The review confirmed:

1. V1 commits to the complete serialized `WorkReport`, not only duplicated
   artifact identity metadata.
2. The commitment envelope has an explicit domain and version, and both enter
   the hashed bytes.
3. The report serialization covers identity, complete generation context,
   ordered sections and citations, every disclosure/note collection, optional
   high-assurance approval posture, sensitivity, and redaction metadata.
4. Ordered collection changes alter the commitment.
5. A golden vector detects accidental changes to canonical V1 encoding.
6. Commitment mismatch, invalid input, and canonicalization failure use stable,
   payload-free errors.
7. Debug output redacts the digest and report body.
8. Legacy artifacts have an explicit `legacy_unverified` posture.
9. The available posture is named `available_unanchored`, not `computed`,
   because a deserialized claim has no Core-owned construction provenance.
10. No store, event, artifact, report, CLI, schema, SDK, or policy path treats
    the new value as verified or authorizing.

## Review Correction

The first implementation draft named the available posture
`computed_unanchored`. Review rejected that wording because
`WorkReportBodyCommitment` is serializable and deserializable. A syntactically
valid deserialized value could then misleadingly claim Core computation.

The accepted name is `available_unanchored`. Fresh construction can be proven
only by the call path that computes the value; the portable value itself does
not carry that provenance.

## Residual Risks

The model does not protect a persisted artifact. A storage owner can still edit
the report because no expected commitment is retained outside the body.

Even after the next local persistence phase, an owner who can rewrite both the
artifact and every local anchor remains inside the trust boundary. Hashing does
not provide independent authorship, signing, transparency, or external
verifiability.

V1 canonicalization depends on the reviewed compact serialization shape. The
golden vector makes drift visible, but maintainers must still choose a new
version when coverage, ordering, framing, or serialization semantics change.

## Validation Reviewed

- Focused tests changed every report body family and collection order and
  observed a different commitment.
- A fixed golden vector proved deterministic V1 bytes.
- Matching verification succeeded; changed prose failed with the stable
  mismatch code and no payload leakage.
- A caller-deserialized false commitment remained unanchored and failed
  comparison.
- Legacy posture round-tripped explicitly as `legacy_unverified`.
- Warnings-denied Core Clippy and the complete Core all-feature suite passed.

## Next Phase

Proceed to the bounded WorkReport body-commitment persistence anchor. Bind the
expected V1 value outside mutable report content using existing terminal
artifact state and supported stores with reviewed atomicity or crash
reconciliation. Keep read/list/inspect/recovery/replay enforcement and product
projection in later phases.
