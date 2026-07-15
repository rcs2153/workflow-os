# Immutable Run Bundle Boundary Plan

Status: Core manifest/reference model implemented and accepted with non-blocking
follow-ups in [Immutable Run Bundle Core Model Review](../concepts/IMMUTABLE_RUN_BUNDLE_CORE_MODEL_REVIEW.md).
The canonical definition-record model is implemented in
[Immutable Run Bundle Definition Record Model Report](../concepts/IMMUTABLE_RUN_BUNDLE_DEFINITION_RECORD_MODEL_REPORT.md)
and accepted with non-blocking follow-ups in
[Immutable Run Bundle Definition Record Model Review](../concepts/IMMUTABLE_RUN_BUNDLE_DEFINITION_RECORD_MODEL_REVIEW.md).
The pure in-memory builder is implemented in
[Immutable Run Bundle Builder Report](../concepts/IMMUTABLE_RUN_BUNDLE_BUILDER_REPORT.md)
and accepted with non-blocking follow-ups in
[Immutable Run Bundle Builder Review](../concepts/IMMUTABLE_RUN_BUNDLE_BUILDER_REVIEW.md).
The create-only local store is implemented in
[Immutable Run Bundle Local Store Report](../concepts/IMMUTABLE_RUN_BUNDLE_LOCAL_STORE_REPORT.md).
It is accepted with non-blocking follow-ups in
[Immutable Run Bundle Local Store Review](../concepts/IMMUTABLE_RUN_BUNDLE_LOCAL_STORE_REVIEW.md).
The first explicit opt-in executor binding is implemented in
[Immutable Run Bundle Executor Binding Report](../concepts/IMMUTABLE_RUN_BUNDLE_EXECUTOR_BINDING_REPORT.md).
It persists or verifies the complete bundle before `RunCreated` and binds the
bundle ID, version, and integrity root into durable run identity. Default
executor paths remain unchanged. The binding is accepted with non-blocking
follow-ups in
[Immutable Run Bundle Executor Binding Review](../concepts/IMMUTABLE_RUN_BUNDLE_EXECUTOR_BINDING_REVIEW.md).

Related foundations:

- [Execution Semantics](../runtime/execution-semantics.md)
- [Run Rehydration](../runtime/run-rehydration.md)
- [Approval Resume Resolved-Context Integrity Plan](approval-resume-resolved-context-integrity-plan.md)
- [Scoped Runtime Authority And Capability Projection Plan](scoped-runtime-authority-capability-projection-plan.md)
- [Governed Work Pattern](../concepts/governed-work-pattern.md)

## 1. Executive Summary

Workflow OS currently records immutable run identity and now binds pending
approvals to a resolved execution-context commitment. Those controls reject
changed current files, but they do not preserve the validated definitions and
governance posture needed to inspect a historical run independently of the
working tree.

The next boundary is a durable immutable run bundle: a versioned manifest plus
content-addressed canonical records for the validated workflow, resolved
skills, referenced policies, and bounded execution/governance posture selected
at run creation.

The bundle is an inspection and integrity substrate. It is not yet an
executable replay package. It does not attest skill-handler binaries, preserve
credentials or provider state, execute checks, resume distributed work, or
authorize mutations.

This plan does not implement anything.

## 2. Goals

- Preserve the exact validated declarative inputs selected for a run.
- Make historical inspection independent of later project-file changes.
- Reuse the accepted resolved execution-context commitment as an integrity
  input where its v1 boundary is sufficient.
- Bind bundle identity into durable run identity before execution begins.
- Keep manifests payload-free and place canonical definition records behind
  content-addressed references.
- Preserve deterministic ordering, hashing, validation, and serde behavior.
- Fail closed on missing, corrupt, mismatched, or partially written records.
- Support backward-readable historical runs that predate bundles without
  pretending they are bundle-backed.
- Prepare later handler/check attestation, scoped authority receipts, governed
  resume, and evidence/report citation without implementing them now.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- raw YAML, arbitrary source-tree, prompt, transcript, log, or provider-payload
  archiving;
- credentials, environment values, authorization headers, private keys, or
  token storage;
- handler binary, container, model, tool, or command attestation;
- executable replay, rollback, compensation, migration, or automatic resume;
- provider calls, provider writes, mutation retries, or recovery;
- CLI commands, public schemas, SDK changes, examples, or UI;
- hosted/distributed storage, remote policy sync, RBAC, IdP, or tenant models;
- cryptographic signatures, notarization, transparency logs, or legal records;
- reasoning lineage, recursive agents, agent swarms, or release changes.

## 4. Current Boundary And Gap

Current run identity contains run ID, workflow ID, workflow version, schema
version, and workflow content hash. Event replay enforces that tuple.

Current approval context integrity adds a versioned commitment over:

- workflow content;
- ordered steps;
- resolved skill content;
- referenced policy content;
- checkpoint and hook posture;
- SideEffect input counts;
- report-artifact policy posture.

That commitment prevents changed current inputs from resuming. It does not
retain the definitions whose hashes it commits to. If the project files are
deleted or replaced, Workflow OS can prove mismatch but cannot show the exact
validated declarative context without relying on Git or another external store.

Git remains useful provenance, but it is not the runtime state contract. Runs
may originate from dirty worktrees, generated files, archives, or future
non-Git sources.

## 5. Source-Of-Truth Boundaries

| Record | Source of truth | Not equivalent to |
| --- | --- | --- |
| Run event stream | Runtime transition history | Definition archive or handler proof |
| Run snapshot | Rehydrated projection | Independent immutable source |
| Run bundle manifest | Selected definition and posture references | Raw payload archive or permission grant |
| Bundle definition record | Canonical validated declarative model | Original YAML bytes or executable handler |
| Approval context commitment | Equality proof for approved resolved posture | Self-contained historical bundle |
| Git commit | Repository provenance when available | Required runtime database |
| Authority receipt | Future invocation authority proof | Bundle membership or approval by itself |
| WorkReport | Governed terminal handoff | Event log or bundle replacement |

## 6. Candidate Core Model

The first implementation should add the smallest domain-neutral types:

- `ImmutableRunBundleManifest`;
- `ImmutableRunBundleId`;
- `ImmutableRunBundleVersion`;
- `ImmutableRunBundleRootHash` or a dedicated wrapper over validated SHA-256;
- `ImmutableRunBundleDefinitionReference`;
- `ImmutableRunBundleDefinitionKind` (`Workflow`, `Skill`, `Policy`);
- `ImmutableRunBundleExecutionPosture`;
- `ImmutableRunBundleHandlerPosture`;
- `ImmutableRunBundleValidationError` only if repository error patterns justify
  a dedicated type.

The first model phase should not add a general artifact graph, arbitrary
metadata map, generic blob API, or hosted storage abstraction.

## 7. Required Manifest Identity

The manifest should capture:

- bundle ID and bundle format version;
- run ID;
- workflow ID, workflow version, and schema version;
- workflow content hash;
- resolved execution-context commitment;
- ordered workflow definition reference;
- ordered resolved skill references keyed by step and resolved version;
- deduplicated referenced-policy records sorted by policy ID;
- bounded execution posture;
- creation timestamp and creating system actor;
- optional repository/Git provenance when explicitly available;
- sensitivity and redaction metadata;
- bundle root hash.

Repository provenance is descriptive, not authoritative. A missing Git commit
must not prevent a valid local bundle.

## 8. Canonical Definition Records

The bundle manifest must not embed raw YAML. Each definition reference should
point to a separately stored canonical validated record containing:

- definition kind;
- stable definition ID and version where modeled;
- schema version;
- canonical content hash already produced by project loading;
- canonical serialized validated model bytes;
- sensitivity and redaction posture;
- optional source-reference metadata that does not expose private absolute
  paths by default.

The bytes are canonical model serialization, not original source text. Comments,
formatting, aliases, and parser diagnostics are not preserved. This reduces
ambiguity and avoids turning the bundle store into a source-code archive.

Policy and skill definitions remain potentially sensitive even when they pass
secret-in-spec validation. Local storage permissions and safe Debug/errors are
required.

## 9. Execution Posture

The manifest must preserve bounded posture that affects execution but is not
fully represented by definition records:

- required checkpoint step IDs;
- explicit hook-input presence and stable hook invocation/reference IDs when
  available, never raw hook payloads;
- SideEffect input presence/count and stable SideEffect IDs when available;
- project validation capability/mode;
- derived report-artifact high-assurance posture;
- derived approval proof-marker posture;
- selected governance profile and policy decision references when they already
  exist at bundle-creation time;
- explicit missing/unavailable posture for fields that cannot yet be bound.

Counts alone may preserve the current fail-closed approval behavior but are not
sufficient for future executable resume. The model must distinguish
`committed_reference`, `present_but_not_preserved`, `not_supplied`, and
`unsupported` rather than implying recoverability.

## 10. Handler And Check Posture

The first bundle must explicitly state that handler execution identity is not
attested. A bounded posture should distinguish:

- handler identity declared by skill ID/version only;
- local handler registered but implementation unattested;
- mock handler selected;
- handler unavailable;
- future attestation reference present.

The bundle must not hash process paths, binaries, command strings, environment
values, or credentials in v1. Real check and handler attestation requires a
separate threat model and implementation phase.

This explicit limitation addresses external dogfood feedback without making a
false execution-evidence claim.

## 11. Root Hash And Determinism

Use an explicit domain such as:

```text
workflow-os/immutable-run-bundle/v1
```

The root hash should cover labeled, length-delimited canonical fields:

- manifest version and immutable run identity;
- resolved execution-context commitment;
- every definition reference in deterministic order;
- execution posture;
- handler/check posture;
- sensitivity and redaction posture;
- optional provenance presence and bounded values.

The hash must not depend on map iteration order, absolute paths, timestamps
other than the manifest's explicit creation field, generated event IDs, Debug
formatting, platform-specific separators, or serializer implementation details.

The approval commitment currently encodes report-artifact policy posture using
internal Debug output. Before reusing it as a long-lived bundle integrity root,
replace that encoding with explicit versioned labels.

## 12. Persistence And Atomicity

Future local persistence should use two contracts:

- content-addressed immutable definition-record store;
- immutable run-bundle manifest store keyed by run ID and bundle ID.

Required write sequence:

1. Load and deterministically validate the project.
2. Build canonical definition records and manifest in memory.
3. Validate every hash and reference.
4. Write missing content-addressed records idempotently.
5. Write the immutable manifest with create-only semantics.
6. Append `RunCreated` carrying bundle ID and root hash.

If step 6 fails, unreferenced immutable records may remain as harmless orphans
for later garbage-collection planning. A run must never reference a missing or
partially written manifest. Existing bundle IDs or run bindings may not be
overwritten.

Manual state-file editing remains forbidden.

## 13. Runtime Identity Integration

Add optional bundle identity fields to `WorkflowRunIdentity` for backward
deserialization:

- bundle ID;
- bundle version;
- bundle root hash.

New local runs on the bundle-enabled path must populate them. Legacy runs with
no bundle remain inspectable but must report `legacy_unbundled`; they must not be
silently upgraded from current files.

Bundle integration should begin as an explicit executor path or capability,
not an automatic change to every existing execution API. Default adoption
requires separate review after model and store behavior are accepted.

## 14. Inspection And Resume Semantics

The first accepted bundle should support:

- loading a manifest by run identity;
- validating the root hash and all definition references;
- inspecting exact canonical declarative inputs;
- comparing current project definitions with the historical bundle;
- citing bundle and definition hashes in audit/report records later.

It must not yet:

- execute directly from stored definitions;
- claim handler/check equivalence;
- dereference secrets or provider payloads;
- replay external events or SideEffects;
- resume a run automatically.

Later governed resume must require compatible handler/check attestation,
authority, external-input, and SideEffect reconciliation in addition to a valid
bundle.

## 15. Privacy And Redaction

- Store canonical validated definitions only after existing secret-in-spec
  checks pass.
- Treat every bundle as at least internal sensitivity by default.
- Never include raw source snippets, parser payloads, command output, provider
  bodies, environment values, credentials, or tokens.
- Omit or repository-relativize source paths; never persist private absolute
  paths by default.
- Debug output should show counts, kinds, and posture, not IDs or content.
- Serialization is a storage boundary and may contain canonical definitions;
  it must not be treated as safe terminal output.
- Deserialization and integrity errors must use stable codes without echoing
  corrupt values or local paths.

## 16. Error Handling

Required stable failure classes include:

- manifest invalid;
- unsupported bundle version;
- duplicate definition reference;
- missing definition record;
- definition hash mismatch;
- root hash mismatch;
- run identity mismatch;
- create-only conflict;
- legacy bundle unavailable;
- execution posture not preserved;
- handler attestation unavailable.

Construction or persistence failure must prevent `RunCreated`. It must not be
converted into a misleading user-project diagnostic or a partially bundled run.

## 17. Proposed Implementation Sequence

1. **Core manifest and reference model only.** Validation, serde, deterministic
   explicit-label root hashing, redaction-safe Debug, and focused tests.
2. **Canonical definition-record model.** Typed validated model serialization;
   no store yet.
3. **Implemented.** In-memory bundle builder consuming a validated
   `ProjectBundle` plus explicit request posture; no runtime mutation.
4. **Implemented.** Local create-only content-addressed canonical records and
   run-bound manifests with corruption, identity, failure-atomicity, and
   restart tests.
5. **Implemented.** Explicit executor bundle path that persists before
   `RunCreated` and binds optional bundle identity into new runs.
6. **Read-only inspection helper.** Validate and compare historical bundle to
   current project state; no CLI initially.
7. **Focused maintainer review.** Review privacy, compatibility, atomicity, and
   non-overclaiming before default adoption.
8. **Handler/check attestation planning.** Only after bundle inspection works.
9. **Governed resume planning.** Only after attestation, authority, external
   input, and SideEffect reconciliation boundaries exist.

Each item is a separate governed implementation and review phase. The local
store must pass focused maintainer review before item 5 begins.

## 18. Test Plan

Future tests must cover:

- valid minimal manifest;
- all required identity fields;
- workflow, skill, and policy references;
- deterministic ordering and root hashing;
- duplicate references rejected;
- root or record hash mismatch rejected;
- same ID/version changed content remains distinguishable;
- unreferenced project definitions excluded;
- execution posture states are explicit;
- mock/unattested handler posture is honest;
- canonical records exclude raw YAML and comments;
- no absolute path, source snippet, provider payload, command output, secret, or
  environment value storage;
- Debug and error non-leakage;
- serde round trip and invalid serde fail closed;
- legacy run identity remains readable and explicitly unbundled;
- create-only idempotency and conflict behavior;
- missing/corrupt record detection after restart;
- failure before `RunCreated` on bundle persistence errors;
- no executable replay or provider calls;
- existing runtime, approval, policy, SideEffect, report, and provider tests.

## 19. Open Questions

- Should canonical definition records use JSON, a dedicated canonical binary
  encoding, or typed field-by-field hashing independent of storage encoding?
- Should bundle identity use a dedicated root-hash newtype immediately?
- Which request inputs need stable references before an executor bundle path can
  safely resume rather than inspect only?
- When should local orphan-record garbage collection be planned?
- Should repository provenance include dirty-worktree posture without storing a
  patch?
- How should future workflow catalog records and bundles share canonical
  definition storage without creating conflicting sources of truth?
- What exact handler/check attestation is useful across local processes,
  containers, hosted workers, and model-backed skills?

These questions should be narrowed during model review. They must not be
answered by broadening the first implementation.

## 20. Final Recommendation

Perform a focused maintainer review of the explicit executor bundle path before
beginning read-only historical inspection or scoped authority work. Review
pre-run publication ordering, durable identity binding, retry behavior, legacy
compatibility, privacy, and non-overclaiming. Do not add default bundle
generation, executable replay, handler attestation, provider mutation
expansion, CLI, schema, hosted, or release work during that review.
