# Immutable Run Bundle Boundary Plan Review

Review date: 2026-07-12

## 1. Executive Verdict

**Plan accepted with non-blocking follow-ups; proceed to the immutable run-bundle core manifest/reference model only.**

## 2. Scope Assessment

The plan stays within architecture and implementation planning. It does not
authorize persistence, executor integration, executable replay, handler/check
attestation, provider mutation, CLI, schemas, examples, hosted runtime, RBAC,
signing, or release changes.

## 3. Boundary Assessment

The plan correctly distinguishes:

- events as transition history;
- snapshots as projections;
- manifests as selected immutable references;
- canonical definition records as declarative inspection inputs;
- approval commitments as equality proofs;
- handler/check attestation as a later execution-proof boundary;
- WorkReports as governed handoffs rather than state or permission sources.

This prevents the bundle from being presented as a replay package before the
runtime can prove handler identity, external inputs, authority, and SideEffect
reconciliation.

## 4. Model And Integrity Assessment

The proposed manifest identity, typed definition references, explicit execution
posture, handler posture, and domain-separated root hash are appropriately
minimal for the first model phase. Deterministic ordering and explicit labels
are required, and the plan correctly rejects Debug formatting as a long-lived
hash encoding.

The first implementation must keep the root hash algorithm field-based and
independent of future canonical-record storage encoding.

## 5. Persistence And Atomicity Assessment

The planned create-only sequence is conservative: validate and build in memory,
write immutable records, write the manifest, then append `RunCreated` with the
bundle reference. Missing or corrupt bundle state fails closed, while orphaned
unreferenced records are explicitly treated as a later garbage-collection
concern rather than repaired by hand.

Persistence remains outside the next model-only phase.

## 6. Privacy Assessment

The plan excludes raw YAML, source trees, prompts, transcripts, logs, provider
payloads, environment values, and credentials. Canonical definition records are
still potentially sensitive and therefore require internal sensitivity,
redaction-safe Debug/errors, repository-relative path posture, and local storage
protection before persistence is accepted.

## 7. Compatibility Assessment

Optional future bundle fields on run identity preserve old-state
deserialization. Legacy runs remain explicitly `legacy_unbundled`; current
files may not be used to fabricate a historical bundle.

## 8. Planning Blockers

None for the core manifest/reference model phase.

## 9. Non-Blocking Follow-Ups

- Select and review canonical definition-record storage encoding before record
  persistence; the root hash must not depend on that choice.
- Define explicit versioned labels for report-artifact policy posture before
  reusing the approval commitment as a bundle integrity input.
- Threat-model local record confidentiality before filesystem persistence.
- Define stable references for transient hook, SideEffect, and future external
  inputs before executable resume is considered.

## 10. Governed Review Evidence

- Workflow: `dg/review`.
- Run: `run-1783875502266437000-2`.
- Approval:
  `approval/run-1783875502266437000-2/review-scope-approved`.
- Presentation: `presentation/dbb5a7508c4ab8f3`.
- Approval outcome: granted through the proof-enforced presentation path.
- Out-of-kernel work: Codex reviewed and authored documentation and ran docs
  checks; the kernel governed scope and approval only.

## 11. Recommended Next Phase

Implement the core manifest/reference model only: validated types, explicit
field-based root hashing, serde, redaction-safe Debug, and focused tests. Do not
add canonical-record persistence, executor integration, replay, attestation,
provider writes, CLI, schemas, hosted behavior, or release changes.
