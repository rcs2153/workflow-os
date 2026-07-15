# Immutable Run Bundle Local Store Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups; proceed to an explicit executor
bundle-path implementation phase.**

The implementation provides a narrow create-only local persistence boundary
for canonical records and run-bound manifests. The review found no blocker in
content addressing, duplicate handling, complete-bundle publication, restart
reads, error safety, or scope control.

## 2. Scope Verification

The phase stayed within the approved storage-only scope. It added a local store,
a validated stored-bundle read result, focused tests, roadmap status, and an
implementation report.

It did not add:

- executor integration or automatic bundle construction;
- `RunCreated` or `WorkflowRunIdentity` bundle fields;
- approval, resume, policy, hook, SideEffect, or report behavior;
- replay or handler/check attestation;
- provider calls or provider writes;
- CLI, workflow schemas, SDKs, examples, or UI;
- hosted storage, garbage collection, scoped authority enforcement, or release
  changes.

## 3. Store API Assessment

`LocalImmutableRunBundleStore` is explicit, local, and testable. It accepts a
caller-selected root and already validated immutable bundle models. It has no
hidden global state, runtime dependency, provider dependency, or `StateBackend`
dependency.

The public methods separate canonical-record writes, manifest publication, and
complete-bundle reads. `StoredImmutableRunBundle` exposes only validated
read-only parts. Debug output redacts the store root and delegates to the
already redaction-safe manifest Debug implementation.

## 4. Addressing And Run-Binding Assessment

Canonical records are addressed by lowercase SHA-256 canonical-record hashes.
Identical writes are idempotent. Different or corrupt content at an occupied
address fails without overwrite.

Run IDs are hex-encoded into one manifest address per run. Manifest writes are
strictly create-only, including identical duplicates, so a run cannot be
silently rebound.

The accepted public manifest commits source-content hashes. The local store
adds a private envelope containing the exact canonical-record hashes selected
at publication. Reads dereference those exact hashes and revalidate kind, ID,
version, schema, source hash, bundle version, sensitivity, and redaction posture
against the public manifest. This avoids scan ambiguity and prevents a later
canonical variant from making an earlier bundle unreadable.

## 5. Atomicity And Failure Assessment

Records and envelopes are written to unique temporary files, synced, and
published with create-new hard-link semantics. Temporary files are removed.

`write_bundle` validates the complete manifest/record relationship, writes
content-addressed records, and publishes the manifest envelope last. The
manifest remains the commit marker. A failed manifest write cannot expose a
partial bundle as complete. Harmless immutable records may remain orphaned,
which matches the accepted plan and is disclosed honestly.

Concurrent identical record publication resolves idempotently. Concurrent or
duplicate manifest publication rejects rather than overwrites.

## 6. Read And Corruption Assessment

Deserialization revalidates both existing manifest roots and canonical-record
hashes. Store reads additionally validate storage addresses and manifest-record
alignment. Missing, corrupt, mismatched, conflicting, unreferenced, and
ambiguous material fails closed with stable fixed messages.

The review added one focused test for deleting a referenced record after a
successful write and reopening the store. The complete bundle becomes
unavailable with a bounded `not_found` error as required.

## 7. Privacy And Error Safety Assessment

The persisted data is canonical validated model content, not raw YAML, source
snippets, parser payloads, command output, provider bodies, environment values,
credentials, or tokens. It remains sensitive storage material and is not
presented as terminal-safe output.

Debug output does not expose roots, IDs, or hashes. Store errors use stable
codes and fixed messages without paths, identities, or corrupt payload values.
Deserialization failures are collapsed to the bounded invalid-record error.

## 8. Compatibility And Boundary Assessment

No existing manifest, definition-record, builder, runtime, state, or public
workflow schema changed. The private store envelope allows the local format to
commit canonical addresses without claiming that the accepted public manifest
root already contains them.

This is suitable for an explicit opt-in executor path after review. It is not
yet suitable for default automatic bundle creation, executable replay, hosted
storage, or cross-version schema guarantees.

## 9. Test Quality Assessment

The 12 focused tests cover:

- complete write/read across a reopened store;
- stable historical reads after another canonical sensitivity variant;
- identical record idempotency;
- same-address conflict rejection without overwrite;
- duplicate manifest rejection;
- safe deterministic file names;
- missing definitions before publication;
- corrupt records without payload leakage;
- missing records after restart;
- manifest storage-identity mismatch;
- temporary-file cleanup; and
- Debug safety plus absence of runtime state/event writes.

The full workspace suite protects existing bundle models/builders, runtime,
approval, policy, SideEffect, report, and provider behavior.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Decide whether a future public manifest version should commit both source and
  canonical-record hashes before default executor adoption or schema exposure.
- Add fault-injection coverage for write, sync, and publish failures if the
  local store becomes production-critical.
- Define local directory permission and durability expectations, including
  parent-directory sync posture, before making stronger crash-durability claims.
- Plan orphan-record inspection and garbage collection separately.
- Keep stored definitions non-executable until handler/check attestation,
  authority, external-input, and SideEffect reconciliation are designed.

## 12. Governed Review Evidence

- Workflow: `dg/review`.
- Run: `run-1784152037305994000-2`.
- Approval: `approval/run-1784152037305994000-2/review-scope-approved`.
- Presentation: `presentation/81126631faa3faa6`.
- Approval outcome: granted through the proof-enforced presentation path under
  delegated-maintainer authority after the complete handoff was relayed.
- Event summary: 39 events, one approval, zero retries, zero escalations, and
  one persisted approval-presentation proof record with event marker present.
- Validation summary: formatting, strict workspace clippy, the focused 12-test
  store suite, the full workspace suite, docs validation, and diff hygiene
  pass. Only existing explicitly opt-in live integration tests remain ignored.
- Out-of-kernel work: Codex inspected the code, tests, model invariants, and
  documentation; added one focused review regression; authored this review; and
  ran validation. The kernel governed scope and approval only.

## 13. Recommended Next Phase

Implement one explicit opt-in executor bundle path that builds and persists the
immutable bundle before `RunCreated`, then binds optional bundle identity into
the new run. Preserve all existing executor APIs and legacy unbundled reads.

Do not make bundle generation automatic by default, execute from stored
definitions, alter approval/resume semantics, add replay or attestation, call
providers, authorize writes, expose CLI/schema behavior, or begin hosted
storage in that phase.
