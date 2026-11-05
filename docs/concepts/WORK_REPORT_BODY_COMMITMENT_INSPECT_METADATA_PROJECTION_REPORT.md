# WorkReport Body Commitment Inspect And Metadata Projection Report

## Summary

Workflow OS now prevents existing inspect and current-authority WorkReport
metadata consumers from projecting verified new-format artifact posture unless
the persisted report body has passed the retained complete-body commitment
boundary.

This phase composes the persistence anchor and ordinary read/list enforcement
into existing projection paths. It does not create a new verification system or
broaden the product surface.

## Inspect Boundary

Human and JSON `workflow-os inspect` output load report artifacts through
`WorkReportArtifactStore::list_work_report_artifacts`. Every supported durable
backend now verifies a new-format artifact before returning it from that method.

Inspect therefore projects one of two explicit read postures:

```text
verified_persisted_envelope_anchor
legacy_unverified
```

The first posture means the store boundary recomputed the complete body
commitment and matched it to the retained artifact-envelope value. The second
means the artifact predates the retained commitment and remains readable
without being silently upgraded.

A structurally valid report-only substitution fails with the stable
`work_report_artifact.read.corrupt` error before human or JSON inspect output can
present report metadata. The error does not expose changed report text or
commitment values.

Adding this field to the existing inspect projection strengthens an existing
surface. It does not add a command, schema, SDK, or generalized artifact
verification API.

## Current-Authority Metadata Boundary

The current-authority WorkReport metadata consumer performs an independent
`WorkReportArtifactRecord::validate` call after its bounded store read and
before projecting report id, run id, terminal status, or sensitivity.

Report-only substitution maps to the consumer's existing payload-free
`StoreFailure` posture. No metadata view or non-authorizing receipt is returned
for the substituted artifact.

This independent check remains useful even when a concrete durable store also
enforces reads. Current-authority tests use an injected store contract, and the
consumer does not assume every implementation has inherited a stronger trust
claim than the trait requires.

## Other Metadata Consumers

Continuation collection lists WorkReport artifacts through the same enforced
store boundary. It records only bounded artifact availability and does not
claim that arbitrary report content was independently attested.

Provider-write paths retain their existing explicit artifact validation. This
phase does not broaden provider mutation behavior.

## Validation Evidence

Focused tests prove that:

- human inspect projects verified persisted-envelope posture after an enforced
  read;
- JSON inspect projects the same posture;
- human and JSON inspect both reject a report-only substitution before output;
- inspect preserves explicit legacy-unverified posture;
- current-authority metadata projection rejects a structurally valid
  report-only substitution as a bounded store failure; and
- failure output does not include substituted report text.

The phase also runs formatter, warnings-denied Clippy, the full relevant core
and CLI test suites, documentation checks, and diff whitespace validation.

## Security Claim

This phase proves that the identified inspect and metadata projection paths do
not bypass the retained commitment boundary for report-only substitution. It
does not prove storage-owner tamper resistance when an actor can rewrite both
the report body and retained commitment. It adds no signature, witness,
producer authentication, or external attestation.

## Out Of Scope

This phase does not add or broaden:

- recovery, duplicate reconciliation, or replay enforcement;
- migration or legacy backfill;
- new CLI commands, schemas, or SDKs;
- signing or external witnesses;
- provider mutations or publication; or
- hosted or distributed trust claims.

## Next Phase

Apply the retained-commitment boundary to recovery and duplicate reconciliation
paths. Keep replay as a later separately reviewed phase.
