# Immutable Run Bundle Definition Record Model Review

Review date: 2026-07-13

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups; proceed to the pure in-memory
immutable run-bundle builder.**

The implementation establishes canonical, source-path-free workflow, skill,
and policy definition records with explicit encoding, source-content
commitments, deterministic record hashes, validated serde, and bounded Debug.
One review blocker was found and fixed: the exported canonical-definition enum
initially allowed standalone deserialization that could bypass the enclosing
record's canonicalization gate.

## 2. Scope Verification

The phase stayed within the approved model-only boundary. It added canonical
definition-record vocabulary, validation, deterministic hashing, serde, safe
Debug, focused tests, roadmap status, and phase documentation.

It did not add a bundle builder, definition store, manifest store, persistence,
executor integration, run creation or resume behavior, executable replay,
handler or check attestation, authority enforcement, provider mutation, CLI,
schemas, examples, hosted behavior, RBAC/IdP, signing, or release changes.

## 3. Model Assessment

The model is domain-neutral and appropriately bounded. It represents:

- an explicit record model version and canonical encoding;
- canonical validated workflow, skill, or policy content;
- the validated loader boundary's source-content hash;
- sensitivity and redaction posture;
- a separate deterministic canonical-record hash;
- a manifest definition reference derived from the record.

The record stores typed definitions rather than raw YAML. Source locations,
step source locations, and the workflow parser's derived content hash are not
retained in canonical content.

## 4. Canonicalization And Integrity Assessment

Construction serializes typed definitions and reparses them through the
existing validated workflow, skill, or policy parser. The result is normalized
typed model content, not caller-selected bytes. The canonical-record hash uses
the explicit `workflow-os/immutable-run-bundle-definition/v1` domain and
labeled, length-delimited fields covering record version, encoding, definition
identity, schema, source-content hash, canonical bytes, sensitivity, and
redaction posture.

Workflow construction additionally proves that the supplied source hash
matches the parser-populated workflow content hash. Skill and policy models do
not currently retain that parser hash, so their source-hash provenance remains
a responsibility of the future validated-project builder boundary.

## 5. Validation Assessment

Stable bounded validation covers:

- workflow source-hash mismatch;
- parser-invalid or secret-like definition content;
- serialization failure;
- unsupported serialized encoding;
- noncanonical serialized definition content;
- tampered canonical-record hashes;
- incompatible manifest-reference step posture through the existing reference
  constructor.

Errors use fixed text and do not echo definition IDs, source paths, payloads,
hashes, or rejected values.

## 6. Serde Boundary Assessment

Valid records serialize and deserialize through the record validation gate.
Deserialization canonicalizes the supplied typed definition, rejects any
source-derived or otherwise noncanonical fields, recomputes the record hash,
and fails closed on mismatches.

The public `ImmutableRunBundleCanonicalDefinition` now supports serialization
but not standalone deserialization. A private wire enum is used only by the
validated record deserializer. This closes the review blocker: callers cannot
materialize a value named canonical from serialized input without the record's
canonicalization and integrity checks.

## 7. Privacy And Redaction Assessment

The record stores validated definition content and must therefore be treated as
sensitive even though it excludes raw source files. It does not store raw YAML,
source paths, prompts, transcripts, command output, provider payloads,
environment values, credentials, authorization headers, private keys, or
tokens.

Manual Debug implementations redact definition identity and both hashes and do
not format nested definition content. Serialization is an integrity/storage
shape, not a safe operator display shape.

## 8. Compatibility And Honesty Assessment

The model is additive and does not change existing specs, runtime state, event
history, approval behavior, or persisted formats. It does not claim complete
cross-definition project validation: parser-local validity is enforced here,
while resolved skill/policy membership and full semantic validation remain the
future builder's responsibility.

Manifest references currently commit the loader source-content hash. The
canonical-record hash is available on the record but is not yet a persisted
store key or manifest reference field; that decision remains appropriately
deferred to the builder/store boundary.

## 9. Test Quality Assessment

Focused tests cover all three definition kinds, explicit encoding,
deterministic and content-sensitive record hashing, workflow source-hash
alignment, programmatic secret rejection, source-derived field removal,
manifest reference creation, serde round trip, tampered hashes, noncanonical
serialized fields, safe Debug, and forbidden payload-field absence.

The full workspace validation remains required before phase close. A future
builder test must prove that skill and policy source hashes come from the
validated loader's `LoadedSpec` records rather than arbitrary caller input.

## 10. Blockers

None after standalone canonical-definition deserialization was removed and the
private validated wire boundary was introduced.

## 11. Non-Blocking Follow-Ups

- Make the future builder source all workflow, skill, and policy source hashes
  from the already validated `ProjectBundle`/`LoadedSpec` boundary.
- Add cross-release canonical encoding compatibility fixtures before any
  persisted record format is declared stable.
- Decide whether manifest references should commit the canonical-record hash,
  source-content hash, or both before implementing a store.
- Keep full cross-definition semantic validation at the future builder
  boundary; do not imply that parser-local record construction is sufficient.
- Treat serialized definition records as sensitive storage artifacts and keep
  them out of default operator output.

## 12. Governed Review Evidence

- Workflow: `dg/review`.
- Run: `run-1783884818214565000-2`.
- Approval:
  `approval/run-1783884818214565000-2/review-scope-approved`.
- Presentation: `presentation/ad0a925e3348132b`.
- Approval outcome: granted through the proof-enforced presentation path under
  delegated-maintainer authority after the complete handoff was relayed.
- Event summary: 39 events, one approval, zero retries, zero escalations, with
  one persisted approval-presentation proof record.
- Validation summary: formatting, strict workspace clippy, the full workspace
  test suite, docs validation, and diff hygiene passed after the blocker fix.
- Out-of-kernel work: Codex inspected code/docs/tests, fixed the serde-boundary
  blocker, authored this review, and ran validation. The kernel governed scope
  and approval only.

## 13. Recommended Next Phase

Implement the pure in-memory immutable run-bundle builder. It should consume an
already validated `ProjectBundle` plus explicit execution posture and produce
canonical definition records and a matching manifest without persistence or
runtime mutation.

Do not add stores, executor integration, replay, handler/check attestation,
capability enforcement, provider writes, CLI, schemas, hosted behavior, or
release changes in that phase. Scoped runtime authority and capability
projection remains sequenced after immutable run-input hardening.
