# WorkReport Body Commitment Persistence Anchor Report

## Summary

Workflow OS now retains the expected V1 complete-body commitment outside the
mutable `WorkReport` body in durable artifact-envelope metadata.

The persistence anchor closes the gap between the existing model-only
commitment and supported create-only artifact storage. It does not yet make
ordinary read, list, inspect, recovery, reconciliation, or replay paths enforce
the commitment.

## Persistence Boundary

`WorkReportArtifactMetadata` now carries an optional
`WorkReportBodyCommitment` field. `WorkReportArtifactRecord::new` computes V1
from the complete validated report and places the expected value in metadata,
outside the report body.

The existing storage shape gives the phase one atomic boundary:

- the local filesystem writes the complete artifact envelope through one
  create-new atomic file operation;
- SQLite inserts the complete serialized envelope as one row; and
- PostgreSQL writes the complete serialized envelope in one serializable,
  create-only record operation.

The phase therefore adds no second-write crash window and needs no
crash-reconciliation protocol. The same serialized artifact envelope flows
through state migration without inventing a separate commitment store.

## Posture And Validation

`WorkReportBodyCommitmentPosture` now distinguishes:

- `available_unanchored`: a portable commitment value alone makes no
  persistence claim;
- `persisted_envelope_anchor`: artifact metadata retains the expected value
  outside the report body; and
- `legacy_unverified`: the artifact predates the field and has no retained
  expected value.

New artifact construction always produces `persisted_envelope_anchor`.
Deserialization treats an absent field as `legacy_unverified`; it never derives
or backfills a commitment from mutable report content.

Public artifact validation recomputes V1 and compares it with the envelope
anchor when present. Every supported artifact write path already calls that
validation boundary, so a deserialized report-only substitution cannot be
persisted through supported stores. The mismatch uses the existing stable,
payload-free error:

```text
work_report.body_commitment.mismatch
```

## Deliberately Deferred Enforcement

Artifact deserialization performs structural and duplicated-identity
validation but does not yet compare the body commitment. This preserves the
approved phase boundary: existing read and list paths can expose the retained
posture without silently becoming integrity-enforcing paths.

The next phase must add explicit enforcement before new-format report content
is returned from read and list operations. Inspect, metadata projection,
recovery, duplicate reconciliation, and replay remain later composition work.

## Security Properties

- New artifact envelopes retain an expected complete-body commitment outside
  mutable report content.
- Envelope and body are persisted atomically through all supported stores.
- Supported writes fail closed when the body does not match the retained
  expected value.
- Legacy records remain readable but explicitly unverified.
- Debug and mismatch errors do not expose commitment values or report payload.

This detects report-only substitution when explicit validation is invoked and
the envelope anchor remains intact. It does not protect against an owner who
can rewrite both the envelope and report body.

## Explicit Non-Goals

This phase did not add:

- general read, list, inspect, metadata-projection, recovery,
  duplicate-reconciliation, or replay enforcement;
- migration or silent backfill of legacy records;
- CLI or product projection;
- schemas or SDK exposure;
- signing, external witnesses, or independent producer identity;
- provider mutations or publication; or
- storage-owner tamper resistance.

## Governed Phase

- Workflow: `dg/implement`
- Run: `run-1793858367858886000-2`
- Approval: `approval/run-1793858367858886000-2/implementation-approved`
- Presentation: `presentation/f1bcf50535096390`
- Approval outcome: granted by the delegated maintainer after complete handoff
  presentation
- Kernel freshness: rebuilt from the exact checkout before phase start
- Repository edits and validation ran outside the kernel under the approved
  execution scope

## Validation

- Focused construction, serde, legacy, report-substitution, local persistence,
  duplicate, and migration tests passed.
- `cargo fmt --all --check`: passed.
- Warnings-denied all-target/all-feature Core Clippy: passed.
- Complete Core all-feature test suite and doc tests: passed.
- Documentation and diff checks: passed.

## Next Phase

Proceed only to bounded read/list enforcement for retained commitments. Keep
inspect, recovery, reconciliation, replay, product projection, migration,
signing, external witness, provider mutation, and publication work separate.
