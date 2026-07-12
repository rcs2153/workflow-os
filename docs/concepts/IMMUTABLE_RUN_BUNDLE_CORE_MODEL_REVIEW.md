# Immutable Run Bundle Core Model Review

Review date: 2026-07-12

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups; proceed to canonical immutable
definition-record model implementation.**

The implementation establishes a deterministic, payload-free manifest model
for immutable run identity, content-addressed workflow/skill/policy references,
bounded execution posture, and honest handler posture. One review blocker was
found and fixed: the initial constructor required one workflow reference but
did not prove that it matched the manifest workflow identity and content hash.

## 2. Scope Verification

The phase stayed within the approved model-only boundary. It added types,
validation, deterministic hashing, serde, safe Debug, focused tests, roadmap
status, and phase documentation.

It did not add canonical definition payload records, stores, persistence,
executor or run-identity integration, executable replay, resume behavior,
handler/check attestation, authority enforcement, provider mutation, CLI,
schemas, examples, hosted behavior, RBAC/IdP, signing, or release changes.

## 3. Model Assessment

The model is domain-neutral and appropriately bounded. It represents:

- bundle identity and version;
- immutable workflow/run/schema identity;
- workflow and resolved-context commitments;
- typed workflow, resolved-skill, and policy references;
- checkpoint and selected execution-input preservation posture;
- explicit declared, unattested, mock, and unavailable handler posture;
- sensitivity, redaction requirement, creation actor/time, and root hash.

Private manifest fields and read-only accessors prevent post-construction
mutation. The model remains an integrity vocabulary, not an executable package.

## 4. Integrity Assessment

The root uses the explicit `workflow-os/immutable-run-bundle/v1` domain and
labeled, length-delimited fields. Definition and handler collections are
canonicalized deterministically. Root computation does not depend on Debug,
serde output, map iteration, local paths, or platform separators.

Construction requires exactly one workflow reference and now proves that its
ID, version, schema, and content hash match the manifest workflow identity.
Definition references are unique. Handler references are unique and must match
the resolved skill identities exactly. A supplied serialized root is
recomputed and mismatches fail closed.

## 5. Validation Assessment

Stable bounded validation covers:

- invalid bundle identity/version;
- invalid definition identity/version;
- incompatible skill-step posture;
- empty, excessive, duplicate, or workflow-missing definition sets;
- workflow-reference identity mismatch;
- excessive or duplicate checkpoint references;
- excessive, duplicate, missing, or unrelated handler references;
- malformed serialized execution posture;
- invalid or tampered serialized manifests.

Standalone execution posture now deserializes through its constructor, so
duplicate checkpoint references cannot bypass validation outside a manifest.
Errors use fixed text and do not echo supplied identifiers or corrupt values.

## 6. Privacy And Redaction Assessment

The manifest stores references and hashes only. It has no raw YAML, source,
prompt, transcript, command-output, provider-payload, environment, credential,
authorization-header, private-key, or token fields.

Manual Debug implementations redact bundle/run/workflow/skill identities and
hashes. Nested definition and handler records also redact their identifiers;
execution posture reports checkpoint counts rather than step values.
Serialization is an integrity/storage shape and is not represented as safe
operator output.

## 7. Compatibility And Honesty Assessment

No existing runtime or persisted event shape changed. The model is additive and
exported from `workflow-core`. No historical run is described as bundle-backed,
and no current executor path creates, persists, or consumes this manifest.

The explicit `RegisteredUnattested`, `MockSelected`, and `Unavailable` handler
postures directly address external dogfood feedback that mocks are not
execution evidence and real handler equivalence is not yet proven.

## 8. Test Quality Assessment

Seventeen focused tests cover deterministic roots, content changes, serde,
tampering, exactly-one-workflow validation, workflow identity alignment,
definition and handler uniqueness, handler/skill alignment, skill-step posture,
checkpoint validation and validated deserialization, canonical ordering, safe
Debug, non-echoing IDs, explicit handler posture, and forbidden payload-field
absence.

The full workspace suite passed, including existing approval/resume,
presentation proof, policy, hook, local-check, SideEffect, artifact, provider,
report, state, and CLI coverage.

## 9. Blockers

None after workflow-reference alignment and standalone execution-posture
deserialization were fixed.

## 10. Non-Blocking Follow-Ups

- Add a dedicated root-hash newtype when persistence or public inspection makes
  hash-domain confusion a practical risk.
- Decide canonical definition-record encoding independently of this field-based
  manifest root.
- Expand execution posture with stable hook, SideEffect, approval-proof,
  validation/check, and governance references before executable resume.
- Model a concrete future handler/check attestation reference only after its
  threat model is accepted.
- Treat local bundle confidentiality and source-reference path handling as
  storage-phase requirements.

## 11. Governed Review Evidence

- Workflow: `dg/review`.
- Run: `run-1783879296562129000-2`.
- Approval:
  `approval/run-1783879296562129000-2/review-scope-approved`.
- Presentation: `presentation/2b7ea47aa98869c7`.
- Approval outcome: granted through the proof-enforced presentation path under
  delegated-maintainer authority after the complete handoff was relayed.
- Event summary: 39 events, one approval, zero retries, zero escalations.
- Validation summary: formatting, strict workspace clippy, the full workspace
  test suite, docs validation, and diff hygiene passed after the blocker fix.
- Out-of-kernel work: Codex inspected code/docs/tests, fixed the alignment and
  validation blocker, authored this review, and ran validation. The kernel
  governed scope and approval only.

## 12. Recommended Next Phase

Proceed to the canonical immutable definition-record model only. Define typed,
validated canonical workflow/skill/policy record shapes and deterministic
content verification without adding a store, executor integration, replay,
attestation, authority enforcement, provider writes, CLI, schemas, hosted
behavior, or release changes.
