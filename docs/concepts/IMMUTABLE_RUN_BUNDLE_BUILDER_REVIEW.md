# Immutable Run Bundle Builder Review

Review date: 2026-07-13

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups; proceed to create-only local
immutable stores.**

The builder composes the accepted manifest and canonical definition-record
models into one deterministic in-memory result without persistence or runtime
mutation. It revalidates the supplied project, resolves one workflow and its
complete current skill and policy reference set, derives manifest references
from the returned records, and fails closed when existing model invariants do
not hold.

## 2. Scope Verification

The phase stayed within the approved pure-construction boundary. It added an
explicit request, an in-memory builder, a read-only result, focused tests, and
honest documentation.

It did not add stores, persistence, create-only writes, executor or resume
integration, event or snapshot changes, executable replay, handler or check
attestation, scoped authority, provider calls or writes, CLI behavior, schemas,
SDK changes, examples, hosted behavior, or release changes.

## 3. Builder API Assessment

`ImmutableRunBundleBuildRequest` makes run identity, bundle identity, resolved
context commitment, execution posture, handler posture, actor, sensitivity,
and redaction explicit. The request borrows a `ProjectBundle` and does not read
hidden process, filesystem, runtime, or provider state.

`build_immutable_run_bundle` returns one validated manifest and the unique
canonical definition records needed by that manifest. `ImmutableRunBundleBuildResult`
keeps fields private and exposes read-only accessors plus an explicit
`into_parts` ownership transfer.

## 4. Selection And Resolution Assessment

The builder resolves exactly one workflow after default project validation has
rejected duplicate workflow IDs. Each ordered step resolves a unique skill by
ID and explicit version, or by the existing single-version rule. Repeated use
of one skill creates one canonical record while preserving a separate
step-scoped manifest reference.

Policy selection covers workflow retry and escalation references plus step
requirement, approval, retry, and escalation references. Policy versioning is
explicitly deferred in the current policy model, so ID-based deduplication is
consistent with existing validation. Unreferenced skills and policies are not
included.

## 5. Integrity And Determinism Assessment

Workflow, skill, and policy source-content hashes come only from loader-owned
`LoadedSpec` values. Manifest references are created from the canonical records
returned beside the manifest rather than reconstructed from unrelated caller
input.

Existing constructors enforce canonical typed definition content, workflow
source-hash alignment, deterministic reference ordering, exactly one workflow
reference, workflow identity alignment, unique references, handler-to-skill
alignment, execution posture validation, and deterministic root hashing.
Identical explicit builder inputs produce identical roots.

The resolved execution-context hash remains caller supplied. The builder does
not claim to recompute executor request posture or make the bundle executable.

## 6. Validation And Error Assessment

The builder rejects an invalid project before constructing records. Missing or
ambiguous workflow, skill, or policy resolution and downstream model
invariant failures return stable structured errors. Builder-owned errors use
fixed messages and do not echo requested identities, paths, definition
contents, hashes, or rejected values.

Default project validation is intentionally conservative. A
`ProjectValidationCapability::ReportArtifactCapable` posture cannot yet be
carried by the builder request, so a project valid only under that explicit
capability cannot currently use this helper. That is an honest limitation, not
a reason to weaken default validation.

## 7. Privacy And Redaction Assessment

Manual request and result Debug implementations omit project data, IDs, hashes,
definition content, and local paths. Canonical records exclude raw YAML,
comments, and source locations. The helper accepts no provider payloads,
command output, environment values, credentials, authorization headers,
private keys, tokens, prompts, or transcripts.

Serialized canonical records contain validated definition content and remain
sensitive storage shapes. They are not safe default operator output, and this
review does not authorize exposing them through a CLI.

## 8. Non-Mutation Assessment

The builder performs no writes and has no dependency on a state backend,
artifact store, runtime executor, adapter, or external system. Tests verify
that construction creates neither runtime state nor filesystem artifacts.
There is no path that appends events, creates a run, resumes a run, or changes
workflow pass/fail semantics.

## 9. Test Quality Assessment

The focused suite covers valid construction, record deduplication, per-step
skill references, loader-owned hash provenance, exclusion of unreferenced
definitions, deterministic roots, handler mismatch failure, missing workflow
non-leakage, invalid-project rejection, Debug and storage-shape privacy, and
absence of runtime/filesystem mutation.

The suite is sufficient for the in-memory phase. Before persistence, add a
fixture that exercises workflow-level retry/escalation and step-level approval,
retry, and escalation references together, and add an explicit cross-check
that every manifest reference resolves to exactly one returned canonical
record.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Decide how an explicit validated capability posture is represented before
  capability-aware projects enter the bundle path.
- Broaden policy-reference-family coverage before implementing persistence.
- Add an explicit result-level manifest-reference-to-record integrity check at
  the future store boundary, even though the current builder derives both from
  the same records.
- Decide whether durable manifest references commit source-content hashes,
  canonical-record hashes, or both before declaring a persisted format.
- Keep handler posture explicitly unattested until a separate threat model and
  implementation are reviewed.

## 12. Governed Review Evidence

- Workflow: `dg/review`.
- Run: `run-1783925238225176000-2`.
- Approval: `approval/run-1783925238225176000-2/review-scope-approved`.
- Presentation: `presentation/0152e6dd69e2a949`.
- Approval outcome: granted through the proof-enforced presentation path under
  delegated-maintainer authority after the complete handoff was relayed.
- Event summary: 39 events, one approval, zero retries, zero escalations, and
  one persisted approval-presentation proof record.
- Validation summary: formatting, strict workspace clippy, the full workspace
  test suite, docs validation, and diff hygiene passed.
- Out-of-kernel work: Codex inspected the implementation and its underlying
  model invariants, authored this review, and ran validation. The kernel
  governed scope and approval only.

## 13. Recommended Next Phase

Implement create-only local immutable stores for canonical definition records
and manifests. The phase should prove deterministic safe keys, duplicate-write
rejection, corruption detection, restart reads, and failure atomicity without
executor integration.

Do not add automatic bundle generation, bind bundles to `RunCreated`, change
approval or resume behavior, expose CLI commands or schemas, attest handlers,
enforce scoped authority, call providers, or authorize writes outside the
bounded local bundle store.
