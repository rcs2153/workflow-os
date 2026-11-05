# WorkReport Body Commitment Read-Path Enforcement Report

## Summary

Workflow OS now verifies every new-format `WorkReport` artifact against its
retained complete-body commitment before an ordinary durable store read or list
operation returns report content.

This closes the bounded gap left by the persistence-anchor phase. The expected
commitment remains in create-only artifact-envelope metadata outside the report
body. The read boundary recomputes the versioned commitment from the complete
validated body and compares it with that retained value.

## Implemented Boundary

One shared fail-closed validator now sits behind all supported artifact stores:

- local filesystem `read_work_report_artifact` and
  `list_work_report_artifacts`;
- SQLite `read_work_report_artifact` and `list_work_report_artifacts`; and
- PostgreSQL `read_work_report_artifact` and
  `list_work_report_artifacts`.

SQLite performs its existing relational-index identity check before commitment
verification. PostgreSQL retains its existing typed-record load boundary.
Local filesystem reads retain their existing bounded path and JSON handling.
No backend returns a new-format artifact until full artifact validation,
including complete-body commitment verification, succeeds.

## Failure Semantics

All backend read/list integrity failures use the existing stable error:

```text
work_report_artifact.read.corrupt
```

The error does not expose:

- expected or actual commitment values;
- report summaries or citations;
- serialized payloads;
- storage paths; or
- run and report identifiers.

A caller therefore learns that the stored artifact cannot be safely returned,
not which field changed or what the protected content contained.

## Legacy Semantics

An artifact that predates `metadata.body_commitment` remains structurally
readable with posture:

```text
legacy_unverified
```

The kernel does not compute a replacement expected value from the report being
read and does not silently upgrade the artifact to verified. Migration remains
a separately reviewed future concern.

## Deliberate Serde Boundary

`WorkReportArtifactRecord` deserialization remains structure-only. This is
intentional:

- callers can decode and classify legacy records;
- ordinary durable reads enforce at an explicit security boundary;
- fixtures and migration tooling do not accidentally acquire an assurance
  claim merely by deserializing data; and
- inspect, recovery, reconciliation, and replay can be integrated through
  separately reviewed composition phases.

Direct serde success is not evidence that a retained commitment was verified.

## Validation Evidence

Focused tests prove that:

- local read rejects a structurally valid report-only substitution;
- local list rejects the same substitution;
- read and list return the same stable payload-free error;
- neither changed report text nor commitment values leak through the error;
- legacy artifacts remain readable and explicitly `legacy_unverified`; and
- the existing explicit validation boundary continues to detect substitution.

The phase also validates all three backend implementations through compile,
Clippy, and the full `workflow-core` test suite.

## Security Claim

This phase detects report-only substitution when the retained envelope
commitment remains intact. It does not protect against a storage owner capable
of rewriting both the report body and its retained commitment. It does not add
signatures, external witnesses, independent attestation, or cryptographic
storage-owner tamper resistance.

## Out Of Scope

This phase does not add or broaden:

- inspect or report-metadata projection enforcement;
- recovery, duplicate reconciliation, or replay enforcement;
- CLI, schema, or SDK surfaces;
- migration or backfill;
- signing or external witnesses;
- provider mutations or publication; or
- hosted or distributed trust claims.

## Next Phase

Apply the same retained-commitment boundary to existing inspect and report
metadata projection consumers. Preserve explicit legacy disclosure and keep
recovery, duplicate reconciliation, and replay as later separately reviewed
composition steps.
