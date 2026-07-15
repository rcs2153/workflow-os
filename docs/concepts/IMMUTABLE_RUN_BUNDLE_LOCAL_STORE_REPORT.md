# Immutable Run Bundle Local Store Report

## 1. Executive Summary

The create-only local immutable run-bundle store is implemented in
`workflow-core`. It persists canonical validated definition records by their
canonical-record hashes and publishes one immutable manifest per run only after
every manifest reference resolves to exactly one stored record.

The phase is storage-only. It does not create bundles automatically, integrate
with the executor, append `RunCreated`, alter approval or resume behavior,
execute from stored definitions, call providers, or authorize external writes.

## 2. Scope Completed

- Added `LocalImmutableRunBundleStore`.
- Added `StoredImmutableRunBundle` for validated read results.
- Added idempotent content-addressed canonical-record writes.
- Added create-only run-bound manifest writes.
- Added complete bundle writes where the manifest is the commit marker.
- Added restart-safe manifest, record, and complete-bundle reads.
- Added missing, corrupt, mismatched, conflicting, and ambiguous-record checks.
- Added redaction-safe Debug and stable non-leaking storage errors.

## 3. Scope Explicitly Not Completed

- No executor integration or automatic bundle creation.
- No bundle identity on `RunCreated` or `WorkflowRunIdentity`.
- No approval, resume, policy, hook, SideEffect, or report behavior changes.
- No executable replay, handler/check attestation, or scoped authority.
- No CLI command, workflow schema, SDK, example, or UI exposure.
- No provider calls, provider writes, hosted store, garbage collection, or
  release-posture changes.

## 4. Store API Summary

`LocalImmutableRunBundleStore` accepts an explicit local root and provides:

- `write_definition_record_if_absent`;
- `read_definition_record`;
- `write_manifest_create_only`;
- `write_bundle`;
- `read_manifest`; and
- `read_bundle`.

The API takes existing validated bundle models. It does not load projects,
invent runtime configuration, create run state, or invoke external systems.

## 5. Addressing And Integrity Boundary

Canonical records use the deterministic `canonical_record_hash` as their safe
file name. Identical validated records are reusable; different or corrupt
content at that address fails closed.

The existing public manifest definition references commit validated
source-content hashes rather than canonical wrapper hashes. At publication, a
private store envelope therefore records the exact canonical-record hashes that
resolved the manifest. Reads dereference those hashes and revalidate definition
kind, ID, version, schema version, source hash, bundle version, sensitivity, and
redaction posture against the public manifest. This preserves the accepted
public manifest/root format while preventing a later valid canonical variant
from making an earlier bundle ambiguous or unreadable.

Run IDs are hex-encoded for safe manifest file names. One file per run is the
create-only run-to-bundle binding; even an identical second manifest write is
rejected.

## 6. Atomicity And Failure Behavior

Records and manifests are serialized to unique temporary files, synced, and
published with create-new hard-link semantics. Temporary files are removed on
success and failure.

`write_bundle` validates the supplied manifest/record relationship, writes
missing immutable records, and publishes the manifest last. The manifest is the
commit marker. A failure cannot expose a partial manifest as a complete bundle;
content records written before a failed manifest publication may remain as
harmless immutable orphans for later garbage-collection planning.

## 7. Privacy And Redaction

- The store persists canonical validated models, not raw YAML or parser input.
- It does not persist source paths, provider payloads, command output,
  environment values, credentials, or tokens.
- Store and read-result Debug output redact the local root and model identities.
- Errors use stable codes and fixed messages without paths, IDs, or corrupt
  payload values.
- Serialized canonical records remain sensitive storage material and are not
  presented as terminal-safe output.

## 8. Test Coverage

Focused tests cover:

- complete write/read across a reopened store;
- stable reads after a second canonical sensitivity variant is stored;
- idempotent identical record writes;
- conflicting content at an existing address without overwrite;
- duplicate manifest rejection and immutable run binding;
- deterministic safe record and manifest file names;
- missing-definition rejection before manifest publication;
- missing-record rejection after reopening a previously complete bundle;
- corrupt-record rejection without payload leakage;
- manifest storage-identity mismatch;
- temporary-file cleanup; and
- redacted Debug plus absence of runtime state or event writes.

Existing immutable bundle builder, model, runtime, approval, policy,
SideEffect, report, and provider behavior remains covered by the workspace
suite.

## 9. Governed Phase Evidence

- Workflow: `dg/implement`.
- Run: `run-1783944684867429000-2`.
- Approval: `approval/run-1783944684867429000-2/implementation-approved`.
- Presentation: `presentation/ccdc8a7ece6a693e`.
- Approval outcome: granted through the proof-enforced presentation path under
  delegated-maintainer authority after the complete handoff was relayed.
- Event summary: 39 events, one approval, zero retries, zero escalations, and
  one persisted approval-presentation proof record with event marker present.
- Out-of-kernel work: Codex inspected the accepted model and store patterns,
  authored the implementation and tests, and ran repository validation. The
  kernel governed scope and approval; it did not edit files or run checks.

## 10. Validation Commands And Results

All required validation passes:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

The focused immutable-store suite passes 12 tests. Formatting, strict workspace
clippy, the full workspace suite, docs validation, and diff hygiene pass; only
existing explicitly opt-in live integration tests remain ignored.

## 11. Remaining Known Limitations

- The exact canonical-record hash mapping is a private local-store envelope,
  not yet part of the public manifest root or schema.
- Immutable orphan cleanup is not implemented.
- File permissions and remote or tenant storage are not modeled.
- Stored definitions are inspection material, not executable replay proof.
- Handler/check identity remains explicitly unattested.

## 12. Recommended Next Phase

Perform a focused maintainer review of the local immutable store. Review
content-address integrity, run-binding semantics, atomic publication, restart
behavior, privacy, compatibility, and non-overclaiming before any executor
integration.

Do not add automatic bundle generation, `RunCreated` binding, replay, handler
attestation, scoped authority enforcement, provider mutation expansion, CLI,
schemas, hosted behavior, or release changes during review.
