# Immutable Run Bundle Definition Record Model Report

Report date: 2026-07-12

## 1. Executive Summary

The canonical immutable workflow, skill, and policy definition-record model is
implemented. It turns typed parser-validated definitions into source-path-free
canonical records, binds them to the loader's source content hash, computes a
separate versioned canonical-record hash, and can produce manifest references
without rereading source files.

The focused review removed standalone deserialization from the exported
canonical-definition enum. Serialized definitions now enter only through the
record's private wire boundary, which canonicalizes and validates before
storage.

This remains model-only. It does not persist records, build bundles from a
project, integrate with the executor, or make records executable.

## 2. Scope Completed

- Added `ImmutableRunBundleCanonicalDefinition` for workflow, skill, and policy
  typed content.
- Added explicit `ImmutableRunBundleDefinitionEncoding::CanonicalJsonV1`.
- Added `ImmutableRunBundleDefinitionRecord` with record version, source hash,
  canonical definition, sensitivity, redaction posture, and record hash.
- Added constructors that reparse canonical typed content through the existing
  project parsers and reject secret-like programmatic model content.
- Removed workflow/skill source locations and workflow derived spec hash from
  canonical content.
- Added validated deserialization, deterministic hashing, safe Debug, read-only
  accessors, and manifest-reference construction.

## 3. Validation Boundary

Workflow construction requires the parser-populated workflow content hash to
match the supplied loader source hash. All three definition kinds are serialized
to canonical JSON and reparsed through the existing typed project parser, which
reapplies schema, unknown-field, identifier, and secret-in-spec checks.

Deserialization reparses the stored typed definition, rejects non-canonical
derived fields, recomputes the record hash, and fails closed on mismatch.

This phase does not claim full cross-definition project semantic validation.
The future in-memory bundle builder must consume an already validated
`ProjectBundle` and resolve workflow/skill/policy relationships before records
are eligible for persistence.

## 4. Canonical Encoding And Hashing

The record declares `canonical_json_v1` explicitly. Canonical content is serde
JSON over the typed definition model after source-derived fields are removed.
The `workflow-os/immutable-run-bundle-definition/v1` hash covers:

- record version and encoding;
- definition kind, ID, version, and schema;
- loader source content hash;
- canonical typed definition bytes;
- sensitivity and redaction posture.

The hash does not use Debug output, raw YAML, source paths, map iteration over
unordered maps, or platform separators.

## 5. Privacy And Redaction

Records contain canonical typed definition content, not raw YAML. Comments,
formatting, aliases, parser diagnostics, and source locations are omitted.
Existing secret-in-spec checks run again at the record boundary, including for
programmatically constructed definitions.

Debug output redacts definition identity, source hash, record hash, and all
definition content. Serialization is a future storage/inspection shape and is
not represented as safe terminal output. It has no provider payload, command
output, environment, credential, authorization-header, private-key, or token
fields.

## 6. Tests

Focused tests cover all definition kinds, deterministic and content-sensitive
record hashes, workflow source-hash alignment, secret-like programmatic content
rejection, source-derived field omission, manifest-reference construction,
serde round trip, record-hash tampering, non-canonical derived-field rejection,
safe Debug, explicit encoding, and forbidden payload-field absence.

## 7. Governed Phase Evidence

- Workflow: `dg/implement`.
- Run: `run-1783882513876422000-2`.
- Approval:
  `approval/run-1783882513876422000-2/implementation-approved`.
- Presentation: `presentation/49a1e31a777e1748`.
- Approval outcome: granted through the proof-enforced presentation path under
  delegated-maintainer authority after the complete handoff was relayed.
- Event summary: 39 events, one approval, zero retries, zero escalations.
- Out-of-kernel work: Codex authored code, tests, and documentation and ran
  validation. The kernel governed scope and approval only.

## 8. Explicitly Not Implemented

No record or manifest store, persistence, filesystem writing, project-to-bundle
builder, executor/run-identity integration, replay, resume, handler/check
attestation, scoped authority enforcement, provider mutation, CLI, schemas,
SDK changes, examples, hosted behavior, signing, or release changes were added.

## 9. Validation Commands

All required validation passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

## 10. Remaining Limitations

- Canonical JSON v1 is explicitly versioned but has not been tested across
  multiple crate releases.
- Skill and policy source hashes are supplied by the caller because those typed
  models do not carry parser-populated hashes; the future builder must source
  them from `LoadedSpec`.
- Manifest references currently carry the loader source hash, while records
  also have a canonical-record hash. Store-key and cross-reference rules remain
  for the persistence plan.
- The model does not prove handler/check implementation identity.

## 11. Recommended Next Phase

Perform a focused maintainer review of the canonical definition-record model.
If accepted, implement the pure in-memory immutable run-bundle builder that
consumes an already validated `ProjectBundle` and explicit execution posture,
without persistence or runtime mutation.
