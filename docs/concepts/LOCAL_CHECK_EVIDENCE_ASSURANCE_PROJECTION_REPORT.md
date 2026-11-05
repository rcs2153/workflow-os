# Local Check Evidence Assurance Projection Report

## Summary

Workflow OS now has one model-only, payload-free projection that distinguishes
what the current local-check path can prove from what it merely records.

The projection keeps four facts separate:

- producer provenance is `operator_selected_content_bound`;
- execution observation is `trusted_host_observed`;
- attestation is either `unattested` or
  `kernel_accepted_local_process`; and
- external verifiability is `unavailable`.

This does not create a second evidence or attestation system. The model reuses
the existing immutable execution binding, handler-selection commitment,
resolved-execution commitment, structured result, immutable run binding, and
crate-private accepted local-process attestation.

## Implemented Boundary

`LocalCheckEvidenceAssuranceProjection` binds:

- immutable run, workflow, run, and step identity;
- local-check command identity and kind;
- immutable execution-binding and handler-selection fingerprints;
- the payload-free resolved-execution commitment;
- local-check result identity and status;
- independent producer, execution, and attestation posture;
- an optional accepted-attestation proof reference; and
- explicit unavailable external-verifiability posture.

The projection has a versioned, domain-separated, fixed-width framed SHA-256
fingerprint. Its `Debug` implementation redacts identities and commitments,
and it never stores paths, command output, environment values, summaries,
credentials, provider payloads, or mutable report text.

## Assurance Integrity

The public constructor always derives the current producer and execution
posture from the supplied existing commitments. A zero exit, executable name,
path, selected content hash, or recomputable execution-binding fingerprint
cannot upgrade producer or attestation posture.

`kernel_accepted_local_process` requires an in-memory
`AcceptedLocalCheckAttestation` returned by the existing crate-private
verifier. The constructor checks exact agreement across:

- immutable execution-binding fingerprint;
- immutable run binding;
- workflow, run, and step identity;
- handler-selection fingerprint; and
- result identity and status.

The projection serializes for bounded disclosure, but deserialization rejects
accepted-attestation posture and accepted-proof references. A stored proof
fingerprint is not the verifier-owned proof object and cannot recreate stronger
assurance. The current weak posture round-trips through validated serde;
unknown vocabulary, contradictory posture, and fingerprint tampering fail
closed.

## Minimum Assurance

`LocalCheckEvidenceAssuranceMinimum` compares producer, execution, and
attestation independently. It does not collapse the dimensions into one global
level. The current unattested validator posture satisfies only an unattested
minimum. A projection constructed from accepted verifier proof satisfies both
the weaker unattested minimum and the stronger accepted-attestation minimum.

This is model vocabulary for a later narrow gate. No policy or runtime consumer
was added in this phase.

## Tests

Focused tests prove:

- the current validator posture round-trips without an assurance upgrade;
- the projection has a stable known fingerprint vector;
- pass and fail results do not alter producer or attestation posture;
- command, handler selection, resolved execution, immutable run, result
  identity, and result status alter the projection commitment;
- unknown vocabulary and serialized tampering fail closed;
- debug and errors disclose no commitments or payloads;
- minimum assurance is dimension-specific and monotonic;
- an exact accepted proof constructs stronger in-memory posture;
- a mismatched result cannot reuse an accepted proof; and
- serialization cannot recreate verifier-owned accepted assurance.

## Explicit Non-Goals

This phase did not add:

- trusted-host or executor integration;
- persistence, events, replay, or recovery behavior;
- WorkReport citations or report-body commitments;
- CLI, schema, or SDK surface;
- automatic local-check execution or arbitrary commands;
- signatures, external witnesses, transparency logs, or remote attestation;
- hosted identity, provider expansion, writes, or publication; or
- storage-owner tamper resistance.

The projection fingerprint remains deterministic model identity. By itself it
is not Core provenance, independent producer provenance, a signature, or an
external receipt.

## Governed Phase

- Workflow: `dg/implement`
- Run: `run-1793852567018268000-2`
- Approval:
  `approval/run-1793852567018268000-2/implementation-approved`
- Presentation: `presentation/a7204706cbc4d5d2`
- Approval outcome: granted by the delegated maintainer after complete handoff
  presentation
- Kernel freshness: rebuilt from the exact checkout before phase start
- Repository edits, tests, and documentation ran outside the kernel under the
  approved execution scope

## Validation

- Focused projection unit tests: passed.
- Existing accepted-attestation verifier integration test: passed with the new
  projection assertions.
- `cargo fmt --all --check`: passed.
- `cargo clippy -p workflow-core --all-targets --all-features -- -D warnings`:
  passed.
- `cargo test -p workflow-core --all-features`: passed, including doc tests;
  opt-in live credential and external-tool smoke tests remained ignored by
  their declared contracts.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## Next Phase

The focused maintainer/security review accepted the model with one explicit
guardrail: a serialized weak projection is representational data, not proof of
a Core-owned observation. See the [review](LOCAL_CHECK_EVIDENCE_ASSURANCE_PROJECTION_REVIEW.md).

The next phase may compose the weak projection into the closed trusted-host
project-validation result path. It must establish Core-owned source provenance
before any minimum comparison can participate in a gate.
