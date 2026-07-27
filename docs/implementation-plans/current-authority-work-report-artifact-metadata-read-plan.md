# Current-Authority WorkReport Artifact Metadata Read Plan

Status: Planning complete; implementation not started.

Related foundations:

- [Scoped Runtime Authority And Capability Projection Plan](scoped-runtime-authority-capability-projection-plan.md)
- [Current-Authority One-Time-Use And Replay Posture Plan](current-authority-one-time-use-replay-posture-plan.md)
- [Current-Authority Use-Boundary Hardening Review](../concepts/CURRENT_AUTHORITY_USE_BOUNDARY_HARDENING_REVIEW.md)
- [EvidenceReference](../concepts/evidence-reference.md)
- [Governed Work Pattern](../concepts/governed-work-pattern.md)

## 1. Executive Summary

Workflow OS now has a private same-call current-authority use boundary. It
freshly reads one registered authority source, reruns capability and
required-context resolution, and invokes one non-reusable borrowed consumer
only when the result is ready. Direct negative-path tests prove that expired or
revoked grants, unresolved prerequisites, changed context, mismatched
contracts, and source failures block before consumer invocation.

The next phase should replace the generic proof consumer with one concrete
Core-owned read-only operation:

```text
resolve current authority
  -> prove one exact required WorkReport target is authorized for bounded metadata
  -> read one artifact from an explicit WorkReportArtifactStore
  -> return only bounded artifact metadata
```

The first implementation must remain private to Core. It must not return the
contained `WorkReport`, expose a generic callback, create a reusable authority
handle, add executor behavior, or change persistence.

## 2. Selected Consumer

The selected first consumer is an exact `WorkReport` artifact metadata read.

It is appropriate because:

- `WorkReportArtifactStore` already provides an exact
  `read_work_report_artifact(run_id, report_id)` operation;
- a stored work report artifact is immutable after creation;
- `GovernedContextReferenceTarget::WorkReport` already provides typed
  exact-target vocabulary;
- `GovernedContextAccessLevel::BoundedMetadata` already distinguishes metadata
  access from reference-only visibility;
- the operation is useful to later governed handoff and context composition;
- it can prove current authority gates a real store read without introducing
  command execution, provider access, or mutation.

The first consumer is intentionally not a local-check handler invocation.
Even a nominally read-only command can execute arbitrary process behavior and
belongs after the read-only dereference boundary is proven.

## 3. Goals

- Add one private Core-owned operation for exact `WorkReport` artifact metadata
  lookup.
- Require fresh registered-source resolution for every call.
- Require the exact target to be declared as required bounded metadata in the
  immutable required-context contract.
- Require the target projection and current capability authority to be ready
  before any store read.
- Accept an explicit caller-supplied `WorkReportArtifactStore`; do not discover
  hidden state.
- Read at most one exact `(run_id, report_id)` artifact.
- Return a small payload-free metadata view.
- Preserve explicit blocked, source-failure, not-found, store-failure,
  succeeded, and ambiguous postures where ambiguity is honestly possible.
- Keep errors stable and non-leaking.
- Prove every pre-use failure performs zero store reads.

## 4. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- a public current-authority API;
- a public generic callback or consumer trait;
- returning or summarizing the contained `WorkReport`;
- reading report sections, citations, notes, risks, or disclosures;
- arbitrary governed-context dereference;
- list or search operations;
- executor integration or runtime defaults;
- local-check or skill execution;
- provider or OpenShell integration;
- sandbox execution;
- SideEffect execution or new mutation families;
- event or audit append behavior;
- new persistence or store implementations;
- workflow schemas, SDKs, CLI behavior, examples, dependencies, or release
  posture changes;
- durable replay prevention, distributed locking, worker leasing, or
  cross-process one-time-use claims.

## 5. Source-Of-Truth Boundaries

| Concern | Source of truth |
| --- | --- |
| Current authority facts | One registered current-authority source snapshot |
| Required target and access level | Immutable required-context contract |
| Execution identity | `RequiredContextExecutionBinding` |
| Current capability readiness | Fresh same-call capability resolution |
| Target availability and sensitivity | Fresh governed-context projection |
| Artifact existence and validated record | Explicit `WorkReportArtifactStore` |
| Artifact body | Stored `WorkReportArtifactRecord`, not returned by this consumer |
| Run history | Workflow event log, not this metadata view |

An available or stored report is not automatically authorized. An authorized
reference does not prove the artifact exists. A successful metadata read does
not authorize report-body access or any subsequent operation.

## 6. Candidate Private API

Names are provisional and should follow the existing private module style:

```rust
pub(super) struct CurrentAuthorityWorkReportMetadataReadInput<'a> {
    pub(super) execution_binding: &'a RequiredContextExecutionBinding,
    pub(super) contract: &'a RequiredContextContractBinding,
    pub(super) report_id: &'a WorkReportId,
    pub(super) evaluated_at: Timestamp,
    pub(super) redaction: &'a RedactionMetadata,
}

pub(super) struct CurrentAuthorityWorkReportMetadataView {
    report_id: WorkReportId,
    run_id: WorkflowRunId,
    terminal_run_status: WorkReportStatus,
    sensitivity: WorkReportSensitivity,
}

pub(super) enum CurrentAuthorityWorkReportMetadataReadOutcome {
    Found(CurrentAuthorityWorkReportMetadataView),
    NotFound,
    Blocked(CurrentAuthorityReadBlock),
    SourceFailure(CurrentAuthoritySourceFailure),
    StoreFailure,
}
```

The concrete method should remain on, or immediately adjacent to,
`RegisteredInMemoryCurrentAuthoritySource` so it can reuse the private
same-call resolver and cannot export the borrowed authority capability.

The existing `use_current_authority` helper returns only a bounded use posture,
not the consumer's value. The implementation must not duplicate authority
resolution to work around that boundary. It should capture one private
store-read result inside the `FnOnce`, map that result to the existing bounded
consumer result, and reconcile the captured result with the returned use
posture before producing the concrete metadata-read outcome. A missing or
inconsistent captured result must fail closed.

The exact shape may be smaller if implementation proves some fields redundant.
It must not become a generic store-read closure.

## 7. Exact Target Contract

Before authority resolution, the helper must validate that:

- the requested target is
  `GovernedContextReferenceTarget::WorkReport(report_id)`;
- the immutable contract contains that exact target;
- its access level is `BoundedMetadata`;
- its obligation is `Required`;
- no conflicting duplicate can exist under existing contract validation;
- the execution binding matches the contract and current request.

`ReferenceOnly` is insufficient. An optional requirement is insufficient for
the first consumer because absence must not silently authorize dereference.

Target-shape failure must return a stable non-leaking error before source or
store access. Error text must not include report IDs, run IDs, paths, report
content, or caller values.

## 8. Same-Call Read Sequence

The implementation sequence must be:

1. Validate the concrete metadata-read input and exact target contract.
2. Invoke the existing private current-authority same-call boundary.
3. Read the registered source and derive one coherent snapshot.
4. Rerun capability resolution and required-context consumption.
5. Stop without touching the artifact store unless authority is `Ready`.
6. Inside the one bounded consumer, verify the satisfied projection still
   covers the exact WorkReport target at `BoundedMetadata`.
7. Read exactly one artifact using the execution binding's run ID and the
   requested report ID.
8. Validate the returned artifact through its existing model boundary.
9. Project only report ID, run ID, terminal status, and sensitivity.
10. Drop the borrowed use capability before returning.

The artifact store must never receive authority objects or resolve policy
itself.

## 9. Outcome And Error Posture

The result must distinguish:

- `Found`: one validated artifact produced bounded metadata;
- `NotFound`: authority was ready, the exact read occurred, and no artifact was
  present;
- `Blocked`: current authority or required context was not ready, and the store
  was not touched;
- `SourceFailure`: the current-authority source could not provide a fresh,
  complete, coherent snapshot, and the store was not touched;
- `StoreFailure`: authority was ready, but the explicit store read failed.

Store errors must be mapped to one stable non-leaking code such as
`current_authority.work_report_metadata.store_read_failed`; the underlying
error message must not be copied.

`NotFound` must not fabricate metadata or evidence. It is not proof that the
artifact never existed outside the explicit store.

## 10. Metadata Boundary

The first metadata view may expose only:

- exact `WorkReportId`;
- exact `WorkflowRunId`;
- `WorkReportStatus`;
- `WorkReportSensitivity`.

Debug output must redact both IDs and show only bounded enum posture. The view
must not contain:

- `WorkReport`;
- report sections or citations;
- summaries, notes, limitations, risks, or incomplete-work disclosures;
- workflow/spec hashes;
- local paths;
- raw redaction metadata;
- provider payloads;
- command output;
- credentials or secret-like strings.

The first implementation should remain non-serializable and private. Public or
serialized context exposure requires a separate compatibility and privacy
review.

## 11. Replay, Freshness, And Concurrency

Every call must rerun current-authority source resolution. The metadata view is
an observation result, not an authority receipt and not permission for a later
read.

This phase can claim only same-process same-call gating. It cannot claim:

- durable one-time use;
- protection against a mutable store changing after the read;
- cross-worker replay prevention;
- transactional coupling between authority source and artifact store;
- distributed snapshot isolation.

The immutable artifact model reduces mutation risk, but later production
consumers still need explicit source/store consistency and restart semantics.

## 12. Privacy And Security

- Use existing validated constructors and store contracts.
- Never expose report bodies to the authority boundary output.
- Treat artifact IDs and run IDs as sensitive in Debug and errors.
- Do not propagate underlying store errors.
- Do not store or copy raw report content, provider payloads, command output,
  logs, environment values, credentials, tokens, or private keys.
- Fail closed on unknown target shape, insufficient access level, optional-only
  declaration, stale source, missing prerequisites, sensitivity mismatch, or
  artifact identity mismatch.
- Preserve the distinction between authorization, availability, existence, and
  successful read.

## 13. Future Tests

The implementation phase should prove:

1. ready exact bounded-metadata authority reads one artifact once;
2. the returned view contains only the four approved metadata fields;
3. the contained `WorkReport` is not returned or Debug-formatted;
4. reference-only access is rejected before source or store read;
5. optional-only context is rejected before source or store read;
6. a different report ID is rejected before store read;
7. a different run binding is rejected before store read;
8. unavailable or unknown target posture blocks before store read;
9. expired and revoked grants block before store read;
10. missing policy, approval, evidence, or check prerequisites block before
    store read;
11. stale, incomplete, future-dated, or incoherent source posture blocks before
    store read;
12. an absent artifact returns explicit `NotFound` after exactly one read;
13. store failure maps to a stable non-leaking outcome or error;
14. corrupt or identity-mismatched stored data fails safely;
15. sensitivity-ceiling mismatch blocks before store read;
16. repeated calls each rerun authority and perform independent exact reads;
17. changed contracts or bindings cannot reuse a previous result;
18. Debug output does not leak IDs, report text, paths, or secret-like values;
19. the private API is not exported from `workflow-core`;
20. no events, writes, executor state changes, files, provider calls, or CLI
    output are produced;
21. existing current-authority, required-context, WorkReport, store, and
    workspace tests still pass.

## 14. Proposed Implementation Sequence

1. Add the private input, view, and outcome types beside the registered source.
2. Add exact-target contract validation.
3. Add a private concrete metadata-read method that reuses the existing
   same-call boundary and reconciles its bounded use posture with one captured
   private store-read result.
4. Add an instrumented in-memory artifact store fixture proving exact read
   counts and zero-read blocked paths.
5. Add focused negative-path, privacy, and fixed-vector tests.
6. Update roadmap and create an implementation report.
7. Perform a separately governed focused maintainer review.
8. Do not expose the operation publicly or wire it into the executor during
   this implementation.

## 15. Acceptance Criteria

- The plan selects exactly one Core-owned read-only consumer.
- The future consumer is exact-target, metadata-only, and private.
- Current authority and required context are resolved in the same call before
  any store read.
- Every blocked/source-failure path requires zero store reads.
- No report body or generic callback can escape.
- Existing store and WorkReport models remain the source of truth.
- Runtime, provider, OpenShell, sandbox, SideEffect, write, event, schema, CLI,
  dependency, and release behavior remain unchanged.

## 16. Final Recommendation

Proceed next to a focused maintainer review of this plan. If accepted, implement
the private exact-target WorkReport artifact metadata read only.

Do not broaden directly into arbitrary context dereference, executor
integration, local-check execution, providers, OpenShell, sandbox execution, or
writes.
