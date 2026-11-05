# WorkReport Body Commitment Model Report

## Summary

Workflow OS now has a model-only, versioned commitment over the complete
validated `WorkReport` body.

The model closes the vocabulary gap identified by report-body substitution
testing without claiming that persisted reports are protected yet. The
commitment is not stored in artifact envelopes, terminal state, events, or
stores in this phase. No read or replay path enforces it.

## V1 Canonical Boundary

V1 hashes the compact UTF-8 JSON serialization of one explicit envelope:

```text
domain: workflow-os/work-report-body-commitment/v1
version: v1
report: complete validated WorkReport body
```

The committed report body includes:

- report ID, report-contract ID, and report-contract version;
- workflow ID and version, schema version, specification hash, run ID,
  terminal status, generation time, generating actor, and optional correlation
  ID;
- ordered sections, summaries, and ordered citations;
- every citation target, optional summary, missing posture, sensitivity, and
  redaction metadata;
- incomplete-work disclosures, known limitations, risks, and handoff notes,
  including their ordered citations;
- the complete optional high-assurance approval disclosure;
- report sensitivity; and
- report redaction metadata.

V1 treats collection order as part of the body. The commitment envelope is
domain-separated, and the version participates in the committed bytes. A
golden-vector test fixes the exact current encoding. Any change to field
coverage, ordering, framing, or serialization semantics requires an explicit
version decision rather than an unnoticed hash change.

## Model Vocabulary

`WorkReportBodyCommitmentVersion` currently admits only `v1`.

`WorkReportBodyCommitment` stores the version and SHA-256 commitment. Its debug
surface redacts the digest. Construction validates the report before creating
canonical bytes.

`WorkReportBodyCommitmentPosture` distinguishes:

- `available_unanchored`: a syntactically valid commitment value is available,
  but no independent persistence anchor or construction provenance is implied;
- `legacy_unverified`: an older artifact has no complete-body commitment and
  must not be called verified.

The posture deliberately does not say `computed`. A caller can deserialize a
syntactically valid commitment claim, so the value alone cannot prove that Core
computed it from the report in the current call.

## In-Memory Verification

The model can recompute V1 from a supplied validated report and compare it with
an expected commitment. A mismatch returns:

```text
work_report.body_commitment.mismatch
```

The error does not expose the expected digest, actual digest, report text,
path, or payload. Invalid input and canonicalization failure also use stable,
payload-free error codes.

This helper is a pure comparison primitive. It does not establish where the
expected value came from or whether a storage owner can rewrite both sides.

## Security Properties

- Complete report prose and metadata participate in the commitment.
- Ordered collections cannot be reordered without changing V1.
- Domain and version are committed with the body.
- Debug output does not reveal the digest or report payload.
- A changed report fails comparison with one stable non-leaking error.
- A deserialized or caller-created commitment remains available but
  unanchored; its hash is not construction provenance.
- Legacy artifacts remain explicitly unverified.

## Explicit Non-Goals

This phase did not add:

- a commitment field to `WorkReportArtifactRecord` or artifact metadata;
- an expected commitment in terminal persistence state, events, receipts, or
  store indexes;
- atomic persistence or crash reconciliation;
- read, list, inspect, metadata-projection, recovery, duplicate-reconciliation,
  or replay enforcement;
- legacy migration;
- human or JSON CLI projection;
- schemas or SDK exposure;
- signing, an external witness, or independent producer identity;
- provider mutations or release publication; or
- storage-owner tamper resistance.

## Governed Phase

- Workflow: `dg/implement`
- Run: `run-1793856771174440000-2`
- Approval: `approval/run-1793856771174440000-2/implementation-approved`
- Presentation: `presentation/90b849309c4c2116`
- Approval outcome: granted by the delegated maintainer after complete handoff
  presentation
- Kernel freshness: rebuilt from the exact checkout before phase start
- Repository edits and validation ran outside the kernel under the approved
  execution scope

## Validation

- Focused deterministic, golden-vector, body-family mutation, legacy-posture,
  deserialized-claim, debug-redaction, and mismatch tests passed.
- `cargo fmt --all --check`: passed.
- Warnings-denied all-target/all-feature Core Clippy: passed.
- Complete Core all-feature test suite and doc tests: passed.
- Documentation and diff checks are required before phase close.

## Next Phase

Proceed only to a bounded persistence-anchor phase. Store the expected V1
commitment outside mutable report content through existing terminal artifact
state and supported stores, with explicit atomicity or crash reconciliation and
legacy-unverified behavior. Do not combine that phase with broad read/replay or
product-surface enforcement.
